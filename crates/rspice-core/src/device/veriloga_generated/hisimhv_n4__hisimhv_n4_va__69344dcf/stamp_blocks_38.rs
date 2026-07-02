#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_230(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64090_e99051, assign64090_e99051_d_n0, assign64090_e99051_d_n2, assign64090_e99051_d_n4, assign64090_e99051_d_n5, assign64090_e99051_d_n6, assign64090_e99051_d_n7, assign64090_e99051_d_n8, assign64090_e99051_d_n9, assign64090_e99051_d_n10, assign64090_e99051_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 == 0.0)) {
        let (assign64090_e99049, assign64090_e99049_d_n0, assign64090_e99049_d_n2, assign64090_e99049_d_n4, assign64090_e99049_d_n5, assign64090_e99049_d_n6, assign64090_e99049_d_n7, assign64090_e99049_d_n8, assign64090_e99049_d_n9, assign64090_e99049_d_n10, assign64090_e99049_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign64090_e99046: f64 = (2.0 * 4.0);
                let assign64090_e99047: f64 = (1.0 / assign64090_e99046);
                let assign64090_e99048: f64 = (locals.var_dnm).powf(assign64090_e99047);
                (assign64090_e99048, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn0)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn2)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn4)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn5)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn6)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn7)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn8)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn9)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn10)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn13)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign64090_e99049, assign64090_e99049_d_n0, assign64090_e99049_d_n2, assign64090_e99049_d_n4, assign64090_e99049_d_n5, assign64090_e99049_d_n6, assign64090_e99049_d_n7, assign64090_e99049_d_n8, assign64090_e99049_d_n9, assign64090_e99049_d_n10, assign64090_e99049_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign64090_e99051;
        locals.var_dnm_dn0 = assign64090_e99051_d_n0;
        locals.var_dnm_dn2 = assign64090_e99051_d_n2;
        locals.var_dnm_dn4 = assign64090_e99051_d_n4;
        locals.var_dnm_dn5 = assign64090_e99051_d_n5;
        locals.var_dnm_dn6 = assign64090_e99051_d_n6;
        locals.var_dnm_dn7 = assign64090_e99051_d_n7;
        locals.var_dnm_dn8 = assign64090_e99051_d_n8;
        locals.var_dnm_dn9 = assign64090_e99051_d_n9;
        locals.var_dnm_dn10 = assign64090_e99051_d_n10;
        locals.var_dnm_dn13 = assign64090_e99051_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign64100_e99062, assign64100_e99062_d_n0, assign64100_e99062_d_n2, assign64100_e99062_d_n4, assign64100_e99062_d_n5, assign64100_e99062_d_n6, assign64100_e99062_d_n7, assign64100_e99062_d_n8, assign64100_e99062_d_n9, assign64100_e99062_d_n10, assign64100_e99062_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64100_e99060: f64 = (1.0 / locals.var_dnm);
        (assign64100_e99060, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign64100_e99062;
        locals.var_dnm_dn0 = assign64100_e99062_d_n0;
        locals.var_dnm_dn2 = assign64100_e99062_d_n2;
        locals.var_dnm_dn4 = assign64100_e99062_d_n4;
        locals.var_dnm_dn5 = assign64100_e99062_d_n5;
        locals.var_dnm_dn6 = assign64100_e99062_d_n6;
        locals.var_dnm_dn7 = assign64100_e99062_d_n7;
        locals.var_dnm_dn8 = assign64100_e99062_d_n8;
        locals.var_dnm_dn9 = assign64100_e99062_d_n9;
        locals.var_dnm_dn10 = assign64100_e99062_d_n10;
        locals.var_dnm_dn13 = assign64100_e99062_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign64110_e99075, assign64110_e99075_d_n0, assign64110_e99075_d_n2, assign64110_e99075_d_n4, assign64110_e99075_d_n5, assign64110_e99075_d_n6, assign64110_e99075_d_n7, assign64110_e99075_d_n8, assign64110_e99075_d_n9, assign64110_e99075_d_n10, assign64110_e99075_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64110_e99071: f64 = locals.var_tx;
        let assign64110_e99073: f64 = (assign64110_e99071 * locals.var_dnm);
        (assign64110_e99073, ((locals.var_tx_dn0 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn2)), ((locals.var_tx_dn4 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn4)), ((locals.var_tx_dn5 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn5)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn7)), ((locals.var_tx_dn8 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn8)), ((locals.var_tx_dn9 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn9)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn10)), ((locals.var_tx_dn13 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign64110_e99075;
        locals.var_ty_dn0 = assign64110_e99075_d_n0;
        locals.var_ty_dn2 = assign64110_e99075_d_n2;
        locals.var_ty_dn4 = assign64110_e99075_d_n4;
        locals.var_ty_dn5 = assign64110_e99075_d_n5;
        locals.var_ty_dn6 = assign64110_e99075_d_n6;
        locals.var_ty_dn7 = assign64110_e99075_d_n7;
        locals.var_ty_dn8 = assign64110_e99075_d_n8;
        locals.var_ty_dn9 = assign64110_e99075_d_n9;
        locals.var_ty_dn10 = assign64110_e99075_d_n10;
        locals.var_ty_dn13 = assign64110_e99075_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign64120_e99090, assign64120_e99090_d_n0, assign64120_e99090_d_n2, assign64120_e99090_d_n4, assign64120_e99090_d_n5, assign64120_e99090_d_n6, assign64120_e99090_d_n7, assign64120_e99090_d_n8, assign64120_e99090_d_n9, assign64120_e99090_d_n10, assign64120_e99090_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64120_e99084: f64 = locals.var_xmp;
        let assign64120_e99086: f64 = (assign64120_e99084 * locals.var_dnm);
        let assign64120_e99088: f64 = (assign64120_e99086 / locals.var_arg);
        (assign64120_e99088, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn0)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn2)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn4)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn5)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn6)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn7)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn8)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn9)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn10)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn13 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn13)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign64120_e99090;
        locals.var_t2_dn0 = assign64120_e99090_d_n0;
        locals.var_t2_dn2 = assign64120_e99090_d_n2;
        locals.var_t2_dn4 = assign64120_e99090_d_n4;
        locals.var_t2_dn5 = assign64120_e99090_d_n5;
        locals.var_t2_dn6 = assign64120_e99090_d_n6;
        locals.var_t2_dn7 = assign64120_e99090_d_n7;
        locals.var_t2_dn8 = assign64120_e99090_d_n8;
        locals.var_t2_dn9 = assign64120_e99090_d_n9;
        locals.var_t2_dn10 = assign64120_e99090_d_n10;
        locals.var_t2_dn13 = assign64120_e99090_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign64130_e99105, assign64130_e99105_d_n0, assign64130_e99105_d_n2, assign64130_e99105_d_n4, assign64130_e99105_d_n5, assign64130_e99105_d_n6, assign64130_e99105_d_n7, assign64130_e99105_d_n8, assign64130_e99105_d_n9, assign64130_e99105_d_n10, assign64130_e99105_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64130_e99099: f64 = (2.0 * locals.var_uc_wsti);
        let assign64130_e99101: f64 = (assign64130_e99099 * p.p7);
        let assign64130_e99103: f64 = (assign64130_e99101 * locals.var_beta_inv);
        (assign64130_e99103, ((((2.0 * locals.var_uc_wsti_dn0) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn0)), ((((2.0 * locals.var_uc_wsti_dn2) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn2)), ((((2.0 * locals.var_uc_wsti_dn4) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn4)), ((((2.0 * locals.var_uc_wsti_dn5) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn5)), ((((2.0 * locals.var_uc_wsti_dn6) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn6)), ((((2.0 * locals.var_uc_wsti_dn7) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn7)), ((((2.0 * locals.var_uc_wsti_dn8) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn8)), ((((2.0 * locals.var_uc_wsti_dn9) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn9)), ((((2.0 * locals.var_uc_wsti_dn10) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn10)), ((((2.0 * locals.var_uc_wsti_dn13) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn0, locals.var_costi7_dn2, locals.var_costi7_dn4, locals.var_costi7_dn5, locals.var_costi7_dn6, locals.var_costi7_dn7, locals.var_costi7_dn8, locals.var_costi7_dn9, locals.var_costi7_dn10, locals.var_costi7_dn13,)
    }
};
        locals.var_costi7 = assign64130_e99105;
        locals.var_costi7_dn0 = assign64130_e99105_d_n0;
        locals.var_costi7_dn2 = assign64130_e99105_d_n2;
        locals.var_costi7_dn4 = assign64130_e99105_d_n4;
        locals.var_costi7_dn5 = assign64130_e99105_d_n5;
        locals.var_costi7_dn6 = assign64130_e99105_d_n6;
        locals.var_costi7_dn7 = assign64130_e99105_d_n7;
        locals.var_costi7_dn8 = assign64130_e99105_d_n8;
        locals.var_costi7_dn9 = assign64130_e99105_d_n9;
        locals.var_costi7_dn10 = assign64130_e99105_d_n10;
        locals.var_costi7_dn13 = assign64130_e99105_d_n13;
        locals.var_costi7_rv = 0.0;

        let (assign64140_e99114, assign64140_e99114_d_n0, assign64140_e99114_d_n2, assign64140_e99114_d_n4, assign64140_e99114_d_n5, assign64140_e99114_d_n6, assign64140_e99114_d_n7, assign64140_e99114_d_n8, assign64140_e99114_d_n9, assign64140_e99114_d_n10, assign64140_e99114_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign64140_e99114;
        locals.var_t1_dn0 = assign64140_e99114_d_n0;
        locals.var_t1_dn2 = assign64140_e99114_d_n2;
        locals.var_t1_dn4 = assign64140_e99114_d_n4;
        locals.var_t1_dn5 = assign64140_e99114_d_n5;
        locals.var_t1_dn6 = assign64140_e99114_d_n6;
        locals.var_t1_dn7 = assign64140_e99114_d_n7;
        locals.var_t1_dn8 = assign64140_e99114_d_n8;
        locals.var_t1_dn9 = assign64140_e99114_d_n9;
        locals.var_t1_dn10 = assign64140_e99114_d_n10;
        locals.var_t1_dn13 = assign64140_e99114_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign64150_e99131, assign64150_e99131_d_n0, assign64150_e99131_d_n2, assign64150_e99131_d_n4, assign64150_e99131_d_n5, assign64150_e99131_d_n6, assign64150_e99131_d_n7, assign64150_e99131_d_n8, assign64150_e99131_d_n9, assign64150_e99131_d_n10, assign64150_e99131_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64150_e99123: f64 = (locals.var_costi7 * locals.var_mu);
        let assign64150_e99125: f64 = (assign64150_e99123 * locals.var_qn0sti);
        let assign64150_e99127: f64 = (assign64150_e99125 * locals.var_ty);
        let assign64150_e99129: f64 = (assign64150_e99127 / locals.var_t1);
        (assign64150_e99129, (((((((((locals.var_costi7_dn0 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn0)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn0)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn0)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn2 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn2)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn2)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn2)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn4 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn4)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn4)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn4)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn5 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn5)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn5)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn5)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn6 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn6)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn6)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn6)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn7 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn7)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn7)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn7)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn8 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn8)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn8)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn8)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn9 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn9)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn9)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn9)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn10)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn10)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn13 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn13)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn13)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn13)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn4, locals.var_idssti_dn5, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn8, locals.var_idssti_dn9, locals.var_idssti_dn10, locals.var_idssti_dn13,)
    }
};
        locals.var_idssti = assign64150_e99131;
        locals.var_idssti_dn0 = assign64150_e99131_d_n0;
        locals.var_idssti_dn2 = assign64150_e99131_d_n2;
        locals.var_idssti_dn4 = assign64150_e99131_d_n4;
        locals.var_idssti_dn5 = assign64150_e99131_d_n5;
        locals.var_idssti_dn6 = assign64150_e99131_d_n6;
        locals.var_idssti_dn7 = assign64150_e99131_d_n7;
        locals.var_idssti_dn8 = assign64150_e99131_d_n8;
        locals.var_idssti_dn9 = assign64150_e99131_d_n9;
        locals.var_idssti_dn10 = assign64150_e99131_d_n10;
        locals.var_idssti_dn13 = assign64150_e99131_d_n13;
        locals.var_idssti_rv = 0.0;

        let (assign64160_e99142, assign64160_e99142_d_n0, assign64160_e99142_d_n2, assign64160_e99142_d_n4, assign64160_e99142_d_n5, assign64160_e99142_d_n6, assign64160_e99142_d_n7, assign64160_e99142_d_n8, assign64160_e99142_d_n9, assign64160_e99142_d_n10, assign64160_e99142_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64160_e99140: f64 = (locals.var_ids + locals.var_idssti);
        (assign64160_e99140, (locals.var_ids_dn0 + locals.var_idssti_dn0), (locals.var_ids_dn2 + locals.var_idssti_dn2), (locals.var_ids_dn4 + locals.var_idssti_dn4), (locals.var_ids_dn5 + locals.var_idssti_dn5), (locals.var_ids_dn6 + locals.var_idssti_dn6), (locals.var_ids_dn7 + locals.var_idssti_dn7), (locals.var_ids_dn8 + locals.var_idssti_dn8), (locals.var_ids_dn9 + locals.var_idssti_dn9), (locals.var_ids_dn10 + locals.var_idssti_dn10), (locals.var_ids_dn13 + locals.var_idssti_dn13),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign64160_e99142;
        locals.var_ids_dn0 = assign64160_e99142_d_n0;
        locals.var_ids_dn2 = assign64160_e99142_d_n2;
        locals.var_ids_dn4 = assign64160_e99142_d_n4;
        locals.var_ids_dn5 = assign64160_e99142_d_n5;
        locals.var_ids_dn6 = assign64160_e99142_d_n6;
        locals.var_ids_dn7 = assign64160_e99142_d_n7;
        locals.var_ids_dn8 = assign64160_e99142_d_n8;
        locals.var_ids_dn9 = assign64160_e99142_d_n9;
        locals.var_ids_dn10 = assign64160_e99142_d_n10;
        locals.var_ids_dn13 = assign64160_e99142_d_n13;
        locals.var_ids_rv = 0.0;

        let assign64170_e99153: f64 = if (((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign64170_e99153;
        locals.var_guard1513_rv = 0.0;

        let (assign64180_e99164, assign64180_e99164_d_n0, assign64180_e99164_d_n2, assign64180_e99164_d_n4, assign64180_e99164_d_n5, assign64180_e99164_d_n6, assign64180_e99164_d_n7, assign64180_e99164_d_n8, assign64180_e99164_d_n9, assign64180_e99164_d_n10, assign64180_e99164_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64180_e99162: f64 = (locals.var_vgvt * locals.var_vgvt);
        (assign64180_e99162, ((locals.var_vgvt_dn0 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn0)), ((locals.var_vgvt_dn2 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn2)), ((locals.var_vgvt_dn4 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn4)), ((locals.var_vgvt_dn5 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn5)), ((locals.var_vgvt_dn6 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn6)), ((locals.var_vgvt_dn7 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn7)), ((locals.var_vgvt_dn8 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn8)), ((locals.var_vgvt_dn9 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn9)), ((locals.var_vgvt_dn10 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn10)), ((locals.var_vgvt_dn13 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn13)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn13,)
    }
};
        locals.var_kusai00 = assign64180_e99164;
        locals.var_kusai00_dn0 = assign64180_e99164_d_n0;
        locals.var_kusai00_dn2 = assign64180_e99164_d_n2;
        locals.var_kusai00_dn4 = assign64180_e99164_d_n4;
        locals.var_kusai00_dn5 = assign64180_e99164_d_n5;
        locals.var_kusai00_dn6 = assign64180_e99164_d_n6;
        locals.var_kusai00_dn7 = assign64180_e99164_d_n7;
        locals.var_kusai00_dn8 = assign64180_e99164_d_n8;
        locals.var_kusai00_dn9 = assign64180_e99164_d_n9;
        locals.var_kusai00_dn10 = assign64180_e99164_d_n10;
        locals.var_kusai00_dn13 = assign64180_e99164_d_n13;
        locals.var_kusai00_rv = 0.0;

        let (assign64190_e99179, assign64190_e99179_d_n0, assign64190_e99179_d_n2, assign64190_e99179_d_n4, assign64190_e99179_d_n5, assign64190_e99179_d_n6, assign64190_e99179_d_n7, assign64190_e99179_d_n8, assign64190_e99179_d_n9, assign64190_e99179_d_n10, assign64190_e99179_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64190_e99173: f64 = (2.0 * locals.var_beta_inv);
        let assign64190_e99175: f64 = (assign64190_e99173 * locals.var_cox_inv);
        let assign64190_e99177: f64 = (assign64190_e99175 * locals.var_idd);
        (assign64190_e99177, (((((2.0 * locals.var_beta_inv_dn0) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn0)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn0)), (((((2.0 * locals.var_beta_inv_dn2) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn2)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn2)), (((((2.0 * locals.var_beta_inv_dn4) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn4)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn4)), (((((2.0 * locals.var_beta_inv_dn5) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn5)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn5)), (((((2.0 * locals.var_beta_inv_dn6) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn6)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn6)), (((((2.0 * locals.var_beta_inv_dn7) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn7)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn7)), (((((2.0 * locals.var_beta_inv_dn8) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn8)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn8)), (((((2.0 * locals.var_beta_inv_dn9) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn9)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn9)), (((((2.0 * locals.var_beta_inv_dn10) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn10)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn10)), (((((2.0 * locals.var_beta_inv_dn13) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn13)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn13)),)
    } else {
        (locals.var_kusaidd, locals.var_kusaidd_dn0, locals.var_kusaidd_dn2, locals.var_kusaidd_dn4, locals.var_kusaidd_dn5, locals.var_kusaidd_dn6, locals.var_kusaidd_dn7, locals.var_kusaidd_dn8, locals.var_kusaidd_dn9, locals.var_kusaidd_dn10, locals.var_kusaidd_dn13,)
    }
};
        locals.var_kusaidd = assign64190_e99179;
        locals.var_kusaidd_dn0 = assign64190_e99179_d_n0;
        locals.var_kusaidd_dn2 = assign64190_e99179_d_n2;
        locals.var_kusaidd_dn4 = assign64190_e99179_d_n4;
        locals.var_kusaidd_dn5 = assign64190_e99179_d_n5;
        locals.var_kusaidd_dn6 = assign64190_e99179_d_n6;
        locals.var_kusaidd_dn7 = assign64190_e99179_d_n7;
        locals.var_kusaidd_dn8 = assign64190_e99179_d_n8;
        locals.var_kusaidd_dn9 = assign64190_e99179_d_n9;
        locals.var_kusaidd_dn10 = assign64190_e99179_d_n10;
        locals.var_kusaidd_dn13 = assign64190_e99179_d_n13;
        locals.var_kusaidd_rv = 0.0;

        let (assign64200_e99190, assign64200_e99190_d_n0, assign64200_e99190_d_n2, assign64200_e99190_d_n4, assign64200_e99190_d_n5, assign64200_e99190_d_n6, assign64200_e99190_d_n7, assign64200_e99190_d_n8, assign64200_e99190_d_n9, assign64200_e99190_d_n10, assign64200_e99190_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64200_e99188: f64 = (locals.var_kusai00 - locals.var_kusaidd);
        (assign64200_e99188, (locals.var_kusai00_dn0 - locals.var_kusaidd_dn0), (locals.var_kusai00_dn2 - locals.var_kusaidd_dn2), (locals.var_kusai00_dn4 - locals.var_kusaidd_dn4), (locals.var_kusai00_dn5 - locals.var_kusaidd_dn5), (locals.var_kusai00_dn6 - locals.var_kusaidd_dn6), (locals.var_kusai00_dn7 - locals.var_kusaidd_dn7), (locals.var_kusai00_dn8 - locals.var_kusaidd_dn8), (locals.var_kusai00_dn9 - locals.var_kusaidd_dn9), (locals.var_kusai00_dn10 - locals.var_kusaidd_dn10), (locals.var_kusai00_dn13 - locals.var_kusaidd_dn13),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn13,)
    }
};
        locals.var_kusail = assign64200_e99190;
        locals.var_kusail_dn0 = assign64200_e99190_d_n0;
        locals.var_kusail_dn2 = assign64200_e99190_d_n2;
        locals.var_kusail_dn4 = assign64200_e99190_d_n4;
        locals.var_kusail_dn5 = assign64200_e99190_d_n5;
        locals.var_kusail_dn6 = assign64200_e99190_d_n6;
        locals.var_kusail_dn7 = assign64200_e99190_d_n7;
        locals.var_kusail_dn8 = assign64200_e99190_d_n8;
        locals.var_kusail_dn9 = assign64200_e99190_d_n9;
        locals.var_kusail_dn10 = assign64200_e99190_d_n10;
        locals.var_kusail_dn13 = assign64200_e99190_d_n13;
        locals.var_kusail_rv = 0.0;

        let (assign64210_e99208, assign64210_e99208_d_n0, assign64210_e99208_d_n2, assign64210_e99208_d_n4, assign64210_e99208_d_n5, assign64210_e99208_d_n6, assign64210_e99208_d_n7, assign64210_e99208_d_n8, assign64210_e99208_d_n9, assign64210_e99208_d_n10, assign64210_e99208_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64210_e99199: f64 = (locals.var_kusai00 * locals.var_kusai00);
        let assign64210_e99202: f64 = (4.0 * 0.001);
        let assign64210_e99204: f64 = (assign64210_e99202 * 0.001);
        let assign64210_e99205: f64 = (assign64210_e99199 + assign64210_e99204);
        let assign64210_e99206: f64 = (assign64210_e99205).sqrt();
        (assign64210_e99206, (((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn13 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn13)) / (2.0 * assign64210_e99206)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64210_e99208;
        locals.var_tmf2_dn0 = assign64210_e99208_d_n0;
        locals.var_tmf2_dn2 = assign64210_e99208_d_n2;
        locals.var_tmf2_dn4 = assign64210_e99208_d_n4;
        locals.var_tmf2_dn5 = assign64210_e99208_d_n5;
        locals.var_tmf2_dn6 = assign64210_e99208_d_n6;
        locals.var_tmf2_dn7 = assign64210_e99208_d_n7;
        locals.var_tmf2_dn8 = assign64210_e99208_d_n8;
        locals.var_tmf2_dn9 = assign64210_e99208_d_n9;
        locals.var_tmf2_dn10 = assign64210_e99208_d_n10;
        locals.var_tmf2_dn13 = assign64210_e99208_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign64220_e99223, assign64220_e99223_d_n0, assign64220_e99223_d_n2, assign64220_e99223_d_n4, assign64220_e99223_d_n5, assign64220_e99223_d_n6, assign64220_e99223_d_n7, assign64220_e99223_d_n8, assign64220_e99223_d_n9, assign64220_e99223_d_n10, assign64220_e99223_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64220_e99219: f64 = (locals.var_kusai00 / locals.var_tmf2);
        let assign64220_e99220: f64 = (1.0 + assign64220_e99219);
        let assign64220_e99221: f64 = (0.5 * assign64220_e99220);
        (assign64220_e99221, (0.5 * (((locals.var_kusai00_dn0 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn2 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn4 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn5 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn6 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn7 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn8 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn9 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn10 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn13 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign64220_e99223;
        locals.var_t0_dn0 = assign64220_e99223_d_n0;
        locals.var_t0_dn2 = assign64220_e99223_d_n2;
        locals.var_t0_dn4 = assign64220_e99223_d_n4;
        locals.var_t0_dn5 = assign64220_e99223_d_n5;
        locals.var_t0_dn6 = assign64220_e99223_d_n6;
        locals.var_t0_dn7 = assign64220_e99223_d_n7;
        locals.var_t0_dn8 = assign64220_e99223_d_n8;
        locals.var_t0_dn9 = assign64220_e99223_d_n9;
        locals.var_t0_dn10 = assign64220_e99223_d_n10;
        locals.var_t0_dn13 = assign64220_e99223_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign64230_e99236, assign64230_e99236_d_n0, assign64230_e99236_d_n2, assign64230_e99236_d_n4, assign64230_e99236_d_n5, assign64230_e99236_d_n6, assign64230_e99236_d_n7, assign64230_e99236_d_n8, assign64230_e99236_d_n9, assign64230_e99236_d_n10, assign64230_e99236_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64230_e99233: f64 = (locals.var_kusai00 + locals.var_tmf2);
        let assign64230_e99234: f64 = (0.5 * assign64230_e99233);
        (assign64230_e99234, (0.5 * (locals.var_kusai00_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusai00_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusai00_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusai00_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusai00_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusai00_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_kusai00_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusai00_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_kusai00_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusai00_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn13,)
    }
};
        locals.var_kusai00 = assign64230_e99236;
        locals.var_kusai00_dn0 = assign64230_e99236_d_n0;
        locals.var_kusai00_dn2 = assign64230_e99236_d_n2;
        locals.var_kusai00_dn4 = assign64230_e99236_d_n4;
        locals.var_kusai00_dn5 = assign64230_e99236_d_n5;
        locals.var_kusai00_dn6 = assign64230_e99236_d_n6;
        locals.var_kusai00_dn7 = assign64230_e99236_d_n7;
        locals.var_kusai00_dn8 = assign64230_e99236_d_n8;
        locals.var_kusai00_dn9 = assign64230_e99236_d_n9;
        locals.var_kusai00_dn10 = assign64230_e99236_d_n10;
        locals.var_kusai00_dn13 = assign64230_e99236_d_n13;
        locals.var_kusai00_rv = 0.0;

        let assign64240_e99239: f64 = if locals.var_kusai00 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign64240_e99239;
        locals.var_guard1514_rv = 0.0;

        let (assign64250_e99250, assign64250_e99250_d_n0, assign64250_e99250_d_n2, assign64250_e99250_d_n4, assign64250_e99250_d_n5, assign64250_e99250_d_n6, assign64250_e99250_d_n7, assign64250_e99250_d_n8, assign64250_e99250_d_n9, assign64250_e99250_d_n10, assign64250_e99250_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn13,)
    }
};
        locals.var_kusai00 = assign64250_e99250;
        locals.var_kusai00_dn0 = assign64250_e99250_d_n0;
        locals.var_kusai00_dn2 = assign64250_e99250_d_n2;
        locals.var_kusai00_dn4 = assign64250_e99250_d_n4;
        locals.var_kusai00_dn5 = assign64250_e99250_d_n5;
        locals.var_kusai00_dn6 = assign64250_e99250_d_n6;
        locals.var_kusai00_dn7 = assign64250_e99250_d_n7;
        locals.var_kusai00_dn8 = assign64250_e99250_d_n8;
        locals.var_kusai00_dn9 = assign64250_e99250_d_n9;
        locals.var_kusai00_dn10 = assign64250_e99250_d_n10;
        locals.var_kusai00_dn13 = assign64250_e99250_d_n13;
        locals.var_kusai00_rv = 0.0;

        let (assign64260_e99261, assign64260_e99261_d_n0, assign64260_e99261_d_n2, assign64260_e99261_d_n4, assign64260_e99261_d_n5, assign64260_e99261_d_n6, assign64260_e99261_d_n7, assign64260_e99261_d_n8, assign64260_e99261_d_n9, assign64260_e99261_d_n10, assign64260_e99261_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign64260_e99261;
        locals.var_t0_dn0 = assign64260_e99261_d_n0;
        locals.var_t0_dn2 = assign64260_e99261_d_n2;
        locals.var_t0_dn4 = assign64260_e99261_d_n4;
        locals.var_t0_dn5 = assign64260_e99261_d_n5;
        locals.var_t0_dn6 = assign64260_e99261_d_n6;
        locals.var_t0_dn7 = assign64260_e99261_d_n7;
        locals.var_t0_dn8 = assign64260_e99261_d_n8;
        locals.var_t0_dn9 = assign64260_e99261_d_n9;
        locals.var_t0_dn10 = assign64260_e99261_d_n10;
        locals.var_t0_dn13 = assign64260_e99261_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign64270_e99279, assign64270_e99279_d_n0, assign64270_e99279_d_n2, assign64270_e99279_d_n4, assign64270_e99279_d_n5, assign64270_e99279_d_n6, assign64270_e99279_d_n7, assign64270_e99279_d_n8, assign64270_e99279_d_n9, assign64270_e99279_d_n10, assign64270_e99279_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64270_e99270: f64 = (locals.var_kusail * locals.var_kusail);
        let assign64270_e99273: f64 = (4.0 * 0.001);
        let assign64270_e99275: f64 = (assign64270_e99273 * 0.001);
        let assign64270_e99276: f64 = (assign64270_e99270 + assign64270_e99275);
        let assign64270_e99277: f64 = (assign64270_e99276).sqrt();
        (assign64270_e99277, (((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn13 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn13)) / (2.0 * assign64270_e99277)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64270_e99279;
        locals.var_tmf2_dn0 = assign64270_e99279_d_n0;
        locals.var_tmf2_dn2 = assign64270_e99279_d_n2;
        locals.var_tmf2_dn4 = assign64270_e99279_d_n4;
        locals.var_tmf2_dn5 = assign64270_e99279_d_n5;
        locals.var_tmf2_dn6 = assign64270_e99279_d_n6;
        locals.var_tmf2_dn7 = assign64270_e99279_d_n7;
        locals.var_tmf2_dn8 = assign64270_e99279_d_n8;
        locals.var_tmf2_dn9 = assign64270_e99279_d_n9;
        locals.var_tmf2_dn10 = assign64270_e99279_d_n10;
        locals.var_tmf2_dn13 = assign64270_e99279_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign64280_e99294, assign64280_e99294_d_n0, assign64280_e99294_d_n2, assign64280_e99294_d_n4, assign64280_e99294_d_n5, assign64280_e99294_d_n6, assign64280_e99294_d_n7, assign64280_e99294_d_n8, assign64280_e99294_d_n9, assign64280_e99294_d_n10, assign64280_e99294_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64280_e99290: f64 = (locals.var_kusail / locals.var_tmf2);
        let assign64280_e99291: f64 = (1.0 + assign64280_e99290);
        let assign64280_e99292: f64 = (0.5 * assign64280_e99291);
        (assign64280_e99292, (0.5 * (((locals.var_kusail_dn0 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn2 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn4 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn5 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn6 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn7 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn8 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn9 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn10 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn13 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign64280_e99294;
        locals.var_t0_dn0 = assign64280_e99294_d_n0;
        locals.var_t0_dn2 = assign64280_e99294_d_n2;
        locals.var_t0_dn4 = assign64280_e99294_d_n4;
        locals.var_t0_dn5 = assign64280_e99294_d_n5;
        locals.var_t0_dn6 = assign64280_e99294_d_n6;
        locals.var_t0_dn7 = assign64280_e99294_d_n7;
        locals.var_t0_dn8 = assign64280_e99294_d_n8;
        locals.var_t0_dn9 = assign64280_e99294_d_n9;
        locals.var_t0_dn10 = assign64280_e99294_d_n10;
        locals.var_t0_dn13 = assign64280_e99294_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign64290_e99307, assign64290_e99307_d_n0, assign64290_e99307_d_n2, assign64290_e99307_d_n4, assign64290_e99307_d_n5, assign64290_e99307_d_n6, assign64290_e99307_d_n7, assign64290_e99307_d_n8, assign64290_e99307_d_n9, assign64290_e99307_d_n10, assign64290_e99307_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64290_e99304: f64 = (locals.var_kusail + locals.var_tmf2);
        let assign64290_e99305: f64 = (0.5 * assign64290_e99304);
        (assign64290_e99305, (0.5 * (locals.var_kusail_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusail_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusail_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusail_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusail_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusail_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_kusail_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusail_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_kusail_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusail_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn13,)
    }
};
        locals.var_kusail = assign64290_e99307;
        locals.var_kusail_dn0 = assign64290_e99307_d_n0;
        locals.var_kusail_dn2 = assign64290_e99307_d_n2;
        locals.var_kusail_dn4 = assign64290_e99307_d_n4;
        locals.var_kusail_dn5 = assign64290_e99307_d_n5;
        locals.var_kusail_dn6 = assign64290_e99307_d_n6;
        locals.var_kusail_dn7 = assign64290_e99307_d_n7;
        locals.var_kusail_dn8 = assign64290_e99307_d_n8;
        locals.var_kusail_dn9 = assign64290_e99307_d_n9;
        locals.var_kusail_dn10 = assign64290_e99307_d_n10;
        locals.var_kusail_dn13 = assign64290_e99307_d_n13;
        locals.var_kusail_rv = 0.0;

        let assign64300_e99310: f64 = if locals.var_kusail < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign64300_e99310;
        locals.var_guard1515_rv = 0.0;

        let (assign64310_e99321, assign64310_e99321_d_n0, assign64310_e99321_d_n2, assign64310_e99321_d_n4, assign64310_e99321_d_n5, assign64310_e99321_d_n6, assign64310_e99321_d_n7, assign64310_e99321_d_n8, assign64310_e99321_d_n9, assign64310_e99321_d_n10, assign64310_e99321_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn13,)
    }
};
        locals.var_kusail = assign64310_e99321;
        locals.var_kusail_dn0 = assign64310_e99321_d_n0;
        locals.var_kusail_dn2 = assign64310_e99321_d_n2;
        locals.var_kusail_dn4 = assign64310_e99321_d_n4;
        locals.var_kusail_dn5 = assign64310_e99321_d_n5;
        locals.var_kusail_dn6 = assign64310_e99321_d_n6;
        locals.var_kusail_dn7 = assign64310_e99321_d_n7;
        locals.var_kusail_dn8 = assign64310_e99321_d_n8;
        locals.var_kusail_dn9 = assign64310_e99321_d_n9;
        locals.var_kusail_dn10 = assign64310_e99321_d_n10;
        locals.var_kusail_dn13 = assign64310_e99321_d_n13;
        locals.var_kusail_rv = 0.0;

        let (assign64320_e99332, assign64320_e99332_d_n0, assign64320_e99332_d_n2, assign64320_e99332_d_n4, assign64320_e99332_d_n5, assign64320_e99332_d_n6, assign64320_e99332_d_n7, assign64320_e99332_d_n8, assign64320_e99332_d_n9, assign64320_e99332_d_n10, assign64320_e99332_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign64320_e99332;
        locals.var_t0_dn0 = assign64320_e99332_d_n0;
        locals.var_t0_dn2 = assign64320_e99332_d_n2;
        locals.var_t0_dn4 = assign64320_e99332_d_n4;
        locals.var_t0_dn5 = assign64320_e99332_d_n5;
        locals.var_t0_dn6 = assign64320_e99332_d_n6;
        locals.var_t0_dn7 = assign64320_e99332_d_n7;
        locals.var_t0_dn8 = assign64320_e99332_d_n8;
        locals.var_t0_dn9 = assign64320_e99332_d_n9;
        locals.var_t0_dn10 = assign64320_e99332_d_n10;
        locals.var_t0_dn13 = assign64320_e99332_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign64330_e99343, assign64330_e99343_d_n0, assign64330_e99343_d_n2, assign64330_e99343_d_n4, assign64330_e99343_d_n5, assign64330_e99343_d_n6, assign64330_e99343_d_n7, assign64330_e99343_d_n8, assign64330_e99343_d_n9, assign64330_e99343_d_n10, assign64330_e99343_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64330_e99341: f64 = (locals.var_kusai00 - locals.var_kusail);
        (assign64330_e99341, (locals.var_kusai00_dn0 - locals.var_kusail_dn0), (locals.var_kusai00_dn2 - locals.var_kusail_dn2), (locals.var_kusai00_dn4 - locals.var_kusail_dn4), (locals.var_kusai00_dn5 - locals.var_kusail_dn5), (locals.var_kusai00_dn6 - locals.var_kusail_dn6), (locals.var_kusai00_dn7 - locals.var_kusail_dn7), (locals.var_kusai00_dn8 - locals.var_kusail_dn8), (locals.var_kusai00_dn9 - locals.var_kusail_dn9), (locals.var_kusai00_dn10 - locals.var_kusail_dn10), (locals.var_kusai00_dn13 - locals.var_kusail_dn13),)
    } else {
        (locals.var_kusai00l, locals.var_kusai00l_dn0, locals.var_kusai00l_dn2, locals.var_kusai00l_dn4, locals.var_kusai00l_dn5, locals.var_kusai00l_dn6, locals.var_kusai00l_dn7, locals.var_kusai00l_dn8, locals.var_kusai00l_dn9, locals.var_kusai00l_dn10, locals.var_kusai00l_dn13,)
    }
};
        locals.var_kusai00l = assign64330_e99343;
        locals.var_kusai00l_dn0 = assign64330_e99343_d_n0;
        locals.var_kusai00l_dn2 = assign64330_e99343_d_n2;
        locals.var_kusai00l_dn4 = assign64330_e99343_d_n4;
        locals.var_kusai00l_dn5 = assign64330_e99343_d_n5;
        locals.var_kusai00l_dn6 = assign64330_e99343_d_n6;
        locals.var_kusai00l_dn7 = assign64330_e99343_d_n7;
        locals.var_kusai00l_dn8 = assign64330_e99343_d_n8;
        locals.var_kusai00l_dn9 = assign64330_e99343_d_n9;
        locals.var_kusai00l_dn10 = assign64330_e99343_d_n10;
        locals.var_kusai00l_dn13 = assign64330_e99343_d_n13;
        locals.var_kusai00l_rv = 0.0;

        let assign64340_e99347: f64 = (10.0 * 2.220446049250313e-16);
        let assign64340_e99352: f64 = (10.0 * 2.220446049250313e-16);
        let assign64340_e99354: f64 = if ((locals.var_qn0 < assign64340_e99347) || (locals.var_kusai00l < assign64340_e99352)) { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign64340_e99354;
        locals.var_guard1516_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_231(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64350_e99365,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1516 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign64350_e99365;
        locals.var_flg_ign_rv = 0.0;

        let (assign64360_e99377,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1516 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign64360_e99377;
        locals.var_flg_ign_rv = 0.0;

        let (assign64370_e99384,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_end_of_part_1 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign64370_e99384;
        locals.var_end_of_part_1_rv = 0.0;

        let assign64380_e99391: f64 = if ((locals.var_flg_noqi == 0.0) && (locals.var_vgvt > 1e-12)) { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign64380_e99391;
        locals.var_guard1517_rv = 0.0;

        let (assign64390_e99404, assign64390_e99404_d_n0, assign64390_e99404_d_n2, assign64390_e99404_d_n4, assign64390_e99404_d_n5, assign64390_e99404_d_n6, assign64390_e99404_d_n7, assign64390_e99404_d_n8, assign64390_e99404_d_n9, assign64390_e99404_d_n10, assign64390_e99404_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1517 != 0.0)) {
        let assign64390_e99398: f64 = (locals.var_fac1 * locals.var_beta);
        let assign64390_e99401: f64 = (2.0 * locals.var_xi0p12);
        let assign64390_e99402: f64 = (assign64390_e99398 / assign64390_e99401);
        (assign64390_e99402, (((((locals.var_fac1_dn0 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn0)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn0))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn2 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn2)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn2))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn4 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn4)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn4))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn5 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn5)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn5))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn6 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn6)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn6))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn7 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn7)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn7))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn8 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn8)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn8))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn9 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn9)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn9))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn10 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn10)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn10))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn13 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn13)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn13))) / (assign64390_e99401 * assign64390_e99401)),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn13,)
    }
};
        locals.var_delta = assign64390_e99404;
        locals.var_delta_dn0 = assign64390_e99404_d_n0;
        locals.var_delta_dn2 = assign64390_e99404_d_n2;
        locals.var_delta_dn4 = assign64390_e99404_d_n4;
        locals.var_delta_dn5 = assign64390_e99404_d_n5;
        locals.var_delta_dn6 = assign64390_e99404_d_n6;
        locals.var_delta_dn7 = assign64390_e99404_d_n7;
        locals.var_delta_dn8 = assign64390_e99404_d_n8;
        locals.var_delta_dn9 = assign64390_e99404_d_n9;
        locals.var_delta_dn10 = assign64390_e99404_d_n10;
        locals.var_delta_dn13 = assign64390_e99404_d_n13;
        locals.var_delta_rv = 0.0;

        let (assign64400_e99417, assign64400_e99417_d_n0, assign64400_e99417_d_n2, assign64400_e99417_d_n4, assign64400_e99417_d_n5, assign64400_e99417_d_n6, assign64400_e99417_d_n7, assign64400_e99417_d_n8, assign64400_e99417_d_n9, assign64400_e99417_d_n10, assign64400_e99417_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1517 != 0.0)) {
        let assign64400_e99412: f64 = (1.0 + locals.var_delta);
        let assign64400_e99413: f64 = (locals.var_vgvt / assign64400_e99412);
        let assign64400_e99415: f64 = (assign64400_e99413 + locals.var_ps0);
        (assign64400_e99415, ((((locals.var_vgvt_dn0 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn0)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn0), ((((locals.var_vgvt_dn2 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn2)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn2), ((((locals.var_vgvt_dn4 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn4)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn4), ((((locals.var_vgvt_dn5 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn5)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn5), ((((locals.var_vgvt_dn6 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn6)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn6), ((((locals.var_vgvt_dn7 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn7)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn7), ((((locals.var_vgvt_dn8 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn8)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn8), ((((locals.var_vgvt_dn9 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn9)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn9), ((((locals.var_vgvt_dn10 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn10)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn10), ((((locals.var_vgvt_dn13 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn13)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn13),)
    } else {
        (locals.var_pslsat, locals.var_pslsat_dn0, locals.var_pslsat_dn2, locals.var_pslsat_dn4, locals.var_pslsat_dn5, locals.var_pslsat_dn6, locals.var_pslsat_dn7, locals.var_pslsat_dn8, locals.var_pslsat_dn9, locals.var_pslsat_dn10, locals.var_pslsat_dn13,)
    }
};
        locals.var_pslsat = assign64400_e99417;
        locals.var_pslsat_dn0 = assign64400_e99417_d_n0;
        locals.var_pslsat_dn2 = assign64400_e99417_d_n2;
        locals.var_pslsat_dn4 = assign64400_e99417_d_n4;
        locals.var_pslsat_dn5 = assign64400_e99417_d_n5;
        locals.var_pslsat_dn6 = assign64400_e99417_d_n6;
        locals.var_pslsat_dn7 = assign64400_e99417_d_n7;
        locals.var_pslsat_dn8 = assign64400_e99417_d_n8;
        locals.var_pslsat_dn9 = assign64400_e99417_d_n9;
        locals.var_pslsat_dn10 = assign64400_e99417_d_n10;
        locals.var_pslsat_dn13 = assign64400_e99417_d_n13;
        locals.var_pslsat_rv = 0.0;

        let (assign64410_e99425, assign64410_e99425_d_n0, assign64410_e99425_d_n2, assign64410_e99425_d_n4, assign64410_e99425_d_n5, assign64410_e99425_d_n6, assign64410_e99425_d_n7, assign64410_e99425_d_n8, assign64410_e99425_d_n9, assign64410_e99425_d_n10, assign64410_e99425_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1517 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pslsat, locals.var_pslsat_dn0, locals.var_pslsat_dn2, locals.var_pslsat_dn4, locals.var_pslsat_dn5, locals.var_pslsat_dn6, locals.var_pslsat_dn7, locals.var_pslsat_dn8, locals.var_pslsat_dn9, locals.var_pslsat_dn10, locals.var_pslsat_dn13,)
    }
};
        locals.var_pslsat = assign64410_e99425;
        locals.var_pslsat_dn0 = assign64410_e99425_d_n0;
        locals.var_pslsat_dn2 = assign64410_e99425_d_n2;
        locals.var_pslsat_dn4 = assign64410_e99425_d_n4;
        locals.var_pslsat_dn5 = assign64410_e99425_d_n5;
        locals.var_pslsat_dn6 = assign64410_e99425_d_n6;
        locals.var_pslsat_dn7 = assign64410_e99425_d_n7;
        locals.var_pslsat_dn8 = assign64410_e99425_d_n8;
        locals.var_pslsat_dn9 = assign64410_e99425_d_n9;
        locals.var_pslsat_dn10 = assign64410_e99425_d_n10;
        locals.var_pslsat_dn13 = assign64410_e99425_d_n13;
        locals.var_pslsat_rv = 0.0;

        let (assign64450_e99447, assign64450_e99447_d_n0, assign64450_e99447_d_n2, assign64450_e99447_d_n4, assign64450_e99447_d_n5, assign64450_e99447_d_n6, assign64450_e99447_d_n7, assign64450_e99447_d_n8, assign64450_e99447_d_n9, assign64450_e99447_d_n10, assign64450_e99447_d_n13,) = {
    if (locals.var_guard443 == 0.0) {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    } else {
        (locals.var_idsorg, locals.var_idsorg_dn0, locals.var_idsorg_dn2, locals.var_idsorg_dn4, locals.var_idsorg_dn5, locals.var_idsorg_dn6, locals.var_idsorg_dn7, locals.var_idsorg_dn8, locals.var_idsorg_dn9, locals.var_idsorg_dn10, locals.var_idsorg_dn13,)
    }
};
        locals.var_idsorg = assign64450_e99447;
        locals.var_idsorg_dn0 = assign64450_e99447_d_n0;
        locals.var_idsorg_dn2 = assign64450_e99447_d_n2;
        locals.var_idsorg_dn4 = assign64450_e99447_d_n4;
        locals.var_idsorg_dn5 = assign64450_e99447_d_n5;
        locals.var_idsorg_dn6 = assign64450_e99447_d_n6;
        locals.var_idsorg_dn7 = assign64450_e99447_d_n7;
        locals.var_idsorg_dn8 = assign64450_e99447_d_n8;
        locals.var_idsorg_dn9 = assign64450_e99447_d_n9;
        locals.var_idsorg_dn10 = assign64450_e99447_d_n10;
        locals.var_idsorg_dn13 = assign64450_e99447_d_n13;
        locals.var_idsorg_rv = 0.0;

        let (assign64460_e99452, assign64460_e99452_d_n0, assign64460_e99452_d_n2, assign64460_e99452_d_n4, assign64460_e99452_d_n5, assign64460_e99452_d_n6, assign64460_e99452_d_n7, assign64460_e99452_d_n8, assign64460_e99452_d_n9, assign64460_e99452_d_n10, assign64460_e99452_d_n13,) = {
    if (locals.var_guard443 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn4, locals.var_idspt1_dn5, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn8, locals.var_idspt1_dn9, locals.var_idspt1_dn10, locals.var_idspt1_dn13,)
    }
};
        locals.var_idspt1 = assign64460_e99452;
        locals.var_idspt1_dn0 = assign64460_e99452_d_n0;
        locals.var_idspt1_dn2 = assign64460_e99452_d_n2;
        locals.var_idspt1_dn4 = assign64460_e99452_d_n4;
        locals.var_idspt1_dn5 = assign64460_e99452_d_n5;
        locals.var_idspt1_dn6 = assign64460_e99452_d_n6;
        locals.var_idspt1_dn7 = assign64460_e99452_d_n7;
        locals.var_idspt1_dn8 = assign64460_e99452_d_n8;
        locals.var_idspt1_dn9 = assign64460_e99452_d_n9;
        locals.var_idspt1_dn10 = assign64460_e99452_d_n10;
        locals.var_idspt1_dn13 = assign64460_e99452_d_n13;
        locals.var_idspt1_rv = 0.0;

        let assign64470_e99459: f64 = if ((p.p450 > 0.0) && (p.p454 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign64470_e99459;
        locals.var_guard1519_rv = 0.0;

        let (assign64480_e99466,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        (1e-5,)
    } else {
        (locals.var_t_sub,)
    }
};
        locals.var_t_sub = assign64480_e99466;
        locals.var_t_sub_rv = 0.0;

        let (assign64490_e99481, assign64490_e99481_d_n0, assign64490_e99481_d_n2, assign64490_e99481_d_n4, assign64490_e99481_d_n5, assign64490_e99481_d_n6, assign64490_e99481_d_n7, assign64490_e99481_d_n8, assign64490_e99481_d_n9, assign64490_e99481_d_n10, assign64490_e99481_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64490_e99473: f64 = (locals.var_vgs - locals.var_vfb);
        let assign64490_e99475: f64 = (assign64490_e99473 + locals.var_dvth);
        let assign64490_e99477: f64 = (assign64490_e99475 - locals.var_dppg);
        let assign64490_e99479: f64 = (assign64490_e99477 - p.p455);
        (assign64490_e99479, (locals.var_dvth_dn0 - locals.var_dppg_dn0), (locals.var_dvth_dn2 - locals.var_dppg_dn2), (locals.var_dvth_dn4 - locals.var_dppg_dn4), ((locals.var_vgs_dn5 + locals.var_dvth_dn5) - locals.var_dppg_dn5), ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), (locals.var_dvth_dn8 - locals.var_dppg_dn8), (locals.var_dvth_dn9 - locals.var_dppg_dn9), (locals.var_dvth_dn10 - locals.var_dppg_dn10), (locals.var_dvth_dn13 - locals.var_dppg_dn13),)
    } else {
        (locals.var_vgp__blk1525, locals.var_vgp__blk1525_dn0, locals.var_vgp__blk1525_dn2, locals.var_vgp__blk1525_dn4, locals.var_vgp__blk1525_dn5, locals.var_vgp__blk1525_dn6, locals.var_vgp__blk1525_dn7, locals.var_vgp__blk1525_dn8, locals.var_vgp__blk1525_dn9, locals.var_vgp__blk1525_dn10, locals.var_vgp__blk1525_dn13,)
    }
};
        locals.var_vgp__blk1525 = assign64490_e99481;
        locals.var_vgp__blk1525_dn0 = assign64490_e99481_d_n0;
        locals.var_vgp__blk1525_dn2 = assign64490_e99481_d_n2;
        locals.var_vgp__blk1525_dn4 = assign64490_e99481_d_n4;
        locals.var_vgp__blk1525_dn5 = assign64490_e99481_d_n5;
        locals.var_vgp__blk1525_dn6 = assign64490_e99481_d_n6;
        locals.var_vgp__blk1525_dn7 = assign64490_e99481_d_n7;
        locals.var_vgp__blk1525_dn8 = assign64490_e99481_d_n8;
        locals.var_vgp__blk1525_dn9 = assign64490_e99481_d_n9;
        locals.var_vgp__blk1525_dn10 = assign64490_e99481_d_n10;
        locals.var_vgp__blk1525_dn13 = assign64490_e99481_d_n13;
        locals.var_vgp__blk1525_rv = 0.0;

        let (assign64500_e99490, assign64500_e99490_d_n0, assign64500_e99490_d_n2, assign64500_e99490_d_n4, assign64500_e99490_d_n5, assign64500_e99490_d_n6, assign64500_e99490_d_n7, assign64500_e99490_d_n8, assign64500_e99490_d_n9, assign64500_e99490_d_n10, assign64500_e99490_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64500_e99488: f64 = (locals.var_vth + p.p455);
        (assign64500_e99488, locals.var_vth_dn0, locals.var_vth_dn2, locals.var_vth_dn4, locals.var_vth_dn5, locals.var_vth_dn6, locals.var_vth_dn7, locals.var_vth_dn8, locals.var_vth_dn9, locals.var_vth_dn10, locals.var_vth_dn13,)
    } else {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn4, locals.var_wk_vth_dn5, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn8, locals.var_wk_vth_dn9, locals.var_wk_vth_dn10, locals.var_wk_vth_dn13,)
    }
};
        locals.var_wk_vth = assign64500_e99490;
        locals.var_wk_vth_dn0 = assign64500_e99490_d_n0;
        locals.var_wk_vth_dn2 = assign64500_e99490_d_n2;
        locals.var_wk_vth_dn4 = assign64500_e99490_d_n4;
        locals.var_wk_vth_dn5 = assign64500_e99490_d_n5;
        locals.var_wk_vth_dn6 = assign64500_e99490_d_n6;
        locals.var_wk_vth_dn7 = assign64500_e99490_d_n7;
        locals.var_wk_vth_dn8 = assign64500_e99490_d_n8;
        locals.var_wk_vth_dn9 = assign64500_e99490_d_n9;
        locals.var_wk_vth_dn10 = assign64500_e99490_d_n10;
        locals.var_wk_vth_dn13 = assign64500_e99490_d_n13;
        locals.var_wk_vth_rv = 0.0;

        let (assign64510_e99510, assign64510_e99510_d_n0, assign64510_e99510_d_n2, assign64510_e99510_d_n4, assign64510_e99510_d_n5, assign64510_e99510_d_n6, assign64510_e99510_d_n7, assign64510_e99510_d_n8, assign64510_e99510_d_n9, assign64510_e99510_d_n10, assign64510_e99510_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64510_e99497: f64 = (locals.var_vbipn - locals.var_vbscl__blk435);
        let assign64510_e99500: f64 = (locals.var_vbipn - locals.var_vbscl__blk435);
        let assign64510_e99501: f64 = (assign64510_e99497 * assign64510_e99500);
        let assign64510_e99504: f64 = (4.0 * 0.01);
        let assign64510_e99506: f64 = (assign64510_e99504 * 0.01);
        let assign64510_e99507: f64 = (assign64510_e99501 + assign64510_e99506);
        let assign64510_e99508: f64 = (assign64510_e99507).sqrt();
        (assign64510_e99508, ((((locals.var_vbipn_dn0 - locals.var_vbscl__blk435_dn0) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn0 - locals.var_vbscl__blk435_dn0))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn2 - locals.var_vbscl__blk435_dn2) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn2 - locals.var_vbscl__blk435_dn2))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn4 - locals.var_vbscl__blk435_dn4) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn4 - locals.var_vbscl__blk435_dn4))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn5 - locals.var_vbscl__blk435_dn5) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn5 - locals.var_vbscl__blk435_dn5))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn6 - locals.var_vbscl__blk435_dn6) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn6 - locals.var_vbscl__blk435_dn6))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn7 - locals.var_vbscl__blk435_dn7) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn7 - locals.var_vbscl__blk435_dn7))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn8 - locals.var_vbscl__blk435_dn8) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn8 - locals.var_vbscl__blk435_dn8))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn9 - locals.var_vbscl__blk435_dn9) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn9 - locals.var_vbscl__blk435_dn9))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn10 - locals.var_vbscl__blk435_dn10) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn10 - locals.var_vbscl__blk435_dn10))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn13 - locals.var_vbscl__blk435_dn13) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn13 - locals.var_vbscl__blk435_dn13))) / (2.0 * assign64510_e99508)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign64510_e99510;
        locals.var_tmf1_dn0 = assign64510_e99510_d_n0;
        locals.var_tmf1_dn2 = assign64510_e99510_d_n2;
        locals.var_tmf1_dn4 = assign64510_e99510_d_n4;
        locals.var_tmf1_dn5 = assign64510_e99510_d_n5;
        locals.var_tmf1_dn6 = assign64510_e99510_d_n6;
        locals.var_tmf1_dn7 = assign64510_e99510_d_n7;
        locals.var_tmf1_dn8 = assign64510_e99510_d_n8;
        locals.var_tmf1_dn9 = assign64510_e99510_d_n9;
        locals.var_tmf1_dn10 = assign64510_e99510_d_n10;
        locals.var_tmf1_dn13 = assign64510_e99510_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign64520_e99523, assign64520_e99523_d_n0, assign64520_e99523_d_n2, assign64520_e99523_d_n4, assign64520_e99523_d_n5, assign64520_e99523_d_n6, assign64520_e99523_d_n7, assign64520_e99523_d_n8, assign64520_e99523_d_n9, assign64520_e99523_d_n10, assign64520_e99523_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64520_e99518: f64 = (locals.var_vbipn - locals.var_vbscl__blk435);
        let assign64520_e99520: f64 = (assign64520_e99518 + locals.var_tmf1);
        let assign64520_e99521: f64 = (0.5 * assign64520_e99520);
        (assign64520_e99521, (0.5 * ((locals.var_vbipn_dn0 - locals.var_vbscl__blk435_dn0) + locals.var_tmf1_dn0)), (0.5 * ((locals.var_vbipn_dn2 - locals.var_vbscl__blk435_dn2) + locals.var_tmf1_dn2)), (0.5 * ((locals.var_vbipn_dn4 - locals.var_vbscl__blk435_dn4) + locals.var_tmf1_dn4)), (0.5 * ((locals.var_vbipn_dn5 - locals.var_vbscl__blk435_dn5) + locals.var_tmf1_dn5)), (0.5 * ((locals.var_vbipn_dn6 - locals.var_vbscl__blk435_dn6) + locals.var_tmf1_dn6)), (0.5 * ((locals.var_vbipn_dn7 - locals.var_vbscl__blk435_dn7) + locals.var_tmf1_dn7)), (0.5 * ((locals.var_vbipn_dn8 - locals.var_vbscl__blk435_dn8) + locals.var_tmf1_dn8)), (0.5 * ((locals.var_vbipn_dn9 - locals.var_vbscl__blk435_dn9) + locals.var_tmf1_dn9)), (0.5 * ((locals.var_vbipn_dn10 - locals.var_vbscl__blk435_dn10) + locals.var_tmf1_dn10)), (0.5 * ((locals.var_vbipn_dn13 - locals.var_vbscl__blk435_dn13) + locals.var_tmf1_dn13)),)
    } else {
        (locals.var_vpositive, locals.var_vpositive_dn0, locals.var_vpositive_dn2, locals.var_vpositive_dn4, locals.var_vpositive_dn5, locals.var_vpositive_dn6, locals.var_vpositive_dn7, locals.var_vpositive_dn8, locals.var_vpositive_dn9, locals.var_vpositive_dn10, locals.var_vpositive_dn13,)
    }
};
        locals.var_vpositive = assign64520_e99523;
        locals.var_vpositive_dn0 = assign64520_e99523_d_n0;
        locals.var_vpositive_dn2 = assign64520_e99523_d_n2;
        locals.var_vpositive_dn4 = assign64520_e99523_d_n4;
        locals.var_vpositive_dn5 = assign64520_e99523_d_n5;
        locals.var_vpositive_dn6 = assign64520_e99523_d_n6;
        locals.var_vpositive_dn7 = assign64520_e99523_d_n7;
        locals.var_vpositive_dn8 = assign64520_e99523_d_n8;
        locals.var_vpositive_dn9 = assign64520_e99523_d_n9;
        locals.var_vpositive_dn10 = assign64520_e99523_d_n10;
        locals.var_vpositive_dn13 = assign64520_e99523_d_n13;
        locals.var_vpositive_rv = 0.0;

        let (assign64530_e99545, assign64530_e99545_d_n0, assign64530_e99545_d_n2, assign64530_e99545_d_n4, assign64530_e99545_d_n5, assign64530_e99545_d_n6, assign64530_e99545_d_n7, assign64530_e99545_d_n8, assign64530_e99545_d_n9, assign64530_e99545_d_n10, assign64530_e99545_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64530_e99530: f64 = (2.0 * 1.6021918e-19);
        let assign64530_e99532: f64 = (assign64530_e99530 * locals.var_vpositive);
        let assign64530_e99534: f64 = (assign64530_e99532 / 1.034943e-10);
        let assign64530_e99536: f64 = (assign64530_e99534 * locals.var_nsub);
        let assign64530_e99538: f64 = (assign64530_e99536 * locals.var_uc_njunc);
        let assign64530_e99541: f64 = (locals.var_nsub + locals.var_uc_njunc);
        let assign64530_e99542: f64 = (assign64530_e99538 / assign64530_e99541);
        let assign64530_e99543: f64 = (assign64530_e99542).sqrt();
        (assign64530_e99543, (((((((((assign64530_e99530 * locals.var_vpositive_dn0) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn0)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn0)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn2) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn2)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn2)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn4) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn4)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn4)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn5) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn5)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn5)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn6) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn6)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn6)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn7) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn7)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn7)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn8) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn8)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn8)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn9) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn9)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn9)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn10) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn10)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn10)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn13) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn13)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn13)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)),)
    } else {
        (locals.var_ec__blk1520, locals.var_ec__blk1520_dn0, locals.var_ec__blk1520_dn2, locals.var_ec__blk1520_dn4, locals.var_ec__blk1520_dn5, locals.var_ec__blk1520_dn6, locals.var_ec__blk1520_dn7, locals.var_ec__blk1520_dn8, locals.var_ec__blk1520_dn9, locals.var_ec__blk1520_dn10, locals.var_ec__blk1520_dn13,)
    }
};
        locals.var_ec__blk1520 = assign64530_e99545;
        locals.var_ec__blk1520_dn0 = assign64530_e99545_d_n0;
        locals.var_ec__blk1520_dn2 = assign64530_e99545_d_n2;
        locals.var_ec__blk1520_dn4 = assign64530_e99545_d_n4;
        locals.var_ec__blk1520_dn5 = assign64530_e99545_d_n5;
        locals.var_ec__blk1520_dn6 = assign64530_e99545_d_n6;
        locals.var_ec__blk1520_dn7 = assign64530_e99545_d_n7;
        locals.var_ec__blk1520_dn8 = assign64530_e99545_d_n8;
        locals.var_ec__blk1520_dn9 = assign64530_e99545_d_n9;
        locals.var_ec__blk1520_dn10 = assign64530_e99545_d_n10;
        locals.var_ec__blk1520_dn13 = assign64530_e99545_d_n13;
        locals.var_ec__blk1520_rv = 0.0;

        let (assign64540_e99554, assign64540_e99554_d_n0, assign64540_e99554_d_n2, assign64540_e99554_d_n4, assign64540_e99554_d_n5, assign64540_e99554_d_n6, assign64540_e99554_d_n7, assign64540_e99554_d_n8, assign64540_e99554_d_n9, assign64540_e99554_d_n10, assign64540_e99554_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64540_e99552: f64 = (locals.var_ec__blk1520 * locals.var_leff);
        (assign64540_e99552, (locals.var_ec__blk1520_dn0 * locals.var_leff), (locals.var_ec__blk1520_dn2 * locals.var_leff), (locals.var_ec__blk1520_dn4 * locals.var_leff), (locals.var_ec__blk1520_dn5 * locals.var_leff), (locals.var_ec__blk1520_dn6 * locals.var_leff), (locals.var_ec__blk1520_dn7 * locals.var_leff), (locals.var_ec__blk1520_dn8 * locals.var_leff), (locals.var_ec__blk1520_dn9 * locals.var_leff), (locals.var_ec__blk1520_dn10 * locals.var_leff), (locals.var_ec__blk1520_dn13 * locals.var_leff),)
    } else {
        (locals.var_wk, locals.var_wk_dn0, locals.var_wk_dn2, locals.var_wk_dn4, locals.var_wk_dn5, locals.var_wk_dn6, locals.var_wk_dn7, locals.var_wk_dn8, locals.var_wk_dn9, locals.var_wk_dn10, locals.var_wk_dn13,)
    }
};
        locals.var_wk = assign64540_e99554;
        locals.var_wk_dn0 = assign64540_e99554_d_n0;
        locals.var_wk_dn2 = assign64540_e99554_d_n2;
        locals.var_wk_dn4 = assign64540_e99554_d_n4;
        locals.var_wk_dn5 = assign64540_e99554_d_n5;
        locals.var_wk_dn6 = assign64540_e99554_d_n6;
        locals.var_wk_dn7 = assign64540_e99554_d_n7;
        locals.var_wk_dn8 = assign64540_e99554_d_n8;
        locals.var_wk_dn9 = assign64540_e99554_d_n9;
        locals.var_wk_dn10 = assign64540_e99554_d_n10;
        locals.var_wk_dn13 = assign64540_e99554_d_n13;
        locals.var_wk_rv = 0.0;

        let (assign64550_e99570, assign64550_e99570_d_n0, assign64550_e99570_d_n2, assign64550_e99570_d_n4, assign64550_e99570_d_n5, assign64550_e99570_d_n6, assign64550_e99570_d_n7, assign64550_e99570_d_n8, assign64550_e99570_d_n9, assign64550_e99570_d_n10, assign64550_e99570_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64550_e99560: f64 = (-0.25);
        let assign64550_e99562: f64 = (assign64550_e99560 * locals.var_wk);
        let assign64550_e99564: f64 = (assign64550_e99562 * locals.var_wk);
        let assign64550_e99567: f64 = (locals.var_vds + locals.var_wk);
        let assign64550_e99568: f64 = (assign64550_e99564 / assign64550_e99567);
        (assign64550_e99568, ((((((assign64550_e99560 * locals.var_wk_dn0) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn0)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn0 + locals.var_wk_dn0))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn2) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn2)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn2 + locals.var_wk_dn2))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn4) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn4)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn4 + locals.var_wk_dn4))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn5) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn5)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn5 + locals.var_wk_dn5))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn6) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn6)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn6 + locals.var_wk_dn6))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn7) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn7)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn7 + locals.var_wk_dn7))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn8) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn8)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn8 + locals.var_wk_dn8))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn9) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn9)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn9 + locals.var_wk_dn9))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn10) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn10)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn10 + locals.var_wk_dn10))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn13) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn13)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn13 + locals.var_wk_dn13))) / (assign64550_e99567 * assign64550_e99567)),)
    } else {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn13,)
    }
};
        locals.var_dphi_vds = assign64550_e99570;
        locals.var_dphi_vds_dn0 = assign64550_e99570_d_n0;
        locals.var_dphi_vds_dn2 = assign64550_e99570_d_n2;
        locals.var_dphi_vds_dn4 = assign64550_e99570_d_n4;
        locals.var_dphi_vds_dn5 = assign64550_e99570_d_n5;
        locals.var_dphi_vds_dn6 = assign64550_e99570_d_n6;
        locals.var_dphi_vds_dn7 = assign64550_e99570_d_n7;
        locals.var_dphi_vds_dn8 = assign64550_e99570_d_n8;
        locals.var_dphi_vds_dn9 = assign64550_e99570_d_n9;
        locals.var_dphi_vds_dn10 = assign64550_e99570_d_n10;
        locals.var_dphi_vds_dn13 = assign64550_e99570_d_n13;
        locals.var_dphi_vds_rv = 0.0;

        let assign64560_e99573: f64 = if p.p457 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1538 = assign64560_e99573;
        locals.var_guard1538_rv = 0.0;

        let (assign64570_e99582, assign64570_e99582_d_n0, assign64570_e99582_d_n2, assign64570_e99582_d_n4, assign64570_e99582_d_n5, assign64570_e99582_d_n6, assign64570_e99582_d_n7, assign64570_e99582_d_n8, assign64570_e99582_d_n9, assign64570_e99582_d_n10, assign64570_e99582_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 != 0.0)) {
        (p.p457, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    }
};
        locals.var_ps0__blk1523 = assign64570_e99582;
        locals.var_ps0__blk1523_dn0 = assign64570_e99582_d_n0;
        locals.var_ps0__blk1523_dn2 = assign64570_e99582_d_n2;
        locals.var_ps0__blk1523_dn4 = assign64570_e99582_d_n4;
        locals.var_ps0__blk1523_dn5 = assign64570_e99582_d_n5;
        locals.var_ps0__blk1523_dn6 = assign64570_e99582_d_n6;
        locals.var_ps0__blk1523_dn7 = assign64570_e99582_d_n7;
        locals.var_ps0__blk1523_dn8 = assign64570_e99582_d_n8;
        locals.var_ps0__blk1523_dn9 = assign64570_e99582_d_n9;
        locals.var_ps0__blk1523_dn10 = assign64570_e99582_d_n10;
        locals.var_ps0__blk1523_dn13 = assign64570_e99582_d_n13;
        locals.var_ps0__blk1523_rv = 0.0;

        let (assign64580_e99592, assign64580_e99592_d_n0, assign64580_e99592_d_n2, assign64580_e99592_d_n4, assign64580_e99592_d_n5, assign64580_e99592_d_n6, assign64580_e99592_d_n7, assign64580_e99592_d_n8, assign64580_e99592_d_n9, assign64580_e99592_d_n10, assign64580_e99592_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn13,)
    } else {
        (locals.var_vbscl__blk1539, locals.var_vbscl__blk1539_dn0, locals.var_vbscl__blk1539_dn2, locals.var_vbscl__blk1539_dn4, locals.var_vbscl__blk1539_dn5, locals.var_vbscl__blk1539_dn6, locals.var_vbscl__blk1539_dn7, locals.var_vbscl__blk1539_dn8, locals.var_vbscl__blk1539_dn9, locals.var_vbscl__blk1539_dn10, locals.var_vbscl__blk1539_dn13,)
    }
};
        locals.var_vbscl__blk1539 = assign64580_e99592;
        locals.var_vbscl__blk1539_dn0 = assign64580_e99592_d_n0;
        locals.var_vbscl__blk1539_dn2 = assign64580_e99592_d_n2;
        locals.var_vbscl__blk1539_dn4 = assign64580_e99592_d_n4;
        locals.var_vbscl__blk1539_dn5 = assign64580_e99592_d_n5;
        locals.var_vbscl__blk1539_dn6 = assign64580_e99592_d_n6;
        locals.var_vbscl__blk1539_dn7 = assign64580_e99592_d_n7;
        locals.var_vbscl__blk1539_dn8 = assign64580_e99592_d_n8;
        locals.var_vbscl__blk1539_dn9 = assign64580_e99592_d_n9;
        locals.var_vbscl__blk1539_dn10 = assign64580_e99592_d_n10;
        locals.var_vbscl__blk1539_dn13 = assign64580_e99592_d_n13;
        locals.var_vbscl__blk1539_rv = 0.0;

        let (assign64590_e99602, assign64590_e99602_d_n0, assign64590_e99602_d_n2, assign64590_e99602_d_n4, assign64590_e99602_d_n5, assign64590_e99602_d_n6, assign64590_e99602_d_n7, assign64590_e99602_d_n8, assign64590_e99602_d_n9, assign64590_e99602_d_n10, assign64590_e99602_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn4, locals.var_wk_vth_dn5, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn8, locals.var_wk_vth_dn9, locals.var_wk_vth_dn10, locals.var_wk_vth_dn13,)
    } else {
        (locals.var_vth__blk1540, locals.var_vth__blk1540_dn0, locals.var_vth__blk1540_dn2, locals.var_vth__blk1540_dn4, locals.var_vth__blk1540_dn5, locals.var_vth__blk1540_dn6, locals.var_vth__blk1540_dn7, locals.var_vth__blk1540_dn8, locals.var_vth__blk1540_dn9, locals.var_vth__blk1540_dn10, locals.var_vth__blk1540_dn13,)
    }
};
        locals.var_vth__blk1540 = assign64590_e99602;
        locals.var_vth__blk1540_dn0 = assign64590_e99602_d_n0;
        locals.var_vth__blk1540_dn2 = assign64590_e99602_d_n2;
        locals.var_vth__blk1540_dn4 = assign64590_e99602_d_n4;
        locals.var_vth__blk1540_dn5 = assign64590_e99602_d_n5;
        locals.var_vth__blk1540_dn6 = assign64590_e99602_d_n6;
        locals.var_vth__blk1540_dn7 = assign64590_e99602_d_n7;
        locals.var_vth__blk1540_dn8 = assign64590_e99602_d_n8;
        locals.var_vth__blk1540_dn9 = assign64590_e99602_d_n9;
        locals.var_vth__blk1540_dn10 = assign64590_e99602_d_n10;
        locals.var_vth__blk1540_dn13 = assign64590_e99602_d_n13;
        locals.var_vth__blk1540_rv = 0.0;

        let (assign64600_e99626, assign64600_e99626_d_n0, assign64600_e99626_d_n2, assign64600_e99626_d_n4, assign64600_e99626_d_n5, assign64600_e99626_d_n6, assign64600_e99626_d_n7, assign64600_e99626_d_n8, assign64600_e99626_d_n9, assign64600_e99626_d_n10, assign64600_e99626_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64600_e99615: f64 = (locals.var_vgp__blk1525 - locals.var_vbscl__blk1539);
        let assign64600_e99616: f64 = (locals.var_beta * assign64600_e99615);
        let assign64600_e99618: f64 = (assign64600_e99616 - 1.0);
        let assign64600_e99619: f64 = (4.0 * assign64600_e99618);
        let assign64600_e99622: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign64600_e99623: f64 = (assign64600_e99619 / assign64600_e99622);
        let assign64600_e99624: f64 = (1.0 + assign64600_e99623);
        (assign64600_e99624, ((((4.0 * ((locals.var_beta_dn0 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn0 - locals.var_vbscl__blk1539_dn0)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn2 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn2 - locals.var_vbscl__blk1539_dn2)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn4 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn4 - locals.var_vbscl__blk1539_dn4)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn5 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn5 - locals.var_vbscl__blk1539_dn5)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn6 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn6 - locals.var_vbscl__blk1539_dn6)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn7 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn7 - locals.var_vbscl__blk1539_dn7)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn8 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn8 - locals.var_vbscl__blk1539_dn8)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn9 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn9 - locals.var_vbscl__blk1539_dn9)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn10 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn10 - locals.var_vbscl__blk1539_dn10)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn13 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn13 - locals.var_vbscl__blk1539_dn13)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign64600_e99622 * assign64600_e99622)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64600_e99626;
        locals.var_tx_dn0 = assign64600_e99626_d_n0;
        locals.var_tx_dn2 = assign64600_e99626_d_n2;
        locals.var_tx_dn4 = assign64600_e99626_d_n4;
        locals.var_tx_dn5 = assign64600_e99626_d_n5;
        locals.var_tx_dn6 = assign64600_e99626_d_n6;
        locals.var_tx_dn7 = assign64600_e99626_d_n7;
        locals.var_tx_dn8 = assign64600_e99626_d_n8;
        locals.var_tx_dn9 = assign64600_e99626_d_n9;
        locals.var_tx_dn10 = assign64600_e99626_d_n10;
        locals.var_tx_dn13 = assign64600_e99626_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign64610_e99645, assign64610_e99645_d_n0, assign64610_e99645_d_n2, assign64610_e99645_d_n4, assign64610_e99645_d_n5, assign64610_e99645_d_n6, assign64610_e99645_d_n7, assign64610_e99645_d_n8, assign64610_e99645_d_n9, assign64610_e99645_d_n10, assign64610_e99645_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64610_e99637: f64 = (10.0 * 2.220446049250313e-16);
        let (assign64610_e99643, assign64610_e99643_d_n0, assign64610_e99643_d_n2, assign64610_e99643_d_n4, assign64610_e99643_d_n5, assign64610_e99643_d_n6, assign64610_e99643_d_n7, assign64610_e99643_d_n8, assign64610_e99643_d_n9, assign64610_e99643_d_n10, assign64610_e99643_d_n13,) = {
            if (locals.var_tx >= assign64610_e99637) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
            } else {
                let assign64610_e99642: f64 = (10.0 * 2.220446049250313e-16);
                (assign64610_e99642, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign64610_e99643, assign64610_e99643_d_n0, assign64610_e99643_d_n2, assign64610_e99643_d_n4, assign64610_e99643_d_n5, assign64610_e99643_d_n6, assign64610_e99643_d_n7, assign64610_e99643_d_n8, assign64610_e99643_d_n9, assign64610_e99643_d_n10, assign64610_e99643_d_n13,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64610_e99645;
        locals.var_tx_dn0 = assign64610_e99645_d_n0;
        locals.var_tx_dn2 = assign64610_e99645_d_n2;
        locals.var_tx_dn4 = assign64610_e99645_d_n4;
        locals.var_tx_dn5 = assign64610_e99645_d_n5;
        locals.var_tx_dn6 = assign64610_e99645_d_n6;
        locals.var_tx_dn7 = assign64610_e99645_d_n7;
        locals.var_tx_dn8 = assign64610_e99645_d_n8;
        locals.var_tx_dn9 = assign64610_e99645_d_n9;
        locals.var_tx_dn10 = assign64610_e99645_d_n10;
        locals.var_tx_dn13 = assign64610_e99645_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign64620_e99666, assign64620_e99666_d_n0, assign64620_e99666_d_n2, assign64620_e99666_d_n4, assign64620_e99666_d_n5, assign64620_e99666_d_n6, assign64620_e99666_d_n7, assign64620_e99666_d_n8, assign64620_e99666_d_n9, assign64620_e99666_d_n10, assign64620_e99666_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64620_e99656: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign64620_e99658: f64 = (assign64620_e99656 * 0.5);
        let assign64620_e99661: f64 = (locals.var_tx).sqrt();
        let assign64620_e99662: f64 = (1.0 - assign64620_e99661);
        let assign64620_e99663: f64 = (assign64620_e99658 * assign64620_e99662);
        let assign64620_e99664: f64 = (locals.var_vgp__blk1525 + assign64620_e99663);
        (assign64620_e99664, (locals.var_vgp__blk1525_dn0 + (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn0 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn2 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn4 + (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn4 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn5 + (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn5 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn6 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn7 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn8 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn9 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn10 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn13 + (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn13 / (2.0 * assign64620_e99661)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign64620_e99666;
        locals.var_ps0_inia_dn0 = assign64620_e99666_d_n0;
        locals.var_ps0_inia_dn2 = assign64620_e99666_d_n2;
        locals.var_ps0_inia_dn4 = assign64620_e99666_d_n4;
        locals.var_ps0_inia_dn5 = assign64620_e99666_d_n5;
        locals.var_ps0_inia_dn6 = assign64620_e99666_d_n6;
        locals.var_ps0_inia_dn7 = assign64620_e99666_d_n7;
        locals.var_ps0_inia_dn8 = assign64620_e99666_d_n8;
        locals.var_ps0_inia_dn9 = assign64620_e99666_d_n9;
        locals.var_ps0_inia_dn10 = assign64620_e99666_d_n10;
        locals.var_ps0_inia_dn13 = assign64620_e99666_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign64630_e99680, assign64630_e99680_d_n0, assign64630_e99680_d_n2, assign64630_e99680_d_n4, assign64630_e99680_d_n5, assign64630_e99680_d_n6, assign64630_e99680_d_n7, assign64630_e99680_d_n8, assign64630_e99680_d_n9, assign64630_e99680_d_n10, assign64630_e99680_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64630_e99677: f64 = (locals.var_ps0_inia - locals.var_vbscl__blk1539);
        let assign64630_e99678: f64 = (locals.var_beta * assign64630_e99677);
        (assign64630_e99678, ((locals.var_beta_dn0 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbscl__blk1539_dn0))), ((locals.var_beta_dn2 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbscl__blk1539_dn2))), ((locals.var_beta_dn4 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn4 - locals.var_vbscl__blk1539_dn4))), ((locals.var_beta_dn5 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn5 - locals.var_vbscl__blk1539_dn5))), ((locals.var_beta_dn6 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbscl__blk1539_dn6))), ((locals.var_beta_dn7 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbscl__blk1539_dn7))), ((locals.var_beta_dn8 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn8 - locals.var_vbscl__blk1539_dn8))), ((locals.var_beta_dn9 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn9 - locals.var_vbscl__blk1539_dn9))), ((locals.var_beta_dn10 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbscl__blk1539_dn10))), ((locals.var_beta_dn13 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn13 - locals.var_vbscl__blk1539_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign64630_e99680;
        locals.var_chi_dn0 = assign64630_e99680_d_n0;
        locals.var_chi_dn2 = assign64630_e99680_d_n2;
        locals.var_chi_dn4 = assign64630_e99680_d_n4;
        locals.var_chi_dn5 = assign64630_e99680_d_n5;
        locals.var_chi_dn6 = assign64630_e99680_d_n6;
        locals.var_chi_dn7 = assign64630_e99680_d_n7;
        locals.var_chi_dn8 = assign64630_e99680_d_n8;
        locals.var_chi_dn9 = assign64630_e99680_d_n9;
        locals.var_chi_dn10 = assign64630_e99680_d_n10;
        locals.var_chi_dn13 = assign64630_e99680_d_n13;
        locals.var_chi_rv = 0.0;

        let assign64640_e99683: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1541 = assign64640_e99683;
        locals.var_guard1541_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_232(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64650_e99699, assign64650_e99699_d_n0, assign64650_e99699_d_n2, assign64650_e99699_d_n4, assign64650_e99699_d_n5, assign64650_e99699_d_n6, assign64650_e99699_d_n7, assign64650_e99699_d_n8, assign64650_e99699_d_n9, assign64650_e99699_d_n10, assign64650_e99699_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64650_e99696: f64 = (locals.var_vgp__blk1525 - locals.var_vbscl__blk1539);
        let assign64650_e99697: f64 = (locals.var_beta * assign64650_e99696);
        (assign64650_e99697, ((locals.var_beta_dn0 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn0 - locals.var_vbscl__blk1539_dn0))), ((locals.var_beta_dn2 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn2 - locals.var_vbscl__blk1539_dn2))), ((locals.var_beta_dn4 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn4 - locals.var_vbscl__blk1539_dn4))), ((locals.var_beta_dn5 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn5 - locals.var_vbscl__blk1539_dn5))), ((locals.var_beta_dn6 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn6 - locals.var_vbscl__blk1539_dn6))), ((locals.var_beta_dn7 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn7 - locals.var_vbscl__blk1539_dn7))), ((locals.var_beta_dn8 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn8 - locals.var_vbscl__blk1539_dn8))), ((locals.var_beta_dn9 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn9 - locals.var_vbscl__blk1539_dn9))), ((locals.var_beta_dn10 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn10 - locals.var_vbscl__blk1539_dn10))), ((locals.var_beta_dn13 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn13 - locals.var_vbscl__blk1539_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign64650_e99699;
        locals.var_ty_dn0 = assign64650_e99699_d_n0;
        locals.var_ty_dn2 = assign64650_e99699_d_n2;
        locals.var_ty_dn4 = assign64650_e99699_d_n4;
        locals.var_ty_dn5 = assign64650_e99699_d_n5;
        locals.var_ty_dn6 = assign64650_e99699_d_n6;
        locals.var_ty_dn7 = assign64650_e99699_d_n7;
        locals.var_ty_dn8 = assign64650_e99699_d_n8;
        locals.var_ty_dn9 = assign64650_e99699_d_n9;
        locals.var_ty_dn10 = assign64650_e99699_d_n10;
        locals.var_ty_dn13 = assign64650_e99699_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign64660_e99719, assign64660_e99719_d_n0, assign64660_e99719_d_n2, assign64660_e99719_d_n4, assign64660_e99719_d_n5, assign64660_e99719_d_n6, assign64660_e99719_d_n7, assign64660_e99719_d_n8, assign64660_e99719_d_n9, assign64660_e99719_d_n10, assign64660_e99719_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64660_e99712: f64 = (1.414213562373095 / 108.0);
        let assign64660_e99714: f64 = (assign64660_e99712 * locals.var_beta);
        let assign64660_e99716: f64 = (assign64660_e99714 * locals.var_fac1);
        let assign64660_e99717: f64 = (1.0 / assign64660_e99716);
        (assign64660_e99717, (-((((assign64660_e99712 * locals.var_beta_dn0) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn0)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn2) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn2)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn4) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn4)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn5) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn5)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn6) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn6)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn7) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn7)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn8) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn8)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn9) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn9)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn10) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn10)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn13) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn13)) / (assign64660_e99716 * assign64660_e99716))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign64660_e99719;
        locals.var_t1_dn0 = assign64660_e99719_d_n0;
        locals.var_t1_dn2 = assign64660_e99719_d_n2;
        locals.var_t1_dn4 = assign64660_e99719_d_n4;
        locals.var_t1_dn5 = assign64660_e99719_d_n5;
        locals.var_t1_dn6 = assign64660_e99719_d_n6;
        locals.var_t1_dn7 = assign64660_e99719_d_n7;
        locals.var_t1_dn8 = assign64660_e99719_d_n8;
        locals.var_t1_dn9 = assign64660_e99719_d_n9;
        locals.var_t1_dn10 = assign64660_e99719_d_n10;
        locals.var_t1_dn13 = assign64660_e99719_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign64670_e99735, assign64670_e99735_d_n0, assign64670_e99735_d_n2, assign64670_e99735_d_n4, assign64670_e99735_d_n5, assign64670_e99735_d_n6, assign64670_e99735_d_n7, assign64670_e99735_d_n8, assign64670_e99735_d_n9, assign64670_e99735_d_n10, assign64670_e99735_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64670_e99732: f64 = (3.0 * locals.var_t1);
        let assign64670_e99733: f64 = (81.0 + assign64670_e99732);
        (assign64670_e99733, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn4), (3.0 * locals.var_t1_dn5), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn8), (3.0 * locals.var_t1_dn9), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign64670_e99735;
        locals.var_t2_dn0 = assign64670_e99735_d_n0;
        locals.var_t2_dn2 = assign64670_e99735_d_n2;
        locals.var_t2_dn4 = assign64670_e99735_d_n4;
        locals.var_t2_dn5 = assign64670_e99735_d_n5;
        locals.var_t2_dn6 = assign64670_e99735_d_n6;
        locals.var_t2_dn7 = assign64670_e99735_d_n7;
        locals.var_t2_dn8 = assign64670_e99735_d_n8;
        locals.var_t2_dn9 = assign64670_e99735_d_n9;
        locals.var_t2_dn10 = assign64670_e99735_d_n10;
        locals.var_t2_dn13 = assign64670_e99735_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign64680_e99758, assign64680_e99758_d_n0, assign64680_e99758_d_n2, assign64680_e99758_d_n4, assign64680_e99758_d_n5, assign64680_e99758_d_n6, assign64680_e99758_d_n7, assign64680_e99758_d_n8, assign64680_e99758_d_n9, assign64680_e99758_d_n10, assign64680_e99758_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64680_e99746: f64 = (-2916.0);
        let assign64680_e99749: f64 = (81.0 * locals.var_t1);
        let assign64680_e99750: f64 = (assign64680_e99746 - assign64680_e99749);
        let assign64680_e99753: f64 = (27.0 * locals.var_t1);
        let assign64680_e99755: f64 = (assign64680_e99753 * locals.var_ty);
        let assign64680_e99756: f64 = (assign64680_e99750 + assign64680_e99755);
        (assign64680_e99756, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn13)) + (((27.0 * locals.var_t1_dn13) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn13))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign64680_e99758;
        locals.var_t3_dn0 = assign64680_e99758_d_n0;
        locals.var_t3_dn2 = assign64680_e99758_d_n2;
        locals.var_t3_dn4 = assign64680_e99758_d_n4;
        locals.var_t3_dn5 = assign64680_e99758_d_n5;
        locals.var_t3_dn6 = assign64680_e99758_d_n6;
        locals.var_t3_dn7 = assign64680_e99758_d_n7;
        locals.var_t3_dn8 = assign64680_e99758_d_n8;
        locals.var_t3_dn9 = assign64680_e99758_d_n9;
        locals.var_t3_dn10 = assign64680_e99758_d_n10;
        locals.var_t3_dn13 = assign64680_e99758_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign64690_e99782, assign64690_e99782_d_n0, assign64690_e99782_d_n2, assign64690_e99782_d_n4, assign64690_e99782_d_n5, assign64690_e99782_d_n6, assign64690_e99782_d_n7, assign64690_e99782_d_n8, assign64690_e99782_d_n9, assign64690_e99782_d_n10, assign64690_e99782_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64690_e99772: f64 = (54.0 + locals.var_t1);
        let assign64690_e99773: f64 = (81.0 * assign64690_e99772);
        let assign64690_e99774: f64 = (1458.0 - assign64690_e99773);
        let assign64690_e99777: f64 = (27.0 * locals.var_t1);
        let assign64690_e99779: f64 = (assign64690_e99777 * locals.var_ty);
        let assign64690_e99780: f64 = (assign64690_e99774 + assign64690_e99779);
        (assign64690_e99780, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn13)) + (((27.0 * locals.var_t1_dn13) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign64690_e99782;
        locals.var_t4_dn0 = assign64690_e99782_d_n0;
        locals.var_t4_dn2 = assign64690_e99782_d_n2;
        locals.var_t4_dn4 = assign64690_e99782_d_n4;
        locals.var_t4_dn5 = assign64690_e99782_d_n5;
        locals.var_t4_dn6 = assign64690_e99782_d_n6;
        locals.var_t4_dn7 = assign64690_e99782_d_n7;
        locals.var_t4_dn8 = assign64690_e99782_d_n8;
        locals.var_t4_dn9 = assign64690_e99782_d_n9;
        locals.var_t4_dn10 = assign64690_e99782_d_n10;
        locals.var_t4_dn13 = assign64690_e99782_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign64700_e99796, assign64700_e99796_d_n0, assign64700_e99796_d_n2, assign64700_e99796_d_n4, assign64700_e99796_d_n5, assign64700_e99796_d_n6, assign64700_e99796_d_n7, assign64700_e99796_d_n8, assign64700_e99796_d_n9, assign64700_e99796_d_n10, assign64700_e99796_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64700_e99794: f64 = (locals.var_t4 * locals.var_t4);
        (assign64700_e99794, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn13 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign64700_e99796;
        locals.var_t4_dn0 = assign64700_e99796_d_n0;
        locals.var_t4_dn2 = assign64700_e99796_d_n2;
        locals.var_t4_dn4 = assign64700_e99796_d_n4;
        locals.var_t4_dn5 = assign64700_e99796_d_n5;
        locals.var_t4_dn6 = assign64700_e99796_d_n6;
        locals.var_t4_dn7 = assign64700_e99796_d_n7;
        locals.var_t4_dn8 = assign64700_e99796_d_n8;
        locals.var_t4_dn9 = assign64700_e99796_d_n9;
        locals.var_t4_dn10 = assign64700_e99796_d_n10;
        locals.var_t4_dn13 = assign64700_e99796_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign64710_e99837, assign64710_e99837_d_n0, assign64710_e99837_d_n2, assign64710_e99837_d_n4, assign64710_e99837_d_n5, assign64710_e99837_d_n6, assign64710_e99837_d_n7, assign64710_e99837_d_n8, assign64710_e99837_d_n9, assign64710_e99837_d_n10, assign64710_e99837_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64710_e99809: f64 = (4.0 * locals.var_t2);
        let assign64710_e99811: f64 = (assign64710_e99809 * locals.var_t2);
        let assign64710_e99813: f64 = (assign64710_e99811 * locals.var_t2);
        let assign64710_e99815: f64 = (assign64710_e99813 + locals.var_t4);
        let assign64710_e99816: f64 = (assign64710_e99815).sqrt();
        let assign64710_e99817: f64 = (locals.var_t3 + assign64710_e99816);
        let (assign64710_e99835, assign64710_e99835_d_n0, assign64710_e99835_d_n2, assign64710_e99835_d_n4, assign64710_e99835_d_n5, assign64710_e99835_d_n6, assign64710_e99835_d_n7, assign64710_e99835_d_n8, assign64710_e99835_d_n9, assign64710_e99835_d_n10, assign64710_e99835_d_n13,) = {
            if (assign64710_e99817 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign64710_e99824: f64 = (4.0 * locals.var_t2);
                let assign64710_e99826: f64 = (assign64710_e99824 * locals.var_t2);
                let assign64710_e99828: f64 = (assign64710_e99826 * locals.var_t2);
                let assign64710_e99830: f64 = (assign64710_e99828 + locals.var_t4);
                let assign64710_e99831: f64 = (assign64710_e99830).sqrt();
                let assign64710_e99832: f64 = (locals.var_t3 + assign64710_e99831);
                let assign64710_e99834: f64 = (assign64710_e99832).powf(0.3333333333333333);
                (assign64710_e99834, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn0)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn0)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn2)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn2)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn4)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn4)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn5)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn5)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn6)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn6)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn7)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn7)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn8)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn8)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn9)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn9)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn10)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn10)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn13 + (((((((4.0 * locals.var_t2_dn13) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn13)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn13)) + locals.var_t4_dn13) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn13 + (((((((4.0 * locals.var_t2_dn13) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn13)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn13)) + locals.var_t4_dn13) / (2.0 * assign64710_e99831))) / assign64710_e99832))) },)
            }
        };
        (assign64710_e99835, assign64710_e99835_d_n0, assign64710_e99835_d_n2, assign64710_e99835_d_n4, assign64710_e99835_d_n5, assign64710_e99835_d_n6, assign64710_e99835_d_n7, assign64710_e99835_d_n8, assign64710_e99835_d_n9, assign64710_e99835_d_n10, assign64710_e99835_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign64710_e99837;
        locals.var_t5_dn0 = assign64710_e99837_d_n0;
        locals.var_t5_dn2 = assign64710_e99837_d_n2;
        locals.var_t5_dn4 = assign64710_e99837_d_n4;
        locals.var_t5_dn5 = assign64710_e99837_d_n5;
        locals.var_t5_dn6 = assign64710_e99837_d_n6;
        locals.var_t5_dn7 = assign64710_e99837_d_n7;
        locals.var_t5_dn8 = assign64710_e99837_d_n8;
        locals.var_t5_dn9 = assign64710_e99837_d_n9;
        locals.var_t5_dn10 = assign64710_e99837_d_n10;
        locals.var_t5_dn13 = assign64710_e99837_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign64720_e99865, assign64720_e99865_d_n0, assign64720_e99865_d_n2, assign64720_e99865_d_n4, assign64720_e99865_d_n5, assign64720_e99865_d_n6, assign64720_e99865_d_n7, assign64720_e99865_d_n8, assign64720_e99865_d_n9, assign64720_e99865_d_n10, assign64720_e99865_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64720_e99850: f64 = (1.259921049894873 * locals.var_t2);
        let assign64720_e99853: f64 = (3.0 * locals.var_t5);
        let assign64720_e99854: f64 = (assign64720_e99850 / assign64720_e99853);
        let assign64720_e99855: f64 = (3.0 - assign64720_e99854);
        let assign64720_e99859: f64 = (3.0 * 1.259921049894873);
        let assign64720_e99860: f64 = (1.0 / assign64720_e99859);
        let assign64720_e99862: f64 = (assign64720_e99860 * locals.var_t5);
        let assign64720_e99863: f64 = (assign64720_e99855 + assign64720_e99862);
        (assign64720_e99863, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn0))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn2))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn4) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn4))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn4)), ((-((((1.259921049894873 * locals.var_t2_dn5) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn5))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn5)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn6))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn7))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn8) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn8))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn8)), ((-((((1.259921049894873 * locals.var_t2_dn9) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn9))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn9)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn10))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn13) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn13))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64720_e99865;
        locals.var_tx_dn0 = assign64720_e99865_d_n0;
        locals.var_tx_dn2 = assign64720_e99865_d_n2;
        locals.var_tx_dn4 = assign64720_e99865_d_n4;
        locals.var_tx_dn5 = assign64720_e99865_d_n5;
        locals.var_tx_dn6 = assign64720_e99865_d_n6;
        locals.var_tx_dn7 = assign64720_e99865_d_n7;
        locals.var_tx_dn8 = assign64720_e99865_d_n8;
        locals.var_tx_dn9 = assign64720_e99865_d_n9;
        locals.var_tx_dn10 = assign64720_e99865_d_n10;
        locals.var_tx_dn13 = assign64720_e99865_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign64730_e99881, assign64730_e99881_d_n0, assign64730_e99881_d_n2, assign64730_e99881_d_n4, assign64730_e99881_d_n5, assign64730_e99881_d_n6, assign64730_e99881_d_n7, assign64730_e99881_d_n8, assign64730_e99881_d_n9, assign64730_e99881_d_n10, assign64730_e99881_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64730_e99877: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign64730_e99879: f64 = (assign64730_e99877 + locals.var_vbscl__blk1539);
        (assign64730_e99879, (((locals.var_tx_dn0 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk1539_dn0), (((locals.var_tx_dn2 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk1539_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk1539_dn4), (((locals.var_tx_dn5 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk1539_dn5), (((locals.var_tx_dn6 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk1539_dn6), (((locals.var_tx_dn7 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk1539_dn7), (((locals.var_tx_dn8 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk1539_dn8), (((locals.var_tx_dn9 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk1539_dn9), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk1539_dn10), (((locals.var_tx_dn13 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn13)) + locals.var_vbscl__blk1539_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign64730_e99881;
        locals.var_ps0_inia_dn0 = assign64730_e99881_d_n0;
        locals.var_ps0_inia_dn2 = assign64730_e99881_d_n2;
        locals.var_ps0_inia_dn4 = assign64730_e99881_d_n4;
        locals.var_ps0_inia_dn5 = assign64730_e99881_d_n5;
        locals.var_ps0_inia_dn6 = assign64730_e99881_d_n6;
        locals.var_ps0_inia_dn7 = assign64730_e99881_d_n7;
        locals.var_ps0_inia_dn8 = assign64730_e99881_d_n8;
        locals.var_ps0_inia_dn9 = assign64730_e99881_d_n9;
        locals.var_ps0_inia_dn10 = assign64730_e99881_d_n10;
        locals.var_ps0_inia_dn13 = assign64730_e99881_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign64740_e99893, assign64740_e99893_d_n0, assign64740_e99893_d_n2, assign64740_e99893_d_n4, assign64740_e99893_d_n5, assign64740_e99893_d_n6, assign64740_e99893_d_n7, assign64740_e99893_d_n8, assign64740_e99893_d_n9, assign64740_e99893_d_n10, assign64740_e99893_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64740_e99893;
        locals.var_ps0_ini_dn0 = assign64740_e99893_d_n0;
        locals.var_ps0_ini_dn2 = assign64740_e99893_d_n2;
        locals.var_ps0_ini_dn4 = assign64740_e99893_d_n4;
        locals.var_ps0_ini_dn5 = assign64740_e99893_d_n5;
        locals.var_ps0_ini_dn6 = assign64740_e99893_d_n6;
        locals.var_ps0_ini_dn7 = assign64740_e99893_d_n7;
        locals.var_ps0_ini_dn8 = assign64740_e99893_d_n8;
        locals.var_ps0_ini_dn9 = assign64740_e99893_d_n9;
        locals.var_ps0_ini_dn10 = assign64740_e99893_d_n10;
        locals.var_ps0_ini_dn13 = assign64740_e99893_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let assign64750_e99896: f64 = if locals.var_vgs <= locals.var_vth__blk1540 { 1.0 } else { 0.0 };
        locals.var_guard1542 = assign64750_e99896;
        locals.var_guard1542_rv = 0.0;

        let (assign64760_e99911, assign64760_e99911_d_n0, assign64760_e99911_d_n2, assign64760_e99911_d_n4, assign64760_e99911_d_n5, assign64760_e99911_d_n6, assign64760_e99911_d_n7, assign64760_e99911_d_n8, assign64760_e99911_d_n9, assign64760_e99911_d_n10, assign64760_e99911_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64760_e99911;
        locals.var_ps0_ini_dn0 = assign64760_e99911_d_n0;
        locals.var_ps0_ini_dn2 = assign64760_e99911_d_n2;
        locals.var_ps0_ini_dn4 = assign64760_e99911_d_n4;
        locals.var_ps0_ini_dn5 = assign64760_e99911_d_n5;
        locals.var_ps0_ini_dn6 = assign64760_e99911_d_n6;
        locals.var_ps0_ini_dn7 = assign64760_e99911_d_n7;
        locals.var_ps0_ini_dn8 = assign64760_e99911_d_n8;
        locals.var_ps0_ini_dn9 = assign64760_e99911_d_n9;
        locals.var_ps0_ini_dn10 = assign64760_e99911_d_n10;
        locals.var_ps0_ini_dn13 = assign64760_e99911_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64770_e99931, assign64770_e99931_d_n0, assign64770_e99931_d_n2, assign64770_e99931_d_n4, assign64770_e99931_d_n5, assign64770_e99931_d_n6, assign64770_e99931_d_n7, assign64770_e99931_d_n8, assign64770_e99931_d_n9, assign64770_e99931_d_n10, assign64770_e99931_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64770_e99927: f64 = (1.0 / locals.var_cnst1);
        let assign64770_e99929: f64 = (assign64770_e99927 / locals.var_cnstcoxi);
        (assign64770_e99929, ((((-(locals.var_cnst1_dn0 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn0)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn2 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn2)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn4 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn4)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn5 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn5)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn6 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn6)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn7 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn7)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn8 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn8)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn9 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn9)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn10 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn10)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn13 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn13)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign64770_e99931;
        locals.var_t1_dn0 = assign64770_e99931_d_n0;
        locals.var_t1_dn2 = assign64770_e99931_d_n2;
        locals.var_t1_dn4 = assign64770_e99931_d_n4;
        locals.var_t1_dn5 = assign64770_e99931_d_n5;
        locals.var_t1_dn6 = assign64770_e99931_d_n6;
        locals.var_t1_dn7 = assign64770_e99931_d_n7;
        locals.var_t1_dn8 = assign64770_e99931_d_n8;
        locals.var_t1_dn9 = assign64770_e99931_d_n9;
        locals.var_t1_dn10 = assign64770_e99931_d_n10;
        locals.var_t1_dn13 = assign64770_e99931_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign64780_e99951, assign64780_e99951_d_n0, assign64780_e99951_d_n2, assign64780_e99951_d_n4, assign64780_e99951_d_n5, assign64780_e99951_d_n6, assign64780_e99951_d_n7, assign64780_e99951_d_n8, assign64780_e99951_d_n9, assign64780_e99951_d_n10, assign64780_e99951_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64780_e99947: f64 = (locals.var_t1 * locals.var_vgp__blk1525);
        let assign64780_e99949: f64 = (assign64780_e99947 * locals.var_vgp__blk1525);
        (assign64780_e99949, ((((locals.var_t1_dn0 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn0)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn2)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn2)), ((((locals.var_t1_dn4 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn4)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn4)), ((((locals.var_t1_dn5 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn5)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn5)), ((((locals.var_t1_dn6 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn6)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn7)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn7)), ((((locals.var_t1_dn8 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn8)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn8)), ((((locals.var_t1_dn9 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn9)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn9)), ((((locals.var_t1_dn10 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn10)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn10)), ((((locals.var_t1_dn13 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn13)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign64780_e99951;
        locals.var_t2_dn0 = assign64780_e99951_d_n0;
        locals.var_t2_dn2 = assign64780_e99951_d_n2;
        locals.var_t2_dn4 = assign64780_e99951_d_n4;
        locals.var_t2_dn5 = assign64780_e99951_d_n5;
        locals.var_t2_dn6 = assign64780_e99951_d_n6;
        locals.var_t2_dn7 = assign64780_e99951_d_n7;
        locals.var_t2_dn8 = assign64780_e99951_d_n8;
        locals.var_t2_dn9 = assign64780_e99951_d_n9;
        locals.var_t2_dn10 = assign64780_e99951_d_n10;
        locals.var_t2_dn13 = assign64780_e99951_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign64790_e99971, assign64790_e99971_d_n0, assign64790_e99971_d_n2, assign64790_e99971_d_n4, assign64790_e99971_d_n5, assign64790_e99971_d_n6, assign64790_e99971_d_n7, assign64790_e99971_d_n8, assign64790_e99971_d_n9, assign64790_e99971_d_n10, assign64790_e99971_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64790_e99968: f64 = (2.0 / locals.var_vgp__blk1525);
        let assign64790_e99969: f64 = (locals.var_beta + assign64790_e99968);
        (assign64790_e99969, (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp__blk1525_dn0) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp__blk1525_dn2) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp__blk1525_dn4) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp__blk1525_dn5) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp__blk1525_dn6) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp__blk1525_dn7) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp__blk1525_dn8) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp__blk1525_dn9) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp__blk1525_dn10) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn13 + (-((2.0 * locals.var_vgp__blk1525_dn13) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign64790_e99971;
        locals.var_t3_dn0 = assign64790_e99971_d_n0;
        locals.var_t3_dn2 = assign64790_e99971_d_n2;
        locals.var_t3_dn4 = assign64790_e99971_d_n4;
        locals.var_t3_dn5 = assign64790_e99971_d_n5;
        locals.var_t3_dn6 = assign64790_e99971_d_n6;
        locals.var_t3_dn7 = assign64790_e99971_d_n7;
        locals.var_t3_dn8 = assign64790_e99971_d_n8;
        locals.var_t3_dn9 = assign64790_e99971_d_n9;
        locals.var_t3_dn10 = assign64790_e99971_d_n10;
        locals.var_t3_dn13 = assign64790_e99971_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign64800_e99992, assign64800_e99992_d_n0, assign64800_e99992_d_n2, assign64800_e99992_d_n4, assign64800_e99992_d_n5, assign64800_e99992_d_n6, assign64800_e99992_d_n7, assign64800_e99992_d_n8, assign64800_e99992_d_n9, assign64800_e99992_d_n10, assign64800_e99992_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64800_e99986: f64 = (locals.var_t2).ln();
        let assign64800_e99988: f64 = (assign64800_e99986 / locals.var_t3);
        let assign64800_e99990: f64 = (assign64800_e99988 + p.p456);
        (assign64800_e99990, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn4 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn5 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn8 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn9 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn13 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn9, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn13,)
    }
};
        locals.var_ps0_inib = assign64800_e99992;
        locals.var_ps0_inib_dn0 = assign64800_e99992_d_n0;
        locals.var_ps0_inib_dn2 = assign64800_e99992_d_n2;
        locals.var_ps0_inib_dn4 = assign64800_e99992_d_n4;
        locals.var_ps0_inib_dn5 = assign64800_e99992_d_n5;
        locals.var_ps0_inib_dn6 = assign64800_e99992_d_n6;
        locals.var_ps0_inib_dn7 = assign64800_e99992_d_n7;
        locals.var_ps0_inib_dn8 = assign64800_e99992_d_n8;
        locals.var_ps0_inib_dn9 = assign64800_e99992_d_n9;
        locals.var_ps0_inib_dn10 = assign64800_e99992_d_n10;
        locals.var_ps0_inib_dn13 = assign64800_e99992_d_n13;
        locals.var_ps0_inib_rv = 0.0;

        let (assign64810_e100012, assign64810_e100012_d_n0, assign64810_e100012_d_n2, assign64810_e100012_d_n4, assign64810_e100012_d_n5, assign64810_e100012_d_n6, assign64810_e100012_d_n7, assign64810_e100012_d_n8, assign64810_e100012_d_n9, assign64810_e100012_d_n10, assign64810_e100012_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64810_e100008: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign64810_e100010: f64 = (assign64810_e100008 - 0.0008);
        (assign64810_e100010, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn4 - locals.var_ps0_inia_dn4), (locals.var_ps0_inib_dn5 - locals.var_ps0_inia_dn5), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn8 - locals.var_ps0_inia_dn8), (locals.var_ps0_inib_dn9 - locals.var_ps0_inia_dn9), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn13 - locals.var_ps0_inia_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign64810_e100012;
        locals.var_tmf1_dn0 = assign64810_e100012_d_n0;
        locals.var_tmf1_dn2 = assign64810_e100012_d_n2;
        locals.var_tmf1_dn4 = assign64810_e100012_d_n4;
        locals.var_tmf1_dn5 = assign64810_e100012_d_n5;
        locals.var_tmf1_dn6 = assign64810_e100012_d_n6;
        locals.var_tmf1_dn7 = assign64810_e100012_d_n7;
        locals.var_tmf1_dn8 = assign64810_e100012_d_n8;
        locals.var_tmf1_dn9 = assign64810_e100012_d_n9;
        locals.var_tmf1_dn10 = assign64810_e100012_d_n10;
        locals.var_tmf1_dn13 = assign64810_e100012_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign64820_e100032, assign64820_e100032_d_n0, assign64820_e100032_d_n2, assign64820_e100032_d_n4, assign64820_e100032_d_n5, assign64820_e100032_d_n6, assign64820_e100032_d_n7, assign64820_e100032_d_n8, assign64820_e100032_d_n9, assign64820_e100032_d_n10, assign64820_e100032_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64820_e100028: f64 = (4.0 * locals.var_ps0_inib);
        let assign64820_e100030: f64 = (assign64820_e100028 * 0.0008);
        (assign64820_e100030, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn4) * 0.0008), ((4.0 * locals.var_ps0_inib_dn5) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn8) * 0.0008), ((4.0 * locals.var_ps0_inib_dn9) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn13) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64820_e100032;
        locals.var_tmf2_dn0 = assign64820_e100032_d_n0;
        locals.var_tmf2_dn2 = assign64820_e100032_d_n2;
        locals.var_tmf2_dn4 = assign64820_e100032_d_n4;
        locals.var_tmf2_dn5 = assign64820_e100032_d_n5;
        locals.var_tmf2_dn6 = assign64820_e100032_d_n6;
        locals.var_tmf2_dn7 = assign64820_e100032_d_n7;
        locals.var_tmf2_dn8 = assign64820_e100032_d_n8;
        locals.var_tmf2_dn9 = assign64820_e100032_d_n9;
        locals.var_tmf2_dn10 = assign64820_e100032_d_n10;
        locals.var_tmf2_dn13 = assign64820_e100032_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign64830_e100054, assign64830_e100054_d_n0, assign64830_e100054_d_n2, assign64830_e100054_d_n4, assign64830_e100054_d_n5, assign64830_e100054_d_n6, assign64830_e100054_d_n7, assign64830_e100054_d_n8, assign64830_e100054_d_n9, assign64830_e100054_d_n10, assign64830_e100054_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let (assign64830_e100052, assign64830_e100052_d_n0, assign64830_e100052_d_n2, assign64830_e100052_d_n4, assign64830_e100052_d_n5, assign64830_e100052_d_n6, assign64830_e100052_d_n7, assign64830_e100052_d_n8, assign64830_e100052_d_n9, assign64830_e100052_d_n10, assign64830_e100052_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign64830_e100051: f64 = (-locals.var_tmf2);
                (assign64830_e100051, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign64830_e100052, assign64830_e100052_d_n0, assign64830_e100052_d_n2, assign64830_e100052_d_n4, assign64830_e100052_d_n5, assign64830_e100052_d_n6, assign64830_e100052_d_n7, assign64830_e100052_d_n8, assign64830_e100052_d_n9, assign64830_e100052_d_n10, assign64830_e100052_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64830_e100054;
        locals.var_tmf2_dn0 = assign64830_e100054_d_n0;
        locals.var_tmf2_dn2 = assign64830_e100054_d_n2;
        locals.var_tmf2_dn4 = assign64830_e100054_d_n4;
        locals.var_tmf2_dn5 = assign64830_e100054_d_n5;
        locals.var_tmf2_dn6 = assign64830_e100054_d_n6;
        locals.var_tmf2_dn7 = assign64830_e100054_d_n7;
        locals.var_tmf2_dn8 = assign64830_e100054_d_n8;
        locals.var_tmf2_dn9 = assign64830_e100054_d_n9;
        locals.var_tmf2_dn10 = assign64830_e100054_d_n10;
        locals.var_tmf2_dn13 = assign64830_e100054_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign64840_e100075, assign64840_e100075_d_n0, assign64840_e100075_d_n2, assign64840_e100075_d_n4, assign64840_e100075_d_n5, assign64840_e100075_d_n6, assign64840_e100075_d_n7, assign64840_e100075_d_n8, assign64840_e100075_d_n9, assign64840_e100075_d_n10, assign64840_e100075_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64840_e100070: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign64840_e100072: f64 = (assign64840_e100070 + locals.var_tmf2);
        let assign64840_e100073: f64 = (assign64840_e100072).sqrt();
        (assign64840_e100073, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign64840_e100073)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64840_e100075;
        locals.var_tmf2_dn0 = assign64840_e100075_d_n0;
        locals.var_tmf2_dn2 = assign64840_e100075_d_n2;
        locals.var_tmf2_dn4 = assign64840_e100075_d_n4;
        locals.var_tmf2_dn5 = assign64840_e100075_d_n5;
        locals.var_tmf2_dn6 = assign64840_e100075_d_n6;
        locals.var_tmf2_dn7 = assign64840_e100075_d_n7;
        locals.var_tmf2_dn8 = assign64840_e100075_d_n8;
        locals.var_tmf2_dn9 = assign64840_e100075_d_n9;
        locals.var_tmf2_dn10 = assign64840_e100075_d_n10;
        locals.var_tmf2_dn13 = assign64840_e100075_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign64850_e100097, assign64850_e100097_d_n0, assign64850_e100097_d_n2, assign64850_e100097_d_n4, assign64850_e100097_d_n5, assign64850_e100097_d_n6, assign64850_e100097_d_n7, assign64850_e100097_d_n8, assign64850_e100097_d_n9, assign64850_e100097_d_n10, assign64850_e100097_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64850_e100093: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign64850_e100094: f64 = (0.5 * assign64850_e100093);
        let assign64850_e100095: f64 = (locals.var_ps0_inib - assign64850_e100094);
        (assign64850_e100095, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_ps0_inib_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_ps0_inib_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64850_e100097;
        locals.var_ps0_ini_dn0 = assign64850_e100097_d_n0;
        locals.var_ps0_ini_dn2 = assign64850_e100097_d_n2;
        locals.var_ps0_ini_dn4 = assign64850_e100097_d_n4;
        locals.var_ps0_ini_dn5 = assign64850_e100097_d_n5;
        locals.var_ps0_ini_dn6 = assign64850_e100097_d_n6;
        locals.var_ps0_ini_dn7 = assign64850_e100097_d_n7;
        locals.var_ps0_ini_dn8 = assign64850_e100097_d_n8;
        locals.var_ps0_ini_dn9 = assign64850_e100097_d_n9;
        locals.var_ps0_ini_dn10 = assign64850_e100097_d_n10;
        locals.var_ps0_ini_dn13 = assign64850_e100097_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64860_e100111, assign64860_e100111_d_n0, assign64860_e100111_d_n2, assign64860_e100111_d_n4, assign64860_e100111_d_n5, assign64860_e100111_d_n6, assign64860_e100111_d_n7, assign64860_e100111_d_n8, assign64860_e100111_d_n9, assign64860_e100111_d_n10, assign64860_e100111_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64860_e100108: f64 = (1e-12 / 2.0);
        let assign64860_e100109: f64 = (locals.var_vbscl__blk1539 + assign64860_e100108);
        (assign64860_e100109, locals.var_vbscl__blk1539_dn0, locals.var_vbscl__blk1539_dn2, locals.var_vbscl__blk1539_dn4, locals.var_vbscl__blk1539_dn5, locals.var_vbscl__blk1539_dn6, locals.var_vbscl__blk1539_dn7, locals.var_vbscl__blk1539_dn8, locals.var_vbscl__blk1539_dn9, locals.var_vbscl__blk1539_dn10, locals.var_vbscl__blk1539_dn13,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64860_e100111;
        locals.var_tx_dn0 = assign64860_e100111_d_n0;
        locals.var_tx_dn2 = assign64860_e100111_d_n2;
        locals.var_tx_dn4 = assign64860_e100111_d_n4;
        locals.var_tx_dn5 = assign64860_e100111_d_n5;
        locals.var_tx_dn6 = assign64860_e100111_d_n6;
        locals.var_tx_dn7 = assign64860_e100111_d_n7;
        locals.var_tx_dn8 = assign64860_e100111_d_n8;
        locals.var_tx_dn9 = assign64860_e100111_d_n9;
        locals.var_tx_dn10 = assign64860_e100111_d_n10;
        locals.var_tx_dn13 = assign64860_e100111_d_n13;
        locals.var_tx_rv = 0.0;

        let assign64870_e100114: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard1543 = assign64870_e100114;
        locals.var_guard1543_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_233(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64880_e100126, assign64880_e100126_d_n0, assign64880_e100126_d_n2, assign64880_e100126_d_n4, assign64880_e100126_d_n5, assign64880_e100126_d_n6, assign64880_e100126_d_n7, assign64880_e100126_d_n8, assign64880_e100126_d_n9, assign64880_e100126_d_n10, assign64880_e100126_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64880_e100126;
        locals.var_ps0_ini_dn0 = assign64880_e100126_d_n0;
        locals.var_ps0_ini_dn2 = assign64880_e100126_d_n2;
        locals.var_ps0_ini_dn4 = assign64880_e100126_d_n4;
        locals.var_ps0_ini_dn5 = assign64880_e100126_d_n5;
        locals.var_ps0_ini_dn6 = assign64880_e100126_d_n6;
        locals.var_ps0_ini_dn7 = assign64880_e100126_d_n7;
        locals.var_ps0_ini_dn8 = assign64880_e100126_d_n8;
        locals.var_ps0_ini_dn9 = assign64880_e100126_d_n9;
        locals.var_ps0_ini_dn10 = assign64880_e100126_d_n10;
        locals.var_ps0_ini_dn13 = assign64880_e100126_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64890_e100136, assign64890_e100136_d_n0, assign64890_e100136_d_n2, assign64890_e100136_d_n4, assign64890_e100136_d_n5, assign64890_e100136_d_n6, assign64890_e100136_d_n7, assign64890_e100136_d_n8, assign64890_e100136_d_n9, assign64890_e100136_d_n10, assign64890_e100136_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    } else {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    }
};
        locals.var_ps0__blk1523 = assign64890_e100136;
        locals.var_ps0__blk1523_dn0 = assign64890_e100136_d_n0;
        locals.var_ps0__blk1523_dn2 = assign64890_e100136_d_n2;
        locals.var_ps0__blk1523_dn4 = assign64890_e100136_d_n4;
        locals.var_ps0__blk1523_dn5 = assign64890_e100136_d_n5;
        locals.var_ps0__blk1523_dn6 = assign64890_e100136_d_n6;
        locals.var_ps0__blk1523_dn7 = assign64890_e100136_d_n7;
        locals.var_ps0__blk1523_dn8 = assign64890_e100136_d_n8;
        locals.var_ps0__blk1523_dn9 = assign64890_e100136_d_n9;
        locals.var_ps0__blk1523_dn10 = assign64890_e100136_d_n10;
        locals.var_ps0__blk1523_dn13 = assign64890_e100136_d_n13;
        locals.var_ps0__blk1523_rv = 0.0;

        let assign64900_e100139: f64 = if p.p451 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1544 = assign64900_e100139;
        locals.var_guard1544_rv = 0.0;

        let (assign64910_e100151, assign64910_e100151_d_n0, assign64910_e100151_d_n2, assign64910_e100151_d_n4, assign64910_e100151_d_n5, assign64910_e100151_d_n6, assign64910_e100151_d_n7, assign64910_e100151_d_n8, assign64910_e100151_d_n9, assign64910_e100151_d_n10, assign64910_e100151_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64910_e100151;
        locals.var_ps0_ini_dn0 = assign64910_e100151_d_n0;
        locals.var_ps0_ini_dn2 = assign64910_e100151_d_n2;
        locals.var_ps0_ini_dn4 = assign64910_e100151_d_n4;
        locals.var_ps0_ini_dn5 = assign64910_e100151_d_n5;
        locals.var_ps0_ini_dn6 = assign64910_e100151_d_n6;
        locals.var_ps0_ini_dn7 = assign64910_e100151_d_n7;
        locals.var_ps0_ini_dn8 = assign64910_e100151_d_n8;
        locals.var_ps0_ini_dn9 = assign64910_e100151_d_n9;
        locals.var_ps0_ini_dn10 = assign64910_e100151_d_n10;
        locals.var_ps0_ini_dn13 = assign64910_e100151_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64920_e100163, assign64920_e100163_d_n0, assign64920_e100163_d_n2, assign64920_e100163_d_n4, assign64920_e100163_d_n5, assign64920_e100163_d_n6, assign64920_e100163_d_n7, assign64920_e100163_d_n8, assign64920_e100163_d_n9, assign64920_e100163_d_n10, assign64920_e100163_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn13,)
    } else {
        (locals.var_vbscl__blk1545, locals.var_vbscl__blk1545_dn0, locals.var_vbscl__blk1545_dn2, locals.var_vbscl__blk1545_dn4, locals.var_vbscl__blk1545_dn5, locals.var_vbscl__blk1545_dn6, locals.var_vbscl__blk1545_dn7, locals.var_vbscl__blk1545_dn8, locals.var_vbscl__blk1545_dn9, locals.var_vbscl__blk1545_dn10, locals.var_vbscl__blk1545_dn13,)
    }
};
        locals.var_vbscl__blk1545 = assign64920_e100163;
        locals.var_vbscl__blk1545_dn0 = assign64920_e100163_d_n0;
        locals.var_vbscl__blk1545_dn2 = assign64920_e100163_d_n2;
        locals.var_vbscl__blk1545_dn4 = assign64920_e100163_d_n4;
        locals.var_vbscl__blk1545_dn5 = assign64920_e100163_d_n5;
        locals.var_vbscl__blk1545_dn6 = assign64920_e100163_d_n6;
        locals.var_vbscl__blk1545_dn7 = assign64920_e100163_d_n7;
        locals.var_vbscl__blk1545_dn8 = assign64920_e100163_d_n8;
        locals.var_vbscl__blk1545_dn9 = assign64920_e100163_d_n9;
        locals.var_vbscl__blk1545_dn10 = assign64920_e100163_d_n10;
        locals.var_vbscl__blk1545_dn13 = assign64920_e100163_d_n13;
        locals.var_vbscl__blk1545_rv = 0.0;

        let (assign64930_e100183, assign64930_e100183_d_n0, assign64930_e100183_d_n2, assign64930_e100183_d_n4, assign64930_e100183_d_n5, assign64930_e100183_d_n6, assign64930_e100183_d_n7, assign64930_e100183_d_n8, assign64930_e100183_d_n9, assign64930_e100183_d_n10, assign64930_e100183_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) {
        let assign64930_e100175: f64 = (locals.var_vfb - locals.var_dvth);
        let assign64930_e100177: f64 = (assign64930_e100175 + locals.var_dppg);
        let assign64930_e100179: f64 = (assign64930_e100177 + locals.var_vbscl__blk1545);
        let assign64930_e100181: f64 = (assign64930_e100179 + p.p455);
        (assign64930_e100181, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbscl__blk1545_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbscl__blk1545_dn2), (((-locals.var_dvth_dn4) + locals.var_dppg_dn4) + locals.var_vbscl__blk1545_dn4), (((-locals.var_dvth_dn5) + locals.var_dppg_dn5) + locals.var_vbscl__blk1545_dn5), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbscl__blk1545_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbscl__blk1545_dn7), (((-locals.var_dvth_dn8) + locals.var_dppg_dn8) + locals.var_vbscl__blk1545_dn8), (((-locals.var_dvth_dn9) + locals.var_dppg_dn9) + locals.var_vbscl__blk1545_dn9), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbscl__blk1545_dn10), (((-locals.var_dvth_dn13) + locals.var_dppg_dn13) + locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn4, locals.var_vgs_fb_dn5, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn8, locals.var_vgs_fb_dn9, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn13,)
    }
};
        locals.var_vgs_fb = assign64930_e100183;
        locals.var_vgs_fb_dn0 = assign64930_e100183_d_n0;
        locals.var_vgs_fb_dn2 = assign64930_e100183_d_n2;
        locals.var_vgs_fb_dn4 = assign64930_e100183_d_n4;
        locals.var_vgs_fb_dn5 = assign64930_e100183_d_n5;
        locals.var_vgs_fb_dn6 = assign64930_e100183_d_n6;
        locals.var_vgs_fb_dn7 = assign64930_e100183_d_n7;
        locals.var_vgs_fb_dn8 = assign64930_e100183_d_n8;
        locals.var_vgs_fb_dn9 = assign64930_e100183_d_n9;
        locals.var_vgs_fb_dn10 = assign64930_e100183_d_n10;
        locals.var_vgs_fb_dn13 = assign64930_e100183_d_n13;
        locals.var_vgs_fb_rv = 0.0;

        let assign64940_e100186: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard1554 = assign64940_e100186;
        locals.var_guard1554_rv = 0.0;

        let (assign64950_e100201,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64950_e100199: f64 = (-1.0);
        (assign64950_e100199,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign64950_e100201;
        locals.var_flg_zone_rv = 0.0;

        let (assign64960_e100223, assign64960_e100223_d_n0, assign64960_e100223_d_n2, assign64960_e100223_d_n4, assign64960_e100223_d_n5, assign64960_e100223_d_n6, assign64960_e100223_d_n7, assign64960_e100223_d_n8, assign64960_e100223_d_n9, assign64960_e100223_d_n10, assign64960_e100223_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64960_e100215: f64 = (2.0 * locals.var_beta_inv);
        let assign64960_e100217: f64 = (-locals.var_vgs_min);
        let assign64960_e100219: f64 = (assign64960_e100217 / locals.var_fac1);
        let assign64960_e100220: f64 = (assign64960_e100219).ln();
        let assign64960_e100221: f64 = (assign64960_e100215 * assign64960_e100220);
        (assign64960_e100221, (((2.0 * locals.var_beta_inv_dn0) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn2) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn4) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn5) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn6) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn7) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn8) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn9) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn10) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn13) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign64960_e100223;
        locals.var_ps0_min_dn0 = assign64960_e100223_d_n0;
        locals.var_ps0_min_dn2 = assign64960_e100223_d_n2;
        locals.var_ps0_min_dn4 = assign64960_e100223_d_n4;
        locals.var_ps0_min_dn5 = assign64960_e100223_d_n5;
        locals.var_ps0_min_dn6 = assign64960_e100223_d_n6;
        locals.var_ps0_min_dn7 = assign64960_e100223_d_n7;
        locals.var_ps0_min_dn8 = assign64960_e100223_d_n8;
        locals.var_ps0_min_dn9 = assign64960_e100223_d_n9;
        locals.var_ps0_min_dn10 = assign64960_e100223_d_n10;
        locals.var_ps0_min_dn13 = assign64960_e100223_d_n13;
        locals.var_ps0_min_rv = 0.0;

        let (assign64970_e100241, assign64970_e100241_d_n0, assign64970_e100241_d_n2, assign64970_e100241_d_n4, assign64970_e100241_d_n5, assign64970_e100241_d_n6, assign64970_e100241_d_n7, assign64970_e100241_d_n8, assign64970_e100241_d_n9, assign64970_e100241_d_n10, assign64970_e100241_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64970_e100238: f64 = (locals.var_vgp__blk1525 - locals.var_vbscl__blk1545);
        let assign64970_e100239: f64 = (locals.var_beta * assign64970_e100238);
        (assign64970_e100239, ((locals.var_beta_dn0 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn0 - locals.var_vbscl__blk1545_dn0))), ((locals.var_beta_dn2 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn2 - locals.var_vbscl__blk1545_dn2))), ((locals.var_beta_dn4 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn4 - locals.var_vbscl__blk1545_dn4))), ((locals.var_beta_dn5 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn5 - locals.var_vbscl__blk1545_dn5))), ((locals.var_beta_dn6 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn6 - locals.var_vbscl__blk1545_dn6))), ((locals.var_beta_dn7 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn7 - locals.var_vbscl__blk1545_dn7))), ((locals.var_beta_dn8 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn8 - locals.var_vbscl__blk1545_dn8))), ((locals.var_beta_dn9 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn9 - locals.var_vbscl__blk1545_dn9))), ((locals.var_beta_dn10 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn10 - locals.var_vbscl__blk1545_dn10))), ((locals.var_beta_dn13 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn13 - locals.var_vbscl__blk1545_dn13))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64970_e100241;
        locals.var_tx_dn0 = assign64970_e100241_d_n0;
        locals.var_tx_dn2 = assign64970_e100241_d_n2;
        locals.var_tx_dn4 = assign64970_e100241_d_n4;
        locals.var_tx_dn5 = assign64970_e100241_d_n5;
        locals.var_tx_dn6 = assign64970_e100241_d_n6;
        locals.var_tx_dn7 = assign64970_e100241_d_n7;
        locals.var_tx_dn8 = assign64970_e100241_d_n8;
        locals.var_tx_dn9 = assign64970_e100241_d_n9;
        locals.var_tx_dn10 = assign64970_e100241_d_n10;
        locals.var_tx_dn13 = assign64970_e100241_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign64980_e100259, assign64980_e100259_d_n0, assign64980_e100259_d_n2, assign64980_e100259_d_n4, assign64980_e100259_d_n5, assign64980_e100259_d_n6, assign64980_e100259_d_n7, assign64980_e100259_d_n8, assign64980_e100259_d_n9, assign64980_e100259_d_n10, assign64980_e100259_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64980_e100256: f64 = (locals.var_beta * locals.var_cnst0);
        let assign64980_e100257: f64 = (1.0 / assign64980_e100256);
        (assign64980_e100257, (-(((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn13 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn13)) / (assign64980_e100256 * assign64980_e100256))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign64980_e100259;
        locals.var_t1_dn0 = assign64980_e100259_d_n0;
        locals.var_t1_dn2 = assign64980_e100259_d_n2;
        locals.var_t1_dn4 = assign64980_e100259_d_n4;
        locals.var_t1_dn5 = assign64980_e100259_d_n5;
        locals.var_t1_dn6 = assign64980_e100259_d_n6;
        locals.var_t1_dn7 = assign64980_e100259_d_n7;
        locals.var_t1_dn8 = assign64980_e100259_d_n8;
        locals.var_t1_dn9 = assign64980_e100259_d_n9;
        locals.var_t1_dn10 = assign64980_e100259_d_n10;
        locals.var_t1_dn13 = assign64980_e100259_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign64990_e100275, assign64990_e100275_d_n0, assign64990_e100275_d_n2, assign64990_e100275_d_n4, assign64990_e100275_d_n5, assign64990_e100275_d_n6, assign64990_e100275_d_n7, assign64990_e100275_d_n8, assign64990_e100275_d_n9, assign64990_e100275_d_n10, assign64990_e100275_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64990_e100273: f64 = (locals.var_t1 * locals.var_cox);
        (assign64990_e100273, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn13 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign64990_e100275;
        locals.var_ty_dn0 = assign64990_e100275_d_n0;
        locals.var_ty_dn2 = assign64990_e100275_d_n2;
        locals.var_ty_dn4 = assign64990_e100275_d_n4;
        locals.var_ty_dn5 = assign64990_e100275_d_n5;
        locals.var_ty_dn6 = assign64990_e100275_d_n6;
        locals.var_ty_dn7 = assign64990_e100275_d_n7;
        locals.var_ty_dn8 = assign64990_e100275_d_n8;
        locals.var_ty_dn9 = assign64990_e100275_d_n9;
        locals.var_ty_dn10 = assign64990_e100275_d_n10;
        locals.var_ty_dn13 = assign64990_e100275_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign65000_e100295, assign65000_e100295_d_n0, assign65000_e100295_d_n2, assign65000_e100295_d_n4, assign65000_e100295_d_n5, assign65000_e100295_d_n6, assign65000_e100295_d_n7, assign65000_e100295_d_n8, assign65000_e100295_d_n9, assign65000_e100295_d_n10, assign65000_e100295_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65000_e100290: f64 = (3.0 * 1.414213562373095);
        let assign65000_e100292: f64 = (assign65000_e100290 * locals.var_ty);
        let assign65000_e100293: f64 = (2.0 + assign65000_e100292);
        (assign65000_e100293, (assign65000_e100290 * locals.var_ty_dn0), (assign65000_e100290 * locals.var_ty_dn2), (assign65000_e100290 * locals.var_ty_dn4), (assign65000_e100290 * locals.var_ty_dn5), (assign65000_e100290 * locals.var_ty_dn6), (assign65000_e100290 * locals.var_ty_dn7), (assign65000_e100290 * locals.var_ty_dn8), (assign65000_e100290 * locals.var_ty_dn9), (assign65000_e100290 * locals.var_ty_dn10), (assign65000_e100290 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign65000_e100295;
        locals.var_ac41_dn0 = assign65000_e100295_d_n0;
        locals.var_ac41_dn2 = assign65000_e100295_d_n2;
        locals.var_ac41_dn4 = assign65000_e100295_d_n4;
        locals.var_ac41_dn5 = assign65000_e100295_d_n5;
        locals.var_ac41_dn6 = assign65000_e100295_d_n6;
        locals.var_ac41_dn7 = assign65000_e100295_d_n7;
        locals.var_ac41_dn8 = assign65000_e100295_d_n8;
        locals.var_ac41_dn9 = assign65000_e100295_d_n9;
        locals.var_ac41_dn10 = assign65000_e100295_d_n10;
        locals.var_ac41_dn13 = assign65000_e100295_d_n13;
        locals.var_ac41_rv = 0.0;

        let (assign65010_e100315, assign65010_e100315_d_n0, assign65010_e100315_d_n2, assign65010_e100315_d_n4, assign65010_e100315_d_n5, assign65010_e100315_d_n6, assign65010_e100315_d_n7, assign65010_e100315_d_n8, assign65010_e100315_d_n9, assign65010_e100315_d_n10, assign65010_e100315_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65010_e100309: f64 = (8.0 * locals.var_ac41);
        let assign65010_e100311: f64 = (assign65010_e100309 * locals.var_ac41);
        let assign65010_e100313: f64 = (assign65010_e100311 * locals.var_ac41);
        (assign65010_e100313, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign65010_e100315;
        locals.var_ac4_dn0 = assign65010_e100315_d_n0;
        locals.var_ac4_dn2 = assign65010_e100315_d_n2;
        locals.var_ac4_dn4 = assign65010_e100315_d_n4;
        locals.var_ac4_dn5 = assign65010_e100315_d_n5;
        locals.var_ac4_dn6 = assign65010_e100315_d_n6;
        locals.var_ac4_dn7 = assign65010_e100315_d_n7;
        locals.var_ac4_dn8 = assign65010_e100315_d_n8;
        locals.var_ac4_dn9 = assign65010_e100315_d_n9;
        locals.var_ac4_dn10 = assign65010_e100315_d_n10;
        locals.var_ac4_dn13 = assign65010_e100315_d_n13;
        locals.var_ac4_rv = 0.0;

        let (assign65020_e100331, assign65020_e100331_d_n0, assign65020_e100331_d_n2, assign65020_e100331_d_n4, assign65020_e100331_d_n5, assign65020_e100331_d_n6, assign65020_e100331_d_n7, assign65020_e100331_d_n8, assign65020_e100331_d_n9, assign65020_e100331_d_n10, assign65020_e100331_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65020_e100329: f64 = (locals.var_tx - 2.0);
        (assign65020_e100329, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign65020_e100331;
        locals.var_t4_dn0 = assign65020_e100331_d_n0;
        locals.var_t4_dn2 = assign65020_e100331_d_n2;
        locals.var_t4_dn4 = assign65020_e100331_d_n4;
        locals.var_t4_dn5 = assign65020_e100331_d_n5;
        locals.var_t4_dn6 = assign65020_e100331_d_n6;
        locals.var_t4_dn7 = assign65020_e100331_d_n7;
        locals.var_t4_dn8 = assign65020_e100331_d_n8;
        locals.var_t4_dn9 = assign65020_e100331_d_n9;
        locals.var_t4_dn10 = assign65020_e100331_d_n10;
        locals.var_t4_dn13 = assign65020_e100331_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign65030_e100349, assign65030_e100349_d_n0, assign65030_e100349_d_n2, assign65030_e100349_d_n4, assign65030_e100349_d_n5, assign65030_e100349_d_n6, assign65030_e100349_d_n7, assign65030_e100349_d_n8, assign65030_e100349_d_n9, assign65030_e100349_d_n10, assign65030_e100349_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65030_e100345: f64 = (9.0 * locals.var_ty);
        let assign65030_e100347: f64 = (assign65030_e100345 * locals.var_t4);
        (assign65030_e100347, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn4) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn4)), (((9.0 * locals.var_ty_dn5) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn5)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn8) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn8)), (((9.0 * locals.var_ty_dn9) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn9)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn13) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign65030_e100349;
        locals.var_t5_dn0 = assign65030_e100349_d_n0;
        locals.var_t5_dn2 = assign65030_e100349_d_n2;
        locals.var_t5_dn4 = assign65030_e100349_d_n4;
        locals.var_t5_dn5 = assign65030_e100349_d_n5;
        locals.var_t5_dn6 = assign65030_e100349_d_n6;
        locals.var_t5_dn7 = assign65030_e100349_d_n7;
        locals.var_t5_dn8 = assign65030_e100349_d_n8;
        locals.var_t5_dn9 = assign65030_e100349_d_n9;
        locals.var_t5_dn10 = assign65030_e100349_d_n10;
        locals.var_t5_dn13 = assign65030_e100349_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign65040_e100367, assign65040_e100367_d_n0, assign65040_e100367_d_n2, assign65040_e100367_d_n4, assign65040_e100367_d_n5, assign65040_e100367_d_n6, assign65040_e100367_d_n7, assign65040_e100367_d_n8, assign65040_e100367_d_n9, assign65040_e100367_d_n10, assign65040_e100367_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65040_e100363: f64 = (7.0 * 1.414213562373095);
        let assign65040_e100365: f64 = (assign65040_e100363 - locals.var_t5);
        (assign65040_e100365, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn13),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign65040_e100367;
        locals.var_ac31_dn0 = assign65040_e100367_d_n0;
        locals.var_ac31_dn2 = assign65040_e100367_d_n2;
        locals.var_ac31_dn4 = assign65040_e100367_d_n4;
        locals.var_ac31_dn5 = assign65040_e100367_d_n5;
        locals.var_ac31_dn6 = assign65040_e100367_d_n6;
        locals.var_ac31_dn7 = assign65040_e100367_d_n7;
        locals.var_ac31_dn8 = assign65040_e100367_d_n8;
        locals.var_ac31_dn9 = assign65040_e100367_d_n9;
        locals.var_ac31_dn10 = assign65040_e100367_d_n10;
        locals.var_ac31_dn13 = assign65040_e100367_d_n13;
        locals.var_ac31_rv = 0.0;

        let (assign65050_e100383, assign65050_e100383_d_n0, assign65050_e100383_d_n2, assign65050_e100383_d_n4, assign65050_e100383_d_n5, assign65050_e100383_d_n6, assign65050_e100383_d_n7, assign65050_e100383_d_n8, assign65050_e100383_d_n9, assign65050_e100383_d_n10, assign65050_e100383_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65050_e100381: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign65050_e100381, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign65050_e100383;
        locals.var_ac3_dn0 = assign65050_e100383_d_n0;
        locals.var_ac3_dn2 = assign65050_e100383_d_n2;
        locals.var_ac3_dn4 = assign65050_e100383_d_n4;
        locals.var_ac3_dn5 = assign65050_e100383_d_n5;
        locals.var_ac3_dn6 = assign65050_e100383_d_n6;
        locals.var_ac3_dn7 = assign65050_e100383_d_n7;
        locals.var_ac3_dn8 = assign65050_e100383_d_n8;
        locals.var_ac3_dn9 = assign65050_e100383_d_n9;
        locals.var_ac3_dn10 = assign65050_e100383_d_n10;
        locals.var_ac3_dn13 = assign65050_e100383_d_n13;
        locals.var_ac3_rv = 0.0;

        let assign65060_e100387: f64 = (locals.var_ac3 * 1e-8);
        let assign65060_e100388: f64 = if locals.var_ac4 < assign65060_e100387 { 1.0 } else { 0.0 };
        locals.var_guard1555 = assign65060_e100388;
        locals.var_guard1555_rv = 0.0;

        let (assign65070_e100417, assign65070_e100417_d_n0, assign65070_e100417_d_n2, assign65070_e100417_d_n4, assign65070_e100417_d_n5, assign65070_e100417_d_n6, assign65070_e100417_d_n7, assign65070_e100417_d_n8, assign65070_e100417_d_n9, assign65070_e100417_d_n10, assign65070_e100417_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) && (locals.var_guard1555 != 0.0)) {
        let assign65070_e100403: f64 = (-7.0);
        let assign65070_e100405: f64 = (assign65070_e100403 * 1.414213562373095);
        let assign65070_e100407: f64 = (assign65070_e100405 + locals.var_ac31);
        let assign65070_e100410: f64 = (0.5 * locals.var_ac4);
        let assign65070_e100412: f64 = (assign65070_e100410 / locals.var_ac31);
        let assign65070_e100413: f64 = (assign65070_e100407 + assign65070_e100412);
        let assign65070_e100415: f64 = (assign65070_e100413 + locals.var_t5);
        (assign65070_e100415, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn4 + ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn4), ((locals.var_ac31_dn5 + ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn5), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn8 + ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn8), ((locals.var_ac31_dn9 + ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn9), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn13 + ((((0.5 * locals.var_ac4_dn13) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn13)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign65070_e100417;
        locals.var_ac1_dn0 = assign65070_e100417_d_n0;
        locals.var_ac1_dn2 = assign65070_e100417_d_n2;
        locals.var_ac1_dn4 = assign65070_e100417_d_n4;
        locals.var_ac1_dn5 = assign65070_e100417_d_n5;
        locals.var_ac1_dn6 = assign65070_e100417_d_n6;
        locals.var_ac1_dn7 = assign65070_e100417_d_n7;
        locals.var_ac1_dn8 = assign65070_e100417_d_n8;
        locals.var_ac1_dn9 = assign65070_e100417_d_n9;
        locals.var_ac1_dn10 = assign65070_e100417_d_n10;
        locals.var_ac1_dn13 = assign65070_e100417_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign65080_e100437, assign65080_e100437_d_n0, assign65080_e100437_d_n2, assign65080_e100437_d_n4, assign65080_e100437_d_n5, assign65080_e100437_d_n6, assign65080_e100437_d_n7, assign65080_e100437_d_n8, assign65080_e100437_d_n9, assign65080_e100437_d_n10, assign65080_e100437_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) && (locals.var_guard1555 == 0.0)) {
        let assign65080_e100434: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign65080_e100435: f64 = (assign65080_e100434).sqrt();
        (assign65080_e100435, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn13 + locals.var_ac3_dn13) / (2.0 * assign65080_e100435)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn13,)
    }
};
        locals.var_ac2 = assign65080_e100437;
        locals.var_ac2_dn0 = assign65080_e100437_d_n0;
        locals.var_ac2_dn2 = assign65080_e100437_d_n2;
        locals.var_ac2_dn4 = assign65080_e100437_d_n4;
        locals.var_ac2_dn5 = assign65080_e100437_d_n5;
        locals.var_ac2_dn6 = assign65080_e100437_d_n6;
        locals.var_ac2_dn7 = assign65080_e100437_d_n7;
        locals.var_ac2_dn8 = assign65080_e100437_d_n8;
        locals.var_ac2_dn9 = assign65080_e100437_d_n9;
        locals.var_ac2_dn10 = assign65080_e100437_d_n10;
        locals.var_ac2_dn13 = assign65080_e100437_d_n13;
        locals.var_ac2_rv = 0.0;

        let (assign65090_e100461, assign65090_e100461_d_n0, assign65090_e100461_d_n2, assign65090_e100461_d_n4, assign65090_e100461_d_n5, assign65090_e100461_d_n6, assign65090_e100461_d_n7, assign65090_e100461_d_n8, assign65090_e100461_d_n9, assign65090_e100461_d_n10, assign65090_e100461_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) && (locals.var_guard1555 == 0.0)) {
        let assign65090_e100453: f64 = (-7.0);
        let assign65090_e100455: f64 = (assign65090_e100453 * 1.414213562373095);
        let assign65090_e100457: f64 = (assign65090_e100455 + locals.var_ac2);
        let assign65090_e100459: f64 = (assign65090_e100457 + locals.var_t5);
        (assign65090_e100459, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn4 + locals.var_t5_dn4), (locals.var_ac2_dn5 + locals.var_t5_dn5), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn8 + locals.var_t5_dn8), (locals.var_ac2_dn9 + locals.var_t5_dn9), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn13 + locals.var_t5_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign65090_e100461;
        locals.var_ac1_dn0 = assign65090_e100461_d_n0;
        locals.var_ac1_dn2 = assign65090_e100461_d_n2;
        locals.var_ac1_dn4 = assign65090_e100461_d_n4;
        locals.var_ac1_dn5 = assign65090_e100461_d_n5;
        locals.var_ac1_dn6 = assign65090_e100461_d_n6;
        locals.var_ac1_dn7 = assign65090_e100461_d_n7;
        locals.var_ac1_dn8 = assign65090_e100461_d_n8;
        locals.var_ac1_dn9 = assign65090_e100461_d_n9;
        locals.var_ac1_dn10 = assign65090_e100461_d_n10;
        locals.var_ac1_dn13 = assign65090_e100461_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign65100_e100482, assign65100_e100482_d_n0, assign65100_e100482_d_n2, assign65100_e100482_d_n4, assign65100_e100482_d_n5, assign65100_e100482_d_n6, assign65100_e100482_d_n7, assign65100_e100482_d_n8, assign65100_e100482_d_n9, assign65100_e100482_d_n10, assign65100_e100482_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let (assign65100_e100480, assign65100_e100480_d_n0, assign65100_e100480_d_n2, assign65100_e100480_d_n4, assign65100_e100480_d_n5, assign65100_e100480_d_n6, assign65100_e100480_d_n7, assign65100_e100480_d_n8, assign65100_e100480_d_n9, assign65100_e100480_d_n10, assign65100_e100480_d_n13,) = {
            if (locals.var_ac1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65100_e100479: f64 = (locals.var_ac1).powf(0.3333333333333333);
                (assign65100_e100479, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn13)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn13 / locals.var_ac1))) },)
            }
        };
        (assign65100_e100480, assign65100_e100480_d_n0, assign65100_e100480_d_n2, assign65100_e100480_d_n4, assign65100_e100480_d_n5, assign65100_e100480_d_n6, assign65100_e100480_d_n7, assign65100_e100480_d_n8, assign65100_e100480_d_n9, assign65100_e100480_d_n10, assign65100_e100480_d_n13,)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn13,)
    }
};
        locals.var_acd = assign65100_e100482;
        locals.var_acd_dn0 = assign65100_e100482_d_n0;
        locals.var_acd_dn2 = assign65100_e100482_d_n2;
        locals.var_acd_dn4 = assign65100_e100482_d_n4;
        locals.var_acd_dn5 = assign65100_e100482_d_n5;
        locals.var_acd_dn6 = assign65100_e100482_d_n6;
        locals.var_acd_dn7 = assign65100_e100482_d_n7;
        locals.var_acd_dn8 = assign65100_e100482_d_n8;
        locals.var_acd_dn9 = assign65100_e100482_d_n9;
        locals.var_acd_dn10 = assign65100_e100482_d_n10;
        locals.var_acd_dn13 = assign65100_e100482_d_n13;
        locals.var_acd_rv = 0.0;

        let (assign65110_e100513, assign65110_e100513_d_n0, assign65110_e100513_d_n2, assign65110_e100513_d_n4, assign65110_e100513_d_n5, assign65110_e100513_d_n6, assign65110_e100513_d_n7, assign65110_e100513_d_n8, assign65110_e100513_d_n9, assign65110_e100513_d_n10, assign65110_e100513_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65110_e100495: f64 = (-4.0);
        let assign65110_e100497: f64 = (assign65110_e100495 * 1.414213562373095);
        let assign65110_e100500: f64 = (12.0 * locals.var_ty);
        let assign65110_e100501: f64 = (assign65110_e100497 - assign65110_e100500);
        let assign65110_e100504: f64 = (2.0 * locals.var_acd);
        let assign65110_e100505: f64 = (assign65110_e100501 + assign65110_e100504);
        let assign65110_e100508: f64 = (1.414213562373095 * locals.var_acd);
        let assign65110_e100510: f64 = (assign65110_e100508 * locals.var_acd);
        let assign65110_e100511: f64 = (assign65110_e100505 + assign65110_e100510);
        (assign65110_e100511, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn13)) + (2.0 * locals.var_acd_dn13)) + (((1.414213562373095 * locals.var_acd_dn13) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn13))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn13,)
    }
};
        locals.var_acn = assign65110_e100513;
        locals.var_acn_dn0 = assign65110_e100513_d_n0;
        locals.var_acn_dn2 = assign65110_e100513_d_n2;
        locals.var_acn_dn4 = assign65110_e100513_d_n4;
        locals.var_acn_dn5 = assign65110_e100513_d_n5;
        locals.var_acn_dn6 = assign65110_e100513_d_n6;
        locals.var_acn_dn7 = assign65110_e100513_d_n7;
        locals.var_acn_dn8 = assign65110_e100513_d_n8;
        locals.var_acn_dn9 = assign65110_e100513_d_n9;
        locals.var_acn_dn10 = assign65110_e100513_d_n10;
        locals.var_acn_dn13 = assign65110_e100513_d_n13;
        locals.var_acn_rv = 0.0;

        let (assign65120_e100529, assign65120_e100529_d_n0, assign65120_e100529_d_n2, assign65120_e100529_d_n4, assign65120_e100529_d_n5, assign65120_e100529_d_n6, assign65120_e100529_d_n7, assign65120_e100529_d_n8, assign65120_e100529_d_n9, assign65120_e100529_d_n10, assign65120_e100529_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65120_e100527: f64 = (1.0 / locals.var_acd);
        (assign65120_e100527, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn4 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn5 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn8 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn9 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn13 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65120_e100529;
        locals.var_t1_dn0 = assign65120_e100529_d_n0;
        locals.var_t1_dn2 = assign65120_e100529_d_n2;
        locals.var_t1_dn4 = assign65120_e100529_d_n4;
        locals.var_t1_dn5 = assign65120_e100529_d_n5;
        locals.var_t1_dn6 = assign65120_e100529_d_n6;
        locals.var_t1_dn7 = assign65120_e100529_d_n7;
        locals.var_t1_dn8 = assign65120_e100529_d_n8;
        locals.var_t1_dn9 = assign65120_e100529_d_n9;
        locals.var_t1_dn10 = assign65120_e100529_d_n10;
        locals.var_t1_dn13 = assign65120_e100529_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_234(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65130_e100545, assign65130_e100545_d_n0, assign65130_e100545_d_n2, assign65130_e100545_d_n4, assign65130_e100545_d_n5, assign65130_e100545_d_n6, assign65130_e100545_d_n7, assign65130_e100545_d_n8, assign65130_e100545_d_n9, assign65130_e100545_d_n10, assign65130_e100545_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65130_e100543: f64 = (locals.var_acn * locals.var_t1);
        (assign65130_e100543, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn4 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn4)), ((locals.var_acn_dn5 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn5)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn8 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn8)), ((locals.var_acn_dn9 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn9)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn13 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign65130_e100545;
        locals.var_chi_dn0 = assign65130_e100545_d_n0;
        locals.var_chi_dn2 = assign65130_e100545_d_n2;
        locals.var_chi_dn4 = assign65130_e100545_d_n4;
        locals.var_chi_dn5 = assign65130_e100545_d_n5;
        locals.var_chi_dn6 = assign65130_e100545_d_n6;
        locals.var_chi_dn7 = assign65130_e100545_d_n7;
        locals.var_chi_dn8 = assign65130_e100545_d_n8;
        locals.var_chi_dn9 = assign65130_e100545_d_n9;
        locals.var_chi_dn10 = assign65130_e100545_d_n10;
        locals.var_chi_dn13 = assign65130_e100545_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign65140_e100563, assign65140_e100563_d_n0, assign65140_e100563_d_n2, assign65140_e100563_d_n4, assign65140_e100563_d_n5, assign65140_e100563_d_n6, assign65140_e100563_d_n7, assign65140_e100563_d_n8, assign65140_e100563_d_n9, assign65140_e100563_d_n10, assign65140_e100563_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65140_e100559: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign65140_e100561: f64 = (assign65140_e100559 + locals.var_vbscl__blk1545);
        (assign65140_e100561, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk1545_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk1545_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk1545_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk1545_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk1545_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk1545_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk1545_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk1545_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk1545_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) + locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn8, locals.var_psa_dn9, locals.var_psa_dn10, locals.var_psa_dn13,)
    }
};
        locals.var_psa = assign65140_e100563;
        locals.var_psa_dn0 = assign65140_e100563_d_n0;
        locals.var_psa_dn2 = assign65140_e100563_d_n2;
        locals.var_psa_dn4 = assign65140_e100563_d_n4;
        locals.var_psa_dn5 = assign65140_e100563_d_n5;
        locals.var_psa_dn6 = assign65140_e100563_d_n6;
        locals.var_psa_dn7 = assign65140_e100563_d_n7;
        locals.var_psa_dn8 = assign65140_e100563_d_n8;
        locals.var_psa_dn9 = assign65140_e100563_d_n9;
        locals.var_psa_dn10 = assign65140_e100563_d_n10;
        locals.var_psa_dn13 = assign65140_e100563_d_n13;
        locals.var_psa_rv = 0.0;

        let (assign65150_e100579, assign65150_e100579_d_n0, assign65150_e100579_d_n2, assign65150_e100579_d_n4, assign65150_e100579_d_n5, assign65150_e100579_d_n6, assign65150_e100579_d_n7, assign65150_e100579_d_n8, assign65150_e100579_d_n9, assign65150_e100579_d_n10, assign65150_e100579_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65150_e100577: f64 = (locals.var_psa - locals.var_vbscl__blk1545);
        (assign65150_e100577, (locals.var_psa_dn0 - locals.var_vbscl__blk1545_dn0), (locals.var_psa_dn2 - locals.var_vbscl__blk1545_dn2), (locals.var_psa_dn4 - locals.var_vbscl__blk1545_dn4), (locals.var_psa_dn5 - locals.var_vbscl__blk1545_dn5), (locals.var_psa_dn6 - locals.var_vbscl__blk1545_dn6), (locals.var_psa_dn7 - locals.var_vbscl__blk1545_dn7), (locals.var_psa_dn8 - locals.var_vbscl__blk1545_dn8), (locals.var_psa_dn9 - locals.var_vbscl__blk1545_dn9), (locals.var_psa_dn10 - locals.var_vbscl__blk1545_dn10), (locals.var_psa_dn13 - locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65150_e100579;
        locals.var_t1_dn0 = assign65150_e100579_d_n0;
        locals.var_t1_dn2 = assign65150_e100579_d_n2;
        locals.var_t1_dn4 = assign65150_e100579_d_n4;
        locals.var_t1_dn5 = assign65150_e100579_d_n5;
        locals.var_t1_dn6 = assign65150_e100579_d_n6;
        locals.var_t1_dn7 = assign65150_e100579_d_n7;
        locals.var_t1_dn8 = assign65150_e100579_d_n8;
        locals.var_t1_dn9 = assign65150_e100579_d_n9;
        locals.var_t1_dn10 = assign65150_e100579_d_n10;
        locals.var_t1_dn13 = assign65150_e100579_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign65160_e100595, assign65160_e100595_d_n0, assign65160_e100595_d_n2, assign65160_e100595_d_n4, assign65160_e100595_d_n5, assign65160_e100595_d_n6, assign65160_e100595_d_n7, assign65160_e100595_d_n8, assign65160_e100595_d_n9, assign65160_e100595_d_n10, assign65160_e100595_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65160_e100593: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign65160_e100593, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn13 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn13)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign65160_e100595;
        locals.var_t2_dn0 = assign65160_e100595_d_n0;
        locals.var_t2_dn2 = assign65160_e100595_d_n2;
        locals.var_t2_dn4 = assign65160_e100595_d_n4;
        locals.var_t2_dn5 = assign65160_e100595_d_n5;
        locals.var_t2_dn6 = assign65160_e100595_d_n6;
        locals.var_t2_dn7 = assign65160_e100595_d_n7;
        locals.var_t2_dn8 = assign65160_e100595_d_n8;
        locals.var_t2_dn9 = assign65160_e100595_d_n9;
        locals.var_t2_dn10 = assign65160_e100595_d_n10;
        locals.var_t2_dn13 = assign65160_e100595_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign65170_e100614, assign65170_e100614_d_n0, assign65170_e100614_d_n2, assign65170_e100614_d_n4, assign65170_e100614_d_n5, assign65170_e100614_d_n6, assign65170_e100614_d_n7, assign65170_e100614_d_n8, assign65170_e100614_d_n9, assign65170_e100614_d_n10, assign65170_e100614_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65170_e100610: f64 = (locals.var_t2 * locals.var_t2);
        let assign65170_e100611: f64 = (1.0 + assign65170_e100610);
        let assign65170_e100612: f64 = (assign65170_e100611).sqrt();
        (assign65170_e100612, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign65170_e100612)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign65170_e100614;
        locals.var_t3_dn0 = assign65170_e100614_d_n0;
        locals.var_t3_dn2 = assign65170_e100614_d_n2;
        locals.var_t3_dn4 = assign65170_e100614_d_n4;
        locals.var_t3_dn5 = assign65170_e100614_d_n5;
        locals.var_t3_dn6 = assign65170_e100614_d_n6;
        locals.var_t3_dn7 = assign65170_e100614_d_n7;
        locals.var_t3_dn8 = assign65170_e100614_d_n8;
        locals.var_t3_dn9 = assign65170_e100614_d_n9;
        locals.var_t3_dn10 = assign65170_e100614_d_n10;
        locals.var_t3_dn13 = assign65170_e100614_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign65180_e100632, assign65180_e100632_d_n0, assign65180_e100632_d_n2, assign65180_e100632_d_n4, assign65180_e100632_d_n5, assign65180_e100632_d_n6, assign65180_e100632_d_n7, assign65180_e100632_d_n8, assign65180_e100632_d_n9, assign65180_e100632_d_n10, assign65180_e100632_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65180_e100628: f64 = (locals.var_t1 / locals.var_t3);
        let assign65180_e100630: f64 = (assign65180_e100628 + locals.var_vbscl__blk1545);
        (assign65180_e100630, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn10), ((((locals.var_t1_dn13 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    }
};
        locals.var_ps0__blk1523 = assign65180_e100632;
        locals.var_ps0__blk1523_dn0 = assign65180_e100632_d_n0;
        locals.var_ps0__blk1523_dn2 = assign65180_e100632_d_n2;
        locals.var_ps0__blk1523_dn4 = assign65180_e100632_d_n4;
        locals.var_ps0__blk1523_dn5 = assign65180_e100632_d_n5;
        locals.var_ps0__blk1523_dn6 = assign65180_e100632_d_n6;
        locals.var_ps0__blk1523_dn7 = assign65180_e100632_d_n7;
        locals.var_ps0__blk1523_dn8 = assign65180_e100632_d_n8;
        locals.var_ps0__blk1523_dn9 = assign65180_e100632_d_n9;
        locals.var_ps0__blk1523_dn10 = assign65180_e100632_d_n10;
        locals.var_ps0__blk1523_dn13 = assign65180_e100632_d_n13;
        locals.var_ps0__blk1523_rv = 0.0;

        let (assign65190_e100652, assign65190_e100652_d_n0, assign65190_e100652_d_n2, assign65190_e100652_d_n4, assign65190_e100652_d_n5, assign65190_e100652_d_n6, assign65190_e100652_d_n7, assign65190_e100652_d_n8, assign65190_e100652_d_n9, assign65190_e100652_d_n10, assign65190_e100652_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65190_e100648: f64 = (locals.var_vbscl__blk1545 - p.p456);
        let assign65190_e100649: f64 = (locals.var_beta * assign65190_e100648);
        let assign65190_e100650: f64 = (assign65190_e100649).exp();
        (assign65190_e100650, (assign65190_e100650 * ((locals.var_beta_dn0 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn0))), (assign65190_e100650 * ((locals.var_beta_dn2 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn2))), (assign65190_e100650 * ((locals.var_beta_dn4 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn4))), (assign65190_e100650 * ((locals.var_beta_dn5 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn5))), (assign65190_e100650 * ((locals.var_beta_dn6 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn6))), (assign65190_e100650 * ((locals.var_beta_dn7 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn7))), (assign65190_e100650 * ((locals.var_beta_dn8 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn8))), (assign65190_e100650 * ((locals.var_beta_dn9 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn9))), (assign65190_e100650 * ((locals.var_beta_dn10 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn10))), (assign65190_e100650 * ((locals.var_beta_dn13 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn13))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn4, locals.var_exp_bvbsvds_dn5, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn8, locals.var_exp_bvbsvds_dn9, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn13,)
    }
};
        locals.var_exp_bvbsvds = assign65190_e100652;
        locals.var_exp_bvbsvds_dn0 = assign65190_e100652_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign65190_e100652_d_n2;
        locals.var_exp_bvbsvds_dn4 = assign65190_e100652_d_n4;
        locals.var_exp_bvbsvds_dn5 = assign65190_e100652_d_n5;
        locals.var_exp_bvbsvds_dn6 = assign65190_e100652_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign65190_e100652_d_n7;
        locals.var_exp_bvbsvds_dn8 = assign65190_e100652_d_n8;
        locals.var_exp_bvbsvds_dn9 = assign65190_e100652_d_n9;
        locals.var_exp_bvbsvds_dn10 = assign65190_e100652_d_n10;
        locals.var_exp_bvbsvds_dn13 = assign65190_e100652_d_n13;
        locals.var_exp_bvbsvds_rv = 0.0;

        let (assign65200_e100667,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign65200_e100667;
        locals.var_flg_conv_rv = 0.0;

        let (assign65210_e100682, assign65210_e100682_d_n0, assign65210_e100682_d_n2, assign65210_e100682_d_n4, assign65210_e100682_d_n5, assign65210_e100682_d_n6, assign65210_e100682_d_n7, assign65210_e100682_d_n8, assign65210_e100682_d_n9, assign65210_e100682_d_n10, assign65210_e100682_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    } else {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn13,)
    }
};
        locals.var_phi_s0 = assign65210_e100682;
        locals.var_phi_s0_dn0 = assign65210_e100682_d_n0;
        locals.var_phi_s0_dn2 = assign65210_e100682_d_n2;
        locals.var_phi_s0_dn4 = assign65210_e100682_d_n4;
        locals.var_phi_s0_dn5 = assign65210_e100682_d_n5;
        locals.var_phi_s0_dn6 = assign65210_e100682_d_n6;
        locals.var_phi_s0_dn7 = assign65210_e100682_d_n7;
        locals.var_phi_s0_dn8 = assign65210_e100682_d_n8;
        locals.var_phi_s0_dn9 = assign65210_e100682_d_n9;
        locals.var_phi_s0_dn10 = assign65210_e100682_d_n10;
        locals.var_phi_s0_dn13 = assign65210_e100682_d_n13;
        locals.var_phi_s0_rv = 0.0;

        let (assign65220_e100705, assign65220_e100705_d_n0, assign65220_e100705_d_n2, assign65220_e100705_d_n4, assign65220_e100705_d_n5, assign65220_e100705_d_n6, assign65220_e100705_d_n7, assign65220_e100705_d_n8, assign65220_e100705_d_n9, assign65220_e100705_d_n10, assign65220_e100705_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65220_e100697: f64 = (locals.var_q_nsub * locals.var_t_sub);
        let assign65220_e100699: f64 = (assign65220_e100697 * locals.var_t_sub);
        let assign65220_e100701: f64 = (assign65220_e100699 / 2.0);
        let assign65220_e100703: f64 = (assign65220_e100701 / 1.034943e-10);
        (assign65220_e100703, ((((locals.var_q_nsub_dn0 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn4 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn5 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn8 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn9 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn13 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1547, locals.var_dphi_sb__blk1547_dn0, locals.var_dphi_sb__blk1547_dn2, locals.var_dphi_sb__blk1547_dn4, locals.var_dphi_sb__blk1547_dn5, locals.var_dphi_sb__blk1547_dn6, locals.var_dphi_sb__blk1547_dn7, locals.var_dphi_sb__blk1547_dn8, locals.var_dphi_sb__blk1547_dn9, locals.var_dphi_sb__blk1547_dn10, locals.var_dphi_sb__blk1547_dn13,)
    }
};
        locals.var_dphi_sb__blk1547 = assign65220_e100705;
        locals.var_dphi_sb__blk1547_dn0 = assign65220_e100705_d_n0;
        locals.var_dphi_sb__blk1547_dn2 = assign65220_e100705_d_n2;
        locals.var_dphi_sb__blk1547_dn4 = assign65220_e100705_d_n4;
        locals.var_dphi_sb__blk1547_dn5 = assign65220_e100705_d_n5;
        locals.var_dphi_sb__blk1547_dn6 = assign65220_e100705_d_n6;
        locals.var_dphi_sb__blk1547_dn7 = assign65220_e100705_d_n7;
        locals.var_dphi_sb__blk1547_dn8 = assign65220_e100705_d_n8;
        locals.var_dphi_sb__blk1547_dn9 = assign65220_e100705_d_n9;
        locals.var_dphi_sb__blk1547_dn10 = assign65220_e100705_d_n10;
        locals.var_dphi_sb__blk1547_dn13 = assign65220_e100705_d_n13;
        locals.var_dphi_sb__blk1547_rv = 0.0;

        let (assign65230_e100725, assign65230_e100725_d_n0, assign65230_e100725_d_n2, assign65230_e100725_d_n4, assign65230_e100725_d_n5, assign65230_e100725_d_n6, assign65230_e100725_d_n7, assign65230_e100725_d_n8, assign65230_e100725_d_n9, assign65230_e100725_d_n10, assign65230_e100725_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65230_e100720: f64 = (2.0 * locals.var_beta);
        let assign65230_e100722: f64 = (assign65230_e100720 * locals.var_dphi_sb__blk1547);
        let assign65230_e100723: f64 = (assign65230_e100722).sqrt();
        (assign65230_e100723, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn0)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn2)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn4)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn5)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn6)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn7)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn8)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn9)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn10)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn13)) / (2.0 * assign65230_e100723)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign65230_e100725;
        locals.var_t0_dn0 = assign65230_e100725_d_n0;
        locals.var_t0_dn2 = assign65230_e100725_d_n2;
        locals.var_t0_dn4 = assign65230_e100725_d_n4;
        locals.var_t0_dn5 = assign65230_e100725_d_n5;
        locals.var_t0_dn6 = assign65230_e100725_d_n6;
        locals.var_t0_dn7 = assign65230_e100725_d_n7;
        locals.var_t0_dn8 = assign65230_e100725_d_n8;
        locals.var_t0_dn9 = assign65230_e100725_d_n9;
        locals.var_t0_dn10 = assign65230_e100725_d_n10;
        locals.var_t0_dn13 = assign65230_e100725_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign65240_e100747, assign65240_e100747_d_n0, assign65240_e100747_d_n2, assign65240_e100747_d_n4, assign65240_e100747_d_n5, assign65240_e100747_d_n6, assign65240_e100747_d_n7, assign65240_e100747_d_n8, assign65240_e100747_d_n9, assign65240_e100747_d_n10, assign65240_e100747_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65240_e100739: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign65240_e100741: f64 = (-locals.var_t0);
        let assign65240_e100742: f64 = { let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign65240_e100743: f64 = (assign65240_e100739 + assign65240_e100742);
        let assign65240_e100745: f64 = (assign65240_e100743 / 2.0);
        (assign65240_e100745, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65240_e100747;
        locals.var_t1_dn0 = assign65240_e100747_d_n0;
        locals.var_t1_dn2 = assign65240_e100747_d_n2;
        locals.var_t1_dn4 = assign65240_e100747_d_n4;
        locals.var_t1_dn5 = assign65240_e100747_d_n5;
        locals.var_t1_dn6 = assign65240_e100747_d_n6;
        locals.var_t1_dn7 = assign65240_e100747_d_n7;
        locals.var_t1_dn8 = assign65240_e100747_d_n8;
        locals.var_t1_dn9 = assign65240_e100747_d_n9;
        locals.var_t1_dn10 = assign65240_e100747_d_n10;
        locals.var_t1_dn13 = assign65240_e100747_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign65250_e100765, assign65250_e100765_d_n0, assign65250_e100765_d_n2, assign65250_e100765_d_n4, assign65250_e100765_d_n5, assign65250_e100765_d_n6, assign65250_e100765_d_n7, assign65250_e100765_d_n8, assign65250_e100765_d_n9, assign65250_e100765_d_n10, assign65250_e100765_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65250_e100761: f64 = (locals.var_t1).ln();
        let assign65250_e100763: f64 = (assign65250_e100761 / locals.var_dphi_sb__blk1547);
        (assign65250_e100763, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn0)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn2)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn4)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn5)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn6)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn7)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn8)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn9)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn10)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn13)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)),)
    } else {
        (locals.var_c_sb__blk1548, locals.var_c_sb__blk1548_dn0, locals.var_c_sb__blk1548_dn2, locals.var_c_sb__blk1548_dn4, locals.var_c_sb__blk1548_dn5, locals.var_c_sb__blk1548_dn6, locals.var_c_sb__blk1548_dn7, locals.var_c_sb__blk1548_dn8, locals.var_c_sb__blk1548_dn9, locals.var_c_sb__blk1548_dn10, locals.var_c_sb__blk1548_dn13,)
    }
};
        locals.var_c_sb__blk1548 = assign65250_e100765;
        locals.var_c_sb__blk1548_dn0 = assign65250_e100765_d_n0;
        locals.var_c_sb__blk1548_dn2 = assign65250_e100765_d_n2;
        locals.var_c_sb__blk1548_dn4 = assign65250_e100765_d_n4;
        locals.var_c_sb__blk1548_dn5 = assign65250_e100765_d_n5;
        locals.var_c_sb__blk1548_dn6 = assign65250_e100765_d_n6;
        locals.var_c_sb__blk1548_dn7 = assign65250_e100765_d_n7;
        locals.var_c_sb__blk1548_dn8 = assign65250_e100765_d_n8;
        locals.var_c_sb__blk1548_dn9 = assign65250_e100765_d_n9;
        locals.var_c_sb__blk1548_dn10 = assign65250_e100765_d_n10;
        locals.var_c_sb__blk1548_dn13 = assign65250_e100765_d_n13;
        locals.var_c_sb__blk1548_rv = 0.0;

        let (assign65260_e100780,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign65260_e100780;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_235(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign65270_loop_guard: usize = 0;
        while {
            let assign65270_cond_e100796: f64 = (locals.var_lp_s0_max + 1.0);
            let assign65270_cond_e100798: f64 = if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_lp_s0 <= assign65270_cond_e100796)) { 1.0 } else { 0.0 };
            assign65270_cond_e100798 != 0.0
        } {
            assign65270_loop_guard += 1;
            assert!(assign65270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign65270_body0_e100815, assign65270_body0_e100815_d_n0, assign65270_body0_e100815_d_n2, assign65270_body0_e100815_d_n4, assign65270_body0_e100815_d_n5, assign65270_body0_e100815_d_n6, assign65270_body0_e100815_d_n7, assign65270_body0_e100815_d_n8, assign65270_body0_e100815_d_n9, assign65270_body0_e100815_d_n10, assign65270_body0_e100815_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body0_e100813: f64 = (locals.var_phi_s0 - locals.var_vbscl__blk1545);
        (assign65270_body0_e100813, (locals.var_phi_s0_dn0 - locals.var_vbscl__blk1545_dn0), (locals.var_phi_s0_dn2 - locals.var_vbscl__blk1545_dn2), (locals.var_phi_s0_dn4 - locals.var_vbscl__blk1545_dn4), (locals.var_phi_s0_dn5 - locals.var_vbscl__blk1545_dn5), (locals.var_phi_s0_dn6 - locals.var_vbscl__blk1545_dn6), (locals.var_phi_s0_dn7 - locals.var_vbscl__blk1545_dn7), (locals.var_phi_s0_dn8 - locals.var_vbscl__blk1545_dn8), (locals.var_phi_s0_dn9 - locals.var_vbscl__blk1545_dn9), (locals.var_phi_s0_dn10 - locals.var_vbscl__blk1545_dn10), (locals.var_phi_s0_dn13 - locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_phi_0, locals.var_phi_0_dn0, locals.var_phi_0_dn2, locals.var_phi_0_dn4, locals.var_phi_0_dn5, locals.var_phi_0_dn6, locals.var_phi_0_dn7, locals.var_phi_0_dn8, locals.var_phi_0_dn9, locals.var_phi_0_dn10, locals.var_phi_0_dn13,)
    }
};
            locals.var_phi_0 = assign65270_body0_e100815;
            locals.var_phi_0_dn0 = assign65270_body0_e100815_d_n0;
            locals.var_phi_0_dn2 = assign65270_body0_e100815_d_n2;
            locals.var_phi_0_dn4 = assign65270_body0_e100815_d_n4;
            locals.var_phi_0_dn5 = assign65270_body0_e100815_d_n5;
            locals.var_phi_0_dn6 = assign65270_body0_e100815_d_n6;
            locals.var_phi_0_dn7 = assign65270_body0_e100815_d_n7;
            locals.var_phi_0_dn8 = assign65270_body0_e100815_d_n8;
            locals.var_phi_0_dn9 = assign65270_body0_e100815_d_n9;
            locals.var_phi_0_dn10 = assign65270_body0_e100815_d_n10;
            locals.var_phi_0_dn13 = assign65270_body0_e100815_d_n13;
            locals.var_phi_0_rv = 0.0;
            let (assign65270_body1_e100832, assign65270_body1_e100832_d_n0, assign65270_body1_e100832_d_n2, assign65270_body1_e100832_d_n4, assign65270_body1_e100832_d_n5, assign65270_body1_e100832_d_n6, assign65270_body1_e100832_d_n7, assign65270_body1_e100832_d_n8, assign65270_body1_e100832_d_n9, assign65270_body1_e100832_d_n10, assign65270_body1_e100832_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body1_e100830: f64 = (locals.var_beta * locals.var_phi_0);
        (assign65270_body1_e100830, ((locals.var_beta_dn0 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn0)), ((locals.var_beta_dn2 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn2)), ((locals.var_beta_dn4 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn4)), ((locals.var_beta_dn5 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn5)), ((locals.var_beta_dn6 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn6)), ((locals.var_beta_dn7 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn7)), ((locals.var_beta_dn8 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn8)), ((locals.var_beta_dn9 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn9)), ((locals.var_beta_dn10 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn10)), ((locals.var_beta_dn13 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign65270_body1_e100832;
            locals.var_chi_dn0 = assign65270_body1_e100832_d_n0;
            locals.var_chi_dn2 = assign65270_body1_e100832_d_n2;
            locals.var_chi_dn4 = assign65270_body1_e100832_d_n4;
            locals.var_chi_dn5 = assign65270_body1_e100832_d_n5;
            locals.var_chi_dn6 = assign65270_body1_e100832_d_n6;
            locals.var_chi_dn7 = assign65270_body1_e100832_d_n7;
            locals.var_chi_dn8 = assign65270_body1_e100832_d_n8;
            locals.var_chi_dn9 = assign65270_body1_e100832_d_n9;
            locals.var_chi_dn10 = assign65270_body1_e100832_d_n10;
            locals.var_chi_dn13 = assign65270_body1_e100832_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign65270_body2_e100851, assign65270_body2_e100851_d_n0, assign65270_body2_e100851_d_n2, assign65270_body2_e100851_d_n4, assign65270_body2_e100851_d_n5, assign65270_body2_e100851_d_n6, assign65270_body2_e100851_d_n7, assign65270_body2_e100851_d_n8, assign65270_body2_e100851_d_n9, assign65270_body2_e100851_d_n10, assign65270_body2_e100851_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body2_e100848: f64 = (locals.var_phi_0 - locals.var_dphi_sb__blk1547);
        let assign65270_body2_e100849: f64 = (locals.var_c_sb__blk1548 * assign65270_body2_e100848);
        (assign65270_body2_e100849, ((locals.var_c_sb__blk1548_dn0 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn0 - locals.var_dphi_sb__blk1547_dn0))), ((locals.var_c_sb__blk1548_dn2 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn2 - locals.var_dphi_sb__blk1547_dn2))), ((locals.var_c_sb__blk1548_dn4 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn4 - locals.var_dphi_sb__blk1547_dn4))), ((locals.var_c_sb__blk1548_dn5 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn5 - locals.var_dphi_sb__blk1547_dn5))), ((locals.var_c_sb__blk1548_dn6 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn6 - locals.var_dphi_sb__blk1547_dn6))), ((locals.var_c_sb__blk1548_dn7 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn7 - locals.var_dphi_sb__blk1547_dn7))), ((locals.var_c_sb__blk1548_dn8 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn8 - locals.var_dphi_sb__blk1547_dn8))), ((locals.var_c_sb__blk1548_dn9 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn9 - locals.var_dphi_sb__blk1547_dn9))), ((locals.var_c_sb__blk1548_dn10 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn10 - locals.var_dphi_sb__blk1547_dn10))), ((locals.var_c_sb__blk1548_dn13 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn13 - locals.var_dphi_sb__blk1547_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign65270_body2_e100851;
            locals.var_ty_dn0 = assign65270_body2_e100851_d_n0;
            locals.var_ty_dn2 = assign65270_body2_e100851_d_n2;
            locals.var_ty_dn4 = assign65270_body2_e100851_d_n4;
            locals.var_ty_dn5 = assign65270_body2_e100851_d_n5;
            locals.var_ty_dn6 = assign65270_body2_e100851_d_n6;
            locals.var_ty_dn7 = assign65270_body2_e100851_d_n7;
            locals.var_ty_dn8 = assign65270_body2_e100851_d_n8;
            locals.var_ty_dn9 = assign65270_body2_e100851_d_n9;
            locals.var_ty_dn10 = assign65270_body2_e100851_d_n10;
            locals.var_ty_dn13 = assign65270_body2_e100851_d_n13;
            locals.var_ty_rv = 0.0;
            let assign65270_body3_e100854: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1556 = assign65270_body3_e100854;
            locals.var_guard1556_rv = 0.0;
            let (assign65270_body4_e100872, assign65270_body4_e100872_d_n0, assign65270_body4_e100872_d_n2, assign65270_body4_e100872_d_n4, assign65270_body4_e100872_d_n5, assign65270_body4_e100872_d_n6, assign65270_body4_e100872_d_n7, assign65270_body4_e100872_d_n8, assign65270_body4_e100872_d_n9, assign65270_body4_e100872_d_n10, assign65270_body4_e100872_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body4_e100870: f64 = (locals.var_ty).exp();
        (assign65270_body4_e100870, (assign65270_body4_e100870 * locals.var_ty_dn0), (assign65270_body4_e100870 * locals.var_ty_dn2), (assign65270_body4_e100870 * locals.var_ty_dn4), (assign65270_body4_e100870 * locals.var_ty_dn5), (assign65270_body4_e100870 * locals.var_ty_dn6), (assign65270_body4_e100870 * locals.var_ty_dn7), (assign65270_body4_e100870 * locals.var_ty_dn8), (assign65270_body4_e100870 * locals.var_ty_dn9), (assign65270_body4_e100870 * locals.var_ty_dn10), (assign65270_body4_e100870 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign65270_body4_e100872;
            locals.var_t1_dn0 = assign65270_body4_e100872_d_n0;
            locals.var_t1_dn2 = assign65270_body4_e100872_d_n2;
            locals.var_t1_dn4 = assign65270_body4_e100872_d_n4;
            locals.var_t1_dn5 = assign65270_body4_e100872_d_n5;
            locals.var_t1_dn6 = assign65270_body4_e100872_d_n6;
            locals.var_t1_dn7 = assign65270_body4_e100872_d_n7;
            locals.var_t1_dn8 = assign65270_body4_e100872_d_n8;
            locals.var_t1_dn9 = assign65270_body4_e100872_d_n9;
            locals.var_t1_dn10 = assign65270_body4_e100872_d_n10;
            locals.var_t1_dn13 = assign65270_body4_e100872_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign65270_body5_e100893, assign65270_body5_e100893_d_n0, assign65270_body5_e100893_d_n2, assign65270_body5_e100893_d_n4, assign65270_body5_e100893_d_n5, assign65270_body5_e100893_d_n6, assign65270_body5_e100893_d_n7, assign65270_body5_e100893_d_n8, assign65270_body5_e100893_d_n9, assign65270_body5_e100893_d_n10, assign65270_body5_e100893_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body5_e100888: f64 = (-locals.var_c_sb__blk1548);
        let assign65270_body5_e100890: f64 = (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547);
        let assign65270_body5_e100891: f64 = (assign65270_body5_e100890).exp();
        (assign65270_body5_e100891, (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn0) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn0))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn2) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn2))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn4) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn4))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn5) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn5))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn6) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn6))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn7) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn7))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn8) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn8))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn9) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn9))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn10) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn10))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn13) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign65270_body5_e100893;
            locals.var_t0_dn0 = assign65270_body5_e100893_d_n0;
            locals.var_t0_dn2 = assign65270_body5_e100893_d_n2;
            locals.var_t0_dn4 = assign65270_body5_e100893_d_n4;
            locals.var_t0_dn5 = assign65270_body5_e100893_d_n5;
            locals.var_t0_dn6 = assign65270_body5_e100893_d_n6;
            locals.var_t0_dn7 = assign65270_body5_e100893_d_n7;
            locals.var_t0_dn8 = assign65270_body5_e100893_d_n8;
            locals.var_t0_dn9 = assign65270_body5_e100893_d_n9;
            locals.var_t0_dn10 = assign65270_body5_e100893_d_n10;
            locals.var_t0_dn13 = assign65270_body5_e100893_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign65270_body6_e100912, assign65270_body6_e100912_d_n0, assign65270_body6_e100912_d_n2, assign65270_body6_e100912_d_n4, assign65270_body6_e100912_d_n5, assign65270_body6_e100912_d_n6, assign65270_body6_e100912_d_n7, assign65270_body6_e100912_d_n8, assign65270_body6_e100912_d_n9, assign65270_body6_e100912_d_n10, assign65270_body6_e100912_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body6_e100910: f64 = (locals.var_t1 - locals.var_t0);
        (assign65270_body6_e100910, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign65270_body6_e100912;
            locals.var_t2_dn0 = assign65270_body6_e100912_d_n0;
            locals.var_t2_dn2 = assign65270_body6_e100912_d_n2;
            locals.var_t2_dn4 = assign65270_body6_e100912_d_n4;
            locals.var_t2_dn5 = assign65270_body6_e100912_d_n5;
            locals.var_t2_dn6 = assign65270_body6_e100912_d_n6;
            locals.var_t2_dn7 = assign65270_body6_e100912_d_n7;
            locals.var_t2_dn8 = assign65270_body6_e100912_d_n8;
            locals.var_t2_dn9 = assign65270_body6_e100912_d_n9;
            locals.var_t2_dn10 = assign65270_body6_e100912_d_n10;
            locals.var_t2_dn13 = assign65270_body6_e100912_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign65270_body7_e100934, assign65270_body7_e100934_d_n0, assign65270_body7_e100934_d_n2, assign65270_body7_e100934_d_n4, assign65270_body7_e100934_d_n5, assign65270_body7_e100934_d_n6, assign65270_body7_e100934_d_n7, assign65270_body7_e100934_d_n8, assign65270_body7_e100934_d_n9, assign65270_body7_e100934_d_n10, assign65270_body7_e100934_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body7_e100929: f64 = (1.0 + locals.var_t2);
        let assign65270_body7_e100930: f64 = (assign65270_body7_e100929).ln();
        let assign65270_body7_e100932: f64 = (assign65270_body7_e100930 / locals.var_c_sb__blk1548);
        (assign65270_body7_e100932, ((((locals.var_t2_dn0 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn0)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn2 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn2)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn4 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn4)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn5 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn5)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn6 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn6)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn7 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn7)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn8 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn8)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn9 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn9)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn10 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn10)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn13 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn13)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)),)
    } else {
        (locals.var_phi_b__blk1551, locals.var_phi_b__blk1551_dn0, locals.var_phi_b__blk1551_dn2, locals.var_phi_b__blk1551_dn4, locals.var_phi_b__blk1551_dn5, locals.var_phi_b__blk1551_dn6, locals.var_phi_b__blk1551_dn7, locals.var_phi_b__blk1551_dn8, locals.var_phi_b__blk1551_dn9, locals.var_phi_b__blk1551_dn10, locals.var_phi_b__blk1551_dn13,)
    }
};
            locals.var_phi_b__blk1551 = assign65270_body7_e100934;
            locals.var_phi_b__blk1551_dn0 = assign65270_body7_e100934_d_n0;
            locals.var_phi_b__blk1551_dn2 = assign65270_body7_e100934_d_n2;
            locals.var_phi_b__blk1551_dn4 = assign65270_body7_e100934_d_n4;
            locals.var_phi_b__blk1551_dn5 = assign65270_body7_e100934_d_n5;
            locals.var_phi_b__blk1551_dn6 = assign65270_body7_e100934_d_n6;
            locals.var_phi_b__blk1551_dn7 = assign65270_body7_e100934_d_n7;
            locals.var_phi_b__blk1551_dn8 = assign65270_body7_e100934_d_n8;
            locals.var_phi_b__blk1551_dn9 = assign65270_body7_e100934_d_n9;
            locals.var_phi_b__blk1551_dn10 = assign65270_body7_e100934_d_n10;
            locals.var_phi_b__blk1551_dn13 = assign65270_body7_e100934_d_n13;
            locals.var_phi_b__blk1551_rv = 0.0;
            let (assign65270_body8_e100955, assign65270_body8_e100955_d_n0, assign65270_body8_e100955_d_n2, assign65270_body8_e100955_d_n4, assign65270_body8_e100955_d_n5, assign65270_body8_e100955_d_n6, assign65270_body8_e100955_d_n7, assign65270_body8_e100955_d_n8, assign65270_body8_e100955_d_n9, assign65270_body8_e100955_d_n10, assign65270_body8_e100955_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body8_e100952: f64 = (1.0 + locals.var_t2);
        let assign65270_body8_e100953: f64 = (locals.var_t1 / assign65270_body8_e100952);
        (assign65270_body8_e100953, (((locals.var_t1_dn0 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn0)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn2 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn2)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn4 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn4)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn5 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn5)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn6 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn6)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn7 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn7)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn8 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn8)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn9 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn9)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn10 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn10)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn13 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn13)) / (assign65270_body8_e100952 * assign65270_body8_e100952)),)
    } else {
        (locals.var_phi_b_dpss__blk1552, locals.var_phi_b_dpss__blk1552_dn0, locals.var_phi_b_dpss__blk1552_dn2, locals.var_phi_b_dpss__blk1552_dn4, locals.var_phi_b_dpss__blk1552_dn5, locals.var_phi_b_dpss__blk1552_dn6, locals.var_phi_b_dpss__blk1552_dn7, locals.var_phi_b_dpss__blk1552_dn8, locals.var_phi_b_dpss__blk1552_dn9, locals.var_phi_b_dpss__blk1552_dn10, locals.var_phi_b_dpss__blk1552_dn13,)
    }
};
            locals.var_phi_b_dpss__blk1552 = assign65270_body8_e100955;
            locals.var_phi_b_dpss__blk1552_dn0 = assign65270_body8_e100955_d_n0;
            locals.var_phi_b_dpss__blk1552_dn2 = assign65270_body8_e100955_d_n2;
            locals.var_phi_b_dpss__blk1552_dn4 = assign65270_body8_e100955_d_n4;
            locals.var_phi_b_dpss__blk1552_dn5 = assign65270_body8_e100955_d_n5;
            locals.var_phi_b_dpss__blk1552_dn6 = assign65270_body8_e100955_d_n6;
            locals.var_phi_b_dpss__blk1552_dn7 = assign65270_body8_e100955_d_n7;
            locals.var_phi_b_dpss__blk1552_dn8 = assign65270_body8_e100955_d_n8;
            locals.var_phi_b_dpss__blk1552_dn9 = assign65270_body8_e100955_d_n9;
            locals.var_phi_b_dpss__blk1552_dn10 = assign65270_body8_e100955_d_n10;
            locals.var_phi_b_dpss__blk1552_dn13 = assign65270_body8_e100955_d_n13;
            locals.var_phi_b_dpss__blk1552_rv = 0.0;
            let (assign65270_body9_e100975, assign65270_body9_e100975_d_n0, assign65270_body9_e100975_d_n2, assign65270_body9_e100975_d_n4, assign65270_body9_e100975_d_n5, assign65270_body9_e100975_d_n6, assign65270_body9_e100975_d_n7, assign65270_body9_e100975_d_n8, assign65270_body9_e100975_d_n9, assign65270_body9_e100975_d_n10, assign65270_body9_e100975_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65270_body9_e100973: f64 = (locals.var_phi_0 - locals.var_dphi_sb__blk1547);
        (assign65270_body9_e100973, (locals.var_phi_0_dn0 - locals.var_dphi_sb__blk1547_dn0), (locals.var_phi_0_dn2 - locals.var_dphi_sb__blk1547_dn2), (locals.var_phi_0_dn4 - locals.var_dphi_sb__blk1547_dn4), (locals.var_phi_0_dn5 - locals.var_dphi_sb__blk1547_dn5), (locals.var_phi_0_dn6 - locals.var_dphi_sb__blk1547_dn6), (locals.var_phi_0_dn7 - locals.var_dphi_sb__blk1547_dn7), (locals.var_phi_0_dn8 - locals.var_dphi_sb__blk1547_dn8), (locals.var_phi_0_dn9 - locals.var_dphi_sb__blk1547_dn9), (locals.var_phi_0_dn10 - locals.var_dphi_sb__blk1547_dn10), (locals.var_phi_0_dn13 - locals.var_dphi_sb__blk1547_dn13),)
    } else {
        (locals.var_phi_b__blk1551, locals.var_phi_b__blk1551_dn0, locals.var_phi_b__blk1551_dn2, locals.var_phi_b__blk1551_dn4, locals.var_phi_b__blk1551_dn5, locals.var_phi_b__blk1551_dn6, locals.var_phi_b__blk1551_dn7, locals.var_phi_b__blk1551_dn8, locals.var_phi_b__blk1551_dn9, locals.var_phi_b__blk1551_dn10, locals.var_phi_b__blk1551_dn13,)
    }
};
            locals.var_phi_b__blk1551 = assign65270_body9_e100975;
            locals.var_phi_b__blk1551_dn0 = assign65270_body9_e100975_d_n0;
            locals.var_phi_b__blk1551_dn2 = assign65270_body9_e100975_d_n2;
            locals.var_phi_b__blk1551_dn4 = assign65270_body9_e100975_d_n4;
            locals.var_phi_b__blk1551_dn5 = assign65270_body9_e100975_d_n5;
            locals.var_phi_b__blk1551_dn6 = assign65270_body9_e100975_d_n6;
            locals.var_phi_b__blk1551_dn7 = assign65270_body9_e100975_d_n7;
            locals.var_phi_b__blk1551_dn8 = assign65270_body9_e100975_d_n8;
            locals.var_phi_b__blk1551_dn9 = assign65270_body9_e100975_d_n9;
            locals.var_phi_b__blk1551_dn10 = assign65270_body9_e100975_d_n10;
            locals.var_phi_b__blk1551_dn13 = assign65270_body9_e100975_d_n13;
            locals.var_phi_b__blk1551_rv = 0.0;
            let (assign65270_body10_e100993, assign65270_body10_e100993_d_n0, assign65270_body10_e100993_d_n2, assign65270_body10_e100993_d_n4, assign65270_body10_e100993_d_n5, assign65270_body10_e100993_d_n6, assign65270_body10_e100993_d_n7, assign65270_body10_e100993_d_n8, assign65270_body10_e100993_d_n9, assign65270_body10_e100993_d_n10, assign65270_body10_e100993_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss__blk1552, locals.var_phi_b_dpss__blk1552_dn0, locals.var_phi_b_dpss__blk1552_dn2, locals.var_phi_b_dpss__blk1552_dn4, locals.var_phi_b_dpss__blk1552_dn5, locals.var_phi_b_dpss__blk1552_dn6, locals.var_phi_b_dpss__blk1552_dn7, locals.var_phi_b_dpss__blk1552_dn8, locals.var_phi_b_dpss__blk1552_dn9, locals.var_phi_b_dpss__blk1552_dn10, locals.var_phi_b_dpss__blk1552_dn13,)
    }
};
            locals.var_phi_b_dpss__blk1552 = assign65270_body10_e100993;
            locals.var_phi_b_dpss__blk1552_dn0 = assign65270_body10_e100993_d_n0;
            locals.var_phi_b_dpss__blk1552_dn2 = assign65270_body10_e100993_d_n2;
            locals.var_phi_b_dpss__blk1552_dn4 = assign65270_body10_e100993_d_n4;
            locals.var_phi_b_dpss__blk1552_dn5 = assign65270_body10_e100993_d_n5;
            locals.var_phi_b_dpss__blk1552_dn6 = assign65270_body10_e100993_d_n6;
            locals.var_phi_b_dpss__blk1552_dn7 = assign65270_body10_e100993_d_n7;
            locals.var_phi_b_dpss__blk1552_dn8 = assign65270_body10_e100993_d_n8;
            locals.var_phi_b_dpss__blk1552_dn9 = assign65270_body10_e100993_d_n9;
            locals.var_phi_b_dpss__blk1552_dn10 = assign65270_body10_e100993_d_n10;
            locals.var_phi_b_dpss__blk1552_dn13 = assign65270_body10_e100993_d_n13;
            locals.var_phi_b_dpss__blk1552_rv = 0.0;
            let (assign65270_body11_e101010, assign65270_body11_e101010_d_n0, assign65270_body11_e101010_d_n2, assign65270_body11_e101010_d_n4, assign65270_body11_e101010_d_n5, assign65270_body11_e101010_d_n6, assign65270_body11_e101010_d_n7, assign65270_body11_e101010_d_n8, assign65270_body11_e101010_d_n9, assign65270_body11_e101010_d_n10, assign65270_body11_e101010_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body11_e101008: f64 = (locals.var_beta * locals.var_phi_b__blk1551);
        (assign65270_body11_e101008, ((locals.var_beta_dn0 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn13)),)
    } else {
        (locals.var_chib__blk1550, locals.var_chib__blk1550_dn0, locals.var_chib__blk1550_dn2, locals.var_chib__blk1550_dn4, locals.var_chib__blk1550_dn5, locals.var_chib__blk1550_dn6, locals.var_chib__blk1550_dn7, locals.var_chib__blk1550_dn8, locals.var_chib__blk1550_dn9, locals.var_chib__blk1550_dn10, locals.var_chib__blk1550_dn13,)
    }
};
            locals.var_chib__blk1550 = assign65270_body11_e101010;
            locals.var_chib__blk1550_dn0 = assign65270_body11_e101010_d_n0;
            locals.var_chib__blk1550_dn2 = assign65270_body11_e101010_d_n2;
            locals.var_chib__blk1550_dn4 = assign65270_body11_e101010_d_n4;
            locals.var_chib__blk1550_dn5 = assign65270_body11_e101010_d_n5;
            locals.var_chib__blk1550_dn6 = assign65270_body11_e101010_d_n6;
            locals.var_chib__blk1550_dn7 = assign65270_body11_e101010_d_n7;
            locals.var_chib__blk1550_dn8 = assign65270_body11_e101010_d_n8;
            locals.var_chib__blk1550_dn9 = assign65270_body11_e101010_d_n9;
            locals.var_chib__blk1550_dn10 = assign65270_body11_e101010_d_n10;
            locals.var_chib__blk1550_dn13 = assign65270_body11_e101010_d_n13;
            locals.var_chib__blk1550_rv = 0.0;
            let assign65270_body12_e101012: f64 = (locals.var_chi).abs();
            let assign65270_body12_e101014: f64 = if assign65270_body12_e101012 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard1557 = assign65270_body12_e101014;
            locals.var_guard1557_rv = 0.0;
            let (assign65270_body13_e101038, assign65270_body13_e101038_d_n0, assign65270_body13_e101038_d_n2, assign65270_body13_e101038_d_n4, assign65270_body13_e101038_d_n5, assign65270_body13_e101038_d_n6, assign65270_body13_e101038_d_n7, assign65270_body13_e101038_d_n8, assign65270_body13_e101038_d_n9, assign65270_body13_e101038_d_n10, assign65270_body13_e101038_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) {
        let assign65270_body13_e101032: f64 = (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552);
        let assign65270_body13_e101033: f64 = (1.0 - assign65270_body13_e101032);
        let assign65270_body13_e101035: f64 = (assign65270_body13_e101033 / 2.0);
        let assign65270_body13_e101036: f64 = (assign65270_body13_e101035).sqrt();
        (assign65270_body13_e101036, (((-((locals.var_phi_b_dpss__blk1552_dn0 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn0))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn2 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn2))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn4 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn4))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn5 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn5))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn6 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn6))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn7 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn7))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn8 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn8))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn9 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn9))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn10 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn10))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn13 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn13))) / 2.0) / (2.0 * assign65270_body13_e101036)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign65270_body13_e101038;
            locals.var_t0_dn0 = assign65270_body13_e101038_d_n0;
            locals.var_t0_dn2 = assign65270_body13_e101038_d_n2;
            locals.var_t0_dn4 = assign65270_body13_e101038_d_n4;
            locals.var_t0_dn5 = assign65270_body13_e101038_d_n5;
            locals.var_t0_dn6 = assign65270_body13_e101038_d_n6;
            locals.var_t0_dn7 = assign65270_body13_e101038_d_n7;
            locals.var_t0_dn8 = assign65270_body13_e101038_d_n8;
            locals.var_t0_dn9 = assign65270_body13_e101038_d_n9;
            locals.var_t0_dn10 = assign65270_body13_e101038_d_n10;
            locals.var_t0_dn13 = assign65270_body13_e101038_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign65270_body14_e101057, assign65270_body14_e101057_d_n0, assign65270_body14_e101057_d_n2, assign65270_body14_e101057_d_n4, assign65270_body14_e101057_d_n5, assign65270_body14_e101057_d_n6, assign65270_body14_e101057_d_n7, assign65270_body14_e101057_d_n8, assign65270_body14_e101057_d_n9, assign65270_body14_e101057_d_n10, assign65270_body14_e101057_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) {
        let assign65270_body14_e101055: f64 = (locals.var_chi * locals.var_t0);
        (assign65270_body14_e101055, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign65270_body14_e101057;
            locals.var_fb_dn0 = assign65270_body14_e101057_d_n0;
            locals.var_fb_dn2 = assign65270_body14_e101057_d_n2;
            locals.var_fb_dn4 = assign65270_body14_e101057_d_n4;
            locals.var_fb_dn5 = assign65270_body14_e101057_d_n5;
            locals.var_fb_dn6 = assign65270_body14_e101057_d_n6;
            locals.var_fb_dn7 = assign65270_body14_e101057_d_n7;
            locals.var_fb_dn8 = assign65270_body14_e101057_d_n8;
            locals.var_fb_dn9 = assign65270_body14_e101057_d_n9;
            locals.var_fb_dn10 = assign65270_body14_e101057_d_n10;
            locals.var_fb_dn13 = assign65270_body14_e101057_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign65270_body15_e101076, assign65270_body15_e101076_d_n0, assign65270_body15_e101076_d_n2, assign65270_body15_e101076_d_n4, assign65270_body15_e101076_d_n5, assign65270_body15_e101076_d_n6, assign65270_body15_e101076_d_n7, assign65270_body15_e101076_d_n8, assign65270_body15_e101076_d_n9, assign65270_body15_e101076_d_n10, assign65270_body15_e101076_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) {
        let assign65270_body15_e101074: f64 = (locals.var_beta * locals.var_t0);
        (assign65270_body15_e101074, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    }
};
            locals.var_fb_dpss__blk1553 = assign65270_body15_e101076;
            locals.var_fb_dpss__blk1553_dn0 = assign65270_body15_e101076_d_n0;
            locals.var_fb_dpss__blk1553_dn2 = assign65270_body15_e101076_d_n2;
            locals.var_fb_dpss__blk1553_dn4 = assign65270_body15_e101076_d_n4;
            locals.var_fb_dpss__blk1553_dn5 = assign65270_body15_e101076_d_n5;
            locals.var_fb_dpss__blk1553_dn6 = assign65270_body15_e101076_d_n6;
            locals.var_fb_dpss__blk1553_dn7 = assign65270_body15_e101076_d_n7;
            locals.var_fb_dpss__blk1553_dn8 = assign65270_body15_e101076_d_n8;
            locals.var_fb_dpss__blk1553_dn9 = assign65270_body15_e101076_d_n9;
            locals.var_fb_dpss__blk1553_dn10 = assign65270_body15_e101076_d_n10;
            locals.var_fb_dpss__blk1553_dn13 = assign65270_body15_e101076_d_n13;
            locals.var_fb_dpss__blk1553_rv = 0.0;
            let assign65270_body16_e101079: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1558 = assign65270_body16_e101079;
            locals.var_guard1558_rv = 0.0;
            let (assign65270_body17_e101099, assign65270_body17_e101099_d_n0, assign65270_body17_e101099_d_n2, assign65270_body17_e101099_d_n4, assign65270_body17_e101099_d_n5, assign65270_body17_e101099_d_n6, assign65270_body17_e101099_d_n7, assign65270_body17_e101099_d_n8, assign65270_body17_e101099_d_n9, assign65270_body17_e101099_d_n10, assign65270_body17_e101099_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65270_body17_e101097: f64 = (-locals.var_fb);
        (assign65270_body17_e101097, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign65270_body17_e101099;
            locals.var_fb_dn0 = assign65270_body17_e101099_d_n0;
            locals.var_fb_dn2 = assign65270_body17_e101099_d_n2;
            locals.var_fb_dn4 = assign65270_body17_e101099_d_n4;
            locals.var_fb_dn5 = assign65270_body17_e101099_d_n5;
            locals.var_fb_dn6 = assign65270_body17_e101099_d_n6;
            locals.var_fb_dn7 = assign65270_body17_e101099_d_n7;
            locals.var_fb_dn8 = assign65270_body17_e101099_d_n8;
            locals.var_fb_dn9 = assign65270_body17_e101099_d_n9;
            locals.var_fb_dn10 = assign65270_body17_e101099_d_n10;
            locals.var_fb_dn13 = assign65270_body17_e101099_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign65270_body18_e101119, assign65270_body18_e101119_d_n0, assign65270_body18_e101119_d_n2, assign65270_body18_e101119_d_n4, assign65270_body18_e101119_d_n5, assign65270_body18_e101119_d_n6, assign65270_body18_e101119_d_n7, assign65270_body18_e101119_d_n8, assign65270_body18_e101119_d_n9, assign65270_body18_e101119_d_n10, assign65270_body18_e101119_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65270_body18_e101117: f64 = (-locals.var_fb_dpss__blk1553);
        (assign65270_body18_e101117, (-locals.var_fb_dpss__blk1553_dn0), (-locals.var_fb_dpss__blk1553_dn2), (-locals.var_fb_dpss__blk1553_dn4), (-locals.var_fb_dpss__blk1553_dn5), (-locals.var_fb_dpss__blk1553_dn6), (-locals.var_fb_dpss__blk1553_dn7), (-locals.var_fb_dpss__blk1553_dn8), (-locals.var_fb_dpss__blk1553_dn9), (-locals.var_fb_dpss__blk1553_dn10), (-locals.var_fb_dpss__blk1553_dn13),)
    } else {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    }
};
            locals.var_fb_dpss__blk1553 = assign65270_body18_e101119;
            locals.var_fb_dpss__blk1553_dn0 = assign65270_body18_e101119_d_n0;
            locals.var_fb_dpss__blk1553_dn2 = assign65270_body18_e101119_d_n2;
            locals.var_fb_dpss__blk1553_dn4 = assign65270_body18_e101119_d_n4;
            locals.var_fb_dpss__blk1553_dn5 = assign65270_body18_e101119_d_n5;
            locals.var_fb_dpss__blk1553_dn6 = assign65270_body18_e101119_d_n6;
            locals.var_fb_dpss__blk1553_dn7 = assign65270_body18_e101119_d_n7;
            locals.var_fb_dpss__blk1553_dn8 = assign65270_body18_e101119_d_n8;
            locals.var_fb_dpss__blk1553_dn9 = assign65270_body18_e101119_d_n9;
            locals.var_fb_dpss__blk1553_dn10 = assign65270_body18_e101119_d_n10;
            locals.var_fb_dpss__blk1553_dn13 = assign65270_body18_e101119_d_n13;
            locals.var_fb_dpss__blk1553_rv = 0.0;
            let assign65270_body19_e101121: f64 = (locals.var_chi).abs();
            let assign65270_body19_e101123: f64 = if assign65270_body19_e101121 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard1559 = assign65270_body19_e101123;
            locals.var_guard1559_rv = 0.0;
            let (assign65270_body20_e101165, assign65270_body20_e101165_d_n0, assign65270_body20_e101165_d_n2, assign65270_body20_e101165_d_n4, assign65270_body20_e101165_d_n5, assign65270_body20_e101165_d_n6, assign65270_body20_e101165_d_n7, assign65270_body20_e101165_d_n8, assign65270_body20_e101165_d_n9, assign65270_body20_e101165_d_n10, assign65270_body20_e101165_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body20_e101143: f64 = (locals.var_chi * locals.var_chi);
        let assign65270_body20_e101145: f64 = (assign65270_body20_e101143 / 2.0);
        let assign65270_body20_e101149: f64 = (locals.var_chi / 3.0);
        let assign65270_body20_e101153: f64 = (locals.var_chi / 4.0);
        let assign65270_body20_e101157: f64 = (locals.var_chi / 5.0);
        let assign65270_body20_e101158: f64 = (1.0 - assign65270_body20_e101157);
        let assign65270_body20_e101159: f64 = (assign65270_body20_e101153 * assign65270_body20_e101158);
        let assign65270_body20_e101160: f64 = (1.0 - assign65270_body20_e101159);
        let assign65270_body20_e101161: f64 = (assign65270_body20_e101149 * assign65270_body20_e101160);
        let assign65270_body20_e101162: f64 = (1.0 - assign65270_body20_e101161);
        let assign65270_body20_e101163: f64 = (assign65270_body20_e101145 * assign65270_body20_e101162);
        (assign65270_body20_e101163, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn0 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn0 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn2 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn2 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn4 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn4 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn5 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn5 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn6 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn6 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn7 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn7 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn8 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn8 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn9 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn9 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn10 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn10 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn13 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn13 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign65270_body20_e101165;
            locals.var_t0_dn0 = assign65270_body20_e101165_d_n0;
            locals.var_t0_dn2 = assign65270_body20_e101165_d_n2;
            locals.var_t0_dn4 = assign65270_body20_e101165_d_n4;
            locals.var_t0_dn5 = assign65270_body20_e101165_d_n5;
            locals.var_t0_dn6 = assign65270_body20_e101165_d_n6;
            locals.var_t0_dn7 = assign65270_body20_e101165_d_n7;
            locals.var_t0_dn8 = assign65270_body20_e101165_d_n8;
            locals.var_t0_dn9 = assign65270_body20_e101165_d_n9;
            locals.var_t0_dn10 = assign65270_body20_e101165_d_n10;
            locals.var_t0_dn13 = assign65270_body20_e101165_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign65270_body21_e101203, assign65270_body21_e101203_d_n0, assign65270_body21_e101203_d_n2, assign65270_body21_e101203_d_n4, assign65270_body21_e101203_d_n5, assign65270_body21_e101203_d_n6, assign65270_body21_e101203_d_n7, assign65270_body21_e101203_d_n8, assign65270_body21_e101203_d_n9, assign65270_body21_e101203_d_n10, assign65270_body21_e101203_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body21_e101187: f64 = (locals.var_chi / 2.0);
        let assign65270_body21_e101191: f64 = (locals.var_chi / 3.0);
        let assign65270_body21_e101195: f64 = (locals.var_chi / 4.0);
        let assign65270_body21_e101196: f64 = (1.0 - assign65270_body21_e101195);
        let assign65270_body21_e101197: f64 = (assign65270_body21_e101191 * assign65270_body21_e101196);
        let assign65270_body21_e101198: f64 = (1.0 - assign65270_body21_e101197);
        let assign65270_body21_e101199: f64 = (assign65270_body21_e101187 * assign65270_body21_e101198);
        let assign65270_body21_e101200: f64 = (1.0 - assign65270_body21_e101199);
        let assign65270_body21_e101201: f64 = (locals.var_chi * assign65270_body21_e101200);
        (assign65270_body21_e101201, ((locals.var_chi_dn0 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn0 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn2 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn4 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn5 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn6 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn7 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn8 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn9 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn10 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn13 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign65270_body21_e101203;
            locals.var_t1_dn0 = assign65270_body21_e101203_d_n0;
            locals.var_t1_dn2 = assign65270_body21_e101203_d_n2;
            locals.var_t1_dn4 = assign65270_body21_e101203_d_n4;
            locals.var_t1_dn5 = assign65270_body21_e101203_d_n5;
            locals.var_t1_dn6 = assign65270_body21_e101203_d_n6;
            locals.var_t1_dn7 = assign65270_body21_e101203_d_n7;
            locals.var_t1_dn8 = assign65270_body21_e101203_d_n8;
            locals.var_t1_dn9 = assign65270_body21_e101203_d_n9;
            locals.var_t1_dn10 = assign65270_body21_e101203_d_n10;
            locals.var_t1_dn13 = assign65270_body21_e101203_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign65270_body22_e101245, assign65270_body22_e101245_d_n0, assign65270_body22_e101245_d_n2, assign65270_body22_e101245_d_n4, assign65270_body22_e101245_d_n5, assign65270_body22_e101245_d_n6, assign65270_body22_e101245_d_n7, assign65270_body22_e101245_d_n8, assign65270_body22_e101245_d_n9, assign65270_body22_e101245_d_n10, assign65270_body22_e101245_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body22_e101223: f64 = (locals.var_chib__blk1550 * locals.var_chib__blk1550);
        let assign65270_body22_e101225: f64 = (assign65270_body22_e101223 / 2.0);
        let assign65270_body22_e101229: f64 = (locals.var_chib__blk1550 / 3.0);
        let assign65270_body22_e101233: f64 = (locals.var_chib__blk1550 / 4.0);
        let assign65270_body22_e101237: f64 = (locals.var_chib__blk1550 / 5.0);
        let assign65270_body22_e101238: f64 = (1.0 - assign65270_body22_e101237);
        let assign65270_body22_e101239: f64 = (assign65270_body22_e101233 * assign65270_body22_e101238);
        let assign65270_body22_e101240: f64 = (1.0 - assign65270_body22_e101239);
        let assign65270_body22_e101241: f64 = (assign65270_body22_e101229 * assign65270_body22_e101240);
        let assign65270_body22_e101242: f64 = (1.0 - assign65270_body22_e101241);
        let assign65270_body22_e101243: f64 = (assign65270_body22_e101225 * assign65270_body22_e101242);
        (assign65270_body22_e101243, (((((locals.var_chib__blk1550_dn0 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn0)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn0 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn0 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn0 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn2 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn2)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn2 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn2 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn2 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn4 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn4)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn4 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn4 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn4 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn5 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn5)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn5 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn5 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn5 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn6 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn6)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn6 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn6 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn6 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn7 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn7)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn7 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn7 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn7 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn8 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn8)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn8 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn8 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn8 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn9 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn9)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn9 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn9 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn9 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn10 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn10)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn10 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn10 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn10 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn13 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn13)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn13 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn13 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign65270_body22_e101245;
            locals.var_t2_dn0 = assign65270_body22_e101245_d_n0;
            locals.var_t2_dn2 = assign65270_body22_e101245_d_n2;
            locals.var_t2_dn4 = assign65270_body22_e101245_d_n4;
            locals.var_t2_dn5 = assign65270_body22_e101245_d_n5;
            locals.var_t2_dn6 = assign65270_body22_e101245_d_n6;
            locals.var_t2_dn7 = assign65270_body22_e101245_d_n7;
            locals.var_t2_dn8 = assign65270_body22_e101245_d_n8;
            locals.var_t2_dn9 = assign65270_body22_e101245_d_n9;
            locals.var_t2_dn10 = assign65270_body22_e101245_d_n10;
            locals.var_t2_dn13 = assign65270_body22_e101245_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign65270_body23_e101283, assign65270_body23_e101283_d_n0, assign65270_body23_e101283_d_n2, assign65270_body23_e101283_d_n4, assign65270_body23_e101283_d_n5, assign65270_body23_e101283_d_n6, assign65270_body23_e101283_d_n7, assign65270_body23_e101283_d_n8, assign65270_body23_e101283_d_n9, assign65270_body23_e101283_d_n10, assign65270_body23_e101283_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body23_e101267: f64 = (locals.var_chib__blk1550 / 2.0);
        let assign65270_body23_e101271: f64 = (locals.var_chib__blk1550 / 3.0);
        let assign65270_body23_e101275: f64 = (locals.var_chib__blk1550 / 4.0);
        let assign65270_body23_e101276: f64 = (1.0 - assign65270_body23_e101275);
        let assign65270_body23_e101277: f64 = (assign65270_body23_e101271 * assign65270_body23_e101276);
        let assign65270_body23_e101278: f64 = (1.0 - assign65270_body23_e101277);
        let assign65270_body23_e101279: f64 = (assign65270_body23_e101267 * assign65270_body23_e101278);
        let assign65270_body23_e101280: f64 = (1.0 - assign65270_body23_e101279);
        let assign65270_body23_e101281: f64 = (locals.var_chib__blk1550 * assign65270_body23_e101280);
        (assign65270_body23_e101281, ((locals.var_chib__blk1550_dn0 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn0 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn0 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn0 / 4.0)))))))))), ((locals.var_chib__blk1550_dn2 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn2 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn2 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn2 / 4.0)))))))))), ((locals.var_chib__blk1550_dn4 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn4 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn4 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn4 / 4.0)))))))))), ((locals.var_chib__blk1550_dn5 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn5 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn5 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn5 / 4.0)))))))))), ((locals.var_chib__blk1550_dn6 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn6 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn6 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn6 / 4.0)))))))))), ((locals.var_chib__blk1550_dn7 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn7 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn7 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn7 / 4.0)))))))))), ((locals.var_chib__blk1550_dn8 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn8 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn8 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn8 / 4.0)))))))))), ((locals.var_chib__blk1550_dn9 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn9 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn9 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn9 / 4.0)))))))))), ((locals.var_chib__blk1550_dn10 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn10 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn10 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn10 / 4.0)))))))))), ((locals.var_chib__blk1550_dn13 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn13 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn13 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign65270_body23_e101283;
            locals.var_t3_dn0 = assign65270_body23_e101283_d_n0;
            locals.var_t3_dn2 = assign65270_body23_e101283_d_n2;
            locals.var_t3_dn4 = assign65270_body23_e101283_d_n4;
            locals.var_t3_dn5 = assign65270_body23_e101283_d_n5;
            locals.var_t3_dn6 = assign65270_body23_e101283_d_n6;
            locals.var_t3_dn7 = assign65270_body23_e101283_d_n7;
            locals.var_t3_dn8 = assign65270_body23_e101283_d_n8;
            locals.var_t3_dn9 = assign65270_body23_e101283_d_n9;
            locals.var_t3_dn10 = assign65270_body23_e101283_d_n10;
            locals.var_t3_dn13 = assign65270_body23_e101283_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign65270_body24_e101306, assign65270_body24_e101306_d_n0, assign65270_body24_e101306_d_n2, assign65270_body24_e101306_d_n4, assign65270_body24_e101306_d_n5, assign65270_body24_e101306_d_n6, assign65270_body24_e101306_d_n7, assign65270_body24_e101306_d_n8, assign65270_body24_e101306_d_n9, assign65270_body24_e101306_d_n10, assign65270_body24_e101306_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body24_e101303: f64 = (locals.var_t0 - locals.var_t2);
        let assign65270_body24_e101304: f64 = (assign65270_body24_e101303).sqrt();
        (assign65270_body24_e101304, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn4 - locals.var_t2_dn4) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn5 - locals.var_t2_dn5) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn8 - locals.var_t2_dn8) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn9 - locals.var_t2_dn9) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn13 - locals.var_t2_dn13) / (2.0 * assign65270_body24_e101304)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign65270_body24_e101306;
            locals.var_fb_dn0 = assign65270_body24_e101306_d_n0;
            locals.var_fb_dn2 = assign65270_body24_e101306_d_n2;
            locals.var_fb_dn4 = assign65270_body24_e101306_d_n4;
            locals.var_fb_dn5 = assign65270_body24_e101306_d_n5;
            locals.var_fb_dn6 = assign65270_body24_e101306_d_n6;
            locals.var_fb_dn7 = assign65270_body24_e101306_d_n7;
            locals.var_fb_dn8 = assign65270_body24_e101306_d_n8;
            locals.var_fb_dn9 = assign65270_body24_e101306_d_n9;
            locals.var_fb_dn10 = assign65270_body24_e101306_d_n10;
            locals.var_fb_dn13 = assign65270_body24_e101306_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign65270_body25_e101336, assign65270_body25_e101336_d_n0, assign65270_body25_e101336_d_n2, assign65270_body25_e101336_d_n4, assign65270_body25_e101336_d_n5, assign65270_body25_e101336_d_n6, assign65270_body25_e101336_d_n7, assign65270_body25_e101336_d_n8, assign65270_body25_e101336_d_n9, assign65270_body25_e101336_d_n10, assign65270_body25_e101336_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body25_e101326: f64 = (locals.var_beta * 0.5);
        let assign65270_body25_e101330: f64 = (locals.var_phi_b_dpss__blk1552 * locals.var_t3);
        let assign65270_body25_e101331: f64 = (locals.var_t1 - assign65270_body25_e101330);
        let assign65270_body25_e101332: f64 = (assign65270_body25_e101326 * assign65270_body25_e101331);
        let assign65270_body25_e101334: f64 = (assign65270_body25_e101332 / locals.var_fb);
        (assign65270_body25_e101334, ((((((locals.var_beta_dn0 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss__blk1552_dn0 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn0))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss__blk1552_dn2 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn2))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss__blk1552_dn4 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn4))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss__blk1552_dn5 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn5))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss__blk1552_dn6 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn6))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss__blk1552_dn7 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn7))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss__blk1552_dn8 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn8))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss__blk1552_dn9 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn9))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss__blk1552_dn10 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn10))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss__blk1552_dn13 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn13))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    }
};
            locals.var_fb_dpss__blk1553 = assign65270_body25_e101336;
            locals.var_fb_dpss__blk1553_dn0 = assign65270_body25_e101336_d_n0;
            locals.var_fb_dpss__blk1553_dn2 = assign65270_body25_e101336_d_n2;
            locals.var_fb_dpss__blk1553_dn4 = assign65270_body25_e101336_d_n4;
            locals.var_fb_dpss__blk1553_dn5 = assign65270_body25_e101336_d_n5;
            locals.var_fb_dpss__blk1553_dn6 = assign65270_body25_e101336_d_n6;
            locals.var_fb_dpss__blk1553_dn7 = assign65270_body25_e101336_d_n7;
            locals.var_fb_dpss__blk1553_dn8 = assign65270_body25_e101336_d_n8;
            locals.var_fb_dpss__blk1553_dn9 = assign65270_body25_e101336_d_n9;
            locals.var_fb_dpss__blk1553_dn10 = assign65270_body25_e101336_d_n10;
            locals.var_fb_dpss__blk1553_dn13 = assign65270_body25_e101336_d_n13;
            locals.var_fb_dpss__blk1553_rv = 0.0;
            let (assign65270_body26_e101359, assign65270_body26_e101359_d_n0, assign65270_body26_e101359_d_n2, assign65270_body26_e101359_d_n4, assign65270_body26_e101359_d_n5, assign65270_body26_e101359_d_n6, assign65270_body26_e101359_d_n7, assign65270_body26_e101359_d_n8, assign65270_body26_e101359_d_n9, assign65270_body26_e101359_d_n10, assign65270_body26_e101359_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65270_body26_e101356: f64 = (-locals.var_chi);
        let assign65270_body26_e101357: f64 = (assign65270_body26_e101356).exp();
        (assign65270_body26_e101357, (assign65270_body26_e101357 * (-locals.var_chi_dn0)), (assign65270_body26_e101357 * (-locals.var_chi_dn2)), (assign65270_body26_e101357 * (-locals.var_chi_dn4)), (assign65270_body26_e101357 * (-locals.var_chi_dn5)), (assign65270_body26_e101357 * (-locals.var_chi_dn6)), (assign65270_body26_e101357 * (-locals.var_chi_dn7)), (assign65270_body26_e101357 * (-locals.var_chi_dn8)), (assign65270_body26_e101357 * (-locals.var_chi_dn9)), (assign65270_body26_e101357 * (-locals.var_chi_dn10)), (assign65270_body26_e101357 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign65270_body26_e101359;
            locals.var_t0_dn0 = assign65270_body26_e101359_d_n0;
            locals.var_t0_dn2 = assign65270_body26_e101359_d_n2;
            locals.var_t0_dn4 = assign65270_body26_e101359_d_n4;
            locals.var_t0_dn5 = assign65270_body26_e101359_d_n5;
            locals.var_t0_dn6 = assign65270_body26_e101359_d_n6;
            locals.var_t0_dn7 = assign65270_body26_e101359_d_n7;
            locals.var_t0_dn8 = assign65270_body26_e101359_d_n8;
            locals.var_t0_dn9 = assign65270_body26_e101359_d_n9;
            locals.var_t0_dn10 = assign65270_body26_e101359_d_n10;
            locals.var_t0_dn13 = assign65270_body26_e101359_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign65270_body27_e101382, assign65270_body27_e101382_d_n0, assign65270_body27_e101382_d_n2, assign65270_body27_e101382_d_n4, assign65270_body27_e101382_d_n5, assign65270_body27_e101382_d_n6, assign65270_body27_e101382_d_n7, assign65270_body27_e101382_d_n8, assign65270_body27_e101382_d_n9, assign65270_body27_e101382_d_n10, assign65270_body27_e101382_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65270_body27_e101379: f64 = (-locals.var_chib__blk1550);
        let assign65270_body27_e101380: f64 = (assign65270_body27_e101379).exp();
        (assign65270_body27_e101380, (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn0)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn2)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn4)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn5)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn6)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn7)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn8)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn9)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn10)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign65270_body27_e101382;
            locals.var_t1_dn0 = assign65270_body27_e101382_d_n0;
            locals.var_t1_dn2 = assign65270_body27_e101382_d_n2;
            locals.var_t1_dn4 = assign65270_body27_e101382_d_n4;
            locals.var_t1_dn5 = assign65270_body27_e101382_d_n5;
            locals.var_t1_dn6 = assign65270_body27_e101382_d_n6;
            locals.var_t1_dn7 = assign65270_body27_e101382_d_n7;
            locals.var_t1_dn8 = assign65270_body27_e101382_d_n8;
            locals.var_t1_dn9 = assign65270_body27_e101382_d_n9;
            locals.var_t1_dn10 = assign65270_body27_e101382_d_n10;
            locals.var_t1_dn13 = assign65270_body27_e101382_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign65270_body28_e101410, assign65270_body28_e101410_d_n0, assign65270_body28_e101410_d_n2, assign65270_body28_e101410_d_n4, assign65270_body28_e101410_d_n5, assign65270_body28_e101410_d_n6, assign65270_body28_e101410_d_n7, assign65270_body28_e101410_d_n8, assign65270_body28_e101410_d_n9, assign65270_body28_e101410_d_n10, assign65270_body28_e101410_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65270_body28_e101403: f64 = (locals.var_chi - locals.var_chib__blk1550);
        let assign65270_body28_e101406: f64 = (locals.var_t0 - locals.var_t1);
        let assign65270_body28_e101407: f64 = (assign65270_body28_e101403 + assign65270_body28_e101406);
        let assign65270_body28_e101408: f64 = (assign65270_body28_e101407).sqrt();
        (assign65270_body28_e101408, (((locals.var_chi_dn0 - locals.var_chib__blk1550_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn2 - locals.var_chib__blk1550_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn4 - locals.var_chib__blk1550_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn5 - locals.var_chib__blk1550_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn6 - locals.var_chib__blk1550_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn7 - locals.var_chib__blk1550_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn8 - locals.var_chib__blk1550_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn9 - locals.var_chib__blk1550_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn10 - locals.var_chib__blk1550_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn13 - locals.var_chib__blk1550_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)) / (2.0 * assign65270_body28_e101408)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign65270_body28_e101410;
            locals.var_fb_dn0 = assign65270_body28_e101410_d_n0;
            locals.var_fb_dn2 = assign65270_body28_e101410_d_n2;
            locals.var_fb_dn4 = assign65270_body28_e101410_d_n4;
            locals.var_fb_dn5 = assign65270_body28_e101410_d_n5;
            locals.var_fb_dn6 = assign65270_body28_e101410_d_n6;
            locals.var_fb_dn7 = assign65270_body28_e101410_d_n7;
            locals.var_fb_dn8 = assign65270_body28_e101410_d_n8;
            locals.var_fb_dn9 = assign65270_body28_e101410_d_n9;
            locals.var_fb_dn10 = assign65270_body28_e101410_d_n10;
            locals.var_fb_dn13 = assign65270_body28_e101410_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign65270_body29_e101445, assign65270_body29_e101445_d_n0, assign65270_body29_e101445_d_n2, assign65270_body29_e101445_d_n4, assign65270_body29_e101445_d_n5, assign65270_body29_e101445_d_n6, assign65270_body29_e101445_d_n7, assign65270_body29_e101445_d_n8, assign65270_body29_e101445_d_n9, assign65270_body29_e101445_d_n10, assign65270_body29_e101445_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65270_body29_e101431: f64 = (locals.var_beta * 0.5);
        let assign65270_body29_e101434: f64 = (1.0 - locals.var_t0);
        let assign65270_body29_e101438: f64 = (1.0 - locals.var_t1);
        let assign65270_body29_e101439: f64 = (locals.var_phi_b_dpss__blk1552 * assign65270_body29_e101438);
        let assign65270_body29_e101440: f64 = (assign65270_body29_e101434 - assign65270_body29_e101439);
        let assign65270_body29_e101441: f64 = (assign65270_body29_e101431 * assign65270_body29_e101440);
        let assign65270_body29_e101443: f64 = (assign65270_body29_e101441 / locals.var_fb);
        (assign65270_body29_e101443, ((((((locals.var_beta_dn0 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss__blk1552_dn0 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss__blk1552_dn2 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss__blk1552_dn4 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss__blk1552_dn5 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss__blk1552_dn6 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss__blk1552_dn7 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss__blk1552_dn8 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss__blk1552_dn9 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss__blk1552_dn10 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss__blk1552_dn13 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    }
};
            locals.var_fb_dpss__blk1553 = assign65270_body29_e101445;
            locals.var_fb_dpss__blk1553_dn0 = assign65270_body29_e101445_d_n0;
            locals.var_fb_dpss__blk1553_dn2 = assign65270_body29_e101445_d_n2;
            locals.var_fb_dpss__blk1553_dn4 = assign65270_body29_e101445_d_n4;
            locals.var_fb_dpss__blk1553_dn5 = assign65270_body29_e101445_d_n5;
            locals.var_fb_dpss__blk1553_dn6 = assign65270_body29_e101445_d_n6;
            locals.var_fb_dpss__blk1553_dn7 = assign65270_body29_e101445_d_n7;
            locals.var_fb_dpss__blk1553_dn8 = assign65270_body29_e101445_d_n8;
            locals.var_fb_dpss__blk1553_dn9 = assign65270_body29_e101445_d_n9;
            locals.var_fb_dpss__blk1553_dn10 = assign65270_body29_e101445_d_n10;
            locals.var_fb_dpss__blk1553_dn13 = assign65270_body29_e101445_d_n13;
            locals.var_fb_dpss__blk1553_rv = 0.0;
            let assign65270_body30_e101452: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard1560 = assign65270_body30_e101452;
            locals.var_guard1560_rv = 0.0;
            let (assign65270_body31_e101470,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65270_body31_e101468: f64 = (-1.0);
        (assign65270_body31_e101468,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign65270_body31_e101470;
            locals.var_flg_zone_rv = 0.0;
            let assign65270_body32_e101473: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1561 = assign65270_body32_e101473;
            locals.var_guard1561_rv = 0.0;
            let (assign65270_body33_e101491, assign65270_body33_e101491_d_n0, assign65270_body33_e101491_d_n2, assign65270_body33_e101491_d_n4, assign65270_body33_e101491_d_n5, assign65270_body33_e101491_d_n6, assign65270_body33_e101491_d_n7, assign65270_body33_e101491_d_n8, assign65270_body33_e101491_d_n9, assign65270_body33_e101491_d_n10, assign65270_body33_e101491_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65270_body33_e101489: f64 = (-locals.var_fb);
        (assign65270_body33_e101489, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign65270_body33_e101491;
            locals.var_fs02_dn0 = assign65270_body33_e101491_d_n0;
            locals.var_fs02_dn2 = assign65270_body33_e101491_d_n2;
            locals.var_fs02_dn4 = assign65270_body33_e101491_d_n4;
            locals.var_fs02_dn5 = assign65270_body33_e101491_d_n5;
            locals.var_fs02_dn6 = assign65270_body33_e101491_d_n6;
            locals.var_fs02_dn7 = assign65270_body33_e101491_d_n7;
            locals.var_fs02_dn8 = assign65270_body33_e101491_d_n8;
            locals.var_fs02_dn9 = assign65270_body33_e101491_d_n9;
            locals.var_fs02_dn10 = assign65270_body33_e101491_d_n10;
            locals.var_fs02_dn13 = assign65270_body33_e101491_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign65270_body34_e101509, assign65270_body34_e101509_d_n0, assign65270_body34_e101509_d_n2, assign65270_body34_e101509_d_n4, assign65270_body34_e101509_d_n5, assign65270_body34_e101509_d_n6, assign65270_body34_e101509_d_n7, assign65270_body34_e101509_d_n8, assign65270_body34_e101509_d_n9, assign65270_body34_e101509_d_n10, assign65270_body34_e101509_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65270_body34_e101507: f64 = (-locals.var_fb_dpss__blk1553);
        (assign65270_body34_e101507, (-locals.var_fb_dpss__blk1553_dn0), (-locals.var_fb_dpss__blk1553_dn2), (-locals.var_fb_dpss__blk1553_dn4), (-locals.var_fb_dpss__blk1553_dn5), (-locals.var_fb_dpss__blk1553_dn6), (-locals.var_fb_dpss__blk1553_dn7), (-locals.var_fb_dpss__blk1553_dn8), (-locals.var_fb_dpss__blk1553_dn9), (-locals.var_fb_dpss__blk1553_dn10), (-locals.var_fb_dpss__blk1553_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign65270_body34_e101509;
            locals.var_fs02_dps0_dn0 = assign65270_body34_e101509_d_n0;
            locals.var_fs02_dps0_dn2 = assign65270_body34_e101509_d_n2;
            locals.var_fs02_dps0_dn4 = assign65270_body34_e101509_d_n4;
            locals.var_fs02_dps0_dn5 = assign65270_body34_e101509_d_n5;
            locals.var_fs02_dps0_dn6 = assign65270_body34_e101509_d_n6;
            locals.var_fs02_dps0_dn7 = assign65270_body34_e101509_d_n7;
            locals.var_fs02_dps0_dn8 = assign65270_body34_e101509_d_n8;
            locals.var_fs02_dps0_dn9 = assign65270_body34_e101509_d_n9;
            locals.var_fs02_dps0_dn10 = assign65270_body34_e101509_d_n10;
            locals.var_fs02_dps0_dn13 = assign65270_body34_e101509_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign65270_body35_e101512: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard1562 = assign65270_body35_e101512;
            locals.var_guard1562_rv = 0.0;
            let (assign65270_body36_e101532, assign65270_body36_e101532_d_n0, assign65270_body36_e101532_d_n2, assign65270_body36_e101532_d_n4, assign65270_body36_e101532_d_n5, assign65270_body36_e101532_d_n6, assign65270_body36_e101532_d_n7, assign65270_body36_e101532_d_n8, assign65270_body36_e101532_d_n9, assign65270_body36_e101532_d_n10, assign65270_body36_e101532_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign65270_body36_e101532;
            locals.var_fs02_dn0 = assign65270_body36_e101532_d_n0;
            locals.var_fs02_dn2 = assign65270_body36_e101532_d_n2;
            locals.var_fs02_dn4 = assign65270_body36_e101532_d_n4;
            locals.var_fs02_dn5 = assign65270_body36_e101532_d_n5;
            locals.var_fs02_dn6 = assign65270_body36_e101532_d_n6;
            locals.var_fs02_dn7 = assign65270_body36_e101532_d_n7;
            locals.var_fs02_dn8 = assign65270_body36_e101532_d_n8;
            locals.var_fs02_dn9 = assign65270_body36_e101532_d_n9;
            locals.var_fs02_dn10 = assign65270_body36_e101532_d_n10;
            locals.var_fs02_dn13 = assign65270_body36_e101532_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign65270_body37_e101552, assign65270_body37_e101552_d_n0, assign65270_body37_e101552_d_n2, assign65270_body37_e101552_d_n4, assign65270_body37_e101552_d_n5, assign65270_body37_e101552_d_n6, assign65270_body37_e101552_d_n7, assign65270_body37_e101552_d_n8, assign65270_body37_e101552_d_n9, assign65270_body37_e101552_d_n10, assign65270_body37_e101552_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 != 0.0)) {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign65270_body37_e101552;
            locals.var_fs02_dps0_dn0 = assign65270_body37_e101552_d_n0;
            locals.var_fs02_dps0_dn2 = assign65270_body37_e101552_d_n2;
            locals.var_fs02_dps0_dn4 = assign65270_body37_e101552_d_n4;
            locals.var_fs02_dps0_dn5 = assign65270_body37_e101552_d_n5;
            locals.var_fs02_dps0_dn6 = assign65270_body37_e101552_d_n6;
            locals.var_fs02_dps0_dn7 = assign65270_body37_e101552_d_n7;
            locals.var_fs02_dps0_dn8 = assign65270_body37_e101552_d_n8;
            locals.var_fs02_dps0_dn9 = assign65270_body37_e101552_d_n9;
            locals.var_fs02_dps0_dn10 = assign65270_body37_e101552_d_n10;
            locals.var_fs02_dps0_dn13 = assign65270_body37_e101552_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign65270_body38_e101577, assign65270_body38_e101577_d_n0, assign65270_body38_e101577_d_n2, assign65270_body38_e101577_d_n4, assign65270_body38_e101577_d_n5, assign65270_body38_e101577_d_n6, assign65270_body38_e101577_d_n7, assign65270_body38_e101577_d_n8, assign65270_body38_e101577_d_n9, assign65270_body38_e101577_d_n10, assign65270_body38_e101577_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body38_e101574: f64 = (locals.var_phi_s0 - p.p456);
        let assign65270_body38_e101575: f64 = (locals.var_beta * assign65270_body38_e101574);
        (assign65270_body38_e101575, ((locals.var_beta_dn0 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn0)), ((locals.var_beta_dn2 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn2)), ((locals.var_beta_dn4 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn4)), ((locals.var_beta_dn5 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn5)), ((locals.var_beta_dn6 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn6)), ((locals.var_beta_dn7 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn7)), ((locals.var_beta_dn8 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn8)), ((locals.var_beta_dn9 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn9)), ((locals.var_beta_dn10 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn10)), ((locals.var_beta_dn13 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn13)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn13,)
    }
};
            locals.var_rho = assign65270_body38_e101577;
            locals.var_rho_dn0 = assign65270_body38_e101577_d_n0;
            locals.var_rho_dn2 = assign65270_body38_e101577_d_n2;
            locals.var_rho_dn4 = assign65270_body38_e101577_d_n4;
            locals.var_rho_dn5 = assign65270_body38_e101577_d_n5;
            locals.var_rho_dn6 = assign65270_body38_e101577_d_n6;
            locals.var_rho_dn7 = assign65270_body38_e101577_d_n7;
            locals.var_rho_dn8 = assign65270_body38_e101577_d_n8;
            locals.var_rho_dn9 = assign65270_body38_e101577_d_n9;
            locals.var_rho_dn10 = assign65270_body38_e101577_d_n10;
            locals.var_rho_dn13 = assign65270_body38_e101577_d_n13;
            locals.var_rho_rv = 0.0;
            let (assign65270_body39_e101599, assign65270_body39_e101599_d_n0, assign65270_body39_e101599_d_n2, assign65270_body39_e101599_d_n4, assign65270_body39_e101599_d_n5, assign65270_body39_e101599_d_n6, assign65270_body39_e101599_d_n7, assign65270_body39_e101599_d_n8, assign65270_body39_e101599_d_n9, assign65270_body39_e101599_d_n10, assign65270_body39_e101599_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body39_e101597: f64 = (locals.var_rho).exp();
        (assign65270_body39_e101597, (assign65270_body39_e101597 * locals.var_rho_dn0), (assign65270_body39_e101597 * locals.var_rho_dn2), (assign65270_body39_e101597 * locals.var_rho_dn4), (assign65270_body39_e101597 * locals.var_rho_dn5), (assign65270_body39_e101597 * locals.var_rho_dn6), (assign65270_body39_e101597 * locals.var_rho_dn7), (assign65270_body39_e101597 * locals.var_rho_dn8), (assign65270_body39_e101597 * locals.var_rho_dn9), (assign65270_body39_e101597 * locals.var_rho_dn10), (assign65270_body39_e101597 * locals.var_rho_dn13),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn4, locals.var_exp_rho_dn5, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn8, locals.var_exp_rho_dn9, locals.var_exp_rho_dn10, locals.var_exp_rho_dn13,)
    }
};
            locals.var_exp_rho = assign65270_body39_e101599;
            locals.var_exp_rho_dn0 = assign65270_body39_e101599_d_n0;
            locals.var_exp_rho_dn2 = assign65270_body39_e101599_d_n2;
            locals.var_exp_rho_dn4 = assign65270_body39_e101599_d_n4;
            locals.var_exp_rho_dn5 = assign65270_body39_e101599_d_n5;
            locals.var_exp_rho_dn6 = assign65270_body39_e101599_d_n6;
            locals.var_exp_rho_dn7 = assign65270_body39_e101599_d_n7;
            locals.var_exp_rho_dn8 = assign65270_body39_e101599_d_n8;
            locals.var_exp_rho_dn9 = assign65270_body39_e101599_d_n9;
            locals.var_exp_rho_dn10 = assign65270_body39_e101599_d_n10;
            locals.var_exp_rho_dn13 = assign65270_body39_e101599_d_n13;
            locals.var_exp_rho_rv = 0.0;
            let (assign65270_body40_e101628, assign65270_body40_e101628_d_n0, assign65270_body40_e101628_d_n2, assign65270_body40_e101628_d_n4, assign65270_body40_e101628_d_n5, assign65270_body40_e101628_d_n6, assign65270_body40_e101628_d_n7, assign65270_body40_e101628_d_n8, assign65270_body40_e101628_d_n9, assign65270_body40_e101628_d_n10, assign65270_body40_e101628_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body40_e101623: f64 = (locals.var_chi + 1.0);
        let assign65270_body40_e101624: f64 = (locals.var_exp_bvbsvds * assign65270_body40_e101623);
        let assign65270_body40_e101625: f64 = (locals.var_exp_rho - assign65270_body40_e101624);
        let assign65270_body40_e101626: f64 = (locals.var_cnst1 * assign65270_body40_e101625);
        (assign65270_body40_e101626, ((locals.var_cnst1_dn0 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1_dn2 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1_dn4 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn4 - ((locals.var_exp_bvbsvds_dn4 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn4))))), ((locals.var_cnst1_dn5 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn5 - ((locals.var_exp_bvbsvds_dn5 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn5))))), ((locals.var_cnst1_dn6 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1_dn7 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1_dn8 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn8 - ((locals.var_exp_bvbsvds_dn8 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn8))))), ((locals.var_cnst1_dn9 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn9 - ((locals.var_exp_bvbsvds_dn9 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn9))))), ((locals.var_cnst1_dn10 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1_dn13 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn13 - ((locals.var_exp_bvbsvds_dn13 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign65270_body40_e101628;
            locals.var_fs01_dn0 = assign65270_body40_e101628_d_n0;
            locals.var_fs01_dn2 = assign65270_body40_e101628_d_n2;
            locals.var_fs01_dn4 = assign65270_body40_e101628_d_n4;
            locals.var_fs01_dn5 = assign65270_body40_e101628_d_n5;
            locals.var_fs01_dn6 = assign65270_body40_e101628_d_n6;
            locals.var_fs01_dn7 = assign65270_body40_e101628_d_n7;
            locals.var_fs01_dn8 = assign65270_body40_e101628_d_n8;
            locals.var_fs01_dn9 = assign65270_body40_e101628_d_n9;
            locals.var_fs01_dn10 = assign65270_body40_e101628_d_n10;
            locals.var_fs01_dn13 = assign65270_body40_e101628_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign65270_body41_e101655, assign65270_body41_e101655_d_n0, assign65270_body41_e101655_d_n2, assign65270_body41_e101655_d_n4, assign65270_body41_e101655_d_n5, assign65270_body41_e101655_d_n6, assign65270_body41_e101655_d_n7, assign65270_body41_e101655_d_n8, assign65270_body41_e101655_d_n9, assign65270_body41_e101655_d_n10, assign65270_body41_e101655_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body41_e101649: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign65270_body41_e101652: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign65270_body41_e101653: f64 = (assign65270_body41_e101649 * assign65270_body41_e101652);
        (assign65270_body41_e101653, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn4 - locals.var_exp_bvbsvds_dn4))), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn5 - locals.var_exp_bvbsvds_dn5))), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn8 - locals.var_exp_bvbsvds_dn8))), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn9 - locals.var_exp_bvbsvds_dn9))), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), ((((locals.var_cnst1_dn13 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn13)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn13 - locals.var_exp_bvbsvds_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign65270_body41_e101655;
            locals.var_fs01_dps0_dn0 = assign65270_body41_e101655_d_n0;
            locals.var_fs01_dps0_dn2 = assign65270_body41_e101655_d_n2;
            locals.var_fs01_dps0_dn4 = assign65270_body41_e101655_d_n4;
            locals.var_fs01_dps0_dn5 = assign65270_body41_e101655_d_n5;
            locals.var_fs01_dps0_dn6 = assign65270_body41_e101655_d_n6;
            locals.var_fs01_dps0_dn7 = assign65270_body41_e101655_d_n7;
            locals.var_fs01_dps0_dn8 = assign65270_body41_e101655_d_n8;
            locals.var_fs01_dps0_dn9 = assign65270_body41_e101655_d_n9;
            locals.var_fs01_dps0_dn10 = assign65270_body41_e101655_d_n10;
            locals.var_fs01_dps0_dn13 = assign65270_body41_e101655_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign65270_body42_e101681, assign65270_body42_e101681_d_n0, assign65270_body42_e101681_d_n2, assign65270_body42_e101681_d_n4, assign65270_body42_e101681_d_n5, assign65270_body42_e101681_d_n6, assign65270_body42_e101681_d_n7, assign65270_body42_e101681_d_n8, assign65270_body42_e101681_d_n9, assign65270_body42_e101681_d_n10, assign65270_body42_e101681_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body42_e101676: f64 = (locals.var_fb * locals.var_fb);
        let assign65270_body42_e101678: f64 = (assign65270_body42_e101676 + locals.var_fs01);
        let assign65270_body42_e101679: f64 = (assign65270_body42_e101678).sqrt();
        (assign65270_body42_e101679, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign65270_body42_e101679)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign65270_body42_e101681;
            locals.var_fs02_dn0 = assign65270_body42_e101681_d_n0;
            locals.var_fs02_dn2 = assign65270_body42_e101681_d_n2;
            locals.var_fs02_dn4 = assign65270_body42_e101681_d_n4;
            locals.var_fs02_dn5 = assign65270_body42_e101681_d_n5;
            locals.var_fs02_dn6 = assign65270_body42_e101681_d_n6;
            locals.var_fs02_dn7 = assign65270_body42_e101681_d_n7;
            locals.var_fs02_dn8 = assign65270_body42_e101681_d_n8;
            locals.var_fs02_dn9 = assign65270_body42_e101681_d_n9;
            locals.var_fs02_dn10 = assign65270_body42_e101681_d_n10;
            locals.var_fs02_dn13 = assign65270_body42_e101681_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign65270_body43_e101712, assign65270_body43_e101712_d_n0, assign65270_body43_e101712_d_n2, assign65270_body43_e101712_d_n4, assign65270_body43_e101712_d_n5, assign65270_body43_e101712_d_n6, assign65270_body43_e101712_d_n7, assign65270_body43_e101712_d_n8, assign65270_body43_e101712_d_n9, assign65270_body43_e101712_d_n10, assign65270_body43_e101712_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body43_e101703: f64 = (2.0 * locals.var_fb_dpss__blk1553);
        let assign65270_body43_e101705: f64 = (assign65270_body43_e101703 * locals.var_fb);
        let assign65270_body43_e101707: f64 = (assign65270_body43_e101705 + locals.var_fs01_dps0);
        let assign65270_body43_e101708: f64 = (0.5 * assign65270_body43_e101707);
        let assign65270_body43_e101710: f64 = (assign65270_body43_e101708 / locals.var_fs02);
        (assign65270_body43_e101710, ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn0) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn2) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn4) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn5) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn6) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn7) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn8) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn9) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn10) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn13) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign65270_body43_e101712;
            locals.var_fs02_dps0_dn0 = assign65270_body43_e101712_d_n0;
            locals.var_fs02_dps0_dn2 = assign65270_body43_e101712_d_n2;
            locals.var_fs02_dps0_dn4 = assign65270_body43_e101712_d_n4;
            locals.var_fs02_dps0_dn5 = assign65270_body43_e101712_d_n5;
            locals.var_fs02_dps0_dn6 = assign65270_body43_e101712_d_n6;
            locals.var_fs02_dps0_dn7 = assign65270_body43_e101712_d_n7;
            locals.var_fs02_dps0_dn8 = assign65270_body43_e101712_d_n8;
            locals.var_fs02_dps0_dn9 = assign65270_body43_e101712_d_n9;
            locals.var_fs02_dps0_dn10 = assign65270_body43_e101712_d_n10;
            locals.var_fs02_dps0_dn13 = assign65270_body43_e101712_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign65270_body44_e101734, assign65270_body44_e101734_d_n0, assign65270_body44_e101734_d_n2, assign65270_body44_e101734_d_n4, assign65270_body44_e101734_d_n5, assign65270_body44_e101734_d_n6, assign65270_body44_e101734_d_n7, assign65270_body44_e101734_d_n8, assign65270_body44_e101734_d_n9, assign65270_body44_e101734_d_n10, assign65270_body44_e101734_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body44_e101726: f64 = (-locals.var_vgp__blk1525);
        let assign65270_body44_e101728: f64 = (assign65270_body44_e101726 + locals.var_phi_s0);
        let assign65270_body44_e101731: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign65270_body44_e101732: f64 = (assign65270_body44_e101728 + assign65270_body44_e101731);
        (assign65270_body44_e101732, (((-locals.var_vgp__blk1525_dn0) + locals.var_phi_s0_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgp__blk1525_dn2) + locals.var_phi_s0_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (((-locals.var_vgp__blk1525_dn4) + locals.var_phi_s0_dn4) + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (((-locals.var_vgp__blk1525_dn5) + locals.var_phi_s0_dn5) + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgp__blk1525_dn6) + locals.var_phi_s0_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgp__blk1525_dn7) + locals.var_phi_s0_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgp__blk1525_dn8) + locals.var_phi_s0_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgp__blk1525_dn9) + locals.var_phi_s0_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (((-locals.var_vgp__blk1525_dn10) + locals.var_phi_s0_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (((-locals.var_vgp__blk1525_dn13) + locals.var_phi_s0_dn13) + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign65270_body44_e101734;
            locals.var_fs0_dn0 = assign65270_body44_e101734_d_n0;
            locals.var_fs0_dn2 = assign65270_body44_e101734_d_n2;
            locals.var_fs0_dn4 = assign65270_body44_e101734_d_n4;
            locals.var_fs0_dn5 = assign65270_body44_e101734_d_n5;
            locals.var_fs0_dn6 = assign65270_body44_e101734_d_n6;
            locals.var_fs0_dn7 = assign65270_body44_e101734_d_n7;
            locals.var_fs0_dn8 = assign65270_body44_e101734_d_n8;
            locals.var_fs0_dn9 = assign65270_body44_e101734_d_n9;
            locals.var_fs0_dn10 = assign65270_body44_e101734_d_n10;
            locals.var_fs0_dn13 = assign65270_body44_e101734_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign65270_body45_e101753, assign65270_body45_e101753_d_n0, assign65270_body45_e101753_d_n2, assign65270_body45_e101753_d_n4, assign65270_body45_e101753_d_n5, assign65270_body45_e101753_d_n6, assign65270_body45_e101753_d_n7, assign65270_body45_e101753_d_n8, assign65270_body45_e101753_d_n9, assign65270_body45_e101753_d_n10, assign65270_body45_e101753_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body45_e101750: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign65270_body45_e101751: f64 = (1.0 + assign65270_body45_e101750);
        (assign65270_body45_e101751, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign65270_body45_e101753;
            locals.var_fs0_dps0_dn0 = assign65270_body45_e101753_d_n0;
            locals.var_fs0_dps0_dn2 = assign65270_body45_e101753_d_n2;
            locals.var_fs0_dps0_dn4 = assign65270_body45_e101753_d_n4;
            locals.var_fs0_dps0_dn5 = assign65270_body45_e101753_d_n5;
            locals.var_fs0_dps0_dn6 = assign65270_body45_e101753_d_n6;
            locals.var_fs0_dps0_dn7 = assign65270_body45_e101753_d_n7;
            locals.var_fs0_dps0_dn8 = assign65270_body45_e101753_d_n8;
            locals.var_fs0_dps0_dn9 = assign65270_body45_e101753_d_n9;
            locals.var_fs0_dps0_dn10 = assign65270_body45_e101753_d_n10;
            locals.var_fs0_dps0_dn13 = assign65270_body45_e101753_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign65270_body46_e101756: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1563 = assign65270_body46_e101756;
            locals.var_guard1563_rv = 0.0;
            let (assign65270_body47_e101775,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65270_body47_e101773: f64 = (locals.var_lp_s0_max + 1.0);
        (assign65270_body47_e101773,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign65270_body47_e101775;
            locals.var_lp_s0_rv = 0.0;
            let (assign65270_body48_e101796, assign65270_body48_e101796_d_n0, assign65270_body48_e101796_d_n2, assign65270_body48_e101796_d_n4, assign65270_body48_e101796_d_n5, assign65270_body48_e101796_d_n6, assign65270_body48_e101796_d_n7, assign65270_body48_e101796_d_n8, assign65270_body48_e101796_d_n9, assign65270_body48_e101796_d_n10, assign65270_body48_e101796_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65270_body48_e101792: f64 = (-locals.var_fs0);
        let assign65270_body48_e101794: f64 = (assign65270_body48_e101792 / locals.var_fs0_dps0);
        (assign65270_body48_e101794, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign65270_body48_e101796;
            locals.var_dps0_dn0 = assign65270_body48_e101796_d_n0;
            locals.var_dps0_dn2 = assign65270_body48_e101796_d_n2;
            locals.var_dps0_dn4 = assign65270_body48_e101796_d_n4;
            locals.var_dps0_dn5 = assign65270_body48_e101796_d_n5;
            locals.var_dps0_dn6 = assign65270_body48_e101796_d_n6;
            locals.var_dps0_dn7 = assign65270_body48_e101796_d_n7;
            locals.var_dps0_dn8 = assign65270_body48_e101796_d_n8;
            locals.var_dps0_dn9 = assign65270_body48_e101796_d_n9;
            locals.var_dps0_dn10 = assign65270_body48_e101796_d_n10;
            locals.var_dps0_dn13 = assign65270_body48_e101796_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign65270_body49_e101827, assign65270_body49_e101827_d_n0, assign65270_body49_e101827_d_n2, assign65270_body49_e101827_d_n4, assign65270_body49_e101827_d_n5, assign65270_body49_e101827_d_n6, assign65270_body49_e101827_d_n7, assign65270_body49_e101827_d_n8, assign65270_body49_e101827_d_n9, assign65270_body49_e101827_d_n10, assign65270_body49_e101827_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65270_body49_e101814: f64 = (0.5 * 0.1);
        let assign65270_body49_e101818: f64 = (locals.var_phi_s0).abs();
        let (assign65270_body49_e101823, assign65270_body49_e101823_d_n0, assign65270_body49_e101823_d_n2, assign65270_body49_e101823_d_n4, assign65270_body49_e101823_d_n5, assign65270_body49_e101823_d_n6, assign65270_body49_e101823_d_n7, assign65270_body49_e101823_d_n8, assign65270_body49_e101823_d_n9, assign65270_body49_e101823_d_n10, assign65270_body49_e101823_d_n13,) = {
            if (1.0 >= assign65270_body49_e101818) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65270_body49_e101822: f64 = (locals.var_phi_s0).abs();
                (assign65270_body49_e101822, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn0 } else { (-locals.var_phi_s0_dn0) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn2 } else { (-locals.var_phi_s0_dn2) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn4 } else { (-locals.var_phi_s0_dn4) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn5 } else { (-locals.var_phi_s0_dn5) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn6 } else { (-locals.var_phi_s0_dn6) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn7 } else { (-locals.var_phi_s0_dn7) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn8 } else { (-locals.var_phi_s0_dn8) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn9 } else { (-locals.var_phi_s0_dn9) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn10 } else { (-locals.var_phi_s0_dn10) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn13 } else { (-locals.var_phi_s0_dn13) },)
            }
        };
        let assign65270_body49_e101824: f64 = (1.0 + assign65270_body49_e101823);
        let assign65270_body49_e101825: f64 = (assign65270_body49_e101814 * assign65270_body49_e101824);
        (assign65270_body49_e101825, (assign65270_body49_e101814 * assign65270_body49_e101823_d_n0), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n2), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n4), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n5), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n6), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n7), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n8), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n9), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n10), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign65270_body49_e101827;
            locals.var_dplim_dn0 = assign65270_body49_e101827_d_n0;
            locals.var_dplim_dn2 = assign65270_body49_e101827_d_n2;
            locals.var_dplim_dn4 = assign65270_body49_e101827_d_n4;
            locals.var_dplim_dn5 = assign65270_body49_e101827_d_n5;
            locals.var_dplim_dn6 = assign65270_body49_e101827_d_n6;
            locals.var_dplim_dn7 = assign65270_body49_e101827_d_n7;
            locals.var_dplim_dn8 = assign65270_body49_e101827_d_n8;
            locals.var_dplim_dn9 = assign65270_body49_e101827_d_n9;
            locals.var_dplim_dn10 = assign65270_body49_e101827_d_n10;
            locals.var_dplim_dn13 = assign65270_body49_e101827_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign65270_body50_e101829: f64 = (locals.var_dps0).abs();
            let assign65270_body50_e101831: f64 = if assign65270_body50_e101829 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1564 = assign65270_body50_e101831;
            locals.var_guard1564_rv = 0.0;
            let (assign65270_body51_e101859, assign65270_body51_e101859_d_n0, assign65270_body51_e101859_d_n2, assign65270_body51_e101859_d_n4, assign65270_body51_e101859_d_n5, assign65270_body51_e101859_d_n6, assign65270_body51_e101859_d_n7, assign65270_body51_e101859_d_n8, assign65270_body51_e101859_d_n9, assign65270_body51_e101859_d_n10, assign65270_body51_e101859_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 != 0.0)) {
        let (assign65270_body51_e101856,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign65270_body51_e101855: f64 = (-1.0);
                (assign65270_body51_e101855,)
            }
        };
        let assign65270_body51_e101857: f64 = (locals.var_dplim * assign65270_body51_e101856);
        (assign65270_body51_e101857, (locals.var_dplim_dn0 * assign65270_body51_e101856), (locals.var_dplim_dn2 * assign65270_body51_e101856), (locals.var_dplim_dn4 * assign65270_body51_e101856), (locals.var_dplim_dn5 * assign65270_body51_e101856), (locals.var_dplim_dn6 * assign65270_body51_e101856), (locals.var_dplim_dn7 * assign65270_body51_e101856), (locals.var_dplim_dn8 * assign65270_body51_e101856), (locals.var_dplim_dn9 * assign65270_body51_e101856), (locals.var_dplim_dn10 * assign65270_body51_e101856), (locals.var_dplim_dn13 * assign65270_body51_e101856),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign65270_body51_e101859;
            locals.var_dps0_dn0 = assign65270_body51_e101859_d_n0;
            locals.var_dps0_dn2 = assign65270_body51_e101859_d_n2;
            locals.var_dps0_dn4 = assign65270_body51_e101859_d_n4;
            locals.var_dps0_dn5 = assign65270_body51_e101859_d_n5;
            locals.var_dps0_dn6 = assign65270_body51_e101859_d_n6;
            locals.var_dps0_dn7 = assign65270_body51_e101859_d_n7;
            locals.var_dps0_dn8 = assign65270_body51_e101859_d_n8;
            locals.var_dps0_dn9 = assign65270_body51_e101859_d_n9;
            locals.var_dps0_dn10 = assign65270_body51_e101859_d_n10;
            locals.var_dps0_dn13 = assign65270_body51_e101859_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign65270_body52_e101879, assign65270_body52_e101879_d_n0, assign65270_body52_e101879_d_n2, assign65270_body52_e101879_d_n4, assign65270_body52_e101879_d_n5, assign65270_body52_e101879_d_n6, assign65270_body52_e101879_d_n7, assign65270_body52_e101879_d_n8, assign65270_body52_e101879_d_n9, assign65270_body52_e101879_d_n10, assign65270_body52_e101879_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65270_body52_e101877: f64 = (locals.var_phi_s0 + locals.var_dps0);
        (assign65270_body52_e101877, (locals.var_phi_s0_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_dn4 + locals.var_dps0_dn4), (locals.var_phi_s0_dn5 + locals.var_dps0_dn5), (locals.var_phi_s0_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_dn8 + locals.var_dps0_dn8), (locals.var_phi_s0_dn9 + locals.var_dps0_dn9), (locals.var_phi_s0_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn13,)
    }
};
            locals.var_phi_s0 = assign65270_body52_e101879;
            locals.var_phi_s0_dn0 = assign65270_body52_e101879_d_n0;
            locals.var_phi_s0_dn2 = assign65270_body52_e101879_d_n2;
            locals.var_phi_s0_dn4 = assign65270_body52_e101879_d_n4;
            locals.var_phi_s0_dn5 = assign65270_body52_e101879_d_n5;
            locals.var_phi_s0_dn6 = assign65270_body52_e101879_d_n6;
            locals.var_phi_s0_dn7 = assign65270_body52_e101879_d_n7;
            locals.var_phi_s0_dn8 = assign65270_body52_e101879_d_n8;
            locals.var_phi_s0_dn9 = assign65270_body52_e101879_d_n9;
            locals.var_phi_s0_dn10 = assign65270_body52_e101879_d_n10;
            locals.var_phi_s0_dn13 = assign65270_body52_e101879_d_n13;
            locals.var_phi_s0_rv = 0.0;
            let assign65270_body53_e101881: f64 = (locals.var_dps0).abs();
            let assign65270_body53_e101885: f64 = (locals.var_fs0).abs();
            let assign65270_body53_e101888: f64 = if ((assign65270_body53_e101881 <= 1e-12) && (assign65270_body53_e101885 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1565 = assign65270_body53_e101888;
            locals.var_guard1565_rv = 0.0;
            let (assign65270_body54_e101908,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1565 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign65270_body54_e101908;
            locals.var_flg_conv_rv = 0.0;
            let (assign65270_body55_e101925,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body55_e101923: f64 = (locals.var_lp_s0 + 1.0);
        (assign65270_body55_e101923,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign65270_body55_e101925;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_236(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65280_e101940, assign65280_e101940_d_n0, assign65280_e101940_d_n2, assign65280_e101940_d_n4, assign65280_e101940_d_n5, assign65280_e101940_d_n6, assign65280_e101940_d_n7, assign65280_e101940_d_n8, assign65280_e101940_d_n9, assign65280_e101940_d_n10, assign65280_e101940_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn13,)
    } else {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    }
};
        locals.var_ps0__blk1523 = assign65280_e101940;
        locals.var_ps0__blk1523_dn0 = assign65280_e101940_d_n0;
        locals.var_ps0__blk1523_dn2 = assign65280_e101940_d_n2;
        locals.var_ps0__blk1523_dn4 = assign65280_e101940_d_n4;
        locals.var_ps0__blk1523_dn5 = assign65280_e101940_d_n5;
        locals.var_ps0__blk1523_dn6 = assign65280_e101940_d_n6;
        locals.var_ps0__blk1523_dn7 = assign65280_e101940_d_n7;
        locals.var_ps0__blk1523_dn8 = assign65280_e101940_d_n8;
        locals.var_ps0__blk1523_dn9 = assign65280_e101940_d_n9;
        locals.var_ps0__blk1523_dn10 = assign65280_e101940_d_n10;
        locals.var_ps0__blk1523_dn13 = assign65280_e101940_d_n13;
        locals.var_ps0__blk1523_rv = 0.0;

        let (assign65290_e101952, assign65290_e101952_d_n0, assign65290_e101952_d_n2, assign65290_e101952_d_n4, assign65290_e101952_d_n5, assign65290_e101952_d_n6, assign65290_e101952_d_n7, assign65290_e101952_d_n8, assign65290_e101952_d_n9, assign65290_e101952_d_n10, assign65290_e101952_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65290_e101946: f64 = (-locals.var_beta);
        let assign65290_e101949: f64 = (locals.var_ps0__blk1523 - locals.var_dphi_vds);
        let assign65290_e101950: f64 = (assign65290_e101946 * assign65290_e101949);
        (assign65290_e101950, (((-locals.var_beta_dn0) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn0 - locals.var_dphi_vds_dn0))), (((-locals.var_beta_dn2) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn2 - locals.var_dphi_vds_dn2))), (((-locals.var_beta_dn4) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn4 - locals.var_dphi_vds_dn4))), (((-locals.var_beta_dn5) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn5 - locals.var_dphi_vds_dn5))), (((-locals.var_beta_dn6) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn6 - locals.var_dphi_vds_dn6))), (((-locals.var_beta_dn7) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn7 - locals.var_dphi_vds_dn7))), (((-locals.var_beta_dn8) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn8 - locals.var_dphi_vds_dn8))), (((-locals.var_beta_dn9) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn9 - locals.var_dphi_vds_dn9))), (((-locals.var_beta_dn10) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn10 - locals.var_dphi_vds_dn10))), (((-locals.var_beta_dn13) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn13 - locals.var_dphi_vds_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign65290_e101952;
        locals.var_t5_dn0 = assign65290_e101952_d_n0;
        locals.var_t5_dn2 = assign65290_e101952_d_n2;
        locals.var_t5_dn4 = assign65290_e101952_d_n4;
        locals.var_t5_dn5 = assign65290_e101952_d_n5;
        locals.var_t5_dn6 = assign65290_e101952_d_n6;
        locals.var_t5_dn7 = assign65290_e101952_d_n7;
        locals.var_t5_dn8 = assign65290_e101952_d_n8;
        locals.var_t5_dn9 = assign65290_e101952_d_n9;
        locals.var_t5_dn10 = assign65290_e101952_d_n10;
        locals.var_t5_dn13 = assign65290_e101952_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign65300_e101960, assign65300_e101960_d_n0, assign65300_e101960_d_n2, assign65300_e101960_d_n4, assign65300_e101960_d_n5, assign65300_e101960_d_n6, assign65300_e101960_d_n7, assign65300_e101960_d_n8, assign65300_e101960_d_n9, assign65300_e101960_d_n10, assign65300_e101960_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65300_e101958: f64 = (locals.var_t5).abs();
        (assign65300_e101958, if locals.var_t5 >= 0.0 { locals.var_t5_dn0 } else { (-locals.var_t5_dn0) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn2 } else { (-locals.var_t5_dn2) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn4 } else { (-locals.var_t5_dn4) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn5 } else { (-locals.var_t5_dn5) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn6 } else { (-locals.var_t5_dn6) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn7 } else { (-locals.var_t5_dn7) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn8 } else { (-locals.var_t5_dn8) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn9 } else { (-locals.var_t5_dn9) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn10 } else { (-locals.var_t5_dn10) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn13 } else { (-locals.var_t5_dn13) },)
    } else {
        (locals.var_t5abs, locals.var_t5abs_dn0, locals.var_t5abs_dn2, locals.var_t5abs_dn4, locals.var_t5abs_dn5, locals.var_t5abs_dn6, locals.var_t5abs_dn7, locals.var_t5abs_dn8, locals.var_t5abs_dn9, locals.var_t5abs_dn10, locals.var_t5abs_dn13,)
    }
};
        locals.var_t5abs = assign65300_e101960;
        locals.var_t5abs_dn0 = assign65300_e101960_d_n0;
        locals.var_t5abs_dn2 = assign65300_e101960_d_n2;
        locals.var_t5abs_dn4 = assign65300_e101960_d_n4;
        locals.var_t5abs_dn5 = assign65300_e101960_d_n5;
        locals.var_t5abs_dn6 = assign65300_e101960_d_n6;
        locals.var_t5abs_dn7 = assign65300_e101960_d_n7;
        locals.var_t5abs_dn8 = assign65300_e101960_d_n8;
        locals.var_t5abs_dn9 = assign65300_e101960_d_n9;
        locals.var_t5abs_dn10 = assign65300_e101960_d_n10;
        locals.var_t5abs_dn13 = assign65300_e101960_d_n13;
        locals.var_t5abs_rv = 0.0;

        let (assign65310_e101968, assign65310_e101968_d_n0, assign65310_e101968_d_n2, assign65310_e101968_d_n4, assign65310_e101968_d_n5, assign65310_e101968_d_n6, assign65310_e101968_d_n7, assign65310_e101968_d_n8, assign65310_e101968_d_n9, assign65310_e101968_d_n10, assign65310_e101968_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65310_e101966: f64 = (locals.var_t5).exp();
        (assign65310_e101966, (assign65310_e101966 * locals.var_t5_dn0), (assign65310_e101966 * locals.var_t5_dn2), (assign65310_e101966 * locals.var_t5_dn4), (assign65310_e101966 * locals.var_t5_dn5), (assign65310_e101966 * locals.var_t5_dn6), (assign65310_e101966 * locals.var_t5_dn7), (assign65310_e101966 * locals.var_t5_dn8), (assign65310_e101966 * locals.var_t5_dn9), (assign65310_e101966 * locals.var_t5_dn10), (assign65310_e101966 * locals.var_t5_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign65310_e101968;
        locals.var_t6_dn0 = assign65310_e101968_d_n0;
        locals.var_t6_dn2 = assign65310_e101968_d_n2;
        locals.var_t6_dn4 = assign65310_e101968_d_n4;
        locals.var_t6_dn5 = assign65310_e101968_d_n5;
        locals.var_t6_dn6 = assign65310_e101968_d_n6;
        locals.var_t6_dn7 = assign65310_e101968_d_n7;
        locals.var_t6_dn8 = assign65310_e101968_d_n8;
        locals.var_t6_dn9 = assign65310_e101968_d_n9;
        locals.var_t6_dn10 = assign65310_e101968_d_n10;
        locals.var_t6_dn13 = assign65310_e101968_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign65320_e101979, assign65320_e101979_d_n0, assign65320_e101979_d_n2, assign65320_e101979_d_n4, assign65320_e101979_d_n5, assign65320_e101979_d_n6, assign65320_e101979_d_n7, assign65320_e101979_d_n8, assign65320_e101979_d_n9, assign65320_e101979_d_n10, assign65320_e101979_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65320_e101975: f64 = (locals.var_t6 - 1.0);
        let assign65320_e101977: f64 = (assign65320_e101975 - locals.var_t5);
        (assign65320_e101977, (locals.var_t6_dn0 - locals.var_t5_dn0), (locals.var_t6_dn2 - locals.var_t5_dn2), (locals.var_t6_dn4 - locals.var_t5_dn4), (locals.var_t6_dn5 - locals.var_t5_dn5), (locals.var_t6_dn6 - locals.var_t5_dn6), (locals.var_t6_dn7 - locals.var_t5_dn7), (locals.var_t6_dn8 - locals.var_t5_dn8), (locals.var_t6_dn9 - locals.var_t5_dn9), (locals.var_t6_dn10 - locals.var_t5_dn10), (locals.var_t6_dn13 - locals.var_t5_dn13),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign65320_e101979;
        locals.var_t7_dn0 = assign65320_e101979_d_n0;
        locals.var_t7_dn2 = assign65320_e101979_d_n2;
        locals.var_t7_dn4 = assign65320_e101979_d_n4;
        locals.var_t7_dn5 = assign65320_e101979_d_n5;
        locals.var_t7_dn6 = assign65320_e101979_d_n6;
        locals.var_t7_dn7 = assign65320_e101979_d_n7;
        locals.var_t7_dn8 = assign65320_e101979_d_n8;
        locals.var_t7_dn9 = assign65320_e101979_d_n9;
        locals.var_t7_dn10 = assign65320_e101979_d_n10;
        locals.var_t7_dn13 = assign65320_e101979_d_n13;
        locals.var_t7_rv = 0.0;

        let assign65330_e101982: f64 = if locals.var_t5 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard1566 = assign65330_e101982;
        locals.var_guard1566_rv = 0.0;

        let (assign65340_e101995, assign65340_e101995_d_n0, assign65340_e101995_d_n2, assign65340_e101995_d_n4, assign65340_e101995_d_n5, assign65340_e101995_d_n6, assign65340_e101995_d_n7, assign65340_e101995_d_n8, assign65340_e101995_d_n9, assign65340_e101995_d_n10, assign65340_e101995_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1566 != 0.0)) {
        let assign65340_e101990: f64 = (-locals.var_cnst0);
        let assign65340_e101992: f64 = (locals.var_t7).sqrt();
        let assign65340_e101993: f64 = (assign65340_e101990 * assign65340_e101992);
        (assign65340_e101993, (((-locals.var_cnst0_dn0) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn0 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn2) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn2 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn4) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn4 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn5) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn5 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn6) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn6 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn7) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn7 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn8) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn8 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn9) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn9 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn10) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn10 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn13) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn13 / (2.0 * assign65340_e101992)))),)
    } else {
        (locals.var_qbu__blk1537, locals.var_qbu__blk1537_dn0, locals.var_qbu__blk1537_dn2, locals.var_qbu__blk1537_dn4, locals.var_qbu__blk1537_dn5, locals.var_qbu__blk1537_dn6, locals.var_qbu__blk1537_dn7, locals.var_qbu__blk1537_dn8, locals.var_qbu__blk1537_dn9, locals.var_qbu__blk1537_dn10, locals.var_qbu__blk1537_dn13,)
    }
};
        locals.var_qbu__blk1537 = assign65340_e101995;
        locals.var_qbu__blk1537_dn0 = assign65340_e101995_d_n0;
        locals.var_qbu__blk1537_dn2 = assign65340_e101995_d_n2;
        locals.var_qbu__blk1537_dn4 = assign65340_e101995_d_n4;
        locals.var_qbu__blk1537_dn5 = assign65340_e101995_d_n5;
        locals.var_qbu__blk1537_dn6 = assign65340_e101995_d_n6;
        locals.var_qbu__blk1537_dn7 = assign65340_e101995_d_n7;
        locals.var_qbu__blk1537_dn8 = assign65340_e101995_d_n8;
        locals.var_qbu__blk1537_dn9 = assign65340_e101995_d_n9;
        locals.var_qbu__blk1537_dn10 = assign65340_e101995_d_n10;
        locals.var_qbu__blk1537_dn13 = assign65340_e101995_d_n13;
        locals.var_qbu__blk1537_rv = 0.0;

        let assign65350_e101998: f64 = if locals.var_t5abs > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard1567 = assign65350_e101998;
        locals.var_guard1567_rv = 0.0;

        let (assign65360_e102013, assign65360_e102013_d_n0, assign65360_e102013_d_n2, assign65360_e102013_d_n4, assign65360_e102013_d_n5, assign65360_e102013_d_n6, assign65360_e102013_d_n7, assign65360_e102013_d_n8, assign65360_e102013_d_n9, assign65360_e102013_d_n10, assign65360_e102013_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1566 == 0.0)) && (locals.var_guard1567 != 0.0)) {
        let assign65360_e102010: f64 = (locals.var_t7).sqrt();
        let assign65360_e102011: f64 = (locals.var_cnst0 * assign65360_e102010);
        (assign65360_e102011, ((locals.var_cnst0_dn0 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn0 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn2 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn2 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn4 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn4 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn5 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn5 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn6 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn6 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn7 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn7 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn8 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn8 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn9 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn9 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn10 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn10 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn13 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn13 / (2.0 * assign65360_e102010)))),)
    } else {
        (locals.var_qbu__blk1537, locals.var_qbu__blk1537_dn0, locals.var_qbu__blk1537_dn2, locals.var_qbu__blk1537_dn4, locals.var_qbu__blk1537_dn5, locals.var_qbu__blk1537_dn6, locals.var_qbu__blk1537_dn7, locals.var_qbu__blk1537_dn8, locals.var_qbu__blk1537_dn9, locals.var_qbu__blk1537_dn10, locals.var_qbu__blk1537_dn13,)
    }
};
        locals.var_qbu__blk1537 = assign65360_e102013;
        locals.var_qbu__blk1537_dn0 = assign65360_e102013_d_n0;
        locals.var_qbu__blk1537_dn2 = assign65360_e102013_d_n2;
        locals.var_qbu__blk1537_dn4 = assign65360_e102013_d_n4;
        locals.var_qbu__blk1537_dn5 = assign65360_e102013_d_n5;
        locals.var_qbu__blk1537_dn6 = assign65360_e102013_d_n6;
        locals.var_qbu__blk1537_dn7 = assign65360_e102013_d_n7;
        locals.var_qbu__blk1537_dn8 = assign65360_e102013_d_n8;
        locals.var_qbu__blk1537_dn9 = assign65360_e102013_d_n9;
        locals.var_qbu__blk1537_dn10 = assign65360_e102013_d_n10;
        locals.var_qbu__blk1537_dn13 = assign65360_e102013_d_n13;
        locals.var_qbu__blk1537_rv = 0.0;

        let (assign65370_e102042, assign65370_e102042_d_n0, assign65370_e102042_d_n2, assign65370_e102042_d_n4, assign65370_e102042_d_n5, assign65370_e102042_d_n6, assign65370_e102042_d_n7, assign65370_e102042_d_n8, assign65370_e102042_d_n9, assign65370_e102042_d_n10, assign65370_e102042_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1566 == 0.0)) && (locals.var_guard1567 == 0.0)) {
        let assign65370_e102025: f64 = (-locals.var_t5);
        let assign65370_e102027: f64 = (assign65370_e102025 * 0.7071067811865475);
        let assign65370_e102031: f64 = (locals.var_t5abs * 0.3333333333333333);
        let assign65370_e102035: f64 = (0.25 * locals.var_t5abs);
        let assign65370_e102036: f64 = (1.0 + assign65370_e102035);
        let assign65370_e102037: f64 = (assign65370_e102031 * assign65370_e102036);
        let assign65370_e102038: f64 = (1.0 + assign65370_e102037);
        let assign65370_e102039: f64 = (assign65370_e102038).sqrt();
        let assign65370_e102040: f64 = (assign65370_e102027 * assign65370_e102039);
        (assign65370_e102040, ((((-locals.var_t5_dn0) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn0 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn0))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn2) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn2 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn2))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn4) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn4 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn4))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn5) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn5 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn5))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn6) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn6 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn6))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn7) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn7 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn7))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn8) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn8 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn8))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn9) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn9 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn9))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn10) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn10 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn10))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn13) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn13 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn13))) / (2.0 * assign65370_e102039)))),)
    } else {
        (locals.var_qbu__blk1537, locals.var_qbu__blk1537_dn0, locals.var_qbu__blk1537_dn2, locals.var_qbu__blk1537_dn4, locals.var_qbu__blk1537_dn5, locals.var_qbu__blk1537_dn6, locals.var_qbu__blk1537_dn7, locals.var_qbu__blk1537_dn8, locals.var_qbu__blk1537_dn9, locals.var_qbu__blk1537_dn10, locals.var_qbu__blk1537_dn13,)
    }
};
        locals.var_qbu__blk1537 = assign65370_e102042;
        locals.var_qbu__blk1537_dn0 = assign65370_e102042_d_n0;
        locals.var_qbu__blk1537_dn2 = assign65370_e102042_d_n2;
        locals.var_qbu__blk1537_dn4 = assign65370_e102042_d_n4;
        locals.var_qbu__blk1537_dn5 = assign65370_e102042_d_n5;
        locals.var_qbu__blk1537_dn6 = assign65370_e102042_d_n6;
        locals.var_qbu__blk1537_dn7 = assign65370_e102042_d_n7;
        locals.var_qbu__blk1537_dn8 = assign65370_e102042_d_n8;
        locals.var_qbu__blk1537_dn9 = assign65370_e102042_d_n9;
        locals.var_qbu__blk1537_dn10 = assign65370_e102042_d_n10;
        locals.var_qbu__blk1537_dn13 = assign65370_e102042_d_n13;
        locals.var_qbu__blk1537_rv = 0.0;

        let (assign65380_e102058, assign65380_e102058_d_n0, assign65380_e102058_d_n2, assign65380_e102058_d_n4, assign65380_e102058_d_n5, assign65380_e102058_d_n6, assign65380_e102058_d_n7, assign65380_e102058_d_n8, assign65380_e102058_d_n9, assign65380_e102058_d_n10, assign65380_e102058_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65380_e102049: f64 = (locals.var_qbu__blk1537 * locals.var_qbu__blk1537);
        let assign65380_e102052: f64 = (4.0 * 1e-6);
        let assign65380_e102054: f64 = (assign65380_e102052 * 1e-6);
        let assign65380_e102055: f64 = (assign65380_e102049 + assign65380_e102054);
        let assign65380_e102056: f64 = (assign65380_e102055).sqrt();
        (assign65380_e102056, (((locals.var_qbu__blk1537_dn0 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn0)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn2 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn2)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn4 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn4)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn5 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn5)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn6 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn6)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn7 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn7)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn8 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn8)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn9 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn9)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn10 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn10)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn13 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn13)) / (2.0 * assign65380_e102056)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign65380_e102058;
        locals.var_tmf1_dn0 = assign65380_e102058_d_n0;
        locals.var_tmf1_dn2 = assign65380_e102058_d_n2;
        locals.var_tmf1_dn4 = assign65380_e102058_d_n4;
        locals.var_tmf1_dn5 = assign65380_e102058_d_n5;
        locals.var_tmf1_dn6 = assign65380_e102058_d_n6;
        locals.var_tmf1_dn7 = assign65380_e102058_d_n7;
        locals.var_tmf1_dn8 = assign65380_e102058_d_n8;
        locals.var_tmf1_dn9 = assign65380_e102058_d_n9;
        locals.var_tmf1_dn10 = assign65380_e102058_d_n10;
        locals.var_tmf1_dn13 = assign65380_e102058_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign65390_e102069, assign65390_e102069_d_n0, assign65390_e102069_d_n2, assign65390_e102069_d_n4, assign65390_e102069_d_n5, assign65390_e102069_d_n6, assign65390_e102069_d_n7, assign65390_e102069_d_n8, assign65390_e102069_d_n9, assign65390_e102069_d_n10, assign65390_e102069_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65390_e102066: f64 = (locals.var_qbu__blk1537 + locals.var_tmf1);
        let assign65390_e102067: f64 = (0.5 * assign65390_e102066);
        (assign65390_e102067, (0.5 * (locals.var_qbu__blk1537_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_qbu__blk1537_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_qbu__blk1537_dn4 + locals.var_tmf1_dn4)), (0.5 * (locals.var_qbu__blk1537_dn5 + locals.var_tmf1_dn5)), (0.5 * (locals.var_qbu__blk1537_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_qbu__blk1537_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_qbu__blk1537_dn8 + locals.var_tmf1_dn8)), (0.5 * (locals.var_qbu__blk1537_dn9 + locals.var_tmf1_dn9)), (0.5 * (locals.var_qbu__blk1537_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_qbu__blk1537_dn13 + locals.var_tmf1_dn13)),)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn4, locals.var_wqbu_dn5, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn8, locals.var_wqbu_dn9, locals.var_wqbu_dn10, locals.var_wqbu_dn13,)
    }
};
        locals.var_wqbu = assign65390_e102069;
        locals.var_wqbu_dn0 = assign65390_e102069_d_n0;
        locals.var_wqbu_dn2 = assign65390_e102069_d_n2;
        locals.var_wqbu_dn4 = assign65390_e102069_d_n4;
        locals.var_wqbu_dn5 = assign65390_e102069_d_n5;
        locals.var_wqbu_dn6 = assign65390_e102069_d_n6;
        locals.var_wqbu_dn7 = assign65390_e102069_d_n7;
        locals.var_wqbu_dn8 = assign65390_e102069_d_n8;
        locals.var_wqbu_dn9 = assign65390_e102069_d_n9;
        locals.var_wqbu_dn10 = assign65390_e102069_d_n10;
        locals.var_wqbu_dn13 = assign65390_e102069_d_n13;
        locals.var_wqbu_rv = 0.0;

        let (assign65400_e102080, assign65400_e102080_d_n0, assign65400_e102080_d_n2, assign65400_e102080_d_n4, assign65400_e102080_d_n5, assign65400_e102080_d_n6, assign65400_e102080_d_n7, assign65400_e102080_d_n8, assign65400_e102080_d_n9, assign65400_e102080_d_n10, assign65400_e102080_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65400_e102077: f64 = (1.6021918e-19 * locals.var_nsub);
        let assign65400_e102078: f64 = (locals.var_wqbu / assign65400_e102077);
        (assign65400_e102078, (((locals.var_wqbu_dn0 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn0))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn2 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn2))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn4 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn4))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn5 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn5))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn6 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn6))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn7 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn7))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn8 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn8))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn9 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn9))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn10 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn10))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn13 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn13))) / (assign65400_e102077 * assign65400_e102077)),)
    } else {
        (locals.var_wdep__blk1533, locals.var_wdep__blk1533_dn0, locals.var_wdep__blk1533_dn2, locals.var_wdep__blk1533_dn4, locals.var_wdep__blk1533_dn5, locals.var_wdep__blk1533_dn6, locals.var_wdep__blk1533_dn7, locals.var_wdep__blk1533_dn8, locals.var_wdep__blk1533_dn9, locals.var_wdep__blk1533_dn10, locals.var_wdep__blk1533_dn13,)
    }
};
        locals.var_wdep__blk1533 = assign65400_e102080;
        locals.var_wdep__blk1533_dn0 = assign65400_e102080_d_n0;
        locals.var_wdep__blk1533_dn2 = assign65400_e102080_d_n2;
        locals.var_wdep__blk1533_dn4 = assign65400_e102080_d_n4;
        locals.var_wdep__blk1533_dn5 = assign65400_e102080_d_n5;
        locals.var_wdep__blk1533_dn6 = assign65400_e102080_d_n6;
        locals.var_wdep__blk1533_dn7 = assign65400_e102080_d_n7;
        locals.var_wdep__blk1533_dn8 = assign65400_e102080_d_n8;
        locals.var_wdep__blk1533_dn9 = assign65400_e102080_d_n9;
        locals.var_wdep__blk1533_dn10 = assign65400_e102080_d_n10;
        locals.var_wdep__blk1533_dn13 = assign65400_e102080_d_n13;
        locals.var_wdep__blk1533_rv = 0.0;

        let (assign65410_e102089, assign65410_e102089_d_n0, assign65410_e102089_d_n2, assign65410_e102089_d_n4, assign65410_e102089_d_n5, assign65410_e102089_d_n6, assign65410_e102089_d_n7, assign65410_e102089_d_n8, assign65410_e102089_d_n9, assign65410_e102089_d_n10, assign65410_e102089_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65410_e102087: f64 = (locals.var_wdep__blk1533 - p.p452);
        (assign65410_e102087, locals.var_wdep__blk1533_dn0, locals.var_wdep__blk1533_dn2, locals.var_wdep__blk1533_dn4, locals.var_wdep__blk1533_dn5, locals.var_wdep__blk1533_dn6, locals.var_wdep__blk1533_dn7, locals.var_wdep__blk1533_dn8, locals.var_wdep__blk1533_dn9, locals.var_wdep__blk1533_dn10, locals.var_wdep__blk1533_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65410_e102089;
        locals.var_t1_dn0 = assign65410_e102089_d_n0;
        locals.var_t1_dn2 = assign65410_e102089_d_n2;
        locals.var_t1_dn4 = assign65410_e102089_d_n4;
        locals.var_t1_dn5 = assign65410_e102089_d_n5;
        locals.var_t1_dn6 = assign65410_e102089_d_n6;
        locals.var_t1_dn7 = assign65410_e102089_d_n7;
        locals.var_t1_dn8 = assign65410_e102089_d_n8;
        locals.var_t1_dn9 = assign65410_e102089_d_n9;
        locals.var_t1_dn10 = assign65410_e102089_d_n10;
        locals.var_t1_dn13 = assign65410_e102089_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign65420_e102098, assign65420_e102098_d_n0, assign65420_e102098_d_n2, assign65420_e102098_d_n4, assign65420_e102098_d_n5, assign65420_e102098_d_n6, assign65420_e102098_d_n7, assign65420_e102098_d_n8, assign65420_e102098_d_n9, assign65420_e102098_d_n10, assign65420_e102098_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65420_e102096: f64 = (locals.var_wdep__blk1533 * 0.01);
        (assign65420_e102096, (locals.var_wdep__blk1533_dn0 * 0.01), (locals.var_wdep__blk1533_dn2 * 0.01), (locals.var_wdep__blk1533_dn4 * 0.01), (locals.var_wdep__blk1533_dn5 * 0.01), (locals.var_wdep__blk1533_dn6 * 0.01), (locals.var_wdep__blk1533_dn7 * 0.01), (locals.var_wdep__blk1533_dn8 * 0.01), (locals.var_wdep__blk1533_dn9 * 0.01), (locals.var_wdep__blk1533_dn10 * 0.01), (locals.var_wdep__blk1533_dn13 * 0.01),)
    } else {
        (locals.var_delta_1, locals.var_delta_1_dn0, locals.var_delta_1_dn2, locals.var_delta_1_dn4, locals.var_delta_1_dn5, locals.var_delta_1_dn6, locals.var_delta_1_dn7, locals.var_delta_1_dn8, locals.var_delta_1_dn9, locals.var_delta_1_dn10, locals.var_delta_1_dn13,)
    }
};
        locals.var_delta_1 = assign65420_e102098;
        locals.var_delta_1_dn0 = assign65420_e102098_d_n0;
        locals.var_delta_1_dn2 = assign65420_e102098_d_n2;
        locals.var_delta_1_dn4 = assign65420_e102098_d_n4;
        locals.var_delta_1_dn5 = assign65420_e102098_d_n5;
        locals.var_delta_1_dn6 = assign65420_e102098_d_n6;
        locals.var_delta_1_dn7 = assign65420_e102098_d_n7;
        locals.var_delta_1_dn8 = assign65420_e102098_d_n8;
        locals.var_delta_1_dn9 = assign65420_e102098_d_n9;
        locals.var_delta_1_dn10 = assign65420_e102098_d_n10;
        locals.var_delta_1_dn13 = assign65420_e102098_d_n13;
        locals.var_delta_1_rv = 0.0;

        let (assign65430_e102114, assign65430_e102114_d_n0, assign65430_e102114_d_n2, assign65430_e102114_d_n4, assign65430_e102114_d_n5, assign65430_e102114_d_n6, assign65430_e102114_d_n7, assign65430_e102114_d_n8, assign65430_e102114_d_n9, assign65430_e102114_d_n10, assign65430_e102114_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65430_e102105: f64 = (locals.var_t1 * locals.var_t1);
        let assign65430_e102108: f64 = (4.0 * locals.var_delta_1);
        let assign65430_e102110: f64 = (assign65430_e102108 * locals.var_delta_1);
        let assign65430_e102111: f64 = (assign65430_e102105 + assign65430_e102110);
        let assign65430_e102112: f64 = (assign65430_e102111).sqrt();
        (assign65430_e102112, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (((4.0 * locals.var_delta_1_dn0) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn0))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (((4.0 * locals.var_delta_1_dn2) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn2))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (((4.0 * locals.var_delta_1_dn4) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn4))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (((4.0 * locals.var_delta_1_dn5) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn5))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (((4.0 * locals.var_delta_1_dn6) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn6))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (((4.0 * locals.var_delta_1_dn7) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn7))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (((4.0 * locals.var_delta_1_dn8) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn8))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (((4.0 * locals.var_delta_1_dn9) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn9))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (((4.0 * locals.var_delta_1_dn10) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn10))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + (((4.0 * locals.var_delta_1_dn13) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn13))) / (2.0 * assign65430_e102112)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign65430_e102114;
        locals.var_tmf1_dn0 = assign65430_e102114_d_n0;
        locals.var_tmf1_dn2 = assign65430_e102114_d_n2;
        locals.var_tmf1_dn4 = assign65430_e102114_d_n4;
        locals.var_tmf1_dn5 = assign65430_e102114_d_n5;
        locals.var_tmf1_dn6 = assign65430_e102114_d_n6;
        locals.var_tmf1_dn7 = assign65430_e102114_d_n7;
        locals.var_tmf1_dn8 = assign65430_e102114_d_n8;
        locals.var_tmf1_dn9 = assign65430_e102114_d_n9;
        locals.var_tmf1_dn10 = assign65430_e102114_d_n10;
        locals.var_tmf1_dn13 = assign65430_e102114_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign65440_e102125, assign65440_e102125_d_n0, assign65440_e102125_d_n2, assign65440_e102125_d_n4, assign65440_e102125_d_n5, assign65440_e102125_d_n6, assign65440_e102125_d_n7, assign65440_e102125_d_n8, assign65440_e102125_d_n9, assign65440_e102125_d_n10, assign65440_e102125_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65440_e102122: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign65440_e102123: f64 = (0.5 * assign65440_e102122);
        (assign65440_e102123, (0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf1_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf1_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf1_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf1_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf1_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign65440_e102125;
        locals.var_t2_dn0 = assign65440_e102125_d_n0;
        locals.var_t2_dn2 = assign65440_e102125_d_n2;
        locals.var_t2_dn4 = assign65440_e102125_d_n4;
        locals.var_t2_dn5 = assign65440_e102125_d_n5;
        locals.var_t2_dn6 = assign65440_e102125_d_n6;
        locals.var_t2_dn7 = assign65440_e102125_d_n7;
        locals.var_t2_dn8 = assign65440_e102125_d_n8;
        locals.var_t2_dn9 = assign65440_e102125_d_n9;
        locals.var_t2_dn10 = assign65440_e102125_d_n10;
        locals.var_t2_dn13 = assign65440_e102125_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign65450_e102138, assign65450_e102138_d_n0, assign65450_e102138_d_n2, assign65450_e102138_d_n4, assign65450_e102138_d_n5, assign65450_e102138_d_n6, assign65450_e102138_d_n7, assign65450_e102138_d_n8, assign65450_e102138_d_n9, assign65450_e102138_d_n10, assign65450_e102138_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65450_e102132: f64 = (locals.var_t2 / locals.var_wdep__blk1533);
        let assign65450_e102134: f64 = (assign65450_e102132 * locals.var_t2);
        let assign65450_e102136: f64 = (assign65450_e102134 / locals.var_wdep__blk1533);
        (assign65450_e102136, ((((((((locals.var_t2_dn0 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn0)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn0)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn0)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn2 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn2)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn2)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn2)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn4 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn4)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn4)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn4)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn5 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn5)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn5)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn5)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn6 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn6)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn6)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn6)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn7 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn7)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn7)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn7)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn8 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn8)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn8)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn8)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn9 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn9)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn9)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn9)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn10 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn10)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn10)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn10)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn13 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn13)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn13)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn13)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)),)
    } else {
        (locals.var_wfactor, locals.var_wfactor_dn0, locals.var_wfactor_dn2, locals.var_wfactor_dn4, locals.var_wfactor_dn5, locals.var_wfactor_dn6, locals.var_wfactor_dn7, locals.var_wfactor_dn8, locals.var_wfactor_dn9, locals.var_wfactor_dn10, locals.var_wfactor_dn13,)
    }
};
        locals.var_wfactor = assign65450_e102138;
        locals.var_wfactor_dn0 = assign65450_e102138_d_n0;
        locals.var_wfactor_dn2 = assign65450_e102138_d_n2;
        locals.var_wfactor_dn4 = assign65450_e102138_d_n4;
        locals.var_wfactor_dn5 = assign65450_e102138_d_n5;
        locals.var_wfactor_dn6 = assign65450_e102138_d_n6;
        locals.var_wfactor_dn7 = assign65450_e102138_d_n7;
        locals.var_wfactor_dn8 = assign65450_e102138_d_n8;
        locals.var_wfactor_dn9 = assign65450_e102138_d_n9;
        locals.var_wfactor_dn10 = assign65450_e102138_d_n10;
        locals.var_wfactor_dn13 = assign65450_e102138_d_n13;
        locals.var_wfactor_rv = 0.0;

        let (assign65460_e102151, assign65460_e102151_d_n0, assign65460_e102151_d_n2, assign65460_e102151_d_n4, assign65460_e102151_d_n5, assign65460_e102151_d_n6, assign65460_e102151_d_n7, assign65460_e102151_d_n8, assign65460_e102151_d_n9, assign65460_e102151_d_n10, assign65460_e102151_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65460_e102145: f64 = (locals.var_ps0__blk1523 - locals.var_dphi_vds);
        let assign65460_e102147: f64 = (assign65460_e102145 * locals.var_wfactor);
        let assign65460_e102149: f64 = (assign65460_e102147 + locals.var_dphi_vds);
        (assign65460_e102149, ((((locals.var_ps0__blk1523_dn0 - locals.var_dphi_vds_dn0) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn0)) + locals.var_dphi_vds_dn0), ((((locals.var_ps0__blk1523_dn2 - locals.var_dphi_vds_dn2) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn2)) + locals.var_dphi_vds_dn2), ((((locals.var_ps0__blk1523_dn4 - locals.var_dphi_vds_dn4) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn4)) + locals.var_dphi_vds_dn4), ((((locals.var_ps0__blk1523_dn5 - locals.var_dphi_vds_dn5) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn5)) + locals.var_dphi_vds_dn5), ((((locals.var_ps0__blk1523_dn6 - locals.var_dphi_vds_dn6) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn6)) + locals.var_dphi_vds_dn6), ((((locals.var_ps0__blk1523_dn7 - locals.var_dphi_vds_dn7) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn7)) + locals.var_dphi_vds_dn7), ((((locals.var_ps0__blk1523_dn8 - locals.var_dphi_vds_dn8) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn8)) + locals.var_dphi_vds_dn8), ((((locals.var_ps0__blk1523_dn9 - locals.var_dphi_vds_dn9) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn9)) + locals.var_dphi_vds_dn9), ((((locals.var_ps0__blk1523_dn10 - locals.var_dphi_vds_dn10) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn10)) + locals.var_dphi_vds_dn10), ((((locals.var_ps0__blk1523_dn13 - locals.var_dphi_vds_dn13) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn13)) + locals.var_dphi_vds_dn13),)
    } else {
        (locals.var_phim, locals.var_phim_dn0, locals.var_phim_dn2, locals.var_phim_dn4, locals.var_phim_dn5, locals.var_phim_dn6, locals.var_phim_dn7, locals.var_phim_dn8, locals.var_phim_dn9, locals.var_phim_dn10, locals.var_phim_dn13,)
    }
};
        locals.var_phim = assign65460_e102151;
        locals.var_phim_dn0 = assign65460_e102151_d_n0;
        locals.var_phim_dn2 = assign65460_e102151_d_n2;
        locals.var_phim_dn4 = assign65460_e102151_d_n4;
        locals.var_phim_dn5 = assign65460_e102151_d_n5;
        locals.var_phim_dn6 = assign65460_e102151_d_n6;
        locals.var_phim_dn7 = assign65460_e102151_d_n7;
        locals.var_phim_dn8 = assign65460_e102151_d_n8;
        locals.var_phim_dn9 = assign65460_e102151_d_n9;
        locals.var_phim_dn10 = assign65460_e102151_d_n10;
        locals.var_phim_dn13 = assign65460_e102151_d_n13;
        locals.var_phim_rv = 0.0;

        let (assign65470_e102173, assign65470_e102173_d_n0, assign65470_e102173_d_n2, assign65470_e102173_d_n4, assign65470_e102173_d_n5, assign65470_e102173_d_n6, assign65470_e102173_d_n7, assign65470_e102173_d_n8, assign65470_e102173_d_n9, assign65470_e102173_d_n10, assign65470_e102173_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65470_e102160: f64 = (locals.var_vbipn - locals.var_vbscl__blk435);
        let assign65470_e102161: f64 = (locals.var_phim - assign65470_e102160);
        let assign65470_e102162: f64 = (locals.var_beta * assign65470_e102161);
        let assign65470_e102163: f64 = (assign65470_e102162).exp();
        let assign65470_e102166: f64 = (-locals.var_beta);
        let assign65470_e102168: f64 = (assign65470_e102166 * locals.var_vds);
        let assign65470_e102169: f64 = (assign65470_e102168).exp();
        let assign65470_e102170: f64 = (1.0 - assign65470_e102169);
        let assign65470_e102171: f64 = (assign65470_e102163 * assign65470_e102170);
        (assign65470_e102171, (((assign65470_e102163 * ((locals.var_beta_dn0 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn0 - (locals.var_vbipn_dn0 - locals.var_vbscl__blk435_dn0))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn0) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn0)))))), (((assign65470_e102163 * ((locals.var_beta_dn2 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn2 - (locals.var_vbipn_dn2 - locals.var_vbscl__blk435_dn2))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn2) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn2)))))), (((assign65470_e102163 * ((locals.var_beta_dn4 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn4 - (locals.var_vbipn_dn4 - locals.var_vbscl__blk435_dn4))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn4) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn4)))))), (((assign65470_e102163 * ((locals.var_beta_dn5 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn5 - (locals.var_vbipn_dn5 - locals.var_vbscl__blk435_dn5))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn5) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn5)))))), (((assign65470_e102163 * ((locals.var_beta_dn6 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn6 - (locals.var_vbipn_dn6 - locals.var_vbscl__blk435_dn6))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn6) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn6)))))), (((assign65470_e102163 * ((locals.var_beta_dn7 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn7 - (locals.var_vbipn_dn7 - locals.var_vbscl__blk435_dn7))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn7) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn7)))))), (((assign65470_e102163 * ((locals.var_beta_dn8 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn8 - (locals.var_vbipn_dn8 - locals.var_vbscl__blk435_dn8))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn8) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn8)))))), (((assign65470_e102163 * ((locals.var_beta_dn9 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn9 - (locals.var_vbipn_dn9 - locals.var_vbscl__blk435_dn9))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn9) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn9)))))), (((assign65470_e102163 * ((locals.var_beta_dn10 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn10 - (locals.var_vbipn_dn10 - locals.var_vbscl__blk435_dn10))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn10) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn10)))))), (((assign65470_e102163 * ((locals.var_beta_dn13 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn13 - (locals.var_vbipn_dn13 - locals.var_vbscl__blk435_dn13))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn13) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn13)))))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign65470_e102173;
        locals.var_ty_dn0 = assign65470_e102173_d_n0;
        locals.var_ty_dn2 = assign65470_e102173_d_n2;
        locals.var_ty_dn4 = assign65470_e102173_d_n4;
        locals.var_ty_dn5 = assign65470_e102173_d_n5;
        locals.var_ty_dn6 = assign65470_e102173_d_n6;
        locals.var_ty_dn7 = assign65470_e102173_d_n7;
        locals.var_ty_dn8 = assign65470_e102173_d_n8;
        locals.var_ty_dn9 = assign65470_e102173_d_n9;
        locals.var_ty_dn10 = assign65470_e102173_d_n10;
        locals.var_ty_dn13 = assign65470_e102173_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign65480_e102187,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65480_e102180: f64 = (2.0 * 1.6021918e-19);
        let assign65480_e102182: f64 = (assign65480_e102180 * locals.var_uc_njunc);
        let assign65480_e102184: f64 = (assign65480_e102182 * 1.034943e-10);
        let assign65480_e102185: f64 = (assign65480_e102184).sqrt();
        (assign65480_e102185,)
    } else {
        (locals.var_conpt00,)
    }
};
        locals.var_conpt00 = assign65480_e102187;
        locals.var_conpt00_rv = 0.0;

        let (assign65490_e102197, assign65490_e102197_d_n0, assign65490_e102197_d_n2, assign65490_e102197_d_n4, assign65490_e102197_d_n5, assign65490_e102197_d_n6, assign65490_e102197_d_n7, assign65490_e102197_d_n8, assign65490_e102197_d_n9, assign65490_e102197_d_n10, assign65490_e102197_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65490_e102194: f64 = (locals.var_beta_inv).sqrt();
        let assign65490_e102195: f64 = (locals.var_conpt00 * assign65490_e102194);
        (assign65490_e102195, (locals.var_conpt00 * (locals.var_beta_inv_dn0 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn2 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn4 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn5 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn6 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn7 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn8 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn9 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn10 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn13 / (2.0 * assign65490_e102194))),)
    } else {
        (locals.var_conpt0, locals.var_conpt0_dn0, locals.var_conpt0_dn2, locals.var_conpt0_dn4, locals.var_conpt0_dn5, locals.var_conpt0_dn6, locals.var_conpt0_dn7, locals.var_conpt0_dn8, locals.var_conpt0_dn9, locals.var_conpt0_dn10, locals.var_conpt0_dn13,)
    }
};
        locals.var_conpt0 = assign65490_e102197;
        locals.var_conpt0_dn0 = assign65490_e102197_d_n0;
        locals.var_conpt0_dn2 = assign65490_e102197_d_n2;
        locals.var_conpt0_dn4 = assign65490_e102197_d_n4;
        locals.var_conpt0_dn5 = assign65490_e102197_d_n5;
        locals.var_conpt0_dn6 = assign65490_e102197_d_n6;
        locals.var_conpt0_dn7 = assign65490_e102197_d_n7;
        locals.var_conpt0_dn8 = assign65490_e102197_d_n8;
        locals.var_conpt0_dn9 = assign65490_e102197_d_n9;
        locals.var_conpt0_dn10 = assign65490_e102197_d_n10;
        locals.var_conpt0_dn13 = assign65490_e102197_d_n13;
        locals.var_conpt0_rv = 0.0;

        let (assign65500_e102208, assign65500_e102208_d_n0, assign65500_e102208_d_n2, assign65500_e102208_d_n4, assign65500_e102208_d_n5, assign65500_e102208_d_n6, assign65500_e102208_d_n7, assign65500_e102208_d_n8, assign65500_e102208_d_n9, assign65500_e102208_d_n10, assign65500_e102208_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65500_e102205: f64 = (locals.var_phim - locals.var_dphi_vds);
        let assign65500_e102206: f64 = (locals.var_beta * assign65500_e102205);
        (assign65500_e102206, ((locals.var_beta_dn0 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn0 - locals.var_dphi_vds_dn0))), ((locals.var_beta_dn2 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn2 - locals.var_dphi_vds_dn2))), ((locals.var_beta_dn4 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn4 - locals.var_dphi_vds_dn4))), ((locals.var_beta_dn5 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn5 - locals.var_dphi_vds_dn5))), ((locals.var_beta_dn6 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn6 - locals.var_dphi_vds_dn6))), ((locals.var_beta_dn7 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn7 - locals.var_dphi_vds_dn7))), ((locals.var_beta_dn8 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn8 - locals.var_dphi_vds_dn8))), ((locals.var_beta_dn9 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn9 - locals.var_dphi_vds_dn9))), ((locals.var_beta_dn10 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_dphi_vds_dn10))), ((locals.var_beta_dn13 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn13 - locals.var_dphi_vds_dn13))),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn4, locals.var_t1w_dn5, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn8, locals.var_t1w_dn9, locals.var_t1w_dn10, locals.var_t1w_dn13,)
    }
};
        locals.var_t1w = assign65500_e102208;
        locals.var_t1w_dn0 = assign65500_e102208_d_n0;
        locals.var_t1w_dn2 = assign65500_e102208_d_n2;
        locals.var_t1w_dn4 = assign65500_e102208_d_n4;
        locals.var_t1w_dn5 = assign65500_e102208_d_n5;
        locals.var_t1w_dn6 = assign65500_e102208_d_n6;
        locals.var_t1w_dn7 = assign65500_e102208_d_n7;
        locals.var_t1w_dn8 = assign65500_e102208_d_n8;
        locals.var_t1w_dn9 = assign65500_e102208_d_n9;
        locals.var_t1w_dn10 = assign65500_e102208_d_n10;
        locals.var_t1w_dn13 = assign65500_e102208_d_n13;
        locals.var_t1w_rv = 0.0;

        let assign65510_e102213: f64 = (0.2 * locals.var_beta);
        let assign65510_e102214: f64 = assign65510_e102213;
        let assign65510_e102218: f64 = (0.2 * locals.var_beta);
        let assign65510_e102221: f64 = if ((locals.var_t1w < assign65510_e102214) && (assign65510_e102218 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1568 = assign65510_e102221;
        locals.var_guard1568_rv = 0.0;

        let (assign65520_e102236, assign65520_e102236_d_n0, assign65520_e102236_d_n2, assign65520_e102236_d_n4, assign65520_e102236_d_n5, assign65520_e102236_d_n6, assign65520_e102236_d_n7, assign65520_e102236_d_n8, assign65520_e102236_d_n9, assign65520_e102236_d_n10, assign65520_e102236_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65520_e102231: f64 = (0.2 * locals.var_beta);
        let assign65520_e102232: f64 = assign65520_e102231;
        let assign65520_e102234: f64 = (assign65520_e102232 - locals.var_t1w);
        (assign65520_e102234, ((0.2 * locals.var_beta_dn0) - locals.var_t1w_dn0), ((0.2 * locals.var_beta_dn2) - locals.var_t1w_dn2), ((0.2 * locals.var_beta_dn4) - locals.var_t1w_dn4), ((0.2 * locals.var_beta_dn5) - locals.var_t1w_dn5), ((0.2 * locals.var_beta_dn6) - locals.var_t1w_dn6), ((0.2 * locals.var_beta_dn7) - locals.var_t1w_dn7), ((0.2 * locals.var_beta_dn8) - locals.var_t1w_dn8), ((0.2 * locals.var_beta_dn9) - locals.var_t1w_dn9), ((0.2 * locals.var_beta_dn10) - locals.var_t1w_dn10), ((0.2 * locals.var_beta_dn13) - locals.var_t1w_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign65520_e102236;
        locals.var_tmf1_dn0 = assign65520_e102236_d_n0;
        locals.var_tmf1_dn2 = assign65520_e102236_d_n2;
        locals.var_tmf1_dn4 = assign65520_e102236_d_n4;
        locals.var_tmf1_dn5 = assign65520_e102236_d_n5;
        locals.var_tmf1_dn6 = assign65520_e102236_d_n6;
        locals.var_tmf1_dn7 = assign65520_e102236_d_n7;
        locals.var_tmf1_dn8 = assign65520_e102236_d_n8;
        locals.var_tmf1_dn9 = assign65520_e102236_d_n9;
        locals.var_tmf1_dn10 = assign65520_e102236_d_n10;
        locals.var_tmf1_dn13 = assign65520_e102236_d_n13;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_237(
        locals: &mut StampLocals,
    ) {
        let (assign65530_e102247, assign65530_e102247_d_n0, assign65530_e102247_d_n2, assign65530_e102247_d_n4, assign65530_e102247_d_n5, assign65530_e102247_d_n6, assign65530_e102247_d_n7, assign65530_e102247_d_n8, assign65530_e102247_d_n9, assign65530_e102247_d_n10, assign65530_e102247_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65530_e102245: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign65530_e102245, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign65530_e102247;
        locals.var_x2_dn0 = assign65530_e102247_d_n0;
        locals.var_x2_dn2 = assign65530_e102247_d_n2;
        locals.var_x2_dn4 = assign65530_e102247_d_n4;
        locals.var_x2_dn5 = assign65530_e102247_d_n5;
        locals.var_x2_dn6 = assign65530_e102247_d_n6;
        locals.var_x2_dn7 = assign65530_e102247_d_n7;
        locals.var_x2_dn8 = assign65530_e102247_d_n8;
        locals.var_x2_dn9 = assign65530_e102247_d_n9;
        locals.var_x2_dn10 = assign65530_e102247_d_n10;
        locals.var_x2_dn13 = assign65530_e102247_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign65540_e102262, assign65540_e102262_d_n0, assign65540_e102262_d_n2, assign65540_e102262_d_n4, assign65540_e102262_d_n5, assign65540_e102262_d_n6, assign65540_e102262_d_n7, assign65540_e102262_d_n8, assign65540_e102262_d_n9, assign65540_e102262_d_n10, assign65540_e102262_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65540_e102256: f64 = (0.2 * locals.var_beta);
        let assign65540_e102259: f64 = (0.2 * locals.var_beta);
        let assign65540_e102260: f64 = (assign65540_e102256 * assign65540_e102259);
        (assign65540_e102260, (((0.2 * locals.var_beta_dn0) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn0))), (((0.2 * locals.var_beta_dn2) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn2))), (((0.2 * locals.var_beta_dn4) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn4))), (((0.2 * locals.var_beta_dn5) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn5))), (((0.2 * locals.var_beta_dn6) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn6))), (((0.2 * locals.var_beta_dn7) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn7))), (((0.2 * locals.var_beta_dn8) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn8))), (((0.2 * locals.var_beta_dn9) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn9))), (((0.2 * locals.var_beta_dn10) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn10))), (((0.2 * locals.var_beta_dn13) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign65540_e102262;
        locals.var_xmax2_dn0 = assign65540_e102262_d_n0;
        locals.var_xmax2_dn2 = assign65540_e102262_d_n2;
        locals.var_xmax2_dn4 = assign65540_e102262_d_n4;
        locals.var_xmax2_dn5 = assign65540_e102262_d_n5;
        locals.var_xmax2_dn6 = assign65540_e102262_d_n6;
        locals.var_xmax2_dn7 = assign65540_e102262_d_n7;
        locals.var_xmax2_dn8 = assign65540_e102262_d_n8;
        locals.var_xmax2_dn9 = assign65540_e102262_d_n9;
        locals.var_xmax2_dn10 = assign65540_e102262_d_n10;
        locals.var_xmax2_dn13 = assign65540_e102262_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign65550_e102271, assign65550_e102271_d_n0, assign65550_e102271_d_n2, assign65550_e102271_d_n4, assign65550_e102271_d_n5, assign65550_e102271_d_n6, assign65550_e102271_d_n7, assign65550_e102271_d_n8, assign65550_e102271_d_n9, assign65550_e102271_d_n10, assign65550_e102271_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign65550_e102271;
        locals.var_xp_dn0 = assign65550_e102271_d_n0;
        locals.var_xp_dn2 = assign65550_e102271_d_n2;
        locals.var_xp_dn4 = assign65550_e102271_d_n4;
        locals.var_xp_dn5 = assign65550_e102271_d_n5;
        locals.var_xp_dn6 = assign65550_e102271_d_n6;
        locals.var_xp_dn7 = assign65550_e102271_d_n7;
        locals.var_xp_dn8 = assign65550_e102271_d_n8;
        locals.var_xp_dn9 = assign65550_e102271_d_n9;
        locals.var_xp_dn10 = assign65550_e102271_d_n10;
        locals.var_xp_dn13 = assign65550_e102271_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign65560_e102280, assign65560_e102280_d_n0, assign65560_e102280_d_n2, assign65560_e102280_d_n4, assign65560_e102280_d_n5, assign65560_e102280_d_n6, assign65560_e102280_d_n7, assign65560_e102280_d_n8, assign65560_e102280_d_n9, assign65560_e102280_d_n10, assign65560_e102280_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign65560_e102280;
        locals.var_xmp_dn0 = assign65560_e102280_d_n0;
        locals.var_xmp_dn2 = assign65560_e102280_d_n2;
        locals.var_xmp_dn4 = assign65560_e102280_d_n4;
        locals.var_xmp_dn5 = assign65560_e102280_d_n5;
        locals.var_xmp_dn6 = assign65560_e102280_d_n6;
        locals.var_xmp_dn7 = assign65560_e102280_d_n7;
        locals.var_xmp_dn8 = assign65560_e102280_d_n8;
        locals.var_xmp_dn9 = assign65560_e102280_d_n9;
        locals.var_xmp_dn10 = assign65560_e102280_d_n10;
        locals.var_xmp_dn13 = assign65560_e102280_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign65570_e102289,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign65570_e102289;
        locals.var_m0_rv = 0.0;

        let (assign65580_e102298,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65580_e102298;
        locals.var_mm_rv = 0.0;

        let (assign65590_e102307, assign65590_e102307_d_n0, assign65590_e102307_d_n2, assign65590_e102307_d_n4, assign65590_e102307_d_n5, assign65590_e102307_d_n6, assign65590_e102307_d_n7, assign65590_e102307_d_n8, assign65590_e102307_d_n9, assign65590_e102307_d_n10, assign65590_e102307_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign65590_e102307;
        locals.var_arg_dn0 = assign65590_e102307_d_n0;
        locals.var_arg_dn2 = assign65590_e102307_d_n2;
        locals.var_arg_dn4 = assign65590_e102307_d_n4;
        locals.var_arg_dn5 = assign65590_e102307_d_n5;
        locals.var_arg_dn6 = assign65590_e102307_d_n6;
        locals.var_arg_dn7 = assign65590_e102307_d_n7;
        locals.var_arg_dn8 = assign65590_e102307_d_n8;
        locals.var_arg_dn9 = assign65590_e102307_d_n9;
        locals.var_arg_dn10 = assign65590_e102307_d_n10;
        locals.var_arg_dn13 = assign65590_e102307_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign65600_e102316, assign65600_e102316_d_n0, assign65600_e102316_d_n2, assign65600_e102316_d_n4, assign65600_e102316_d_n5, assign65600_e102316_d_n6, assign65600_e102316_d_n7, assign65600_e102316_d_n8, assign65600_e102316_d_n9, assign65600_e102316_d_n10, assign65600_e102316_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign65600_e102316;
        locals.var_dnm_dn0 = assign65600_e102316_d_n0;
        locals.var_dnm_dn2 = assign65600_e102316_d_n2;
        locals.var_dnm_dn4 = assign65600_e102316_d_n4;
        locals.var_dnm_dn5 = assign65600_e102316_d_n5;
        locals.var_dnm_dn6 = assign65600_e102316_d_n6;
        locals.var_dnm_dn7 = assign65600_e102316_d_n7;
        locals.var_dnm_dn8 = assign65600_e102316_d_n8;
        locals.var_dnm_dn9 = assign65600_e102316_d_n9;
        locals.var_dnm_dn10 = assign65600_e102316_d_n10;
        locals.var_dnm_dn13 = assign65600_e102316_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign65610_e102327, assign65610_e102327_d_n0, assign65610_e102327_d_n2, assign65610_e102327_d_n4, assign65610_e102327_d_n5, assign65610_e102327_d_n6, assign65610_e102327_d_n7, assign65610_e102327_d_n8, assign65610_e102327_d_n9, assign65610_e102327_d_n10, assign65610_e102327_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65610_e102325: f64 = (locals.var_xp * locals.var_x2);
        (assign65610_e102325, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign65610_e102327;
        locals.var_xp_dn0 = assign65610_e102327_d_n0;
        locals.var_xp_dn2 = assign65610_e102327_d_n2;
        locals.var_xp_dn4 = assign65610_e102327_d_n4;
        locals.var_xp_dn5 = assign65610_e102327_d_n5;
        locals.var_xp_dn6 = assign65610_e102327_d_n6;
        locals.var_xp_dn7 = assign65610_e102327_d_n7;
        locals.var_xp_dn8 = assign65610_e102327_d_n8;
        locals.var_xp_dn9 = assign65610_e102327_d_n9;
        locals.var_xp_dn10 = assign65610_e102327_d_n10;
        locals.var_xp_dn13 = assign65610_e102327_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign65620_e102338, assign65620_e102338_d_n0, assign65620_e102338_d_n2, assign65620_e102338_d_n4, assign65620_e102338_d_n5, assign65620_e102338_d_n6, assign65620_e102338_d_n7, assign65620_e102338_d_n8, assign65620_e102338_d_n9, assign65620_e102338_d_n10, assign65620_e102338_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65620_e102336: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign65620_e102336, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign65620_e102338;
        locals.var_xmp_dn0 = assign65620_e102338_d_n0;
        locals.var_xmp_dn2 = assign65620_e102338_d_n2;
        locals.var_xmp_dn4 = assign65620_e102338_d_n4;
        locals.var_xmp_dn5 = assign65620_e102338_d_n5;
        locals.var_xmp_dn6 = assign65620_e102338_d_n6;
        locals.var_xmp_dn7 = assign65620_e102338_d_n7;
        locals.var_xmp_dn8 = assign65620_e102338_d_n8;
        locals.var_xmp_dn9 = assign65620_e102338_d_n9;
        locals.var_xmp_dn10 = assign65620_e102338_d_n10;
        locals.var_xmp_dn13 = assign65620_e102338_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign65630_e102349, assign65630_e102349_d_n0, assign65630_e102349_d_n2, assign65630_e102349_d_n4, assign65630_e102349_d_n5, assign65630_e102349_d_n6, assign65630_e102349_d_n7, assign65630_e102349_d_n8, assign65630_e102349_d_n9, assign65630_e102349_d_n10, assign65630_e102349_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65630_e102347: f64 = (locals.var_xp + locals.var_xmp);
        (assign65630_e102347, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign65630_e102349;
        locals.var_arg_dn0 = assign65630_e102349_d_n0;
        locals.var_arg_dn2 = assign65630_e102349_d_n2;
        locals.var_arg_dn4 = assign65630_e102349_d_n4;
        locals.var_arg_dn5 = assign65630_e102349_d_n5;
        locals.var_arg_dn6 = assign65630_e102349_d_n6;
        locals.var_arg_dn7 = assign65630_e102349_d_n7;
        locals.var_arg_dn8 = assign65630_e102349_d_n8;
        locals.var_arg_dn9 = assign65630_e102349_d_n9;
        locals.var_arg_dn10 = assign65630_e102349_d_n10;
        locals.var_arg_dn13 = assign65630_e102349_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign65640_e102358, assign65640_e102358_d_n0, assign65640_e102358_d_n2, assign65640_e102358_d_n4, assign65640_e102358_d_n5, assign65640_e102358_d_n6, assign65640_e102358_d_n7, assign65640_e102358_d_n8, assign65640_e102358_d_n9, assign65640_e102358_d_n10, assign65640_e102358_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign65640_e102358;
        locals.var_dnm_dn0 = assign65640_e102358_d_n0;
        locals.var_dnm_dn2 = assign65640_e102358_d_n2;
        locals.var_dnm_dn4 = assign65640_e102358_d_n4;
        locals.var_dnm_dn5 = assign65640_e102358_d_n5;
        locals.var_dnm_dn6 = assign65640_e102358_d_n6;
        locals.var_dnm_dn7 = assign65640_e102358_d_n7;
        locals.var_dnm_dn8 = assign65640_e102358_d_n8;
        locals.var_dnm_dn9 = assign65640_e102358_d_n9;
        locals.var_dnm_dn10 = assign65640_e102358_d_n10;
        locals.var_dnm_dn13 = assign65640_e102358_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign65650_e102373: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1569 = assign65650_e102373;
        locals.var_guard1569_rv = 0.0;

        let assign65660_e102376: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1570 = assign65660_e102376;
        locals.var_guard1570_rv = 0.0;

        let (assign65670_e102389,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65670_e102389;
        locals.var_mm_rv = 0.0;

        let assign65680_e102392: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1571 = assign65680_e102392;
        locals.var_guard1571_rv = 0.0;

        let (assign65690_e102408,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_guard1570 == 0.0)) && (locals.var_guard1571 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65690_e102408;
        locals.var_mm_rv = 0.0;

        let assign65700_e102411: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1572 = assign65700_e102411;
        locals.var_guard1572_rv = 0.0;

        let (assign65710_e102430,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_guard1570 == 0.0)) && (locals.var_guard1571 == 0.0)) && (locals.var_guard1572 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65710_e102430;
        locals.var_mm_rv = 0.0;

        let assign65720_e102433: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1573 = assign65720_e102433;
        locals.var_guard1573_rv = 0.0;

        let (assign65730_e102455,) = {
    if ((((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_guard1570 == 0.0)) && (locals.var_guard1571 == 0.0)) && (locals.var_guard1572 == 0.0)) && (locals.var_guard1573 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65730_e102455;
        locals.var_mm_rv = 0.0;

        let (assign65740_e102466,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign65740_e102466;
        locals.var_m0_rv = 0.0;

        let mut assign65750_loop_guard: usize = 0;
        while {
            let assign65750_cond_e102478: f64 = if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign65750_cond_e102478 != 0.0
        } {
            assign65750_loop_guard += 1;
            assert!(assign65750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign65750_body0_e102490, assign65750_body0_e102490_d_n0, assign65750_body0_e102490_d_n2, assign65750_body0_e102490_d_n4, assign65750_body0_e102490_d_n5, assign65750_body0_e102490_d_n6, assign65750_body0_e102490_d_n7, assign65750_body0_e102490_d_n8, assign65750_body0_e102490_d_n9, assign65750_body0_e102490_d_n10, assign65750_body0_e102490_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) {
        let assign65750_body0_e102488: f64 = (locals.var_dnm).sqrt();
        (assign65750_body0_e102488, (locals.var_dnm_dn0 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn2 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn4 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn5 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn6 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn7 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn8 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn9 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn10 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn13 / (2.0 * assign65750_body0_e102488)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign65750_body0_e102490;
            locals.var_dnm_dn0 = assign65750_body0_e102490_d_n0;
            locals.var_dnm_dn2 = assign65750_body0_e102490_d_n2;
            locals.var_dnm_dn4 = assign65750_body0_e102490_d_n4;
            locals.var_dnm_dn5 = assign65750_body0_e102490_d_n5;
            locals.var_dnm_dn6 = assign65750_body0_e102490_d_n6;
            locals.var_dnm_dn7 = assign65750_body0_e102490_d_n7;
            locals.var_dnm_dn8 = assign65750_body0_e102490_d_n8;
            locals.var_dnm_dn9 = assign65750_body0_e102490_d_n9;
            locals.var_dnm_dn10 = assign65750_body0_e102490_d_n10;
            locals.var_dnm_dn13 = assign65750_body0_e102490_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign65750_body1_e102503,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) {
        let assign65750_body1_e102501: f64 = (locals.var_m0 + 1.0);
        (assign65750_body1_e102501,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign65750_body1_e102503;
            locals.var_m0_rv = 0.0;
        }

        let (assign65760_e102526, assign65760_e102526_d_n0, assign65760_e102526_d_n2, assign65760_e102526_d_n4, assign65760_e102526_d_n5, assign65760_e102526_d_n6, assign65760_e102526_d_n7, assign65760_e102526_d_n8, assign65760_e102526_d_n9, assign65760_e102526_d_n10, assign65760_e102526_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 == 0.0)) {
        let (assign65760_e102524, assign65760_e102524_d_n0, assign65760_e102524_d_n2, assign65760_e102524_d_n4, assign65760_e102524_d_n5, assign65760_e102524_d_n6, assign65760_e102524_d_n7, assign65760_e102524_d_n8, assign65760_e102524_d_n9, assign65760_e102524_d_n10, assign65760_e102524_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65760_e102521: f64 = 2.0;
                let assign65760_e102522: f64 = (1.0 / assign65760_e102521);
                let assign65760_e102523: f64 = (locals.var_dnm).powf(assign65760_e102522);
                (assign65760_e102523, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn0)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn2)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn4)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn5)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn6)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn7)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn8)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn9)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn10)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn13)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign65760_e102524, assign65760_e102524_d_n0, assign65760_e102524_d_n2, assign65760_e102524_d_n4, assign65760_e102524_d_n5, assign65760_e102524_d_n6, assign65760_e102524_d_n7, assign65760_e102524_d_n8, assign65760_e102524_d_n9, assign65760_e102524_d_n10, assign65760_e102524_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign65760_e102526;
        locals.var_dnm_dn0 = assign65760_e102526_d_n0;
        locals.var_dnm_dn2 = assign65760_e102526_d_n2;
        locals.var_dnm_dn4 = assign65760_e102526_d_n4;
        locals.var_dnm_dn5 = assign65760_e102526_d_n5;
        locals.var_dnm_dn6 = assign65760_e102526_d_n6;
        locals.var_dnm_dn7 = assign65760_e102526_d_n7;
        locals.var_dnm_dn8 = assign65760_e102526_d_n8;
        locals.var_dnm_dn9 = assign65760_e102526_d_n9;
        locals.var_dnm_dn10 = assign65760_e102526_d_n10;
        locals.var_dnm_dn13 = assign65760_e102526_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign65770_e102537, assign65770_e102537_d_n0, assign65770_e102537_d_n2, assign65770_e102537_d_n4, assign65770_e102537_d_n5, assign65770_e102537_d_n6, assign65770_e102537_d_n7, assign65770_e102537_d_n8, assign65770_e102537_d_n9, assign65770_e102537_d_n10, assign65770_e102537_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65770_e102535: f64 = (1.0 / locals.var_dnm);
        (assign65770_e102535, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign65770_e102537;
        locals.var_dnm_dn0 = assign65770_e102537_d_n0;
        locals.var_dnm_dn2 = assign65770_e102537_d_n2;
        locals.var_dnm_dn4 = assign65770_e102537_d_n4;
        locals.var_dnm_dn5 = assign65770_e102537_d_n5;
        locals.var_dnm_dn6 = assign65770_e102537_d_n6;
        locals.var_dnm_dn7 = assign65770_e102537_d_n7;
        locals.var_dnm_dn8 = assign65770_e102537_d_n8;
        locals.var_dnm_dn9 = assign65770_e102537_d_n9;
        locals.var_dnm_dn10 = assign65770_e102537_d_n10;
        locals.var_dnm_dn13 = assign65770_e102537_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign65780_e102552, assign65780_e102552_d_n0, assign65780_e102552_d_n2, assign65780_e102552_d_n4, assign65780_e102552_d_n5, assign65780_e102552_d_n6, assign65780_e102552_d_n7, assign65780_e102552_d_n8, assign65780_e102552_d_n9, assign65780_e102552_d_n10, assign65780_e102552_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65780_e102547: f64 = (0.2 * locals.var_beta);
        let assign65780_e102548: f64 = (locals.var_tmf1 * assign65780_e102547);
        let assign65780_e102550: f64 = (assign65780_e102548 * locals.var_dnm);
        (assign65780_e102550, ((((locals.var_tmf1_dn0 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn0))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn2))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn4))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn5))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn6))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn7))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn8))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn9))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn10))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn13))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign65780_e102552;
        locals.var_tmf0_dn0 = assign65780_e102552_d_n0;
        locals.var_tmf0_dn2 = assign65780_e102552_d_n2;
        locals.var_tmf0_dn4 = assign65780_e102552_d_n4;
        locals.var_tmf0_dn5 = assign65780_e102552_d_n5;
        locals.var_tmf0_dn6 = assign65780_e102552_d_n6;
        locals.var_tmf0_dn7 = assign65780_e102552_d_n7;
        locals.var_tmf0_dn8 = assign65780_e102552_d_n8;
        locals.var_tmf0_dn9 = assign65780_e102552_d_n9;
        locals.var_tmf0_dn10 = assign65780_e102552_d_n10;
        locals.var_tmf0_dn13 = assign65780_e102552_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign65790_e102569, assign65790_e102569_d_n0, assign65790_e102569_d_n2, assign65790_e102569_d_n4, assign65790_e102569_d_n5, assign65790_e102569_d_n6, assign65790_e102569_d_n7, assign65790_e102569_d_n8, assign65790_e102569_d_n9, assign65790_e102569_d_n10, assign65790_e102569_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65790_e102561: f64 = (0.2 * locals.var_beta);
        let assign65790_e102563: f64 = (assign65790_e102561 * locals.var_xmp);
        let assign65790_e102565: f64 = (assign65790_e102563 * locals.var_dnm);
        let assign65790_e102567: f64 = (assign65790_e102565 / locals.var_arg);
        (assign65790_e102567, ((((((((0.2 * locals.var_beta_dn0) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn0)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn2) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn2)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn4) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn4)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn5) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn5)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn6) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn6)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn7) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn7)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn8) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn8)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn9) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn9)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn10) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn10)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn13) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn13)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign65790_e102569;
        locals.var_t0_dn0 = assign65790_e102569_d_n0;
        locals.var_t0_dn2 = assign65790_e102569_d_n2;
        locals.var_t0_dn4 = assign65790_e102569_d_n4;
        locals.var_t0_dn5 = assign65790_e102569_d_n5;
        locals.var_t0_dn6 = assign65790_e102569_d_n6;
        locals.var_t0_dn7 = assign65790_e102569_d_n7;
        locals.var_t0_dn8 = assign65790_e102569_d_n8;
        locals.var_t0_dn9 = assign65790_e102569_d_n9;
        locals.var_t0_dn10 = assign65790_e102569_d_n10;
        locals.var_t0_dn13 = assign65790_e102569_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign65800_e102584, assign65800_e102584_d_n0, assign65800_e102584_d_n2, assign65800_e102584_d_n4, assign65800_e102584_d_n5, assign65800_e102584_d_n6, assign65800_e102584_d_n7, assign65800_e102584_d_n8, assign65800_e102584_d_n9, assign65800_e102584_d_n10, assign65800_e102584_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65800_e102579: f64 = (0.2 * locals.var_beta);
        let assign65800_e102580: f64 = assign65800_e102579;
        let assign65800_e102582: f64 = (assign65800_e102580 - locals.var_tmf0);
        (assign65800_e102582, ((0.2 * locals.var_beta_dn0) - locals.var_tmf0_dn0), ((0.2 * locals.var_beta_dn2) - locals.var_tmf0_dn2), ((0.2 * locals.var_beta_dn4) - locals.var_tmf0_dn4), ((0.2 * locals.var_beta_dn5) - locals.var_tmf0_dn5), ((0.2 * locals.var_beta_dn6) - locals.var_tmf0_dn6), ((0.2 * locals.var_beta_dn7) - locals.var_tmf0_dn7), ((0.2 * locals.var_beta_dn8) - locals.var_tmf0_dn8), ((0.2 * locals.var_beta_dn9) - locals.var_tmf0_dn9), ((0.2 * locals.var_beta_dn10) - locals.var_tmf0_dn10), ((0.2 * locals.var_beta_dn13) - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65800_e102584;
        locals.var_t1_dn0 = assign65800_e102584_d_n0;
        locals.var_t1_dn2 = assign65800_e102584_d_n2;
        locals.var_t1_dn4 = assign65800_e102584_d_n4;
        locals.var_t1_dn5 = assign65800_e102584_d_n5;
        locals.var_t1_dn6 = assign65800_e102584_d_n6;
        locals.var_t1_dn7 = assign65800_e102584_d_n7;
        locals.var_t1_dn8 = assign65800_e102584_d_n8;
        locals.var_t1_dn9 = assign65800_e102584_d_n9;
        locals.var_t1_dn10 = assign65800_e102584_d_n10;
        locals.var_t1_dn13 = assign65800_e102584_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign65810_e102593, assign65810_e102593_d_n0, assign65810_e102593_d_n2, assign65810_e102593_d_n4, assign65810_e102593_d_n5, assign65810_e102593_d_n6, assign65810_e102593_d_n7, assign65810_e102593_d_n8, assign65810_e102593_d_n9, assign65810_e102593_d_n10, assign65810_e102593_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign65810_e102593;
        locals.var_t0_dn0 = assign65810_e102593_d_n0;
        locals.var_t0_dn2 = assign65810_e102593_d_n2;
        locals.var_t0_dn4 = assign65810_e102593_d_n4;
        locals.var_t0_dn5 = assign65810_e102593_d_n5;
        locals.var_t0_dn6 = assign65810_e102593_d_n6;
        locals.var_t0_dn7 = assign65810_e102593_d_n7;
        locals.var_t0_dn8 = assign65810_e102593_d_n8;
        locals.var_t0_dn9 = assign65810_e102593_d_n9;
        locals.var_t0_dn10 = assign65810_e102593_d_n10;
        locals.var_t0_dn13 = assign65810_e102593_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign65820_e102603, assign65820_e102603_d_n0, assign65820_e102603_d_n2, assign65820_e102603_d_n4, assign65820_e102603_d_n5, assign65820_e102603_d_n6, assign65820_e102603_d_n7, assign65820_e102603_d_n8, assign65820_e102603_d_n9, assign65820_e102603_d_n10, assign65820_e102603_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 == 0.0)) {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn4, locals.var_t1w_dn5, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn8, locals.var_t1w_dn9, locals.var_t1w_dn10, locals.var_t1w_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65820_e102603;
        locals.var_t1_dn0 = assign65820_e102603_d_n0;
        locals.var_t1_dn2 = assign65820_e102603_d_n2;
        locals.var_t1_dn4 = assign65820_e102603_d_n4;
        locals.var_t1_dn5 = assign65820_e102603_d_n5;
        locals.var_t1_dn6 = assign65820_e102603_d_n6;
        locals.var_t1_dn7 = assign65820_e102603_d_n7;
        locals.var_t1_dn8 = assign65820_e102603_d_n8;
        locals.var_t1_dn9 = assign65820_e102603_d_n9;
        locals.var_t1_dn10 = assign65820_e102603_d_n10;
        locals.var_t1_dn13 = assign65820_e102603_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_238(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65830_e102613, assign65830_e102613_d_n0, assign65830_e102613_d_n2, assign65830_e102613_d_n4, assign65830_e102613_d_n5, assign65830_e102613_d_n6, assign65830_e102613_d_n7, assign65830_e102613_d_n8, assign65830_e102613_d_n9, assign65830_e102613_d_n10, assign65830_e102613_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign65830_e102613;
        locals.var_t0_dn0 = assign65830_e102613_d_n0;
        locals.var_t0_dn2 = assign65830_e102613_d_n2;
        locals.var_t0_dn4 = assign65830_e102613_d_n4;
        locals.var_t0_dn5 = assign65830_e102613_d_n5;
        locals.var_t0_dn6 = assign65830_e102613_d_n6;
        locals.var_t0_dn7 = assign65830_e102613_d_n7;
        locals.var_t0_dn8 = assign65830_e102613_d_n8;
        locals.var_t0_dn9 = assign65830_e102613_d_n9;
        locals.var_t0_dn10 = assign65830_e102613_d_n10;
        locals.var_t0_dn13 = assign65830_e102613_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign65840_e102625, assign65840_e102625_d_n0, assign65840_e102625_d_n2, assign65840_e102625_d_n4, assign65840_e102625_d_n5, assign65840_e102625_d_n6, assign65840_e102625_d_n7, assign65840_e102625_d_n8, assign65840_e102625_d_n9, assign65840_e102625_d_n10, assign65840_e102625_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65840_e102621: f64 = (10.0 * 2.220446049250313e-16);
        let assign65840_e102622: f64 = (locals.var_t1 + assign65840_e102621);
        let assign65840_e102623: f64 = (assign65840_e102622).sqrt();
        (assign65840_e102623, (locals.var_t1_dn0 / (2.0 * assign65840_e102623)), (locals.var_t1_dn2 / (2.0 * assign65840_e102623)), (locals.var_t1_dn4 / (2.0 * assign65840_e102623)), (locals.var_t1_dn5 / (2.0 * assign65840_e102623)), (locals.var_t1_dn6 / (2.0 * assign65840_e102623)), (locals.var_t1_dn7 / (2.0 * assign65840_e102623)), (locals.var_t1_dn8 / (2.0 * assign65840_e102623)), (locals.var_t1_dn9 / (2.0 * assign65840_e102623)), (locals.var_t1_dn10 / (2.0 * assign65840_e102623)), (locals.var_t1_dn13 / (2.0 * assign65840_e102623)),)
    } else {
        (locals.var_sq1npt, locals.var_sq1npt_dn0, locals.var_sq1npt_dn2, locals.var_sq1npt_dn4, locals.var_sq1npt_dn5, locals.var_sq1npt_dn6, locals.var_sq1npt_dn7, locals.var_sq1npt_dn8, locals.var_sq1npt_dn9, locals.var_sq1npt_dn10, locals.var_sq1npt_dn13,)
    }
};
        locals.var_sq1npt = assign65840_e102625;
        locals.var_sq1npt_dn0 = assign65840_e102625_d_n0;
        locals.var_sq1npt_dn2 = assign65840_e102625_d_n2;
        locals.var_sq1npt_dn4 = assign65840_e102625_d_n4;
        locals.var_sq1npt_dn5 = assign65840_e102625_d_n5;
        locals.var_sq1npt_dn6 = assign65840_e102625_d_n6;
        locals.var_sq1npt_dn7 = assign65840_e102625_d_n7;
        locals.var_sq1npt_dn8 = assign65840_e102625_d_n8;
        locals.var_sq1npt_dn9 = assign65840_e102625_d_n9;
        locals.var_sq1npt_dn10 = assign65840_e102625_d_n10;
        locals.var_sq1npt_dn13 = assign65840_e102625_d_n13;
        locals.var_sq1npt_rv = 0.0;

        let (assign65850_e102634, assign65850_e102634_d_n0, assign65850_e102634_d_n2, assign65850_e102634_d_n4, assign65850_e102634_d_n5, assign65850_e102634_d_n6, assign65850_e102634_d_n7, assign65850_e102634_d_n8, assign65850_e102634_d_n9, assign65850_e102634_d_n10, assign65850_e102634_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65850_e102632: f64 = (locals.var_conpt0 * locals.var_sq1npt);
        (assign65850_e102632, ((locals.var_conpt0_dn0 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn0)), ((locals.var_conpt0_dn2 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn2)), ((locals.var_conpt0_dn4 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn4)), ((locals.var_conpt0_dn5 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn5)), ((locals.var_conpt0_dn6 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn6)), ((locals.var_conpt0_dn7 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn7)), ((locals.var_conpt0_dn8 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn8)), ((locals.var_conpt0_dn9 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn9)), ((locals.var_conpt0_dn10 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn10)), ((locals.var_conpt0_dn13 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn13)),)
    } else {
        (locals.var_qn0npt, locals.var_qn0npt_dn0, locals.var_qn0npt_dn2, locals.var_qn0npt_dn4, locals.var_qn0npt_dn5, locals.var_qn0npt_dn6, locals.var_qn0npt_dn7, locals.var_qn0npt_dn8, locals.var_qn0npt_dn9, locals.var_qn0npt_dn10, locals.var_qn0npt_dn13,)
    }
};
        locals.var_qn0npt = assign65850_e102634;
        locals.var_qn0npt_dn0 = assign65850_e102634_d_n0;
        locals.var_qn0npt_dn2 = assign65850_e102634_d_n2;
        locals.var_qn0npt_dn4 = assign65850_e102634_d_n4;
        locals.var_qn0npt_dn5 = assign65850_e102634_d_n5;
        locals.var_qn0npt_dn6 = assign65850_e102634_d_n6;
        locals.var_qn0npt_dn7 = assign65850_e102634_d_n7;
        locals.var_qn0npt_dn8 = assign65850_e102634_d_n8;
        locals.var_qn0npt_dn9 = assign65850_e102634_d_n9;
        locals.var_qn0npt_dn10 = assign65850_e102634_d_n10;
        locals.var_qn0npt_dn13 = assign65850_e102634_d_n13;
        locals.var_qn0npt_rv = 0.0;

        let (assign65860_e102649, assign65860_e102649_d_n0, assign65860_e102649_d_n2, assign65860_e102649_d_n4, assign65860_e102649_d_n5, assign65860_e102649_d_n6, assign65860_e102649_d_n7, assign65860_e102649_d_n8, assign65860_e102649_d_n9, assign65860_e102649_d_n10, assign65860_e102649_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65860_e102641: f64 = (2.0 * locals.var_beta_inv);
        let assign65860_e102643: f64 = (assign65860_e102641 / locals.var_leff);
        let assign65860_e102645: f64 = (assign65860_e102643 * locals.var_qn0npt);
        let assign65860_e102647: f64 = (assign65860_e102645 * p.p454);
        (assign65860_e102647, (((((2.0 * locals.var_beta_inv_dn0) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn0)) * p.p454), (((((2.0 * locals.var_beta_inv_dn2) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn2)) * p.p454), (((((2.0 * locals.var_beta_inv_dn4) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn4)) * p.p454), (((((2.0 * locals.var_beta_inv_dn5) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn5)) * p.p454), (((((2.0 * locals.var_beta_inv_dn6) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn6)) * p.p454), (((((2.0 * locals.var_beta_inv_dn7) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn7)) * p.p454), (((((2.0 * locals.var_beta_inv_dn8) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn8)) * p.p454), (((((2.0 * locals.var_beta_inv_dn9) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn9)) * p.p454), (((((2.0 * locals.var_beta_inv_dn10) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn10)) * p.p454), (((((2.0 * locals.var_beta_inv_dn13) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn13)) * p.p454),)
    } else {
        (locals.var_wk_jnpt_a, locals.var_wk_jnpt_a_dn0, locals.var_wk_jnpt_a_dn2, locals.var_wk_jnpt_a_dn4, locals.var_wk_jnpt_a_dn5, locals.var_wk_jnpt_a_dn6, locals.var_wk_jnpt_a_dn7, locals.var_wk_jnpt_a_dn8, locals.var_wk_jnpt_a_dn9, locals.var_wk_jnpt_a_dn10, locals.var_wk_jnpt_a_dn13,)
    }
};
        locals.var_wk_jnpt_a = assign65860_e102649;
        locals.var_wk_jnpt_a_dn0 = assign65860_e102649_d_n0;
        locals.var_wk_jnpt_a_dn2 = assign65860_e102649_d_n2;
        locals.var_wk_jnpt_a_dn4 = assign65860_e102649_d_n4;
        locals.var_wk_jnpt_a_dn5 = assign65860_e102649_d_n5;
        locals.var_wk_jnpt_a_dn6 = assign65860_e102649_d_n6;
        locals.var_wk_jnpt_a_dn7 = assign65860_e102649_d_n7;
        locals.var_wk_jnpt_a_dn8 = assign65860_e102649_d_n8;
        locals.var_wk_jnpt_a_dn9 = assign65860_e102649_d_n9;
        locals.var_wk_jnpt_a_dn10 = assign65860_e102649_d_n10;
        locals.var_wk_jnpt_a_dn13 = assign65860_e102649_d_n13;
        locals.var_wk_jnpt_a_rv = 0.0;

        let (assign65870_e102660, assign65870_e102660_d_n0, assign65870_e102660_d_n2, assign65870_e102660_d_n4, assign65870_e102660_d_n5, assign65870_e102660_d_n6, assign65870_e102660_d_n7, assign65870_e102660_d_n8, assign65870_e102660_d_n9, assign65870_e102660_d_n10, assign65870_e102660_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65870_e102656: f64 = (locals.var_wk_jnpt_a * locals.var_weff_nf);
        let assign65870_e102658: f64 = (assign65870_e102656 * locals.var_ty);
        (assign65870_e102658, (((locals.var_wk_jnpt_a_dn0 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn0)), (((locals.var_wk_jnpt_a_dn2 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn2)), (((locals.var_wk_jnpt_a_dn4 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn4)), (((locals.var_wk_jnpt_a_dn5 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn5)), (((locals.var_wk_jnpt_a_dn6 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn6)), (((locals.var_wk_jnpt_a_dn7 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn7)), (((locals.var_wk_jnpt_a_dn8 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn8)), (((locals.var_wk_jnpt_a_dn9 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn9)), (((locals.var_wk_jnpt_a_dn10 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn10)), (((locals.var_wk_jnpt_a_dn13 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn13)),)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn4, locals.var_idspt1_dn5, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn8, locals.var_idspt1_dn9, locals.var_idspt1_dn10, locals.var_idspt1_dn13,)
    }
};
        locals.var_idspt1 = assign65870_e102660;
        locals.var_idspt1_dn0 = assign65870_e102660_d_n0;
        locals.var_idspt1_dn2 = assign65870_e102660_d_n2;
        locals.var_idspt1_dn4 = assign65870_e102660_d_n4;
        locals.var_idspt1_dn5 = assign65870_e102660_d_n5;
        locals.var_idspt1_dn6 = assign65870_e102660_d_n6;
        locals.var_idspt1_dn7 = assign65870_e102660_d_n7;
        locals.var_idspt1_dn8 = assign65870_e102660_d_n8;
        locals.var_idspt1_dn9 = assign65870_e102660_d_n9;
        locals.var_idspt1_dn10 = assign65870_e102660_d_n10;
        locals.var_idspt1_dn13 = assign65870_e102660_d_n13;
        locals.var_idspt1_rv = 0.0;

        let (assign65880_e102669, assign65880_e102669_d_n0, assign65880_e102669_d_n2, assign65880_e102669_d_n4, assign65880_e102669_d_n5, assign65880_e102669_d_n6, assign65880_e102669_d_n7, assign65880_e102669_d_n8, assign65880_e102669_d_n9, assign65880_e102669_d_n10, assign65880_e102669_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65880_e102667: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign65880_e102667, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn4 + locals.var_idspt1_dn4), (locals.var_idsorg_dn5 + locals.var_idspt1_dn5), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn8 + locals.var_idspt1_dn8), (locals.var_idsorg_dn9 + locals.var_idspt1_dn9), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn13 + locals.var_idspt1_dn13),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign65880_e102669;
        locals.var_ids_dn0 = assign65880_e102669_d_n0;
        locals.var_ids_dn2 = assign65880_e102669_d_n2;
        locals.var_ids_dn4 = assign65880_e102669_d_n4;
        locals.var_ids_dn5 = assign65880_e102669_d_n5;
        locals.var_ids_dn6 = assign65880_e102669_d_n6;
        locals.var_ids_dn7 = assign65880_e102669_d_n7;
        locals.var_ids_dn8 = assign65880_e102669_d_n8;
        locals.var_ids_dn9 = assign65880_e102669_d_n9;
        locals.var_ids_dn10 = assign65880_e102669_d_n10;
        locals.var_ids_dn13 = assign65880_e102669_d_n13;
        locals.var_ids_rv = 0.0;

        let (assign65890_e102676, assign65890_e102676_d_n0, assign65890_e102676_d_n2, assign65890_e102676_d_n4, assign65890_e102676_d_n5, assign65890_e102676_d_n6, assign65890_e102676_d_n7, assign65890_e102676_d_n8, assign65890_e102676_d_n9, assign65890_e102676_d_n10, assign65890_e102676_d_n13,) = {
    if (locals.var_guard443 == 0.0) {
        let assign65890_e102674: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign65890_e102674, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn4 + locals.var_idspt1_dn4), (locals.var_idsorg_dn5 + locals.var_idspt1_dn5), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn8 + locals.var_idspt1_dn8), (locals.var_idsorg_dn9 + locals.var_idspt1_dn9), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn13 + locals.var_idspt1_dn13),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign65890_e102676;
        locals.var_ids_dn0 = assign65890_e102676_d_n0;
        locals.var_ids_dn2 = assign65890_e102676_d_n2;
        locals.var_ids_dn4 = assign65890_e102676_d_n4;
        locals.var_ids_dn5 = assign65890_e102676_d_n5;
        locals.var_ids_dn6 = assign65890_e102676_d_n6;
        locals.var_ids_dn7 = assign65890_e102676_d_n7;
        locals.var_ids_dn8 = assign65890_e102676_d_n8;
        locals.var_ids_dn9 = assign65890_e102676_d_n9;
        locals.var_ids_dn10 = assign65890_e102676_d_n10;
        locals.var_ids_dn13 = assign65890_e102676_d_n13;
        locals.var_ids_rv = 0.0;

        let (assign65910_e102688, assign65910_e102688_d_n0, assign65910_e102688_d_n2, assign65910_e102688_d_n4, assign65910_e102688_d_n5, assign65910_e102688_d_n6, assign65910_e102688_d_n7, assign65910_e102688_d_n8, assign65910_e102688_d_n9, assign65910_e102688_d_n10, assign65910_e102688_d_n13,) = {
    if (locals.var_guard443 == 0.0) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    } else {
        (locals.var_qiu_noi, locals.var_qiu_noi_dn0, locals.var_qiu_noi_dn2, locals.var_qiu_noi_dn4, locals.var_qiu_noi_dn5, locals.var_qiu_noi_dn6, locals.var_qiu_noi_dn7, locals.var_qiu_noi_dn8, locals.var_qiu_noi_dn9, locals.var_qiu_noi_dn10, locals.var_qiu_noi_dn13,)
    }
};
        locals.var_qiu_noi = assign65910_e102688;
        locals.var_qiu_noi_dn0 = assign65910_e102688_d_n0;
        locals.var_qiu_noi_dn2 = assign65910_e102688_d_n2;
        locals.var_qiu_noi_dn4 = assign65910_e102688_d_n4;
        locals.var_qiu_noi_dn5 = assign65910_e102688_d_n5;
        locals.var_qiu_noi_dn6 = assign65910_e102688_d_n6;
        locals.var_qiu_noi_dn7 = assign65910_e102688_d_n7;
        locals.var_qiu_noi_dn8 = assign65910_e102688_d_n8;
        locals.var_qiu_noi_dn9 = assign65910_e102688_d_n9;
        locals.var_qiu_noi_dn10 = assign65910_e102688_d_n10;
        locals.var_qiu_noi_dn13 = assign65910_e102688_d_n13;
        locals.var_qiu_noi_rv = 0.0;

        let assign65920_e102690: f64 = (-locals.var_weffcv_nf);
        let assign65920_e102692: f64 = (assign65920_e102690 * locals.var_leff);
        locals.var_t1 = assign65920_e102692;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign65930_e102695: f64 = (locals.var_t1 * locals.var_qbu);
        locals.var_qb = assign65930_e102695;
        locals.var_qb_dn0 = ((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0));
        locals.var_qb_dn2 = ((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2));
        locals.var_qb_dn4 = ((locals.var_t1_dn4 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn4));
        locals.var_qb_dn5 = ((locals.var_t1_dn5 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn5));
        locals.var_qb_dn6 = ((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6));
        locals.var_qb_dn7 = ((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7));
        locals.var_qb_dn8 = ((locals.var_t1_dn8 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn8));
        locals.var_qb_dn9 = ((locals.var_t1_dn9 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn9));
        locals.var_qb_dn10 = ((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10));
        locals.var_qb_dn13 = ((locals.var_t1_dn13 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn13));
        locals.var_qb_rv = 0.0;

        let assign65940_e102698: f64 = (locals.var_t1 * locals.var_qiu);
        locals.var_qi = assign65940_e102698;
        locals.var_qi_dn0 = ((locals.var_t1_dn0 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn0));
        locals.var_qi_dn2 = ((locals.var_t1_dn2 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn2));
        locals.var_qi_dn4 = ((locals.var_t1_dn4 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn4));
        locals.var_qi_dn5 = ((locals.var_t1_dn5 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn5));
        locals.var_qi_dn6 = ((locals.var_t1_dn6 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn6));
        locals.var_qi_dn7 = ((locals.var_t1_dn7 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn7));
        locals.var_qi_dn8 = ((locals.var_t1_dn8 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn8));
        locals.var_qi_dn9 = ((locals.var_t1_dn9 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn9));
        locals.var_qi_dn10 = ((locals.var_t1_dn10 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn10));
        locals.var_qi_dn13 = ((locals.var_t1_dn13 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn13));
        locals.var_qi_rv = 0.0;

        let assign65950_e102701: f64 = (locals.var_qi * locals.var_qdrat);
        locals.var_qd = assign65950_e102701;
        locals.var_qd_dn0 = ((locals.var_qi_dn0 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn0));
        locals.var_qd_dn2 = ((locals.var_qi_dn2 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn2));
        locals.var_qd_dn4 = ((locals.var_qi_dn4 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn4));
        locals.var_qd_dn5 = ((locals.var_qi_dn5 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn5));
        locals.var_qd_dn6 = ((locals.var_qi_dn6 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn6));
        locals.var_qd_dn7 = ((locals.var_qi_dn7 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn7));
        locals.var_qd_dn8 = ((locals.var_qi_dn8 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn8));
        locals.var_qd_dn9 = ((locals.var_qi_dn9 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn9));
        locals.var_qd_dn10 = ((locals.var_qi_dn10 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn10));
        locals.var_qd_dn13 = ((locals.var_qi_dn13 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn13));
        locals.var_qd_rv = 0.0;

        let assign65960_e102704: f64 = (locals.var_t1 * locals.var_qiu_noi);
        locals.var_qi_noi = assign65960_e102704;
        locals.var_qi_noi_dn0 = ((locals.var_t1_dn0 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn0));
        locals.var_qi_noi_dn2 = ((locals.var_t1_dn2 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn2));
        locals.var_qi_noi_dn4 = ((locals.var_t1_dn4 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn4));
        locals.var_qi_noi_dn5 = ((locals.var_t1_dn5 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn5));
        locals.var_qi_noi_dn6 = ((locals.var_t1_dn6 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn6));
        locals.var_qi_noi_dn7 = ((locals.var_t1_dn7 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn7));
        locals.var_qi_noi_dn8 = ((locals.var_t1_dn8 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn8));
        locals.var_qi_noi_dn9 = ((locals.var_t1_dn9 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn9));
        locals.var_qi_noi_dn10 = ((locals.var_t1_dn10 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn10));
        locals.var_qi_noi_dn13 = ((locals.var_t1_dn13 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn13));
        locals.var_qi_noi_rv = 0.0;

        let assign65970_e102707: f64 = (locals.var_vds - locals.var_pds);
        let assign65970_e102709: f64 = (assign65970_e102707 / 2.0);
        locals.var_t1 = assign65970_e102709;
        locals.var_t1_dn0 = ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0);
        locals.var_t1_dn2 = ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0);
        locals.var_t1_dn4 = ((locals.var_vds_dn4 - locals.var_pds_dn4) / 2.0);
        locals.var_t1_dn5 = ((locals.var_vds_dn5 - locals.var_pds_dn5) / 2.0);
        locals.var_t1_dn6 = ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0);
        locals.var_t1_dn7 = ((locals.var_vds_dn7 - locals.var_pds_dn7) / 2.0);
        locals.var_t1_dn8 = ((locals.var_vds_dn8 - locals.var_pds_dn8) / 2.0);
        locals.var_t1_dn9 = ((locals.var_vds_dn9 - locals.var_pds_dn9) / 2.0);
        locals.var_t1_dn10 = ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0);
        locals.var_t1_dn13 = ((locals.var_vds_dn13 - locals.var_pds_dn13) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign65980_e102712: f64 = (2.0 * locals.var_t1);
        let assign65980_e102714: f64 = (assign65980_e102712 / p.p263);
        locals.var_tmf1 = assign65980_e102714;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p263);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p263);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p263);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p263);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p263);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p263);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p263);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p263);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p263);
        locals.var_tmf1_dn13 = ((2.0 * locals.var_t1_dn13) / p.p263);
        locals.var_tmf1_rv = 0.0;

        let assign65990_e102719: f64 = (1.0 / 2.0);
        let assign65990_e102723: f64 = (1.0 / 6.0);
        let assign65990_e102727: f64 = (1.0 / 24.0);
        let assign65990_e102731: f64 = (1.0 / 120.0);
        let assign65990_e102735: f64 = (1.0 / 720.0);
        let assign65990_e102739: f64 = (1.0 / 5040.0);
        let assign65990_e102740: f64 = (locals.var_tmf1 * assign65990_e102739);
        let assign65990_e102741: f64 = (assign65990_e102735 + assign65990_e102740);
        let assign65990_e102742: f64 = (locals.var_tmf1 * assign65990_e102741);
        let assign65990_e102743: f64 = (assign65990_e102731 + assign65990_e102742);
        let assign65990_e102744: f64 = (locals.var_tmf1 * assign65990_e102743);
        let assign65990_e102745: f64 = (assign65990_e102727 + assign65990_e102744);
        let assign65990_e102746: f64 = (locals.var_tmf1 * assign65990_e102745);
        let assign65990_e102747: f64 = (assign65990_e102723 + assign65990_e102746);
        let assign65990_e102748: f64 = (locals.var_tmf1 * assign65990_e102747);
        let assign65990_e102749: f64 = (assign65990_e102719 + assign65990_e102748);
        let assign65990_e102750: f64 = (locals.var_tmf1 * assign65990_e102749);
        let assign65990_e102751: f64 = (1.0 + assign65990_e102750);
        locals.var_tmf2 = assign65990_e102751;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn13 = ((locals.var_tmf1_dn13 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign65990_e102739)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign66000_e102754: f64 = (1.0 / 2.0);
        let assign66000_e102758: f64 = (1.0 / 3.0);
        let assign66000_e102762: f64 = (1.0 / 8.0);
        let assign66000_e102766: f64 = (1.0 / 30.0);
        let assign66000_e102770: f64 = (1.0 / 144.0);
        let assign66000_e102774: f64 = (1.0 / 840.0);
        let assign66000_e102775: f64 = (locals.var_tmf1 * assign66000_e102774);
        let assign66000_e102776: f64 = (assign66000_e102770 + assign66000_e102775);
        let assign66000_e102777: f64 = (locals.var_tmf1 * assign66000_e102776);
        let assign66000_e102778: f64 = (assign66000_e102766 + assign66000_e102777);
        let assign66000_e102779: f64 = (locals.var_tmf1 * assign66000_e102778);
        let assign66000_e102780: f64 = (assign66000_e102762 + assign66000_e102779);
        let assign66000_e102781: f64 = (locals.var_tmf1 * assign66000_e102780);
        let assign66000_e102782: f64 = (assign66000_e102758 + assign66000_e102781);
        let assign66000_e102783: f64 = (locals.var_tmf1 * assign66000_e102782);
        let assign66000_e102784: f64 = (assign66000_e102754 + assign66000_e102783);
        locals.var_tmf3 = assign66000_e102784;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign66000_e102774)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign66000_e102774)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign66000_e102774)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign66000_e102774)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign66000_e102774)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign66000_e102774)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign66000_e102774)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign66000_e102774)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign66000_e102774)))))))));
        locals.var_tmf3_dn13 = ((locals.var_tmf1_dn13 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign66000_e102774)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign66010_e102787: f64 = (p.p263 / locals.var_tmf2);
        locals.var_pzadd = assign66010_e102787;
        locals.var_pzadd_dn0 = (-((p.p263 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn2 = (-((p.p263 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn4 = (-((p.p263 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn5 = (-((p.p263 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn6 = (-((p.p263 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn7 = (-((p.p263 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn8 = (-((p.p263 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn9 = (-((p.p263 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn10 = (-((p.p263 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn13 = (-((p.p263 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_rv = 0.0;

        let assign66020_e102789: f64 = (-2.0);
        let assign66020_e102791: f64 = (assign66020_e102789 * locals.var_tmf3);
        let assign66020_e102794: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign66020_e102795: f64 = (assign66020_e102791 / assign66020_e102794);
        locals.var_t2 = assign66020_e102795;
        locals.var_t2_dn0 = ((((assign66020_e102789 * locals.var_tmf3_dn0) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn2 = ((((assign66020_e102789 * locals.var_tmf3_dn2) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn4 = ((((assign66020_e102789 * locals.var_tmf3_dn4) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn5 = ((((assign66020_e102789 * locals.var_tmf3_dn5) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn6 = ((((assign66020_e102789 * locals.var_tmf3_dn6) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn7 = ((((assign66020_e102789 * locals.var_tmf3_dn7) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn8 = ((((assign66020_e102789 * locals.var_tmf3_dn8) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn9 = ((((assign66020_e102789 * locals.var_tmf3_dn9) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn10 = ((((assign66020_e102789 * locals.var_tmf3_dn10) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn13 = ((((assign66020_e102789 * locals.var_tmf3_dn13) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_rv = 0.0;

        let assign66030_e102799: f64 = (10.0 * 2.220446049250313e-16);
        let assign66030_e102802: f64 = (10.0 * 2.220446049250313e-16);
        let assign66030_e102803: f64 = (assign66030_e102799 + assign66030_e102802);
        let assign66030_e102807: f64 = (10.0 * 2.220446049250313e-16);
        let assign66030_e102810: f64 = if ((locals.var_pzadd < assign66030_e102803) && (assign66030_e102807 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1574 = assign66030_e102810;
        locals.var_guard1574_rv = 0.0;

        let (assign66040_e102822, assign66040_e102822_d_n0, assign66040_e102822_d_n2, assign66040_e102822_d_n4, assign66040_e102822_d_n5, assign66040_e102822_d_n6, assign66040_e102822_d_n7, assign66040_e102822_d_n8, assign66040_e102822_d_n9, assign66040_e102822_d_n10, assign66040_e102822_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66040_e102814: f64 = (10.0 * 2.220446049250313e-16);
        let assign66040_e102817: f64 = (10.0 * 2.220446049250313e-16);
        let assign66040_e102818: f64 = (assign66040_e102814 + assign66040_e102817);
        let assign66040_e102820: f64 = (assign66040_e102818 - locals.var_pzadd);
        (assign66040_e102820, (-locals.var_pzadd_dn0), (-locals.var_pzadd_dn2), (-locals.var_pzadd_dn4), (-locals.var_pzadd_dn5), (-locals.var_pzadd_dn6), (-locals.var_pzadd_dn7), (-locals.var_pzadd_dn8), (-locals.var_pzadd_dn9), (-locals.var_pzadd_dn10), (-locals.var_pzadd_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign66040_e102822;
        locals.var_tmf1_dn0 = assign66040_e102822_d_n0;
        locals.var_tmf1_dn2 = assign66040_e102822_d_n2;
        locals.var_tmf1_dn4 = assign66040_e102822_d_n4;
        locals.var_tmf1_dn5 = assign66040_e102822_d_n5;
        locals.var_tmf1_dn6 = assign66040_e102822_d_n6;
        locals.var_tmf1_dn7 = assign66040_e102822_d_n7;
        locals.var_tmf1_dn8 = assign66040_e102822_d_n8;
        locals.var_tmf1_dn9 = assign66040_e102822_d_n9;
        locals.var_tmf1_dn10 = assign66040_e102822_d_n10;
        locals.var_tmf1_dn13 = assign66040_e102822_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign66050_e102828, assign66050_e102828_d_n0, assign66050_e102828_d_n2, assign66050_e102828_d_n4, assign66050_e102828_d_n5, assign66050_e102828_d_n6, assign66050_e102828_d_n7, assign66050_e102828_d_n8, assign66050_e102828_d_n9, assign66050_e102828_d_n10, assign66050_e102828_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66050_e102826: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign66050_e102826, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign66050_e102828;
        locals.var_x2_dn0 = assign66050_e102828_d_n0;
        locals.var_x2_dn2 = assign66050_e102828_d_n2;
        locals.var_x2_dn4 = assign66050_e102828_d_n4;
        locals.var_x2_dn5 = assign66050_e102828_d_n5;
        locals.var_x2_dn6 = assign66050_e102828_d_n6;
        locals.var_x2_dn7 = assign66050_e102828_d_n7;
        locals.var_x2_dn8 = assign66050_e102828_d_n8;
        locals.var_x2_dn9 = assign66050_e102828_d_n9;
        locals.var_x2_dn10 = assign66050_e102828_d_n10;
        locals.var_x2_dn13 = assign66050_e102828_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign66060_e102838, assign66060_e102838_d_n0, assign66060_e102838_d_n2, assign66060_e102838_d_n4, assign66060_e102838_d_n5, assign66060_e102838_d_n6, assign66060_e102838_d_n7, assign66060_e102838_d_n8, assign66060_e102838_d_n9, assign66060_e102838_d_n10, assign66060_e102838_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66060_e102832: f64 = (10.0 * 2.220446049250313e-16);
        let assign66060_e102835: f64 = (10.0 * 2.220446049250313e-16);
        let assign66060_e102836: f64 = (assign66060_e102832 * assign66060_e102835);
        (assign66060_e102836, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign66060_e102838;
        locals.var_xmax2_dn0 = assign66060_e102838_d_n0;
        locals.var_xmax2_dn2 = assign66060_e102838_d_n2;
        locals.var_xmax2_dn4 = assign66060_e102838_d_n4;
        locals.var_xmax2_dn5 = assign66060_e102838_d_n5;
        locals.var_xmax2_dn6 = assign66060_e102838_d_n6;
        locals.var_xmax2_dn7 = assign66060_e102838_d_n7;
        locals.var_xmax2_dn8 = assign66060_e102838_d_n8;
        locals.var_xmax2_dn9 = assign66060_e102838_d_n9;
        locals.var_xmax2_dn10 = assign66060_e102838_d_n10;
        locals.var_xmax2_dn13 = assign66060_e102838_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign66070_e102842, assign66070_e102842_d_n0, assign66070_e102842_d_n2, assign66070_e102842_d_n4, assign66070_e102842_d_n5, assign66070_e102842_d_n6, assign66070_e102842_d_n7, assign66070_e102842_d_n8, assign66070_e102842_d_n9, assign66070_e102842_d_n10, assign66070_e102842_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66070_e102842;
        locals.var_xp_dn0 = assign66070_e102842_d_n0;
        locals.var_xp_dn2 = assign66070_e102842_d_n2;
        locals.var_xp_dn4 = assign66070_e102842_d_n4;
        locals.var_xp_dn5 = assign66070_e102842_d_n5;
        locals.var_xp_dn6 = assign66070_e102842_d_n6;
        locals.var_xp_dn7 = assign66070_e102842_d_n7;
        locals.var_xp_dn8 = assign66070_e102842_d_n8;
        locals.var_xp_dn9 = assign66070_e102842_d_n9;
        locals.var_xp_dn10 = assign66070_e102842_d_n10;
        locals.var_xp_dn13 = assign66070_e102842_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign66080_e102846, assign66080_e102846_d_n0, assign66080_e102846_d_n2, assign66080_e102846_d_n4, assign66080_e102846_d_n5, assign66080_e102846_d_n6, assign66080_e102846_d_n7, assign66080_e102846_d_n8, assign66080_e102846_d_n9, assign66080_e102846_d_n10, assign66080_e102846_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66080_e102846;
        locals.var_xmp_dn0 = assign66080_e102846_d_n0;
        locals.var_xmp_dn2 = assign66080_e102846_d_n2;
        locals.var_xmp_dn4 = assign66080_e102846_d_n4;
        locals.var_xmp_dn5 = assign66080_e102846_d_n5;
        locals.var_xmp_dn6 = assign66080_e102846_d_n6;
        locals.var_xmp_dn7 = assign66080_e102846_d_n7;
        locals.var_xmp_dn8 = assign66080_e102846_d_n8;
        locals.var_xmp_dn9 = assign66080_e102846_d_n9;
        locals.var_xmp_dn10 = assign66080_e102846_d_n10;
        locals.var_xmp_dn13 = assign66080_e102846_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign66090_e102850,) = {
    if (locals.var_guard1574 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66090_e102850;
        locals.var_m0_rv = 0.0;

        let (assign66100_e102854,) = {
    if (locals.var_guard1574 != 0.0) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66100_e102854;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_239(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign66110_e102858, assign66110_e102858_d_n0, assign66110_e102858_d_n2, assign66110_e102858_d_n4, assign66110_e102858_d_n5, assign66110_e102858_d_n6, assign66110_e102858_d_n7, assign66110_e102858_d_n8, assign66110_e102858_d_n9, assign66110_e102858_d_n10, assign66110_e102858_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign66110_e102858;
        locals.var_arg_dn0 = assign66110_e102858_d_n0;
        locals.var_arg_dn2 = assign66110_e102858_d_n2;
        locals.var_arg_dn4 = assign66110_e102858_d_n4;
        locals.var_arg_dn5 = assign66110_e102858_d_n5;
        locals.var_arg_dn6 = assign66110_e102858_d_n6;
        locals.var_arg_dn7 = assign66110_e102858_d_n7;
        locals.var_arg_dn8 = assign66110_e102858_d_n8;
        locals.var_arg_dn9 = assign66110_e102858_d_n9;
        locals.var_arg_dn10 = assign66110_e102858_d_n10;
        locals.var_arg_dn13 = assign66110_e102858_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign66120_e102862, assign66120_e102862_d_n0, assign66120_e102862_d_n2, assign66120_e102862_d_n4, assign66120_e102862_d_n5, assign66120_e102862_d_n6, assign66120_e102862_d_n7, assign66120_e102862_d_n8, assign66120_e102862_d_n9, assign66120_e102862_d_n10, assign66120_e102862_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66120_e102862;
        locals.var_dnm_dn0 = assign66120_e102862_d_n0;
        locals.var_dnm_dn2 = assign66120_e102862_d_n2;
        locals.var_dnm_dn4 = assign66120_e102862_d_n4;
        locals.var_dnm_dn5 = assign66120_e102862_d_n5;
        locals.var_dnm_dn6 = assign66120_e102862_d_n6;
        locals.var_dnm_dn7 = assign66120_e102862_d_n7;
        locals.var_dnm_dn8 = assign66120_e102862_d_n8;
        locals.var_dnm_dn9 = assign66120_e102862_d_n9;
        locals.var_dnm_dn10 = assign66120_e102862_d_n10;
        locals.var_dnm_dn13 = assign66120_e102862_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign66130_e102868, assign66130_e102868_d_n0, assign66130_e102868_d_n2, assign66130_e102868_d_n4, assign66130_e102868_d_n5, assign66130_e102868_d_n6, assign66130_e102868_d_n7, assign66130_e102868_d_n8, assign66130_e102868_d_n9, assign66130_e102868_d_n10, assign66130_e102868_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66130_e102866: f64 = (locals.var_xp * locals.var_x2);
        (assign66130_e102866, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66130_e102868;
        locals.var_xp_dn0 = assign66130_e102868_d_n0;
        locals.var_xp_dn2 = assign66130_e102868_d_n2;
        locals.var_xp_dn4 = assign66130_e102868_d_n4;
        locals.var_xp_dn5 = assign66130_e102868_d_n5;
        locals.var_xp_dn6 = assign66130_e102868_d_n6;
        locals.var_xp_dn7 = assign66130_e102868_d_n7;
        locals.var_xp_dn8 = assign66130_e102868_d_n8;
        locals.var_xp_dn9 = assign66130_e102868_d_n9;
        locals.var_xp_dn10 = assign66130_e102868_d_n10;
        locals.var_xp_dn13 = assign66130_e102868_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign66140_e102874, assign66140_e102874_d_n0, assign66140_e102874_d_n2, assign66140_e102874_d_n4, assign66140_e102874_d_n5, assign66140_e102874_d_n6, assign66140_e102874_d_n7, assign66140_e102874_d_n8, assign66140_e102874_d_n9, assign66140_e102874_d_n10, assign66140_e102874_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66140_e102872: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66140_e102872, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66140_e102874;
        locals.var_xmp_dn0 = assign66140_e102874_d_n0;
        locals.var_xmp_dn2 = assign66140_e102874_d_n2;
        locals.var_xmp_dn4 = assign66140_e102874_d_n4;
        locals.var_xmp_dn5 = assign66140_e102874_d_n5;
        locals.var_xmp_dn6 = assign66140_e102874_d_n6;
        locals.var_xmp_dn7 = assign66140_e102874_d_n7;
        locals.var_xmp_dn8 = assign66140_e102874_d_n8;
        locals.var_xmp_dn9 = assign66140_e102874_d_n9;
        locals.var_xmp_dn10 = assign66140_e102874_d_n10;
        locals.var_xmp_dn13 = assign66140_e102874_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign66150_e102880, assign66150_e102880_d_n0, assign66150_e102880_d_n2, assign66150_e102880_d_n4, assign66150_e102880_d_n5, assign66150_e102880_d_n6, assign66150_e102880_d_n7, assign66150_e102880_d_n8, assign66150_e102880_d_n9, assign66150_e102880_d_n10, assign66150_e102880_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66150_e102878: f64 = (locals.var_xp * locals.var_x2);
        (assign66150_e102878, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66150_e102880;
        locals.var_xp_dn0 = assign66150_e102880_d_n0;
        locals.var_xp_dn2 = assign66150_e102880_d_n2;
        locals.var_xp_dn4 = assign66150_e102880_d_n4;
        locals.var_xp_dn5 = assign66150_e102880_d_n5;
        locals.var_xp_dn6 = assign66150_e102880_d_n6;
        locals.var_xp_dn7 = assign66150_e102880_d_n7;
        locals.var_xp_dn8 = assign66150_e102880_d_n8;
        locals.var_xp_dn9 = assign66150_e102880_d_n9;
        locals.var_xp_dn10 = assign66150_e102880_d_n10;
        locals.var_xp_dn13 = assign66150_e102880_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign66160_e102886, assign66160_e102886_d_n0, assign66160_e102886_d_n2, assign66160_e102886_d_n4, assign66160_e102886_d_n5, assign66160_e102886_d_n6, assign66160_e102886_d_n7, assign66160_e102886_d_n8, assign66160_e102886_d_n9, assign66160_e102886_d_n10, assign66160_e102886_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66160_e102884: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66160_e102884, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66160_e102886;
        locals.var_xmp_dn0 = assign66160_e102886_d_n0;
        locals.var_xmp_dn2 = assign66160_e102886_d_n2;
        locals.var_xmp_dn4 = assign66160_e102886_d_n4;
        locals.var_xmp_dn5 = assign66160_e102886_d_n5;
        locals.var_xmp_dn6 = assign66160_e102886_d_n6;
        locals.var_xmp_dn7 = assign66160_e102886_d_n7;
        locals.var_xmp_dn8 = assign66160_e102886_d_n8;
        locals.var_xmp_dn9 = assign66160_e102886_d_n9;
        locals.var_xmp_dn10 = assign66160_e102886_d_n10;
        locals.var_xmp_dn13 = assign66160_e102886_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign66170_e102892, assign66170_e102892_d_n0, assign66170_e102892_d_n2, assign66170_e102892_d_n4, assign66170_e102892_d_n5, assign66170_e102892_d_n6, assign66170_e102892_d_n7, assign66170_e102892_d_n8, assign66170_e102892_d_n9, assign66170_e102892_d_n10, assign66170_e102892_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66170_e102890: f64 = (locals.var_xp + locals.var_xmp);
        (assign66170_e102890, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign66170_e102892;
        locals.var_arg_dn0 = assign66170_e102892_d_n0;
        locals.var_arg_dn2 = assign66170_e102892_d_n2;
        locals.var_arg_dn4 = assign66170_e102892_d_n4;
        locals.var_arg_dn5 = assign66170_e102892_d_n5;
        locals.var_arg_dn6 = assign66170_e102892_d_n6;
        locals.var_arg_dn7 = assign66170_e102892_d_n7;
        locals.var_arg_dn8 = assign66170_e102892_d_n8;
        locals.var_arg_dn9 = assign66170_e102892_d_n9;
        locals.var_arg_dn10 = assign66170_e102892_d_n10;
        locals.var_arg_dn13 = assign66170_e102892_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign66180_e102896, assign66180_e102896_d_n0, assign66180_e102896_d_n2, assign66180_e102896_d_n4, assign66180_e102896_d_n5, assign66180_e102896_d_n6, assign66180_e102896_d_n7, assign66180_e102896_d_n8, assign66180_e102896_d_n9, assign66180_e102896_d_n10, assign66180_e102896_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66180_e102896;
        locals.var_dnm_dn0 = assign66180_e102896_d_n0;
        locals.var_dnm_dn2 = assign66180_e102896_d_n2;
        locals.var_dnm_dn4 = assign66180_e102896_d_n4;
        locals.var_dnm_dn5 = assign66180_e102896_d_n5;
        locals.var_dnm_dn6 = assign66180_e102896_d_n6;
        locals.var_dnm_dn7 = assign66180_e102896_d_n7;
        locals.var_dnm_dn8 = assign66180_e102896_d_n8;
        locals.var_dnm_dn9 = assign66180_e102896_d_n9;
        locals.var_dnm_dn10 = assign66180_e102896_d_n10;
        locals.var_dnm_dn13 = assign66180_e102896_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign66190_e102911: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1575 = assign66190_e102911;
        locals.var_guard1575_rv = 0.0;

        let assign66200_e102914: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1576 = assign66200_e102914;
        locals.var_guard1576_rv = 0.0;

        let (assign66210_e102922,) = {
    if (((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_guard1576 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66210_e102922;
        locals.var_mm_rv = 0.0;

        let assign66220_e102925: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1577 = assign66220_e102925;
        locals.var_guard1577_rv = 0.0;

        let (assign66230_e102936,) = {
    if ((((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_guard1576 == 0.0)) && (locals.var_guard1577 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66230_e102936;
        locals.var_mm_rv = 0.0;

        let assign66240_e102939: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1578 = assign66240_e102939;
        locals.var_guard1578_rv = 0.0;

        let (assign66250_e102953,) = {
    if (((((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_guard1576 == 0.0)) && (locals.var_guard1577 == 0.0)) && (locals.var_guard1578 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66250_e102953;
        locals.var_mm_rv = 0.0;

        let assign66260_e102956: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1579 = assign66260_e102956;
        locals.var_guard1579_rv = 0.0;

        let (assign66270_e102973,) = {
    if ((((((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_guard1576 == 0.0)) && (locals.var_guard1577 == 0.0)) && (locals.var_guard1578 == 0.0)) && (locals.var_guard1579 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66270_e102973;
        locals.var_mm_rv = 0.0;

        let (assign66280_e102979,) = {
    if ((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66280_e102979;
        locals.var_m0_rv = 0.0;

        let mut assign66290_loop_guard: usize = 0;
        while {
            let assign66290_cond_e102986: f64 = if (((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign66290_cond_e102986 != 0.0
        } {
            assign66290_loop_guard += 1;
            assert!(assign66290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign66290_body0_e102993, assign66290_body0_e102993_d_n0, assign66290_body0_e102993_d_n2, assign66290_body0_e102993_d_n4, assign66290_body0_e102993_d_n5, assign66290_body0_e102993_d_n6, assign66290_body0_e102993_d_n7, assign66290_body0_e102993_d_n8, assign66290_body0_e102993_d_n9, assign66290_body0_e102993_d_n10, assign66290_body0_e102993_d_n13,) = {
    if ((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) {
        let assign66290_body0_e102991: f64 = (locals.var_dnm).sqrt();
        (assign66290_body0_e102991, (locals.var_dnm_dn0 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn2 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn4 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn5 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn6 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn7 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn8 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn9 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn10 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn13 / (2.0 * assign66290_body0_e102991)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign66290_body0_e102993;
            locals.var_dnm_dn0 = assign66290_body0_e102993_d_n0;
            locals.var_dnm_dn2 = assign66290_body0_e102993_d_n2;
            locals.var_dnm_dn4 = assign66290_body0_e102993_d_n4;
            locals.var_dnm_dn5 = assign66290_body0_e102993_d_n5;
            locals.var_dnm_dn6 = assign66290_body0_e102993_d_n6;
            locals.var_dnm_dn7 = assign66290_body0_e102993_d_n7;
            locals.var_dnm_dn8 = assign66290_body0_e102993_d_n8;
            locals.var_dnm_dn9 = assign66290_body0_e102993_d_n9;
            locals.var_dnm_dn10 = assign66290_body0_e102993_d_n10;
            locals.var_dnm_dn13 = assign66290_body0_e102993_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign66290_body1_e103001,) = {
    if ((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) {
        let assign66290_body1_e102999: f64 = (locals.var_m0 + 1.0);
        (assign66290_body1_e102999,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign66290_body1_e103001;
            locals.var_m0_rv = 0.0;
        }

        let (assign66300_e103019, assign66300_e103019_d_n0, assign66300_e103019_d_n2, assign66300_e103019_d_n4, assign66300_e103019_d_n5, assign66300_e103019_d_n6, assign66300_e103019_d_n7, assign66300_e103019_d_n8, assign66300_e103019_d_n9, assign66300_e103019_d_n10, assign66300_e103019_d_n13,) = {
    if ((locals.var_guard1574 != 0.0) && (locals.var_guard1575 == 0.0)) {
        let (assign66300_e103017, assign66300_e103017_d_n0, assign66300_e103017_d_n2, assign66300_e103017_d_n4, assign66300_e103017_d_n5, assign66300_e103017_d_n6, assign66300_e103017_d_n7, assign66300_e103017_d_n8, assign66300_e103017_d_n9, assign66300_e103017_d_n10, assign66300_e103017_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign66300_e103014: f64 = (2.0 * 2.0);
                let assign66300_e103015: f64 = (1.0 / assign66300_e103014);
                let assign66300_e103016: f64 = (locals.var_dnm).powf(assign66300_e103015);
                (assign66300_e103016, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn0)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn2)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn4)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn5)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn6)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn7)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn8)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn9)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn10)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn13)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign66300_e103017, assign66300_e103017_d_n0, assign66300_e103017_d_n2, assign66300_e103017_d_n4, assign66300_e103017_d_n5, assign66300_e103017_d_n6, assign66300_e103017_d_n7, assign66300_e103017_d_n8, assign66300_e103017_d_n9, assign66300_e103017_d_n10, assign66300_e103017_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66300_e103019;
        locals.var_dnm_dn0 = assign66300_e103019_d_n0;
        locals.var_dnm_dn2 = assign66300_e103019_d_n2;
        locals.var_dnm_dn4 = assign66300_e103019_d_n4;
        locals.var_dnm_dn5 = assign66300_e103019_d_n5;
        locals.var_dnm_dn6 = assign66300_e103019_d_n6;
        locals.var_dnm_dn7 = assign66300_e103019_d_n7;
        locals.var_dnm_dn8 = assign66300_e103019_d_n8;
        locals.var_dnm_dn9 = assign66300_e103019_d_n9;
        locals.var_dnm_dn10 = assign66300_e103019_d_n10;
        locals.var_dnm_dn13 = assign66300_e103019_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign66310_e103025, assign66310_e103025_d_n0, assign66310_e103025_d_n2, assign66310_e103025_d_n4, assign66310_e103025_d_n5, assign66310_e103025_d_n6, assign66310_e103025_d_n7, assign66310_e103025_d_n8, assign66310_e103025_d_n9, assign66310_e103025_d_n10, assign66310_e103025_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66310_e103023: f64 = (1.0 / locals.var_dnm);
        (assign66310_e103023, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66310_e103025;
        locals.var_dnm_dn0 = assign66310_e103025_d_n0;
        locals.var_dnm_dn2 = assign66310_e103025_d_n2;
        locals.var_dnm_dn4 = assign66310_e103025_d_n4;
        locals.var_dnm_dn5 = assign66310_e103025_d_n5;
        locals.var_dnm_dn6 = assign66310_e103025_d_n6;
        locals.var_dnm_dn7 = assign66310_e103025_d_n7;
        locals.var_dnm_dn8 = assign66310_e103025_d_n8;
        locals.var_dnm_dn9 = assign66310_e103025_d_n9;
        locals.var_dnm_dn10 = assign66310_e103025_d_n10;
        locals.var_dnm_dn13 = assign66310_e103025_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign66320_e103035, assign66320_e103035_d_n0, assign66320_e103035_d_n2, assign66320_e103035_d_n4, assign66320_e103035_d_n5, assign66320_e103035_d_n6, assign66320_e103035_d_n7, assign66320_e103035_d_n8, assign66320_e103035_d_n9, assign66320_e103035_d_n10, assign66320_e103035_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66320_e103030: f64 = (10.0 * 2.220446049250313e-16);
        let assign66320_e103031: f64 = (locals.var_tmf1 * assign66320_e103030);
        let assign66320_e103033: f64 = (assign66320_e103031 * locals.var_dnm);
        (assign66320_e103033, (((locals.var_tmf1_dn0 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign66320_e103035;
        locals.var_tmf0_dn0 = assign66320_e103035_d_n0;
        locals.var_tmf0_dn2 = assign66320_e103035_d_n2;
        locals.var_tmf0_dn4 = assign66320_e103035_d_n4;
        locals.var_tmf0_dn5 = assign66320_e103035_d_n5;
        locals.var_tmf0_dn6 = assign66320_e103035_d_n6;
        locals.var_tmf0_dn7 = assign66320_e103035_d_n7;
        locals.var_tmf0_dn8 = assign66320_e103035_d_n8;
        locals.var_tmf0_dn9 = assign66320_e103035_d_n9;
        locals.var_tmf0_dn10 = assign66320_e103035_d_n10;
        locals.var_tmf0_dn13 = assign66320_e103035_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign66330_e103047, assign66330_e103047_d_n0, assign66330_e103047_d_n2, assign66330_e103047_d_n4, assign66330_e103047_d_n5, assign66330_e103047_d_n6, assign66330_e103047_d_n7, assign66330_e103047_d_n8, assign66330_e103047_d_n9, assign66330_e103047_d_n10, assign66330_e103047_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66330_e103039: f64 = (10.0 * 2.220446049250313e-16);
        let assign66330_e103041: f64 = (assign66330_e103039 * locals.var_xmp);
        let assign66330_e103043: f64 = (assign66330_e103041 * locals.var_dnm);
        let assign66330_e103045: f64 = (assign66330_e103043 / locals.var_arg);
        (assign66330_e103045, ((((((assign66330_e103039 * locals.var_xmp_dn0) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn0)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn2) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn2)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn4) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn4)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn5) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn5)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn6) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn6)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn7) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn7)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn8) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn8)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn9) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn9)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn10) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn10)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn13) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn13)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66330_e103047;
        locals.var_t0_dn0 = assign66330_e103047_d_n0;
        locals.var_t0_dn2 = assign66330_e103047_d_n2;
        locals.var_t0_dn4 = assign66330_e103047_d_n4;
        locals.var_t0_dn5 = assign66330_e103047_d_n5;
        locals.var_t0_dn6 = assign66330_e103047_d_n6;
        locals.var_t0_dn7 = assign66330_e103047_d_n7;
        locals.var_t0_dn8 = assign66330_e103047_d_n8;
        locals.var_t0_dn9 = assign66330_e103047_d_n9;
        locals.var_t0_dn10 = assign66330_e103047_d_n10;
        locals.var_t0_dn13 = assign66330_e103047_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign66340_e103059, assign66340_e103059_d_n0, assign66340_e103059_d_n2, assign66340_e103059_d_n4, assign66340_e103059_d_n5, assign66340_e103059_d_n6, assign66340_e103059_d_n7, assign66340_e103059_d_n8, assign66340_e103059_d_n9, assign66340_e103059_d_n10, assign66340_e103059_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66340_e103051: f64 = (10.0 * 2.220446049250313e-16);
        let assign66340_e103054: f64 = (10.0 * 2.220446049250313e-16);
        let assign66340_e103055: f64 = (assign66340_e103051 + assign66340_e103054);
        let assign66340_e103057: f64 = (assign66340_e103055 - locals.var_tmf0);
        (assign66340_e103057, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    }
};
        locals.var_pzadd = assign66340_e103059;
        locals.var_pzadd_dn0 = assign66340_e103059_d_n0;
        locals.var_pzadd_dn2 = assign66340_e103059_d_n2;
        locals.var_pzadd_dn4 = assign66340_e103059_d_n4;
        locals.var_pzadd_dn5 = assign66340_e103059_d_n5;
        locals.var_pzadd_dn6 = assign66340_e103059_d_n6;
        locals.var_pzadd_dn7 = assign66340_e103059_d_n7;
        locals.var_pzadd_dn8 = assign66340_e103059_d_n8;
        locals.var_pzadd_dn9 = assign66340_e103059_d_n9;
        locals.var_pzadd_dn10 = assign66340_e103059_d_n10;
        locals.var_pzadd_dn13 = assign66340_e103059_d_n13;
        locals.var_pzadd_rv = 0.0;

        let (assign66350_e103063, assign66350_e103063_d_n0, assign66350_e103063_d_n2, assign66350_e103063_d_n4, assign66350_e103063_d_n5, assign66350_e103063_d_n6, assign66350_e103063_d_n7, assign66350_e103063_d_n8, assign66350_e103063_d_n9, assign66350_e103063_d_n10, assign66350_e103063_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66350_e103063;
        locals.var_t0_dn0 = assign66350_e103063_d_n0;
        locals.var_t0_dn2 = assign66350_e103063_d_n2;
        locals.var_t0_dn4 = assign66350_e103063_d_n4;
        locals.var_t0_dn5 = assign66350_e103063_d_n5;
        locals.var_t0_dn6 = assign66350_e103063_d_n6;
        locals.var_t0_dn7 = assign66350_e103063_d_n7;
        locals.var_t0_dn8 = assign66350_e103063_d_n8;
        locals.var_t0_dn9 = assign66350_e103063_d_n9;
        locals.var_t0_dn10 = assign66350_e103063_d_n10;
        locals.var_t0_dn13 = assign66350_e103063_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign66360_e103068, assign66360_e103068_d_n0, assign66360_e103068_d_n2, assign66360_e103068_d_n4, assign66360_e103068_d_n5, assign66360_e103068_d_n6, assign66360_e103068_d_n7, assign66360_e103068_d_n8, assign66360_e103068_d_n9, assign66360_e103068_d_n10, assign66360_e103068_d_n13,) = {
    if (locals.var_guard1574 == 0.0) {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    }
};
        locals.var_pzadd = assign66360_e103068;
        locals.var_pzadd_dn0 = assign66360_e103068_d_n0;
        locals.var_pzadd_dn2 = assign66360_e103068_d_n2;
        locals.var_pzadd_dn4 = assign66360_e103068_d_n4;
        locals.var_pzadd_dn5 = assign66360_e103068_d_n5;
        locals.var_pzadd_dn6 = assign66360_e103068_d_n6;
        locals.var_pzadd_dn7 = assign66360_e103068_d_n7;
        locals.var_pzadd_dn8 = assign66360_e103068_d_n8;
        locals.var_pzadd_dn9 = assign66360_e103068_d_n9;
        locals.var_pzadd_dn10 = assign66360_e103068_d_n10;
        locals.var_pzadd_dn13 = assign66360_e103068_d_n13;
        locals.var_pzadd_rv = 0.0;

        let (assign66370_e103073, assign66370_e103073_d_n0, assign66370_e103073_d_n2, assign66370_e103073_d_n4, assign66370_e103073_d_n5, assign66370_e103073_d_n6, assign66370_e103073_d_n7, assign66370_e103073_d_n8, assign66370_e103073_d_n9, assign66370_e103073_d_n10, assign66370_e103073_d_n13,) = {
    if (locals.var_guard1574 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66370_e103073;
        locals.var_t0_dn0 = assign66370_e103073_d_n0;
        locals.var_t0_dn2 = assign66370_e103073_d_n2;
        locals.var_t0_dn4 = assign66370_e103073_d_n4;
        locals.var_t0_dn5 = assign66370_e103073_d_n5;
        locals.var_t0_dn6 = assign66370_e103073_d_n6;
        locals.var_t0_dn7 = assign66370_e103073_d_n7;
        locals.var_t0_dn8 = assign66370_e103073_d_n8;
        locals.var_t0_dn9 = assign66370_e103073_d_n9;
        locals.var_t0_dn10 = assign66370_e103073_d_n10;
        locals.var_t0_dn13 = assign66370_e103073_d_n13;
        locals.var_t0_rv = 0.0;

        let assign66380_e103076: f64 = (locals.var_ps0 + locals.var_pzadd);
        locals.var_ps0z = assign66380_e103076;
        locals.var_ps0z_dn0 = (locals.var_ps0_dn0 + locals.var_pzadd_dn0);
        locals.var_ps0z_dn2 = (locals.var_ps0_dn2 + locals.var_pzadd_dn2);
        locals.var_ps0z_dn4 = (locals.var_ps0_dn4 + locals.var_pzadd_dn4);
        locals.var_ps0z_dn5 = (locals.var_ps0_dn5 + locals.var_pzadd_dn5);
        locals.var_ps0z_dn6 = (locals.var_ps0_dn6 + locals.var_pzadd_dn6);
        locals.var_ps0z_dn7 = (locals.var_ps0_dn7 + locals.var_pzadd_dn7);
        locals.var_ps0z_dn8 = (locals.var_ps0_dn8 + locals.var_pzadd_dn8);
        locals.var_ps0z_dn9 = (locals.var_ps0_dn9 + locals.var_pzadd_dn9);
        locals.var_ps0z_dn10 = (locals.var_ps0_dn10 + locals.var_pzadd_dn10);
        locals.var_ps0z_dn13 = (locals.var_ps0_dn13 + locals.var_pzadd_dn13);
        locals.var_ps0z_rv = 0.0;

        let assign66390_e103080: f64 = (locals.var_weff / locals.var_leff);
        let assign66390_e103082: f64 = (assign66390_e103080 * p.p435);
        let assign66390_e103084: f64 = (assign66390_e103082 * locals.var_vds);
        let assign66390_e103085: f64 = (locals.var_ids + assign66390_e103084);
        locals.var_ids = assign66390_e103085;
        locals.var_ids_dn0 = (locals.var_ids_dn0 + (assign66390_e103082 * locals.var_vds_dn0));
        locals.var_ids_dn2 = (locals.var_ids_dn2 + (assign66390_e103082 * locals.var_vds_dn2));
        locals.var_ids_dn4 = (locals.var_ids_dn4 + (assign66390_e103082 * locals.var_vds_dn4));
        locals.var_ids_dn5 = (locals.var_ids_dn5 + (assign66390_e103082 * locals.var_vds_dn5));
        locals.var_ids_dn6 = (locals.var_ids_dn6 + (assign66390_e103082 * locals.var_vds_dn6));
        locals.var_ids_dn7 = (locals.var_ids_dn7 + (assign66390_e103082 * locals.var_vds_dn7));
        locals.var_ids_dn8 = (locals.var_ids_dn8 + (assign66390_e103082 * locals.var_vds_dn8));
        locals.var_ids_dn9 = (locals.var_ids_dn9 + (assign66390_e103082 * locals.var_vds_dn9));
        locals.var_ids_dn10 = (locals.var_ids_dn10 + (assign66390_e103082 * locals.var_vds_dn10));
        locals.var_ids_dn13 = (locals.var_ids_dn13 + (assign66390_e103082 * locals.var_vds_dn13));
        locals.var_ids_rv = 0.0;

        let assign66400_e103088: f64 = if p.p23 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1580 = assign66400_e103088;
        locals.var_guard1580_rv = 0.0;

        let (assign66410_e103092, assign66410_e103092_d_n0, assign66410_e103092_d_n2, assign66410_e103092_d_n4, assign66410_e103092_d_n5, assign66410_e103092_d_n6, assign66410_e103092_d_n7, assign66410_e103092_d_n8, assign66410_e103092_d_n9, assign66410_e103092_d_n10, assign66410_e103092_d_n13,) = {
    if (locals.var_guard1580 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13,)
    }
};
        locals.var_isub = assign66410_e103092;
        locals.var_isub_dn0 = assign66410_e103092_d_n0;
        locals.var_isub_dn2 = assign66410_e103092_d_n2;
        locals.var_isub_dn4 = assign66410_e103092_d_n4;
        locals.var_isub_dn5 = assign66410_e103092_d_n5;
        locals.var_isub_dn6 = assign66410_e103092_d_n6;
        locals.var_isub_dn7 = assign66410_e103092_d_n7;
        locals.var_isub_dn8 = assign66410_e103092_d_n8;
        locals.var_isub_dn9 = assign66410_e103092_d_n9;
        locals.var_isub_dn10 = assign66410_e103092_d_n10;
        locals.var_isub_dn13 = assign66410_e103092_d_n13;
        locals.var_isub_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_240(
        locals: &mut StampLocals,
    ) {
        let (assign66420_e103096, assign66420_e103096_d_n0, assign66420_e103096_d_n2, assign66420_e103096_d_n4, assign66420_e103096_d_n5, assign66420_e103096_d_n6, assign66420_e103096_d_n7, assign66420_e103096_d_n8, assign66420_e103096_d_n9, assign66420_e103096_d_n10, assign66420_e103096_d_n13,) = {
    if (locals.var_guard1580 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wk_ii, locals.var_wk_ii_dn0, locals.var_wk_ii_dn2, locals.var_wk_ii_dn4, locals.var_wk_ii_dn5, locals.var_wk_ii_dn6, locals.var_wk_ii_dn7, locals.var_wk_ii_dn8, locals.var_wk_ii_dn9, locals.var_wk_ii_dn10, locals.var_wk_ii_dn13,)
    }
};
        locals.var_wk_ii = assign66420_e103096;
        locals.var_wk_ii_dn0 = assign66420_e103096_d_n0;
        locals.var_wk_ii_dn2 = assign66420_e103096_d_n2;
        locals.var_wk_ii_dn4 = assign66420_e103096_d_n4;
        locals.var_wk_ii_dn5 = assign66420_e103096_d_n5;
        locals.var_wk_ii_dn6 = assign66420_e103096_d_n6;
        locals.var_wk_ii_dn7 = assign66420_e103096_d_n7;
        locals.var_wk_ii_dn8 = assign66420_e103096_d_n8;
        locals.var_wk_ii_dn9 = assign66420_e103096_d_n9;
        locals.var_wk_ii_dn10 = assign66420_e103096_d_n10;
        locals.var_wk_ii_dn13 = assign66420_e103096_d_n13;
        locals.var_wk_ii_rv = 0.0;

        let assign66430_e103103: f64 = if ((locals.var_uc_sub1 > 0.0) && (locals.var_uc_vmax > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1581 = assign66430_e103103;
        locals.var_guard1581_rv = 0.0;

        let (assign66440_e103112, assign66440_e103112_d_n0, assign66440_e103112_d_n2, assign66440_e103112_d_n4, assign66440_e103112_d_n5, assign66440_e103112_d_n6, assign66440_e103112_d_n7, assign66440_e103112_d_n8, assign66440_e103112_d_n9, assign66440_e103112_d_n10, assign66440_e103112_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66440_e103110: f64 = (locals.var_vg2const * locals.var_vgp);
        (assign66440_e103110, ((locals.var_vg2const_dn0 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn0)), ((locals.var_vg2const_dn2 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn2)), ((locals.var_vg2const_dn4 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn4)), ((locals.var_vg2const_dn5 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn5)), ((locals.var_vg2const_dn6 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn6)), ((locals.var_vg2const_dn7 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn7)), ((locals.var_vg2const_dn8 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn8)), ((locals.var_vg2const_dn9 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn9)), ((locals.var_vg2const_dn10 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn10)), ((locals.var_vg2const_dn13 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign66440_e103112;
        locals.var_t1_dn0 = assign66440_e103112_d_n0;
        locals.var_t1_dn2 = assign66440_e103112_d_n2;
        locals.var_t1_dn4 = assign66440_e103112_d_n4;
        locals.var_t1_dn5 = assign66440_e103112_d_n5;
        locals.var_t1_dn6 = assign66440_e103112_d_n6;
        locals.var_t1_dn7 = assign66440_e103112_d_n7;
        locals.var_t1_dn8 = assign66440_e103112_d_n8;
        locals.var_t1_dn9 = assign66440_e103112_d_n9;
        locals.var_t1_dn10 = assign66440_e103112_d_n10;
        locals.var_t1_dn13 = assign66440_e103112_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign66450_e103123, assign66450_e103123_d_n0, assign66450_e103123_d_n2, assign66450_e103123_d_n4, assign66450_e103123_d_n5, assign66450_e103123_d_n6, assign66450_e103123_d_n7, assign66450_e103123_d_n8, assign66450_e103123_d_n9, assign66450_e103123_d_n10, assign66450_e103123_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66450_e103120: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign66450_e103121: f64 = (locals.var_qnsub_esi / assign66450_e103120);
        (assign66450_e103121, (locals.var_qnsub_esi_dn0 / assign66450_e103120), (locals.var_qnsub_esi_dn2 / assign66450_e103120), (locals.var_qnsub_esi_dn4 / assign66450_e103120), (locals.var_qnsub_esi_dn5 / assign66450_e103120), (locals.var_qnsub_esi_dn6 / assign66450_e103120), (locals.var_qnsub_esi_dn7 / assign66450_e103120), (locals.var_qnsub_esi_dn8 / assign66450_e103120), (locals.var_qnsub_esi_dn9 / assign66450_e103120), (locals.var_qnsub_esi_dn10 / assign66450_e103120), (locals.var_qnsub_esi_dn13 / assign66450_e103120),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign66450_e103123;
        locals.var_t3_dn0 = assign66450_e103123_d_n0;
        locals.var_t3_dn2 = assign66450_e103123_d_n2;
        locals.var_t3_dn4 = assign66450_e103123_d_n4;
        locals.var_t3_dn5 = assign66450_e103123_d_n5;
        locals.var_t3_dn6 = assign66450_e103123_d_n6;
        locals.var_t3_dn7 = assign66450_e103123_d_n7;
        locals.var_t3_dn8 = assign66450_e103123_d_n8;
        locals.var_t3_dn9 = assign66450_e103123_d_n9;
        locals.var_t3_dn10 = assign66450_e103123_d_n10;
        locals.var_t3_dn13 = assign66450_e103123_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign66460_e103136, assign66460_e103136_d_n0, assign66460_e103136_d_n2, assign66460_e103136_d_n4, assign66460_e103136_d_n5, assign66460_e103136_d_n6, assign66460_e103136_d_n7, assign66460_e103136_d_n8, assign66460_e103136_d_n9, assign66460_e103136_d_n10, assign66460_e103136_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66460_e103130: f64 = (2.0 / locals.var_qnsub_esi);
        let assign66460_e103133: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign66460_e103134: f64 = (assign66460_e103130 * assign66460_e103133);
        (assign66460_e103134, ((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn9) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn13) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign66460_e103136;
        locals.var_t4_dn0 = assign66460_e103136_d_n0;
        locals.var_t4_dn2 = assign66460_e103136_d_n2;
        locals.var_t4_dn4 = assign66460_e103136_d_n4;
        locals.var_t4_dn5 = assign66460_e103136_d_n5;
        locals.var_t4_dn6 = assign66460_e103136_d_n6;
        locals.var_t4_dn7 = assign66460_e103136_d_n7;
        locals.var_t4_dn8 = assign66460_e103136_d_n8;
        locals.var_t4_dn9 = assign66460_e103136_d_n9;
        locals.var_t4_dn10 = assign66460_e103136_d_n10;
        locals.var_t4_dn13 = assign66460_e103136_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign66470_e103149, assign66470_e103149_d_n0, assign66470_e103149_d_n2, assign66470_e103149_d_n4, assign66470_e103149_d_n5, assign66470_e103149_d_n6, assign66470_e103149_d_n7, assign66470_e103149_d_n8, assign66470_e103149_d_n9, assign66470_e103149_d_n10, assign66470_e103149_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66470_e103143: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign66470_e103146: f64 = (locals.var_xvbs * locals.var_vbsz__blk438);
        let assign66470_e103147: f64 = (assign66470_e103143 - assign66470_e103146);
        (assign66470_e103147, ((locals.var_t1_dn0 - locals.var_beta_inv_dn0) - (locals.var_xvbs * locals.var_vbsz__blk438_dn0)), ((locals.var_t1_dn2 - locals.var_beta_inv_dn2) - (locals.var_xvbs * locals.var_vbsz__blk438_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs * locals.var_vbsz__blk438_dn4)), ((locals.var_t1_dn5 - locals.var_beta_inv_dn5) - (locals.var_xvbs * locals.var_vbsz__blk438_dn5)), ((locals.var_t1_dn6 - locals.var_beta_inv_dn6) - (locals.var_xvbs * locals.var_vbsz__blk438_dn6)), ((locals.var_t1_dn7 - locals.var_beta_inv_dn7) - (locals.var_xvbs * locals.var_vbsz__blk438_dn7)), ((locals.var_t1_dn8 - locals.var_beta_inv_dn8) - (locals.var_xvbs * locals.var_vbsz__blk438_dn8)), ((locals.var_t1_dn9 - locals.var_beta_inv_dn9) - (locals.var_xvbs * locals.var_vbsz__blk438_dn9)), ((locals.var_t1_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbsz__blk438_dn10)), ((locals.var_t1_dn13 - locals.var_beta_inv_dn13) - (locals.var_xvbs * locals.var_vbsz__blk438_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign66470_e103149;
        locals.var_t5_dn0 = assign66470_e103149_d_n0;
        locals.var_t5_dn2 = assign66470_e103149_d_n2;
        locals.var_t5_dn4 = assign66470_e103149_d_n4;
        locals.var_t5_dn5 = assign66470_e103149_d_n5;
        locals.var_t5_dn6 = assign66470_e103149_d_n6;
        locals.var_t5_dn7 = assign66470_e103149_d_n7;
        locals.var_t5_dn8 = assign66470_e103149_d_n8;
        locals.var_t5_dn9 = assign66470_e103149_d_n9;
        locals.var_t5_dn10 = assign66470_e103149_d_n10;
        locals.var_t5_dn13 = assign66470_e103149_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign66480_e103160, assign66480_e103160_d_n0, assign66480_e103160_d_n2, assign66480_e103160_d_n4, assign66480_e103160_d_n5, assign66480_e103160_d_n6, assign66480_e103160_d_n7, assign66480_e103160_d_n8, assign66480_e103160_d_n9, assign66480_e103160_d_n10, assign66480_e103160_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66480_e103157: f64 = (locals.var_t4 * locals.var_t5);
        let assign66480_e103158: f64 = (1.0 + assign66480_e103157);
        (assign66480_e103158, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn13 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign66480_e103160;
        locals.var_t6_dn0 = assign66480_e103160_d_n0;
        locals.var_t6_dn2 = assign66480_e103160_d_n2;
        locals.var_t6_dn4 = assign66480_e103160_d_n4;
        locals.var_t6_dn5 = assign66480_e103160_d_n5;
        locals.var_t6_dn6 = assign66480_e103160_d_n6;
        locals.var_t6_dn7 = assign66480_e103160_d_n7;
        locals.var_t6_dn8 = assign66480_e103160_d_n8;
        locals.var_t6_dn9 = assign66480_e103160_d_n9;
        locals.var_t6_dn10 = assign66480_e103160_d_n10;
        locals.var_t6_dn13 = assign66480_e103160_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign66490_e103171, assign66490_e103171_d_n0, assign66490_e103171_d_n2, assign66490_e103171_d_n4, assign66490_e103171_d_n5, assign66490_e103171_d_n6, assign66490_e103171_d_n7, assign66490_e103171_d_n8, assign66490_e103171_d_n9, assign66490_e103171_d_n10, assign66490_e103171_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66490_e103168: f64 = (1.0 + locals.var_t4);
        let assign66490_e103169: f64 = (2.0 * assign66490_e103168);
        (assign66490_e103169, (2.0 * locals.var_t4_dn0), (2.0 * locals.var_t4_dn2), (2.0 * locals.var_t4_dn4), (2.0 * locals.var_t4_dn5), (2.0 * locals.var_t4_dn6), (2.0 * locals.var_t4_dn7), (2.0 * locals.var_t4_dn8), (2.0 * locals.var_t4_dn9), (2.0 * locals.var_t4_dn10), (2.0 * locals.var_t4_dn13),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign66490_e103171;
        locals.var_t7_dn0 = assign66490_e103171_d_n0;
        locals.var_t7_dn2 = assign66490_e103171_d_n2;
        locals.var_t7_dn4 = assign66490_e103171_d_n4;
        locals.var_t7_dn5 = assign66490_e103171_d_n5;
        locals.var_t7_dn6 = assign66490_e103171_d_n6;
        locals.var_t7_dn7 = assign66490_e103171_d_n7;
        locals.var_t7_dn8 = assign66490_e103171_d_n8;
        locals.var_t7_dn9 = assign66490_e103171_d_n9;
        locals.var_t7_dn10 = assign66490_e103171_d_n10;
        locals.var_t7_dn13 = assign66490_e103171_d_n13;
        locals.var_t7_rv = 0.0;

        let assign66500_e103175: f64 = (1e-6 + locals.var_t7);
        let assign66500_e103180: f64 = if ((locals.var_t6 < assign66500_e103175) && (locals.var_t7 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1582 = assign66500_e103180;
        locals.var_guard1582_rv = 0.0;

        let (assign66510_e103193, assign66510_e103193_d_n0, assign66510_e103193_d_n2, assign66510_e103193_d_n4, assign66510_e103193_d_n5, assign66510_e103193_d_n6, assign66510_e103193_d_n7, assign66510_e103193_d_n8, assign66510_e103193_d_n9, assign66510_e103193_d_n10, assign66510_e103193_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66510_e103189: f64 = (1e-6 + locals.var_t7);
        let assign66510_e103191: f64 = (assign66510_e103189 - locals.var_t6);
        (assign66510_e103191, (locals.var_t7_dn0 - locals.var_t6_dn0), (locals.var_t7_dn2 - locals.var_t6_dn2), (locals.var_t7_dn4 - locals.var_t6_dn4), (locals.var_t7_dn5 - locals.var_t6_dn5), (locals.var_t7_dn6 - locals.var_t6_dn6), (locals.var_t7_dn7 - locals.var_t6_dn7), (locals.var_t7_dn8 - locals.var_t6_dn8), (locals.var_t7_dn9 - locals.var_t6_dn9), (locals.var_t7_dn10 - locals.var_t6_dn10), (locals.var_t7_dn13 - locals.var_t6_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign66510_e103193;
        locals.var_tmf1_dn0 = assign66510_e103193_d_n0;
        locals.var_tmf1_dn2 = assign66510_e103193_d_n2;
        locals.var_tmf1_dn4 = assign66510_e103193_d_n4;
        locals.var_tmf1_dn5 = assign66510_e103193_d_n5;
        locals.var_tmf1_dn6 = assign66510_e103193_d_n6;
        locals.var_tmf1_dn7 = assign66510_e103193_d_n7;
        locals.var_tmf1_dn8 = assign66510_e103193_d_n8;
        locals.var_tmf1_dn9 = assign66510_e103193_d_n9;
        locals.var_tmf1_dn10 = assign66510_e103193_d_n10;
        locals.var_tmf1_dn13 = assign66510_e103193_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign66520_e103204, assign66520_e103204_d_n0, assign66520_e103204_d_n2, assign66520_e103204_d_n4, assign66520_e103204_d_n5, assign66520_e103204_d_n6, assign66520_e103204_d_n7, assign66520_e103204_d_n8, assign66520_e103204_d_n9, assign66520_e103204_d_n10, assign66520_e103204_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66520_e103202: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign66520_e103202, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign66520_e103204;
        locals.var_x2_dn0 = assign66520_e103204_d_n0;
        locals.var_x2_dn2 = assign66520_e103204_d_n2;
        locals.var_x2_dn4 = assign66520_e103204_d_n4;
        locals.var_x2_dn5 = assign66520_e103204_d_n5;
        locals.var_x2_dn6 = assign66520_e103204_d_n6;
        locals.var_x2_dn7 = assign66520_e103204_d_n7;
        locals.var_x2_dn8 = assign66520_e103204_d_n8;
        locals.var_x2_dn9 = assign66520_e103204_d_n9;
        locals.var_x2_dn10 = assign66520_e103204_d_n10;
        locals.var_x2_dn13 = assign66520_e103204_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign66530_e103215, assign66530_e103215_d_n0, assign66530_e103215_d_n2, assign66530_e103215_d_n4, assign66530_e103215_d_n5, assign66530_e103215_d_n6, assign66530_e103215_d_n7, assign66530_e103215_d_n8, assign66530_e103215_d_n9, assign66530_e103215_d_n10, assign66530_e103215_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66530_e103213: f64 = (locals.var_t7 * locals.var_t7);
        (assign66530_e103213, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign66530_e103215;
        locals.var_xmax2_dn0 = assign66530_e103215_d_n0;
        locals.var_xmax2_dn2 = assign66530_e103215_d_n2;
        locals.var_xmax2_dn4 = assign66530_e103215_d_n4;
        locals.var_xmax2_dn5 = assign66530_e103215_d_n5;
        locals.var_xmax2_dn6 = assign66530_e103215_d_n6;
        locals.var_xmax2_dn7 = assign66530_e103215_d_n7;
        locals.var_xmax2_dn8 = assign66530_e103215_d_n8;
        locals.var_xmax2_dn9 = assign66530_e103215_d_n9;
        locals.var_xmax2_dn10 = assign66530_e103215_d_n10;
        locals.var_xmax2_dn13 = assign66530_e103215_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign66540_e103224, assign66540_e103224_d_n0, assign66540_e103224_d_n2, assign66540_e103224_d_n4, assign66540_e103224_d_n5, assign66540_e103224_d_n6, assign66540_e103224_d_n7, assign66540_e103224_d_n8, assign66540_e103224_d_n9, assign66540_e103224_d_n10, assign66540_e103224_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66540_e103224;
        locals.var_xp_dn0 = assign66540_e103224_d_n0;
        locals.var_xp_dn2 = assign66540_e103224_d_n2;
        locals.var_xp_dn4 = assign66540_e103224_d_n4;
        locals.var_xp_dn5 = assign66540_e103224_d_n5;
        locals.var_xp_dn6 = assign66540_e103224_d_n6;
        locals.var_xp_dn7 = assign66540_e103224_d_n7;
        locals.var_xp_dn8 = assign66540_e103224_d_n8;
        locals.var_xp_dn9 = assign66540_e103224_d_n9;
        locals.var_xp_dn10 = assign66540_e103224_d_n10;
        locals.var_xp_dn13 = assign66540_e103224_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign66550_e103233, assign66550_e103233_d_n0, assign66550_e103233_d_n2, assign66550_e103233_d_n4, assign66550_e103233_d_n5, assign66550_e103233_d_n6, assign66550_e103233_d_n7, assign66550_e103233_d_n8, assign66550_e103233_d_n9, assign66550_e103233_d_n10, assign66550_e103233_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66550_e103233;
        locals.var_xmp_dn0 = assign66550_e103233_d_n0;
        locals.var_xmp_dn2 = assign66550_e103233_d_n2;
        locals.var_xmp_dn4 = assign66550_e103233_d_n4;
        locals.var_xmp_dn5 = assign66550_e103233_d_n5;
        locals.var_xmp_dn6 = assign66550_e103233_d_n6;
        locals.var_xmp_dn7 = assign66550_e103233_d_n7;
        locals.var_xmp_dn8 = assign66550_e103233_d_n8;
        locals.var_xmp_dn9 = assign66550_e103233_d_n9;
        locals.var_xmp_dn10 = assign66550_e103233_d_n10;
        locals.var_xmp_dn13 = assign66550_e103233_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign66560_e103242,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66560_e103242;
        locals.var_m0_rv = 0.0;

        let (assign66570_e103251,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66570_e103251;
        locals.var_mm_rv = 0.0;

        let (assign66580_e103260, assign66580_e103260_d_n0, assign66580_e103260_d_n2, assign66580_e103260_d_n4, assign66580_e103260_d_n5, assign66580_e103260_d_n6, assign66580_e103260_d_n7, assign66580_e103260_d_n8, assign66580_e103260_d_n9, assign66580_e103260_d_n10, assign66580_e103260_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign66580_e103260;
        locals.var_arg_dn0 = assign66580_e103260_d_n0;
        locals.var_arg_dn2 = assign66580_e103260_d_n2;
        locals.var_arg_dn4 = assign66580_e103260_d_n4;
        locals.var_arg_dn5 = assign66580_e103260_d_n5;
        locals.var_arg_dn6 = assign66580_e103260_d_n6;
        locals.var_arg_dn7 = assign66580_e103260_d_n7;
        locals.var_arg_dn8 = assign66580_e103260_d_n8;
        locals.var_arg_dn9 = assign66580_e103260_d_n9;
        locals.var_arg_dn10 = assign66580_e103260_d_n10;
        locals.var_arg_dn13 = assign66580_e103260_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign66590_e103269, assign66590_e103269_d_n0, assign66590_e103269_d_n2, assign66590_e103269_d_n4, assign66590_e103269_d_n5, assign66590_e103269_d_n6, assign66590_e103269_d_n7, assign66590_e103269_d_n8, assign66590_e103269_d_n9, assign66590_e103269_d_n10, assign66590_e103269_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66590_e103269;
        locals.var_dnm_dn0 = assign66590_e103269_d_n0;
        locals.var_dnm_dn2 = assign66590_e103269_d_n2;
        locals.var_dnm_dn4 = assign66590_e103269_d_n4;
        locals.var_dnm_dn5 = assign66590_e103269_d_n5;
        locals.var_dnm_dn6 = assign66590_e103269_d_n6;
        locals.var_dnm_dn7 = assign66590_e103269_d_n7;
        locals.var_dnm_dn8 = assign66590_e103269_d_n8;
        locals.var_dnm_dn9 = assign66590_e103269_d_n9;
        locals.var_dnm_dn10 = assign66590_e103269_d_n10;
        locals.var_dnm_dn13 = assign66590_e103269_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign66600_e103280, assign66600_e103280_d_n0, assign66600_e103280_d_n2, assign66600_e103280_d_n4, assign66600_e103280_d_n5, assign66600_e103280_d_n6, assign66600_e103280_d_n7, assign66600_e103280_d_n8, assign66600_e103280_d_n9, assign66600_e103280_d_n10, assign66600_e103280_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66600_e103278: f64 = (locals.var_xp * locals.var_x2);
        (assign66600_e103278, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66600_e103280;
        locals.var_xp_dn0 = assign66600_e103280_d_n0;
        locals.var_xp_dn2 = assign66600_e103280_d_n2;
        locals.var_xp_dn4 = assign66600_e103280_d_n4;
        locals.var_xp_dn5 = assign66600_e103280_d_n5;
        locals.var_xp_dn6 = assign66600_e103280_d_n6;
        locals.var_xp_dn7 = assign66600_e103280_d_n7;
        locals.var_xp_dn8 = assign66600_e103280_d_n8;
        locals.var_xp_dn9 = assign66600_e103280_d_n9;
        locals.var_xp_dn10 = assign66600_e103280_d_n10;
        locals.var_xp_dn13 = assign66600_e103280_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign66610_e103291, assign66610_e103291_d_n0, assign66610_e103291_d_n2, assign66610_e103291_d_n4, assign66610_e103291_d_n5, assign66610_e103291_d_n6, assign66610_e103291_d_n7, assign66610_e103291_d_n8, assign66610_e103291_d_n9, assign66610_e103291_d_n10, assign66610_e103291_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66610_e103289: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66610_e103289, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66610_e103291;
        locals.var_xmp_dn0 = assign66610_e103291_d_n0;
        locals.var_xmp_dn2 = assign66610_e103291_d_n2;
        locals.var_xmp_dn4 = assign66610_e103291_d_n4;
        locals.var_xmp_dn5 = assign66610_e103291_d_n5;
        locals.var_xmp_dn6 = assign66610_e103291_d_n6;
        locals.var_xmp_dn7 = assign66610_e103291_d_n7;
        locals.var_xmp_dn8 = assign66610_e103291_d_n8;
        locals.var_xmp_dn9 = assign66610_e103291_d_n9;
        locals.var_xmp_dn10 = assign66610_e103291_d_n10;
        locals.var_xmp_dn13 = assign66610_e103291_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign66620_e103302, assign66620_e103302_d_n0, assign66620_e103302_d_n2, assign66620_e103302_d_n4, assign66620_e103302_d_n5, assign66620_e103302_d_n6, assign66620_e103302_d_n7, assign66620_e103302_d_n8, assign66620_e103302_d_n9, assign66620_e103302_d_n10, assign66620_e103302_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66620_e103300: f64 = (locals.var_xp * locals.var_x2);
        (assign66620_e103300, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66620_e103302;
        locals.var_xp_dn0 = assign66620_e103302_d_n0;
        locals.var_xp_dn2 = assign66620_e103302_d_n2;
        locals.var_xp_dn4 = assign66620_e103302_d_n4;
        locals.var_xp_dn5 = assign66620_e103302_d_n5;
        locals.var_xp_dn6 = assign66620_e103302_d_n6;
        locals.var_xp_dn7 = assign66620_e103302_d_n7;
        locals.var_xp_dn8 = assign66620_e103302_d_n8;
        locals.var_xp_dn9 = assign66620_e103302_d_n9;
        locals.var_xp_dn10 = assign66620_e103302_d_n10;
        locals.var_xp_dn13 = assign66620_e103302_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign66630_e103313, assign66630_e103313_d_n0, assign66630_e103313_d_n2, assign66630_e103313_d_n4, assign66630_e103313_d_n5, assign66630_e103313_d_n6, assign66630_e103313_d_n7, assign66630_e103313_d_n8, assign66630_e103313_d_n9, assign66630_e103313_d_n10, assign66630_e103313_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66630_e103311: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66630_e103311, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66630_e103313;
        locals.var_xmp_dn0 = assign66630_e103313_d_n0;
        locals.var_xmp_dn2 = assign66630_e103313_d_n2;
        locals.var_xmp_dn4 = assign66630_e103313_d_n4;
        locals.var_xmp_dn5 = assign66630_e103313_d_n5;
        locals.var_xmp_dn6 = assign66630_e103313_d_n6;
        locals.var_xmp_dn7 = assign66630_e103313_d_n7;
        locals.var_xmp_dn8 = assign66630_e103313_d_n8;
        locals.var_xmp_dn9 = assign66630_e103313_d_n9;
        locals.var_xmp_dn10 = assign66630_e103313_d_n10;
        locals.var_xmp_dn13 = assign66630_e103313_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign66640_e103324, assign66640_e103324_d_n0, assign66640_e103324_d_n2, assign66640_e103324_d_n4, assign66640_e103324_d_n5, assign66640_e103324_d_n6, assign66640_e103324_d_n7, assign66640_e103324_d_n8, assign66640_e103324_d_n9, assign66640_e103324_d_n10, assign66640_e103324_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66640_e103322: f64 = (locals.var_xp * locals.var_x2);
        (assign66640_e103322, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66640_e103324;
        locals.var_xp_dn0 = assign66640_e103324_d_n0;
        locals.var_xp_dn2 = assign66640_e103324_d_n2;
        locals.var_xp_dn4 = assign66640_e103324_d_n4;
        locals.var_xp_dn5 = assign66640_e103324_d_n5;
        locals.var_xp_dn6 = assign66640_e103324_d_n6;
        locals.var_xp_dn7 = assign66640_e103324_d_n7;
        locals.var_xp_dn8 = assign66640_e103324_d_n8;
        locals.var_xp_dn9 = assign66640_e103324_d_n9;
        locals.var_xp_dn10 = assign66640_e103324_d_n10;
        locals.var_xp_dn13 = assign66640_e103324_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign66650_e103335, assign66650_e103335_d_n0, assign66650_e103335_d_n2, assign66650_e103335_d_n4, assign66650_e103335_d_n5, assign66650_e103335_d_n6, assign66650_e103335_d_n7, assign66650_e103335_d_n8, assign66650_e103335_d_n9, assign66650_e103335_d_n10, assign66650_e103335_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66650_e103333: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66650_e103333, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66650_e103335;
        locals.var_xmp_dn0 = assign66650_e103335_d_n0;
        locals.var_xmp_dn2 = assign66650_e103335_d_n2;
        locals.var_xmp_dn4 = assign66650_e103335_d_n4;
        locals.var_xmp_dn5 = assign66650_e103335_d_n5;
        locals.var_xmp_dn6 = assign66650_e103335_d_n6;
        locals.var_xmp_dn7 = assign66650_e103335_d_n7;
        locals.var_xmp_dn8 = assign66650_e103335_d_n8;
        locals.var_xmp_dn9 = assign66650_e103335_d_n9;
        locals.var_xmp_dn10 = assign66650_e103335_d_n10;
        locals.var_xmp_dn13 = assign66650_e103335_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign66660_e103346, assign66660_e103346_d_n0, assign66660_e103346_d_n2, assign66660_e103346_d_n4, assign66660_e103346_d_n5, assign66660_e103346_d_n6, assign66660_e103346_d_n7, assign66660_e103346_d_n8, assign66660_e103346_d_n9, assign66660_e103346_d_n10, assign66660_e103346_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66660_e103344: f64 = (locals.var_xp * locals.var_x2);
        (assign66660_e103344, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66660_e103346;
        locals.var_xp_dn0 = assign66660_e103346_d_n0;
        locals.var_xp_dn2 = assign66660_e103346_d_n2;
        locals.var_xp_dn4 = assign66660_e103346_d_n4;
        locals.var_xp_dn5 = assign66660_e103346_d_n5;
        locals.var_xp_dn6 = assign66660_e103346_d_n6;
        locals.var_xp_dn7 = assign66660_e103346_d_n7;
        locals.var_xp_dn8 = assign66660_e103346_d_n8;
        locals.var_xp_dn9 = assign66660_e103346_d_n9;
        locals.var_xp_dn10 = assign66660_e103346_d_n10;
        locals.var_xp_dn13 = assign66660_e103346_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign66670_e103357, assign66670_e103357_d_n0, assign66670_e103357_d_n2, assign66670_e103357_d_n4, assign66670_e103357_d_n5, assign66670_e103357_d_n6, assign66670_e103357_d_n7, assign66670_e103357_d_n8, assign66670_e103357_d_n9, assign66670_e103357_d_n10, assign66670_e103357_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66670_e103355: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66670_e103355, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66670_e103357;
        locals.var_xmp_dn0 = assign66670_e103357_d_n0;
        locals.var_xmp_dn2 = assign66670_e103357_d_n2;
        locals.var_xmp_dn4 = assign66670_e103357_d_n4;
        locals.var_xmp_dn5 = assign66670_e103357_d_n5;
        locals.var_xmp_dn6 = assign66670_e103357_d_n6;
        locals.var_xmp_dn7 = assign66670_e103357_d_n7;
        locals.var_xmp_dn8 = assign66670_e103357_d_n8;
        locals.var_xmp_dn9 = assign66670_e103357_d_n9;
        locals.var_xmp_dn10 = assign66670_e103357_d_n10;
        locals.var_xmp_dn13 = assign66670_e103357_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_241(
        locals: &mut StampLocals,
    ) {
        let (assign66680_e103368, assign66680_e103368_d_n0, assign66680_e103368_d_n2, assign66680_e103368_d_n4, assign66680_e103368_d_n5, assign66680_e103368_d_n6, assign66680_e103368_d_n7, assign66680_e103368_d_n8, assign66680_e103368_d_n9, assign66680_e103368_d_n10, assign66680_e103368_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66680_e103366: f64 = (locals.var_xp + locals.var_xmp);
        (assign66680_e103366, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign66680_e103368;
        locals.var_arg_dn0 = assign66680_e103368_d_n0;
        locals.var_arg_dn2 = assign66680_e103368_d_n2;
        locals.var_arg_dn4 = assign66680_e103368_d_n4;
        locals.var_arg_dn5 = assign66680_e103368_d_n5;
        locals.var_arg_dn6 = assign66680_e103368_d_n6;
        locals.var_arg_dn7 = assign66680_e103368_d_n7;
        locals.var_arg_dn8 = assign66680_e103368_d_n8;
        locals.var_arg_dn9 = assign66680_e103368_d_n9;
        locals.var_arg_dn10 = assign66680_e103368_d_n10;
        locals.var_arg_dn13 = assign66680_e103368_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign66690_e103377, assign66690_e103377_d_n0, assign66690_e103377_d_n2, assign66690_e103377_d_n4, assign66690_e103377_d_n5, assign66690_e103377_d_n6, assign66690_e103377_d_n7, assign66690_e103377_d_n8, assign66690_e103377_d_n9, assign66690_e103377_d_n10, assign66690_e103377_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66690_e103377;
        locals.var_dnm_dn0 = assign66690_e103377_d_n0;
        locals.var_dnm_dn2 = assign66690_e103377_d_n2;
        locals.var_dnm_dn4 = assign66690_e103377_d_n4;
        locals.var_dnm_dn5 = assign66690_e103377_d_n5;
        locals.var_dnm_dn6 = assign66690_e103377_d_n6;
        locals.var_dnm_dn7 = assign66690_e103377_d_n7;
        locals.var_dnm_dn8 = assign66690_e103377_d_n8;
        locals.var_dnm_dn9 = assign66690_e103377_d_n9;
        locals.var_dnm_dn10 = assign66690_e103377_d_n10;
        locals.var_dnm_dn13 = assign66690_e103377_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign66700_e103392: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1583 = assign66700_e103392;
        locals.var_guard1583_rv = 0.0;

        let assign66710_e103395: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1584 = assign66710_e103395;
        locals.var_guard1584_rv = 0.0;

        let (assign66720_e103408,) = {
    if (((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66720_e103408;
        locals.var_mm_rv = 0.0;

        let assign66730_e103411: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1585 = assign66730_e103411;
        locals.var_guard1585_rv = 0.0;

        let (assign66740_e103427,) = {
    if ((((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 == 0.0)) && (locals.var_guard1585 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66740_e103427;
        locals.var_mm_rv = 0.0;

        let assign66750_e103430: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1586 = assign66750_e103430;
        locals.var_guard1586_rv = 0.0;

        let (assign66760_e103449,) = {
    if (((((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 == 0.0)) && (locals.var_guard1585 == 0.0)) && (locals.var_guard1586 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66760_e103449;
        locals.var_mm_rv = 0.0;

        let assign66770_e103452: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1587 = assign66770_e103452;
        locals.var_guard1587_rv = 0.0;

        let (assign66780_e103474,) = {
    if ((((((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 == 0.0)) && (locals.var_guard1585 == 0.0)) && (locals.var_guard1586 == 0.0)) && (locals.var_guard1587 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66780_e103474;
        locals.var_mm_rv = 0.0;

        let (assign66790_e103485,) = {
    if ((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66790_e103485;
        locals.var_m0_rv = 0.0;

        let mut assign66800_loop_guard: usize = 0;
        while {
            let assign66800_cond_e103497: f64 = if (((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign66800_cond_e103497 != 0.0
        } {
            assign66800_loop_guard += 1;
            assert!(assign66800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign66800_body0_e103509, assign66800_body0_e103509_d_n0, assign66800_body0_e103509_d_n2, assign66800_body0_e103509_d_n4, assign66800_body0_e103509_d_n5, assign66800_body0_e103509_d_n6, assign66800_body0_e103509_d_n7, assign66800_body0_e103509_d_n8, assign66800_body0_e103509_d_n9, assign66800_body0_e103509_d_n10, assign66800_body0_e103509_d_n13,) = {
    if ((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) {
        let assign66800_body0_e103507: f64 = (locals.var_dnm).sqrt();
        (assign66800_body0_e103507, (locals.var_dnm_dn0 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn2 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn4 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn5 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn6 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn7 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn8 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn9 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn10 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn13 / (2.0 * assign66800_body0_e103507)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign66800_body0_e103509;
            locals.var_dnm_dn0 = assign66800_body0_e103509_d_n0;
            locals.var_dnm_dn2 = assign66800_body0_e103509_d_n2;
            locals.var_dnm_dn4 = assign66800_body0_e103509_d_n4;
            locals.var_dnm_dn5 = assign66800_body0_e103509_d_n5;
            locals.var_dnm_dn6 = assign66800_body0_e103509_d_n6;
            locals.var_dnm_dn7 = assign66800_body0_e103509_d_n7;
            locals.var_dnm_dn8 = assign66800_body0_e103509_d_n8;
            locals.var_dnm_dn9 = assign66800_body0_e103509_d_n9;
            locals.var_dnm_dn10 = assign66800_body0_e103509_d_n10;
            locals.var_dnm_dn13 = assign66800_body0_e103509_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign66800_body1_e103522,) = {
    if ((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) {
        let assign66800_body1_e103520: f64 = (locals.var_m0 + 1.0);
        (assign66800_body1_e103520,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign66800_body1_e103522;
            locals.var_m0_rv = 0.0;
        }

        let (assign66810_e103545, assign66810_e103545_d_n0, assign66810_e103545_d_n2, assign66810_e103545_d_n4, assign66810_e103545_d_n5, assign66810_e103545_d_n6, assign66810_e103545_d_n7, assign66810_e103545_d_n8, assign66810_e103545_d_n9, assign66810_e103545_d_n10, assign66810_e103545_d_n13,) = {
    if ((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 == 0.0)) {
        let (assign66810_e103543, assign66810_e103543_d_n0, assign66810_e103543_d_n2, assign66810_e103543_d_n4, assign66810_e103543_d_n5, assign66810_e103543_d_n6, assign66810_e103543_d_n7, assign66810_e103543_d_n8, assign66810_e103543_d_n9, assign66810_e103543_d_n10, assign66810_e103543_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign66810_e103540: f64 = (2.0 * 4.0);
                let assign66810_e103541: f64 = (1.0 / assign66810_e103540);
                let assign66810_e103542: f64 = (locals.var_dnm).powf(assign66810_e103541);
                (assign66810_e103542, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn0)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn2)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn4)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn5)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn6)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn7)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn8)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn9)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn10)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn13)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign66810_e103543, assign66810_e103543_d_n0, assign66810_e103543_d_n2, assign66810_e103543_d_n4, assign66810_e103543_d_n5, assign66810_e103543_d_n6, assign66810_e103543_d_n7, assign66810_e103543_d_n8, assign66810_e103543_d_n9, assign66810_e103543_d_n10, assign66810_e103543_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66810_e103545;
        locals.var_dnm_dn0 = assign66810_e103545_d_n0;
        locals.var_dnm_dn2 = assign66810_e103545_d_n2;
        locals.var_dnm_dn4 = assign66810_e103545_d_n4;
        locals.var_dnm_dn5 = assign66810_e103545_d_n5;
        locals.var_dnm_dn6 = assign66810_e103545_d_n6;
        locals.var_dnm_dn7 = assign66810_e103545_d_n7;
        locals.var_dnm_dn8 = assign66810_e103545_d_n8;
        locals.var_dnm_dn9 = assign66810_e103545_d_n9;
        locals.var_dnm_dn10 = assign66810_e103545_d_n10;
        locals.var_dnm_dn13 = assign66810_e103545_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign66820_e103556, assign66820_e103556_d_n0, assign66820_e103556_d_n2, assign66820_e103556_d_n4, assign66820_e103556_d_n5, assign66820_e103556_d_n6, assign66820_e103556_d_n7, assign66820_e103556_d_n8, assign66820_e103556_d_n9, assign66820_e103556_d_n10, assign66820_e103556_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66820_e103554: f64 = (1.0 / locals.var_dnm);
        (assign66820_e103554, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66820_e103556;
        locals.var_dnm_dn0 = assign66820_e103556_d_n0;
        locals.var_dnm_dn2 = assign66820_e103556_d_n2;
        locals.var_dnm_dn4 = assign66820_e103556_d_n4;
        locals.var_dnm_dn5 = assign66820_e103556_d_n5;
        locals.var_dnm_dn6 = assign66820_e103556_d_n6;
        locals.var_dnm_dn7 = assign66820_e103556_d_n7;
        locals.var_dnm_dn8 = assign66820_e103556_d_n8;
        locals.var_dnm_dn9 = assign66820_e103556_d_n9;
        locals.var_dnm_dn10 = assign66820_e103556_d_n10;
        locals.var_dnm_dn13 = assign66820_e103556_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign66830_e103569, assign66830_e103569_d_n0, assign66830_e103569_d_n2, assign66830_e103569_d_n4, assign66830_e103569_d_n5, assign66830_e103569_d_n6, assign66830_e103569_d_n7, assign66830_e103569_d_n8, assign66830_e103569_d_n9, assign66830_e103569_d_n10, assign66830_e103569_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66830_e103565: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign66830_e103567: f64 = (assign66830_e103565 * locals.var_dnm);
        (assign66830_e103567, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn13)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign66830_e103569;
        locals.var_tmf0_dn0 = assign66830_e103569_d_n0;
        locals.var_tmf0_dn2 = assign66830_e103569_d_n2;
        locals.var_tmf0_dn4 = assign66830_e103569_d_n4;
        locals.var_tmf0_dn5 = assign66830_e103569_d_n5;
        locals.var_tmf0_dn6 = assign66830_e103569_d_n6;
        locals.var_tmf0_dn7 = assign66830_e103569_d_n7;
        locals.var_tmf0_dn8 = assign66830_e103569_d_n8;
        locals.var_tmf0_dn9 = assign66830_e103569_d_n9;
        locals.var_tmf0_dn10 = assign66830_e103569_d_n10;
        locals.var_tmf0_dn13 = assign66830_e103569_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign66840_e103584, assign66840_e103584_d_n0, assign66840_e103584_d_n2, assign66840_e103584_d_n4, assign66840_e103584_d_n5, assign66840_e103584_d_n6, assign66840_e103584_d_n7, assign66840_e103584_d_n8, assign66840_e103584_d_n9, assign66840_e103584_d_n10, assign66840_e103584_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66840_e103578: f64 = (locals.var_t7 * locals.var_xmp);
        let assign66840_e103580: f64 = (assign66840_e103578 * locals.var_dnm);
        let assign66840_e103582: f64 = (assign66840_e103580 / locals.var_arg);
        (assign66840_e103582, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn0)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn2)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn4)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn5)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn6)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn7)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn8)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn9)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn10)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn13 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn13)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66840_e103584;
        locals.var_t0_dn0 = assign66840_e103584_d_n0;
        locals.var_t0_dn2 = assign66840_e103584_d_n2;
        locals.var_t0_dn4 = assign66840_e103584_d_n4;
        locals.var_t0_dn5 = assign66840_e103584_d_n5;
        locals.var_t0_dn6 = assign66840_e103584_d_n6;
        locals.var_t0_dn7 = assign66840_e103584_d_n7;
        locals.var_t0_dn8 = assign66840_e103584_d_n8;
        locals.var_t0_dn9 = assign66840_e103584_d_n9;
        locals.var_t0_dn10 = assign66840_e103584_d_n10;
        locals.var_t0_dn13 = assign66840_e103584_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign66850_e103597, assign66850_e103597_d_n0, assign66850_e103597_d_n2, assign66850_e103597_d_n4, assign66850_e103597_d_n5, assign66850_e103597_d_n6, assign66850_e103597_d_n7, assign66850_e103597_d_n8, assign66850_e103597_d_n9, assign66850_e103597_d_n10, assign66850_e103597_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66850_e103593: f64 = (1e-6 + locals.var_t7);
        let assign66850_e103595: f64 = (assign66850_e103593 - locals.var_tmf0);
        (assign66850_e103595, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign66850_e103597;
        locals.var_t6_dn0 = assign66850_e103597_d_n0;
        locals.var_t6_dn2 = assign66850_e103597_d_n2;
        locals.var_t6_dn4 = assign66850_e103597_d_n4;
        locals.var_t6_dn5 = assign66850_e103597_d_n5;
        locals.var_t6_dn6 = assign66850_e103597_d_n6;
        locals.var_t6_dn7 = assign66850_e103597_d_n7;
        locals.var_t6_dn8 = assign66850_e103597_d_n8;
        locals.var_t6_dn9 = assign66850_e103597_d_n9;
        locals.var_t6_dn10 = assign66850_e103597_d_n10;
        locals.var_t6_dn13 = assign66850_e103597_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign66860_e103606, assign66860_e103606_d_n0, assign66860_e103606_d_n2, assign66860_e103606_d_n4, assign66860_e103606_d_n5, assign66860_e103606_d_n6, assign66860_e103606_d_n7, assign66860_e103606_d_n8, assign66860_e103606_d_n9, assign66860_e103606_d_n10, assign66860_e103606_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66860_e103606;
        locals.var_t0_dn0 = assign66860_e103606_d_n0;
        locals.var_t0_dn2 = assign66860_e103606_d_n2;
        locals.var_t0_dn4 = assign66860_e103606_d_n4;
        locals.var_t0_dn5 = assign66860_e103606_d_n5;
        locals.var_t0_dn6 = assign66860_e103606_d_n6;
        locals.var_t0_dn7 = assign66860_e103606_d_n7;
        locals.var_t0_dn8 = assign66860_e103606_d_n8;
        locals.var_t0_dn9 = assign66860_e103606_d_n9;
        locals.var_t0_dn10 = assign66860_e103606_d_n10;
        locals.var_t0_dn13 = assign66860_e103606_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign66870_e103616, assign66870_e103616_d_n0, assign66870_e103616_d_n2, assign66870_e103616_d_n4, assign66870_e103616_d_n5, assign66870_e103616_d_n6, assign66870_e103616_d_n7, assign66870_e103616_d_n8, assign66870_e103616_d_n9, assign66870_e103616_d_n10, assign66870_e103616_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign66870_e103616;
        locals.var_t6_dn0 = assign66870_e103616_d_n0;
        locals.var_t6_dn2 = assign66870_e103616_d_n2;
        locals.var_t6_dn4 = assign66870_e103616_d_n4;
        locals.var_t6_dn5 = assign66870_e103616_d_n5;
        locals.var_t6_dn6 = assign66870_e103616_d_n6;
        locals.var_t6_dn7 = assign66870_e103616_d_n7;
        locals.var_t6_dn8 = assign66870_e103616_d_n8;
        locals.var_t6_dn9 = assign66870_e103616_d_n9;
        locals.var_t6_dn10 = assign66870_e103616_d_n10;
        locals.var_t6_dn13 = assign66870_e103616_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign66880_e103626, assign66880_e103626_d_n0, assign66880_e103626_d_n2, assign66880_e103626_d_n4, assign66880_e103626_d_n5, assign66880_e103626_d_n6, assign66880_e103626_d_n7, assign66880_e103626_d_n8, assign66880_e103626_d_n9, assign66880_e103626_d_n10, assign66880_e103626_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66880_e103626;
        locals.var_t0_dn0 = assign66880_e103626_d_n0;
        locals.var_t0_dn2 = assign66880_e103626_d_n2;
        locals.var_t0_dn4 = assign66880_e103626_d_n4;
        locals.var_t0_dn5 = assign66880_e103626_d_n5;
        locals.var_t0_dn6 = assign66880_e103626_d_n6;
        locals.var_t0_dn7 = assign66880_e103626_d_n7;
        locals.var_t0_dn8 = assign66880_e103626_d_n8;
        locals.var_t0_dn9 = assign66880_e103626_d_n9;
        locals.var_t0_dn10 = assign66880_e103626_d_n10;
        locals.var_t0_dn13 = assign66880_e103626_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign66890_e103634, assign66890_e103634_d_n0, assign66890_e103634_d_n2, assign66890_e103634_d_n4, assign66890_e103634_d_n5, assign66890_e103634_d_n6, assign66890_e103634_d_n7, assign66890_e103634_d_n8, assign66890_e103634_d_n9, assign66890_e103634_d_n10, assign66890_e103634_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66890_e103632: f64 = (locals.var_t6).sqrt();
        (assign66890_e103632, (locals.var_t6_dn0 / (2.0 * assign66890_e103632)), (locals.var_t6_dn2 / (2.0 * assign66890_e103632)), (locals.var_t6_dn4 / (2.0 * assign66890_e103632)), (locals.var_t6_dn5 / (2.0 * assign66890_e103632)), (locals.var_t6_dn6 / (2.0 * assign66890_e103632)), (locals.var_t6_dn7 / (2.0 * assign66890_e103632)), (locals.var_t6_dn8 / (2.0 * assign66890_e103632)), (locals.var_t6_dn9 / (2.0 * assign66890_e103632)), (locals.var_t6_dn10 / (2.0 * assign66890_e103632)), (locals.var_t6_dn13 / (2.0 * assign66890_e103632)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign66890_e103634;
        locals.var_t6_dn0 = assign66890_e103634_d_n0;
        locals.var_t6_dn2 = assign66890_e103634_d_n2;
        locals.var_t6_dn4 = assign66890_e103634_d_n4;
        locals.var_t6_dn5 = assign66890_e103634_d_n5;
        locals.var_t6_dn6 = assign66890_e103634_d_n6;
        locals.var_t6_dn7 = assign66890_e103634_d_n7;
        locals.var_t6_dn8 = assign66890_e103634_d_n8;
        locals.var_t6_dn9 = assign66890_e103634_d_n9;
        locals.var_t6_dn10 = assign66890_e103634_d_n10;
        locals.var_t6_dn13 = assign66890_e103634_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign66900_e103647, assign66900_e103647_d_n0, assign66900_e103647_d_n2, assign66900_e103647_d_n4, assign66900_e103647_d_n5, assign66900_e103647_d_n6, assign66900_e103647_d_n7, assign66900_e103647_d_n8, assign66900_e103647_d_n9, assign66900_e103647_d_n10, assign66900_e103647_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66900_e103643: f64 = (1.0 - locals.var_t6);
        let assign66900_e103644: f64 = (locals.var_t3 * assign66900_e103643);
        let assign66900_e103645: f64 = (locals.var_t1 + assign66900_e103644);
        (assign66900_e103645, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn13 + ((locals.var_t3_dn13 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn13)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn13,)
    }
};
        locals.var_psislsat = assign66900_e103647;
        locals.var_psislsat_dn0 = assign66900_e103647_d_n0;
        locals.var_psislsat_dn2 = assign66900_e103647_d_n2;
        locals.var_psislsat_dn4 = assign66900_e103647_d_n4;
        locals.var_psislsat_dn5 = assign66900_e103647_d_n5;
        locals.var_psislsat_dn6 = assign66900_e103647_d_n6;
        locals.var_psislsat_dn7 = assign66900_e103647_d_n7;
        locals.var_psislsat_dn8 = assign66900_e103647_d_n8;
        locals.var_psislsat_dn9 = assign66900_e103647_d_n9;
        locals.var_psislsat_dn10 = assign66900_e103647_d_n10;
        locals.var_psislsat_dn13 = assign66900_e103647_d_n13;
        locals.var_psislsat_rv = 0.0;

        let (assign66910_e103658, assign66910_e103658_d_n0, assign66910_e103658_d_n2, assign66910_e103658_d_n4, assign66910_e103658_d_n5, assign66910_e103658_d_n6, assign66910_e103658_d_n7, assign66910_e103658_d_n8, assign66910_e103658_d_n9, assign66910_e103658_d_n10, assign66910_e103658_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66910_e103655: f64 = (locals.var_xgate + locals.var_lgate);
        let assign66910_e103656: f64 = (locals.var_lgate / assign66910_e103655);
        (assign66910_e103656, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign66910_e103658;
        locals.var_t2_dn0 = assign66910_e103658_d_n0;
        locals.var_t2_dn2 = assign66910_e103658_d_n2;
        locals.var_t2_dn4 = assign66910_e103658_d_n4;
        locals.var_t2_dn5 = assign66910_e103658_d_n5;
        locals.var_t2_dn6 = assign66910_e103658_d_n6;
        locals.var_t2_dn7 = assign66910_e103658_d_n7;
        locals.var_t2_dn8 = assign66910_e103658_d_n8;
        locals.var_t2_dn9 = assign66910_e103658_d_n9;
        locals.var_t2_dn10 = assign66910_e103658_d_n10;
        locals.var_t2_dn13 = assign66910_e103658_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign66920_e103673, assign66920_e103673_d_n0, assign66920_e103673_d_n2, assign66920_e103673_d_n4, assign66920_e103673_d_n5, assign66920_e103673_d_n6, assign66920_e103673_d_n7, assign66920_e103673_d_n8, assign66920_e103673_d_n9, assign66920_e103673_d_n10, assign66920_e103673_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66920_e103665: f64 = (locals.var_uc_svds * locals.var_vdsz__blk439);
        let assign66920_e103667: f64 = (assign66920_e103665 + locals.var_ps0z);
        let assign66920_e103670: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign66920_e103671: f64 = (assign66920_e103667 - assign66920_e103670);
        (assign66920_e103671, (((locals.var_uc_svds * locals.var_vdsz__blk439_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn13) + locals.var_ps0z_dn13) - ((locals.var_t2_dn13 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn13))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign66920_e103673;
        locals.var_psisubsat_dn0 = assign66920_e103673_d_n0;
        locals.var_psisubsat_dn2 = assign66920_e103673_d_n2;
        locals.var_psisubsat_dn4 = assign66920_e103673_d_n4;
        locals.var_psisubsat_dn5 = assign66920_e103673_d_n5;
        locals.var_psisubsat_dn6 = assign66920_e103673_d_n6;
        locals.var_psisubsat_dn7 = assign66920_e103673_d_n7;
        locals.var_psisubsat_dn8 = assign66920_e103673_d_n8;
        locals.var_psisubsat_dn9 = assign66920_e103673_d_n9;
        locals.var_psisubsat_dn10 = assign66920_e103673_d_n10;
        locals.var_psisubsat_dn13 = assign66920_e103673_d_n13;
        locals.var_psisubsat_rv = 0.0;

        let (assign66930_e103689, assign66930_e103689_d_n0, assign66930_e103689_d_n2, assign66930_e103689_d_n4, assign66930_e103689_d_n5, assign66930_e103689_d_n6, assign66930_e103689_d_n7, assign66930_e103689_d_n8, assign66930_e103689_d_n9, assign66930_e103689_d_n10, assign66930_e103689_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66930_e103680: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign66930_e103683: f64 = (4.0 * 0.001);
        let assign66930_e103685: f64 = (assign66930_e103683 * 0.001);
        let assign66930_e103686: f64 = (assign66930_e103680 + assign66930_e103685);
        let assign66930_e103687: f64 = (assign66930_e103686).sqrt();
        (assign66930_e103687, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn13 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn13)) / (2.0 * assign66930_e103687)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign66930_e103689;
        locals.var_tmf2_dn0 = assign66930_e103689_d_n0;
        locals.var_tmf2_dn2 = assign66930_e103689_d_n2;
        locals.var_tmf2_dn4 = assign66930_e103689_d_n4;
        locals.var_tmf2_dn5 = assign66930_e103689_d_n5;
        locals.var_tmf2_dn6 = assign66930_e103689_d_n6;
        locals.var_tmf2_dn7 = assign66930_e103689_d_n7;
        locals.var_tmf2_dn8 = assign66930_e103689_d_n8;
        locals.var_tmf2_dn9 = assign66930_e103689_d_n9;
        locals.var_tmf2_dn10 = assign66930_e103689_d_n10;
        locals.var_tmf2_dn13 = assign66930_e103689_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign66940_e103702, assign66940_e103702_d_n0, assign66940_e103702_d_n2, assign66940_e103702_d_n4, assign66940_e103702_d_n5, assign66940_e103702_d_n6, assign66940_e103702_d_n7, assign66940_e103702_d_n8, assign66940_e103702_d_n9, assign66940_e103702_d_n10, assign66940_e103702_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66940_e103698: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign66940_e103699: f64 = (1.0 + assign66940_e103698);
        let assign66940_e103700: f64 = (0.5 * assign66940_e103699);
        (assign66940_e103700, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn13 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign66940_e103702;
        locals.var_t9_dn0 = assign66940_e103702_d_n0;
        locals.var_t9_dn2 = assign66940_e103702_d_n2;
        locals.var_t9_dn4 = assign66940_e103702_d_n4;
        locals.var_t9_dn5 = assign66940_e103702_d_n5;
        locals.var_t9_dn6 = assign66940_e103702_d_n6;
        locals.var_t9_dn7 = assign66940_e103702_d_n7;
        locals.var_t9_dn8 = assign66940_e103702_d_n8;
        locals.var_t9_dn9 = assign66940_e103702_d_n9;
        locals.var_t9_dn10 = assign66940_e103702_d_n10;
        locals.var_t9_dn13 = assign66940_e103702_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign66950_e103713, assign66950_e103713_d_n0, assign66950_e103713_d_n2, assign66950_e103713_d_n4, assign66950_e103713_d_n5, assign66950_e103713_d_n6, assign66950_e103713_d_n7, assign66950_e103713_d_n8, assign66950_e103713_d_n9, assign66950_e103713_d_n10, assign66950_e103713_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66950_e103710: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign66950_e103711: f64 = (0.5 * assign66950_e103710);
        (assign66950_e103711, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign66950_e103713;
        locals.var_psisubsat_dn0 = assign66950_e103713_d_n0;
        locals.var_psisubsat_dn2 = assign66950_e103713_d_n2;
        locals.var_psisubsat_dn4 = assign66950_e103713_d_n4;
        locals.var_psisubsat_dn5 = assign66950_e103713_d_n5;
        locals.var_psisubsat_dn6 = assign66950_e103713_d_n6;
        locals.var_psisubsat_dn7 = assign66950_e103713_d_n7;
        locals.var_psisubsat_dn8 = assign66950_e103713_d_n8;
        locals.var_psisubsat_dn9 = assign66950_e103713_d_n9;
        locals.var_psisubsat_dn10 = assign66950_e103713_d_n10;
        locals.var_psisubsat_dn13 = assign66950_e103713_d_n13;
        locals.var_psisubsat_rv = 0.0;

        let assign66960_e103716: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1588 = assign66960_e103716;
        locals.var_guard1588_rv = 0.0;

        let (assign66970_e103725, assign66970_e103725_d_n0, assign66970_e103725_d_n2, assign66970_e103725_d_n4, assign66970_e103725_d_n5, assign66970_e103725_d_n6, assign66970_e103725_d_n7, assign66970_e103725_d_n8, assign66970_e103725_d_n9, assign66970_e103725_d_n10, assign66970_e103725_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1588 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign66970_e103725;
        locals.var_psisubsat_dn0 = assign66970_e103725_d_n0;
        locals.var_psisubsat_dn2 = assign66970_e103725_d_n2;
        locals.var_psisubsat_dn4 = assign66970_e103725_d_n4;
        locals.var_psisubsat_dn5 = assign66970_e103725_d_n5;
        locals.var_psisubsat_dn6 = assign66970_e103725_d_n6;
        locals.var_psisubsat_dn7 = assign66970_e103725_d_n7;
        locals.var_psisubsat_dn8 = assign66970_e103725_d_n8;
        locals.var_psisubsat_dn9 = assign66970_e103725_d_n9;
        locals.var_psisubsat_dn10 = assign66970_e103725_d_n10;
        locals.var_psisubsat_dn13 = assign66970_e103725_d_n13;
        locals.var_psisubsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_242(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign66980_e103734, assign66980_e103734_d_n0, assign66980_e103734_d_n2, assign66980_e103734_d_n4, assign66980_e103734_d_n5, assign66980_e103734_d_n6, assign66980_e103734_d_n7, assign66980_e103734_d_n8, assign66980_e103734_d_n9, assign66980_e103734_d_n10, assign66980_e103734_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1588 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign66980_e103734;
        locals.var_t9_dn0 = assign66980_e103734_d_n0;
        locals.var_t9_dn2 = assign66980_e103734_d_n2;
        locals.var_t9_dn4 = assign66980_e103734_d_n4;
        locals.var_t9_dn5 = assign66980_e103734_d_n5;
        locals.var_t9_dn6 = assign66980_e103734_d_n6;
        locals.var_t9_dn7 = assign66980_e103734_d_n7;
        locals.var_t9_dn8 = assign66980_e103734_d_n8;
        locals.var_t9_dn9 = assign66980_e103734_d_n9;
        locals.var_t9_dn10 = assign66980_e103734_d_n10;
        locals.var_t9_dn13 = assign66980_e103734_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign66990_e103743, assign66990_e103743_d_n0, assign66990_e103743_d_n2, assign66990_e103743_d_n4, assign66990_e103743_d_n5, assign66990_e103743_d_n6, assign66990_e103743_d_n7, assign66990_e103743_d_n8, assign66990_e103743_d_n9, assign66990_e103743_d_n10, assign66990_e103743_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66990_e103741: f64 = (locals.var_psisubsat + 1e-25);
        (assign66990_e103741, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign66990_e103743;
        locals.var_psisubsat_dn0 = assign66990_e103743_d_n0;
        locals.var_psisubsat_dn2 = assign66990_e103743_d_n2;
        locals.var_psisubsat_dn4 = assign66990_e103743_d_n4;
        locals.var_psisubsat_dn5 = assign66990_e103743_d_n5;
        locals.var_psisubsat_dn6 = assign66990_e103743_d_n6;
        locals.var_psisubsat_dn7 = assign66990_e103743_d_n7;
        locals.var_psisubsat_dn8 = assign66990_e103743_d_n8;
        locals.var_psisubsat_dn9 = assign66990_e103743_d_n9;
        locals.var_psisubsat_dn10 = assign66990_e103743_d_n10;
        locals.var_psisubsat_dn13 = assign66990_e103743_d_n13;
        locals.var_psisubsat_rv = 0.0;

        let (assign67000_e103756, assign67000_e103756_d_n0, assign67000_e103756_d_n2, assign67000_e103756_d_n4, assign67000_e103756_d_n5, assign67000_e103756_d_n6, assign67000_e103756_d_n7, assign67000_e103756_d_n8, assign67000_e103756_d_n9, assign67000_e103756_d_n10, assign67000_e103756_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67000_e103752: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign67000_e103753: f64 = (locals.var_uc_subtmp * assign67000_e103752);
        let assign67000_e103754: f64 = (1.0 + assign67000_e103753);
        (assign67000_e103754, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn13),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
    }
};
        locals.var_xsubtmp = assign67000_e103756;
        locals.var_xsubtmp_dn0 = assign67000_e103756_d_n0;
        locals.var_xsubtmp_dn2 = assign67000_e103756_d_n2;
        locals.var_xsubtmp_dn4 = assign67000_e103756_d_n4;
        locals.var_xsubtmp_dn5 = assign67000_e103756_d_n5;
        locals.var_xsubtmp_dn6 = assign67000_e103756_d_n6;
        locals.var_xsubtmp_dn7 = assign67000_e103756_d_n7;
        locals.var_xsubtmp_dn8 = assign67000_e103756_d_n8;
        locals.var_xsubtmp_dn9 = assign67000_e103756_d_n9;
        locals.var_xsubtmp_dn10 = assign67000_e103756_d_n10;
        locals.var_xsubtmp_dn13 = assign67000_e103756_d_n13;
        locals.var_xsubtmp_rv = 0.0;

        let (assign67010_e103768, assign67010_e103768_d_n0, assign67010_e103768_d_n2, assign67010_e103768_d_n4, assign67010_e103768_d_n5, assign67010_e103768_d_n6, assign67010_e103768_d_n7, assign67010_e103768_d_n8, assign67010_e103768_d_n9, assign67010_e103768_d_n10, assign67010_e103768_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let (assign67010_e103766, assign67010_e103766_d_n0, assign67010_e103766_d_n2, assign67010_e103766_d_n4, assign67010_e103766_d_n5, assign67010_e103766_d_n6, assign67010_e103766_d_n7, assign67010_e103766_d_n8, assign67010_e103766_d_n9, assign67010_e103766_d_n10, assign67010_e103766_d_n13,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
            }
        };
        (assign67010_e103766, assign67010_e103766_d_n0, assign67010_e103766_d_n2, assign67010_e103766_d_n4, assign67010_e103766_d_n5, assign67010_e103766_d_n6, assign67010_e103766_d_n7, assign67010_e103766_d_n8, assign67010_e103766_d_n9, assign67010_e103766_d_n10, assign67010_e103766_d_n13,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
    }
};
        locals.var_xsubtmp = assign67010_e103768;
        locals.var_xsubtmp_dn0 = assign67010_e103768_d_n0;
        locals.var_xsubtmp_dn2 = assign67010_e103768_d_n2;
        locals.var_xsubtmp_dn4 = assign67010_e103768_d_n4;
        locals.var_xsubtmp_dn5 = assign67010_e103768_d_n5;
        locals.var_xsubtmp_dn6 = assign67010_e103768_d_n6;
        locals.var_xsubtmp_dn7 = assign67010_e103768_d_n7;
        locals.var_xsubtmp_dn8 = assign67010_e103768_d_n8;
        locals.var_xsubtmp_dn9 = assign67010_e103768_d_n9;
        locals.var_xsubtmp_dn10 = assign67010_e103768_d_n10;
        locals.var_xsubtmp_dn13 = assign67010_e103768_d_n13;
        locals.var_xsubtmp_rv = 0.0;

        let (assign67020_e103777, assign67020_e103777_d_n0, assign67020_e103777_d_n2, assign67020_e103777_d_n4, assign67020_e103777_d_n5, assign67020_e103777_d_n6, assign67020_e103777_d_n7, assign67020_e103777_d_n8, assign67020_e103777_d_n9, assign67020_e103777_d_n10, assign67020_e103777_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67020_e103775: f64 = (locals.var_xsub1 / locals.var_xsubtmp);
        (assign67020_e103775, (-((locals.var_xsub1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn13) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign67020_e103777;
        locals.var_t5_dn0 = assign67020_e103777_d_n0;
        locals.var_t5_dn2 = assign67020_e103777_d_n2;
        locals.var_t5_dn4 = assign67020_e103777_d_n4;
        locals.var_t5_dn5 = assign67020_e103777_d_n5;
        locals.var_t5_dn6 = assign67020_e103777_d_n6;
        locals.var_t5_dn7 = assign67020_e103777_d_n7;
        locals.var_t5_dn8 = assign67020_e103777_d_n8;
        locals.var_t5_dn9 = assign67020_e103777_d_n9;
        locals.var_t5_dn10 = assign67020_e103777_d_n10;
        locals.var_t5_dn13 = assign67020_e103777_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign67030_e103786, assign67030_e103786_d_n0, assign67030_e103786_d_n2, assign67030_e103786_d_n4, assign67030_e103786_d_n5, assign67030_e103786_d_n6, assign67030_e103786_d_n7, assign67030_e103786_d_n8, assign67030_e103786_d_n9, assign67030_e103786_d_n10, assign67030_e103786_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67030_e103784: f64 = (locals.var_xsub2 * locals.var_xsubtmp);
        (assign67030_e103784, (locals.var_xsub2 * locals.var_xsubtmp_dn0), (locals.var_xsub2 * locals.var_xsubtmp_dn2), (locals.var_xsub2 * locals.var_xsubtmp_dn4), (locals.var_xsub2 * locals.var_xsubtmp_dn5), (locals.var_xsub2 * locals.var_xsubtmp_dn6), (locals.var_xsub2 * locals.var_xsubtmp_dn7), (locals.var_xsub2 * locals.var_xsubtmp_dn8), (locals.var_xsub2 * locals.var_xsubtmp_dn9), (locals.var_xsub2 * locals.var_xsubtmp_dn10), (locals.var_xsub2 * locals.var_xsubtmp_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign67030_e103786;
        locals.var_t6_dn0 = assign67030_e103786_d_n0;
        locals.var_t6_dn2 = assign67030_e103786_d_n2;
        locals.var_t6_dn4 = assign67030_e103786_d_n4;
        locals.var_t6_dn5 = assign67030_e103786_d_n5;
        locals.var_t6_dn6 = assign67030_e103786_d_n6;
        locals.var_t6_dn7 = assign67030_e103786_d_n7;
        locals.var_t6_dn8 = assign67030_e103786_d_n8;
        locals.var_t6_dn9 = assign67030_e103786_d_n9;
        locals.var_t6_dn10 = assign67030_e103786_d_n10;
        locals.var_t6_dn13 = assign67030_e103786_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign67040_e103797, assign67040_e103797_d_n0, assign67040_e103797_d_n2, assign67040_e103797_d_n4, assign67040_e103797_d_n5, assign67040_e103797_d_n6, assign67040_e103797_d_n7, assign67040_e103797_d_n8, assign67040_e103797_d_n9, assign67040_e103797_d_n10, assign67040_e103797_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67040_e103792: f64 = (-locals.var_t6);
        let assign67040_e103794: f64 = (assign67040_e103792 / locals.var_psisubsat);
        let assign67040_e103795: f64 = (assign67040_e103794).exp();
        (assign67040_e103795, (assign67040_e103795 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn13) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn13)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67040_e103797;
        locals.var_t2_dn0 = assign67040_e103797_d_n0;
        locals.var_t2_dn2 = assign67040_e103797_d_n2;
        locals.var_t2_dn4 = assign67040_e103797_d_n4;
        locals.var_t2_dn5 = assign67040_e103797_d_n5;
        locals.var_t2_dn6 = assign67040_e103797_d_n6;
        locals.var_t2_dn7 = assign67040_e103797_d_n7;
        locals.var_t2_dn8 = assign67040_e103797_d_n8;
        locals.var_t2_dn9 = assign67040_e103797_d_n9;
        locals.var_t2_dn10 = assign67040_e103797_d_n10;
        locals.var_t2_dn13 = assign67040_e103797_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign67050_e103810, assign67050_e103810_d_n0, assign67050_e103810_d_n2, assign67050_e103810_d_n4, assign67050_e103810_d_n5, assign67050_e103810_d_n6, assign67050_e103810_d_n7, assign67050_e103810_d_n8, assign67050_e103810_d_n9, assign67050_e103810_d_n10, assign67050_e103810_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67050_e103804: f64 = (locals.var_t5 * locals.var_psisubsat);
        let assign67050_e103806: f64 = (assign67050_e103804 * locals.var_ids);
        let assign67050_e103808: f64 = (assign67050_e103806 * locals.var_t2);
        (assign67050_e103808, ((((((locals.var_t5_dn0 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn0)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn0)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn0)), ((((((locals.var_t5_dn2 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn2)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn2)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn2)), ((((((locals.var_t5_dn4 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn4)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn4)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn4)), ((((((locals.var_t5_dn5 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn5)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn5)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn5)), ((((((locals.var_t5_dn6 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn6)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn6)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn6)), ((((((locals.var_t5_dn7 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn7)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn7)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn7)), ((((((locals.var_t5_dn8 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn8)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn8)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn8)), ((((((locals.var_t5_dn9 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn9)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn9)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn9)), ((((((locals.var_t5_dn10 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn10)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn10)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn10)), ((((((locals.var_t5_dn13 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn13)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn13)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn13)),)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13,)
    }
};
        locals.var_isub = assign67050_e103810;
        locals.var_isub_dn0 = assign67050_e103810_d_n0;
        locals.var_isub_dn2 = assign67050_e103810_d_n2;
        locals.var_isub_dn4 = assign67050_e103810_d_n4;
        locals.var_isub_dn5 = assign67050_e103810_d_n5;
        locals.var_isub_dn6 = assign67050_e103810_d_n6;
        locals.var_isub_dn7 = assign67050_e103810_d_n7;
        locals.var_isub_dn8 = assign67050_e103810_d_n8;
        locals.var_isub_dn9 = assign67050_e103810_d_n9;
        locals.var_isub_dn10 = assign67050_e103810_d_n10;
        locals.var_isub_dn13 = assign67050_e103810_d_n13;
        locals.var_isub_rv = 0.0;

        let (assign67060_e103821, assign67060_e103821_d_n0, assign67060_e103821_d_n2, assign67060_e103821_d_n4, assign67060_e103821_d_n5, assign67060_e103821_d_n6, assign67060_e103821_d_n7, assign67060_e103821_d_n8, assign67060_e103821_d_n9, assign67060_e103821_d_n10, assign67060_e103821_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67060_e103817: f64 = (locals.var_t5 * locals.var_psisubsat);
        let assign67060_e103819: f64 = (assign67060_e103817 * locals.var_t2);
        (assign67060_e103819, ((((locals.var_t5_dn0 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn0)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn0)), ((((locals.var_t5_dn2 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn2)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn2)), ((((locals.var_t5_dn4 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn4)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn4)), ((((locals.var_t5_dn5 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn5)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn5)), ((((locals.var_t5_dn6 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn6)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn6)), ((((locals.var_t5_dn7 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn7)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn7)), ((((locals.var_t5_dn8 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn8)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn8)), ((((locals.var_t5_dn9 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn9)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn9)), ((((locals.var_t5_dn10 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn10)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn10)), ((((locals.var_t5_dn13 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn13)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn13)),)
    } else {
        (locals.var_wk_ii, locals.var_wk_ii_dn0, locals.var_wk_ii_dn2, locals.var_wk_ii_dn4, locals.var_wk_ii_dn5, locals.var_wk_ii_dn6, locals.var_wk_ii_dn7, locals.var_wk_ii_dn8, locals.var_wk_ii_dn9, locals.var_wk_ii_dn10, locals.var_wk_ii_dn13,)
    }
};
        locals.var_wk_ii = assign67060_e103821;
        locals.var_wk_ii_dn0 = assign67060_e103821_d_n0;
        locals.var_wk_ii_dn2 = assign67060_e103821_d_n2;
        locals.var_wk_ii_dn4 = assign67060_e103821_d_n4;
        locals.var_wk_ii_dn5 = assign67060_e103821_d_n5;
        locals.var_wk_ii_dn6 = assign67060_e103821_d_n6;
        locals.var_wk_ii_dn7 = assign67060_e103821_d_n7;
        locals.var_wk_ii_dn8 = assign67060_e103821_d_n8;
        locals.var_wk_ii_dn9 = assign67060_e103821_d_n9;
        locals.var_wk_ii_dn10 = assign67060_e103821_d_n10;
        locals.var_wk_ii_dn13 = assign67060_e103821_d_n13;
        locals.var_wk_ii_rv = 0.0;

        let (assign67070_e103829, assign67070_e103829_d_n0, assign67070_e103829_d_n2, assign67070_e103829_d_n4, assign67070_e103829_d_n5, assign67070_e103829_d_n6, assign67070_e103829_d_n7, assign67070_e103829_d_n8, assign67070_e103829_d_n9, assign67070_e103829_d_n10, assign67070_e103829_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13,)
    }
};
        locals.var_isub = assign67070_e103829;
        locals.var_isub_dn0 = assign67070_e103829_d_n0;
        locals.var_isub_dn2 = assign67070_e103829_d_n2;
        locals.var_isub_dn4 = assign67070_e103829_d_n4;
        locals.var_isub_dn5 = assign67070_e103829_d_n5;
        locals.var_isub_dn6 = assign67070_e103829_d_n6;
        locals.var_isub_dn7 = assign67070_e103829_d_n7;
        locals.var_isub_dn8 = assign67070_e103829_d_n8;
        locals.var_isub_dn9 = assign67070_e103829_d_n9;
        locals.var_isub_dn10 = assign67070_e103829_d_n10;
        locals.var_isub_dn13 = assign67070_e103829_d_n13;
        locals.var_isub_rv = 0.0;

        let assign67080_e103832: f64 = if locals.var_uc_subld1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1589 = assign67080_e103832;
        locals.var_guard1589_rv = 0.0;

        let (assign67090_e103839, assign67090_e103839_d_n0, assign67090_e103839_d_n2, assign67090_e103839_d_n4, assign67090_e103839_d_n5, assign67090_e103839_d_n6, assign67090_e103839_d_n7, assign67090_e103839_d_n8, assign67090_e103839_d_n9, assign67090_e103839_d_n10, assign67090_e103839_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        (locals.var_vddp, locals.var_vddp_dn0, 0.0, 0.0, locals.var_vddp_dn5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67090_e103839;
        locals.var_t0_dn0 = assign67090_e103839_d_n0;
        locals.var_t0_dn2 = assign67090_e103839_d_n2;
        locals.var_t0_dn4 = assign67090_e103839_d_n4;
        locals.var_t0_dn5 = assign67090_e103839_d_n5;
        locals.var_t0_dn6 = assign67090_e103839_d_n6;
        locals.var_t0_dn7 = assign67090_e103839_d_n7;
        locals.var_t0_dn8 = assign67090_e103839_d_n8;
        locals.var_t0_dn9 = assign67090_e103839_d_n9;
        locals.var_t0_dn10 = assign67090_e103839_d_n10;
        locals.var_t0_dn13 = assign67090_e103839_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign67100_e103855, assign67100_e103855_d_n0, assign67100_e103855_d_n2, assign67100_e103855_d_n4, assign67100_e103855_d_n5, assign67100_e103855_d_n6, assign67100_e103855_d_n7, assign67100_e103855_d_n8, assign67100_e103855_d_n9, assign67100_e103855_d_n10, assign67100_e103855_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67100_e103846: f64 = (locals.var_t0 * locals.var_t0);
        let assign67100_e103849: f64 = (4.0 * 1e-6);
        let assign67100_e103851: f64 = (assign67100_e103849 * 1e-6);
        let assign67100_e103852: f64 = (assign67100_e103846 + assign67100_e103851);
        let assign67100_e103853: f64 = (assign67100_e103852).sqrt();
        (assign67100_e103853, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign67100_e103853)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign67100_e103855;
        locals.var_tmf2_dn0 = assign67100_e103855_d_n0;
        locals.var_tmf2_dn2 = assign67100_e103855_d_n2;
        locals.var_tmf2_dn4 = assign67100_e103855_d_n4;
        locals.var_tmf2_dn5 = assign67100_e103855_d_n5;
        locals.var_tmf2_dn6 = assign67100_e103855_d_n6;
        locals.var_tmf2_dn7 = assign67100_e103855_d_n7;
        locals.var_tmf2_dn8 = assign67100_e103855_d_n8;
        locals.var_tmf2_dn9 = assign67100_e103855_d_n9;
        locals.var_tmf2_dn10 = assign67100_e103855_d_n10;
        locals.var_tmf2_dn13 = assign67100_e103855_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign67110_e103868, assign67110_e103868_d_n0, assign67110_e103868_d_n2, assign67110_e103868_d_n4, assign67110_e103868_d_n5, assign67110_e103868_d_n6, assign67110_e103868_d_n7, assign67110_e103868_d_n8, assign67110_e103868_d_n9, assign67110_e103868_d_n10, assign67110_e103868_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67110_e103864: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign67110_e103865: f64 = (1.0 + assign67110_e103864);
        let assign67110_e103866: f64 = (0.5 * assign67110_e103865);
        (assign67110_e103866, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67110_e103868;
        locals.var_t1_dn0 = assign67110_e103868_d_n0;
        locals.var_t1_dn2 = assign67110_e103868_d_n2;
        locals.var_t1_dn4 = assign67110_e103868_d_n4;
        locals.var_t1_dn5 = assign67110_e103868_d_n5;
        locals.var_t1_dn6 = assign67110_e103868_d_n6;
        locals.var_t1_dn7 = assign67110_e103868_d_n7;
        locals.var_t1_dn8 = assign67110_e103868_d_n8;
        locals.var_t1_dn9 = assign67110_e103868_d_n9;
        locals.var_t1_dn10 = assign67110_e103868_d_n10;
        locals.var_t1_dn13 = assign67110_e103868_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign67120_e103879, assign67120_e103879_d_n0, assign67120_e103879_d_n2, assign67120_e103879_d_n4, assign67120_e103879_d_n5, assign67120_e103879_d_n6, assign67120_e103879_d_n7, assign67120_e103879_d_n8, assign67120_e103879_d_n9, assign67120_e103879_d_n10, assign67120_e103879_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67120_e103876: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign67120_e103877: f64 = (0.5 * assign67120_e103876);
        (assign67120_e103877, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67120_e103879;
        locals.var_t0_dn0 = assign67120_e103879_d_n0;
        locals.var_t0_dn2 = assign67120_e103879_d_n2;
        locals.var_t0_dn4 = assign67120_e103879_d_n4;
        locals.var_t0_dn5 = assign67120_e103879_d_n5;
        locals.var_t0_dn6 = assign67120_e103879_d_n6;
        locals.var_t0_dn7 = assign67120_e103879_d_n7;
        locals.var_t0_dn8 = assign67120_e103879_d_n8;
        locals.var_t0_dn9 = assign67120_e103879_d_n9;
        locals.var_t0_dn10 = assign67120_e103879_d_n10;
        locals.var_t0_dn13 = assign67120_e103879_d_n13;
        locals.var_t0_rv = 0.0;

        let assign67130_e103882: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1590 = assign67130_e103882;
        locals.var_guard1590_rv = 0.0;

        let (assign67140_e103891, assign67140_e103891_d_n0, assign67140_e103891_d_n2, assign67140_e103891_d_n4, assign67140_e103891_d_n5, assign67140_e103891_d_n6, assign67140_e103891_d_n7, assign67140_e103891_d_n8, assign67140_e103891_d_n9, assign67140_e103891_d_n10, assign67140_e103891_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1590 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67140_e103891;
        locals.var_t0_dn0 = assign67140_e103891_d_n0;
        locals.var_t0_dn2 = assign67140_e103891_d_n2;
        locals.var_t0_dn4 = assign67140_e103891_d_n4;
        locals.var_t0_dn5 = assign67140_e103891_d_n5;
        locals.var_t0_dn6 = assign67140_e103891_d_n6;
        locals.var_t0_dn7 = assign67140_e103891_d_n7;
        locals.var_t0_dn8 = assign67140_e103891_d_n8;
        locals.var_t0_dn9 = assign67140_e103891_d_n9;
        locals.var_t0_dn10 = assign67140_e103891_d_n10;
        locals.var_t0_dn13 = assign67140_e103891_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign67150_e103900, assign67150_e103900_d_n0, assign67150_e103900_d_n2, assign67150_e103900_d_n4, assign67150_e103900_d_n5, assign67150_e103900_d_n6, assign67150_e103900_d_n7, assign67150_e103900_d_n8, assign67150_e103900_d_n9, assign67150_e103900_d_n10, assign67150_e103900_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1590 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67150_e103900;
        locals.var_t1_dn0 = assign67150_e103900_d_n0;
        locals.var_t1_dn2 = assign67150_e103900_d_n2;
        locals.var_t1_dn4 = assign67150_e103900_d_n4;
        locals.var_t1_dn5 = assign67150_e103900_d_n5;
        locals.var_t1_dn6 = assign67150_e103900_d_n6;
        locals.var_t1_dn7 = assign67150_e103900_d_n7;
        locals.var_t1_dn8 = assign67150_e103900_d_n8;
        locals.var_t1_dn9 = assign67150_e103900_d_n9;
        locals.var_t1_dn10 = assign67150_e103900_d_n10;
        locals.var_t1_dn13 = assign67150_e103900_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign67160_e103910, assign67160_e103910_d_n0, assign67160_e103910_d_n2, assign67160_e103910_d_n4, assign67160_e103910_d_n5, assign67160_e103910_d_n6, assign67160_e103910_d_n7, assign67160_e103910_d_n8, assign67160_e103910_d_n9, assign67160_e103910_d_n10, assign67160_e103910_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67160_e103907: f64 = (locals.var_vgvt + 1e-25);
        let assign67160_e103908: f64 = (assign67160_e103907).sqrt();
        (assign67160_e103908, (locals.var_vgvt_dn0 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn2 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn4 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn5 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn6 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn7 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn8 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn9 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn10 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn13 / (2.0 * assign67160_e103908)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67160_e103910;
        locals.var_t1_dn0 = assign67160_e103910_d_n0;
        locals.var_t1_dn2 = assign67160_e103910_d_n2;
        locals.var_t1_dn4 = assign67160_e103910_d_n4;
        locals.var_t1_dn5 = assign67160_e103910_d_n5;
        locals.var_t1_dn6 = assign67160_e103910_d_n6;
        locals.var_t1_dn7 = assign67160_e103910_d_n7;
        locals.var_t1_dn8 = assign67160_e103910_d_n8;
        locals.var_t1_dn9 = assign67160_e103910_d_n9;
        locals.var_t1_dn10 = assign67160_e103910_d_n10;
        locals.var_t1_dn13 = assign67160_e103910_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign67170_e103921, assign67170_e103921_d_n0, assign67170_e103921_d_n2, assign67170_e103921_d_n4, assign67170_e103921_d_n5, assign67170_e103921_d_n6, assign67170_e103921_d_n7, assign67170_e103921_d_n8, assign67170_e103921_d_n9, assign67170_e103921_d_n10, assign67170_e103921_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67170_e103918: f64 = (2.0 * locals.var_t1);
        let assign67170_e103919: f64 = (1.0 / assign67170_e103918);
        (assign67170_e103919, (-((2.0 * locals.var_t1_dn0) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn2) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn4) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn5) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn6) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn7) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn8) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn9) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn10) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn13) / (assign67170_e103918 * assign67170_e103918))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign67170_e103921;
        locals.var_t3_dn0 = assign67170_e103921_d_n0;
        locals.var_t3_dn2 = assign67170_e103921_d_n2;
        locals.var_t3_dn4 = assign67170_e103921_d_n4;
        locals.var_t3_dn5 = assign67170_e103921_d_n5;
        locals.var_t3_dn6 = assign67170_e103921_d_n6;
        locals.var_t3_dn7 = assign67170_e103921_d_n7;
        locals.var_t3_dn8 = assign67170_e103921_d_n8;
        locals.var_t3_dn9 = assign67170_e103921_d_n9;
        locals.var_t3_dn10 = assign67170_e103921_d_n10;
        locals.var_t3_dn13 = assign67170_e103921_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign67180_e103936, assign67180_e103936_d_n0, assign67180_e103936_d_n2, assign67180_e103936_d_n4, assign67180_e103936_d_n5, assign67180_e103936_d_n6, assign67180_e103936_d_n7, assign67180_e103936_d_n8, assign67180_e103936_d_n9, assign67180_e103936_d_n10, assign67180_e103936_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67180_e103931: f64 = (p.p106 * locals.var_vgs);
        let assign67180_e103932: f64 = (1.0 + assign67180_e103931);
        let assign67180_e103933: f64 = (p.p105 * assign67180_e103932);
        let assign67180_e103934: f64 = (locals.var_t0 - assign67180_e103933);
        (assign67180_e103934, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, (locals.var_t0_dn5 - (p.p105 * (p.p106 * locals.var_vgs_dn5))), (locals.var_t0_dn6 - (p.p105 * (p.p106 * locals.var_vgs_dn6))), (locals.var_t0_dn7 - (p.p105 * (p.p106 * locals.var_vgs_dn7))), locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67180_e103936;
        locals.var_t4_dn0 = assign67180_e103936_d_n0;
        locals.var_t4_dn2 = assign67180_e103936_d_n2;
        locals.var_t4_dn4 = assign67180_e103936_d_n4;
        locals.var_t4_dn5 = assign67180_e103936_d_n5;
        locals.var_t4_dn6 = assign67180_e103936_d_n6;
        locals.var_t4_dn7 = assign67180_e103936_d_n7;
        locals.var_t4_dn8 = assign67180_e103936_d_n8;
        locals.var_t4_dn9 = assign67180_e103936_d_n9;
        locals.var_t4_dn10 = assign67180_e103936_d_n10;
        locals.var_t4_dn13 = assign67180_e103936_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign67190_e103952, assign67190_e103952_d_n0, assign67190_e103952_d_n2, assign67190_e103952_d_n4, assign67190_e103952_d_n5, assign67190_e103952_d_n6, assign67190_e103952_d_n7, assign67190_e103952_d_n8, assign67190_e103952_d_n9, assign67190_e103952_d_n10, assign67190_e103952_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67190_e103943: f64 = (locals.var_t4 * locals.var_t4);
        let assign67190_e103946: f64 = (4.0 * 0.01);
        let assign67190_e103948: f64 = (assign67190_e103946 * 0.01);
        let assign67190_e103949: f64 = (assign67190_e103943 + assign67190_e103948);
        let assign67190_e103950: f64 = (assign67190_e103949).sqrt();
        (assign67190_e103950, (((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn13 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn13)) / (2.0 * assign67190_e103950)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign67190_e103952;
        locals.var_tmf2_dn0 = assign67190_e103952_d_n0;
        locals.var_tmf2_dn2 = assign67190_e103952_d_n2;
        locals.var_tmf2_dn4 = assign67190_e103952_d_n4;
        locals.var_tmf2_dn5 = assign67190_e103952_d_n5;
        locals.var_tmf2_dn6 = assign67190_e103952_d_n6;
        locals.var_tmf2_dn7 = assign67190_e103952_d_n7;
        locals.var_tmf2_dn8 = assign67190_e103952_d_n8;
        locals.var_tmf2_dn9 = assign67190_e103952_d_n9;
        locals.var_tmf2_dn10 = assign67190_e103952_d_n10;
        locals.var_tmf2_dn13 = assign67190_e103952_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign67200_e103965, assign67200_e103965_d_n0, assign67200_e103965_d_n2, assign67200_e103965_d_n4, assign67200_e103965_d_n5, assign67200_e103965_d_n6, assign67200_e103965_d_n7, assign67200_e103965_d_n8, assign67200_e103965_d_n9, assign67200_e103965_d_n10, assign67200_e103965_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67200_e103961: f64 = (locals.var_t4 / locals.var_tmf2);
        let assign67200_e103962: f64 = (1.0 + assign67200_e103961);
        let assign67200_e103963: f64 = (0.5 * assign67200_e103962);
        (assign67200_e103963, (0.5 * (((locals.var_t4_dn0 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn2 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn4 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn5 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn6 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn7 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn8 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn9 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn10 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn13 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign67200_e103965;
        locals.var_t9_dn0 = assign67200_e103965_d_n0;
        locals.var_t9_dn2 = assign67200_e103965_d_n2;
        locals.var_t9_dn4 = assign67200_e103965_d_n4;
        locals.var_t9_dn5 = assign67200_e103965_d_n5;
        locals.var_t9_dn6 = assign67200_e103965_d_n6;
        locals.var_t9_dn7 = assign67200_e103965_d_n7;
        locals.var_t9_dn8 = assign67200_e103965_d_n8;
        locals.var_t9_dn9 = assign67200_e103965_d_n9;
        locals.var_t9_dn10 = assign67200_e103965_d_n10;
        locals.var_t9_dn13 = assign67200_e103965_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign67210_e103976, assign67210_e103976_d_n0, assign67210_e103976_d_n2, assign67210_e103976_d_n4, assign67210_e103976_d_n5, assign67210_e103976_d_n6, assign67210_e103976_d_n7, assign67210_e103976_d_n8, assign67210_e103976_d_n9, assign67210_e103976_d_n10, assign67210_e103976_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67210_e103973: f64 = (locals.var_t4 + locals.var_tmf2);
        let assign67210_e103974: f64 = (0.5 * assign67210_e103973);
        (assign67210_e103974, (0.5 * (locals.var_t4_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t4_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t4_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t4_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t4_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t4_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t4_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t4_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t4_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t4_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67210_e103976;
        locals.var_t4_dn0 = assign67210_e103976_d_n0;
        locals.var_t4_dn2 = assign67210_e103976_d_n2;
        locals.var_t4_dn4 = assign67210_e103976_d_n4;
        locals.var_t4_dn5 = assign67210_e103976_d_n5;
        locals.var_t4_dn6 = assign67210_e103976_d_n6;
        locals.var_t4_dn7 = assign67210_e103976_d_n7;
        locals.var_t4_dn8 = assign67210_e103976_d_n8;
        locals.var_t4_dn9 = assign67210_e103976_d_n9;
        locals.var_t4_dn10 = assign67210_e103976_d_n10;
        locals.var_t4_dn13 = assign67210_e103976_d_n13;
        locals.var_t4_rv = 0.0;

        let assign67220_e103979: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1591 = assign67220_e103979;
        locals.var_guard1591_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_243(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67230_e103988, assign67230_e103988_d_n0, assign67230_e103988_d_n2, assign67230_e103988_d_n4, assign67230_e103988_d_n5, assign67230_e103988_d_n6, assign67230_e103988_d_n7, assign67230_e103988_d_n8, assign67230_e103988_d_n9, assign67230_e103988_d_n10, assign67230_e103988_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1591 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67230_e103988;
        locals.var_t4_dn0 = assign67230_e103988_d_n0;
        locals.var_t4_dn2 = assign67230_e103988_d_n2;
        locals.var_t4_dn4 = assign67230_e103988_d_n4;
        locals.var_t4_dn5 = assign67230_e103988_d_n5;
        locals.var_t4_dn6 = assign67230_e103988_d_n6;
        locals.var_t4_dn7 = assign67230_e103988_d_n7;
        locals.var_t4_dn8 = assign67230_e103988_d_n8;
        locals.var_t4_dn9 = assign67230_e103988_d_n9;
        locals.var_t4_dn10 = assign67230_e103988_d_n10;
        locals.var_t4_dn13 = assign67230_e103988_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign67240_e103997, assign67240_e103997_d_n0, assign67240_e103997_d_n2, assign67240_e103997_d_n4, assign67240_e103997_d_n5, assign67240_e103997_d_n6, assign67240_e103997_d_n7, assign67240_e103997_d_n8, assign67240_e103997_d_n9, assign67240_e103997_d_n10, assign67240_e103997_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1591 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign67240_e103997;
        locals.var_t9_dn0 = assign67240_e103997_d_n0;
        locals.var_t9_dn2 = assign67240_e103997_d_n2;
        locals.var_t9_dn4 = assign67240_e103997_d_n4;
        locals.var_t9_dn5 = assign67240_e103997_d_n5;
        locals.var_t9_dn6 = assign67240_e103997_d_n6;
        locals.var_t9_dn7 = assign67240_e103997_d_n7;
        locals.var_t9_dn8 = assign67240_e103997_d_n8;
        locals.var_t9_dn9 = assign67240_e103997_d_n9;
        locals.var_t9_dn10 = assign67240_e103997_d_n10;
        locals.var_t9_dn13 = assign67240_e103997_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign67250_e104006, assign67250_e104006_d_n0, assign67250_e104006_d_n2, assign67250_e104006_d_n4, assign67250_e104006_d_n5, assign67250_e104006_d_n6, assign67250_e104006_d_n7, assign67250_e104006_d_n8, assign67250_e104006_d_n9, assign67250_e104006_d_n10, assign67250_e104006_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67250_e104004: f64 = (locals.var_t4 + 1e-25);
        (assign67250_e104004, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67250_e104006;
        locals.var_t4_dn0 = assign67250_e104006_d_n0;
        locals.var_t4_dn2 = assign67250_e104006_d_n2;
        locals.var_t4_dn4 = assign67250_e104006_d_n4;
        locals.var_t4_dn5 = assign67250_e104006_d_n5;
        locals.var_t4_dn6 = assign67250_e104006_d_n6;
        locals.var_t4_dn7 = assign67250_e104006_d_n7;
        locals.var_t4_dn8 = assign67250_e104006_d_n8;
        locals.var_t4_dn9 = assign67250_e104006_d_n9;
        locals.var_t4_dn10 = assign67250_e104006_d_n10;
        locals.var_t4_dn13 = assign67250_e104006_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign67260_e104021, assign67260_e104021_d_n0, assign67260_e104021_d_n2, assign67260_e104021_d_n4, assign67260_e104021_d_n5, assign67260_e104021_d_n6, assign67260_e104021_d_n7, assign67260_e104021_d_n8, assign67260_e104021_d_n9, assign67260_e104021_d_n10, assign67260_e104021_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67260_e104013: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign67260_e104015: f64 = (-1.0);
        let assign67260_e104017: f64 = (assign67260_e104015 / locals.var_t4);
        let assign67260_e104018: f64 = (assign67260_e104017).exp();
        let assign67260_e104019: f64 = (assign67260_e104013 * assign67260_e104018);
        (assign67260_e104019, (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn0) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn2) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn13) / (locals.var_t4 * locals.var_t4))))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign67260_e104021;
        locals.var_t10_dn0 = assign67260_e104021_d_n0;
        locals.var_t10_dn2 = assign67260_e104021_d_n2;
        locals.var_t10_dn4 = assign67260_e104021_d_n4;
        locals.var_t10_dn5 = assign67260_e104021_d_n5;
        locals.var_t10_dn6 = assign67260_e104021_d_n6;
        locals.var_t10_dn7 = assign67260_e104021_d_n7;
        locals.var_t10_dn8 = assign67260_e104021_d_n8;
        locals.var_t10_dn9 = assign67260_e104021_d_n9;
        locals.var_t10_dn10 = assign67260_e104021_d_n10;
        locals.var_t10_dn13 = assign67260_e104021_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign67270_e104034, assign67270_e104034_d_n0, assign67270_e104034_d_n2, assign67270_e104034_d_n4, assign67270_e104034_d_n5, assign67270_e104034_d_n6, assign67270_e104034_d_n7, assign67270_e104034_d_n8, assign67270_e104034_d_n9, assign67270_e104034_d_n10, assign67270_e104034_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67270_e104030: f64 = (1.0 / locals.var_t4);
        let assign67270_e104031: f64 = (1.0 + assign67270_e104030);
        let assign67270_e104032: f64 = (locals.var_t10 * assign67270_e104031);
        (assign67270_e104032, ((locals.var_t10_dn0 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn2 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn4 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn5 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn6 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn7 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn8 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn9 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn10 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn13 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))))),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign67270_e104034;
        locals.var_t11_dn0 = assign67270_e104034_d_n0;
        locals.var_t11_dn2 = assign67270_e104034_d_n2;
        locals.var_t11_dn4 = assign67270_e104034_d_n4;
        locals.var_t11_dn5 = assign67270_e104034_d_n5;
        locals.var_t11_dn6 = assign67270_e104034_d_n6;
        locals.var_t11_dn7 = assign67270_e104034_d_n7;
        locals.var_t11_dn8 = assign67270_e104034_d_n8;
        locals.var_t11_dn9 = assign67270_e104034_d_n9;
        locals.var_t11_dn10 = assign67270_e104034_d_n10;
        locals.var_t11_dn13 = assign67270_e104034_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign67280_e104043, assign67280_e104043_d_n0, assign67280_e104043_d_n2, assign67280_e104043_d_n4, assign67280_e104043_d_n5, assign67280_e104043_d_n6, assign67280_e104043_d_n7, assign67280_e104043_d_n8, assign67280_e104043_d_n9, assign67280_e104043_d_n10, assign67280_e104043_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67280_e104041: f64 = (locals.var_t4 * locals.var_t10);
        (assign67280_e104041, ((locals.var_t4_dn0 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn0)), ((locals.var_t4_dn2 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn2)), ((locals.var_t4_dn4 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn4)), ((locals.var_t4_dn5 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn5)), ((locals.var_t4_dn6 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn6)), ((locals.var_t4_dn7 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn7)), ((locals.var_t4_dn8 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn8)), ((locals.var_t4_dn9 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn9)), ((locals.var_t4_dn10 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn10)), ((locals.var_t4_dn13 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign67280_e104043;
        locals.var_t3_dn0 = assign67280_e104043_d_n0;
        locals.var_t3_dn2 = assign67280_e104043_d_n2;
        locals.var_t3_dn4 = assign67280_e104043_d_n4;
        locals.var_t3_dn5 = assign67280_e104043_d_n5;
        locals.var_t3_dn6 = assign67280_e104043_d_n6;
        locals.var_t3_dn7 = assign67280_e104043_d_n7;
        locals.var_t3_dn8 = assign67280_e104043_d_n8;
        locals.var_t3_dn9 = assign67280_e104043_d_n9;
        locals.var_t3_dn10 = assign67280_e104043_d_n10;
        locals.var_t3_dn13 = assign67280_e104043_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign67290_e104052, assign67290_e104052_d_n0, assign67290_e104052_d_n2, assign67290_e104052_d_n4, assign67290_e104052_d_n5, assign67290_e104052_d_n6, assign67290_e104052_d_n7, assign67290_e104052_d_n8, assign67290_e104052_d_n9, assign67290_e104052_d_n10, assign67290_e104052_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67290_e104050: f64 = (locals.var_t0 - locals.var_t3);
        (assign67290_e104050, (locals.var_t0_dn0 - locals.var_t3_dn0), (locals.var_t0_dn2 - locals.var_t3_dn2), (locals.var_t0_dn4 - locals.var_t3_dn4), (locals.var_t0_dn5 - locals.var_t3_dn5), (locals.var_t0_dn6 - locals.var_t3_dn6), (locals.var_t0_dn7 - locals.var_t3_dn7), (locals.var_t0_dn8 - locals.var_t3_dn8), (locals.var_t0_dn9 - locals.var_t3_dn9), (locals.var_t0_dn10 - locals.var_t3_dn10), (locals.var_t0_dn13 - locals.var_t3_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67290_e104052;
        locals.var_t0_dn0 = assign67290_e104052_d_n0;
        locals.var_t0_dn2 = assign67290_e104052_d_n2;
        locals.var_t0_dn4 = assign67290_e104052_d_n4;
        locals.var_t0_dn5 = assign67290_e104052_d_n5;
        locals.var_t0_dn6 = assign67290_e104052_d_n6;
        locals.var_t0_dn7 = assign67290_e104052_d_n7;
        locals.var_t0_dn8 = assign67290_e104052_d_n8;
        locals.var_t0_dn9 = assign67290_e104052_d_n9;
        locals.var_t0_dn10 = assign67290_e104052_d_n10;
        locals.var_t0_dn13 = assign67290_e104052_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign67300_e104068, assign67300_e104068_d_n0, assign67300_e104068_d_n2, assign67300_e104068_d_n4, assign67300_e104068_d_n5, assign67300_e104068_d_n6, assign67300_e104068_d_n7, assign67300_e104068_d_n8, assign67300_e104068_d_n9, assign67300_e104068_d_n10, assign67300_e104068_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67300_e104059: f64 = (locals.var_t0 * locals.var_t0);
        let assign67300_e104062: f64 = (4.0 * 0.01);
        let assign67300_e104064: f64 = (assign67300_e104062 * 0.01);
        let assign67300_e104065: f64 = (assign67300_e104059 + assign67300_e104064);
        let assign67300_e104066: f64 = (assign67300_e104065).sqrt();
        (assign67300_e104066, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign67300_e104066)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign67300_e104068;
        locals.var_tmf2_dn0 = assign67300_e104068_d_n0;
        locals.var_tmf2_dn2 = assign67300_e104068_d_n2;
        locals.var_tmf2_dn4 = assign67300_e104068_d_n4;
        locals.var_tmf2_dn5 = assign67300_e104068_d_n5;
        locals.var_tmf2_dn6 = assign67300_e104068_d_n6;
        locals.var_tmf2_dn7 = assign67300_e104068_d_n7;
        locals.var_tmf2_dn8 = assign67300_e104068_d_n8;
        locals.var_tmf2_dn9 = assign67300_e104068_d_n9;
        locals.var_tmf2_dn10 = assign67300_e104068_d_n10;
        locals.var_tmf2_dn13 = assign67300_e104068_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign67310_e104081, assign67310_e104081_d_n0, assign67310_e104081_d_n2, assign67310_e104081_d_n4, assign67310_e104081_d_n5, assign67310_e104081_d_n6, assign67310_e104081_d_n7, assign67310_e104081_d_n8, assign67310_e104081_d_n9, assign67310_e104081_d_n10, assign67310_e104081_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67310_e104077: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign67310_e104078: f64 = (1.0 + assign67310_e104077);
        let assign67310_e104079: f64 = (0.5 * assign67310_e104078);
        (assign67310_e104079, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign67310_e104081;
        locals.var_t9_dn0 = assign67310_e104081_d_n0;
        locals.var_t9_dn2 = assign67310_e104081_d_n2;
        locals.var_t9_dn4 = assign67310_e104081_d_n4;
        locals.var_t9_dn5 = assign67310_e104081_d_n5;
        locals.var_t9_dn6 = assign67310_e104081_d_n6;
        locals.var_t9_dn7 = assign67310_e104081_d_n7;
        locals.var_t9_dn8 = assign67310_e104081_d_n8;
        locals.var_t9_dn9 = assign67310_e104081_d_n9;
        locals.var_t9_dn10 = assign67310_e104081_d_n10;
        locals.var_t9_dn13 = assign67310_e104081_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign67320_e104092, assign67320_e104092_d_n0, assign67320_e104092_d_n2, assign67320_e104092_d_n4, assign67320_e104092_d_n5, assign67320_e104092_d_n6, assign67320_e104092_d_n7, assign67320_e104092_d_n8, assign67320_e104092_d_n9, assign67320_e104092_d_n10, assign67320_e104092_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67320_e104089: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign67320_e104090: f64 = (0.5 * assign67320_e104089);
        (assign67320_e104090, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67320_e104092;
        locals.var_t0_dn0 = assign67320_e104092_d_n0;
        locals.var_t0_dn2 = assign67320_e104092_d_n2;
        locals.var_t0_dn4 = assign67320_e104092_d_n4;
        locals.var_t0_dn5 = assign67320_e104092_d_n5;
        locals.var_t0_dn6 = assign67320_e104092_d_n6;
        locals.var_t0_dn7 = assign67320_e104092_d_n7;
        locals.var_t0_dn8 = assign67320_e104092_d_n8;
        locals.var_t0_dn9 = assign67320_e104092_d_n9;
        locals.var_t0_dn10 = assign67320_e104092_d_n10;
        locals.var_t0_dn13 = assign67320_e104092_d_n13;
        locals.var_t0_rv = 0.0;

        let assign67330_e104095: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1592 = assign67330_e104095;
        locals.var_guard1592_rv = 0.0;

        let (assign67340_e104104, assign67340_e104104_d_n0, assign67340_e104104_d_n2, assign67340_e104104_d_n4, assign67340_e104104_d_n5, assign67340_e104104_d_n6, assign67340_e104104_d_n7, assign67340_e104104_d_n8, assign67340_e104104_d_n9, assign67340_e104104_d_n10, assign67340_e104104_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1592 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67340_e104104;
        locals.var_t0_dn0 = assign67340_e104104_d_n0;
        locals.var_t0_dn2 = assign67340_e104104_d_n2;
        locals.var_t0_dn4 = assign67340_e104104_d_n4;
        locals.var_t0_dn5 = assign67340_e104104_d_n5;
        locals.var_t0_dn6 = assign67340_e104104_d_n6;
        locals.var_t0_dn7 = assign67340_e104104_d_n7;
        locals.var_t0_dn8 = assign67340_e104104_d_n8;
        locals.var_t0_dn9 = assign67340_e104104_d_n9;
        locals.var_t0_dn10 = assign67340_e104104_d_n10;
        locals.var_t0_dn13 = assign67340_e104104_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign67350_e104113, assign67350_e104113_d_n0, assign67350_e104113_d_n2, assign67350_e104113_d_n4, assign67350_e104113_d_n5, assign67350_e104113_d_n6, assign67350_e104113_d_n7, assign67350_e104113_d_n8, assign67350_e104113_d_n9, assign67350_e104113_d_n10, assign67350_e104113_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1592 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign67350_e104113;
        locals.var_t9_dn0 = assign67350_e104113_d_n0;
        locals.var_t9_dn2 = assign67350_e104113_d_n2;
        locals.var_t9_dn4 = assign67350_e104113_d_n4;
        locals.var_t9_dn5 = assign67350_e104113_d_n5;
        locals.var_t9_dn6 = assign67350_e104113_d_n6;
        locals.var_t9_dn7 = assign67350_e104113_d_n7;
        locals.var_t9_dn8 = assign67350_e104113_d_n8;
        locals.var_t9_dn9 = assign67350_e104113_d_n9;
        locals.var_t9_dn10 = assign67350_e104113_d_n10;
        locals.var_t9_dn13 = assign67350_e104113_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign67360_e104122, assign67360_e104122_d_n0, assign67360_e104122_d_n2, assign67360_e104122_d_n4, assign67360_e104122_d_n5, assign67360_e104122_d_n6, assign67360_e104122_d_n7, assign67360_e104122_d_n8, assign67360_e104122_d_n9, assign67360_e104122_d_n10, assign67360_e104122_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67360_e104120: f64 = (locals.var_t0 + 1e-25);
        (assign67360_e104120, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67360_e104122;
        locals.var_t0_dn0 = assign67360_e104122_d_n0;
        locals.var_t0_dn2 = assign67360_e104122_d_n2;
        locals.var_t0_dn4 = assign67360_e104122_d_n4;
        locals.var_t0_dn5 = assign67360_e104122_d_n5;
        locals.var_t0_dn6 = assign67360_e104122_d_n6;
        locals.var_t0_dn7 = assign67360_e104122_d_n7;
        locals.var_t0_dn8 = assign67360_e104122_d_n8;
        locals.var_t0_dn9 = assign67360_e104122_d_n9;
        locals.var_t0_dn10 = assign67360_e104122_d_n10;
        locals.var_t0_dn13 = assign67360_e104122_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign67370_e104133, assign67370_e104133_d_n0, assign67370_e104133_d_n2, assign67370_e104133_d_n4, assign67370_e104133_d_n5, assign67370_e104133_d_n6, assign67370_e104133_d_n7, assign67370_e104133_d_n8, assign67370_e104133_d_n9, assign67370_e104133_d_n10, assign67370_e104133_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67370_e104130: f64 = (locals.var_t0 * locals.var_t1);
        let assign67370_e104131: f64 = (1.0 / assign67370_e104130);
        (assign67370_e104131, (-(((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn13 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn13)) / (assign67370_e104130 * assign67370_e104130))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67370_e104133;
        locals.var_t4_dn0 = assign67370_e104133_d_n0;
        locals.var_t4_dn2 = assign67370_e104133_d_n2;
        locals.var_t4_dn4 = assign67370_e104133_d_n4;
        locals.var_t4_dn5 = assign67370_e104133_d_n5;
        locals.var_t4_dn6 = assign67370_e104133_d_n6;
        locals.var_t4_dn7 = assign67370_e104133_d_n7;
        locals.var_t4_dn8 = assign67370_e104133_d_n8;
        locals.var_t4_dn9 = assign67370_e104133_d_n9;
        locals.var_t4_dn10 = assign67370_e104133_d_n10;
        locals.var_t4_dn13 = assign67370_e104133_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign67380_e104142, assign67380_e104142_d_n0, assign67380_e104142_d_n2, assign67380_e104142_d_n4, assign67380_e104142_d_n5, assign67380_e104142_d_n6, assign67380_e104142_d_n7, assign67380_e104142_d_n8, assign67380_e104142_d_n9, assign67380_e104142_d_n10, assign67380_e104142_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67380_e104140: f64 = (locals.var_ldrift0 * locals.var_mks_subld2);
        (assign67380_e104140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign67380_e104142;
        locals.var_t7_dn0 = assign67380_e104142_d_n0;
        locals.var_t7_dn2 = assign67380_e104142_d_n2;
        locals.var_t7_dn4 = assign67380_e104142_d_n4;
        locals.var_t7_dn5 = assign67380_e104142_d_n5;
        locals.var_t7_dn6 = assign67380_e104142_d_n6;
        locals.var_t7_dn7 = assign67380_e104142_d_n7;
        locals.var_t7_dn8 = assign67380_e104142_d_n8;
        locals.var_t7_dn9 = assign67380_e104142_d_n9;
        locals.var_t7_dn10 = assign67380_e104142_d_n10;
        locals.var_t7_dn13 = assign67380_e104142_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign67390_e104153, assign67390_e104153_d_n0, assign67390_e104153_d_n2, assign67390_e104153_d_n4, assign67390_e104153_d_n5, assign67390_e104153_d_n6, assign67390_e104153_d_n7, assign67390_e104153_d_n8, assign67390_e104153_d_n9, assign67390_e104153_d_n10, assign67390_e104153_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67390_e104148: f64 = (-locals.var_t7);
        let assign67390_e104150: f64 = (assign67390_e104148 * locals.var_t4);
        let assign67390_e104151: f64 = (assign67390_e104150).exp();
        (assign67390_e104151, (assign67390_e104151 * (((-locals.var_t7_dn0) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn0))), (assign67390_e104151 * (((-locals.var_t7_dn2) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn2))), (assign67390_e104151 * (((-locals.var_t7_dn4) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn4))), (assign67390_e104151 * (((-locals.var_t7_dn5) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn5))), (assign67390_e104151 * (((-locals.var_t7_dn6) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn6))), (assign67390_e104151 * (((-locals.var_t7_dn7) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn7))), (assign67390_e104151 * (((-locals.var_t7_dn8) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn8))), (assign67390_e104151 * (((-locals.var_t7_dn9) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn9))), (assign67390_e104151 * (((-locals.var_t7_dn10) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn10))), (assign67390_e104151 * (((-locals.var_t7_dn13) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67390_e104153;
        locals.var_t2_dn0 = assign67390_e104153_d_n0;
        locals.var_t2_dn2 = assign67390_e104153_d_n2;
        locals.var_t2_dn4 = assign67390_e104153_d_n4;
        locals.var_t2_dn5 = assign67390_e104153_d_n5;
        locals.var_t2_dn6 = assign67390_e104153_d_n6;
        locals.var_t2_dn7 = assign67390_e104153_d_n7;
        locals.var_t2_dn8 = assign67390_e104153_d_n8;
        locals.var_t2_dn9 = assign67390_e104153_d_n9;
        locals.var_t2_dn10 = assign67390_e104153_d_n10;
        locals.var_t2_dn13 = assign67390_e104153_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign67400_e104166, assign67400_e104166_d_n0, assign67400_e104166_d_n2, assign67400_e104166_d_n4, assign67400_e104166_d_n5, assign67400_e104166_d_n6, assign67400_e104166_d_n7, assign67400_e104166_d_n8, assign67400_e104166_d_n9, assign67400_e104166_d_n10, assign67400_e104166_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67400_e104160: f64 = (locals.var_t7 * locals.var_t2);
        let assign67400_e104162: f64 = (assign67400_e104160 * locals.var_t4);
        let assign67400_e104164: f64 = (assign67400_e104162 * locals.var_t4);
        (assign67400_e104164, ((((((locals.var_t7_dn0 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn0)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn0)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn0)), ((((((locals.var_t7_dn2 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn2)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn2)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn2)), ((((((locals.var_t7_dn4 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn4)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn4)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn4)), ((((((locals.var_t7_dn5 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn5)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn5)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn5)), ((((((locals.var_t7_dn6 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn6)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn6)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn6)), ((((((locals.var_t7_dn7 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn7)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn7)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn7)), ((((((locals.var_t7_dn8 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn8)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn8)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn8)), ((((((locals.var_t7_dn9 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn9)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn9)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn9)), ((((((locals.var_t7_dn10 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn10)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn10)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn10)), ((((((locals.var_t7_dn13 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn13)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn13)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign67400_e104166;
        locals.var_t6_dn0 = assign67400_e104166_d_n0;
        locals.var_t6_dn2 = assign67400_e104166_d_n2;
        locals.var_t6_dn4 = assign67400_e104166_d_n4;
        locals.var_t6_dn5 = assign67400_e104166_d_n5;
        locals.var_t6_dn6 = assign67400_e104166_d_n6;
        locals.var_t6_dn7 = assign67400_e104166_d_n7;
        locals.var_t6_dn8 = assign67400_e104166_d_n8;
        locals.var_t6_dn9 = assign67400_e104166_d_n9;
        locals.var_t6_dn10 = assign67400_e104166_d_n10;
        locals.var_t6_dn13 = assign67400_e104166_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign67410_e104179, assign67410_e104179_d_n0, assign67410_e104179_d_n2, assign67410_e104179_d_n4, assign67410_e104179_d_n5, assign67410_e104179_d_n6, assign67410_e104179_d_n7, assign67410_e104179_d_n8, assign67410_e104179_d_n9, assign67410_e104179_d_n10, assign67410_e104179_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67410_e104173: f64 = (locals.var_uc_subld1 * locals.var_ids);
        let assign67410_e104175: f64 = (assign67410_e104173 * locals.var_t0);
        let assign67410_e104177: f64 = (assign67410_e104175 * locals.var_t2);
        (assign67410_e104177, (((((locals.var_uc_subld1 * locals.var_ids_dn0) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn0)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn0)), (((((locals.var_uc_subld1 * locals.var_ids_dn2) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn2)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn2)), (((((locals.var_uc_subld1 * locals.var_ids_dn4) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn4)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn4)), (((((locals.var_uc_subld1 * locals.var_ids_dn5) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn5)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn5)), (((((locals.var_uc_subld1 * locals.var_ids_dn6) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn6)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn6)), (((((locals.var_uc_subld1 * locals.var_ids_dn7) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn7)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn7)), (((((locals.var_uc_subld1 * locals.var_ids_dn8) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn8)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn8)), (((((locals.var_uc_subld1 * locals.var_ids_dn9) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn9)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn9)), (((((locals.var_uc_subld1 * locals.var_ids_dn10) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn10)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn10)), (((((locals.var_uc_subld1 * locals.var_ids_dn13) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn13)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn13)),)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn13,)
    }
};
        locals.var_isubld = assign67410_e104179;
        locals.var_isubld_dn0 = assign67410_e104179_d_n0;
        locals.var_isubld_dn2 = assign67410_e104179_d_n2;
        locals.var_isubld_dn4 = assign67410_e104179_d_n4;
        locals.var_isubld_dn5 = assign67410_e104179_d_n5;
        locals.var_isubld_dn6 = assign67410_e104179_d_n6;
        locals.var_isubld_dn7 = assign67410_e104179_d_n7;
        locals.var_isubld_dn8 = assign67410_e104179_d_n8;
        locals.var_isubld_dn9 = assign67410_e104179_d_n9;
        locals.var_isubld_dn10 = assign67410_e104179_d_n10;
        locals.var_isubld_dn13 = assign67410_e104179_d_n13;
        locals.var_isubld_rv = 0.0;

        let assign67420_e104182: f64 = if p.p45 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1593 = assign67420_e104182;
        locals.var_guard1593_rv = 0.0;

        let (assign67430_e104186, assign67430_e104186_d_n0, assign67430_e104186_d_n2, assign67430_e104186_d_n4, assign67430_e104186_d_n5, assign67430_e104186_d_n6, assign67430_e104186_d_n7, assign67430_e104186_d_n8, assign67430_e104186_d_n9, assign67430_e104186_d_n10, assign67430_e104186_d_n13,) = {
    if (locals.var_guard1593 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn13,)
    }
};
        locals.var_ibreakhe = assign67430_e104186;
        locals.var_ibreakhe_dn0 = assign67430_e104186_d_n0;
        locals.var_ibreakhe_dn2 = assign67430_e104186_d_n2;
        locals.var_ibreakhe_dn4 = assign67430_e104186_d_n4;
        locals.var_ibreakhe_dn5 = assign67430_e104186_d_n5;
        locals.var_ibreakhe_dn6 = assign67430_e104186_d_n6;
        locals.var_ibreakhe_dn7 = assign67430_e104186_d_n7;
        locals.var_ibreakhe_dn8 = assign67430_e104186_d_n8;
        locals.var_ibreakhe_dn9 = assign67430_e104186_d_n9;
        locals.var_ibreakhe_dn10 = assign67430_e104186_d_n10;
        locals.var_ibreakhe_dn13 = assign67430_e104186_d_n13;
        locals.var_ibreakhe_rv = 0.0;

        let assign67440_e104190: f64 = (locals.var_vgse - p.p446);
        let assign67440_e104191: f64 = (p.p45 * assign67440_e104190);
        let assign67440_e104193: f64 = if assign67440_e104191 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1594 = assign67440_e104193;
        locals.var_guard1594_rv = 0.0;

        let (assign67450_e104200, assign67450_e104200_d_n0, assign67450_e104200_d_n2, assign67450_e104200_d_n4, assign67450_e104200_d_n5, assign67450_e104200_d_n6, assign67450_e104200_d_n7, assign67450_e104200_d_n8, assign67450_e104200_d_n9, assign67450_e104200_d_n10, assign67450_e104200_d_n13,) = {
    if ((locals.var_guard1593 == 0.0) && (locals.var_guard1594 != 0.0)) {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn13,)
    } else {
        (locals.var_hbdv, locals.var_hbdv_dn0, locals.var_hbdv_dn2, locals.var_hbdv_dn4, locals.var_hbdv_dn5, locals.var_hbdv_dn6, locals.var_hbdv_dn7, locals.var_hbdv_dn8, locals.var_hbdv_dn9, locals.var_hbdv_dn10, locals.var_hbdv_dn13,)
    }
};
        locals.var_hbdv = assign67450_e104200;
        locals.var_hbdv_dn0 = assign67450_e104200_d_n0;
        locals.var_hbdv_dn2 = assign67450_e104200_d_n2;
        locals.var_hbdv_dn4 = assign67450_e104200_d_n4;
        locals.var_hbdv_dn5 = assign67450_e104200_d_n5;
        locals.var_hbdv_dn6 = assign67450_e104200_d_n6;
        locals.var_hbdv_dn7 = assign67450_e104200_d_n7;
        locals.var_hbdv_dn8 = assign67450_e104200_d_n8;
        locals.var_hbdv_dn9 = assign67450_e104200_d_n9;
        locals.var_hbdv_dn10 = assign67450_e104200_d_n10;
        locals.var_hbdv_dn13 = assign67450_e104200_d_n13;
        locals.var_hbdv_rv = 0.0;

        let (assign67460_e104216, assign67460_e104216_d_n0, assign67460_e104216_d_n2, assign67460_e104216_d_n4, assign67460_e104216_d_n5, assign67460_e104216_d_n6, assign67460_e104216_d_n7, assign67460_e104216_d_n8, assign67460_e104216_d_n9, assign67460_e104216_d_n10, assign67460_e104216_d_n13,) = {
    if ((locals.var_guard1593 == 0.0) && (locals.var_guard1594 == 0.0)) {
        let assign67460_e104209: f64 = (locals.var_vgse - p.p446);
        let assign67460_e104211: f64 = (assign67460_e104209).powf(2.0);
        let assign67460_e104212: f64 = (p.p445 * assign67460_e104211);
        let assign67460_e104214: f64 = (assign67460_e104212 + locals.var_hbdceff);
        (assign67460_e104214, ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67460_e104209).powf(2.0 - 1.0) * locals.var_vgse_dn0)) } } else { (assign67460_e104211 * (2.0 * (locals.var_vgse_dn0 / assign67460_e104209))) }) + locals.var_hbdceff_dn0), ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67460_e104209).powf(2.0 - 1.0) * locals.var_vgse_dn2)) } } else { (assign67460_e104211 * (2.0 * (locals.var_vgse_dn2 / assign67460_e104209))) }) + locals.var_hbdceff_dn2), locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67460_e104209).powf(2.0 - 1.0) * locals.var_vgse_dn6)) } } else { (assign67460_e104211 * (2.0 * (locals.var_vgse_dn6 / assign67460_e104209))) }) + locals.var_hbdceff_dn6), locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn13,)
    } else {
        (locals.var_hbdv, locals.var_hbdv_dn0, locals.var_hbdv_dn2, locals.var_hbdv_dn4, locals.var_hbdv_dn5, locals.var_hbdv_dn6, locals.var_hbdv_dn7, locals.var_hbdv_dn8, locals.var_hbdv_dn9, locals.var_hbdv_dn10, locals.var_hbdv_dn13,)
    }
};
        locals.var_hbdv = assign67460_e104216;
        locals.var_hbdv_dn0 = assign67460_e104216_d_n0;
        locals.var_hbdv_dn2 = assign67460_e104216_d_n2;
        locals.var_hbdv_dn4 = assign67460_e104216_d_n4;
        locals.var_hbdv_dn5 = assign67460_e104216_d_n5;
        locals.var_hbdv_dn6 = assign67460_e104216_d_n6;
        locals.var_hbdv_dn7 = assign67460_e104216_d_n7;
        locals.var_hbdv_dn8 = assign67460_e104216_d_n8;
        locals.var_hbdv_dn9 = assign67460_e104216_d_n9;
        locals.var_hbdv_dn10 = assign67460_e104216_d_n10;
        locals.var_hbdv_dn13 = assign67460_e104216_d_n13;
        locals.var_hbdv_rv = 0.0;

        let (assign67470_e104228, assign67470_e104228_d_n0, assign67470_e104228_d_n2, assign67470_e104228_d_n4, assign67470_e104228_d_n5, assign67470_e104228_d_n6, assign67470_e104228_d_n7, assign67470_e104228_d_n8, assign67470_e104228_d_n9, assign67470_e104228_d_n10, assign67470_e104228_d_n13,) = {
    if (locals.var_guard1593 == 0.0) {
        let assign67470_e104223: f64 = (locals.var_vdse - locals.var_hbdv);
        let assign67470_e104224: f64 = (locals.var_beta * assign67470_e104223);
        let assign67470_e104225: f64 = { let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign67470_e104226: f64 = (p.p449 * assign67470_e104225);
        (assign67470_e104226, (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn0 * assign67470_e104223) + (locals.var_beta * (locals.var_vdse_dn0 - locals.var_hbdv_dn0))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn2 * assign67470_e104223) + (locals.var_beta * (locals.var_vdse_dn2 - locals.var_hbdv_dn2))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn4 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn4))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn5 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn5))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn6 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn6))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn7 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn7))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn8 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn8))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn9 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn9))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn10 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn10))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn13 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn13))))),)
    } else {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn13,)
    }
};
        locals.var_ibreakhe = assign67470_e104228;
        locals.var_ibreakhe_dn0 = assign67470_e104228_d_n0;
        locals.var_ibreakhe_dn2 = assign67470_e104228_d_n2;
        locals.var_ibreakhe_dn4 = assign67470_e104228_d_n4;
        locals.var_ibreakhe_dn5 = assign67470_e104228_d_n5;
        locals.var_ibreakhe_dn6 = assign67470_e104228_d_n6;
        locals.var_ibreakhe_dn7 = assign67470_e104228_d_n7;
        locals.var_ibreakhe_dn8 = assign67470_e104228_d_n8;
        locals.var_ibreakhe_dn9 = assign67470_e104228_d_n9;
        locals.var_ibreakhe_dn10 = assign67470_e104228_d_n10;
        locals.var_ibreakhe_dn13 = assign67470_e104228_d_n13;
        locals.var_ibreakhe_rv = 0.0;

        let assign67480_e104231: f64 = if locals.var_ibreakhe > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1595 = assign67480_e104231;
        locals.var_guard1595_rv = 0.0;

        let assign67490_e104235: f64 = (100000.0 - 50000.0);
        let assign67490_e104240: f64 = if ((locals.var_ibreakhe > assign67490_e104235) && (50000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1596 = assign67490_e104240;
        locals.var_guard1596_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_244(
        locals: &mut StampLocals,
    ) {
        let (assign67500_e104250, assign67500_e104250_d_n0, assign67500_e104250_d_n2, assign67500_e104250_d_n4, assign67500_e104250_d_n5, assign67500_e104250_d_n6, assign67500_e104250_d_n7, assign67500_e104250_d_n8, assign67500_e104250_d_n9, assign67500_e104250_d_n10, assign67500_e104250_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67500_e104246: f64 = (locals.var_ibreakhe - 100000.0);
        let assign67500_e104248: f64 = (assign67500_e104246 + 50000.0);
        (assign67500_e104248, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign67500_e104250;
        locals.var_tmf1_dn0 = assign67500_e104250_d_n0;
        locals.var_tmf1_dn2 = assign67500_e104250_d_n2;
        locals.var_tmf1_dn4 = assign67500_e104250_d_n4;
        locals.var_tmf1_dn5 = assign67500_e104250_d_n5;
        locals.var_tmf1_dn6 = assign67500_e104250_d_n6;
        locals.var_tmf1_dn7 = assign67500_e104250_d_n7;
        locals.var_tmf1_dn8 = assign67500_e104250_d_n8;
        locals.var_tmf1_dn9 = assign67500_e104250_d_n9;
        locals.var_tmf1_dn10 = assign67500_e104250_d_n10;
        locals.var_tmf1_dn13 = assign67500_e104250_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign67510_e104258, assign67510_e104258_d_n0, assign67510_e104258_d_n2, assign67510_e104258_d_n4, assign67510_e104258_d_n5, assign67510_e104258_d_n6, assign67510_e104258_d_n7, assign67510_e104258_d_n8, assign67510_e104258_d_n9, assign67510_e104258_d_n10, assign67510_e104258_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67510_e104256: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign67510_e104256, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign67510_e104258;
        locals.var_x2_dn0 = assign67510_e104258_d_n0;
        locals.var_x2_dn2 = assign67510_e104258_d_n2;
        locals.var_x2_dn4 = assign67510_e104258_d_n4;
        locals.var_x2_dn5 = assign67510_e104258_d_n5;
        locals.var_x2_dn6 = assign67510_e104258_d_n6;
        locals.var_x2_dn7 = assign67510_e104258_d_n7;
        locals.var_x2_dn8 = assign67510_e104258_d_n8;
        locals.var_x2_dn9 = assign67510_e104258_d_n9;
        locals.var_x2_dn10 = assign67510_e104258_d_n10;
        locals.var_x2_dn13 = assign67510_e104258_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign67520_e104266, assign67520_e104266_d_n0, assign67520_e104266_d_n2, assign67520_e104266_d_n4, assign67520_e104266_d_n5, assign67520_e104266_d_n6, assign67520_e104266_d_n7, assign67520_e104266_d_n8, assign67520_e104266_d_n9, assign67520_e104266_d_n10, assign67520_e104266_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67520_e104264: f64 = (50000.0 * 50000.0);
        (assign67520_e104264, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign67520_e104266;
        locals.var_xmax2_dn0 = assign67520_e104266_d_n0;
        locals.var_xmax2_dn2 = assign67520_e104266_d_n2;
        locals.var_xmax2_dn4 = assign67520_e104266_d_n4;
        locals.var_xmax2_dn5 = assign67520_e104266_d_n5;
        locals.var_xmax2_dn6 = assign67520_e104266_d_n6;
        locals.var_xmax2_dn7 = assign67520_e104266_d_n7;
        locals.var_xmax2_dn8 = assign67520_e104266_d_n8;
        locals.var_xmax2_dn9 = assign67520_e104266_d_n9;
        locals.var_xmax2_dn10 = assign67520_e104266_d_n10;
        locals.var_xmax2_dn13 = assign67520_e104266_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign67530_e104272, assign67530_e104272_d_n0, assign67530_e104272_d_n2, assign67530_e104272_d_n4, assign67530_e104272_d_n5, assign67530_e104272_d_n6, assign67530_e104272_d_n7, assign67530_e104272_d_n8, assign67530_e104272_d_n9, assign67530_e104272_d_n10, assign67530_e104272_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign67530_e104272;
        locals.var_xp_dn0 = assign67530_e104272_d_n0;
        locals.var_xp_dn2 = assign67530_e104272_d_n2;
        locals.var_xp_dn4 = assign67530_e104272_d_n4;
        locals.var_xp_dn5 = assign67530_e104272_d_n5;
        locals.var_xp_dn6 = assign67530_e104272_d_n6;
        locals.var_xp_dn7 = assign67530_e104272_d_n7;
        locals.var_xp_dn8 = assign67530_e104272_d_n8;
        locals.var_xp_dn9 = assign67530_e104272_d_n9;
        locals.var_xp_dn10 = assign67530_e104272_d_n10;
        locals.var_xp_dn13 = assign67530_e104272_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign67540_e104278, assign67540_e104278_d_n0, assign67540_e104278_d_n2, assign67540_e104278_d_n4, assign67540_e104278_d_n5, assign67540_e104278_d_n6, assign67540_e104278_d_n7, assign67540_e104278_d_n8, assign67540_e104278_d_n9, assign67540_e104278_d_n10, assign67540_e104278_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign67540_e104278;
        locals.var_xmp_dn0 = assign67540_e104278_d_n0;
        locals.var_xmp_dn2 = assign67540_e104278_d_n2;
        locals.var_xmp_dn4 = assign67540_e104278_d_n4;
        locals.var_xmp_dn5 = assign67540_e104278_d_n5;
        locals.var_xmp_dn6 = assign67540_e104278_d_n6;
        locals.var_xmp_dn7 = assign67540_e104278_d_n7;
        locals.var_xmp_dn8 = assign67540_e104278_d_n8;
        locals.var_xmp_dn9 = assign67540_e104278_d_n9;
        locals.var_xmp_dn10 = assign67540_e104278_d_n10;
        locals.var_xmp_dn13 = assign67540_e104278_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign67550_e104284,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign67550_e104284;
        locals.var_m0_rv = 0.0;

        let (assign67560_e104290,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67560_e104290;
        locals.var_mm_rv = 0.0;

        let (assign67570_e104296, assign67570_e104296_d_n0, assign67570_e104296_d_n2, assign67570_e104296_d_n4, assign67570_e104296_d_n5, assign67570_e104296_d_n6, assign67570_e104296_d_n7, assign67570_e104296_d_n8, assign67570_e104296_d_n9, assign67570_e104296_d_n10, assign67570_e104296_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign67570_e104296;
        locals.var_arg_dn0 = assign67570_e104296_d_n0;
        locals.var_arg_dn2 = assign67570_e104296_d_n2;
        locals.var_arg_dn4 = assign67570_e104296_d_n4;
        locals.var_arg_dn5 = assign67570_e104296_d_n5;
        locals.var_arg_dn6 = assign67570_e104296_d_n6;
        locals.var_arg_dn7 = assign67570_e104296_d_n7;
        locals.var_arg_dn8 = assign67570_e104296_d_n8;
        locals.var_arg_dn9 = assign67570_e104296_d_n9;
        locals.var_arg_dn10 = assign67570_e104296_d_n10;
        locals.var_arg_dn13 = assign67570_e104296_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign67580_e104302, assign67580_e104302_d_n0, assign67580_e104302_d_n2, assign67580_e104302_d_n4, assign67580_e104302_d_n5, assign67580_e104302_d_n6, assign67580_e104302_d_n7, assign67580_e104302_d_n8, assign67580_e104302_d_n9, assign67580_e104302_d_n10, assign67580_e104302_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign67580_e104302;
        locals.var_dnm_dn0 = assign67580_e104302_d_n0;
        locals.var_dnm_dn2 = assign67580_e104302_d_n2;
        locals.var_dnm_dn4 = assign67580_e104302_d_n4;
        locals.var_dnm_dn5 = assign67580_e104302_d_n5;
        locals.var_dnm_dn6 = assign67580_e104302_d_n6;
        locals.var_dnm_dn7 = assign67580_e104302_d_n7;
        locals.var_dnm_dn8 = assign67580_e104302_d_n8;
        locals.var_dnm_dn9 = assign67580_e104302_d_n9;
        locals.var_dnm_dn10 = assign67580_e104302_d_n10;
        locals.var_dnm_dn13 = assign67580_e104302_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign67590_e104310, assign67590_e104310_d_n0, assign67590_e104310_d_n2, assign67590_e104310_d_n4, assign67590_e104310_d_n5, assign67590_e104310_d_n6, assign67590_e104310_d_n7, assign67590_e104310_d_n8, assign67590_e104310_d_n9, assign67590_e104310_d_n10, assign67590_e104310_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67590_e104308: f64 = (locals.var_xp * locals.var_x2);
        (assign67590_e104308, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign67590_e104310;
        locals.var_xp_dn0 = assign67590_e104310_d_n0;
        locals.var_xp_dn2 = assign67590_e104310_d_n2;
        locals.var_xp_dn4 = assign67590_e104310_d_n4;
        locals.var_xp_dn5 = assign67590_e104310_d_n5;
        locals.var_xp_dn6 = assign67590_e104310_d_n6;
        locals.var_xp_dn7 = assign67590_e104310_d_n7;
        locals.var_xp_dn8 = assign67590_e104310_d_n8;
        locals.var_xp_dn9 = assign67590_e104310_d_n9;
        locals.var_xp_dn10 = assign67590_e104310_d_n10;
        locals.var_xp_dn13 = assign67590_e104310_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign67600_e104318, assign67600_e104318_d_n0, assign67600_e104318_d_n2, assign67600_e104318_d_n4, assign67600_e104318_d_n5, assign67600_e104318_d_n6, assign67600_e104318_d_n7, assign67600_e104318_d_n8, assign67600_e104318_d_n9, assign67600_e104318_d_n10, assign67600_e104318_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67600_e104316: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign67600_e104316, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign67600_e104318;
        locals.var_xmp_dn0 = assign67600_e104318_d_n0;
        locals.var_xmp_dn2 = assign67600_e104318_d_n2;
        locals.var_xmp_dn4 = assign67600_e104318_d_n4;
        locals.var_xmp_dn5 = assign67600_e104318_d_n5;
        locals.var_xmp_dn6 = assign67600_e104318_d_n6;
        locals.var_xmp_dn7 = assign67600_e104318_d_n7;
        locals.var_xmp_dn8 = assign67600_e104318_d_n8;
        locals.var_xmp_dn9 = assign67600_e104318_d_n9;
        locals.var_xmp_dn10 = assign67600_e104318_d_n10;
        locals.var_xmp_dn13 = assign67600_e104318_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign67610_e104326, assign67610_e104326_d_n0, assign67610_e104326_d_n2, assign67610_e104326_d_n4, assign67610_e104326_d_n5, assign67610_e104326_d_n6, assign67610_e104326_d_n7, assign67610_e104326_d_n8, assign67610_e104326_d_n9, assign67610_e104326_d_n10, assign67610_e104326_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67610_e104324: f64 = (locals.var_xp + locals.var_xmp);
        (assign67610_e104324, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign67610_e104326;
        locals.var_arg_dn0 = assign67610_e104326_d_n0;
        locals.var_arg_dn2 = assign67610_e104326_d_n2;
        locals.var_arg_dn4 = assign67610_e104326_d_n4;
        locals.var_arg_dn5 = assign67610_e104326_d_n5;
        locals.var_arg_dn6 = assign67610_e104326_d_n6;
        locals.var_arg_dn7 = assign67610_e104326_d_n7;
        locals.var_arg_dn8 = assign67610_e104326_d_n8;
        locals.var_arg_dn9 = assign67610_e104326_d_n9;
        locals.var_arg_dn10 = assign67610_e104326_d_n10;
        locals.var_arg_dn13 = assign67610_e104326_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign67620_e104332, assign67620_e104332_d_n0, assign67620_e104332_d_n2, assign67620_e104332_d_n4, assign67620_e104332_d_n5, assign67620_e104332_d_n6, assign67620_e104332_d_n7, assign67620_e104332_d_n8, assign67620_e104332_d_n9, assign67620_e104332_d_n10, assign67620_e104332_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign67620_e104332;
        locals.var_dnm_dn0 = assign67620_e104332_d_n0;
        locals.var_dnm_dn2 = assign67620_e104332_d_n2;
        locals.var_dnm_dn4 = assign67620_e104332_d_n4;
        locals.var_dnm_dn5 = assign67620_e104332_d_n5;
        locals.var_dnm_dn6 = assign67620_e104332_d_n6;
        locals.var_dnm_dn7 = assign67620_e104332_d_n7;
        locals.var_dnm_dn8 = assign67620_e104332_d_n8;
        locals.var_dnm_dn9 = assign67620_e104332_d_n9;
        locals.var_dnm_dn10 = assign67620_e104332_d_n10;
        locals.var_dnm_dn13 = assign67620_e104332_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign67630_e104347: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1597 = assign67630_e104347;
        locals.var_guard1597_rv = 0.0;

        let assign67640_e104350: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1598 = assign67640_e104350;
        locals.var_guard1598_rv = 0.0;

        let (assign67650_e104360,) = {
    if ((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_guard1598 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67650_e104360;
        locals.var_mm_rv = 0.0;

        let assign67660_e104363: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1599 = assign67660_e104363;
        locals.var_guard1599_rv = 0.0;

        let (assign67670_e104376,) = {
    if (((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_guard1598 == 0.0)) && (locals.var_guard1599 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67670_e104376;
        locals.var_mm_rv = 0.0;

        let assign67680_e104379: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1600 = assign67680_e104379;
        locals.var_guard1600_rv = 0.0;

        let (assign67690_e104395,) = {
    if ((((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_guard1598 == 0.0)) && (locals.var_guard1599 == 0.0)) && (locals.var_guard1600 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67690_e104395;
        locals.var_mm_rv = 0.0;

        let assign67700_e104398: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1601 = assign67700_e104398;
        locals.var_guard1601_rv = 0.0;

        let (assign67710_e104417,) = {
    if (((((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_guard1598 == 0.0)) && (locals.var_guard1599 == 0.0)) && (locals.var_guard1600 == 0.0)) && (locals.var_guard1601 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67710_e104417;
        locals.var_mm_rv = 0.0;

        let (assign67720_e104425,) = {
    if (((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign67720_e104425;
        locals.var_m0_rv = 0.0;

        let mut assign67730_loop_guard: usize = 0;
        while {
            let assign67730_cond_e104434: f64 = if ((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign67730_cond_e104434 != 0.0
        } {
            assign67730_loop_guard += 1;
            assert!(assign67730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign67730_body0_e104443, assign67730_body0_e104443_d_n0, assign67730_body0_e104443_d_n2, assign67730_body0_e104443_d_n4, assign67730_body0_e104443_d_n5, assign67730_body0_e104443_d_n6, assign67730_body0_e104443_d_n7, assign67730_body0_e104443_d_n8, assign67730_body0_e104443_d_n9, assign67730_body0_e104443_d_n10, assign67730_body0_e104443_d_n13,) = {
    if (((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) {
        let assign67730_body0_e104441: f64 = (locals.var_dnm).sqrt();
        (assign67730_body0_e104441, (locals.var_dnm_dn0 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn2 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn4 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn5 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn6 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn7 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn8 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn9 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn10 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn13 / (2.0 * assign67730_body0_e104441)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign67730_body0_e104443;
            locals.var_dnm_dn0 = assign67730_body0_e104443_d_n0;
            locals.var_dnm_dn2 = assign67730_body0_e104443_d_n2;
            locals.var_dnm_dn4 = assign67730_body0_e104443_d_n4;
            locals.var_dnm_dn5 = assign67730_body0_e104443_d_n5;
            locals.var_dnm_dn6 = assign67730_body0_e104443_d_n6;
            locals.var_dnm_dn7 = assign67730_body0_e104443_d_n7;
            locals.var_dnm_dn8 = assign67730_body0_e104443_d_n8;
            locals.var_dnm_dn9 = assign67730_body0_e104443_d_n9;
            locals.var_dnm_dn10 = assign67730_body0_e104443_d_n10;
            locals.var_dnm_dn13 = assign67730_body0_e104443_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign67730_body1_e104453,) = {
    if (((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) {
        let assign67730_body1_e104451: f64 = (locals.var_m0 + 1.0);
        (assign67730_body1_e104451,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign67730_body1_e104453;
            locals.var_m0_rv = 0.0;
        }

        let (assign67740_e104473, assign67740_e104473_d_n0, assign67740_e104473_d_n2, assign67740_e104473_d_n4, assign67740_e104473_d_n5, assign67740_e104473_d_n6, assign67740_e104473_d_n7, assign67740_e104473_d_n8, assign67740_e104473_d_n9, assign67740_e104473_d_n10, assign67740_e104473_d_n13,) = {
    if (((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 == 0.0)) {
        let (assign67740_e104471, assign67740_e104471_d_n0, assign67740_e104471_d_n2, assign67740_e104471_d_n4, assign67740_e104471_d_n5, assign67740_e104471_d_n6, assign67740_e104471_d_n7, assign67740_e104471_d_n8, assign67740_e104471_d_n9, assign67740_e104471_d_n10, assign67740_e104471_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign67740_e104468: f64 = 2.0;
                let assign67740_e104469: f64 = (1.0 / assign67740_e104468);
                let assign67740_e104470: f64 = (locals.var_dnm).powf(assign67740_e104469);
                (assign67740_e104470, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn0)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn2)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn4)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn5)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn6)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn7)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn8)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn9)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn10)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn13)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign67740_e104471, assign67740_e104471_d_n0, assign67740_e104471_d_n2, assign67740_e104471_d_n4, assign67740_e104471_d_n5, assign67740_e104471_d_n6, assign67740_e104471_d_n7, assign67740_e104471_d_n8, assign67740_e104471_d_n9, assign67740_e104471_d_n10, assign67740_e104471_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign67740_e104473;
        locals.var_dnm_dn0 = assign67740_e104473_d_n0;
        locals.var_dnm_dn2 = assign67740_e104473_d_n2;
        locals.var_dnm_dn4 = assign67740_e104473_d_n4;
        locals.var_dnm_dn5 = assign67740_e104473_d_n5;
        locals.var_dnm_dn6 = assign67740_e104473_d_n6;
        locals.var_dnm_dn7 = assign67740_e104473_d_n7;
        locals.var_dnm_dn8 = assign67740_e104473_d_n8;
        locals.var_dnm_dn9 = assign67740_e104473_d_n9;
        locals.var_dnm_dn10 = assign67740_e104473_d_n10;
        locals.var_dnm_dn13 = assign67740_e104473_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign67750_e104481, assign67750_e104481_d_n0, assign67750_e104481_d_n2, assign67750_e104481_d_n4, assign67750_e104481_d_n5, assign67750_e104481_d_n6, assign67750_e104481_d_n7, assign67750_e104481_d_n8, assign67750_e104481_d_n9, assign67750_e104481_d_n10, assign67750_e104481_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67750_e104479: f64 = (1.0 / locals.var_dnm);
        (assign67750_e104479, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign67750_e104481;
        locals.var_dnm_dn0 = assign67750_e104481_d_n0;
        locals.var_dnm_dn2 = assign67750_e104481_d_n2;
        locals.var_dnm_dn4 = assign67750_e104481_d_n4;
        locals.var_dnm_dn5 = assign67750_e104481_d_n5;
        locals.var_dnm_dn6 = assign67750_e104481_d_n6;
        locals.var_dnm_dn7 = assign67750_e104481_d_n7;
        locals.var_dnm_dn8 = assign67750_e104481_d_n8;
        locals.var_dnm_dn9 = assign67750_e104481_d_n9;
        locals.var_dnm_dn10 = assign67750_e104481_d_n10;
        locals.var_dnm_dn13 = assign67750_e104481_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign67760_e104491, assign67760_e104491_d_n0, assign67760_e104491_d_n2, assign67760_e104491_d_n4, assign67760_e104491_d_n5, assign67760_e104491_d_n6, assign67760_e104491_d_n7, assign67760_e104491_d_n8, assign67760_e104491_d_n9, assign67760_e104491_d_n10, assign67760_e104491_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67760_e104487: f64 = (locals.var_tmf1 * 50000.0);
        let assign67760_e104489: f64 = (assign67760_e104487 * locals.var_dnm);
        (assign67760_e104489, (((locals.var_tmf1_dn0 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign67760_e104491;
        locals.var_tmf0_dn0 = assign67760_e104491_d_n0;
        locals.var_tmf0_dn2 = assign67760_e104491_d_n2;
        locals.var_tmf0_dn4 = assign67760_e104491_d_n4;
        locals.var_tmf0_dn5 = assign67760_e104491_d_n5;
        locals.var_tmf0_dn6 = assign67760_e104491_d_n6;
        locals.var_tmf0_dn7 = assign67760_e104491_d_n7;
        locals.var_tmf0_dn8 = assign67760_e104491_d_n8;
        locals.var_tmf0_dn9 = assign67760_e104491_d_n9;
        locals.var_tmf0_dn10 = assign67760_e104491_d_n10;
        locals.var_tmf0_dn13 = assign67760_e104491_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign67770_e104503, assign67770_e104503_d_n0, assign67770_e104503_d_n2, assign67770_e104503_d_n4, assign67770_e104503_d_n5, assign67770_e104503_d_n6, assign67770_e104503_d_n7, assign67770_e104503_d_n8, assign67770_e104503_d_n9, assign67770_e104503_d_n10, assign67770_e104503_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67770_e104497: f64 = (50000.0 * locals.var_xmp);
        let assign67770_e104499: f64 = (assign67770_e104497 * locals.var_dnm);
        let assign67770_e104501: f64 = (assign67770_e104499 / locals.var_arg);
        (assign67770_e104501, ((((((50000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn0)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn2)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn4)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn5)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn6)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn7)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn8)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn9)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn10)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn13) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn13)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67770_e104503;
        locals.var_t0_dn0 = assign67770_e104503_d_n0;
        locals.var_t0_dn2 = assign67770_e104503_d_n2;
        locals.var_t0_dn4 = assign67770_e104503_d_n4;
        locals.var_t0_dn5 = assign67770_e104503_d_n5;
        locals.var_t0_dn6 = assign67770_e104503_d_n6;
        locals.var_t0_dn7 = assign67770_e104503_d_n7;
        locals.var_t0_dn8 = assign67770_e104503_d_n8;
        locals.var_t0_dn9 = assign67770_e104503_d_n9;
        locals.var_t0_dn10 = assign67770_e104503_d_n10;
        locals.var_t0_dn13 = assign67770_e104503_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign67780_e104513, assign67780_e104513_d_n0, assign67780_e104513_d_n2, assign67780_e104513_d_n4, assign67780_e104513_d_n5, assign67780_e104513_d_n6, assign67780_e104513_d_n7, assign67780_e104513_d_n8, assign67780_e104513_d_n9, assign67780_e104513_d_n10, assign67780_e104513_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67780_e104509: f64 = (100000.0 - 50000.0);
        let assign67780_e104511: f64 = (assign67780_e104509 + locals.var_tmf0);
        (assign67780_e104511, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67780_e104513;
        locals.var_t2_dn0 = assign67780_e104513_d_n0;
        locals.var_t2_dn2 = assign67780_e104513_d_n2;
        locals.var_t2_dn4 = assign67780_e104513_d_n4;
        locals.var_t2_dn5 = assign67780_e104513_d_n5;
        locals.var_t2_dn6 = assign67780_e104513_d_n6;
        locals.var_t2_dn7 = assign67780_e104513_d_n7;
        locals.var_t2_dn8 = assign67780_e104513_d_n8;
        locals.var_t2_dn9 = assign67780_e104513_d_n9;
        locals.var_t2_dn10 = assign67780_e104513_d_n10;
        locals.var_t2_dn13 = assign67780_e104513_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign67790_e104519, assign67790_e104519_d_n0, assign67790_e104519_d_n2, assign67790_e104519_d_n4, assign67790_e104519_d_n5, assign67790_e104519_d_n6, assign67790_e104519_d_n7, assign67790_e104519_d_n8, assign67790_e104519_d_n9, assign67790_e104519_d_n10, assign67790_e104519_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67790_e104519;
        locals.var_t0_dn0 = assign67790_e104519_d_n0;
        locals.var_t0_dn2 = assign67790_e104519_d_n2;
        locals.var_t0_dn4 = assign67790_e104519_d_n4;
        locals.var_t0_dn5 = assign67790_e104519_d_n5;
        locals.var_t0_dn6 = assign67790_e104519_d_n6;
        locals.var_t0_dn7 = assign67790_e104519_d_n7;
        locals.var_t0_dn8 = assign67790_e104519_d_n8;
        locals.var_t0_dn9 = assign67790_e104519_d_n9;
        locals.var_t0_dn10 = assign67790_e104519_d_n10;
        locals.var_t0_dn13 = assign67790_e104519_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_245(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67800_e104526, assign67800_e104526_d_n0, assign67800_e104526_d_n2, assign67800_e104526_d_n4, assign67800_e104526_d_n5, assign67800_e104526_d_n6, assign67800_e104526_d_n7, assign67800_e104526_d_n8, assign67800_e104526_d_n9, assign67800_e104526_d_n10, assign67800_e104526_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 == 0.0)) {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67800_e104526;
        locals.var_t2_dn0 = assign67800_e104526_d_n0;
        locals.var_t2_dn2 = assign67800_e104526_d_n2;
        locals.var_t2_dn4 = assign67800_e104526_d_n4;
        locals.var_t2_dn5 = assign67800_e104526_d_n5;
        locals.var_t2_dn6 = assign67800_e104526_d_n6;
        locals.var_t2_dn7 = assign67800_e104526_d_n7;
        locals.var_t2_dn8 = assign67800_e104526_d_n8;
        locals.var_t2_dn9 = assign67800_e104526_d_n9;
        locals.var_t2_dn10 = assign67800_e104526_d_n10;
        locals.var_t2_dn13 = assign67800_e104526_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign67810_e104533, assign67810_e104533_d_n0, assign67810_e104533_d_n2, assign67810_e104533_d_n4, assign67810_e104533_d_n5, assign67810_e104533_d_n6, assign67810_e104533_d_n7, assign67810_e104533_d_n8, assign67810_e104533_d_n9, assign67810_e104533_d_n10, assign67810_e104533_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67810_e104533;
        locals.var_t0_dn0 = assign67810_e104533_d_n0;
        locals.var_t0_dn2 = assign67810_e104533_d_n2;
        locals.var_t0_dn4 = assign67810_e104533_d_n4;
        locals.var_t0_dn5 = assign67810_e104533_d_n5;
        locals.var_t0_dn6 = assign67810_e104533_d_n6;
        locals.var_t0_dn7 = assign67810_e104533_d_n7;
        locals.var_t0_dn8 = assign67810_e104533_d_n8;
        locals.var_t0_dn9 = assign67810_e104533_d_n9;
        locals.var_t0_dn10 = assign67810_e104533_d_n10;
        locals.var_t0_dn13 = assign67810_e104533_d_n13;
        locals.var_t0_rv = 0.0;

        let assign67840_e104549: f64 = (locals.var_isub + locals.var_isubld);
        let assign67840_e104559: f64 = if (((assign67840_e104549 > 0.0) && (locals.var_uc_ibpc1 != 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1602 = assign67840_e104559;
        locals.var_guard1602_rv = 0.0;

        let (assign67850_e104567, assign67850_e104567_d_n0, assign67850_e104567_d_n2, assign67850_e104567_d_n4, assign67850_e104567_d_n5, assign67850_e104567_d_n6, assign67850_e104567_d_n7, assign67850_e104567_d_n8, assign67850_e104567_d_n9, assign67850_e104567_d_n10, assign67850_e104567_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67850_e104564: f64 = (locals.var_uc_ibpc2 * locals.var_dvth);
        let assign67850_e104565: f64 = (1.0 + assign67850_e104564);
        (assign67850_e104565, (locals.var_uc_ibpc2 * locals.var_dvth_dn0), (locals.var_uc_ibpc2 * locals.var_dvth_dn2), (locals.var_uc_ibpc2 * locals.var_dvth_dn4), (locals.var_uc_ibpc2 * locals.var_dvth_dn5), (locals.var_uc_ibpc2 * locals.var_dvth_dn6), (locals.var_uc_ibpc2 * locals.var_dvth_dn7), (locals.var_uc_ibpc2 * locals.var_dvth_dn8), (locals.var_uc_ibpc2 * locals.var_dvth_dn9), (locals.var_uc_ibpc2 * locals.var_dvth_dn10), (locals.var_uc_ibpc2 * locals.var_dvth_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67850_e104567;
        locals.var_t0_dn0 = assign67850_e104567_d_n0;
        locals.var_t0_dn2 = assign67850_e104567_d_n2;
        locals.var_t0_dn4 = assign67850_e104567_d_n4;
        locals.var_t0_dn5 = assign67850_e104567_d_n5;
        locals.var_t0_dn6 = assign67850_e104567_d_n6;
        locals.var_t0_dn7 = assign67850_e104567_d_n7;
        locals.var_t0_dn8 = assign67850_e104567_d_n8;
        locals.var_t0_dn9 = assign67850_e104567_d_n9;
        locals.var_t0_dn10 = assign67850_e104567_d_n10;
        locals.var_t0_dn13 = assign67850_e104567_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign67860_e104573, assign67860_e104573_d_n0, assign67860_e104573_d_n2, assign67860_e104573_d_n4, assign67860_e104573_d_n5, assign67860_e104573_d_n6, assign67860_e104573_d_n7, assign67860_e104573_d_n8, assign67860_e104573_d_n9, assign67860_e104573_d_n10, assign67860_e104573_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67860_e104571: f64 = (locals.var_isub + locals.var_isubld);
        (assign67860_e104571, (locals.var_isub_dn0 + locals.var_isubld_dn0), (locals.var_isub_dn2 + locals.var_isubld_dn2), (locals.var_isub_dn4 + locals.var_isubld_dn4), (locals.var_isub_dn5 + locals.var_isubld_dn5), (locals.var_isub_dn6 + locals.var_isubld_dn6), (locals.var_isub_dn7 + locals.var_isubld_dn7), (locals.var_isub_dn8 + locals.var_isubld_dn8), (locals.var_isub_dn9 + locals.var_isubld_dn9), (locals.var_isub_dn10 + locals.var_isubld_dn10), (locals.var_isub_dn13 + locals.var_isubld_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67860_e104573;
        locals.var_t1_dn0 = assign67860_e104573_d_n0;
        locals.var_t1_dn2 = assign67860_e104573_d_n2;
        locals.var_t1_dn4 = assign67860_e104573_d_n4;
        locals.var_t1_dn5 = assign67860_e104573_d_n5;
        locals.var_t1_dn6 = assign67860_e104573_d_n6;
        locals.var_t1_dn7 = assign67860_e104573_d_n7;
        locals.var_t1_dn8 = assign67860_e104573_d_n8;
        locals.var_t1_dn9 = assign67860_e104573_d_n9;
        locals.var_t1_dn10 = assign67860_e104573_d_n10;
        locals.var_t1_dn13 = assign67860_e104573_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign67870_e104581, assign67870_e104581_d_n0, assign67870_e104581_d_n2, assign67870_e104581_d_n4, assign67870_e104581_d_n5, assign67870_e104581_d_n6, assign67870_e104581_d_n7, assign67870_e104581_d_n8, assign67870_e104581_d_n9, assign67870_e104581_d_n10, assign67870_e104581_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67870_e104577: f64 = (locals.var_uc_ibpc1 * locals.var_t0);
        let assign67870_e104579: f64 = (assign67870_e104577 * locals.var_t1);
        (assign67870_e104579, (((locals.var_uc_ibpc1 * locals.var_t0_dn0) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn0)), (((locals.var_uc_ibpc1 * locals.var_t0_dn2) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn2)), (((locals.var_uc_ibpc1 * locals.var_t0_dn4) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn4)), (((locals.var_uc_ibpc1 * locals.var_t0_dn5) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn5)), (((locals.var_uc_ibpc1 * locals.var_t0_dn6) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn6)), (((locals.var_uc_ibpc1 * locals.var_t0_dn7) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn7)), (((locals.var_uc_ibpc1 * locals.var_t0_dn8) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn8)), (((locals.var_uc_ibpc1 * locals.var_t0_dn9) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn9)), (((locals.var_uc_ibpc1 * locals.var_t0_dn10) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn10)), (((locals.var_uc_ibpc1 * locals.var_t0_dn13) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn13)),)
    } else {
        (locals.var_dvbsibpc, locals.var_dvbsibpc_dn0, locals.var_dvbsibpc_dn2, locals.var_dvbsibpc_dn4, locals.var_dvbsibpc_dn5, locals.var_dvbsibpc_dn6, locals.var_dvbsibpc_dn7, locals.var_dvbsibpc_dn8, locals.var_dvbsibpc_dn9, locals.var_dvbsibpc_dn10, locals.var_dvbsibpc_dn13,)
    }
};
        locals.var_dvbsibpc = assign67870_e104581;
        locals.var_dvbsibpc_dn0 = assign67870_e104581_d_n0;
        locals.var_dvbsibpc_dn2 = assign67870_e104581_d_n2;
        locals.var_dvbsibpc_dn4 = assign67870_e104581_d_n4;
        locals.var_dvbsibpc_dn5 = assign67870_e104581_d_n5;
        locals.var_dvbsibpc_dn6 = assign67870_e104581_d_n6;
        locals.var_dvbsibpc_dn7 = assign67870_e104581_d_n7;
        locals.var_dvbsibpc_dn8 = assign67870_e104581_d_n8;
        locals.var_dvbsibpc_dn9 = assign67870_e104581_d_n9;
        locals.var_dvbsibpc_dn10 = assign67870_e104581_d_n10;
        locals.var_dvbsibpc_dn13 = assign67870_e104581_d_n13;
        locals.var_dvbsibpc_rv = 0.0;

        let (assign67880_e104587, assign67880_e104587_d_n0, assign67880_e104587_d_n2, assign67880_e104587_d_n4, assign67880_e104587_d_n5, assign67880_e104587_d_n6, assign67880_e104587_d_n7, assign67880_e104587_d_n8, assign67880_e104587_d_n9, assign67880_e104587_d_n10, assign67880_e104587_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67880_e104585: f64 = (1.0 / locals.var_xi0);
        (assign67880_e104585, (-(locals.var_xi0_dn0 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn2 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn4 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn5 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn6 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn7 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn8 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn9 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn10 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn13 / (locals.var_xi0 * locals.var_xi0))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign67880_e104587;
        locals.var_t10_dn0 = assign67880_e104587_d_n0;
        locals.var_t10_dn2 = assign67880_e104587_d_n2;
        locals.var_t10_dn4 = assign67880_e104587_d_n4;
        locals.var_t10_dn5 = assign67880_e104587_d_n5;
        locals.var_t10_dn6 = assign67880_e104587_d_n6;
        locals.var_t10_dn7 = assign67880_e104587_d_n7;
        locals.var_t10_dn8 = assign67880_e104587_d_n8;
        locals.var_t10_dn9 = assign67880_e104587_d_n9;
        locals.var_t10_dn10 = assign67880_e104587_d_n10;
        locals.var_t10_dn13 = assign67880_e104587_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign67890_e104595, assign67890_e104595_d_n0, assign67890_e104595_d_n2, assign67890_e104595_d_n4, assign67890_e104595_d_n5, assign67890_e104595_d_n6, assign67890_e104595_d_n7, assign67890_e104595_d_n8, assign67890_e104595_d_n9, assign67890_e104595_d_n10, assign67890_e104595_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67890_e104591: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign67890_e104593: f64 = (assign67890_e104591 * locals.var_t10);
        (assign67890_e104593, ((((locals.var_beta_dn0 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn0)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn0)), ((((locals.var_beta_dn2 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn2)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn2)), ((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn4)), ((((locals.var_beta_dn5 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn5)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn5)), ((((locals.var_beta_dn6 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn6)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn6)), ((((locals.var_beta_dn7 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn7)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn7)), ((((locals.var_beta_dn8 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn8)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn8)), ((((locals.var_beta_dn9 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn9)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn9)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn10)), ((((locals.var_beta_dn13 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn13)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67890_e104595;
        locals.var_t1_dn0 = assign67890_e104595_d_n0;
        locals.var_t1_dn2 = assign67890_e104595_d_n2;
        locals.var_t1_dn4 = assign67890_e104595_d_n4;
        locals.var_t1_dn5 = assign67890_e104595_d_n5;
        locals.var_t1_dn6 = assign67890_e104595_d_n6;
        locals.var_t1_dn7 = assign67890_e104595_d_n7;
        locals.var_t1_dn8 = assign67890_e104595_d_n8;
        locals.var_t1_dn9 = assign67890_e104595_d_n9;
        locals.var_t1_dn10 = assign67890_e104595_d_n10;
        locals.var_t1_dn13 = assign67890_e104595_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign67900_e104601, assign67900_e104601_d_n0, assign67900_e104601_d_n2, assign67900_e104601_d_n4, assign67900_e104601_d_n5, assign67900_e104601_d_n6, assign67900_e104601_d_n7, assign67900_e104601_d_n8, assign67900_e104601_d_n9, assign67900_e104601_d_n10, assign67900_e104601_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67900_e104599: f64 = (locals.var_t10 * locals.var_t10);
        (assign67900_e104599, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn13 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign67900_e104601;
        locals.var_t11_dn0 = assign67900_e104601_d_n0;
        locals.var_t11_dn2 = assign67900_e104601_d_n2;
        locals.var_t11_dn4 = assign67900_e104601_d_n4;
        locals.var_t11_dn5 = assign67900_e104601_d_n5;
        locals.var_t11_dn6 = assign67900_e104601_d_n6;
        locals.var_t11_dn7 = assign67900_e104601_d_n7;
        locals.var_t11_dn8 = assign67900_e104601_d_n8;
        locals.var_t11_dn9 = assign67900_e104601_d_n9;
        locals.var_t11_dn10 = assign67900_e104601_d_n10;
        locals.var_t11_dn13 = assign67900_e104601_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign67910_e104607, assign67910_e104607_d_n0, assign67910_e104607_d_n2, assign67910_e104607_d_n4, assign67910_e104607_d_n5, assign67910_e104607_d_n6, assign67910_e104607_d_n7, assign67910_e104607_d_n8, assign67910_e104607_d_n9, assign67910_e104607_d_n10, assign67910_e104607_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67910_e104605: f64 = (1.0 / locals.var_xil);
        (assign67910_e104605, (-(locals.var_xil_dn0 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn2 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn4 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn5 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn6 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn7 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn8 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn9 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn10 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn13 / (locals.var_xil * locals.var_xil))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign67910_e104607;
        locals.var_t10_dn0 = assign67910_e104607_d_n0;
        locals.var_t10_dn2 = assign67910_e104607_d_n2;
        locals.var_t10_dn4 = assign67910_e104607_d_n4;
        locals.var_t10_dn5 = assign67910_e104607_d_n5;
        locals.var_t10_dn6 = assign67910_e104607_d_n6;
        locals.var_t10_dn7 = assign67910_e104607_d_n7;
        locals.var_t10_dn8 = assign67910_e104607_d_n8;
        locals.var_t10_dn9 = assign67910_e104607_d_n9;
        locals.var_t10_dn10 = assign67910_e104607_d_n10;
        locals.var_t10_dn13 = assign67910_e104607_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign67920_e104615, assign67920_e104615_d_n0, assign67920_e104615_d_n2, assign67920_e104615_d_n4, assign67920_e104615_d_n5, assign67920_e104615_d_n6, assign67920_e104615_d_n7, assign67920_e104615_d_n8, assign67920_e104615_d_n9, assign67920_e104615_d_n10, assign67920_e104615_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67920_e104611: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign67920_e104613: f64 = (assign67920_e104611 * locals.var_t10);
        (assign67920_e104613, ((((locals.var_beta_dn0 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn0)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn0)), ((((locals.var_beta_dn2 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn2)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn2)), ((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn4)), ((((locals.var_beta_dn5 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn5)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn5)), ((((locals.var_beta_dn6 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn6)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn6)), ((((locals.var_beta_dn7 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn7)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn7)), ((((locals.var_beta_dn8 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn8)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn8)), ((((locals.var_beta_dn9 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn9)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn9)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn10)), ((((locals.var_beta_dn13 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn13)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67920_e104615;
        locals.var_t2_dn0 = assign67920_e104615_d_n0;
        locals.var_t2_dn2 = assign67920_e104615_d_n2;
        locals.var_t2_dn4 = assign67920_e104615_d_n4;
        locals.var_t2_dn5 = assign67920_e104615_d_n5;
        locals.var_t2_dn6 = assign67920_e104615_d_n6;
        locals.var_t2_dn7 = assign67920_e104615_d_n7;
        locals.var_t2_dn8 = assign67920_e104615_d_n8;
        locals.var_t2_dn9 = assign67920_e104615_d_n9;
        locals.var_t2_dn10 = assign67920_e104615_d_n10;
        locals.var_t2_dn13 = assign67920_e104615_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign67930_e104621, assign67930_e104621_d_n0, assign67930_e104621_d_n2, assign67930_e104621_d_n4, assign67930_e104621_d_n5, assign67930_e104621_d_n6, assign67930_e104621_d_n7, assign67930_e104621_d_n8, assign67930_e104621_d_n9, assign67930_e104621_d_n10, assign67930_e104621_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67930_e104619: f64 = (locals.var_t10 * locals.var_t10);
        (assign67930_e104619, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn13 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign67930_e104621;
        locals.var_t11_dn0 = assign67930_e104621_d_n0;
        locals.var_t11_dn2 = assign67930_e104621_d_n2;
        locals.var_t11_dn4 = assign67930_e104621_d_n4;
        locals.var_t11_dn5 = assign67930_e104621_d_n5;
        locals.var_t11_dn6 = assign67930_e104621_d_n6;
        locals.var_t11_dn7 = assign67930_e104621_d_n7;
        locals.var_t11_dn8 = assign67930_e104621_d_n8;
        locals.var_t11_dn9 = assign67930_e104621_d_n9;
        locals.var_t11_dn10 = assign67930_e104621_d_n10;
        locals.var_t11_dn13 = assign67930_e104621_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign67940_e104633, assign67940_e104633_d_n0, assign67940_e104633_d_n2, assign67940_e104633_d_n4, assign67940_e104633_d_n5, assign67940_e104633_d_n6, assign67940_e104633_d_n7, assign67940_e104633_d_n8, assign67940_e104633_d_n9, assign67940_e104633_d_n10, assign67940_e104633_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67940_e104626: f64 = (locals.var_xilp32 * locals.var_t2);
        let assign67940_e104629: f64 = (locals.var_xi0p32 * locals.var_t1);
        let assign67940_e104630: f64 = (assign67940_e104626 - assign67940_e104629);
        let assign67940_e104631: f64 = (locals.var_cnst0 * assign67940_e104630);
        (assign67940_e104631, ((locals.var_cnst0_dn0 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn0 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn0)) - ((locals.var_xi0p32_dn0 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn0))))), ((locals.var_cnst0_dn2 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn2 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn2)) - ((locals.var_xi0p32_dn2 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn2))))), ((locals.var_cnst0_dn4 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn4 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn4)) - ((locals.var_xi0p32_dn4 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn4))))), ((locals.var_cnst0_dn5 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn5 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn5)) - ((locals.var_xi0p32_dn5 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn5))))), ((locals.var_cnst0_dn6 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn6 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn6)) - ((locals.var_xi0p32_dn6 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn6))))), ((locals.var_cnst0_dn7 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn7 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn7)) - ((locals.var_xi0p32_dn7 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn7))))), ((locals.var_cnst0_dn8 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn8 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn8)) - ((locals.var_xi0p32_dn8 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn8))))), ((locals.var_cnst0_dn9 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn9 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn9)) - ((locals.var_xi0p32_dn9 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn9))))), ((locals.var_cnst0_dn10 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn10 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn10)) - ((locals.var_xi0p32_dn10 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn10))))), ((locals.var_cnst0_dn13 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn13 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn13)) - ((locals.var_xi0p32_dn13 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn13))))),)
    } else {
        (locals.var_dg3, locals.var_dg3_dn0, locals.var_dg3_dn2, locals.var_dg3_dn4, locals.var_dg3_dn5, locals.var_dg3_dn6, locals.var_dg3_dn7, locals.var_dg3_dn8, locals.var_dg3_dn9, locals.var_dg3_dn10, locals.var_dg3_dn13,)
    }
};
        locals.var_dg3 = assign67940_e104633;
        locals.var_dg3_dn0 = assign67940_e104633_d_n0;
        locals.var_dg3_dn2 = assign67940_e104633_d_n2;
        locals.var_dg3_dn4 = assign67940_e104633_d_n4;
        locals.var_dg3_dn5 = assign67940_e104633_d_n5;
        locals.var_dg3_dn6 = assign67940_e104633_d_n6;
        locals.var_dg3_dn7 = assign67940_e104633_d_n7;
        locals.var_dg3_dn8 = assign67940_e104633_d_n8;
        locals.var_dg3_dn9 = assign67940_e104633_d_n9;
        locals.var_dg3_dn10 = assign67940_e104633_d_n10;
        locals.var_dg3_dn13 = assign67940_e104633_d_n13;
        locals.var_dg3_rv = 0.0;

        let (assign67950_e104648, assign67950_e104648_d_n0, assign67950_e104648_d_n2, assign67950_e104648_d_n4, assign67950_e104648_d_n5, assign67950_e104648_d_n6, assign67950_e104648_d_n7, assign67950_e104648_d_n8, assign67950_e104648_d_n9, assign67950_e104648_d_n10, assign67950_e104648_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67950_e104637: f64 = (locals.var_cnst0 * 0.5);
        let assign67950_e104639: f64 = (-locals.var_xilp12);
        let assign67950_e104641: f64 = (assign67950_e104639 * locals.var_t2);
        let assign67950_e104644: f64 = (locals.var_xi0p12 * locals.var_t1);
        let assign67950_e104645: f64 = (assign67950_e104641 + assign67950_e104644);
        let assign67950_e104646: f64 = (assign67950_e104637 * assign67950_e104645);
        (assign67950_e104646, (((locals.var_cnst0_dn0 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn0) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn0)) + ((locals.var_xi0p12_dn0 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn0))))), (((locals.var_cnst0_dn2 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn2) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn2)) + ((locals.var_xi0p12_dn2 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn2))))), (((locals.var_cnst0_dn4 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn4) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn4)) + ((locals.var_xi0p12_dn4 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn4))))), (((locals.var_cnst0_dn5 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn5) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn5)) + ((locals.var_xi0p12_dn5 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn5))))), (((locals.var_cnst0_dn6 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn6) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn6)) + ((locals.var_xi0p12_dn6 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn6))))), (((locals.var_cnst0_dn7 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn7) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn7)) + ((locals.var_xi0p12_dn7 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn7))))), (((locals.var_cnst0_dn8 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn8) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn8)) + ((locals.var_xi0p12_dn8 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn8))))), (((locals.var_cnst0_dn9 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn9) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn9)) + ((locals.var_xi0p12_dn9 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn9))))), (((locals.var_cnst0_dn10 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn10) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn10)) + ((locals.var_xi0p12_dn10 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn10))))), (((locals.var_cnst0_dn13 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn13) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn13)) + ((locals.var_xi0p12_dn13 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn13))))),)
    } else {
        (locals.var_dg4, locals.var_dg4_dn0, locals.var_dg4_dn2, locals.var_dg4_dn4, locals.var_dg4_dn5, locals.var_dg4_dn6, locals.var_dg4_dn7, locals.var_dg4_dn8, locals.var_dg4_dn9, locals.var_dg4_dn10, locals.var_dg4_dn13,)
    }
};
        locals.var_dg4 = assign67950_e104648;
        locals.var_dg4_dn0 = assign67950_e104648_d_n0;
        locals.var_dg4_dn2 = assign67950_e104648_d_n2;
        locals.var_dg4_dn4 = assign67950_e104648_d_n4;
        locals.var_dg4_dn5 = assign67950_e104648_d_n5;
        locals.var_dg4_dn6 = assign67950_e104648_d_n6;
        locals.var_dg4_dn7 = assign67950_e104648_d_n7;
        locals.var_dg4_dn8 = assign67950_e104648_d_n8;
        locals.var_dg4_dn9 = assign67950_e104648_d_n9;
        locals.var_dg4_dn10 = assign67950_e104648_d_n10;
        locals.var_dg4_dn13 = assign67950_e104648_d_n13;
        locals.var_dg4_rv = 0.0;

        let (assign67960_e104654, assign67960_e104654_d_n0, assign67960_e104654_d_n2, assign67960_e104654_d_n4, assign67960_e104654_d_n5, assign67960_e104654_d_n6, assign67960_e104654_d_n7, assign67960_e104654_d_n8, assign67960_e104654_d_n9, assign67960_e104654_d_n10, assign67960_e104654_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67960_e104652: f64 = (locals.var_dg3 + locals.var_dg4);
        (assign67960_e104652, (locals.var_dg3_dn0 + locals.var_dg4_dn0), (locals.var_dg3_dn2 + locals.var_dg4_dn2), (locals.var_dg3_dn4 + locals.var_dg4_dn4), (locals.var_dg3_dn5 + locals.var_dg4_dn5), (locals.var_dg3_dn6 + locals.var_dg4_dn6), (locals.var_dg3_dn7 + locals.var_dg4_dn7), (locals.var_dg3_dn8 + locals.var_dg4_dn8), (locals.var_dg3_dn9 + locals.var_dg4_dn9), (locals.var_dg3_dn10 + locals.var_dg4_dn10), (locals.var_dg3_dn13 + locals.var_dg4_dn13),)
    } else {
        (locals.var_didd, locals.var_didd_dn0, locals.var_didd_dn2, locals.var_didd_dn4, locals.var_didd_dn5, locals.var_didd_dn6, locals.var_didd_dn7, locals.var_didd_dn8, locals.var_didd_dn9, locals.var_didd_dn10, locals.var_didd_dn13,)
    }
};
        locals.var_didd = assign67960_e104654;
        locals.var_didd_dn0 = assign67960_e104654_d_n0;
        locals.var_didd_dn2 = assign67960_e104654_d_n2;
        locals.var_didd_dn4 = assign67960_e104654_d_n4;
        locals.var_didd_dn5 = assign67960_e104654_d_n5;
        locals.var_didd_dn6 = assign67960_e104654_d_n6;
        locals.var_didd_dn7 = assign67960_e104654_d_n7;
        locals.var_didd_dn8 = assign67960_e104654_d_n8;
        locals.var_didd_dn9 = assign67960_e104654_d_n9;
        locals.var_didd_dn10 = assign67960_e104654_d_n10;
        locals.var_didd_dn13 = assign67960_e104654_d_n13;
        locals.var_didd_rv = 0.0;

        let (assign67970_e104662, assign67970_e104662_d_n0, assign67970_e104662_d_n2, assign67970_e104662_d_n4, assign67970_e104662_d_n5, assign67970_e104662_d_n6, assign67970_e104662_d_n7, assign67970_e104662_d_n8, assign67970_e104662_d_n9, assign67970_e104662_d_n10, assign67970_e104662_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67970_e104658: f64 = (locals.var_betawl * locals.var_didd);
        let assign67970_e104660: f64 = (assign67970_e104658 * locals.var_mu);
        (assign67970_e104660, ((((locals.var_betawl_dn0 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn0)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn2)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn4)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn5)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn6)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn7)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn8)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn9)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn10)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn10)), ((((locals.var_betawl_dn13 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn13)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn13)),)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn13,)
    }
};
        locals.var_idsibpc = assign67970_e104662;
        locals.var_idsibpc_dn0 = assign67970_e104662_d_n0;
        locals.var_idsibpc_dn2 = assign67970_e104662_d_n2;
        locals.var_idsibpc_dn4 = assign67970_e104662_d_n4;
        locals.var_idsibpc_dn5 = assign67970_e104662_d_n5;
        locals.var_idsibpc_dn6 = assign67970_e104662_d_n6;
        locals.var_idsibpc_dn7 = assign67970_e104662_d_n7;
        locals.var_idsibpc_dn8 = assign67970_e104662_d_n8;
        locals.var_idsibpc_dn9 = assign67970_e104662_d_n9;
        locals.var_idsibpc_dn10 = assign67970_e104662_d_n10;
        locals.var_idsibpc_dn13 = assign67970_e104662_d_n13;
        locals.var_idsibpc_rv = 0.0;

        let (assign67980_e104668, assign67980_e104668_d_n0, assign67980_e104668_d_n2, assign67980_e104668_d_n4, assign67980_e104668_d_n5, assign67980_e104668_d_n6, assign67980_e104668_d_n7, assign67980_e104668_d_n8, assign67980_e104668_d_n9, assign67980_e104668_d_n10, assign67980_e104668_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67980_e104666: f64 = (locals.var_wk_ii * locals.var_idsibpc);
        (assign67980_e104666, ((locals.var_wk_ii_dn0 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn0)), ((locals.var_wk_ii_dn2 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn2)), ((locals.var_wk_ii_dn4 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn4)), ((locals.var_wk_ii_dn5 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn5)), ((locals.var_wk_ii_dn6 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn6)), ((locals.var_wk_ii_dn7 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn7)), ((locals.var_wk_ii_dn8 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn8)), ((locals.var_wk_ii_dn9 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn9)), ((locals.var_wk_ii_dn10 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn10)), ((locals.var_wk_ii_dn13 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn13)),)
    } else {
        (locals.var_isubibpc, locals.var_isubibpc_dn0, locals.var_isubibpc_dn2, locals.var_isubibpc_dn4, locals.var_isubibpc_dn5, locals.var_isubibpc_dn6, locals.var_isubibpc_dn7, locals.var_isubibpc_dn8, locals.var_isubibpc_dn9, locals.var_isubibpc_dn10, locals.var_isubibpc_dn13,)
    }
};
        locals.var_isubibpc = assign67980_e104668;
        locals.var_isubibpc_dn0 = assign67980_e104668_d_n0;
        locals.var_isubibpc_dn2 = assign67980_e104668_d_n2;
        locals.var_isubibpc_dn4 = assign67980_e104668_d_n4;
        locals.var_isubibpc_dn5 = assign67980_e104668_d_n5;
        locals.var_isubibpc_dn6 = assign67980_e104668_d_n6;
        locals.var_isubibpc_dn7 = assign67980_e104668_d_n7;
        locals.var_isubibpc_dn8 = assign67980_e104668_d_n8;
        locals.var_isubibpc_dn9 = assign67980_e104668_d_n9;
        locals.var_isubibpc_dn10 = assign67980_e104668_d_n10;
        locals.var_isubibpc_dn13 = assign67980_e104668_d_n13;
        locals.var_isubibpc_rv = 0.0;

        let assign67990_e104671: f64 = if p.p24 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1603 = assign67990_e104671;
        locals.var_guard1603_rv = 0.0;

        let assign68000_e104674: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1604 = assign68000_e104674;
        locals.var_guard1604_rv = 0.0;

        let (assign68010_e104686, assign68010_e104686_d_n0, assign68010_e104686_d_n2, assign68010_e104686_d_n4, assign68010_e104686_d_n5, assign68010_e104686_d_n6, assign68010_e104686_d_n7, assign68010_e104686_d_n8, assign68010_e104686_d_n9, assign68010_e104686_d_n10, assign68010_e104686_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68010_e104680: f64 = (locals.var_ps0z + locals.var_vdsz__blk439);
        let assign68010_e104683: f64 = (10.0 * 2.220446049250313e-16);
        let assign68010_e104684: f64 = (assign68010_e104680 - assign68010_e104683);
        (assign68010_e104684, (locals.var_ps0z_dn0 + locals.var_vdsz__blk439_dn0), (locals.var_ps0z_dn2 + locals.var_vdsz__blk439_dn2), (locals.var_ps0z_dn4 + locals.var_vdsz__blk439_dn4), (locals.var_ps0z_dn5 + locals.var_vdsz__blk439_dn5), (locals.var_ps0z_dn6 + locals.var_vdsz__blk439_dn6), (locals.var_ps0z_dn7 + locals.var_vdsz__blk439_dn7), (locals.var_ps0z_dn8 + locals.var_vdsz__blk439_dn8), (locals.var_ps0z_dn9 + locals.var_vdsz__blk439_dn9), (locals.var_ps0z_dn10 + locals.var_vdsz__blk439_dn10), (locals.var_ps0z_dn13 + locals.var_vdsz__blk439_dn13),)
    } else {
        (locals.var_psdlz, locals.var_psdlz_dn0, locals.var_psdlz_dn2, locals.var_psdlz_dn4, locals.var_psdlz_dn5, locals.var_psdlz_dn6, locals.var_psdlz_dn7, locals.var_psdlz_dn8, locals.var_psdlz_dn9, locals.var_psdlz_dn10, locals.var_psdlz_dn13,)
    }
};
        locals.var_psdlz = assign68010_e104686;
        locals.var_psdlz_dn0 = assign68010_e104686_d_n0;
        locals.var_psdlz_dn2 = assign68010_e104686_d_n2;
        locals.var_psdlz_dn4 = assign68010_e104686_d_n4;
        locals.var_psdlz_dn5 = assign68010_e104686_d_n5;
        locals.var_psdlz_dn6 = assign68010_e104686_d_n6;
        locals.var_psdlz_dn7 = assign68010_e104686_d_n7;
        locals.var_psdlz_dn8 = assign68010_e104686_d_n8;
        locals.var_psdlz_dn9 = assign68010_e104686_d_n9;
        locals.var_psdlz_dn10 = assign68010_e104686_d_n10;
        locals.var_psdlz_dn13 = assign68010_e104686_d_n13;
        locals.var_psdlz_rv = 0.0;

        let (assign68020_e104706, assign68020_e104706_d_n0, assign68020_e104706_d_n2, assign68020_e104706_d_n4, assign68020_e104706_d_n5, assign68020_e104706_d_n6, assign68020_e104706_d_n7, assign68020_e104706_d_n8, assign68020_e104706_d_n9, assign68020_e104706_d_n10, assign68020_e104706_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68020_e104692: f64 = (locals.var_vgsz__blk440 - locals.var_vfb);
        let assign68020_e104696: f64 = (locals.var_dvth - locals.var_dppg);
        let assign68020_e104697: f64 = (locals.var_mks_gleak4 * assign68020_e104696);
        let assign68020_e104699: f64 = (assign68020_e104697 * locals.var_leff);
        let assign68020_e104700: f64 = (assign68020_e104692 + assign68020_e104699);
        let assign68020_e104703: f64 = (locals.var_psdlz * locals.var_uc_gleak3);
        let assign68020_e104704: f64 = (assign68020_e104700 - assign68020_e104703);
        (assign68020_e104704, ((locals.var_vgsz__blk440_dn0 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn0 - locals.var_dppg_dn0)) * locals.var_leff)) - (locals.var_psdlz_dn0 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn2 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn2 - locals.var_dppg_dn2)) * locals.var_leff)) - (locals.var_psdlz_dn2 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn4 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn4 - locals.var_dppg_dn4)) * locals.var_leff)) - (locals.var_psdlz_dn4 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn5 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn5 - locals.var_dppg_dn5)) * locals.var_leff)) - (locals.var_psdlz_dn5 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn6 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn6 - locals.var_dppg_dn6)) * locals.var_leff)) - (locals.var_psdlz_dn6 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn7 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn7 - locals.var_dppg_dn7)) * locals.var_leff)) - (locals.var_psdlz_dn7 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn8 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn8 - locals.var_dppg_dn8)) * locals.var_leff)) - (locals.var_psdlz_dn8 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn9 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn9 - locals.var_dppg_dn9)) * locals.var_leff)) - (locals.var_psdlz_dn9 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn10 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn10 - locals.var_dppg_dn10)) * locals.var_leff)) - (locals.var_psdlz_dn10 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn13 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn13 - locals.var_dppg_dn13)) * locals.var_leff)) - (locals.var_psdlz_dn13 * locals.var_uc_gleak3)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68020_e104706;
        locals.var_t1_dn0 = assign68020_e104706_d_n0;
        locals.var_t1_dn2 = assign68020_e104706_d_n2;
        locals.var_t1_dn4 = assign68020_e104706_d_n4;
        locals.var_t1_dn5 = assign68020_e104706_d_n5;
        locals.var_t1_dn6 = assign68020_e104706_d_n6;
        locals.var_t1_dn7 = assign68020_e104706_d_n7;
        locals.var_t1_dn8 = assign68020_e104706_d_n8;
        locals.var_t1_dn9 = assign68020_e104706_d_n9;
        locals.var_t1_dn10 = assign68020_e104706_d_n10;
        locals.var_t1_dn13 = assign68020_e104706_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign68030_e104714, assign68030_e104714_d_n0, assign68030_e104714_d_n2, assign68030_e104714_d_n4, assign68030_e104714_d_n5, assign68030_e104714_d_n6, assign68030_e104714_d_n7, assign68030_e104714_d_n8, assign68030_e104714_d_n9, assign68030_e104714_d_n10, assign68030_e104714_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68030_e104712: f64 = (locals.var_t1 * locals.var_t1);
        (assign68030_e104712, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68030_e104714;
        locals.var_t1_dn0 = assign68030_e104714_d_n0;
        locals.var_t1_dn2 = assign68030_e104714_d_n2;
        locals.var_t1_dn4 = assign68030_e104714_d_n4;
        locals.var_t1_dn5 = assign68030_e104714_d_n5;
        locals.var_t1_dn6 = assign68030_e104714_d_n6;
        locals.var_t1_dn7 = assign68030_e104714_d_n7;
        locals.var_t1_dn8 = assign68030_e104714_d_n8;
        locals.var_t1_dn9 = assign68030_e104714_d_n9;
        locals.var_t1_dn10 = assign68030_e104714_d_n10;
        locals.var_t1_dn13 = assign68030_e104714_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign68040_e104722, assign68040_e104722_d_n0, assign68040_e104722_d_n2, assign68040_e104722_d_n4, assign68040_e104722_d_n5, assign68040_e104722_d_n6, assign68040_e104722_d_n7, assign68040_e104722_d_n8, assign68040_e104722_d_n9, assign68040_e104722_d_n10, assign68040_e104722_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68040_e104720: f64 = (1.0 / locals.var_tox0);
        (assign68040_e104720, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68040_e104722;
        locals.var_t3_dn0 = assign68040_e104722_d_n0;
        locals.var_t3_dn2 = assign68040_e104722_d_n2;
        locals.var_t3_dn4 = assign68040_e104722_d_n4;
        locals.var_t3_dn5 = assign68040_e104722_d_n5;
        locals.var_t3_dn6 = assign68040_e104722_d_n6;
        locals.var_t3_dn7 = assign68040_e104722_d_n7;
        locals.var_t3_dn8 = assign68040_e104722_d_n8;
        locals.var_t3_dn9 = assign68040_e104722_d_n9;
        locals.var_t3_dn10 = assign68040_e104722_d_n10;
        locals.var_t3_dn13 = assign68040_e104722_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign68050_e104730, assign68050_e104730_d_n0, assign68050_e104730_d_n2, assign68050_e104730_d_n4, assign68050_e104730_d_n5, assign68050_e104730_d_n6, assign68050_e104730_d_n7, assign68050_e104730_d_n8, assign68050_e104730_d_n9, assign68050_e104730_d_n10, assign68050_e104730_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68050_e104728: f64 = (locals.var_t1 * locals.var_t3);
        (assign68050_e104728, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68050_e104730;
        locals.var_t2_dn0 = assign68050_e104730_d_n0;
        locals.var_t2_dn2 = assign68050_e104730_d_n2;
        locals.var_t2_dn4 = assign68050_e104730_d_n4;
        locals.var_t2_dn5 = assign68050_e104730_d_n5;
        locals.var_t2_dn6 = assign68050_e104730_d_n6;
        locals.var_t2_dn7 = assign68050_e104730_d_n7;
        locals.var_t2_dn8 = assign68050_e104730_d_n8;
        locals.var_t2_dn9 = assign68050_e104730_d_n9;
        locals.var_t2_dn10 = assign68050_e104730_d_n10;
        locals.var_t2_dn13 = assign68050_e104730_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign68060_e104738, assign68060_e104738_d_n0, assign68060_e104738_d_n2, assign68060_e104738_d_n4, assign68060_e104738_d_n5, assign68060_e104738_d_n6, assign68060_e104738_d_n7, assign68060_e104738_d_n8, assign68060_e104738_d_n9, assign68060_e104738_d_n10, assign68060_e104738_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68060_e104736: f64 = (1.0 / locals.var_mks_gleak5);
        (assign68060_e104736, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68060_e104738;
        locals.var_t3_dn0 = assign68060_e104738_d_n0;
        locals.var_t3_dn2 = assign68060_e104738_d_n2;
        locals.var_t3_dn4 = assign68060_e104738_d_n4;
        locals.var_t3_dn5 = assign68060_e104738_d_n5;
        locals.var_t3_dn6 = assign68060_e104738_d_n6;
        locals.var_t3_dn7 = assign68060_e104738_d_n7;
        locals.var_t3_dn8 = assign68060_e104738_d_n8;
        locals.var_t3_dn9 = assign68060_e104738_d_n9;
        locals.var_t3_dn10 = assign68060_e104738_d_n10;
        locals.var_t3_dn13 = assign68060_e104738_d_n13;
        locals.var_t3_rv = 0.0;

    }
}
