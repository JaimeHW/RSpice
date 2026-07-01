#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_203(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign55840_e86483, assign55840_e86483_d_n0, assign55840_e86483_d_n2, assign55840_e86483_d_n4, assign55840_e86483_d_n5, assign55840_e86483_d_n6, assign55840_e86483_d_n7, assign55840_e86483_d_n8, assign55840_e86483_d_n9, assign55840_e86483_d_n10, assign55840_e86483_d_n11, assign55840_e86483_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 == 0.0)) {
        let (assign55840_e86481, assign55840_e86481_d_n0, assign55840_e86481_d_n2, assign55840_e86481_d_n4, assign55840_e86481_d_n5, assign55840_e86481_d_n6, assign55840_e86481_d_n7, assign55840_e86481_d_n8, assign55840_e86481_d_n9, assign55840_e86481_d_n10, assign55840_e86481_d_n11, assign55840_e86481_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign55840_e86478: f64 = (2.0 * 2.0);
                let assign55840_e86479: f64 = (1.0 / assign55840_e86478);
                let assign55840_e86480: f64 = (locals.var_dnm).powf(assign55840_e86479);
                (assign55840_e86480, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn0)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn2)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn4)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn5)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn6)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn7)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn8)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn9)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn10)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn11)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn14)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign55840_e86481, assign55840_e86481_d_n0, assign55840_e86481_d_n2, assign55840_e86481_d_n4, assign55840_e86481_d_n5, assign55840_e86481_d_n6, assign55840_e86481_d_n7, assign55840_e86481_d_n8, assign55840_e86481_d_n9, assign55840_e86481_d_n10, assign55840_e86481_d_n11, assign55840_e86481_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55840_e86483;
        locals.var_dnm_dn0 = assign55840_e86483_d_n0;
        locals.var_dnm_dn2 = assign55840_e86483_d_n2;
        locals.var_dnm_dn4 = assign55840_e86483_d_n4;
        locals.var_dnm_dn5 = assign55840_e86483_d_n5;
        locals.var_dnm_dn6 = assign55840_e86483_d_n6;
        locals.var_dnm_dn7 = assign55840_e86483_d_n7;
        locals.var_dnm_dn8 = assign55840_e86483_d_n8;
        locals.var_dnm_dn9 = assign55840_e86483_d_n9;
        locals.var_dnm_dn10 = assign55840_e86483_d_n10;
        locals.var_dnm_dn11 = assign55840_e86483_d_n11;
        locals.var_dnm_dn14 = assign55840_e86483_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign55850_e86500, assign55850_e86500_d_n0, assign55850_e86500_d_n2, assign55850_e86500_d_n4, assign55850_e86500_d_n5, assign55850_e86500_d_n6, assign55850_e86500_d_n7, assign55850_e86500_d_n8, assign55850_e86500_d_n9, assign55850_e86500_d_n10, assign55850_e86500_d_n11, assign55850_e86500_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55850_e86498: f64 = (1.0 / locals.var_dnm);
        (assign55850_e86498, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55850_e86500;
        locals.var_dnm_dn0 = assign55850_e86500_d_n0;
        locals.var_dnm_dn2 = assign55850_e86500_d_n2;
        locals.var_dnm_dn4 = assign55850_e86500_d_n4;
        locals.var_dnm_dn5 = assign55850_e86500_d_n5;
        locals.var_dnm_dn6 = assign55850_e86500_d_n6;
        locals.var_dnm_dn7 = assign55850_e86500_d_n7;
        locals.var_dnm_dn8 = assign55850_e86500_d_n8;
        locals.var_dnm_dn9 = assign55850_e86500_d_n9;
        locals.var_dnm_dn10 = assign55850_e86500_d_n10;
        locals.var_dnm_dn11 = assign55850_e86500_d_n11;
        locals.var_dnm_dn14 = assign55850_e86500_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign55860_e86519, assign55860_e86519_d_n0, assign55860_e86519_d_n2, assign55860_e86519_d_n4, assign55860_e86519_d_n5, assign55860_e86519_d_n6, assign55860_e86519_d_n7, assign55860_e86519_d_n8, assign55860_e86519_d_n9, assign55860_e86519_d_n10, assign55860_e86519_d_n11, assign55860_e86519_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55860_e86515: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign55860_e86517: f64 = (assign55860_e86515 * locals.var_dnm);
        (assign55860_e86517, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn4)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn5)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn8)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn9)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn14)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign55860_e86519;
        locals.var_tmf0_dn0 = assign55860_e86519_d_n0;
        locals.var_tmf0_dn2 = assign55860_e86519_d_n2;
        locals.var_tmf0_dn4 = assign55860_e86519_d_n4;
        locals.var_tmf0_dn5 = assign55860_e86519_d_n5;
        locals.var_tmf0_dn6 = assign55860_e86519_d_n6;
        locals.var_tmf0_dn7 = assign55860_e86519_d_n7;
        locals.var_tmf0_dn8 = assign55860_e86519_d_n8;
        locals.var_tmf0_dn9 = assign55860_e86519_d_n9;
        locals.var_tmf0_dn10 = assign55860_e86519_d_n10;
        locals.var_tmf0_dn11 = assign55860_e86519_d_n11;
        locals.var_tmf0_dn14 = assign55860_e86519_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign55870_e86540, assign55870_e86540_d_n0, assign55870_e86540_d_n2, assign55870_e86540_d_n4, assign55870_e86540_d_n5, assign55870_e86540_d_n6, assign55870_e86540_d_n7, assign55870_e86540_d_n8, assign55870_e86540_d_n9, assign55870_e86540_d_n10, assign55870_e86540_d_n11, assign55870_e86540_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55870_e86534: f64 = (locals.var_t5 * locals.var_xmp);
        let assign55870_e86536: f64 = (assign55870_e86534 * locals.var_dnm);
        let assign55870_e86538: f64 = (assign55870_e86536 / locals.var_arg);
        (assign55870_e86538, (((((((locals.var_t5_dn0 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn0)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn2 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn2)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn4 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn4)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn5 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn5)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn6 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn6)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn7 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn7)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn8 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn8)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn9 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn9)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn10 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn10)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn11 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn11)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn14 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn14)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55870_e86540;
        locals.var_t0_dn0 = assign55870_e86540_d_n0;
        locals.var_t0_dn2 = assign55870_e86540_d_n2;
        locals.var_t0_dn4 = assign55870_e86540_d_n4;
        locals.var_t0_dn5 = assign55870_e86540_d_n5;
        locals.var_t0_dn6 = assign55870_e86540_d_n6;
        locals.var_t0_dn7 = assign55870_e86540_d_n7;
        locals.var_t0_dn8 = assign55870_e86540_d_n8;
        locals.var_t0_dn9 = assign55870_e86540_d_n9;
        locals.var_t0_dn10 = assign55870_e86540_d_n10;
        locals.var_t0_dn11 = assign55870_e86540_d_n11;
        locals.var_t0_dn14 = assign55870_e86540_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign55880_e86559, assign55880_e86559_d_n0, assign55880_e86559_d_n2, assign55880_e86559_d_n4, assign55880_e86559_d_n5, assign55880_e86559_d_n6, assign55880_e86559_d_n7, assign55880_e86559_d_n8, assign55880_e86559_d_n9, assign55880_e86559_d_n10, assign55880_e86559_d_n11, assign55880_e86559_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55880_e86555: f64 = locals.var_t5;
        let assign55880_e86557: f64 = (assign55880_e86555 - locals.var_tmf0);
        (assign55880_e86557, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn4 - locals.var_tmf0_dn4), (locals.var_t5_dn5 - locals.var_tmf0_dn5), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn8 - locals.var_tmf0_dn8), (locals.var_t5_dn9 - locals.var_tmf0_dn9), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign55880_e86559;
        locals.var_t4_dn0 = assign55880_e86559_d_n0;
        locals.var_t4_dn2 = assign55880_e86559_d_n2;
        locals.var_t4_dn4 = assign55880_e86559_d_n4;
        locals.var_t4_dn5 = assign55880_e86559_d_n5;
        locals.var_t4_dn6 = assign55880_e86559_d_n6;
        locals.var_t4_dn7 = assign55880_e86559_d_n7;
        locals.var_t4_dn8 = assign55880_e86559_d_n8;
        locals.var_t4_dn9 = assign55880_e86559_d_n9;
        locals.var_t4_dn10 = assign55880_e86559_d_n10;
        locals.var_t4_dn11 = assign55880_e86559_d_n11;
        locals.var_t4_dn14 = assign55880_e86559_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign55890_e86574, assign55890_e86574_d_n0, assign55890_e86574_d_n2, assign55890_e86574_d_n4, assign55890_e86574_d_n5, assign55890_e86574_d_n6, assign55890_e86574_d_n7, assign55890_e86574_d_n8, assign55890_e86574_d_n9, assign55890_e86574_d_n10, assign55890_e86574_d_n11, assign55890_e86574_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55890_e86574;
        locals.var_t0_dn0 = assign55890_e86574_d_n0;
        locals.var_t0_dn2 = assign55890_e86574_d_n2;
        locals.var_t0_dn4 = assign55890_e86574_d_n4;
        locals.var_t0_dn5 = assign55890_e86574_d_n5;
        locals.var_t0_dn6 = assign55890_e86574_d_n6;
        locals.var_t0_dn7 = assign55890_e86574_d_n7;
        locals.var_t0_dn8 = assign55890_e86574_d_n8;
        locals.var_t0_dn9 = assign55890_e86574_d_n9;
        locals.var_t0_dn10 = assign55890_e86574_d_n10;
        locals.var_t0_dn11 = assign55890_e86574_d_n11;
        locals.var_t0_dn14 = assign55890_e86574_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign55900_e86590, assign55900_e86590_d_n0, assign55900_e86590_d_n2, assign55900_e86590_d_n4, assign55900_e86590_d_n5, assign55900_e86590_d_n6, assign55900_e86590_d_n7, assign55900_e86590_d_n8, assign55900_e86590_d_n9, assign55900_e86590_d_n10, assign55900_e86590_d_n11, assign55900_e86590_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign55900_e86590;
        locals.var_t4_dn0 = assign55900_e86590_d_n0;
        locals.var_t4_dn2 = assign55900_e86590_d_n2;
        locals.var_t4_dn4 = assign55900_e86590_d_n4;
        locals.var_t4_dn5 = assign55900_e86590_d_n5;
        locals.var_t4_dn6 = assign55900_e86590_d_n6;
        locals.var_t4_dn7 = assign55900_e86590_d_n7;
        locals.var_t4_dn8 = assign55900_e86590_d_n8;
        locals.var_t4_dn9 = assign55900_e86590_d_n9;
        locals.var_t4_dn10 = assign55900_e86590_d_n10;
        locals.var_t4_dn11 = assign55900_e86590_d_n11;
        locals.var_t4_dn14 = assign55900_e86590_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign55910_e86606, assign55910_e86606_d_n0, assign55910_e86606_d_n2, assign55910_e86606_d_n4, assign55910_e86606_d_n5, assign55910_e86606_d_n6, assign55910_e86606_d_n7, assign55910_e86606_d_n8, assign55910_e86606_d_n9, assign55910_e86606_d_n10, assign55910_e86606_d_n11, assign55910_e86606_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55910_e86606;
        locals.var_t0_dn0 = assign55910_e86606_d_n0;
        locals.var_t0_dn2 = assign55910_e86606_d_n2;
        locals.var_t0_dn4 = assign55910_e86606_d_n4;
        locals.var_t0_dn5 = assign55910_e86606_d_n5;
        locals.var_t0_dn6 = assign55910_e86606_d_n6;
        locals.var_t0_dn7 = assign55910_e86606_d_n7;
        locals.var_t0_dn8 = assign55910_e86606_d_n8;
        locals.var_t0_dn9 = assign55910_e86606_d_n9;
        locals.var_t0_dn10 = assign55910_e86606_d_n10;
        locals.var_t0_dn11 = assign55910_e86606_d_n11;
        locals.var_t0_dn14 = assign55910_e86606_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign55920_e86620, assign55920_e86620_d_n0, assign55920_e86620_d_n2, assign55920_e86620_d_n4, assign55920_e86620_d_n5, assign55920_e86620_d_n6, assign55920_e86620_d_n7, assign55920_e86620_d_n8, assign55920_e86620_d_n9, assign55920_e86620_d_n10, assign55920_e86620_d_n11, assign55920_e86620_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) {
        let assign55920_e86618: f64 = (locals.var_t4).sqrt();
        (assign55920_e86618, (locals.var_t4_dn0 / (2.0 * assign55920_e86618)), (locals.var_t4_dn2 / (2.0 * assign55920_e86618)), (locals.var_t4_dn4 / (2.0 * assign55920_e86618)), (locals.var_t4_dn5 / (2.0 * assign55920_e86618)), (locals.var_t4_dn6 / (2.0 * assign55920_e86618)), (locals.var_t4_dn7 / (2.0 * assign55920_e86618)), (locals.var_t4_dn8 / (2.0 * assign55920_e86618)), (locals.var_t4_dn9 / (2.0 * assign55920_e86618)), (locals.var_t4_dn10 / (2.0 * assign55920_e86618)), (locals.var_t4_dn11 / (2.0 * assign55920_e86618)), (locals.var_t4_dn14 / (2.0 * assign55920_e86618)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign55920_e86620;
        locals.var_t3_dn0 = assign55920_e86620_d_n0;
        locals.var_t3_dn2 = assign55920_e86620_d_n2;
        locals.var_t3_dn4 = assign55920_e86620_d_n4;
        locals.var_t3_dn5 = assign55920_e86620_d_n5;
        locals.var_t3_dn6 = assign55920_e86620_d_n6;
        locals.var_t3_dn7 = assign55920_e86620_d_n7;
        locals.var_t3_dn8 = assign55920_e86620_d_n8;
        locals.var_t3_dn9 = assign55920_e86620_d_n9;
        locals.var_t3_dn10 = assign55920_e86620_d_n10;
        locals.var_t3_dn11 = assign55920_e86620_d_n11;
        locals.var_t3_dn14 = assign55920_e86620_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign55930_e86639, assign55930_e86639_d_n0, assign55930_e86639_d_n2, assign55930_e86639_d_n4, assign55930_e86639_d_n5, assign55930_e86639_d_n6, assign55930_e86639_d_n7, assign55930_e86639_d_n8, assign55930_e86639_d_n9, assign55930_e86639_d_n10, assign55930_e86639_d_n11, assign55930_e86639_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) {
        let assign55930_e86635: f64 = (1.0 - locals.var_t3);
        let assign55930_e86636: f64 = (locals.var_q_ndepm_esi_cox_inv2__blk1138 * assign55930_e86635);
        let assign55930_e86637: f64 = (locals.var_vgp + assign55930_e86636);
        (assign55930_e86637, (locals.var_vgp_dn0 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn0 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn2 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn4 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn5 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn6 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn7 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn8 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn9 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn10 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn11 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn11 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn11)))), (locals.var_vgp_dn14 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn14 * assign55930_e86635) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn14)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign55930_e86639;
        locals.var_t10_dn0 = assign55930_e86639_d_n0;
        locals.var_t10_dn2 = assign55930_e86639_d_n2;
        locals.var_t10_dn4 = assign55930_e86639_d_n4;
        locals.var_t10_dn5 = assign55930_e86639_d_n5;
        locals.var_t10_dn6 = assign55930_e86639_d_n6;
        locals.var_t10_dn7 = assign55930_e86639_d_n7;
        locals.var_t10_dn8 = assign55930_e86639_d_n8;
        locals.var_t10_dn9 = assign55930_e86639_d_n9;
        locals.var_t10_dn10 = assign55930_e86639_d_n10;
        locals.var_t10_dn11 = assign55930_e86639_d_n11;
        locals.var_t10_dn14 = assign55930_e86639_d_n14;
        locals.var_t10_rv = 0.0;

        let assign55940_e86643: f64 = (locals.var_uc_depleak + p.p405);
        let assign55940_e86648: f64 = if ((locals.var_t10 < assign55940_e86643) && (p.p405 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1406 = assign55940_e86648;
        locals.var_guard1406_rv = 0.0;

        let (assign55950_e86667, assign55950_e86667_d_n0, assign55950_e86667_d_n2, assign55950_e86667_d_n4, assign55950_e86667_d_n5, assign55950_e86667_d_n6, assign55950_e86667_d_n7, assign55950_e86667_d_n8, assign55950_e86667_d_n9, assign55950_e86667_d_n10, assign55950_e86667_d_n11, assign55950_e86667_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign55950_e86663: f64 = (locals.var_uc_depleak + p.p405);
        let assign55950_e86665: f64 = (assign55950_e86663 - locals.var_t10);
        (assign55950_e86665, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn11 - locals.var_t10_dn11), (locals.var_uc_depleak_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign55950_e86667;
        locals.var_tmf1_dn0 = assign55950_e86667_d_n0;
        locals.var_tmf1_dn2 = assign55950_e86667_d_n2;
        locals.var_tmf1_dn4 = assign55950_e86667_d_n4;
        locals.var_tmf1_dn5 = assign55950_e86667_d_n5;
        locals.var_tmf1_dn6 = assign55950_e86667_d_n6;
        locals.var_tmf1_dn7 = assign55950_e86667_d_n7;
        locals.var_tmf1_dn8 = assign55950_e86667_d_n8;
        locals.var_tmf1_dn9 = assign55950_e86667_d_n9;
        locals.var_tmf1_dn10 = assign55950_e86667_d_n10;
        locals.var_tmf1_dn11 = assign55950_e86667_d_n11;
        locals.var_tmf1_dn14 = assign55950_e86667_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign55960_e86684, assign55960_e86684_d_n0, assign55960_e86684_d_n2, assign55960_e86684_d_n4, assign55960_e86684_d_n5, assign55960_e86684_d_n6, assign55960_e86684_d_n7, assign55960_e86684_d_n8, assign55960_e86684_d_n9, assign55960_e86684_d_n10, assign55960_e86684_d_n11, assign55960_e86684_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign55960_e86682: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55960_e86682, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign55960_e86684;
        locals.var_x2_dn0 = assign55960_e86684_d_n0;
        locals.var_x2_dn2 = assign55960_e86684_d_n2;
        locals.var_x2_dn4 = assign55960_e86684_d_n4;
        locals.var_x2_dn5 = assign55960_e86684_d_n5;
        locals.var_x2_dn6 = assign55960_e86684_d_n6;
        locals.var_x2_dn7 = assign55960_e86684_d_n7;
        locals.var_x2_dn8 = assign55960_e86684_d_n8;
        locals.var_x2_dn9 = assign55960_e86684_d_n9;
        locals.var_x2_dn10 = assign55960_e86684_d_n10;
        locals.var_x2_dn11 = assign55960_e86684_d_n11;
        locals.var_x2_dn14 = assign55960_e86684_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign55970_e86701, assign55970_e86701_d_n0, assign55970_e86701_d_n2, assign55970_e86701_d_n4, assign55970_e86701_d_n5, assign55970_e86701_d_n6, assign55970_e86701_d_n7, assign55970_e86701_d_n8, assign55970_e86701_d_n9, assign55970_e86701_d_n10, assign55970_e86701_d_n11, assign55970_e86701_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign55970_e86699: f64 = (p.p405 * p.p405);
        (assign55970_e86699, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign55970_e86701;
        locals.var_xmax2_dn0 = assign55970_e86701_d_n0;
        locals.var_xmax2_dn2 = assign55970_e86701_d_n2;
        locals.var_xmax2_dn4 = assign55970_e86701_d_n4;
        locals.var_xmax2_dn5 = assign55970_e86701_d_n5;
        locals.var_xmax2_dn6 = assign55970_e86701_d_n6;
        locals.var_xmax2_dn7 = assign55970_e86701_d_n7;
        locals.var_xmax2_dn8 = assign55970_e86701_d_n8;
        locals.var_xmax2_dn9 = assign55970_e86701_d_n9;
        locals.var_xmax2_dn10 = assign55970_e86701_d_n10;
        locals.var_xmax2_dn11 = assign55970_e86701_d_n11;
        locals.var_xmax2_dn14 = assign55970_e86701_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign55980_e86716, assign55980_e86716_d_n0, assign55980_e86716_d_n2, assign55980_e86716_d_n4, assign55980_e86716_d_n5, assign55980_e86716_d_n6, assign55980_e86716_d_n7, assign55980_e86716_d_n8, assign55980_e86716_d_n9, assign55980_e86716_d_n10, assign55980_e86716_d_n11, assign55980_e86716_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55980_e86716;
        locals.var_xp_dn0 = assign55980_e86716_d_n0;
        locals.var_xp_dn2 = assign55980_e86716_d_n2;
        locals.var_xp_dn4 = assign55980_e86716_d_n4;
        locals.var_xp_dn5 = assign55980_e86716_d_n5;
        locals.var_xp_dn6 = assign55980_e86716_d_n6;
        locals.var_xp_dn7 = assign55980_e86716_d_n7;
        locals.var_xp_dn8 = assign55980_e86716_d_n8;
        locals.var_xp_dn9 = assign55980_e86716_d_n9;
        locals.var_xp_dn10 = assign55980_e86716_d_n10;
        locals.var_xp_dn11 = assign55980_e86716_d_n11;
        locals.var_xp_dn14 = assign55980_e86716_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55990_e86731, assign55990_e86731_d_n0, assign55990_e86731_d_n2, assign55990_e86731_d_n4, assign55990_e86731_d_n5, assign55990_e86731_d_n6, assign55990_e86731_d_n7, assign55990_e86731_d_n8, assign55990_e86731_d_n9, assign55990_e86731_d_n10, assign55990_e86731_d_n11, assign55990_e86731_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55990_e86731;
        locals.var_xmp_dn0 = assign55990_e86731_d_n0;
        locals.var_xmp_dn2 = assign55990_e86731_d_n2;
        locals.var_xmp_dn4 = assign55990_e86731_d_n4;
        locals.var_xmp_dn5 = assign55990_e86731_d_n5;
        locals.var_xmp_dn6 = assign55990_e86731_d_n6;
        locals.var_xmp_dn7 = assign55990_e86731_d_n7;
        locals.var_xmp_dn8 = assign55990_e86731_d_n8;
        locals.var_xmp_dn9 = assign55990_e86731_d_n9;
        locals.var_xmp_dn10 = assign55990_e86731_d_n10;
        locals.var_xmp_dn11 = assign55990_e86731_d_n11;
        locals.var_xmp_dn14 = assign55990_e86731_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56000_e86746,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56000_e86746;
        locals.var_m0_rv = 0.0;

        let (assign56010_e86761,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56010_e86761;
        locals.var_mm_rv = 0.0;

        let (assign56020_e86776, assign56020_e86776_d_n0, assign56020_e86776_d_n2, assign56020_e86776_d_n4, assign56020_e86776_d_n5, assign56020_e86776_d_n6, assign56020_e86776_d_n7, assign56020_e86776_d_n8, assign56020_e86776_d_n9, assign56020_e86776_d_n10, assign56020_e86776_d_n11, assign56020_e86776_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign56020_e86776;
        locals.var_arg_dn0 = assign56020_e86776_d_n0;
        locals.var_arg_dn2 = assign56020_e86776_d_n2;
        locals.var_arg_dn4 = assign56020_e86776_d_n4;
        locals.var_arg_dn5 = assign56020_e86776_d_n5;
        locals.var_arg_dn6 = assign56020_e86776_d_n6;
        locals.var_arg_dn7 = assign56020_e86776_d_n7;
        locals.var_arg_dn8 = assign56020_e86776_d_n8;
        locals.var_arg_dn9 = assign56020_e86776_d_n9;
        locals.var_arg_dn10 = assign56020_e86776_d_n10;
        locals.var_arg_dn11 = assign56020_e86776_d_n11;
        locals.var_arg_dn14 = assign56020_e86776_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign56030_e86791, assign56030_e86791_d_n0, assign56030_e86791_d_n2, assign56030_e86791_d_n4, assign56030_e86791_d_n5, assign56030_e86791_d_n6, assign56030_e86791_d_n7, assign56030_e86791_d_n8, assign56030_e86791_d_n9, assign56030_e86791_d_n10, assign56030_e86791_d_n11, assign56030_e86791_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56030_e86791;
        locals.var_dnm_dn0 = assign56030_e86791_d_n0;
        locals.var_dnm_dn2 = assign56030_e86791_d_n2;
        locals.var_dnm_dn4 = assign56030_e86791_d_n4;
        locals.var_dnm_dn5 = assign56030_e86791_d_n5;
        locals.var_dnm_dn6 = assign56030_e86791_d_n6;
        locals.var_dnm_dn7 = assign56030_e86791_d_n7;
        locals.var_dnm_dn8 = assign56030_e86791_d_n8;
        locals.var_dnm_dn9 = assign56030_e86791_d_n9;
        locals.var_dnm_dn10 = assign56030_e86791_d_n10;
        locals.var_dnm_dn11 = assign56030_e86791_d_n11;
        locals.var_dnm_dn14 = assign56030_e86791_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56040_e86808, assign56040_e86808_d_n0, assign56040_e86808_d_n2, assign56040_e86808_d_n4, assign56040_e86808_d_n5, assign56040_e86808_d_n6, assign56040_e86808_d_n7, assign56040_e86808_d_n8, assign56040_e86808_d_n9, assign56040_e86808_d_n10, assign56040_e86808_d_n11, assign56040_e86808_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56040_e86806: f64 = (locals.var_xp * locals.var_x2);
        (assign56040_e86806, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56040_e86808;
        locals.var_xp_dn0 = assign56040_e86808_d_n0;
        locals.var_xp_dn2 = assign56040_e86808_d_n2;
        locals.var_xp_dn4 = assign56040_e86808_d_n4;
        locals.var_xp_dn5 = assign56040_e86808_d_n5;
        locals.var_xp_dn6 = assign56040_e86808_d_n6;
        locals.var_xp_dn7 = assign56040_e86808_d_n7;
        locals.var_xp_dn8 = assign56040_e86808_d_n8;
        locals.var_xp_dn9 = assign56040_e86808_d_n9;
        locals.var_xp_dn10 = assign56040_e86808_d_n10;
        locals.var_xp_dn11 = assign56040_e86808_d_n11;
        locals.var_xp_dn14 = assign56040_e86808_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56050_e86825, assign56050_e86825_d_n0, assign56050_e86825_d_n2, assign56050_e86825_d_n4, assign56050_e86825_d_n5, assign56050_e86825_d_n6, assign56050_e86825_d_n7, assign56050_e86825_d_n8, assign56050_e86825_d_n9, assign56050_e86825_d_n10, assign56050_e86825_d_n11, assign56050_e86825_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56050_e86823: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56050_e86823, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56050_e86825;
        locals.var_xmp_dn0 = assign56050_e86825_d_n0;
        locals.var_xmp_dn2 = assign56050_e86825_d_n2;
        locals.var_xmp_dn4 = assign56050_e86825_d_n4;
        locals.var_xmp_dn5 = assign56050_e86825_d_n5;
        locals.var_xmp_dn6 = assign56050_e86825_d_n6;
        locals.var_xmp_dn7 = assign56050_e86825_d_n7;
        locals.var_xmp_dn8 = assign56050_e86825_d_n8;
        locals.var_xmp_dn9 = assign56050_e86825_d_n9;
        locals.var_xmp_dn10 = assign56050_e86825_d_n10;
        locals.var_xmp_dn11 = assign56050_e86825_d_n11;
        locals.var_xmp_dn14 = assign56050_e86825_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56060_e86842, assign56060_e86842_d_n0, assign56060_e86842_d_n2, assign56060_e86842_d_n4, assign56060_e86842_d_n5, assign56060_e86842_d_n6, assign56060_e86842_d_n7, assign56060_e86842_d_n8, assign56060_e86842_d_n9, assign56060_e86842_d_n10, assign56060_e86842_d_n11, assign56060_e86842_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56060_e86840: f64 = (locals.var_xp * locals.var_x2);
        (assign56060_e86840, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56060_e86842;
        locals.var_xp_dn0 = assign56060_e86842_d_n0;
        locals.var_xp_dn2 = assign56060_e86842_d_n2;
        locals.var_xp_dn4 = assign56060_e86842_d_n4;
        locals.var_xp_dn5 = assign56060_e86842_d_n5;
        locals.var_xp_dn6 = assign56060_e86842_d_n6;
        locals.var_xp_dn7 = assign56060_e86842_d_n7;
        locals.var_xp_dn8 = assign56060_e86842_d_n8;
        locals.var_xp_dn9 = assign56060_e86842_d_n9;
        locals.var_xp_dn10 = assign56060_e86842_d_n10;
        locals.var_xp_dn11 = assign56060_e86842_d_n11;
        locals.var_xp_dn14 = assign56060_e86842_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56070_e86859, assign56070_e86859_d_n0, assign56070_e86859_d_n2, assign56070_e86859_d_n4, assign56070_e86859_d_n5, assign56070_e86859_d_n6, assign56070_e86859_d_n7, assign56070_e86859_d_n8, assign56070_e86859_d_n9, assign56070_e86859_d_n10, assign56070_e86859_d_n11, assign56070_e86859_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56070_e86857: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56070_e86857, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56070_e86859;
        locals.var_xmp_dn0 = assign56070_e86859_d_n0;
        locals.var_xmp_dn2 = assign56070_e86859_d_n2;
        locals.var_xmp_dn4 = assign56070_e86859_d_n4;
        locals.var_xmp_dn5 = assign56070_e86859_d_n5;
        locals.var_xmp_dn6 = assign56070_e86859_d_n6;
        locals.var_xmp_dn7 = assign56070_e86859_d_n7;
        locals.var_xmp_dn8 = assign56070_e86859_d_n8;
        locals.var_xmp_dn9 = assign56070_e86859_d_n9;
        locals.var_xmp_dn10 = assign56070_e86859_d_n10;
        locals.var_xmp_dn11 = assign56070_e86859_d_n11;
        locals.var_xmp_dn14 = assign56070_e86859_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_204(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56080_e86876, assign56080_e86876_d_n0, assign56080_e86876_d_n2, assign56080_e86876_d_n4, assign56080_e86876_d_n5, assign56080_e86876_d_n6, assign56080_e86876_d_n7, assign56080_e86876_d_n8, assign56080_e86876_d_n9, assign56080_e86876_d_n10, assign56080_e86876_d_n11, assign56080_e86876_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56080_e86874: f64 = (locals.var_xp + locals.var_xmp);
        (assign56080_e86874, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign56080_e86876;
        locals.var_arg_dn0 = assign56080_e86876_d_n0;
        locals.var_arg_dn2 = assign56080_e86876_d_n2;
        locals.var_arg_dn4 = assign56080_e86876_d_n4;
        locals.var_arg_dn5 = assign56080_e86876_d_n5;
        locals.var_arg_dn6 = assign56080_e86876_d_n6;
        locals.var_arg_dn7 = assign56080_e86876_d_n7;
        locals.var_arg_dn8 = assign56080_e86876_d_n8;
        locals.var_arg_dn9 = assign56080_e86876_d_n9;
        locals.var_arg_dn10 = assign56080_e86876_d_n10;
        locals.var_arg_dn11 = assign56080_e86876_d_n11;
        locals.var_arg_dn14 = assign56080_e86876_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign56090_e86891, assign56090_e86891_d_n0, assign56090_e86891_d_n2, assign56090_e86891_d_n4, assign56090_e86891_d_n5, assign56090_e86891_d_n6, assign56090_e86891_d_n7, assign56090_e86891_d_n8, assign56090_e86891_d_n9, assign56090_e86891_d_n10, assign56090_e86891_d_n11, assign56090_e86891_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56090_e86891;
        locals.var_dnm_dn0 = assign56090_e86891_d_n0;
        locals.var_dnm_dn2 = assign56090_e86891_d_n2;
        locals.var_dnm_dn4 = assign56090_e86891_d_n4;
        locals.var_dnm_dn5 = assign56090_e86891_d_n5;
        locals.var_dnm_dn6 = assign56090_e86891_d_n6;
        locals.var_dnm_dn7 = assign56090_e86891_d_n7;
        locals.var_dnm_dn8 = assign56090_e86891_d_n8;
        locals.var_dnm_dn9 = assign56090_e86891_d_n9;
        locals.var_dnm_dn10 = assign56090_e86891_d_n10;
        locals.var_dnm_dn11 = assign56090_e86891_d_n11;
        locals.var_dnm_dn14 = assign56090_e86891_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign56100_e86906: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1407 = assign56100_e86906;
        locals.var_guard1407_rv = 0.0;

        let assign56110_e86909: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1408 = assign56110_e86909;
        locals.var_guard1408_rv = 0.0;

        let (assign56120_e86928,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 != 0.0)) && (locals.var_guard1408 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56120_e86928;
        locals.var_mm_rv = 0.0;

        let assign56130_e86931: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1409 = assign56130_e86931;
        locals.var_guard1409_rv = 0.0;

        let (assign56140_e86953,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 != 0.0)) && (locals.var_guard1408 == 0.0)) && (locals.var_guard1409 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56140_e86953;
        locals.var_mm_rv = 0.0;

        let assign56150_e86956: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1410 = assign56150_e86956;
        locals.var_guard1410_rv = 0.0;

        let (assign56160_e86981,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 != 0.0)) && (locals.var_guard1408 == 0.0)) && (locals.var_guard1409 == 0.0)) && (locals.var_guard1410 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56160_e86981;
        locals.var_mm_rv = 0.0;

        let assign56170_e86984: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1411 = assign56170_e86984;
        locals.var_guard1411_rv = 0.0;

        let (assign56180_e87012,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 != 0.0)) && (locals.var_guard1408 == 0.0)) && (locals.var_guard1409 == 0.0)) && (locals.var_guard1410 == 0.0)) && (locals.var_guard1411 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56180_e87012;
        locals.var_mm_rv = 0.0;

        let (assign56190_e87029,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56190_e87029;
        locals.var_m0_rv = 0.0;

        let mut assign56200_loop_guard: usize = 0;
        while {
            let assign56200_cond_e87047: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign56200_cond_e87047 != 0.0
        } {
            assign56200_loop_guard += 1;
            assert!(assign56200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign56200_body0_e87065, assign56200_body0_e87065_d_n0, assign56200_body0_e87065_d_n2, assign56200_body0_e87065_d_n4, assign56200_body0_e87065_d_n5, assign56200_body0_e87065_d_n6, assign56200_body0_e87065_d_n7, assign56200_body0_e87065_d_n8, assign56200_body0_e87065_d_n9, assign56200_body0_e87065_d_n10, assign56200_body0_e87065_d_n11, assign56200_body0_e87065_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 != 0.0)) {
        let assign56200_body0_e87063: f64 = (locals.var_dnm).sqrt();
        (assign56200_body0_e87063, (locals.var_dnm_dn0 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn2 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn4 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn5 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn6 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn7 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn8 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn9 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn10 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn11 / (2.0 * assign56200_body0_e87063)), (locals.var_dnm_dn14 / (2.0 * assign56200_body0_e87063)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign56200_body0_e87065;
            locals.var_dnm_dn0 = assign56200_body0_e87065_d_n0;
            locals.var_dnm_dn2 = assign56200_body0_e87065_d_n2;
            locals.var_dnm_dn4 = assign56200_body0_e87065_d_n4;
            locals.var_dnm_dn5 = assign56200_body0_e87065_d_n5;
            locals.var_dnm_dn6 = assign56200_body0_e87065_d_n6;
            locals.var_dnm_dn7 = assign56200_body0_e87065_d_n7;
            locals.var_dnm_dn8 = assign56200_body0_e87065_d_n8;
            locals.var_dnm_dn9 = assign56200_body0_e87065_d_n9;
            locals.var_dnm_dn10 = assign56200_body0_e87065_d_n10;
            locals.var_dnm_dn11 = assign56200_body0_e87065_d_n11;
            locals.var_dnm_dn14 = assign56200_body0_e87065_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign56200_body1_e87084,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 != 0.0)) {
        let assign56200_body1_e87082: f64 = (locals.var_m0 + 1.0);
        (assign56200_body1_e87082,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign56200_body1_e87084;
            locals.var_m0_rv = 0.0;
        }

        let (assign56210_e87113, assign56210_e87113_d_n0, assign56210_e87113_d_n2, assign56210_e87113_d_n4, assign56210_e87113_d_n5, assign56210_e87113_d_n6, assign56210_e87113_d_n7, assign56210_e87113_d_n8, assign56210_e87113_d_n9, assign56210_e87113_d_n10, assign56210_e87113_d_n11, assign56210_e87113_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) && (locals.var_guard1407 == 0.0)) {
        let (assign56210_e87111, assign56210_e87111_d_n0, assign56210_e87111_d_n2, assign56210_e87111_d_n4, assign56210_e87111_d_n5, assign56210_e87111_d_n6, assign56210_e87111_d_n7, assign56210_e87111_d_n8, assign56210_e87111_d_n9, assign56210_e87111_d_n10, assign56210_e87111_d_n11, assign56210_e87111_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56210_e87108: f64 = (2.0 * 2.0);
                let assign56210_e87109: f64 = (1.0 / assign56210_e87108);
                let assign56210_e87110: f64 = (locals.var_dnm).powf(assign56210_e87109);
                (assign56210_e87110, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn0)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn2)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn4)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn5)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn6)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn7)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn8)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn9)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn10)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn11)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56210_e87109) as f64).is_finite() && ((assign56210_e87109) as f64).fract() == 0.0 { if assign56210_e87109 == 0.0 { 0.0 } else { (assign56210_e87109 * ((locals.var_dnm).powf(assign56210_e87109 - 1.0) * locals.var_dnm_dn14)) } } else { (assign56210_e87110 * (assign56210_e87109 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign56210_e87111, assign56210_e87111_d_n0, assign56210_e87111_d_n2, assign56210_e87111_d_n4, assign56210_e87111_d_n5, assign56210_e87111_d_n6, assign56210_e87111_d_n7, assign56210_e87111_d_n8, assign56210_e87111_d_n9, assign56210_e87111_d_n10, assign56210_e87111_d_n11, assign56210_e87111_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56210_e87113;
        locals.var_dnm_dn0 = assign56210_e87113_d_n0;
        locals.var_dnm_dn2 = assign56210_e87113_d_n2;
        locals.var_dnm_dn4 = assign56210_e87113_d_n4;
        locals.var_dnm_dn5 = assign56210_e87113_d_n5;
        locals.var_dnm_dn6 = assign56210_e87113_d_n6;
        locals.var_dnm_dn7 = assign56210_e87113_d_n7;
        locals.var_dnm_dn8 = assign56210_e87113_d_n8;
        locals.var_dnm_dn9 = assign56210_e87113_d_n9;
        locals.var_dnm_dn10 = assign56210_e87113_d_n10;
        locals.var_dnm_dn11 = assign56210_e87113_d_n11;
        locals.var_dnm_dn14 = assign56210_e87113_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56220_e87130, assign56220_e87130_d_n0, assign56220_e87130_d_n2, assign56220_e87130_d_n4, assign56220_e87130_d_n5, assign56220_e87130_d_n6, assign56220_e87130_d_n7, assign56220_e87130_d_n8, assign56220_e87130_d_n9, assign56220_e87130_d_n10, assign56220_e87130_d_n11, assign56220_e87130_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56220_e87128: f64 = (1.0 / locals.var_dnm);
        (assign56220_e87128, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56220_e87130;
        locals.var_dnm_dn0 = assign56220_e87130_d_n0;
        locals.var_dnm_dn2 = assign56220_e87130_d_n2;
        locals.var_dnm_dn4 = assign56220_e87130_d_n4;
        locals.var_dnm_dn5 = assign56220_e87130_d_n5;
        locals.var_dnm_dn6 = assign56220_e87130_d_n6;
        locals.var_dnm_dn7 = assign56220_e87130_d_n7;
        locals.var_dnm_dn8 = assign56220_e87130_d_n8;
        locals.var_dnm_dn9 = assign56220_e87130_d_n9;
        locals.var_dnm_dn10 = assign56220_e87130_d_n10;
        locals.var_dnm_dn11 = assign56220_e87130_d_n11;
        locals.var_dnm_dn14 = assign56220_e87130_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56230_e87149, assign56230_e87149_d_n0, assign56230_e87149_d_n2, assign56230_e87149_d_n4, assign56230_e87149_d_n5, assign56230_e87149_d_n6, assign56230_e87149_d_n7, assign56230_e87149_d_n8, assign56230_e87149_d_n9, assign56230_e87149_d_n10, assign56230_e87149_d_n11, assign56230_e87149_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56230_e87145: f64 = (locals.var_tmf1 * p.p405);
        let assign56230_e87147: f64 = (assign56230_e87145 * locals.var_dnm);
        (assign56230_e87147, (((locals.var_tmf1_dn0 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * p.p405) * locals.var_dnm) + (assign56230_e87145 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign56230_e87149;
        locals.var_tmf0_dn0 = assign56230_e87149_d_n0;
        locals.var_tmf0_dn2 = assign56230_e87149_d_n2;
        locals.var_tmf0_dn4 = assign56230_e87149_d_n4;
        locals.var_tmf0_dn5 = assign56230_e87149_d_n5;
        locals.var_tmf0_dn6 = assign56230_e87149_d_n6;
        locals.var_tmf0_dn7 = assign56230_e87149_d_n7;
        locals.var_tmf0_dn8 = assign56230_e87149_d_n8;
        locals.var_tmf0_dn9 = assign56230_e87149_d_n9;
        locals.var_tmf0_dn10 = assign56230_e87149_d_n10;
        locals.var_tmf0_dn11 = assign56230_e87149_d_n11;
        locals.var_tmf0_dn14 = assign56230_e87149_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign56240_e87170, assign56240_e87170_d_n0, assign56240_e87170_d_n2, assign56240_e87170_d_n4, assign56240_e87170_d_n5, assign56240_e87170_d_n6, assign56240_e87170_d_n7, assign56240_e87170_d_n8, assign56240_e87170_d_n9, assign56240_e87170_d_n10, assign56240_e87170_d_n11, assign56240_e87170_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56240_e87164: f64 = (p.p405 * locals.var_xmp);
        let assign56240_e87166: f64 = (assign56240_e87164 * locals.var_dnm);
        let assign56240_e87168: f64 = (assign56240_e87166 / locals.var_arg);
        (assign56240_e87168, ((((((p.p405 * locals.var_xmp_dn0) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn0)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn2) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn2)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn4) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn4)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn5) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn5)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn6) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn6)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn7) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn7)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn8) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn8)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn9) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn9)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn10) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn10)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn11) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn11)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn14) * locals.var_dnm) + (assign56240_e87164 * locals.var_dnm_dn14)) * locals.var_arg) - (assign56240_e87166 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56240_e87170;
        locals.var_t0_dn0 = assign56240_e87170_d_n0;
        locals.var_t0_dn2 = assign56240_e87170_d_n2;
        locals.var_t0_dn4 = assign56240_e87170_d_n4;
        locals.var_t0_dn5 = assign56240_e87170_d_n5;
        locals.var_t0_dn6 = assign56240_e87170_d_n6;
        locals.var_t0_dn7 = assign56240_e87170_d_n7;
        locals.var_t0_dn8 = assign56240_e87170_d_n8;
        locals.var_t0_dn9 = assign56240_e87170_d_n9;
        locals.var_t0_dn10 = assign56240_e87170_d_n10;
        locals.var_t0_dn11 = assign56240_e87170_d_n11;
        locals.var_t0_dn14 = assign56240_e87170_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56250_e87189, assign56250_e87189_d_n0, assign56250_e87189_d_n2, assign56250_e87189_d_n4, assign56250_e87189_d_n5, assign56250_e87189_d_n6, assign56250_e87189_d_n7, assign56250_e87189_d_n8, assign56250_e87189_d_n9, assign56250_e87189_d_n10, assign56250_e87189_d_n11, assign56250_e87189_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        let assign56250_e87185: f64 = (locals.var_uc_depleak + p.p405);
        let assign56250_e87187: f64 = (assign56250_e87185 - locals.var_tmf0);
        (assign56250_e87187, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn11 - locals.var_tmf0_dn11), (locals.var_uc_depleak_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56250_e87189;
        locals.var_vdssat_res_dn0 = assign56250_e87189_d_n0;
        locals.var_vdssat_res_dn2 = assign56250_e87189_d_n2;
        locals.var_vdssat_res_dn4 = assign56250_e87189_d_n4;
        locals.var_vdssat_res_dn5 = assign56250_e87189_d_n5;
        locals.var_vdssat_res_dn6 = assign56250_e87189_d_n6;
        locals.var_vdssat_res_dn7 = assign56250_e87189_d_n7;
        locals.var_vdssat_res_dn8 = assign56250_e87189_d_n8;
        locals.var_vdssat_res_dn9 = assign56250_e87189_d_n9;
        locals.var_vdssat_res_dn10 = assign56250_e87189_d_n10;
        locals.var_vdssat_res_dn11 = assign56250_e87189_d_n11;
        locals.var_vdssat_res_dn14 = assign56250_e87189_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let (assign56260_e87204, assign56260_e87204_d_n0, assign56260_e87204_d_n2, assign56260_e87204_d_n4, assign56260_e87204_d_n5, assign56260_e87204_d_n6, assign56260_e87204_d_n7, assign56260_e87204_d_n8, assign56260_e87204_d_n9, assign56260_e87204_d_n10, assign56260_e87204_d_n11, assign56260_e87204_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56260_e87204;
        locals.var_t0_dn0 = assign56260_e87204_d_n0;
        locals.var_t0_dn2 = assign56260_e87204_d_n2;
        locals.var_t0_dn4 = assign56260_e87204_d_n4;
        locals.var_t0_dn5 = assign56260_e87204_d_n5;
        locals.var_t0_dn6 = assign56260_e87204_d_n6;
        locals.var_t0_dn7 = assign56260_e87204_d_n7;
        locals.var_t0_dn8 = assign56260_e87204_d_n8;
        locals.var_t0_dn9 = assign56260_e87204_d_n9;
        locals.var_t0_dn10 = assign56260_e87204_d_n10;
        locals.var_t0_dn11 = assign56260_e87204_d_n11;
        locals.var_t0_dn14 = assign56260_e87204_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56270_e87220, assign56270_e87220_d_n0, assign56270_e87220_d_n2, assign56270_e87220_d_n4, assign56270_e87220_d_n5, assign56270_e87220_d_n6, assign56270_e87220_d_n7, assign56270_e87220_d_n8, assign56270_e87220_d_n9, assign56270_e87220_d_n10, assign56270_e87220_d_n11, assign56270_e87220_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56270_e87220;
        locals.var_vdssat_res_dn0 = assign56270_e87220_d_n0;
        locals.var_vdssat_res_dn2 = assign56270_e87220_d_n2;
        locals.var_vdssat_res_dn4 = assign56270_e87220_d_n4;
        locals.var_vdssat_res_dn5 = assign56270_e87220_d_n5;
        locals.var_vdssat_res_dn6 = assign56270_e87220_d_n6;
        locals.var_vdssat_res_dn7 = assign56270_e87220_d_n7;
        locals.var_vdssat_res_dn8 = assign56270_e87220_d_n8;
        locals.var_vdssat_res_dn9 = assign56270_e87220_d_n9;
        locals.var_vdssat_res_dn10 = assign56270_e87220_d_n10;
        locals.var_vdssat_res_dn11 = assign56270_e87220_d_n11;
        locals.var_vdssat_res_dn14 = assign56270_e87220_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let (assign56280_e87236, assign56280_e87236_d_n0, assign56280_e87236_d_n2, assign56280_e87236_d_n4, assign56280_e87236_d_n5, assign56280_e87236_d_n6, assign56280_e87236_d_n7, assign56280_e87236_d_n8, assign56280_e87236_d_n9, assign56280_e87236_d_n10, assign56280_e87236_d_n11, assign56280_e87236_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1406 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56280_e87236;
        locals.var_t0_dn0 = assign56280_e87236_d_n0;
        locals.var_t0_dn2 = assign56280_e87236_d_n2;
        locals.var_t0_dn4 = assign56280_e87236_d_n4;
        locals.var_t0_dn5 = assign56280_e87236_d_n5;
        locals.var_t0_dn6 = assign56280_e87236_d_n6;
        locals.var_t0_dn7 = assign56280_e87236_d_n7;
        locals.var_t0_dn8 = assign56280_e87236_d_n8;
        locals.var_t0_dn9 = assign56280_e87236_d_n9;
        locals.var_t0_dn10 = assign56280_e87236_d_n10;
        locals.var_t0_dn11 = assign56280_e87236_d_n11;
        locals.var_t0_dn14 = assign56280_e87236_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56290_e87250, assign56290_e87250_d_n0, assign56290_e87250_d_n2, assign56290_e87250_d_n4, assign56290_e87250_d_n5, assign56290_e87250_d_n6, assign56290_e87250_d_n7, assign56290_e87250_d_n8, assign56290_e87250_d_n9, assign56290_e87250_d_n10, assign56290_e87250_d_n11, assign56290_e87250_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        (locals.var_vgp_res__blk1149, locals.var_vgp_res__blk1149_dn0, locals.var_vgp_res__blk1149_dn2, locals.var_vgp_res__blk1149_dn4, locals.var_vgp_res__blk1149_dn5, locals.var_vgp_res__blk1149_dn6, locals.var_vgp_res__blk1149_dn7, locals.var_vgp_res__blk1149_dn8, locals.var_vgp_res__blk1149_dn9, locals.var_vgp_res__blk1149_dn10, locals.var_vgp_res__blk1149_dn11, locals.var_vgp_res__blk1149_dn14,)
    } else {
        (locals.var_vgpsat, locals.var_vgpsat_dn0, locals.var_vgpsat_dn2, locals.var_vgpsat_dn4, locals.var_vgpsat_dn5, locals.var_vgpsat_dn6, locals.var_vgpsat_dn7, locals.var_vgpsat_dn8, locals.var_vgpsat_dn9, locals.var_vgpsat_dn10, locals.var_vgpsat_dn11, locals.var_vgpsat_dn14,)
    }
};
        locals.var_vgpsat = assign56290_e87250;
        locals.var_vgpsat_dn0 = assign56290_e87250_d_n0;
        locals.var_vgpsat_dn2 = assign56290_e87250_d_n2;
        locals.var_vgpsat_dn4 = assign56290_e87250_d_n4;
        locals.var_vgpsat_dn5 = assign56290_e87250_d_n5;
        locals.var_vgpsat_dn6 = assign56290_e87250_d_n6;
        locals.var_vgpsat_dn7 = assign56290_e87250_d_n7;
        locals.var_vgpsat_dn8 = assign56290_e87250_d_n8;
        locals.var_vgpsat_dn9 = assign56290_e87250_d_n9;
        locals.var_vgpsat_dn10 = assign56290_e87250_d_n10;
        locals.var_vgpsat_dn11 = assign56290_e87250_d_n11;
        locals.var_vgpsat_dn14 = assign56290_e87250_d_n14;
        locals.var_vgpsat_rv = 0.0;

        let (assign56300_e87268, assign56300_e87268_d_n0, assign56300_e87268_d_n2, assign56300_e87268_d_n4, assign56300_e87268_d_n5, assign56300_e87268_d_n6, assign56300_e87268_d_n7, assign56300_e87268_d_n8, assign56300_e87268_d_n9, assign56300_e87268_d_n10, assign56300_e87268_d_n11, assign56300_e87268_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56300_e87265: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat);
        let assign56300_e87266: f64 = (1.0 + assign56300_e87265);
        (assign56300_e87266, ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn0 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn0)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn2 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn2)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn4 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn4)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn5 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn5)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn6 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn6)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn7 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn7)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn8 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn8)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn9 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn9)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn10 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn10)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn11 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn11)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn14 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * locals.var_vgpsat_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign56300_e87268;
        locals.var_t4_dn0 = assign56300_e87268_d_n0;
        locals.var_t4_dn2 = assign56300_e87268_d_n2;
        locals.var_t4_dn4 = assign56300_e87268_d_n4;
        locals.var_t4_dn5 = assign56300_e87268_d_n5;
        locals.var_t4_dn6 = assign56300_e87268_d_n6;
        locals.var_t4_dn7 = assign56300_e87268_d_n7;
        locals.var_t4_dn8 = assign56300_e87268_d_n8;
        locals.var_t4_dn9 = assign56300_e87268_d_n9;
        locals.var_t4_dn10 = assign56300_e87268_d_n10;
        locals.var_t4_dn11 = assign56300_e87268_d_n11;
        locals.var_t4_dn14 = assign56300_e87268_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign56310_e87291, assign56310_e87291_d_n0, assign56310_e87291_d_n2, assign56310_e87291_d_n4, assign56310_e87291_d_n5, assign56310_e87291_d_n6, assign56310_e87291_d_n7, assign56310_e87291_d_n8, assign56310_e87291_d_n9, assign56310_e87291_d_n10, assign56310_e87291_d_n11, assign56310_e87291_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let (assign56310_e87289, assign56310_e87289_d_n0, assign56310_e87289_d_n2, assign56310_e87289_d_n4, assign56310_e87289_d_n5, assign56310_e87289_d_n6, assign56310_e87289_d_n7, assign56310_e87289_d_n8, assign56310_e87289_d_n9, assign56310_e87289_d_n10, assign56310_e87289_d_n11, assign56310_e87289_d_n14,) = {
            if (locals.var_t4 > 0.0) {
                let assign56310_e87284: f64 = (locals.var_t4).sqrt();
                (assign56310_e87284, (locals.var_t4_dn0 / (2.0 * assign56310_e87284)), (locals.var_t4_dn2 / (2.0 * assign56310_e87284)), (locals.var_t4_dn4 / (2.0 * assign56310_e87284)), (locals.var_t4_dn5 / (2.0 * assign56310_e87284)), (locals.var_t4_dn6 / (2.0 * assign56310_e87284)), (locals.var_t4_dn7 / (2.0 * assign56310_e87284)), (locals.var_t4_dn8 / (2.0 * assign56310_e87284)), (locals.var_t4_dn9 / (2.0 * assign56310_e87284)), (locals.var_t4_dn10 / (2.0 * assign56310_e87284)), (locals.var_t4_dn11 / (2.0 * assign56310_e87284)), (locals.var_t4_dn14 / (2.0 * assign56310_e87284)),)
            } else {
                let assign56310_e87286: f64 = (-locals.var_t4);
                let assign56310_e87287: f64 = (assign56310_e87286).sqrt();
                let assign56310_e87288: f64 = (-assign56310_e87287);
                (assign56310_e87288, (-((-locals.var_t4_dn0) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn2) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn4) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn5) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn6) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn7) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn8) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn9) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn10) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn11) / (2.0 * assign56310_e87287))), (-((-locals.var_t4_dn14) / (2.0 * assign56310_e87287))),)
            }
        };
        (assign56310_e87289, assign56310_e87289_d_n0, assign56310_e87289_d_n2, assign56310_e87289_d_n4, assign56310_e87289_d_n5, assign56310_e87289_d_n6, assign56310_e87289_d_n7, assign56310_e87289_d_n8, assign56310_e87289_d_n9, assign56310_e87289_d_n10, assign56310_e87289_d_n11, assign56310_e87289_d_n14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign56310_e87291;
        locals.var_t3_dn0 = assign56310_e87291_d_n0;
        locals.var_t3_dn2 = assign56310_e87291_d_n2;
        locals.var_t3_dn4 = assign56310_e87291_d_n4;
        locals.var_t3_dn5 = assign56310_e87291_d_n5;
        locals.var_t3_dn6 = assign56310_e87291_d_n6;
        locals.var_t3_dn7 = assign56310_e87291_d_n7;
        locals.var_t3_dn8 = assign56310_e87291_d_n8;
        locals.var_t3_dn9 = assign56310_e87291_d_n9;
        locals.var_t3_dn10 = assign56310_e87291_d_n10;
        locals.var_t3_dn11 = assign56310_e87291_d_n11;
        locals.var_t3_dn14 = assign56310_e87291_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign56320_e87311, assign56320_e87311_d_n0, assign56320_e87311_d_n2, assign56320_e87311_d_n4, assign56320_e87311_d_n5, assign56320_e87311_d_n6, assign56320_e87311_d_n7, assign56320_e87311_d_n8, assign56320_e87311_d_n9, assign56320_e87311_d_n10, assign56320_e87311_d_n11, assign56320_e87311_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56320_e87307: f64 = (1.0 - locals.var_t3);
        let assign56320_e87308: f64 = (locals.var_q_ndepm_esi_cox_inv2__blk1138 * assign56320_e87307);
        let assign56320_e87309: f64 = (locals.var_vgpsat + assign56320_e87308);
        (assign56320_e87309, (locals.var_vgpsat_dn0 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn0 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn0)))), (locals.var_vgpsat_dn2 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn2 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn2)))), (locals.var_vgpsat_dn4 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn4 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn4)))), (locals.var_vgpsat_dn5 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn5 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn5)))), (locals.var_vgpsat_dn6 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn6 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn6)))), (locals.var_vgpsat_dn7 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn7 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn7)))), (locals.var_vgpsat_dn8 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn8 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn8)))), (locals.var_vgpsat_dn9 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn9 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn9)))), (locals.var_vgpsat_dn10 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn10 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn10)))), (locals.var_vgpsat_dn11 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn11 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn11)))), (locals.var_vgpsat_dn14 + ((locals.var_q_ndepm_esi_cox_inv2__blk1138_dn14 * assign56320_e87307) + (locals.var_q_ndepm_esi_cox_inv2__blk1138 * (-locals.var_t3_dn14)))),)
    } else {
        (locals.var_vdssat_ini, locals.var_vdssat_ini_dn0, locals.var_vdssat_ini_dn2, locals.var_vdssat_ini_dn4, locals.var_vdssat_ini_dn5, locals.var_vdssat_ini_dn6, locals.var_vdssat_ini_dn7, locals.var_vdssat_ini_dn8, locals.var_vdssat_ini_dn9, locals.var_vdssat_ini_dn10, locals.var_vdssat_ini_dn11, locals.var_vdssat_ini_dn14,)
    }
};
        locals.var_vdssat_ini = assign56320_e87311;
        locals.var_vdssat_ini_dn0 = assign56320_e87311_d_n0;
        locals.var_vdssat_ini_dn2 = assign56320_e87311_d_n2;
        locals.var_vdssat_ini_dn4 = assign56320_e87311_d_n4;
        locals.var_vdssat_ini_dn5 = assign56320_e87311_d_n5;
        locals.var_vdssat_ini_dn6 = assign56320_e87311_d_n6;
        locals.var_vdssat_ini_dn7 = assign56320_e87311_d_n7;
        locals.var_vdssat_ini_dn8 = assign56320_e87311_d_n8;
        locals.var_vdssat_ini_dn9 = assign56320_e87311_d_n9;
        locals.var_vdssat_ini_dn10 = assign56320_e87311_d_n10;
        locals.var_vdssat_ini_dn11 = assign56320_e87311_d_n11;
        locals.var_vdssat_ini_dn14 = assign56320_e87311_d_n14;
        locals.var_vdssat_ini_rv = 0.0;

        let (assign56330_e87325, assign56330_e87325_d_n0, assign56330_e87325_d_n2, assign56330_e87325_d_n4, assign56330_e87325_d_n5, assign56330_e87325_d_n6, assign56330_e87325_d_n7, assign56330_e87325_d_n8, assign56330_e87325_d_n9, assign56330_e87325_d_n10, assign56330_e87325_d_n11, assign56330_e87325_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        (locals.var_vdssat_ini, locals.var_vdssat_ini_dn0, locals.var_vdssat_ini_dn2, locals.var_vdssat_ini_dn4, locals.var_vdssat_ini_dn5, locals.var_vdssat_ini_dn6, locals.var_vdssat_ini_dn7, locals.var_vdssat_ini_dn8, locals.var_vdssat_ini_dn9, locals.var_vdssat_ini_dn10, locals.var_vdssat_ini_dn11, locals.var_vdssat_ini_dn14,)
    } else {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn11, locals.var_phi_vsat_dn14,)
    }
};
        locals.var_phi_vsat = assign56330_e87325;
        locals.var_phi_vsat_dn0 = assign56330_e87325_d_n0;
        locals.var_phi_vsat_dn2 = assign56330_e87325_d_n2;
        locals.var_phi_vsat_dn4 = assign56330_e87325_d_n4;
        locals.var_phi_vsat_dn5 = assign56330_e87325_d_n5;
        locals.var_phi_vsat_dn6 = assign56330_e87325_d_n6;
        locals.var_phi_vsat_dn7 = assign56330_e87325_d_n7;
        locals.var_phi_vsat_dn8 = assign56330_e87325_d_n8;
        locals.var_phi_vsat_dn9 = assign56330_e87325_d_n9;
        locals.var_phi_vsat_dn10 = assign56330_e87325_d_n10;
        locals.var_phi_vsat_dn11 = assign56330_e87325_d_n11;
        locals.var_phi_vsat_dn14 = assign56330_e87325_d_n14;
        locals.var_phi_vsat_rv = 0.0;

        let (assign56340_e87339,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign56340_e87339;
        locals.var_flg_conv_rv = 0.0;

        let (assign56350_e87353,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign56350_e87353;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_205(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign56360_loop_guard: usize = 0;
        while {
            let assign56360_cond_e87368: f64 = if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign56360_cond_e87368 != 0.0
        } {
            assign56360_loop_guard += 1;
            assert!(assign56360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign56360_body0_e87385, assign56360_body0_e87385_d_n0, assign56360_body0_e87385_d_n2, assign56360_body0_e87385_d_n4, assign56360_body0_e87385_d_n5, assign56360_body0_e87385_d_n6, assign56360_body0_e87385_d_n7, assign56360_body0_e87385_d_n8, assign56360_body0_e87385_d_n9, assign56360_body0_e87385_d_n10, assign56360_body0_e87385_d_n11, assign56360_body0_e87385_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56360_body0_e87381: f64 = (-locals.var_beta);
        let assign56360_body0_e87383: f64 = (assign56360_body0_e87381 * locals.var_phi_vsat);
        (assign56360_body0_e87383, (((-locals.var_beta_dn0) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn0)), (((-locals.var_beta_dn2) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn2)), (((-locals.var_beta_dn4) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn4)), (((-locals.var_beta_dn5) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn5)), (((-locals.var_beta_dn6) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn6)), (((-locals.var_beta_dn7) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn7)), (((-locals.var_beta_dn8) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn8)), (((-locals.var_beta_dn9) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn9)), (((-locals.var_beta_dn10) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn10)), (((-locals.var_beta_dn11) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn11)), (((-locals.var_beta_dn14) * locals.var_phi_vsat) + (assign56360_body0_e87381 * locals.var_phi_vsat_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign56360_body0_e87385;
            locals.var_t1_dn0 = assign56360_body0_e87385_d_n0;
            locals.var_t1_dn2 = assign56360_body0_e87385_d_n2;
            locals.var_t1_dn4 = assign56360_body0_e87385_d_n4;
            locals.var_t1_dn5 = assign56360_body0_e87385_d_n5;
            locals.var_t1_dn6 = assign56360_body0_e87385_d_n6;
            locals.var_t1_dn7 = assign56360_body0_e87385_d_n7;
            locals.var_t1_dn8 = assign56360_body0_e87385_d_n8;
            locals.var_t1_dn9 = assign56360_body0_e87385_d_n9;
            locals.var_t1_dn10 = assign56360_body0_e87385_d_n10;
            locals.var_t1_dn11 = assign56360_body0_e87385_d_n11;
            locals.var_t1_dn14 = assign56360_body0_e87385_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign56360_body1_e87400, assign56360_body1_e87400_d_n0, assign56360_body1_e87400_d_n2, assign56360_body1_e87400_d_n4, assign56360_body1_e87400_d_n5, assign56360_body1_e87400_d_n6, assign56360_body1_e87400_d_n7, assign56360_body1_e87400_d_n8, assign56360_body1_e87400_d_n9, assign56360_body1_e87400_d_n10, assign56360_body1_e87400_d_n11, assign56360_body1_e87400_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56360_body1_e87398: f64 = (locals.var_t1).exp();
        (assign56360_body1_e87398, (assign56360_body1_e87398 * locals.var_t1_dn0), (assign56360_body1_e87398 * locals.var_t1_dn2), (assign56360_body1_e87398 * locals.var_t1_dn4), (assign56360_body1_e87398 * locals.var_t1_dn5), (assign56360_body1_e87398 * locals.var_t1_dn6), (assign56360_body1_e87398 * locals.var_t1_dn7), (assign56360_body1_e87398 * locals.var_t1_dn8), (assign56360_body1_e87398 * locals.var_t1_dn9), (assign56360_body1_e87398 * locals.var_t1_dn10), (assign56360_body1_e87398 * locals.var_t1_dn11), (assign56360_body1_e87398 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign56360_body1_e87400;
            locals.var_t2_dn0 = assign56360_body1_e87400_d_n0;
            locals.var_t2_dn2 = assign56360_body1_e87400_d_n2;
            locals.var_t2_dn4 = assign56360_body1_e87400_d_n4;
            locals.var_t2_dn5 = assign56360_body1_e87400_d_n5;
            locals.var_t2_dn6 = assign56360_body1_e87400_d_n6;
            locals.var_t2_dn7 = assign56360_body1_e87400_d_n7;
            locals.var_t2_dn8 = assign56360_body1_e87400_d_n8;
            locals.var_t2_dn9 = assign56360_body1_e87400_d_n9;
            locals.var_t2_dn10 = assign56360_body1_e87400_d_n10;
            locals.var_t2_dn11 = assign56360_body1_e87400_d_n11;
            locals.var_t2_dn14 = assign56360_body1_e87400_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign56360_body2_e87419, assign56360_body2_e87419_d_n0, assign56360_body2_e87419_d_n2, assign56360_body2_e87419_d_n4, assign56360_body2_e87419_d_n5, assign56360_body2_e87419_d_n6, assign56360_body2_e87419_d_n7, assign56360_body2_e87419_d_n8, assign56360_body2_e87419_d_n9, assign56360_body2_e87419_d_n10, assign56360_body2_e87419_d_n11, assign56360_body2_e87419_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56360_body2_e87414: f64 = (2.0 * locals.var_q_ndepm_esi__blk1118);
        let assign56360_body2_e87416: f64 = (assign56360_body2_e87414 / locals.var_beta);
        let assign56360_body2_e87417: f64 = (assign56360_body2_e87416).sqrt();
        (assign56360_body2_e87417, (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn0) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn0)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn2) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn2)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn4) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn5) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn5)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn6) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn6)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn7) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn7)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn8) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn8)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn9) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn9)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn10) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn11) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn11)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)), (((((2.0 * locals.var_q_ndepm_esi__blk1118_dn14) * locals.var_beta) - (assign56360_body2_e87414 * locals.var_beta_dn14)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56360_body2_e87417)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign56360_body2_e87419;
            locals.var_t4_dn0 = assign56360_body2_e87419_d_n0;
            locals.var_t4_dn2 = assign56360_body2_e87419_d_n2;
            locals.var_t4_dn4 = assign56360_body2_e87419_d_n4;
            locals.var_t4_dn5 = assign56360_body2_e87419_d_n5;
            locals.var_t4_dn6 = assign56360_body2_e87419_d_n6;
            locals.var_t4_dn7 = assign56360_body2_e87419_d_n7;
            locals.var_t4_dn8 = assign56360_body2_e87419_d_n8;
            locals.var_t4_dn9 = assign56360_body2_e87419_d_n9;
            locals.var_t4_dn10 = assign56360_body2_e87419_d_n10;
            locals.var_t4_dn11 = assign56360_body2_e87419_d_n11;
            locals.var_t4_dn14 = assign56360_body2_e87419_d_n14;
            locals.var_t4_rv = 0.0;
            let (assign56360_body3_e87437, assign56360_body3_e87437_d_n0, assign56360_body3_e87437_d_n2, assign56360_body3_e87437_d_n4, assign56360_body3_e87437_d_n5, assign56360_body3_e87437_d_n6, assign56360_body3_e87437_d_n7, assign56360_body3_e87437_d_n8, assign56360_body3_e87437_d_n9, assign56360_body3_e87437_d_n10, assign56360_body3_e87437_d_n11, assign56360_body3_e87437_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56360_body3_e87433: f64 = (locals.var_t2 - locals.var_t1);
        let assign56360_body3_e87435: f64 = (assign56360_body3_e87433 - 1.0);
        (assign56360_body3_e87435, (locals.var_t2_dn0 - locals.var_t1_dn0), (locals.var_t2_dn2 - locals.var_t1_dn2), (locals.var_t2_dn4 - locals.var_t1_dn4), (locals.var_t2_dn5 - locals.var_t1_dn5), (locals.var_t2_dn6 - locals.var_t1_dn6), (locals.var_t2_dn7 - locals.var_t1_dn7), (locals.var_t2_dn8 - locals.var_t1_dn8), (locals.var_t2_dn9 - locals.var_t1_dn9), (locals.var_t2_dn10 - locals.var_t1_dn10), (locals.var_t2_dn11 - locals.var_t1_dn11), (locals.var_t2_dn14 - locals.var_t1_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
            locals.var_t10 = assign56360_body3_e87437;
            locals.var_t10_dn0 = assign56360_body3_e87437_d_n0;
            locals.var_t10_dn2 = assign56360_body3_e87437_d_n2;
            locals.var_t10_dn4 = assign56360_body3_e87437_d_n4;
            locals.var_t10_dn5 = assign56360_body3_e87437_d_n5;
            locals.var_t10_dn6 = assign56360_body3_e87437_d_n6;
            locals.var_t10_dn7 = assign56360_body3_e87437_d_n7;
            locals.var_t10_dn8 = assign56360_body3_e87437_d_n8;
            locals.var_t10_dn9 = assign56360_body3_e87437_d_n9;
            locals.var_t10_dn10 = assign56360_body3_e87437_d_n10;
            locals.var_t10_dn11 = assign56360_body3_e87437_d_n11;
            locals.var_t10_dn14 = assign56360_body3_e87437_d_n14;
            locals.var_t10_rv = 0.0;
            let (assign56360_body4_e87456, assign56360_body4_e87456_d_n0, assign56360_body4_e87456_d_n2, assign56360_body4_e87456_d_n4, assign56360_body4_e87456_d_n5, assign56360_body4_e87456_d_n6, assign56360_body4_e87456_d_n7, assign56360_body4_e87456_d_n8, assign56360_body4_e87456_d_n9, assign56360_body4_e87456_d_n10, assign56360_body4_e87456_d_n11, assign56360_body4_e87456_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56360_body4_e87452: f64 = (locals.var_t10 + 1e-15);
        let assign56360_body4_e87453: f64 = (assign56360_body4_e87452).sqrt();
        let assign56360_body4_e87454: f64 = (locals.var_t4 * assign56360_body4_e87453);
        (assign56360_body4_e87454, ((locals.var_t4_dn0 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn0 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn2 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn2 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn4 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn4 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn5 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn5 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn6 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn6 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn7 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn7 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn8 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn8 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn9 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn9 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn10 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn10 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn11 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn11 / (2.0 * assign56360_body4_e87453)))), ((locals.var_t4_dn14 * assign56360_body4_e87453) + (locals.var_t4 * (locals.var_t10_dn14 / (2.0 * assign56360_body4_e87453)))),)
    } else {
        (locals.var_q_sat, locals.var_q_sat_dn0, locals.var_q_sat_dn2, locals.var_q_sat_dn4, locals.var_q_sat_dn5, locals.var_q_sat_dn6, locals.var_q_sat_dn7, locals.var_q_sat_dn8, locals.var_q_sat_dn9, locals.var_q_sat_dn10, locals.var_q_sat_dn11, locals.var_q_sat_dn14,)
    }
};
            locals.var_q_sat = assign56360_body4_e87456;
            locals.var_q_sat_dn0 = assign56360_body4_e87456_d_n0;
            locals.var_q_sat_dn2 = assign56360_body4_e87456_d_n2;
            locals.var_q_sat_dn4 = assign56360_body4_e87456_d_n4;
            locals.var_q_sat_dn5 = assign56360_body4_e87456_d_n5;
            locals.var_q_sat_dn6 = assign56360_body4_e87456_d_n6;
            locals.var_q_sat_dn7 = assign56360_body4_e87456_d_n7;
            locals.var_q_sat_dn8 = assign56360_body4_e87456_d_n8;
            locals.var_q_sat_dn9 = assign56360_body4_e87456_d_n9;
            locals.var_q_sat_dn10 = assign56360_body4_e87456_d_n10;
            locals.var_q_sat_dn11 = assign56360_body4_e87456_d_n11;
            locals.var_q_sat_dn14 = assign56360_body4_e87456_d_n14;
            locals.var_q_sat_rv = 0.0;
            let assign56360_body5_e87459: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1412 = assign56360_body5_e87459;
            locals.var_guard1412_rv = 0.0;
            let (assign56360_body6_e87476, assign56360_body6_e87476_d_n0, assign56360_body6_e87476_d_n2, assign56360_body6_e87476_d_n4, assign56360_body6_e87476_d_n5, assign56360_body6_e87476_d_n6, assign56360_body6_e87476_d_n7, assign56360_body6_e87476_d_n8, assign56360_body6_e87476_d_n9, assign56360_body6_e87476_d_n10, assign56360_body6_e87476_d_n11, assign56360_body6_e87476_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_guard1412 != 0.0)) {
        let assign56360_body6_e87474: f64 = (-locals.var_q_sat);
        (assign56360_body6_e87474, (-locals.var_q_sat_dn0), (-locals.var_q_sat_dn2), (-locals.var_q_sat_dn4), (-locals.var_q_sat_dn5), (-locals.var_q_sat_dn6), (-locals.var_q_sat_dn7), (-locals.var_q_sat_dn8), (-locals.var_q_sat_dn9), (-locals.var_q_sat_dn10), (-locals.var_q_sat_dn11), (-locals.var_q_sat_dn14),)
    } else {
        (locals.var_q_sat, locals.var_q_sat_dn0, locals.var_q_sat_dn2, locals.var_q_sat_dn4, locals.var_q_sat_dn5, locals.var_q_sat_dn6, locals.var_q_sat_dn7, locals.var_q_sat_dn8, locals.var_q_sat_dn9, locals.var_q_sat_dn10, locals.var_q_sat_dn11, locals.var_q_sat_dn14,)
    }
};
            locals.var_q_sat = assign56360_body6_e87476;
            locals.var_q_sat_dn0 = assign56360_body6_e87476_d_n0;
            locals.var_q_sat_dn2 = assign56360_body6_e87476_d_n2;
            locals.var_q_sat_dn4 = assign56360_body6_e87476_d_n4;
            locals.var_q_sat_dn5 = assign56360_body6_e87476_d_n5;
            locals.var_q_sat_dn6 = assign56360_body6_e87476_d_n6;
            locals.var_q_sat_dn7 = assign56360_body6_e87476_d_n7;
            locals.var_q_sat_dn8 = assign56360_body6_e87476_d_n8;
            locals.var_q_sat_dn9 = assign56360_body6_e87476_d_n9;
            locals.var_q_sat_dn10 = assign56360_body6_e87476_d_n10;
            locals.var_q_sat_dn11 = assign56360_body6_e87476_d_n11;
            locals.var_q_sat_dn14 = assign56360_body6_e87476_d_n14;
            locals.var_q_sat_rv = 0.0;
            let (assign56360_body7_e87498, assign56360_body7_e87498_d_n0, assign56360_body7_e87498_d_n2, assign56360_body7_e87498_d_n4, assign56360_body7_e87498_d_n5, assign56360_body7_e87498_d_n6, assign56360_body7_e87498_d_n7, assign56360_body7_e87498_d_n8, assign56360_body7_e87498_d_n9, assign56360_body7_e87498_d_n10, assign56360_body7_e87498_d_n11, assign56360_body7_e87498_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56360_body7_e87490: f64 = (0.5 * locals.var_t4);
        let assign56360_body7_e87492: f64 = (assign56360_body7_e87490 * locals.var_t4);
        let assign56360_body7_e87494: f64 = (assign56360_body7_e87492 * locals.var_beta);
        let assign56360_body7_e87496: f64 = (assign56360_body7_e87494 / locals.var_q_sat);
        (assign56360_body7_e87496, ((((((((0.5 * locals.var_t4_dn0) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn0)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn0)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn0)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn2) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn2)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn2)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn2)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn4) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn4)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn4)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn4)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn5) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn5)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn5)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn5)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn6) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn6)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn6)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn6)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn7) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn7)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn7)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn7)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn8) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn8)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn8)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn8)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn9) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn9)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn9)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn9)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn10) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn10)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn10)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn10)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn11) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn11)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn11)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn11)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn14) * locals.var_t4) + (assign56360_body7_e87490 * locals.var_t4_dn14)) * locals.var_beta) + (assign56360_body7_e87492 * locals.var_beta_dn14)) * locals.var_q_sat) - (assign56360_body7_e87494 * locals.var_q_sat_dn14)) / (locals.var_q_sat * locals.var_q_sat)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
            locals.var_t11 = assign56360_body7_e87498;
            locals.var_t11_dn0 = assign56360_body7_e87498_d_n0;
            locals.var_t11_dn2 = assign56360_body7_e87498_d_n2;
            locals.var_t11_dn4 = assign56360_body7_e87498_d_n4;
            locals.var_t11_dn5 = assign56360_body7_e87498_d_n5;
            locals.var_t11_dn6 = assign56360_body7_e87498_d_n6;
            locals.var_t11_dn7 = assign56360_body7_e87498_d_n7;
            locals.var_t11_dn8 = assign56360_body7_e87498_d_n8;
            locals.var_t11_dn9 = assign56360_body7_e87498_d_n9;
            locals.var_t11_dn10 = assign56360_body7_e87498_d_n10;
            locals.var_t11_dn11 = assign56360_body7_e87498_d_n11;
            locals.var_t11_dn14 = assign56360_body7_e87498_d_n14;
            locals.var_t11_rv = 0.0;
            let (assign56360_body8_e87517, assign56360_body8_e87517_d_n0, assign56360_body8_e87517_d_n2, assign56360_body8_e87517_d_n4, assign56360_body8_e87517_d_n5, assign56360_body8_e87517_d_n6, assign56360_body8_e87517_d_n7, assign56360_body8_e87517_d_n8, assign56360_body8_e87517_d_n9, assign56360_body8_e87517_d_n10, assign56360_body8_e87517_d_n11, assign56360_body8_e87517_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56360_body8_e87512: f64 = (-locals.var_t2);
        let assign56360_body8_e87514: f64 = (assign56360_body8_e87512 + 1.0);
        let assign56360_body8_e87515: f64 = (locals.var_t11 * assign56360_body8_e87514);
        (assign56360_body8_e87515, ((locals.var_t11_dn0 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn0))), ((locals.var_t11_dn2 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn2))), ((locals.var_t11_dn4 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn4))), ((locals.var_t11_dn5 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn5))), ((locals.var_t11_dn6 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn6))), ((locals.var_t11_dn7 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn7))), ((locals.var_t11_dn8 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn8))), ((locals.var_t11_dn9 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn9))), ((locals.var_t11_dn10 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn10))), ((locals.var_t11_dn11 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn11))), ((locals.var_t11_dn14 * assign56360_body8_e87514) + (locals.var_t11 * (-locals.var_t2_dn14))),)
    } else {
        (locals.var_q_sat_dps, locals.var_q_sat_dps_dn0, locals.var_q_sat_dps_dn2, locals.var_q_sat_dps_dn4, locals.var_q_sat_dps_dn5, locals.var_q_sat_dps_dn6, locals.var_q_sat_dps_dn7, locals.var_q_sat_dps_dn8, locals.var_q_sat_dps_dn9, locals.var_q_sat_dps_dn10, locals.var_q_sat_dps_dn11, locals.var_q_sat_dps_dn14,)
    }
};
            locals.var_q_sat_dps = assign56360_body8_e87517;
            locals.var_q_sat_dps_dn0 = assign56360_body8_e87517_d_n0;
            locals.var_q_sat_dps_dn2 = assign56360_body8_e87517_d_n2;
            locals.var_q_sat_dps_dn4 = assign56360_body8_e87517_d_n4;
            locals.var_q_sat_dps_dn5 = assign56360_body8_e87517_d_n5;
            locals.var_q_sat_dps_dn6 = assign56360_body8_e87517_d_n6;
            locals.var_q_sat_dps_dn7 = assign56360_body8_e87517_d_n7;
            locals.var_q_sat_dps_dn8 = assign56360_body8_e87517_d_n8;
            locals.var_q_sat_dps_dn9 = assign56360_body8_e87517_d_n9;
            locals.var_q_sat_dps_dn10 = assign56360_body8_e87517_d_n10;
            locals.var_q_sat_dps_dn11 = assign56360_body8_e87517_d_n11;
            locals.var_q_sat_dps_dn14 = assign56360_body8_e87517_d_n14;
            locals.var_q_sat_dps_rv = 0.0;
            let (assign56360_body9_e87535,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign56360_body9_e87533: f64 = (150.0 + 1.0);
        (assign56360_body9_e87533,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign56360_body9_e87535;
            locals.var_lp_s0_rv = 0.0;
            let (assign56360_body10_e87559, assign56360_body10_e87559_d_n0, assign56360_body10_e87559_d_n2, assign56360_body10_e87559_d_n4, assign56360_body10_e87559_d_n5, assign56360_body10_e87559_d_n6, assign56360_body10_e87559_d_n7, assign56360_body10_e87559_d_n8, assign56360_body10_e87559_d_n9, assign56360_body10_e87559_d_n10, assign56360_body10_e87559_d_n11, assign56360_body10_e87559_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56360_body10_e87551: f64 = (-locals.var_cox);
        let assign56360_body10_e87554: f64 = (locals.var_vgpsat - locals.var_phi_vsat);
        let assign56360_body10_e87555: f64 = (assign56360_body10_e87551 * assign56360_body10_e87554);
        let assign56360_body10_e87557: f64 = (assign56360_body10_e87555 + locals.var_q_sat);
        (assign56360_body10_e87557, ((((-locals.var_cox_dn0) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn0 - locals.var_phi_vsat_dn0))) + locals.var_q_sat_dn0), ((((-locals.var_cox_dn2) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn2 - locals.var_phi_vsat_dn2))) + locals.var_q_sat_dn2), ((((-locals.var_cox_dn4) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn4 - locals.var_phi_vsat_dn4))) + locals.var_q_sat_dn4), ((((-locals.var_cox_dn5) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn5 - locals.var_phi_vsat_dn5))) + locals.var_q_sat_dn5), ((((-locals.var_cox_dn6) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn6 - locals.var_phi_vsat_dn6))) + locals.var_q_sat_dn6), ((((-locals.var_cox_dn7) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn7 - locals.var_phi_vsat_dn7))) + locals.var_q_sat_dn7), ((((-locals.var_cox_dn8) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn8 - locals.var_phi_vsat_dn8))) + locals.var_q_sat_dn8), ((((-locals.var_cox_dn9) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn9 - locals.var_phi_vsat_dn9))) + locals.var_q_sat_dn9), ((((-locals.var_cox_dn10) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn10 - locals.var_phi_vsat_dn10))) + locals.var_q_sat_dn10), ((((-locals.var_cox_dn11) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn11 - locals.var_phi_vsat_dn11))) + locals.var_q_sat_dn11), ((((-locals.var_cox_dn14) * assign56360_body10_e87554) + (assign56360_body10_e87551 * (locals.var_vgpsat_dn14 - locals.var_phi_vsat_dn14))) + locals.var_q_sat_dn14),)
    } else {
        (locals.var_pf1__blk1104, locals.var_pf1__blk1104_dn0, locals.var_pf1__blk1104_dn2, locals.var_pf1__blk1104_dn4, locals.var_pf1__blk1104_dn5, locals.var_pf1__blk1104_dn6, locals.var_pf1__blk1104_dn7, locals.var_pf1__blk1104_dn8, locals.var_pf1__blk1104_dn9, locals.var_pf1__blk1104_dn10, locals.var_pf1__blk1104_dn11, locals.var_pf1__blk1104_dn14,)
    }
};
            locals.var_pf1__blk1104 = assign56360_body10_e87559;
            locals.var_pf1__blk1104_dn0 = assign56360_body10_e87559_d_n0;
            locals.var_pf1__blk1104_dn2 = assign56360_body10_e87559_d_n2;
            locals.var_pf1__blk1104_dn4 = assign56360_body10_e87559_d_n4;
            locals.var_pf1__blk1104_dn5 = assign56360_body10_e87559_d_n5;
            locals.var_pf1__blk1104_dn6 = assign56360_body10_e87559_d_n6;
            locals.var_pf1__blk1104_dn7 = assign56360_body10_e87559_d_n7;
            locals.var_pf1__blk1104_dn8 = assign56360_body10_e87559_d_n8;
            locals.var_pf1__blk1104_dn9 = assign56360_body10_e87559_d_n9;
            locals.var_pf1__blk1104_dn10 = assign56360_body10_e87559_d_n10;
            locals.var_pf1__blk1104_dn11 = assign56360_body10_e87559_d_n11;
            locals.var_pf1__blk1104_dn14 = assign56360_body10_e87559_d_n14;
            locals.var_pf1__blk1104_rv = 0.0;
            let (assign56360_body11_e87578, assign56360_body11_e87578_d_n0, assign56360_body11_e87578_d_n2, assign56360_body11_e87578_d_n4, assign56360_body11_e87578_d_n5, assign56360_body11_e87578_d_n6, assign56360_body11_e87578_d_n7, assign56360_body11_e87578_d_n8, assign56360_body11_e87578_d_n9, assign56360_body11_e87578_d_n10, assign56360_body11_e87578_d_n11, assign56360_body11_e87578_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56360_body11_e87576: f64 = (locals.var_cox + locals.var_q_sat_dps);
        (assign56360_body11_e87576, (locals.var_cox_dn0 + locals.var_q_sat_dps_dn0), (locals.var_cox_dn2 + locals.var_q_sat_dps_dn2), (locals.var_cox_dn4 + locals.var_q_sat_dps_dn4), (locals.var_cox_dn5 + locals.var_q_sat_dps_dn5), (locals.var_cox_dn6 + locals.var_q_sat_dps_dn6), (locals.var_cox_dn7 + locals.var_q_sat_dps_dn7), (locals.var_cox_dn8 + locals.var_q_sat_dps_dn8), (locals.var_cox_dn9 + locals.var_q_sat_dps_dn9), (locals.var_cox_dn10 + locals.var_q_sat_dps_dn10), (locals.var_cox_dn11 + locals.var_q_sat_dps_dn11), (locals.var_cox_dn14 + locals.var_q_sat_dps_dn14),)
    } else {
        (locals.var_pf11__blk1105, locals.var_pf11__blk1105_dn0, locals.var_pf11__blk1105_dn2, locals.var_pf11__blk1105_dn4, locals.var_pf11__blk1105_dn5, locals.var_pf11__blk1105_dn6, locals.var_pf11__blk1105_dn7, locals.var_pf11__blk1105_dn8, locals.var_pf11__blk1105_dn9, locals.var_pf11__blk1105_dn10, locals.var_pf11__blk1105_dn11, locals.var_pf11__blk1105_dn14,)
    }
};
            locals.var_pf11__blk1105 = assign56360_body11_e87578;
            locals.var_pf11__blk1105_dn0 = assign56360_body11_e87578_d_n0;
            locals.var_pf11__blk1105_dn2 = assign56360_body11_e87578_d_n2;
            locals.var_pf11__blk1105_dn4 = assign56360_body11_e87578_d_n4;
            locals.var_pf11__blk1105_dn5 = assign56360_body11_e87578_d_n5;
            locals.var_pf11__blk1105_dn6 = assign56360_body11_e87578_d_n6;
            locals.var_pf11__blk1105_dn7 = assign56360_body11_e87578_d_n7;
            locals.var_pf11__blk1105_dn8 = assign56360_body11_e87578_d_n8;
            locals.var_pf11__blk1105_dn9 = assign56360_body11_e87578_d_n9;
            locals.var_pf11__blk1105_dn10 = assign56360_body11_e87578_d_n10;
            locals.var_pf11__blk1105_dn11 = assign56360_body11_e87578_d_n11;
            locals.var_pf11__blk1105_dn14 = assign56360_body11_e87578_d_n14;
            locals.var_pf11__blk1105_rv = 0.0;
            let (assign56360_body12_e87598, assign56360_body12_e87598_d_n0, assign56360_body12_e87598_d_n2, assign56360_body12_e87598_d_n4, assign56360_body12_e87598_d_n5, assign56360_body12_e87598_d_n6, assign56360_body12_e87598_d_n7, assign56360_body12_e87598_d_n8, assign56360_body12_e87598_d_n9, assign56360_body12_e87598_d_n10, assign56360_body12_e87598_d_n11, assign56360_body12_e87598_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56360_body12_e87594: f64 = (-locals.var_pf1__blk1104);
        let assign56360_body12_e87596: f64 = (assign56360_body12_e87594 / locals.var_pf11__blk1105);
        (assign56360_body12_e87596, ((((-locals.var_pf1__blk1104_dn0) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn0)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn2) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn2)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn4) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn4)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn5) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn5)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn6) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn6)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn7) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn7)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn8) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn8)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn9) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn9)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn10) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn10)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn11) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn11)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn14) * locals.var_pf11__blk1105) - (assign56360_body12_e87594 * locals.var_pf11__blk1105_dn14)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)),)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign56360_body12_e87598;
            locals.var_dps__blk1116_dn0 = assign56360_body12_e87598_d_n0;
            locals.var_dps__blk1116_dn2 = assign56360_body12_e87598_d_n2;
            locals.var_dps__blk1116_dn4 = assign56360_body12_e87598_d_n4;
            locals.var_dps__blk1116_dn5 = assign56360_body12_e87598_d_n5;
            locals.var_dps__blk1116_dn6 = assign56360_body12_e87598_d_n6;
            locals.var_dps__blk1116_dn7 = assign56360_body12_e87598_d_n7;
            locals.var_dps__blk1116_dn8 = assign56360_body12_e87598_d_n8;
            locals.var_dps__blk1116_dn9 = assign56360_body12_e87598_d_n9;
            locals.var_dps__blk1116_dn10 = assign56360_body12_e87598_d_n10;
            locals.var_dps__blk1116_dn11 = assign56360_body12_e87598_d_n11;
            locals.var_dps__blk1116_dn14 = assign56360_body12_e87598_d_n14;
            locals.var_dps__blk1116_rv = 0.0;
            let assign56360_body13_e87600: f64 = (locals.var_dps__blk1116).abs();
            let assign56360_body13_e87602: f64 = if assign56360_body13_e87600 < 1e-10 { 1.0 } else { 0.0 };
            locals.var_guard1413 = assign56360_body13_e87602;
            locals.var_guard1413_rv = 0.0;
            let (assign56360_body14_e87621,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1413 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign56360_body14_e87621;
            locals.var_flg_conv_rv = 0.0;
            let assign56360_body15_e87624: f64 = if locals.var_dps__blk1116 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1414 = assign56360_body15_e87624;
            locals.var_guard1414_rv = 0.0;
            let (assign56360_body16_e87646, assign56360_body16_e87646_d_n0, assign56360_body16_e87646_d_n2, assign56360_body16_e87646_d_n4, assign56360_body16_e87646_d_n5, assign56360_body16_e87646_d_n6, assign56360_body16_e87646_d_n7, assign56360_body16_e87646_d_n8, assign56360_body16_e87646_d_n9, assign56360_body16_e87646_d_n10, assign56360_body16_e87646_d_n11, assign56360_body16_e87646_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1413 == 0.0)) && (locals.var_guard1414 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign56360_body16_e87646;
            locals.var_dps__blk1116_dn0 = assign56360_body16_e87646_d_n0;
            locals.var_dps__blk1116_dn2 = assign56360_body16_e87646_d_n2;
            locals.var_dps__blk1116_dn4 = assign56360_body16_e87646_d_n4;
            locals.var_dps__blk1116_dn5 = assign56360_body16_e87646_d_n5;
            locals.var_dps__blk1116_dn6 = assign56360_body16_e87646_d_n6;
            locals.var_dps__blk1116_dn7 = assign56360_body16_e87646_d_n7;
            locals.var_dps__blk1116_dn8 = assign56360_body16_e87646_d_n8;
            locals.var_dps__blk1116_dn9 = assign56360_body16_e87646_d_n9;
            locals.var_dps__blk1116_dn10 = assign56360_body16_e87646_d_n10;
            locals.var_dps__blk1116_dn11 = assign56360_body16_e87646_d_n11;
            locals.var_dps__blk1116_dn14 = assign56360_body16_e87646_d_n14;
            locals.var_dps__blk1116_rv = 0.0;
            let assign56360_body17_e87649: f64 = (-0.1);
            let assign56360_body17_e87650: f64 = if locals.var_dps__blk1116 < assign56360_body17_e87649 { 1.0 } else { 0.0 };
            locals.var_guard1415 = assign56360_body17_e87650;
            locals.var_guard1415_rv = 0.0;
            let (assign56360_body18_e87676, assign56360_body18_e87676_d_n0, assign56360_body18_e87676_d_n2, assign56360_body18_e87676_d_n4, assign56360_body18_e87676_d_n5, assign56360_body18_e87676_d_n6, assign56360_body18_e87676_d_n7, assign56360_body18_e87676_d_n8, assign56360_body18_e87676_d_n9, assign56360_body18_e87676_d_n10, assign56360_body18_e87676_d_n11, assign56360_body18_e87676_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1413 == 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1415 != 0.0)) {
        let assign56360_body18_e87674: f64 = (-0.1);
        (assign56360_body18_e87674, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign56360_body18_e87676;
            locals.var_dps__blk1116_dn0 = assign56360_body18_e87676_d_n0;
            locals.var_dps__blk1116_dn2 = assign56360_body18_e87676_d_n2;
            locals.var_dps__blk1116_dn4 = assign56360_body18_e87676_d_n4;
            locals.var_dps__blk1116_dn5 = assign56360_body18_e87676_d_n5;
            locals.var_dps__blk1116_dn6 = assign56360_body18_e87676_d_n6;
            locals.var_dps__blk1116_dn7 = assign56360_body18_e87676_d_n7;
            locals.var_dps__blk1116_dn8 = assign56360_body18_e87676_d_n8;
            locals.var_dps__blk1116_dn9 = assign56360_body18_e87676_d_n9;
            locals.var_dps__blk1116_dn10 = assign56360_body18_e87676_d_n10;
            locals.var_dps__blk1116_dn11 = assign56360_body18_e87676_d_n11;
            locals.var_dps__blk1116_dn14 = assign56360_body18_e87676_d_n14;
            locals.var_dps__blk1116_rv = 0.0;
            let (assign56360_body19_e87695, assign56360_body19_e87695_d_n0, assign56360_body19_e87695_d_n2, assign56360_body19_e87695_d_n4, assign56360_body19_e87695_d_n5, assign56360_body19_e87695_d_n6, assign56360_body19_e87695_d_n7, assign56360_body19_e87695_d_n8, assign56360_body19_e87695_d_n9, assign56360_body19_e87695_d_n10, assign56360_body19_e87695_d_n11, assign56360_body19_e87695_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56360_body19_e87693: f64 = (locals.var_phi_vsat + locals.var_dps__blk1116);
        (assign56360_body19_e87693, (locals.var_phi_vsat_dn0 + locals.var_dps__blk1116_dn0), (locals.var_phi_vsat_dn2 + locals.var_dps__blk1116_dn2), (locals.var_phi_vsat_dn4 + locals.var_dps__blk1116_dn4), (locals.var_phi_vsat_dn5 + locals.var_dps__blk1116_dn5), (locals.var_phi_vsat_dn6 + locals.var_dps__blk1116_dn6), (locals.var_phi_vsat_dn7 + locals.var_dps__blk1116_dn7), (locals.var_phi_vsat_dn8 + locals.var_dps__blk1116_dn8), (locals.var_phi_vsat_dn9 + locals.var_dps__blk1116_dn9), (locals.var_phi_vsat_dn10 + locals.var_dps__blk1116_dn10), (locals.var_phi_vsat_dn11 + locals.var_dps__blk1116_dn11), (locals.var_phi_vsat_dn14 + locals.var_dps__blk1116_dn14),)
    } else {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn11, locals.var_phi_vsat_dn14,)
    }
};
            locals.var_phi_vsat = assign56360_body19_e87695;
            locals.var_phi_vsat_dn0 = assign56360_body19_e87695_d_n0;
            locals.var_phi_vsat_dn2 = assign56360_body19_e87695_d_n2;
            locals.var_phi_vsat_dn4 = assign56360_body19_e87695_d_n4;
            locals.var_phi_vsat_dn5 = assign56360_body19_e87695_d_n5;
            locals.var_phi_vsat_dn6 = assign56360_body19_e87695_d_n6;
            locals.var_phi_vsat_dn7 = assign56360_body19_e87695_d_n7;
            locals.var_phi_vsat_dn8 = assign56360_body19_e87695_d_n8;
            locals.var_phi_vsat_dn9 = assign56360_body19_e87695_d_n9;
            locals.var_phi_vsat_dn10 = assign56360_body19_e87695_d_n10;
            locals.var_phi_vsat_dn11 = assign56360_body19_e87695_d_n11;
            locals.var_phi_vsat_dn14 = assign56360_body19_e87695_d_n14;
            locals.var_phi_vsat_rv = 0.0;
            let (assign56360_body20_e87711,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56360_body20_e87709: f64 = (locals.var_lp_s0 + 1.0);
        (assign56360_body20_e87709,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign56360_body20_e87711;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign56370_e87725, assign56370_e87725_d_n0, assign56370_e87725_d_n2, assign56370_e87725_d_n4, assign56370_e87725_d_n5, assign56370_e87725_d_n6, assign56370_e87725_d_n7, assign56370_e87725_d_n8, assign56370_e87725_d_n9, assign56370_e87725_d_n10, assign56370_e87725_d_n11, assign56370_e87725_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn11, locals.var_phi_vsat_dn14,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    }
};
        locals.var_ps0_res = assign56370_e87725;
        locals.var_ps0_res_dn0 = assign56370_e87725_d_n0;
        locals.var_ps0_res_dn2 = assign56370_e87725_d_n2;
        locals.var_ps0_res_dn4 = assign56370_e87725_d_n4;
        locals.var_ps0_res_dn5 = assign56370_e87725_d_n5;
        locals.var_ps0_res_dn6 = assign56370_e87725_d_n6;
        locals.var_ps0_res_dn7 = assign56370_e87725_d_n7;
        locals.var_ps0_res_dn8 = assign56370_e87725_d_n8;
        locals.var_ps0_res_dn9 = assign56370_e87725_d_n9;
        locals.var_ps0_res_dn10 = assign56370_e87725_d_n10;
        locals.var_ps0_res_dn11 = assign56370_e87725_d_n11;
        locals.var_ps0_res_dn14 = assign56370_e87725_d_n14;
        locals.var_ps0_res_rv = 0.0;

        let (assign56380_e87739, assign56380_e87739_d_n0, assign56380_e87739_d_n2, assign56380_e87739_d_n4, assign56380_e87739_d_n5, assign56380_e87739_d_n6, assign56380_e87739_d_n7, assign56380_e87739_d_n8, assign56380_e87739_d_n9, assign56380_e87739_d_n10, assign56380_e87739_d_n11, assign56380_e87739_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign56380_e87739;
        locals.var_vds_res_dn0 = assign56380_e87739_d_n0;
        locals.var_vds_res_dn2 = assign56380_e87739_d_n2;
        locals.var_vds_res_dn4 = assign56380_e87739_d_n4;
        locals.var_vds_res_dn5 = assign56380_e87739_d_n5;
        locals.var_vds_res_dn6 = assign56380_e87739_d_n6;
        locals.var_vds_res_dn7 = assign56380_e87739_d_n7;
        locals.var_vds_res_dn8 = assign56380_e87739_d_n8;
        locals.var_vds_res_dn9 = assign56380_e87739_d_n9;
        locals.var_vds_res_dn10 = assign56380_e87739_d_n10;
        locals.var_vds_res_dn11 = assign56380_e87739_d_n11;
        locals.var_vds_res_dn14 = assign56380_e87739_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign56390_e87762, assign56390_e87762_d_n0, assign56390_e87762_d_n2, assign56390_e87762_d_n4, assign56390_e87762_d_n5, assign56390_e87762_d_n6, assign56390_e87762_d_n7, assign56390_e87762_d_n8, assign56390_e87762_d_n9, assign56390_e87762_d_n10, assign56390_e87762_d_n11, assign56390_e87762_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56390_e87753: f64 = (locals.var_ps0_res * locals.var_ps0_res);
        let assign56390_e87756: f64 = (4.0 * p.p405);
        let assign56390_e87758: f64 = (assign56390_e87756 * p.p405);
        let assign56390_e87759: f64 = (assign56390_e87753 + assign56390_e87758);
        let assign56390_e87760: f64 = (assign56390_e87759).sqrt();
        (assign56390_e87760, (((locals.var_ps0_res_dn0 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn0)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn2 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn2)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn4 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn4)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn5 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn5)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn6 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn6)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn7 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn7)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn8 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn8)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn9 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn9)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn10 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn10)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn11 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn11)) / (2.0 * assign56390_e87760)), (((locals.var_ps0_res_dn14 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn14)) / (2.0 * assign56390_e87760)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign56390_e87762;
        locals.var_tmf2_dn0 = assign56390_e87762_d_n0;
        locals.var_tmf2_dn2 = assign56390_e87762_d_n2;
        locals.var_tmf2_dn4 = assign56390_e87762_d_n4;
        locals.var_tmf2_dn5 = assign56390_e87762_d_n5;
        locals.var_tmf2_dn6 = assign56390_e87762_d_n6;
        locals.var_tmf2_dn7 = assign56390_e87762_d_n7;
        locals.var_tmf2_dn8 = assign56390_e87762_d_n8;
        locals.var_tmf2_dn9 = assign56390_e87762_d_n9;
        locals.var_tmf2_dn10 = assign56390_e87762_d_n10;
        locals.var_tmf2_dn11 = assign56390_e87762_d_n11;
        locals.var_tmf2_dn14 = assign56390_e87762_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign56400_e87782, assign56400_e87782_d_n0, assign56400_e87782_d_n2, assign56400_e87782_d_n4, assign56400_e87782_d_n5, assign56400_e87782_d_n6, assign56400_e87782_d_n7, assign56400_e87782_d_n8, assign56400_e87782_d_n9, assign56400_e87782_d_n10, assign56400_e87782_d_n11, assign56400_e87782_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56400_e87778: f64 = (locals.var_ps0_res / locals.var_tmf2);
        let assign56400_e87779: f64 = (1.0 + assign56400_e87778);
        let assign56400_e87780: f64 = (0.5 * assign56400_e87779);
        (assign56400_e87780, (0.5 * (((locals.var_ps0_res_dn0 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn2 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn4 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn5 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn6 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn7 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn8 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn9 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn10 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn11 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn14 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56400_e87782;
        locals.var_t0_dn0 = assign56400_e87782_d_n0;
        locals.var_t0_dn2 = assign56400_e87782_d_n2;
        locals.var_t0_dn4 = assign56400_e87782_d_n4;
        locals.var_t0_dn5 = assign56400_e87782_d_n5;
        locals.var_t0_dn6 = assign56400_e87782_d_n6;
        locals.var_t0_dn7 = assign56400_e87782_d_n7;
        locals.var_t0_dn8 = assign56400_e87782_d_n8;
        locals.var_t0_dn9 = assign56400_e87782_d_n9;
        locals.var_t0_dn10 = assign56400_e87782_d_n10;
        locals.var_t0_dn11 = assign56400_e87782_d_n11;
        locals.var_t0_dn14 = assign56400_e87782_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56410_e87800, assign56410_e87800_d_n0, assign56410_e87800_d_n2, assign56410_e87800_d_n4, assign56410_e87800_d_n5, assign56410_e87800_d_n6, assign56410_e87800_d_n7, assign56410_e87800_d_n8, assign56410_e87800_d_n9, assign56410_e87800_d_n10, assign56410_e87800_d_n11, assign56410_e87800_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) {
        let assign56410_e87797: f64 = (locals.var_ps0_res + locals.var_tmf2);
        let assign56410_e87798: f64 = (0.5 * assign56410_e87797);
        (assign56410_e87798, (0.5 * (locals.var_ps0_res_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_ps0_res_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_ps0_res_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_ps0_res_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_ps0_res_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_ps0_res_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_ps0_res_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_ps0_res_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_ps0_res_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_ps0_res_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_ps0_res_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56410_e87800;
        locals.var_vdssat_res_dn0 = assign56410_e87800_d_n0;
        locals.var_vdssat_res_dn2 = assign56410_e87800_d_n2;
        locals.var_vdssat_res_dn4 = assign56410_e87800_d_n4;
        locals.var_vdssat_res_dn5 = assign56410_e87800_d_n5;
        locals.var_vdssat_res_dn6 = assign56410_e87800_d_n6;
        locals.var_vdssat_res_dn7 = assign56410_e87800_d_n7;
        locals.var_vdssat_res_dn8 = assign56410_e87800_d_n8;
        locals.var_vdssat_res_dn9 = assign56410_e87800_d_n9;
        locals.var_vdssat_res_dn10 = assign56410_e87800_d_n10;
        locals.var_vdssat_res_dn11 = assign56410_e87800_d_n11;
        locals.var_vdssat_res_dn14 = assign56410_e87800_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let assign56420_e87803: f64 = if locals.var_vdssat_res < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1416 = assign56420_e87803;
        locals.var_guard1416_rv = 0.0;

        let (assign56430_e87819, assign56430_e87819_d_n0, assign56430_e87819_d_n2, assign56430_e87819_d_n4, assign56430_e87819_d_n5, assign56430_e87819_d_n6, assign56430_e87819_d_n7, assign56430_e87819_d_n8, assign56430_e87819_d_n9, assign56430_e87819_d_n10, assign56430_e87819_d_n11, assign56430_e87819_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_guard1416 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56430_e87819;
        locals.var_vdssat_res_dn0 = assign56430_e87819_d_n0;
        locals.var_vdssat_res_dn2 = assign56430_e87819_d_n2;
        locals.var_vdssat_res_dn4 = assign56430_e87819_d_n4;
        locals.var_vdssat_res_dn5 = assign56430_e87819_d_n5;
        locals.var_vdssat_res_dn6 = assign56430_e87819_d_n6;
        locals.var_vdssat_res_dn7 = assign56430_e87819_d_n7;
        locals.var_vdssat_res_dn8 = assign56430_e87819_d_n8;
        locals.var_vdssat_res_dn9 = assign56430_e87819_d_n9;
        locals.var_vdssat_res_dn10 = assign56430_e87819_d_n10;
        locals.var_vdssat_res_dn11 = assign56430_e87819_d_n11;
        locals.var_vdssat_res_dn14 = assign56430_e87819_d_n14;
        locals.var_vdssat_res_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_206(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56440_e87835, assign56440_e87835_d_n0, assign56440_e87835_d_n2, assign56440_e87835_d_n4, assign56440_e87835_d_n5, assign56440_e87835_d_n6, assign56440_e87835_d_n7, assign56440_e87835_d_n8, assign56440_e87835_d_n9, assign56440_e87835_d_n10, assign56440_e87835_d_n11, assign56440_e87835_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 == 0.0)) && (locals.var_guard1416 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56440_e87835;
        locals.var_t0_dn0 = assign56440_e87835_d_n0;
        locals.var_t0_dn2 = assign56440_e87835_d_n2;
        locals.var_t0_dn4 = assign56440_e87835_d_n4;
        locals.var_t0_dn5 = assign56440_e87835_d_n5;
        locals.var_t0_dn6 = assign56440_e87835_d_n6;
        locals.var_t0_dn7 = assign56440_e87835_d_n7;
        locals.var_t0_dn8 = assign56440_e87835_d_n8;
        locals.var_t0_dn9 = assign56440_e87835_d_n9;
        locals.var_t0_dn10 = assign56440_e87835_d_n10;
        locals.var_t0_dn11 = assign56440_e87835_d_n11;
        locals.var_t0_dn14 = assign56440_e87835_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56450_e87848, assign56450_e87848_d_n0, assign56450_e87848_d_n2, assign56450_e87848_d_n4, assign56450_e87848_d_n5, assign56450_e87848_d_n6, assign56450_e87848_d_n7, assign56450_e87848_d_n8, assign56450_e87848_d_n9, assign56450_e87848_d_n10, assign56450_e87848_d_n11, assign56450_e87848_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign56450_e87846: f64 = (locals.var_vds_res / locals.var_vdssat_res);
        (assign56450_e87846, (((locals.var_vds_res_dn0 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn0)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn2 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn2)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn4 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn4)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn5 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn5)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn6 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn6)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn7 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn7)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn8 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn8)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn9 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn9)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn10 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn10)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn11 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn11)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn14 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn14)) / (locals.var_vdssat_res * locals.var_vdssat_res)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign56450_e87848;
        locals.var_t1_dn0 = assign56450_e87848_d_n0;
        locals.var_t1_dn2 = assign56450_e87848_d_n2;
        locals.var_t1_dn4 = assign56450_e87848_d_n4;
        locals.var_t1_dn5 = assign56450_e87848_d_n5;
        locals.var_t1_dn6 = assign56450_e87848_d_n6;
        locals.var_t1_dn7 = assign56450_e87848_d_n7;
        locals.var_t1_dn8 = assign56450_e87848_d_n8;
        locals.var_t1_dn9 = assign56450_e87848_d_n9;
        locals.var_t1_dn10 = assign56450_e87848_d_n10;
        locals.var_t1_dn11 = assign56450_e87848_d_n11;
        locals.var_t1_dn14 = assign56450_e87848_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign56460_e87868, assign56460_e87868_d_n0, assign56460_e87868_d_n2, assign56460_e87868_d_n4, assign56460_e87868_d_n5, assign56460_e87868_d_n6, assign56460_e87868_d_n7, assign56460_e87868_d_n8, assign56460_e87868_d_n9, assign56460_e87868_d_n10, assign56460_e87868_d_n11, assign56460_e87868_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let (assign56460_e87866, assign56460_e87866_d_n0, assign56460_e87866_d_n2, assign56460_e87866_d_n4, assign56460_e87866_d_n5, assign56460_e87866_d_n6, assign56460_e87866_d_n7, assign56460_e87866_d_n8, assign56460_e87866_d_n9, assign56460_e87866_d_n10, assign56460_e87866_d_n11, assign56460_e87866_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56460_e87864: f64 = (p.p383 - 1.0);
                let assign56460_e87865: f64 = (locals.var_t1).powf(assign56460_e87864);
                (assign56460_e87865, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn0)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn2)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn4)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn5)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn6)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn7)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn8)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn9)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn10)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn11)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56460_e87864) as f64).is_finite() && ((assign56460_e87864) as f64).fract() == 0.0 { if assign56460_e87864 == 0.0 { 0.0 } else { (assign56460_e87864 * ((locals.var_t1).powf(assign56460_e87864 - 1.0) * locals.var_t1_dn14)) } } else { (assign56460_e87865 * (assign56460_e87864 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign56460_e87866, assign56460_e87866_d_n0, assign56460_e87866_d_n2, assign56460_e87866_d_n4, assign56460_e87866_d_n5, assign56460_e87866_d_n6, assign56460_e87866_d_n7, assign56460_e87866_d_n8, assign56460_e87866_d_n9, assign56460_e87866_d_n10, assign56460_e87866_d_n11, assign56460_e87866_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign56460_e87868;
        locals.var_t2_dn0 = assign56460_e87868_d_n0;
        locals.var_t2_dn2 = assign56460_e87868_d_n2;
        locals.var_t2_dn4 = assign56460_e87868_d_n4;
        locals.var_t2_dn5 = assign56460_e87868_d_n5;
        locals.var_t2_dn6 = assign56460_e87868_d_n6;
        locals.var_t2_dn7 = assign56460_e87868_d_n7;
        locals.var_t2_dn8 = assign56460_e87868_d_n8;
        locals.var_t2_dn9 = assign56460_e87868_d_n9;
        locals.var_t2_dn10 = assign56460_e87868_d_n10;
        locals.var_t2_dn11 = assign56460_e87868_d_n11;
        locals.var_t2_dn14 = assign56460_e87868_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign56470_e87883, assign56470_e87883_d_n0, assign56470_e87883_d_n2, assign56470_e87883_d_n4, assign56470_e87883_d_n5, assign56470_e87883_d_n6, assign56470_e87883_d_n7, assign56470_e87883_d_n8, assign56470_e87883_d_n9, assign56470_e87883_d_n10, assign56470_e87883_d_n11, assign56470_e87883_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign56470_e87880: f64 = (locals.var_t2 * locals.var_t1);
        let assign56470_e87881: f64 = (1.0 + assign56470_e87880);
        (assign56470_e87881, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign56470_e87883;
        locals.var_t3_dn0 = assign56470_e87883_d_n0;
        locals.var_t3_dn2 = assign56470_e87883_d_n2;
        locals.var_t3_dn4 = assign56470_e87883_d_n4;
        locals.var_t3_dn5 = assign56470_e87883_d_n5;
        locals.var_t3_dn6 = assign56470_e87883_d_n6;
        locals.var_t3_dn7 = assign56470_e87883_d_n7;
        locals.var_t3_dn8 = assign56470_e87883_d_n8;
        locals.var_t3_dn9 = assign56470_e87883_d_n9;
        locals.var_t3_dn10 = assign56470_e87883_d_n10;
        locals.var_t3_dn11 = assign56470_e87883_d_n11;
        locals.var_t3_dn14 = assign56470_e87883_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign56480_e87905, assign56480_e87905_d_n0, assign56480_e87905_d_n2, assign56480_e87905_d_n4, assign56480_e87905_d_n5, assign56480_e87905_d_n6, assign56480_e87905_d_n7, assign56480_e87905_d_n8, assign56480_e87905_d_n9, assign56480_e87905_d_n10, assign56480_e87905_d_n11, assign56480_e87905_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let (assign56480_e87903, assign56480_e87903_d_n0, assign56480_e87903_d_n2, assign56480_e87903_d_n4, assign56480_e87903_d_n5, assign56480_e87903_d_n6, assign56480_e87903_d_n7, assign56480_e87903_d_n8, assign56480_e87903_d_n9, assign56480_e87903_d_n10, assign56480_e87903_d_n11, assign56480_e87903_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56480_e87899: f64 = (1.0 / p.p383);
                let assign56480_e87901: f64 = (assign56480_e87899 - 1.0);
                let assign56480_e87902: f64 = (locals.var_t3).powf(assign56480_e87901);
                (assign56480_e87902, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn0)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn2)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn4)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn5)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn6)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn7)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn8)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn9)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn10)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn11)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56480_e87901) as f64).is_finite() && ((assign56480_e87901) as f64).fract() == 0.0 { if assign56480_e87901 == 0.0 { 0.0 } else { (assign56480_e87901 * ((locals.var_t3).powf(assign56480_e87901 - 1.0) * locals.var_t3_dn14)) } } else { (assign56480_e87902 * (assign56480_e87901 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign56480_e87903, assign56480_e87903_d_n0, assign56480_e87903_d_n2, assign56480_e87903_d_n4, assign56480_e87903_d_n5, assign56480_e87903_d_n6, assign56480_e87903_d_n7, assign56480_e87903_d_n8, assign56480_e87903_d_n9, assign56480_e87903_d_n10, assign56480_e87903_d_n11, assign56480_e87903_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign56480_e87905;
        locals.var_t4_dn0 = assign56480_e87905_d_n0;
        locals.var_t4_dn2 = assign56480_e87905_d_n2;
        locals.var_t4_dn4 = assign56480_e87905_d_n4;
        locals.var_t4_dn5 = assign56480_e87905_d_n5;
        locals.var_t4_dn6 = assign56480_e87905_d_n6;
        locals.var_t4_dn7 = assign56480_e87905_d_n7;
        locals.var_t4_dn8 = assign56480_e87905_d_n8;
        locals.var_t4_dn9 = assign56480_e87905_d_n9;
        locals.var_t4_dn10 = assign56480_e87905_d_n10;
        locals.var_t4_dn11 = assign56480_e87905_d_n11;
        locals.var_t4_dn14 = assign56480_e87905_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign56490_e87918, assign56490_e87918_d_n0, assign56490_e87918_d_n2, assign56490_e87918_d_n4, assign56490_e87918_d_n5, assign56490_e87918_d_n6, assign56490_e87918_d_n7, assign56490_e87918_d_n8, assign56490_e87918_d_n9, assign56490_e87918_d_n10, assign56490_e87918_d_n11, assign56490_e87918_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign56490_e87916: f64 = (locals.var_t4 * locals.var_t3);
        (assign56490_e87916, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign56490_e87918;
        locals.var_t6_dn0 = assign56490_e87918_d_n0;
        locals.var_t6_dn2 = assign56490_e87918_d_n2;
        locals.var_t6_dn4 = assign56490_e87918_d_n4;
        locals.var_t6_dn5 = assign56490_e87918_d_n5;
        locals.var_t6_dn6 = assign56490_e87918_d_n6;
        locals.var_t6_dn7 = assign56490_e87918_d_n7;
        locals.var_t6_dn8 = assign56490_e87918_d_n8;
        locals.var_t6_dn9 = assign56490_e87918_d_n9;
        locals.var_t6_dn10 = assign56490_e87918_d_n10;
        locals.var_t6_dn11 = assign56490_e87918_d_n11;
        locals.var_t6_dn14 = assign56490_e87918_d_n14;
        locals.var_t6_rv = 0.0;

        let assign56500_e87923: f64 = (locals.var_uc_depleak * 0.5);
        let assign56500_e87924: f64 = (locals.var_uc_depleak - assign56500_e87923);
        let assign56500_e87928: f64 = (locals.var_uc_depleak * 0.5);
        let assign56500_e87931: f64 = if ((locals.var_vdsorg > assign56500_e87924) && (assign56500_e87928 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1417 = assign56500_e87931;
        locals.var_guard1417_rv = 0.0;

        let (assign56510_e87950, assign56510_e87950_d_n0, assign56510_e87950_d_n2, assign56510_e87950_d_n4, assign56510_e87950_d_n5, assign56510_e87950_d_n6, assign56510_e87950_d_n7, assign56510_e87950_d_n8, assign56510_e87950_d_n9, assign56510_e87950_d_n10, assign56510_e87950_d_n11, assign56510_e87950_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56510_e87944: f64 = (locals.var_vdsorg - locals.var_uc_depleak);
        let assign56510_e87947: f64 = (locals.var_uc_depleak * 0.5);
        let assign56510_e87948: f64 = (assign56510_e87944 + assign56510_e87947);
        (assign56510_e87948, ((locals.var_vdsorg_dn0 - locals.var_uc_depleak_dn0) + (locals.var_uc_depleak_dn0 * 0.5)), ((locals.var_vdsorg_dn2 - locals.var_uc_depleak_dn2) + (locals.var_uc_depleak_dn2 * 0.5)), ((locals.var_vdsorg_dn4 - locals.var_uc_depleak_dn4) + (locals.var_uc_depleak_dn4 * 0.5)), ((locals.var_vdsorg_dn5 - locals.var_uc_depleak_dn5) + (locals.var_uc_depleak_dn5 * 0.5)), ((locals.var_vdsorg_dn6 - locals.var_uc_depleak_dn6) + (locals.var_uc_depleak_dn6 * 0.5)), ((locals.var_vdsorg_dn7 - locals.var_uc_depleak_dn7) + (locals.var_uc_depleak_dn7 * 0.5)), ((locals.var_vdsorg_dn8 - locals.var_uc_depleak_dn8) + (locals.var_uc_depleak_dn8 * 0.5)), ((locals.var_vdsorg_dn9 - locals.var_uc_depleak_dn9) + (locals.var_uc_depleak_dn9 * 0.5)), ((locals.var_vdsorg_dn10 - locals.var_uc_depleak_dn10) + (locals.var_uc_depleak_dn10 * 0.5)), ((locals.var_vdsorg_dn11 - locals.var_uc_depleak_dn11) + (locals.var_uc_depleak_dn11 * 0.5)), ((locals.var_vdsorg_dn14 - locals.var_uc_depleak_dn14) + (locals.var_uc_depleak_dn14 * 0.5)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign56510_e87950;
        locals.var_tmf1_dn0 = assign56510_e87950_d_n0;
        locals.var_tmf1_dn2 = assign56510_e87950_d_n2;
        locals.var_tmf1_dn4 = assign56510_e87950_d_n4;
        locals.var_tmf1_dn5 = assign56510_e87950_d_n5;
        locals.var_tmf1_dn6 = assign56510_e87950_d_n6;
        locals.var_tmf1_dn7 = assign56510_e87950_d_n7;
        locals.var_tmf1_dn8 = assign56510_e87950_d_n8;
        locals.var_tmf1_dn9 = assign56510_e87950_d_n9;
        locals.var_tmf1_dn10 = assign56510_e87950_d_n10;
        locals.var_tmf1_dn11 = assign56510_e87950_d_n11;
        locals.var_tmf1_dn14 = assign56510_e87950_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign56520_e87965, assign56520_e87965_d_n0, assign56520_e87965_d_n2, assign56520_e87965_d_n4, assign56520_e87965_d_n5, assign56520_e87965_d_n6, assign56520_e87965_d_n7, assign56520_e87965_d_n8, assign56520_e87965_d_n9, assign56520_e87965_d_n10, assign56520_e87965_d_n11, assign56520_e87965_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56520_e87963: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign56520_e87963, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign56520_e87965;
        locals.var_x2_dn0 = assign56520_e87965_d_n0;
        locals.var_x2_dn2 = assign56520_e87965_d_n2;
        locals.var_x2_dn4 = assign56520_e87965_d_n4;
        locals.var_x2_dn5 = assign56520_e87965_d_n5;
        locals.var_x2_dn6 = assign56520_e87965_d_n6;
        locals.var_x2_dn7 = assign56520_e87965_d_n7;
        locals.var_x2_dn8 = assign56520_e87965_d_n8;
        locals.var_x2_dn9 = assign56520_e87965_d_n9;
        locals.var_x2_dn10 = assign56520_e87965_d_n10;
        locals.var_x2_dn11 = assign56520_e87965_d_n11;
        locals.var_x2_dn14 = assign56520_e87965_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign56530_e87984, assign56530_e87984_d_n0, assign56530_e87984_d_n2, assign56530_e87984_d_n4, assign56530_e87984_d_n5, assign56530_e87984_d_n6, assign56530_e87984_d_n7, assign56530_e87984_d_n8, assign56530_e87984_d_n9, assign56530_e87984_d_n10, assign56530_e87984_d_n11, assign56530_e87984_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56530_e87978: f64 = (locals.var_uc_depleak * 0.5);
        let assign56530_e87981: f64 = (locals.var_uc_depleak * 0.5);
        let assign56530_e87982: f64 = (assign56530_e87978 * assign56530_e87981);
        (assign56530_e87982, (((locals.var_uc_depleak_dn0 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn0 * 0.5))), (((locals.var_uc_depleak_dn2 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn2 * 0.5))), (((locals.var_uc_depleak_dn4 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn4 * 0.5))), (((locals.var_uc_depleak_dn5 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn5 * 0.5))), (((locals.var_uc_depleak_dn6 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn6 * 0.5))), (((locals.var_uc_depleak_dn7 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn7 * 0.5))), (((locals.var_uc_depleak_dn8 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn8 * 0.5))), (((locals.var_uc_depleak_dn9 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn9 * 0.5))), (((locals.var_uc_depleak_dn10 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn10 * 0.5))), (((locals.var_uc_depleak_dn11 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn11 * 0.5))), (((locals.var_uc_depleak_dn14 * 0.5) * assign56530_e87981) + (assign56530_e87978 * (locals.var_uc_depleak_dn14 * 0.5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign56530_e87984;
        locals.var_xmax2_dn0 = assign56530_e87984_d_n0;
        locals.var_xmax2_dn2 = assign56530_e87984_d_n2;
        locals.var_xmax2_dn4 = assign56530_e87984_d_n4;
        locals.var_xmax2_dn5 = assign56530_e87984_d_n5;
        locals.var_xmax2_dn6 = assign56530_e87984_d_n6;
        locals.var_xmax2_dn7 = assign56530_e87984_d_n7;
        locals.var_xmax2_dn8 = assign56530_e87984_d_n8;
        locals.var_xmax2_dn9 = assign56530_e87984_d_n9;
        locals.var_xmax2_dn10 = assign56530_e87984_d_n10;
        locals.var_xmax2_dn11 = assign56530_e87984_d_n11;
        locals.var_xmax2_dn14 = assign56530_e87984_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign56540_e87997, assign56540_e87997_d_n0, assign56540_e87997_d_n2, assign56540_e87997_d_n4, assign56540_e87997_d_n5, assign56540_e87997_d_n6, assign56540_e87997_d_n7, assign56540_e87997_d_n8, assign56540_e87997_d_n9, assign56540_e87997_d_n10, assign56540_e87997_d_n11, assign56540_e87997_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56540_e87997;
        locals.var_xp_dn0 = assign56540_e87997_d_n0;
        locals.var_xp_dn2 = assign56540_e87997_d_n2;
        locals.var_xp_dn4 = assign56540_e87997_d_n4;
        locals.var_xp_dn5 = assign56540_e87997_d_n5;
        locals.var_xp_dn6 = assign56540_e87997_d_n6;
        locals.var_xp_dn7 = assign56540_e87997_d_n7;
        locals.var_xp_dn8 = assign56540_e87997_d_n8;
        locals.var_xp_dn9 = assign56540_e87997_d_n9;
        locals.var_xp_dn10 = assign56540_e87997_d_n10;
        locals.var_xp_dn11 = assign56540_e87997_d_n11;
        locals.var_xp_dn14 = assign56540_e87997_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56550_e88010, assign56550_e88010_d_n0, assign56550_e88010_d_n2, assign56550_e88010_d_n4, assign56550_e88010_d_n5, assign56550_e88010_d_n6, assign56550_e88010_d_n7, assign56550_e88010_d_n8, assign56550_e88010_d_n9, assign56550_e88010_d_n10, assign56550_e88010_d_n11, assign56550_e88010_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56550_e88010;
        locals.var_xmp_dn0 = assign56550_e88010_d_n0;
        locals.var_xmp_dn2 = assign56550_e88010_d_n2;
        locals.var_xmp_dn4 = assign56550_e88010_d_n4;
        locals.var_xmp_dn5 = assign56550_e88010_d_n5;
        locals.var_xmp_dn6 = assign56550_e88010_d_n6;
        locals.var_xmp_dn7 = assign56550_e88010_d_n7;
        locals.var_xmp_dn8 = assign56550_e88010_d_n8;
        locals.var_xmp_dn9 = assign56550_e88010_d_n9;
        locals.var_xmp_dn10 = assign56550_e88010_d_n10;
        locals.var_xmp_dn11 = assign56550_e88010_d_n11;
        locals.var_xmp_dn14 = assign56550_e88010_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56560_e88023,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56560_e88023;
        locals.var_m0_rv = 0.0;

        let (assign56570_e88036,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56570_e88036;
        locals.var_mm_rv = 0.0;

        let (assign56580_e88049, assign56580_e88049_d_n0, assign56580_e88049_d_n2, assign56580_e88049_d_n4, assign56580_e88049_d_n5, assign56580_e88049_d_n6, assign56580_e88049_d_n7, assign56580_e88049_d_n8, assign56580_e88049_d_n9, assign56580_e88049_d_n10, assign56580_e88049_d_n11, assign56580_e88049_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign56580_e88049;
        locals.var_arg_dn0 = assign56580_e88049_d_n0;
        locals.var_arg_dn2 = assign56580_e88049_d_n2;
        locals.var_arg_dn4 = assign56580_e88049_d_n4;
        locals.var_arg_dn5 = assign56580_e88049_d_n5;
        locals.var_arg_dn6 = assign56580_e88049_d_n6;
        locals.var_arg_dn7 = assign56580_e88049_d_n7;
        locals.var_arg_dn8 = assign56580_e88049_d_n8;
        locals.var_arg_dn9 = assign56580_e88049_d_n9;
        locals.var_arg_dn10 = assign56580_e88049_d_n10;
        locals.var_arg_dn11 = assign56580_e88049_d_n11;
        locals.var_arg_dn14 = assign56580_e88049_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign56590_e88062, assign56590_e88062_d_n0, assign56590_e88062_d_n2, assign56590_e88062_d_n4, assign56590_e88062_d_n5, assign56590_e88062_d_n6, assign56590_e88062_d_n7, assign56590_e88062_d_n8, assign56590_e88062_d_n9, assign56590_e88062_d_n10, assign56590_e88062_d_n11, assign56590_e88062_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56590_e88062;
        locals.var_dnm_dn0 = assign56590_e88062_d_n0;
        locals.var_dnm_dn2 = assign56590_e88062_d_n2;
        locals.var_dnm_dn4 = assign56590_e88062_d_n4;
        locals.var_dnm_dn5 = assign56590_e88062_d_n5;
        locals.var_dnm_dn6 = assign56590_e88062_d_n6;
        locals.var_dnm_dn7 = assign56590_e88062_d_n7;
        locals.var_dnm_dn8 = assign56590_e88062_d_n8;
        locals.var_dnm_dn9 = assign56590_e88062_d_n9;
        locals.var_dnm_dn10 = assign56590_e88062_d_n10;
        locals.var_dnm_dn11 = assign56590_e88062_d_n11;
        locals.var_dnm_dn14 = assign56590_e88062_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56600_e88077, assign56600_e88077_d_n0, assign56600_e88077_d_n2, assign56600_e88077_d_n4, assign56600_e88077_d_n5, assign56600_e88077_d_n6, assign56600_e88077_d_n7, assign56600_e88077_d_n8, assign56600_e88077_d_n9, assign56600_e88077_d_n10, assign56600_e88077_d_n11, assign56600_e88077_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56600_e88075: f64 = (locals.var_xp * locals.var_x2);
        (assign56600_e88075, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56600_e88077;
        locals.var_xp_dn0 = assign56600_e88077_d_n0;
        locals.var_xp_dn2 = assign56600_e88077_d_n2;
        locals.var_xp_dn4 = assign56600_e88077_d_n4;
        locals.var_xp_dn5 = assign56600_e88077_d_n5;
        locals.var_xp_dn6 = assign56600_e88077_d_n6;
        locals.var_xp_dn7 = assign56600_e88077_d_n7;
        locals.var_xp_dn8 = assign56600_e88077_d_n8;
        locals.var_xp_dn9 = assign56600_e88077_d_n9;
        locals.var_xp_dn10 = assign56600_e88077_d_n10;
        locals.var_xp_dn11 = assign56600_e88077_d_n11;
        locals.var_xp_dn14 = assign56600_e88077_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56610_e88092, assign56610_e88092_d_n0, assign56610_e88092_d_n2, assign56610_e88092_d_n4, assign56610_e88092_d_n5, assign56610_e88092_d_n6, assign56610_e88092_d_n7, assign56610_e88092_d_n8, assign56610_e88092_d_n9, assign56610_e88092_d_n10, assign56610_e88092_d_n11, assign56610_e88092_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56610_e88090: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56610_e88090, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56610_e88092;
        locals.var_xmp_dn0 = assign56610_e88092_d_n0;
        locals.var_xmp_dn2 = assign56610_e88092_d_n2;
        locals.var_xmp_dn4 = assign56610_e88092_d_n4;
        locals.var_xmp_dn5 = assign56610_e88092_d_n5;
        locals.var_xmp_dn6 = assign56610_e88092_d_n6;
        locals.var_xmp_dn7 = assign56610_e88092_d_n7;
        locals.var_xmp_dn8 = assign56610_e88092_d_n8;
        locals.var_xmp_dn9 = assign56610_e88092_d_n9;
        locals.var_xmp_dn10 = assign56610_e88092_d_n10;
        locals.var_xmp_dn11 = assign56610_e88092_d_n11;
        locals.var_xmp_dn14 = assign56610_e88092_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56620_e88107, assign56620_e88107_d_n0, assign56620_e88107_d_n2, assign56620_e88107_d_n4, assign56620_e88107_d_n5, assign56620_e88107_d_n6, assign56620_e88107_d_n7, assign56620_e88107_d_n8, assign56620_e88107_d_n9, assign56620_e88107_d_n10, assign56620_e88107_d_n11, assign56620_e88107_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56620_e88105: f64 = (locals.var_xp * locals.var_x2);
        (assign56620_e88105, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56620_e88107;
        locals.var_xp_dn0 = assign56620_e88107_d_n0;
        locals.var_xp_dn2 = assign56620_e88107_d_n2;
        locals.var_xp_dn4 = assign56620_e88107_d_n4;
        locals.var_xp_dn5 = assign56620_e88107_d_n5;
        locals.var_xp_dn6 = assign56620_e88107_d_n6;
        locals.var_xp_dn7 = assign56620_e88107_d_n7;
        locals.var_xp_dn8 = assign56620_e88107_d_n8;
        locals.var_xp_dn9 = assign56620_e88107_d_n9;
        locals.var_xp_dn10 = assign56620_e88107_d_n10;
        locals.var_xp_dn11 = assign56620_e88107_d_n11;
        locals.var_xp_dn14 = assign56620_e88107_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56630_e88122, assign56630_e88122_d_n0, assign56630_e88122_d_n2, assign56630_e88122_d_n4, assign56630_e88122_d_n5, assign56630_e88122_d_n6, assign56630_e88122_d_n7, assign56630_e88122_d_n8, assign56630_e88122_d_n9, assign56630_e88122_d_n10, assign56630_e88122_d_n11, assign56630_e88122_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56630_e88120: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56630_e88120, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56630_e88122;
        locals.var_xmp_dn0 = assign56630_e88122_d_n0;
        locals.var_xmp_dn2 = assign56630_e88122_d_n2;
        locals.var_xmp_dn4 = assign56630_e88122_d_n4;
        locals.var_xmp_dn5 = assign56630_e88122_d_n5;
        locals.var_xmp_dn6 = assign56630_e88122_d_n6;
        locals.var_xmp_dn7 = assign56630_e88122_d_n7;
        locals.var_xmp_dn8 = assign56630_e88122_d_n8;
        locals.var_xmp_dn9 = assign56630_e88122_d_n9;
        locals.var_xmp_dn10 = assign56630_e88122_d_n10;
        locals.var_xmp_dn11 = assign56630_e88122_d_n11;
        locals.var_xmp_dn14 = assign56630_e88122_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56640_e88137, assign56640_e88137_d_n0, assign56640_e88137_d_n2, assign56640_e88137_d_n4, assign56640_e88137_d_n5, assign56640_e88137_d_n6, assign56640_e88137_d_n7, assign56640_e88137_d_n8, assign56640_e88137_d_n9, assign56640_e88137_d_n10, assign56640_e88137_d_n11, assign56640_e88137_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56640_e88135: f64 = (locals.var_xp + locals.var_xmp);
        (assign56640_e88135, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign56640_e88137;
        locals.var_arg_dn0 = assign56640_e88137_d_n0;
        locals.var_arg_dn2 = assign56640_e88137_d_n2;
        locals.var_arg_dn4 = assign56640_e88137_d_n4;
        locals.var_arg_dn5 = assign56640_e88137_d_n5;
        locals.var_arg_dn6 = assign56640_e88137_d_n6;
        locals.var_arg_dn7 = assign56640_e88137_d_n7;
        locals.var_arg_dn8 = assign56640_e88137_d_n8;
        locals.var_arg_dn9 = assign56640_e88137_d_n9;
        locals.var_arg_dn10 = assign56640_e88137_d_n10;
        locals.var_arg_dn11 = assign56640_e88137_d_n11;
        locals.var_arg_dn14 = assign56640_e88137_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign56650_e88150, assign56650_e88150_d_n0, assign56650_e88150_d_n2, assign56650_e88150_d_n4, assign56650_e88150_d_n5, assign56650_e88150_d_n6, assign56650_e88150_d_n7, assign56650_e88150_d_n8, assign56650_e88150_d_n9, assign56650_e88150_d_n10, assign56650_e88150_d_n11, assign56650_e88150_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56650_e88150;
        locals.var_dnm_dn0 = assign56650_e88150_d_n0;
        locals.var_dnm_dn2 = assign56650_e88150_d_n2;
        locals.var_dnm_dn4 = assign56650_e88150_d_n4;
        locals.var_dnm_dn5 = assign56650_e88150_d_n5;
        locals.var_dnm_dn6 = assign56650_e88150_d_n6;
        locals.var_dnm_dn7 = assign56650_e88150_d_n7;
        locals.var_dnm_dn8 = assign56650_e88150_d_n8;
        locals.var_dnm_dn9 = assign56650_e88150_d_n9;
        locals.var_dnm_dn10 = assign56650_e88150_d_n10;
        locals.var_dnm_dn11 = assign56650_e88150_d_n11;
        locals.var_dnm_dn14 = assign56650_e88150_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign56660_e88165: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1418 = assign56660_e88165;
        locals.var_guard1418_rv = 0.0;

        let assign56670_e88168: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1419 = assign56670_e88168;
        locals.var_guard1419_rv = 0.0;

        let (assign56680_e88185,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 != 0.0)) && (locals.var_guard1419 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56680_e88185;
        locals.var_mm_rv = 0.0;

        let assign56690_e88188: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1420 = assign56690_e88188;
        locals.var_guard1420_rv = 0.0;

        let (assign56700_e88208,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 != 0.0)) && (locals.var_guard1419 == 0.0)) && (locals.var_guard1420 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56700_e88208;
        locals.var_mm_rv = 0.0;

        let assign56710_e88211: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1421 = assign56710_e88211;
        locals.var_guard1421_rv = 0.0;

        let (assign56720_e88234,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 != 0.0)) && (locals.var_guard1419 == 0.0)) && (locals.var_guard1420 == 0.0)) && (locals.var_guard1421 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56720_e88234;
        locals.var_mm_rv = 0.0;

        let assign56730_e88237: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1422 = assign56730_e88237;
        locals.var_guard1422_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_207(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56740_e88263,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 != 0.0)) && (locals.var_guard1419 == 0.0)) && (locals.var_guard1420 == 0.0)) && (locals.var_guard1421 == 0.0)) && (locals.var_guard1422 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56740_e88263;
        locals.var_mm_rv = 0.0;

        let (assign56750_e88278,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56750_e88278;
        locals.var_m0_rv = 0.0;

        let mut assign56760_loop_guard: usize = 0;
        while {
            let assign56760_cond_e88294: f64 = if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign56760_cond_e88294 != 0.0
        } {
            assign56760_loop_guard += 1;
            assert!(assign56760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign56760_body0_e88310, assign56760_body0_e88310_d_n0, assign56760_body0_e88310_d_n2, assign56760_body0_e88310_d_n4, assign56760_body0_e88310_d_n5, assign56760_body0_e88310_d_n6, assign56760_body0_e88310_d_n7, assign56760_body0_e88310_d_n8, assign56760_body0_e88310_d_n9, assign56760_body0_e88310_d_n10, assign56760_body0_e88310_d_n11, assign56760_body0_e88310_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 != 0.0)) {
        let assign56760_body0_e88308: f64 = (locals.var_dnm).sqrt();
        (assign56760_body0_e88308, (locals.var_dnm_dn0 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn2 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn4 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn5 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn6 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn7 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn8 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn9 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn10 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn11 / (2.0 * assign56760_body0_e88308)), (locals.var_dnm_dn14 / (2.0 * assign56760_body0_e88308)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign56760_body0_e88310;
            locals.var_dnm_dn0 = assign56760_body0_e88310_d_n0;
            locals.var_dnm_dn2 = assign56760_body0_e88310_d_n2;
            locals.var_dnm_dn4 = assign56760_body0_e88310_d_n4;
            locals.var_dnm_dn5 = assign56760_body0_e88310_d_n5;
            locals.var_dnm_dn6 = assign56760_body0_e88310_d_n6;
            locals.var_dnm_dn7 = assign56760_body0_e88310_d_n7;
            locals.var_dnm_dn8 = assign56760_body0_e88310_d_n8;
            locals.var_dnm_dn9 = assign56760_body0_e88310_d_n9;
            locals.var_dnm_dn10 = assign56760_body0_e88310_d_n10;
            locals.var_dnm_dn11 = assign56760_body0_e88310_d_n11;
            locals.var_dnm_dn14 = assign56760_body0_e88310_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign56760_body1_e88327,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 != 0.0)) {
        let assign56760_body1_e88325: f64 = (locals.var_m0 + 1.0);
        (assign56760_body1_e88325,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign56760_body1_e88327;
            locals.var_m0_rv = 0.0;
        }

        let (assign56770_e88354, assign56770_e88354_d_n0, assign56770_e88354_d_n2, assign56770_e88354_d_n4, assign56770_e88354_d_n5, assign56770_e88354_d_n6, assign56770_e88354_d_n7, assign56770_e88354_d_n8, assign56770_e88354_d_n9, assign56770_e88354_d_n10, assign56770_e88354_d_n11, assign56770_e88354_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) && (locals.var_guard1418 == 0.0)) {
        let (assign56770_e88352, assign56770_e88352_d_n0, assign56770_e88352_d_n2, assign56770_e88352_d_n4, assign56770_e88352_d_n5, assign56770_e88352_d_n6, assign56770_e88352_d_n7, assign56770_e88352_d_n8, assign56770_e88352_d_n9, assign56770_e88352_d_n10, assign56770_e88352_d_n11, assign56770_e88352_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56770_e88349: f64 = (2.0 * 2.0);
                let assign56770_e88350: f64 = (1.0 / assign56770_e88349);
                let assign56770_e88351: f64 = (locals.var_dnm).powf(assign56770_e88350);
                (assign56770_e88351, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn0)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn2)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn4)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn5)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn6)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn7)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn8)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn9)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn10)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn11)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56770_e88350) as f64).is_finite() && ((assign56770_e88350) as f64).fract() == 0.0 { if assign56770_e88350 == 0.0 { 0.0 } else { (assign56770_e88350 * ((locals.var_dnm).powf(assign56770_e88350 - 1.0) * locals.var_dnm_dn14)) } } else { (assign56770_e88351 * (assign56770_e88350 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign56770_e88352, assign56770_e88352_d_n0, assign56770_e88352_d_n2, assign56770_e88352_d_n4, assign56770_e88352_d_n5, assign56770_e88352_d_n6, assign56770_e88352_d_n7, assign56770_e88352_d_n8, assign56770_e88352_d_n9, assign56770_e88352_d_n10, assign56770_e88352_d_n11, assign56770_e88352_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56770_e88354;
        locals.var_dnm_dn0 = assign56770_e88354_d_n0;
        locals.var_dnm_dn2 = assign56770_e88354_d_n2;
        locals.var_dnm_dn4 = assign56770_e88354_d_n4;
        locals.var_dnm_dn5 = assign56770_e88354_d_n5;
        locals.var_dnm_dn6 = assign56770_e88354_d_n6;
        locals.var_dnm_dn7 = assign56770_e88354_d_n7;
        locals.var_dnm_dn8 = assign56770_e88354_d_n8;
        locals.var_dnm_dn9 = assign56770_e88354_d_n9;
        locals.var_dnm_dn10 = assign56770_e88354_d_n10;
        locals.var_dnm_dn11 = assign56770_e88354_d_n11;
        locals.var_dnm_dn14 = assign56770_e88354_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56780_e88369, assign56780_e88369_d_n0, assign56780_e88369_d_n2, assign56780_e88369_d_n4, assign56780_e88369_d_n5, assign56780_e88369_d_n6, assign56780_e88369_d_n7, assign56780_e88369_d_n8, assign56780_e88369_d_n9, assign56780_e88369_d_n10, assign56780_e88369_d_n11, assign56780_e88369_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56780_e88367: f64 = (1.0 / locals.var_dnm);
        (assign56780_e88367, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56780_e88369;
        locals.var_dnm_dn0 = assign56780_e88369_d_n0;
        locals.var_dnm_dn2 = assign56780_e88369_d_n2;
        locals.var_dnm_dn4 = assign56780_e88369_d_n4;
        locals.var_dnm_dn5 = assign56780_e88369_d_n5;
        locals.var_dnm_dn6 = assign56780_e88369_d_n6;
        locals.var_dnm_dn7 = assign56780_e88369_d_n7;
        locals.var_dnm_dn8 = assign56780_e88369_d_n8;
        locals.var_dnm_dn9 = assign56780_e88369_d_n9;
        locals.var_dnm_dn10 = assign56780_e88369_d_n10;
        locals.var_dnm_dn11 = assign56780_e88369_d_n11;
        locals.var_dnm_dn14 = assign56780_e88369_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56790_e88388, assign56790_e88388_d_n0, assign56790_e88388_d_n2, assign56790_e88388_d_n4, assign56790_e88388_d_n5, assign56790_e88388_d_n6, assign56790_e88388_d_n7, assign56790_e88388_d_n8, assign56790_e88388_d_n9, assign56790_e88388_d_n10, assign56790_e88388_d_n11, assign56790_e88388_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56790_e88383: f64 = (locals.var_uc_depleak * 0.5);
        let assign56790_e88384: f64 = (locals.var_tmf1 * assign56790_e88383);
        let assign56790_e88386: f64 = (assign56790_e88384 * locals.var_dnm);
        (assign56790_e88386, ((((locals.var_tmf1_dn0 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn0 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn2 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn4 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn5 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn6 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn7 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn8 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn9 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn10 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn11 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign56790_e88383) + (locals.var_tmf1 * (locals.var_uc_depleak_dn14 * 0.5))) * locals.var_dnm) + (assign56790_e88384 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign56790_e88388;
        locals.var_tmf0_dn0 = assign56790_e88388_d_n0;
        locals.var_tmf0_dn2 = assign56790_e88388_d_n2;
        locals.var_tmf0_dn4 = assign56790_e88388_d_n4;
        locals.var_tmf0_dn5 = assign56790_e88388_d_n5;
        locals.var_tmf0_dn6 = assign56790_e88388_d_n6;
        locals.var_tmf0_dn7 = assign56790_e88388_d_n7;
        locals.var_tmf0_dn8 = assign56790_e88388_d_n8;
        locals.var_tmf0_dn9 = assign56790_e88388_d_n9;
        locals.var_tmf0_dn10 = assign56790_e88388_d_n10;
        locals.var_tmf0_dn11 = assign56790_e88388_d_n11;
        locals.var_tmf0_dn14 = assign56790_e88388_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign56800_e88409, assign56800_e88409_d_n0, assign56800_e88409_d_n2, assign56800_e88409_d_n4, assign56800_e88409_d_n5, assign56800_e88409_d_n6, assign56800_e88409_d_n7, assign56800_e88409_d_n8, assign56800_e88409_d_n9, assign56800_e88409_d_n10, assign56800_e88409_d_n11, assign56800_e88409_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56800_e88401: f64 = (locals.var_uc_depleak * 0.5);
        let assign56800_e88403: f64 = (assign56800_e88401 * locals.var_xmp);
        let assign56800_e88405: f64 = (assign56800_e88403 * locals.var_dnm);
        let assign56800_e88407: f64 = (assign56800_e88405 / locals.var_arg);
        (assign56800_e88407, ((((((((locals.var_uc_depleak_dn0 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn0)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn2 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn2)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn4 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn4)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn5 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn5)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn6 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn6)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn7 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn7)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn8 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn8)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn9 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn9)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn10 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn10)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn11 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn11)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn14 * 0.5) * locals.var_xmp) + (assign56800_e88401 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign56800_e88403 * locals.var_dnm_dn14)) * locals.var_arg) - (assign56800_e88405 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56800_e88409;
        locals.var_t0_dn0 = assign56800_e88409_d_n0;
        locals.var_t0_dn2 = assign56800_e88409_d_n2;
        locals.var_t0_dn4 = assign56800_e88409_d_n4;
        locals.var_t0_dn5 = assign56800_e88409_d_n5;
        locals.var_t0_dn6 = assign56800_e88409_d_n6;
        locals.var_t0_dn7 = assign56800_e88409_d_n7;
        locals.var_t0_dn8 = assign56800_e88409_d_n8;
        locals.var_t0_dn9 = assign56800_e88409_d_n9;
        locals.var_t0_dn10 = assign56800_e88409_d_n10;
        locals.var_t0_dn11 = assign56800_e88409_d_n11;
        locals.var_t0_dn14 = assign56800_e88409_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56810_e88428, assign56810_e88428_d_n0, assign56810_e88428_d_n2, assign56810_e88428_d_n4, assign56810_e88428_d_n5, assign56810_e88428_d_n6, assign56810_e88428_d_n7, assign56810_e88428_d_n8, assign56810_e88428_d_n9, assign56810_e88428_d_n10, assign56810_e88428_d_n11, assign56810_e88428_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        let assign56810_e88423: f64 = (locals.var_uc_depleak * 0.5);
        let assign56810_e88424: f64 = (locals.var_uc_depleak - assign56810_e88423);
        let assign56810_e88426: f64 = (assign56810_e88424 + locals.var_tmf0);
        (assign56810_e88426, ((locals.var_uc_depleak_dn0 - (locals.var_uc_depleak_dn0 * 0.5)) + locals.var_tmf0_dn0), ((locals.var_uc_depleak_dn2 - (locals.var_uc_depleak_dn2 * 0.5)) + locals.var_tmf0_dn2), ((locals.var_uc_depleak_dn4 - (locals.var_uc_depleak_dn4 * 0.5)) + locals.var_tmf0_dn4), ((locals.var_uc_depleak_dn5 - (locals.var_uc_depleak_dn5 * 0.5)) + locals.var_tmf0_dn5), ((locals.var_uc_depleak_dn6 - (locals.var_uc_depleak_dn6 * 0.5)) + locals.var_tmf0_dn6), ((locals.var_uc_depleak_dn7 - (locals.var_uc_depleak_dn7 * 0.5)) + locals.var_tmf0_dn7), ((locals.var_uc_depleak_dn8 - (locals.var_uc_depleak_dn8 * 0.5)) + locals.var_tmf0_dn8), ((locals.var_uc_depleak_dn9 - (locals.var_uc_depleak_dn9 * 0.5)) + locals.var_tmf0_dn9), ((locals.var_uc_depleak_dn10 - (locals.var_uc_depleak_dn10 * 0.5)) + locals.var_tmf0_dn10), ((locals.var_uc_depleak_dn11 - (locals.var_uc_depleak_dn11 * 0.5)) + locals.var_tmf0_dn11), ((locals.var_uc_depleak_dn14 - (locals.var_uc_depleak_dn14 * 0.5)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_vds_res0, locals.var_vds_res0_dn0, locals.var_vds_res0_dn2, locals.var_vds_res0_dn4, locals.var_vds_res0_dn5, locals.var_vds_res0_dn6, locals.var_vds_res0_dn7, locals.var_vds_res0_dn8, locals.var_vds_res0_dn9, locals.var_vds_res0_dn10, locals.var_vds_res0_dn11, locals.var_vds_res0_dn14,)
    }
};
        locals.var_vds_res0 = assign56810_e88428;
        locals.var_vds_res0_dn0 = assign56810_e88428_d_n0;
        locals.var_vds_res0_dn2 = assign56810_e88428_d_n2;
        locals.var_vds_res0_dn4 = assign56810_e88428_d_n4;
        locals.var_vds_res0_dn5 = assign56810_e88428_d_n5;
        locals.var_vds_res0_dn6 = assign56810_e88428_d_n6;
        locals.var_vds_res0_dn7 = assign56810_e88428_d_n7;
        locals.var_vds_res0_dn8 = assign56810_e88428_d_n8;
        locals.var_vds_res0_dn9 = assign56810_e88428_d_n9;
        locals.var_vds_res0_dn10 = assign56810_e88428_d_n10;
        locals.var_vds_res0_dn11 = assign56810_e88428_d_n11;
        locals.var_vds_res0_dn14 = assign56810_e88428_d_n14;
        locals.var_vds_res0_rv = 0.0;

        let (assign56820_e88441, assign56820_e88441_d_n0, assign56820_e88441_d_n2, assign56820_e88441_d_n4, assign56820_e88441_d_n5, assign56820_e88441_d_n6, assign56820_e88441_d_n7, assign56820_e88441_d_n8, assign56820_e88441_d_n9, assign56820_e88441_d_n10, assign56820_e88441_d_n11, assign56820_e88441_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56820_e88441;
        locals.var_t0_dn0 = assign56820_e88441_d_n0;
        locals.var_t0_dn2 = assign56820_e88441_d_n2;
        locals.var_t0_dn4 = assign56820_e88441_d_n4;
        locals.var_t0_dn5 = assign56820_e88441_d_n5;
        locals.var_t0_dn6 = assign56820_e88441_d_n6;
        locals.var_t0_dn7 = assign56820_e88441_d_n7;
        locals.var_t0_dn8 = assign56820_e88441_d_n8;
        locals.var_t0_dn9 = assign56820_e88441_d_n9;
        locals.var_t0_dn10 = assign56820_e88441_d_n10;
        locals.var_t0_dn11 = assign56820_e88441_d_n11;
        locals.var_t0_dn14 = assign56820_e88441_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56830_e88455, assign56830_e88455_d_n0, assign56830_e88455_d_n2, assign56830_e88455_d_n4, assign56830_e88455_d_n5, assign56830_e88455_d_n6, assign56830_e88455_d_n7, assign56830_e88455_d_n8, assign56830_e88455_d_n9, assign56830_e88455_d_n10, assign56830_e88455_d_n11, assign56830_e88455_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res0, locals.var_vds_res0_dn0, locals.var_vds_res0_dn2, locals.var_vds_res0_dn4, locals.var_vds_res0_dn5, locals.var_vds_res0_dn6, locals.var_vds_res0_dn7, locals.var_vds_res0_dn8, locals.var_vds_res0_dn9, locals.var_vds_res0_dn10, locals.var_vds_res0_dn11, locals.var_vds_res0_dn14,)
    }
};
        locals.var_vds_res0 = assign56830_e88455;
        locals.var_vds_res0_dn0 = assign56830_e88455_d_n0;
        locals.var_vds_res0_dn2 = assign56830_e88455_d_n2;
        locals.var_vds_res0_dn4 = assign56830_e88455_d_n4;
        locals.var_vds_res0_dn5 = assign56830_e88455_d_n5;
        locals.var_vds_res0_dn6 = assign56830_e88455_d_n6;
        locals.var_vds_res0_dn7 = assign56830_e88455_d_n7;
        locals.var_vds_res0_dn8 = assign56830_e88455_d_n8;
        locals.var_vds_res0_dn9 = assign56830_e88455_d_n9;
        locals.var_vds_res0_dn10 = assign56830_e88455_d_n10;
        locals.var_vds_res0_dn11 = assign56830_e88455_d_n11;
        locals.var_vds_res0_dn14 = assign56830_e88455_d_n14;
        locals.var_vds_res0_rv = 0.0;

        let (assign56840_e88469, assign56840_e88469_d_n0, assign56840_e88469_d_n2, assign56840_e88469_d_n4, assign56840_e88469_d_n5, assign56840_e88469_d_n6, assign56840_e88469_d_n7, assign56840_e88469_d_n8, assign56840_e88469_d_n9, assign56840_e88469_d_n10, assign56840_e88469_d_n11, assign56840_e88469_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1417 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56840_e88469;
        locals.var_t0_dn0 = assign56840_e88469_d_n0;
        locals.var_t0_dn2 = assign56840_e88469_d_n2;
        locals.var_t0_dn4 = assign56840_e88469_d_n4;
        locals.var_t0_dn5 = assign56840_e88469_d_n5;
        locals.var_t0_dn6 = assign56840_e88469_d_n6;
        locals.var_t0_dn7 = assign56840_e88469_d_n7;
        locals.var_t0_dn8 = assign56840_e88469_d_n8;
        locals.var_t0_dn9 = assign56840_e88469_d_n9;
        locals.var_t0_dn10 = assign56840_e88469_d_n10;
        locals.var_t0_dn11 = assign56840_e88469_d_n11;
        locals.var_t0_dn14 = assign56840_e88469_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56850_e88484, assign56850_e88484_d_n0, assign56850_e88484_d_n2, assign56850_e88484_d_n4, assign56850_e88484_d_n5, assign56850_e88484_d_n6, assign56850_e88484_d_n7, assign56850_e88484_d_n8, assign56850_e88484_d_n9, assign56850_e88484_d_n10, assign56850_e88484_d_n11, assign56850_e88484_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign56850_e88480: f64 = (locals.var_vds_res / locals.var_t6);
        let assign56850_e88482: f64 = (assign56850_e88480 + locals.var_vds_res0);
        (assign56850_e88482, ((((locals.var_vds_res_dn0 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn0), ((((locals.var_vds_res_dn2 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn2), ((((locals.var_vds_res_dn4 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn4), ((((locals.var_vds_res_dn5 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn5), ((((locals.var_vds_res_dn6 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn6), ((((locals.var_vds_res_dn7 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn7), ((((locals.var_vds_res_dn8 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn8), ((((locals.var_vds_res_dn9 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn9), ((((locals.var_vds_res_dn10 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn10), ((((locals.var_vds_res_dn11 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn11), ((((locals.var_vds_res_dn14 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn14),)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign56850_e88484;
        locals.var_vds_res_dn0 = assign56850_e88484_d_n0;
        locals.var_vds_res_dn2 = assign56850_e88484_d_n2;
        locals.var_vds_res_dn4 = assign56850_e88484_d_n4;
        locals.var_vds_res_dn5 = assign56850_e88484_d_n5;
        locals.var_vds_res_dn6 = assign56850_e88484_d_n6;
        locals.var_vds_res_dn7 = assign56850_e88484_d_n7;
        locals.var_vds_res_dn8 = assign56850_e88484_d_n8;
        locals.var_vds_res_dn9 = assign56850_e88484_d_n9;
        locals.var_vds_res_dn10 = assign56850_e88484_d_n10;
        locals.var_vds_res_dn11 = assign56850_e88484_d_n11;
        locals.var_vds_res_dn14 = assign56850_e88484_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign56860_e88499, assign56860_e88499_d_n0, assign56860_e88499_d_n2, assign56860_e88499_d_n4, assign56860_e88499_d_n5, assign56860_e88499_d_n6, assign56860_e88499_d_n7, assign56860_e88499_d_n8, assign56860_e88499_d_n9, assign56860_e88499_d_n10, assign56860_e88499_d_n11, assign56860_e88499_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign56860_e88495: f64 = (locals.var_vds_res0 * locals.var_vds_res0);
        let assign56860_e88497: f64 = (assign56860_e88495 * locals.var_vds_res0);
        (assign56860_e88497, ((((locals.var_vds_res0_dn0 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn0)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn0)), ((((locals.var_vds_res0_dn2 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn2)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn2)), ((((locals.var_vds_res0_dn4 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn4)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn4)), ((((locals.var_vds_res0_dn5 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn5)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn5)), ((((locals.var_vds_res0_dn6 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn6)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn6)), ((((locals.var_vds_res0_dn7 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn7)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn7)), ((((locals.var_vds_res0_dn8 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn8)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn8)), ((((locals.var_vds_res0_dn9 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn9)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn9)), ((((locals.var_vds_res0_dn10 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn10)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn10)), ((((locals.var_vds_res0_dn11 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn11)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn11)), ((((locals.var_vds_res0_dn14 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn14)) * locals.var_vds_res0) + (assign56860_e88495 * locals.var_vds_res0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign56860_e88499;
        locals.var_t4_dn0 = assign56860_e88499_d_n0;
        locals.var_t4_dn2 = assign56860_e88499_d_n2;
        locals.var_t4_dn4 = assign56860_e88499_d_n4;
        locals.var_t4_dn5 = assign56860_e88499_d_n5;
        locals.var_t4_dn6 = assign56860_e88499_d_n6;
        locals.var_t4_dn7 = assign56860_e88499_d_n7;
        locals.var_t4_dn8 = assign56860_e88499_d_n8;
        locals.var_t4_dn9 = assign56860_e88499_d_n9;
        locals.var_t4_dn10 = assign56860_e88499_d_n10;
        locals.var_t4_dn11 = assign56860_e88499_d_n11;
        locals.var_t4_dn14 = assign56860_e88499_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign56870_e88512, assign56870_e88512_d_n0, assign56870_e88512_d_n2, assign56870_e88512_d_n4, assign56870_e88512_d_n5, assign56870_e88512_d_n6, assign56870_e88512_d_n7, assign56870_e88512_d_n8, assign56870_e88512_d_n9, assign56870_e88512_d_n10, assign56870_e88512_d_n11, assign56870_e88512_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign56870_e88510: f64 = (locals.var_t4 + 0.0001);
        (assign56870_e88510, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56870_e88512;
        locals.var_t0_dn0 = assign56870_e88512_d_n0;
        locals.var_t0_dn2 = assign56870_e88512_d_n2;
        locals.var_t0_dn4 = assign56870_e88512_d_n4;
        locals.var_t0_dn5 = assign56870_e88512_d_n5;
        locals.var_t0_dn6 = assign56870_e88512_d_n6;
        locals.var_t0_dn7 = assign56870_e88512_d_n7;
        locals.var_t0_dn8 = assign56870_e88512_d_n8;
        locals.var_t0_dn9 = assign56870_e88512_d_n9;
        locals.var_t0_dn10 = assign56870_e88512_d_n10;
        locals.var_t0_dn11 = assign56870_e88512_d_n11;
        locals.var_t0_dn14 = assign56870_e88512_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56880_e88525, assign56880_e88525_d_n0, assign56880_e88525_d_n2, assign56880_e88525_d_n4, assign56880_e88525_d_n5, assign56880_e88525_d_n6, assign56880_e88525_d_n7, assign56880_e88525_d_n8, assign56880_e88525_d_n9, assign56880_e88525_d_n10, assign56880_e88525_d_n11, assign56880_e88525_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign56880_e88523: f64 = (locals.var_t4 / locals.var_t0);
        (assign56880_e88523, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn11 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn14 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_vds_res0_sym, locals.var_vds_res0_sym_dn0, locals.var_vds_res0_sym_dn2, locals.var_vds_res0_sym_dn4, locals.var_vds_res0_sym_dn5, locals.var_vds_res0_sym_dn6, locals.var_vds_res0_sym_dn7, locals.var_vds_res0_sym_dn8, locals.var_vds_res0_sym_dn9, locals.var_vds_res0_sym_dn10, locals.var_vds_res0_sym_dn11, locals.var_vds_res0_sym_dn14,)
    }
};
        locals.var_vds_res0_sym = assign56880_e88525;
        locals.var_vds_res0_sym_dn0 = assign56880_e88525_d_n0;
        locals.var_vds_res0_sym_dn2 = assign56880_e88525_d_n2;
        locals.var_vds_res0_sym_dn4 = assign56880_e88525_d_n4;
        locals.var_vds_res0_sym_dn5 = assign56880_e88525_d_n5;
        locals.var_vds_res0_sym_dn6 = assign56880_e88525_d_n6;
        locals.var_vds_res0_sym_dn7 = assign56880_e88525_d_n7;
        locals.var_vds_res0_sym_dn8 = assign56880_e88525_d_n8;
        locals.var_vds_res0_sym_dn9 = assign56880_e88525_d_n9;
        locals.var_vds_res0_sym_dn10 = assign56880_e88525_d_n10;
        locals.var_vds_res0_sym_dn11 = assign56880_e88525_d_n11;
        locals.var_vds_res0_sym_dn14 = assign56880_e88525_d_n14;
        locals.var_vds_res0_sym_rv = 0.0;

        let assign56890_e88528: f64 = (-1.0);
        let assign56890_e88529: f64 = if p.p43 == assign56890_e88528 { 1.0 } else { 0.0 };
        locals.var_guard1423 = assign56890_e88529;
        locals.var_guard1423_rv = 0.0;

        let (assign56900_e88542, assign56900_e88542_d_n0, assign56900_e88542_d_n2, assign56900_e88542_d_n4, assign56900_e88542_d_n5, assign56900_e88542_d_n6, assign56900_e88542_d_n7, assign56900_e88542_d_n8, assign56900_e88542_d_n9, assign56900_e88542_d_n10, assign56900_e88542_d_n11, assign56900_e88542_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_res0_sym, locals.var_vds_res0_sym_dn0, locals.var_vds_res0_sym_dn2, locals.var_vds_res0_sym_dn4, locals.var_vds_res0_sym_dn5, locals.var_vds_res0_sym_dn6, locals.var_vds_res0_sym_dn7, locals.var_vds_res0_sym_dn8, locals.var_vds_res0_sym_dn9, locals.var_vds_res0_sym_dn10, locals.var_vds_res0_sym_dn11, locals.var_vds_res0_sym_dn14,)
    }
};
        locals.var_vds_res0_sym = assign56900_e88542;
        locals.var_vds_res0_sym_dn0 = assign56900_e88542_d_n0;
        locals.var_vds_res0_sym_dn2 = assign56900_e88542_d_n2;
        locals.var_vds_res0_sym_dn4 = assign56900_e88542_d_n4;
        locals.var_vds_res0_sym_dn5 = assign56900_e88542_d_n5;
        locals.var_vds_res0_sym_dn6 = assign56900_e88542_d_n6;
        locals.var_vds_res0_sym_dn7 = assign56900_e88542_d_n7;
        locals.var_vds_res0_sym_dn8 = assign56900_e88542_d_n8;
        locals.var_vds_res0_sym_dn9 = assign56900_e88542_d_n9;
        locals.var_vds_res0_sym_dn10 = assign56900_e88542_d_n10;
        locals.var_vds_res0_sym_dn11 = assign56900_e88542_d_n11;
        locals.var_vds_res0_sym_dn14 = assign56900_e88542_d_n14;
        locals.var_vds_res0_sym_rv = 0.0;

        let (assign56910_e88555, assign56910_e88555_d_n0, assign56910_e88555_d_n2, assign56910_e88555_d_n4, assign56910_e88555_d_n5, assign56910_e88555_d_n6, assign56910_e88555_d_n7, assign56910_e88555_d_n8, assign56910_e88555_d_n9, assign56910_e88555_d_n10, assign56910_e88555_d_n11, assign56910_e88555_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign56910_e88555;
        locals.var_vds_res_dn0 = assign56910_e88555_d_n0;
        locals.var_vds_res_dn2 = assign56910_e88555_d_n2;
        locals.var_vds_res_dn4 = assign56910_e88555_d_n4;
        locals.var_vds_res_dn5 = assign56910_e88555_d_n5;
        locals.var_vds_res_dn6 = assign56910_e88555_d_n6;
        locals.var_vds_res_dn7 = assign56910_e88555_d_n7;
        locals.var_vds_res_dn8 = assign56910_e88555_d_n8;
        locals.var_vds_res_dn9 = assign56910_e88555_d_n9;
        locals.var_vds_res_dn10 = assign56910_e88555_d_n10;
        locals.var_vds_res_dn11 = assign56910_e88555_d_n11;
        locals.var_vds_res_dn14 = assign56910_e88555_d_n14;
        locals.var_vds_res_rv = 0.0;

        let assign56920_e88558: f64 = if p.p43 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1424 = assign56920_e88558;
        locals.var_guard1424_rv = 0.0;

        let (assign56930_e88574, assign56930_e88574_d_n0, assign56930_e88574_d_n2, assign56930_e88574_d_n4, assign56930_e88574_d_n5, assign56930_e88574_d_n6, assign56930_e88574_d_n7, assign56930_e88574_d_n8, assign56930_e88574_d_n9, assign56930_e88574_d_n10, assign56930_e88574_d_n11, assign56930_e88574_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) && (locals.var_guard1424 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign56930_e88574;
        locals.var_vds_res_dn0 = assign56930_e88574_d_n0;
        locals.var_vds_res_dn2 = assign56930_e88574_d_n2;
        locals.var_vds_res_dn4 = assign56930_e88574_d_n4;
        locals.var_vds_res_dn5 = assign56930_e88574_d_n5;
        locals.var_vds_res_dn6 = assign56930_e88574_d_n6;
        locals.var_vds_res_dn7 = assign56930_e88574_d_n7;
        locals.var_vds_res_dn8 = assign56930_e88574_d_n8;
        locals.var_vds_res_dn9 = assign56930_e88574_d_n9;
        locals.var_vds_res_dn10 = assign56930_e88574_d_n10;
        locals.var_vds_res_dn11 = assign56930_e88574_d_n11;
        locals.var_vds_res_dn14 = assign56930_e88574_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign56940_e88590, assign56940_e88590_d_n0, assign56940_e88590_d_n2, assign56940_e88590_d_n4, assign56940_e88590_d_n5, assign56940_e88590_d_n6, assign56940_e88590_d_n7, assign56940_e88590_d_n8, assign56940_e88590_d_n9, assign56940_e88590_d_n10, assign56940_e88590_d_n11, assign56940_e88590_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) && (locals.var_guard1424 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_res0, locals.var_vds_res0_dn0, locals.var_vds_res0_dn2, locals.var_vds_res0_dn4, locals.var_vds_res0_dn5, locals.var_vds_res0_dn6, locals.var_vds_res0_dn7, locals.var_vds_res0_dn8, locals.var_vds_res0_dn9, locals.var_vds_res0_dn10, locals.var_vds_res0_dn11, locals.var_vds_res0_dn14,)
    }
};
        locals.var_vds_res0 = assign56940_e88590;
        locals.var_vds_res0_dn0 = assign56940_e88590_d_n0;
        locals.var_vds_res0_dn2 = assign56940_e88590_d_n2;
        locals.var_vds_res0_dn4 = assign56940_e88590_d_n4;
        locals.var_vds_res0_dn5 = assign56940_e88590_d_n5;
        locals.var_vds_res0_dn6 = assign56940_e88590_d_n6;
        locals.var_vds_res0_dn7 = assign56940_e88590_d_n7;
        locals.var_vds_res0_dn8 = assign56940_e88590_d_n8;
        locals.var_vds_res0_dn9 = assign56940_e88590_d_n9;
        locals.var_vds_res0_dn10 = assign56940_e88590_d_n10;
        locals.var_vds_res0_dn11 = assign56940_e88590_d_n11;
        locals.var_vds_res0_dn14 = assign56940_e88590_d_n14;
        locals.var_vds_res0_rv = 0.0;

        let (assign56950_e88606, assign56950_e88606_d_n0, assign56950_e88606_d_n2, assign56950_e88606_d_n4, assign56950_e88606_d_n5, assign56950_e88606_d_n6, assign56950_e88606_d_n7, assign56950_e88606_d_n8, assign56950_e88606_d_n9, assign56950_e88606_d_n10, assign56950_e88606_d_n11, assign56950_e88606_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) && (locals.var_guard1424 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_res0_sym, locals.var_vds_res0_sym_dn0, locals.var_vds_res0_sym_dn2, locals.var_vds_res0_sym_dn4, locals.var_vds_res0_sym_dn5, locals.var_vds_res0_sym_dn6, locals.var_vds_res0_sym_dn7, locals.var_vds_res0_sym_dn8, locals.var_vds_res0_sym_dn9, locals.var_vds_res0_sym_dn10, locals.var_vds_res0_sym_dn11, locals.var_vds_res0_sym_dn14,)
    }
};
        locals.var_vds_res0_sym = assign56950_e88606;
        locals.var_vds_res0_sym_dn0 = assign56950_e88606_d_n0;
        locals.var_vds_res0_sym_dn2 = assign56950_e88606_d_n2;
        locals.var_vds_res0_sym_dn4 = assign56950_e88606_d_n4;
        locals.var_vds_res0_sym_dn5 = assign56950_e88606_d_n5;
        locals.var_vds_res0_sym_dn6 = assign56950_e88606_d_n6;
        locals.var_vds_res0_sym_dn7 = assign56950_e88606_d_n7;
        locals.var_vds_res0_sym_dn8 = assign56950_e88606_d_n8;
        locals.var_vds_res0_sym_dn9 = assign56950_e88606_d_n9;
        locals.var_vds_res0_sym_dn10 = assign56950_e88606_d_n10;
        locals.var_vds_res0_sym_dn11 = assign56950_e88606_d_n11;
        locals.var_vds_res0_sym_dn14 = assign56950_e88606_d_n14;
        locals.var_vds_res0_sym_rv = 0.0;

        let (assign56960_e88624, assign56960_e88624_d_n0, assign56960_e88624_d_n2, assign56960_e88624_d_n4, assign56960_e88624_d_n5, assign56960_e88624_d_n6, assign56960_e88624_d_n7, assign56960_e88624_d_n8, assign56960_e88624_d_n9, assign56960_e88624_d_n10, assign56960_e88624_d_n11, assign56960_e88624_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) && (locals.var_guard1424 != 0.0)) {
        let assign56960_e88622: f64 = (locals.var_vgp_res_raw - locals.var_uc_depleak);
        (assign56960_e88622, (locals.var_vgp_res_raw_dn0 - locals.var_uc_depleak_dn0), (locals.var_vgp_res_raw_dn2 - locals.var_uc_depleak_dn2), (locals.var_vgp_res_raw_dn4 - locals.var_uc_depleak_dn4), (locals.var_vgp_res_raw_dn5 - locals.var_uc_depleak_dn5), (locals.var_vgp_res_raw_dn6 - locals.var_uc_depleak_dn6), (locals.var_vgp_res_raw_dn7 - locals.var_uc_depleak_dn7), (locals.var_vgp_res_raw_dn8 - locals.var_uc_depleak_dn8), (locals.var_vgp_res_raw_dn9 - locals.var_uc_depleak_dn9), (locals.var_vgp_res_raw_dn10 - locals.var_uc_depleak_dn10), (locals.var_vgp_res_raw_dn11 - locals.var_uc_depleak_dn11), (locals.var_vgp_res_raw_dn14 - locals.var_uc_depleak_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign56960_e88624;
        locals.var_t1_dn0 = assign56960_e88624_d_n0;
        locals.var_t1_dn2 = assign56960_e88624_d_n2;
        locals.var_t1_dn4 = assign56960_e88624_d_n4;
        locals.var_t1_dn5 = assign56960_e88624_d_n5;
        locals.var_t1_dn6 = assign56960_e88624_d_n6;
        locals.var_t1_dn7 = assign56960_e88624_d_n7;
        locals.var_t1_dn8 = assign56960_e88624_d_n8;
        locals.var_t1_dn9 = assign56960_e88624_d_n9;
        locals.var_t1_dn10 = assign56960_e88624_d_n10;
        locals.var_t1_dn11 = assign56960_e88624_d_n11;
        locals.var_t1_dn14 = assign56960_e88624_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign56970_e88651, assign56970_e88651_d_n0, assign56970_e88651_d_n2, assign56970_e88651_d_n4, assign56970_e88651_d_n5, assign56970_e88651_d_n6, assign56970_e88651_d_n7, assign56970_e88651_d_n8, assign56970_e88651_d_n9, assign56970_e88651_d_n10, assign56970_e88651_d_n11, assign56970_e88651_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) && (locals.var_guard1424 != 0.0)) {
        let assign56970_e88641: f64 = (locals.var_t1).cosh();
        let assign56970_e88642: f64 = (assign56970_e88641).ln();
        let assign56970_e88643: f64 = (locals.var_t1 + assign56970_e88642);
        let assign56970_e88645: f64 = (2.0_f64).ln();
        let assign56970_e88646: f64 = (assign56970_e88643 + assign56970_e88645);
        let assign56970_e88647: f64 = (0.5 * assign56970_e88646);
        let assign56970_e88649: f64 = (assign56970_e88647 + locals.var_uc_depleak);
        (assign56970_e88649, ((0.5 * (locals.var_t1_dn0 + (((locals.var_t1).sinh() * locals.var_t1_dn0) / assign56970_e88641))) + locals.var_uc_depleak_dn0), ((0.5 * (locals.var_t1_dn2 + (((locals.var_t1).sinh() * locals.var_t1_dn2) / assign56970_e88641))) + locals.var_uc_depleak_dn2), ((0.5 * (locals.var_t1_dn4 + (((locals.var_t1).sinh() * locals.var_t1_dn4) / assign56970_e88641))) + locals.var_uc_depleak_dn4), ((0.5 * (locals.var_t1_dn5 + (((locals.var_t1).sinh() * locals.var_t1_dn5) / assign56970_e88641))) + locals.var_uc_depleak_dn5), ((0.5 * (locals.var_t1_dn6 + (((locals.var_t1).sinh() * locals.var_t1_dn6) / assign56970_e88641))) + locals.var_uc_depleak_dn6), ((0.5 * (locals.var_t1_dn7 + (((locals.var_t1).sinh() * locals.var_t1_dn7) / assign56970_e88641))) + locals.var_uc_depleak_dn7), ((0.5 * (locals.var_t1_dn8 + (((locals.var_t1).sinh() * locals.var_t1_dn8) / assign56970_e88641))) + locals.var_uc_depleak_dn8), ((0.5 * (locals.var_t1_dn9 + (((locals.var_t1).sinh() * locals.var_t1_dn9) / assign56970_e88641))) + locals.var_uc_depleak_dn9), ((0.5 * (locals.var_t1_dn10 + (((locals.var_t1).sinh() * locals.var_t1_dn10) / assign56970_e88641))) + locals.var_uc_depleak_dn10), ((0.5 * (locals.var_t1_dn11 + (((locals.var_t1).sinh() * locals.var_t1_dn11) / assign56970_e88641))) + locals.var_uc_depleak_dn11), ((0.5 * (locals.var_t1_dn14 + (((locals.var_t1).sinh() * locals.var_t1_dn14) / assign56970_e88641))) + locals.var_uc_depleak_dn14),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56970_e88651;
        locals.var_vdssat_res_dn0 = assign56970_e88651_d_n0;
        locals.var_vdssat_res_dn2 = assign56970_e88651_d_n2;
        locals.var_vdssat_res_dn4 = assign56970_e88651_d_n4;
        locals.var_vdssat_res_dn5 = assign56970_e88651_d_n5;
        locals.var_vdssat_res_dn6 = assign56970_e88651_d_n6;
        locals.var_vdssat_res_dn7 = assign56970_e88651_d_n7;
        locals.var_vdssat_res_dn8 = assign56970_e88651_d_n8;
        locals.var_vdssat_res_dn9 = assign56970_e88651_d_n9;
        locals.var_vdssat_res_dn10 = assign56970_e88651_d_n10;
        locals.var_vdssat_res_dn11 = assign56970_e88651_d_n11;
        locals.var_vdssat_res_dn14 = assign56970_e88651_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let assign56980_e88654: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1425 = assign56980_e88654;
        locals.var_guard1425_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_208(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign56990_e88681, assign56990_e88681_d_n0, assign56990_e88681_d_n2, assign56990_e88681_d_n4, assign56990_e88681_d_n5, assign56990_e88681_d_n6, assign56990_e88681_d_n7, assign56990_e88681_d_n8, assign56990_e88681_d_n9, assign56990_e88681_d_n10, assign56990_e88681_d_n11, assign56990_e88681_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) && (locals.var_guard1424 == 0.0)) && (locals.var_guard1425 != 0.0)) {
        let assign56990_e88674: f64 = (locals.var_vgp_res_raw - locals.var_uc_depleak);
        let assign56990_e88675: f64 = (assign56990_e88674).exp();
        let assign56990_e88676: f64 = (1.0 + assign56990_e88675);
        let assign56990_e88677: f64 = (assign56990_e88676).ln();
        let assign56990_e88679: f64 = (assign56990_e88677 + locals.var_uc_depleak);
        (assign56990_e88679, (((assign56990_e88675 * (locals.var_vgp_res_raw_dn0 - locals.var_uc_depleak_dn0)) / assign56990_e88676) + locals.var_uc_depleak_dn0), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn2 - locals.var_uc_depleak_dn2)) / assign56990_e88676) + locals.var_uc_depleak_dn2), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn4 - locals.var_uc_depleak_dn4)) / assign56990_e88676) + locals.var_uc_depleak_dn4), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn5 - locals.var_uc_depleak_dn5)) / assign56990_e88676) + locals.var_uc_depleak_dn5), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn6 - locals.var_uc_depleak_dn6)) / assign56990_e88676) + locals.var_uc_depleak_dn6), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn7 - locals.var_uc_depleak_dn7)) / assign56990_e88676) + locals.var_uc_depleak_dn7), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn8 - locals.var_uc_depleak_dn8)) / assign56990_e88676) + locals.var_uc_depleak_dn8), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn9 - locals.var_uc_depleak_dn9)) / assign56990_e88676) + locals.var_uc_depleak_dn9), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn10 - locals.var_uc_depleak_dn10)) / assign56990_e88676) + locals.var_uc_depleak_dn10), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn11 - locals.var_uc_depleak_dn11)) / assign56990_e88676) + locals.var_uc_depleak_dn11), (((assign56990_e88675 * (locals.var_vgp_res_raw_dn14 - locals.var_uc_depleak_dn14)) / assign56990_e88676) + locals.var_uc_depleak_dn14),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56990_e88681;
        locals.var_vdssat_res_dn0 = assign56990_e88681_d_n0;
        locals.var_vdssat_res_dn2 = assign56990_e88681_d_n2;
        locals.var_vdssat_res_dn4 = assign56990_e88681_d_n4;
        locals.var_vdssat_res_dn5 = assign56990_e88681_d_n5;
        locals.var_vdssat_res_dn6 = assign56990_e88681_d_n6;
        locals.var_vdssat_res_dn7 = assign56990_e88681_d_n7;
        locals.var_vdssat_res_dn8 = assign56990_e88681_d_n8;
        locals.var_vdssat_res_dn9 = assign56990_e88681_d_n9;
        locals.var_vdssat_res_dn10 = assign56990_e88681_d_n10;
        locals.var_vdssat_res_dn11 = assign56990_e88681_d_n11;
        locals.var_vdssat_res_dn14 = assign56990_e88681_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let (assign57000_e88697, assign57000_e88697_d_n0, assign57000_e88697_d_n2, assign57000_e88697_d_n4, assign57000_e88697_d_n5, assign57000_e88697_d_n6, assign57000_e88697_d_n7, assign57000_e88697_d_n8, assign57000_e88697_d_n9, assign57000_e88697_d_n10, assign57000_e88697_d_n11, assign57000_e88697_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) {
        let assign57000_e88695: f64 = (locals.var_vds_res / locals.var_vdssat_res);
        (assign57000_e88695, (((locals.var_vds_res_dn0 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn0)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn2 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn2)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn4 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn4)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn5 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn5)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn6 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn6)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn7 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn7)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn8 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn8)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn9 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn9)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn10 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn10)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn11 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn11)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn14 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn14)) / (locals.var_vdssat_res * locals.var_vdssat_res)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57000_e88697;
        locals.var_t1_dn0 = assign57000_e88697_d_n0;
        locals.var_t1_dn2 = assign57000_e88697_d_n2;
        locals.var_t1_dn4 = assign57000_e88697_d_n4;
        locals.var_t1_dn5 = assign57000_e88697_d_n5;
        locals.var_t1_dn6 = assign57000_e88697_d_n6;
        locals.var_t1_dn7 = assign57000_e88697_d_n7;
        locals.var_t1_dn8 = assign57000_e88697_d_n8;
        locals.var_t1_dn9 = assign57000_e88697_d_n9;
        locals.var_t1_dn10 = assign57000_e88697_d_n10;
        locals.var_t1_dn11 = assign57000_e88697_d_n11;
        locals.var_t1_dn14 = assign57000_e88697_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57010_e88720, assign57010_e88720_d_n0, assign57010_e88720_d_n2, assign57010_e88720_d_n4, assign57010_e88720_d_n5, assign57010_e88720_d_n6, assign57010_e88720_d_n7, assign57010_e88720_d_n8, assign57010_e88720_d_n9, assign57010_e88720_d_n10, assign57010_e88720_d_n11, assign57010_e88720_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) {
        let (assign57010_e88718, assign57010_e88718_d_n0, assign57010_e88718_d_n2, assign57010_e88718_d_n4, assign57010_e88718_d_n5, assign57010_e88718_d_n6, assign57010_e88718_d_n7, assign57010_e88718_d_n8, assign57010_e88718_d_n9, assign57010_e88718_d_n10, assign57010_e88718_d_n11, assign57010_e88718_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57010_e88716: f64 = (p.p383 - 1.0);
                let assign57010_e88717: f64 = (locals.var_t1).powf(assign57010_e88716);
                (assign57010_e88717, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn0)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn2)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn4)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn5)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn6)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn7)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn8)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn9)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn10)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn11)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign57010_e88716) as f64).is_finite() && ((assign57010_e88716) as f64).fract() == 0.0 { if assign57010_e88716 == 0.0 { 0.0 } else { (assign57010_e88716 * ((locals.var_t1).powf(assign57010_e88716 - 1.0) * locals.var_t1_dn14)) } } else { (assign57010_e88717 * (assign57010_e88716 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign57010_e88718, assign57010_e88718_d_n0, assign57010_e88718_d_n2, assign57010_e88718_d_n4, assign57010_e88718_d_n5, assign57010_e88718_d_n6, assign57010_e88718_d_n7, assign57010_e88718_d_n8, assign57010_e88718_d_n9, assign57010_e88718_d_n10, assign57010_e88718_d_n11, assign57010_e88718_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57010_e88720;
        locals.var_t2_dn0 = assign57010_e88720_d_n0;
        locals.var_t2_dn2 = assign57010_e88720_d_n2;
        locals.var_t2_dn4 = assign57010_e88720_d_n4;
        locals.var_t2_dn5 = assign57010_e88720_d_n5;
        locals.var_t2_dn6 = assign57010_e88720_d_n6;
        locals.var_t2_dn7 = assign57010_e88720_d_n7;
        locals.var_t2_dn8 = assign57010_e88720_d_n8;
        locals.var_t2_dn9 = assign57010_e88720_d_n9;
        locals.var_t2_dn10 = assign57010_e88720_d_n10;
        locals.var_t2_dn11 = assign57010_e88720_d_n11;
        locals.var_t2_dn14 = assign57010_e88720_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57020_e88738, assign57020_e88738_d_n0, assign57020_e88738_d_n2, assign57020_e88738_d_n4, assign57020_e88738_d_n5, assign57020_e88738_d_n6, assign57020_e88738_d_n7, assign57020_e88738_d_n8, assign57020_e88738_d_n9, assign57020_e88738_d_n10, assign57020_e88738_d_n11, assign57020_e88738_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) {
        let assign57020_e88735: f64 = (locals.var_t2 * locals.var_t1);
        let assign57020_e88736: f64 = (1.0 + assign57020_e88735);
        (assign57020_e88736, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign57020_e88738;
        locals.var_t3_dn0 = assign57020_e88738_d_n0;
        locals.var_t3_dn2 = assign57020_e88738_d_n2;
        locals.var_t3_dn4 = assign57020_e88738_d_n4;
        locals.var_t3_dn5 = assign57020_e88738_d_n5;
        locals.var_t3_dn6 = assign57020_e88738_d_n6;
        locals.var_t3_dn7 = assign57020_e88738_d_n7;
        locals.var_t3_dn8 = assign57020_e88738_d_n8;
        locals.var_t3_dn9 = assign57020_e88738_d_n9;
        locals.var_t3_dn10 = assign57020_e88738_d_n10;
        locals.var_t3_dn11 = assign57020_e88738_d_n11;
        locals.var_t3_dn14 = assign57020_e88738_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign57030_e88763, assign57030_e88763_d_n0, assign57030_e88763_d_n2, assign57030_e88763_d_n4, assign57030_e88763_d_n5, assign57030_e88763_d_n6, assign57030_e88763_d_n7, assign57030_e88763_d_n8, assign57030_e88763_d_n9, assign57030_e88763_d_n10, assign57030_e88763_d_n11, assign57030_e88763_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) {
        let (assign57030_e88761, assign57030_e88761_d_n0, assign57030_e88761_d_n2, assign57030_e88761_d_n4, assign57030_e88761_d_n5, assign57030_e88761_d_n6, assign57030_e88761_d_n7, assign57030_e88761_d_n8, assign57030_e88761_d_n9, assign57030_e88761_d_n10, assign57030_e88761_d_n11, assign57030_e88761_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57030_e88757: f64 = (1.0 / p.p383);
                let assign57030_e88759: f64 = (assign57030_e88757 - 1.0);
                let assign57030_e88760: f64 = (locals.var_t3).powf(assign57030_e88759);
                (assign57030_e88760, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn0)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn2)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn4)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn5)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn6)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn7)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn8)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn9)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn10)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn11)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57030_e88759) as f64).is_finite() && ((assign57030_e88759) as f64).fract() == 0.0 { if assign57030_e88759 == 0.0 { 0.0 } else { (assign57030_e88759 * ((locals.var_t3).powf(assign57030_e88759 - 1.0) * locals.var_t3_dn14)) } } else { (assign57030_e88760 * (assign57030_e88759 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign57030_e88761, assign57030_e88761_d_n0, assign57030_e88761_d_n2, assign57030_e88761_d_n4, assign57030_e88761_d_n5, assign57030_e88761_d_n6, assign57030_e88761_d_n7, assign57030_e88761_d_n8, assign57030_e88761_d_n9, assign57030_e88761_d_n10, assign57030_e88761_d_n11, assign57030_e88761_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57030_e88763;
        locals.var_t4_dn0 = assign57030_e88763_d_n0;
        locals.var_t4_dn2 = assign57030_e88763_d_n2;
        locals.var_t4_dn4 = assign57030_e88763_d_n4;
        locals.var_t4_dn5 = assign57030_e88763_d_n5;
        locals.var_t4_dn6 = assign57030_e88763_d_n6;
        locals.var_t4_dn7 = assign57030_e88763_d_n7;
        locals.var_t4_dn8 = assign57030_e88763_d_n8;
        locals.var_t4_dn9 = assign57030_e88763_d_n9;
        locals.var_t4_dn10 = assign57030_e88763_d_n10;
        locals.var_t4_dn11 = assign57030_e88763_d_n11;
        locals.var_t4_dn14 = assign57030_e88763_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57040_e88779, assign57040_e88779_d_n0, assign57040_e88779_d_n2, assign57040_e88779_d_n4, assign57040_e88779_d_n5, assign57040_e88779_d_n6, assign57040_e88779_d_n7, assign57040_e88779_d_n8, assign57040_e88779_d_n9, assign57040_e88779_d_n10, assign57040_e88779_d_n11, assign57040_e88779_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) {
        let assign57040_e88777: f64 = (locals.var_t4 * locals.var_t3);
        (assign57040_e88777, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign57040_e88779;
        locals.var_t6_dn0 = assign57040_e88779_d_n0;
        locals.var_t6_dn2 = assign57040_e88779_d_n2;
        locals.var_t6_dn4 = assign57040_e88779_d_n4;
        locals.var_t6_dn5 = assign57040_e88779_d_n5;
        locals.var_t6_dn6 = assign57040_e88779_d_n6;
        locals.var_t6_dn7 = assign57040_e88779_d_n7;
        locals.var_t6_dn8 = assign57040_e88779_d_n8;
        locals.var_t6_dn9 = assign57040_e88779_d_n9;
        locals.var_t6_dn10 = assign57040_e88779_d_n10;
        locals.var_t6_dn11 = assign57040_e88779_d_n11;
        locals.var_t6_dn14 = assign57040_e88779_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign57050_e88797, assign57050_e88797_d_n0, assign57050_e88797_d_n2, assign57050_e88797_d_n4, assign57050_e88797_d_n5, assign57050_e88797_d_n6, assign57050_e88797_d_n7, assign57050_e88797_d_n8, assign57050_e88797_d_n9, assign57050_e88797_d_n10, assign57050_e88797_d_n11, assign57050_e88797_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1423 == 0.0)) {
        let assign57050_e88793: f64 = (locals.var_vds_res / locals.var_t6);
        let assign57050_e88795: f64 = (assign57050_e88793 + locals.var_vds_res0);
        (assign57050_e88795, ((((locals.var_vds_res_dn0 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn0), ((((locals.var_vds_res_dn2 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn2), ((((locals.var_vds_res_dn4 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn4), ((((locals.var_vds_res_dn5 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn5), ((((locals.var_vds_res_dn6 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn6), ((((locals.var_vds_res_dn7 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn7), ((((locals.var_vds_res_dn8 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn8), ((((locals.var_vds_res_dn9 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn9), ((((locals.var_vds_res_dn10 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn10), ((((locals.var_vds_res_dn11 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn11), ((((locals.var_vds_res_dn14 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn14),)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign57050_e88797;
        locals.var_vds_res_dn0 = assign57050_e88797_d_n0;
        locals.var_vds_res_dn2 = assign57050_e88797_d_n2;
        locals.var_vds_res_dn4 = assign57050_e88797_d_n4;
        locals.var_vds_res_dn5 = assign57050_e88797_d_n5;
        locals.var_vds_res_dn6 = assign57050_e88797_d_n6;
        locals.var_vds_res_dn7 = assign57050_e88797_d_n7;
        locals.var_vds_res_dn8 = assign57050_e88797_d_n8;
        locals.var_vds_res_dn9 = assign57050_e88797_d_n9;
        locals.var_vds_res_dn10 = assign57050_e88797_d_n10;
        locals.var_vds_res_dn11 = assign57050_e88797_d_n11;
        locals.var_vds_res_dn14 = assign57050_e88797_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign57060_e88810, assign57060_e88810_d_n0, assign57060_e88810_d_n2, assign57060_e88810_d_n4, assign57060_e88810_d_n5, assign57060_e88810_d_n6, assign57060_e88810_d_n7, assign57060_e88810_d_n8, assign57060_e88810_d_n9, assign57060_e88810_d_n10, assign57060_e88810_d_n11, assign57060_e88810_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57060_e88808: f64 = (locals.var_w_res * locals.var_q_ndepm__blk1137);
        (assign57060_e88808, ((locals.var_w_res_dn0 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn0)), ((locals.var_w_res_dn2 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn2)), ((locals.var_w_res_dn4 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn4)), ((locals.var_w_res_dn5 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn5)), ((locals.var_w_res_dn6 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn6)), ((locals.var_w_res_dn7 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn7)), ((locals.var_w_res_dn8 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn8)), ((locals.var_w_res_dn9 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn9)), ((locals.var_w_res_dn10 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn10)), ((locals.var_w_res_dn11 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn11)), ((locals.var_w_res_dn14 * locals.var_q_ndepm__blk1137) + (locals.var_w_res * locals.var_q_ndepm__blk1137_dn14)),)
    } else {
        (locals.var_qn_res__blk1128, locals.var_qn_res__blk1128_dn0, locals.var_qn_res__blk1128_dn2, locals.var_qn_res__blk1128_dn4, locals.var_qn_res__blk1128_dn5, locals.var_qn_res__blk1128_dn6, locals.var_qn_res__blk1128_dn7, locals.var_qn_res__blk1128_dn8, locals.var_qn_res__blk1128_dn9, locals.var_qn_res__blk1128_dn10, locals.var_qn_res__blk1128_dn11, locals.var_qn_res__blk1128_dn14,)
    }
};
        locals.var_qn_res__blk1128 = assign57060_e88810;
        locals.var_qn_res__blk1128_dn0 = assign57060_e88810_d_n0;
        locals.var_qn_res__blk1128_dn2 = assign57060_e88810_d_n2;
        locals.var_qn_res__blk1128_dn4 = assign57060_e88810_d_n4;
        locals.var_qn_res__blk1128_dn5 = assign57060_e88810_d_n5;
        locals.var_qn_res__blk1128_dn6 = assign57060_e88810_d_n6;
        locals.var_qn_res__blk1128_dn7 = assign57060_e88810_d_n7;
        locals.var_qn_res__blk1128_dn8 = assign57060_e88810_d_n8;
        locals.var_qn_res__blk1128_dn9 = assign57060_e88810_d_n9;
        locals.var_qn_res__blk1128_dn10 = assign57060_e88810_d_n10;
        locals.var_qn_res__blk1128_dn11 = assign57060_e88810_d_n11;
        locals.var_qn_res__blk1128_dn14 = assign57060_e88810_d_n14;
        locals.var_qn_res__blk1128_rv = 0.0;

        let (assign57070_e88823, assign57070_e88823_d_n0, assign57070_e88823_d_n2, assign57070_e88823_d_n4, assign57070_e88823_d_n5, assign57070_e88823_d_n6, assign57070_e88823_d_n7, assign57070_e88823_d_n8, assign57070_e88823_d_n9, assign57070_e88823_d_n10, assign57070_e88823_d_n11, assign57070_e88823_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57070_e88821: f64 = (1.6021918e-19 * 10000.0);
        (assign57070_e88821, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign57070_e88823;
        locals.var_t9_dn0 = assign57070_e88823_d_n0;
        locals.var_t9_dn2 = assign57070_e88823_d_n2;
        locals.var_t9_dn4 = assign57070_e88823_d_n4;
        locals.var_t9_dn5 = assign57070_e88823_d_n5;
        locals.var_t9_dn6 = assign57070_e88823_d_n6;
        locals.var_t9_dn7 = assign57070_e88823_d_n7;
        locals.var_t9_dn8 = assign57070_e88823_d_n8;
        locals.var_t9_dn9 = assign57070_e88823_d_n9;
        locals.var_t9_dn10 = assign57070_e88823_d_n10;
        locals.var_t9_dn11 = assign57070_e88823_d_n11;
        locals.var_t9_dn14 = assign57070_e88823_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign57080_e88836, assign57080_e88836_d_n0, assign57080_e88836_d_n2, assign57080_e88836_d_n4, assign57080_e88836_d_n5, assign57080_e88836_d_n6, assign57080_e88836_d_n7, assign57080_e88836_d_n8, assign57080_e88836_d_n9, assign57080_e88836_d_n10, assign57080_e88836_d_n11, assign57080_e88836_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57080_e88834: f64 = (locals.var_qn_res__blk1128 / locals.var_t9);
        (assign57080_e88834, (((locals.var_qn_res__blk1128_dn0 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn2 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn4 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn5 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn6 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn7 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn8 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn9 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn10 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn11 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1128_dn14 * locals.var_t9) - (locals.var_qn_res__blk1128 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign57080_e88836;
        locals.var_rns_dn0 = assign57080_e88836_d_n0;
        locals.var_rns_dn2 = assign57080_e88836_d_n2;
        locals.var_rns_dn4 = assign57080_e88836_d_n4;
        locals.var_rns_dn5 = assign57080_e88836_d_n5;
        locals.var_rns_dn6 = assign57080_e88836_d_n6;
        locals.var_rns_dn7 = assign57080_e88836_d_n7;
        locals.var_rns_dn8 = assign57080_e88836_d_n8;
        locals.var_rns_dn9 = assign57080_e88836_d_n9;
        locals.var_rns_dn10 = assign57080_e88836_d_n10;
        locals.var_rns_dn11 = assign57080_e88836_d_n11;
        locals.var_rns_dn14 = assign57080_e88836_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign57090_e88855, assign57090_e88855_d_n0, assign57090_e88855_d_n2, assign57090_e88855_d_n4, assign57090_e88855_d_n5, assign57090_e88855_d_n6, assign57090_e88855_d_n7, assign57090_e88855_d_n8, assign57090_e88855_d_n9, assign57090_e88855_d_n10, assign57090_e88855_d_n11, assign57090_e88855_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57090_e88847: f64 = (locals.var_vds_res * locals.var_vds_res);
        let assign57090_e88849: f64 = (assign57090_e88847 + p.p262);
        let assign57090_e88850: f64 = (assign57090_e88849).sqrt();
        let assign57090_e88852: f64 = (p.p262).sqrt();
        let assign57090_e88853: f64 = (assign57090_e88850 - assign57090_e88852);
        (assign57090_e88853, (((locals.var_vds_res_dn0 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn0)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn2 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn2)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn4 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn4)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn5 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn5)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn6 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn6)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn7 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn7)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn8 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn8)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn9 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn9)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn10 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn10)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn11 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn11)) / (2.0 * assign57090_e88850)), (((locals.var_vds_res_dn14 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn14)) / (2.0 * assign57090_e88850)),)
    } else {
        (locals.var_vds_resz, locals.var_vds_resz_dn0, locals.var_vds_resz_dn2, locals.var_vds_resz_dn4, locals.var_vds_resz_dn5, locals.var_vds_resz_dn6, locals.var_vds_resz_dn7, locals.var_vds_resz_dn8, locals.var_vds_resz_dn9, locals.var_vds_resz_dn10, locals.var_vds_resz_dn11, locals.var_vds_resz_dn14,)
    }
};
        locals.var_vds_resz = assign57090_e88855;
        locals.var_vds_resz_dn0 = assign57090_e88855_d_n0;
        locals.var_vds_resz_dn2 = assign57090_e88855_d_n2;
        locals.var_vds_resz_dn4 = assign57090_e88855_d_n4;
        locals.var_vds_resz_dn5 = assign57090_e88855_d_n5;
        locals.var_vds_resz_dn6 = assign57090_e88855_d_n6;
        locals.var_vds_resz_dn7 = assign57090_e88855_d_n7;
        locals.var_vds_resz_dn8 = assign57090_e88855_d_n8;
        locals.var_vds_resz_dn9 = assign57090_e88855_d_n9;
        locals.var_vds_resz_dn10 = assign57090_e88855_d_n10;
        locals.var_vds_resz_dn11 = assign57090_e88855_d_n11;
        locals.var_vds_resz_dn14 = assign57090_e88855_d_n14;
        locals.var_vds_resz_rv = 0.0;

        let (assign57100_e88870, assign57100_e88870_d_n0, assign57100_e88870_d_n2, assign57100_e88870_d_n4, assign57100_e88870_d_n5, assign57100_e88870_d_n6, assign57100_e88870_d_n7, assign57100_e88870_d_n8, assign57100_e88870_d_n9, assign57100_e88870_d_n10, assign57100_e88870_d_n11, assign57100_e88870_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57100_e88867: f64 = (locals.var_vds_resz * locals.var_ninvdecres);
        let assign57100_e88868: f64 = (1.0 + assign57100_e88867);
        (assign57100_e88868, ((locals.var_vds_resz_dn0 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn0)), ((locals.var_vds_resz_dn2 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn2)), ((locals.var_vds_resz_dn4 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn4)), ((locals.var_vds_resz_dn5 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn5)), ((locals.var_vds_resz_dn6 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn6)), ((locals.var_vds_resz_dn7 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn7)), ((locals.var_vds_resz_dn8 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn8)), ((locals.var_vds_resz_dn9 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn9)), ((locals.var_vds_resz_dn10 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn10)), ((locals.var_vds_resz_dn11 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn11)), ((locals.var_vds_resz_dn14 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57100_e88870;
        locals.var_t4_dn0 = assign57100_e88870_d_n0;
        locals.var_t4_dn2 = assign57100_e88870_d_n2;
        locals.var_t4_dn4 = assign57100_e88870_d_n4;
        locals.var_t4_dn5 = assign57100_e88870_d_n5;
        locals.var_t4_dn6 = assign57100_e88870_d_n6;
        locals.var_t4_dn7 = assign57100_e88870_d_n7;
        locals.var_t4_dn8 = assign57100_e88870_d_n8;
        locals.var_t4_dn9 = assign57100_e88870_d_n9;
        locals.var_t4_dn10 = assign57100_e88870_d_n10;
        locals.var_t4_dn11 = assign57100_e88870_d_n11;
        locals.var_t4_dn14 = assign57100_e88870_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57110_e88885, assign57110_e88885_d_n0, assign57110_e88885_d_n2, assign57110_e88885_d_n4, assign57110_e88885_d_n5, assign57110_e88885_d_n6, assign57110_e88885_d_n7, assign57110_e88885_d_n8, assign57110_e88885_d_n9, assign57110_e88885_d_n10, assign57110_e88885_d_n11, assign57110_e88885_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57110_e88882: f64 = (locals.var_vds_resz * locals.var_ninvdehres);
        let assign57110_e88883: f64 = (1.0 + assign57110_e88882);
        (assign57110_e88883, ((locals.var_vds_resz_dn0 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn0)), ((locals.var_vds_resz_dn2 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn2)), ((locals.var_vds_resz_dn4 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn4)), ((locals.var_vds_resz_dn5 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn5)), ((locals.var_vds_resz_dn6 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn6)), ((locals.var_vds_resz_dn7 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn7)), ((locals.var_vds_resz_dn8 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn8)), ((locals.var_vds_resz_dn9 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn9)), ((locals.var_vds_resz_dn10 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn10)), ((locals.var_vds_resz_dn11 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn11)), ((locals.var_vds_resz_dn14 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign57110_e88885;
        locals.var_t5_dn0 = assign57110_e88885_d_n0;
        locals.var_t5_dn2 = assign57110_e88885_d_n2;
        locals.var_t5_dn4 = assign57110_e88885_d_n4;
        locals.var_t5_dn5 = assign57110_e88885_d_n5;
        locals.var_t5_dn6 = assign57110_e88885_d_n6;
        locals.var_t5_dn7 = assign57110_e88885_d_n7;
        locals.var_t5_dn8 = assign57110_e88885_d_n8;
        locals.var_t5_dn9 = assign57110_e88885_d_n9;
        locals.var_t5_dn10 = assign57110_e88885_d_n10;
        locals.var_t5_dn11 = assign57110_e88885_d_n11;
        locals.var_t5_dn14 = assign57110_e88885_d_n14;
        locals.var_t5_rv = 0.0;

        let assign57120_e88887: f64 = if param_given[408] { 1.0 } else { 0.0 };
        locals.var_guard1426 = assign57120_e88887;
        locals.var_guard1426_rv = 0.0;

        let (assign57130_e88908, assign57130_e88908_d_n0, assign57130_e88908_d_n2, assign57130_e88908_d_n4, assign57130_e88908_d_n5, assign57130_e88908_d_n6, assign57130_e88908_d_n7, assign57130_e88908_d_n8, assign57130_e88908_d_n9, assign57130_e88908_d_n10, assign57130_e88908_d_n11, assign57130_e88908_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1426 != 0.0)) {
        let assign57130_e88900: f64 = (p.p408 - locals.var_phi_b0_dep__blk1096);
        let assign57130_e88903: f64 = (100.0 * locals.var_uc_depthn);
        let assign57130_e88904: f64 = (assign57130_e88900 / assign57130_e88903);
        let assign57130_e88906: f64 = (assign57130_e88904 / locals.var_t5);
        (assign57130_e88906, (((((((-locals.var_phi_b0_dep__blk1096_dn0) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn0))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn2) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn2))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn4) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn4))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn5) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn5))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn6) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn6))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn7) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn7))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn8) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn8))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn9) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn9))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn10) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn10))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn11) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn11))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1096_dn14) * assign57130_e88903) - (assign57130_e88900 * (100.0 * locals.var_uc_depthn_dn14))) / (assign57130_e88903 * assign57130_e88903)) * locals.var_t5) - (assign57130_e88904 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_eeff_res, locals.var_eeff_res_dn0, locals.var_eeff_res_dn2, locals.var_eeff_res_dn4, locals.var_eeff_res_dn5, locals.var_eeff_res_dn6, locals.var_eeff_res_dn7, locals.var_eeff_res_dn8, locals.var_eeff_res_dn9, locals.var_eeff_res_dn10, locals.var_eeff_res_dn11, locals.var_eeff_res_dn14,)
    }
};
        locals.var_eeff_res = assign57130_e88908;
        locals.var_eeff_res_dn0 = assign57130_e88908_d_n0;
        locals.var_eeff_res_dn2 = assign57130_e88908_d_n2;
        locals.var_eeff_res_dn4 = assign57130_e88908_d_n4;
        locals.var_eeff_res_dn5 = assign57130_e88908_d_n5;
        locals.var_eeff_res_dn6 = assign57130_e88908_d_n6;
        locals.var_eeff_res_dn7 = assign57130_e88908_d_n7;
        locals.var_eeff_res_dn8 = assign57130_e88908_d_n8;
        locals.var_eeff_res_dn9 = assign57130_e88908_d_n9;
        locals.var_eeff_res_dn10 = assign57130_e88908_d_n10;
        locals.var_eeff_res_dn11 = assign57130_e88908_d_n11;
        locals.var_eeff_res_dn14 = assign57130_e88908_d_n14;
        locals.var_eeff_res_rv = 0.0;

        let (assign57140_e88926, assign57140_e88926_d_n0, assign57140_e88926_d_n2, assign57140_e88926_d_n4, assign57140_e88926_d_n5, assign57140_e88926_d_n6, assign57140_e88926_d_n7, assign57140_e88926_d_n8, assign57140_e88926_d_n9, assign57140_e88926_d_n10, assign57140_e88926_d_n11, assign57140_e88926_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1426 == 0.0)) {
        let assign57140_e88922: f64 = (locals.var_qn_res__blk1128 / 1.034943e-10);
        let assign57140_e88924: f64 = (assign57140_e88922 / locals.var_t5);
        (assign57140_e88924, ((((locals.var_qn_res__blk1128_dn0 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn2 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn4 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn5 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn6 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn7 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn8 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn9 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn10 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn11 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1128_dn14 / 1.034943e-10) * locals.var_t5) - (assign57140_e88922 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_eeff_res, locals.var_eeff_res_dn0, locals.var_eeff_res_dn2, locals.var_eeff_res_dn4, locals.var_eeff_res_dn5, locals.var_eeff_res_dn6, locals.var_eeff_res_dn7, locals.var_eeff_res_dn8, locals.var_eeff_res_dn9, locals.var_eeff_res_dn10, locals.var_eeff_res_dn11, locals.var_eeff_res_dn14,)
    }
};
        locals.var_eeff_res = assign57140_e88926;
        locals.var_eeff_res_dn0 = assign57140_e88926_d_n0;
        locals.var_eeff_res_dn2 = assign57140_e88926_d_n2;
        locals.var_eeff_res_dn4 = assign57140_e88926_d_n4;
        locals.var_eeff_res_dn5 = assign57140_e88926_d_n5;
        locals.var_eeff_res_dn6 = assign57140_e88926_d_n6;
        locals.var_eeff_res_dn7 = assign57140_e88926_d_n7;
        locals.var_eeff_res_dn8 = assign57140_e88926_d_n8;
        locals.var_eeff_res_dn9 = assign57140_e88926_d_n9;
        locals.var_eeff_res_dn10 = assign57140_e88926_d_n10;
        locals.var_eeff_res_dn11 = assign57140_e88926_d_n11;
        locals.var_eeff_res_dn14 = assign57140_e88926_d_n14;
        locals.var_eeff_res_rv = 0.0;

        let (assign57150_e88944, assign57150_e88944_d_n0, assign57150_e88944_d_n2, assign57150_e88944_d_n4, assign57150_e88944_d_n5, assign57150_e88944_d_n6, assign57150_e88944_d_n7, assign57150_e88944_d_n8, assign57150_e88944_d_n9, assign57150_e88944_d_n10, assign57150_e88944_d_n11, assign57150_e88944_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let (assign57150_e88942, assign57150_e88942_d_n0, assign57150_e88942_d_n2, assign57150_e88942_d_n4, assign57150_e88942_d_n5, assign57150_e88942_d_n6, assign57150_e88942_d_n7, assign57150_e88942_d_n8, assign57150_e88942_d_n9, assign57150_e88942_d_n10, assign57150_e88942_d_n11, assign57150_e88942_d_n14,) = {
            if (locals.var_eeff_res == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57150_e88941: f64 = (locals.var_eeff_res).powf(p.p376);
                (assign57150_e88941, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn0)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn0 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn2)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn2 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn4)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn4 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn5)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn5 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn6)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn6 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn7)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn7 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn8)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn8 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn9)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn9 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn10)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn10 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn11)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn11 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn14)) } } else { (assign57150_e88941 * (p.p376 * (locals.var_eeff_res_dn14 / locals.var_eeff_res))) },)
            }
        };
        (assign57150_e88942, assign57150_e88942_d_n0, assign57150_e88942_d_n2, assign57150_e88942_d_n4, assign57150_e88942_d_n5, assign57150_e88942_d_n6, assign57150_e88942_d_n7, assign57150_e88942_d_n8, assign57150_e88942_d_n9, assign57150_e88942_d_n10, assign57150_e88942_d_n11, assign57150_e88942_d_n14,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign57150_e88944;
        locals.var_t8_dn0 = assign57150_e88944_d_n0;
        locals.var_t8_dn2 = assign57150_e88944_d_n2;
        locals.var_t8_dn4 = assign57150_e88944_d_n4;
        locals.var_t8_dn5 = assign57150_e88944_d_n5;
        locals.var_t8_dn6 = assign57150_e88944_d_n6;
        locals.var_t8_dn7 = assign57150_e88944_d_n7;
        locals.var_t8_dn8 = assign57150_e88944_d_n8;
        locals.var_t8_dn9 = assign57150_e88944_d_n9;
        locals.var_t8_dn10 = assign57150_e88944_d_n10;
        locals.var_t8_dn11 = assign57150_e88944_d_n11;
        locals.var_t8_dn14 = assign57150_e88944_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign57160_e88975, assign57160_e88975_d_n0, assign57160_e88975_d_n2, assign57160_e88975_d_n4, assign57160_e88975_d_n5, assign57160_e88975_d_n6, assign57160_e88975_d_n7, assign57160_e88975_d_n8, assign57160_e88975_d_n9, assign57160_e88975_d_n10, assign57160_e88975_d_n11, assign57160_e88975_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57160_e88955: f64 = 1.0;
        let assign57160_e88959: f64 = (locals.var_uc_depmue1 * locals.var_t4);
        let assign57160_e88961: f64 = (assign57160_e88959 * locals.var_rns);
        let assign57160_e88963: f64 = (assign57160_e88961 / 10000000000.0);
        let assign57160_e88964: f64 = (locals.var_uc_depmue0 + assign57160_e88963);
        let assign57160_e88966: f64 = (assign57160_e88964 + 1e-25);
        let assign57160_e88967: f64 = (assign57160_e88955 / assign57160_e88966);
        let assign57160_e88970: f64 = locals.var_depmphn0;
        let assign57160_e88972: f64 = (assign57160_e88970 * locals.var_t8);
        let assign57160_e88973: f64 = (assign57160_e88967 + assign57160_e88972);
        (assign57160_e88973, ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn0 + (((((locals.var_uc_depmue1_dn0 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn0)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn0)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn0 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn0))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn2 + (((((locals.var_uc_depmue1_dn2 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn2)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn2)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn2 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn2))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn4 + (((((locals.var_uc_depmue1_dn4 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn4)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn4)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn4 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn4))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn5 + (((((locals.var_uc_depmue1_dn5 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn5)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn5)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn5 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn5))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn6 + (((((locals.var_uc_depmue1_dn6 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn6)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn6)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn6 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn6))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn7 + (((((locals.var_uc_depmue1_dn7 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn7)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn7)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn7 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn7))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn8 + (((((locals.var_uc_depmue1_dn8 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn8)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn8)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn8 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn8))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn9 + (((((locals.var_uc_depmue1_dn9 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn9)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn9)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn9 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn9))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn10 + (((((locals.var_uc_depmue1_dn10 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn10)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn10)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn10 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn10))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn11 + (((((locals.var_uc_depmue1_dn11 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn11)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn11)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn11 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn11))), ((-((assign57160_e88955 * (locals.var_uc_depmue0_dn14 + (((((locals.var_uc_depmue1_dn14 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn14)) * locals.var_rns) + (assign57160_e88959 * locals.var_rns_dn14)) / 10000000000.0))) / (assign57160_e88966 * assign57160_e88966))) + ((locals.var_depmphn0_dn14 * locals.var_t8) + (assign57160_e88970 * locals.var_t8_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57160_e88975;
        locals.var_t1_dn0 = assign57160_e88975_d_n0;
        locals.var_t1_dn2 = assign57160_e88975_d_n2;
        locals.var_t1_dn4 = assign57160_e88975_d_n4;
        locals.var_t1_dn5 = assign57160_e88975_d_n5;
        locals.var_t1_dn6 = assign57160_e88975_d_n6;
        locals.var_t1_dn7 = assign57160_e88975_d_n7;
        locals.var_t1_dn8 = assign57160_e88975_d_n8;
        locals.var_t1_dn9 = assign57160_e88975_d_n9;
        locals.var_t1_dn10 = assign57160_e88975_d_n10;
        locals.var_t1_dn11 = assign57160_e88975_d_n11;
        locals.var_t1_dn14 = assign57160_e88975_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57170_e88988, assign57170_e88988_d_n0, assign57170_e88988_d_n2, assign57170_e88988_d_n4, assign57170_e88988_d_n5, assign57170_e88988_d_n6, assign57170_e88988_d_n7, assign57170_e88988_d_n8, assign57170_e88988_d_n9, assign57170_e88988_d_n10, assign57170_e88988_d_n11, assign57170_e88988_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57170_e88986: f64 = (1.0 / locals.var_t1);
        (assign57170_e88986, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign57170_e88988;
        locals.var_muun_dn0 = assign57170_e88988_d_n0;
        locals.var_muun_dn2 = assign57170_e88988_d_n2;
        locals.var_muun_dn4 = assign57170_e88988_d_n4;
        locals.var_muun_dn5 = assign57170_e88988_d_n5;
        locals.var_muun_dn6 = assign57170_e88988_d_n6;
        locals.var_muun_dn7 = assign57170_e88988_d_n7;
        locals.var_muun_dn8 = assign57170_e88988_d_n8;
        locals.var_muun_dn9 = assign57170_e88988_d_n9;
        locals.var_muun_dn10 = assign57170_e88988_d_n10;
        locals.var_muun_dn11 = assign57170_e88988_d_n11;
        locals.var_muun_dn14 = assign57170_e88988_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign57180_e89001, assign57180_e89001_d_n0, assign57180_e89001_d_n2, assign57180_e89001_d_n4, assign57180_e89001_d_n5, assign57180_e89001_d_n6, assign57180_e89001_d_n7, assign57180_e89001_d_n8, assign57180_e89001_d_n9, assign57180_e89001_d_n10, assign57180_e89001_d_n11, assign57180_e89001_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57180_e88999: f64 = (locals.var_muun / 10000.0);
        (assign57180_e88999, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign57180_e89001;
        locals.var_muun_dn0 = assign57180_e89001_d_n0;
        locals.var_muun_dn2 = assign57180_e89001_d_n2;
        locals.var_muun_dn4 = assign57180_e89001_d_n4;
        locals.var_muun_dn5 = assign57180_e89001_d_n5;
        locals.var_muun_dn6 = assign57180_e89001_d_n6;
        locals.var_muun_dn7 = assign57180_e89001_d_n7;
        locals.var_muun_dn8 = assign57180_e89001_d_n8;
        locals.var_muun_dn9 = assign57180_e89001_d_n9;
        locals.var_muun_dn10 = assign57180_e89001_d_n10;
        locals.var_muun_dn11 = assign57180_e89001_d_n11;
        locals.var_muun_dn14 = assign57180_e89001_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign57190_e89016, assign57190_e89016_d_n0, assign57190_e89016_d_n2, assign57190_e89016_d_n4, assign57190_e89016_d_n5, assign57190_e89016_d_n6, assign57190_e89016_d_n7, assign57190_e89016_d_n8, assign57190_e89016_d_n9, assign57190_e89016_d_n10, assign57190_e89016_d_n11, assign57190_e89016_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57190_e89013: f64 = (locals.var_leff + p.p401);
        let assign57190_e89014: f64 = (locals.var_vds_res / assign57190_e89013);
        (assign57190_e89014, (locals.var_vds_res_dn0 / assign57190_e89013), (locals.var_vds_res_dn2 / assign57190_e89013), (locals.var_vds_res_dn4 / assign57190_e89013), (locals.var_vds_res_dn5 / assign57190_e89013), (locals.var_vds_res_dn6 / assign57190_e89013), (locals.var_vds_res_dn7 / assign57190_e89013), (locals.var_vds_res_dn8 / assign57190_e89013), (locals.var_vds_res_dn9 / assign57190_e89013), (locals.var_vds_res_dn10 / assign57190_e89013), (locals.var_vds_res_dn11 / assign57190_e89013), (locals.var_vds_res_dn14 / assign57190_e89013),)
    } else {
        (locals.var_edri__blk1119, locals.var_edri__blk1119_dn0, locals.var_edri__blk1119_dn2, locals.var_edri__blk1119_dn4, locals.var_edri__blk1119_dn5, locals.var_edri__blk1119_dn6, locals.var_edri__blk1119_dn7, locals.var_edri__blk1119_dn8, locals.var_edri__blk1119_dn9, locals.var_edri__blk1119_dn10, locals.var_edri__blk1119_dn11, locals.var_edri__blk1119_dn14,)
    }
};
        locals.var_edri__blk1119 = assign57190_e89016;
        locals.var_edri__blk1119_dn0 = assign57190_e89016_d_n0;
        locals.var_edri__blk1119_dn2 = assign57190_e89016_d_n2;
        locals.var_edri__blk1119_dn4 = assign57190_e89016_d_n4;
        locals.var_edri__blk1119_dn5 = assign57190_e89016_d_n5;
        locals.var_edri__blk1119_dn6 = assign57190_e89016_d_n6;
        locals.var_edri__blk1119_dn7 = assign57190_e89016_d_n7;
        locals.var_edri__blk1119_dn8 = assign57190_e89016_d_n8;
        locals.var_edri__blk1119_dn9 = assign57190_e89016_d_n9;
        locals.var_edri__blk1119_dn10 = assign57190_e89016_d_n10;
        locals.var_edri__blk1119_dn11 = assign57190_e89016_d_n11;
        locals.var_edri__blk1119_dn14 = assign57190_e89016_d_n14;
        locals.var_edri__blk1119_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_209(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign57200_e89029, assign57200_e89029_d_n0, assign57200_e89029_d_n2, assign57200_e89029_d_n4, assign57200_e89029_d_n5, assign57200_e89029_d_n6, assign57200_e89029_d_n7, assign57200_e89029_d_n8, assign57200_e89029_d_n9, assign57200_e89029_d_n10, assign57200_e89029_d_n11, assign57200_e89029_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57200_e89027: f64 = (locals.var_vds_res).powf(2.0);
        (assign57200_e89027, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn0)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn0 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn2)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn2 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn4)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn4 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn5)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn5 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn6)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn6 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn7)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn7 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn8)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn8 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn9)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn9 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn10)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn10 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn11)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn11 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn14)) } } else { (assign57200_e89027 * (2.0 * (locals.var_vds_res_dn14 / locals.var_vds_res))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign57200_e89029;
        locals.var_tmf1_dn0 = assign57200_e89029_d_n0;
        locals.var_tmf1_dn2 = assign57200_e89029_d_n2;
        locals.var_tmf1_dn4 = assign57200_e89029_d_n4;
        locals.var_tmf1_dn5 = assign57200_e89029_d_n5;
        locals.var_tmf1_dn6 = assign57200_e89029_d_n6;
        locals.var_tmf1_dn7 = assign57200_e89029_d_n7;
        locals.var_tmf1_dn8 = assign57200_e89029_d_n8;
        locals.var_tmf1_dn9 = assign57200_e89029_d_n9;
        locals.var_tmf1_dn10 = assign57200_e89029_d_n10;
        locals.var_tmf1_dn11 = assign57200_e89029_d_n11;
        locals.var_tmf1_dn14 = assign57200_e89029_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign57210_e89042, assign57210_e89042_d_n0, assign57210_e89042_d_n2, assign57210_e89042_d_n4, assign57210_e89042_d_n5, assign57210_e89042_d_n6, assign57210_e89042_d_n7, assign57210_e89042_d_n8, assign57210_e89042_d_n9, assign57210_e89042_d_n10, assign57210_e89042_d_n11, assign57210_e89042_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57210_e89040: f64 = (0.01_f64).powf(2.0);
        (assign57210_e89040, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign57210_e89042;
        locals.var_tmf2_dn0 = assign57210_e89042_d_n0;
        locals.var_tmf2_dn2 = assign57210_e89042_d_n2;
        locals.var_tmf2_dn4 = assign57210_e89042_d_n4;
        locals.var_tmf2_dn5 = assign57210_e89042_d_n5;
        locals.var_tmf2_dn6 = assign57210_e89042_d_n6;
        locals.var_tmf2_dn7 = assign57210_e89042_d_n7;
        locals.var_tmf2_dn8 = assign57210_e89042_d_n8;
        locals.var_tmf2_dn9 = assign57210_e89042_d_n9;
        locals.var_tmf2_dn10 = assign57210_e89042_d_n10;
        locals.var_tmf2_dn11 = assign57210_e89042_d_n11;
        locals.var_tmf2_dn14 = assign57210_e89042_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign57220_e89065, assign57220_e89065_d_n0, assign57220_e89065_d_n2, assign57220_e89065_d_n4, assign57220_e89065_d_n5, assign57220_e89065_d_n6, assign57220_e89065_d_n7, assign57220_e89065_d_n8, assign57220_e89065_d_n9, assign57220_e89065_d_n10, assign57220_e89065_d_n11, assign57220_e89065_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57220_e89053: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign57220_e89056: f64 = (1.0 / 2.0);
        let assign57220_e89057: f64 = (assign57220_e89053).powf(assign57220_e89056);
        let assign57220_e89061: f64 = (1.0 / 2.0);
        let assign57220_e89062: f64 = (locals.var_tmf2).powf(assign57220_e89061);
        let assign57220_e89063: f64 = (assign57220_e89057 - assign57220_e89062);
        (assign57220_e89063, (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn0)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn0 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn2)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn2 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn4)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn4 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn5)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn5 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn6)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn6 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn7)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn7 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn8)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn8 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn9)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn9 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn10)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn10 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn11)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn11 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57220_e89056) as f64).is_finite() && ((assign57220_e89056) as f64).fract() == 0.0 { if assign57220_e89056 == 0.0 { 0.0 } else { (assign57220_e89056 * ((assign57220_e89053).powf(assign57220_e89056 - 1.0) * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))) } } else { (assign57220_e89057 * (assign57220_e89056 * ((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) / assign57220_e89053))) } - if 0.0 == 0.0 && ((assign57220_e89061) as f64).is_finite() && ((assign57220_e89061) as f64).fract() == 0.0 { if assign57220_e89061 == 0.0 { 0.0 } else { (assign57220_e89061 * ((locals.var_tmf2).powf(assign57220_e89061 - 1.0) * locals.var_tmf2_dn14)) } } else { (assign57220_e89062 * (assign57220_e89061 * (locals.var_tmf2_dn14 / locals.var_tmf2))) }),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57220_e89065;
        locals.var_t0_dn0 = assign57220_e89065_d_n0;
        locals.var_t0_dn2 = assign57220_e89065_d_n2;
        locals.var_t0_dn4 = assign57220_e89065_d_n4;
        locals.var_t0_dn5 = assign57220_e89065_d_n5;
        locals.var_t0_dn6 = assign57220_e89065_d_n6;
        locals.var_t0_dn7 = assign57220_e89065_d_n7;
        locals.var_t0_dn8 = assign57220_e89065_d_n8;
        locals.var_t0_dn9 = assign57220_e89065_d_n9;
        locals.var_t0_dn10 = assign57220_e89065_d_n10;
        locals.var_t0_dn11 = assign57220_e89065_d_n11;
        locals.var_t0_dn14 = assign57220_e89065_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57230_e89080, assign57230_e89080_d_n0, assign57230_e89080_d_n2, assign57230_e89080_d_n4, assign57230_e89080_d_n5, assign57230_e89080_d_n6, assign57230_e89080_d_n7, assign57230_e89080_d_n8, assign57230_e89080_d_n9, assign57230_e89080_d_n10, assign57230_e89080_d_n11, assign57230_e89080_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57230_e89077: f64 = (locals.var_leff - p.p402);
        let assign57230_e89078: f64 = (locals.var_t0 / assign57230_e89077);
        (assign57230_e89078, (locals.var_t0_dn0 / assign57230_e89077), (locals.var_t0_dn2 / assign57230_e89077), (locals.var_t0_dn4 / assign57230_e89077), (locals.var_t0_dn5 / assign57230_e89077), (locals.var_t0_dn6 / assign57230_e89077), (locals.var_t0_dn7 / assign57230_e89077), (locals.var_t0_dn8 / assign57230_e89077), (locals.var_t0_dn9 / assign57230_e89077), (locals.var_t0_dn10 / assign57230_e89077), (locals.var_t0_dn11 / assign57230_e89077), (locals.var_t0_dn14 / assign57230_e89077),)
    } else {
        (locals.var_edri2, locals.var_edri2_dn0, locals.var_edri2_dn2, locals.var_edri2_dn4, locals.var_edri2_dn5, locals.var_edri2_dn6, locals.var_edri2_dn7, locals.var_edri2_dn8, locals.var_edri2_dn9, locals.var_edri2_dn10, locals.var_edri2_dn11, locals.var_edri2_dn14,)
    }
};
        locals.var_edri2 = assign57230_e89080;
        locals.var_edri2_dn0 = assign57230_e89080_d_n0;
        locals.var_edri2_dn2 = assign57230_e89080_d_n2;
        locals.var_edri2_dn4 = assign57230_e89080_d_n4;
        locals.var_edri2_dn5 = assign57230_e89080_d_n5;
        locals.var_edri2_dn6 = assign57230_e89080_d_n6;
        locals.var_edri2_dn7 = assign57230_e89080_d_n7;
        locals.var_edri2_dn8 = assign57230_e89080_d_n8;
        locals.var_edri2_dn9 = assign57230_e89080_d_n9;
        locals.var_edri2_dn10 = assign57230_e89080_d_n10;
        locals.var_edri2_dn11 = assign57230_e89080_d_n11;
        locals.var_edri2_dn14 = assign57230_e89080_d_n14;
        locals.var_edri2_rv = 0.0;

        let (assign57240_e89095, assign57240_e89095_d_n0, assign57240_e89095_d_n2, assign57240_e89095_d_n4, assign57240_e89095_d_n5, assign57240_e89095_d_n6, assign57240_e89095_d_n7, assign57240_e89095_d_n8, assign57240_e89095_d_n9, assign57240_e89095_d_n10, assign57240_e89095_d_n11, assign57240_e89095_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57240_e89091: f64 = (locals.var_muun * locals.var_edri2);
        let assign57240_e89093: f64 = (assign57240_e89091 / locals.var_uc_depvmax);
        (assign57240_e89093, (((((locals.var_muun_dn0 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn0)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn2)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn4)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn5)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn6)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn7)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn8)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn9)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn10)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn11 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn11)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn14 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn14)) * locals.var_uc_depvmax) - (assign57240_e89091 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57240_e89095;
        locals.var_t1_dn0 = assign57240_e89095_d_n0;
        locals.var_t1_dn2 = assign57240_e89095_d_n2;
        locals.var_t1_dn4 = assign57240_e89095_d_n4;
        locals.var_t1_dn5 = assign57240_e89095_d_n5;
        locals.var_t1_dn6 = assign57240_e89095_d_n6;
        locals.var_t1_dn7 = assign57240_e89095_d_n7;
        locals.var_t1_dn8 = assign57240_e89095_d_n8;
        locals.var_t1_dn9 = assign57240_e89095_d_n9;
        locals.var_t1_dn10 = assign57240_e89095_d_n10;
        locals.var_t1_dn11 = assign57240_e89095_d_n11;
        locals.var_t1_dn14 = assign57240_e89095_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57250_e89113, assign57250_e89113_d_n0, assign57250_e89113_d_n2, assign57250_e89113_d_n4, assign57250_e89113_d_n5, assign57250_e89113_d_n6, assign57250_e89113_d_n7, assign57250_e89113_d_n8, assign57250_e89113_d_n9, assign57250_e89113_d_n10, assign57250_e89113_d_n11, assign57250_e89113_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let (assign57250_e89111, assign57250_e89111_d_n0, assign57250_e89111_d_n2, assign57250_e89111_d_n4, assign57250_e89111_d_n5, assign57250_e89111_d_n6, assign57250_e89111_d_n7, assign57250_e89111_d_n8, assign57250_e89111_d_n9, assign57250_e89111_d_n10, assign57250_e89111_d_n11, assign57250_e89111_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57250_e89110: f64 = (locals.var_t1).powf(p.p378);
                (assign57250_e89110, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn11)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn14)) } } else { (assign57250_e89110 * (p.p378 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign57250_e89111, assign57250_e89111_d_n0, assign57250_e89111_d_n2, assign57250_e89111_d_n4, assign57250_e89111_d_n5, assign57250_e89111_d_n6, assign57250_e89111_d_n7, assign57250_e89111_d_n8, assign57250_e89111_d_n9, assign57250_e89111_d_n10, assign57250_e89111_d_n11, assign57250_e89111_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57250_e89113;
        locals.var_t2_dn0 = assign57250_e89113_d_n0;
        locals.var_t2_dn2 = assign57250_e89113_d_n2;
        locals.var_t2_dn4 = assign57250_e89113_d_n4;
        locals.var_t2_dn5 = assign57250_e89113_d_n5;
        locals.var_t2_dn6 = assign57250_e89113_d_n6;
        locals.var_t2_dn7 = assign57250_e89113_d_n7;
        locals.var_t2_dn8 = assign57250_e89113_d_n8;
        locals.var_t2_dn9 = assign57250_e89113_d_n9;
        locals.var_t2_dn10 = assign57250_e89113_d_n10;
        locals.var_t2_dn11 = assign57250_e89113_d_n11;
        locals.var_t2_dn14 = assign57250_e89113_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57260_e89126, assign57260_e89126_d_n0, assign57260_e89126_d_n2, assign57260_e89126_d_n4, assign57260_e89126_d_n5, assign57260_e89126_d_n6, assign57260_e89126_d_n7, assign57260_e89126_d_n8, assign57260_e89126_d_n9, assign57260_e89126_d_n10, assign57260_e89126_d_n11, assign57260_e89126_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57260_e89124: f64 = (1.0 + locals.var_t2);
        (assign57260_e89124, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign57260_e89126;
        locals.var_t3_dn0 = assign57260_e89126_d_n0;
        locals.var_t3_dn2 = assign57260_e89126_d_n2;
        locals.var_t3_dn4 = assign57260_e89126_d_n4;
        locals.var_t3_dn5 = assign57260_e89126_d_n5;
        locals.var_t3_dn6 = assign57260_e89126_d_n6;
        locals.var_t3_dn7 = assign57260_e89126_d_n7;
        locals.var_t3_dn8 = assign57260_e89126_d_n8;
        locals.var_t3_dn9 = assign57260_e89126_d_n9;
        locals.var_t3_dn10 = assign57260_e89126_d_n10;
        locals.var_t3_dn11 = assign57260_e89126_d_n11;
        locals.var_t3_dn14 = assign57260_e89126_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign57270_e89146, assign57270_e89146_d_n0, assign57270_e89146_d_n2, assign57270_e89146_d_n4, assign57270_e89146_d_n5, assign57270_e89146_d_n6, assign57270_e89146_d_n7, assign57270_e89146_d_n8, assign57270_e89146_d_n9, assign57270_e89146_d_n10, assign57270_e89146_d_n11, assign57270_e89146_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let (assign57270_e89144, assign57270_e89144_d_n0, assign57270_e89144_d_n2, assign57270_e89144_d_n4, assign57270_e89144_d_n5, assign57270_e89144_d_n6, assign57270_e89144_d_n7, assign57270_e89144_d_n8, assign57270_e89144_d_n9, assign57270_e89144_d_n10, assign57270_e89144_d_n11, assign57270_e89144_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57270_e89142: f64 = (1.0 / p.p378);
                let assign57270_e89143: f64 = (locals.var_t3).powf(assign57270_e89142);
                (assign57270_e89143, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn0)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn2)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn4)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn5)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn6)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn7)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn8)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn9)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn10)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn11)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57270_e89142) as f64).is_finite() && ((assign57270_e89142) as f64).fract() == 0.0 { if assign57270_e89142 == 0.0 { 0.0 } else { (assign57270_e89142 * ((locals.var_t3).powf(assign57270_e89142 - 1.0) * locals.var_t3_dn14)) } } else { (assign57270_e89143 * (assign57270_e89142 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign57270_e89144, assign57270_e89144_d_n0, assign57270_e89144_d_n2, assign57270_e89144_d_n4, assign57270_e89144_d_n5, assign57270_e89144_d_n6, assign57270_e89144_d_n7, assign57270_e89144_d_n8, assign57270_e89144_d_n9, assign57270_e89144_d_n10, assign57270_e89144_d_n11, assign57270_e89144_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57270_e89146;
        locals.var_t4_dn0 = assign57270_e89146_d_n0;
        locals.var_t4_dn2 = assign57270_e89146_d_n2;
        locals.var_t4_dn4 = assign57270_e89146_d_n4;
        locals.var_t4_dn5 = assign57270_e89146_d_n5;
        locals.var_t4_dn6 = assign57270_e89146_d_n6;
        locals.var_t4_dn7 = assign57270_e89146_d_n7;
        locals.var_t4_dn8 = assign57270_e89146_d_n8;
        locals.var_t4_dn9 = assign57270_e89146_d_n9;
        locals.var_t4_dn10 = assign57270_e89146_d_n10;
        locals.var_t4_dn11 = assign57270_e89146_d_n11;
        locals.var_t4_dn14 = assign57270_e89146_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57280_e89159, assign57280_e89159_d_n0, assign57280_e89159_d_n2, assign57280_e89159_d_n4, assign57280_e89159_d_n5, assign57280_e89159_d_n6, assign57280_e89159_d_n7, assign57280_e89159_d_n8, assign57280_e89159_d_n9, assign57280_e89159_d_n10, assign57280_e89159_d_n11, assign57280_e89159_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57280_e89157: f64 = (locals.var_muun / locals.var_t4);
        (assign57280_e89157, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn11 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn14 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_res, locals.var_mu_res_dn0, locals.var_mu_res_dn2, locals.var_mu_res_dn4, locals.var_mu_res_dn5, locals.var_mu_res_dn6, locals.var_mu_res_dn7, locals.var_mu_res_dn8, locals.var_mu_res_dn9, locals.var_mu_res_dn10, locals.var_mu_res_dn11, locals.var_mu_res_dn14,)
    }
};
        locals.var_mu_res = assign57280_e89159;
        locals.var_mu_res_dn0 = assign57280_e89159_d_n0;
        locals.var_mu_res_dn2 = assign57280_e89159_d_n2;
        locals.var_mu_res_dn4 = assign57280_e89159_d_n4;
        locals.var_mu_res_dn5 = assign57280_e89159_d_n5;
        locals.var_mu_res_dn6 = assign57280_e89159_d_n6;
        locals.var_mu_res_dn7 = assign57280_e89159_d_n7;
        locals.var_mu_res_dn8 = assign57280_e89159_d_n8;
        locals.var_mu_res_dn9 = assign57280_e89159_d_n9;
        locals.var_mu_res_dn10 = assign57280_e89159_d_n10;
        locals.var_mu_res_dn11 = assign57280_e89159_d_n11;
        locals.var_mu_res_dn14 = assign57280_e89159_d_n14;
        locals.var_mu_res_rv = 0.0;

        let (assign57290_e89188, assign57290_e89188_d_n0, assign57290_e89188_d_n2, assign57290_e89188_d_n4, assign57290_e89188_d_n5, assign57290_e89188_d_n6, assign57290_e89188_d_n7, assign57290_e89188_d_n8, assign57290_e89188_d_n9, assign57290_e89188_d_n10, assign57290_e89188_d_n11, assign57290_e89188_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57290_e89172: f64 = (p.p400 * locals.var_edri__blk1119);
        let assign57290_e89178: f64 = (locals.var_muun * locals.var_edri__blk1119);
        let assign57290_e89180: f64 = (assign57290_e89178 / locals.var_uc_depvmax);
        let assign57290_e89181: f64 = (1.0 + assign57290_e89180);
        let assign57290_e89182: f64 = (1.0 / assign57290_e89181);
        let assign57290_e89183: f64 = (1.0 - assign57290_e89182);
        let assign57290_e89184: f64 = (assign57290_e89172 * assign57290_e89183);
        let assign57290_e89185: f64 = (1.0 + assign57290_e89184);
        let assign57290_e89186: f64 = (locals.var_uc_ndepm * assign57290_e89185);
        (assign57290_e89186, ((locals.var_uc_ndepm_dn0 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn0) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn0 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn0)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn2 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn2) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn2 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn2)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn4 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn4) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn4 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn4)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn5 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn5) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn5 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn5)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn6 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn6) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn6 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn6)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn7 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn7) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn7 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn7)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn8 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn8) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn8 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn8)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn9 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn9) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn9 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn9)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn10 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn10) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn10 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn10)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn11 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn11) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn11 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn11)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))), ((locals.var_uc_ndepm_dn14 * assign57290_e89185) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1119_dn14) * assign57290_e89183) + (assign57290_e89172 * (-(-((((((locals.var_muun_dn14 * locals.var_edri__blk1119) + (locals.var_muun * locals.var_edri__blk1119_dn14)) * locals.var_uc_depvmax) - (assign57290_e89178 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57290_e89181 * assign57290_e89181)))))))),)
    } else {
        (locals.var_n_res, locals.var_n_res_dn0, locals.var_n_res_dn2, locals.var_n_res_dn4, locals.var_n_res_dn5, locals.var_n_res_dn6, locals.var_n_res_dn7, locals.var_n_res_dn8, locals.var_n_res_dn9, locals.var_n_res_dn10, locals.var_n_res_dn11, locals.var_n_res_dn14,)
    }
};
        locals.var_n_res = assign57290_e89188;
        locals.var_n_res_dn0 = assign57290_e89188_d_n0;
        locals.var_n_res_dn2 = assign57290_e89188_d_n2;
        locals.var_n_res_dn4 = assign57290_e89188_d_n4;
        locals.var_n_res_dn5 = assign57290_e89188_d_n5;
        locals.var_n_res_dn6 = assign57290_e89188_d_n6;
        locals.var_n_res_dn7 = assign57290_e89188_d_n7;
        locals.var_n_res_dn8 = assign57290_e89188_d_n8;
        locals.var_n_res_dn9 = assign57290_e89188_d_n9;
        locals.var_n_res_dn10 = assign57290_e89188_d_n10;
        locals.var_n_res_dn11 = assign57290_e89188_d_n11;
        locals.var_n_res_dn14 = assign57290_e89188_d_n14;
        locals.var_n_res_rv = 0.0;

        let (assign57300_e89203, assign57300_e89203_d_n0, assign57300_e89203_d_n2, assign57300_e89203_d_n4, assign57300_e89203_d_n5, assign57300_e89203_d_n6, assign57300_e89203_d_n7, assign57300_e89203_d_n8, assign57300_e89203_d_n9, assign57300_e89203_d_n10, assign57300_e89203_d_n11, assign57300_e89203_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57300_e89199: f64 = (locals.var_w_res * 1.6021918e-19);
        let assign57300_e89201: f64 = (assign57300_e89199 * locals.var_n_res);
        (assign57300_e89201, (((locals.var_w_res_dn0 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn0)), (((locals.var_w_res_dn2 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn2)), (((locals.var_w_res_dn4 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn4)), (((locals.var_w_res_dn5 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn5)), (((locals.var_w_res_dn6 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn6)), (((locals.var_w_res_dn7 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn7)), (((locals.var_w_res_dn8 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn8)), (((locals.var_w_res_dn9 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn9)), (((locals.var_w_res_dn10 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn10)), (((locals.var_w_res_dn11 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn11)), (((locals.var_w_res_dn14 * 1.6021918e-19) * locals.var_n_res) + (assign57300_e89199 * locals.var_n_res_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57300_e89203;
        locals.var_t1_dn0 = assign57300_e89203_d_n0;
        locals.var_t1_dn2 = assign57300_e89203_d_n2;
        locals.var_t1_dn4 = assign57300_e89203_d_n4;
        locals.var_t1_dn5 = assign57300_e89203_d_n5;
        locals.var_t1_dn6 = assign57300_e89203_d_n6;
        locals.var_t1_dn7 = assign57300_e89203_d_n7;
        locals.var_t1_dn8 = assign57300_e89203_d_n8;
        locals.var_t1_dn9 = assign57300_e89203_d_n9;
        locals.var_t1_dn10 = assign57300_e89203_d_n10;
        locals.var_t1_dn11 = assign57300_e89203_d_n11;
        locals.var_t1_dn14 = assign57300_e89203_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57310_e89220, assign57310_e89220_d_n0, assign57310_e89220_d_n2, assign57310_e89220_d_n4, assign57310_e89220_d_n5, assign57310_e89220_d_n6, assign57310_e89220_d_n7, assign57310_e89220_d_n8, assign57310_e89220_d_n9, assign57310_e89220_d_n10, assign57310_e89220_d_n11, assign57310_e89220_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57310_e89214: f64 = (locals.var_weff / locals.var_leff);
        let assign57310_e89216: f64 = (assign57310_e89214).powf(locals.var_uc_depwlp);
        let assign57310_e89218: f64 = (assign57310_e89216 * p.p7);
        (assign57310_e89218, (if locals.var_uc_depwlp_dn0 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn0 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn2 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn2 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn4 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn4 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn5 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn5 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn6 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn6 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn7 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn7 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn8 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn8 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn9 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn9 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn10 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn10 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn11 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn11 * (assign57310_e89214).ln())) } * p.p7), (if locals.var_uc_depwlp_dn14 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57310_e89216 * (locals.var_uc_depwlp_dn14 * (assign57310_e89214).ln())) } * p.p7),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57310_e89220;
        locals.var_t2_dn0 = assign57310_e89220_d_n0;
        locals.var_t2_dn2 = assign57310_e89220_d_n2;
        locals.var_t2_dn4 = assign57310_e89220_d_n4;
        locals.var_t2_dn5 = assign57310_e89220_d_n5;
        locals.var_t2_dn6 = assign57310_e89220_d_n6;
        locals.var_t2_dn7 = assign57310_e89220_d_n7;
        locals.var_t2_dn8 = assign57310_e89220_d_n8;
        locals.var_t2_dn9 = assign57310_e89220_d_n9;
        locals.var_t2_dn10 = assign57310_e89220_d_n10;
        locals.var_t2_dn11 = assign57310_e89220_d_n11;
        locals.var_t2_dn14 = assign57310_e89220_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57320_e89237, assign57320_e89237_d_n0, assign57320_e89237_d_n2, assign57320_e89237_d_n4, assign57320_e89237_d_n5, assign57320_e89237_d_n6, assign57320_e89237_d_n7, assign57320_e89237_d_n8, assign57320_e89237_d_n9, assign57320_e89237_d_n10, assign57320_e89237_d_n11, assign57320_e89237_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57320_e89231: f64 = (locals.var_weff_nf * locals.var_t1);
        let assign57320_e89233: f64 = (assign57320_e89231 * locals.var_mu_res);
        let assign57320_e89235: f64 = (assign57320_e89233 * locals.var_edri__blk1119);
        (assign57320_e89235, (((((locals.var_weff_nf * locals.var_t1_dn0) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn0)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn0)), (((((locals.var_weff_nf * locals.var_t1_dn2) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn2)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn2)), (((((locals.var_weff_nf * locals.var_t1_dn4) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn4)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn4)), (((((locals.var_weff_nf * locals.var_t1_dn5) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn5)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn5)), (((((locals.var_weff_nf * locals.var_t1_dn6) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn6)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn6)), (((((locals.var_weff_nf * locals.var_t1_dn7) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn7)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn7)), (((((locals.var_weff_nf * locals.var_t1_dn8) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn8)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn8)), (((((locals.var_weff_nf * locals.var_t1_dn9) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn9)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn9)), (((((locals.var_weff_nf * locals.var_t1_dn10) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn10)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn10)), (((((locals.var_weff_nf * locals.var_t1_dn11) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn11)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn11)), (((((locals.var_weff_nf * locals.var_t1_dn14) * locals.var_mu_res) + (assign57320_e89231 * locals.var_mu_res_dn14)) * locals.var_edri__blk1119) + (assign57320_e89233 * locals.var_edri__blk1119_dn14)),)
    } else {
        (locals.var_ids_res, locals.var_ids_res_dn0, locals.var_ids_res_dn2, locals.var_ids_res_dn4, locals.var_ids_res_dn5, locals.var_ids_res_dn6, locals.var_ids_res_dn7, locals.var_ids_res_dn8, locals.var_ids_res_dn9, locals.var_ids_res_dn10, locals.var_ids_res_dn11, locals.var_ids_res_dn14,)
    }
};
        locals.var_ids_res = assign57320_e89237;
        locals.var_ids_res_dn0 = assign57320_e89237_d_n0;
        locals.var_ids_res_dn2 = assign57320_e89237_d_n2;
        locals.var_ids_res_dn4 = assign57320_e89237_d_n4;
        locals.var_ids_res_dn5 = assign57320_e89237_d_n5;
        locals.var_ids_res_dn6 = assign57320_e89237_d_n6;
        locals.var_ids_res_dn7 = assign57320_e89237_d_n7;
        locals.var_ids_res_dn8 = assign57320_e89237_d_n8;
        locals.var_ids_res_dn9 = assign57320_e89237_d_n9;
        locals.var_ids_res_dn10 = assign57320_e89237_d_n10;
        locals.var_ids_res_dn11 = assign57320_e89237_d_n11;
        locals.var_ids_res_dn14 = assign57320_e89237_d_n14;
        locals.var_ids_res_rv = 0.0;

        let (assign57330_e89254, assign57330_e89254_d_n0, assign57330_e89254_d_n2, assign57330_e89254_d_n4, assign57330_e89254_d_n5, assign57330_e89254_d_n6, assign57330_e89254_d_n7, assign57330_e89254_d_n8, assign57330_e89254_d_n9, assign57330_e89254_d_n10, assign57330_e89254_d_n11, assign57330_e89254_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57330_e89248: f64 = (locals.var_t2 * locals.var_w_res_leak);
        let assign57330_e89250: f64 = (assign57330_e89248 * p.p363);
        let assign57330_e89252: f64 = (assign57330_e89250 * locals.var_vds_res0_sym);
        (assign57330_e89252, (((((locals.var_t2_dn0 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn0)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn0)), (((((locals.var_t2_dn2 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn2)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn2)), (((((locals.var_t2_dn4 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn4)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn4)), (((((locals.var_t2_dn5 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn5)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn5)), (((((locals.var_t2_dn6 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn6)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn6)), (((((locals.var_t2_dn7 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn7)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn7)), (((((locals.var_t2_dn8 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn8)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn8)), (((((locals.var_t2_dn9 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn9)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn9)), (((((locals.var_t2_dn10 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn10)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn10)), (((((locals.var_t2_dn11 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn11)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn11)), (((((locals.var_t2_dn14 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn14)) * p.p363) * locals.var_vds_res0_sym) + (assign57330_e89250 * locals.var_vds_res0_sym_dn14)),)
    } else {
        (locals.var_ires_leak, locals.var_ires_leak_dn0, locals.var_ires_leak_dn2, locals.var_ires_leak_dn4, locals.var_ires_leak_dn5, locals.var_ires_leak_dn6, locals.var_ires_leak_dn7, locals.var_ires_leak_dn8, locals.var_ires_leak_dn9, locals.var_ires_leak_dn10, locals.var_ires_leak_dn11, locals.var_ires_leak_dn14,)
    }
};
        locals.var_ires_leak = assign57330_e89254;
        locals.var_ires_leak_dn0 = assign57330_e89254_d_n0;
        locals.var_ires_leak_dn2 = assign57330_e89254_d_n2;
        locals.var_ires_leak_dn4 = assign57330_e89254_d_n4;
        locals.var_ires_leak_dn5 = assign57330_e89254_d_n5;
        locals.var_ires_leak_dn6 = assign57330_e89254_d_n6;
        locals.var_ires_leak_dn7 = assign57330_e89254_d_n7;
        locals.var_ires_leak_dn8 = assign57330_e89254_d_n8;
        locals.var_ires_leak_dn9 = assign57330_e89254_d_n9;
        locals.var_ires_leak_dn10 = assign57330_e89254_d_n10;
        locals.var_ires_leak_dn11 = assign57330_e89254_d_n11;
        locals.var_ires_leak_dn14 = assign57330_e89254_d_n14;
        locals.var_ires_leak_rv = 0.0;

        let (assign57340_e89269, assign57340_e89269_d_n0, assign57340_e89269_d_n2, assign57340_e89269_d_n4, assign57340_e89269_d_n5, assign57340_e89269_d_n6, assign57340_e89269_d_n7, assign57340_e89269_d_n8, assign57340_e89269_d_n9, assign57340_e89269_d_n10, assign57340_e89269_d_n11, assign57340_e89269_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57340_e89265: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign57340_e89267: f64 = (assign57340_e89265 / locals.var_lch);
        (assign57340_e89267, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn11) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn14) * locals.var_lch) - (assign57340_e89265 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn14,)
    }
};
        locals.var_betawl = assign57340_e89269;
        locals.var_betawl_dn0 = assign57340_e89269_d_n0;
        locals.var_betawl_dn2 = assign57340_e89269_d_n2;
        locals.var_betawl_dn4 = assign57340_e89269_d_n4;
        locals.var_betawl_dn5 = assign57340_e89269_d_n5;
        locals.var_betawl_dn6 = assign57340_e89269_d_n6;
        locals.var_betawl_dn7 = assign57340_e89269_d_n7;
        locals.var_betawl_dn8 = assign57340_e89269_d_n8;
        locals.var_betawl_dn9 = assign57340_e89269_d_n9;
        locals.var_betawl_dn10 = assign57340_e89269_d_n10;
        locals.var_betawl_dn11 = assign57340_e89269_d_n11;
        locals.var_betawl_dn14 = assign57340_e89269_d_n14;
        locals.var_betawl_rv = 0.0;

        let (assign57350_e89284, assign57350_e89284_d_n0, assign57350_e89284_d_n2, assign57350_e89284_d_n4, assign57350_e89284_d_n5, assign57350_e89284_d_n6, assign57350_e89284_d_n7, assign57350_e89284_d_n8, assign57350_e89284_d_n9, assign57350_e89284_d_n10, assign57350_e89284_d_n11, assign57350_e89284_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57350_e89280: f64 = (locals.var_betawl * locals.var_idd);
        let assign57350_e89282: f64 = (assign57350_e89280 * locals.var_mu_acc);
        (assign57350_e89282, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu_acc) + (assign57350_e89280 * locals.var_mu_acc_dn14)),)
    } else {
        (locals.var_ids_acc, locals.var_ids_acc_dn0, locals.var_ids_acc_dn2, locals.var_ids_acc_dn4, locals.var_ids_acc_dn5, locals.var_ids_acc_dn6, locals.var_ids_acc_dn7, locals.var_ids_acc_dn8, locals.var_ids_acc_dn9, locals.var_ids_acc_dn10, locals.var_ids_acc_dn11, locals.var_ids_acc_dn14,)
    }
};
        locals.var_ids_acc = assign57350_e89284;
        locals.var_ids_acc_dn0 = assign57350_e89284_d_n0;
        locals.var_ids_acc_dn2 = assign57350_e89284_d_n2;
        locals.var_ids_acc_dn4 = assign57350_e89284_d_n4;
        locals.var_ids_acc_dn5 = assign57350_e89284_d_n5;
        locals.var_ids_acc_dn6 = assign57350_e89284_d_n6;
        locals.var_ids_acc_dn7 = assign57350_e89284_d_n7;
        locals.var_ids_acc_dn8 = assign57350_e89284_d_n8;
        locals.var_ids_acc_dn9 = assign57350_e89284_d_n9;
        locals.var_ids_acc_dn10 = assign57350_e89284_d_n10;
        locals.var_ids_acc_dn11 = assign57350_e89284_d_n11;
        locals.var_ids_acc_dn14 = assign57350_e89284_d_n14;
        locals.var_ids_acc_rv = 0.0;

        let (assign57360_e89305, assign57360_e89305_d_n0, assign57360_e89305_d_n2, assign57360_e89305_d_n4, assign57360_e89305_d_n5, assign57360_e89305_d_n6, assign57360_e89305_d_n7, assign57360_e89305_d_n8, assign57360_e89305_d_n9, assign57360_e89305_d_n10, assign57360_e89305_d_n11, assign57360_e89305_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57360_e89295: f64 = locals.var_ids_acc;
        let assign57360_e89298: f64 = locals.var_ids_res;
        let assign57360_e89299: f64 = (assign57360_e89295 + assign57360_e89298);
        let assign57360_e89302: f64 = locals.var_ires_leak;
        let assign57360_e89303: f64 = (assign57360_e89299 + assign57360_e89302);
        (assign57360_e89303, ((locals.var_ids_acc_dn0 + locals.var_ids_res_dn0) + locals.var_ires_leak_dn0), ((locals.var_ids_acc_dn2 + locals.var_ids_res_dn2) + locals.var_ires_leak_dn2), ((locals.var_ids_acc_dn4 + locals.var_ids_res_dn4) + locals.var_ires_leak_dn4), ((locals.var_ids_acc_dn5 + locals.var_ids_res_dn5) + locals.var_ires_leak_dn5), ((locals.var_ids_acc_dn6 + locals.var_ids_res_dn6) + locals.var_ires_leak_dn6), ((locals.var_ids_acc_dn7 + locals.var_ids_res_dn7) + locals.var_ires_leak_dn7), ((locals.var_ids_acc_dn8 + locals.var_ids_res_dn8) + locals.var_ires_leak_dn8), ((locals.var_ids_acc_dn9 + locals.var_ids_res_dn9) + locals.var_ires_leak_dn9), ((locals.var_ids_acc_dn10 + locals.var_ids_res_dn10) + locals.var_ires_leak_dn10), ((locals.var_ids_acc_dn11 + locals.var_ids_res_dn11) + locals.var_ires_leak_dn11), ((locals.var_ids_acc_dn14 + locals.var_ids_res_dn14) + locals.var_ires_leak_dn14),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign57360_e89305;
        locals.var_ids0_dn0 = assign57360_e89305_d_n0;
        locals.var_ids0_dn2 = assign57360_e89305_d_n2;
        locals.var_ids0_dn4 = assign57360_e89305_d_n4;
        locals.var_ids0_dn5 = assign57360_e89305_d_n5;
        locals.var_ids0_dn6 = assign57360_e89305_d_n6;
        locals.var_ids0_dn7 = assign57360_e89305_d_n7;
        locals.var_ids0_dn8 = assign57360_e89305_d_n8;
        locals.var_ids0_dn9 = assign57360_e89305_d_n9;
        locals.var_ids0_dn10 = assign57360_e89305_d_n10;
        locals.var_ids0_dn11 = assign57360_e89305_d_n11;
        locals.var_ids0_dn14 = assign57360_e89305_d_n14;
        locals.var_ids0_rv = 0.0;

        let (assign57370_e89316, assign57370_e89316_d_n0, assign57370_e89316_d_n2, assign57370_e89316_d_n4, assign57370_e89316_d_n5, assign57370_e89316_d_n6, assign57370_e89316_d_n7, assign57370_e89316_d_n8, assign57370_e89316_d_n9, assign57370_e89316_d_n10, assign57370_e89316_d_n11, assign57370_e89316_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign57370_e89316;
        locals.var_vds_dn0 = assign57370_e89316_d_n0;
        locals.var_vds_dn2 = assign57370_e89316_d_n2;
        locals.var_vds_dn4 = assign57370_e89316_d_n4;
        locals.var_vds_dn5 = assign57370_e89316_d_n5;
        locals.var_vds_dn6 = assign57370_e89316_d_n6;
        locals.var_vds_dn7 = assign57370_e89316_d_n7;
        locals.var_vds_dn8 = assign57370_e89316_d_n8;
        locals.var_vds_dn9 = assign57370_e89316_d_n9;
        locals.var_vds_dn10 = assign57370_e89316_d_n10;
        locals.var_vds_dn11 = assign57370_e89316_d_n11;
        locals.var_vds_dn14 = assign57370_e89316_d_n14;
        locals.var_vds_rv = 0.0;

        let assign57380_e89319: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1427 = assign57380_e89319;
        locals.var_guard1427_rv = 0.0;

        let (assign57390_e89336, assign57390_e89336_d_n0, assign57390_e89336_d_n2, assign57390_e89336_d_n4, assign57390_e89336_d_n5, assign57390_e89336_d_n6, assign57390_e89336_d_n7, assign57390_e89336_d_n8, assign57390_e89336_d_n9, assign57390_e89336_d_n10, assign57390_e89336_d_n11, assign57390_e89336_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57390_e89333: f64 = (locals.var_vds - locals.var_pds);
        let assign57390_e89334: f64 = (0.5 * assign57390_e89333);
        (assign57390_e89334, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57390_e89336;
        locals.var_t1_dn0 = assign57390_e89336_d_n0;
        locals.var_t1_dn2 = assign57390_e89336_d_n2;
        locals.var_t1_dn4 = assign57390_e89336_d_n4;
        locals.var_t1_dn5 = assign57390_e89336_d_n5;
        locals.var_t1_dn6 = assign57390_e89336_d_n6;
        locals.var_t1_dn7 = assign57390_e89336_d_n7;
        locals.var_t1_dn8 = assign57390_e89336_d_n8;
        locals.var_t1_dn9 = assign57390_e89336_d_n9;
        locals.var_t1_dn10 = assign57390_e89336_d_n10;
        locals.var_t1_dn11 = assign57390_e89336_d_n11;
        locals.var_t1_dn14 = assign57390_e89336_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57400_e89353, assign57400_e89353_d_n0, assign57400_e89353_d_n2, assign57400_e89353_d_n4, assign57400_e89353_d_n5, assign57400_e89353_d_n6, assign57400_e89353_d_n7, assign57400_e89353_d_n8, assign57400_e89353_d_n9, assign57400_e89353_d_n10, assign57400_e89353_d_n11, assign57400_e89353_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57400_e89349: f64 = (2.0 * locals.var_t1);
        let assign57400_e89351: f64 = (assign57400_e89349 / 0.01);
        (assign57400_e89351, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn14) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign57400_e89353;
        locals.var_tmf1_dn0 = assign57400_e89353_d_n0;
        locals.var_tmf1_dn2 = assign57400_e89353_d_n2;
        locals.var_tmf1_dn4 = assign57400_e89353_d_n4;
        locals.var_tmf1_dn5 = assign57400_e89353_d_n5;
        locals.var_tmf1_dn6 = assign57400_e89353_d_n6;
        locals.var_tmf1_dn7 = assign57400_e89353_d_n7;
        locals.var_tmf1_dn8 = assign57400_e89353_d_n8;
        locals.var_tmf1_dn9 = assign57400_e89353_d_n9;
        locals.var_tmf1_dn10 = assign57400_e89353_d_n10;
        locals.var_tmf1_dn11 = assign57400_e89353_d_n11;
        locals.var_tmf1_dn14 = assign57400_e89353_d_n14;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_210(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign57410_e89402, assign57410_e89402_d_n0, assign57410_e89402_d_n2, assign57410_e89402_d_n4, assign57410_e89402_d_n5, assign57410_e89402_d_n6, assign57410_e89402_d_n7, assign57410_e89402_d_n8, assign57410_e89402_d_n9, assign57410_e89402_d_n10, assign57410_e89402_d_n11, assign57410_e89402_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57410_e89368: f64 = (1.0 / 2.0);
        let assign57410_e89372: f64 = (1.0 / 6.0);
        let assign57410_e89376: f64 = (1.0 / 24.0);
        let assign57410_e89380: f64 = (1.0 / 120.0);
        let assign57410_e89384: f64 = (1.0 / 720.0);
        let assign57410_e89388: f64 = (1.0 / 5040.0);
        let assign57410_e89389: f64 = (locals.var_tmf1 * assign57410_e89388);
        let assign57410_e89390: f64 = (assign57410_e89384 + assign57410_e89389);
        let assign57410_e89391: f64 = (locals.var_tmf1 * assign57410_e89390);
        let assign57410_e89392: f64 = (assign57410_e89380 + assign57410_e89391);
        let assign57410_e89393: f64 = (locals.var_tmf1 * assign57410_e89392);
        let assign57410_e89394: f64 = (assign57410_e89376 + assign57410_e89393);
        let assign57410_e89395: f64 = (locals.var_tmf1 * assign57410_e89394);
        let assign57410_e89396: f64 = (assign57410_e89372 + assign57410_e89395);
        let assign57410_e89397: f64 = (locals.var_tmf1 * assign57410_e89396);
        let assign57410_e89398: f64 = (assign57410_e89368 + assign57410_e89397);
        let assign57410_e89399: f64 = (locals.var_tmf1 * assign57410_e89398);
        let assign57410_e89400: f64 = (1.0 + assign57410_e89399);
        (assign57410_e89400, ((locals.var_tmf1_dn0 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn2 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn4 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn5 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn6 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn7 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn8 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn9 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn10 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn11 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign57410_e89388))))))))))), ((locals.var_tmf1_dn14 * assign57410_e89398) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57410_e89396) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57410_e89394) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57410_e89392) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57410_e89390) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign57410_e89388))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign57410_e89402;
        locals.var_tmf2_dn0 = assign57410_e89402_d_n0;
        locals.var_tmf2_dn2 = assign57410_e89402_d_n2;
        locals.var_tmf2_dn4 = assign57410_e89402_d_n4;
        locals.var_tmf2_dn5 = assign57410_e89402_d_n5;
        locals.var_tmf2_dn6 = assign57410_e89402_d_n6;
        locals.var_tmf2_dn7 = assign57410_e89402_d_n7;
        locals.var_tmf2_dn8 = assign57410_e89402_d_n8;
        locals.var_tmf2_dn9 = assign57410_e89402_d_n9;
        locals.var_tmf2_dn10 = assign57410_e89402_d_n10;
        locals.var_tmf2_dn11 = assign57410_e89402_d_n11;
        locals.var_tmf2_dn14 = assign57410_e89402_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign57420_e89447, assign57420_e89447_d_n0, assign57420_e89447_d_n2, assign57420_e89447_d_n4, assign57420_e89447_d_n5, assign57420_e89447_d_n6, assign57420_e89447_d_n7, assign57420_e89447_d_n8, assign57420_e89447_d_n9, assign57420_e89447_d_n10, assign57420_e89447_d_n11, assign57420_e89447_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57420_e89415: f64 = (1.0 / 2.0);
        let assign57420_e89419: f64 = (1.0 / 3.0);
        let assign57420_e89423: f64 = (1.0 / 8.0);
        let assign57420_e89427: f64 = (1.0 / 30.0);
        let assign57420_e89431: f64 = (1.0 / 144.0);
        let assign57420_e89435: f64 = (1.0 / 840.0);
        let assign57420_e89436: f64 = (locals.var_tmf1 * assign57420_e89435);
        let assign57420_e89437: f64 = (assign57420_e89431 + assign57420_e89436);
        let assign57420_e89438: f64 = (locals.var_tmf1 * assign57420_e89437);
        let assign57420_e89439: f64 = (assign57420_e89427 + assign57420_e89438);
        let assign57420_e89440: f64 = (locals.var_tmf1 * assign57420_e89439);
        let assign57420_e89441: f64 = (assign57420_e89423 + assign57420_e89440);
        let assign57420_e89442: f64 = (locals.var_tmf1 * assign57420_e89441);
        let assign57420_e89443: f64 = (assign57420_e89419 + assign57420_e89442);
        let assign57420_e89444: f64 = (locals.var_tmf1 * assign57420_e89443);
        let assign57420_e89445: f64 = (assign57420_e89415 + assign57420_e89444);
        (assign57420_e89445, ((locals.var_tmf1_dn0 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign57420_e89435))))))))), ((locals.var_tmf1_dn2 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign57420_e89435))))))))), ((locals.var_tmf1_dn4 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign57420_e89435))))))))), ((locals.var_tmf1_dn5 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign57420_e89435))))))))), ((locals.var_tmf1_dn6 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign57420_e89435))))))))), ((locals.var_tmf1_dn7 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign57420_e89435))))))))), ((locals.var_tmf1_dn8 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign57420_e89435))))))))), ((locals.var_tmf1_dn9 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign57420_e89435))))))))), ((locals.var_tmf1_dn10 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign57420_e89435))))))))), ((locals.var_tmf1_dn11 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign57420_e89435))))))))), ((locals.var_tmf1_dn14 * assign57420_e89443) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57420_e89441) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57420_e89439) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57420_e89437) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign57420_e89435))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign57420_e89447;
        locals.var_tmf3_dn0 = assign57420_e89447_d_n0;
        locals.var_tmf3_dn2 = assign57420_e89447_d_n2;
        locals.var_tmf3_dn4 = assign57420_e89447_d_n4;
        locals.var_tmf3_dn5 = assign57420_e89447_d_n5;
        locals.var_tmf3_dn6 = assign57420_e89447_d_n6;
        locals.var_tmf3_dn7 = assign57420_e89447_d_n7;
        locals.var_tmf3_dn8 = assign57420_e89447_d_n8;
        locals.var_tmf3_dn9 = assign57420_e89447_d_n9;
        locals.var_tmf3_dn10 = assign57420_e89447_d_n10;
        locals.var_tmf3_dn11 = assign57420_e89447_d_n11;
        locals.var_tmf3_dn14 = assign57420_e89447_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign57430_e89462, assign57430_e89462_d_n0, assign57430_e89462_d_n2, assign57430_e89462_d_n4, assign57430_e89462_d_n5, assign57430_e89462_d_n6, assign57430_e89462_d_n7, assign57430_e89462_d_n8, assign57430_e89462_d_n9, assign57430_e89462_d_n10, assign57430_e89462_d_n11, assign57430_e89462_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57430_e89460: f64 = (0.01 / locals.var_tmf2);
        (assign57430_e89460, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign57430_e89462;
        locals.var_t6_dn0 = assign57430_e89462_d_n0;
        locals.var_t6_dn2 = assign57430_e89462_d_n2;
        locals.var_t6_dn4 = assign57430_e89462_d_n4;
        locals.var_t6_dn5 = assign57430_e89462_d_n5;
        locals.var_t6_dn6 = assign57430_e89462_d_n6;
        locals.var_t6_dn7 = assign57430_e89462_d_n7;
        locals.var_t6_dn8 = assign57430_e89462_d_n8;
        locals.var_t6_dn9 = assign57430_e89462_d_n9;
        locals.var_t6_dn10 = assign57430_e89462_d_n10;
        locals.var_t6_dn11 = assign57430_e89462_d_n11;
        locals.var_t6_dn14 = assign57430_e89462_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign57440_e89482, assign57440_e89482_d_n0, assign57440_e89482_d_n2, assign57440_e89482_d_n4, assign57440_e89482_d_n5, assign57440_e89482_d_n6, assign57440_e89482_d_n7, assign57440_e89482_d_n8, assign57440_e89482_d_n9, assign57440_e89482_d_n10, assign57440_e89482_d_n11, assign57440_e89482_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57440_e89474: f64 = (-2.0);
        let assign57440_e89476: f64 = (assign57440_e89474 * locals.var_tmf3);
        let assign57440_e89479: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign57440_e89480: f64 = (assign57440_e89476 / assign57440_e89479);
        (assign57440_e89480, ((((assign57440_e89474 * locals.var_tmf3_dn0) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn2) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn4) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn5) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn6) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn7) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn8) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn9) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn10) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn11) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign57440_e89479 * assign57440_e89479)), ((((assign57440_e89474 * locals.var_tmf3_dn14) * assign57440_e89479) - (assign57440_e89476 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign57440_e89479 * assign57440_e89479)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57440_e89482;
        locals.var_t2_dn0 = assign57440_e89482_d_n0;
        locals.var_t2_dn2 = assign57440_e89482_d_n2;
        locals.var_t2_dn4 = assign57440_e89482_d_n4;
        locals.var_t2_dn5 = assign57440_e89482_d_n5;
        locals.var_t2_dn6 = assign57440_e89482_d_n6;
        locals.var_t2_dn7 = assign57440_e89482_d_n7;
        locals.var_t2_dn8 = assign57440_e89482_d_n8;
        locals.var_t2_dn9 = assign57440_e89482_d_n9;
        locals.var_t2_dn10 = assign57440_e89482_d_n10;
        locals.var_t2_dn11 = assign57440_e89482_d_n11;
        locals.var_t2_dn14 = assign57440_e89482_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57450_e89499, assign57450_e89499_d_n0, assign57450_e89499_d_n2, assign57450_e89499_d_n4, assign57450_e89499_d_n5, assign57450_e89499_d_n6, assign57450_e89499_d_n7, assign57450_e89499_d_n8, assign57450_e89499_d_n9, assign57450_e89499_d_n10, assign57450_e89499_d_n11, assign57450_e89499_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57450_e89496: f64 = (locals.var_phi_s0_dep__blk1093 + locals.var_t6);
        let assign57450_e89497: f64 = (1.1 - assign57450_e89496);
        (assign57450_e89497, (-(locals.var_phi_s0_dep__blk1093_dn0 + locals.var_t6_dn0)), (-(locals.var_phi_s0_dep__blk1093_dn2 + locals.var_t6_dn2)), (-(locals.var_phi_s0_dep__blk1093_dn4 + locals.var_t6_dn4)), (-(locals.var_phi_s0_dep__blk1093_dn5 + locals.var_t6_dn5)), (-(locals.var_phi_s0_dep__blk1093_dn6 + locals.var_t6_dn6)), (-(locals.var_phi_s0_dep__blk1093_dn7 + locals.var_t6_dn7)), (-(locals.var_phi_s0_dep__blk1093_dn8 + locals.var_t6_dn8)), (-(locals.var_phi_s0_dep__blk1093_dn9 + locals.var_t6_dn9)), (-(locals.var_phi_s0_dep__blk1093_dn10 + locals.var_t6_dn10)), (-(locals.var_phi_s0_dep__blk1093_dn11 + locals.var_t6_dn11)), (-(locals.var_phi_s0_dep__blk1093_dn14 + locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57450_e89499;
        locals.var_t1_dn0 = assign57450_e89499_d_n0;
        locals.var_t1_dn2 = assign57450_e89499_d_n2;
        locals.var_t1_dn4 = assign57450_e89499_d_n4;
        locals.var_t1_dn5 = assign57450_e89499_d_n5;
        locals.var_t1_dn6 = assign57450_e89499_d_n6;
        locals.var_t1_dn7 = assign57450_e89499_d_n7;
        locals.var_t1_dn8 = assign57450_e89499_d_n8;
        locals.var_t1_dn9 = assign57450_e89499_d_n9;
        locals.var_t1_dn10 = assign57450_e89499_d_n10;
        locals.var_t1_dn11 = assign57450_e89499_d_n11;
        locals.var_t1_dn14 = assign57450_e89499_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57460_e89521, assign57460_e89521_d_n0, assign57460_e89521_d_n2, assign57460_e89521_d_n4, assign57460_e89521_d_n5, assign57460_e89521_d_n6, assign57460_e89521_d_n7, assign57460_e89521_d_n8, assign57460_e89521_d_n9, assign57460_e89521_d_n10, assign57460_e89521_d_n11, assign57460_e89521_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57460_e89512: f64 = (locals.var_t1 * locals.var_t1);
        let assign57460_e89515: f64 = (4.0 * 0.05);
        let assign57460_e89517: f64 = (assign57460_e89515 * 0.05);
        let assign57460_e89518: f64 = (assign57460_e89512 + assign57460_e89517);
        let assign57460_e89519: f64 = (assign57460_e89518).sqrt();
        (assign57460_e89519, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign57460_e89519)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign57460_e89519)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign57460_e89521;
        locals.var_tmf2_dn0 = assign57460_e89521_d_n0;
        locals.var_tmf2_dn2 = assign57460_e89521_d_n2;
        locals.var_tmf2_dn4 = assign57460_e89521_d_n4;
        locals.var_tmf2_dn5 = assign57460_e89521_d_n5;
        locals.var_tmf2_dn6 = assign57460_e89521_d_n6;
        locals.var_tmf2_dn7 = assign57460_e89521_d_n7;
        locals.var_tmf2_dn8 = assign57460_e89521_d_n8;
        locals.var_tmf2_dn9 = assign57460_e89521_d_n9;
        locals.var_tmf2_dn10 = assign57460_e89521_d_n10;
        locals.var_tmf2_dn11 = assign57460_e89521_d_n11;
        locals.var_tmf2_dn14 = assign57460_e89521_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign57470_e89540, assign57470_e89540_d_n0, assign57470_e89540_d_n2, assign57470_e89540_d_n4, assign57470_e89540_d_n5, assign57470_e89540_d_n6, assign57470_e89540_d_n7, assign57470_e89540_d_n8, assign57470_e89540_d_n9, assign57470_e89540_d_n10, assign57470_e89540_d_n11, assign57470_e89540_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57470_e89536: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign57470_e89537: f64 = (1.0 + assign57470_e89536);
        let assign57470_e89538: f64 = (0.5 * assign57470_e89537);
        (assign57470_e89538, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57470_e89540;
        locals.var_t0_dn0 = assign57470_e89540_d_n0;
        locals.var_t0_dn2 = assign57470_e89540_d_n2;
        locals.var_t0_dn4 = assign57470_e89540_d_n4;
        locals.var_t0_dn5 = assign57470_e89540_d_n5;
        locals.var_t0_dn6 = assign57470_e89540_d_n6;
        locals.var_t0_dn7 = assign57470_e89540_d_n7;
        locals.var_t0_dn8 = assign57470_e89540_d_n8;
        locals.var_t0_dn9 = assign57470_e89540_d_n9;
        locals.var_t0_dn10 = assign57470_e89540_d_n10;
        locals.var_t0_dn11 = assign57470_e89540_d_n11;
        locals.var_t0_dn14 = assign57470_e89540_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57480_e89557, assign57480_e89557_d_n0, assign57480_e89557_d_n2, assign57480_e89557_d_n4, assign57480_e89557_d_n5, assign57480_e89557_d_n6, assign57480_e89557_d_n7, assign57480_e89557_d_n8, assign57480_e89557_d_n9, assign57480_e89557_d_n10, assign57480_e89557_d_n11, assign57480_e89557_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57480_e89554: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign57480_e89555: f64 = (0.5 * assign57480_e89554);
        (assign57480_e89555, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57480_e89557;
        locals.var_t2_dn0 = assign57480_e89557_d_n0;
        locals.var_t2_dn2 = assign57480_e89557_d_n2;
        locals.var_t2_dn4 = assign57480_e89557_d_n4;
        locals.var_t2_dn5 = assign57480_e89557_d_n5;
        locals.var_t2_dn6 = assign57480_e89557_d_n6;
        locals.var_t2_dn7 = assign57480_e89557_d_n7;
        locals.var_t2_dn8 = assign57480_e89557_d_n8;
        locals.var_t2_dn9 = assign57480_e89557_d_n9;
        locals.var_t2_dn10 = assign57480_e89557_d_n10;
        locals.var_t2_dn11 = assign57480_e89557_d_n11;
        locals.var_t2_dn14 = assign57480_e89557_d_n14;
        locals.var_t2_rv = 0.0;

        let assign57490_e89560: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1428 = assign57490_e89560;
        locals.var_guard1428_rv = 0.0;

        let (assign57500_e89575, assign57500_e89575_d_n0, assign57500_e89575_d_n2, assign57500_e89575_d_n4, assign57500_e89575_d_n5, assign57500_e89575_d_n6, assign57500_e89575_d_n7, assign57500_e89575_d_n8, assign57500_e89575_d_n9, assign57500_e89575_d_n10, assign57500_e89575_d_n11, assign57500_e89575_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) && (locals.var_guard1428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57500_e89575;
        locals.var_t2_dn0 = assign57500_e89575_d_n0;
        locals.var_t2_dn2 = assign57500_e89575_d_n2;
        locals.var_t2_dn4 = assign57500_e89575_d_n4;
        locals.var_t2_dn5 = assign57500_e89575_d_n5;
        locals.var_t2_dn6 = assign57500_e89575_d_n6;
        locals.var_t2_dn7 = assign57500_e89575_d_n7;
        locals.var_t2_dn8 = assign57500_e89575_d_n8;
        locals.var_t2_dn9 = assign57500_e89575_d_n9;
        locals.var_t2_dn10 = assign57500_e89575_d_n10;
        locals.var_t2_dn11 = assign57500_e89575_d_n11;
        locals.var_t2_dn14 = assign57500_e89575_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57510_e89590, assign57510_e89590_d_n0, assign57510_e89590_d_n2, assign57510_e89590_d_n4, assign57510_e89590_d_n5, assign57510_e89590_d_n6, assign57510_e89590_d_n7, assign57510_e89590_d_n8, assign57510_e89590_d_n9, assign57510_e89590_d_n10, assign57510_e89590_d_n11, assign57510_e89590_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) && (locals.var_guard1428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57510_e89590;
        locals.var_t0_dn0 = assign57510_e89590_d_n0;
        locals.var_t0_dn2 = assign57510_e89590_d_n2;
        locals.var_t0_dn4 = assign57510_e89590_d_n4;
        locals.var_t0_dn5 = assign57510_e89590_d_n5;
        locals.var_t0_dn6 = assign57510_e89590_d_n6;
        locals.var_t0_dn7 = assign57510_e89590_d_n7;
        locals.var_t0_dn8 = assign57510_e89590_d_n8;
        locals.var_t0_dn9 = assign57510_e89590_d_n9;
        locals.var_t0_dn10 = assign57510_e89590_d_n10;
        locals.var_t0_dn11 = assign57510_e89590_d_n11;
        locals.var_t0_dn14 = assign57510_e89590_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57520_e89605, assign57520_e89605_d_n0, assign57520_e89605_d_n2, assign57520_e89605_d_n4, assign57520_e89605_d_n5, assign57520_e89605_d_n6, assign57520_e89605_d_n7, assign57520_e89605_d_n8, assign57520_e89605_d_n9, assign57520_e89605_d_n10, assign57520_e89605_d_n11, assign57520_e89605_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57520_e89603: f64 = (locals.var_t2 + 1e-25);
        (assign57520_e89603, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57520_e89605;
        locals.var_t2_dn0 = assign57520_e89605_d_n0;
        locals.var_t2_dn2 = assign57520_e89605_d_n2;
        locals.var_t2_dn4 = assign57520_e89605_d_n4;
        locals.var_t2_dn5 = assign57520_e89605_d_n5;
        locals.var_t2_dn6 = assign57520_e89605_d_n6;
        locals.var_t2_dn7 = assign57520_e89605_d_n7;
        locals.var_t2_dn8 = assign57520_e89605_d_n8;
        locals.var_t2_dn9 = assign57520_e89605_d_n9;
        locals.var_t2_dn10 = assign57520_e89605_d_n10;
        locals.var_t2_dn11 = assign57520_e89605_d_n11;
        locals.var_t2_dn14 = assign57520_e89605_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57530_e89620, assign57530_e89620_d_n0, assign57530_e89620_d_n2, assign57530_e89620_d_n4, assign57530_e89620_d_n5, assign57530_e89620_d_n6, assign57530_e89620_d_n7, assign57530_e89620_d_n8, assign57530_e89620_d_n9, assign57530_e89620_d_n10, assign57530_e89620_d_n11, assign57530_e89620_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57530_e89618: f64 = (locals.var_beta * locals.var_ptl0);
        (assign57530_e89618, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn11 * locals.var_ptl0), (locals.var_beta_dn14 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57530_e89620;
        locals.var_t0_dn0 = assign57530_e89620_d_n0;
        locals.var_t0_dn2 = assign57530_e89620_d_n2;
        locals.var_t0_dn4 = assign57530_e89620_d_n4;
        locals.var_t0_dn5 = assign57530_e89620_d_n5;
        locals.var_t0_dn6 = assign57530_e89620_d_n6;
        locals.var_t0_dn7 = assign57530_e89620_d_n7;
        locals.var_t0_dn8 = assign57530_e89620_d_n8;
        locals.var_t0_dn9 = assign57530_e89620_d_n9;
        locals.var_t0_dn10 = assign57530_e89620_d_n10;
        locals.var_t0_dn11 = assign57530_e89620_d_n11;
        locals.var_t0_dn14 = assign57530_e89620_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57540_e89635, assign57540_e89635_d_n0, assign57540_e89635_d_n2, assign57540_e89635_d_n4, assign57540_e89635_d_n5, assign57540_e89635_d_n6, assign57540_e89635_d_n7, assign57540_e89635_d_n8, assign57540_e89635_d_n9, assign57540_e89635_d_n10, assign57540_e89635_d_n11, assign57540_e89635_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57540_e89633: f64 = (locals.var_cox * locals.var_t0);
        (assign57540_e89633, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn11 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn11)), ((locals.var_cox_dn14 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign57540_e89635;
        locals.var_t3_dn0 = assign57540_e89635_d_n0;
        locals.var_t3_dn2 = assign57540_e89635_d_n2;
        locals.var_t3_dn4 = assign57540_e89635_d_n4;
        locals.var_t3_dn5 = assign57540_e89635_d_n5;
        locals.var_t3_dn6 = assign57540_e89635_d_n6;
        locals.var_t3_dn7 = assign57540_e89635_d_n7;
        locals.var_t3_dn8 = assign57540_e89635_d_n8;
        locals.var_t3_dn9 = assign57540_e89635_d_n9;
        locals.var_t3_dn10 = assign57540_e89635_d_n10;
        locals.var_t3_dn11 = assign57540_e89635_d_n11;
        locals.var_t3_dn14 = assign57540_e89635_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign57550_e89650, assign57550_e89650_d_n0, assign57550_e89650_d_n2, assign57550_e89650_d_n4, assign57550_e89650_d_n5, assign57550_e89650_d_n6, assign57550_e89650_d_n7, assign57550_e89650_d_n8, assign57550_e89650_d_n9, assign57550_e89650_d_n10, assign57550_e89650_d_n11, assign57550_e89650_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57550_e89648: f64 = (locals.var_t2).powf(p.p284);
        (assign57550_e89648, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn11)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn11 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn14)) } } else { (assign57550_e89648 * (p.p284 * (locals.var_t2_dn14 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57550_e89650;
        locals.var_t0_dn0 = assign57550_e89650_d_n0;
        locals.var_t0_dn2 = assign57550_e89650_d_n2;
        locals.var_t0_dn4 = assign57550_e89650_d_n4;
        locals.var_t0_dn5 = assign57550_e89650_d_n5;
        locals.var_t0_dn6 = assign57550_e89650_d_n6;
        locals.var_t0_dn7 = assign57550_e89650_d_n7;
        locals.var_t0_dn8 = assign57550_e89650_d_n8;
        locals.var_t0_dn9 = assign57550_e89650_d_n9;
        locals.var_t0_dn10 = assign57550_e89650_d_n10;
        locals.var_t0_dn11 = assign57550_e89650_d_n11;
        locals.var_t0_dn14 = assign57550_e89650_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57560_e89665, assign57560_e89665_d_n0, assign57560_e89665_d_n2, assign57560_e89665_d_n4, assign57560_e89665_d_n5, assign57560_e89665_d_n6, assign57560_e89665_d_n7, assign57560_e89665_d_n8, assign57560_e89665_d_n9, assign57560_e89665_d_n10, assign57560_e89665_d_n11, assign57560_e89665_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57560_e89663: f64 = (locals.var_t3 * locals.var_t0);
        (assign57560_e89663, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11)), ((locals.var_t3_dn14 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign57560_e89665;
        locals.var_t9_dn0 = assign57560_e89665_d_n0;
        locals.var_t9_dn2 = assign57560_e89665_d_n2;
        locals.var_t9_dn4 = assign57560_e89665_d_n4;
        locals.var_t9_dn5 = assign57560_e89665_d_n5;
        locals.var_t9_dn6 = assign57560_e89665_d_n6;
        locals.var_t9_dn7 = assign57560_e89665_d_n7;
        locals.var_t9_dn8 = assign57560_e89665_d_n8;
        locals.var_t9_dn9 = assign57560_e89665_d_n9;
        locals.var_t9_dn10 = assign57560_e89665_d_n10;
        locals.var_t9_dn11 = assign57560_e89665_d_n11;
        locals.var_t9_dn14 = assign57560_e89665_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign57570_e89682, assign57570_e89682_d_n0, assign57570_e89682_d_n2, assign57570_e89682_d_n4, assign57570_e89682_d_n5, assign57570_e89682_d_n6, assign57570_e89682_d_n7, assign57570_e89682_d_n8, assign57570_e89682_d_n9, assign57570_e89682_d_n10, assign57570_e89682_d_n11, assign57570_e89682_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57570_e89679: f64 = (locals.var_vdsz__blk443 * p.p285);
        let assign57570_e89680: f64 = (1.0 + assign57570_e89679);
        (assign57570_e89680, (locals.var_vdsz__blk443_dn0 * p.p285), (locals.var_vdsz__blk443_dn2 * p.p285), (locals.var_vdsz__blk443_dn4 * p.p285), (locals.var_vdsz__blk443_dn5 * p.p285), (locals.var_vdsz__blk443_dn6 * p.p285), (locals.var_vdsz__blk443_dn7 * p.p285), (locals.var_vdsz__blk443_dn8 * p.p285), (locals.var_vdsz__blk443_dn9 * p.p285), (locals.var_vdsz__blk443_dn10 * p.p285), (locals.var_vdsz__blk443_dn11 * p.p285), (locals.var_vdsz__blk443_dn14 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57570_e89682;
        locals.var_t4_dn0 = assign57570_e89682_d_n0;
        locals.var_t4_dn2 = assign57570_e89682_d_n2;
        locals.var_t4_dn4 = assign57570_e89682_d_n4;
        locals.var_t4_dn5 = assign57570_e89682_d_n5;
        locals.var_t4_dn6 = assign57570_e89682_d_n6;
        locals.var_t4_dn7 = assign57570_e89682_d_n7;
        locals.var_t4_dn8 = assign57570_e89682_d_n8;
        locals.var_t4_dn9 = assign57570_e89682_d_n9;
        locals.var_t4_dn10 = assign57570_e89682_d_n10;
        locals.var_t4_dn11 = assign57570_e89682_d_n11;
        locals.var_t4_dn14 = assign57570_e89682_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57580_e89695, assign57580_e89695_d_n0, assign57580_e89695_d_n2, assign57580_e89695_d_n4, assign57580_e89695_d_n5, assign57580_e89695_d_n6, assign57580_e89695_d_n7, assign57580_e89695_d_n8, assign57580_e89695_d_n9, assign57580_e89695_d_n10, assign57580_e89695_d_n11, assign57580_e89695_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57580_e89695;
        locals.var_t0_dn0 = assign57580_e89695_d_n0;
        locals.var_t0_dn2 = assign57580_e89695_d_n2;
        locals.var_t0_dn4 = assign57580_e89695_d_n4;
        locals.var_t0_dn5 = assign57580_e89695_d_n5;
        locals.var_t0_dn6 = assign57580_e89695_d_n6;
        locals.var_t0_dn7 = assign57580_e89695_d_n7;
        locals.var_t0_dn8 = assign57580_e89695_d_n8;
        locals.var_t0_dn9 = assign57580_e89695_d_n9;
        locals.var_t0_dn10 = assign57580_e89695_d_n10;
        locals.var_t0_dn11 = assign57580_e89695_d_n11;
        locals.var_t0_dn14 = assign57580_e89695_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57590_e89712, assign57590_e89712_d_n0, assign57590_e89712_d_n2, assign57590_e89712_d_n4, assign57590_e89712_d_n5, assign57590_e89712_d_n6, assign57590_e89712_d_n7, assign57590_e89712_d_n8, assign57590_e89712_d_n9, assign57590_e89712_d_n10, assign57590_e89712_d_n11, assign57590_e89712_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57590_e89708: f64 = (locals.var_phi_s0_dep__blk1093 + locals.var_t6);
        let assign57590_e89710: f64 = (assign57590_e89708 - locals.var_vbsz__blk442);
        (assign57590_e89710, ((locals.var_phi_s0_dep__blk1093_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk442_dn0), ((locals.var_phi_s0_dep__blk1093_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk442_dn2), ((locals.var_phi_s0_dep__blk1093_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk442_dn4), ((locals.var_phi_s0_dep__blk1093_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk442_dn5), ((locals.var_phi_s0_dep__blk1093_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk442_dn6), ((locals.var_phi_s0_dep__blk1093_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk442_dn7), ((locals.var_phi_s0_dep__blk1093_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk442_dn8), ((locals.var_phi_s0_dep__blk1093_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk442_dn9), ((locals.var_phi_s0_dep__blk1093_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk442_dn10), ((locals.var_phi_s0_dep__blk1093_dn11 + locals.var_t6_dn11) - locals.var_vbsz__blk442_dn11), ((locals.var_phi_s0_dep__blk1093_dn14 + locals.var_t6_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign57590_e89712;
        locals.var_t5_dn0 = assign57590_e89712_d_n0;
        locals.var_t5_dn2 = assign57590_e89712_d_n2;
        locals.var_t5_dn4 = assign57590_e89712_d_n4;
        locals.var_t5_dn5 = assign57590_e89712_d_n5;
        locals.var_t5_dn6 = assign57590_e89712_d_n6;
        locals.var_t5_dn7 = assign57590_e89712_d_n7;
        locals.var_t5_dn8 = assign57590_e89712_d_n8;
        locals.var_t5_dn9 = assign57590_e89712_d_n9;
        locals.var_t5_dn10 = assign57590_e89712_d_n10;
        locals.var_t5_dn11 = assign57590_e89712_d_n11;
        locals.var_t5_dn14 = assign57590_e89712_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign57600_e89731, assign57600_e89731_d_n0, assign57600_e89731_d_n2, assign57600_e89731_d_n4, assign57600_e89731_d_n5, assign57600_e89731_d_n6, assign57600_e89731_d_n7, assign57600_e89731_d_n8, assign57600_e89731_d_n9, assign57600_e89731_d_n10, assign57600_e89731_d_n11, assign57600_e89731_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57600_e89726: f64 = (locals.var_vdsz__blk443 * locals.var_t0);
        let assign57600_e89728: f64 = (assign57600_e89726 * locals.var_t5);
        let assign57600_e89729: f64 = (locals.var_t4 + assign57600_e89728);
        (assign57600_e89729, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk443_dn0 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn0)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk443_dn2 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn2)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk443_dn4 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn4)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk443_dn5 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn5)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk443_dn6 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn6)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk443_dn7 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn7)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk443_dn8 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn8)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk443_dn9 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn9)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk443_dn10 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn10)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn10))), (locals.var_t4_dn11 + ((((locals.var_vdsz__blk443_dn11 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn11)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn11))), (locals.var_t4_dn14 + ((((locals.var_vdsz__blk443_dn14 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn14)) * locals.var_t5) + (assign57600_e89726 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57600_e89731;
        locals.var_t4_dn0 = assign57600_e89731_d_n0;
        locals.var_t4_dn2 = assign57600_e89731_d_n2;
        locals.var_t4_dn4 = assign57600_e89731_d_n4;
        locals.var_t4_dn5 = assign57600_e89731_d_n5;
        locals.var_t4_dn6 = assign57600_e89731_d_n6;
        locals.var_t4_dn7 = assign57600_e89731_d_n7;
        locals.var_t4_dn8 = assign57600_e89731_d_n8;
        locals.var_t4_dn9 = assign57600_e89731_d_n9;
        locals.var_t4_dn10 = assign57600_e89731_d_n10;
        locals.var_t4_dn11 = assign57600_e89731_d_n11;
        locals.var_t4_dn14 = assign57600_e89731_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57610_e89746, assign57610_e89746_d_n0, assign57610_e89746_d_n2, assign57610_e89746_d_n4, assign57610_e89746_d_n5, assign57610_e89746_d_n6, assign57610_e89746_d_n7, assign57610_e89746_d_n8, assign57610_e89746_d_n9, assign57610_e89746_d_n10, assign57610_e89746_d_n11, assign57610_e89746_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57610_e89744: f64 = (locals.var_t9 * locals.var_t4);
        (assign57610_e89744, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn11 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn11)), ((locals.var_t9_dn14 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign57610_e89746;
        locals.var_t6_dn0 = assign57610_e89746_d_n0;
        locals.var_t6_dn2 = assign57610_e89746_d_n2;
        locals.var_t6_dn4 = assign57610_e89746_d_n4;
        locals.var_t6_dn5 = assign57610_e89746_d_n5;
        locals.var_t6_dn6 = assign57610_e89746_d_n6;
        locals.var_t6_dn7 = assign57610_e89746_d_n7;
        locals.var_t6_dn8 = assign57610_e89746_d_n8;
        locals.var_t6_dn9 = assign57610_e89746_d_n9;
        locals.var_t6_dn10 = assign57610_e89746_d_n10;
        locals.var_t6_dn11 = assign57610_e89746_d_n11;
        locals.var_t6_dn14 = assign57610_e89746_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign57620_e89759, assign57620_e89759_d_n0, assign57620_e89759_d_n2, assign57620_e89759_d_n4, assign57620_e89759_d_n5, assign57620_e89759_d_n6, assign57620_e89759_d_n7, assign57620_e89759_d_n8, assign57620_e89759_d_n9, assign57620_e89759_d_n10, assign57620_e89759_d_n11, assign57620_e89759_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign57620_e89759;
        locals.var_t9_dn0 = assign57620_e89759_d_n0;
        locals.var_t9_dn2 = assign57620_e89759_d_n2;
        locals.var_t9_dn4 = assign57620_e89759_d_n4;
        locals.var_t9_dn5 = assign57620_e89759_d_n5;
        locals.var_t9_dn6 = assign57620_e89759_d_n6;
        locals.var_t9_dn7 = assign57620_e89759_d_n7;
        locals.var_t9_dn8 = assign57620_e89759_d_n8;
        locals.var_t9_dn9 = assign57620_e89759_d_n9;
        locals.var_t9_dn10 = assign57620_e89759_d_n10;
        locals.var_t9_dn11 = assign57620_e89759_d_n11;
        locals.var_t9_dn14 = assign57620_e89759_d_n14;
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_211(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign57630_e89773, assign57630_e89773_d_n0, assign57630_e89773_d_n2, assign57630_e89773_d_n4, assign57630_e89773_d_n5, assign57630_e89773_d_n6, assign57630_e89773_d_n7, assign57630_e89773_d_n8, assign57630_e89773_d_n9, assign57630_e89773_d_n10, assign57630_e89773_d_n11, assign57630_e89773_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1427 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign57630_e89773;
        locals.var_t9_dn0 = assign57630_e89773_d_n0;
        locals.var_t9_dn2 = assign57630_e89773_d_n2;
        locals.var_t9_dn4 = assign57630_e89773_d_n4;
        locals.var_t9_dn5 = assign57630_e89773_d_n5;
        locals.var_t9_dn6 = assign57630_e89773_d_n6;
        locals.var_t9_dn7 = assign57630_e89773_d_n7;
        locals.var_t9_dn8 = assign57630_e89773_d_n8;
        locals.var_t9_dn9 = assign57630_e89773_d_n9;
        locals.var_t9_dn10 = assign57630_e89773_d_n10;
        locals.var_t9_dn11 = assign57630_e89773_d_n11;
        locals.var_t9_dn14 = assign57630_e89773_d_n14;
        locals.var_t9_rv = 0.0;

        let assign57640_e89776: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1429 = assign57640_e89776;
        locals.var_guard1429_rv = 0.0;

        let (assign57650_e89791, assign57650_e89791_d_n0, assign57650_e89791_d_n2, assign57650_e89791_d_n4, assign57650_e89791_d_n5, assign57650_e89791_d_n6, assign57650_e89791_d_n7, assign57650_e89791_d_n8, assign57650_e89791_d_n9, assign57650_e89791_d_n10, assign57650_e89791_d_n11, assign57650_e89791_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1429 != 0.0)) {
        let assign57650_e89789: f64 = (locals.var_beta * locals.var_gdl0);
        (assign57650_e89789, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn11 * locals.var_gdl0), (locals.var_beta_dn14 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57650_e89791;
        locals.var_t1_dn0 = assign57650_e89791_d_n0;
        locals.var_t1_dn2 = assign57650_e89791_d_n2;
        locals.var_t1_dn4 = assign57650_e89791_d_n4;
        locals.var_t1_dn5 = assign57650_e89791_d_n5;
        locals.var_t1_dn6 = assign57650_e89791_d_n6;
        locals.var_t1_dn7 = assign57650_e89791_d_n7;
        locals.var_t1_dn8 = assign57650_e89791_d_n8;
        locals.var_t1_dn9 = assign57650_e89791_d_n9;
        locals.var_t1_dn10 = assign57650_e89791_d_n10;
        locals.var_t1_dn11 = assign57650_e89791_d_n11;
        locals.var_t1_dn14 = assign57650_e89791_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57660_e89806, assign57660_e89806_d_n0, assign57660_e89806_d_n2, assign57660_e89806_d_n4, assign57660_e89806_d_n5, assign57660_e89806_d_n6, assign57660_e89806_d_n7, assign57660_e89806_d_n8, assign57660_e89806_d_n9, assign57660_e89806_d_n10, assign57660_e89806_d_n11, assign57660_e89806_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1429 != 0.0)) {
        let assign57660_e89804: f64 = (locals.var_cox * locals.var_t1);
        (assign57660_e89804, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn11 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn11)), ((locals.var_cox_dn14 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57660_e89806;
        locals.var_t2_dn0 = assign57660_e89806_d_n0;
        locals.var_t2_dn2 = assign57660_e89806_d_n2;
        locals.var_t2_dn4 = assign57660_e89806_d_n4;
        locals.var_t2_dn5 = assign57660_e89806_d_n5;
        locals.var_t2_dn6 = assign57660_e89806_d_n6;
        locals.var_t2_dn7 = assign57660_e89806_d_n7;
        locals.var_t2_dn8 = assign57660_e89806_d_n8;
        locals.var_t2_dn9 = assign57660_e89806_d_n9;
        locals.var_t2_dn10 = assign57660_e89806_d_n10;
        locals.var_t2_dn11 = assign57660_e89806_d_n11;
        locals.var_t2_dn14 = assign57660_e89806_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57670_e89821, assign57670_e89821_d_n0, assign57670_e89821_d_n2, assign57670_e89821_d_n4, assign57670_e89821_d_n5, assign57670_e89821_d_n6, assign57670_e89821_d_n7, assign57670_e89821_d_n8, assign57670_e89821_d_n9, assign57670_e89821_d_n10, assign57670_e89821_d_n11, assign57670_e89821_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1429 != 0.0)) {
        let assign57670_e89819: f64 = (locals.var_t2 * locals.var_vdsz__blk443);
        (assign57670_e89819, ((locals.var_t2_dn0 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn10)), ((locals.var_t2_dn11 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn11)), ((locals.var_t2_dn14 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign57670_e89821;
        locals.var_t8_dn0 = assign57670_e89821_d_n0;
        locals.var_t8_dn2 = assign57670_e89821_d_n2;
        locals.var_t8_dn4 = assign57670_e89821_d_n4;
        locals.var_t8_dn5 = assign57670_e89821_d_n5;
        locals.var_t8_dn6 = assign57670_e89821_d_n6;
        locals.var_t8_dn7 = assign57670_e89821_d_n7;
        locals.var_t8_dn8 = assign57670_e89821_d_n8;
        locals.var_t8_dn9 = assign57670_e89821_d_n9;
        locals.var_t8_dn10 = assign57670_e89821_d_n10;
        locals.var_t8_dn11 = assign57670_e89821_d_n11;
        locals.var_t8_dn14 = assign57670_e89821_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign57680_e89835, assign57680_e89835_d_n0, assign57680_e89835_d_n2, assign57680_e89835_d_n4, assign57680_e89835_d_n5, assign57680_e89835_d_n6, assign57680_e89835_d_n7, assign57680_e89835_d_n8, assign57680_e89835_d_n9, assign57680_e89835_d_n10, assign57680_e89835_d_n11, assign57680_e89835_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1429 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign57680_e89835;
        locals.var_t8_dn0 = assign57680_e89835_d_n0;
        locals.var_t8_dn2 = assign57680_e89835_d_n2;
        locals.var_t8_dn4 = assign57680_e89835_d_n4;
        locals.var_t8_dn5 = assign57680_e89835_d_n5;
        locals.var_t8_dn6 = assign57680_e89835_d_n6;
        locals.var_t8_dn7 = assign57680_e89835_d_n7;
        locals.var_t8_dn8 = assign57680_e89835_d_n8;
        locals.var_t8_dn9 = assign57680_e89835_d_n9;
        locals.var_t8_dn10 = assign57680_e89835_d_n10;
        locals.var_t8_dn11 = assign57680_e89835_d_n11;
        locals.var_t8_dn14 = assign57680_e89835_d_n14;
        locals.var_t8_rv = 0.0;

        let assign57690_e89838: f64 = (locals.var_t9 + locals.var_t8);
        let assign57690_e89840: f64 = if assign57690_e89838 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1430 = assign57690_e89840;
        locals.var_guard1430_rv = 0.0;

        let (assign57700_e89857, assign57700_e89857_d_n0, assign57700_e89857_d_n2, assign57700_e89857_d_n4, assign57700_e89857_d_n5, assign57700_e89857_d_n6, assign57700_e89857_d_n7, assign57700_e89857_d_n8, assign57700_e89857_d_n9, assign57700_e89857_d_n10, assign57700_e89857_d_n11, assign57700_e89857_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1430 != 0.0)) {
        let assign57700_e89854: f64 = (locals.var_t9 + locals.var_t8);
        let assign57700_e89855: f64 = (locals.var_pds * assign57700_e89854);
        (assign57700_e89855, ((locals.var_pds_dn0 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn11 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn11 + locals.var_t8_dn11))), ((locals.var_pds_dn14 * assign57700_e89854) + (locals.var_pds * (locals.var_t9_dn14 + locals.var_t8_dn14))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn14,)
    }
};
        locals.var_idd1 = assign57700_e89857;
        locals.var_idd1_dn0 = assign57700_e89857_d_n0;
        locals.var_idd1_dn2 = assign57700_e89857_d_n2;
        locals.var_idd1_dn4 = assign57700_e89857_d_n4;
        locals.var_idd1_dn5 = assign57700_e89857_d_n5;
        locals.var_idd1_dn6 = assign57700_e89857_d_n6;
        locals.var_idd1_dn7 = assign57700_e89857_d_n7;
        locals.var_idd1_dn8 = assign57700_e89857_d_n8;
        locals.var_idd1_dn9 = assign57700_e89857_d_n9;
        locals.var_idd1_dn10 = assign57700_e89857_d_n10;
        locals.var_idd1_dn11 = assign57700_e89857_d_n11;
        locals.var_idd1_dn14 = assign57700_e89857_d_n14;
        locals.var_idd1_rv = 0.0;

        let (assign57710_e89876, assign57710_e89876_d_n0, assign57710_e89876_d_n2, assign57710_e89876_d_n4, assign57710_e89876_d_n5, assign57710_e89876_d_n6, assign57710_e89876_d_n7, assign57710_e89876_d_n8, assign57710_e89876_d_n9, assign57710_e89876_d_n10, assign57710_e89876_d_n11, assign57710_e89876_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1430 != 0.0)) {
        let assign57710_e89871: f64 = (locals.var_betawl * locals.var_idd1);
        let assign57710_e89873: f64 = (assign57710_e89871 * locals.var_mu);
        let assign57710_e89874: f64 = (locals.var_ids0 + assign57710_e89873);
        (assign57710_e89874, (locals.var_ids0_dn0 + ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn0))), (locals.var_ids0_dn2 + ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn2))), (locals.var_ids0_dn4 + ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn4))), (locals.var_ids0_dn5 + ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn5))), (locals.var_ids0_dn6 + ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn6))), (locals.var_ids0_dn7 + ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn7))), (locals.var_ids0_dn8 + ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn8))), (locals.var_ids0_dn9 + ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn9))), (locals.var_ids0_dn10 + ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn10))), (locals.var_ids0_dn11 + ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn11))), (locals.var_ids0_dn14 + ((((locals.var_betawl_dn14 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn14)) * locals.var_mu) + (assign57710_e89871 * locals.var_mu_dn14))),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign57710_e89876;
        locals.var_ids0_dn0 = assign57710_e89876_d_n0;
        locals.var_ids0_dn2 = assign57710_e89876_d_n2;
        locals.var_ids0_dn4 = assign57710_e89876_d_n4;
        locals.var_ids0_dn5 = assign57710_e89876_d_n5;
        locals.var_ids0_dn6 = assign57710_e89876_d_n6;
        locals.var_ids0_dn7 = assign57710_e89876_d_n7;
        locals.var_ids0_dn8 = assign57710_e89876_d_n8;
        locals.var_ids0_dn9 = assign57710_e89876_d_n9;
        locals.var_ids0_dn10 = assign57710_e89876_d_n10;
        locals.var_ids0_dn11 = assign57710_e89876_d_n11;
        locals.var_ids0_dn14 = assign57710_e89876_d_n14;
        locals.var_ids0_rv = 0.0;

        let (assign57720_e89887, assign57720_e89887_d_n0, assign57720_e89887_d_n2, assign57720_e89887_d_n4, assign57720_e89887_d_n5, assign57720_e89887_d_n6, assign57720_e89887_d_n7, assign57720_e89887_d_n8, assign57720_e89887_d_n9, assign57720_e89887_d_n10, assign57720_e89887_d_n11, assign57720_e89887_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign57720_e89887;
        locals.var_ids_dn0 = assign57720_e89887_d_n0;
        locals.var_ids_dn2 = assign57720_e89887_d_n2;
        locals.var_ids_dn4 = assign57720_e89887_d_n4;
        locals.var_ids_dn5 = assign57720_e89887_d_n5;
        locals.var_ids_dn6 = assign57720_e89887_d_n6;
        locals.var_ids_dn7 = assign57720_e89887_d_n7;
        locals.var_ids_dn8 = assign57720_e89887_d_n8;
        locals.var_ids_dn9 = assign57720_e89887_d_n9;
        locals.var_ids_dn10 = assign57720_e89887_d_n10;
        locals.var_ids_dn11 = assign57720_e89887_d_n11;
        locals.var_ids_dn14 = assign57720_e89887_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign57730_e89907, assign57730_e89907_d_n0, assign57730_e89907_d_n2, assign57730_e89907_d_n4, assign57730_e89907_d_n5, assign57730_e89907_d_n6, assign57730_e89907_d_n7, assign57730_e89907_d_n8, assign57730_e89907_d_n9, assign57730_e89907_d_n10, assign57730_e89907_d_n11, assign57730_e89907_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57730_e89897: f64 = (-0.5);
        let assign57730_e89900: f64 = (locals.var_q_s0__blk1102 - locals.var_q_n0__blk1126);
        let assign57730_e89902: f64 = (assign57730_e89900 + locals.var_q_sl__blk1103);
        let assign57730_e89904: f64 = (assign57730_e89902 - locals.var_q_nl__blk1127);
        let assign57730_e89905: f64 = (assign57730_e89897 * assign57730_e89904);
        (assign57730_e89905, (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn0 - locals.var_q_n0__blk1126_dn0) + locals.var_q_sl__blk1103_dn0) - locals.var_q_nl__blk1127_dn0)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn2 - locals.var_q_n0__blk1126_dn2) + locals.var_q_sl__blk1103_dn2) - locals.var_q_nl__blk1127_dn2)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn4 - locals.var_q_n0__blk1126_dn4) + locals.var_q_sl__blk1103_dn4) - locals.var_q_nl__blk1127_dn4)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn5 - locals.var_q_n0__blk1126_dn5) + locals.var_q_sl__blk1103_dn5) - locals.var_q_nl__blk1127_dn5)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn6 - locals.var_q_n0__blk1126_dn6) + locals.var_q_sl__blk1103_dn6) - locals.var_q_nl__blk1127_dn6)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn7 - locals.var_q_n0__blk1126_dn7) + locals.var_q_sl__blk1103_dn7) - locals.var_q_nl__blk1127_dn7)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn8 - locals.var_q_n0__blk1126_dn8) + locals.var_q_sl__blk1103_dn8) - locals.var_q_nl__blk1127_dn8)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn9 - locals.var_q_n0__blk1126_dn9) + locals.var_q_sl__blk1103_dn9) - locals.var_q_nl__blk1127_dn9)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn10 - locals.var_q_n0__blk1126_dn10) + locals.var_q_sl__blk1103_dn10) - locals.var_q_nl__blk1127_dn10)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn11 - locals.var_q_n0__blk1126_dn11) + locals.var_q_sl__blk1103_dn11) - locals.var_q_nl__blk1127_dn11)), (assign57730_e89897 * (((locals.var_q_s0__blk1102_dn14 - locals.var_q_n0__blk1126_dn14) + locals.var_q_sl__blk1103_dn14) - locals.var_q_nl__blk1127_dn14)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign57730_e89907;
        locals.var_qbu_dn0 = assign57730_e89907_d_n0;
        locals.var_qbu_dn2 = assign57730_e89907_d_n2;
        locals.var_qbu_dn4 = assign57730_e89907_d_n4;
        locals.var_qbu_dn5 = assign57730_e89907_d_n5;
        locals.var_qbu_dn6 = assign57730_e89907_d_n6;
        locals.var_qbu_dn7 = assign57730_e89907_d_n7;
        locals.var_qbu_dn8 = assign57730_e89907_d_n8;
        locals.var_qbu_dn9 = assign57730_e89907_d_n9;
        locals.var_qbu_dn10 = assign57730_e89907_d_n10;
        locals.var_qbu_dn11 = assign57730_e89907_d_n11;
        locals.var_qbu_dn14 = assign57730_e89907_d_n14;
        locals.var_qbu_rv = 0.0;

        let (assign57740_e89923, assign57740_e89923_d_n0, assign57740_e89923_d_n2, assign57740_e89923_d_n4, assign57740_e89923_d_n5, assign57740_e89923_d_n6, assign57740_e89923_d_n7, assign57740_e89923_d_n8, assign57740_e89923_d_n9, assign57740_e89923_d_n10, assign57740_e89923_d_n11, assign57740_e89923_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57740_e89917: f64 = (-0.5);
        let assign57740_e89920: f64 = (locals.var_q_n0__blk1126 + locals.var_q_nl__blk1127);
        let assign57740_e89921: f64 = (assign57740_e89917 * assign57740_e89920);
        (assign57740_e89921, (assign57740_e89917 * (locals.var_q_n0__blk1126_dn0 + locals.var_q_nl__blk1127_dn0)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn2 + locals.var_q_nl__blk1127_dn2)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn4 + locals.var_q_nl__blk1127_dn4)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn5 + locals.var_q_nl__blk1127_dn5)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn6 + locals.var_q_nl__blk1127_dn6)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn7 + locals.var_q_nl__blk1127_dn7)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn8 + locals.var_q_nl__blk1127_dn8)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn9 + locals.var_q_nl__blk1127_dn9)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn10 + locals.var_q_nl__blk1127_dn10)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn11 + locals.var_q_nl__blk1127_dn11)), (assign57740_e89917 * (locals.var_q_n0__blk1126_dn14 + locals.var_q_nl__blk1127_dn14)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign57740_e89923;
        locals.var_qiu_dn0 = assign57740_e89923_d_n0;
        locals.var_qiu_dn2 = assign57740_e89923_d_n2;
        locals.var_qiu_dn4 = assign57740_e89923_d_n4;
        locals.var_qiu_dn5 = assign57740_e89923_d_n5;
        locals.var_qiu_dn6 = assign57740_e89923_d_n6;
        locals.var_qiu_dn7 = assign57740_e89923_d_n7;
        locals.var_qiu_dn8 = assign57740_e89923_d_n8;
        locals.var_qiu_dn9 = assign57740_e89923_d_n9;
        locals.var_qiu_dn10 = assign57740_e89923_d_n10;
        locals.var_qiu_dn11 = assign57740_e89923_d_n11;
        locals.var_qiu_dn14 = assign57740_e89923_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign57750_e89934, assign57750_e89934_d_n0, assign57750_e89934_d_n2, assign57750_e89934_d_n4, assign57750_e89934_d_n5, assign57750_e89934_d_n6, assign57750_e89934_d_n7, assign57750_e89934_d_n8, assign57750_e89934_d_n9, assign57750_e89934_d_n10, assign57750_e89934_d_n11, assign57750_e89934_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign57750_e89934;
        locals.var_qdrat_dn0 = assign57750_e89934_d_n0;
        locals.var_qdrat_dn2 = assign57750_e89934_d_n2;
        locals.var_qdrat_dn4 = assign57750_e89934_d_n4;
        locals.var_qdrat_dn5 = assign57750_e89934_d_n5;
        locals.var_qdrat_dn6 = assign57750_e89934_d_n6;
        locals.var_qdrat_dn7 = assign57750_e89934_d_n7;
        locals.var_qdrat_dn8 = assign57750_e89934_d_n8;
        locals.var_qdrat_dn9 = assign57750_e89934_d_n9;
        locals.var_qdrat_dn10 = assign57750_e89934_d_n10;
        locals.var_qdrat_dn11 = assign57750_e89934_d_n11;
        locals.var_qdrat_dn14 = assign57750_e89934_d_n14;
        locals.var_qdrat_rv = 0.0;

        let (assign57760_e89950, assign57760_e89950_d_n0, assign57760_e89950_d_n2, assign57760_e89950_d_n4, assign57760_e89950_d_n5, assign57760_e89950_d_n6, assign57760_e89950_d_n7, assign57760_e89950_d_n8, assign57760_e89950_d_n9, assign57760_e89950_d_n10, assign57760_e89950_d_n11, assign57760_e89950_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57760_e89944: f64 = (-0.5);
        let assign57760_e89947: f64 = (locals.var_q_n0__blk1126 + locals.var_q_nl__blk1127);
        let assign57760_e89948: f64 = (assign57760_e89944 * assign57760_e89947);
        (assign57760_e89948, (assign57760_e89944 * (locals.var_q_n0__blk1126_dn0 + locals.var_q_nl__blk1127_dn0)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn2 + locals.var_q_nl__blk1127_dn2)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn4 + locals.var_q_nl__blk1127_dn4)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn5 + locals.var_q_nl__blk1127_dn5)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn6 + locals.var_q_nl__blk1127_dn6)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn7 + locals.var_q_nl__blk1127_dn7)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn8 + locals.var_q_nl__blk1127_dn8)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn9 + locals.var_q_nl__blk1127_dn9)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn10 + locals.var_q_nl__blk1127_dn10)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn11 + locals.var_q_nl__blk1127_dn11)), (assign57760_e89944 * (locals.var_q_n0__blk1126_dn14 + locals.var_q_nl__blk1127_dn14)),)
    } else {
        (locals.var_qiu_noi, locals.var_qiu_noi_dn0, locals.var_qiu_noi_dn2, locals.var_qiu_noi_dn4, locals.var_qiu_noi_dn5, locals.var_qiu_noi_dn6, locals.var_qiu_noi_dn7, locals.var_qiu_noi_dn8, locals.var_qiu_noi_dn9, locals.var_qiu_noi_dn10, locals.var_qiu_noi_dn11, locals.var_qiu_noi_dn14,)
    }
};
        locals.var_qiu_noi = assign57760_e89950;
        locals.var_qiu_noi_dn0 = assign57760_e89950_d_n0;
        locals.var_qiu_noi_dn2 = assign57760_e89950_d_n2;
        locals.var_qiu_noi_dn4 = assign57760_e89950_d_n4;
        locals.var_qiu_noi_dn5 = assign57760_e89950_d_n5;
        locals.var_qiu_noi_dn6 = assign57760_e89950_d_n6;
        locals.var_qiu_noi_dn7 = assign57760_e89950_d_n7;
        locals.var_qiu_noi_dn8 = assign57760_e89950_d_n8;
        locals.var_qiu_noi_dn9 = assign57760_e89950_d_n9;
        locals.var_qiu_noi_dn10 = assign57760_e89950_d_n10;
        locals.var_qiu_noi_dn11 = assign57760_e89950_d_n11;
        locals.var_qiu_noi_dn14 = assign57760_e89950_d_n14;
        locals.var_qiu_noi_rv = 0.0;

        let (assign57770_e89962, assign57770_e89962_d_n0, assign57770_e89962_d_n2, assign57770_e89962_d_n4, assign57770_e89962_d_n5, assign57770_e89962_d_n6, assign57770_e89962_d_n7, assign57770_e89962_d_n8, assign57770_e89962_d_n9, assign57770_e89962_d_n10, assign57770_e89962_d_n11, assign57770_e89962_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign57770_e89960: f64 = (-locals.var_q_n0__blk1126);
        (assign57770_e89960, (-locals.var_q_n0__blk1126_dn0), (-locals.var_q_n0__blk1126_dn2), (-locals.var_q_n0__blk1126_dn4), (-locals.var_q_n0__blk1126_dn5), (-locals.var_q_n0__blk1126_dn6), (-locals.var_q_n0__blk1126_dn7), (-locals.var_q_n0__blk1126_dn8), (-locals.var_q_n0__blk1126_dn9), (-locals.var_q_n0__blk1126_dn10), (-locals.var_q_n0__blk1126_dn11), (-locals.var_q_n0__blk1126_dn14),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn14,)
    }
};
        locals.var_qn0 = assign57770_e89962;
        locals.var_qn0_dn0 = assign57770_e89962_d_n0;
        locals.var_qn0_dn2 = assign57770_e89962_d_n2;
        locals.var_qn0_dn4 = assign57770_e89962_d_n4;
        locals.var_qn0_dn5 = assign57770_e89962_d_n5;
        locals.var_qn0_dn6 = assign57770_e89962_d_n6;
        locals.var_qn0_dn7 = assign57770_e89962_d_n7;
        locals.var_qn0_dn8 = assign57770_e89962_d_n8;
        locals.var_qn0_dn9 = assign57770_e89962_d_n9;
        locals.var_qn0_dn10 = assign57770_e89962_d_n10;
        locals.var_qn0_dn11 = assign57770_e89962_d_n11;
        locals.var_qn0_dn14 = assign57770_e89962_d_n14;
        locals.var_qn0_rv = 0.0;

        let (assign57780_e89973, assign57780_e89973_d_n0, assign57780_e89973_d_n2, assign57780_e89973_d_n4, assign57780_e89973_d_n5, assign57780_e89973_d_n6, assign57780_e89973_d_n7, assign57780_e89973_d_n8, assign57780_e89973_d_n9, assign57780_e89973_d_n10, assign57780_e89973_d_n11, assign57780_e89973_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (locals.var_ey_acc__blk1120, locals.var_ey_acc__blk1120_dn0, locals.var_ey_acc__blk1120_dn2, locals.var_ey_acc__blk1120_dn4, locals.var_ey_acc__blk1120_dn5, locals.var_ey_acc__blk1120_dn6, locals.var_ey_acc__blk1120_dn7, locals.var_ey_acc__blk1120_dn8, locals.var_ey_acc__blk1120_dn9, locals.var_ey_acc__blk1120_dn10, locals.var_ey_acc__blk1120_dn11, locals.var_ey_acc__blk1120_dn14,)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign57780_e89973;
        locals.var_ey_dn0 = assign57780_e89973_d_n0;
        locals.var_ey_dn2 = assign57780_e89973_d_n2;
        locals.var_ey_dn4 = assign57780_e89973_d_n4;
        locals.var_ey_dn5 = assign57780_e89973_d_n5;
        locals.var_ey_dn6 = assign57780_e89973_d_n6;
        locals.var_ey_dn7 = assign57780_e89973_d_n7;
        locals.var_ey_dn8 = assign57780_e89973_d_n8;
        locals.var_ey_dn9 = assign57780_e89973_d_n9;
        locals.var_ey_dn10 = assign57780_e89973_d_n10;
        locals.var_ey_dn11 = assign57780_e89973_d_n11;
        locals.var_ey_dn14 = assign57780_e89973_d_n14;
        locals.var_ey_rv = 0.0;

        let assign57790_e89980: f64 = if ((locals.var_qn0 < 1e-25) || (locals.var_qiu < 1e-25)) { 1.0 } else { 0.0 };
        locals.var_guard1431 = assign57790_e89980;
        locals.var_guard1431_rv = 0.0;

        let (assign57800_e89993,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1431 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign57800_e89993;
        locals.var_flg_noqi_rv = 0.0;

        let assign57810_e89996: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard1432 = assign57810_e89996;
        locals.var_guard1432_rv = 0.0;

        let (assign57820_e90004,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57820_e90002: f64 = (-1.0);
        (assign57820_e90002,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign57820_e90004;
        locals.var_flg_zone_rv = 0.0;

        let (assign57830_e90019, assign57830_e90019_d_n0, assign57830_e90019_d_n2, assign57830_e90019_d_n4, assign57830_e90019_d_n5, assign57830_e90019_d_n6, assign57830_e90019_d_n7, assign57830_e90019_d_n8, assign57830_e90019_d_n9, assign57830_e90019_d_n10, assign57830_e90019_d_n11, assign57830_e90019_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57830_e90011: f64 = (2.0 * locals.var_beta_inv);
        let assign57830_e90013: f64 = (-locals.var_vgs_min);
        let assign57830_e90015: f64 = (assign57830_e90013 / locals.var_fac1);
        let assign57830_e90016: f64 = (assign57830_e90015).ln();
        let assign57830_e90017: f64 = (assign57830_e90011 * assign57830_e90016);
        (assign57830_e90017, (((2.0 * locals.var_beta_inv_dn0) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn2) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn4) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn5) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn6) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn7) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn8) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn9) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn10) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn11) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))), (((2.0 * locals.var_beta_inv_dn14) * assign57830_e90016) + (assign57830_e90011 * ((-((assign57830_e90013 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign57830_e90015))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign57830_e90019;
        locals.var_ps0_min_dn0 = assign57830_e90019_d_n0;
        locals.var_ps0_min_dn2 = assign57830_e90019_d_n2;
        locals.var_ps0_min_dn4 = assign57830_e90019_d_n4;
        locals.var_ps0_min_dn5 = assign57830_e90019_d_n5;
        locals.var_ps0_min_dn6 = assign57830_e90019_d_n6;
        locals.var_ps0_min_dn7 = assign57830_e90019_d_n7;
        locals.var_ps0_min_dn8 = assign57830_e90019_d_n8;
        locals.var_ps0_min_dn9 = assign57830_e90019_d_n9;
        locals.var_ps0_min_dn10 = assign57830_e90019_d_n10;
        locals.var_ps0_min_dn11 = assign57830_e90019_d_n11;
        locals.var_ps0_min_dn14 = assign57830_e90019_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign57840_e90030, assign57840_e90030_d_n0, assign57840_e90030_d_n2, assign57840_e90030_d_n4, assign57840_e90030_d_n5, assign57840_e90030_d_n6, assign57840_e90030_d_n7, assign57840_e90030_d_n8, assign57840_e90030_d_n9, assign57840_e90030_d_n10, assign57840_e90030_d_n11, assign57840_e90030_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57840_e90027: f64 = (locals.var_vgp - locals.var_vbscl__blk439);
        let assign57840_e90028: f64 = (locals.var_beta * assign57840_e90027);
        (assign57840_e90028, ((locals.var_beta_dn0 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk439_dn0))), ((locals.var_beta_dn2 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk439_dn2))), ((locals.var_beta_dn4 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk439_dn4))), ((locals.var_beta_dn5 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk439_dn5))), ((locals.var_beta_dn6 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk439_dn6))), ((locals.var_beta_dn7 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk439_dn7))), ((locals.var_beta_dn8 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk439_dn8))), ((locals.var_beta_dn9 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk439_dn9))), ((locals.var_beta_dn10 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk439_dn10))), ((locals.var_beta_dn11 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbscl__blk439_dn11))), ((locals.var_beta_dn14 * assign57840_e90027) + (locals.var_beta * (locals.var_vgp_dn14 - locals.var_vbscl__blk439_dn14))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign57840_e90030;
        locals.var_tx_dn0 = assign57840_e90030_d_n0;
        locals.var_tx_dn2 = assign57840_e90030_d_n2;
        locals.var_tx_dn4 = assign57840_e90030_d_n4;
        locals.var_tx_dn5 = assign57840_e90030_d_n5;
        locals.var_tx_dn6 = assign57840_e90030_d_n6;
        locals.var_tx_dn7 = assign57840_e90030_d_n7;
        locals.var_tx_dn8 = assign57840_e90030_d_n8;
        locals.var_tx_dn9 = assign57840_e90030_d_n9;
        locals.var_tx_dn10 = assign57840_e90030_d_n10;
        locals.var_tx_dn11 = assign57840_e90030_d_n11;
        locals.var_tx_dn14 = assign57840_e90030_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign57850_e90041, assign57850_e90041_d_n0, assign57850_e90041_d_n2, assign57850_e90041_d_n4, assign57850_e90041_d_n5, assign57850_e90041_d_n6, assign57850_e90041_d_n7, assign57850_e90041_d_n8, assign57850_e90041_d_n9, assign57850_e90041_d_n10, assign57850_e90041_d_n11, assign57850_e90041_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57850_e90038: f64 = (locals.var_beta * locals.var_cnst0);
        let assign57850_e90039: f64 = (1.0 / assign57850_e90038);
        (assign57850_e90039, (-(((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn11 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn11)) / (assign57850_e90038 * assign57850_e90038))), (-(((locals.var_beta_dn14 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn14)) / (assign57850_e90038 * assign57850_e90038))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57850_e90041;
        locals.var_t1_dn0 = assign57850_e90041_d_n0;
        locals.var_t1_dn2 = assign57850_e90041_d_n2;
        locals.var_t1_dn4 = assign57850_e90041_d_n4;
        locals.var_t1_dn5 = assign57850_e90041_d_n5;
        locals.var_t1_dn6 = assign57850_e90041_d_n6;
        locals.var_t1_dn7 = assign57850_e90041_d_n7;
        locals.var_t1_dn8 = assign57850_e90041_d_n8;
        locals.var_t1_dn9 = assign57850_e90041_d_n9;
        locals.var_t1_dn10 = assign57850_e90041_d_n10;
        locals.var_t1_dn11 = assign57850_e90041_d_n11;
        locals.var_t1_dn14 = assign57850_e90041_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57860_e90050, assign57860_e90050_d_n0, assign57860_e90050_d_n2, assign57860_e90050_d_n4, assign57860_e90050_d_n5, assign57860_e90050_d_n6, assign57860_e90050_d_n7, assign57860_e90050_d_n8, assign57860_e90050_d_n9, assign57860_e90050_d_n10, assign57860_e90050_d_n11, assign57860_e90050_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57860_e90048: f64 = (locals.var_t1 * locals.var_cox);
        (assign57860_e90048, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn11 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn11)), ((locals.var_t1_dn14 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign57860_e90050;
        locals.var_ty_dn0 = assign57860_e90050_d_n0;
        locals.var_ty_dn2 = assign57860_e90050_d_n2;
        locals.var_ty_dn4 = assign57860_e90050_d_n4;
        locals.var_ty_dn5 = assign57860_e90050_d_n5;
        locals.var_ty_dn6 = assign57860_e90050_d_n6;
        locals.var_ty_dn7 = assign57860_e90050_d_n7;
        locals.var_ty_dn8 = assign57860_e90050_d_n8;
        locals.var_ty_dn9 = assign57860_e90050_d_n9;
        locals.var_ty_dn10 = assign57860_e90050_d_n10;
        locals.var_ty_dn11 = assign57860_e90050_d_n11;
        locals.var_ty_dn14 = assign57860_e90050_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign57870_e90063, assign57870_e90063_d_n0, assign57870_e90063_d_n2, assign57870_e90063_d_n4, assign57870_e90063_d_n5, assign57870_e90063_d_n6, assign57870_e90063_d_n7, assign57870_e90063_d_n8, assign57870_e90063_d_n9, assign57870_e90063_d_n10, assign57870_e90063_d_n11, assign57870_e90063_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57870_e90058: f64 = (3.0 * 1.414213562373095);
        let assign57870_e90060: f64 = (assign57870_e90058 * locals.var_ty);
        let assign57870_e90061: f64 = (2.0 + assign57870_e90060);
        (assign57870_e90061, (assign57870_e90058 * locals.var_ty_dn0), (assign57870_e90058 * locals.var_ty_dn2), (assign57870_e90058 * locals.var_ty_dn4), (assign57870_e90058 * locals.var_ty_dn5), (assign57870_e90058 * locals.var_ty_dn6), (assign57870_e90058 * locals.var_ty_dn7), (assign57870_e90058 * locals.var_ty_dn8), (assign57870_e90058 * locals.var_ty_dn9), (assign57870_e90058 * locals.var_ty_dn10), (assign57870_e90058 * locals.var_ty_dn11), (assign57870_e90058 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign57870_e90063;
        locals.var_ac41_dn0 = assign57870_e90063_d_n0;
        locals.var_ac41_dn2 = assign57870_e90063_d_n2;
        locals.var_ac41_dn4 = assign57870_e90063_d_n4;
        locals.var_ac41_dn5 = assign57870_e90063_d_n5;
        locals.var_ac41_dn6 = assign57870_e90063_d_n6;
        locals.var_ac41_dn7 = assign57870_e90063_d_n7;
        locals.var_ac41_dn8 = assign57870_e90063_d_n8;
        locals.var_ac41_dn9 = assign57870_e90063_d_n9;
        locals.var_ac41_dn10 = assign57870_e90063_d_n10;
        locals.var_ac41_dn11 = assign57870_e90063_d_n11;
        locals.var_ac41_dn14 = assign57870_e90063_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign57880_e90076, assign57880_e90076_d_n0, assign57880_e90076_d_n2, assign57880_e90076_d_n4, assign57880_e90076_d_n5, assign57880_e90076_d_n6, assign57880_e90076_d_n7, assign57880_e90076_d_n8, assign57880_e90076_d_n9, assign57880_e90076_d_n10, assign57880_e90076_d_n11, assign57880_e90076_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57880_e90070: f64 = (8.0 * locals.var_ac41);
        let assign57880_e90072: f64 = (assign57880_e90070 * locals.var_ac41);
        let assign57880_e90074: f64 = (assign57880_e90072 * locals.var_ac41);
        (assign57880_e90074, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign57880_e90070 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign57880_e90072 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign57880_e90076;
        locals.var_ac4_dn0 = assign57880_e90076_d_n0;
        locals.var_ac4_dn2 = assign57880_e90076_d_n2;
        locals.var_ac4_dn4 = assign57880_e90076_d_n4;
        locals.var_ac4_dn5 = assign57880_e90076_d_n5;
        locals.var_ac4_dn6 = assign57880_e90076_d_n6;
        locals.var_ac4_dn7 = assign57880_e90076_d_n7;
        locals.var_ac4_dn8 = assign57880_e90076_d_n8;
        locals.var_ac4_dn9 = assign57880_e90076_d_n9;
        locals.var_ac4_dn10 = assign57880_e90076_d_n10;
        locals.var_ac4_dn11 = assign57880_e90076_d_n11;
        locals.var_ac4_dn14 = assign57880_e90076_d_n14;
        locals.var_ac4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_212(
        locals: &mut StampLocals,
    ) {
        let (assign57890_e90085, assign57890_e90085_d_n0, assign57890_e90085_d_n2, assign57890_e90085_d_n4, assign57890_e90085_d_n5, assign57890_e90085_d_n6, assign57890_e90085_d_n7, assign57890_e90085_d_n8, assign57890_e90085_d_n9, assign57890_e90085_d_n10, assign57890_e90085_d_n11, assign57890_e90085_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57890_e90083: f64 = (locals.var_tx - 2.0);
        (assign57890_e90083, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57890_e90085;
        locals.var_t4_dn0 = assign57890_e90085_d_n0;
        locals.var_t4_dn2 = assign57890_e90085_d_n2;
        locals.var_t4_dn4 = assign57890_e90085_d_n4;
        locals.var_t4_dn5 = assign57890_e90085_d_n5;
        locals.var_t4_dn6 = assign57890_e90085_d_n6;
        locals.var_t4_dn7 = assign57890_e90085_d_n7;
        locals.var_t4_dn8 = assign57890_e90085_d_n8;
        locals.var_t4_dn9 = assign57890_e90085_d_n9;
        locals.var_t4_dn10 = assign57890_e90085_d_n10;
        locals.var_t4_dn11 = assign57890_e90085_d_n11;
        locals.var_t4_dn14 = assign57890_e90085_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57900_e90096, assign57900_e90096_d_n0, assign57900_e90096_d_n2, assign57900_e90096_d_n4, assign57900_e90096_d_n5, assign57900_e90096_d_n6, assign57900_e90096_d_n7, assign57900_e90096_d_n8, assign57900_e90096_d_n9, assign57900_e90096_d_n10, assign57900_e90096_d_n11, assign57900_e90096_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57900_e90092: f64 = (9.0 * locals.var_ty);
        let assign57900_e90094: f64 = (assign57900_e90092 * locals.var_t4);
        (assign57900_e90094, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn4) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn4)), (((9.0 * locals.var_ty_dn5) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn5)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn8) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn8)), (((9.0 * locals.var_ty_dn9) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn9)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn14) * locals.var_t4) + (assign57900_e90092 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign57900_e90096;
        locals.var_t5_dn0 = assign57900_e90096_d_n0;
        locals.var_t5_dn2 = assign57900_e90096_d_n2;
        locals.var_t5_dn4 = assign57900_e90096_d_n4;
        locals.var_t5_dn5 = assign57900_e90096_d_n5;
        locals.var_t5_dn6 = assign57900_e90096_d_n6;
        locals.var_t5_dn7 = assign57900_e90096_d_n7;
        locals.var_t5_dn8 = assign57900_e90096_d_n8;
        locals.var_t5_dn9 = assign57900_e90096_d_n9;
        locals.var_t5_dn10 = assign57900_e90096_d_n10;
        locals.var_t5_dn11 = assign57900_e90096_d_n11;
        locals.var_t5_dn14 = assign57900_e90096_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign57910_e90107, assign57910_e90107_d_n0, assign57910_e90107_d_n2, assign57910_e90107_d_n4, assign57910_e90107_d_n5, assign57910_e90107_d_n6, assign57910_e90107_d_n7, assign57910_e90107_d_n8, assign57910_e90107_d_n9, assign57910_e90107_d_n10, assign57910_e90107_d_n11, assign57910_e90107_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57910_e90103: f64 = (7.0 * 1.414213562373095);
        let assign57910_e90105: f64 = (assign57910_e90103 - locals.var_t5);
        (assign57910_e90105, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn14),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign57910_e90107;
        locals.var_ac31_dn0 = assign57910_e90107_d_n0;
        locals.var_ac31_dn2 = assign57910_e90107_d_n2;
        locals.var_ac31_dn4 = assign57910_e90107_d_n4;
        locals.var_ac31_dn5 = assign57910_e90107_d_n5;
        locals.var_ac31_dn6 = assign57910_e90107_d_n6;
        locals.var_ac31_dn7 = assign57910_e90107_d_n7;
        locals.var_ac31_dn8 = assign57910_e90107_d_n8;
        locals.var_ac31_dn9 = assign57910_e90107_d_n9;
        locals.var_ac31_dn10 = assign57910_e90107_d_n10;
        locals.var_ac31_dn11 = assign57910_e90107_d_n11;
        locals.var_ac31_dn14 = assign57910_e90107_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign57920_e90116, assign57920_e90116_d_n0, assign57920_e90116_d_n2, assign57920_e90116_d_n4, assign57920_e90116_d_n5, assign57920_e90116_d_n6, assign57920_e90116_d_n7, assign57920_e90116_d_n8, assign57920_e90116_d_n9, assign57920_e90116_d_n10, assign57920_e90116_d_n11, assign57920_e90116_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57920_e90114: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign57920_e90114, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign57920_e90116;
        locals.var_ac3_dn0 = assign57920_e90116_d_n0;
        locals.var_ac3_dn2 = assign57920_e90116_d_n2;
        locals.var_ac3_dn4 = assign57920_e90116_d_n4;
        locals.var_ac3_dn5 = assign57920_e90116_d_n5;
        locals.var_ac3_dn6 = assign57920_e90116_d_n6;
        locals.var_ac3_dn7 = assign57920_e90116_d_n7;
        locals.var_ac3_dn8 = assign57920_e90116_d_n8;
        locals.var_ac3_dn9 = assign57920_e90116_d_n9;
        locals.var_ac3_dn10 = assign57920_e90116_d_n10;
        locals.var_ac3_dn11 = assign57920_e90116_d_n11;
        locals.var_ac3_dn14 = assign57920_e90116_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign57930_e90120: f64 = (locals.var_ac3 * 1e-8);
        let assign57930_e90121: f64 = if locals.var_ac4 < assign57930_e90120 { 1.0 } else { 0.0 };
        locals.var_guard1433 = assign57930_e90121;
        locals.var_guard1433_rv = 0.0;

        let (assign57940_e90134, assign57940_e90134_d_n0, assign57940_e90134_d_n2, assign57940_e90134_d_n4, assign57940_e90134_d_n5, assign57940_e90134_d_n6, assign57940_e90134_d_n7, assign57940_e90134_d_n8, assign57940_e90134_d_n9, assign57940_e90134_d_n10, assign57940_e90134_d_n11, assign57940_e90134_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) {
        let assign57940_e90130: f64 = (0.5 * locals.var_ac4);
        let assign57940_e90132: f64 = (assign57940_e90130 / locals.var_ac31);
        (assign57940_e90132, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign57940_e90130 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign57940_e90134;
        locals.var_ac1_dn0 = assign57940_e90134_d_n0;
        locals.var_ac1_dn2 = assign57940_e90134_d_n2;
        locals.var_ac1_dn4 = assign57940_e90134_d_n4;
        locals.var_ac1_dn5 = assign57940_e90134_d_n5;
        locals.var_ac1_dn6 = assign57940_e90134_d_n6;
        locals.var_ac1_dn7 = assign57940_e90134_d_n7;
        locals.var_ac1_dn8 = assign57940_e90134_d_n8;
        locals.var_ac1_dn9 = assign57940_e90134_d_n9;
        locals.var_ac1_dn10 = assign57940_e90134_d_n10;
        locals.var_ac1_dn11 = assign57940_e90134_d_n11;
        locals.var_ac1_dn14 = assign57940_e90134_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign57950_e90147, assign57950_e90147_d_n0, assign57950_e90147_d_n2, assign57950_e90147_d_n4, assign57950_e90147_d_n5, assign57950_e90147_d_n6, assign57950_e90147_d_n7, assign57950_e90147_d_n8, assign57950_e90147_d_n9, assign57950_e90147_d_n10, assign57950_e90147_d_n11, assign57950_e90147_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign57950_e90144: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign57950_e90145: f64 = (assign57950_e90144).sqrt();
        (assign57950_e90145, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign57950_e90145)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign57950_e90145)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign57950_e90147;
        locals.var_ac2_dn0 = assign57950_e90147_d_n0;
        locals.var_ac2_dn2 = assign57950_e90147_d_n2;
        locals.var_ac2_dn4 = assign57950_e90147_d_n4;
        locals.var_ac2_dn5 = assign57950_e90147_d_n5;
        locals.var_ac2_dn6 = assign57950_e90147_d_n6;
        locals.var_ac2_dn7 = assign57950_e90147_d_n7;
        locals.var_ac2_dn8 = assign57950_e90147_d_n8;
        locals.var_ac2_dn9 = assign57950_e90147_d_n9;
        locals.var_ac2_dn10 = assign57950_e90147_d_n10;
        locals.var_ac2_dn11 = assign57950_e90147_d_n11;
        locals.var_ac2_dn14 = assign57950_e90147_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign57960_e90160, assign57960_e90160_d_n0, assign57960_e90160_d_n2, assign57960_e90160_d_n4, assign57960_e90160_d_n5, assign57960_e90160_d_n6, assign57960_e90160_d_n7, assign57960_e90160_d_n8, assign57960_e90160_d_n9, assign57960_e90160_d_n10, assign57960_e90160_d_n11, assign57960_e90160_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign57960_e90156: f64 = (-locals.var_ac31);
        let assign57960_e90158: f64 = (assign57960_e90156 + locals.var_ac2);
        (assign57960_e90158, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign57960_e90160;
        locals.var_ac1_dn0 = assign57960_e90160_d_n0;
        locals.var_ac1_dn2 = assign57960_e90160_d_n2;
        locals.var_ac1_dn4 = assign57960_e90160_d_n4;
        locals.var_ac1_dn5 = assign57960_e90160_d_n5;
        locals.var_ac1_dn6 = assign57960_e90160_d_n6;
        locals.var_ac1_dn7 = assign57960_e90160_d_n7;
        locals.var_ac1_dn8 = assign57960_e90160_d_n8;
        locals.var_ac1_dn9 = assign57960_e90160_d_n9;
        locals.var_ac1_dn10 = assign57960_e90160_d_n10;
        locals.var_ac1_dn11 = assign57960_e90160_d_n11;
        locals.var_ac1_dn14 = assign57960_e90160_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign57970_e90174, assign57970_e90174_d_n0, assign57970_e90174_d_n2, assign57970_e90174_d_n4, assign57970_e90174_d_n5, assign57970_e90174_d_n6, assign57970_e90174_d_n7, assign57970_e90174_d_n8, assign57970_e90174_d_n9, assign57970_e90174_d_n10, assign57970_e90174_d_n11, assign57970_e90174_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let (assign57970_e90172, assign57970_e90172_d_n0, assign57970_e90172_d_n2, assign57970_e90172_d_n4, assign57970_e90172_d_n5, assign57970_e90172_d_n6, assign57970_e90172_d_n7, assign57970_e90172_d_n8, assign57970_e90172_d_n9, assign57970_e90172_d_n10, assign57970_e90172_d_n11, assign57970_e90172_d_n14,) = {
            if (locals.var_ac1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57970_e90171: f64 = (locals.var_ac1).powf(0.3333333333333333);
                (assign57970_e90171, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign57970_e90171 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
            }
        };
        (assign57970_e90172, assign57970_e90172_d_n0, assign57970_e90172_d_n2, assign57970_e90172_d_n4, assign57970_e90172_d_n5, assign57970_e90172_d_n6, assign57970_e90172_d_n7, assign57970_e90172_d_n8, assign57970_e90172_d_n9, assign57970_e90172_d_n10, assign57970_e90172_d_n11, assign57970_e90172_d_n14,)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign57970_e90174;
        locals.var_acd_dn0 = assign57970_e90174_d_n0;
        locals.var_acd_dn2 = assign57970_e90174_d_n2;
        locals.var_acd_dn4 = assign57970_e90174_d_n4;
        locals.var_acd_dn5 = assign57970_e90174_d_n5;
        locals.var_acd_dn6 = assign57970_e90174_d_n6;
        locals.var_acd_dn7 = assign57970_e90174_d_n7;
        locals.var_acd_dn8 = assign57970_e90174_d_n8;
        locals.var_acd_dn9 = assign57970_e90174_d_n9;
        locals.var_acd_dn10 = assign57970_e90174_d_n10;
        locals.var_acd_dn11 = assign57970_e90174_d_n11;
        locals.var_acd_dn14 = assign57970_e90174_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign57980_e90198, assign57980_e90198_d_n0, assign57980_e90198_d_n2, assign57980_e90198_d_n4, assign57980_e90198_d_n5, assign57980_e90198_d_n6, assign57980_e90198_d_n7, assign57980_e90198_d_n8, assign57980_e90198_d_n9, assign57980_e90198_d_n10, assign57980_e90198_d_n11, assign57980_e90198_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57980_e90180: f64 = (-4.0);
        let assign57980_e90182: f64 = (assign57980_e90180 * 1.414213562373095);
        let assign57980_e90185: f64 = (12.0 * locals.var_ty);
        let assign57980_e90186: f64 = (assign57980_e90182 - assign57980_e90185);
        let assign57980_e90189: f64 = (2.0 * locals.var_acd);
        let assign57980_e90190: f64 = (assign57980_e90186 + assign57980_e90189);
        let assign57980_e90193: f64 = (1.414213562373095 * locals.var_acd);
        let assign57980_e90195: f64 = (assign57980_e90193 * locals.var_acd);
        let assign57980_e90196: f64 = (assign57980_e90190 + assign57980_e90195);
        (assign57980_e90196, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign57980_e90193 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign57980_e90198;
        locals.var_acn_dn0 = assign57980_e90198_d_n0;
        locals.var_acn_dn2 = assign57980_e90198_d_n2;
        locals.var_acn_dn4 = assign57980_e90198_d_n4;
        locals.var_acn_dn5 = assign57980_e90198_d_n5;
        locals.var_acn_dn6 = assign57980_e90198_d_n6;
        locals.var_acn_dn7 = assign57980_e90198_d_n7;
        locals.var_acn_dn8 = assign57980_e90198_d_n8;
        locals.var_acn_dn9 = assign57980_e90198_d_n9;
        locals.var_acn_dn10 = assign57980_e90198_d_n10;
        locals.var_acn_dn11 = assign57980_e90198_d_n11;
        locals.var_acn_dn14 = assign57980_e90198_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign57990_e90207, assign57990_e90207_d_n0, assign57990_e90207_d_n2, assign57990_e90207_d_n4, assign57990_e90207_d_n5, assign57990_e90207_d_n6, assign57990_e90207_d_n7, assign57990_e90207_d_n8, assign57990_e90207_d_n9, assign57990_e90207_d_n10, assign57990_e90207_d_n11, assign57990_e90207_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign57990_e90205: f64 = (1.0 / locals.var_acd);
        (assign57990_e90205, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn4 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn5 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn8 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn9 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn14 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57990_e90207;
        locals.var_t1_dn0 = assign57990_e90207_d_n0;
        locals.var_t1_dn2 = assign57990_e90207_d_n2;
        locals.var_t1_dn4 = assign57990_e90207_d_n4;
        locals.var_t1_dn5 = assign57990_e90207_d_n5;
        locals.var_t1_dn6 = assign57990_e90207_d_n6;
        locals.var_t1_dn7 = assign57990_e90207_d_n7;
        locals.var_t1_dn8 = assign57990_e90207_d_n8;
        locals.var_t1_dn9 = assign57990_e90207_d_n9;
        locals.var_t1_dn10 = assign57990_e90207_d_n10;
        locals.var_t1_dn11 = assign57990_e90207_d_n11;
        locals.var_t1_dn14 = assign57990_e90207_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58000_e90216, assign58000_e90216_d_n0, assign58000_e90216_d_n2, assign58000_e90216_d_n4, assign58000_e90216_d_n5, assign58000_e90216_d_n6, assign58000_e90216_d_n7, assign58000_e90216_d_n8, assign58000_e90216_d_n9, assign58000_e90216_d_n10, assign58000_e90216_d_n11, assign58000_e90216_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58000_e90214: f64 = (locals.var_acn * locals.var_t1);
        (assign58000_e90214, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn4 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn4)), ((locals.var_acn_dn5 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn5)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn8 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn8)), ((locals.var_acn_dn9 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn9)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn14 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign58000_e90216;
        locals.var_chi_dn0 = assign58000_e90216_d_n0;
        locals.var_chi_dn2 = assign58000_e90216_d_n2;
        locals.var_chi_dn4 = assign58000_e90216_d_n4;
        locals.var_chi_dn5 = assign58000_e90216_d_n5;
        locals.var_chi_dn6 = assign58000_e90216_d_n6;
        locals.var_chi_dn7 = assign58000_e90216_d_n7;
        locals.var_chi_dn8 = assign58000_e90216_d_n8;
        locals.var_chi_dn9 = assign58000_e90216_d_n9;
        locals.var_chi_dn10 = assign58000_e90216_d_n10;
        locals.var_chi_dn11 = assign58000_e90216_d_n11;
        locals.var_chi_dn14 = assign58000_e90216_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign58010_e90227, assign58010_e90227_d_n0, assign58010_e90227_d_n2, assign58010_e90227_d_n4, assign58010_e90227_d_n5, assign58010_e90227_d_n6, assign58010_e90227_d_n7, assign58010_e90227_d_n8, assign58010_e90227_d_n9, assign58010_e90227_d_n10, assign58010_e90227_d_n11, assign58010_e90227_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58010_e90223: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign58010_e90225: f64 = (assign58010_e90223 + locals.var_vbscl__blk439);
        (assign58010_e90225, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk439_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk439_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk439_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk439_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk439_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk439_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk439_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk439_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk439_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) + locals.var_vbscl__blk439_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) + locals.var_vbscl__blk439_dn14),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn8, locals.var_psa_dn9, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn14,)
    }
};
        locals.var_psa = assign58010_e90227;
        locals.var_psa_dn0 = assign58010_e90227_d_n0;
        locals.var_psa_dn2 = assign58010_e90227_d_n2;
        locals.var_psa_dn4 = assign58010_e90227_d_n4;
        locals.var_psa_dn5 = assign58010_e90227_d_n5;
        locals.var_psa_dn6 = assign58010_e90227_d_n6;
        locals.var_psa_dn7 = assign58010_e90227_d_n7;
        locals.var_psa_dn8 = assign58010_e90227_d_n8;
        locals.var_psa_dn9 = assign58010_e90227_d_n9;
        locals.var_psa_dn10 = assign58010_e90227_d_n10;
        locals.var_psa_dn11 = assign58010_e90227_d_n11;
        locals.var_psa_dn14 = assign58010_e90227_d_n14;
        locals.var_psa_rv = 0.0;

        let (assign58020_e90236, assign58020_e90236_d_n0, assign58020_e90236_d_n2, assign58020_e90236_d_n4, assign58020_e90236_d_n5, assign58020_e90236_d_n6, assign58020_e90236_d_n7, assign58020_e90236_d_n8, assign58020_e90236_d_n9, assign58020_e90236_d_n10, assign58020_e90236_d_n11, assign58020_e90236_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58020_e90234: f64 = (locals.var_psa - locals.var_vbscl__blk439);
        (assign58020_e90234, (locals.var_psa_dn0 - locals.var_vbscl__blk439_dn0), (locals.var_psa_dn2 - locals.var_vbscl__blk439_dn2), (locals.var_psa_dn4 - locals.var_vbscl__blk439_dn4), (locals.var_psa_dn5 - locals.var_vbscl__blk439_dn5), (locals.var_psa_dn6 - locals.var_vbscl__blk439_dn6), (locals.var_psa_dn7 - locals.var_vbscl__blk439_dn7), (locals.var_psa_dn8 - locals.var_vbscl__blk439_dn8), (locals.var_psa_dn9 - locals.var_vbscl__blk439_dn9), (locals.var_psa_dn10 - locals.var_vbscl__blk439_dn10), (locals.var_psa_dn11 - locals.var_vbscl__blk439_dn11), (locals.var_psa_dn14 - locals.var_vbscl__blk439_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58020_e90236;
        locals.var_t1_dn0 = assign58020_e90236_d_n0;
        locals.var_t1_dn2 = assign58020_e90236_d_n2;
        locals.var_t1_dn4 = assign58020_e90236_d_n4;
        locals.var_t1_dn5 = assign58020_e90236_d_n5;
        locals.var_t1_dn6 = assign58020_e90236_d_n6;
        locals.var_t1_dn7 = assign58020_e90236_d_n7;
        locals.var_t1_dn8 = assign58020_e90236_d_n8;
        locals.var_t1_dn9 = assign58020_e90236_d_n9;
        locals.var_t1_dn10 = assign58020_e90236_d_n10;
        locals.var_t1_dn11 = assign58020_e90236_d_n11;
        locals.var_t1_dn14 = assign58020_e90236_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58030_e90245, assign58030_e90245_d_n0, assign58030_e90245_d_n2, assign58030_e90245_d_n4, assign58030_e90245_d_n5, assign58030_e90245_d_n6, assign58030_e90245_d_n7, assign58030_e90245_d_n8, assign58030_e90245_d_n9, assign58030_e90245_d_n10, assign58030_e90245_d_n11, assign58030_e90245_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58030_e90243: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign58030_e90243, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58030_e90245;
        locals.var_t2_dn0 = assign58030_e90245_d_n0;
        locals.var_t2_dn2 = assign58030_e90245_d_n2;
        locals.var_t2_dn4 = assign58030_e90245_d_n4;
        locals.var_t2_dn5 = assign58030_e90245_d_n5;
        locals.var_t2_dn6 = assign58030_e90245_d_n6;
        locals.var_t2_dn7 = assign58030_e90245_d_n7;
        locals.var_t2_dn8 = assign58030_e90245_d_n8;
        locals.var_t2_dn9 = assign58030_e90245_d_n9;
        locals.var_t2_dn10 = assign58030_e90245_d_n10;
        locals.var_t2_dn11 = assign58030_e90245_d_n11;
        locals.var_t2_dn14 = assign58030_e90245_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58040_e90257, assign58040_e90257_d_n0, assign58040_e90257_d_n2, assign58040_e90257_d_n4, assign58040_e90257_d_n5, assign58040_e90257_d_n6, assign58040_e90257_d_n7, assign58040_e90257_d_n8, assign58040_e90257_d_n9, assign58040_e90257_d_n10, assign58040_e90257_d_n11, assign58040_e90257_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58040_e90253: f64 = (locals.var_t2 * locals.var_t2);
        let assign58040_e90254: f64 = (1.0 + assign58040_e90253);
        let assign58040_e90255: f64 = (assign58040_e90254).sqrt();
        (assign58040_e90255, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign58040_e90255)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign58040_e90255)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign58040_e90257;
        locals.var_t3_dn0 = assign58040_e90257_d_n0;
        locals.var_t3_dn2 = assign58040_e90257_d_n2;
        locals.var_t3_dn4 = assign58040_e90257_d_n4;
        locals.var_t3_dn5 = assign58040_e90257_d_n5;
        locals.var_t3_dn6 = assign58040_e90257_d_n6;
        locals.var_t3_dn7 = assign58040_e90257_d_n7;
        locals.var_t3_dn8 = assign58040_e90257_d_n8;
        locals.var_t3_dn9 = assign58040_e90257_d_n9;
        locals.var_t3_dn10 = assign58040_e90257_d_n10;
        locals.var_t3_dn11 = assign58040_e90257_d_n11;
        locals.var_t3_dn14 = assign58040_e90257_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign58050_e90268, assign58050_e90268_d_n0, assign58050_e90268_d_n2, assign58050_e90268_d_n4, assign58050_e90268_d_n5, assign58050_e90268_d_n6, assign58050_e90268_d_n7, assign58050_e90268_d_n8, assign58050_e90268_d_n9, assign58050_e90268_d_n10, assign58050_e90268_d_n11, assign58050_e90268_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58050_e90264: f64 = (locals.var_t1 / locals.var_t3);
        let assign58050_e90266: f64 = (assign58050_e90264 + locals.var_vbscl__blk439);
        (assign58050_e90266, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk439_dn14),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
        locals.var_ps0 = assign58050_e90268;
        locals.var_ps0_dn0 = assign58050_e90268_d_n0;
        locals.var_ps0_dn2 = assign58050_e90268_d_n2;
        locals.var_ps0_dn4 = assign58050_e90268_d_n4;
        locals.var_ps0_dn5 = assign58050_e90268_d_n5;
        locals.var_ps0_dn6 = assign58050_e90268_d_n6;
        locals.var_ps0_dn7 = assign58050_e90268_d_n7;
        locals.var_ps0_dn8 = assign58050_e90268_d_n8;
        locals.var_ps0_dn9 = assign58050_e90268_d_n9;
        locals.var_ps0_dn10 = assign58050_e90268_d_n10;
        locals.var_ps0_dn11 = assign58050_e90268_d_n11;
        locals.var_ps0_dn14 = assign58050_e90268_d_n14;
        locals.var_ps0_rv = 0.0;

        let (assign58060_e90275, assign58060_e90275_d_n0, assign58060_e90275_d_n2, assign58060_e90275_d_n4, assign58060_e90275_d_n5, assign58060_e90275_d_n6, assign58060_e90275_d_n7, assign58060_e90275_d_n8, assign58060_e90275_d_n9, assign58060_e90275_d_n10, assign58060_e90275_d_n11, assign58060_e90275_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign58060_e90275;
        locals.var_psl_dn0 = assign58060_e90275_d_n0;
        locals.var_psl_dn2 = assign58060_e90275_d_n2;
        locals.var_psl_dn4 = assign58060_e90275_d_n4;
        locals.var_psl_dn5 = assign58060_e90275_d_n5;
        locals.var_psl_dn6 = assign58060_e90275_d_n6;
        locals.var_psl_dn7 = assign58060_e90275_d_n7;
        locals.var_psl_dn8 = assign58060_e90275_d_n8;
        locals.var_psl_dn9 = assign58060_e90275_d_n9;
        locals.var_psl_dn10 = assign58060_e90275_d_n10;
        locals.var_psl_dn11 = assign58060_e90275_d_n11;
        locals.var_psl_dn14 = assign58060_e90275_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign58070_e90282, assign58070_e90282_d_n0, assign58070_e90282_d_n2, assign58070_e90282_d_n4, assign58070_e90282_d_n5, assign58070_e90282_d_n6, assign58070_e90282_d_n7, assign58070_e90282_d_n8, assign58070_e90282_d_n9, assign58070_e90282_d_n10, assign58070_e90282_d_n11, assign58070_e90282_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign58070_e90282;
        locals.var_pds_dn0 = assign58070_e90282_d_n0;
        locals.var_pds_dn2 = assign58070_e90282_d_n2;
        locals.var_pds_dn4 = assign58070_e90282_d_n4;
        locals.var_pds_dn5 = assign58070_e90282_d_n5;
        locals.var_pds_dn6 = assign58070_e90282_d_n6;
        locals.var_pds_dn7 = assign58070_e90282_d_n7;
        locals.var_pds_dn8 = assign58070_e90282_d_n8;
        locals.var_pds_dn9 = assign58070_e90282_d_n9;
        locals.var_pds_dn10 = assign58070_e90282_d_n10;
        locals.var_pds_dn11 = assign58070_e90282_d_n11;
        locals.var_pds_dn14 = assign58070_e90282_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign58080_e90291, assign58080_e90291_d_n0, assign58080_e90291_d_n2, assign58080_e90291_d_n4, assign58080_e90291_d_n5, assign58080_e90291_d_n6, assign58080_e90291_d_n7, assign58080_e90291_d_n8, assign58080_e90291_d_n9, assign58080_e90291_d_n10, assign58080_e90291_d_n11, assign58080_e90291_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58080_e90289: f64 = (locals.var_vgp - locals.var_ps0);
        (assign58080_e90289, (locals.var_vgp_dn0 - locals.var_ps0_dn0), (locals.var_vgp_dn2 - locals.var_ps0_dn2), (locals.var_vgp_dn4 - locals.var_ps0_dn4), (locals.var_vgp_dn5 - locals.var_ps0_dn5), (locals.var_vgp_dn6 - locals.var_ps0_dn6), (locals.var_vgp_dn7 - locals.var_ps0_dn7), (locals.var_vgp_dn8 - locals.var_ps0_dn8), (locals.var_vgp_dn9 - locals.var_ps0_dn9), (locals.var_vgp_dn10 - locals.var_ps0_dn10), (locals.var_vgp_dn11 - locals.var_ps0_dn11), (locals.var_vgp_dn14 - locals.var_ps0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58080_e90291;
        locals.var_t2_dn0 = assign58080_e90291_d_n0;
        locals.var_t2_dn2 = assign58080_e90291_d_n2;
        locals.var_t2_dn4 = assign58080_e90291_d_n4;
        locals.var_t2_dn5 = assign58080_e90291_d_n5;
        locals.var_t2_dn6 = assign58080_e90291_d_n6;
        locals.var_t2_dn7 = assign58080_e90291_d_n7;
        locals.var_t2_dn8 = assign58080_e90291_d_n8;
        locals.var_t2_dn9 = assign58080_e90291_d_n9;
        locals.var_t2_dn10 = assign58080_e90291_d_n10;
        locals.var_t2_dn11 = assign58080_e90291_d_n11;
        locals.var_t2_dn14 = assign58080_e90291_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58090_e90300, assign58090_e90300_d_n0, assign58090_e90300_d_n2, assign58090_e90300_d_n4, assign58090_e90300_d_n5, assign58090_e90300_d_n6, assign58090_e90300_d_n7, assign58090_e90300_d_n8, assign58090_e90300_d_n9, assign58090_e90300_d_n10, assign58090_e90300_d_n11, assign58090_e90300_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58090_e90298: f64 = (locals.var_cox * locals.var_t2);
        (assign58090_e90298, ((locals.var_cox_dn0 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn0)), ((locals.var_cox_dn2 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn2)), ((locals.var_cox_dn4 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn4)), ((locals.var_cox_dn5 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn5)), ((locals.var_cox_dn6 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn6)), ((locals.var_cox_dn7 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn7)), ((locals.var_cox_dn8 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn8)), ((locals.var_cox_dn9 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn9)), ((locals.var_cox_dn10 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn10)), ((locals.var_cox_dn11 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn11)), ((locals.var_cox_dn14 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign58090_e90300;
        locals.var_qbu_dn0 = assign58090_e90300_d_n0;
        locals.var_qbu_dn2 = assign58090_e90300_d_n2;
        locals.var_qbu_dn4 = assign58090_e90300_d_n4;
        locals.var_qbu_dn5 = assign58090_e90300_d_n5;
        locals.var_qbu_dn6 = assign58090_e90300_d_n6;
        locals.var_qbu_dn7 = assign58090_e90300_d_n7;
        locals.var_qbu_dn8 = assign58090_e90300_d_n8;
        locals.var_qbu_dn9 = assign58090_e90300_d_n9;
        locals.var_qbu_dn10 = assign58090_e90300_d_n10;
        locals.var_qbu_dn11 = assign58090_e90300_d_n11;
        locals.var_qbu_dn14 = assign58090_e90300_d_n14;
        locals.var_qbu_rv = 0.0;

        let (assign58100_e90307, assign58100_e90307_d_n0, assign58100_e90307_d_n2, assign58100_e90307_d_n4, assign58100_e90307_d_n5, assign58100_e90307_d_n6, assign58100_e90307_d_n7, assign58100_e90307_d_n8, assign58100_e90307_d_n9, assign58100_e90307_d_n10, assign58100_e90307_d_n11, assign58100_e90307_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign58100_e90307;
        locals.var_qiu_dn0 = assign58100_e90307_d_n0;
        locals.var_qiu_dn2 = assign58100_e90307_d_n2;
        locals.var_qiu_dn4 = assign58100_e90307_d_n4;
        locals.var_qiu_dn5 = assign58100_e90307_d_n5;
        locals.var_qiu_dn6 = assign58100_e90307_d_n6;
        locals.var_qiu_dn7 = assign58100_e90307_d_n7;
        locals.var_qiu_dn8 = assign58100_e90307_d_n8;
        locals.var_qiu_dn9 = assign58100_e90307_d_n9;
        locals.var_qiu_dn10 = assign58100_e90307_d_n10;
        locals.var_qiu_dn11 = assign58100_e90307_d_n11;
        locals.var_qiu_dn14 = assign58100_e90307_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign58110_e90314, assign58110_e90314_d_n0, assign58110_e90314_d_n2, assign58110_e90314_d_n4, assign58110_e90314_d_n5, assign58110_e90314_d_n6, assign58110_e90314_d_n7, assign58110_e90314_d_n8, assign58110_e90314_d_n9, assign58110_e90314_d_n10, assign58110_e90314_d_n11, assign58110_e90314_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign58110_e90314;
        locals.var_qdrat_dn0 = assign58110_e90314_d_n0;
        locals.var_qdrat_dn2 = assign58110_e90314_d_n2;
        locals.var_qdrat_dn4 = assign58110_e90314_d_n4;
        locals.var_qdrat_dn5 = assign58110_e90314_d_n5;
        locals.var_qdrat_dn6 = assign58110_e90314_d_n6;
        locals.var_qdrat_dn7 = assign58110_e90314_d_n7;
        locals.var_qdrat_dn8 = assign58110_e90314_d_n8;
        locals.var_qdrat_dn9 = assign58110_e90314_d_n9;
        locals.var_qdrat_dn10 = assign58110_e90314_d_n10;
        locals.var_qdrat_dn11 = assign58110_e90314_d_n11;
        locals.var_qdrat_dn14 = assign58110_e90314_d_n14;
        locals.var_qdrat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_213(
        locals: &mut StampLocals,
    ) {
        let (assign58120_e90321, assign58120_e90321_d_n0, assign58120_e90321_d_n2, assign58120_e90321_d_n4, assign58120_e90321_d_n5, assign58120_e90321_d_n6, assign58120_e90321_d_n7, assign58120_e90321_d_n8, assign58120_e90321_d_n9, assign58120_e90321_d_n10, assign58120_e90321_d_n11, assign58120_e90321_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign58120_e90321;
        locals.var_lred_dn0 = assign58120_e90321_d_n0;
        locals.var_lred_dn2 = assign58120_e90321_d_n2;
        locals.var_lred_dn4 = assign58120_e90321_d_n4;
        locals.var_lred_dn5 = assign58120_e90321_d_n5;
        locals.var_lred_dn6 = assign58120_e90321_d_n6;
        locals.var_lred_dn7 = assign58120_e90321_d_n7;
        locals.var_lred_dn8 = assign58120_e90321_d_n8;
        locals.var_lred_dn9 = assign58120_e90321_d_n9;
        locals.var_lred_dn10 = assign58120_e90321_d_n10;
        locals.var_lred_dn11 = assign58120_e90321_d_n11;
        locals.var_lred_dn14 = assign58120_e90321_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign58130_e90328, assign58130_e90328_d_n0, assign58130_e90328_d_n2, assign58130_e90328_d_n4, assign58130_e90328_d_n5, assign58130_e90328_d_n6, assign58130_e90328_d_n7, assign58130_e90328_d_n8, assign58130_e90328_d_n9, assign58130_e90328_d_n10, assign58130_e90328_d_n11, assign58130_e90328_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign58130_e90328;
        locals.var_ids_dn0 = assign58130_e90328_d_n0;
        locals.var_ids_dn2 = assign58130_e90328_d_n2;
        locals.var_ids_dn4 = assign58130_e90328_d_n4;
        locals.var_ids_dn5 = assign58130_e90328_d_n5;
        locals.var_ids_dn6 = assign58130_e90328_d_n6;
        locals.var_ids_dn7 = assign58130_e90328_d_n7;
        locals.var_ids_dn8 = assign58130_e90328_d_n8;
        locals.var_ids_dn9 = assign58130_e90328_d_n9;
        locals.var_ids_dn10 = assign58130_e90328_d_n10;
        locals.var_ids_dn11 = assign58130_e90328_d_n11;
        locals.var_ids_dn14 = assign58130_e90328_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign58140_e90335, assign58140_e90335_d_n0, assign58140_e90335_d_n2, assign58140_e90335_d_n4, assign58140_e90335_d_n5, assign58140_e90335_d_n6, assign58140_e90335_d_n7, assign58140_e90335_d_n8, assign58140_e90335_d_n9, assign58140_e90335_d_n10, assign58140_e90335_d_n11, assign58140_e90335_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn4, locals.var_vgvt_dn5, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn8, locals.var_vgvt_dn9, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn14,)
    }
};
        locals.var_vgvt = assign58140_e90335;
        locals.var_vgvt_dn0 = assign58140_e90335_d_n0;
        locals.var_vgvt_dn2 = assign58140_e90335_d_n2;
        locals.var_vgvt_dn4 = assign58140_e90335_d_n4;
        locals.var_vgvt_dn5 = assign58140_e90335_d_n5;
        locals.var_vgvt_dn6 = assign58140_e90335_d_n6;
        locals.var_vgvt_dn7 = assign58140_e90335_d_n7;
        locals.var_vgvt_dn8 = assign58140_e90335_d_n8;
        locals.var_vgvt_dn9 = assign58140_e90335_d_n9;
        locals.var_vgvt_dn10 = assign58140_e90335_d_n10;
        locals.var_vgvt_dn11 = assign58140_e90335_d_n11;
        locals.var_vgvt_dn14 = assign58140_e90335_d_n14;
        locals.var_vgvt_rv = 0.0;

        let (assign58150_e90342,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58150_e90342;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58160_e90349,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign58160_e90349;
        locals.var_end_of_part_1_rv = 0.0;

        let assign58170_e90352: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1434 = assign58170_e90352;
        locals.var_guard1434_rv = 0.0;

        let (assign58180_e90373, assign58180_e90373_d_n0, assign58180_e90373_d_n2, assign58180_e90373_d_n4, assign58180_e90373_d_n5, assign58180_e90373_d_n6, assign58180_e90373_d_n7, assign58180_e90373_d_n8, assign58180_e90373_d_n9, assign58180_e90373_d_n10, assign58180_e90373_d_n11, assign58180_e90373_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58180_e90362: f64 = (locals.var_vgp - locals.var_vbscl__blk439);
        let assign58180_e90363: f64 = (locals.var_beta * assign58180_e90362);
        let assign58180_e90365: f64 = (assign58180_e90363 - 1.0);
        let assign58180_e90366: f64 = (4.0 * assign58180_e90365);
        let assign58180_e90369: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign58180_e90370: f64 = (assign58180_e90366 / assign58180_e90369);
        let assign58180_e90371: f64 = (1.0 + assign58180_e90370);
        (assign58180_e90371, ((((4.0 * ((locals.var_beta_dn0 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk439_dn0)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn2 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk439_dn2)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn4 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk439_dn4)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn5 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk439_dn5)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn6 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk439_dn6)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn7 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk439_dn7)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn8 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk439_dn8)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn9 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk439_dn9)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn10 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk439_dn10)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn11 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbscl__blk439_dn11)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign58180_e90369 * assign58180_e90369)), ((((4.0 * ((locals.var_beta_dn14 * assign58180_e90362) + (locals.var_beta * (locals.var_vgp_dn14 - locals.var_vbscl__blk439_dn14)))) * assign58180_e90369) - (assign58180_e90366 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign58180_e90369 * assign58180_e90369)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58180_e90373;
        locals.var_tx_dn0 = assign58180_e90373_d_n0;
        locals.var_tx_dn2 = assign58180_e90373_d_n2;
        locals.var_tx_dn4 = assign58180_e90373_d_n4;
        locals.var_tx_dn5 = assign58180_e90373_d_n5;
        locals.var_tx_dn6 = assign58180_e90373_d_n6;
        locals.var_tx_dn7 = assign58180_e90373_d_n7;
        locals.var_tx_dn8 = assign58180_e90373_d_n8;
        locals.var_tx_dn9 = assign58180_e90373_d_n9;
        locals.var_tx_dn10 = assign58180_e90373_d_n10;
        locals.var_tx_dn11 = assign58180_e90373_d_n11;
        locals.var_tx_dn14 = assign58180_e90373_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58190_e90389, assign58190_e90389_d_n0, assign58190_e90389_d_n2, assign58190_e90389_d_n4, assign58190_e90389_d_n5, assign58190_e90389_d_n6, assign58190_e90389_d_n7, assign58190_e90389_d_n8, assign58190_e90389_d_n9, assign58190_e90389_d_n10, assign58190_e90389_d_n11, assign58190_e90389_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58190_e90381: f64 = (10.0 * 2.220446049250313e-16);
        let (assign58190_e90387, assign58190_e90387_d_n0, assign58190_e90387_d_n2, assign58190_e90387_d_n4, assign58190_e90387_d_n5, assign58190_e90387_d_n6, assign58190_e90387_d_n7, assign58190_e90387_d_n8, assign58190_e90387_d_n9, assign58190_e90387_d_n10, assign58190_e90387_d_n11, assign58190_e90387_d_n14,) = {
            if (locals.var_tx >= assign58190_e90381) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
            } else {
                let assign58190_e90386: f64 = (10.0 * 2.220446049250313e-16);
                (assign58190_e90386, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign58190_e90387, assign58190_e90387_d_n0, assign58190_e90387_d_n2, assign58190_e90387_d_n4, assign58190_e90387_d_n5, assign58190_e90387_d_n6, assign58190_e90387_d_n7, assign58190_e90387_d_n8, assign58190_e90387_d_n9, assign58190_e90387_d_n10, assign58190_e90387_d_n11, assign58190_e90387_d_n14,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58190_e90389;
        locals.var_tx_dn0 = assign58190_e90389_d_n0;
        locals.var_tx_dn2 = assign58190_e90389_d_n2;
        locals.var_tx_dn4 = assign58190_e90389_d_n4;
        locals.var_tx_dn5 = assign58190_e90389_d_n5;
        locals.var_tx_dn6 = assign58190_e90389_d_n6;
        locals.var_tx_dn7 = assign58190_e90389_d_n7;
        locals.var_tx_dn8 = assign58190_e90389_d_n8;
        locals.var_tx_dn9 = assign58190_e90389_d_n9;
        locals.var_tx_dn10 = assign58190_e90389_d_n10;
        locals.var_tx_dn11 = assign58190_e90389_d_n11;
        locals.var_tx_dn14 = assign58190_e90389_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58200_e90407, assign58200_e90407_d_n0, assign58200_e90407_d_n2, assign58200_e90407_d_n4, assign58200_e90407_d_n5, assign58200_e90407_d_n6, assign58200_e90407_d_n7, assign58200_e90407_d_n8, assign58200_e90407_d_n9, assign58200_e90407_d_n10, assign58200_e90407_d_n11, assign58200_e90407_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58200_e90397: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign58200_e90399: f64 = (assign58200_e90397 * 0.5);
        let assign58200_e90402: f64 = (locals.var_tx).sqrt();
        let assign58200_e90403: f64 = (1.0 - assign58200_e90402);
        let assign58200_e90404: f64 = (assign58200_e90399 * assign58200_e90403);
        let assign58200_e90405: f64 = (locals.var_vgp + assign58200_e90404);
        (assign58200_e90405, (locals.var_vgp_dn0 + (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn0 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn2 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn4 + (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn4 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn5 + (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn5 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn6 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn7 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn8 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn9 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn10 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn11 + (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn11 / (2.0 * assign58200_e90402)))))), (locals.var_vgp_dn14 + (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) * 0.5) * assign58200_e90403) + (assign58200_e90399 * (-(locals.var_tx_dn14 / (2.0 * assign58200_e90402)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign58200_e90407;
        locals.var_ps0_inia_dn0 = assign58200_e90407_d_n0;
        locals.var_ps0_inia_dn2 = assign58200_e90407_d_n2;
        locals.var_ps0_inia_dn4 = assign58200_e90407_d_n4;
        locals.var_ps0_inia_dn5 = assign58200_e90407_d_n5;
        locals.var_ps0_inia_dn6 = assign58200_e90407_d_n6;
        locals.var_ps0_inia_dn7 = assign58200_e90407_d_n7;
        locals.var_ps0_inia_dn8 = assign58200_e90407_d_n8;
        locals.var_ps0_inia_dn9 = assign58200_e90407_d_n9;
        locals.var_ps0_inia_dn10 = assign58200_e90407_d_n10;
        locals.var_ps0_inia_dn11 = assign58200_e90407_d_n11;
        locals.var_ps0_inia_dn14 = assign58200_e90407_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let assign58210_e90410: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1435 = assign58210_e90410;
        locals.var_guard1435_rv = 0.0;

        let (assign58220_e90423, assign58220_e90423_d_n0, assign58220_e90423_d_n2, assign58220_e90423_d_n4, assign58220_e90423_d_n5, assign58220_e90423_d_n6, assign58220_e90423_d_n7, assign58220_e90423_d_n8, assign58220_e90423_d_n9, assign58220_e90423_d_n10, assign58220_e90423_d_n11, assign58220_e90423_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58220_e90420: f64 = (locals.var_ps0_inia - locals.var_vbscl__blk439);
        let assign58220_e90421: f64 = (locals.var_beta * assign58220_e90420);
        (assign58220_e90421, ((locals.var_beta_dn0 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbscl__blk439_dn0))), ((locals.var_beta_dn2 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbscl__blk439_dn2))), ((locals.var_beta_dn4 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn4 - locals.var_vbscl__blk439_dn4))), ((locals.var_beta_dn5 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn5 - locals.var_vbscl__blk439_dn5))), ((locals.var_beta_dn6 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbscl__blk439_dn6))), ((locals.var_beta_dn7 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbscl__blk439_dn7))), ((locals.var_beta_dn8 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn8 - locals.var_vbscl__blk439_dn8))), ((locals.var_beta_dn9 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn9 - locals.var_vbscl__blk439_dn9))), ((locals.var_beta_dn10 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbscl__blk439_dn10))), ((locals.var_beta_dn11 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbscl__blk439_dn11))), ((locals.var_beta_dn14 * assign58220_e90420) + (locals.var_beta * (locals.var_ps0_inia_dn14 - locals.var_vbscl__blk439_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign58220_e90423;
        locals.var_chi_dn0 = assign58220_e90423_d_n0;
        locals.var_chi_dn2 = assign58220_e90423_d_n2;
        locals.var_chi_dn4 = assign58220_e90423_d_n4;
        locals.var_chi_dn5 = assign58220_e90423_d_n5;
        locals.var_chi_dn6 = assign58220_e90423_d_n6;
        locals.var_chi_dn7 = assign58220_e90423_d_n7;
        locals.var_chi_dn8 = assign58220_e90423_d_n8;
        locals.var_chi_dn9 = assign58220_e90423_d_n9;
        locals.var_chi_dn10 = assign58220_e90423_d_n10;
        locals.var_chi_dn11 = assign58220_e90423_d_n11;
        locals.var_chi_dn14 = assign58220_e90423_d_n14;
        locals.var_chi_rv = 0.0;

        let assign58230_e90426: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1436 = assign58230_e90426;
        locals.var_guard1436_rv = 0.0;

        let (assign58240_e90441, assign58240_e90441_d_n0, assign58240_e90441_d_n2, assign58240_e90441_d_n4, assign58240_e90441_d_n5, assign58240_e90441_d_n6, assign58240_e90441_d_n7, assign58240_e90441_d_n8, assign58240_e90441_d_n9, assign58240_e90441_d_n10, assign58240_e90441_d_n11, assign58240_e90441_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58240_e90438: f64 = (locals.var_vgp - locals.var_vbscl__blk439);
        let assign58240_e90439: f64 = (locals.var_beta * assign58240_e90438);
        (assign58240_e90439, ((locals.var_beta_dn0 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk439_dn0))), ((locals.var_beta_dn2 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk439_dn2))), ((locals.var_beta_dn4 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk439_dn4))), ((locals.var_beta_dn5 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk439_dn5))), ((locals.var_beta_dn6 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk439_dn6))), ((locals.var_beta_dn7 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk439_dn7))), ((locals.var_beta_dn8 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk439_dn8))), ((locals.var_beta_dn9 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk439_dn9))), ((locals.var_beta_dn10 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk439_dn10))), ((locals.var_beta_dn11 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbscl__blk439_dn11))), ((locals.var_beta_dn14 * assign58240_e90438) + (locals.var_beta * (locals.var_vgp_dn14 - locals.var_vbscl__blk439_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign58240_e90441;
        locals.var_ty_dn0 = assign58240_e90441_d_n0;
        locals.var_ty_dn2 = assign58240_e90441_d_n2;
        locals.var_ty_dn4 = assign58240_e90441_d_n4;
        locals.var_ty_dn5 = assign58240_e90441_d_n5;
        locals.var_ty_dn6 = assign58240_e90441_d_n6;
        locals.var_ty_dn7 = assign58240_e90441_d_n7;
        locals.var_ty_dn8 = assign58240_e90441_d_n8;
        locals.var_ty_dn9 = assign58240_e90441_d_n9;
        locals.var_ty_dn10 = assign58240_e90441_d_n10;
        locals.var_ty_dn11 = assign58240_e90441_d_n11;
        locals.var_ty_dn14 = assign58240_e90441_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign58250_e90460, assign58250_e90460_d_n0, assign58250_e90460_d_n2, assign58250_e90460_d_n4, assign58250_e90460_d_n5, assign58250_e90460_d_n6, assign58250_e90460_d_n7, assign58250_e90460_d_n8, assign58250_e90460_d_n9, assign58250_e90460_d_n10, assign58250_e90460_d_n11, assign58250_e90460_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58250_e90453: f64 = (1.414213562373095 / 108.0);
        let assign58250_e90455: f64 = (assign58250_e90453 * locals.var_beta);
        let assign58250_e90457: f64 = (assign58250_e90455 * locals.var_fac1);
        let assign58250_e90458: f64 = (1.0 / assign58250_e90457);
        (assign58250_e90458, (-((((assign58250_e90453 * locals.var_beta_dn0) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn0)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn2) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn2)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn4) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn4)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn5) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn5)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn6) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn6)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn7) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn7)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn8) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn8)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn9) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn9)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn10) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn10)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn11) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn11)) / (assign58250_e90457 * assign58250_e90457))), (-((((assign58250_e90453 * locals.var_beta_dn14) * locals.var_fac1) + (assign58250_e90455 * locals.var_fac1_dn14)) / (assign58250_e90457 * assign58250_e90457))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58250_e90460;
        locals.var_t1_dn0 = assign58250_e90460_d_n0;
        locals.var_t1_dn2 = assign58250_e90460_d_n2;
        locals.var_t1_dn4 = assign58250_e90460_d_n4;
        locals.var_t1_dn5 = assign58250_e90460_d_n5;
        locals.var_t1_dn6 = assign58250_e90460_d_n6;
        locals.var_t1_dn7 = assign58250_e90460_d_n7;
        locals.var_t1_dn8 = assign58250_e90460_d_n8;
        locals.var_t1_dn9 = assign58250_e90460_d_n9;
        locals.var_t1_dn10 = assign58250_e90460_d_n10;
        locals.var_t1_dn11 = assign58250_e90460_d_n11;
        locals.var_t1_dn14 = assign58250_e90460_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58260_e90475, assign58260_e90475_d_n0, assign58260_e90475_d_n2, assign58260_e90475_d_n4, assign58260_e90475_d_n5, assign58260_e90475_d_n6, assign58260_e90475_d_n7, assign58260_e90475_d_n8, assign58260_e90475_d_n9, assign58260_e90475_d_n10, assign58260_e90475_d_n11, assign58260_e90475_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58260_e90472: f64 = (3.0 * locals.var_t1);
        let assign58260_e90473: f64 = (81.0 + assign58260_e90472);
        (assign58260_e90473, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn4), (3.0 * locals.var_t1_dn5), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn8), (3.0 * locals.var_t1_dn9), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58260_e90475;
        locals.var_t2_dn0 = assign58260_e90475_d_n0;
        locals.var_t2_dn2 = assign58260_e90475_d_n2;
        locals.var_t2_dn4 = assign58260_e90475_d_n4;
        locals.var_t2_dn5 = assign58260_e90475_d_n5;
        locals.var_t2_dn6 = assign58260_e90475_d_n6;
        locals.var_t2_dn7 = assign58260_e90475_d_n7;
        locals.var_t2_dn8 = assign58260_e90475_d_n8;
        locals.var_t2_dn9 = assign58260_e90475_d_n9;
        locals.var_t2_dn10 = assign58260_e90475_d_n10;
        locals.var_t2_dn11 = assign58260_e90475_d_n11;
        locals.var_t2_dn14 = assign58260_e90475_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58270_e90497, assign58270_e90497_d_n0, assign58270_e90497_d_n2, assign58270_e90497_d_n4, assign58270_e90497_d_n5, assign58270_e90497_d_n6, assign58270_e90497_d_n7, assign58270_e90497_d_n8, assign58270_e90497_d_n9, assign58270_e90497_d_n10, assign58270_e90497_d_n11, assign58270_e90497_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58270_e90485: f64 = (-2916.0);
        let assign58270_e90488: f64 = (81.0 * locals.var_t1);
        let assign58270_e90489: f64 = (assign58270_e90485 - assign58270_e90488);
        let assign58270_e90492: f64 = (27.0 * locals.var_t1);
        let assign58270_e90494: f64 = (assign58270_e90492 * locals.var_ty);
        let assign58270_e90495: f64 = (assign58270_e90489 + assign58270_e90494);
        (assign58270_e90495, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn14)) + (((27.0 * locals.var_t1_dn14) * locals.var_ty) + (assign58270_e90492 * locals.var_ty_dn14))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign58270_e90497;
        locals.var_t3_dn0 = assign58270_e90497_d_n0;
        locals.var_t3_dn2 = assign58270_e90497_d_n2;
        locals.var_t3_dn4 = assign58270_e90497_d_n4;
        locals.var_t3_dn5 = assign58270_e90497_d_n5;
        locals.var_t3_dn6 = assign58270_e90497_d_n6;
        locals.var_t3_dn7 = assign58270_e90497_d_n7;
        locals.var_t3_dn8 = assign58270_e90497_d_n8;
        locals.var_t3_dn9 = assign58270_e90497_d_n9;
        locals.var_t3_dn10 = assign58270_e90497_d_n10;
        locals.var_t3_dn11 = assign58270_e90497_d_n11;
        locals.var_t3_dn14 = assign58270_e90497_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign58280_e90520, assign58280_e90520_d_n0, assign58280_e90520_d_n2, assign58280_e90520_d_n4, assign58280_e90520_d_n5, assign58280_e90520_d_n6, assign58280_e90520_d_n7, assign58280_e90520_d_n8, assign58280_e90520_d_n9, assign58280_e90520_d_n10, assign58280_e90520_d_n11, assign58280_e90520_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58280_e90510: f64 = (54.0 + locals.var_t1);
        let assign58280_e90511: f64 = (81.0 * assign58280_e90510);
        let assign58280_e90512: f64 = (1458.0 - assign58280_e90511);
        let assign58280_e90515: f64 = (27.0 * locals.var_t1);
        let assign58280_e90517: f64 = (assign58280_e90515 * locals.var_ty);
        let assign58280_e90518: f64 = (assign58280_e90512 + assign58280_e90517);
        (assign58280_e90518, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn14)) + (((27.0 * locals.var_t1_dn14) * locals.var_ty) + (assign58280_e90515 * locals.var_ty_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign58280_e90520;
        locals.var_t4_dn0 = assign58280_e90520_d_n0;
        locals.var_t4_dn2 = assign58280_e90520_d_n2;
        locals.var_t4_dn4 = assign58280_e90520_d_n4;
        locals.var_t4_dn5 = assign58280_e90520_d_n5;
        locals.var_t4_dn6 = assign58280_e90520_d_n6;
        locals.var_t4_dn7 = assign58280_e90520_d_n7;
        locals.var_t4_dn8 = assign58280_e90520_d_n8;
        locals.var_t4_dn9 = assign58280_e90520_d_n9;
        locals.var_t4_dn10 = assign58280_e90520_d_n10;
        locals.var_t4_dn11 = assign58280_e90520_d_n11;
        locals.var_t4_dn14 = assign58280_e90520_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign58290_e90533, assign58290_e90533_d_n0, assign58290_e90533_d_n2, assign58290_e90533_d_n4, assign58290_e90533_d_n5, assign58290_e90533_d_n6, assign58290_e90533_d_n7, assign58290_e90533_d_n8, assign58290_e90533_d_n9, assign58290_e90533_d_n10, assign58290_e90533_d_n11, assign58290_e90533_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58290_e90531: f64 = (locals.var_t4 * locals.var_t4);
        (assign58290_e90531, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign58290_e90533;
        locals.var_t4_dn0 = assign58290_e90533_d_n0;
        locals.var_t4_dn2 = assign58290_e90533_d_n2;
        locals.var_t4_dn4 = assign58290_e90533_d_n4;
        locals.var_t4_dn5 = assign58290_e90533_d_n5;
        locals.var_t4_dn6 = assign58290_e90533_d_n6;
        locals.var_t4_dn7 = assign58290_e90533_d_n7;
        locals.var_t4_dn8 = assign58290_e90533_d_n8;
        locals.var_t4_dn9 = assign58290_e90533_d_n9;
        locals.var_t4_dn10 = assign58290_e90533_d_n10;
        locals.var_t4_dn11 = assign58290_e90533_d_n11;
        locals.var_t4_dn14 = assign58290_e90533_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign58300_e90573, assign58300_e90573_d_n0, assign58300_e90573_d_n2, assign58300_e90573_d_n4, assign58300_e90573_d_n5, assign58300_e90573_d_n6, assign58300_e90573_d_n7, assign58300_e90573_d_n8, assign58300_e90573_d_n9, assign58300_e90573_d_n10, assign58300_e90573_d_n11, assign58300_e90573_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58300_e90545: f64 = (4.0 * locals.var_t2);
        let assign58300_e90547: f64 = (assign58300_e90545 * locals.var_t2);
        let assign58300_e90549: f64 = (assign58300_e90547 * locals.var_t2);
        let assign58300_e90551: f64 = (assign58300_e90549 + locals.var_t4);
        let assign58300_e90552: f64 = (assign58300_e90551).sqrt();
        let assign58300_e90553: f64 = (locals.var_t3 + assign58300_e90552);
        let (assign58300_e90571, assign58300_e90571_d_n0, assign58300_e90571_d_n2, assign58300_e90571_d_n4, assign58300_e90571_d_n5, assign58300_e90571_d_n6, assign58300_e90571_d_n7, assign58300_e90571_d_n8, assign58300_e90571_d_n9, assign58300_e90571_d_n10, assign58300_e90571_d_n11, assign58300_e90571_d_n14,) = {
            if (assign58300_e90553 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign58300_e90560: f64 = (4.0 * locals.var_t2);
                let assign58300_e90562: f64 = (assign58300_e90560 * locals.var_t2);
                let assign58300_e90564: f64 = (assign58300_e90562 * locals.var_t2);
                let assign58300_e90566: f64 = (assign58300_e90564 + locals.var_t4);
                let assign58300_e90567: f64 = (assign58300_e90566).sqrt();
                let assign58300_e90568: f64 = (locals.var_t3 + assign58300_e90567);
                let assign58300_e90570: f64 = (assign58300_e90568).powf(0.3333333333333333);
                (assign58300_e90570, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn0)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn0)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn2)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn2)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn4)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn4)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn5)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn5)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn6)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn6)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn7)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn7)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn8)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn8)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn9)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn9)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn10)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn10)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn11)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn11)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign58300_e90567))) / assign58300_e90568))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58300_e90568).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn14 + (((((((4.0 * locals.var_t2_dn14) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn14)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn14)) + locals.var_t4_dn14) / (2.0 * assign58300_e90567))))) } } else { (assign58300_e90570 * (0.3333333333333333 * ((locals.var_t3_dn14 + (((((((4.0 * locals.var_t2_dn14) * locals.var_t2) + (assign58300_e90560 * locals.var_t2_dn14)) * locals.var_t2) + (assign58300_e90562 * locals.var_t2_dn14)) + locals.var_t4_dn14) / (2.0 * assign58300_e90567))) / assign58300_e90568))) },)
            }
        };
        (assign58300_e90571, assign58300_e90571_d_n0, assign58300_e90571_d_n2, assign58300_e90571_d_n4, assign58300_e90571_d_n5, assign58300_e90571_d_n6, assign58300_e90571_d_n7, assign58300_e90571_d_n8, assign58300_e90571_d_n9, assign58300_e90571_d_n10, assign58300_e90571_d_n11, assign58300_e90571_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign58300_e90573;
        locals.var_t5_dn0 = assign58300_e90573_d_n0;
        locals.var_t5_dn2 = assign58300_e90573_d_n2;
        locals.var_t5_dn4 = assign58300_e90573_d_n4;
        locals.var_t5_dn5 = assign58300_e90573_d_n5;
        locals.var_t5_dn6 = assign58300_e90573_d_n6;
        locals.var_t5_dn7 = assign58300_e90573_d_n7;
        locals.var_t5_dn8 = assign58300_e90573_d_n8;
        locals.var_t5_dn9 = assign58300_e90573_d_n9;
        locals.var_t5_dn10 = assign58300_e90573_d_n10;
        locals.var_t5_dn11 = assign58300_e90573_d_n11;
        locals.var_t5_dn14 = assign58300_e90573_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign58310_e90600, assign58310_e90600_d_n0, assign58310_e90600_d_n2, assign58310_e90600_d_n4, assign58310_e90600_d_n5, assign58310_e90600_d_n6, assign58310_e90600_d_n7, assign58310_e90600_d_n8, assign58310_e90600_d_n9, assign58310_e90600_d_n10, assign58310_e90600_d_n11, assign58310_e90600_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58310_e90585: f64 = (1.259921049894873 * locals.var_t2);
        let assign58310_e90588: f64 = (3.0 * locals.var_t5);
        let assign58310_e90589: f64 = (assign58310_e90585 / assign58310_e90588);
        let assign58310_e90590: f64 = (3.0 - assign58310_e90589);
        let assign58310_e90594: f64 = (3.0 * 1.259921049894873);
        let assign58310_e90595: f64 = (1.0 / assign58310_e90594);
        let assign58310_e90597: f64 = (assign58310_e90595 * locals.var_t5);
        let assign58310_e90598: f64 = (assign58310_e90590 + assign58310_e90597);
        (assign58310_e90598, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn0))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn2))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn4) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn4))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn4)), ((-((((1.259921049894873 * locals.var_t2_dn5) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn5))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn5)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn6))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn7))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn8) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn8))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn8)), ((-((((1.259921049894873 * locals.var_t2_dn9) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn9))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn9)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn10))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn11))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn14) * assign58310_e90588) - (assign58310_e90585 * (3.0 * locals.var_t5_dn14))) / (assign58310_e90588 * assign58310_e90588))) + (assign58310_e90595 * locals.var_t5_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58310_e90600;
        locals.var_tx_dn0 = assign58310_e90600_d_n0;
        locals.var_tx_dn2 = assign58310_e90600_d_n2;
        locals.var_tx_dn4 = assign58310_e90600_d_n4;
        locals.var_tx_dn5 = assign58310_e90600_d_n5;
        locals.var_tx_dn6 = assign58310_e90600_d_n6;
        locals.var_tx_dn7 = assign58310_e90600_d_n7;
        locals.var_tx_dn8 = assign58310_e90600_d_n8;
        locals.var_tx_dn9 = assign58310_e90600_d_n9;
        locals.var_tx_dn10 = assign58310_e90600_d_n10;
        locals.var_tx_dn11 = assign58310_e90600_d_n11;
        locals.var_tx_dn14 = assign58310_e90600_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58320_e90615, assign58320_e90615_d_n0, assign58320_e90615_d_n2, assign58320_e90615_d_n4, assign58320_e90615_d_n5, assign58320_e90615_d_n6, assign58320_e90615_d_n7, assign58320_e90615_d_n8, assign58320_e90615_d_n9, assign58320_e90615_d_n10, assign58320_e90615_d_n11, assign58320_e90615_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58320_e90611: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign58320_e90613: f64 = (assign58320_e90611 + locals.var_vbscl__blk439);
        (assign58320_e90613, (((locals.var_tx_dn0 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk439_dn0), (((locals.var_tx_dn2 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk439_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk439_dn4), (((locals.var_tx_dn5 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk439_dn5), (((locals.var_tx_dn6 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk439_dn6), (((locals.var_tx_dn7 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk439_dn7), (((locals.var_tx_dn8 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk439_dn8), (((locals.var_tx_dn9 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk439_dn9), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk439_dn10), (((locals.var_tx_dn11 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn11)) + locals.var_vbscl__blk439_dn11), (((locals.var_tx_dn14 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn14)) + locals.var_vbscl__blk439_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign58320_e90615;
        locals.var_ps0_inia_dn0 = assign58320_e90615_d_n0;
        locals.var_ps0_inia_dn2 = assign58320_e90615_d_n2;
        locals.var_ps0_inia_dn4 = assign58320_e90615_d_n4;
        locals.var_ps0_inia_dn5 = assign58320_e90615_d_n5;
        locals.var_ps0_inia_dn6 = assign58320_e90615_d_n6;
        locals.var_ps0_inia_dn7 = assign58320_e90615_d_n7;
        locals.var_ps0_inia_dn8 = assign58320_e90615_d_n8;
        locals.var_ps0_inia_dn9 = assign58320_e90615_d_n9;
        locals.var_ps0_inia_dn10 = assign58320_e90615_d_n10;
        locals.var_ps0_inia_dn11 = assign58320_e90615_d_n11;
        locals.var_ps0_inia_dn14 = assign58320_e90615_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign58330_e90626, assign58330_e90626_d_n0, assign58330_e90626_d_n2, assign58330_e90626_d_n4, assign58330_e90626_d_n5, assign58330_e90626_d_n6, assign58330_e90626_d_n7, assign58330_e90626_d_n8, assign58330_e90626_d_n9, assign58330_e90626_d_n10, assign58330_e90626_d_n11, assign58330_e90626_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign58330_e90626;
        locals.var_ps0_ini_dn0 = assign58330_e90626_d_n0;
        locals.var_ps0_ini_dn2 = assign58330_e90626_d_n2;
        locals.var_ps0_ini_dn4 = assign58330_e90626_d_n4;
        locals.var_ps0_ini_dn5 = assign58330_e90626_d_n5;
        locals.var_ps0_ini_dn6 = assign58330_e90626_d_n6;
        locals.var_ps0_ini_dn7 = assign58330_e90626_d_n7;
        locals.var_ps0_ini_dn8 = assign58330_e90626_d_n8;
        locals.var_ps0_ini_dn9 = assign58330_e90626_d_n9;
        locals.var_ps0_ini_dn10 = assign58330_e90626_d_n10;
        locals.var_ps0_ini_dn11 = assign58330_e90626_d_n11;
        locals.var_ps0_ini_dn14 = assign58330_e90626_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let assign58340_e90629: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard1437 = assign58340_e90629;
        locals.var_guard1437_rv = 0.0;

        let (assign58350_e90643, assign58350_e90643_d_n0, assign58350_e90643_d_n2, assign58350_e90643_d_n4, assign58350_e90643_d_n5, assign58350_e90643_d_n6, assign58350_e90643_d_n7, assign58350_e90643_d_n8, assign58350_e90643_d_n9, assign58350_e90643_d_n10, assign58350_e90643_d_n11, assign58350_e90643_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign58350_e90643;
        locals.var_ps0_ini_dn0 = assign58350_e90643_d_n0;
        locals.var_ps0_ini_dn2 = assign58350_e90643_d_n2;
        locals.var_ps0_ini_dn4 = assign58350_e90643_d_n4;
        locals.var_ps0_ini_dn5 = assign58350_e90643_d_n5;
        locals.var_ps0_ini_dn6 = assign58350_e90643_d_n6;
        locals.var_ps0_ini_dn7 = assign58350_e90643_d_n7;
        locals.var_ps0_ini_dn8 = assign58350_e90643_d_n8;
        locals.var_ps0_ini_dn9 = assign58350_e90643_d_n9;
        locals.var_ps0_ini_dn10 = assign58350_e90643_d_n10;
        locals.var_ps0_ini_dn11 = assign58350_e90643_d_n11;
        locals.var_ps0_ini_dn14 = assign58350_e90643_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58360_e90662, assign58360_e90662_d_n0, assign58360_e90662_d_n2, assign58360_e90662_d_n4, assign58360_e90662_d_n5, assign58360_e90662_d_n6, assign58360_e90662_d_n7, assign58360_e90662_d_n8, assign58360_e90662_d_n9, assign58360_e90662_d_n10, assign58360_e90662_d_n11, assign58360_e90662_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58360_e90658: f64 = (1.0 / locals.var_cnst1);
        let assign58360_e90660: f64 = (assign58360_e90658 / locals.var_cnstcoxi);
        (assign58360_e90660, ((((-(locals.var_cnst1_dn0 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn0)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn2 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn2)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn4 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn4)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn5 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn5)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn6 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn6)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn7 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn7)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn8 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn8)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn9 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn9)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn10 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn10)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn11 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn11)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn14 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58360_e90658 * locals.var_cnstcoxi_dn14)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58360_e90662;
        locals.var_t1_dn0 = assign58360_e90662_d_n0;
        locals.var_t1_dn2 = assign58360_e90662_d_n2;
        locals.var_t1_dn4 = assign58360_e90662_d_n4;
        locals.var_t1_dn5 = assign58360_e90662_d_n5;
        locals.var_t1_dn6 = assign58360_e90662_d_n6;
        locals.var_t1_dn7 = assign58360_e90662_d_n7;
        locals.var_t1_dn8 = assign58360_e90662_d_n8;
        locals.var_t1_dn9 = assign58360_e90662_d_n9;
        locals.var_t1_dn10 = assign58360_e90662_d_n10;
        locals.var_t1_dn11 = assign58360_e90662_d_n11;
        locals.var_t1_dn14 = assign58360_e90662_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_214(
        locals: &mut StampLocals,
    ) {
        let (assign58370_e90681, assign58370_e90681_d_n0, assign58370_e90681_d_n2, assign58370_e90681_d_n4, assign58370_e90681_d_n5, assign58370_e90681_d_n6, assign58370_e90681_d_n7, assign58370_e90681_d_n8, assign58370_e90681_d_n9, assign58370_e90681_d_n10, assign58370_e90681_d_n11, assign58370_e90681_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58370_e90677: f64 = (locals.var_t1 * locals.var_vgp);
        let assign58370_e90679: f64 = (assign58370_e90677 * locals.var_vgp);
        (assign58370_e90679, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn2)), ((((locals.var_t1_dn4 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn4)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn4)), ((((locals.var_t1_dn5 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn5)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn5)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn7)), ((((locals.var_t1_dn8 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn8)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn8)), ((((locals.var_t1_dn9 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn9)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn9)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn11)), ((((locals.var_t1_dn14 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn14)) * locals.var_vgp) + (assign58370_e90677 * locals.var_vgp_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58370_e90681;
        locals.var_t2_dn0 = assign58370_e90681_d_n0;
        locals.var_t2_dn2 = assign58370_e90681_d_n2;
        locals.var_t2_dn4 = assign58370_e90681_d_n4;
        locals.var_t2_dn5 = assign58370_e90681_d_n5;
        locals.var_t2_dn6 = assign58370_e90681_d_n6;
        locals.var_t2_dn7 = assign58370_e90681_d_n7;
        locals.var_t2_dn8 = assign58370_e90681_d_n8;
        locals.var_t2_dn9 = assign58370_e90681_d_n9;
        locals.var_t2_dn10 = assign58370_e90681_d_n10;
        locals.var_t2_dn11 = assign58370_e90681_d_n11;
        locals.var_t2_dn14 = assign58370_e90681_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58380_e90700, assign58380_e90700_d_n0, assign58380_e90700_d_n2, assign58380_e90700_d_n4, assign58380_e90700_d_n5, assign58380_e90700_d_n6, assign58380_e90700_d_n7, assign58380_e90700_d_n8, assign58380_e90700_d_n9, assign58380_e90700_d_n10, assign58380_e90700_d_n11, assign58380_e90700_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58380_e90697: f64 = (2.0 / locals.var_vgp);
        let assign58380_e90698: f64 = (locals.var_beta + assign58380_e90697);
        (assign58380_e90698, (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn11 + (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn14 + (-((2.0 * locals.var_vgp_dn14) / (locals.var_vgp * locals.var_vgp)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign58380_e90700;
        locals.var_t3_dn0 = assign58380_e90700_d_n0;
        locals.var_t3_dn2 = assign58380_e90700_d_n2;
        locals.var_t3_dn4 = assign58380_e90700_d_n4;
        locals.var_t3_dn5 = assign58380_e90700_d_n5;
        locals.var_t3_dn6 = assign58380_e90700_d_n6;
        locals.var_t3_dn7 = assign58380_e90700_d_n7;
        locals.var_t3_dn8 = assign58380_e90700_d_n8;
        locals.var_t3_dn9 = assign58380_e90700_d_n9;
        locals.var_t3_dn10 = assign58380_e90700_d_n10;
        locals.var_t3_dn11 = assign58380_e90700_d_n11;
        locals.var_t3_dn14 = assign58380_e90700_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign58390_e90718, assign58390_e90718_d_n0, assign58390_e90718_d_n2, assign58390_e90718_d_n4, assign58390_e90718_d_n5, assign58390_e90718_d_n6, assign58390_e90718_d_n7, assign58390_e90718_d_n8, assign58390_e90718_d_n9, assign58390_e90718_d_n10, assign58390_e90718_d_n11, assign58390_e90718_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58390_e90714: f64 = (locals.var_t2).ln();
        let assign58390_e90716: f64 = (assign58390_e90714 / locals.var_t3);
        (assign58390_e90716, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn4 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn5 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn8 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn9 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn14 / locals.var_t2) * locals.var_t3) - (assign58390_e90714 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn9, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn14,)
    }
};
        locals.var_ps0_inib = assign58390_e90718;
        locals.var_ps0_inib_dn0 = assign58390_e90718_d_n0;
        locals.var_ps0_inib_dn2 = assign58390_e90718_d_n2;
        locals.var_ps0_inib_dn4 = assign58390_e90718_d_n4;
        locals.var_ps0_inib_dn5 = assign58390_e90718_d_n5;
        locals.var_ps0_inib_dn6 = assign58390_e90718_d_n6;
        locals.var_ps0_inib_dn7 = assign58390_e90718_d_n7;
        locals.var_ps0_inib_dn8 = assign58390_e90718_d_n8;
        locals.var_ps0_inib_dn9 = assign58390_e90718_d_n9;
        locals.var_ps0_inib_dn10 = assign58390_e90718_d_n10;
        locals.var_ps0_inib_dn11 = assign58390_e90718_d_n11;
        locals.var_ps0_inib_dn14 = assign58390_e90718_d_n14;
        locals.var_ps0_inib_rv = 0.0;

        let (assign58400_e90737, assign58400_e90737_d_n0, assign58400_e90737_d_n2, assign58400_e90737_d_n4, assign58400_e90737_d_n5, assign58400_e90737_d_n6, assign58400_e90737_d_n7, assign58400_e90737_d_n8, assign58400_e90737_d_n9, assign58400_e90737_d_n10, assign58400_e90737_d_n11, assign58400_e90737_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58400_e90733: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign58400_e90735: f64 = (assign58400_e90733 - 0.0008);
        (assign58400_e90735, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn4 - locals.var_ps0_inia_dn4), (locals.var_ps0_inib_dn5 - locals.var_ps0_inia_dn5), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn8 - locals.var_ps0_inia_dn8), (locals.var_ps0_inib_dn9 - locals.var_ps0_inia_dn9), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn14 - locals.var_ps0_inia_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign58400_e90737;
        locals.var_tmf1_dn0 = assign58400_e90737_d_n0;
        locals.var_tmf1_dn2 = assign58400_e90737_d_n2;
        locals.var_tmf1_dn4 = assign58400_e90737_d_n4;
        locals.var_tmf1_dn5 = assign58400_e90737_d_n5;
        locals.var_tmf1_dn6 = assign58400_e90737_d_n6;
        locals.var_tmf1_dn7 = assign58400_e90737_d_n7;
        locals.var_tmf1_dn8 = assign58400_e90737_d_n8;
        locals.var_tmf1_dn9 = assign58400_e90737_d_n9;
        locals.var_tmf1_dn10 = assign58400_e90737_d_n10;
        locals.var_tmf1_dn11 = assign58400_e90737_d_n11;
        locals.var_tmf1_dn14 = assign58400_e90737_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign58410_e90756, assign58410_e90756_d_n0, assign58410_e90756_d_n2, assign58410_e90756_d_n4, assign58410_e90756_d_n5, assign58410_e90756_d_n6, assign58410_e90756_d_n7, assign58410_e90756_d_n8, assign58410_e90756_d_n9, assign58410_e90756_d_n10, assign58410_e90756_d_n11, assign58410_e90756_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58410_e90752: f64 = (4.0 * locals.var_ps0_inib);
        let assign58410_e90754: f64 = (assign58410_e90752 * 0.0008);
        (assign58410_e90754, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn4) * 0.0008), ((4.0 * locals.var_ps0_inib_dn5) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn8) * 0.0008), ((4.0 * locals.var_ps0_inib_dn9) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn14) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58410_e90756;
        locals.var_tmf2_dn0 = assign58410_e90756_d_n0;
        locals.var_tmf2_dn2 = assign58410_e90756_d_n2;
        locals.var_tmf2_dn4 = assign58410_e90756_d_n4;
        locals.var_tmf2_dn5 = assign58410_e90756_d_n5;
        locals.var_tmf2_dn6 = assign58410_e90756_d_n6;
        locals.var_tmf2_dn7 = assign58410_e90756_d_n7;
        locals.var_tmf2_dn8 = assign58410_e90756_d_n8;
        locals.var_tmf2_dn9 = assign58410_e90756_d_n9;
        locals.var_tmf2_dn10 = assign58410_e90756_d_n10;
        locals.var_tmf2_dn11 = assign58410_e90756_d_n11;
        locals.var_tmf2_dn14 = assign58410_e90756_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58420_e90777, assign58420_e90777_d_n0, assign58420_e90777_d_n2, assign58420_e90777_d_n4, assign58420_e90777_d_n5, assign58420_e90777_d_n6, assign58420_e90777_d_n7, assign58420_e90777_d_n8, assign58420_e90777_d_n9, assign58420_e90777_d_n10, assign58420_e90777_d_n11, assign58420_e90777_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let (assign58420_e90775, assign58420_e90775_d_n0, assign58420_e90775_d_n2, assign58420_e90775_d_n4, assign58420_e90775_d_n5, assign58420_e90775_d_n6, assign58420_e90775_d_n7, assign58420_e90775_d_n8, assign58420_e90775_d_n9, assign58420_e90775_d_n10, assign58420_e90775_d_n11, assign58420_e90775_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign58420_e90774: f64 = (-locals.var_tmf2);
                (assign58420_e90774, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign58420_e90775, assign58420_e90775_d_n0, assign58420_e90775_d_n2, assign58420_e90775_d_n4, assign58420_e90775_d_n5, assign58420_e90775_d_n6, assign58420_e90775_d_n7, assign58420_e90775_d_n8, assign58420_e90775_d_n9, assign58420_e90775_d_n10, assign58420_e90775_d_n11, assign58420_e90775_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58420_e90777;
        locals.var_tmf2_dn0 = assign58420_e90777_d_n0;
        locals.var_tmf2_dn2 = assign58420_e90777_d_n2;
        locals.var_tmf2_dn4 = assign58420_e90777_d_n4;
        locals.var_tmf2_dn5 = assign58420_e90777_d_n5;
        locals.var_tmf2_dn6 = assign58420_e90777_d_n6;
        locals.var_tmf2_dn7 = assign58420_e90777_d_n7;
        locals.var_tmf2_dn8 = assign58420_e90777_d_n8;
        locals.var_tmf2_dn9 = assign58420_e90777_d_n9;
        locals.var_tmf2_dn10 = assign58420_e90777_d_n10;
        locals.var_tmf2_dn11 = assign58420_e90777_d_n11;
        locals.var_tmf2_dn14 = assign58420_e90777_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58430_e90797, assign58430_e90797_d_n0, assign58430_e90797_d_n2, assign58430_e90797_d_n4, assign58430_e90797_d_n5, assign58430_e90797_d_n6, assign58430_e90797_d_n7, assign58430_e90797_d_n8, assign58430_e90797_d_n9, assign58430_e90797_d_n10, assign58430_e90797_d_n11, assign58430_e90797_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58430_e90792: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign58430_e90794: f64 = (assign58430_e90792 + locals.var_tmf2);
        let assign58430_e90795: f64 = (assign58430_e90794).sqrt();
        (assign58430_e90795, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign58430_e90795)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign58430_e90795)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58430_e90797;
        locals.var_tmf2_dn0 = assign58430_e90797_d_n0;
        locals.var_tmf2_dn2 = assign58430_e90797_d_n2;
        locals.var_tmf2_dn4 = assign58430_e90797_d_n4;
        locals.var_tmf2_dn5 = assign58430_e90797_d_n5;
        locals.var_tmf2_dn6 = assign58430_e90797_d_n6;
        locals.var_tmf2_dn7 = assign58430_e90797_d_n7;
        locals.var_tmf2_dn8 = assign58430_e90797_d_n8;
        locals.var_tmf2_dn9 = assign58430_e90797_d_n9;
        locals.var_tmf2_dn10 = assign58430_e90797_d_n10;
        locals.var_tmf2_dn11 = assign58430_e90797_d_n11;
        locals.var_tmf2_dn14 = assign58430_e90797_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58440_e90818, assign58440_e90818_d_n0, assign58440_e90818_d_n2, assign58440_e90818_d_n4, assign58440_e90818_d_n5, assign58440_e90818_d_n6, assign58440_e90818_d_n7, assign58440_e90818_d_n8, assign58440_e90818_d_n9, assign58440_e90818_d_n10, assign58440_e90818_d_n11, assign58440_e90818_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58440_e90814: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign58440_e90815: f64 = (1.0 + assign58440_e90814);
        let assign58440_e90816: f64 = (0.5 * assign58440_e90815);
        (assign58440_e90816, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58440_e90818;
        locals.var_t1_dn0 = assign58440_e90818_d_n0;
        locals.var_t1_dn2 = assign58440_e90818_d_n2;
        locals.var_t1_dn4 = assign58440_e90818_d_n4;
        locals.var_t1_dn5 = assign58440_e90818_d_n5;
        locals.var_t1_dn6 = assign58440_e90818_d_n6;
        locals.var_t1_dn7 = assign58440_e90818_d_n7;
        locals.var_t1_dn8 = assign58440_e90818_d_n8;
        locals.var_t1_dn9 = assign58440_e90818_d_n9;
        locals.var_t1_dn10 = assign58440_e90818_d_n10;
        locals.var_t1_dn11 = assign58440_e90818_d_n11;
        locals.var_t1_dn14 = assign58440_e90818_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58450_e90839, assign58450_e90839_d_n0, assign58450_e90839_d_n2, assign58450_e90839_d_n4, assign58450_e90839_d_n5, assign58450_e90839_d_n6, assign58450_e90839_d_n7, assign58450_e90839_d_n8, assign58450_e90839_d_n9, assign58450_e90839_d_n10, assign58450_e90839_d_n11, assign58450_e90839_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1435 != 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58450_e90835: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign58450_e90836: f64 = (0.5 * assign58450_e90835);
        let assign58450_e90837: f64 = (locals.var_ps0_inib - assign58450_e90836);
        (assign58450_e90837, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_ps0_inib_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_ps0_inib_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign58450_e90839;
        locals.var_ps0_ini_dn0 = assign58450_e90839_d_n0;
        locals.var_ps0_ini_dn2 = assign58450_e90839_d_n2;
        locals.var_ps0_ini_dn4 = assign58450_e90839_d_n4;
        locals.var_ps0_ini_dn5 = assign58450_e90839_d_n5;
        locals.var_ps0_ini_dn6 = assign58450_e90839_d_n6;
        locals.var_ps0_ini_dn7 = assign58450_e90839_d_n7;
        locals.var_ps0_ini_dn8 = assign58450_e90839_d_n8;
        locals.var_ps0_ini_dn9 = assign58450_e90839_d_n9;
        locals.var_ps0_ini_dn10 = assign58450_e90839_d_n10;
        locals.var_ps0_ini_dn11 = assign58450_e90839_d_n11;
        locals.var_ps0_ini_dn14 = assign58450_e90839_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58460_e90850, assign58460_e90850_d_n0, assign58460_e90850_d_n2, assign58460_e90850_d_n4, assign58460_e90850_d_n5, assign58460_e90850_d_n6, assign58460_e90850_d_n7, assign58460_e90850_d_n8, assign58460_e90850_d_n9, assign58460_e90850_d_n10, assign58460_e90850_d_n11, assign58460_e90850_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58460_e90847: f64 = (1e-12 / 2.0);
        let assign58460_e90848: f64 = (locals.var_vbscl__blk439 + assign58460_e90847);
        (assign58460_e90848, locals.var_vbscl__blk439_dn0, locals.var_vbscl__blk439_dn2, locals.var_vbscl__blk439_dn4, locals.var_vbscl__blk439_dn5, locals.var_vbscl__blk439_dn6, locals.var_vbscl__blk439_dn7, locals.var_vbscl__blk439_dn8, locals.var_vbscl__blk439_dn9, locals.var_vbscl__blk439_dn10, locals.var_vbscl__blk439_dn11, locals.var_vbscl__blk439_dn14,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58460_e90850;
        locals.var_tx_dn0 = assign58460_e90850_d_n0;
        locals.var_tx_dn2 = assign58460_e90850_d_n2;
        locals.var_tx_dn4 = assign58460_e90850_d_n4;
        locals.var_tx_dn5 = assign58460_e90850_d_n5;
        locals.var_tx_dn6 = assign58460_e90850_d_n6;
        locals.var_tx_dn7 = assign58460_e90850_d_n7;
        locals.var_tx_dn8 = assign58460_e90850_d_n8;
        locals.var_tx_dn9 = assign58460_e90850_d_n9;
        locals.var_tx_dn10 = assign58460_e90850_d_n10;
        locals.var_tx_dn11 = assign58460_e90850_d_n11;
        locals.var_tx_dn14 = assign58460_e90850_d_n14;
        locals.var_tx_rv = 0.0;

        let assign58470_e90853: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard1438 = assign58470_e90853;
        locals.var_guard1438_rv = 0.0;

        let (assign58480_e90862, assign58480_e90862_d_n0, assign58480_e90862_d_n2, assign58480_e90862_d_n4, assign58480_e90862_d_n5, assign58480_e90862_d_n6, assign58480_e90862_d_n7, assign58480_e90862_d_n8, assign58480_e90862_d_n9, assign58480_e90862_d_n10, assign58480_e90862_d_n11, assign58480_e90862_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1438 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign58480_e90862;
        locals.var_ps0_ini_dn0 = assign58480_e90862_d_n0;
        locals.var_ps0_ini_dn2 = assign58480_e90862_d_n2;
        locals.var_ps0_ini_dn4 = assign58480_e90862_d_n4;
        locals.var_ps0_ini_dn5 = assign58480_e90862_d_n5;
        locals.var_ps0_ini_dn6 = assign58480_e90862_d_n6;
        locals.var_ps0_ini_dn7 = assign58480_e90862_d_n7;
        locals.var_ps0_ini_dn8 = assign58480_e90862_d_n8;
        locals.var_ps0_ini_dn9 = assign58480_e90862_d_n9;
        locals.var_ps0_ini_dn10 = assign58480_e90862_d_n10;
        locals.var_ps0_ini_dn11 = assign58480_e90862_d_n11;
        locals.var_ps0_ini_dn14 = assign58480_e90862_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58490_e90869, assign58490_e90869_d_n0, assign58490_e90869_d_n2, assign58490_e90869_d_n4, assign58490_e90869_d_n5, assign58490_e90869_d_n6, assign58490_e90869_d_n7, assign58490_e90869_d_n8, assign58490_e90869_d_n9, assign58490_e90869_d_n10, assign58490_e90869_d_n11, assign58490_e90869_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
        locals.var_ps0 = assign58490_e90869;
        locals.var_ps0_dn0 = assign58490_e90869_d_n0;
        locals.var_ps0_dn2 = assign58490_e90869_d_n2;
        locals.var_ps0_dn4 = assign58490_e90869_d_n4;
        locals.var_ps0_dn5 = assign58490_e90869_d_n5;
        locals.var_ps0_dn6 = assign58490_e90869_d_n6;
        locals.var_ps0_dn7 = assign58490_e90869_d_n7;
        locals.var_ps0_dn8 = assign58490_e90869_d_n8;
        locals.var_ps0_dn9 = assign58490_e90869_d_n9;
        locals.var_ps0_dn10 = assign58490_e90869_d_n10;
        locals.var_ps0_dn11 = assign58490_e90869_d_n11;
        locals.var_ps0_dn14 = assign58490_e90869_d_n14;
        locals.var_ps0_rv = 0.0;

        let (assign58500_e90876, assign58500_e90876_d_n0, assign58500_e90876_d_n2, assign58500_e90876_d_n4, assign58500_e90876_d_n5, assign58500_e90876_d_n6, assign58500_e90876_d_n7, assign58500_e90876_d_n8, assign58500_e90876_d_n9, assign58500_e90876_d_n10, assign58500_e90876_d_n11, assign58500_e90876_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn4, locals.var_psl_lim_dn5, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn8, locals.var_psl_lim_dn9, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn14,)
    }
};
        locals.var_psl_lim = assign58500_e90876;
        locals.var_psl_lim_dn0 = assign58500_e90876_d_n0;
        locals.var_psl_lim_dn2 = assign58500_e90876_d_n2;
        locals.var_psl_lim_dn4 = assign58500_e90876_d_n4;
        locals.var_psl_lim_dn5 = assign58500_e90876_d_n5;
        locals.var_psl_lim_dn6 = assign58500_e90876_d_n6;
        locals.var_psl_lim_dn7 = assign58500_e90876_d_n7;
        locals.var_psl_lim_dn8 = assign58500_e90876_d_n8;
        locals.var_psl_lim_dn9 = assign58500_e90876_d_n9;
        locals.var_psl_lim_dn10 = assign58500_e90876_d_n10;
        locals.var_psl_lim_dn11 = assign58500_e90876_d_n11;
        locals.var_psl_lim_dn14 = assign58500_e90876_d_n14;
        locals.var_psl_lim_rv = 0.0;

        let (assign58510_e90886, assign58510_e90886_d_n0, assign58510_e90886_d_n2, assign58510_e90886_d_n4, assign58510_e90886_d_n5, assign58510_e90886_d_n6, assign58510_e90886_d_n7, assign58510_e90886_d_n8, assign58510_e90886_d_n9, assign58510_e90886_d_n10, assign58510_e90886_d_n11, assign58510_e90886_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58510_e90883: f64 = (locals.var_beta * locals.var_vbscl__blk439);
        let assign58510_e90884: f64 = (assign58510_e90883).exp();
        (assign58510_e90884, (assign58510_e90884 * ((locals.var_beta_dn0 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn0))), (assign58510_e90884 * ((locals.var_beta_dn2 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn2))), (assign58510_e90884 * ((locals.var_beta_dn4 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn4))), (assign58510_e90884 * ((locals.var_beta_dn5 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn5))), (assign58510_e90884 * ((locals.var_beta_dn6 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn6))), (assign58510_e90884 * ((locals.var_beta_dn7 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn7))), (assign58510_e90884 * ((locals.var_beta_dn8 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn8))), (assign58510_e90884 * ((locals.var_beta_dn9 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn9))), (assign58510_e90884 * ((locals.var_beta_dn10 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn10))), (assign58510_e90884 * ((locals.var_beta_dn11 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn11))), (assign58510_e90884 * ((locals.var_beta_dn14 * locals.var_vbscl__blk439) + (locals.var_beta * locals.var_vbscl__blk439_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign58510_e90886;
        locals.var_exp_bvbs_dn0 = assign58510_e90886_d_n0;
        locals.var_exp_bvbs_dn2 = assign58510_e90886_d_n2;
        locals.var_exp_bvbs_dn4 = assign58510_e90886_d_n4;
        locals.var_exp_bvbs_dn5 = assign58510_e90886_d_n5;
        locals.var_exp_bvbs_dn6 = assign58510_e90886_d_n6;
        locals.var_exp_bvbs_dn7 = assign58510_e90886_d_n7;
        locals.var_exp_bvbs_dn8 = assign58510_e90886_d_n8;
        locals.var_exp_bvbs_dn9 = assign58510_e90886_d_n9;
        locals.var_exp_bvbs_dn10 = assign58510_e90886_d_n10;
        locals.var_exp_bvbs_dn11 = assign58510_e90886_d_n11;
        locals.var_exp_bvbs_dn14 = assign58510_e90886_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign58520_e90895, assign58520_e90895_d_n0, assign58520_e90895_d_n2, assign58520_e90895_d_n4, assign58520_e90895_d_n5, assign58520_e90895_d_n6, assign58520_e90895_d_n7, assign58520_e90895_d_n8, assign58520_e90895_d_n9, assign58520_e90895_d_n10, assign58520_e90895_d_n11, assign58520_e90895_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58520_e90893: f64 = (locals.var_cnst1 * locals.var_exp_bvbs);
        (assign58520_e90893, ((locals.var_cnst1_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign58520_e90895;
        locals.var_cfs1_dn0 = assign58520_e90895_d_n0;
        locals.var_cfs1_dn2 = assign58520_e90895_d_n2;
        locals.var_cfs1_dn4 = assign58520_e90895_d_n4;
        locals.var_cfs1_dn5 = assign58520_e90895_d_n5;
        locals.var_cfs1_dn6 = assign58520_e90895_d_n6;
        locals.var_cfs1_dn7 = assign58520_e90895_d_n7;
        locals.var_cfs1_dn8 = assign58520_e90895_d_n8;
        locals.var_cfs1_dn9 = assign58520_e90895_d_n9;
        locals.var_cfs1_dn10 = assign58520_e90895_d_n10;
        locals.var_cfs1_dn11 = assign58520_e90895_d_n11;
        locals.var_cfs1_dn14 = assign58520_e90895_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign58530_e90902,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign58530_e90902;
        locals.var_flg_conv_rv = 0.0;

        let (assign58540_e90909,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign58540_e90909;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_215(
        locals: &mut StampLocals,
    ) {
        let mut assign58550_loop_guard: usize = 0;
        while {
            let assign58550_cond_e90917: f64 = (locals.var_lp_s0_max + 1.0);
            let assign58550_cond_e90919: f64 = if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_lp_s0 <= assign58550_cond_e90917)) { 1.0 } else { 0.0 };
            assign58550_cond_e90919 != 0.0
        } {
            assign58550_loop_guard += 1;
            assert!(assign58550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign58550_body1_e90939, assign58550_body1_e90939_d_n0, assign58550_body1_e90939_d_n2, assign58550_body1_e90939_d_n4, assign58550_body1_e90939_d_n5, assign58550_body1_e90939_d_n6, assign58550_body1_e90939_d_n7, assign58550_body1_e90939_d_n8, assign58550_body1_e90939_d_n9, assign58550_body1_e90939_d_n10, assign58550_body1_e90939_d_n11, assign58550_body1_e90939_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58550_body1_e90936: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign58550_body1_e90937: f64 = (locals.var_beta * assign58550_body1_e90936);
        (assign58550_body1_e90937, ((locals.var_beta_dn0 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0))), ((locals.var_beta_dn2 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2))), ((locals.var_beta_dn4 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4))), ((locals.var_beta_dn5 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5))), ((locals.var_beta_dn6 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6))), ((locals.var_beta_dn7 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7))), ((locals.var_beta_dn8 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8))), ((locals.var_beta_dn9 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9))), ((locals.var_beta_dn10 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10))), ((locals.var_beta_dn11 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11))), ((locals.var_beta_dn14 * assign58550_body1_e90936) + (locals.var_beta * (locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign58550_body1_e90939;
            locals.var_chi_dn0 = assign58550_body1_e90939_d_n0;
            locals.var_chi_dn2 = assign58550_body1_e90939_d_n2;
            locals.var_chi_dn4 = assign58550_body1_e90939_d_n4;
            locals.var_chi_dn5 = assign58550_body1_e90939_d_n5;
            locals.var_chi_dn6 = assign58550_body1_e90939_d_n6;
            locals.var_chi_dn7 = assign58550_body1_e90939_d_n7;
            locals.var_chi_dn8 = assign58550_body1_e90939_d_n8;
            locals.var_chi_dn9 = assign58550_body1_e90939_d_n9;
            locals.var_chi_dn10 = assign58550_body1_e90939_d_n10;
            locals.var_chi_dn11 = assign58550_body1_e90939_d_n11;
            locals.var_chi_dn14 = assign58550_body1_e90939_d_n14;
            locals.var_chi_rv = 0.0;
            let assign58550_body2_e90942: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1439 = assign58550_body2_e90942;
            locals.var_guard1439_rv = 0.0;
            let (assign58550_body3_e90966, assign58550_body3_e90966_d_n0, assign58550_body3_e90966_d_n2, assign58550_body3_e90966_d_n4, assign58550_body3_e90966_d_n5, assign58550_body3_e90966_d_n6, assign58550_body3_e90966_d_n7, assign58550_body3_e90966_d_n8, assign58550_body3_e90966_d_n9, assign58550_body3_e90966_d_n10, assign58550_body3_e90966_d_n11, assign58550_body3_e90966_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let assign58550_body3_e90951: f64 = (locals.var_chi * locals.var_chi);
        let assign58550_body3_e90953: f64 = (assign58550_body3_e90951 * locals.var_chi);
        let assign58550_body3_e90957: f64 = (-0.07053654284009761);
        let assign58550_body3_e90960: f64 = (locals.var_chi * 0.006115288895133179);
        let assign58550_body3_e90961: f64 = (assign58550_body3_e90957 + assign58550_body3_e90960);
        let assign58550_body3_e90962: f64 = (locals.var_chi * assign58550_body3_e90961);
        let assign58550_body3_e90963: f64 = (0.29693154855771 + assign58550_body3_e90962);
        let assign58550_body3_e90964: f64 = (assign58550_body3_e90953 * assign58550_body3_e90963);
        (assign58550_body3_e90964, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn0)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn0 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn2)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn2 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn4)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn4 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn5)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn5 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn6)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn6 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn7)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn7 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn7 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn8)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn8 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn9)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn9 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn9 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn10)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn10 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn11)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn11 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn11 * 0.006115288895133179))))), ((((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) * locals.var_chi) + (assign58550_body3_e90951 * locals.var_chi_dn14)) * assign58550_body3_e90963) + (assign58550_body3_e90953 * ((locals.var_chi_dn14 * assign58550_body3_e90961) + (locals.var_chi * (locals.var_chi_dn14 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn8, locals.var_fi_dn9, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn14,)
    }
};
            locals.var_fi = assign58550_body3_e90966;
            locals.var_fi_dn0 = assign58550_body3_e90966_d_n0;
            locals.var_fi_dn2 = assign58550_body3_e90966_d_n2;
            locals.var_fi_dn4 = assign58550_body3_e90966_d_n4;
            locals.var_fi_dn5 = assign58550_body3_e90966_d_n5;
            locals.var_fi_dn6 = assign58550_body3_e90966_d_n6;
            locals.var_fi_dn7 = assign58550_body3_e90966_d_n7;
            locals.var_fi_dn8 = assign58550_body3_e90966_d_n8;
            locals.var_fi_dn9 = assign58550_body3_e90966_d_n9;
            locals.var_fi_dn10 = assign58550_body3_e90966_d_n10;
            locals.var_fi_dn11 = assign58550_body3_e90966_d_n11;
            locals.var_fi_dn14 = assign58550_body3_e90966_d_n14;
            locals.var_fi_rv = 0.0;
            let (assign58550_body4_e90994, assign58550_body4_e90994_d_n0, assign58550_body4_e90994_d_n2, assign58550_body4_e90994_d_n4, assign58550_body4_e90994_d_n5, assign58550_body4_e90994_d_n6, assign58550_body4_e90994_d_n7, assign58550_body4_e90994_d_n8, assign58550_body4_e90994_d_n9, assign58550_body4_e90994_d_n10, assign58550_body4_e90994_d_n11, assign58550_body4_e90994_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let assign58550_body4_e90975: f64 = (locals.var_chi * locals.var_chi);
        let assign58550_body4_e90978: f64 = (3.0 * 0.29693154855771);
        let assign58550_body4_e90982: f64 = (-0.07053654284009761);
        let assign58550_body4_e90983: f64 = (4.0 * assign58550_body4_e90982);
        let assign58550_body4_e90986: f64 = (locals.var_chi * 5.0);
        let assign58550_body4_e90988: f64 = (assign58550_body4_e90986 * 0.006115288895133179);
        let assign58550_body4_e90989: f64 = (assign58550_body4_e90983 + assign58550_body4_e90988);
        let assign58550_body4_e90990: f64 = (locals.var_chi * assign58550_body4_e90989);
        let assign58550_body4_e90991: f64 = (assign58550_body4_e90978 + assign58550_body4_e90990);
        let assign58550_body4_e90992: f64 = (assign58550_body4_e90975 * assign58550_body4_e90991);
        (assign58550_body4_e90992, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn0 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn2 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn4 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn5 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn6 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn7 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn8 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn9 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn10 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn11 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) * assign58550_body4_e90991) + (assign58550_body4_e90975 * ((locals.var_chi_dn14 * assign58550_body4_e90989) + (locals.var_chi * ((locals.var_chi_dn14 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn9, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn14,)
    }
};
            locals.var_fi_dchi = assign58550_body4_e90994;
            locals.var_fi_dchi_dn0 = assign58550_body4_e90994_d_n0;
            locals.var_fi_dchi_dn2 = assign58550_body4_e90994_d_n2;
            locals.var_fi_dchi_dn4 = assign58550_body4_e90994_d_n4;
            locals.var_fi_dchi_dn5 = assign58550_body4_e90994_d_n5;
            locals.var_fi_dchi_dn6 = assign58550_body4_e90994_d_n6;
            locals.var_fi_dchi_dn7 = assign58550_body4_e90994_d_n7;
            locals.var_fi_dchi_dn8 = assign58550_body4_e90994_d_n8;
            locals.var_fi_dchi_dn9 = assign58550_body4_e90994_d_n9;
            locals.var_fi_dchi_dn10 = assign58550_body4_e90994_d_n10;
            locals.var_fi_dchi_dn11 = assign58550_body4_e90994_d_n11;
            locals.var_fi_dchi_dn14 = assign58550_body4_e90994_d_n14;
            locals.var_fi_dchi_rv = 0.0;
            let (assign58550_body5_e91007, assign58550_body5_e91007_d_n0, assign58550_body5_e91007_d_n2, assign58550_body5_e91007_d_n4, assign58550_body5_e91007_d_n5, assign58550_body5_e91007_d_n6, assign58550_body5_e91007_d_n7, assign58550_body5_e91007_d_n8, assign58550_body5_e91007_d_n9, assign58550_body5_e91007_d_n10, assign58550_body5_e91007_d_n11, assign58550_body5_e91007_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let assign58550_body5_e91003: f64 = (locals.var_cfs1 * locals.var_fi);
        let assign58550_body5_e91005: f64 = (assign58550_body5_e91003 * locals.var_fi);
        (assign58550_body5_e91005, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn7)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn9)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn11)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn11)), ((((locals.var_cfs1_dn14 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn14)) * locals.var_fi) + (assign58550_body5_e91003 * locals.var_fi_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign58550_body5_e91007;
            locals.var_fs01_dn0 = assign58550_body5_e91007_d_n0;
            locals.var_fs01_dn2 = assign58550_body5_e91007_d_n2;
            locals.var_fs01_dn4 = assign58550_body5_e91007_d_n4;
            locals.var_fs01_dn5 = assign58550_body5_e91007_d_n5;
            locals.var_fs01_dn6 = assign58550_body5_e91007_d_n6;
            locals.var_fs01_dn7 = assign58550_body5_e91007_d_n7;
            locals.var_fs01_dn8 = assign58550_body5_e91007_d_n8;
            locals.var_fs01_dn9 = assign58550_body5_e91007_d_n9;
            locals.var_fs01_dn10 = assign58550_body5_e91007_d_n10;
            locals.var_fs01_dn11 = assign58550_body5_e91007_d_n11;
            locals.var_fs01_dn14 = assign58550_body5_e91007_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign58550_body6_e91024, assign58550_body6_e91024_d_n0, assign58550_body6_e91024_d_n2, assign58550_body6_e91024_d_n4, assign58550_body6_e91024_d_n5, assign58550_body6_e91024_d_n6, assign58550_body6_e91024_d_n7, assign58550_body6_e91024_d_n8, assign58550_body6_e91024_d_n9, assign58550_body6_e91024_d_n10, assign58550_body6_e91024_d_n11, assign58550_body6_e91024_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let assign58550_body6_e91016: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign58550_body6_e91018: f64 = (assign58550_body6_e91016 * 2.0);
        let assign58550_body6_e91020: f64 = (assign58550_body6_e91018 * locals.var_fi);
        let assign58550_body6_e91022: f64 = (assign58550_body6_e91020 * locals.var_fi_dchi);
        (assign58550_body6_e91022, (((((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn0)), (((((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn4)), (((((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn5)), (((((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn6)), (((((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn8)), (((((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn9)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn9)), (((((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn10)), (((((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn11)), (((((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * 2.0) * locals.var_fi) + (assign58550_body6_e91018 * locals.var_fi_dn14)) * locals.var_fi_dchi) + (assign58550_body6_e91020 * locals.var_fi_dchi_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign58550_body6_e91024;
            locals.var_fs01_dps0_dn0 = assign58550_body6_e91024_d_n0;
            locals.var_fs01_dps0_dn2 = assign58550_body6_e91024_d_n2;
            locals.var_fs01_dps0_dn4 = assign58550_body6_e91024_d_n4;
            locals.var_fs01_dps0_dn5 = assign58550_body6_e91024_d_n5;
            locals.var_fs01_dps0_dn6 = assign58550_body6_e91024_d_n6;
            locals.var_fs01_dps0_dn7 = assign58550_body6_e91024_d_n7;
            locals.var_fs01_dps0_dn8 = assign58550_body6_e91024_d_n8;
            locals.var_fs01_dps0_dn9 = assign58550_body6_e91024_d_n9;
            locals.var_fs01_dps0_dn10 = assign58550_body6_e91024_d_n10;
            locals.var_fs01_dps0_dn11 = assign58550_body6_e91024_d_n11;
            locals.var_fs01_dps0_dn14 = assign58550_body6_e91024_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58550_body7_e91053, assign58550_body7_e91053_d_n0, assign58550_body7_e91053_d_n2, assign58550_body7_e91053_d_n4, assign58550_body7_e91053_d_n5, assign58550_body7_e91053_d_n6, assign58550_body7_e91053_d_n7, assign58550_body7_e91053_d_n8, assign58550_body7_e91053_d_n9, assign58550_body7_e91053_d_n10, assign58550_body7_e91053_d_n11, assign58550_body7_e91053_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let assign58550_body7_e91035: f64 = (-0.117851130197758);
        let assign58550_body7_e91040: f64 = (-0.00163730162779191);
        let assign58550_body7_e91043: f64 = (locals.var_chi * 6.36964918866352e-5);
        let assign58550_body7_e91044: f64 = (assign58550_body7_e91040 + assign58550_body7_e91043);
        let assign58550_body7_e91045: f64 = (locals.var_chi * assign58550_body7_e91044);
        let assign58550_body7_e91046: f64 = (0.0178800506338833 + assign58550_body7_e91045);
        let assign58550_body7_e91047: f64 = (locals.var_chi * assign58550_body7_e91046);
        let assign58550_body7_e91048: f64 = (assign58550_body7_e91035 + assign58550_body7_e91047);
        let assign58550_body7_e91049: f64 = (locals.var_chi * assign58550_body7_e91048);
        let assign58550_body7_e91050: f64 = (0.707106781186548 + assign58550_body7_e91049);
        let assign58550_body7_e91051: f64 = (locals.var_chi * assign58550_body7_e91050);
        (assign58550_body7_e91051, ((locals.var_chi_dn0 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn0 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn0 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn0 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn2 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn2 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn2 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn4 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn4 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn4 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn5 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn5 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn5 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn6 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn6 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn6 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn7 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn7 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn7 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn7 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn8 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn8 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn8 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn9 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn9 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn9 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn9 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn9 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn10 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn10 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn10 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn11 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn11 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn11 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn11 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn14 * assign58550_body7_e91050) + (locals.var_chi * ((locals.var_chi_dn14 * assign58550_body7_e91048) + (locals.var_chi * ((locals.var_chi_dn14 * assign58550_body7_e91046) + (locals.var_chi * ((locals.var_chi_dn14 * assign58550_body7_e91044) + (locals.var_chi * (locals.var_chi_dn14 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign58550_body7_e91053;
            locals.var_fb_dn0 = assign58550_body7_e91053_d_n0;
            locals.var_fb_dn2 = assign58550_body7_e91053_d_n2;
            locals.var_fb_dn4 = assign58550_body7_e91053_d_n4;
            locals.var_fb_dn5 = assign58550_body7_e91053_d_n5;
            locals.var_fb_dn6 = assign58550_body7_e91053_d_n6;
            locals.var_fb_dn7 = assign58550_body7_e91053_d_n7;
            locals.var_fb_dn8 = assign58550_body7_e91053_d_n8;
            locals.var_fb_dn9 = assign58550_body7_e91053_d_n9;
            locals.var_fb_dn10 = assign58550_body7_e91053_d_n10;
            locals.var_fb_dn11 = assign58550_body7_e91053_d_n11;
            locals.var_fb_dn14 = assign58550_body7_e91053_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign58550_body8_e91088, assign58550_body8_e91088_d_n0, assign58550_body8_e91088_d_n2, assign58550_body8_e91088_d_n4, assign58550_body8_e91088_d_n5, assign58550_body8_e91088_d_n6, assign58550_body8_e91088_d_n7, assign58550_body8_e91088_d_n8, assign58550_body8_e91088_d_n9, assign58550_body8_e91088_d_n10, assign58550_body8_e91088_d_n11, assign58550_body8_e91088_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let assign58550_body8_e91064: f64 = (-0.117851130197758);
        let assign58550_body8_e91065: f64 = (2.0 * assign58550_body8_e91064);
        let assign58550_body8_e91069: f64 = (3.0 * 0.0178800506338833);
        let assign58550_body8_e91073: f64 = (-0.00163730162779191);
        let assign58550_body8_e91074: f64 = (4.0 * assign58550_body8_e91073);
        let assign58550_body8_e91077: f64 = (locals.var_chi * 5.0);
        let assign58550_body8_e91079: f64 = (assign58550_body8_e91077 * 6.36964918866352e-5);
        let assign58550_body8_e91080: f64 = (assign58550_body8_e91074 + assign58550_body8_e91079);
        let assign58550_body8_e91081: f64 = (locals.var_chi * assign58550_body8_e91080);
        let assign58550_body8_e91082: f64 = (assign58550_body8_e91069 + assign58550_body8_e91081);
        let assign58550_body8_e91083: f64 = (locals.var_chi * assign58550_body8_e91082);
        let assign58550_body8_e91084: f64 = (assign58550_body8_e91065 + assign58550_body8_e91083);
        let assign58550_body8_e91085: f64 = (locals.var_chi * assign58550_body8_e91084);
        let assign58550_body8_e91086: f64 = (0.707106781186548 + assign58550_body8_e91085);
        (assign58550_body8_e91086, ((locals.var_chi_dn0 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn0 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn0 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn2 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn2 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn4 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn4 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn5 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn5 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn6 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn6 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn7 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn7 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn7 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn8 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn8 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn9 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn9 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn9 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn10 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn10 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn11 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn11 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn11 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn14 * assign58550_body8_e91084) + (locals.var_chi * ((locals.var_chi_dn14 * assign58550_body8_e91082) + (locals.var_chi * ((locals.var_chi_dn14 * assign58550_body8_e91080) + (locals.var_chi * ((locals.var_chi_dn14 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn9, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn14,)
    }
};
            locals.var_fb_dchi = assign58550_body8_e91088;
            locals.var_fb_dchi_dn0 = assign58550_body8_e91088_d_n0;
            locals.var_fb_dchi_dn2 = assign58550_body8_e91088_d_n2;
            locals.var_fb_dchi_dn4 = assign58550_body8_e91088_d_n4;
            locals.var_fb_dchi_dn5 = assign58550_body8_e91088_d_n5;
            locals.var_fb_dchi_dn6 = assign58550_body8_e91088_d_n6;
            locals.var_fb_dchi_dn7 = assign58550_body8_e91088_d_n7;
            locals.var_fb_dchi_dn8 = assign58550_body8_e91088_d_n8;
            locals.var_fb_dchi_dn9 = assign58550_body8_e91088_d_n9;
            locals.var_fb_dchi_dn10 = assign58550_body8_e91088_d_n10;
            locals.var_fb_dchi_dn11 = assign58550_body8_e91088_d_n11;
            locals.var_fb_dchi_dn14 = assign58550_body8_e91088_d_n14;
            locals.var_fb_dchi_rv = 0.0;
            let (assign58550_body9_e91102, assign58550_body9_e91102_d_n0, assign58550_body9_e91102_d_n2, assign58550_body9_e91102_d_n4, assign58550_body9_e91102_d_n5, assign58550_body9_e91102_d_n6, assign58550_body9_e91102_d_n7, assign58550_body9_e91102_d_n8, assign58550_body9_e91102_d_n9, assign58550_body9_e91102_d_n10, assign58550_body9_e91102_d_n11, assign58550_body9_e91102_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let assign58550_body9_e91097: f64 = (locals.var_fb * locals.var_fb);
        let assign58550_body9_e91099: f64 = (assign58550_body9_e91097 + locals.var_fs01);
        let assign58550_body9_e91100: f64 = (assign58550_body9_e91099).sqrt();
        (assign58550_body9_e91100, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign58550_body9_e91100)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign58550_body9_e91100)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign58550_body9_e91102;
            locals.var_fs02_dn0 = assign58550_body9_e91102_d_n0;
            locals.var_fs02_dn2 = assign58550_body9_e91102_d_n2;
            locals.var_fs02_dn4 = assign58550_body9_e91102_d_n4;
            locals.var_fs02_dn5 = assign58550_body9_e91102_d_n5;
            locals.var_fs02_dn6 = assign58550_body9_e91102_d_n6;
            locals.var_fs02_dn7 = assign58550_body9_e91102_d_n7;
            locals.var_fs02_dn8 = assign58550_body9_e91102_d_n8;
            locals.var_fs02_dn9 = assign58550_body9_e91102_d_n9;
            locals.var_fs02_dn10 = assign58550_body9_e91102_d_n10;
            locals.var_fs02_dn11 = assign58550_body9_e91102_d_n11;
            locals.var_fs02_dn14 = assign58550_body9_e91102_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign58550_body10_e91123, assign58550_body10_e91123_d_n0, assign58550_body10_e91123_d_n2, assign58550_body10_e91123_d_n4, assign58550_body10_e91123_d_n5, assign58550_body10_e91123_d_n6, assign58550_body10_e91123_d_n7, assign58550_body10_e91123_d_n8, assign58550_body10_e91123_d_n9, assign58550_body10_e91123_d_n10, assign58550_body10_e91123_d_n11, assign58550_body10_e91123_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let assign58550_body10_e91111: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign58550_body10_e91113: f64 = (assign58550_body10_e91111 * 2.0);
        let assign58550_body10_e91115: f64 = (assign58550_body10_e91113 * locals.var_fb);
        let assign58550_body10_e91117: f64 = (assign58550_body10_e91115 + locals.var_fs01_dps0);
        let assign58550_body10_e91120: f64 = (locals.var_fs02 + locals.var_fs02);
        let assign58550_body10_e91121: f64 = (assign58550_body10_e91117 / assign58550_body10_e91120);
        (assign58550_body10_e91121, (((((((((locals.var_beta_dn0 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn0)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn2 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn2)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn5 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn5)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn6 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn6)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn7 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn7)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn7 + locals.var_fs02_dn7))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn8 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn8)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn9 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn9)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn9 + locals.var_fs02_dn9))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn11 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn11)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn11 + locals.var_fs02_dn11))) / (assign58550_body10_e91120 * assign58550_body10_e91120)), (((((((((locals.var_beta_dn14 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn14)) * 2.0) * locals.var_fb) + (assign58550_body10_e91113 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14) * assign58550_body10_e91120) - (assign58550_body10_e91117 * (locals.var_fs02_dn14 + locals.var_fs02_dn14))) / (assign58550_body10_e91120 * assign58550_body10_e91120)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign58550_body10_e91123;
            locals.var_fs02_dps0_dn0 = assign58550_body10_e91123_d_n0;
            locals.var_fs02_dps0_dn2 = assign58550_body10_e91123_d_n2;
            locals.var_fs02_dps0_dn4 = assign58550_body10_e91123_d_n4;
            locals.var_fs02_dps0_dn5 = assign58550_body10_e91123_d_n5;
            locals.var_fs02_dps0_dn6 = assign58550_body10_e91123_d_n6;
            locals.var_fs02_dps0_dn7 = assign58550_body10_e91123_d_n7;
            locals.var_fs02_dps0_dn8 = assign58550_body10_e91123_d_n8;
            locals.var_fs02_dps0_dn9 = assign58550_body10_e91123_d_n9;
            locals.var_fs02_dps0_dn10 = assign58550_body10_e91123_d_n10;
            locals.var_fs02_dps0_dn11 = assign58550_body10_e91123_d_n11;
            locals.var_fs02_dps0_dn14 = assign58550_body10_e91123_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign58550_body11_e91126: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1440 = assign58550_body11_e91126;
            locals.var_guard1440_rv = 0.0;
            let (assign58550_body12_e91139, assign58550_body12_e91139_d_n0, assign58550_body12_e91139_d_n2, assign58550_body12_e91139_d_n4, assign58550_body12_e91139_d_n5, assign58550_body12_e91139_d_n6, assign58550_body12_e91139_d_n7, assign58550_body12_e91139_d_n8, assign58550_body12_e91139_d_n9, assign58550_body12_e91139_d_n10, assign58550_body12_e91139_d_n11, assign58550_body12_e91139_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 == 0.0)) && (locals.var_guard1440 != 0.0)) {
        let assign58550_body12_e91137: f64 = (locals.var_chi).exp();
        (assign58550_body12_e91137, (assign58550_body12_e91137 * locals.var_chi_dn0), (assign58550_body12_e91137 * locals.var_chi_dn2), (assign58550_body12_e91137 * locals.var_chi_dn4), (assign58550_body12_e91137 * locals.var_chi_dn5), (assign58550_body12_e91137 * locals.var_chi_dn6), (assign58550_body12_e91137 * locals.var_chi_dn7), (assign58550_body12_e91137 * locals.var_chi_dn8), (assign58550_body12_e91137 * locals.var_chi_dn9), (assign58550_body12_e91137 * locals.var_chi_dn10), (assign58550_body12_e91137 * locals.var_chi_dn11), (assign58550_body12_e91137 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign58550_body12_e91139;
            locals.var_exp_chi_dn0 = assign58550_body12_e91139_d_n0;
            locals.var_exp_chi_dn2 = assign58550_body12_e91139_d_n2;
            locals.var_exp_chi_dn4 = assign58550_body12_e91139_d_n4;
            locals.var_exp_chi_dn5 = assign58550_body12_e91139_d_n5;
            locals.var_exp_chi_dn6 = assign58550_body12_e91139_d_n6;
            locals.var_exp_chi_dn7 = assign58550_body12_e91139_d_n7;
            locals.var_exp_chi_dn8 = assign58550_body12_e91139_d_n8;
            locals.var_exp_chi_dn9 = assign58550_body12_e91139_d_n9;
            locals.var_exp_chi_dn10 = assign58550_body12_e91139_d_n10;
            locals.var_exp_chi_dn11 = assign58550_body12_e91139_d_n11;
            locals.var_exp_chi_dn14 = assign58550_body12_e91139_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign58550_body13_e91155, assign58550_body13_e91155_d_n0, assign58550_body13_e91155_d_n2, assign58550_body13_e91155_d_n4, assign58550_body13_e91155_d_n5, assign58550_body13_e91155_d_n6, assign58550_body13_e91155_d_n7, assign58550_body13_e91155_d_n8, assign58550_body13_e91155_d_n9, assign58550_body13_e91155_d_n10, assign58550_body13_e91155_d_n11, assign58550_body13_e91155_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 == 0.0)) && (locals.var_guard1440 != 0.0)) {
        let assign58550_body13_e91152: f64 = (locals.var_exp_chi - 1.0);
        let assign58550_body13_e91153: f64 = (locals.var_cfs1 * assign58550_body13_e91152);
        (assign58550_body13_e91153, ((locals.var_cfs1_dn0 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn0)), ((locals.var_cfs1_dn2 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn2)), ((locals.var_cfs1_dn4 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn4)), ((locals.var_cfs1_dn5 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn5)), ((locals.var_cfs1_dn6 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn6)), ((locals.var_cfs1_dn7 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn7)), ((locals.var_cfs1_dn8 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn8)), ((locals.var_cfs1_dn9 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn9)), ((locals.var_cfs1_dn10 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn10)), ((locals.var_cfs1_dn11 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn11)), ((locals.var_cfs1_dn14 * assign58550_body13_e91152) + (locals.var_cfs1 * locals.var_exp_chi_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign58550_body13_e91155;
            locals.var_fs01_dn0 = assign58550_body13_e91155_d_n0;
            locals.var_fs01_dn2 = assign58550_body13_e91155_d_n2;
            locals.var_fs01_dn4 = assign58550_body13_e91155_d_n4;
            locals.var_fs01_dn5 = assign58550_body13_e91155_d_n5;
            locals.var_fs01_dn6 = assign58550_body13_e91155_d_n6;
            locals.var_fs01_dn7 = assign58550_body13_e91155_d_n7;
            locals.var_fs01_dn8 = assign58550_body13_e91155_d_n8;
            locals.var_fs01_dn9 = assign58550_body13_e91155_d_n9;
            locals.var_fs01_dn10 = assign58550_body13_e91155_d_n10;
            locals.var_fs01_dn11 = assign58550_body13_e91155_d_n11;
            locals.var_fs01_dn14 = assign58550_body13_e91155_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign58550_body14_e91171, assign58550_body14_e91171_d_n0, assign58550_body14_e91171_d_n2, assign58550_body14_e91171_d_n4, assign58550_body14_e91171_d_n5, assign58550_body14_e91171_d_n6, assign58550_body14_e91171_d_n7, assign58550_body14_e91171_d_n8, assign58550_body14_e91171_d_n9, assign58550_body14_e91171_d_n10, assign58550_body14_e91171_d_n11, assign58550_body14_e91171_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 == 0.0)) && (locals.var_guard1440 != 0.0)) {
        let assign58550_body14_e91167: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign58550_body14_e91169: f64 = (assign58550_body14_e91167 * locals.var_exp_chi);
        (assign58550_body14_e91169, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_exp_chi) + (assign58550_body14_e91167 * locals.var_exp_chi_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign58550_body14_e91171;
            locals.var_fs01_dps0_dn0 = assign58550_body14_e91171_d_n0;
            locals.var_fs01_dps0_dn2 = assign58550_body14_e91171_d_n2;
            locals.var_fs01_dps0_dn4 = assign58550_body14_e91171_d_n4;
            locals.var_fs01_dps0_dn5 = assign58550_body14_e91171_d_n5;
            locals.var_fs01_dps0_dn6 = assign58550_body14_e91171_d_n6;
            locals.var_fs01_dps0_dn7 = assign58550_body14_e91171_d_n7;
            locals.var_fs01_dps0_dn8 = assign58550_body14_e91171_d_n8;
            locals.var_fs01_dps0_dn9 = assign58550_body14_e91171_d_n9;
            locals.var_fs01_dps0_dn10 = assign58550_body14_e91171_d_n10;
            locals.var_fs01_dps0_dn11 = assign58550_body14_e91171_d_n11;
            locals.var_fs01_dps0_dn14 = assign58550_body14_e91171_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58550_body15_e91187, assign58550_body15_e91187_d_n0, assign58550_body15_e91187_d_n2, assign58550_body15_e91187_d_n4, assign58550_body15_e91187_d_n5, assign58550_body15_e91187_d_n6, assign58550_body15_e91187_d_n7, assign58550_body15_e91187_d_n8, assign58550_body15_e91187_d_n9, assign58550_body15_e91187_d_n10, assign58550_body15_e91187_d_n11, assign58550_body15_e91187_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 == 0.0)) && (locals.var_guard1440 == 0.0)) {
        let assign58550_body15_e91184: f64 = (locals.var_beta * locals.var_ps0);
        let assign58550_body15_e91185: f64 = (assign58550_body15_e91184).exp();
        (assign58550_body15_e91185, (assign58550_body15_e91185 * ((locals.var_beta_dn0 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn0))), (assign58550_body15_e91185 * ((locals.var_beta_dn2 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn2))), (assign58550_body15_e91185 * ((locals.var_beta_dn4 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn4))), (assign58550_body15_e91185 * ((locals.var_beta_dn5 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn5))), (assign58550_body15_e91185 * ((locals.var_beta_dn6 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn6))), (assign58550_body15_e91185 * ((locals.var_beta_dn7 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn7))), (assign58550_body15_e91185 * ((locals.var_beta_dn8 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn8))), (assign58550_body15_e91185 * ((locals.var_beta_dn9 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn9))), (assign58550_body15_e91185 * ((locals.var_beta_dn10 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn10))), (assign58550_body15_e91185 * ((locals.var_beta_dn11 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn11))), (assign58550_body15_e91185 * ((locals.var_beta_dn14 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign58550_body15_e91187;
            locals.var_exp_bps0_dn0 = assign58550_body15_e91187_d_n0;
            locals.var_exp_bps0_dn2 = assign58550_body15_e91187_d_n2;
            locals.var_exp_bps0_dn4 = assign58550_body15_e91187_d_n4;
            locals.var_exp_bps0_dn5 = assign58550_body15_e91187_d_n5;
            locals.var_exp_bps0_dn6 = assign58550_body15_e91187_d_n6;
            locals.var_exp_bps0_dn7 = assign58550_body15_e91187_d_n7;
            locals.var_exp_bps0_dn8 = assign58550_body15_e91187_d_n8;
            locals.var_exp_bps0_dn9 = assign58550_body15_e91187_d_n9;
            locals.var_exp_bps0_dn10 = assign58550_body15_e91187_d_n10;
            locals.var_exp_bps0_dn11 = assign58550_body15_e91187_d_n11;
            locals.var_exp_bps0_dn14 = assign58550_body15_e91187_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign58550_body16_e91204, assign58550_body16_e91204_d_n0, assign58550_body16_e91204_d_n2, assign58550_body16_e91204_d_n4, assign58550_body16_e91204_d_n5, assign58550_body16_e91204_d_n6, assign58550_body16_e91204_d_n7, assign58550_body16_e91204_d_n8, assign58550_body16_e91204_d_n9, assign58550_body16_e91204_d_n10, assign58550_body16_e91204_d_n11, assign58550_body16_e91204_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 == 0.0)) && (locals.var_guard1440 == 0.0)) {
        let assign58550_body16_e91201: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign58550_body16_e91202: f64 = (locals.var_cnst1 * assign58550_body16_e91201);
        (assign58550_body16_e91202, ((locals.var_cnst1_dn0 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((locals.var_cnst1_dn2 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((locals.var_cnst1_dn4 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((locals.var_cnst1_dn5 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((locals.var_cnst1_dn6 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((locals.var_cnst1_dn7 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((locals.var_cnst1_dn8 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((locals.var_cnst1_dn9 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((locals.var_cnst1_dn10 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((locals.var_cnst1_dn11 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((locals.var_cnst1_dn14 * assign58550_body16_e91201) + (locals.var_cnst1 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign58550_body16_e91204;
            locals.var_fs01_dn0 = assign58550_body16_e91204_d_n0;
            locals.var_fs01_dn2 = assign58550_body16_e91204_d_n2;
            locals.var_fs01_dn4 = assign58550_body16_e91204_d_n4;
            locals.var_fs01_dn5 = assign58550_body16_e91204_d_n5;
            locals.var_fs01_dn6 = assign58550_body16_e91204_d_n6;
            locals.var_fs01_dn7 = assign58550_body16_e91204_d_n7;
            locals.var_fs01_dn8 = assign58550_body16_e91204_d_n8;
            locals.var_fs01_dn9 = assign58550_body16_e91204_d_n9;
            locals.var_fs01_dn10 = assign58550_body16_e91204_d_n10;
            locals.var_fs01_dn11 = assign58550_body16_e91204_d_n11;
            locals.var_fs01_dn14 = assign58550_body16_e91204_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign58550_body17_e91221, assign58550_body17_e91221_d_n0, assign58550_body17_e91221_d_n2, assign58550_body17_e91221_d_n4, assign58550_body17_e91221_d_n5, assign58550_body17_e91221_d_n6, assign58550_body17_e91221_d_n7, assign58550_body17_e91221_d_n8, assign58550_body17_e91221_d_n9, assign58550_body17_e91221_d_n10, assign58550_body17_e91221_d_n11, assign58550_body17_e91221_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 == 0.0)) && (locals.var_guard1440 == 0.0)) {
        let assign58550_body17_e91217: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign58550_body17_e91219: f64 = (assign58550_body17_e91217 * locals.var_exp_bps0);
        (assign58550_body17_e91219, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn0)), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn2)), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn4)), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn5)), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn6)), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn7)), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn8)), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn9)), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn10)), ((((locals.var_cnst1_dn11 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn11)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn11)), ((((locals.var_cnst1_dn14 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn14)) * locals.var_exp_bps0) + (assign58550_body17_e91217 * locals.var_exp_bps0_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign58550_body17_e91221;
            locals.var_fs01_dps0_dn0 = assign58550_body17_e91221_d_n0;
            locals.var_fs01_dps0_dn2 = assign58550_body17_e91221_d_n2;
            locals.var_fs01_dps0_dn4 = assign58550_body17_e91221_d_n4;
            locals.var_fs01_dps0_dn5 = assign58550_body17_e91221_d_n5;
            locals.var_fs01_dps0_dn6 = assign58550_body17_e91221_d_n6;
            locals.var_fs01_dps0_dn7 = assign58550_body17_e91221_d_n7;
            locals.var_fs01_dps0_dn8 = assign58550_body17_e91221_d_n8;
            locals.var_fs01_dps0_dn9 = assign58550_body17_e91221_d_n9;
            locals.var_fs01_dps0_dn10 = assign58550_body17_e91221_d_n10;
            locals.var_fs01_dps0_dn11 = assign58550_body17_e91221_d_n11;
            locals.var_fs01_dps0_dn14 = assign58550_body17_e91221_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58550_body18_e91236, assign58550_body18_e91236_d_n0, assign58550_body18_e91236_d_n2, assign58550_body18_e91236_d_n4, assign58550_body18_e91236_d_n5, assign58550_body18_e91236_d_n6, assign58550_body18_e91236_d_n7, assign58550_body18_e91236_d_n8, assign58550_body18_e91236_d_n9, assign58550_body18_e91236_d_n10, assign58550_body18_e91236_d_n11, assign58550_body18_e91236_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 == 0.0)) {
        let assign58550_body18_e91231: f64 = (locals.var_chi - 1.0);
        let assign58550_body18_e91233: f64 = (assign58550_body18_e91231 + locals.var_fs01);
        let assign58550_body18_e91234: f64 = (assign58550_body18_e91233).sqrt();
        (assign58550_body18_e91234, ((locals.var_chi_dn0 + locals.var_fs01_dn0) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn2 + locals.var_fs01_dn2) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn4 + locals.var_fs01_dn4) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn5 + locals.var_fs01_dn5) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn6 + locals.var_fs01_dn6) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn7 + locals.var_fs01_dn7) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn8 + locals.var_fs01_dn8) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn9 + locals.var_fs01_dn9) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn10 + locals.var_fs01_dn10) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn11 + locals.var_fs01_dn11) / (2.0 * assign58550_body18_e91234)), ((locals.var_chi_dn14 + locals.var_fs01_dn14) / (2.0 * assign58550_body18_e91234)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign58550_body18_e91236;
            locals.var_fs02_dn0 = assign58550_body18_e91236_d_n0;
            locals.var_fs02_dn2 = assign58550_body18_e91236_d_n2;
            locals.var_fs02_dn4 = assign58550_body18_e91236_d_n4;
            locals.var_fs02_dn5 = assign58550_body18_e91236_d_n5;
            locals.var_fs02_dn6 = assign58550_body18_e91236_d_n6;
            locals.var_fs02_dn7 = assign58550_body18_e91236_d_n7;
            locals.var_fs02_dn8 = assign58550_body18_e91236_d_n8;
            locals.var_fs02_dn9 = assign58550_body18_e91236_d_n9;
            locals.var_fs02_dn10 = assign58550_body18_e91236_d_n10;
            locals.var_fs02_dn11 = assign58550_body18_e91236_d_n11;
            locals.var_fs02_dn14 = assign58550_body18_e91236_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign58550_body19_e91252, assign58550_body19_e91252_d_n0, assign58550_body19_e91252_d_n2, assign58550_body19_e91252_d_n4, assign58550_body19_e91252_d_n5, assign58550_body19_e91252_d_n6, assign58550_body19_e91252_d_n7, assign58550_body19_e91252_d_n8, assign58550_body19_e91252_d_n9, assign58550_body19_e91252_d_n10, assign58550_body19_e91252_d_n11, assign58550_body19_e91252_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1439 == 0.0)) {
        let assign58550_body19_e91246: f64 = (locals.var_beta + locals.var_fs01_dps0);
        let assign58550_body19_e91249: f64 = (locals.var_fs02 + locals.var_fs02);
        let assign58550_body19_e91250: f64 = (assign58550_body19_e91246 / assign58550_body19_e91249);
        (assign58550_body19_e91250, ((((locals.var_beta_dn0 + locals.var_fs01_dps0_dn0) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn2 + locals.var_fs01_dps0_dn2) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn4 + locals.var_fs01_dps0_dn4) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn5 + locals.var_fs01_dps0_dn5) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn6 + locals.var_fs01_dps0_dn6) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn7 + locals.var_fs01_dps0_dn7) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn7 + locals.var_fs02_dn7))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn8 + locals.var_fs01_dps0_dn8) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn9 + locals.var_fs01_dps0_dn9) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn9 + locals.var_fs02_dn9))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn10 + locals.var_fs01_dps0_dn10) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn11 + locals.var_fs01_dps0_dn11) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn11 + locals.var_fs02_dn11))) / (assign58550_body19_e91249 * assign58550_body19_e91249)), ((((locals.var_beta_dn14 + locals.var_fs01_dps0_dn14) * assign58550_body19_e91249) - (assign58550_body19_e91246 * (locals.var_fs02_dn14 + locals.var_fs02_dn14))) / (assign58550_body19_e91249 * assign58550_body19_e91249)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign58550_body19_e91252;
            locals.var_fs02_dps0_dn0 = assign58550_body19_e91252_d_n0;
            locals.var_fs02_dps0_dn2 = assign58550_body19_e91252_d_n2;
            locals.var_fs02_dps0_dn4 = assign58550_body19_e91252_d_n4;
            locals.var_fs02_dps0_dn5 = assign58550_body19_e91252_d_n5;
            locals.var_fs02_dps0_dn6 = assign58550_body19_e91252_d_n6;
            locals.var_fs02_dps0_dn7 = assign58550_body19_e91252_d_n7;
            locals.var_fs02_dps0_dn8 = assign58550_body19_e91252_d_n8;
            locals.var_fs02_dps0_dn9 = assign58550_body19_e91252_d_n9;
            locals.var_fs02_dps0_dn10 = assign58550_body19_e91252_d_n10;
            locals.var_fs02_dps0_dn11 = assign58550_body19_e91252_d_n11;
            locals.var_fs02_dps0_dn14 = assign58550_body19_e91252_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign58550_body20_e91265, assign58550_body20_e91265_d_n0, assign58550_body20_e91265_d_n2, assign58550_body20_e91265_d_n4, assign58550_body20_e91265_d_n5, assign58550_body20_e91265_d_n6, assign58550_body20_e91265_d_n7, assign58550_body20_e91265_d_n8, assign58550_body20_e91265_d_n9, assign58550_body20_e91265_d_n10, assign58550_body20_e91265_d_n11, assign58550_body20_e91265_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58550_body20_e91259: f64 = (locals.var_vgp - locals.var_ps0);
        let assign58550_body20_e91262: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign58550_body20_e91263: f64 = (assign58550_body20_e91259 - assign58550_body20_e91262);
        (assign58550_body20_e91263, ((locals.var_vgp_dn0 - locals.var_ps0_dn0) - ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), ((locals.var_vgp_dn2 - locals.var_ps0_dn2) - ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), ((locals.var_vgp_dn4 - locals.var_ps0_dn4) - ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), ((locals.var_vgp_dn5 - locals.var_ps0_dn5) - ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), ((locals.var_vgp_dn6 - locals.var_ps0_dn6) - ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), ((locals.var_vgp_dn7 - locals.var_ps0_dn7) - ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), ((locals.var_vgp_dn8 - locals.var_ps0_dn8) - ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), ((locals.var_vgp_dn9 - locals.var_ps0_dn9) - ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), ((locals.var_vgp_dn10 - locals.var_ps0_dn10) - ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), ((locals.var_vgp_dn11 - locals.var_ps0_dn11) - ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), ((locals.var_vgp_dn14 - locals.var_ps0_dn14) - ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign58550_body20_e91265;
            locals.var_fs0_dn0 = assign58550_body20_e91265_d_n0;
            locals.var_fs0_dn2 = assign58550_body20_e91265_d_n2;
            locals.var_fs0_dn4 = assign58550_body20_e91265_d_n4;
            locals.var_fs0_dn5 = assign58550_body20_e91265_d_n5;
            locals.var_fs0_dn6 = assign58550_body20_e91265_d_n6;
            locals.var_fs0_dn7 = assign58550_body20_e91265_d_n7;
            locals.var_fs0_dn8 = assign58550_body20_e91265_d_n8;
            locals.var_fs0_dn9 = assign58550_body20_e91265_d_n9;
            locals.var_fs0_dn10 = assign58550_body20_e91265_d_n10;
            locals.var_fs0_dn11 = assign58550_body20_e91265_d_n11;
            locals.var_fs0_dn14 = assign58550_body20_e91265_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign58550_body21_e91277, assign58550_body21_e91277_d_n0, assign58550_body21_e91277_d_n2, assign58550_body21_e91277_d_n4, assign58550_body21_e91277_d_n5, assign58550_body21_e91277_d_n6, assign58550_body21_e91277_d_n7, assign58550_body21_e91277_d_n8, assign58550_body21_e91277_d_n9, assign58550_body21_e91277_d_n10, assign58550_body21_e91277_d_n11, assign58550_body21_e91277_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58550_body21_e91271: f64 = (-1.0);
        let assign58550_body21_e91274: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign58550_body21_e91275: f64 = (assign58550_body21_e91271 - assign58550_body21_e91274);
        (assign58550_body21_e91275, (-((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0))), (-((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2))), (-((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4))), (-((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5))), (-((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6))), (-((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7))), (-((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8))), (-((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9))), (-((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10))), (-((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11))), (-((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14))),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign58550_body21_e91277;
            locals.var_fs0_dps0_dn0 = assign58550_body21_e91277_d_n0;
            locals.var_fs0_dps0_dn2 = assign58550_body21_e91277_d_n2;
            locals.var_fs0_dps0_dn4 = assign58550_body21_e91277_d_n4;
            locals.var_fs0_dps0_dn5 = assign58550_body21_e91277_d_n5;
            locals.var_fs0_dps0_dn6 = assign58550_body21_e91277_d_n6;
            locals.var_fs0_dps0_dn7 = assign58550_body21_e91277_d_n7;
            locals.var_fs0_dps0_dn8 = assign58550_body21_e91277_d_n8;
            locals.var_fs0_dps0_dn9 = assign58550_body21_e91277_d_n9;
            locals.var_fs0_dps0_dn10 = assign58550_body21_e91277_d_n10;
            locals.var_fs0_dps0_dn11 = assign58550_body21_e91277_d_n11;
            locals.var_fs0_dps0_dn14 = assign58550_body21_e91277_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign58550_body22_e91280: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1441 = assign58550_body22_e91280;
            locals.var_guard1441_rv = 0.0;
            let (assign58550_body23_e91289,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1441 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_brk1,)
    }
};
            locals.var_flg_brk1 = assign58550_body23_e91289;
            locals.var_flg_brk1_rv = 0.0;
            let assign58550_body24_e91292: f64 = if locals.var_flg_brk1 == 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1442 = assign58550_body24_e91292;
            locals.var_guard1442_rv = 0.0;
            let (assign58550_body25_e91304, assign58550_body25_e91304_d_n0, assign58550_body25_e91304_d_n2, assign58550_body25_e91304_d_n4, assign58550_body25_e91304_d_n5, assign58550_body25_e91304_d_n6, assign58550_body25_e91304_d_n7, assign58550_body25_e91304_d_n8, assign58550_body25_e91304_d_n9, assign58550_body25_e91304_d_n10, assign58550_body25_e91304_d_n11, assign58550_body25_e91304_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1442 != 0.0)) {
        let assign58550_body25_e91300: f64 = (-locals.var_fs0);
        let assign58550_body25_e91302: f64 = (assign58550_body25_e91300 / locals.var_fs0_dps0);
        (assign58550_body25_e91302, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign58550_body25_e91300 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign58550_body25_e91304;
            locals.var_dps0_dn0 = assign58550_body25_e91304_d_n0;
            locals.var_dps0_dn2 = assign58550_body25_e91304_d_n2;
            locals.var_dps0_dn4 = assign58550_body25_e91304_d_n4;
            locals.var_dps0_dn5 = assign58550_body25_e91304_d_n5;
            locals.var_dps0_dn6 = assign58550_body25_e91304_d_n6;
            locals.var_dps0_dn7 = assign58550_body25_e91304_d_n7;
            locals.var_dps0_dn8 = assign58550_body25_e91304_d_n8;
            locals.var_dps0_dn9 = assign58550_body25_e91304_d_n9;
            locals.var_dps0_dn10 = assign58550_body25_e91304_d_n10;
            locals.var_dps0_dn11 = assign58550_body25_e91304_d_n11;
            locals.var_dps0_dn14 = assign58550_body25_e91304_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign58550_body26_e91326, assign58550_body26_e91326_d_n0, assign58550_body26_e91326_d_n2, assign58550_body26_e91326_d_n4, assign58550_body26_e91326_d_n5, assign58550_body26_e91326_d_n6, assign58550_body26_e91326_d_n7, assign58550_body26_e91326_d_n8, assign58550_body26_e91326_d_n9, assign58550_body26_e91326_d_n10, assign58550_body26_e91326_d_n11, assign58550_body26_e91326_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1442 != 0.0)) {
        let assign58550_body26_e91313: f64 = (0.5 * 0.1);
        let assign58550_body26_e91317: f64 = (locals.var_ps0).abs();
        let (assign58550_body26_e91322, assign58550_body26_e91322_d_n0, assign58550_body26_e91322_d_n2, assign58550_body26_e91322_d_n4, assign58550_body26_e91322_d_n5, assign58550_body26_e91322_d_n6, assign58550_body26_e91322_d_n7, assign58550_body26_e91322_d_n8, assign58550_body26_e91322_d_n9, assign58550_body26_e91322_d_n10, assign58550_body26_e91322_d_n11, assign58550_body26_e91322_d_n14,) = {
            if (1.0 >= assign58550_body26_e91317) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign58550_body26_e91321: f64 = (locals.var_ps0).abs();
                (assign58550_body26_e91321, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn0 } else { (-locals.var_ps0_dn0) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn2 } else { (-locals.var_ps0_dn2) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn4 } else { (-locals.var_ps0_dn4) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn5 } else { (-locals.var_ps0_dn5) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn6 } else { (-locals.var_ps0_dn6) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn7 } else { (-locals.var_ps0_dn7) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn8 } else { (-locals.var_ps0_dn8) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn9 } else { (-locals.var_ps0_dn9) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn10 } else { (-locals.var_ps0_dn10) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn11 } else { (-locals.var_ps0_dn11) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn14 } else { (-locals.var_ps0_dn14) },)
            }
        };
        let assign58550_body26_e91323: f64 = (1.0 + assign58550_body26_e91322);
        let assign58550_body26_e91324: f64 = (assign58550_body26_e91313 * assign58550_body26_e91323);
        (assign58550_body26_e91324, (assign58550_body26_e91313 * assign58550_body26_e91322_d_n0), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n2), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n4), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n5), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n6), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n7), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n8), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n9), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n10), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n11), (assign58550_body26_e91313 * assign58550_body26_e91322_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign58550_body26_e91326;
            locals.var_dplim_dn0 = assign58550_body26_e91326_d_n0;
            locals.var_dplim_dn2 = assign58550_body26_e91326_d_n2;
            locals.var_dplim_dn4 = assign58550_body26_e91326_d_n4;
            locals.var_dplim_dn5 = assign58550_body26_e91326_d_n5;
            locals.var_dplim_dn6 = assign58550_body26_e91326_d_n6;
            locals.var_dplim_dn7 = assign58550_body26_e91326_d_n7;
            locals.var_dplim_dn8 = assign58550_body26_e91326_d_n8;
            locals.var_dplim_dn9 = assign58550_body26_e91326_d_n9;
            locals.var_dplim_dn10 = assign58550_body26_e91326_d_n10;
            locals.var_dplim_dn11 = assign58550_body26_e91326_d_n11;
            locals.var_dplim_dn14 = assign58550_body26_e91326_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign58550_body27_e91328: f64 = (locals.var_dps0).abs();
            let assign58550_body27_e91330: f64 = if assign58550_body27_e91328 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1443 = assign58550_body27_e91330;
            locals.var_guard1443_rv = 0.0;
            let (assign58550_body28_e91349, assign58550_body28_e91349_d_n0, assign58550_body28_e91349_d_n2, assign58550_body28_e91349_d_n4, assign58550_body28_e91349_d_n5, assign58550_body28_e91349_d_n6, assign58550_body28_e91349_d_n7, assign58550_body28_e91349_d_n8, assign58550_body28_e91349_d_n9, assign58550_body28_e91349_d_n10, assign58550_body28_e91349_d_n11, assign58550_body28_e91349_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1442 != 0.0)) && (locals.var_guard1443 != 0.0)) {
        let (assign58550_body28_e91346,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign58550_body28_e91345: f64 = (-1.0);
                (assign58550_body28_e91345,)
            }
        };
        let assign58550_body28_e91347: f64 = (locals.var_dplim * assign58550_body28_e91346);
        (assign58550_body28_e91347, (locals.var_dplim_dn0 * assign58550_body28_e91346), (locals.var_dplim_dn2 * assign58550_body28_e91346), (locals.var_dplim_dn4 * assign58550_body28_e91346), (locals.var_dplim_dn5 * assign58550_body28_e91346), (locals.var_dplim_dn6 * assign58550_body28_e91346), (locals.var_dplim_dn7 * assign58550_body28_e91346), (locals.var_dplim_dn8 * assign58550_body28_e91346), (locals.var_dplim_dn9 * assign58550_body28_e91346), (locals.var_dplim_dn10 * assign58550_body28_e91346), (locals.var_dplim_dn11 * assign58550_body28_e91346), (locals.var_dplim_dn14 * assign58550_body28_e91346),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign58550_body28_e91349;
            locals.var_dps0_dn0 = assign58550_body28_e91349_d_n0;
            locals.var_dps0_dn2 = assign58550_body28_e91349_d_n2;
            locals.var_dps0_dn4 = assign58550_body28_e91349_d_n4;
            locals.var_dps0_dn5 = assign58550_body28_e91349_d_n5;
            locals.var_dps0_dn6 = assign58550_body28_e91349_d_n6;
            locals.var_dps0_dn7 = assign58550_body28_e91349_d_n7;
            locals.var_dps0_dn8 = assign58550_body28_e91349_d_n8;
            locals.var_dps0_dn9 = assign58550_body28_e91349_d_n9;
            locals.var_dps0_dn10 = assign58550_body28_e91349_d_n10;
            locals.var_dps0_dn11 = assign58550_body28_e91349_d_n11;
            locals.var_dps0_dn14 = assign58550_body28_e91349_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign58550_body29_e91360, assign58550_body29_e91360_d_n0, assign58550_body29_e91360_d_n2, assign58550_body29_e91360_d_n4, assign58550_body29_e91360_d_n5, assign58550_body29_e91360_d_n6, assign58550_body29_e91360_d_n7, assign58550_body29_e91360_d_n8, assign58550_body29_e91360_d_n9, assign58550_body29_e91360_d_n10, assign58550_body29_e91360_d_n11, assign58550_body29_e91360_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1442 != 0.0)) {
        let assign58550_body29_e91358: f64 = (locals.var_ps0 + locals.var_dps0);
        (assign58550_body29_e91358, (locals.var_ps0_dn0 + locals.var_dps0_dn0), (locals.var_ps0_dn2 + locals.var_dps0_dn2), (locals.var_ps0_dn4 + locals.var_dps0_dn4), (locals.var_ps0_dn5 + locals.var_dps0_dn5), (locals.var_ps0_dn6 + locals.var_dps0_dn6), (locals.var_ps0_dn7 + locals.var_dps0_dn7), (locals.var_ps0_dn8 + locals.var_dps0_dn8), (locals.var_ps0_dn9 + locals.var_dps0_dn9), (locals.var_ps0_dn10 + locals.var_dps0_dn10), (locals.var_ps0_dn11 + locals.var_dps0_dn11), (locals.var_ps0_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
            locals.var_ps0 = assign58550_body29_e91360;
            locals.var_ps0_dn0 = assign58550_body29_e91360_d_n0;
            locals.var_ps0_dn2 = assign58550_body29_e91360_d_n2;
            locals.var_ps0_dn4 = assign58550_body29_e91360_d_n4;
            locals.var_ps0_dn5 = assign58550_body29_e91360_d_n5;
            locals.var_ps0_dn6 = assign58550_body29_e91360_d_n6;
            locals.var_ps0_dn7 = assign58550_body29_e91360_d_n7;
            locals.var_ps0_dn8 = assign58550_body29_e91360_d_n8;
            locals.var_ps0_dn9 = assign58550_body29_e91360_d_n9;
            locals.var_ps0_dn10 = assign58550_body29_e91360_d_n10;
            locals.var_ps0_dn11 = assign58550_body29_e91360_d_n11;
            locals.var_ps0_dn14 = assign58550_body29_e91360_d_n14;
            locals.var_ps0_rv = 0.0;
            let assign58550_body30_e91362: f64 = (locals.var_dps0).abs();
            let assign58550_body30_e91366: f64 = (locals.var_fs0).abs();
            let assign58550_body30_e91369: f64 = if ((assign58550_body30_e91362 <= 1e-12) && (assign58550_body30_e91366 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1444 = assign58550_body30_e91369;
            locals.var_guard1444_rv = 0.0;
            let (assign58550_body31_e91380,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1442 != 0.0)) && (locals.var_guard1444 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign58550_body31_e91380;
            locals.var_flg_conv_rv = 0.0;
            let (assign58550_body32_e91391,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_flg_brk1 != 0.0)) {
        let assign58550_body32_e91389: f64 = (locals.var_lp_s0_max + 1.0);
        (assign58550_body32_e91389,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign58550_body32_e91391;
            locals.var_lp_s0_rv = 0.0;
            let (assign58550_body33_e91398,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk1,)
    }
};
            locals.var_flg_brk1 = assign58550_body33_e91398;
            locals.var_flg_brk1_rv = 0.0;
            let (assign58550_body34_e91407,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58550_body34_e91405: f64 = (locals.var_lp_s0 + 1.0);
        (assign58550_body34_e91405,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign58550_body34_e91407;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_216(
        locals: &mut StampLocals,
    ) {
        let (assign58560_e91416,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58560_e91414: f64 = (locals.var_lp_s0 - 1.0);
        (assign58560_e91414,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign58560_e91416;
        locals.var_lp_s0_rv = 0.0;

        let assign58580_e91422: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1446 = assign58580_e91422;
        locals.var_guard1446_rv = 0.0;

        let (assign58590_e91437, assign58590_e91437_d_n0, assign58590_e91437_d_n2, assign58590_e91437_d_n4, assign58590_e91437_d_n5, assign58590_e91437_d_n6, assign58590_e91437_d_n7, assign58590_e91437_d_n8, assign58590_e91437_d_n9, assign58590_e91437_d_n10, assign58590_e91437_d_n11, assign58590_e91437_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1446 != 0.0)) {
        let assign58590_e91431: f64 = (locals.var_fb * locals.var_fb);
        let assign58590_e91434: f64 = (10.0 * 2.220446049250313e-16);
        let assign58590_e91435: f64 = (assign58590_e91431 + assign58590_e91434);
        (assign58590_e91435, ((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)), ((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)), ((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)), ((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)), ((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)), ((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)), ((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)), ((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)), ((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)), ((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)), ((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn7, locals.var_xi0_dn8, locals.var_xi0_dn9, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn14,)
    }
};
        locals.var_xi0 = assign58590_e91437;
        locals.var_xi0_dn0 = assign58590_e91437_d_n0;
        locals.var_xi0_dn2 = assign58590_e91437_d_n2;
        locals.var_xi0_dn4 = assign58590_e91437_d_n4;
        locals.var_xi0_dn5 = assign58590_e91437_d_n5;
        locals.var_xi0_dn6 = assign58590_e91437_d_n6;
        locals.var_xi0_dn7 = assign58590_e91437_d_n7;
        locals.var_xi0_dn8 = assign58590_e91437_d_n8;
        locals.var_xi0_dn9 = assign58590_e91437_d_n9;
        locals.var_xi0_dn10 = assign58590_e91437_d_n10;
        locals.var_xi0_dn11 = assign58590_e91437_d_n11;
        locals.var_xi0_dn14 = assign58590_e91437_d_n14;
        locals.var_xi0_rv = 0.0;

        let (assign58600_e91450, assign58600_e91450_d_n0, assign58600_e91450_d_n2, assign58600_e91450_d_n4, assign58600_e91450_d_n5, assign58600_e91450_d_n6, assign58600_e91450_d_n7, assign58600_e91450_d_n8, assign58600_e91450_d_n9, assign58600_e91450_d_n10, assign58600_e91450_d_n11, assign58600_e91450_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1446 != 0.0)) {
        let assign58600_e91447: f64 = (10.0 * 2.220446049250313e-16);
        let assign58600_e91448: f64 = (locals.var_fb + assign58600_e91447);
        (assign58600_e91448, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign58600_e91450;
        locals.var_xi0p12_dn0 = assign58600_e91450_d_n0;
        locals.var_xi0p12_dn2 = assign58600_e91450_d_n2;
        locals.var_xi0p12_dn4 = assign58600_e91450_d_n4;
        locals.var_xi0p12_dn5 = assign58600_e91450_d_n5;
        locals.var_xi0p12_dn6 = assign58600_e91450_d_n6;
        locals.var_xi0p12_dn7 = assign58600_e91450_d_n7;
        locals.var_xi0p12_dn8 = assign58600_e91450_d_n8;
        locals.var_xi0p12_dn9 = assign58600_e91450_d_n9;
        locals.var_xi0p12_dn10 = assign58600_e91450_d_n10;
        locals.var_xi0p12_dn11 = assign58600_e91450_d_n11;
        locals.var_xi0p12_dn14 = assign58600_e91450_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign58610_e91467, assign58610_e91467_d_n0, assign58610_e91467_d_n2, assign58610_e91467_d_n4, assign58610_e91467_d_n5, assign58610_e91467_d_n6, assign58610_e91467_d_n7, assign58610_e91467_d_n8, assign58610_e91467_d_n9, assign58610_e91467_d_n10, assign58610_e91467_d_n11, assign58610_e91467_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1446 != 0.0)) {
        let assign58610_e91459: f64 = (locals.var_fb * locals.var_fb);
        let assign58610_e91461: f64 = (assign58610_e91459 * locals.var_fb);
        let assign58610_e91464: f64 = (10.0 * 2.220446049250313e-16);
        let assign58610_e91465: f64 = (assign58610_e91461 + assign58610_e91464);
        (assign58610_e91465, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn0)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn2)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn4)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn5)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn6)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn7)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn8)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn9)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn10)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn11)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) * locals.var_fb) + (assign58610_e91459 * locals.var_fb_dn14)),)
    } else {
        (locals.var_xi0p32, locals.var_xi0p32_dn0, locals.var_xi0p32_dn2, locals.var_xi0p32_dn4, locals.var_xi0p32_dn5, locals.var_xi0p32_dn6, locals.var_xi0p32_dn7, locals.var_xi0p32_dn8, locals.var_xi0p32_dn9, locals.var_xi0p32_dn10, locals.var_xi0p32_dn11, locals.var_xi0p32_dn14,)
    }
};
        locals.var_xi0p32 = assign58610_e91467;
        locals.var_xi0p32_dn0 = assign58610_e91467_d_n0;
        locals.var_xi0p32_dn2 = assign58610_e91467_d_n2;
        locals.var_xi0p32_dn4 = assign58610_e91467_d_n4;
        locals.var_xi0p32_dn5 = assign58610_e91467_d_n5;
        locals.var_xi0p32_dn6 = assign58610_e91467_d_n6;
        locals.var_xi0p32_dn7 = assign58610_e91467_d_n7;
        locals.var_xi0p32_dn8 = assign58610_e91467_d_n8;
        locals.var_xi0p32_dn9 = assign58610_e91467_d_n9;
        locals.var_xi0p32_dn10 = assign58610_e91467_d_n10;
        locals.var_xi0p32_dn11 = assign58610_e91467_d_n11;
        locals.var_xi0p32_dn14 = assign58610_e91467_d_n14;
        locals.var_xi0p32_rv = 0.0;

        let (assign58620_e91477,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1446 == 0.0)) {
        (3.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign58620_e91477;
        locals.var_flg_zone_rv = 0.0;

        let (assign58630_e91487,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1446 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58630_e91487;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58640_e91499, assign58640_e91499_d_n0, assign58640_e91499_d_n2, assign58640_e91499_d_n4, assign58640_e91499_d_n5, assign58640_e91499_d_n6, assign58640_e91499_d_n7, assign58640_e91499_d_n8, assign58640_e91499_d_n9, assign58640_e91499_d_n10, assign58640_e91499_d_n11, assign58640_e91499_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1446 == 0.0)) {
        let assign58640_e91497: f64 = (locals.var_chi - 1.0);
        (assign58640_e91497, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn7, locals.var_xi0_dn8, locals.var_xi0_dn9, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn14,)
    }
};
        locals.var_xi0 = assign58640_e91499;
        locals.var_xi0_dn0 = assign58640_e91499_d_n0;
        locals.var_xi0_dn2 = assign58640_e91499_d_n2;
        locals.var_xi0_dn4 = assign58640_e91499_d_n4;
        locals.var_xi0_dn5 = assign58640_e91499_d_n5;
        locals.var_xi0_dn6 = assign58640_e91499_d_n6;
        locals.var_xi0_dn7 = assign58640_e91499_d_n7;
        locals.var_xi0_dn8 = assign58640_e91499_d_n8;
        locals.var_xi0_dn9 = assign58640_e91499_d_n9;
        locals.var_xi0_dn10 = assign58640_e91499_d_n10;
        locals.var_xi0_dn11 = assign58640_e91499_d_n11;
        locals.var_xi0_dn14 = assign58640_e91499_d_n14;
        locals.var_xi0_rv = 0.0;

        let (assign58650_e91510, assign58650_e91510_d_n0, assign58650_e91510_d_n2, assign58650_e91510_d_n4, assign58650_e91510_d_n5, assign58650_e91510_d_n6, assign58650_e91510_d_n7, assign58650_e91510_d_n8, assign58650_e91510_d_n9, assign58650_e91510_d_n10, assign58650_e91510_d_n11, assign58650_e91510_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1446 == 0.0)) {
        let assign58650_e91508: f64 = (locals.var_xi0).sqrt();
        (assign58650_e91508, (locals.var_xi0_dn0 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn2 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn4 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn5 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn6 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn7 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn8 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn9 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn10 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn11 / (2.0 * assign58650_e91508)), (locals.var_xi0_dn14 / (2.0 * assign58650_e91508)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign58650_e91510;
        locals.var_xi0p12_dn0 = assign58650_e91510_d_n0;
        locals.var_xi0p12_dn2 = assign58650_e91510_d_n2;
        locals.var_xi0p12_dn4 = assign58650_e91510_d_n4;
        locals.var_xi0p12_dn5 = assign58650_e91510_d_n5;
        locals.var_xi0p12_dn6 = assign58650_e91510_d_n6;
        locals.var_xi0p12_dn7 = assign58650_e91510_d_n7;
        locals.var_xi0p12_dn8 = assign58650_e91510_d_n8;
        locals.var_xi0p12_dn9 = assign58650_e91510_d_n9;
        locals.var_xi0p12_dn10 = assign58650_e91510_d_n10;
        locals.var_xi0p12_dn11 = assign58650_e91510_d_n11;
        locals.var_xi0p12_dn14 = assign58650_e91510_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign58660_e91522, assign58660_e91522_d_n0, assign58660_e91522_d_n2, assign58660_e91522_d_n4, assign58660_e91522_d_n5, assign58660_e91522_d_n6, assign58660_e91522_d_n7, assign58660_e91522_d_n8, assign58660_e91522_d_n9, assign58660_e91522_d_n10, assign58660_e91522_d_n11, assign58660_e91522_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1446 == 0.0)) {
        let assign58660_e91520: f64 = (locals.var_xi0 * locals.var_xi0p12);
        (assign58660_e91520, ((locals.var_xi0_dn0 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn0)), ((locals.var_xi0_dn2 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn2)), ((locals.var_xi0_dn4 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn4)), ((locals.var_xi0_dn5 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn5)), ((locals.var_xi0_dn6 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn6)), ((locals.var_xi0_dn7 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn7)), ((locals.var_xi0_dn8 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn8)), ((locals.var_xi0_dn9 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn9)), ((locals.var_xi0_dn10 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn10)), ((locals.var_xi0_dn11 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn11)), ((locals.var_xi0_dn14 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_xi0p32, locals.var_xi0p32_dn0, locals.var_xi0p32_dn2, locals.var_xi0p32_dn4, locals.var_xi0p32_dn5, locals.var_xi0p32_dn6, locals.var_xi0p32_dn7, locals.var_xi0p32_dn8, locals.var_xi0p32_dn9, locals.var_xi0p32_dn10, locals.var_xi0p32_dn11, locals.var_xi0p32_dn14,)
    }
};
        locals.var_xi0p32 = assign58660_e91522;
        locals.var_xi0p32_dn0 = assign58660_e91522_d_n0;
        locals.var_xi0p32_dn2 = assign58660_e91522_d_n2;
        locals.var_xi0p32_dn4 = assign58660_e91522_d_n4;
        locals.var_xi0p32_dn5 = assign58660_e91522_d_n5;
        locals.var_xi0p32_dn6 = assign58660_e91522_d_n6;
        locals.var_xi0p32_dn7 = assign58660_e91522_d_n7;
        locals.var_xi0p32_dn8 = assign58660_e91522_d_n8;
        locals.var_xi0p32_dn9 = assign58660_e91522_d_n9;
        locals.var_xi0p32_dn10 = assign58660_e91522_d_n10;
        locals.var_xi0p32_dn11 = assign58660_e91522_d_n11;
        locals.var_xi0p32_dn14 = assign58660_e91522_d_n14;
        locals.var_xi0p32_rv = 0.0;

        let (assign58670_e91531, assign58670_e91531_d_n0, assign58670_e91531_d_n2, assign58670_e91531_d_n4, assign58670_e91531_d_n5, assign58670_e91531_d_n6, assign58670_e91531_d_n7, assign58670_e91531_d_n8, assign58670_e91531_d_n9, assign58670_e91531_d_n10, assign58670_e91531_d_n11, assign58670_e91531_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58670_e91529: f64 = (locals.var_cnst0 * locals.var_xi0p12);
        (assign58670_e91529, ((locals.var_cnst0_dn0 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn0)), ((locals.var_cnst0_dn2 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn2)), ((locals.var_cnst0_dn4 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn4)), ((locals.var_cnst0_dn5 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn5)), ((locals.var_cnst0_dn6 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn6)), ((locals.var_cnst0_dn7 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn7)), ((locals.var_cnst0_dn8 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn8)), ((locals.var_cnst0_dn9 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn9)), ((locals.var_cnst0_dn10 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn10)), ((locals.var_cnst0_dn11 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn11)), ((locals.var_cnst0_dn14 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qb0, locals.var_qb0_dn0, locals.var_qb0_dn2, locals.var_qb0_dn4, locals.var_qb0_dn5, locals.var_qb0_dn6, locals.var_qb0_dn7, locals.var_qb0_dn8, locals.var_qb0_dn9, locals.var_qb0_dn10, locals.var_qb0_dn11, locals.var_qb0_dn14,)
    }
};
        locals.var_qb0 = assign58670_e91531;
        locals.var_qb0_dn0 = assign58670_e91531_d_n0;
        locals.var_qb0_dn2 = assign58670_e91531_d_n2;
        locals.var_qb0_dn4 = assign58670_e91531_d_n4;
        locals.var_qb0_dn5 = assign58670_e91531_d_n5;
        locals.var_qb0_dn6 = assign58670_e91531_d_n6;
        locals.var_qb0_dn7 = assign58670_e91531_d_n7;
        locals.var_qb0_dn8 = assign58670_e91531_d_n8;
        locals.var_qb0_dn9 = assign58670_e91531_d_n9;
        locals.var_qb0_dn10 = assign58670_e91531_d_n10;
        locals.var_qb0_dn11 = assign58670_e91531_d_n11;
        locals.var_qb0_dn14 = assign58670_e91531_d_n14;
        locals.var_qb0_rv = 0.0;

        let (assign58680_e91542, assign58680_e91542_d_n0, assign58680_e91542_d_n2, assign58680_e91542_d_n4, assign58680_e91542_d_n5, assign58680_e91542_d_n6, assign58680_e91542_d_n7, assign58680_e91542_d_n8, assign58680_e91542_d_n9, assign58680_e91542_d_n10, assign58680_e91542_d_n11, assign58680_e91542_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58680_e91539: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign58680_e91540: f64 = (1.0 / assign58680_e91539);
        (assign58680_e91540, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign58680_e91539 * assign58680_e91539))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign58680_e91539 * assign58680_e91539))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58680_e91542;
        locals.var_t1_dn0 = assign58680_e91542_d_n0;
        locals.var_t1_dn2 = assign58680_e91542_d_n2;
        locals.var_t1_dn4 = assign58680_e91542_d_n4;
        locals.var_t1_dn5 = assign58680_e91542_d_n5;
        locals.var_t1_dn6 = assign58680_e91542_d_n6;
        locals.var_t1_dn7 = assign58680_e91542_d_n7;
        locals.var_t1_dn8 = assign58680_e91542_d_n8;
        locals.var_t1_dn9 = assign58680_e91542_d_n9;
        locals.var_t1_dn10 = assign58680_e91542_d_n10;
        locals.var_t1_dn11 = assign58680_e91542_d_n11;
        locals.var_t1_dn14 = assign58680_e91542_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58690_e91555, assign58690_e91555_d_n0, assign58690_e91555_d_n2, assign58690_e91555_d_n4, assign58690_e91555_d_n5, assign58690_e91555_d_n6, assign58690_e91555_d_n7, assign58690_e91555_d_n8, assign58690_e91555_d_n9, assign58690_e91555_d_n10, assign58690_e91555_d_n11, assign58690_e91555_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58690_e91549: f64 = (locals.var_cnst0 * locals.var_fs01);
        let assign58690_e91551: f64 = (assign58690_e91549 * locals.var_t1);
        let assign58690_e91553: f64 = (assign58690_e91551 + 1e-25);
        (assign58690_e91553, ((((locals.var_cnst0_dn0 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn0)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn0)), ((((locals.var_cnst0_dn2 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn2)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn2)), ((((locals.var_cnst0_dn4 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn4)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn4)), ((((locals.var_cnst0_dn5 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn5)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn5)), ((((locals.var_cnst0_dn6 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn6)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn6)), ((((locals.var_cnst0_dn7 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn7)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn7)), ((((locals.var_cnst0_dn8 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn8)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn8)), ((((locals.var_cnst0_dn9 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn9)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn9)), ((((locals.var_cnst0_dn10 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn10)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn10)), ((((locals.var_cnst0_dn11 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn11)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn11)), ((((locals.var_cnst0_dn14 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn14)) * locals.var_t1) + (assign58690_e91549 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn14,)
    }
};
        locals.var_qn0 = assign58690_e91555;
        locals.var_qn0_dn0 = assign58690_e91555_d_n0;
        locals.var_qn0_dn2 = assign58690_e91555_d_n2;
        locals.var_qn0_dn4 = assign58690_e91555_d_n4;
        locals.var_qn0_dn5 = assign58690_e91555_d_n5;
        locals.var_qn0_dn6 = assign58690_e91555_d_n6;
        locals.var_qn0_dn7 = assign58690_e91555_d_n7;
        locals.var_qn0_dn8 = assign58690_e91555_d_n8;
        locals.var_qn0_dn9 = assign58690_e91555_d_n9;
        locals.var_qn0_dn10 = assign58690_e91555_d_n10;
        locals.var_qn0_dn11 = assign58690_e91555_d_n11;
        locals.var_qn0_dn14 = assign58690_e91555_d_n14;
        locals.var_qn0_rv = 0.0;

        let assign58700_e91558: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1447 = assign58700_e91558;
        locals.var_guard1447_rv = 0.0;

        let assign58710_e91561: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1448 = assign58710_e91561;
        locals.var_guard1448_rv = 0.0;

        let (assign58720_e91572,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign58720_e91572;
        locals.var_flg_zone_rv = 0.0;

        let (assign58730_e91583,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58730_e91583;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58740_e91594, assign58740_e91594_d_n0, assign58740_e91594_d_n2, assign58740_e91594_d_n4, assign58740_e91594_d_n5, assign58740_e91594_d_n6, assign58740_e91594_d_n7, assign58740_e91594_d_n8, assign58740_e91594_d_n9, assign58740_e91594_d_n10, assign58740_e91594_d_n11, assign58740_e91594_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn14,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign58740_e91594;
        locals.var_qiu_dn0 = assign58740_e91594_d_n0;
        locals.var_qiu_dn2 = assign58740_e91594_d_n2;
        locals.var_qiu_dn4 = assign58740_e91594_d_n4;
        locals.var_qiu_dn5 = assign58740_e91594_d_n5;
        locals.var_qiu_dn6 = assign58740_e91594_d_n6;
        locals.var_qiu_dn7 = assign58740_e91594_d_n7;
        locals.var_qiu_dn8 = assign58740_e91594_d_n8;
        locals.var_qiu_dn9 = assign58740_e91594_d_n9;
        locals.var_qiu_dn10 = assign58740_e91594_d_n10;
        locals.var_qiu_dn11 = assign58740_e91594_d_n11;
        locals.var_qiu_dn14 = assign58740_e91594_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign58750_e91605, assign58750_e91605_d_n0, assign58750_e91605_d_n2, assign58750_e91605_d_n4, assign58750_e91605_d_n5, assign58750_e91605_d_n6, assign58750_e91605_d_n7, assign58750_e91605_d_n8, assign58750_e91605_d_n9, assign58750_e91605_d_n10, assign58750_e91605_d_n11, assign58750_e91605_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        (locals.var_qb0, locals.var_qb0_dn0, locals.var_qb0_dn2, locals.var_qb0_dn4, locals.var_qb0_dn5, locals.var_qb0_dn6, locals.var_qb0_dn7, locals.var_qb0_dn8, locals.var_qb0_dn9, locals.var_qb0_dn10, locals.var_qb0_dn11, locals.var_qb0_dn14,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign58750_e91605;
        locals.var_qbu_dn0 = assign58750_e91605_d_n0;
        locals.var_qbu_dn2 = assign58750_e91605_d_n2;
        locals.var_qbu_dn4 = assign58750_e91605_d_n4;
        locals.var_qbu_dn5 = assign58750_e91605_d_n5;
        locals.var_qbu_dn6 = assign58750_e91605_d_n6;
        locals.var_qbu_dn7 = assign58750_e91605_d_n7;
        locals.var_qbu_dn8 = assign58750_e91605_d_n8;
        locals.var_qbu_dn9 = assign58750_e91605_d_n9;
        locals.var_qbu_dn10 = assign58750_e91605_d_n10;
        locals.var_qbu_dn11 = assign58750_e91605_d_n11;
        locals.var_qbu_dn14 = assign58750_e91605_d_n14;
        locals.var_qbu_rv = 0.0;

        let (assign58760_e91616, assign58760_e91616_d_n0, assign58760_e91616_d_n2, assign58760_e91616_d_n4, assign58760_e91616_d_n5, assign58760_e91616_d_n6, assign58760_e91616_d_n7, assign58760_e91616_d_n8, assign58760_e91616_d_n9, assign58760_e91616_d_n10, assign58760_e91616_d_n11, assign58760_e91616_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign58760_e91616;
        locals.var_qdrat_dn0 = assign58760_e91616_d_n0;
        locals.var_qdrat_dn2 = assign58760_e91616_d_n2;
        locals.var_qdrat_dn4 = assign58760_e91616_d_n4;
        locals.var_qdrat_dn5 = assign58760_e91616_d_n5;
        locals.var_qdrat_dn6 = assign58760_e91616_d_n6;
        locals.var_qdrat_dn7 = assign58760_e91616_d_n7;
        locals.var_qdrat_dn8 = assign58760_e91616_d_n8;
        locals.var_qdrat_dn9 = assign58760_e91616_d_n9;
        locals.var_qdrat_dn10 = assign58760_e91616_d_n10;
        locals.var_qdrat_dn11 = assign58760_e91616_d_n11;
        locals.var_qdrat_dn14 = assign58760_e91616_d_n14;
        locals.var_qdrat_rv = 0.0;

        let (assign58770_e91627, assign58770_e91627_d_n0, assign58770_e91627_d_n2, assign58770_e91627_d_n4, assign58770_e91627_d_n5, assign58770_e91627_d_n6, assign58770_e91627_d_n7, assign58770_e91627_d_n8, assign58770_e91627_d_n9, assign58770_e91627_d_n10, assign58770_e91627_d_n11, assign58770_e91627_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign58770_e91627;
        locals.var_lred_dn0 = assign58770_e91627_d_n0;
        locals.var_lred_dn2 = assign58770_e91627_d_n2;
        locals.var_lred_dn4 = assign58770_e91627_d_n4;
        locals.var_lred_dn5 = assign58770_e91627_d_n5;
        locals.var_lred_dn6 = assign58770_e91627_d_n6;
        locals.var_lred_dn7 = assign58770_e91627_d_n7;
        locals.var_lred_dn8 = assign58770_e91627_d_n8;
        locals.var_lred_dn9 = assign58770_e91627_d_n9;
        locals.var_lred_dn10 = assign58770_e91627_d_n10;
        locals.var_lred_dn11 = assign58770_e91627_d_n11;
        locals.var_lred_dn14 = assign58770_e91627_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign58780_e91639,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign58780_e91639;
        locals.var_flg_zone_rv = 0.0;

        let (assign58790_e91651,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58790_e91651;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58800_e91667, assign58800_e91667_d_n0, assign58800_e91667_d_n2, assign58800_e91667_d_n4, assign58800_e91667_d_n5, assign58800_e91667_d_n6, assign58800_e91667_d_n7, assign58800_e91667_d_n8, assign58800_e91667_d_n9, assign58800_e91667_d_n10, assign58800_e91667_d_n11, assign58800_e91667_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 == 0.0)) {
        let assign58800_e91664: f64 = (5.0 - 3.0);
        let assign58800_e91665: f64 = (1.0 / assign58800_e91664);
        (assign58800_e91665, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58800_e91667;
        locals.var_t1_dn0 = assign58800_e91667_d_n0;
        locals.var_t1_dn2 = assign58800_e91667_d_n2;
        locals.var_t1_dn4 = assign58800_e91667_d_n4;
        locals.var_t1_dn5 = assign58800_e91667_d_n5;
        locals.var_t1_dn6 = assign58800_e91667_d_n6;
        locals.var_t1_dn7 = assign58800_e91667_d_n7;
        locals.var_t1_dn8 = assign58800_e91667_d_n8;
        locals.var_t1_dn9 = assign58800_e91667_d_n9;
        locals.var_t1_dn10 = assign58800_e91667_d_n10;
        locals.var_t1_dn11 = assign58800_e91667_d_n11;
        locals.var_t1_dn14 = assign58800_e91667_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58810_e91683, assign58810_e91683_d_n0, assign58810_e91683_d_n2, assign58810_e91683_d_n4, assign58810_e91683_d_n5, assign58810_e91683_d_n6, assign58810_e91683_d_n7, assign58810_e91683_d_n8, assign58810_e91683_d_n9, assign58810_e91683_d_n10, assign58810_e91683_d_n11, assign58810_e91683_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 == 0.0)) {
        let assign58810_e91680: f64 = (locals.var_chi - 3.0);
        let assign58810_e91681: f64 = (locals.var_t1 * assign58810_e91680);
        (assign58810_e91681, ((locals.var_t1_dn0 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn0)), ((locals.var_t1_dn2 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn2)), ((locals.var_t1_dn4 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn4)), ((locals.var_t1_dn5 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn5)), ((locals.var_t1_dn6 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn6)), ((locals.var_t1_dn7 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn7)), ((locals.var_t1_dn8 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn8)), ((locals.var_t1_dn9 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn9)), ((locals.var_t1_dn10 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn10)), ((locals.var_t1_dn11 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn11)), ((locals.var_t1_dn14 * assign58810_e91680) + (locals.var_t1 * locals.var_chi_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58810_e91683;
        locals.var_tx_dn0 = assign58810_e91683_d_n0;
        locals.var_tx_dn2 = assign58810_e91683_d_n2;
        locals.var_tx_dn4 = assign58810_e91683_d_n4;
        locals.var_tx_dn5 = assign58810_e91683_d_n5;
        locals.var_tx_dn6 = assign58810_e91683_d_n6;
        locals.var_tx_dn7 = assign58810_e91683_d_n7;
        locals.var_tx_dn8 = assign58810_e91683_d_n8;
        locals.var_tx_dn9 = assign58810_e91683_d_n9;
        locals.var_tx_dn10 = assign58810_e91683_d_n10;
        locals.var_tx_dn11 = assign58810_e91683_d_n11;
        locals.var_tx_dn14 = assign58810_e91683_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58820_e91710, assign58820_e91710_d_n0, assign58820_e91710_d_n2, assign58820_e91710_d_n4, assign58820_e91710_d_n5, assign58820_e91710_d_n6, assign58820_e91710_d_n7, assign58820_e91710_d_n8, assign58820_e91710_d_n9, assign58820_e91710_d_n10, assign58820_e91710_d_n11, assign58820_e91710_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1447 != 0.0)) && (locals.var_guard1448 == 0.0)) {
        let assign58820_e91695: f64 = (locals.var_tx * locals.var_tx);
        let assign58820_e91697: f64 = (assign58820_e91695 * locals.var_tx);
        let assign58820_e91701: f64 = (-15.0);
        let assign58820_e91704: f64 = (locals.var_tx * 6.0);
        let assign58820_e91705: f64 = (assign58820_e91701 + assign58820_e91704);
        let assign58820_e91706: f64 = (locals.var_tx * assign58820_e91705);
        let assign58820_e91707: f64 = (10.0 + assign58820_e91706);
        let assign58820_e91708: f64 = (assign58820_e91697 * assign58820_e91707);
        (assign58820_e91708, ((((((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn0)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn0 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn0 * 6.0))))), ((((((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn2)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn2 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn2 * 6.0))))), ((((((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn4)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn4 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn4 * 6.0))))), ((((((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn5)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn5 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn5 * 6.0))))), ((((((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn6)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn6 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn6 * 6.0))))), ((((((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn7)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn7 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn7 * 6.0))))), ((((((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn8)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn8 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn8 * 6.0))))), ((((((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn9)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn9 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn9 * 6.0))))), ((((((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn10)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn10 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn10 * 6.0))))), ((((((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn11)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn11 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn11 * 6.0))))), ((((((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14)) * locals.var_tx) + (assign58820_e91695 * locals.var_tx_dn14)) * assign58820_e91707) + (assign58820_e91697 * ((locals.var_tx_dn14 * assign58820_e91705) + (locals.var_tx * (locals.var_tx_dn14 * 6.0))))),)
    } else {
        (locals.var_fd2, locals.var_fd2_dn0, locals.var_fd2_dn2, locals.var_fd2_dn4, locals.var_fd2_dn5, locals.var_fd2_dn6, locals.var_fd2_dn7, locals.var_fd2_dn8, locals.var_fd2_dn9, locals.var_fd2_dn10, locals.var_fd2_dn11, locals.var_fd2_dn14,)
    }
};
        locals.var_fd2 = assign58820_e91710;
        locals.var_fd2_dn0 = assign58820_e91710_d_n0;
        locals.var_fd2_dn2 = assign58820_e91710_d_n2;
        locals.var_fd2_dn4 = assign58820_e91710_d_n4;
        locals.var_fd2_dn5 = assign58820_e91710_d_n5;
        locals.var_fd2_dn6 = assign58820_e91710_d_n6;
        locals.var_fd2_dn7 = assign58820_e91710_d_n7;
        locals.var_fd2_dn8 = assign58820_e91710_d_n8;
        locals.var_fd2_dn9 = assign58820_e91710_d_n9;
        locals.var_fd2_dn10 = assign58820_e91710_d_n10;
        locals.var_fd2_dn11 = assign58820_e91710_d_n11;
        locals.var_fd2_dn14 = assign58820_e91710_d_n14;
        locals.var_fd2_rv = 0.0;

        let (assign58830_e91719, assign58830_e91719_d_n0, assign58830_e91719_d_n2, assign58830_e91719_d_n4, assign58830_e91719_d_n5, assign58830_e91719_d_n6, assign58830_e91719_d_n7, assign58830_e91719_d_n8, assign58830_e91719_d_n9, assign58830_e91719_d_n10, assign58830_e91719_d_n11, assign58830_e91719_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58830_e91717: f64 = (locals.var_qn0 * locals.var_cox_inv);
        (assign58830_e91717, ((locals.var_qn0_dn0 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn2)), ((locals.var_qn0_dn4 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn4)), ((locals.var_qn0_dn5 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn5)), ((locals.var_qn0_dn6 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn7)), ((locals.var_qn0_dn8 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn8)), ((locals.var_qn0_dn9 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn9)), ((locals.var_qn0_dn10 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn10)), ((locals.var_qn0_dn11 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn11)), ((locals.var_qn0_dn14 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn4, locals.var_vgvt_dn5, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn8, locals.var_vgvt_dn9, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn14,)
    }
};
        locals.var_vgvt = assign58830_e91719;
        locals.var_vgvt_dn0 = assign58830_e91719_d_n0;
        locals.var_vgvt_dn2 = assign58830_e91719_d_n2;
        locals.var_vgvt_dn4 = assign58830_e91719_d_n4;
        locals.var_vgvt_dn5 = assign58830_e91719_d_n5;
        locals.var_vgvt_dn6 = assign58830_e91719_d_n6;
        locals.var_vgvt_dn7 = assign58830_e91719_d_n7;
        locals.var_vgvt_dn8 = assign58830_e91719_d_n8;
        locals.var_vgvt_dn9 = assign58830_e91719_d_n9;
        locals.var_vgvt_dn10 = assign58830_e91719_d_n10;
        locals.var_vgvt_dn11 = assign58830_e91719_d_n11;
        locals.var_vgvt_dn14 = assign58830_e91719_d_n14;
        locals.var_vgvt_rv = 0.0;

        let (assign58840_e91726, assign58840_e91726_d_n0, assign58840_e91726_d_n2, assign58840_e91726_d_n4, assign58840_e91726_d_n5, assign58840_e91726_d_n6, assign58840_e91726_d_n7, assign58840_e91726_d_n8, assign58840_e91726_d_n9, assign58840_e91726_d_n10, assign58840_e91726_d_n11, assign58840_e91726_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    }
};
        locals.var_vdsorg = assign58840_e91726;
        locals.var_vdsorg_dn0 = assign58840_e91726_d_n0;
        locals.var_vdsorg_dn2 = assign58840_e91726_d_n2;
        locals.var_vdsorg_dn4 = assign58840_e91726_d_n4;
        locals.var_vdsorg_dn5 = assign58840_e91726_d_n5;
        locals.var_vdsorg_dn6 = assign58840_e91726_d_n6;
        locals.var_vdsorg_dn7 = assign58840_e91726_d_n7;
        locals.var_vdsorg_dn8 = assign58840_e91726_d_n8;
        locals.var_vdsorg_dn9 = assign58840_e91726_d_n9;
        locals.var_vdsorg_dn10 = assign58840_e91726_d_n10;
        locals.var_vdsorg_dn11 = assign58840_e91726_d_n11;
        locals.var_vdsorg_dn14 = assign58840_e91726_d_n14;
        locals.var_vdsorg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_217(
        locals: &mut StampLocals,
    ) {
        let (assign58850_e91737, assign58850_e91737_d_n0, assign58850_e91737_d_n2, assign58850_e91737_d_n4, assign58850_e91737_d_n5, assign58850_e91737_d_n6, assign58850_e91737_d_n7, assign58850_e91737_d_n8, assign58850_e91737_d_n9, assign58850_e91737_d_n10, assign58850_e91737_d_n11, assign58850_e91737_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58850_e91734: f64 = (locals.var_cox * locals.var_cox);
        let assign58850_e91735: f64 = (locals.var_qnsub_esi / assign58850_e91734);
        (assign58850_e91735, (((locals.var_qnsub_esi_dn0 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn2 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn4 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn5 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn6 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn7 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn8 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn9 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn10 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn11 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn14 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)))) / (assign58850_e91734 * assign58850_e91734)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58850_e91737;
        locals.var_t2_dn0 = assign58850_e91737_d_n0;
        locals.var_t2_dn2 = assign58850_e91737_d_n2;
        locals.var_t2_dn4 = assign58850_e91737_d_n4;
        locals.var_t2_dn5 = assign58850_e91737_d_n5;
        locals.var_t2_dn6 = assign58850_e91737_d_n6;
        locals.var_t2_dn7 = assign58850_e91737_d_n7;
        locals.var_t2_dn8 = assign58850_e91737_d_n8;
        locals.var_t2_dn9 = assign58850_e91737_d_n9;
        locals.var_t2_dn10 = assign58850_e91737_d_n10;
        locals.var_t2_dn11 = assign58850_e91737_d_n11;
        locals.var_t2_dn14 = assign58850_e91737_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58860_e91748, assign58860_e91748_d_n0, assign58860_e91748_d_n2, assign58860_e91748_d_n4, assign58860_e91748_d_n5, assign58860_e91748_d_n6, assign58860_e91748_d_n7, assign58860_e91748_d_n8, assign58860_e91748_d_n9, assign58860_e91748_d_n10, assign58860_e91748_d_n11, assign58860_e91748_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58860_e91744: f64 = (locals.var_vgp - locals.var_beta_inv);
        let assign58860_e91746: f64 = (assign58860_e91744 - locals.var_vbsz__blk442);
        (assign58860_e91746, ((locals.var_vgp_dn0 - locals.var_beta_inv_dn0) - locals.var_vbsz__blk442_dn0), ((locals.var_vgp_dn2 - locals.var_beta_inv_dn2) - locals.var_vbsz__blk442_dn2), ((locals.var_vgp_dn4 - locals.var_beta_inv_dn4) - locals.var_vbsz__blk442_dn4), ((locals.var_vgp_dn5 - locals.var_beta_inv_dn5) - locals.var_vbsz__blk442_dn5), ((locals.var_vgp_dn6 - locals.var_beta_inv_dn6) - locals.var_vbsz__blk442_dn6), ((locals.var_vgp_dn7 - locals.var_beta_inv_dn7) - locals.var_vbsz__blk442_dn7), ((locals.var_vgp_dn8 - locals.var_beta_inv_dn8) - locals.var_vbsz__blk442_dn8), ((locals.var_vgp_dn9 - locals.var_beta_inv_dn9) - locals.var_vbsz__blk442_dn9), ((locals.var_vgp_dn10 - locals.var_beta_inv_dn10) - locals.var_vbsz__blk442_dn10), ((locals.var_vgp_dn11 - locals.var_beta_inv_dn11) - locals.var_vbsz__blk442_dn11), ((locals.var_vgp_dn14 - locals.var_beta_inv_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign58860_e91748;
        locals.var_t0_dn0 = assign58860_e91748_d_n0;
        locals.var_t0_dn2 = assign58860_e91748_d_n2;
        locals.var_t0_dn4 = assign58860_e91748_d_n4;
        locals.var_t0_dn5 = assign58860_e91748_d_n5;
        locals.var_t0_dn6 = assign58860_e91748_d_n6;
        locals.var_t0_dn7 = assign58860_e91748_d_n7;
        locals.var_t0_dn8 = assign58860_e91748_d_n8;
        locals.var_t0_dn9 = assign58860_e91748_d_n9;
        locals.var_t0_dn10 = assign58860_e91748_d_n10;
        locals.var_t0_dn11 = assign58860_e91748_d_n11;
        locals.var_t0_dn14 = assign58860_e91748_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign58870_e91761, assign58870_e91761_d_n0, assign58870_e91761_d_n2, assign58870_e91761_d_n4, assign58870_e91761_d_n5, assign58870_e91761_d_n6, assign58870_e91761_d_n7, assign58870_e91761_d_n8, assign58870_e91761_d_n9, assign58870_e91761_d_n10, assign58870_e91761_d_n11, assign58870_e91761_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58870_e91756: f64 = (2.0 / locals.var_t2);
        let assign58870_e91758: f64 = (assign58870_e91756 * locals.var_t0);
        let assign58870_e91759: f64 = (1.0 + assign58870_e91758);
        (assign58870_e91759, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn11)), (((-((2.0 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58870_e91761;
        locals.var_t1_dn0 = assign58870_e91761_d_n0;
        locals.var_t1_dn2 = assign58870_e91761_d_n2;
        locals.var_t1_dn4 = assign58870_e91761_d_n4;
        locals.var_t1_dn5 = assign58870_e91761_d_n5;
        locals.var_t1_dn6 = assign58870_e91761_d_n6;
        locals.var_t1_dn7 = assign58870_e91761_d_n7;
        locals.var_t1_dn8 = assign58870_e91761_d_n8;
        locals.var_t1_dn9 = assign58870_e91761_d_n9;
        locals.var_t1_dn10 = assign58870_e91761_d_n10;
        locals.var_t1_dn11 = assign58870_e91761_d_n11;
        locals.var_t1_dn14 = assign58870_e91761_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58880_e91777, assign58880_e91777_d_n0, assign58880_e91777_d_n2, assign58880_e91777_d_n4, assign58880_e91777_d_n5, assign58880_e91777_d_n6, assign58880_e91777_d_n7, assign58880_e91777_d_n8, assign58880_e91777_d_n9, assign58880_e91777_d_n10, assign58880_e91777_d_n11, assign58880_e91777_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58880_e91768: f64 = (locals.var_t1 * locals.var_t1);
        let assign58880_e91771: f64 = (4.0 * 0.001);
        let assign58880_e91773: f64 = (assign58880_e91771 * 0.001);
        let assign58880_e91774: f64 = (assign58880_e91768 + assign58880_e91773);
        let assign58880_e91775: f64 = (assign58880_e91774).sqrt();
        (assign58880_e91775, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign58880_e91775)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58880_e91777;
        locals.var_tmf2_dn0 = assign58880_e91777_d_n0;
        locals.var_tmf2_dn2 = assign58880_e91777_d_n2;
        locals.var_tmf2_dn4 = assign58880_e91777_d_n4;
        locals.var_tmf2_dn5 = assign58880_e91777_d_n5;
        locals.var_tmf2_dn6 = assign58880_e91777_d_n6;
        locals.var_tmf2_dn7 = assign58880_e91777_d_n7;
        locals.var_tmf2_dn8 = assign58880_e91777_d_n8;
        locals.var_tmf2_dn9 = assign58880_e91777_d_n9;
        locals.var_tmf2_dn10 = assign58880_e91777_d_n10;
        locals.var_tmf2_dn11 = assign58880_e91777_d_n11;
        locals.var_tmf2_dn14 = assign58880_e91777_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58890_e91790, assign58890_e91790_d_n0, assign58890_e91790_d_n2, assign58890_e91790_d_n4, assign58890_e91790_d_n5, assign58890_e91790_d_n6, assign58890_e91790_d_n7, assign58890_e91790_d_n8, assign58890_e91790_d_n9, assign58890_e91790_d_n10, assign58890_e91790_d_n11, assign58890_e91790_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58890_e91786: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign58890_e91787: f64 = (1.0 + assign58890_e91786);
        let assign58890_e91788: f64 = (0.5 * assign58890_e91787);
        (assign58890_e91788, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58890_e91790;
        locals.var_tx_dn0 = assign58890_e91790_d_n0;
        locals.var_tx_dn2 = assign58890_e91790_d_n2;
        locals.var_tx_dn4 = assign58890_e91790_d_n4;
        locals.var_tx_dn5 = assign58890_e91790_d_n5;
        locals.var_tx_dn6 = assign58890_e91790_d_n6;
        locals.var_tx_dn7 = assign58890_e91790_d_n7;
        locals.var_tx_dn8 = assign58890_e91790_d_n8;
        locals.var_tx_dn9 = assign58890_e91790_d_n9;
        locals.var_tx_dn10 = assign58890_e91790_d_n10;
        locals.var_tx_dn11 = assign58890_e91790_d_n11;
        locals.var_tx_dn14 = assign58890_e91790_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58900_e91801, assign58900_e91801_d_n0, assign58900_e91801_d_n2, assign58900_e91801_d_n4, assign58900_e91801_d_n5, assign58900_e91801_d_n6, assign58900_e91801_d_n7, assign58900_e91801_d_n8, assign58900_e91801_d_n9, assign58900_e91801_d_n10, assign58900_e91801_d_n11, assign58900_e91801_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58900_e91798: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign58900_e91799: f64 = (0.5 * assign58900_e91798);
        (assign58900_e91799, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign58900_e91801;
        locals.var_t9_dn0 = assign58900_e91801_d_n0;
        locals.var_t9_dn2 = assign58900_e91801_d_n2;
        locals.var_t9_dn4 = assign58900_e91801_d_n4;
        locals.var_t9_dn5 = assign58900_e91801_d_n5;
        locals.var_t9_dn6 = assign58900_e91801_d_n6;
        locals.var_t9_dn7 = assign58900_e91801_d_n7;
        locals.var_t9_dn8 = assign58900_e91801_d_n8;
        locals.var_t9_dn9 = assign58900_e91801_d_n9;
        locals.var_t9_dn10 = assign58900_e91801_d_n10;
        locals.var_t9_dn11 = assign58900_e91801_d_n11;
        locals.var_t9_dn14 = assign58900_e91801_d_n14;
        locals.var_t9_rv = 0.0;

        let assign58910_e91804: f64 = if locals.var_t9 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1449 = assign58910_e91804;
        locals.var_guard1449_rv = 0.0;

        let (assign58920_e91813, assign58920_e91813_d_n0, assign58920_e91813_d_n2, assign58920_e91813_d_n4, assign58920_e91813_d_n5, assign58920_e91813_d_n6, assign58920_e91813_d_n7, assign58920_e91813_d_n8, assign58920_e91813_d_n9, assign58920_e91813_d_n10, assign58920_e91813_d_n11, assign58920_e91813_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1449 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign58920_e91813;
        locals.var_t9_dn0 = assign58920_e91813_d_n0;
        locals.var_t9_dn2 = assign58920_e91813_d_n2;
        locals.var_t9_dn4 = assign58920_e91813_d_n4;
        locals.var_t9_dn5 = assign58920_e91813_d_n5;
        locals.var_t9_dn6 = assign58920_e91813_d_n6;
        locals.var_t9_dn7 = assign58920_e91813_d_n7;
        locals.var_t9_dn8 = assign58920_e91813_d_n8;
        locals.var_t9_dn9 = assign58920_e91813_d_n9;
        locals.var_t9_dn10 = assign58920_e91813_d_n10;
        locals.var_t9_dn11 = assign58920_e91813_d_n11;
        locals.var_t9_dn14 = assign58920_e91813_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign58930_e91822, assign58930_e91822_d_n0, assign58930_e91822_d_n2, assign58930_e91822_d_n4, assign58930_e91822_d_n5, assign58930_e91822_d_n6, assign58930_e91822_d_n7, assign58930_e91822_d_n8, assign58930_e91822_d_n9, assign58930_e91822_d_n10, assign58930_e91822_d_n11, assign58930_e91822_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1449 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58930_e91822;
        locals.var_tx_dn0 = assign58930_e91822_d_n0;
        locals.var_tx_dn2 = assign58930_e91822_d_n2;
        locals.var_tx_dn4 = assign58930_e91822_d_n4;
        locals.var_tx_dn5 = assign58930_e91822_d_n5;
        locals.var_tx_dn6 = assign58930_e91822_d_n6;
        locals.var_tx_dn7 = assign58930_e91822_d_n7;
        locals.var_tx_dn8 = assign58930_e91822_d_n8;
        locals.var_tx_dn9 = assign58930_e91822_d_n9;
        locals.var_tx_dn10 = assign58930_e91822_d_n10;
        locals.var_tx_dn11 = assign58930_e91822_d_n11;
        locals.var_tx_dn14 = assign58930_e91822_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58940_e91831, assign58940_e91831_d_n0, assign58940_e91831_d_n2, assign58940_e91831_d_n4, assign58940_e91831_d_n5, assign58940_e91831_d_n6, assign58940_e91831_d_n7, assign58940_e91831_d_n8, assign58940_e91831_d_n9, assign58940_e91831_d_n10, assign58940_e91831_d_n11, assign58940_e91831_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58940_e91829: f64 = (locals.var_t9 + 1e-25);
        (assign58940_e91829, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign58940_e91831;
        locals.var_t9_dn0 = assign58940_e91831_d_n0;
        locals.var_t9_dn2 = assign58940_e91831_d_n2;
        locals.var_t9_dn4 = assign58940_e91831_d_n4;
        locals.var_t9_dn5 = assign58940_e91831_d_n5;
        locals.var_t9_dn6 = assign58940_e91831_d_n6;
        locals.var_t9_dn7 = assign58940_e91831_d_n7;
        locals.var_t9_dn8 = assign58940_e91831_d_n8;
        locals.var_t9_dn9 = assign58940_e91831_d_n9;
        locals.var_t9_dn10 = assign58940_e91831_d_n10;
        locals.var_t9_dn11 = assign58940_e91831_d_n11;
        locals.var_t9_dn14 = assign58940_e91831_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign58950_e91839, assign58950_e91839_d_n0, assign58950_e91839_d_n2, assign58950_e91839_d_n4, assign58950_e91839_d_n5, assign58950_e91839_d_n6, assign58950_e91839_d_n7, assign58950_e91839_d_n8, assign58950_e91839_d_n9, assign58950_e91839_d_n10, assign58950_e91839_d_n11, assign58950_e91839_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58950_e91837: f64 = (locals.var_t9).sqrt();
        (assign58950_e91837, (locals.var_t9_dn0 / (2.0 * assign58950_e91837)), (locals.var_t9_dn2 / (2.0 * assign58950_e91837)), (locals.var_t9_dn4 / (2.0 * assign58950_e91837)), (locals.var_t9_dn5 / (2.0 * assign58950_e91837)), (locals.var_t9_dn6 / (2.0 * assign58950_e91837)), (locals.var_t9_dn7 / (2.0 * assign58950_e91837)), (locals.var_t9_dn8 / (2.0 * assign58950_e91837)), (locals.var_t9_dn9 / (2.0 * assign58950_e91837)), (locals.var_t9_dn10 / (2.0 * assign58950_e91837)), (locals.var_t9_dn11 / (2.0 * assign58950_e91837)), (locals.var_t9_dn14 / (2.0 * assign58950_e91837)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign58950_e91839;
        locals.var_t3_dn0 = assign58950_e91839_d_n0;
        locals.var_t3_dn2 = assign58950_e91839_d_n2;
        locals.var_t3_dn4 = assign58950_e91839_d_n4;
        locals.var_t3_dn5 = assign58950_e91839_d_n5;
        locals.var_t3_dn6 = assign58950_e91839_d_n6;
        locals.var_t3_dn7 = assign58950_e91839_d_n7;
        locals.var_t3_dn8 = assign58950_e91839_d_n8;
        locals.var_t3_dn9 = assign58950_e91839_d_n9;
        locals.var_t3_dn10 = assign58950_e91839_d_n10;
        locals.var_t3_dn11 = assign58950_e91839_d_n11;
        locals.var_t3_dn14 = assign58950_e91839_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign58960_e91852, assign58960_e91852_d_n0, assign58960_e91852_d_n2, assign58960_e91852_d_n4, assign58960_e91852_d_n5, assign58960_e91852_d_n6, assign58960_e91852_d_n7, assign58960_e91852_d_n8, assign58960_e91852_d_n9, assign58960_e91852_d_n10, assign58960_e91852_d_n11, assign58960_e91852_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58960_e91848: f64 = (1.0 - locals.var_t3);
        let assign58960_e91849: f64 = (locals.var_t2 * assign58960_e91848);
        let assign58960_e91850: f64 = (locals.var_vgp + assign58960_e91849);
        (assign58960_e91850, (locals.var_vgp_dn0 + ((locals.var_t2_dn0 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2_dn2 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_t2_dn4 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_t2_dn5 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_t2_dn6 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2_dn7 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_t2_dn8 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_t2_dn9 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_t2_dn10 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2_dn11 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn11)))), (locals.var_vgp_dn14 + ((locals.var_t2_dn14 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn14)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign58960_e91852;
        locals.var_t10_dn0 = assign58960_e91852_d_n0;
        locals.var_t10_dn2 = assign58960_e91852_d_n2;
        locals.var_t10_dn4 = assign58960_e91852_d_n4;
        locals.var_t10_dn5 = assign58960_e91852_d_n5;
        locals.var_t10_dn6 = assign58960_e91852_d_n6;
        locals.var_t10_dn7 = assign58960_e91852_d_n7;
        locals.var_t10_dn8 = assign58960_e91852_d_n8;
        locals.var_t10_dn9 = assign58960_e91852_d_n9;
        locals.var_t10_dn10 = assign58960_e91852_d_n10;
        locals.var_t10_dn11 = assign58960_e91852_d_n11;
        locals.var_t10_dn14 = assign58960_e91852_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign58970_e91868, assign58970_e91868_d_n0, assign58970_e91868_d_n2, assign58970_e91868_d_n4, assign58970_e91868_d_n5, assign58970_e91868_d_n6, assign58970_e91868_d_n7, assign58970_e91868_d_n8, assign58970_e91868_d_n9, assign58970_e91868_d_n10, assign58970_e91868_d_n11, assign58970_e91868_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58970_e91859: f64 = (locals.var_t10 * locals.var_t10);
        let assign58970_e91862: f64 = (4.0 * 0.01);
        let assign58970_e91864: f64 = (assign58970_e91862 * 0.01);
        let assign58970_e91865: f64 = (assign58970_e91859 + assign58970_e91864);
        let assign58970_e91866: f64 = (assign58970_e91865).sqrt();
        (assign58970_e91866, (((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)) / (2.0 * assign58970_e91866)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58970_e91868;
        locals.var_tmf2_dn0 = assign58970_e91868_d_n0;
        locals.var_tmf2_dn2 = assign58970_e91868_d_n2;
        locals.var_tmf2_dn4 = assign58970_e91868_d_n4;
        locals.var_tmf2_dn5 = assign58970_e91868_d_n5;
        locals.var_tmf2_dn6 = assign58970_e91868_d_n6;
        locals.var_tmf2_dn7 = assign58970_e91868_d_n7;
        locals.var_tmf2_dn8 = assign58970_e91868_d_n8;
        locals.var_tmf2_dn9 = assign58970_e91868_d_n9;
        locals.var_tmf2_dn10 = assign58970_e91868_d_n10;
        locals.var_tmf2_dn11 = assign58970_e91868_d_n11;
        locals.var_tmf2_dn14 = assign58970_e91868_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58980_e91881, assign58980_e91881_d_n0, assign58980_e91881_d_n2, assign58980_e91881_d_n4, assign58980_e91881_d_n5, assign58980_e91881_d_n6, assign58980_e91881_d_n7, assign58980_e91881_d_n8, assign58980_e91881_d_n9, assign58980_e91881_d_n10, assign58980_e91881_d_n11, assign58980_e91881_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58980_e91877: f64 = (locals.var_t10 / locals.var_tmf2);
        let assign58980_e91878: f64 = (1.0 + assign58980_e91877);
        let assign58980_e91879: f64 = (0.5 * assign58980_e91878);
        (assign58980_e91879, (0.5 * (((locals.var_t10_dn0 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn2 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn4 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn5 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn6 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn7 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn8 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn9 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn10 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn11 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn14 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign58980_e91881;
        locals.var_t0_dn0 = assign58980_e91881_d_n0;
        locals.var_t0_dn2 = assign58980_e91881_d_n2;
        locals.var_t0_dn4 = assign58980_e91881_d_n4;
        locals.var_t0_dn5 = assign58980_e91881_d_n5;
        locals.var_t0_dn6 = assign58980_e91881_d_n6;
        locals.var_t0_dn7 = assign58980_e91881_d_n7;
        locals.var_t0_dn8 = assign58980_e91881_d_n8;
        locals.var_t0_dn9 = assign58980_e91881_d_n9;
        locals.var_t0_dn10 = assign58980_e91881_d_n10;
        locals.var_t0_dn11 = assign58980_e91881_d_n11;
        locals.var_t0_dn14 = assign58980_e91881_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign58990_e91892, assign58990_e91892_d_n0, assign58990_e91892_d_n2, assign58990_e91892_d_n4, assign58990_e91892_d_n5, assign58990_e91892_d_n6, assign58990_e91892_d_n7, assign58990_e91892_d_n8, assign58990_e91892_d_n9, assign58990_e91892_d_n10, assign58990_e91892_d_n11, assign58990_e91892_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58990_e91889: f64 = (locals.var_t10 + locals.var_tmf2);
        let assign58990_e91890: f64 = (0.5 * assign58990_e91889);
        (assign58990_e91890, (0.5 * (locals.var_t10_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t10_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t10_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t10_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t10_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t10_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t10_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t10_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t10_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t10_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t10_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign58990_e91892;
        locals.var_t10_dn0 = assign58990_e91892_d_n0;
        locals.var_t10_dn2 = assign58990_e91892_d_n2;
        locals.var_t10_dn4 = assign58990_e91892_d_n4;
        locals.var_t10_dn5 = assign58990_e91892_d_n5;
        locals.var_t10_dn6 = assign58990_e91892_d_n6;
        locals.var_t10_dn7 = assign58990_e91892_d_n7;
        locals.var_t10_dn8 = assign58990_e91892_d_n8;
        locals.var_t10_dn9 = assign58990_e91892_d_n9;
        locals.var_t10_dn10 = assign58990_e91892_d_n10;
        locals.var_t10_dn11 = assign58990_e91892_d_n11;
        locals.var_t10_dn14 = assign58990_e91892_d_n14;
        locals.var_t10_rv = 0.0;

        let assign59000_e91895: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1450 = assign59000_e91895;
        locals.var_guard1450_rv = 0.0;

        let (assign59010_e91904, assign59010_e91904_d_n0, assign59010_e91904_d_n2, assign59010_e91904_d_n4, assign59010_e91904_d_n5, assign59010_e91904_d_n6, assign59010_e91904_d_n7, assign59010_e91904_d_n8, assign59010_e91904_d_n9, assign59010_e91904_d_n10, assign59010_e91904_d_n11, assign59010_e91904_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign59010_e91904;
        locals.var_t10_dn0 = assign59010_e91904_d_n0;
        locals.var_t10_dn2 = assign59010_e91904_d_n2;
        locals.var_t10_dn4 = assign59010_e91904_d_n4;
        locals.var_t10_dn5 = assign59010_e91904_d_n5;
        locals.var_t10_dn6 = assign59010_e91904_d_n6;
        locals.var_t10_dn7 = assign59010_e91904_d_n7;
        locals.var_t10_dn8 = assign59010_e91904_d_n8;
        locals.var_t10_dn9 = assign59010_e91904_d_n9;
        locals.var_t10_dn10 = assign59010_e91904_d_n10;
        locals.var_t10_dn11 = assign59010_e91904_d_n11;
        locals.var_t10_dn14 = assign59010_e91904_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign59020_e91913, assign59020_e91913_d_n0, assign59020_e91913_d_n2, assign59020_e91913_d_n4, assign59020_e91913_d_n5, assign59020_e91913_d_n6, assign59020_e91913_d_n7, assign59020_e91913_d_n8, assign59020_e91913_d_n9, assign59020_e91913_d_n10, assign59020_e91913_d_n11, assign59020_e91913_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign59020_e91913;
        locals.var_t0_dn0 = assign59020_e91913_d_n0;
        locals.var_t0_dn2 = assign59020_e91913_d_n2;
        locals.var_t0_dn4 = assign59020_e91913_d_n4;
        locals.var_t0_dn5 = assign59020_e91913_d_n5;
        locals.var_t0_dn6 = assign59020_e91913_d_n6;
        locals.var_t0_dn7 = assign59020_e91913_d_n7;
        locals.var_t0_dn8 = assign59020_e91913_d_n8;
        locals.var_t0_dn9 = assign59020_e91913_d_n9;
        locals.var_t0_dn10 = assign59020_e91913_d_n10;
        locals.var_t0_dn11 = assign59020_e91913_d_n11;
        locals.var_t0_dn14 = assign59020_e91913_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign59030_e91924, assign59030_e91924_d_n0, assign59030_e91924_d_n2, assign59030_e91924_d_n4, assign59030_e91924_d_n5, assign59030_e91924_d_n6, assign59030_e91924_d_n7, assign59030_e91924_d_n8, assign59030_e91924_d_n9, assign59030_e91924_d_n10, assign59030_e91924_d_n11, assign59030_e91924_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59030_e91921: f64 = (10.0 * 2.220446049250313e-16);
        let assign59030_e91922: f64 = (locals.var_t10 + assign59030_e91921);
        (assign59030_e91922, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign59030_e91924;
        locals.var_t10_dn0 = assign59030_e91924_d_n0;
        locals.var_t10_dn2 = assign59030_e91924_d_n2;
        locals.var_t10_dn4 = assign59030_e91924_d_n4;
        locals.var_t10_dn5 = assign59030_e91924_d_n5;
        locals.var_t10_dn6 = assign59030_e91924_d_n6;
        locals.var_t10_dn7 = assign59030_e91924_d_n7;
        locals.var_t10_dn8 = assign59030_e91924_d_n8;
        locals.var_t10_dn9 = assign59030_e91924_d_n9;
        locals.var_t10_dn10 = assign59030_e91924_d_n10;
        locals.var_t10_dn11 = assign59030_e91924_d_n11;
        locals.var_t10_dn14 = assign59030_e91924_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign59040_e91933, assign59040_e91933_d_n0, assign59040_e91933_d_n2, assign59040_e91933_d_n4, assign59040_e91933_d_n5, assign59040_e91933_d_n6, assign59040_e91933_d_n7, assign59040_e91933_d_n8, assign59040_e91933_d_n9, assign59040_e91933_d_n10, assign59040_e91933_d_n11, assign59040_e91933_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59040_e91931: f64 = (locals.var_vds / locals.var_t10);
        (assign59040_e91931, (((locals.var_vds_dn0 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn2 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn4 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn5 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn6 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn7 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn8 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn9 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn10 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn11 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn14 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59040_e91933;
        locals.var_t1_dn0 = assign59040_e91933_d_n0;
        locals.var_t1_dn2 = assign59040_e91933_d_n2;
        locals.var_t1_dn4 = assign59040_e91933_d_n4;
        locals.var_t1_dn5 = assign59040_e91933_d_n5;
        locals.var_t1_dn6 = assign59040_e91933_d_n6;
        locals.var_t1_dn7 = assign59040_e91933_d_n7;
        locals.var_t1_dn8 = assign59040_e91933_d_n8;
        locals.var_t1_dn9 = assign59040_e91933_d_n9;
        locals.var_t1_dn10 = assign59040_e91933_d_n10;
        locals.var_t1_dn11 = assign59040_e91933_d_n11;
        locals.var_t1_dn14 = assign59040_e91933_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign59050_e91949, assign59050_e91949_d_n0, assign59050_e91949_d_n2, assign59050_e91949_d_n4, assign59050_e91949_d_n5, assign59050_e91949_d_n6, assign59050_e91949_d_n7, assign59050_e91949_d_n8, assign59050_e91949_d_n9, assign59050_e91949_d_n10, assign59050_e91949_d_n11, assign59050_e91949_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let (assign59050_e91947, assign59050_e91947_d_n0, assign59050_e91947_d_n2, assign59050_e91947_d_n4, assign59050_e91947_d_n5, assign59050_e91947_d_n6, assign59050_e91947_d_n7, assign59050_e91947_d_n8, assign59050_e91947_d_n9, assign59050_e91947_d_n10, assign59050_e91947_d_n11, assign59050_e91947_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59050_e91945: f64 = (locals.var_ddlte - 1.0);
                let assign59050_e91946: f64 = (locals.var_t1).powf(assign59050_e91945);
                (assign59050_e91946, if locals.var_ddlte_dn0 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn0)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn0 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_ddlte_dn2 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn2)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn2 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_ddlte_dn4 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn4)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn4 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_ddlte_dn5 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn5)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn5 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_ddlte_dn6 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn6)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn6 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_ddlte_dn7 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn7)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn7 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_ddlte_dn8 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn8)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn8 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_ddlte_dn9 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn9)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn9 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_ddlte_dn10 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn10)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn10 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_ddlte_dn11 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn11)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn11 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_ddlte_dn14 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn14)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn14 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn14 / locals.var_t1)))) },)
            }
        };
        (assign59050_e91947, assign59050_e91947_d_n0, assign59050_e91947_d_n2, assign59050_e91947_d_n4, assign59050_e91947_d_n5, assign59050_e91947_d_n6, assign59050_e91947_d_n7, assign59050_e91947_d_n8, assign59050_e91947_d_n9, assign59050_e91947_d_n10, assign59050_e91947_d_n11, assign59050_e91947_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign59050_e91949;
        locals.var_t2_dn0 = assign59050_e91949_d_n0;
        locals.var_t2_dn2 = assign59050_e91949_d_n2;
        locals.var_t2_dn4 = assign59050_e91949_d_n4;
        locals.var_t2_dn5 = assign59050_e91949_d_n5;
        locals.var_t2_dn6 = assign59050_e91949_d_n6;
        locals.var_t2_dn7 = assign59050_e91949_d_n7;
        locals.var_t2_dn8 = assign59050_e91949_d_n8;
        locals.var_t2_dn9 = assign59050_e91949_d_n9;
        locals.var_t2_dn10 = assign59050_e91949_d_n10;
        locals.var_t2_dn11 = assign59050_e91949_d_n11;
        locals.var_t2_dn14 = assign59050_e91949_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign59060_e91960, assign59060_e91960_d_n0, assign59060_e91960_d_n2, assign59060_e91960_d_n4, assign59060_e91960_d_n5, assign59060_e91960_d_n6, assign59060_e91960_d_n7, assign59060_e91960_d_n8, assign59060_e91960_d_n9, assign59060_e91960_d_n10, assign59060_e91960_d_n11, assign59060_e91960_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59060_e91957: f64 = (locals.var_t2 * locals.var_t1);
        let assign59060_e91958: f64 = (1.0 + assign59060_e91957);
        (assign59060_e91958, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign59060_e91960;
        locals.var_t3_dn0 = assign59060_e91960_d_n0;
        locals.var_t3_dn2 = assign59060_e91960_d_n2;
        locals.var_t3_dn4 = assign59060_e91960_d_n4;
        locals.var_t3_dn5 = assign59060_e91960_d_n5;
        locals.var_t3_dn6 = assign59060_e91960_d_n6;
        locals.var_t3_dn7 = assign59060_e91960_d_n7;
        locals.var_t3_dn8 = assign59060_e91960_d_n8;
        locals.var_t3_dn9 = assign59060_e91960_d_n9;
        locals.var_t3_dn10 = assign59060_e91960_d_n10;
        locals.var_t3_dn11 = assign59060_e91960_d_n11;
        locals.var_t3_dn14 = assign59060_e91960_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign59070_e91978, assign59070_e91978_d_n0, assign59070_e91978_d_n2, assign59070_e91978_d_n4, assign59070_e91978_d_n5, assign59070_e91978_d_n6, assign59070_e91978_d_n7, assign59070_e91978_d_n8, assign59070_e91978_d_n9, assign59070_e91978_d_n10, assign59070_e91978_d_n11, assign59070_e91978_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let (assign59070_e91976, assign59070_e91976_d_n0, assign59070_e91976_d_n2, assign59070_e91976_d_n4, assign59070_e91976_d_n5, assign59070_e91976_d_n6, assign59070_e91976_d_n7, assign59070_e91976_d_n8, assign59070_e91976_d_n9, assign59070_e91976_d_n10, assign59070_e91976_d_n11, assign59070_e91976_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59070_e91972: f64 = (1.0 / locals.var_ddlte);
                let assign59070_e91974: f64 = (assign59070_e91972 - 1.0);
                let assign59070_e91975: f64 = (locals.var_t3).powf(assign59070_e91974);
                (assign59070_e91975, if (-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn0)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn0 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn2)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn2 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn4)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn4 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn5)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn5 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn6)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn6 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn7)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn7 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn8)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn8 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn9)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn9 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn10)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn10 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn11)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn11 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn14)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn14 / locals.var_t3)))) },)
            }
        };
        (assign59070_e91976, assign59070_e91976_d_n0, assign59070_e91976_d_n2, assign59070_e91976_d_n4, assign59070_e91976_d_n5, assign59070_e91976_d_n6, assign59070_e91976_d_n7, assign59070_e91976_d_n8, assign59070_e91976_d_n9, assign59070_e91976_d_n10, assign59070_e91976_d_n11, assign59070_e91976_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign59070_e91978;
        locals.var_t4_dn0 = assign59070_e91978_d_n0;
        locals.var_t4_dn2 = assign59070_e91978_d_n2;
        locals.var_t4_dn4 = assign59070_e91978_d_n4;
        locals.var_t4_dn5 = assign59070_e91978_d_n5;
        locals.var_t4_dn6 = assign59070_e91978_d_n6;
        locals.var_t4_dn7 = assign59070_e91978_d_n7;
        locals.var_t4_dn8 = assign59070_e91978_d_n8;
        locals.var_t4_dn9 = assign59070_e91978_d_n9;
        locals.var_t4_dn10 = assign59070_e91978_d_n10;
        locals.var_t4_dn11 = assign59070_e91978_d_n11;
        locals.var_t4_dn14 = assign59070_e91978_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_218(
        locals: &mut StampLocals,
    ) {
        let (assign59080_e91987, assign59080_e91987_d_n0, assign59080_e91987_d_n2, assign59080_e91987_d_n4, assign59080_e91987_d_n5, assign59080_e91987_d_n6, assign59080_e91987_d_n7, assign59080_e91987_d_n8, assign59080_e91987_d_n9, assign59080_e91987_d_n10, assign59080_e91987_d_n11, assign59080_e91987_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59080_e91985: f64 = (locals.var_t4 * locals.var_t3);
        (assign59080_e91985, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign59080_e91987;
        locals.var_t6_dn0 = assign59080_e91987_d_n0;
        locals.var_t6_dn2 = assign59080_e91987_d_n2;
        locals.var_t6_dn4 = assign59080_e91987_d_n4;
        locals.var_t6_dn5 = assign59080_e91987_d_n5;
        locals.var_t6_dn6 = assign59080_e91987_d_n6;
        locals.var_t6_dn7 = assign59080_e91987_d_n7;
        locals.var_t6_dn8 = assign59080_e91987_d_n8;
        locals.var_t6_dn9 = assign59080_e91987_d_n9;
        locals.var_t6_dn10 = assign59080_e91987_d_n10;
        locals.var_t6_dn11 = assign59080_e91987_d_n11;
        locals.var_t6_dn14 = assign59080_e91987_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign59090_e91996, assign59090_e91996_d_n0, assign59090_e91996_d_n2, assign59090_e91996_d_n4, assign59090_e91996_d_n5, assign59090_e91996_d_n6, assign59090_e91996_d_n7, assign59090_e91996_d_n8, assign59090_e91996_d_n9, assign59090_e91996_d_n10, assign59090_e91996_d_n11, assign59090_e91996_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59090_e91994: f64 = (locals.var_vds / locals.var_t6);
        (assign59090_e91994, (((locals.var_vds_dn0 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn2 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn4 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn5 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn6 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn7 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn8 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn9 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn10 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn11 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn14 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign59090_e91996;
        locals.var_vdseff_dn0 = assign59090_e91996_d_n0;
        locals.var_vdseff_dn2 = assign59090_e91996_d_n2;
        locals.var_vdseff_dn4 = assign59090_e91996_d_n4;
        locals.var_vdseff_dn5 = assign59090_e91996_d_n5;
        locals.var_vdseff_dn6 = assign59090_e91996_d_n6;
        locals.var_vdseff_dn7 = assign59090_e91996_d_n7;
        locals.var_vdseff_dn8 = assign59090_e91996_d_n8;
        locals.var_vdseff_dn9 = assign59090_e91996_d_n9;
        locals.var_vdseff_dn10 = assign59090_e91996_d_n10;
        locals.var_vdseff_dn11 = assign59090_e91996_d_n11;
        locals.var_vdseff_dn14 = assign59090_e91996_d_n14;
        locals.var_vdseff_rv = 0.0;

        let (assign59100_e92003, assign59100_e92003_d_n0, assign59100_e92003_d_n2, assign59100_e92003_d_n4, assign59100_e92003_d_n5, assign59100_e92003_d_n6, assign59100_e92003_d_n7, assign59100_e92003_d_n8, assign59100_e92003_d_n9, assign59100_e92003_d_n10, assign59100_e92003_d_n11, assign59100_e92003_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign59100_e92003;
        locals.var_vds_dn0 = assign59100_e92003_d_n0;
        locals.var_vds_dn2 = assign59100_e92003_d_n2;
        locals.var_vds_dn4 = assign59100_e92003_d_n4;
        locals.var_vds_dn5 = assign59100_e92003_d_n5;
        locals.var_vds_dn6 = assign59100_e92003_d_n6;
        locals.var_vds_dn7 = assign59100_e92003_d_n7;
        locals.var_vds_dn8 = assign59100_e92003_d_n8;
        locals.var_vds_dn9 = assign59100_e92003_d_n9;
        locals.var_vds_dn10 = assign59100_e92003_d_n10;
        locals.var_vds_dn11 = assign59100_e92003_d_n11;
        locals.var_vds_dn14 = assign59100_e92003_d_n14;
        locals.var_vds_rv = 0.0;

        let (assign59120_e92022, assign59120_e92022_d_n0, assign59120_e92022_d_n2, assign59120_e92022_d_n4, assign59120_e92022_d_n5, assign59120_e92022_d_n6, assign59120_e92022_d_n7, assign59120_e92022_d_n8, assign59120_e92022_d_n9, assign59120_e92022_d_n10, assign59120_e92022_d_n11, assign59120_e92022_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59120_e92018: f64 = (locals.var_vbscl__blk439 - locals.var_vds);
        let assign59120_e92019: f64 = (locals.var_beta * assign59120_e92018);
        let assign59120_e92020: f64 = (assign59120_e92019).exp();
        (assign59120_e92020, (assign59120_e92020 * ((locals.var_beta_dn0 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn0 - locals.var_vds_dn0)))), (assign59120_e92020 * ((locals.var_beta_dn2 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn2 - locals.var_vds_dn2)))), (assign59120_e92020 * ((locals.var_beta_dn4 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn4 - locals.var_vds_dn4)))), (assign59120_e92020 * ((locals.var_beta_dn5 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn5 - locals.var_vds_dn5)))), (assign59120_e92020 * ((locals.var_beta_dn6 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn6 - locals.var_vds_dn6)))), (assign59120_e92020 * ((locals.var_beta_dn7 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn7 - locals.var_vds_dn7)))), (assign59120_e92020 * ((locals.var_beta_dn8 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn8 - locals.var_vds_dn8)))), (assign59120_e92020 * ((locals.var_beta_dn9 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn9 - locals.var_vds_dn9)))), (assign59120_e92020 * ((locals.var_beta_dn10 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn10 - locals.var_vds_dn10)))), (assign59120_e92020 * ((locals.var_beta_dn11 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn11 - locals.var_vds_dn11)))), (assign59120_e92020 * ((locals.var_beta_dn14 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn14 - locals.var_vds_dn14)))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn4, locals.var_exp_bvbsvds_dn5, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn8, locals.var_exp_bvbsvds_dn9, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn14,)
    }
};
        locals.var_exp_bvbsvds = assign59120_e92022;
        locals.var_exp_bvbsvds_dn0 = assign59120_e92022_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign59120_e92022_d_n2;
        locals.var_exp_bvbsvds_dn4 = assign59120_e92022_d_n4;
        locals.var_exp_bvbsvds_dn5 = assign59120_e92022_d_n5;
        locals.var_exp_bvbsvds_dn6 = assign59120_e92022_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign59120_e92022_d_n7;
        locals.var_exp_bvbsvds_dn8 = assign59120_e92022_d_n8;
        locals.var_exp_bvbsvds_dn9 = assign59120_e92022_d_n9;
        locals.var_exp_bvbsvds_dn10 = assign59120_e92022_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign59120_e92022_d_n11;
        locals.var_exp_bvbsvds_dn14 = assign59120_e92022_d_n14;
        locals.var_exp_bvbsvds_rv = 0.0;

        let assign59130_e92025: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1451 = assign59130_e92025;
        locals.var_guard1451_rv = 0.0;

        let (assign59140_e92034, assign59140_e92034_d_n0, assign59140_e92034_d_n2, assign59140_e92034_d_n4, assign59140_e92034_d_n5, assign59140_e92034_d_n6, assign59140_e92034_d_n7, assign59140_e92034_d_n8, assign59140_e92034_d_n9, assign59140_e92034_d_n10, assign59140_e92034_d_n11, assign59140_e92034_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1451 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign59140_e92034;
        locals.var_pds_dn0 = assign59140_e92034_d_n0;
        locals.var_pds_dn2 = assign59140_e92034_d_n2;
        locals.var_pds_dn4 = assign59140_e92034_d_n4;
        locals.var_pds_dn5 = assign59140_e92034_d_n5;
        locals.var_pds_dn6 = assign59140_e92034_d_n6;
        locals.var_pds_dn7 = assign59140_e92034_d_n7;
        locals.var_pds_dn8 = assign59140_e92034_d_n8;
        locals.var_pds_dn9 = assign59140_e92034_d_n9;
        locals.var_pds_dn10 = assign59140_e92034_d_n10;
        locals.var_pds_dn11 = assign59140_e92034_d_n11;
        locals.var_pds_dn14 = assign59140_e92034_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign59150_e92043, assign59150_e92043_d_n0, assign59150_e92043_d_n2, assign59150_e92043_d_n4, assign59150_e92043_d_n5, assign59150_e92043_d_n6, assign59150_e92043_d_n7, assign59150_e92043_d_n8, assign59150_e92043_d_n9, assign59150_e92043_d_n10, assign59150_e92043_d_n11, assign59150_e92043_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1451 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign59150_e92043;
        locals.var_psl_dn0 = assign59150_e92043_d_n0;
        locals.var_psl_dn2 = assign59150_e92043_d_n2;
        locals.var_psl_dn4 = assign59150_e92043_d_n4;
        locals.var_psl_dn5 = assign59150_e92043_d_n5;
        locals.var_psl_dn6 = assign59150_e92043_d_n6;
        locals.var_psl_dn7 = assign59150_e92043_d_n7;
        locals.var_psl_dn8 = assign59150_e92043_d_n8;
        locals.var_psl_dn9 = assign59150_e92043_d_n9;
        locals.var_psl_dn10 = assign59150_e92043_d_n10;
        locals.var_psl_dn11 = assign59150_e92043_d_n11;
        locals.var_psl_dn14 = assign59150_e92043_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign59160_e92052,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1451 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_start_of_loopl,)
    }
};
        locals.var_start_of_loopl = assign59160_e92052;
        locals.var_start_of_loopl_rv = 0.0;

        let assign59170_e92055: f64 = if locals.var_start_of_loopl == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1452 = assign59170_e92055;
        locals.var_guard1452_rv = 0.0;

        let assign59180_e92058: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1453 = assign59180_e92058;
        locals.var_guard1453_rv = 0.0;

        let (assign59190_e92078, assign59190_e92078_d_n0, assign59190_e92078_d_n2, assign59190_e92078_d_n4, assign59190_e92078_d_n5, assign59190_e92078_d_n6, assign59190_e92078_d_n7, assign59190_e92078_d_n8, assign59190_e92078_d_n9, assign59190_e92078_d_n10, assign59190_e92078_d_n11, assign59190_e92078_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59190_e92069: f64 = (locals.var_psl_lim - locals.var_ps0);
        let (assign59190_e92076, assign59190_e92076_d_n0, assign59190_e92076_d_n2, assign59190_e92076_d_n4, assign59190_e92076_d_n5, assign59190_e92076_d_n6, assign59190_e92076_d_n7, assign59190_e92076_d_n8, assign59190_e92076_d_n9, assign59190_e92076_d_n10, assign59190_e92076_d_n11, assign59190_e92076_d_n14,) = {
            if (assign59190_e92069 >= 0.0) {
                let assign59190_e92074: f64 = (locals.var_psl_lim - locals.var_ps0);
                (assign59190_e92074, (locals.var_psl_lim_dn0 - locals.var_ps0_dn0), (locals.var_psl_lim_dn2 - locals.var_ps0_dn2), (locals.var_psl_lim_dn4 - locals.var_ps0_dn4), (locals.var_psl_lim_dn5 - locals.var_ps0_dn5), (locals.var_psl_lim_dn6 - locals.var_ps0_dn6), (locals.var_psl_lim_dn7 - locals.var_ps0_dn7), (locals.var_psl_lim_dn8 - locals.var_ps0_dn8), (locals.var_psl_lim_dn9 - locals.var_ps0_dn9), (locals.var_psl_lim_dn10 - locals.var_ps0_dn10), (locals.var_psl_lim_dn11 - locals.var_ps0_dn11), (locals.var_psl_lim_dn14 - locals.var_ps0_dn14),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign59190_e92076, assign59190_e92076_d_n0, assign59190_e92076_d_n2, assign59190_e92076_d_n4, assign59190_e92076_d_n5, assign59190_e92076_d_n6, assign59190_e92076_d_n7, assign59190_e92076_d_n8, assign59190_e92076_d_n9, assign59190_e92076_d_n10, assign59190_e92076_d_n11, assign59190_e92076_d_n14,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn4, locals.var_pds_max_dn5, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn8, locals.var_pds_max_dn9, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn14,)
    }
};
        locals.var_pds_max = assign59190_e92078;
        locals.var_pds_max_dn0 = assign59190_e92078_d_n0;
        locals.var_pds_max_dn2 = assign59190_e92078_d_n2;
        locals.var_pds_max_dn4 = assign59190_e92078_d_n4;
        locals.var_pds_max_dn5 = assign59190_e92078_d_n5;
        locals.var_pds_max_dn6 = assign59190_e92078_d_n6;
        locals.var_pds_max_dn7 = assign59190_e92078_d_n7;
        locals.var_pds_max_dn8 = assign59190_e92078_d_n8;
        locals.var_pds_max_dn9 = assign59190_e92078_d_n9;
        locals.var_pds_max_dn10 = assign59190_e92078_d_n10;
        locals.var_pds_max_dn11 = assign59190_e92078_d_n11;
        locals.var_pds_max_dn14 = assign59190_e92078_d_n14;
        locals.var_pds_max_rv = 0.0;

        let assign59200_e92081: f64 = (1.0 + 0.3);
        let assign59200_e92083: f64 = (assign59200_e92081 * locals.var_pds_max);
        let assign59200_e92085: f64 = if assign59200_e92083 > 0.03 { 1.0 } else { 0.0 };
        locals.var_guard1454 = assign59200_e92085;
        locals.var_guard1454_rv = 0.0;

        let (assign59210_e92106, assign59210_e92106_d_n0, assign59210_e92106_d_n2, assign59210_e92106_d_n4, assign59210_e92106_d_n5, assign59210_e92106_d_n6, assign59210_e92106_d_n7, assign59210_e92106_d_n8, assign59210_e92106_d_n9, assign59210_e92106_d_n10, assign59210_e92106_d_n11, assign59210_e92106_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59210_e92098: f64 = (1.0 + 0.3);
        let assign59210_e92100: f64 = (assign59210_e92098 * locals.var_pds_max);
        let assign59210_e92102: f64 = (assign59210_e92100 - locals.var_vds);
        let assign59210_e92104: f64 = (assign59210_e92102 - 0.03);
        (assign59210_e92104, ((assign59210_e92098 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign59210_e92098 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign59210_e92098 * locals.var_pds_max_dn4) - locals.var_vds_dn4), ((assign59210_e92098 * locals.var_pds_max_dn5) - locals.var_vds_dn5), ((assign59210_e92098 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign59210_e92098 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign59210_e92098 * locals.var_pds_max_dn8) - locals.var_vds_dn8), ((assign59210_e92098 * locals.var_pds_max_dn9) - locals.var_vds_dn9), ((assign59210_e92098 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign59210_e92098 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign59210_e92098 * locals.var_pds_max_dn14) - locals.var_vds_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign59210_e92106;
        locals.var_tmf1_dn0 = assign59210_e92106_d_n0;
        locals.var_tmf1_dn2 = assign59210_e92106_d_n2;
        locals.var_tmf1_dn4 = assign59210_e92106_d_n4;
        locals.var_tmf1_dn5 = assign59210_e92106_d_n5;
        locals.var_tmf1_dn6 = assign59210_e92106_d_n6;
        locals.var_tmf1_dn7 = assign59210_e92106_d_n7;
        locals.var_tmf1_dn8 = assign59210_e92106_d_n8;
        locals.var_tmf1_dn9 = assign59210_e92106_d_n9;
        locals.var_tmf1_dn10 = assign59210_e92106_d_n10;
        locals.var_tmf1_dn11 = assign59210_e92106_d_n11;
        locals.var_tmf1_dn14 = assign59210_e92106_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign59220_e92127, assign59220_e92127_d_n0, assign59220_e92127_d_n2, assign59220_e92127_d_n4, assign59220_e92127_d_n5, assign59220_e92127_d_n6, assign59220_e92127_d_n7, assign59220_e92127_d_n8, assign59220_e92127_d_n9, assign59220_e92127_d_n10, assign59220_e92127_d_n11, assign59220_e92127_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59220_e92120: f64 = (1.0 + 0.3);
        let assign59220_e92122: f64 = (assign59220_e92120 * locals.var_pds_max);
        let assign59220_e92123: f64 = (4.0 * assign59220_e92122);
        let assign59220_e92125: f64 = (assign59220_e92123 * 0.03);
        (assign59220_e92125, ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn4)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn5)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn8)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn9)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn14)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign59220_e92127;
        locals.var_tmf2_dn0 = assign59220_e92127_d_n0;
        locals.var_tmf2_dn2 = assign59220_e92127_d_n2;
        locals.var_tmf2_dn4 = assign59220_e92127_d_n4;
        locals.var_tmf2_dn5 = assign59220_e92127_d_n5;
        locals.var_tmf2_dn6 = assign59220_e92127_d_n6;
        locals.var_tmf2_dn7 = assign59220_e92127_d_n7;
        locals.var_tmf2_dn8 = assign59220_e92127_d_n8;
        locals.var_tmf2_dn9 = assign59220_e92127_d_n9;
        locals.var_tmf2_dn10 = assign59220_e92127_d_n10;
        locals.var_tmf2_dn11 = assign59220_e92127_d_n11;
        locals.var_tmf2_dn14 = assign59220_e92127_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign59230_e92146, assign59230_e92146_d_n0, assign59230_e92146_d_n2, assign59230_e92146_d_n4, assign59230_e92146_d_n5, assign59230_e92146_d_n6, assign59230_e92146_d_n7, assign59230_e92146_d_n8, assign59230_e92146_d_n9, assign59230_e92146_d_n10, assign59230_e92146_d_n11, assign59230_e92146_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let (assign59230_e92144, assign59230_e92144_d_n0, assign59230_e92144_d_n2, assign59230_e92144_d_n4, assign59230_e92144_d_n5, assign59230_e92144_d_n6, assign59230_e92144_d_n7, assign59230_e92144_d_n8, assign59230_e92144_d_n9, assign59230_e92144_d_n10, assign59230_e92144_d_n11, assign59230_e92144_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign59230_e92143: f64 = (-locals.var_tmf2);
                (assign59230_e92143, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign59230_e92144, assign59230_e92144_d_n0, assign59230_e92144_d_n2, assign59230_e92144_d_n4, assign59230_e92144_d_n5, assign59230_e92144_d_n6, assign59230_e92144_d_n7, assign59230_e92144_d_n8, assign59230_e92144_d_n9, assign59230_e92144_d_n10, assign59230_e92144_d_n11, assign59230_e92144_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign59230_e92146;
        locals.var_tmf2_dn0 = assign59230_e92146_d_n0;
        locals.var_tmf2_dn2 = assign59230_e92146_d_n2;
        locals.var_tmf2_dn4 = assign59230_e92146_d_n4;
        locals.var_tmf2_dn5 = assign59230_e92146_d_n5;
        locals.var_tmf2_dn6 = assign59230_e92146_d_n6;
        locals.var_tmf2_dn7 = assign59230_e92146_d_n7;
        locals.var_tmf2_dn8 = assign59230_e92146_d_n8;
        locals.var_tmf2_dn9 = assign59230_e92146_d_n9;
        locals.var_tmf2_dn10 = assign59230_e92146_d_n10;
        locals.var_tmf2_dn11 = assign59230_e92146_d_n11;
        locals.var_tmf2_dn14 = assign59230_e92146_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign59240_e92164, assign59240_e92164_d_n0, assign59240_e92164_d_n2, assign59240_e92164_d_n4, assign59240_e92164_d_n5, assign59240_e92164_d_n6, assign59240_e92164_d_n7, assign59240_e92164_d_n8, assign59240_e92164_d_n9, assign59240_e92164_d_n10, assign59240_e92164_d_n11, assign59240_e92164_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59240_e92159: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign59240_e92161: f64 = (assign59240_e92159 + locals.var_tmf2);
        let assign59240_e92162: f64 = (assign59240_e92161).sqrt();
        (assign59240_e92162, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign59240_e92162)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign59240_e92164;
        locals.var_tmf2_dn0 = assign59240_e92164_d_n0;
        locals.var_tmf2_dn2 = assign59240_e92164_d_n2;
        locals.var_tmf2_dn4 = assign59240_e92164_d_n4;
        locals.var_tmf2_dn5 = assign59240_e92164_d_n5;
        locals.var_tmf2_dn6 = assign59240_e92164_d_n6;
        locals.var_tmf2_dn7 = assign59240_e92164_d_n7;
        locals.var_tmf2_dn8 = assign59240_e92164_d_n8;
        locals.var_tmf2_dn9 = assign59240_e92164_d_n9;
        locals.var_tmf2_dn10 = assign59240_e92164_d_n10;
        locals.var_tmf2_dn11 = assign59240_e92164_d_n11;
        locals.var_tmf2_dn14 = assign59240_e92164_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign59250_e92183, assign59250_e92183_d_n0, assign59250_e92183_d_n2, assign59250_e92183_d_n4, assign59250_e92183_d_n5, assign59250_e92183_d_n6, assign59250_e92183_d_n7, assign59250_e92183_d_n8, assign59250_e92183_d_n9, assign59250_e92183_d_n10, assign59250_e92183_d_n11, assign59250_e92183_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59250_e92179: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign59250_e92180: f64 = (1.0 + assign59250_e92179);
        let assign59250_e92181: f64 = (0.5 * assign59250_e92180);
        (assign59250_e92181, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59250_e92183;
        locals.var_t1_dn0 = assign59250_e92183_d_n0;
        locals.var_t1_dn2 = assign59250_e92183_d_n2;
        locals.var_t1_dn4 = assign59250_e92183_d_n4;
        locals.var_t1_dn5 = assign59250_e92183_d_n5;
        locals.var_t1_dn6 = assign59250_e92183_d_n6;
        locals.var_t1_dn7 = assign59250_e92183_d_n7;
        locals.var_t1_dn8 = assign59250_e92183_d_n8;
        locals.var_t1_dn9 = assign59250_e92183_d_n9;
        locals.var_t1_dn10 = assign59250_e92183_d_n10;
        locals.var_t1_dn11 = assign59250_e92183_d_n11;
        locals.var_t1_dn14 = assign59250_e92183_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign59260_e92206, assign59260_e92206_d_n0, assign59260_e92206_d_n2, assign59260_e92206_d_n4, assign59260_e92206_d_n5, assign59260_e92206_d_n6, assign59260_e92206_d_n7, assign59260_e92206_d_n8, assign59260_e92206_d_n9, assign59260_e92206_d_n10, assign59260_e92206_d_n11, assign59260_e92206_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59260_e92196: f64 = (1.0 + 0.3);
        let assign59260_e92198: f64 = (assign59260_e92196 * locals.var_pds_max);
        let assign59260_e92202: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign59260_e92203: f64 = (0.5 * assign59260_e92202);
        let assign59260_e92204: f64 = (assign59260_e92198 - assign59260_e92203);
        (assign59260_e92204, ((assign59260_e92196 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign59260_e92196 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign59260_e92196 * locals.var_pds_max_dn4) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((assign59260_e92196 * locals.var_pds_max_dn5) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((assign59260_e92196 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign59260_e92196 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign59260_e92196 * locals.var_pds_max_dn8) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((assign59260_e92196 * locals.var_pds_max_dn9) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((assign59260_e92196 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign59260_e92196 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign59260_e92196 * locals.var_pds_max_dn14) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59260_e92206;
        locals.var_pds_ini_dn0 = assign59260_e92206_d_n0;
        locals.var_pds_ini_dn2 = assign59260_e92206_d_n2;
        locals.var_pds_ini_dn4 = assign59260_e92206_d_n4;
        locals.var_pds_ini_dn5 = assign59260_e92206_d_n5;
        locals.var_pds_ini_dn6 = assign59260_e92206_d_n6;
        locals.var_pds_ini_dn7 = assign59260_e92206_d_n7;
        locals.var_pds_ini_dn8 = assign59260_e92206_d_n8;
        locals.var_pds_ini_dn9 = assign59260_e92206_d_n9;
        locals.var_pds_ini_dn10 = assign59260_e92206_d_n10;
        locals.var_pds_ini_dn11 = assign59260_e92206_d_n11;
        locals.var_pds_ini_dn14 = assign59260_e92206_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let (assign59270_e92224, assign59270_e92224_d_n0, assign59270_e92224_d_n2, assign59270_e92224_d_n4, assign59270_e92224_d_n5, assign59270_e92224_d_n6, assign59270_e92224_d_n7, assign59270_e92224_d_n8, assign59270_e92224_d_n9, assign59270_e92224_d_n10, assign59270_e92224_d_n11, assign59270_e92224_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 == 0.0)) {
        let assign59270_e92220: f64 = (1.0 + 0.3);
        let assign59270_e92222: f64 = (assign59270_e92220 * locals.var_pds_max);
        (assign59270_e92222, (assign59270_e92220 * locals.var_pds_max_dn0), (assign59270_e92220 * locals.var_pds_max_dn2), (assign59270_e92220 * locals.var_pds_max_dn4), (assign59270_e92220 * locals.var_pds_max_dn5), (assign59270_e92220 * locals.var_pds_max_dn6), (assign59270_e92220 * locals.var_pds_max_dn7), (assign59270_e92220 * locals.var_pds_max_dn8), (assign59270_e92220 * locals.var_pds_max_dn9), (assign59270_e92220 * locals.var_pds_max_dn10), (assign59270_e92220 * locals.var_pds_max_dn11), (assign59270_e92220 * locals.var_pds_max_dn14),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59270_e92224;
        locals.var_pds_ini_dn0 = assign59270_e92224_d_n0;
        locals.var_pds_ini_dn2 = assign59270_e92224_d_n2;
        locals.var_pds_ini_dn4 = assign59270_e92224_d_n4;
        locals.var_pds_ini_dn5 = assign59270_e92224_d_n5;
        locals.var_pds_ini_dn6 = assign59270_e92224_d_n6;
        locals.var_pds_ini_dn7 = assign59270_e92224_d_n7;
        locals.var_pds_ini_dn8 = assign59270_e92224_d_n8;
        locals.var_pds_ini_dn9 = assign59270_e92224_d_n9;
        locals.var_pds_ini_dn10 = assign59270_e92224_d_n10;
        locals.var_pds_ini_dn11 = assign59270_e92224_d_n11;
        locals.var_pds_ini_dn14 = assign59270_e92224_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let (assign59280_e92240, assign59280_e92240_d_n0, assign59280_e92240_d_n2, assign59280_e92240_d_n4, assign59280_e92240_d_n5, assign59280_e92240_d_n6, assign59280_e92240_d_n7, assign59280_e92240_d_n8, assign59280_e92240_d_n9, assign59280_e92240_d_n10, assign59280_e92240_d_n11, assign59280_e92240_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let (assign59280_e92238, assign59280_e92238_d_n0, assign59280_e92238_d_n2, assign59280_e92238_d_n4, assign59280_e92238_d_n5, assign59280_e92238_d_n6, assign59280_e92238_d_n7, assign59280_e92238_d_n8, assign59280_e92238_d_n9, assign59280_e92238_d_n10, assign59280_e92238_d_n11, assign59280_e92238_d_n14,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn4, locals.var_pds_max_dn5, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn8, locals.var_pds_max_dn9, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn14,)
            }
        };
        (assign59280_e92238, assign59280_e92238_d_n0, assign59280_e92238_d_n2, assign59280_e92238_d_n4, assign59280_e92238_d_n5, assign59280_e92238_d_n6, assign59280_e92238_d_n7, assign59280_e92238_d_n8, assign59280_e92238_d_n9, assign59280_e92238_d_n10, assign59280_e92238_d_n11, assign59280_e92238_d_n14,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59280_e92240;
        locals.var_pds_ini_dn0 = assign59280_e92240_d_n0;
        locals.var_pds_ini_dn2 = assign59280_e92240_d_n2;
        locals.var_pds_ini_dn4 = assign59280_e92240_d_n4;
        locals.var_pds_ini_dn5 = assign59280_e92240_d_n5;
        locals.var_pds_ini_dn6 = assign59280_e92240_d_n6;
        locals.var_pds_ini_dn7 = assign59280_e92240_d_n7;
        locals.var_pds_ini_dn8 = assign59280_e92240_d_n8;
        locals.var_pds_ini_dn9 = assign59280_e92240_d_n9;
        locals.var_pds_ini_dn10 = assign59280_e92240_d_n10;
        locals.var_pds_ini_dn11 = assign59280_e92240_d_n11;
        locals.var_pds_ini_dn14 = assign59280_e92240_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let assign59290_e92243: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1455 = assign59290_e92243;
        locals.var_guard1455_rv = 0.0;

        let (assign59300_e92254, assign59300_e92254_d_n0, assign59300_e92254_d_n2, assign59300_e92254_d_n4, assign59300_e92254_d_n5, assign59300_e92254_d_n6, assign59300_e92254_d_n7, assign59300_e92254_d_n8, assign59300_e92254_d_n9, assign59300_e92254_d_n10, assign59300_e92254_d_n11, assign59300_e92254_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59300_e92254;
        locals.var_pds_ini_dn0 = assign59300_e92254_d_n0;
        locals.var_pds_ini_dn2 = assign59300_e92254_d_n2;
        locals.var_pds_ini_dn4 = assign59300_e92254_d_n4;
        locals.var_pds_ini_dn5 = assign59300_e92254_d_n5;
        locals.var_pds_ini_dn6 = assign59300_e92254_d_n6;
        locals.var_pds_ini_dn7 = assign59300_e92254_d_n7;
        locals.var_pds_ini_dn8 = assign59300_e92254_d_n8;
        locals.var_pds_ini_dn9 = assign59300_e92254_d_n9;
        locals.var_pds_ini_dn10 = assign59300_e92254_d_n10;
        locals.var_pds_ini_dn11 = assign59300_e92254_d_n11;
        locals.var_pds_ini_dn14 = assign59300_e92254_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let assign59310_e92257: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard1456 = assign59310_e92257;
        locals.var_guard1456_rv = 0.0;

        let (assign59320_e92271, assign59320_e92271_d_n0, assign59320_e92271_d_n2, assign59320_e92271_d_n4, assign59320_e92271_d_n5, assign59320_e92271_d_n6, assign59320_e92271_d_n7, assign59320_e92271_d_n8, assign59320_e92271_d_n9, assign59320_e92271_d_n10, assign59320_e92271_d_n11, assign59320_e92271_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1455 == 0.0)) && (locals.var_guard1456 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59320_e92271;
        locals.var_pds_ini_dn0 = assign59320_e92271_d_n0;
        locals.var_pds_ini_dn2 = assign59320_e92271_d_n2;
        locals.var_pds_ini_dn4 = assign59320_e92271_d_n4;
        locals.var_pds_ini_dn5 = assign59320_e92271_d_n5;
        locals.var_pds_ini_dn6 = assign59320_e92271_d_n6;
        locals.var_pds_ini_dn7 = assign59320_e92271_d_n7;
        locals.var_pds_ini_dn8 = assign59320_e92271_d_n8;
        locals.var_pds_ini_dn9 = assign59320_e92271_d_n9;
        locals.var_pds_ini_dn10 = assign59320_e92271_d_n10;
        locals.var_pds_ini_dn11 = assign59320_e92271_d_n11;
        locals.var_pds_ini_dn14 = assign59320_e92271_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let (assign59340_e92289, assign59340_e92289_d_n0, assign59340_e92289_d_n2, assign59340_e92289_d_n4, assign59340_e92289_d_n5, assign59340_e92289_d_n6, assign59340_e92289_d_n7, assign59340_e92289_d_n8, assign59340_e92289_d_n9, assign59340_e92289_d_n10, assign59340_e92289_d_n11, assign59340_e92289_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign59340_e92289;
        locals.var_pds_dn0 = assign59340_e92289_d_n0;
        locals.var_pds_dn2 = assign59340_e92289_d_n2;
        locals.var_pds_dn4 = assign59340_e92289_d_n4;
        locals.var_pds_dn5 = assign59340_e92289_d_n5;
        locals.var_pds_dn6 = assign59340_e92289_d_n6;
        locals.var_pds_dn7 = assign59340_e92289_d_n7;
        locals.var_pds_dn8 = assign59340_e92289_d_n8;
        locals.var_pds_dn9 = assign59340_e92289_d_n9;
        locals.var_pds_dn10 = assign59340_e92289_d_n10;
        locals.var_pds_dn11 = assign59340_e92289_d_n11;
        locals.var_pds_dn14 = assign59340_e92289_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign59350_e92300, assign59350_e92300_d_n0, assign59350_e92300_d_n2, assign59350_e92300_d_n4, assign59350_e92300_d_n5, assign59350_e92300_d_n6, assign59350_e92300_d_n7, assign59350_e92300_d_n8, assign59350_e92300_d_n9, assign59350_e92300_d_n10, assign59350_e92300_d_n11, assign59350_e92300_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) {
        let assign59350_e92298: f64 = (locals.var_ps0 + locals.var_pds);
        (assign59350_e92298, (locals.var_ps0_dn0 + locals.var_pds_dn0), (locals.var_ps0_dn2 + locals.var_pds_dn2), (locals.var_ps0_dn4 + locals.var_pds_dn4), (locals.var_ps0_dn5 + locals.var_pds_dn5), (locals.var_ps0_dn6 + locals.var_pds_dn6), (locals.var_ps0_dn7 + locals.var_pds_dn7), (locals.var_ps0_dn8 + locals.var_pds_dn8), (locals.var_ps0_dn9 + locals.var_pds_dn9), (locals.var_ps0_dn10 + locals.var_pds_dn10), (locals.var_ps0_dn11 + locals.var_pds_dn11), (locals.var_ps0_dn14 + locals.var_pds_dn14),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign59350_e92300;
        locals.var_psl_dn0 = assign59350_e92300_d_n0;
        locals.var_psl_dn2 = assign59350_e92300_d_n2;
        locals.var_psl_dn4 = assign59350_e92300_d_n4;
        locals.var_psl_dn5 = assign59350_e92300_d_n5;
        locals.var_psl_dn6 = assign59350_e92300_d_n6;
        locals.var_psl_dn7 = assign59350_e92300_d_n7;
        locals.var_psl_dn8 = assign59350_e92300_d_n8;
        locals.var_psl_dn9 = assign59350_e92300_d_n9;
        locals.var_psl_dn10 = assign59350_e92300_d_n10;
        locals.var_psl_dn11 = assign59350_e92300_d_n11;
        locals.var_psl_dn14 = assign59350_e92300_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign59360_e92309,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign59360_e92309;
        locals.var_flg_conv_rv = 0.0;

        let (assign59370_e92318,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_start_of_loopl != 0.0)) {
        (0.0,)
    } else {
        (locals.var_start_of_loopl,)
    }
};
        locals.var_start_of_loopl = assign59370_e92318;
        locals.var_start_of_loopl_rv = 0.0;

    }
}
