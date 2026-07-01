#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_328(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign85640_e131317, assign85640_e131317_d_n0, assign85640_e131317_d_n2, assign85640_e131317_d_n4, assign85640_e131317_d_n5, assign85640_e131317_d_n6, assign85640_e131317_d_n7, assign85640_e131317_d_n8, assign85640_e131317_d_n9, assign85640_e131317_d_n10, assign85640_e131317_d_n11, assign85640_e131317_d_n14,) = {
    if ((((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) && (locals.var_guard2001 == 0.0)) {
        let (assign85640_e131315,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign85640_e131313: f64 = (-1.0);
                (assign85640_e131313,)
            } else {
                (1.0,)
            }
        };
        (assign85640_e131315, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign85640_e131317;
        locals.var_tmf3_dn0 = assign85640_e131317_d_n0;
        locals.var_tmf3_dn2 = assign85640_e131317_d_n2;
        locals.var_tmf3_dn4 = assign85640_e131317_d_n4;
        locals.var_tmf3_dn5 = assign85640_e131317_d_n5;
        locals.var_tmf3_dn6 = assign85640_e131317_d_n6;
        locals.var_tmf3_dn7 = assign85640_e131317_d_n7;
        locals.var_tmf3_dn8 = assign85640_e131317_d_n8;
        locals.var_tmf3_dn9 = assign85640_e131317_d_n9;
        locals.var_tmf3_dn10 = assign85640_e131317_d_n10;
        locals.var_tmf3_dn11 = assign85640_e131317_d_n11;
        locals.var_tmf3_dn14 = assign85640_e131317_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign85650_e131337, assign85650_e131337_d_n0, assign85650_e131337_d_n2, assign85650_e131337_d_n4, assign85650_e131337_d_n5, assign85650_e131337_d_n6, assign85650_e131337_d_n7, assign85650_e131337_d_n8, assign85650_e131337_d_n9, assign85650_e131337_d_n10, assign85650_e131337_d_n11, assign85650_e131337_d_n14,) = {
    if ((((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) && (locals.var_guard2001 == 0.0)) {
        let assign85650_e131335: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign85650_e131335, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn11 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn11)), ((locals.var_tmf3_dn14 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign85650_e131337;
        locals.var_tmf4_dn0 = assign85650_e131337_d_n0;
        locals.var_tmf4_dn2 = assign85650_e131337_d_n2;
        locals.var_tmf4_dn4 = assign85650_e131337_d_n4;
        locals.var_tmf4_dn5 = assign85650_e131337_d_n5;
        locals.var_tmf4_dn6 = assign85650_e131337_d_n6;
        locals.var_tmf4_dn7 = assign85650_e131337_d_n7;
        locals.var_tmf4_dn8 = assign85650_e131337_d_n8;
        locals.var_tmf4_dn9 = assign85650_e131337_d_n9;
        locals.var_tmf4_dn10 = assign85650_e131337_d_n10;
        locals.var_tmf4_dn11 = assign85650_e131337_d_n11;
        locals.var_tmf4_dn14 = assign85650_e131337_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign85660_e131361, assign85660_e131361_d_n0, assign85660_e131361_d_n2, assign85660_e131361_d_n4, assign85660_e131361_d_n5, assign85660_e131361_d_n6, assign85660_e131361_d_n7, assign85660_e131361_d_n8, assign85660_e131361_d_n9, assign85660_e131361_d_n10, assign85660_e131361_d_n11, assign85660_e131361_d_n14,) = {
    if ((((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) && (locals.var_guard2001 == 0.0)) {
        let assign85660_e131356: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign85660_e131358: f64 = (assign85660_e131356).powf(p.p113);
        let assign85660_e131359: f64 = (1.0 + assign85660_e131358);
        (assign85660_e131359, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85660_e131356).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85660_e131358 * (p.p113 * ((((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85660_e131356))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign85660_e131361;
        locals.var_tmf1_dn0 = assign85660_e131361_d_n0;
        locals.var_tmf1_dn2 = assign85660_e131361_d_n2;
        locals.var_tmf1_dn4 = assign85660_e131361_d_n4;
        locals.var_tmf1_dn5 = assign85660_e131361_d_n5;
        locals.var_tmf1_dn6 = assign85660_e131361_d_n6;
        locals.var_tmf1_dn7 = assign85660_e131361_d_n7;
        locals.var_tmf1_dn8 = assign85660_e131361_d_n8;
        locals.var_tmf1_dn9 = assign85660_e131361_d_n9;
        locals.var_tmf1_dn10 = assign85660_e131361_d_n10;
        locals.var_tmf1_dn11 = assign85660_e131361_d_n11;
        locals.var_tmf1_dn14 = assign85660_e131361_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign85670_e131383, assign85670_e131383_d_n0, assign85670_e131383_d_n2, assign85670_e131383_d_n4, assign85670_e131383_d_n5, assign85670_e131383_d_n6, assign85670_e131383_d_n7, assign85670_e131383_d_n8, assign85670_e131383_d_n9, assign85670_e131383_d_n10, assign85670_e131383_d_n11, assign85670_e131383_d_n14,) = {
    if ((((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) && (locals.var_guard2001 == 0.0)) {
        let assign85670_e131380: f64 = (1.0 / p.p113);
        let assign85670_e131381: f64 = (locals.var_tmf1).powf(assign85670_e131380);
        (assign85670_e131381, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn11)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn11 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85670_e131380) as f64).is_finite() && ((assign85670_e131380) as f64).fract() == 0.0 { if assign85670_e131380 == 0.0 { 0.0 } else { (assign85670_e131380 * ((locals.var_tmf1).powf(assign85670_e131380 - 1.0) * locals.var_tmf1_dn14)) } } else { (assign85670_e131381 * (assign85670_e131380 * (locals.var_tmf1_dn14 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign85670_e131383;
        locals.var_tmf2_dn0 = assign85670_e131383_d_n0;
        locals.var_tmf2_dn2 = assign85670_e131383_d_n2;
        locals.var_tmf2_dn4 = assign85670_e131383_d_n4;
        locals.var_tmf2_dn5 = assign85670_e131383_d_n5;
        locals.var_tmf2_dn6 = assign85670_e131383_d_n6;
        locals.var_tmf2_dn7 = assign85670_e131383_d_n7;
        locals.var_tmf2_dn8 = assign85670_e131383_d_n8;
        locals.var_tmf2_dn9 = assign85670_e131383_d_n9;
        locals.var_tmf2_dn10 = assign85670_e131383_d_n10;
        locals.var_tmf2_dn11 = assign85670_e131383_d_n11;
        locals.var_tmf2_dn14 = assign85670_e131383_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign85680_e131405, assign85680_e131405_d_n0, assign85680_e131405_d_n2, assign85680_e131405_d_n4, assign85680_e131405_d_n5, assign85680_e131405_d_n6, assign85680_e131405_d_n7, assign85680_e131405_d_n8, assign85680_e131405_d_n9, assign85680_e131405_d_n10, assign85680_e131405_d_n11, assign85680_e131405_d_n14,) = {
    if ((((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) && (locals.var_guard2001 == 0.0)) {
        let assign85680_e131401: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign85680_e131403: f64 = (assign85680_e131401 / locals.var_tmf2);
        (assign85680_e131403, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn11 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn11)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn14 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn14)) * locals.var_tmf2) - (assign85680_e131401 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign85680_e131405;
        locals.var_vxbgmt_dn0 = assign85680_e131405_d_n0;
        locals.var_vxbgmt_dn2 = assign85680_e131405_d_n2;
        locals.var_vxbgmt_dn4 = assign85680_e131405_d_n4;
        locals.var_vxbgmt_dn5 = assign85680_e131405_d_n5;
        locals.var_vxbgmt_dn6 = assign85680_e131405_d_n6;
        locals.var_vxbgmt_dn7 = assign85680_e131405_d_n7;
        locals.var_vxbgmt_dn8 = assign85680_e131405_d_n8;
        locals.var_vxbgmt_dn9 = assign85680_e131405_d_n9;
        locals.var_vxbgmt_dn10 = assign85680_e131405_d_n10;
        locals.var_vxbgmt_dn11 = assign85680_e131405_d_n11;
        locals.var_vxbgmt_dn14 = assign85680_e131405_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign85690_e131433, assign85690_e131433_d_n0, assign85690_e131433_d_n2, assign85690_e131433_d_n4, assign85690_e131433_d_n5, assign85690_e131433_d_n6, assign85690_e131433_d_n7, assign85690_e131433_d_n8, assign85690_e131433_d_n9, assign85690_e131433_d_n10, assign85690_e131433_d_n11, assign85690_e131433_d_n14,) = {
    if (((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        let assign85690_e131420: f64 = (locals.var_vxbgmt + p.p137);
        let assign85690_e131423: f64 = (locals.var_vxbgmt + p.p137);
        let assign85690_e131424: f64 = (assign85690_e131420 * assign85690_e131423);
        let assign85690_e131427: f64 = (4.0 * 0.1);
        let assign85690_e131429: f64 = (assign85690_e131427 * 0.1);
        let assign85690_e131430: f64 = (assign85690_e131424 + assign85690_e131429);
        let assign85690_e131431: f64 = (assign85690_e131430).sqrt();
        (assign85690_e131431, (((locals.var_vxbgmt_dn0 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn0)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn2 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn2)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn4 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn4)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn5 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn5)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn6 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn6)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn7 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn7)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn8 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn8)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn9 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn9)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn10 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn10)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn11 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn11)) / (2.0 * assign85690_e131431)), (((locals.var_vxbgmt_dn14 * assign85690_e131423) + (assign85690_e131420 * locals.var_vxbgmt_dn14)) / (2.0 * assign85690_e131431)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign85690_e131433;
        locals.var_tmf2_dn0 = assign85690_e131433_d_n0;
        locals.var_tmf2_dn2 = assign85690_e131433_d_n2;
        locals.var_tmf2_dn4 = assign85690_e131433_d_n4;
        locals.var_tmf2_dn5 = assign85690_e131433_d_n5;
        locals.var_tmf2_dn6 = assign85690_e131433_d_n6;
        locals.var_tmf2_dn7 = assign85690_e131433_d_n7;
        locals.var_tmf2_dn8 = assign85690_e131433_d_n8;
        locals.var_tmf2_dn9 = assign85690_e131433_d_n9;
        locals.var_tmf2_dn10 = assign85690_e131433_d_n10;
        locals.var_tmf2_dn11 = assign85690_e131433_d_n11;
        locals.var_tmf2_dn14 = assign85690_e131433_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign85700_e131456, assign85700_e131456_d_n0, assign85700_e131456_d_n2, assign85700_e131456_d_n4, assign85700_e131456_d_n5, assign85700_e131456_d_n6, assign85700_e131456_d_n7, assign85700_e131456_d_n8, assign85700_e131456_d_n9, assign85700_e131456_d_n10, assign85700_e131456_d_n11, assign85700_e131456_d_n14,) = {
    if (((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        let assign85700_e131450: f64 = (locals.var_vxbgmt + p.p137);
        let assign85700_e131452: f64 = (assign85700_e131450 / locals.var_tmf2);
        let assign85700_e131453: f64 = (1.0 + assign85700_e131452);
        let assign85700_e131454: f64 = (0.5 * assign85700_e131453);
        (assign85700_e131454, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn11 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn14 * locals.var_tmf2) - (assign85700_e131450 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign85700_e131456;
        locals.var_t9_dn0 = assign85700_e131456_d_n0;
        locals.var_t9_dn2 = assign85700_e131456_d_n2;
        locals.var_t9_dn4 = assign85700_e131456_d_n4;
        locals.var_t9_dn5 = assign85700_e131456_d_n5;
        locals.var_t9_dn6 = assign85700_e131456_d_n6;
        locals.var_t9_dn7 = assign85700_e131456_d_n7;
        locals.var_t9_dn8 = assign85700_e131456_d_n8;
        locals.var_t9_dn9 = assign85700_e131456_d_n9;
        locals.var_t9_dn10 = assign85700_e131456_d_n10;
        locals.var_t9_dn11 = assign85700_e131456_d_n11;
        locals.var_t9_dn14 = assign85700_e131456_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign85710_e131477, assign85710_e131477_d_n0, assign85710_e131477_d_n2, assign85710_e131477_d_n4, assign85710_e131477_d_n5, assign85710_e131477_d_n6, assign85710_e131477_d_n7, assign85710_e131477_d_n8, assign85710_e131477_d_n9, assign85710_e131477_d_n10, assign85710_e131477_d_n11, assign85710_e131477_d_n14,) = {
    if (((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        let assign85710_e131472: f64 = (locals.var_vxbgmt + p.p137);
        let assign85710_e131474: f64 = (assign85710_e131472 + locals.var_tmf2);
        let assign85710_e131475: f64 = (0.5 * assign85710_e131474);
        (assign85710_e131475, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vxbgmt_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign85710_e131477;
        locals.var_t2_dn0 = assign85710_e131477_d_n0;
        locals.var_t2_dn2 = assign85710_e131477_d_n2;
        locals.var_t2_dn4 = assign85710_e131477_d_n4;
        locals.var_t2_dn5 = assign85710_e131477_d_n5;
        locals.var_t2_dn6 = assign85710_e131477_d_n6;
        locals.var_t2_dn7 = assign85710_e131477_d_n7;
        locals.var_t2_dn8 = assign85710_e131477_d_n8;
        locals.var_t2_dn9 = assign85710_e131477_d_n9;
        locals.var_t2_dn10 = assign85710_e131477_d_n10;
        locals.var_t2_dn11 = assign85710_e131477_d_n11;
        locals.var_t2_dn14 = assign85710_e131477_d_n14;
        locals.var_t2_rv = 0.0;

        let assign85720_e131480: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2002 = assign85720_e131480;
        locals.var_guard2002_rv = 0.0;

        let (assign85730_e131497, assign85730_e131497_d_n0, assign85730_e131497_d_n2, assign85730_e131497_d_n4, assign85730_e131497_d_n5, assign85730_e131497_d_n6, assign85730_e131497_d_n7, assign85730_e131497_d_n8, assign85730_e131497_d_n9, assign85730_e131497_d_n10, assign85730_e131497_d_n11, assign85730_e131497_d_n14,) = {
    if ((((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) && (locals.var_guard2002 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign85730_e131497;
        locals.var_t2_dn0 = assign85730_e131497_d_n0;
        locals.var_t2_dn2 = assign85730_e131497_d_n2;
        locals.var_t2_dn4 = assign85730_e131497_d_n4;
        locals.var_t2_dn5 = assign85730_e131497_d_n5;
        locals.var_t2_dn6 = assign85730_e131497_d_n6;
        locals.var_t2_dn7 = assign85730_e131497_d_n7;
        locals.var_t2_dn8 = assign85730_e131497_d_n8;
        locals.var_t2_dn9 = assign85730_e131497_d_n9;
        locals.var_t2_dn10 = assign85730_e131497_d_n10;
        locals.var_t2_dn11 = assign85730_e131497_d_n11;
        locals.var_t2_dn14 = assign85730_e131497_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign85740_e131514, assign85740_e131514_d_n0, assign85740_e131514_d_n2, assign85740_e131514_d_n4, assign85740_e131514_d_n5, assign85740_e131514_d_n6, assign85740_e131514_d_n7, assign85740_e131514_d_n8, assign85740_e131514_d_n9, assign85740_e131514_d_n10, assign85740_e131514_d_n11, assign85740_e131514_d_n14,) = {
    if ((((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) && (locals.var_guard2002 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign85740_e131514;
        locals.var_t9_dn0 = assign85740_e131514_d_n0;
        locals.var_t9_dn2 = assign85740_e131514_d_n2;
        locals.var_t9_dn4 = assign85740_e131514_d_n4;
        locals.var_t9_dn5 = assign85740_e131514_d_n5;
        locals.var_t9_dn6 = assign85740_e131514_d_n6;
        locals.var_t9_dn7 = assign85740_e131514_d_n7;
        locals.var_t9_dn8 = assign85740_e131514_d_n8;
        locals.var_t9_dn9 = assign85740_e131514_d_n9;
        locals.var_t9_dn10 = assign85740_e131514_d_n10;
        locals.var_t9_dn11 = assign85740_e131514_d_n11;
        locals.var_t9_dn14 = assign85740_e131514_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign85750_e131534, assign85750_e131534_d_n0, assign85750_e131534_d_n2, assign85750_e131534_d_n4, assign85750_e131534_d_n5, assign85750_e131534_d_n6, assign85750_e131534_d_n7, assign85750_e131534_d_n8, assign85750_e131534_d_n9, assign85750_e131534_d_n10, assign85750_e131534_d_n11, assign85750_e131534_d_n14,) = {
    if (((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        let assign85750_e131529: f64 = (locals.var_kjunc * locals.var_t2);
        let assign85750_e131530: f64 = (assign85750_e131529).sqrt();
        let assign85750_e131532: f64 = (assign85750_e131530 * p.p432);
        (assign85750_e131532, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign85750_e131530)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign85750_e131530)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign85750_e131534;
        locals.var_wjunc0_dn0 = assign85750_e131534_d_n0;
        locals.var_wjunc0_dn2 = assign85750_e131534_d_n2;
        locals.var_wjunc0_dn4 = assign85750_e131534_d_n4;
        locals.var_wjunc0_dn5 = assign85750_e131534_d_n5;
        locals.var_wjunc0_dn6 = assign85750_e131534_d_n6;
        locals.var_wjunc0_dn7 = assign85750_e131534_d_n7;
        locals.var_wjunc0_dn8 = assign85750_e131534_d_n8;
        locals.var_wjunc0_dn9 = assign85750_e131534_d_n9;
        locals.var_wjunc0_dn10 = assign85750_e131534_d_n10;
        locals.var_wjunc0_dn11 = assign85750_e131534_d_n11;
        locals.var_wjunc0_dn14 = assign85750_e131534_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign85760_e131551, assign85760_e131551_d_n0, assign85760_e131551_d_n2, assign85760_e131551_d_n4, assign85760_e131551_d_n5, assign85760_e131551_d_n6, assign85760_e131551_d_n7, assign85760_e131551_d_n8, assign85760_e131551_d_n9, assign85760_e131551_d_n10, assign85760_e131551_d_n11, assign85760_e131551_d_n14,) = {
    if (((((locals.var_guard1994 != 0.0) && (!((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)))) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        let assign85760_e131549: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign85760_e131549, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn11 - locals.var_wjunc0_dn11), (locals.var_lover_func_dn14 - locals.var_wjunc0_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign85760_e131551;
        locals.var_lover_func_dn0 = assign85760_e131551_d_n0;
        locals.var_lover_func_dn2 = assign85760_e131551_d_n2;
        locals.var_lover_func_dn4 = assign85760_e131551_d_n4;
        locals.var_lover_func_dn5 = assign85760_e131551_d_n5;
        locals.var_lover_func_dn6 = assign85760_e131551_d_n6;
        locals.var_lover_func_dn7 = assign85760_e131551_d_n7;
        locals.var_lover_func_dn8 = assign85760_e131551_d_n8;
        locals.var_lover_func_dn9 = assign85760_e131551_d_n9;
        locals.var_lover_func_dn10 = assign85760_e131551_d_n10;
        locals.var_lover_func_dn11 = assign85760_e131551_d_n11;
        locals.var_lover_func_dn14 = assign85760_e131551_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign85770_e131570: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard2003 = assign85770_e131570;
        locals.var_guard2003_rv = 0.0;

        let (assign85780_e131583,) = {
    if (((locals.var_guard1995 != 0.0) && (!(((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)) || (locals.var_guard1994 != 0.0)))) && (locals.var_guard2003 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85780_e131583;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign85790_e131598, assign85790_e131598_d_n2, assign85790_e131598_d_n7, assign85790_e131598_d_n8, assign85790_e131598_d_n9,) = {
    if (((locals.var_guard1995 != 0.0) && (!(((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)) || (locals.var_guard1994 != 0.0)))) && (locals.var_guard2003 != 0.0)) {
        let assign85790_e131596: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign85790_e131596, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign85790_e131598;
        locals.var_vgbgmt_dn2 = assign85790_e131598_d_n2;
        locals.var_vgbgmt_dn7 = assign85790_e131598_d_n7;
        locals.var_vgbgmt_dn8 = assign85790_e131598_d_n8;
        locals.var_vgbgmt_dn9 = assign85790_e131598_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign85800_e131613, assign85800_e131613_d_n0, assign85800_e131613_d_n2, assign85800_e131613_d_n4, assign85800_e131613_d_n5, assign85800_e131613_d_n6, assign85800_e131613_d_n7, assign85800_e131613_d_n8, assign85800_e131613_d_n9, assign85800_e131613_d_n10, assign85800_e131613_d_n11, assign85800_e131613_d_n14,) = {
    if (((locals.var_guard1995 != 0.0) && (!(((locals.var_guard1992 != 0.0) || (locals.var_guard1993 != 0.0)) || (locals.var_guard1994 != 0.0)))) && (locals.var_guard2003 != 0.0)) {
        let assign85800_e131611: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign85800_e131611, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign85800_e131613;
        locals.var_vxbgmt_dn0 = assign85800_e131613_d_n0;
        locals.var_vxbgmt_dn2 = assign85800_e131613_d_n2;
        locals.var_vxbgmt_dn4 = assign85800_e131613_d_n4;
        locals.var_vxbgmt_dn5 = assign85800_e131613_d_n5;
        locals.var_vxbgmt_dn6 = assign85800_e131613_d_n6;
        locals.var_vxbgmt_dn7 = assign85800_e131613_d_n7;
        locals.var_vxbgmt_dn8 = assign85800_e131613_d_n8;
        locals.var_vxbgmt_dn9 = assign85800_e131613_d_n9;
        locals.var_vxbgmt_dn10 = assign85800_e131613_d_n10;
        locals.var_vxbgmt_dn11 = assign85800_e131613_d_n11;
        locals.var_vxbgmt_dn14 = assign85800_e131613_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign85810_e131617, assign85810_e131617_d_n0, assign85810_e131617_d_n2, assign85810_e131617_d_n4, assign85810_e131617_d_n5, assign85810_e131617_d_n6, assign85810_e131617_d_n7, assign85810_e131617_d_n8, assign85810_e131617_d_n9, assign85810_e131617_d_n10, assign85810_e131617_d_n11, assign85810_e131617_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2011, locals.var_vbs_bnd_over__blk2011_dn0, locals.var_vbs_bnd_over__blk2011_dn2, locals.var_vbs_bnd_over__blk2011_dn4, locals.var_vbs_bnd_over__blk2011_dn5, locals.var_vbs_bnd_over__blk2011_dn6, locals.var_vbs_bnd_over__blk2011_dn7, locals.var_vbs_bnd_over__blk2011_dn8, locals.var_vbs_bnd_over__blk2011_dn9, locals.var_vbs_bnd_over__blk2011_dn10, locals.var_vbs_bnd_over__blk2011_dn11, locals.var_vbs_bnd_over__blk2011_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2011 = assign85810_e131617;
        locals.var_vbs_bnd_over__blk2011_dn0 = assign85810_e131617_d_n0;
        locals.var_vbs_bnd_over__blk2011_dn2 = assign85810_e131617_d_n2;
        locals.var_vbs_bnd_over__blk2011_dn4 = assign85810_e131617_d_n4;
        locals.var_vbs_bnd_over__blk2011_dn5 = assign85810_e131617_d_n5;
        locals.var_vbs_bnd_over__blk2011_dn6 = assign85810_e131617_d_n6;
        locals.var_vbs_bnd_over__blk2011_dn7 = assign85810_e131617_d_n7;
        locals.var_vbs_bnd_over__blk2011_dn8 = assign85810_e131617_d_n8;
        locals.var_vbs_bnd_over__blk2011_dn9 = assign85810_e131617_d_n9;
        locals.var_vbs_bnd_over__blk2011_dn10 = assign85810_e131617_d_n10;
        locals.var_vbs_bnd_over__blk2011_dn11 = assign85810_e131617_d_n11;
        locals.var_vbs_bnd_over__blk2011_dn14 = assign85810_e131617_d_n14;
        locals.var_vbs_bnd_over__blk2011_rv = 0.0;

        let (assign85830_e131625,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk2012,)
    }
};
        locals.var_flg_fd_mode__blk2012 = assign85830_e131625;
        locals.var_flg_fd_mode__blk2012_rv = 0.0;

        let (assign85840_e131629, assign85840_e131629_d_n0, assign85840_e131629_d_n2, assign85840_e131629_d_n4, assign85840_e131629_d_n5, assign85840_e131629_d_n6, assign85840_e131629_d_n7, assign85840_e131629_d_n8, assign85840_e131629_d_n9, assign85840_e131629_d_n10, assign85840_e131629_d_n11, assign85840_e131629_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign85840_e131629;
        locals.var_fb_dn0 = assign85840_e131629_d_n0;
        locals.var_fb_dn2 = assign85840_e131629_d_n2;
        locals.var_fb_dn4 = assign85840_e131629_d_n4;
        locals.var_fb_dn5 = assign85840_e131629_d_n5;
        locals.var_fb_dn6 = assign85840_e131629_d_n6;
        locals.var_fb_dn7 = assign85840_e131629_d_n7;
        locals.var_fb_dn8 = assign85840_e131629_d_n8;
        locals.var_fb_dn9 = assign85840_e131629_d_n9;
        locals.var_fb_dn10 = assign85840_e131629_d_n10;
        locals.var_fb_dn11 = assign85840_e131629_d_n11;
        locals.var_fb_dn14 = assign85840_e131629_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign85850_e131633, assign85850_e131633_d_n0, assign85850_e131633_d_n2, assign85850_e131633_d_n4, assign85850_e131633_d_n5, assign85850_e131633_d_n6, assign85850_e131633_d_n7, assign85850_e131633_d_n8, assign85850_e131633_d_n9, assign85850_e131633_d_n10, assign85850_e131633_d_n11, assign85850_e131633_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
        locals.var_fs01 = assign85850_e131633;
        locals.var_fs01_dn0 = assign85850_e131633_d_n0;
        locals.var_fs01_dn2 = assign85850_e131633_d_n2;
        locals.var_fs01_dn4 = assign85850_e131633_d_n4;
        locals.var_fs01_dn5 = assign85850_e131633_d_n5;
        locals.var_fs01_dn6 = assign85850_e131633_d_n6;
        locals.var_fs01_dn7 = assign85850_e131633_d_n7;
        locals.var_fs01_dn8 = assign85850_e131633_d_n8;
        locals.var_fs01_dn9 = assign85850_e131633_d_n9;
        locals.var_fs01_dn10 = assign85850_e131633_d_n10;
        locals.var_fs01_dn11 = assign85850_e131633_d_n11;
        locals.var_fs01_dn14 = assign85850_e131633_d_n14;
        locals.var_fs01_rv = 0.0;

        let (assign85860_e131637, assign85860_e131637_d_n0, assign85860_e131637_d_n2, assign85860_e131637_d_n4, assign85860_e131637_d_n5, assign85860_e131637_d_n6, assign85860_e131637_d_n7, assign85860_e131637_d_n8, assign85860_e131637_d_n9, assign85860_e131637_d_n10, assign85860_e131637_d_n11, assign85860_e131637_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
        locals.var_fs02 = assign85860_e131637;
        locals.var_fs02_dn0 = assign85860_e131637_d_n0;
        locals.var_fs02_dn2 = assign85860_e131637_d_n2;
        locals.var_fs02_dn4 = assign85860_e131637_d_n4;
        locals.var_fs02_dn5 = assign85860_e131637_d_n5;
        locals.var_fs02_dn6 = assign85860_e131637_d_n6;
        locals.var_fs02_dn7 = assign85860_e131637_d_n7;
        locals.var_fs02_dn8 = assign85860_e131637_d_n8;
        locals.var_fs02_dn9 = assign85860_e131637_d_n9;
        locals.var_fs02_dn10 = assign85860_e131637_d_n10;
        locals.var_fs02_dn11 = assign85860_e131637_d_n11;
        locals.var_fs02_dn14 = assign85860_e131637_d_n14;
        locals.var_fs02_rv = 0.0;

        let (assign85870_e131641, assign85870_e131641_d_n0, assign85870_e131641_d_n2, assign85870_e131641_d_n4, assign85870_e131641_d_n5, assign85870_e131641_d_n6, assign85870_e131641_d_n7, assign85870_e131641_d_n8, assign85870_e131641_d_n9, assign85870_e131641_d_n10, assign85870_e131641_d_n11, assign85870_e131641_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
        locals.var_fs0 = assign85870_e131641;
        locals.var_fs0_dn0 = assign85870_e131641_d_n0;
        locals.var_fs0_dn2 = assign85870_e131641_d_n2;
        locals.var_fs0_dn4 = assign85870_e131641_d_n4;
        locals.var_fs0_dn5 = assign85870_e131641_d_n5;
        locals.var_fs0_dn6 = assign85870_e131641_d_n6;
        locals.var_fs0_dn7 = assign85870_e131641_d_n7;
        locals.var_fs0_dn8 = assign85870_e131641_d_n8;
        locals.var_fs0_dn9 = assign85870_e131641_d_n9;
        locals.var_fs0_dn10 = assign85870_e131641_d_n10;
        locals.var_fs0_dn11 = assign85870_e131641_d_n11;
        locals.var_fs0_dn14 = assign85870_e131641_d_n14;
        locals.var_fs0_rv = 0.0;

        let (assign85880_e131645, assign85880_e131645_d_n0, assign85880_e131645_d_n2, assign85880_e131645_d_n4, assign85880_e131645_d_n5, assign85880_e131645_d_n6, assign85880_e131645_d_n7, assign85880_e131645_d_n8, assign85880_e131645_d_n9, assign85880_e131645_d_n10, assign85880_e131645_d_n11, assign85880_e131645_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
        locals.var_dps0 = assign85880_e131645;
        locals.var_dps0_dn0 = assign85880_e131645_d_n0;
        locals.var_dps0_dn2 = assign85880_e131645_d_n2;
        locals.var_dps0_dn4 = assign85880_e131645_d_n4;
        locals.var_dps0_dn5 = assign85880_e131645_d_n5;
        locals.var_dps0_dn6 = assign85880_e131645_d_n6;
        locals.var_dps0_dn7 = assign85880_e131645_d_n7;
        locals.var_dps0_dn8 = assign85880_e131645_d_n8;
        locals.var_dps0_dn9 = assign85880_e131645_d_n9;
        locals.var_dps0_dn10 = assign85880_e131645_d_n10;
        locals.var_dps0_dn11 = assign85880_e131645_d_n11;
        locals.var_dps0_dn14 = assign85880_e131645_d_n14;
        locals.var_dps0_rv = 0.0;

        let (assign85890_e131649, assign85890_e131649_d_n0, assign85890_e131649_d_n2, assign85890_e131649_d_n4, assign85890_e131649_d_n5, assign85890_e131649_d_n6, assign85890_e131649_d_n7, assign85890_e131649_d_n8, assign85890_e131649_d_n9, assign85890_e131649_d_n10, assign85890_e131649_d_n11, assign85890_e131649_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
        locals.var_fs0_dps0 = assign85890_e131649;
        locals.var_fs0_dps0_dn0 = assign85890_e131649_d_n0;
        locals.var_fs0_dps0_dn2 = assign85890_e131649_d_n2;
        locals.var_fs0_dps0_dn4 = assign85890_e131649_d_n4;
        locals.var_fs0_dps0_dn5 = assign85890_e131649_d_n5;
        locals.var_fs0_dps0_dn6 = assign85890_e131649_d_n6;
        locals.var_fs0_dps0_dn7 = assign85890_e131649_d_n7;
        locals.var_fs0_dps0_dn8 = assign85890_e131649_d_n8;
        locals.var_fs0_dps0_dn9 = assign85890_e131649_d_n9;
        locals.var_fs0_dps0_dn10 = assign85890_e131649_d_n10;
        locals.var_fs0_dps0_dn11 = assign85890_e131649_d_n11;
        locals.var_fs0_dps0_dn14 = assign85890_e131649_d_n14;
        locals.var_fs0_dps0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_329(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign85900_e131653, assign85900_e131653_d_n0, assign85900_e131653_d_n2, assign85900_e131653_d_n4, assign85900_e131653_d_n5, assign85900_e131653_d_n6, assign85900_e131653_d_n7, assign85900_e131653_d_n8, assign85900_e131653_d_n9, assign85900_e131653_d_n10, assign85900_e131653_d_n11, assign85900_e131653_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
        locals.var_fs02_dps0 = assign85900_e131653;
        locals.var_fs02_dps0_dn0 = assign85900_e131653_d_n0;
        locals.var_fs02_dps0_dn2 = assign85900_e131653_d_n2;
        locals.var_fs02_dps0_dn4 = assign85900_e131653_d_n4;
        locals.var_fs02_dps0_dn5 = assign85900_e131653_d_n5;
        locals.var_fs02_dps0_dn6 = assign85900_e131653_d_n6;
        locals.var_fs02_dps0_dn7 = assign85900_e131653_d_n7;
        locals.var_fs02_dps0_dn8 = assign85900_e131653_d_n8;
        locals.var_fs02_dps0_dn9 = assign85900_e131653_d_n9;
        locals.var_fs02_dps0_dn10 = assign85900_e131653_d_n10;
        locals.var_fs02_dps0_dn11 = assign85900_e131653_d_n11;
        locals.var_fs02_dps0_dn14 = assign85900_e131653_d_n14;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign85910_e131657, assign85910_e131657_d_n0, assign85910_e131657_d_n2, assign85910_e131657_d_n4, assign85910_e131657_d_n5, assign85910_e131657_d_n6, assign85910_e131657_d_n7, assign85910_e131657_d_n8, assign85910_e131657_d_n9, assign85910_e131657_d_n10, assign85910_e131657_d_n11, assign85910_e131657_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
        locals.var_fb_dpss = assign85910_e131657;
        locals.var_fb_dpss_dn0 = assign85910_e131657_d_n0;
        locals.var_fb_dpss_dn2 = assign85910_e131657_d_n2;
        locals.var_fb_dpss_dn4 = assign85910_e131657_d_n4;
        locals.var_fb_dpss_dn5 = assign85910_e131657_d_n5;
        locals.var_fb_dpss_dn6 = assign85910_e131657_d_n6;
        locals.var_fb_dpss_dn7 = assign85910_e131657_d_n7;
        locals.var_fb_dpss_dn8 = assign85910_e131657_d_n8;
        locals.var_fb_dpss_dn9 = assign85910_e131657_d_n9;
        locals.var_fb_dpss_dn10 = assign85910_e131657_d_n10;
        locals.var_fb_dpss_dn11 = assign85910_e131657_d_n11;
        locals.var_fb_dpss_dn14 = assign85910_e131657_d_n14;
        locals.var_fb_dpss_rv = 0.0;

        let (assign85920_e131661, assign85920_e131661_d_n0, assign85920_e131661_d_n2, assign85920_e131661_d_n4, assign85920_e131661_d_n5, assign85920_e131661_d_n6, assign85920_e131661_d_n7, assign85920_e131661_d_n8, assign85920_e131661_d_n9, assign85920_e131661_d_n10, assign85920_e131661_d_n11, assign85920_e131661_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
        locals.var_fs01_dps0 = assign85920_e131661;
        locals.var_fs01_dps0_dn0 = assign85920_e131661_d_n0;
        locals.var_fs01_dps0_dn2 = assign85920_e131661_d_n2;
        locals.var_fs01_dps0_dn4 = assign85920_e131661_d_n4;
        locals.var_fs01_dps0_dn5 = assign85920_e131661_d_n5;
        locals.var_fs01_dps0_dn6 = assign85920_e131661_d_n6;
        locals.var_fs01_dps0_dn7 = assign85920_e131661_d_n7;
        locals.var_fs01_dps0_dn8 = assign85920_e131661_d_n8;
        locals.var_fs01_dps0_dn9 = assign85920_e131661_d_n9;
        locals.var_fs01_dps0_dn10 = assign85920_e131661_d_n10;
        locals.var_fs01_dps0_dn11 = assign85920_e131661_d_n11;
        locals.var_fs01_dps0_dn14 = assign85920_e131661_d_n14;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign85930_e131665, assign85930_e131665_d_n0, assign85930_e131665_d_n2, assign85930_e131665_d_n4, assign85930_e131665_d_n5, assign85930_e131665_d_n6, assign85930_e131665_d_n7, assign85930_e131665_d_n8, assign85930_e131665_d_n9, assign85930_e131665_d_n10, assign85930_e131665_d_n11, assign85930_e131665_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign85930_e131665;
        locals.var_chi_1_dn0 = assign85930_e131665_d_n0;
        locals.var_chi_1_dn2 = assign85930_e131665_d_n2;
        locals.var_chi_1_dn4 = assign85930_e131665_d_n4;
        locals.var_chi_1_dn5 = assign85930_e131665_d_n5;
        locals.var_chi_1_dn6 = assign85930_e131665_d_n6;
        locals.var_chi_1_dn7 = assign85930_e131665_d_n7;
        locals.var_chi_1_dn8 = assign85930_e131665_d_n8;
        locals.var_chi_1_dn9 = assign85930_e131665_d_n9;
        locals.var_chi_1_dn10 = assign85930_e131665_d_n10;
        locals.var_chi_1_dn11 = assign85930_e131665_d_n11;
        locals.var_chi_1_dn14 = assign85930_e131665_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign85940_e131669, assign85940_e131669_d_n0, assign85940_e131669_d_n2, assign85940_e131669_d_n4, assign85940_e131669_d_n5, assign85940_e131669_d_n6, assign85940_e131669_d_n7, assign85940_e131669_d_n8, assign85940_e131669_d_n9, assign85940_e131669_d_n10, assign85940_e131669_d_n11, assign85940_e131669_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign85940_e131669;
        locals.var_chi_a_dn0 = assign85940_e131669_d_n0;
        locals.var_chi_a_dn2 = assign85940_e131669_d_n2;
        locals.var_chi_a_dn4 = assign85940_e131669_d_n4;
        locals.var_chi_a_dn5 = assign85940_e131669_d_n5;
        locals.var_chi_a_dn6 = assign85940_e131669_d_n6;
        locals.var_chi_a_dn7 = assign85940_e131669_d_n7;
        locals.var_chi_a_dn8 = assign85940_e131669_d_n8;
        locals.var_chi_a_dn9 = assign85940_e131669_d_n9;
        locals.var_chi_a_dn10 = assign85940_e131669_d_n10;
        locals.var_chi_a_dn11 = assign85940_e131669_d_n11;
        locals.var_chi_a_dn14 = assign85940_e131669_d_n14;
        locals.var_chi_a_rv = 0.0;

        let (assign85950_e131673, assign85950_e131673_d_n0, assign85950_e131673_d_n2, assign85950_e131673_d_n4, assign85950_e131673_d_n5, assign85950_e131673_d_n6, assign85950_e131673_d_n7, assign85950_e131673_d_n8, assign85950_e131673_d_n9, assign85950_e131673_d_n10, assign85950_e131673_d_n11, assign85950_e131673_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign85950_e131673;
        locals.var_chi_b_dn0 = assign85950_e131673_d_n0;
        locals.var_chi_b_dn2 = assign85950_e131673_d_n2;
        locals.var_chi_b_dn4 = assign85950_e131673_d_n4;
        locals.var_chi_b_dn5 = assign85950_e131673_d_n5;
        locals.var_chi_b_dn6 = assign85950_e131673_d_n6;
        locals.var_chi_b_dn7 = assign85950_e131673_d_n7;
        locals.var_chi_b_dn8 = assign85950_e131673_d_n8;
        locals.var_chi_b_dn9 = assign85950_e131673_d_n9;
        locals.var_chi_b_dn10 = assign85950_e131673_d_n10;
        locals.var_chi_b_dn11 = assign85950_e131673_d_n11;
        locals.var_chi_b_dn14 = assign85950_e131673_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign85960_e131678,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85960_e131676: f64 = (-1.0);
        (assign85960_e131676,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign85960_e131678;
        locals.var_flg_conv_rv = 0.0;

        let (assign85970_e131682, assign85970_e131682_d_n0, assign85970_e131682_d_n2, assign85970_e131682_d_n4, assign85970_e131682_d_n5, assign85970_e131682_d_n6, assign85970_e131682_d_n7, assign85970_e131682_d_n8, assign85970_e131682_d_n9, assign85970_e131682_d_n10, assign85970_e131682_d_n11, assign85970_e131682_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk2013, locals.var_ps0ld_ini__blk2013_dn0, locals.var_ps0ld_ini__blk2013_dn2, locals.var_ps0ld_ini__blk2013_dn4, locals.var_ps0ld_ini__blk2013_dn5, locals.var_ps0ld_ini__blk2013_dn6, locals.var_ps0ld_ini__blk2013_dn7, locals.var_ps0ld_ini__blk2013_dn8, locals.var_ps0ld_ini__blk2013_dn9, locals.var_ps0ld_ini__blk2013_dn10, locals.var_ps0ld_ini__blk2013_dn11, locals.var_ps0ld_ini__blk2013_dn14,)
    }
};
        locals.var_ps0ld_ini__blk2013 = assign85970_e131682;
        locals.var_ps0ld_ini__blk2013_dn0 = assign85970_e131682_d_n0;
        locals.var_ps0ld_ini__blk2013_dn2 = assign85970_e131682_d_n2;
        locals.var_ps0ld_ini__blk2013_dn4 = assign85970_e131682_d_n4;
        locals.var_ps0ld_ini__blk2013_dn5 = assign85970_e131682_d_n5;
        locals.var_ps0ld_ini__blk2013_dn6 = assign85970_e131682_d_n6;
        locals.var_ps0ld_ini__blk2013_dn7 = assign85970_e131682_d_n7;
        locals.var_ps0ld_ini__blk2013_dn8 = assign85970_e131682_d_n8;
        locals.var_ps0ld_ini__blk2013_dn9 = assign85970_e131682_d_n9;
        locals.var_ps0ld_ini__blk2013_dn10 = assign85970_e131682_d_n10;
        locals.var_ps0ld_ini__blk2013_dn11 = assign85970_e131682_d_n11;
        locals.var_ps0ld_ini__blk2013_dn14 = assign85970_e131682_d_n14;
        locals.var_ps0ld_ini__blk2013_rv = 0.0;

        let (assign85980_e131686, assign85980_e131686_d_n0, assign85980_e131686_d_n2, assign85980_e131686_d_n4, assign85980_e131686_d_n5, assign85980_e131686_d_n6, assign85980_e131686_d_n7, assign85980_e131686_d_n8, assign85980_e131686_d_n9, assign85980_e131686_d_n10, assign85980_e131686_d_n11, assign85980_e131686_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk2014, locals.var_fbsq__blk2014_dn0, locals.var_fbsq__blk2014_dn2, locals.var_fbsq__blk2014_dn4, locals.var_fbsq__blk2014_dn5, locals.var_fbsq__blk2014_dn6, locals.var_fbsq__blk2014_dn7, locals.var_fbsq__blk2014_dn8, locals.var_fbsq__blk2014_dn9, locals.var_fbsq__blk2014_dn10, locals.var_fbsq__blk2014_dn11, locals.var_fbsq__blk2014_dn14,)
    }
};
        locals.var_fbsq__blk2014 = assign85980_e131686;
        locals.var_fbsq__blk2014_dn0 = assign85980_e131686_d_n0;
        locals.var_fbsq__blk2014_dn2 = assign85980_e131686_d_n2;
        locals.var_fbsq__blk2014_dn4 = assign85980_e131686_d_n4;
        locals.var_fbsq__blk2014_dn5 = assign85980_e131686_d_n5;
        locals.var_fbsq__blk2014_dn6 = assign85980_e131686_d_n6;
        locals.var_fbsq__blk2014_dn7 = assign85980_e131686_d_n7;
        locals.var_fbsq__blk2014_dn8 = assign85980_e131686_d_n8;
        locals.var_fbsq__blk2014_dn9 = assign85980_e131686_d_n9;
        locals.var_fbsq__blk2014_dn10 = assign85980_e131686_d_n10;
        locals.var_fbsq__blk2014_dn11 = assign85980_e131686_d_n11;
        locals.var_fbsq__blk2014_dn14 = assign85980_e131686_d_n14;
        locals.var_fbsq__blk2014_rv = 0.0;

        let (assign85990_e131697, assign85990_e131697_d_n0, assign85990_e131697_d_n2, assign85990_e131697_d_n4, assign85990_e131697_d_n5, assign85990_e131697_d_n6, assign85990_e131697_d_n7, assign85990_e131697_d_n8, assign85990_e131697_d_n9, assign85990_e131697_d_n10, assign85990_e131697_d_n11, assign85990_e131697_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85990_e131690: f64 = (2.0 * locals.var_beta_inv);
        let assign85990_e131693: f64 = (locals.var_nover_func / locals.var_nin);
        let assign85990_e131694: f64 = (assign85990_e131693).ln();
        let assign85990_e131695: f64 = (assign85990_e131690 * assign85990_e131694);
        (assign85990_e131695, (((2.0 * locals.var_beta_inv_dn0) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn2) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn4) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn5) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn6) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn7) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn8) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn9) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn10) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn11) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))), (((2.0 * locals.var_beta_inv_dn14) * assign85990_e131694) + (assign85990_e131690 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign85990_e131693))),)
    } else {
        (locals.var_pb2over__blk2009, locals.var_pb2over__blk2009_dn0, locals.var_pb2over__blk2009_dn2, locals.var_pb2over__blk2009_dn4, locals.var_pb2over__blk2009_dn5, locals.var_pb2over__blk2009_dn6, locals.var_pb2over__blk2009_dn7, locals.var_pb2over__blk2009_dn8, locals.var_pb2over__blk2009_dn9, locals.var_pb2over__blk2009_dn10, locals.var_pb2over__blk2009_dn11, locals.var_pb2over__blk2009_dn14,)
    }
};
        locals.var_pb2over__blk2009 = assign85990_e131697;
        locals.var_pb2over__blk2009_dn0 = assign85990_e131697_d_n0;
        locals.var_pb2over__blk2009_dn2 = assign85990_e131697_d_n2;
        locals.var_pb2over__blk2009_dn4 = assign85990_e131697_d_n4;
        locals.var_pb2over__blk2009_dn5 = assign85990_e131697_d_n5;
        locals.var_pb2over__blk2009_dn6 = assign85990_e131697_d_n6;
        locals.var_pb2over__blk2009_dn7 = assign85990_e131697_d_n7;
        locals.var_pb2over__blk2009_dn8 = assign85990_e131697_d_n8;
        locals.var_pb2over__blk2009_dn9 = assign85990_e131697_d_n9;
        locals.var_pb2over__blk2009_dn10 = assign85990_e131697_d_n10;
        locals.var_pb2over__blk2009_dn11 = assign85990_e131697_d_n11;
        locals.var_pb2over__blk2009_dn14 = assign85990_e131697_d_n14;
        locals.var_pb2over__blk2009_rv = 0.0;

        let (assign86000_e131705, assign86000_e131705_d_n0, assign86000_e131705_d_n2, assign86000_e131705_d_n4, assign86000_e131705_d_n5, assign86000_e131705_d_n6, assign86000_e131705_d_n7, assign86000_e131705_d_n8, assign86000_e131705_d_n9, assign86000_e131705_d_n10, assign86000_e131705_d_n11, assign86000_e131705_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86000_e131701: f64 = (0.8 - locals.var_pb2over__blk2009);
        let assign86000_e131703: f64 = (assign86000_e131701 - 0.1);
        (assign86000_e131703, (-locals.var_pb2over__blk2009_dn0), (-locals.var_pb2over__blk2009_dn2), (-locals.var_pb2over__blk2009_dn4), (-locals.var_pb2over__blk2009_dn5), (-locals.var_pb2over__blk2009_dn6), (-locals.var_pb2over__blk2009_dn7), (-locals.var_pb2over__blk2009_dn8), (-locals.var_pb2over__blk2009_dn9), (-locals.var_pb2over__blk2009_dn10), (-locals.var_pb2over__blk2009_dn11), (-locals.var_pb2over__blk2009_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign86000_e131705;
        locals.var_tmf1_dn0 = assign86000_e131705_d_n0;
        locals.var_tmf1_dn2 = assign86000_e131705_d_n2;
        locals.var_tmf1_dn4 = assign86000_e131705_d_n4;
        locals.var_tmf1_dn5 = assign86000_e131705_d_n5;
        locals.var_tmf1_dn6 = assign86000_e131705_d_n6;
        locals.var_tmf1_dn7 = assign86000_e131705_d_n7;
        locals.var_tmf1_dn8 = assign86000_e131705_d_n8;
        locals.var_tmf1_dn9 = assign86000_e131705_d_n9;
        locals.var_tmf1_dn10 = assign86000_e131705_d_n10;
        locals.var_tmf1_dn11 = assign86000_e131705_d_n11;
        locals.var_tmf1_dn14 = assign86000_e131705_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign86010_e131713, assign86010_e131713_d_n0, assign86010_e131713_d_n2, assign86010_e131713_d_n4, assign86010_e131713_d_n5, assign86010_e131713_d_n6, assign86010_e131713_d_n7, assign86010_e131713_d_n8, assign86010_e131713_d_n9, assign86010_e131713_d_n10, assign86010_e131713_d_n11, assign86010_e131713_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86010_e131709: f64 = (4.0 * 0.8);
        let assign86010_e131711: f64 = (assign86010_e131709 * 0.1);
        (assign86010_e131711, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign86010_e131713;
        locals.var_tmf2_dn0 = assign86010_e131713_d_n0;
        locals.var_tmf2_dn2 = assign86010_e131713_d_n2;
        locals.var_tmf2_dn4 = assign86010_e131713_d_n4;
        locals.var_tmf2_dn5 = assign86010_e131713_d_n5;
        locals.var_tmf2_dn6 = assign86010_e131713_d_n6;
        locals.var_tmf2_dn7 = assign86010_e131713_d_n7;
        locals.var_tmf2_dn8 = assign86010_e131713_d_n8;
        locals.var_tmf2_dn9 = assign86010_e131713_d_n9;
        locals.var_tmf2_dn10 = assign86010_e131713_d_n10;
        locals.var_tmf2_dn11 = assign86010_e131713_d_n11;
        locals.var_tmf2_dn14 = assign86010_e131713_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign86020_e131723, assign86020_e131723_d_n0, assign86020_e131723_d_n2, assign86020_e131723_d_n4, assign86020_e131723_d_n5, assign86020_e131723_d_n6, assign86020_e131723_d_n7, assign86020_e131723_d_n8, assign86020_e131723_d_n9, assign86020_e131723_d_n10, assign86020_e131723_d_n11, assign86020_e131723_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign86020_e131721, assign86020_e131721_d_n0, assign86020_e131721_d_n2, assign86020_e131721_d_n4, assign86020_e131721_d_n5, assign86020_e131721_d_n6, assign86020_e131721_d_n7, assign86020_e131721_d_n8, assign86020_e131721_d_n9, assign86020_e131721_d_n10, assign86020_e131721_d_n11, assign86020_e131721_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign86020_e131720: f64 = (-locals.var_tmf2);
                (assign86020_e131720, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign86020_e131721, assign86020_e131721_d_n0, assign86020_e131721_d_n2, assign86020_e131721_d_n4, assign86020_e131721_d_n5, assign86020_e131721_d_n6, assign86020_e131721_d_n7, assign86020_e131721_d_n8, assign86020_e131721_d_n9, assign86020_e131721_d_n10, assign86020_e131721_d_n11, assign86020_e131721_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign86020_e131723;
        locals.var_tmf2_dn0 = assign86020_e131723_d_n0;
        locals.var_tmf2_dn2 = assign86020_e131723_d_n2;
        locals.var_tmf2_dn4 = assign86020_e131723_d_n4;
        locals.var_tmf2_dn5 = assign86020_e131723_d_n5;
        locals.var_tmf2_dn6 = assign86020_e131723_d_n6;
        locals.var_tmf2_dn7 = assign86020_e131723_d_n7;
        locals.var_tmf2_dn8 = assign86020_e131723_d_n8;
        locals.var_tmf2_dn9 = assign86020_e131723_d_n9;
        locals.var_tmf2_dn10 = assign86020_e131723_d_n10;
        locals.var_tmf2_dn11 = assign86020_e131723_d_n11;
        locals.var_tmf2_dn14 = assign86020_e131723_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign86030_e131732, assign86030_e131732_d_n0, assign86030_e131732_d_n2, assign86030_e131732_d_n4, assign86030_e131732_d_n5, assign86030_e131732_d_n6, assign86030_e131732_d_n7, assign86030_e131732_d_n8, assign86030_e131732_d_n9, assign86030_e131732_d_n10, assign86030_e131732_d_n11, assign86030_e131732_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86030_e131727: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign86030_e131729: f64 = (assign86030_e131727 + locals.var_tmf2);
        let assign86030_e131730: f64 = (assign86030_e131729).sqrt();
        (assign86030_e131730, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign86030_e131730)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign86030_e131730)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign86030_e131732;
        locals.var_tmf2_dn0 = assign86030_e131732_d_n0;
        locals.var_tmf2_dn2 = assign86030_e131732_d_n2;
        locals.var_tmf2_dn4 = assign86030_e131732_d_n4;
        locals.var_tmf2_dn5 = assign86030_e131732_d_n5;
        locals.var_tmf2_dn6 = assign86030_e131732_d_n6;
        locals.var_tmf2_dn7 = assign86030_e131732_d_n7;
        locals.var_tmf2_dn8 = assign86030_e131732_d_n8;
        locals.var_tmf2_dn9 = assign86030_e131732_d_n9;
        locals.var_tmf2_dn10 = assign86030_e131732_d_n10;
        locals.var_tmf2_dn11 = assign86030_e131732_d_n11;
        locals.var_tmf2_dn14 = assign86030_e131732_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign86040_e131742, assign86040_e131742_d_n0, assign86040_e131742_d_n2, assign86040_e131742_d_n4, assign86040_e131742_d_n5, assign86040_e131742_d_n6, assign86040_e131742_d_n7, assign86040_e131742_d_n8, assign86040_e131742_d_n9, assign86040_e131742_d_n10, assign86040_e131742_d_n11, assign86040_e131742_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86040_e131738: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign86040_e131739: f64 = (1.0 + assign86040_e131738);
        let assign86040_e131740: f64 = (0.5 * assign86040_e131739);
        (assign86040_e131740, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86040_e131742;
        locals.var_t0_dn0 = assign86040_e131742_d_n0;
        locals.var_t0_dn2 = assign86040_e131742_d_n2;
        locals.var_t0_dn4 = assign86040_e131742_d_n4;
        locals.var_t0_dn5 = assign86040_e131742_d_n5;
        locals.var_t0_dn6 = assign86040_e131742_d_n6;
        locals.var_t0_dn7 = assign86040_e131742_d_n7;
        locals.var_t0_dn8 = assign86040_e131742_d_n8;
        locals.var_t0_dn9 = assign86040_e131742_d_n9;
        locals.var_t0_dn10 = assign86040_e131742_d_n10;
        locals.var_t0_dn11 = assign86040_e131742_d_n11;
        locals.var_t0_dn14 = assign86040_e131742_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86050_e131752, assign86050_e131752_d_n0, assign86050_e131752_d_n2, assign86050_e131752_d_n4, assign86050_e131752_d_n5, assign86050_e131752_d_n6, assign86050_e131752_d_n7, assign86050_e131752_d_n8, assign86050_e131752_d_n9, assign86050_e131752_d_n10, assign86050_e131752_d_n11, assign86050_e131752_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86050_e131748: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign86050_e131749: f64 = (0.5 * assign86050_e131748);
        let assign86050_e131750: f64 = (0.8 - assign86050_e131749);
        (assign86050_e131750, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over__blk2010, locals.var_vbs_max_over__blk2010_dn0, locals.var_vbs_max_over__blk2010_dn2, locals.var_vbs_max_over__blk2010_dn4, locals.var_vbs_max_over__blk2010_dn5, locals.var_vbs_max_over__blk2010_dn6, locals.var_vbs_max_over__blk2010_dn7, locals.var_vbs_max_over__blk2010_dn8, locals.var_vbs_max_over__blk2010_dn9, locals.var_vbs_max_over__blk2010_dn10, locals.var_vbs_max_over__blk2010_dn11, locals.var_vbs_max_over__blk2010_dn14,)
    }
};
        locals.var_vbs_max_over__blk2010 = assign86050_e131752;
        locals.var_vbs_max_over__blk2010_dn0 = assign86050_e131752_d_n0;
        locals.var_vbs_max_over__blk2010_dn2 = assign86050_e131752_d_n2;
        locals.var_vbs_max_over__blk2010_dn4 = assign86050_e131752_d_n4;
        locals.var_vbs_max_over__blk2010_dn5 = assign86050_e131752_d_n5;
        locals.var_vbs_max_over__blk2010_dn6 = assign86050_e131752_d_n6;
        locals.var_vbs_max_over__blk2010_dn7 = assign86050_e131752_d_n7;
        locals.var_vbs_max_over__blk2010_dn8 = assign86050_e131752_d_n8;
        locals.var_vbs_max_over__blk2010_dn9 = assign86050_e131752_d_n9;
        locals.var_vbs_max_over__blk2010_dn10 = assign86050_e131752_d_n10;
        locals.var_vbs_max_over__blk2010_dn11 = assign86050_e131752_d_n11;
        locals.var_vbs_max_over__blk2010_dn14 = assign86050_e131752_d_n14;
        locals.var_vbs_max_over__blk2010_rv = 0.0;

        let assign86060_e131756: f64 = (locals.var_vbs_max_over__blk2010 * 0.5);
        let assign86060_e131757: f64 = if locals.var_vbs_bnd_over__blk2011 > assign86060_e131756 { 1.0 } else { 0.0 };
        locals.var_guard2016 = assign86060_e131757;
        locals.var_guard2016_rv = 0.0;

        let (assign86070_e131765, assign86070_e131765_d_n0, assign86070_e131765_d_n2, assign86070_e131765_d_n4, assign86070_e131765_d_n5, assign86070_e131765_d_n6, assign86070_e131765_d_n7, assign86070_e131765_d_n8, assign86070_e131765_d_n9, assign86070_e131765_d_n10, assign86070_e131765_d_n11, assign86070_e131765_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2016 != 0.0)) {
        let assign86070_e131763: f64 = (0.5 * locals.var_vbs_max_over__blk2010);
        (assign86070_e131763, (0.5 * locals.var_vbs_max_over__blk2010_dn0), (0.5 * locals.var_vbs_max_over__blk2010_dn2), (0.5 * locals.var_vbs_max_over__blk2010_dn4), (0.5 * locals.var_vbs_max_over__blk2010_dn5), (0.5 * locals.var_vbs_max_over__blk2010_dn6), (0.5 * locals.var_vbs_max_over__blk2010_dn7), (0.5 * locals.var_vbs_max_over__blk2010_dn8), (0.5 * locals.var_vbs_max_over__blk2010_dn9), (0.5 * locals.var_vbs_max_over__blk2010_dn10), (0.5 * locals.var_vbs_max_over__blk2010_dn11), (0.5 * locals.var_vbs_max_over__blk2010_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2011, locals.var_vbs_bnd_over__blk2011_dn0, locals.var_vbs_bnd_over__blk2011_dn2, locals.var_vbs_bnd_over__blk2011_dn4, locals.var_vbs_bnd_over__blk2011_dn5, locals.var_vbs_bnd_over__blk2011_dn6, locals.var_vbs_bnd_over__blk2011_dn7, locals.var_vbs_bnd_over__blk2011_dn8, locals.var_vbs_bnd_over__blk2011_dn9, locals.var_vbs_bnd_over__blk2011_dn10, locals.var_vbs_bnd_over__blk2011_dn11, locals.var_vbs_bnd_over__blk2011_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2011 = assign86070_e131765;
        locals.var_vbs_bnd_over__blk2011_dn0 = assign86070_e131765_d_n0;
        locals.var_vbs_bnd_over__blk2011_dn2 = assign86070_e131765_d_n2;
        locals.var_vbs_bnd_over__blk2011_dn4 = assign86070_e131765_d_n4;
        locals.var_vbs_bnd_over__blk2011_dn5 = assign86070_e131765_d_n5;
        locals.var_vbs_bnd_over__blk2011_dn6 = assign86070_e131765_d_n6;
        locals.var_vbs_bnd_over__blk2011_dn7 = assign86070_e131765_d_n7;
        locals.var_vbs_bnd_over__blk2011_dn8 = assign86070_e131765_d_n8;
        locals.var_vbs_bnd_over__blk2011_dn9 = assign86070_e131765_d_n9;
        locals.var_vbs_bnd_over__blk2011_dn10 = assign86070_e131765_d_n10;
        locals.var_vbs_bnd_over__blk2011_dn11 = assign86070_e131765_d_n11;
        locals.var_vbs_bnd_over__blk2011_dn14 = assign86070_e131765_d_n14;
        locals.var_vbs_bnd_over__blk2011_rv = 0.0;

        let assign86080_e131767: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2017 = assign86080_e131767;
        locals.var_guard2017_rv = 0.0;

        let (assign86090_e131773, assign86090_e131773_d_n0, assign86090_e131773_d_n2, assign86090_e131773_d_n4, assign86090_e131773_d_n5, assign86090_e131773_d_n6, assign86090_e131773_d_n7, assign86090_e131773_d_n8, assign86090_e131773_d_n9, assign86090_e131773_d_n10, assign86090_e131773_d_n11, assign86090_e131773_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2017 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk2010, locals.var_vbs_max_over__blk2010_dn0, locals.var_vbs_max_over__blk2010_dn2, locals.var_vbs_max_over__blk2010_dn4, locals.var_vbs_max_over__blk2010_dn5, locals.var_vbs_max_over__blk2010_dn6, locals.var_vbs_max_over__blk2010_dn7, locals.var_vbs_max_over__blk2010_dn8, locals.var_vbs_max_over__blk2010_dn9, locals.var_vbs_max_over__blk2010_dn10, locals.var_vbs_max_over__blk2010_dn11, locals.var_vbs_max_over__blk2010_dn14,)
    }
};
        locals.var_vbs_max_over__blk2010 = assign86090_e131773;
        locals.var_vbs_max_over__blk2010_dn0 = assign86090_e131773_d_n0;
        locals.var_vbs_max_over__blk2010_dn2 = assign86090_e131773_d_n2;
        locals.var_vbs_max_over__blk2010_dn4 = assign86090_e131773_d_n4;
        locals.var_vbs_max_over__blk2010_dn5 = assign86090_e131773_d_n5;
        locals.var_vbs_max_over__blk2010_dn6 = assign86090_e131773_d_n6;
        locals.var_vbs_max_over__blk2010_dn7 = assign86090_e131773_d_n7;
        locals.var_vbs_max_over__blk2010_dn8 = assign86090_e131773_d_n8;
        locals.var_vbs_max_over__blk2010_dn9 = assign86090_e131773_d_n9;
        locals.var_vbs_max_over__blk2010_dn10 = assign86090_e131773_d_n10;
        locals.var_vbs_max_over__blk2010_dn11 = assign86090_e131773_d_n11;
        locals.var_vbs_max_over__blk2010_dn14 = assign86090_e131773_d_n14;
        locals.var_vbs_max_over__blk2010_rv = 0.0;

        let assign86100_e131775: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard2018 = assign86100_e131775;
        locals.var_guard2018_rv = 0.0;

        let (assign86110_e131781, assign86110_e131781_d_n0, assign86110_e131781_d_n2, assign86110_e131781_d_n4, assign86110_e131781_d_n5, assign86110_e131781_d_n6, assign86110_e131781_d_n7, assign86110_e131781_d_n8, assign86110_e131781_d_n9, assign86110_e131781_d_n10, assign86110_e131781_d_n11, assign86110_e131781_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2018 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2011, locals.var_vbs_bnd_over__blk2011_dn0, locals.var_vbs_bnd_over__blk2011_dn2, locals.var_vbs_bnd_over__blk2011_dn4, locals.var_vbs_bnd_over__blk2011_dn5, locals.var_vbs_bnd_over__blk2011_dn6, locals.var_vbs_bnd_over__blk2011_dn7, locals.var_vbs_bnd_over__blk2011_dn8, locals.var_vbs_bnd_over__blk2011_dn9, locals.var_vbs_bnd_over__blk2011_dn10, locals.var_vbs_bnd_over__blk2011_dn11, locals.var_vbs_bnd_over__blk2011_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2011 = assign86110_e131781;
        locals.var_vbs_bnd_over__blk2011_dn0 = assign86110_e131781_d_n0;
        locals.var_vbs_bnd_over__blk2011_dn2 = assign86110_e131781_d_n2;
        locals.var_vbs_bnd_over__blk2011_dn4 = assign86110_e131781_d_n4;
        locals.var_vbs_bnd_over__blk2011_dn5 = assign86110_e131781_d_n5;
        locals.var_vbs_bnd_over__blk2011_dn6 = assign86110_e131781_d_n6;
        locals.var_vbs_bnd_over__blk2011_dn7 = assign86110_e131781_d_n7;
        locals.var_vbs_bnd_over__blk2011_dn8 = assign86110_e131781_d_n8;
        locals.var_vbs_bnd_over__blk2011_dn9 = assign86110_e131781_d_n9;
        locals.var_vbs_bnd_over__blk2011_dn10 = assign86110_e131781_d_n10;
        locals.var_vbs_bnd_over__blk2011_dn11 = assign86110_e131781_d_n11;
        locals.var_vbs_bnd_over__blk2011_dn14 = assign86110_e131781_d_n14;
        locals.var_vbs_bnd_over__blk2011_rv = 0.0;

        let assign86120_e131783: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2019 = assign86120_e131783;
        locals.var_guard2019_rv = 0.0;

        let (assign86130_e131794, assign86130_e131794_d_n0, assign86130_e131794_d_n2, assign86130_e131794_d_n4, assign86130_e131794_d_n5, assign86130_e131794_d_n6, assign86130_e131794_d_n7, assign86130_e131794_d_n8, assign86130_e131794_d_n9, assign86130_e131794_d_n10, assign86130_e131794_d_n11, assign86130_e131794_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2018 == 0.0)) && (locals.var_guard2019 != 0.0)) {
        let assign86130_e131792: f64 = (0.5 * locals.var_vbs_max_over__blk2010);
        (assign86130_e131792, (0.5 * locals.var_vbs_max_over__blk2010_dn0), (0.5 * locals.var_vbs_max_over__blk2010_dn2), (0.5 * locals.var_vbs_max_over__blk2010_dn4), (0.5 * locals.var_vbs_max_over__blk2010_dn5), (0.5 * locals.var_vbs_max_over__blk2010_dn6), (0.5 * locals.var_vbs_max_over__blk2010_dn7), (0.5 * locals.var_vbs_max_over__blk2010_dn8), (0.5 * locals.var_vbs_max_over__blk2010_dn9), (0.5 * locals.var_vbs_max_over__blk2010_dn10), (0.5 * locals.var_vbs_max_over__blk2010_dn11), (0.5 * locals.var_vbs_max_over__blk2010_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2011, locals.var_vbs_bnd_over__blk2011_dn0, locals.var_vbs_bnd_over__blk2011_dn2, locals.var_vbs_bnd_over__blk2011_dn4, locals.var_vbs_bnd_over__blk2011_dn5, locals.var_vbs_bnd_over__blk2011_dn6, locals.var_vbs_bnd_over__blk2011_dn7, locals.var_vbs_bnd_over__blk2011_dn8, locals.var_vbs_bnd_over__blk2011_dn9, locals.var_vbs_bnd_over__blk2011_dn10, locals.var_vbs_bnd_over__blk2011_dn11, locals.var_vbs_bnd_over__blk2011_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2011 = assign86130_e131794;
        locals.var_vbs_bnd_over__blk2011_dn0 = assign86130_e131794_d_n0;
        locals.var_vbs_bnd_over__blk2011_dn2 = assign86130_e131794_d_n2;
        locals.var_vbs_bnd_over__blk2011_dn4 = assign86130_e131794_d_n4;
        locals.var_vbs_bnd_over__blk2011_dn5 = assign86130_e131794_d_n5;
        locals.var_vbs_bnd_over__blk2011_dn6 = assign86130_e131794_d_n6;
        locals.var_vbs_bnd_over__blk2011_dn7 = assign86130_e131794_d_n7;
        locals.var_vbs_bnd_over__blk2011_dn8 = assign86130_e131794_d_n8;
        locals.var_vbs_bnd_over__blk2011_dn9 = assign86130_e131794_d_n9;
        locals.var_vbs_bnd_over__blk2011_dn10 = assign86130_e131794_d_n10;
        locals.var_vbs_bnd_over__blk2011_dn11 = assign86130_e131794_d_n11;
        locals.var_vbs_bnd_over__blk2011_dn14 = assign86130_e131794_d_n14;
        locals.var_vbs_bnd_over__blk2011_rv = 0.0;

        let assign86140_e131798: f64 = (locals.var_vbs_max_over__blk2010 * 0.5);
        let assign86140_e131799: f64 = if locals.var_vbs_bnd_over__blk2011 > assign86140_e131798 { 1.0 } else { 0.0 };
        locals.var_guard2020 = assign86140_e131799;
        locals.var_guard2020_rv = 0.0;

        let (assign86150_e131807, assign86150_e131807_d_n0, assign86150_e131807_d_n2, assign86150_e131807_d_n4, assign86150_e131807_d_n5, assign86150_e131807_d_n6, assign86150_e131807_d_n7, assign86150_e131807_d_n8, assign86150_e131807_d_n9, assign86150_e131807_d_n10, assign86150_e131807_d_n11, assign86150_e131807_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2020 != 0.0)) {
        let assign86150_e131805: f64 = (0.5 * locals.var_vbs_max_over__blk2010);
        (assign86150_e131805, (0.5 * locals.var_vbs_max_over__blk2010_dn0), (0.5 * locals.var_vbs_max_over__blk2010_dn2), (0.5 * locals.var_vbs_max_over__blk2010_dn4), (0.5 * locals.var_vbs_max_over__blk2010_dn5), (0.5 * locals.var_vbs_max_over__blk2010_dn6), (0.5 * locals.var_vbs_max_over__blk2010_dn7), (0.5 * locals.var_vbs_max_over__blk2010_dn8), (0.5 * locals.var_vbs_max_over__blk2010_dn9), (0.5 * locals.var_vbs_max_over__blk2010_dn10), (0.5 * locals.var_vbs_max_over__blk2010_dn11), (0.5 * locals.var_vbs_max_over__blk2010_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2011, locals.var_vbs_bnd_over__blk2011_dn0, locals.var_vbs_bnd_over__blk2011_dn2, locals.var_vbs_bnd_over__blk2011_dn4, locals.var_vbs_bnd_over__blk2011_dn5, locals.var_vbs_bnd_over__blk2011_dn6, locals.var_vbs_bnd_over__blk2011_dn7, locals.var_vbs_bnd_over__blk2011_dn8, locals.var_vbs_bnd_over__blk2011_dn9, locals.var_vbs_bnd_over__blk2011_dn10, locals.var_vbs_bnd_over__blk2011_dn11, locals.var_vbs_bnd_over__blk2011_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2011 = assign86150_e131807;
        locals.var_vbs_bnd_over__blk2011_dn0 = assign86150_e131807_d_n0;
        locals.var_vbs_bnd_over__blk2011_dn2 = assign86150_e131807_d_n2;
        locals.var_vbs_bnd_over__blk2011_dn4 = assign86150_e131807_d_n4;
        locals.var_vbs_bnd_over__blk2011_dn5 = assign86150_e131807_d_n5;
        locals.var_vbs_bnd_over__blk2011_dn6 = assign86150_e131807_d_n6;
        locals.var_vbs_bnd_over__blk2011_dn7 = assign86150_e131807_d_n7;
        locals.var_vbs_bnd_over__blk2011_dn8 = assign86150_e131807_d_n8;
        locals.var_vbs_bnd_over__blk2011_dn9 = assign86150_e131807_d_n9;
        locals.var_vbs_bnd_over__blk2011_dn10 = assign86150_e131807_d_n10;
        locals.var_vbs_bnd_over__blk2011_dn11 = assign86150_e131807_d_n11;
        locals.var_vbs_bnd_over__blk2011_dn14 = assign86150_e131807_d_n14;
        locals.var_vbs_bnd_over__blk2011_rv = 0.0;

        let assign86160_e131810: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2021 = assign86160_e131810;
        locals.var_guard2021_rv = 0.0;

        let (assign86170_e131817, assign86170_e131817_d_n0, assign86170_e131817_d_n2, assign86170_e131817_d_n4, assign86170_e131817_d_n5, assign86170_e131817_d_n6, assign86170_e131817_d_n7, assign86170_e131817_d_n8, assign86170_e131817_d_n9, assign86170_e131817_d_n10, assign86170_e131817_d_n11, assign86170_e131817_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) {
        let assign86170_e131815: f64 = (-locals.var_vxbgmt);
        (assign86170_e131815, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86170_e131817;
        locals.var_t0_dn0 = assign86170_e131817_d_n0;
        locals.var_t0_dn2 = assign86170_e131817_d_n2;
        locals.var_t0_dn4 = assign86170_e131817_d_n4;
        locals.var_t0_dn5 = assign86170_e131817_d_n5;
        locals.var_t0_dn6 = assign86170_e131817_d_n6;
        locals.var_t0_dn7 = assign86170_e131817_d_n7;
        locals.var_t0_dn8 = assign86170_e131817_d_n8;
        locals.var_t0_dn9 = assign86170_e131817_d_n9;
        locals.var_t0_dn10 = assign86170_e131817_d_n10;
        locals.var_t0_dn11 = assign86170_e131817_d_n11;
        locals.var_t0_dn14 = assign86170_e131817_d_n14;
        locals.var_t0_rv = 0.0;

        let assign86180_e131820: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk2011 { 1.0 } else { 0.0 };
        locals.var_guard2022 = assign86180_e131820;
        locals.var_guard2022_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_330(
        locals: &mut StampLocals,
    ) {
        let (assign86190_e131830, assign86190_e131830_d_n0, assign86190_e131830_d_n2, assign86190_e131830_d_n4, assign86190_e131830_d_n5, assign86190_e131830_d_n6, assign86190_e131830_d_n7, assign86190_e131830_d_n8, assign86190_e131830_d_n9, assign86190_e131830_d_n10, assign86190_e131830_d_n11, assign86190_e131830_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86190_e131828: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk2011);
        (assign86190_e131828, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk2011_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk2011_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk2011_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk2011_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk2011_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk2011_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk2011_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk2011_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk2011_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over__blk2011_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over__blk2011_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign86190_e131830;
        locals.var_t1_dn0 = assign86190_e131830_d_n0;
        locals.var_t1_dn2 = assign86190_e131830_d_n2;
        locals.var_t1_dn4 = assign86190_e131830_d_n4;
        locals.var_t1_dn5 = assign86190_e131830_d_n5;
        locals.var_t1_dn6 = assign86190_e131830_d_n6;
        locals.var_t1_dn7 = assign86190_e131830_d_n7;
        locals.var_t1_dn8 = assign86190_e131830_d_n8;
        locals.var_t1_dn9 = assign86190_e131830_d_n9;
        locals.var_t1_dn10 = assign86190_e131830_d_n10;
        locals.var_t1_dn11 = assign86190_e131830_d_n11;
        locals.var_t1_dn14 = assign86190_e131830_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign86200_e131840, assign86200_e131840_d_n0, assign86200_e131840_d_n2, assign86200_e131840_d_n4, assign86200_e131840_d_n5, assign86200_e131840_d_n6, assign86200_e131840_d_n7, assign86200_e131840_d_n8, assign86200_e131840_d_n9, assign86200_e131840_d_n10, assign86200_e131840_d_n11, assign86200_e131840_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86200_e131838: f64 = (locals.var_vbs_max_over__blk2010 - locals.var_vbs_bnd_over__blk2011);
        (assign86200_e131838, (locals.var_vbs_max_over__blk2010_dn0 - locals.var_vbs_bnd_over__blk2011_dn0), (locals.var_vbs_max_over__blk2010_dn2 - locals.var_vbs_bnd_over__blk2011_dn2), (locals.var_vbs_max_over__blk2010_dn4 - locals.var_vbs_bnd_over__blk2011_dn4), (locals.var_vbs_max_over__blk2010_dn5 - locals.var_vbs_bnd_over__blk2011_dn5), (locals.var_vbs_max_over__blk2010_dn6 - locals.var_vbs_bnd_over__blk2011_dn6), (locals.var_vbs_max_over__blk2010_dn7 - locals.var_vbs_bnd_over__blk2011_dn7), (locals.var_vbs_max_over__blk2010_dn8 - locals.var_vbs_bnd_over__blk2011_dn8), (locals.var_vbs_max_over__blk2010_dn9 - locals.var_vbs_bnd_over__blk2011_dn9), (locals.var_vbs_max_over__blk2010_dn10 - locals.var_vbs_bnd_over__blk2011_dn10), (locals.var_vbs_max_over__blk2010_dn11 - locals.var_vbs_bnd_over__blk2011_dn11), (locals.var_vbs_max_over__blk2010_dn14 - locals.var_vbs_bnd_over__blk2011_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign86200_e131840;
        locals.var_t2_dn0 = assign86200_e131840_d_n0;
        locals.var_t2_dn2 = assign86200_e131840_d_n2;
        locals.var_t2_dn4 = assign86200_e131840_d_n4;
        locals.var_t2_dn5 = assign86200_e131840_d_n5;
        locals.var_t2_dn6 = assign86200_e131840_d_n6;
        locals.var_t2_dn7 = assign86200_e131840_d_n7;
        locals.var_t2_dn8 = assign86200_e131840_d_n8;
        locals.var_t2_dn9 = assign86200_e131840_d_n9;
        locals.var_t2_dn10 = assign86200_e131840_d_n10;
        locals.var_t2_dn11 = assign86200_e131840_d_n11;
        locals.var_t2_dn14 = assign86200_e131840_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign86210_e131850, assign86210_e131850_d_n0, assign86210_e131850_d_n2, assign86210_e131850_d_n4, assign86210_e131850_d_n5, assign86210_e131850_d_n6, assign86210_e131850_d_n7, assign86210_e131850_d_n8, assign86210_e131850_d_n9, assign86210_e131850_d_n10, assign86210_e131850_d_n11, assign86210_e131850_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86210_e131848: f64 = (locals.var_t1 / locals.var_t2);
        (assign86210_e131848, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign86210_e131850;
        locals.var_tmf1_dn0 = assign86210_e131850_d_n0;
        locals.var_tmf1_dn2 = assign86210_e131850_d_n2;
        locals.var_tmf1_dn4 = assign86210_e131850_d_n4;
        locals.var_tmf1_dn5 = assign86210_e131850_d_n5;
        locals.var_tmf1_dn6 = assign86210_e131850_d_n6;
        locals.var_tmf1_dn7 = assign86210_e131850_d_n7;
        locals.var_tmf1_dn8 = assign86210_e131850_d_n8;
        locals.var_tmf1_dn9 = assign86210_e131850_d_n9;
        locals.var_tmf1_dn10 = assign86210_e131850_d_n10;
        locals.var_tmf1_dn11 = assign86210_e131850_d_n11;
        locals.var_tmf1_dn14 = assign86210_e131850_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign86220_e131860, assign86220_e131860_d_n0, assign86220_e131860_d_n2, assign86220_e131860_d_n4, assign86220_e131860_d_n5, assign86220_e131860_d_n6, assign86220_e131860_d_n7, assign86220_e131860_d_n8, assign86220_e131860_d_n9, assign86220_e131860_d_n10, assign86220_e131860_d_n11, assign86220_e131860_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86220_e131858: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign86220_e131858, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign86220_e131860;
        locals.var_tmf2_dn0 = assign86220_e131860_d_n0;
        locals.var_tmf2_dn2 = assign86220_e131860_d_n2;
        locals.var_tmf2_dn4 = assign86220_e131860_d_n4;
        locals.var_tmf2_dn5 = assign86220_e131860_d_n5;
        locals.var_tmf2_dn6 = assign86220_e131860_d_n6;
        locals.var_tmf2_dn7 = assign86220_e131860_d_n7;
        locals.var_tmf2_dn8 = assign86220_e131860_d_n8;
        locals.var_tmf2_dn9 = assign86220_e131860_d_n9;
        locals.var_tmf2_dn10 = assign86220_e131860_d_n10;
        locals.var_tmf2_dn11 = assign86220_e131860_d_n11;
        locals.var_tmf2_dn14 = assign86220_e131860_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign86230_e131870, assign86230_e131870_d_n0, assign86230_e131870_d_n2, assign86230_e131870_d_n4, assign86230_e131870_d_n5, assign86230_e131870_d_n6, assign86230_e131870_d_n7, assign86230_e131870_d_n8, assign86230_e131870_d_n9, assign86230_e131870_d_n10, assign86230_e131870_d_n11, assign86230_e131870_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86230_e131868: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign86230_e131868, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign86230_e131870;
        locals.var_tmf3_dn0 = assign86230_e131870_d_n0;
        locals.var_tmf3_dn2 = assign86230_e131870_d_n2;
        locals.var_tmf3_dn4 = assign86230_e131870_d_n4;
        locals.var_tmf3_dn5 = assign86230_e131870_d_n5;
        locals.var_tmf3_dn6 = assign86230_e131870_d_n6;
        locals.var_tmf3_dn7 = assign86230_e131870_d_n7;
        locals.var_tmf3_dn8 = assign86230_e131870_d_n8;
        locals.var_tmf3_dn9 = assign86230_e131870_d_n9;
        locals.var_tmf3_dn10 = assign86230_e131870_d_n10;
        locals.var_tmf3_dn11 = assign86230_e131870_d_n11;
        locals.var_tmf3_dn14 = assign86230_e131870_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign86240_e131880, assign86240_e131880_d_n0, assign86240_e131880_d_n2, assign86240_e131880_d_n4, assign86240_e131880_d_n5, assign86240_e131880_d_n6, assign86240_e131880_d_n7, assign86240_e131880_d_n8, assign86240_e131880_d_n9, assign86240_e131880_d_n10, assign86240_e131880_d_n11, assign86240_e131880_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86240_e131878: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign86240_e131878, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign86240_e131880;
        locals.var_tmf4_dn0 = assign86240_e131880_d_n0;
        locals.var_tmf4_dn2 = assign86240_e131880_d_n2;
        locals.var_tmf4_dn4 = assign86240_e131880_d_n4;
        locals.var_tmf4_dn5 = assign86240_e131880_d_n5;
        locals.var_tmf4_dn6 = assign86240_e131880_d_n6;
        locals.var_tmf4_dn7 = assign86240_e131880_d_n7;
        locals.var_tmf4_dn8 = assign86240_e131880_d_n8;
        locals.var_tmf4_dn9 = assign86240_e131880_d_n9;
        locals.var_tmf4_dn10 = assign86240_e131880_d_n10;
        locals.var_tmf4_dn11 = assign86240_e131880_d_n11;
        locals.var_tmf4_dn14 = assign86240_e131880_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign86250_e131898, assign86250_e131898_d_n0, assign86250_e131898_d_n2, assign86250_e131898_d_n4, assign86250_e131898_d_n5, assign86250_e131898_d_n6, assign86250_e131898_d_n7, assign86250_e131898_d_n8, assign86250_e131898_d_n9, assign86250_e131898_d_n10, assign86250_e131898_d_n11, assign86250_e131898_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86250_e131889: f64 = (1.0 + locals.var_tmf1);
        let assign86250_e131891: f64 = (assign86250_e131889 + locals.var_tmf2);
        let assign86250_e131893: f64 = (assign86250_e131891 + locals.var_tmf3);
        let assign86250_e131895: f64 = (assign86250_e131893 + locals.var_tmf4);
        let assign86250_e131896: f64 = (1.0 / assign86250_e131895);
        (assign86250_e131896, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign86250_e131895 * assign86250_e131895))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign86250_e131895 * assign86250_e131895))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign86250_e131898;
        locals.var_tmf0_dn0 = assign86250_e131898_d_n0;
        locals.var_tmf0_dn2 = assign86250_e131898_d_n2;
        locals.var_tmf0_dn4 = assign86250_e131898_d_n4;
        locals.var_tmf0_dn5 = assign86250_e131898_d_n5;
        locals.var_tmf0_dn6 = assign86250_e131898_d_n6;
        locals.var_tmf0_dn7 = assign86250_e131898_d_n7;
        locals.var_tmf0_dn8 = assign86250_e131898_d_n8;
        locals.var_tmf0_dn9 = assign86250_e131898_d_n9;
        locals.var_tmf0_dn10 = assign86250_e131898_d_n10;
        locals.var_tmf0_dn11 = assign86250_e131898_d_n11;
        locals.var_tmf0_dn14 = assign86250_e131898_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign86260_e131923, assign86260_e131923_d_n0, assign86260_e131923_d_n2, assign86260_e131923_d_n4, assign86260_e131923_d_n5, assign86260_e131923_d_n6, assign86260_e131923_d_n7, assign86260_e131923_d_n8, assign86260_e131923_d_n9, assign86260_e131923_d_n10, assign86260_e131923_d_n11, assign86260_e131923_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86260_e131907: f64 = (2.0 * locals.var_tmf1);
        let assign86260_e131908: f64 = (1.0 + assign86260_e131907);
        let assign86260_e131911: f64 = (3.0 * locals.var_tmf2);
        let assign86260_e131912: f64 = (assign86260_e131908 + assign86260_e131911);
        let assign86260_e131915: f64 = (4.0 * locals.var_tmf3);
        let assign86260_e131916: f64 = (assign86260_e131912 + assign86260_e131915);
        let assign86260_e131917: f64 = (-assign86260_e131916);
        let assign86260_e131919: f64 = (assign86260_e131917 * locals.var_tmf0);
        let assign86260_e131921: f64 = (assign86260_e131919 * locals.var_tmf0);
        (assign86260_e131921, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign86260_e131917 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign86260_e131919 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign86260_e131923;
        locals.var_t11_dn0 = assign86260_e131923_d_n0;
        locals.var_t11_dn2 = assign86260_e131923_d_n2;
        locals.var_t11_dn4 = assign86260_e131923_d_n4;
        locals.var_t11_dn5 = assign86260_e131923_d_n5;
        locals.var_t11_dn6 = assign86260_e131923_d_n6;
        locals.var_t11_dn7 = assign86260_e131923_d_n7;
        locals.var_t11_dn8 = assign86260_e131923_d_n8;
        locals.var_t11_dn9 = assign86260_e131923_d_n9;
        locals.var_t11_dn10 = assign86260_e131923_d_n10;
        locals.var_t11_dn11 = assign86260_e131923_d_n11;
        locals.var_t11_dn14 = assign86260_e131923_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign86270_e131935, assign86270_e131935_d_n0, assign86270_e131935_d_n2, assign86270_e131935_d_n4, assign86270_e131935_d_n5, assign86270_e131935_d_n6, assign86270_e131935_d_n7, assign86270_e131935_d_n8, assign86270_e131935_d_n9, assign86270_e131935_d_n10, assign86270_e131935_d_n11, assign86270_e131935_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86270_e131932: f64 = (1.0 - locals.var_tmf0);
        let assign86270_e131933: f64 = (locals.var_t2 * assign86270_e131932);
        (assign86270_e131933, ((locals.var_t2_dn0 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign86270_e131932) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign86270_e131935;
        locals.var_ty_dn0 = assign86270_e131935_d_n0;
        locals.var_ty_dn2 = assign86270_e131935_d_n2;
        locals.var_ty_dn4 = assign86270_e131935_d_n4;
        locals.var_ty_dn5 = assign86270_e131935_d_n5;
        locals.var_ty_dn6 = assign86270_e131935_d_n6;
        locals.var_ty_dn7 = assign86270_e131935_d_n7;
        locals.var_ty_dn8 = assign86270_e131935_d_n8;
        locals.var_ty_dn9 = assign86270_e131935_d_n9;
        locals.var_ty_dn10 = assign86270_e131935_d_n10;
        locals.var_ty_dn11 = assign86270_e131935_d_n11;
        locals.var_ty_dn14 = assign86270_e131935_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign86280_e131949, assign86280_e131949_d_n0, assign86280_e131949_d_n2, assign86280_e131949_d_n4, assign86280_e131949_d_n5, assign86280_e131949_d_n6, assign86280_e131949_d_n7, assign86280_e131949_d_n8, assign86280_e131949_d_n9, assign86280_e131949_d_n10, assign86280_e131949_d_n11, assign86280_e131949_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86280_e131943: f64 = (1.0 - locals.var_tmf0);
        let assign86280_e131946: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign86280_e131947: f64 = (assign86280_e131943 + assign86280_e131946);
        (assign86280_e131947, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86280_e131949;
        locals.var_t0_dn0 = assign86280_e131949_d_n0;
        locals.var_t0_dn2 = assign86280_e131949_d_n2;
        locals.var_t0_dn4 = assign86280_e131949_d_n4;
        locals.var_t0_dn5 = assign86280_e131949_d_n5;
        locals.var_t0_dn6 = assign86280_e131949_d_n6;
        locals.var_t0_dn7 = assign86280_e131949_d_n7;
        locals.var_t0_dn8 = assign86280_e131949_d_n8;
        locals.var_t0_dn9 = assign86280_e131949_d_n9;
        locals.var_t0_dn10 = assign86280_e131949_d_n10;
        locals.var_t0_dn11 = assign86280_e131949_d_n11;
        locals.var_t0_dn14 = assign86280_e131949_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86290_e131958, assign86290_e131958_d_n0, assign86290_e131958_d_n2, assign86290_e131958_d_n4, assign86290_e131958_d_n5, assign86290_e131958_d_n6, assign86290_e131958_d_n7, assign86290_e131958_d_n8, assign86290_e131958_d_n9, assign86290_e131958_d_n10, assign86290_e131958_d_n11, assign86290_e131958_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86290_e131956: f64 = (-locals.var_t11);
        (assign86290_e131956, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign86290_e131958;
        locals.var_t11_dn0 = assign86290_e131958_d_n0;
        locals.var_t11_dn2 = assign86290_e131958_d_n2;
        locals.var_t11_dn4 = assign86290_e131958_d_n4;
        locals.var_t11_dn5 = assign86290_e131958_d_n5;
        locals.var_t11_dn6 = assign86290_e131958_d_n6;
        locals.var_t11_dn7 = assign86290_e131958_d_n7;
        locals.var_t11_dn8 = assign86290_e131958_d_n8;
        locals.var_t11_dn9 = assign86290_e131958_d_n9;
        locals.var_t11_dn10 = assign86290_e131958_d_n10;
        locals.var_t11_dn11 = assign86290_e131958_d_n11;
        locals.var_t11_dn14 = assign86290_e131958_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign86300_e131968, assign86300_e131968_d_n0, assign86300_e131968_d_n2, assign86300_e131968_d_n4, assign86300_e131968_d_n5, assign86300_e131968_d_n6, assign86300_e131968_d_n7, assign86300_e131968_d_n8, assign86300_e131968_d_n9, assign86300_e131968_d_n10, assign86300_e131968_d_n11, assign86300_e131968_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86300_e131966: f64 = (locals.var_vbs_bnd_over__blk2011 + locals.var_ty);
        (assign86300_e131966, (locals.var_vbs_bnd_over__blk2011_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk2011_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk2011_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk2011_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk2011_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk2011_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk2011_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk2011_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk2011_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk2011_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over__blk2011_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign86300_e131968;
        locals.var_t10_dn0 = assign86300_e131968_d_n0;
        locals.var_t10_dn2 = assign86300_e131968_d_n2;
        locals.var_t10_dn4 = assign86300_e131968_d_n4;
        locals.var_t10_dn5 = assign86300_e131968_d_n5;
        locals.var_t10_dn6 = assign86300_e131968_d_n6;
        locals.var_t10_dn7 = assign86300_e131968_d_n7;
        locals.var_t10_dn8 = assign86300_e131968_d_n8;
        locals.var_t10_dn9 = assign86300_e131968_d_n9;
        locals.var_t10_dn10 = assign86300_e131968_d_n10;
        locals.var_t10_dn11 = assign86300_e131968_d_n11;
        locals.var_t10_dn14 = assign86300_e131968_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign86310_e131977, assign86310_e131977_d_n0, assign86310_e131977_d_n2, assign86310_e131977_d_n4, assign86310_e131977_d_n5, assign86310_e131977_d_n6, assign86310_e131977_d_n7, assign86310_e131977_d_n8, assign86310_e131977_d_n9, assign86310_e131977_d_n10, assign86310_e131977_d_n11, assign86310_e131977_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) && (locals.var_guard2022 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign86310_e131977;
        locals.var_t10_dn0 = assign86310_e131977_d_n0;
        locals.var_t10_dn2 = assign86310_e131977_d_n2;
        locals.var_t10_dn4 = assign86310_e131977_d_n4;
        locals.var_t10_dn5 = assign86310_e131977_d_n5;
        locals.var_t10_dn6 = assign86310_e131977_d_n6;
        locals.var_t10_dn7 = assign86310_e131977_d_n7;
        locals.var_t10_dn8 = assign86310_e131977_d_n8;
        locals.var_t10_dn9 = assign86310_e131977_d_n9;
        locals.var_t10_dn10 = assign86310_e131977_d_n10;
        locals.var_t10_dn11 = assign86310_e131977_d_n11;
        locals.var_t10_dn14 = assign86310_e131977_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign86320_e131984, assign86320_e131984_d_n0, assign86320_e131984_d_n2, assign86320_e131984_d_n4, assign86320_e131984_d_n5, assign86320_e131984_d_n6, assign86320_e131984_d_n7, assign86320_e131984_d_n8, assign86320_e131984_d_n9, assign86320_e131984_d_n10, assign86320_e131984_d_n11, assign86320_e131984_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) {
        let assign86320_e131982: f64 = (-locals.var_t10);
        (assign86320_e131982, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign86320_e131984;
        locals.var_vxbgmtcl_dn0 = assign86320_e131984_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86320_e131984_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86320_e131984_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86320_e131984_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86320_e131984_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86320_e131984_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86320_e131984_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86320_e131984_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86320_e131984_d_n10;
        locals.var_vxbgmtcl_dn11 = assign86320_e131984_d_n11;
        locals.var_vxbgmtcl_dn14 = assign86320_e131984_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign86330_e131991, assign86330_e131991_d_n0, assign86330_e131991_d_n2, assign86330_e131991_d_n4, assign86330_e131991_d_n5, assign86330_e131991_d_n6, assign86330_e131991_d_n7, assign86330_e131991_d_n8, assign86330_e131991_d_n9, assign86330_e131991_d_n10, assign86330_e131991_d_n11, assign86330_e131991_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign86330_e131991;
        locals.var_vxbgmtcl_dn0 = assign86330_e131991_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86330_e131991_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86330_e131991_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86330_e131991_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86330_e131991_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86330_e131991_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86330_e131991_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86330_e131991_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86330_e131991_d_n10;
        locals.var_vxbgmtcl_dn11 = assign86330_e131991_d_n11;
        locals.var_vxbgmtcl_dn14 = assign86330_e131991_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign86340_e131997, assign86340_e131997_d_n0, assign86340_e131997_d_n2, assign86340_e131997_d_n4, assign86340_e131997_d_n5, assign86340_e131997_d_n6, assign86340_e131997_d_n7, assign86340_e131997_d_n8, assign86340_e131997_d_n9, assign86340_e131997_d_n10, assign86340_e131997_d_n11, assign86340_e131997_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86340_e131995: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign86340_e131995, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign86340_e131997;
        locals.var_fac1_dn0 = assign86340_e131997_d_n0;
        locals.var_fac1_dn2 = assign86340_e131997_d_n2;
        locals.var_fac1_dn4 = assign86340_e131997_d_n4;
        locals.var_fac1_dn5 = assign86340_e131997_d_n5;
        locals.var_fac1_dn6 = assign86340_e131997_d_n6;
        locals.var_fac1_dn7 = assign86340_e131997_d_n7;
        locals.var_fac1_dn8 = assign86340_e131997_d_n8;
        locals.var_fac1_dn9 = assign86340_e131997_d_n9;
        locals.var_fac1_dn10 = assign86340_e131997_d_n10;
        locals.var_fac1_dn11 = assign86340_e131997_d_n11;
        locals.var_fac1_dn14 = assign86340_e131997_d_n14;
        locals.var_fac1_rv = 0.0;

        let (assign86350_e132003, assign86350_e132003_d_n0, assign86350_e132003_d_n2, assign86350_e132003_d_n4, assign86350_e132003_d_n5, assign86350_e132003_d_n6, assign86350_e132003_d_n7, assign86350_e132003_d_n8, assign86350_e132003_d_n9, assign86350_e132003_d_n10, assign86350_e132003_d_n11, assign86350_e132003_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86350_e132001: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign86350_e132001, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign86350_e132003;
        locals.var_fac1p2_dn0 = assign86350_e132003_d_n0;
        locals.var_fac1p2_dn2 = assign86350_e132003_d_n2;
        locals.var_fac1p2_dn4 = assign86350_e132003_d_n4;
        locals.var_fac1p2_dn5 = assign86350_e132003_d_n5;
        locals.var_fac1p2_dn6 = assign86350_e132003_d_n6;
        locals.var_fac1p2_dn7 = assign86350_e132003_d_n7;
        locals.var_fac1p2_dn8 = assign86350_e132003_d_n8;
        locals.var_fac1p2_dn9 = assign86350_e132003_d_n9;
        locals.var_fac1p2_dn10 = assign86350_e132003_d_n10;
        locals.var_fac1p2_dn11 = assign86350_e132003_d_n11;
        locals.var_fac1p2_dn14 = assign86350_e132003_d_n14;
        locals.var_fac1p2_rv = 0.0;

        let (assign86360_e132010, assign86360_e132010_d_n2, assign86360_e132010_d_n7, assign86360_e132010_d_n8, assign86360_e132010_d_n9,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86360_e132006: f64 = (-locals.var_vgbgmt);
        let assign86360_e132008: f64 = (assign86360_e132006 + locals.var_uc_vfbover);
        (assign86360_e132008, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign86360_e132010;
        locals.var_vgpld_dn2 = assign86360_e132010_d_n2;
        locals.var_vgpld_dn7 = assign86360_e132010_d_n7;
        locals.var_vgpld_dn8 = assign86360_e132010_d_n8;
        locals.var_vgpld_dn9 = assign86360_e132010_d_n9;
        locals.var_vgpld_rv = 0.0;

        let (assign86370_e132019, assign86370_e132019_d_n0, assign86370_e132019_d_n2, assign86370_e132019_d_n4, assign86370_e132019_d_n5, assign86370_e132019_d_n6, assign86370_e132019_d_n7, assign86370_e132019_d_n8, assign86370_e132019_d_n9, assign86370_e132019_d_n10, assign86370_e132019_d_n11, assign86370_e132019_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86370_e132013: f64 = (-locals.var_vxbgmtcl);
        let assign86370_e132016: f64 = (10.0 * 2.220446049250313e-16);
        let assign86370_e132017: f64 = (assign86370_e132013 + assign86370_e132016);
        (assign86370_e132017, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign86370_e132019;
        locals.var_vgb_fb_ld_dn0 = assign86370_e132019_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign86370_e132019_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign86370_e132019_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign86370_e132019_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign86370_e132019_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign86370_e132019_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign86370_e132019_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign86370_e132019_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign86370_e132019_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign86370_e132019_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign86370_e132019_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign86380_e132023, assign86380_e132023_d_n0, assign86380_e132023_d_n2, assign86380_e132023_d_n4, assign86380_e132023_d_n5, assign86380_e132023_d_n6, assign86380_e132023_d_n7, assign86380_e132023_d_n8, assign86380_e132023_d_n9, assign86380_e132023_d_n10, assign86380_e132023_d_n11, assign86380_e132023_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk2005, locals.var_q_dep_ld__blk2005_dn0, locals.var_q_dep_ld__blk2005_dn2, locals.var_q_dep_ld__blk2005_dn4, locals.var_q_dep_ld__blk2005_dn5, locals.var_q_dep_ld__blk2005_dn6, locals.var_q_dep_ld__blk2005_dn7, locals.var_q_dep_ld__blk2005_dn8, locals.var_q_dep_ld__blk2005_dn9, locals.var_q_dep_ld__blk2005_dn10, locals.var_q_dep_ld__blk2005_dn11, locals.var_q_dep_ld__blk2005_dn14,)
    }
};
        locals.var_q_dep_ld__blk2005 = assign86380_e132023;
        locals.var_q_dep_ld__blk2005_dn0 = assign86380_e132023_d_n0;
        locals.var_q_dep_ld__blk2005_dn2 = assign86380_e132023_d_n2;
        locals.var_q_dep_ld__blk2005_dn4 = assign86380_e132023_d_n4;
        locals.var_q_dep_ld__blk2005_dn5 = assign86380_e132023_d_n5;
        locals.var_q_dep_ld__blk2005_dn6 = assign86380_e132023_d_n6;
        locals.var_q_dep_ld__blk2005_dn7 = assign86380_e132023_d_n7;
        locals.var_q_dep_ld__blk2005_dn8 = assign86380_e132023_d_n8;
        locals.var_q_dep_ld__blk2005_dn9 = assign86380_e132023_d_n9;
        locals.var_q_dep_ld__blk2005_dn10 = assign86380_e132023_d_n10;
        locals.var_q_dep_ld__blk2005_dn11 = assign86380_e132023_d_n11;
        locals.var_q_dep_ld__blk2005_dn14 = assign86380_e132023_d_n14;
        locals.var_q_dep_ld__blk2005_rv = 0.0;

        let (assign86390_e132029,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86390_e132027: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign86390_e132027,)
    } else {
        (locals.var_q_nsubld__blk2006,)
    }
};
        locals.var_q_nsubld__blk2006 = assign86390_e132029;
        locals.var_q_nsubld__blk2006_rv = 0.0;

        let (assign86400_e132035, assign86400_e132035_d_n0, assign86400_e132035_d_n2, assign86400_e132035_d_n4, assign86400_e132035_d_n5, assign86400_e132035_d_n6, assign86400_e132035_d_n7, assign86400_e132035_d_n8, assign86400_e132035_d_n9, assign86400_e132035_d_n10, assign86400_e132035_d_n11, assign86400_e132035_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86400_e132033: f64 = (locals.var_nin / locals.var_nover_func);
        (assign86400_e132033, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86400_e132035;
        locals.var_t0_dn0 = assign86400_e132035_d_n0;
        locals.var_t0_dn2 = assign86400_e132035_d_n2;
        locals.var_t0_dn4 = assign86400_e132035_d_n4;
        locals.var_t0_dn5 = assign86400_e132035_d_n5;
        locals.var_t0_dn6 = assign86400_e132035_d_n6;
        locals.var_t0_dn7 = assign86400_e132035_d_n7;
        locals.var_t0_dn8 = assign86400_e132035_d_n8;
        locals.var_t0_dn9 = assign86400_e132035_d_n9;
        locals.var_t0_dn10 = assign86400_e132035_d_n10;
        locals.var_t0_dn11 = assign86400_e132035_d_n11;
        locals.var_t0_dn14 = assign86400_e132035_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86410_e132041, assign86410_e132041_d_n0, assign86410_e132041_d_n2, assign86410_e132041_d_n4, assign86410_e132041_d_n5, assign86410_e132041_d_n6, assign86410_e132041_d_n7, assign86410_e132041_d_n8, assign86410_e132041_d_n9, assign86410_e132041_d_n10, assign86410_e132041_d_n11, assign86410_e132041_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86410_e132039: f64 = (locals.var_t0 * locals.var_t0);
        (assign86410_e132039, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign86410_e132041;
        locals.var_cnst1over_dn0 = assign86410_e132041_d_n0;
        locals.var_cnst1over_dn2 = assign86410_e132041_d_n2;
        locals.var_cnst1over_dn4 = assign86410_e132041_d_n4;
        locals.var_cnst1over_dn5 = assign86410_e132041_d_n5;
        locals.var_cnst1over_dn6 = assign86410_e132041_d_n6;
        locals.var_cnst1over_dn7 = assign86410_e132041_d_n7;
        locals.var_cnst1over_dn8 = assign86410_e132041_d_n8;
        locals.var_cnst1over_dn9 = assign86410_e132041_d_n9;
        locals.var_cnst1over_dn10 = assign86410_e132041_d_n10;
        locals.var_cnst1over_dn11 = assign86410_e132041_d_n11;
        locals.var_cnst1over_dn14 = assign86410_e132041_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let assign86420_e132044: f64 = (-locals.var_vxbgmtcl);
        let assign86420_e132045: f64 = (locals.var_beta * assign86420_e132044);
        let assign86420_e132047: f64 = if assign86420_e132045 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard2023 = assign86420_e132047;
        locals.var_guard2023_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_331(
        locals: &mut StampLocals,
    ) {
        let (assign86430_e132062, assign86430_e132062_d_n0, assign86430_e132062_d_n2, assign86430_e132062_d_n4, assign86430_e132062_d_n5, assign86430_e132062_d_n6, assign86430_e132062_d_n7, assign86430_e132062_d_n8, assign86430_e132062_d_n9, assign86430_e132062_d_n10, assign86430_e132062_d_n11, assign86430_e132062_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 != 0.0)) {
        let assign86430_e132055: f64 = (-locals.var_vxbgmtcl);
        let assign86430_e132056: f64 = (locals.var_beta * assign86430_e132055);
        let assign86430_e132057: f64 = (1.0 + assign86430_e132056);
        let assign86430_e132059: f64 = (assign86430_e132057 - 500.0);
        let assign86430_e132060: f64 = (1.403592217853e217 * assign86430_e132059);
        (assign86430_e132060, (1.403592217853e217 * ((locals.var_beta_dn0 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign86430_e132055) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign86430_e132062;
        locals.var_exp_bvbs_dn0 = assign86430_e132062_d_n0;
        locals.var_exp_bvbs_dn2 = assign86430_e132062_d_n2;
        locals.var_exp_bvbs_dn4 = assign86430_e132062_d_n4;
        locals.var_exp_bvbs_dn5 = assign86430_e132062_d_n5;
        locals.var_exp_bvbs_dn6 = assign86430_e132062_d_n6;
        locals.var_exp_bvbs_dn7 = assign86430_e132062_d_n7;
        locals.var_exp_bvbs_dn8 = assign86430_e132062_d_n8;
        locals.var_exp_bvbs_dn9 = assign86430_e132062_d_n9;
        locals.var_exp_bvbs_dn10 = assign86430_e132062_d_n10;
        locals.var_exp_bvbs_dn11 = assign86430_e132062_d_n11;
        locals.var_exp_bvbs_dn14 = assign86430_e132062_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign86440_e132068, assign86440_e132068_d_n0, assign86440_e132068_d_n2, assign86440_e132068_d_n4, assign86440_e132068_d_n5, assign86440_e132068_d_n6, assign86440_e132068_d_n7, assign86440_e132068_d_n8, assign86440_e132068_d_n9, assign86440_e132068_d_n10, assign86440_e132068_d_n11, assign86440_e132068_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86440_e132068;
        locals.var_t0_dn0 = assign86440_e132068_d_n0;
        locals.var_t0_dn2 = assign86440_e132068_d_n2;
        locals.var_t0_dn4 = assign86440_e132068_d_n4;
        locals.var_t0_dn5 = assign86440_e132068_d_n5;
        locals.var_t0_dn6 = assign86440_e132068_d_n6;
        locals.var_t0_dn7 = assign86440_e132068_d_n7;
        locals.var_t0_dn8 = assign86440_e132068_d_n8;
        locals.var_t0_dn9 = assign86440_e132068_d_n9;
        locals.var_t0_dn10 = assign86440_e132068_d_n10;
        locals.var_t0_dn11 = assign86440_e132068_d_n11;
        locals.var_t0_dn14 = assign86440_e132068_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86450_e132078, assign86450_e132078_d_n0, assign86450_e132078_d_n2, assign86450_e132078_d_n4, assign86450_e132078_d_n5, assign86450_e132078_d_n6, assign86450_e132078_d_n7, assign86450_e132078_d_n8, assign86450_e132078_d_n9, assign86450_e132078_d_n10, assign86450_e132078_d_n11, assign86450_e132078_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 == 0.0)) {
        let assign86450_e132075: f64 = (-locals.var_vxbgmtcl);
        let assign86450_e132076: f64 = (locals.var_beta * assign86450_e132075);
        (assign86450_e132076, ((locals.var_beta_dn0 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign86450_e132075) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign86450_e132078;
        locals.var_tmf1_dn0 = assign86450_e132078_d_n0;
        locals.var_tmf1_dn2 = assign86450_e132078_d_n2;
        locals.var_tmf1_dn4 = assign86450_e132078_d_n4;
        locals.var_tmf1_dn5 = assign86450_e132078_d_n5;
        locals.var_tmf1_dn6 = assign86450_e132078_d_n6;
        locals.var_tmf1_dn7 = assign86450_e132078_d_n7;
        locals.var_tmf1_dn8 = assign86450_e132078_d_n8;
        locals.var_tmf1_dn9 = assign86450_e132078_d_n9;
        locals.var_tmf1_dn10 = assign86450_e132078_d_n10;
        locals.var_tmf1_dn11 = assign86450_e132078_d_n11;
        locals.var_tmf1_dn14 = assign86450_e132078_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign86460_e132085, assign86460_e132085_d_n0, assign86460_e132085_d_n2, assign86460_e132085_d_n4, assign86460_e132085_d_n5, assign86460_e132085_d_n6, assign86460_e132085_d_n7, assign86460_e132085_d_n8, assign86460_e132085_d_n9, assign86460_e132085_d_n10, assign86460_e132085_d_n11, assign86460_e132085_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign86460_e132085;
        locals.var_exp_bvbs_dn0 = assign86460_e132085_d_n0;
        locals.var_exp_bvbs_dn2 = assign86460_e132085_d_n2;
        locals.var_exp_bvbs_dn4 = assign86460_e132085_d_n4;
        locals.var_exp_bvbs_dn5 = assign86460_e132085_d_n5;
        locals.var_exp_bvbs_dn6 = assign86460_e132085_d_n6;
        locals.var_exp_bvbs_dn7 = assign86460_e132085_d_n7;
        locals.var_exp_bvbs_dn8 = assign86460_e132085_d_n8;
        locals.var_exp_bvbs_dn9 = assign86460_e132085_d_n9;
        locals.var_exp_bvbs_dn10 = assign86460_e132085_d_n10;
        locals.var_exp_bvbs_dn11 = assign86460_e132085_d_n11;
        locals.var_exp_bvbs_dn14 = assign86460_e132085_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign86470_loop_guard: usize = 0;
        while {
            let assign86470_cond_e132093: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign86470_cond_e132093 != 0.0
        } {
            assign86470_loop_guard += 1;
            assert!(assign86470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign86470_body0_e132102, assign86470_body0_e132102_d_n0, assign86470_body0_e132102_d_n2, assign86470_body0_e132102_d_n4, assign86470_body0_e132102_d_n5, assign86470_body0_e132102_d_n6, assign86470_body0_e132102_d_n7, assign86470_body0_e132102_d_n8, assign86470_body0_e132102_d_n9, assign86470_body0_e132102_d_n10, assign86470_body0_e132102_d_n11, assign86470_body0_e132102_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 == 0.0)) {
        let assign86470_body0_e132100: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign86470_body0_e132100, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign86470_body0_e132102;
            locals.var_exp_bvbs_dn0 = assign86470_body0_e132102_d_n0;
            locals.var_exp_bvbs_dn2 = assign86470_body0_e132102_d_n2;
            locals.var_exp_bvbs_dn4 = assign86470_body0_e132102_d_n4;
            locals.var_exp_bvbs_dn5 = assign86470_body0_e132102_d_n5;
            locals.var_exp_bvbs_dn6 = assign86470_body0_e132102_d_n6;
            locals.var_exp_bvbs_dn7 = assign86470_body0_e132102_d_n7;
            locals.var_exp_bvbs_dn8 = assign86470_body0_e132102_d_n8;
            locals.var_exp_bvbs_dn9 = assign86470_body0_e132102_d_n9;
            locals.var_exp_bvbs_dn10 = assign86470_body0_e132102_d_n10;
            locals.var_exp_bvbs_dn11 = assign86470_body0_e132102_d_n11;
            locals.var_exp_bvbs_dn14 = assign86470_body0_e132102_d_n14;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign86470_body1_e132111, assign86470_body1_e132111_d_n0, assign86470_body1_e132111_d_n2, assign86470_body1_e132111_d_n4, assign86470_body1_e132111_d_n5, assign86470_body1_e132111_d_n6, assign86470_body1_e132111_d_n7, assign86470_body1_e132111_d_n8, assign86470_body1_e132111_d_n9, assign86470_body1_e132111_d_n10, assign86470_body1_e132111_d_n11, assign86470_body1_e132111_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 == 0.0)) {
        let assign86470_body1_e132109: f64 = (locals.var_tmf1 - 60.0);
        (assign86470_body1_e132109, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign86470_body1_e132111;
            locals.var_tmf1_dn0 = assign86470_body1_e132111_d_n0;
            locals.var_tmf1_dn2 = assign86470_body1_e132111_d_n2;
            locals.var_tmf1_dn4 = assign86470_body1_e132111_d_n4;
            locals.var_tmf1_dn5 = assign86470_body1_e132111_d_n5;
            locals.var_tmf1_dn6 = assign86470_body1_e132111_d_n6;
            locals.var_tmf1_dn7 = assign86470_body1_e132111_d_n7;
            locals.var_tmf1_dn8 = assign86470_body1_e132111_d_n8;
            locals.var_tmf1_dn9 = assign86470_body1_e132111_d_n9;
            locals.var_tmf1_dn10 = assign86470_body1_e132111_d_n10;
            locals.var_tmf1_dn11 = assign86470_body1_e132111_d_n11;
            locals.var_tmf1_dn14 = assign86470_body1_e132111_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign86480_e132121, assign86480_e132121_d_n0, assign86480_e132121_d_n2, assign86480_e132121_d_n4, assign86480_e132121_d_n5, assign86480_e132121_d_n6, assign86480_e132121_d_n7, assign86480_e132121_d_n8, assign86480_e132121_d_n9, assign86480_e132121_d_n10, assign86480_e132121_d_n11, assign86480_e132121_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 == 0.0)) {
        let assign86480_e132118: f64 = (locals.var_tmf1).exp();
        let assign86480_e132119: f64 = (locals.var_exp_bvbs * assign86480_e132118);
        (assign86480_e132119, ((locals.var_exp_bvbs_dn0 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign86480_e132118) + (locals.var_exp_bvbs * (assign86480_e132118 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign86480_e132121;
        locals.var_exp_bvbs_dn0 = assign86480_e132121_d_n0;
        locals.var_exp_bvbs_dn2 = assign86480_e132121_d_n2;
        locals.var_exp_bvbs_dn4 = assign86480_e132121_d_n4;
        locals.var_exp_bvbs_dn5 = assign86480_e132121_d_n5;
        locals.var_exp_bvbs_dn6 = assign86480_e132121_d_n6;
        locals.var_exp_bvbs_dn7 = assign86480_e132121_d_n7;
        locals.var_exp_bvbs_dn8 = assign86480_e132121_d_n8;
        locals.var_exp_bvbs_dn9 = assign86480_e132121_d_n9;
        locals.var_exp_bvbs_dn10 = assign86480_e132121_d_n10;
        locals.var_exp_bvbs_dn11 = assign86480_e132121_d_n11;
        locals.var_exp_bvbs_dn14 = assign86480_e132121_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign86490_e132128, assign86490_e132128_d_n0, assign86490_e132128_d_n2, assign86490_e132128_d_n4, assign86490_e132128_d_n5, assign86490_e132128_d_n6, assign86490_e132128_d_n7, assign86490_e132128_d_n8, assign86490_e132128_d_n9, assign86490_e132128_d_n10, assign86490_e132128_d_n11, assign86490_e132128_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2023 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86490_e132128;
        locals.var_t0_dn0 = assign86490_e132128_d_n0;
        locals.var_t0_dn2 = assign86490_e132128_d_n2;
        locals.var_t0_dn4 = assign86490_e132128_d_n4;
        locals.var_t0_dn5 = assign86490_e132128_d_n5;
        locals.var_t0_dn6 = assign86490_e132128_d_n6;
        locals.var_t0_dn7 = assign86490_e132128_d_n7;
        locals.var_t0_dn8 = assign86490_e132128_d_n8;
        locals.var_t0_dn9 = assign86490_e132128_d_n9;
        locals.var_t0_dn10 = assign86490_e132128_d_n10;
        locals.var_t0_dn11 = assign86490_e132128_d_n11;
        locals.var_t0_dn14 = assign86490_e132128_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86500_e132141, assign86500_e132141_d_n0, assign86500_e132141_d_n2, assign86500_e132141_d_n4, assign86500_e132141_d_n5, assign86500_e132141_d_n6, assign86500_e132141_d_n7, assign86500_e132141_d_n8, assign86500_e132141_d_n9, assign86500_e132141_d_n10, assign86500_e132141_d_n11, assign86500_e132141_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86500_e132133: f64 = (-locals.var_vgpld);
        let assign86500_e132135: f64 = (assign86500_e132133 * 0.5);
        let assign86500_e132137: f64 = (assign86500_e132135 - 0.5);
        let assign86500_e132139: f64 = (assign86500_e132137 - 1.0);
        (assign86500_e132139, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign86500_e132141;
        locals.var_tmf1_dn0 = assign86500_e132141_d_n0;
        locals.var_tmf1_dn2 = assign86500_e132141_d_n2;
        locals.var_tmf1_dn4 = assign86500_e132141_d_n4;
        locals.var_tmf1_dn5 = assign86500_e132141_d_n5;
        locals.var_tmf1_dn6 = assign86500_e132141_d_n6;
        locals.var_tmf1_dn7 = assign86500_e132141_d_n7;
        locals.var_tmf1_dn8 = assign86500_e132141_d_n8;
        locals.var_tmf1_dn9 = assign86500_e132141_d_n9;
        locals.var_tmf1_dn10 = assign86500_e132141_d_n10;
        locals.var_tmf1_dn11 = assign86500_e132141_d_n11;
        locals.var_tmf1_dn14 = assign86500_e132141_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign86510_e132151, assign86510_e132151_d_n0, assign86510_e132151_d_n2, assign86510_e132151_d_n4, assign86510_e132151_d_n5, assign86510_e132151_d_n6, assign86510_e132151_d_n7, assign86510_e132151_d_n8, assign86510_e132151_d_n9, assign86510_e132151_d_n10, assign86510_e132151_d_n11, assign86510_e132151_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86510_e132147: f64 = (4.0 * 0.5);
        let assign86510_e132149: f64 = assign86510_e132147;
        (assign86510_e132149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign86510_e132151;
        locals.var_tmf2_dn0 = assign86510_e132151_d_n0;
        locals.var_tmf2_dn2 = assign86510_e132151_d_n2;
        locals.var_tmf2_dn4 = assign86510_e132151_d_n4;
        locals.var_tmf2_dn5 = assign86510_e132151_d_n5;
        locals.var_tmf2_dn6 = assign86510_e132151_d_n6;
        locals.var_tmf2_dn7 = assign86510_e132151_d_n7;
        locals.var_tmf2_dn8 = assign86510_e132151_d_n8;
        locals.var_tmf2_dn9 = assign86510_e132151_d_n9;
        locals.var_tmf2_dn10 = assign86510_e132151_d_n10;
        locals.var_tmf2_dn11 = assign86510_e132151_d_n11;
        locals.var_tmf2_dn14 = assign86510_e132151_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign86520_e132163, assign86520_e132163_d_n0, assign86520_e132163_d_n2, assign86520_e132163_d_n4, assign86520_e132163_d_n5, assign86520_e132163_d_n6, assign86520_e132163_d_n7, assign86520_e132163_d_n8, assign86520_e132163_d_n9, assign86520_e132163_d_n10, assign86520_e132163_d_n11, assign86520_e132163_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign86520_e132161, assign86520_e132161_d_n0, assign86520_e132161_d_n2, assign86520_e132161_d_n4, assign86520_e132161_d_n5, assign86520_e132161_d_n6, assign86520_e132161_d_n7, assign86520_e132161_d_n8, assign86520_e132161_d_n9, assign86520_e132161_d_n10, assign86520_e132161_d_n11, assign86520_e132161_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign86520_e132160: f64 = (-locals.var_tmf2);
                (assign86520_e132160, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign86520_e132161, assign86520_e132161_d_n0, assign86520_e132161_d_n2, assign86520_e132161_d_n4, assign86520_e132161_d_n5, assign86520_e132161_d_n6, assign86520_e132161_d_n7, assign86520_e132161_d_n8, assign86520_e132161_d_n9, assign86520_e132161_d_n10, assign86520_e132161_d_n11, assign86520_e132161_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign86520_e132163;
        locals.var_tmf2_dn0 = assign86520_e132163_d_n0;
        locals.var_tmf2_dn2 = assign86520_e132163_d_n2;
        locals.var_tmf2_dn4 = assign86520_e132163_d_n4;
        locals.var_tmf2_dn5 = assign86520_e132163_d_n5;
        locals.var_tmf2_dn6 = assign86520_e132163_d_n6;
        locals.var_tmf2_dn7 = assign86520_e132163_d_n7;
        locals.var_tmf2_dn8 = assign86520_e132163_d_n8;
        locals.var_tmf2_dn9 = assign86520_e132163_d_n9;
        locals.var_tmf2_dn10 = assign86520_e132163_d_n10;
        locals.var_tmf2_dn11 = assign86520_e132163_d_n11;
        locals.var_tmf2_dn14 = assign86520_e132163_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign86530_e132174, assign86530_e132174_d_n0, assign86530_e132174_d_n2, assign86530_e132174_d_n4, assign86530_e132174_d_n5, assign86530_e132174_d_n6, assign86530_e132174_d_n7, assign86530_e132174_d_n8, assign86530_e132174_d_n9, assign86530_e132174_d_n10, assign86530_e132174_d_n11, assign86530_e132174_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86530_e132169: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign86530_e132171: f64 = (assign86530_e132169 + locals.var_tmf2);
        let assign86530_e132172: f64 = (assign86530_e132171).sqrt();
        (assign86530_e132172, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign86530_e132172)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign86530_e132172)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign86530_e132174;
        locals.var_tmf2_dn0 = assign86530_e132174_d_n0;
        locals.var_tmf2_dn2 = assign86530_e132174_d_n2;
        locals.var_tmf2_dn4 = assign86530_e132174_d_n4;
        locals.var_tmf2_dn5 = assign86530_e132174_d_n5;
        locals.var_tmf2_dn6 = assign86530_e132174_d_n6;
        locals.var_tmf2_dn7 = assign86530_e132174_d_n7;
        locals.var_tmf2_dn8 = assign86530_e132174_d_n8;
        locals.var_tmf2_dn9 = assign86530_e132174_d_n9;
        locals.var_tmf2_dn10 = assign86530_e132174_d_n10;
        locals.var_tmf2_dn11 = assign86530_e132174_d_n11;
        locals.var_tmf2_dn14 = assign86530_e132174_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign86540_e132186, assign86540_e132186_d_n0, assign86540_e132186_d_n2, assign86540_e132186_d_n4, assign86540_e132186_d_n5, assign86540_e132186_d_n6, assign86540_e132186_d_n7, assign86540_e132186_d_n8, assign86540_e132186_d_n9, assign86540_e132186_d_n10, assign86540_e132186_d_n11, assign86540_e132186_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86540_e132182: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign86540_e132183: f64 = (1.0 + assign86540_e132182);
        let assign86540_e132184: f64 = (0.5 * assign86540_e132183);
        (assign86540_e132184, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86540_e132186;
        locals.var_t0_dn0 = assign86540_e132186_d_n0;
        locals.var_t0_dn2 = assign86540_e132186_d_n2;
        locals.var_t0_dn4 = assign86540_e132186_d_n4;
        locals.var_t0_dn5 = assign86540_e132186_d_n5;
        locals.var_t0_dn6 = assign86540_e132186_d_n6;
        locals.var_t0_dn7 = assign86540_e132186_d_n7;
        locals.var_t0_dn8 = assign86540_e132186_d_n8;
        locals.var_t0_dn9 = assign86540_e132186_d_n9;
        locals.var_t0_dn10 = assign86540_e132186_d_n10;
        locals.var_t0_dn11 = assign86540_e132186_d_n11;
        locals.var_t0_dn14 = assign86540_e132186_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86550_e132198, assign86550_e132198_d_n0, assign86550_e132198_d_n2, assign86550_e132198_d_n4, assign86550_e132198_d_n5, assign86550_e132198_d_n6, assign86550_e132198_d_n7, assign86550_e132198_d_n8, assign86550_e132198_d_n9, assign86550_e132198_d_n10, assign86550_e132198_d_n11, assign86550_e132198_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86550_e132194: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign86550_e132195: f64 = (0.5 * assign86550_e132194);
        let assign86550_e132196: f64 = (0.5 + assign86550_e132195);
        (assign86550_e132196, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign86550_e132198;
        locals.var_t1_dn0 = assign86550_e132198_d_n0;
        locals.var_t1_dn2 = assign86550_e132198_d_n2;
        locals.var_t1_dn4 = assign86550_e132198_d_n4;
        locals.var_t1_dn5 = assign86550_e132198_d_n5;
        locals.var_t1_dn6 = assign86550_e132198_d_n6;
        locals.var_t1_dn7 = assign86550_e132198_d_n7;
        locals.var_t1_dn8 = assign86550_e132198_d_n8;
        locals.var_t1_dn9 = assign86550_e132198_d_n9;
        locals.var_t1_dn10 = assign86550_e132198_d_n10;
        locals.var_t1_dn11 = assign86550_e132198_d_n11;
        locals.var_t1_dn14 = assign86550_e132198_d_n14;
        locals.var_t1_rv = 0.0;

        let assign86560_e132201: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86560_e132204: f64 = (-locals.var_t1);
        let assign86560_e132209: f64 = if ((assign86560_e132201 > assign86560_e132204) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2024 = assign86560_e132209;
        locals.var_guard2024_rv = 0.0;

        let (assign86570_e132223, assign86570_e132223_d_n0, assign86570_e132223_d_n2, assign86570_e132223_d_n4, assign86570_e132223_d_n5, assign86570_e132223_d_n6, assign86570_e132223_d_n7, assign86570_e132223_d_n8, assign86570_e132223_d_n9, assign86570_e132223_d_n10, assign86570_e132223_d_n11, assign86570_e132223_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86570_e132217: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86570_e132219: f64 = assign86570_e132217;
        let assign86570_e132221: f64 = (assign86570_e132219 + locals.var_t1);
        (assign86570_e132221, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign86570_e132223;
        locals.var_tmf1_dn0 = assign86570_e132223_d_n0;
        locals.var_tmf1_dn2 = assign86570_e132223_d_n2;
        locals.var_tmf1_dn4 = assign86570_e132223_d_n4;
        locals.var_tmf1_dn5 = assign86570_e132223_d_n5;
        locals.var_tmf1_dn6 = assign86570_e132223_d_n6;
        locals.var_tmf1_dn7 = assign86570_e132223_d_n7;
        locals.var_tmf1_dn8 = assign86570_e132223_d_n8;
        locals.var_tmf1_dn9 = assign86570_e132223_d_n9;
        locals.var_tmf1_dn10 = assign86570_e132223_d_n10;
        locals.var_tmf1_dn11 = assign86570_e132223_d_n11;
        locals.var_tmf1_dn14 = assign86570_e132223_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign86580_e132233, assign86580_e132233_d_n0, assign86580_e132233_d_n2, assign86580_e132233_d_n4, assign86580_e132233_d_n5, assign86580_e132233_d_n6, assign86580_e132233_d_n7, assign86580_e132233_d_n8, assign86580_e132233_d_n9, assign86580_e132233_d_n10, assign86580_e132233_d_n11, assign86580_e132233_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86580_e132231: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign86580_e132231, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign86580_e132233;
        locals.var_x2_dn0 = assign86580_e132233_d_n0;
        locals.var_x2_dn2 = assign86580_e132233_d_n2;
        locals.var_x2_dn4 = assign86580_e132233_d_n4;
        locals.var_x2_dn5 = assign86580_e132233_d_n5;
        locals.var_x2_dn6 = assign86580_e132233_d_n6;
        locals.var_x2_dn7 = assign86580_e132233_d_n7;
        locals.var_x2_dn8 = assign86580_e132233_d_n8;
        locals.var_x2_dn9 = assign86580_e132233_d_n9;
        locals.var_x2_dn10 = assign86580_e132233_d_n10;
        locals.var_x2_dn11 = assign86580_e132233_d_n11;
        locals.var_x2_dn14 = assign86580_e132233_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign86590_e132243, assign86590_e132243_d_n0, assign86590_e132243_d_n2, assign86590_e132243_d_n4, assign86590_e132243_d_n5, assign86590_e132243_d_n6, assign86590_e132243_d_n7, assign86590_e132243_d_n8, assign86590_e132243_d_n9, assign86590_e132243_d_n10, assign86590_e132243_d_n11, assign86590_e132243_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86590_e132241: f64 = (locals.var_t1 * locals.var_t1);
        (assign86590_e132241, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign86590_e132243;
        locals.var_xmax2_dn0 = assign86590_e132243_d_n0;
        locals.var_xmax2_dn2 = assign86590_e132243_d_n2;
        locals.var_xmax2_dn4 = assign86590_e132243_d_n4;
        locals.var_xmax2_dn5 = assign86590_e132243_d_n5;
        locals.var_xmax2_dn6 = assign86590_e132243_d_n6;
        locals.var_xmax2_dn7 = assign86590_e132243_d_n7;
        locals.var_xmax2_dn8 = assign86590_e132243_d_n8;
        locals.var_xmax2_dn9 = assign86590_e132243_d_n9;
        locals.var_xmax2_dn10 = assign86590_e132243_d_n10;
        locals.var_xmax2_dn11 = assign86590_e132243_d_n11;
        locals.var_xmax2_dn14 = assign86590_e132243_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign86600_e132251, assign86600_e132251_d_n0, assign86600_e132251_d_n2, assign86600_e132251_d_n4, assign86600_e132251_d_n5, assign86600_e132251_d_n6, assign86600_e132251_d_n7, assign86600_e132251_d_n8, assign86600_e132251_d_n9, assign86600_e132251_d_n10, assign86600_e132251_d_n11, assign86600_e132251_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign86600_e132251;
        locals.var_xp_dn0 = assign86600_e132251_d_n0;
        locals.var_xp_dn2 = assign86600_e132251_d_n2;
        locals.var_xp_dn4 = assign86600_e132251_d_n4;
        locals.var_xp_dn5 = assign86600_e132251_d_n5;
        locals.var_xp_dn6 = assign86600_e132251_d_n6;
        locals.var_xp_dn7 = assign86600_e132251_d_n7;
        locals.var_xp_dn8 = assign86600_e132251_d_n8;
        locals.var_xp_dn9 = assign86600_e132251_d_n9;
        locals.var_xp_dn10 = assign86600_e132251_d_n10;
        locals.var_xp_dn11 = assign86600_e132251_d_n11;
        locals.var_xp_dn14 = assign86600_e132251_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign86610_e132259, assign86610_e132259_d_n0, assign86610_e132259_d_n2, assign86610_e132259_d_n4, assign86610_e132259_d_n5, assign86610_e132259_d_n6, assign86610_e132259_d_n7, assign86610_e132259_d_n8, assign86610_e132259_d_n9, assign86610_e132259_d_n10, assign86610_e132259_d_n11, assign86610_e132259_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign86610_e132259;
        locals.var_xmp_dn0 = assign86610_e132259_d_n0;
        locals.var_xmp_dn2 = assign86610_e132259_d_n2;
        locals.var_xmp_dn4 = assign86610_e132259_d_n4;
        locals.var_xmp_dn5 = assign86610_e132259_d_n5;
        locals.var_xmp_dn6 = assign86610_e132259_d_n6;
        locals.var_xmp_dn7 = assign86610_e132259_d_n7;
        locals.var_xmp_dn8 = assign86610_e132259_d_n8;
        locals.var_xmp_dn9 = assign86610_e132259_d_n9;
        locals.var_xmp_dn10 = assign86610_e132259_d_n10;
        locals.var_xmp_dn11 = assign86610_e132259_d_n11;
        locals.var_xmp_dn14 = assign86610_e132259_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign86620_e132267,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign86620_e132267;
        locals.var_m0_rv = 0.0;

        let (assign86630_e132275,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86630_e132275;
        locals.var_mm_rv = 0.0;

        let (assign86640_e132283, assign86640_e132283_d_n0, assign86640_e132283_d_n2, assign86640_e132283_d_n4, assign86640_e132283_d_n5, assign86640_e132283_d_n6, assign86640_e132283_d_n7, assign86640_e132283_d_n8, assign86640_e132283_d_n9, assign86640_e132283_d_n10, assign86640_e132283_d_n11, assign86640_e132283_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign86640_e132283;
        locals.var_arg_dn0 = assign86640_e132283_d_n0;
        locals.var_arg_dn2 = assign86640_e132283_d_n2;
        locals.var_arg_dn4 = assign86640_e132283_d_n4;
        locals.var_arg_dn5 = assign86640_e132283_d_n5;
        locals.var_arg_dn6 = assign86640_e132283_d_n6;
        locals.var_arg_dn7 = assign86640_e132283_d_n7;
        locals.var_arg_dn8 = assign86640_e132283_d_n8;
        locals.var_arg_dn9 = assign86640_e132283_d_n9;
        locals.var_arg_dn10 = assign86640_e132283_d_n10;
        locals.var_arg_dn11 = assign86640_e132283_d_n11;
        locals.var_arg_dn14 = assign86640_e132283_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_332(
        locals: &mut StampLocals,
    ) {
        let (assign86650_e132291, assign86650_e132291_d_n0, assign86650_e132291_d_n2, assign86650_e132291_d_n4, assign86650_e132291_d_n5, assign86650_e132291_d_n6, assign86650_e132291_d_n7, assign86650_e132291_d_n8, assign86650_e132291_d_n9, assign86650_e132291_d_n10, assign86650_e132291_d_n11, assign86650_e132291_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign86650_e132291;
        locals.var_dnm_dn0 = assign86650_e132291_d_n0;
        locals.var_dnm_dn2 = assign86650_e132291_d_n2;
        locals.var_dnm_dn4 = assign86650_e132291_d_n4;
        locals.var_dnm_dn5 = assign86650_e132291_d_n5;
        locals.var_dnm_dn6 = assign86650_e132291_d_n6;
        locals.var_dnm_dn7 = assign86650_e132291_d_n7;
        locals.var_dnm_dn8 = assign86650_e132291_d_n8;
        locals.var_dnm_dn9 = assign86650_e132291_d_n9;
        locals.var_dnm_dn10 = assign86650_e132291_d_n10;
        locals.var_dnm_dn11 = assign86650_e132291_d_n11;
        locals.var_dnm_dn14 = assign86650_e132291_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign86660_e132301, assign86660_e132301_d_n0, assign86660_e132301_d_n2, assign86660_e132301_d_n4, assign86660_e132301_d_n5, assign86660_e132301_d_n6, assign86660_e132301_d_n7, assign86660_e132301_d_n8, assign86660_e132301_d_n9, assign86660_e132301_d_n10, assign86660_e132301_d_n11, assign86660_e132301_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86660_e132299: f64 = (locals.var_xp * locals.var_x2);
        (assign86660_e132299, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign86660_e132301;
        locals.var_xp_dn0 = assign86660_e132301_d_n0;
        locals.var_xp_dn2 = assign86660_e132301_d_n2;
        locals.var_xp_dn4 = assign86660_e132301_d_n4;
        locals.var_xp_dn5 = assign86660_e132301_d_n5;
        locals.var_xp_dn6 = assign86660_e132301_d_n6;
        locals.var_xp_dn7 = assign86660_e132301_d_n7;
        locals.var_xp_dn8 = assign86660_e132301_d_n8;
        locals.var_xp_dn9 = assign86660_e132301_d_n9;
        locals.var_xp_dn10 = assign86660_e132301_d_n10;
        locals.var_xp_dn11 = assign86660_e132301_d_n11;
        locals.var_xp_dn14 = assign86660_e132301_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign86670_e132311, assign86670_e132311_d_n0, assign86670_e132311_d_n2, assign86670_e132311_d_n4, assign86670_e132311_d_n5, assign86670_e132311_d_n6, assign86670_e132311_d_n7, assign86670_e132311_d_n8, assign86670_e132311_d_n9, assign86670_e132311_d_n10, assign86670_e132311_d_n11, assign86670_e132311_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86670_e132309: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign86670_e132309, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign86670_e132311;
        locals.var_xmp_dn0 = assign86670_e132311_d_n0;
        locals.var_xmp_dn2 = assign86670_e132311_d_n2;
        locals.var_xmp_dn4 = assign86670_e132311_d_n4;
        locals.var_xmp_dn5 = assign86670_e132311_d_n5;
        locals.var_xmp_dn6 = assign86670_e132311_d_n6;
        locals.var_xmp_dn7 = assign86670_e132311_d_n7;
        locals.var_xmp_dn8 = assign86670_e132311_d_n8;
        locals.var_xmp_dn9 = assign86670_e132311_d_n9;
        locals.var_xmp_dn10 = assign86670_e132311_d_n10;
        locals.var_xmp_dn11 = assign86670_e132311_d_n11;
        locals.var_xmp_dn14 = assign86670_e132311_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign86680_e132321, assign86680_e132321_d_n0, assign86680_e132321_d_n2, assign86680_e132321_d_n4, assign86680_e132321_d_n5, assign86680_e132321_d_n6, assign86680_e132321_d_n7, assign86680_e132321_d_n8, assign86680_e132321_d_n9, assign86680_e132321_d_n10, assign86680_e132321_d_n11, assign86680_e132321_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86680_e132319: f64 = (locals.var_xp + locals.var_xmp);
        (assign86680_e132319, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign86680_e132321;
        locals.var_arg_dn0 = assign86680_e132321_d_n0;
        locals.var_arg_dn2 = assign86680_e132321_d_n2;
        locals.var_arg_dn4 = assign86680_e132321_d_n4;
        locals.var_arg_dn5 = assign86680_e132321_d_n5;
        locals.var_arg_dn6 = assign86680_e132321_d_n6;
        locals.var_arg_dn7 = assign86680_e132321_d_n7;
        locals.var_arg_dn8 = assign86680_e132321_d_n8;
        locals.var_arg_dn9 = assign86680_e132321_d_n9;
        locals.var_arg_dn10 = assign86680_e132321_d_n10;
        locals.var_arg_dn11 = assign86680_e132321_d_n11;
        locals.var_arg_dn14 = assign86680_e132321_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign86690_e132329, assign86690_e132329_d_n0, assign86690_e132329_d_n2, assign86690_e132329_d_n4, assign86690_e132329_d_n5, assign86690_e132329_d_n6, assign86690_e132329_d_n7, assign86690_e132329_d_n8, assign86690_e132329_d_n9, assign86690_e132329_d_n10, assign86690_e132329_d_n11, assign86690_e132329_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign86690_e132329;
        locals.var_dnm_dn0 = assign86690_e132329_d_n0;
        locals.var_dnm_dn2 = assign86690_e132329_d_n2;
        locals.var_dnm_dn4 = assign86690_e132329_d_n4;
        locals.var_dnm_dn5 = assign86690_e132329_d_n5;
        locals.var_dnm_dn6 = assign86690_e132329_d_n6;
        locals.var_dnm_dn7 = assign86690_e132329_d_n7;
        locals.var_dnm_dn8 = assign86690_e132329_d_n8;
        locals.var_dnm_dn9 = assign86690_e132329_d_n9;
        locals.var_dnm_dn10 = assign86690_e132329_d_n10;
        locals.var_dnm_dn11 = assign86690_e132329_d_n11;
        locals.var_dnm_dn14 = assign86690_e132329_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign86700_e132344: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2025 = assign86700_e132344;
        locals.var_guard2025_rv = 0.0;

        let assign86710_e132347: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2026 = assign86710_e132347;
        locals.var_guard2026_rv = 0.0;

        let (assign86720_e132359,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 != 0.0)) && (locals.var_guard2026 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86720_e132359;
        locals.var_mm_rv = 0.0;

        let assign86730_e132362: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2027 = assign86730_e132362;
        locals.var_guard2027_rv = 0.0;

        let (assign86740_e132377,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 != 0.0)) && (locals.var_guard2026 == 0.0)) && (locals.var_guard2027 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86740_e132377;
        locals.var_mm_rv = 0.0;

        let assign86750_e132380: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2028 = assign86750_e132380;
        locals.var_guard2028_rv = 0.0;

        let (assign86760_e132398,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 != 0.0)) && (locals.var_guard2026 == 0.0)) && (locals.var_guard2027 == 0.0)) && (locals.var_guard2028 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86760_e132398;
        locals.var_mm_rv = 0.0;

        let assign86770_e132401: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2029 = assign86770_e132401;
        locals.var_guard2029_rv = 0.0;

        let (assign86780_e132422,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 != 0.0)) && (locals.var_guard2026 == 0.0)) && (locals.var_guard2027 == 0.0)) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2029 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86780_e132422;
        locals.var_mm_rv = 0.0;

        let (assign86790_e132432,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign86790_e132432;
        locals.var_m0_rv = 0.0;

        let mut assign86800_loop_guard: usize = 0;
        while {
            let assign86800_cond_e132443: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign86800_cond_e132443 != 0.0
        } {
            assign86800_loop_guard += 1;
            assert!(assign86800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign86800_body0_e132454, assign86800_body0_e132454_d_n0, assign86800_body0_e132454_d_n2, assign86800_body0_e132454_d_n4, assign86800_body0_e132454_d_n5, assign86800_body0_e132454_d_n6, assign86800_body0_e132454_d_n7, assign86800_body0_e132454_d_n8, assign86800_body0_e132454_d_n9, assign86800_body0_e132454_d_n10, assign86800_body0_e132454_d_n11, assign86800_body0_e132454_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 != 0.0)) {
        let assign86800_body0_e132452: f64 = (locals.var_dnm).sqrt();
        (assign86800_body0_e132452, (locals.var_dnm_dn0 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn2 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn4 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn5 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn6 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn7 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn8 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn9 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn10 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn11 / (2.0 * assign86800_body0_e132452)), (locals.var_dnm_dn14 / (2.0 * assign86800_body0_e132452)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign86800_body0_e132454;
            locals.var_dnm_dn0 = assign86800_body0_e132454_d_n0;
            locals.var_dnm_dn2 = assign86800_body0_e132454_d_n2;
            locals.var_dnm_dn4 = assign86800_body0_e132454_d_n4;
            locals.var_dnm_dn5 = assign86800_body0_e132454_d_n5;
            locals.var_dnm_dn6 = assign86800_body0_e132454_d_n6;
            locals.var_dnm_dn7 = assign86800_body0_e132454_d_n7;
            locals.var_dnm_dn8 = assign86800_body0_e132454_d_n8;
            locals.var_dnm_dn9 = assign86800_body0_e132454_d_n9;
            locals.var_dnm_dn10 = assign86800_body0_e132454_d_n10;
            locals.var_dnm_dn11 = assign86800_body0_e132454_d_n11;
            locals.var_dnm_dn14 = assign86800_body0_e132454_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign86800_body1_e132466,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 != 0.0)) {
        let assign86800_body1_e132464: f64 = (locals.var_m0 + 1.0);
        (assign86800_body1_e132464,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign86800_body1_e132466;
            locals.var_m0_rv = 0.0;
        }

        let (assign86810_e132488, assign86810_e132488_d_n0, assign86810_e132488_d_n2, assign86810_e132488_d_n4, assign86810_e132488_d_n5, assign86810_e132488_d_n6, assign86810_e132488_d_n7, assign86810_e132488_d_n8, assign86810_e132488_d_n9, assign86810_e132488_d_n10, assign86810_e132488_d_n11, assign86810_e132488_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) && (locals.var_guard2025 == 0.0)) {
        let (assign86810_e132486, assign86810_e132486_d_n0, assign86810_e132486_d_n2, assign86810_e132486_d_n4, assign86810_e132486_d_n5, assign86810_e132486_d_n6, assign86810_e132486_d_n7, assign86810_e132486_d_n8, assign86810_e132486_d_n9, assign86810_e132486_d_n10, assign86810_e132486_d_n11, assign86810_e132486_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign86810_e132483: f64 = 2.0;
                let assign86810_e132484: f64 = (1.0 / assign86810_e132483);
                let assign86810_e132485: f64 = (locals.var_dnm).powf(assign86810_e132484);
                (assign86810_e132485, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn0)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn2)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn4)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn5)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn6)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn7)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn8)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn9)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn10)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn11)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86810_e132484) as f64).is_finite() && ((assign86810_e132484) as f64).fract() == 0.0 { if assign86810_e132484 == 0.0 { 0.0 } else { (assign86810_e132484 * ((locals.var_dnm).powf(assign86810_e132484 - 1.0) * locals.var_dnm_dn14)) } } else { (assign86810_e132485 * (assign86810_e132484 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign86810_e132486, assign86810_e132486_d_n0, assign86810_e132486_d_n2, assign86810_e132486_d_n4, assign86810_e132486_d_n5, assign86810_e132486_d_n6, assign86810_e132486_d_n7, assign86810_e132486_d_n8, assign86810_e132486_d_n9, assign86810_e132486_d_n10, assign86810_e132486_d_n11, assign86810_e132486_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign86810_e132488;
        locals.var_dnm_dn0 = assign86810_e132488_d_n0;
        locals.var_dnm_dn2 = assign86810_e132488_d_n2;
        locals.var_dnm_dn4 = assign86810_e132488_d_n4;
        locals.var_dnm_dn5 = assign86810_e132488_d_n5;
        locals.var_dnm_dn6 = assign86810_e132488_d_n6;
        locals.var_dnm_dn7 = assign86810_e132488_d_n7;
        locals.var_dnm_dn8 = assign86810_e132488_d_n8;
        locals.var_dnm_dn9 = assign86810_e132488_d_n9;
        locals.var_dnm_dn10 = assign86810_e132488_d_n10;
        locals.var_dnm_dn11 = assign86810_e132488_d_n11;
        locals.var_dnm_dn14 = assign86810_e132488_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign86820_e132498, assign86820_e132498_d_n0, assign86820_e132498_d_n2, assign86820_e132498_d_n4, assign86820_e132498_d_n5, assign86820_e132498_d_n6, assign86820_e132498_d_n7, assign86820_e132498_d_n8, assign86820_e132498_d_n9, assign86820_e132498_d_n10, assign86820_e132498_d_n11, assign86820_e132498_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86820_e132496: f64 = (1.0 / locals.var_dnm);
        (assign86820_e132496, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign86820_e132498;
        locals.var_dnm_dn0 = assign86820_e132498_d_n0;
        locals.var_dnm_dn2 = assign86820_e132498_d_n2;
        locals.var_dnm_dn4 = assign86820_e132498_d_n4;
        locals.var_dnm_dn5 = assign86820_e132498_d_n5;
        locals.var_dnm_dn6 = assign86820_e132498_d_n6;
        locals.var_dnm_dn7 = assign86820_e132498_d_n7;
        locals.var_dnm_dn8 = assign86820_e132498_d_n8;
        locals.var_dnm_dn9 = assign86820_e132498_d_n9;
        locals.var_dnm_dn10 = assign86820_e132498_d_n10;
        locals.var_dnm_dn11 = assign86820_e132498_d_n11;
        locals.var_dnm_dn14 = assign86820_e132498_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign86830_e132510, assign86830_e132510_d_n0, assign86830_e132510_d_n2, assign86830_e132510_d_n4, assign86830_e132510_d_n5, assign86830_e132510_d_n6, assign86830_e132510_d_n7, assign86830_e132510_d_n8, assign86830_e132510_d_n9, assign86830_e132510_d_n10, assign86830_e132510_d_n11, assign86830_e132510_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86830_e132506: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign86830_e132508: f64 = (assign86830_e132506 * locals.var_dnm);
        (assign86830_e132508, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign86830_e132506 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign86830_e132510;
        locals.var_tmf0_dn0 = assign86830_e132510_d_n0;
        locals.var_tmf0_dn2 = assign86830_e132510_d_n2;
        locals.var_tmf0_dn4 = assign86830_e132510_d_n4;
        locals.var_tmf0_dn5 = assign86830_e132510_d_n5;
        locals.var_tmf0_dn6 = assign86830_e132510_d_n6;
        locals.var_tmf0_dn7 = assign86830_e132510_d_n7;
        locals.var_tmf0_dn8 = assign86830_e132510_d_n8;
        locals.var_tmf0_dn9 = assign86830_e132510_d_n9;
        locals.var_tmf0_dn10 = assign86830_e132510_d_n10;
        locals.var_tmf0_dn11 = assign86830_e132510_d_n11;
        locals.var_tmf0_dn14 = assign86830_e132510_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign86840_e132524, assign86840_e132524_d_n0, assign86840_e132524_d_n2, assign86840_e132524_d_n4, assign86840_e132524_d_n5, assign86840_e132524_d_n6, assign86840_e132524_d_n7, assign86840_e132524_d_n8, assign86840_e132524_d_n9, assign86840_e132524_d_n10, assign86840_e132524_d_n11, assign86840_e132524_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86840_e132518: f64 = (locals.var_t1 * locals.var_xmp);
        let assign86840_e132520: f64 = (assign86840_e132518 * locals.var_dnm);
        let assign86840_e132522: f64 = (assign86840_e132520 / locals.var_arg);
        (assign86840_e132522, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn0)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn2)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn4)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn5)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn6)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn7)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn8)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn9)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn10)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn11)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign86840_e132518 * locals.var_dnm_dn14)) * locals.var_arg) - (assign86840_e132520 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86840_e132524;
        locals.var_t0_dn0 = assign86840_e132524_d_n0;
        locals.var_t0_dn2 = assign86840_e132524_d_n2;
        locals.var_t0_dn4 = assign86840_e132524_d_n4;
        locals.var_t0_dn5 = assign86840_e132524_d_n5;
        locals.var_t0_dn6 = assign86840_e132524_d_n6;
        locals.var_t0_dn7 = assign86840_e132524_d_n7;
        locals.var_t0_dn8 = assign86840_e132524_d_n8;
        locals.var_t0_dn9 = assign86840_e132524_d_n9;
        locals.var_t0_dn10 = assign86840_e132524_d_n10;
        locals.var_t0_dn11 = assign86840_e132524_d_n11;
        locals.var_t0_dn14 = assign86840_e132524_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86850_e132536, assign86850_e132536_d_n0, assign86850_e132536_d_n2, assign86850_e132536_d_n4, assign86850_e132536_d_n5, assign86850_e132536_d_n6, assign86850_e132536_d_n7, assign86850_e132536_d_n8, assign86850_e132536_d_n9, assign86850_e132536_d_n10, assign86850_e132536_d_n11, assign86850_e132536_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        let assign86850_e132532: f64 = (-locals.var_t1);
        let assign86850_e132534: f64 = (assign86850_e132532 + locals.var_tmf0);
        (assign86850_e132534, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign86850_e132536;
        locals.var_t1_dn0 = assign86850_e132536_d_n0;
        locals.var_t1_dn2 = assign86850_e132536_d_n2;
        locals.var_t1_dn4 = assign86850_e132536_d_n4;
        locals.var_t1_dn5 = assign86850_e132536_d_n5;
        locals.var_t1_dn6 = assign86850_e132536_d_n6;
        locals.var_t1_dn7 = assign86850_e132536_d_n7;
        locals.var_t1_dn8 = assign86850_e132536_d_n8;
        locals.var_t1_dn9 = assign86850_e132536_d_n9;
        locals.var_t1_dn10 = assign86850_e132536_d_n10;
        locals.var_t1_dn11 = assign86850_e132536_d_n11;
        locals.var_t1_dn14 = assign86850_e132536_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign86860_e132544, assign86860_e132544_d_n0, assign86860_e132544_d_n2, assign86860_e132544_d_n4, assign86860_e132544_d_n5, assign86860_e132544_d_n6, assign86860_e132544_d_n7, assign86860_e132544_d_n8, assign86860_e132544_d_n9, assign86860_e132544_d_n10, assign86860_e132544_d_n11, assign86860_e132544_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86860_e132544;
        locals.var_t0_dn0 = assign86860_e132544_d_n0;
        locals.var_t0_dn2 = assign86860_e132544_d_n2;
        locals.var_t0_dn4 = assign86860_e132544_d_n4;
        locals.var_t0_dn5 = assign86860_e132544_d_n5;
        locals.var_t0_dn6 = assign86860_e132544_d_n6;
        locals.var_t0_dn7 = assign86860_e132544_d_n7;
        locals.var_t0_dn8 = assign86860_e132544_d_n8;
        locals.var_t0_dn9 = assign86860_e132544_d_n9;
        locals.var_t0_dn10 = assign86860_e132544_d_n10;
        locals.var_t0_dn11 = assign86860_e132544_d_n11;
        locals.var_t0_dn14 = assign86860_e132544_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86870_e132555, assign86870_e132555_d_n0, assign86870_e132555_d_n2, assign86870_e132555_d_n4, assign86870_e132555_d_n5, assign86870_e132555_d_n6, assign86870_e132555_d_n7, assign86870_e132555_d_n8, assign86870_e132555_d_n9, assign86870_e132555_d_n10, assign86870_e132555_d_n11, assign86870_e132555_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 == 0.0)) {
        let assign86870_e132553: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign86870_e132553, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign86870_e132555;
        locals.var_t1_dn0 = assign86870_e132555_d_n0;
        locals.var_t1_dn2 = assign86870_e132555_d_n2;
        locals.var_t1_dn4 = assign86870_e132555_d_n4;
        locals.var_t1_dn5 = assign86870_e132555_d_n5;
        locals.var_t1_dn6 = assign86870_e132555_d_n6;
        locals.var_t1_dn7 = assign86870_e132555_d_n7;
        locals.var_t1_dn8 = assign86870_e132555_d_n8;
        locals.var_t1_dn9 = assign86870_e132555_d_n9;
        locals.var_t1_dn10 = assign86870_e132555_d_n10;
        locals.var_t1_dn11 = assign86870_e132555_d_n11;
        locals.var_t1_dn14 = assign86870_e132555_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign86880_e132564, assign86880_e132564_d_n0, assign86880_e132564_d_n2, assign86880_e132564_d_n4, assign86880_e132564_d_n5, assign86880_e132564_d_n6, assign86880_e132564_d_n7, assign86880_e132564_d_n8, assign86880_e132564_d_n9, assign86880_e132564_d_n10, assign86880_e132564_d_n11, assign86880_e132564_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2024 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign86880_e132564;
        locals.var_t0_dn0 = assign86880_e132564_d_n0;
        locals.var_t0_dn2 = assign86880_e132564_d_n2;
        locals.var_t0_dn4 = assign86880_e132564_d_n4;
        locals.var_t0_dn5 = assign86880_e132564_d_n5;
        locals.var_t0_dn6 = assign86880_e132564_d_n6;
        locals.var_t0_dn7 = assign86880_e132564_d_n7;
        locals.var_t0_dn8 = assign86880_e132564_d_n8;
        locals.var_t0_dn9 = assign86880_e132564_d_n9;
        locals.var_t0_dn10 = assign86880_e132564_d_n10;
        locals.var_t0_dn11 = assign86880_e132564_d_n11;
        locals.var_t0_dn14 = assign86880_e132564_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign86890_e132572, assign86890_e132572_d_n0, assign86890_e132572_d_n2, assign86890_e132572_d_n4, assign86890_e132572_d_n5, assign86890_e132572_d_n6, assign86890_e132572_d_n7, assign86890_e132572_d_n8, assign86890_e132572_d_n9, assign86890_e132572_d_n10, assign86890_e132572_d_n11, assign86890_e132572_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86890_e132570: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign86890_e132570, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign86890_e132572;
        locals.var_vxbgmtcl_dn0 = assign86890_e132572_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86890_e132572_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86890_e132572_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86890_e132572_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86890_e132572_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86890_e132572_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86890_e132572_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86890_e132572_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86890_e132572_d_n10;
        locals.var_vxbgmtcl_dn11 = assign86890_e132572_d_n11;
        locals.var_vxbgmtcl_dn14 = assign86890_e132572_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign86900_e132583, assign86900_e132583_d_n0, assign86900_e132583_d_n2, assign86900_e132583_d_n4, assign86900_e132583_d_n5, assign86900_e132583_d_n6, assign86900_e132583_d_n7, assign86900_e132583_d_n8, assign86900_e132583_d_n9, assign86900_e132583_d_n10, assign86900_e132583_d_n11, assign86900_e132583_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86900_e132577: f64 = (-locals.var_vxbgmtcl);
        let assign86900_e132580: f64 = (10.0 * 2.220446049250313e-16);
        let assign86900_e132581: f64 = (assign86900_e132577 + assign86900_e132580);
        (assign86900_e132581, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign86900_e132583;
        locals.var_vgb_fb_ld_dn0 = assign86900_e132583_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign86900_e132583_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign86900_e132583_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign86900_e132583_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign86900_e132583_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign86900_e132583_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign86900_e132583_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign86900_e132583_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign86900_e132583_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign86900_e132583_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign86900_e132583_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign86910_e132586: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard2030 = assign86910_e132586;
        locals.var_guard2030_rv = 0.0;

        let (assign86930_e132607, assign86930_e132607_d_n0, assign86930_e132607_d_n2, assign86930_e132607_d_n4, assign86930_e132607_d_n5, assign86930_e132607_d_n6, assign86930_e132607_d_n7, assign86930_e132607_d_n8, assign86930_e132607_d_n9, assign86930_e132607_d_n10, assign86930_e132607_d_n11, assign86930_e132607_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign86930_e132599: f64 = (2.0 * locals.var_beta_inv);
        let assign86930_e132601: f64 = (-locals.var_vgs_min);
        let assign86930_e132603: f64 = (assign86930_e132601 / locals.var_fac1);
        let assign86930_e132604: f64 = (assign86930_e132603).ln();
        let assign86930_e132605: f64 = (assign86930_e132599 * assign86930_e132604);
        (assign86930_e132605, (((2.0 * locals.var_beta_inv_dn0) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn2) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn4) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn5) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn6) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn7) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn8) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn9) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn10) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn11) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))), (((2.0 * locals.var_beta_inv_dn14) * assign86930_e132604) + (assign86930_e132599 * ((-((assign86930_e132601 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign86930_e132603))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign86930_e132607;
        locals.var_ps0_min_dn0 = assign86930_e132607_d_n0;
        locals.var_ps0_min_dn2 = assign86930_e132607_d_n2;
        locals.var_ps0_min_dn4 = assign86930_e132607_d_n4;
        locals.var_ps0_min_dn5 = assign86930_e132607_d_n5;
        locals.var_ps0_min_dn6 = assign86930_e132607_d_n6;
        locals.var_ps0_min_dn7 = assign86930_e132607_d_n7;
        locals.var_ps0_min_dn8 = assign86930_e132607_d_n8;
        locals.var_ps0_min_dn9 = assign86930_e132607_d_n9;
        locals.var_ps0_min_dn10 = assign86930_e132607_d_n10;
        locals.var_ps0_min_dn11 = assign86930_e132607_d_n11;
        locals.var_ps0_min_dn14 = assign86930_e132607_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign86940_e132617, assign86940_e132617_d_n0, assign86940_e132617_d_n2, assign86940_e132617_d_n4, assign86940_e132617_d_n5, assign86940_e132617_d_n6, assign86940_e132617_d_n7, assign86940_e132617_d_n8, assign86940_e132617_d_n9, assign86940_e132617_d_n10, assign86940_e132617_d_n11, assign86940_e132617_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign86940_e132614: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86940_e132615: f64 = (locals.var_beta * assign86940_e132614);
        (assign86940_e132615, ((locals.var_beta_dn0 * assign86940_e132614) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign86940_e132614) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign86940_e132614) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign86940_e132614) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign86940_e132614) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign86940_e132614) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign86940_e132614) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign86940_e132614) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign86940_e132614) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign86940_e132614) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign86940_e132614) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign86940_e132617;
        locals.var_tx_dn0 = assign86940_e132617_d_n0;
        locals.var_tx_dn2 = assign86940_e132617_d_n2;
        locals.var_tx_dn4 = assign86940_e132617_d_n4;
        locals.var_tx_dn5 = assign86940_e132617_d_n5;
        locals.var_tx_dn6 = assign86940_e132617_d_n6;
        locals.var_tx_dn7 = assign86940_e132617_d_n7;
        locals.var_tx_dn8 = assign86940_e132617_d_n8;
        locals.var_tx_dn9 = assign86940_e132617_d_n9;
        locals.var_tx_dn10 = assign86940_e132617_d_n10;
        locals.var_tx_dn11 = assign86940_e132617_d_n11;
        locals.var_tx_dn14 = assign86940_e132617_d_n14;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_333(
        locals: &mut StampLocals,
    ) {
        let (assign86950_e132627, assign86950_e132627_d_n0, assign86950_e132627_d_n2, assign86950_e132627_d_n4, assign86950_e132627_d_n5, assign86950_e132627_d_n6, assign86950_e132627_d_n7, assign86950_e132627_d_n8, assign86950_e132627_d_n9, assign86950_e132627_d_n10, assign86950_e132627_d_n11, assign86950_e132627_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign86950_e132624: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign86950_e132625: f64 = (1.0 / assign86950_e132624);
        (assign86950_e132625, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign86950_e132624 * assign86950_e132624))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign86950_e132624 * assign86950_e132624))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign86950_e132627;
        locals.var_t1_dn0 = assign86950_e132627_d_n0;
        locals.var_t1_dn2 = assign86950_e132627_d_n2;
        locals.var_t1_dn4 = assign86950_e132627_d_n4;
        locals.var_t1_dn5 = assign86950_e132627_d_n5;
        locals.var_t1_dn6 = assign86950_e132627_d_n6;
        locals.var_t1_dn7 = assign86950_e132627_d_n7;
        locals.var_t1_dn8 = assign86950_e132627_d_n8;
        locals.var_t1_dn9 = assign86950_e132627_d_n9;
        locals.var_t1_dn10 = assign86950_e132627_d_n10;
        locals.var_t1_dn11 = assign86950_e132627_d_n11;
        locals.var_t1_dn14 = assign86950_e132627_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign86960_e132635, assign86960_e132635_d_n0, assign86960_e132635_d_n2, assign86960_e132635_d_n4, assign86960_e132635_d_n5, assign86960_e132635_d_n6, assign86960_e132635_d_n7, assign86960_e132635_d_n8, assign86960_e132635_d_n9, assign86960_e132635_d_n10, assign86960_e132635_d_n11, assign86960_e132635_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign86960_e132633: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign86960_e132633, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign86960_e132635;
        locals.var_ty_dn0 = assign86960_e132635_d_n0;
        locals.var_ty_dn2 = assign86960_e132635_d_n2;
        locals.var_ty_dn4 = assign86960_e132635_d_n4;
        locals.var_ty_dn5 = assign86960_e132635_d_n5;
        locals.var_ty_dn6 = assign86960_e132635_d_n6;
        locals.var_ty_dn7 = assign86960_e132635_d_n7;
        locals.var_ty_dn8 = assign86960_e132635_d_n8;
        locals.var_ty_dn9 = assign86960_e132635_d_n9;
        locals.var_ty_dn10 = assign86960_e132635_d_n10;
        locals.var_ty_dn11 = assign86960_e132635_d_n11;
        locals.var_ty_dn14 = assign86960_e132635_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign86970_e132647, assign86970_e132647_d_n0, assign86970_e132647_d_n2, assign86970_e132647_d_n4, assign86970_e132647_d_n5, assign86970_e132647_d_n6, assign86970_e132647_d_n7, assign86970_e132647_d_n8, assign86970_e132647_d_n9, assign86970_e132647_d_n10, assign86970_e132647_d_n11, assign86970_e132647_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign86970_e132642: f64 = (3.0 * 1.414213562373095);
        let assign86970_e132644: f64 = (assign86970_e132642 * locals.var_ty);
        let assign86970_e132645: f64 = (2.0 + assign86970_e132644);
        (assign86970_e132645, (assign86970_e132642 * locals.var_ty_dn0), (assign86970_e132642 * locals.var_ty_dn2), (assign86970_e132642 * locals.var_ty_dn4), (assign86970_e132642 * locals.var_ty_dn5), (assign86970_e132642 * locals.var_ty_dn6), (assign86970_e132642 * locals.var_ty_dn7), (assign86970_e132642 * locals.var_ty_dn8), (assign86970_e132642 * locals.var_ty_dn9), (assign86970_e132642 * locals.var_ty_dn10), (assign86970_e132642 * locals.var_ty_dn11), (assign86970_e132642 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign86970_e132647;
        locals.var_ac41_dn0 = assign86970_e132647_d_n0;
        locals.var_ac41_dn2 = assign86970_e132647_d_n2;
        locals.var_ac41_dn4 = assign86970_e132647_d_n4;
        locals.var_ac41_dn5 = assign86970_e132647_d_n5;
        locals.var_ac41_dn6 = assign86970_e132647_d_n6;
        locals.var_ac41_dn7 = assign86970_e132647_d_n7;
        locals.var_ac41_dn8 = assign86970_e132647_d_n8;
        locals.var_ac41_dn9 = assign86970_e132647_d_n9;
        locals.var_ac41_dn10 = assign86970_e132647_d_n10;
        locals.var_ac41_dn11 = assign86970_e132647_d_n11;
        locals.var_ac41_dn14 = assign86970_e132647_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign86980_e132659, assign86980_e132659_d_n0, assign86980_e132659_d_n2, assign86980_e132659_d_n4, assign86980_e132659_d_n5, assign86980_e132659_d_n6, assign86980_e132659_d_n7, assign86980_e132659_d_n8, assign86980_e132659_d_n9, assign86980_e132659_d_n10, assign86980_e132659_d_n11, assign86980_e132659_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign86980_e132653: f64 = (8.0 * locals.var_ac41);
        let assign86980_e132655: f64 = (assign86980_e132653 * locals.var_ac41);
        let assign86980_e132657: f64 = (assign86980_e132655 * locals.var_ac41);
        (assign86980_e132657, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign86980_e132653 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign86980_e132655 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign86980_e132659;
        locals.var_ac4_dn0 = assign86980_e132659_d_n0;
        locals.var_ac4_dn2 = assign86980_e132659_d_n2;
        locals.var_ac4_dn4 = assign86980_e132659_d_n4;
        locals.var_ac4_dn5 = assign86980_e132659_d_n5;
        locals.var_ac4_dn6 = assign86980_e132659_d_n6;
        locals.var_ac4_dn7 = assign86980_e132659_d_n7;
        locals.var_ac4_dn8 = assign86980_e132659_d_n8;
        locals.var_ac4_dn9 = assign86980_e132659_d_n9;
        locals.var_ac4_dn10 = assign86980_e132659_d_n10;
        locals.var_ac4_dn11 = assign86980_e132659_d_n11;
        locals.var_ac4_dn14 = assign86980_e132659_d_n14;
        locals.var_ac4_rv = 0.0;

        let (assign86990_e132675, assign86990_e132675_d_n0, assign86990_e132675_d_n2, assign86990_e132675_d_n4, assign86990_e132675_d_n5, assign86990_e132675_d_n6, assign86990_e132675_d_n7, assign86990_e132675_d_n8, assign86990_e132675_d_n9, assign86990_e132675_d_n10, assign86990_e132675_d_n11, assign86990_e132675_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign86990_e132665: f64 = (7.0 * 1.414213562373095);
        let assign86990_e132668: f64 = (9.0 * locals.var_ty);
        let assign86990_e132671: f64 = (locals.var_tx - 2.0);
        let assign86990_e132672: f64 = (assign86990_e132668 * assign86990_e132671);
        let assign86990_e132673: f64 = (assign86990_e132665 - assign86990_e132672);
        (assign86990_e132673, (-(((9.0 * locals.var_ty_dn0) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign86990_e132671) + (assign86990_e132668 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign86990_e132675;
        locals.var_ac31_dn0 = assign86990_e132675_d_n0;
        locals.var_ac31_dn2 = assign86990_e132675_d_n2;
        locals.var_ac31_dn4 = assign86990_e132675_d_n4;
        locals.var_ac31_dn5 = assign86990_e132675_d_n5;
        locals.var_ac31_dn6 = assign86990_e132675_d_n6;
        locals.var_ac31_dn7 = assign86990_e132675_d_n7;
        locals.var_ac31_dn8 = assign86990_e132675_d_n8;
        locals.var_ac31_dn9 = assign86990_e132675_d_n9;
        locals.var_ac31_dn10 = assign86990_e132675_d_n10;
        locals.var_ac31_dn11 = assign86990_e132675_d_n11;
        locals.var_ac31_dn14 = assign86990_e132675_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign87000_e132683, assign87000_e132683_d_n0, assign87000_e132683_d_n2, assign87000_e132683_d_n4, assign87000_e132683_d_n5, assign87000_e132683_d_n6, assign87000_e132683_d_n7, assign87000_e132683_d_n8, assign87000_e132683_d_n9, assign87000_e132683_d_n10, assign87000_e132683_d_n11, assign87000_e132683_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87000_e132681: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign87000_e132681, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign87000_e132683;
        locals.var_ac3_dn0 = assign87000_e132683_d_n0;
        locals.var_ac3_dn2 = assign87000_e132683_d_n2;
        locals.var_ac3_dn4 = assign87000_e132683_d_n4;
        locals.var_ac3_dn5 = assign87000_e132683_d_n5;
        locals.var_ac3_dn6 = assign87000_e132683_d_n6;
        locals.var_ac3_dn7 = assign87000_e132683_d_n7;
        locals.var_ac3_dn8 = assign87000_e132683_d_n8;
        locals.var_ac3_dn9 = assign87000_e132683_d_n9;
        locals.var_ac3_dn10 = assign87000_e132683_d_n10;
        locals.var_ac3_dn11 = assign87000_e132683_d_n11;
        locals.var_ac3_dn14 = assign87000_e132683_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign87010_e132687: f64 = (locals.var_ac3 * 1e-8);
        let assign87010_e132688: f64 = if locals.var_ac4 < assign87010_e132687 { 1.0 } else { 0.0 };
        locals.var_guard2031 = assign87010_e132688;
        locals.var_guard2031_rv = 0.0;

        let (assign87030_e132709, assign87030_e132709_d_n0, assign87030_e132709_d_n2, assign87030_e132709_d_n4, assign87030_e132709_d_n5, assign87030_e132709_d_n6, assign87030_e132709_d_n7, assign87030_e132709_d_n8, assign87030_e132709_d_n9, assign87030_e132709_d_n10, assign87030_e132709_d_n11, assign87030_e132709_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87030_e132705: f64 = (0.5 * locals.var_ac4);
        let assign87030_e132707: f64 = (assign87030_e132705 / locals.var_ac31);
        (assign87030_e132707, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign87030_e132705 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign87030_e132709;
        locals.var_ac1_dn0 = assign87030_e132709_d_n0;
        locals.var_ac1_dn2 = assign87030_e132709_d_n2;
        locals.var_ac1_dn4 = assign87030_e132709_d_n4;
        locals.var_ac1_dn5 = assign87030_e132709_d_n5;
        locals.var_ac1_dn6 = assign87030_e132709_d_n6;
        locals.var_ac1_dn7 = assign87030_e132709_d_n7;
        locals.var_ac1_dn8 = assign87030_e132709_d_n8;
        locals.var_ac1_dn9 = assign87030_e132709_d_n9;
        locals.var_ac1_dn10 = assign87030_e132709_d_n10;
        locals.var_ac1_dn11 = assign87030_e132709_d_n11;
        locals.var_ac1_dn14 = assign87030_e132709_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign87040_e132721, assign87040_e132721_d_n0, assign87040_e132721_d_n2, assign87040_e132721_d_n4, assign87040_e132721_d_n5, assign87040_e132721_d_n6, assign87040_e132721_d_n7, assign87040_e132721_d_n8, assign87040_e132721_d_n9, assign87040_e132721_d_n10, assign87040_e132721_d_n11, assign87040_e132721_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87040_e132718: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign87040_e132719: f64 = (assign87040_e132718).sqrt();
        (assign87040_e132719, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign87040_e132719)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign87040_e132719)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign87040_e132721;
        locals.var_ac2_dn0 = assign87040_e132721_d_n0;
        locals.var_ac2_dn2 = assign87040_e132721_d_n2;
        locals.var_ac2_dn4 = assign87040_e132721_d_n4;
        locals.var_ac2_dn5 = assign87040_e132721_d_n5;
        locals.var_ac2_dn6 = assign87040_e132721_d_n6;
        locals.var_ac2_dn7 = assign87040_e132721_d_n7;
        locals.var_ac2_dn8 = assign87040_e132721_d_n8;
        locals.var_ac2_dn9 = assign87040_e132721_d_n9;
        locals.var_ac2_dn10 = assign87040_e132721_d_n10;
        locals.var_ac2_dn11 = assign87040_e132721_d_n11;
        locals.var_ac2_dn14 = assign87040_e132721_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign87050_e132733, assign87050_e132733_d_n0, assign87050_e132733_d_n2, assign87050_e132733_d_n4, assign87050_e132733_d_n5, assign87050_e132733_d_n6, assign87050_e132733_d_n7, assign87050_e132733_d_n8, assign87050_e132733_d_n9, assign87050_e132733_d_n10, assign87050_e132733_d_n11, assign87050_e132733_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87050_e132729: f64 = (-locals.var_ac31);
        let assign87050_e132731: f64 = (assign87050_e132729 + locals.var_ac2);
        (assign87050_e132731, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign87050_e132733;
        locals.var_ac1_dn0 = assign87050_e132733_d_n0;
        locals.var_ac1_dn2 = assign87050_e132733_d_n2;
        locals.var_ac1_dn4 = assign87050_e132733_d_n4;
        locals.var_ac1_dn5 = assign87050_e132733_d_n5;
        locals.var_ac1_dn6 = assign87050_e132733_d_n6;
        locals.var_ac1_dn7 = assign87050_e132733_d_n7;
        locals.var_ac1_dn8 = assign87050_e132733_d_n8;
        locals.var_ac1_dn9 = assign87050_e132733_d_n9;
        locals.var_ac1_dn10 = assign87050_e132733_d_n10;
        locals.var_ac1_dn11 = assign87050_e132733_d_n11;
        locals.var_ac1_dn14 = assign87050_e132733_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign87060_e132741, assign87060_e132741_d_n0, assign87060_e132741_d_n2, assign87060_e132741_d_n4, assign87060_e132741_d_n5, assign87060_e132741_d_n6, assign87060_e132741_d_n7, assign87060_e132741_d_n8, assign87060_e132741_d_n9, assign87060_e132741_d_n10, assign87060_e132741_d_n11, assign87060_e132741_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87060_e132739: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign87060_e132739, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign87060_e132739 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign87060_e132741;
        locals.var_acd_dn0 = assign87060_e132741_d_n0;
        locals.var_acd_dn2 = assign87060_e132741_d_n2;
        locals.var_acd_dn4 = assign87060_e132741_d_n4;
        locals.var_acd_dn5 = assign87060_e132741_d_n5;
        locals.var_acd_dn6 = assign87060_e132741_d_n6;
        locals.var_acd_dn7 = assign87060_e132741_d_n7;
        locals.var_acd_dn8 = assign87060_e132741_d_n8;
        locals.var_acd_dn9 = assign87060_e132741_d_n9;
        locals.var_acd_dn10 = assign87060_e132741_d_n10;
        locals.var_acd_dn11 = assign87060_e132741_d_n11;
        locals.var_acd_dn14 = assign87060_e132741_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign87070_e132764, assign87070_e132764_d_n0, assign87070_e132764_d_n2, assign87070_e132764_d_n4, assign87070_e132764_d_n5, assign87070_e132764_d_n6, assign87070_e132764_d_n7, assign87070_e132764_d_n8, assign87070_e132764_d_n9, assign87070_e132764_d_n10, assign87070_e132764_d_n11, assign87070_e132764_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87070_e132746: f64 = (-4.0);
        let assign87070_e132748: f64 = (assign87070_e132746 * 1.414213562373095);
        let assign87070_e132751: f64 = (12.0 * locals.var_ty);
        let assign87070_e132752: f64 = (assign87070_e132748 - assign87070_e132751);
        let assign87070_e132755: f64 = (2.0 * locals.var_acd);
        let assign87070_e132756: f64 = (assign87070_e132752 + assign87070_e132755);
        let assign87070_e132759: f64 = (1.414213562373095 * locals.var_acd);
        let assign87070_e132761: f64 = (assign87070_e132759 * locals.var_acd);
        let assign87070_e132762: f64 = (assign87070_e132756 + assign87070_e132761);
        (assign87070_e132762, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign87070_e132759 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign87070_e132764;
        locals.var_acn_dn0 = assign87070_e132764_d_n0;
        locals.var_acn_dn2 = assign87070_e132764_d_n2;
        locals.var_acn_dn4 = assign87070_e132764_d_n4;
        locals.var_acn_dn5 = assign87070_e132764_d_n5;
        locals.var_acn_dn6 = assign87070_e132764_d_n6;
        locals.var_acn_dn7 = assign87070_e132764_d_n7;
        locals.var_acn_dn8 = assign87070_e132764_d_n8;
        locals.var_acn_dn9 = assign87070_e132764_d_n9;
        locals.var_acn_dn10 = assign87070_e132764_d_n10;
        locals.var_acn_dn11 = assign87070_e132764_d_n11;
        locals.var_acn_dn14 = assign87070_e132764_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign87080_e132772, assign87080_e132772_d_n0, assign87080_e132772_d_n2, assign87080_e132772_d_n4, assign87080_e132772_d_n5, assign87080_e132772_d_n6, assign87080_e132772_d_n7, assign87080_e132772_d_n8, assign87080_e132772_d_n9, assign87080_e132772_d_n10, assign87080_e132772_d_n11, assign87080_e132772_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87080_e132770: f64 = (locals.var_acn / locals.var_acd);
        (assign87080_e132770, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign87080_e132772;
        locals.var_chi_dn0 = assign87080_e132772_d_n0;
        locals.var_chi_dn2 = assign87080_e132772_d_n2;
        locals.var_chi_dn4 = assign87080_e132772_d_n4;
        locals.var_chi_dn5 = assign87080_e132772_d_n5;
        locals.var_chi_dn6 = assign87080_e132772_d_n6;
        locals.var_chi_dn7 = assign87080_e132772_d_n7;
        locals.var_chi_dn8 = assign87080_e132772_d_n8;
        locals.var_chi_dn9 = assign87080_e132772_d_n9;
        locals.var_chi_dn10 = assign87080_e132772_d_n10;
        locals.var_chi_dn11 = assign87080_e132772_d_n11;
        locals.var_chi_dn14 = assign87080_e132772_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign87090_e132780, assign87090_e132780_d_n0, assign87090_e132780_d_n2, assign87090_e132780_d_n4, assign87090_e132780_d_n5, assign87090_e132780_d_n6, assign87090_e132780_d_n7, assign87090_e132780_d_n8, assign87090_e132780_d_n9, assign87090_e132780_d_n10, assign87090_e132780_d_n11, assign87090_e132780_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87090_e132778: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign87090_e132778, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign87090_e132780;
        locals.var_t1_dn0 = assign87090_e132780_d_n0;
        locals.var_t1_dn2 = assign87090_e132780_d_n2;
        locals.var_t1_dn4 = assign87090_e132780_d_n4;
        locals.var_t1_dn5 = assign87090_e132780_d_n5;
        locals.var_t1_dn6 = assign87090_e132780_d_n6;
        locals.var_t1_dn7 = assign87090_e132780_d_n7;
        locals.var_t1_dn8 = assign87090_e132780_d_n8;
        locals.var_t1_dn9 = assign87090_e132780_d_n9;
        locals.var_t1_dn10 = assign87090_e132780_d_n10;
        locals.var_t1_dn11 = assign87090_e132780_d_n11;
        locals.var_t1_dn14 = assign87090_e132780_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign87100_e132788, assign87100_e132788_d_n0, assign87100_e132788_d_n2, assign87100_e132788_d_n4, assign87100_e132788_d_n5, assign87100_e132788_d_n6, assign87100_e132788_d_n7, assign87100_e132788_d_n8, assign87100_e132788_d_n9, assign87100_e132788_d_n10, assign87100_e132788_d_n11, assign87100_e132788_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87100_e132786: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign87100_e132786, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign87100_e132788;
        locals.var_t2_dn0 = assign87100_e132788_d_n0;
        locals.var_t2_dn2 = assign87100_e132788_d_n2;
        locals.var_t2_dn4 = assign87100_e132788_d_n4;
        locals.var_t2_dn5 = assign87100_e132788_d_n5;
        locals.var_t2_dn6 = assign87100_e132788_d_n6;
        locals.var_t2_dn7 = assign87100_e132788_d_n7;
        locals.var_t2_dn8 = assign87100_e132788_d_n8;
        locals.var_t2_dn9 = assign87100_e132788_d_n9;
        locals.var_t2_dn10 = assign87100_e132788_d_n10;
        locals.var_t2_dn11 = assign87100_e132788_d_n11;
        locals.var_t2_dn14 = assign87100_e132788_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign87110_e132799, assign87110_e132799_d_n0, assign87110_e132799_d_n2, assign87110_e132799_d_n4, assign87110_e132799_d_n5, assign87110_e132799_d_n6, assign87110_e132799_d_n7, assign87110_e132799_d_n8, assign87110_e132799_d_n9, assign87110_e132799_d_n10, assign87110_e132799_d_n11, assign87110_e132799_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87110_e132795: f64 = (locals.var_t2 * locals.var_t2);
        let assign87110_e132796: f64 = (1.0 + assign87110_e132795);
        let assign87110_e132797: f64 = (assign87110_e132796).sqrt();
        (assign87110_e132797, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign87110_e132797)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign87110_e132797)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign87110_e132799;
        locals.var_t3_dn0 = assign87110_e132799_d_n0;
        locals.var_t3_dn2 = assign87110_e132799_d_n2;
        locals.var_t3_dn4 = assign87110_e132799_d_n4;
        locals.var_t3_dn5 = assign87110_e132799_d_n5;
        locals.var_t3_dn6 = assign87110_e132799_d_n6;
        locals.var_t3_dn7 = assign87110_e132799_d_n7;
        locals.var_t3_dn8 = assign87110_e132799_d_n8;
        locals.var_t3_dn9 = assign87110_e132799_d_n9;
        locals.var_t3_dn10 = assign87110_e132799_d_n10;
        locals.var_t3_dn11 = assign87110_e132799_d_n11;
        locals.var_t3_dn14 = assign87110_e132799_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign87120_e132809, assign87120_e132809_d_n0, assign87120_e132809_d_n2, assign87120_e132809_d_n4, assign87120_e132809_d_n5, assign87120_e132809_d_n6, assign87120_e132809_d_n7, assign87120_e132809_d_n8, assign87120_e132809_d_n9, assign87120_e132809_d_n10, assign87120_e132809_d_n11, assign87120_e132809_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87120_e132805: f64 = (locals.var_t1 / locals.var_t3);
        let assign87120_e132807: f64 = (assign87120_e132805 - locals.var_vxbgmtcl);
        (assign87120_e132807, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign87120_e132809;
        locals.var_ps0ld_dn0 = assign87120_e132809_d_n0;
        locals.var_ps0ld_dn2 = assign87120_e132809_d_n2;
        locals.var_ps0ld_dn4 = assign87120_e132809_d_n4;
        locals.var_ps0ld_dn5 = assign87120_e132809_d_n5;
        locals.var_ps0ld_dn6 = assign87120_e132809_d_n6;
        locals.var_ps0ld_dn7 = assign87120_e132809_d_n7;
        locals.var_ps0ld_dn8 = assign87120_e132809_d_n8;
        locals.var_ps0ld_dn9 = assign87120_e132809_d_n9;
        locals.var_ps0ld_dn10 = assign87120_e132809_d_n10;
        locals.var_ps0ld_dn11 = assign87120_e132809_d_n11;
        locals.var_ps0ld_dn14 = assign87120_e132809_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign87130_e132817, assign87130_e132817_d_n0, assign87130_e132817_d_n2, assign87130_e132817_d_n4, assign87130_e132817_d_n5, assign87130_e132817_d_n6, assign87130_e132817_d_n7, assign87130_e132817_d_n8, assign87130_e132817_d_n9, assign87130_e132817_d_n10, assign87130_e132817_d_n11, assign87130_e132817_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87130_e132815: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign87130_e132815, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign87130_e132817;
        locals.var_t2_dn0 = assign87130_e132817_d_n0;
        locals.var_t2_dn2 = assign87130_e132817_d_n2;
        locals.var_t2_dn4 = assign87130_e132817_d_n4;
        locals.var_t2_dn5 = assign87130_e132817_d_n5;
        locals.var_t2_dn6 = assign87130_e132817_d_n6;
        locals.var_t2_dn7 = assign87130_e132817_d_n7;
        locals.var_t2_dn8 = assign87130_e132817_d_n8;
        locals.var_t2_dn9 = assign87130_e132817_d_n9;
        locals.var_t2_dn10 = assign87130_e132817_d_n10;
        locals.var_t2_dn11 = assign87130_e132817_d_n11;
        locals.var_t2_dn14 = assign87130_e132817_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign87140_e132825, assign87140_e132825_d_n0, assign87140_e132825_d_n2, assign87140_e132825_d_n4, assign87140_e132825_d_n5, assign87140_e132825_d_n6, assign87140_e132825_d_n7, assign87140_e132825_d_n8, assign87140_e132825_d_n9, assign87140_e132825_d_n10, assign87140_e132825_d_n11, assign87140_e132825_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        let assign87140_e132823: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign87140_e132823, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign87140_e132825;
        locals.var_qsuld_dn0 = assign87140_e132825_d_n0;
        locals.var_qsuld_dn2 = assign87140_e132825_d_n2;
        locals.var_qsuld_dn4 = assign87140_e132825_d_n4;
        locals.var_qsuld_dn5 = assign87140_e132825_d_n5;
        locals.var_qsuld_dn6 = assign87140_e132825_d_n6;
        locals.var_qsuld_dn7 = assign87140_e132825_d_n7;
        locals.var_qsuld_dn8 = assign87140_e132825_d_n8;
        locals.var_qsuld_dn9 = assign87140_e132825_d_n9;
        locals.var_qsuld_dn10 = assign87140_e132825_d_n10;
        locals.var_qsuld_dn11 = assign87140_e132825_d_n11;
        locals.var_qsuld_dn14 = assign87140_e132825_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign87150_e132831, assign87150_e132831_d_n0, assign87150_e132831_d_n2, assign87150_e132831_d_n4, assign87150_e132831_d_n5, assign87150_e132831_d_n6, assign87150_e132831_d_n7, assign87150_e132831_d_n8, assign87150_e132831_d_n9, assign87150_e132831_d_n10, assign87150_e132831_d_n11, assign87150_e132831_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign87150_e132831;
        locals.var_qbuld_dn0 = assign87150_e132831_d_n0;
        locals.var_qbuld_dn2 = assign87150_e132831_d_n2;
        locals.var_qbuld_dn4 = assign87150_e132831_d_n4;
        locals.var_qbuld_dn5 = assign87150_e132831_d_n5;
        locals.var_qbuld_dn6 = assign87150_e132831_d_n6;
        locals.var_qbuld_dn7 = assign87150_e132831_d_n7;
        locals.var_qbuld_dn8 = assign87150_e132831_d_n8;
        locals.var_qbuld_dn9 = assign87150_e132831_d_n9;
        locals.var_qbuld_dn10 = assign87150_e132831_d_n10;
        locals.var_qbuld_dn11 = assign87150_e132831_d_n11;
        locals.var_qbuld_dn14 = assign87150_e132831_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign87160_e132837, assign87160_e132837_d_n0, assign87160_e132837_d_n2, assign87160_e132837_d_n4, assign87160_e132837_d_n5, assign87160_e132837_d_n6, assign87160_e132837_d_n7, assign87160_e132837_d_n8, assign87160_e132837_d_n9, assign87160_e132837_d_n10, assign87160_e132837_d_n11, assign87160_e132837_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk2013, locals.var_ps0ld_ini__blk2013_dn0, locals.var_ps0ld_ini__blk2013_dn2, locals.var_ps0ld_ini__blk2013_dn4, locals.var_ps0ld_ini__blk2013_dn5, locals.var_ps0ld_ini__blk2013_dn6, locals.var_ps0ld_ini__blk2013_dn7, locals.var_ps0ld_ini__blk2013_dn8, locals.var_ps0ld_ini__blk2013_dn9, locals.var_ps0ld_ini__blk2013_dn10, locals.var_ps0ld_ini__blk2013_dn11, locals.var_ps0ld_ini__blk2013_dn14,)
    }
};
        locals.var_ps0ld_ini__blk2013 = assign87160_e132837;
        locals.var_ps0ld_ini__blk2013_dn0 = assign87160_e132837_d_n0;
        locals.var_ps0ld_ini__blk2013_dn2 = assign87160_e132837_d_n2;
        locals.var_ps0ld_ini__blk2013_dn4 = assign87160_e132837_d_n4;
        locals.var_ps0ld_ini__blk2013_dn5 = assign87160_e132837_d_n5;
        locals.var_ps0ld_ini__blk2013_dn6 = assign87160_e132837_d_n6;
        locals.var_ps0ld_ini__blk2013_dn7 = assign87160_e132837_d_n7;
        locals.var_ps0ld_ini__blk2013_dn8 = assign87160_e132837_d_n8;
        locals.var_ps0ld_ini__blk2013_dn9 = assign87160_e132837_d_n9;
        locals.var_ps0ld_ini__blk2013_dn10 = assign87160_e132837_d_n10;
        locals.var_ps0ld_ini__blk2013_dn11 = assign87160_e132837_d_n11;
        locals.var_ps0ld_ini__blk2013_dn14 = assign87160_e132837_d_n14;
        locals.var_ps0ld_ini__blk2013_rv = 0.0;

        let assign87170_e132841: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87170_e132842: f64 = (locals.var_beta * assign87170_e132841);
        let assign87170_e132846: f64 = (10.0 * 2.220446049250313e-16);
        let assign87170_e132848: f64 = (assign87170_e132846 - 1.0);
        let assign87170_e132850: f64 = (assign87170_e132848 * locals.var_fac1p2);
        let assign87170_e132852: f64 = (assign87170_e132850 * locals.var_beta2);
        let assign87170_e132854: f64 = (assign87170_e132852 / 4.0);
        let assign87170_e132855: f64 = (1.0 + assign87170_e132854);
        let assign87170_e132856: f64 = if assign87170_e132842 < assign87170_e132855 { 1.0 } else { 0.0 };
        locals.var_guard2032 = assign87170_e132856;
        locals.var_guard2032_rv = 0.0;

        let (assign87180_e132871, assign87180_e132871_d_n0, assign87180_e132871_d_n2, assign87180_e132871_d_n4, assign87180_e132871_d_n5, assign87180_e132871_d_n6, assign87180_e132871_d_n7, assign87180_e132871_d_n8, assign87180_e132871_d_n9, assign87180_e132871_d_n10, assign87180_e132871_d_n11, assign87180_e132871_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87180_e132866: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87180_e132868: f64 = (assign87180_e132866 / 2.0);
        let assign87180_e132869: f64 = (locals.var_vgpld + assign87180_e132868);
        (assign87180_e132869, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign87180_e132871;
        locals.var_ps0_inia_dn0 = assign87180_e132871_d_n0;
        locals.var_ps0_inia_dn2 = assign87180_e132871_d_n2;
        locals.var_ps0_inia_dn4 = assign87180_e132871_d_n4;
        locals.var_ps0_inia_dn5 = assign87180_e132871_d_n5;
        locals.var_ps0_inia_dn6 = assign87180_e132871_d_n6;
        locals.var_ps0_inia_dn7 = assign87180_e132871_d_n7;
        locals.var_ps0_inia_dn8 = assign87180_e132871_d_n8;
        locals.var_ps0_inia_dn9 = assign87180_e132871_d_n9;
        locals.var_ps0_inia_dn10 = assign87180_e132871_d_n10;
        locals.var_ps0_inia_dn11 = assign87180_e132871_d_n11;
        locals.var_ps0_inia_dn14 = assign87180_e132871_d_n14;
        locals.var_ps0_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_334(
        locals: &mut StampLocals,
    ) {
        let (assign87190_e132895, assign87190_e132895_d_n0, assign87190_e132895_d_n2, assign87190_e132895_d_n4, assign87190_e132895_d_n5, assign87190_e132895_d_n6, assign87190_e132895_d_n7, assign87190_e132895_d_n8, assign87190_e132895_d_n9, assign87190_e132895_d_n10, assign87190_e132895_d_n11, assign87190_e132895_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2032 == 0.0)) {
        let assign87190_e132884: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87190_e132885: f64 = (locals.var_beta * assign87190_e132884);
        let assign87190_e132887: f64 = (assign87190_e132885 - 1.0);
        let assign87190_e132888: f64 = (4.0 * assign87190_e132887);
        let assign87190_e132891: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87190_e132892: f64 = (assign87190_e132888 / assign87190_e132891);
        let assign87190_e132893: f64 = (1.0 + assign87190_e132892);
        (assign87190_e132893, ((((4.0 * ((locals.var_beta_dn0 * assign87190_e132884) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn2 * assign87190_e132884) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn4 * assign87190_e132884) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn5 * assign87190_e132884) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn6 * assign87190_e132884) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn7 * assign87190_e132884) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn8 * assign87190_e132884) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn9 * assign87190_e132884) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn10 * assign87190_e132884) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn11 * assign87190_e132884) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign87190_e132891 * assign87190_e132891)), ((((4.0 * ((locals.var_beta_dn14 * assign87190_e132884) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign87190_e132891) - (assign87190_e132888 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign87190_e132891 * assign87190_e132891)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign87190_e132895;
        locals.var_tx_dn0 = assign87190_e132895_d_n0;
        locals.var_tx_dn2 = assign87190_e132895_d_n2;
        locals.var_tx_dn4 = assign87190_e132895_d_n4;
        locals.var_tx_dn5 = assign87190_e132895_d_n5;
        locals.var_tx_dn6 = assign87190_e132895_d_n6;
        locals.var_tx_dn7 = assign87190_e132895_d_n7;
        locals.var_tx_dn8 = assign87190_e132895_d_n8;
        locals.var_tx_dn9 = assign87190_e132895_d_n9;
        locals.var_tx_dn10 = assign87190_e132895_d_n10;
        locals.var_tx_dn11 = assign87190_e132895_d_n11;
        locals.var_tx_dn14 = assign87190_e132895_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign87200_e132916, assign87200_e132916_d_n0, assign87200_e132916_d_n2, assign87200_e132916_d_n4, assign87200_e132916_d_n5, assign87200_e132916_d_n6, assign87200_e132916_d_n7, assign87200_e132916_d_n8, assign87200_e132916_d_n9, assign87200_e132916_d_n10, assign87200_e132916_d_n11, assign87200_e132916_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2032 == 0.0)) {
        let assign87200_e132906: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87200_e132908: f64 = (assign87200_e132906 / 2.0);
        let assign87200_e132911: f64 = (locals.var_tx).sqrt();
        let assign87200_e132912: f64 = (1.0 - assign87200_e132911);
        let assign87200_e132913: f64 = (assign87200_e132908 * assign87200_e132912);
        let assign87200_e132914: f64 = (locals.var_vgpld + assign87200_e132913);
        (assign87200_e132914, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn0 / (2.0 * assign87200_e132911))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn2 / (2.0 * assign87200_e132911)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn4 / (2.0 * assign87200_e132911))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn5 / (2.0 * assign87200_e132911))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn6 / (2.0 * assign87200_e132911))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn7 / (2.0 * assign87200_e132911)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn8 / (2.0 * assign87200_e132911)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn9 / (2.0 * assign87200_e132911)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn10 / (2.0 * assign87200_e132911))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn11 / (2.0 * assign87200_e132911))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign87200_e132912) + (assign87200_e132908 * (-(locals.var_tx_dn14 / (2.0 * assign87200_e132911))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign87200_e132916;
        locals.var_ps0_inia_dn0 = assign87200_e132916_d_n0;
        locals.var_ps0_inia_dn2 = assign87200_e132916_d_n2;
        locals.var_ps0_inia_dn4 = assign87200_e132916_d_n4;
        locals.var_ps0_inia_dn5 = assign87200_e132916_d_n5;
        locals.var_ps0_inia_dn6 = assign87200_e132916_d_n6;
        locals.var_ps0_inia_dn7 = assign87200_e132916_d_n7;
        locals.var_ps0_inia_dn8 = assign87200_e132916_d_n8;
        locals.var_ps0_inia_dn9 = assign87200_e132916_d_n9;
        locals.var_ps0_inia_dn10 = assign87200_e132916_d_n10;
        locals.var_ps0_inia_dn11 = assign87200_e132916_d_n11;
        locals.var_ps0_inia_dn14 = assign87200_e132916_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign87210_e132927, assign87210_e132927_d_n0, assign87210_e132927_d_n2, assign87210_e132927_d_n4, assign87210_e132927_d_n5, assign87210_e132927_d_n6, assign87210_e132927_d_n7, assign87210_e132927_d_n8, assign87210_e132927_d_n9, assign87210_e132927_d_n10, assign87210_e132927_d_n11, assign87210_e132927_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) {
        let assign87210_e132924: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87210_e132925: f64 = (locals.var_beta * assign87210_e132924);
        (assign87210_e132925, ((locals.var_beta_dn0 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign87210_e132924) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign87210_e132927;
        locals.var_chi_dn0 = assign87210_e132927_d_n0;
        locals.var_chi_dn2 = assign87210_e132927_d_n2;
        locals.var_chi_dn4 = assign87210_e132927_d_n4;
        locals.var_chi_dn5 = assign87210_e132927_d_n5;
        locals.var_chi_dn6 = assign87210_e132927_d_n6;
        locals.var_chi_dn7 = assign87210_e132927_d_n7;
        locals.var_chi_dn8 = assign87210_e132927_d_n8;
        locals.var_chi_dn9 = assign87210_e132927_d_n9;
        locals.var_chi_dn10 = assign87210_e132927_d_n10;
        locals.var_chi_dn11 = assign87210_e132927_d_n11;
        locals.var_chi_dn14 = assign87210_e132927_d_n14;
        locals.var_chi_rv = 0.0;

        let assign87220_e132930: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard2033 = assign87220_e132930;
        locals.var_guard2033_rv = 0.0;

        let (assign87240_e132950, assign87240_e132950_d_n0, assign87240_e132950_d_n2, assign87240_e132950_d_n4, assign87240_e132950_d_n5, assign87240_e132950_d_n6, assign87240_e132950_d_n7, assign87240_e132950_d_n8, assign87240_e132950_d_n9, assign87240_e132950_d_n10, assign87240_e132950_d_n11, assign87240_e132950_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87240_e132947: f64 = (-locals.var_chi);
        let assign87240_e132948: f64 = (assign87240_e132947).exp();
        (assign87240_e132948, (assign87240_e132948 * (-locals.var_chi_dn0)), (assign87240_e132948 * (-locals.var_chi_dn2)), (assign87240_e132948 * (-locals.var_chi_dn4)), (assign87240_e132948 * (-locals.var_chi_dn5)), (assign87240_e132948 * (-locals.var_chi_dn6)), (assign87240_e132948 * (-locals.var_chi_dn7)), (assign87240_e132948 * (-locals.var_chi_dn8)), (assign87240_e132948 * (-locals.var_chi_dn9)), (assign87240_e132948 * (-locals.var_chi_dn10)), (assign87240_e132948 * (-locals.var_chi_dn11)), (assign87240_e132948 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign87240_e132950;
        locals.var_ty_dn0 = assign87240_e132950_d_n0;
        locals.var_ty_dn2 = assign87240_e132950_d_n2;
        locals.var_ty_dn4 = assign87240_e132950_d_n4;
        locals.var_ty_dn5 = assign87240_e132950_d_n5;
        locals.var_ty_dn6 = assign87240_e132950_d_n6;
        locals.var_ty_dn7 = assign87240_e132950_d_n7;
        locals.var_ty_dn8 = assign87240_e132950_d_n8;
        locals.var_ty_dn9 = assign87240_e132950_d_n9;
        locals.var_ty_dn10 = assign87240_e132950_d_n10;
        locals.var_ty_dn11 = assign87240_e132950_d_n11;
        locals.var_ty_dn14 = assign87240_e132950_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign87250_e132975, assign87250_e132975_d_n0, assign87250_e132975_d_n2, assign87250_e132975_d_n4, assign87250_e132975_d_n5, assign87250_e132975_d_n6, assign87250_e132975_d_n7, assign87250_e132975_d_n8, assign87250_e132975_d_n9, assign87250_e132975_d_n10, assign87250_e132975_d_n11, assign87250_e132975_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87250_e132962: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87250_e132963: f64 = (locals.var_beta * assign87250_e132962);
        let assign87250_e132965: f64 = (assign87250_e132963 - 1.0);
        let assign87250_e132967: f64 = (assign87250_e132965 + locals.var_ty);
        let assign87250_e132968: f64 = (4.0 * assign87250_e132967);
        let assign87250_e132971: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87250_e132972: f64 = (assign87250_e132968 / assign87250_e132971);
        let assign87250_e132973: f64 = (1.0 + assign87250_e132972);
        (assign87250_e132973, ((((4.0 * (((locals.var_beta_dn0 * assign87250_e132962) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn2 * assign87250_e132962) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn4 * assign87250_e132962) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn5 * assign87250_e132962) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn6 * assign87250_e132962) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn7 * assign87250_e132962) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn8 * assign87250_e132962) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn9 * assign87250_e132962) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn10 * assign87250_e132962) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn11 * assign87250_e132962) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign87250_e132971 * assign87250_e132971)), ((((4.0 * (((locals.var_beta_dn14 * assign87250_e132962) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign87250_e132971) - (assign87250_e132968 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign87250_e132971 * assign87250_e132971)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign87250_e132975;
        locals.var_tx_dn0 = assign87250_e132975_d_n0;
        locals.var_tx_dn2 = assign87250_e132975_d_n2;
        locals.var_tx_dn4 = assign87250_e132975_d_n4;
        locals.var_tx_dn5 = assign87250_e132975_d_n5;
        locals.var_tx_dn6 = assign87250_e132975_d_n6;
        locals.var_tx_dn7 = assign87250_e132975_d_n7;
        locals.var_tx_dn8 = assign87250_e132975_d_n8;
        locals.var_tx_dn9 = assign87250_e132975_d_n9;
        locals.var_tx_dn10 = assign87250_e132975_d_n10;
        locals.var_tx_dn11 = assign87250_e132975_d_n11;
        locals.var_tx_dn14 = assign87250_e132975_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign87260_e132995, assign87260_e132995_d_n0, assign87260_e132995_d_n2, assign87260_e132995_d_n4, assign87260_e132995_d_n5, assign87260_e132995_d_n6, assign87260_e132995_d_n7, assign87260_e132995_d_n8, assign87260_e132995_d_n9, assign87260_e132995_d_n10, assign87260_e132995_d_n11, assign87260_e132995_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87260_e132985: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87260_e132987: f64 = (assign87260_e132985 / 2.0);
        let assign87260_e132990: f64 = (locals.var_tx).sqrt();
        let assign87260_e132991: f64 = (1.0 - assign87260_e132990);
        let assign87260_e132992: f64 = (assign87260_e132987 * assign87260_e132991);
        let assign87260_e132993: f64 = (locals.var_vgpld + assign87260_e132992);
        (assign87260_e132993, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn0 / (2.0 * assign87260_e132990))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn2 / (2.0 * assign87260_e132990)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn4 / (2.0 * assign87260_e132990))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn5 / (2.0 * assign87260_e132990))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn6 / (2.0 * assign87260_e132990))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn7 / (2.0 * assign87260_e132990)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn8 / (2.0 * assign87260_e132990)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn9 / (2.0 * assign87260_e132990)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn10 / (2.0 * assign87260_e132990))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn11 / (2.0 * assign87260_e132990))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign87260_e132991) + (assign87260_e132987 * (-(locals.var_tx_dn14 / (2.0 * assign87260_e132990))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign87260_e132995;
        locals.var_ps0_inia_dn0 = assign87260_e132995_d_n0;
        locals.var_ps0_inia_dn2 = assign87260_e132995_d_n2;
        locals.var_ps0_inia_dn4 = assign87260_e132995_d_n4;
        locals.var_ps0_inia_dn5 = assign87260_e132995_d_n5;
        locals.var_ps0_inia_dn6 = assign87260_e132995_d_n6;
        locals.var_ps0_inia_dn7 = assign87260_e132995_d_n7;
        locals.var_ps0_inia_dn8 = assign87260_e132995_d_n8;
        locals.var_ps0_inia_dn9 = assign87260_e132995_d_n9;
        locals.var_ps0_inia_dn10 = assign87260_e132995_d_n10;
        locals.var_ps0_inia_dn11 = assign87260_e132995_d_n11;
        locals.var_ps0_inia_dn14 = assign87260_e132995_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign87270_e133008, assign87270_e133008_d_n0, assign87270_e133008_d_n2, assign87270_e133008_d_n4, assign87270_e133008_d_n5, assign87270_e133008_d_n6, assign87270_e133008_d_n7, assign87270_e133008_d_n8, assign87270_e133008_d_n9, assign87270_e133008_d_n10, assign87270_e133008_d_n11, assign87270_e133008_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87270_e133005: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87270_e133006: f64 = (locals.var_beta * assign87270_e133005);
        (assign87270_e133006, ((locals.var_beta_dn0 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign87270_e133005) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign87270_e133008;
        locals.var_chi_dn0 = assign87270_e133008_d_n0;
        locals.var_chi_dn2 = assign87270_e133008_d_n2;
        locals.var_chi_dn4 = assign87270_e133008_d_n4;
        locals.var_chi_dn5 = assign87270_e133008_d_n5;
        locals.var_chi_dn6 = assign87270_e133008_d_n6;
        locals.var_chi_dn7 = assign87270_e133008_d_n7;
        locals.var_chi_dn8 = assign87270_e133008_d_n8;
        locals.var_chi_dn9 = assign87270_e133008_d_n9;
        locals.var_chi_dn10 = assign87270_e133008_d_n10;
        locals.var_chi_dn11 = assign87270_e133008_d_n11;
        locals.var_chi_dn14 = assign87270_e133008_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign87280_e133019, assign87280_e133019_d_n0, assign87280_e133019_d_n2, assign87280_e133019_d_n4, assign87280_e133019_d_n5, assign87280_e133019_d_n6, assign87280_e133019_d_n7, assign87280_e133019_d_n8, assign87280_e133019_d_n9, assign87280_e133019_d_n10, assign87280_e133019_d_n11, assign87280_e133019_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87280_e133016: f64 = (-locals.var_chi);
        let assign87280_e133017: f64 = (assign87280_e133016).exp();
        (assign87280_e133017, (assign87280_e133017 * (-locals.var_chi_dn0)), (assign87280_e133017 * (-locals.var_chi_dn2)), (assign87280_e133017 * (-locals.var_chi_dn4)), (assign87280_e133017 * (-locals.var_chi_dn5)), (assign87280_e133017 * (-locals.var_chi_dn6)), (assign87280_e133017 * (-locals.var_chi_dn7)), (assign87280_e133017 * (-locals.var_chi_dn8)), (assign87280_e133017 * (-locals.var_chi_dn9)), (assign87280_e133017 * (-locals.var_chi_dn10)), (assign87280_e133017 * (-locals.var_chi_dn11)), (assign87280_e133017 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign87280_e133019;
        locals.var_ty_dn0 = assign87280_e133019_d_n0;
        locals.var_ty_dn2 = assign87280_e133019_d_n2;
        locals.var_ty_dn4 = assign87280_e133019_d_n4;
        locals.var_ty_dn5 = assign87280_e133019_d_n5;
        locals.var_ty_dn6 = assign87280_e133019_d_n6;
        locals.var_ty_dn7 = assign87280_e133019_d_n7;
        locals.var_ty_dn8 = assign87280_e133019_d_n8;
        locals.var_ty_dn9 = assign87280_e133019_d_n9;
        locals.var_ty_dn10 = assign87280_e133019_d_n10;
        locals.var_ty_dn11 = assign87280_e133019_d_n11;
        locals.var_ty_dn14 = assign87280_e133019_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign87290_e133044, assign87290_e133044_d_n0, assign87290_e133044_d_n2, assign87290_e133044_d_n4, assign87290_e133044_d_n5, assign87290_e133044_d_n6, assign87290_e133044_d_n7, assign87290_e133044_d_n8, assign87290_e133044_d_n9, assign87290_e133044_d_n10, assign87290_e133044_d_n11, assign87290_e133044_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87290_e133031: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87290_e133032: f64 = (locals.var_beta * assign87290_e133031);
        let assign87290_e133034: f64 = (assign87290_e133032 - 1.0);
        let assign87290_e133036: f64 = (assign87290_e133034 + locals.var_ty);
        let assign87290_e133037: f64 = (4.0 * assign87290_e133036);
        let assign87290_e133040: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87290_e133041: f64 = (assign87290_e133037 / assign87290_e133040);
        let assign87290_e133042: f64 = (1.0 + assign87290_e133041);
        (assign87290_e133042, ((((4.0 * (((locals.var_beta_dn0 * assign87290_e133031) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn2 * assign87290_e133031) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn4 * assign87290_e133031) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn5 * assign87290_e133031) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn6 * assign87290_e133031) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn7 * assign87290_e133031) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn8 * assign87290_e133031) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn9 * assign87290_e133031) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn10 * assign87290_e133031) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn11 * assign87290_e133031) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign87290_e133040 * assign87290_e133040)), ((((4.0 * (((locals.var_beta_dn14 * assign87290_e133031) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign87290_e133040) - (assign87290_e133037 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign87290_e133040 * assign87290_e133040)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign87290_e133044;
        locals.var_tx_dn0 = assign87290_e133044_d_n0;
        locals.var_tx_dn2 = assign87290_e133044_d_n2;
        locals.var_tx_dn4 = assign87290_e133044_d_n4;
        locals.var_tx_dn5 = assign87290_e133044_d_n5;
        locals.var_tx_dn6 = assign87290_e133044_d_n6;
        locals.var_tx_dn7 = assign87290_e133044_d_n7;
        locals.var_tx_dn8 = assign87290_e133044_d_n8;
        locals.var_tx_dn9 = assign87290_e133044_d_n9;
        locals.var_tx_dn10 = assign87290_e133044_d_n10;
        locals.var_tx_dn11 = assign87290_e133044_d_n11;
        locals.var_tx_dn14 = assign87290_e133044_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign87300_e133064, assign87300_e133064_d_n0, assign87300_e133064_d_n2, assign87300_e133064_d_n4, assign87300_e133064_d_n5, assign87300_e133064_d_n6, assign87300_e133064_d_n7, assign87300_e133064_d_n8, assign87300_e133064_d_n9, assign87300_e133064_d_n10, assign87300_e133064_d_n11, assign87300_e133064_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87300_e133054: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87300_e133056: f64 = (assign87300_e133054 / 2.0);
        let assign87300_e133059: f64 = (locals.var_tx).sqrt();
        let assign87300_e133060: f64 = (1.0 - assign87300_e133059);
        let assign87300_e133061: f64 = (assign87300_e133056 * assign87300_e133060);
        let assign87300_e133062: f64 = (locals.var_vgpld + assign87300_e133061);
        (assign87300_e133062, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn0 / (2.0 * assign87300_e133059))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn2 / (2.0 * assign87300_e133059)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn4 / (2.0 * assign87300_e133059))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn5 / (2.0 * assign87300_e133059))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn6 / (2.0 * assign87300_e133059))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn7 / (2.0 * assign87300_e133059)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn8 / (2.0 * assign87300_e133059)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn9 / (2.0 * assign87300_e133059)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn10 / (2.0 * assign87300_e133059))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn11 / (2.0 * assign87300_e133059))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign87300_e133060) + (assign87300_e133056 * (-(locals.var_tx_dn14 / (2.0 * assign87300_e133059))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign87300_e133064;
        locals.var_ps0_inia_dn0 = assign87300_e133064_d_n0;
        locals.var_ps0_inia_dn2 = assign87300_e133064_d_n2;
        locals.var_ps0_inia_dn4 = assign87300_e133064_d_n4;
        locals.var_ps0_inia_dn5 = assign87300_e133064_d_n5;
        locals.var_ps0_inia_dn6 = assign87300_e133064_d_n6;
        locals.var_ps0_inia_dn7 = assign87300_e133064_d_n7;
        locals.var_ps0_inia_dn8 = assign87300_e133064_d_n8;
        locals.var_ps0_inia_dn9 = assign87300_e133064_d_n9;
        locals.var_ps0_inia_dn10 = assign87300_e133064_d_n10;
        locals.var_ps0_inia_dn11 = assign87300_e133064_d_n11;
        locals.var_ps0_inia_dn14 = assign87300_e133064_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign87310_e133077, assign87310_e133077_d_n0, assign87310_e133077_d_n2, assign87310_e133077_d_n4, assign87310_e133077_d_n5, assign87310_e133077_d_n6, assign87310_e133077_d_n7, assign87310_e133077_d_n8, assign87310_e133077_d_n9, assign87310_e133077_d_n10, assign87310_e133077_d_n11, assign87310_e133077_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87310_e133074: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87310_e133075: f64 = (locals.var_beta * assign87310_e133074);
        (assign87310_e133075, ((locals.var_beta_dn0 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign87310_e133074) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign87310_e133077;
        locals.var_chi_dn0 = assign87310_e133077_d_n0;
        locals.var_chi_dn2 = assign87310_e133077_d_n2;
        locals.var_chi_dn4 = assign87310_e133077_d_n4;
        locals.var_chi_dn5 = assign87310_e133077_d_n5;
        locals.var_chi_dn6 = assign87310_e133077_d_n6;
        locals.var_chi_dn7 = assign87310_e133077_d_n7;
        locals.var_chi_dn8 = assign87310_e133077_d_n8;
        locals.var_chi_dn9 = assign87310_e133077_d_n9;
        locals.var_chi_dn10 = assign87310_e133077_d_n10;
        locals.var_chi_dn11 = assign87310_e133077_d_n11;
        locals.var_chi_dn14 = assign87310_e133077_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign87330_e133119,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87330_e133098: f64 = (2.0_f64).sqrt();
        let assign87330_e133099: f64 = (9.0 * assign87330_e133098);
        let assign87330_e133100: f64 = (1.0 / assign87330_e133099);
        let assign87330_e133104: f64 = (-3.0);
        let assign87330_e133105: f64 = (assign87330_e133104).exp();
        let assign87330_e133106: f64 = (7.0 * assign87330_e133105);
        let assign87330_e133107: f64 = (5.0 + assign87330_e133106);
        let assign87330_e133111: f64 = (-3.0);
        let assign87330_e133112: f64 = (assign87330_e133111).exp();
        let assign87330_e133113: f64 = (2.0 + assign87330_e133112);
        let assign87330_e133114: f64 = (assign87330_e133113).sqrt();
        let assign87330_e133115: f64 = (54.0 * assign87330_e133114);
        let assign87330_e133116: f64 = (assign87330_e133107 / assign87330_e133115);
        let assign87330_e133117: f64 = (assign87330_e133100 - assign87330_e133116);
        (assign87330_e133117,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign87330_e133119;
        locals.var_ta_rv = 0.0;

        let (assign87340_e133147,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87340_e133129: f64 = (-3.0);
        let assign87340_e133130: f64 = (assign87340_e133129).exp();
        let assign87340_e133131: f64 = (1.0 + assign87340_e133130);
        let assign87340_e133135: f64 = (-3.0);
        let assign87340_e133136: f64 = (assign87340_e133135).exp();
        let assign87340_e133137: f64 = (2.0 + assign87340_e133136);
        let assign87340_e133138: f64 = (assign87340_e133137).sqrt();
        let assign87340_e133139: f64 = (2.0 * assign87340_e133138);
        let assign87340_e133140: f64 = (assign87340_e133131 / assign87340_e133139);
        let assign87340_e133142: f64 = (2.0_f64).sqrt();
        let assign87340_e133144: f64 = (assign87340_e133142 / 3.0);
        let assign87340_e133145: f64 = (assign87340_e133140 - assign87340_e133144);
        (assign87340_e133145,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign87340_e133147;
        locals.var_tb_rv = 0.0;

        let (assign87350_e133166, assign87350_e133166_d_n0, assign87350_e133166_d_n2, assign87350_e133166_d_n4, assign87350_e133166_d_n5, assign87350_e133166_d_n6, assign87350_e133166_d_n7, assign87350_e133166_d_n8, assign87350_e133166_d_n9, assign87350_e133166_d_n10, assign87350_e133166_d_n11, assign87350_e133166_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87350_e133157: f64 = (2.0_f64).sqrt();
        let assign87350_e133158: f64 = (1.0 / assign87350_e133157);
        let assign87350_e133162: f64 = (locals.var_beta * locals.var_fac1);
        let assign87350_e133163: f64 = (1.0 / assign87350_e133162);
        let assign87350_e133164: f64 = (assign87350_e133158 + assign87350_e133163);
        (assign87350_e133164, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign87350_e133162 * assign87350_e133162))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign87350_e133162 * assign87350_e133162))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign87350_e133166;
        locals.var_tc_dn0 = assign87350_e133166_d_n0;
        locals.var_tc_dn2 = assign87350_e133166_d_n2;
        locals.var_tc_dn4 = assign87350_e133166_d_n4;
        locals.var_tc_dn5 = assign87350_e133166_d_n5;
        locals.var_tc_dn6 = assign87350_e133166_d_n6;
        locals.var_tc_dn7 = assign87350_e133166_d_n7;
        locals.var_tc_dn8 = assign87350_e133166_d_n8;
        locals.var_tc_dn9 = assign87350_e133166_d_n9;
        locals.var_tc_dn10 = assign87350_e133166_d_n10;
        locals.var_tc_dn11 = assign87350_e133166_d_n11;
        locals.var_tc_dn14 = assign87350_e133166_d_n14;
        locals.var_tc_rv = 0.0;

        let (assign87360_e133181, assign87360_e133181_d_n0, assign87360_e133181_d_n2, assign87360_e133181_d_n4, assign87360_e133181_d_n5, assign87360_e133181_d_n6, assign87360_e133181_d_n7, assign87360_e133181_d_n8, assign87360_e133181_d_n9, assign87360_e133181_d_n10, assign87360_e133181_d_n11, assign87360_e133181_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87360_e133176: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87360_e133177: f64 = (-assign87360_e133176);
        let assign87360_e133179: f64 = (assign87360_e133177 / locals.var_fac1);
        (assign87360_e133179, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign87360_e133177 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign87360_e133181;
        locals.var_td_dn0 = assign87360_e133181_d_n0;
        locals.var_td_dn2 = assign87360_e133181_d_n2;
        locals.var_td_dn4 = assign87360_e133181_d_n4;
        locals.var_td_dn5 = assign87360_e133181_d_n5;
        locals.var_td_dn6 = assign87360_e133181_d_n6;
        locals.var_td_dn7 = assign87360_e133181_d_n7;
        locals.var_td_dn8 = assign87360_e133181_d_n8;
        locals.var_td_dn9 = assign87360_e133181_d_n9;
        locals.var_td_dn10 = assign87360_e133181_d_n10;
        locals.var_td_dn11 = assign87360_e133181_d_n11;
        locals.var_td_dn14 = assign87360_e133181_d_n14;
        locals.var_td_rv = 0.0;

        let (assign87370_e133219, assign87370_e133219_d_n0, assign87370_e133219_d_n2, assign87370_e133219_d_n4, assign87370_e133219_d_n5, assign87370_e133219_d_n6, assign87370_e133219_d_n7, assign87370_e133219_d_n8, assign87370_e133219_d_n9, assign87370_e133219_d_n10, assign87370_e133219_d_n11, assign87370_e133219_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87370_e133191: f64 = (locals.var_tb * locals.var_tb);
        let assign87370_e133193: f64 = (assign87370_e133191 * locals.var_tb);
        let assign87370_e133196: f64 = (27.0 * locals.var_ta);
        let assign87370_e133198: f64 = (assign87370_e133196 * locals.var_ta);
        let assign87370_e133200: f64 = (assign87370_e133198 * locals.var_ta);
        let assign87370_e133201: f64 = (assign87370_e133193 / assign87370_e133200);
        let assign87370_e133204: f64 = (locals.var_tb * locals.var_tc);
        let assign87370_e133207: f64 = (6.0 * locals.var_ta);
        let assign87370_e133209: f64 = (assign87370_e133207 * locals.var_ta);
        let assign87370_e133210: f64 = (assign87370_e133204 / assign87370_e133209);
        let assign87370_e133211: f64 = (assign87370_e133201 - assign87370_e133210);
        let assign87370_e133215: f64 = (2.0 * locals.var_ta);
        let assign87370_e133216: f64 = (locals.var_td / assign87370_e133215);
        let assign87370_e133217: f64 = (assign87370_e133211 + assign87370_e133216);
        (assign87370_e133217, ((-((locals.var_tb * locals.var_tc_dn0) / assign87370_e133209)) + (locals.var_td_dn0 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn2) / assign87370_e133209)) + (locals.var_td_dn2 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn4) / assign87370_e133209)) + (locals.var_td_dn4 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn5) / assign87370_e133209)) + (locals.var_td_dn5 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn6) / assign87370_e133209)) + (locals.var_td_dn6 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn7) / assign87370_e133209)) + (locals.var_td_dn7 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn8) / assign87370_e133209)) + (locals.var_td_dn8 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn9) / assign87370_e133209)) + (locals.var_td_dn9 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn10) / assign87370_e133209)) + (locals.var_td_dn10 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn11) / assign87370_e133209)) + (locals.var_td_dn11 / assign87370_e133215)), ((-((locals.var_tb * locals.var_tc_dn14) / assign87370_e133209)) + (locals.var_td_dn14 / assign87370_e133215)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign87370_e133219;
        locals.var_tq_dn0 = assign87370_e133219_d_n0;
        locals.var_tq_dn2 = assign87370_e133219_d_n2;
        locals.var_tq_dn4 = assign87370_e133219_d_n4;
        locals.var_tq_dn5 = assign87370_e133219_d_n5;
        locals.var_tq_dn6 = assign87370_e133219_d_n6;
        locals.var_tq_dn7 = assign87370_e133219_d_n7;
        locals.var_tq_dn8 = assign87370_e133219_d_n8;
        locals.var_tq_dn9 = assign87370_e133219_d_n9;
        locals.var_tq_dn10 = assign87370_e133219_d_n10;
        locals.var_tq_dn11 = assign87370_e133219_d_n11;
        locals.var_tq_dn14 = assign87370_e133219_d_n14;
        locals.var_tq_rv = 0.0;

        let (assign87380_e133243, assign87380_e133243_d_n0, assign87380_e133243_d_n2, assign87380_e133243_d_n4, assign87380_e133243_d_n5, assign87380_e133243_d_n6, assign87380_e133243_d_n7, assign87380_e133243_d_n8, assign87380_e133243_d_n9, assign87380_e133243_d_n10, assign87380_e133243_d_n11, assign87380_e133243_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87380_e133229: f64 = (3.0 * locals.var_ta);
        let assign87380_e133231: f64 = (assign87380_e133229 * locals.var_tc);
        let assign87380_e133234: f64 = (locals.var_tb * locals.var_tb);
        let assign87380_e133235: f64 = (assign87380_e133231 - assign87380_e133234);
        let assign87380_e133238: f64 = (9.0 * locals.var_ta);
        let assign87380_e133240: f64 = (assign87380_e133238 * locals.var_ta);
        let assign87380_e133241: f64 = (assign87380_e133235 / assign87380_e133240);
        (assign87380_e133241, ((assign87380_e133229 * locals.var_tc_dn0) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn2) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn4) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn5) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn6) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn7) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn8) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn9) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn10) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn11) / assign87380_e133240), ((assign87380_e133229 * locals.var_tc_dn14) / assign87380_e133240),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign87380_e133243;
        locals.var_tp_dn0 = assign87380_e133243_d_n0;
        locals.var_tp_dn2 = assign87380_e133243_d_n2;
        locals.var_tp_dn4 = assign87380_e133243_d_n4;
        locals.var_tp_dn5 = assign87380_e133243_d_n5;
        locals.var_tp_dn6 = assign87380_e133243_d_n6;
        locals.var_tp_dn7 = assign87380_e133243_d_n7;
        locals.var_tp_dn8 = assign87380_e133243_d_n8;
        locals.var_tp_dn9 = assign87380_e133243_d_n9;
        locals.var_tp_dn10 = assign87380_e133243_d_n10;
        locals.var_tp_dn11 = assign87380_e133243_d_n11;
        locals.var_tp_dn14 = assign87380_e133243_d_n14;
        locals.var_tp_rv = 0.0;

        let (assign87390_e133262, assign87390_e133262_d_n0, assign87390_e133262_d_n2, assign87390_e133262_d_n4, assign87390_e133262_d_n5, assign87390_e133262_d_n6, assign87390_e133262_d_n7, assign87390_e133262_d_n8, assign87390_e133262_d_n9, assign87390_e133262_d_n10, assign87390_e133262_d_n11, assign87390_e133262_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87390_e133253: f64 = (locals.var_tq * locals.var_tq);
        let assign87390_e133256: f64 = (locals.var_tp * locals.var_tp);
        let assign87390_e133258: f64 = (assign87390_e133256 * locals.var_tp);
        let assign87390_e133259: f64 = (assign87390_e133253 + assign87390_e133258);
        let assign87390_e133260: f64 = (assign87390_e133259).sqrt();
        (assign87390_e133260, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn0))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn2))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn4))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn5))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn6))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn7))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn8))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn9))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn10))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn11))) / (2.0 * assign87390_e133260)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign87390_e133256 * locals.var_tp_dn14))) / (2.0 * assign87390_e133260)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign87390_e133262;
        locals.var_t5_dn0 = assign87390_e133262_d_n0;
        locals.var_t5_dn2 = assign87390_e133262_d_n2;
        locals.var_t5_dn4 = assign87390_e133262_d_n4;
        locals.var_t5_dn5 = assign87390_e133262_d_n5;
        locals.var_t5_dn6 = assign87390_e133262_d_n6;
        locals.var_t5_dn7 = assign87390_e133262_d_n7;
        locals.var_t5_dn8 = assign87390_e133262_d_n8;
        locals.var_t5_dn9 = assign87390_e133262_d_n9;
        locals.var_t5_dn10 = assign87390_e133262_d_n10;
        locals.var_t5_dn11 = assign87390_e133262_d_n11;
        locals.var_t5_dn14 = assign87390_e133262_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign87400_e133277, assign87400_e133277_d_n0, assign87400_e133277_d_n2, assign87400_e133277_d_n4, assign87400_e133277_d_n5, assign87400_e133277_d_n6, assign87400_e133277_d_n7, assign87400_e133277_d_n8, assign87400_e133277_d_n9, assign87400_e133277_d_n10, assign87400_e133277_d_n11, assign87400_e133277_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87400_e133271: f64 = (-locals.var_tq);
        let assign87400_e133273: f64 = (assign87400_e133271 + locals.var_t5);
        let assign87400_e133275: f64 = (assign87400_e133273).powf(0.3333333333333333);
        (assign87400_e133275, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign87400_e133273))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87400_e133273).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign87400_e133275 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign87400_e133273))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign87400_e133277;
        locals.var_tu_dn0 = assign87400_e133277_d_n0;
        locals.var_tu_dn2 = assign87400_e133277_d_n2;
        locals.var_tu_dn4 = assign87400_e133277_d_n4;
        locals.var_tu_dn5 = assign87400_e133277_d_n5;
        locals.var_tu_dn6 = assign87400_e133277_d_n6;
        locals.var_tu_dn7 = assign87400_e133277_d_n7;
        locals.var_tu_dn8 = assign87400_e133277_d_n8;
        locals.var_tu_dn9 = assign87400_e133277_d_n9;
        locals.var_tu_dn10 = assign87400_e133277_d_n10;
        locals.var_tu_dn11 = assign87400_e133277_d_n11;
        locals.var_tu_dn14 = assign87400_e133277_d_n14;
        locals.var_tu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_335(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87410_e133292, assign87410_e133292_d_n0, assign87410_e133292_d_n2, assign87410_e133292_d_n4, assign87410_e133292_d_n5, assign87410_e133292_d_n6, assign87410_e133292_d_n7, assign87410_e133292_d_n8, assign87410_e133292_d_n9, assign87410_e133292_d_n10, assign87410_e133292_d_n11, assign87410_e133292_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87410_e133287: f64 = (locals.var_tq + locals.var_t5);
        let assign87410_e133289: f64 = (assign87410_e133287).powf(0.3333333333333333);
        let assign87410_e133290: f64 = (-assign87410_e133289);
        (assign87410_e133290, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign87410_e133287))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87410_e133287).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign87410_e133289 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign87410_e133287))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign87410_e133292;
        locals.var_tv_dn0 = assign87410_e133292_d_n0;
        locals.var_tv_dn2 = assign87410_e133292_d_n2;
        locals.var_tv_dn4 = assign87410_e133292_d_n4;
        locals.var_tv_dn5 = assign87410_e133292_d_n5;
        locals.var_tv_dn6 = assign87410_e133292_d_n6;
        locals.var_tv_dn7 = assign87410_e133292_d_n7;
        locals.var_tv_dn8 = assign87410_e133292_d_n8;
        locals.var_tv_dn9 = assign87410_e133292_d_n9;
        locals.var_tv_dn10 = assign87410_e133292_d_n10;
        locals.var_tv_dn11 = assign87410_e133292_d_n11;
        locals.var_tv_dn14 = assign87410_e133292_d_n14;
        locals.var_tv_rv = 0.0;

        let (assign87420_e133310, assign87420_e133310_d_n0, assign87420_e133310_d_n2, assign87420_e133310_d_n4, assign87420_e133310_d_n5, assign87420_e133310_d_n6, assign87420_e133310_d_n7, assign87420_e133310_d_n8, assign87420_e133310_d_n9, assign87420_e133310_d_n10, assign87420_e133310_d_n11, assign87420_e133310_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87420_e133302: f64 = (locals.var_tu + locals.var_tv);
        let assign87420_e133306: f64 = (3.0 * locals.var_ta);
        let assign87420_e133307: f64 = (locals.var_tb / assign87420_e133306);
        let assign87420_e133308: f64 = (assign87420_e133302 - assign87420_e133307);
        (assign87420_e133308, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn14 + locals.var_tv_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign87420_e133310;
        locals.var_chi_dn0 = assign87420_e133310_d_n0;
        locals.var_chi_dn2 = assign87420_e133310_d_n2;
        locals.var_chi_dn4 = assign87420_e133310_d_n4;
        locals.var_chi_dn5 = assign87420_e133310_d_n5;
        locals.var_chi_dn6 = assign87420_e133310_d_n6;
        locals.var_chi_dn7 = assign87420_e133310_d_n7;
        locals.var_chi_dn8 = assign87420_e133310_d_n8;
        locals.var_chi_dn9 = assign87420_e133310_d_n9;
        locals.var_chi_dn10 = assign87420_e133310_d_n10;
        locals.var_chi_dn11 = assign87420_e133310_d_n11;
        locals.var_chi_dn14 = assign87420_e133310_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign87430_e133324, assign87430_e133324_d_n0, assign87430_e133324_d_n2, assign87430_e133324_d_n4, assign87430_e133324_d_n5, assign87430_e133324_d_n6, assign87430_e133324_d_n7, assign87430_e133324_d_n8, assign87430_e133324_d_n9, assign87430_e133324_d_n10, assign87430_e133324_d_n11, assign87430_e133324_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2033 == 0.0)) {
        let assign87430_e133320: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign87430_e133322: f64 = (assign87430_e133320 - locals.var_vxbgmtcl);
        (assign87430_e133322, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign87430_e133324;
        locals.var_ps0_inia_dn0 = assign87430_e133324_d_n0;
        locals.var_ps0_inia_dn2 = assign87430_e133324_d_n2;
        locals.var_ps0_inia_dn4 = assign87430_e133324_d_n4;
        locals.var_ps0_inia_dn5 = assign87430_e133324_d_n5;
        locals.var_ps0_inia_dn6 = assign87430_e133324_d_n6;
        locals.var_ps0_inia_dn7 = assign87430_e133324_d_n7;
        locals.var_ps0_inia_dn8 = assign87430_e133324_d_n8;
        locals.var_ps0_inia_dn9 = assign87430_e133324_d_n9;
        locals.var_ps0_inia_dn10 = assign87430_e133324_d_n10;
        locals.var_ps0_inia_dn11 = assign87430_e133324_d_n11;
        locals.var_ps0_inia_dn14 = assign87430_e133324_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let assign87440_e133327: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2034 = assign87440_e133327;
        locals.var_guard2034_rv = 0.0;

        let (assign87450_e133340, assign87450_e133340_d_n0, assign87450_e133340_d_n2, assign87450_e133340_d_n4, assign87450_e133340_d_n5, assign87450_e133340_d_n6, assign87450_e133340_d_n7, assign87450_e133340_d_n8, assign87450_e133340_d_n9, assign87450_e133340_d_n10, assign87450_e133340_d_n11, assign87450_e133340_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87450_e133336: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87450_e133338: f64 = (assign87450_e133336 + 0.1);
        (assign87450_e133338, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn14,)
    }
};
        locals.var_vgpld_shift = assign87450_e133340;
        locals.var_vgpld_shift_dn0 = assign87450_e133340_d_n0;
        locals.var_vgpld_shift_dn2 = assign87450_e133340_d_n2;
        locals.var_vgpld_shift_dn4 = assign87450_e133340_d_n4;
        locals.var_vgpld_shift_dn5 = assign87450_e133340_d_n5;
        locals.var_vgpld_shift_dn6 = assign87450_e133340_d_n6;
        locals.var_vgpld_shift_dn7 = assign87450_e133340_d_n7;
        locals.var_vgpld_shift_dn8 = assign87450_e133340_d_n8;
        locals.var_vgpld_shift_dn9 = assign87450_e133340_d_n9;
        locals.var_vgpld_shift_dn10 = assign87450_e133340_d_n10;
        locals.var_vgpld_shift_dn11 = assign87450_e133340_d_n11;
        locals.var_vgpld_shift_dn14 = assign87450_e133340_d_n14;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign87460_e133351, assign87460_e133351_d_n0, assign87460_e133351_d_n2, assign87460_e133351_d_n4, assign87460_e133351_d_n5, assign87460_e133351_d_n6, assign87460_e133351_d_n7, assign87460_e133351_d_n8, assign87460_e133351_d_n9, assign87460_e133351_d_n10, assign87460_e133351_d_n11, assign87460_e133351_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87460_e133349: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign87460_e133349, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign87460_e133351;
        locals.var_cfs1_dn0 = assign87460_e133351_d_n0;
        locals.var_cfs1_dn2 = assign87460_e133351_d_n2;
        locals.var_cfs1_dn4 = assign87460_e133351_d_n4;
        locals.var_cfs1_dn5 = assign87460_e133351_d_n5;
        locals.var_cfs1_dn6 = assign87460_e133351_d_n6;
        locals.var_cfs1_dn7 = assign87460_e133351_d_n7;
        locals.var_cfs1_dn8 = assign87460_e133351_d_n8;
        locals.var_cfs1_dn9 = assign87460_e133351_d_n9;
        locals.var_cfs1_dn10 = assign87460_e133351_d_n10;
        locals.var_cfs1_dn11 = assign87460_e133351_d_n11;
        locals.var_cfs1_dn14 = assign87460_e133351_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign87470_e133362, assign87470_e133362_d_n0, assign87470_e133362_d_n2, assign87470_e133362_d_n4, assign87470_e133362_d_n5, assign87470_e133362_d_n6, assign87470_e133362_d_n7, assign87470_e133362_d_n8, assign87470_e133362_d_n9, assign87470_e133362_d_n10, assign87470_e133362_d_n11, assign87470_e133362_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87470_e133360: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign87470_e133360, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn14,)
    }
};
        locals.var_gammachi = assign87470_e133362;
        locals.var_gammachi_dn0 = assign87470_e133362_d_n0;
        locals.var_gammachi_dn2 = assign87470_e133362_d_n2;
        locals.var_gammachi_dn4 = assign87470_e133362_d_n4;
        locals.var_gammachi_dn5 = assign87470_e133362_d_n5;
        locals.var_gammachi_dn6 = assign87470_e133362_d_n6;
        locals.var_gammachi_dn7 = assign87470_e133362_d_n7;
        locals.var_gammachi_dn8 = assign87470_e133362_d_n8;
        locals.var_gammachi_dn9 = assign87470_e133362_d_n9;
        locals.var_gammachi_dn10 = assign87470_e133362_d_n10;
        locals.var_gammachi_dn11 = assign87470_e133362_d_n11;
        locals.var_gammachi_dn14 = assign87470_e133362_d_n14;
        locals.var_gammachi_rv = 0.0;

        let (assign87480_e133373, assign87480_e133373_d_n0, assign87480_e133373_d_n2, assign87480_e133373_d_n4, assign87480_e133373_d_n5, assign87480_e133373_d_n6, assign87480_e133373_d_n7, assign87480_e133373_d_n8, assign87480_e133373_d_n9, assign87480_e133373_d_n10, assign87480_e133373_d_n11, assign87480_e133373_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87480_e133371: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign87480_e133371, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn11 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn11)), ((locals.var_beta2_dn14 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign87480_e133373;
        locals.var_t0_dn0 = assign87480_e133373_d_n0;
        locals.var_t0_dn2 = assign87480_e133373_d_n2;
        locals.var_t0_dn4 = assign87480_e133373_d_n4;
        locals.var_t0_dn5 = assign87480_e133373_d_n5;
        locals.var_t0_dn6 = assign87480_e133373_d_n6;
        locals.var_t0_dn7 = assign87480_e133373_d_n7;
        locals.var_t0_dn8 = assign87480_e133373_d_n8;
        locals.var_t0_dn9 = assign87480_e133373_d_n9;
        locals.var_t0_dn10 = assign87480_e133373_d_n10;
        locals.var_t0_dn11 = assign87480_e133373_d_n11;
        locals.var_t0_dn14 = assign87480_e133373_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign87490_e133384, assign87490_e133384_d_n0, assign87490_e133384_d_n2, assign87490_e133384_d_n4, assign87490_e133384_d_n5, assign87490_e133384_d_n6, assign87490_e133384_d_n7, assign87490_e133384_d_n8, assign87490_e133384_d_n9, assign87490_e133384_d_n10, assign87490_e133384_d_n11, assign87490_e133384_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87490_e133382: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign87490_e133382, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn11 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn11)), ((locals.var_beta_dn14 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn14)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign87490_e133384;
        locals.var_psi_dn0 = assign87490_e133384_d_n0;
        locals.var_psi_dn2 = assign87490_e133384_d_n2;
        locals.var_psi_dn4 = assign87490_e133384_d_n4;
        locals.var_psi_dn5 = assign87490_e133384_d_n5;
        locals.var_psi_dn6 = assign87490_e133384_d_n6;
        locals.var_psi_dn7 = assign87490_e133384_d_n7;
        locals.var_psi_dn8 = assign87490_e133384_d_n8;
        locals.var_psi_dn9 = assign87490_e133384_d_n9;
        locals.var_psi_dn10 = assign87490_e133384_d_n10;
        locals.var_psi_dn11 = assign87490_e133384_d_n11;
        locals.var_psi_dn14 = assign87490_e133384_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign87500_e133409, assign87500_e133409_d_n0, assign87500_e133409_d_n2, assign87500_e133409_d_n4, assign87500_e133409_d_n5, assign87500_e133409_d_n6, assign87500_e133409_d_n7, assign87500_e133409_d_n8, assign87500_e133409_d_n9, assign87500_e133409_d_n10, assign87500_e133409_d_n11, assign87500_e133409_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87500_e133393: f64 = (locals.var_gammachi * locals.var_t0);
        let assign87500_e133396: f64 = (locals.var_psi * locals.var_psi);
        let assign87500_e133397: f64 = (assign87500_e133393 + assign87500_e133396);
        let assign87500_e133398: f64 = (assign87500_e133397).ln();
        let assign87500_e133401: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign87500_e133402: f64 = (assign87500_e133401).ln();
        let assign87500_e133403: f64 = (assign87500_e133398 - assign87500_e133402);
        let assign87500_e133406: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign87500_e133407: f64 = (assign87500_e133403 + assign87500_e133406);
        (assign87500_e133407, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign87500_e133397) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign87500_e133401)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign87500_e133397) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign87500_e133401)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign87500_e133397) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign87500_e133401)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign87500_e133397) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign87500_e133401)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign87500_e133397) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign87500_e133401)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign87500_e133397) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign87500_e133401)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign87500_e133397) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign87500_e133401)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign87500_e133397) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign87500_e133401)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign87500_e133397) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign87500_e133401)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign87500_e133397) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign87500_e133401)) + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), ((((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign87500_e133397) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign87500_e133401)) + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign87500_e133409;
        locals.var_chi_1_dn0 = assign87500_e133409_d_n0;
        locals.var_chi_1_dn2 = assign87500_e133409_d_n2;
        locals.var_chi_1_dn4 = assign87500_e133409_d_n4;
        locals.var_chi_1_dn5 = assign87500_e133409_d_n5;
        locals.var_chi_1_dn6 = assign87500_e133409_d_n6;
        locals.var_chi_1_dn7 = assign87500_e133409_d_n7;
        locals.var_chi_1_dn8 = assign87500_e133409_d_n8;
        locals.var_chi_1_dn9 = assign87500_e133409_d_n9;
        locals.var_chi_1_dn10 = assign87500_e133409_d_n10;
        locals.var_chi_1_dn11 = assign87500_e133409_d_n11;
        locals.var_chi_1_dn14 = assign87500_e133409_d_n14;
        locals.var_chi_1_rv = 0.0;

        let assign87510_e133412: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2035 = assign87510_e133412;
        locals.var_guard2035_rv = 0.0;

        let (assign87520_e133427, assign87520_e133427_d_n0, assign87520_e133427_d_n2, assign87520_e133427_d_n4, assign87520_e133427_d_n5, assign87520_e133427_d_n6, assign87520_e133427_d_n7, assign87520_e133427_d_n8, assign87520_e133427_d_n9, assign87520_e133427_d_n10, assign87520_e133427_d_n11, assign87520_e133427_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87520_e133423: f64 = (locals.var_psi - locals.var_chi_1);
        let assign87520_e133425: f64 = (assign87520_e133423 - 1.0);
        (assign87520_e133425, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign87520_e133427;
        locals.var_tmf1_dn0 = assign87520_e133427_d_n0;
        locals.var_tmf1_dn2 = assign87520_e133427_d_n2;
        locals.var_tmf1_dn4 = assign87520_e133427_d_n4;
        locals.var_tmf1_dn5 = assign87520_e133427_d_n5;
        locals.var_tmf1_dn6 = assign87520_e133427_d_n6;
        locals.var_tmf1_dn7 = assign87520_e133427_d_n7;
        locals.var_tmf1_dn8 = assign87520_e133427_d_n8;
        locals.var_tmf1_dn9 = assign87520_e133427_d_n9;
        locals.var_tmf1_dn10 = assign87520_e133427_d_n10;
        locals.var_tmf1_dn11 = assign87520_e133427_d_n11;
        locals.var_tmf1_dn14 = assign87520_e133427_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign87530_e133442, assign87530_e133442_d_n0, assign87530_e133442_d_n2, assign87530_e133442_d_n4, assign87530_e133442_d_n5, assign87530_e133442_d_n6, assign87530_e133442_d_n7, assign87530_e133442_d_n8, assign87530_e133442_d_n9, assign87530_e133442_d_n10, assign87530_e133442_d_n11, assign87530_e133442_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87530_e133438: f64 = (4.0 * locals.var_psi);
        let assign87530_e133440: f64 = assign87530_e133438;
        (assign87530_e133440, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn14),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign87530_e133442;
        locals.var_tmf2_dn0 = assign87530_e133442_d_n0;
        locals.var_tmf2_dn2 = assign87530_e133442_d_n2;
        locals.var_tmf2_dn4 = assign87530_e133442_d_n4;
        locals.var_tmf2_dn5 = assign87530_e133442_d_n5;
        locals.var_tmf2_dn6 = assign87530_e133442_d_n6;
        locals.var_tmf2_dn7 = assign87530_e133442_d_n7;
        locals.var_tmf2_dn8 = assign87530_e133442_d_n8;
        locals.var_tmf2_dn9 = assign87530_e133442_d_n9;
        locals.var_tmf2_dn10 = assign87530_e133442_d_n10;
        locals.var_tmf2_dn11 = assign87530_e133442_d_n11;
        locals.var_tmf2_dn14 = assign87530_e133442_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign87540_e133459, assign87540_e133459_d_n0, assign87540_e133459_d_n2, assign87540_e133459_d_n4, assign87540_e133459_d_n5, assign87540_e133459_d_n6, assign87540_e133459_d_n7, assign87540_e133459_d_n8, assign87540_e133459_d_n9, assign87540_e133459_d_n10, assign87540_e133459_d_n11, assign87540_e133459_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let (assign87540_e133457, assign87540_e133457_d_n0, assign87540_e133457_d_n2, assign87540_e133457_d_n4, assign87540_e133457_d_n5, assign87540_e133457_d_n6, assign87540_e133457_d_n7, assign87540_e133457_d_n8, assign87540_e133457_d_n9, assign87540_e133457_d_n10, assign87540_e133457_d_n11, assign87540_e133457_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign87540_e133456: f64 = (-locals.var_tmf2);
                (assign87540_e133456, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign87540_e133457, assign87540_e133457_d_n0, assign87540_e133457_d_n2, assign87540_e133457_d_n4, assign87540_e133457_d_n5, assign87540_e133457_d_n6, assign87540_e133457_d_n7, assign87540_e133457_d_n8, assign87540_e133457_d_n9, assign87540_e133457_d_n10, assign87540_e133457_d_n11, assign87540_e133457_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign87540_e133459;
        locals.var_tmf2_dn0 = assign87540_e133459_d_n0;
        locals.var_tmf2_dn2 = assign87540_e133459_d_n2;
        locals.var_tmf2_dn4 = assign87540_e133459_d_n4;
        locals.var_tmf2_dn5 = assign87540_e133459_d_n5;
        locals.var_tmf2_dn6 = assign87540_e133459_d_n6;
        locals.var_tmf2_dn7 = assign87540_e133459_d_n7;
        locals.var_tmf2_dn8 = assign87540_e133459_d_n8;
        locals.var_tmf2_dn9 = assign87540_e133459_d_n9;
        locals.var_tmf2_dn10 = assign87540_e133459_d_n10;
        locals.var_tmf2_dn11 = assign87540_e133459_d_n11;
        locals.var_tmf2_dn14 = assign87540_e133459_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign87550_e133475, assign87550_e133475_d_n0, assign87550_e133475_d_n2, assign87550_e133475_d_n4, assign87550_e133475_d_n5, assign87550_e133475_d_n6, assign87550_e133475_d_n7, assign87550_e133475_d_n8, assign87550_e133475_d_n9, assign87550_e133475_d_n10, assign87550_e133475_d_n11, assign87550_e133475_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87550_e133470: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign87550_e133472: f64 = (assign87550_e133470 + locals.var_tmf2);
        let assign87550_e133473: f64 = (assign87550_e133472).sqrt();
        (assign87550_e133473, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign87550_e133473)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign87550_e133473)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign87550_e133475;
        locals.var_tmf2_dn0 = assign87550_e133475_d_n0;
        locals.var_tmf2_dn2 = assign87550_e133475_d_n2;
        locals.var_tmf2_dn4 = assign87550_e133475_d_n4;
        locals.var_tmf2_dn5 = assign87550_e133475_d_n5;
        locals.var_tmf2_dn6 = assign87550_e133475_d_n6;
        locals.var_tmf2_dn7 = assign87550_e133475_d_n7;
        locals.var_tmf2_dn8 = assign87550_e133475_d_n8;
        locals.var_tmf2_dn9 = assign87550_e133475_d_n9;
        locals.var_tmf2_dn10 = assign87550_e133475_d_n10;
        locals.var_tmf2_dn11 = assign87550_e133475_d_n11;
        locals.var_tmf2_dn14 = assign87550_e133475_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign87560_e133492, assign87560_e133492_d_n0, assign87560_e133492_d_n2, assign87560_e133492_d_n4, assign87560_e133492_d_n5, assign87560_e133492_d_n6, assign87560_e133492_d_n7, assign87560_e133492_d_n8, assign87560_e133492_d_n9, assign87560_e133492_d_n10, assign87560_e133492_d_n11, assign87560_e133492_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87560_e133488: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign87560_e133489: f64 = (1.0 + assign87560_e133488);
        let assign87560_e133490: f64 = (0.5 * assign87560_e133489);
        (assign87560_e133490, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign87560_e133492;
        locals.var_t1_dn0 = assign87560_e133492_d_n0;
        locals.var_t1_dn2 = assign87560_e133492_d_n2;
        locals.var_t1_dn4 = assign87560_e133492_d_n4;
        locals.var_t1_dn5 = assign87560_e133492_d_n5;
        locals.var_t1_dn6 = assign87560_e133492_d_n6;
        locals.var_t1_dn7 = assign87560_e133492_d_n7;
        locals.var_t1_dn8 = assign87560_e133492_d_n8;
        locals.var_t1_dn9 = assign87560_e133492_d_n9;
        locals.var_t1_dn10 = assign87560_e133492_d_n10;
        locals.var_t1_dn11 = assign87560_e133492_d_n11;
        locals.var_t1_dn14 = assign87560_e133492_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign87570_e133509, assign87570_e133509_d_n0, assign87570_e133509_d_n2, assign87570_e133509_d_n4, assign87570_e133509_d_n5, assign87570_e133509_d_n6, assign87570_e133509_d_n7, assign87570_e133509_d_n8, assign87570_e133509_d_n9, assign87570_e133509_d_n10, assign87570_e133509_d_n11, assign87570_e133509_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87570_e133505: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign87570_e133506: f64 = (0.5 * assign87570_e133505);
        let assign87570_e133507: f64 = (locals.var_psi - assign87570_e133506);
        (assign87570_e133507, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign87570_e133509;
        locals.var_chi_1_dn0 = assign87570_e133509_d_n0;
        locals.var_chi_1_dn2 = assign87570_e133509_d_n2;
        locals.var_chi_1_dn4 = assign87570_e133509_d_n4;
        locals.var_chi_1_dn5 = assign87570_e133509_d_n5;
        locals.var_chi_1_dn6 = assign87570_e133509_d_n6;
        locals.var_chi_1_dn7 = assign87570_e133509_d_n7;
        locals.var_chi_1_dn8 = assign87570_e133509_d_n8;
        locals.var_chi_1_dn9 = assign87570_e133509_d_n9;
        locals.var_chi_1_dn10 = assign87570_e133509_d_n10;
        locals.var_chi_1_dn11 = assign87570_e133509_d_n11;
        locals.var_chi_1_dn14 = assign87570_e133509_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign87580_e133526, assign87580_e133526_d_n0, assign87580_e133526_d_n2, assign87580_e133526_d_n4, assign87580_e133526_d_n5, assign87580_e133526_d_n6, assign87580_e133526_d_n7, assign87580_e133526_d_n8, assign87580_e133526_d_n9, assign87580_e133526_d_n10, assign87580_e133526_d_n11, assign87580_e133526_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 == 0.0)) {
        let (assign87580_e133524, assign87580_e133524_d_n0, assign87580_e133524_d_n2, assign87580_e133524_d_n4, assign87580_e133524_d_n5, assign87580_e133524_d_n6, assign87580_e133524_d_n7, assign87580_e133524_d_n8, assign87580_e133524_d_n9, assign87580_e133524_d_n10, assign87580_e133524_d_n11, assign87580_e133524_d_n14,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
            }
        };
        (assign87580_e133524, assign87580_e133524_d_n0, assign87580_e133524_d_n2, assign87580_e133524_d_n4, assign87580_e133524_d_n5, assign87580_e133524_d_n6, assign87580_e133524_d_n7, assign87580_e133524_d_n8, assign87580_e133524_d_n9, assign87580_e133524_d_n10, assign87580_e133524_d_n11, assign87580_e133524_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign87580_e133526;
        locals.var_chi_1_dn0 = assign87580_e133526_d_n0;
        locals.var_chi_1_dn2 = assign87580_e133526_d_n2;
        locals.var_chi_1_dn4 = assign87580_e133526_d_n4;
        locals.var_chi_1_dn5 = assign87580_e133526_d_n5;
        locals.var_chi_1_dn6 = assign87580_e133526_d_n6;
        locals.var_chi_1_dn7 = assign87580_e133526_d_n7;
        locals.var_chi_1_dn8 = assign87580_e133526_d_n8;
        locals.var_chi_1_dn9 = assign87580_e133526_d_n9;
        locals.var_chi_1_dn10 = assign87580_e133526_d_n10;
        locals.var_chi_1_dn11 = assign87580_e133526_d_n11;
        locals.var_chi_1_dn14 = assign87580_e133526_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign87590_e133540, assign87590_e133540_d_n0, assign87590_e133540_d_n2, assign87590_e133540_d_n4, assign87590_e133540_d_n5, assign87590_e133540_d_n6, assign87590_e133540_d_n7, assign87590_e133540_d_n8, assign87590_e133540_d_n9, assign87590_e133540_d_n10, assign87590_e133540_d_n11, assign87590_e133540_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let (assign87590_e133538, assign87590_e133538_d_n0, assign87590_e133538_d_n2, assign87590_e133538_d_n4, assign87590_e133538_d_n5, assign87590_e133538_d_n6, assign87590_e133538_d_n7, assign87590_e133538_d_n8, assign87590_e133538_d_n9, assign87590_e133538_d_n10, assign87590_e133538_d_n11, assign87590_e133538_d_n14,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign87590_e133538, assign87590_e133538_d_n0, assign87590_e133538_d_n2, assign87590_e133538_d_n4, assign87590_e133538_d_n5, assign87590_e133538_d_n6, assign87590_e133538_d_n7, assign87590_e133538_d_n8, assign87590_e133538_d_n9, assign87590_e133538_d_n10, assign87590_e133538_d_n11, assign87590_e133538_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign87590_e133540;
        locals.var_chi_1_dn0 = assign87590_e133540_d_n0;
        locals.var_chi_1_dn2 = assign87590_e133540_d_n2;
        locals.var_chi_1_dn4 = assign87590_e133540_d_n4;
        locals.var_chi_1_dn5 = assign87590_e133540_d_n5;
        locals.var_chi_1_dn6 = assign87590_e133540_d_n6;
        locals.var_chi_1_dn7 = assign87590_e133540_d_n7;
        locals.var_chi_1_dn8 = assign87590_e133540_d_n8;
        locals.var_chi_1_dn9 = assign87590_e133540_d_n9;
        locals.var_chi_1_dn10 = assign87590_e133540_d_n10;
        locals.var_chi_1_dn11 = assign87590_e133540_d_n11;
        locals.var_chi_1_dn14 = assign87590_e133540_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign87600_e133551, assign87600_e133551_d_n0, assign87600_e133551_d_n2, assign87600_e133551_d_n4, assign87600_e133551_d_n5, assign87600_e133551_d_n6, assign87600_e133551_d_n7, assign87600_e133551_d_n8, assign87600_e133551_d_n9, assign87600_e133551_d_n10, assign87600_e133551_d_n11, assign87600_e133551_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87600_e133549: f64 = (locals.var_psi - locals.var_chi_1);
        (assign87600_e133549, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign87600_e133551;
        locals.var_psi_dn0 = assign87600_e133551_d_n0;
        locals.var_psi_dn2 = assign87600_e133551_d_n2;
        locals.var_psi_dn4 = assign87600_e133551_d_n4;
        locals.var_psi_dn5 = assign87600_e133551_d_n5;
        locals.var_psi_dn6 = assign87600_e133551_d_n6;
        locals.var_psi_dn7 = assign87600_e133551_d_n7;
        locals.var_psi_dn8 = assign87600_e133551_d_n8;
        locals.var_psi_dn9 = assign87600_e133551_d_n9;
        locals.var_psi_dn10 = assign87600_e133551_d_n10;
        locals.var_psi_dn11 = assign87600_e133551_d_n11;
        locals.var_psi_dn14 = assign87600_e133551_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign87610_e133564, assign87610_e133564_d_n0, assign87610_e133564_d_n2, assign87610_e133564_d_n4, assign87610_e133564_d_n5, assign87610_e133564_d_n6, assign87610_e133564_d_n7, assign87610_e133564_d_n8, assign87610_e133564_d_n9, assign87610_e133564_d_n10, assign87610_e133564_d_n11, assign87610_e133564_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87610_e133561: f64 = (locals.var_beta * 0.1);
        let assign87610_e133562: f64 = (locals.var_psi + assign87610_e133561);
        (assign87610_e133562, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn11 + (locals.var_beta_dn11 * 0.1)), (locals.var_psi_dn14 + (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign87610_e133564;
        locals.var_psi_dn0 = assign87610_e133564_d_n0;
        locals.var_psi_dn2 = assign87610_e133564_d_n2;
        locals.var_psi_dn4 = assign87610_e133564_d_n4;
        locals.var_psi_dn5 = assign87610_e133564_d_n5;
        locals.var_psi_dn6 = assign87610_e133564_d_n6;
        locals.var_psi_dn7 = assign87610_e133564_d_n7;
        locals.var_psi_dn8 = assign87610_e133564_d_n8;
        locals.var_psi_dn9 = assign87610_e133564_d_n9;
        locals.var_psi_dn10 = assign87610_e133564_d_n10;
        locals.var_psi_dn11 = assign87610_e133564_d_n11;
        locals.var_psi_dn14 = assign87610_e133564_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign87620_e133585, assign87620_e133585_d_n0, assign87620_e133585_d_n2, assign87620_e133585_d_n4, assign87620_e133585_d_n5, assign87620_e133585_d_n6, assign87620_e133585_d_n7, assign87620_e133585_d_n8, assign87620_e133585_d_n9, assign87620_e133585_d_n10, assign87620_e133585_d_n11, assign87620_e133585_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87620_e133573: f64 = (locals.var_gammachi * locals.var_t0);
        let assign87620_e133576: f64 = (locals.var_psi * locals.var_psi);
        let assign87620_e133577: f64 = (assign87620_e133573 + assign87620_e133576);
        let assign87620_e133578: f64 = (assign87620_e133577).ln();
        let assign87620_e133581: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign87620_e133582: f64 = (assign87620_e133581).ln();
        let assign87620_e133583: f64 = (assign87620_e133578 - assign87620_e133582);
        (assign87620_e133583, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign87620_e133577) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign87620_e133581)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign87620_e133577) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign87620_e133581)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign87620_e133577) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign87620_e133581)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign87620_e133577) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign87620_e133581)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign87620_e133577) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign87620_e133581)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign87620_e133577) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign87620_e133581)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign87620_e133577) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign87620_e133581)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign87620_e133577) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign87620_e133581)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign87620_e133577) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign87620_e133581)), (((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign87620_e133577) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign87620_e133581)), (((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign87620_e133577) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign87620_e133581)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign87620_e133585;
        locals.var_t1_dn0 = assign87620_e133585_d_n0;
        locals.var_t1_dn2 = assign87620_e133585_d_n2;
        locals.var_t1_dn4 = assign87620_e133585_d_n4;
        locals.var_t1_dn5 = assign87620_e133585_d_n5;
        locals.var_t1_dn6 = assign87620_e133585_d_n6;
        locals.var_t1_dn7 = assign87620_e133585_d_n7;
        locals.var_t1_dn8 = assign87620_e133585_d_n8;
        locals.var_t1_dn9 = assign87620_e133585_d_n9;
        locals.var_t1_dn10 = assign87620_e133585_d_n10;
        locals.var_t1_dn11 = assign87620_e133585_d_n11;
        locals.var_t1_dn14 = assign87620_e133585_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_336(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87630_e133598, assign87630_e133598_d_n0, assign87630_e133598_d_n2, assign87630_e133598_d_n4, assign87630_e133598_d_n5, assign87630_e133598_d_n6, assign87630_e133598_d_n7, assign87630_e133598_d_n8, assign87630_e133598_d_n9, assign87630_e133598_d_n10, assign87630_e133598_d_n11, assign87630_e133598_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let assign87630_e133595: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign87630_e133596: f64 = (locals.var_t1 + assign87630_e133595);
        (assign87630_e133596, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn11 + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), (locals.var_t1_dn14 + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign87630_e133598;
        locals.var_chi_b_dn0 = assign87630_e133598_d_n0;
        locals.var_chi_b_dn2 = assign87630_e133598_d_n2;
        locals.var_chi_b_dn4 = assign87630_e133598_d_n4;
        locals.var_chi_b_dn5 = assign87630_e133598_d_n5;
        locals.var_chi_b_dn6 = assign87630_e133598_d_n6;
        locals.var_chi_b_dn7 = assign87630_e133598_d_n7;
        locals.var_chi_b_dn8 = assign87630_e133598_d_n8;
        locals.var_chi_b_dn9 = assign87630_e133598_d_n9;
        locals.var_chi_b_dn10 = assign87630_e133598_d_n10;
        locals.var_chi_b_dn11 = assign87630_e133598_d_n11;
        locals.var_chi_b_dn14 = assign87630_e133598_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign87640_e133612, assign87640_e133612_d_n0, assign87640_e133612_d_n2, assign87640_e133612_d_n4, assign87640_e133612_d_n5, assign87640_e133612_d_n6, assign87640_e133612_d_n7, assign87640_e133612_d_n8, assign87640_e133612_d_n9, assign87640_e133612_d_n10, assign87640_e133612_d_n11, assign87640_e133612_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        let (assign87640_e133610, assign87640_e133610_d_n0, assign87640_e133610_d_n2, assign87640_e133610_d_n4, assign87640_e133610_d_n5, assign87640_e133610_d_n6, assign87640_e133610_d_n7, assign87640_e133610_d_n8, assign87640_e133610_d_n9, assign87640_e133610_d_n10, assign87640_e133610_d_n11, assign87640_e133610_d_n14,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign87640_e133610, assign87640_e133610_d_n0, assign87640_e133610_d_n2, assign87640_e133610_d_n4, assign87640_e133610_d_n5, assign87640_e133610_d_n6, assign87640_e133610_d_n7, assign87640_e133610_d_n8, assign87640_e133610_d_n9, assign87640_e133610_d_n10, assign87640_e133610_d_n11, assign87640_e133610_d_n14,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign87640_e133612;
        locals.var_chi_b_dn0 = assign87640_e133612_d_n0;
        locals.var_chi_b_dn2 = assign87640_e133612_d_n2;
        locals.var_chi_b_dn4 = assign87640_e133612_d_n4;
        locals.var_chi_b_dn5 = assign87640_e133612_d_n5;
        locals.var_chi_b_dn6 = assign87640_e133612_d_n6;
        locals.var_chi_b_dn7 = assign87640_e133612_d_n7;
        locals.var_chi_b_dn8 = assign87640_e133612_d_n8;
        locals.var_chi_b_dn9 = assign87640_e133612_d_n9;
        locals.var_chi_b_dn10 = assign87640_e133612_d_n10;
        locals.var_chi_b_dn11 = assign87640_e133612_d_n11;
        locals.var_chi_b_dn14 = assign87640_e133612_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign87650_e133621, assign87650_e133621_d_n0, assign87650_e133621_d_n2, assign87650_e133621_d_n4, assign87650_e133621_d_n5, assign87650_e133621_d_n6, assign87650_e133621_d_n7, assign87650_e133621_d_n8, assign87650_e133621_d_n9, assign87650_e133621_d_n10, assign87650_e133621_d_n11, assign87650_e133621_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign87650_e133621;
        locals.var_chi_a_dn0 = assign87650_e133621_d_n0;
        locals.var_chi_a_dn2 = assign87650_e133621_d_n2;
        locals.var_chi_a_dn4 = assign87650_e133621_d_n4;
        locals.var_chi_a_dn5 = assign87650_e133621_d_n5;
        locals.var_chi_a_dn6 = assign87650_e133621_d_n6;
        locals.var_chi_a_dn7 = assign87650_e133621_d_n7;
        locals.var_chi_a_dn8 = assign87650_e133621_d_n8;
        locals.var_chi_a_dn9 = assign87650_e133621_d_n9;
        locals.var_chi_a_dn10 = assign87650_e133621_d_n10;
        locals.var_chi_a_dn11 = assign87650_e133621_d_n11;
        locals.var_chi_a_dn14 = assign87650_e133621_d_n14;
        locals.var_chi_a_rv = 0.0;

        let assign87660_e133624: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2036 = assign87660_e133624;
        locals.var_guard2036_rv = 0.0;

        let assign87670_e133629: f64 = (0.2 * locals.var_chi_b);
        let assign87670_e133630: f64 = (locals.var_chi_b - assign87670_e133629);
        let assign87670_e133634: f64 = (0.2 * locals.var_chi_b);
        let assign87670_e133637: f64 = if ((locals.var_chi_a > assign87670_e133630) && (assign87670_e133634 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2037 = assign87670_e133637;
        locals.var_guard2037_rv = 0.0;

        let (assign87680_e133656, assign87680_e133656_d_n0, assign87680_e133656_d_n2, assign87680_e133656_d_n4, assign87680_e133656_d_n5, assign87680_e133656_d_n6, assign87680_e133656_d_n7, assign87680_e133656_d_n8, assign87680_e133656_d_n9, assign87680_e133656_d_n10, assign87680_e133656_d_n11, assign87680_e133656_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87680_e133650: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign87680_e133653: f64 = (0.2 * locals.var_chi_b);
        let assign87680_e133654: f64 = (assign87680_e133650 + assign87680_e133653);
        (assign87680_e133654, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn11 - locals.var_chi_b_dn11) + (0.2 * locals.var_chi_b_dn11)), ((locals.var_chi_a_dn14 - locals.var_chi_b_dn14) + (0.2 * locals.var_chi_b_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign87680_e133656;
        locals.var_tmf1_dn0 = assign87680_e133656_d_n0;
        locals.var_tmf1_dn2 = assign87680_e133656_d_n2;
        locals.var_tmf1_dn4 = assign87680_e133656_d_n4;
        locals.var_tmf1_dn5 = assign87680_e133656_d_n5;
        locals.var_tmf1_dn6 = assign87680_e133656_d_n6;
        locals.var_tmf1_dn7 = assign87680_e133656_d_n7;
        locals.var_tmf1_dn8 = assign87680_e133656_d_n8;
        locals.var_tmf1_dn9 = assign87680_e133656_d_n9;
        locals.var_tmf1_dn10 = assign87680_e133656_d_n10;
        locals.var_tmf1_dn11 = assign87680_e133656_d_n11;
        locals.var_tmf1_dn14 = assign87680_e133656_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign87690_e133671, assign87690_e133671_d_n0, assign87690_e133671_d_n2, assign87690_e133671_d_n4, assign87690_e133671_d_n5, assign87690_e133671_d_n6, assign87690_e133671_d_n7, assign87690_e133671_d_n8, assign87690_e133671_d_n9, assign87690_e133671_d_n10, assign87690_e133671_d_n11, assign87690_e133671_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87690_e133669: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign87690_e133669, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign87690_e133671;
        locals.var_x2_dn0 = assign87690_e133671_d_n0;
        locals.var_x2_dn2 = assign87690_e133671_d_n2;
        locals.var_x2_dn4 = assign87690_e133671_d_n4;
        locals.var_x2_dn5 = assign87690_e133671_d_n5;
        locals.var_x2_dn6 = assign87690_e133671_d_n6;
        locals.var_x2_dn7 = assign87690_e133671_d_n7;
        locals.var_x2_dn8 = assign87690_e133671_d_n8;
        locals.var_x2_dn9 = assign87690_e133671_d_n9;
        locals.var_x2_dn10 = assign87690_e133671_d_n10;
        locals.var_x2_dn11 = assign87690_e133671_d_n11;
        locals.var_x2_dn14 = assign87690_e133671_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign87700_e133690, assign87700_e133690_d_n0, assign87700_e133690_d_n2, assign87700_e133690_d_n4, assign87700_e133690_d_n5, assign87700_e133690_d_n6, assign87700_e133690_d_n7, assign87700_e133690_d_n8, assign87700_e133690_d_n9, assign87700_e133690_d_n10, assign87700_e133690_d_n11, assign87700_e133690_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87700_e133684: f64 = (0.2 * locals.var_chi_b);
        let assign87700_e133687: f64 = (0.2 * locals.var_chi_b);
        let assign87700_e133688: f64 = (assign87700_e133684 * assign87700_e133687);
        (assign87700_e133688, (((0.2 * locals.var_chi_b_dn0) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn11) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn11))), (((0.2 * locals.var_chi_b_dn14) * assign87700_e133687) + (assign87700_e133684 * (0.2 * locals.var_chi_b_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign87700_e133690;
        locals.var_xmax2_dn0 = assign87700_e133690_d_n0;
        locals.var_xmax2_dn2 = assign87700_e133690_d_n2;
        locals.var_xmax2_dn4 = assign87700_e133690_d_n4;
        locals.var_xmax2_dn5 = assign87700_e133690_d_n5;
        locals.var_xmax2_dn6 = assign87700_e133690_d_n6;
        locals.var_xmax2_dn7 = assign87700_e133690_d_n7;
        locals.var_xmax2_dn8 = assign87700_e133690_d_n8;
        locals.var_xmax2_dn9 = assign87700_e133690_d_n9;
        locals.var_xmax2_dn10 = assign87700_e133690_d_n10;
        locals.var_xmax2_dn11 = assign87700_e133690_d_n11;
        locals.var_xmax2_dn14 = assign87700_e133690_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign87710_e133703, assign87710_e133703_d_n0, assign87710_e133703_d_n2, assign87710_e133703_d_n4, assign87710_e133703_d_n5, assign87710_e133703_d_n6, assign87710_e133703_d_n7, assign87710_e133703_d_n8, assign87710_e133703_d_n9, assign87710_e133703_d_n10, assign87710_e133703_d_n11, assign87710_e133703_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign87710_e133703;
        locals.var_xp_dn0 = assign87710_e133703_d_n0;
        locals.var_xp_dn2 = assign87710_e133703_d_n2;
        locals.var_xp_dn4 = assign87710_e133703_d_n4;
        locals.var_xp_dn5 = assign87710_e133703_d_n5;
        locals.var_xp_dn6 = assign87710_e133703_d_n6;
        locals.var_xp_dn7 = assign87710_e133703_d_n7;
        locals.var_xp_dn8 = assign87710_e133703_d_n8;
        locals.var_xp_dn9 = assign87710_e133703_d_n9;
        locals.var_xp_dn10 = assign87710_e133703_d_n10;
        locals.var_xp_dn11 = assign87710_e133703_d_n11;
        locals.var_xp_dn14 = assign87710_e133703_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign87720_e133716, assign87720_e133716_d_n0, assign87720_e133716_d_n2, assign87720_e133716_d_n4, assign87720_e133716_d_n5, assign87720_e133716_d_n6, assign87720_e133716_d_n7, assign87720_e133716_d_n8, assign87720_e133716_d_n9, assign87720_e133716_d_n10, assign87720_e133716_d_n11, assign87720_e133716_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign87720_e133716;
        locals.var_xmp_dn0 = assign87720_e133716_d_n0;
        locals.var_xmp_dn2 = assign87720_e133716_d_n2;
        locals.var_xmp_dn4 = assign87720_e133716_d_n4;
        locals.var_xmp_dn5 = assign87720_e133716_d_n5;
        locals.var_xmp_dn6 = assign87720_e133716_d_n6;
        locals.var_xmp_dn7 = assign87720_e133716_d_n7;
        locals.var_xmp_dn8 = assign87720_e133716_d_n8;
        locals.var_xmp_dn9 = assign87720_e133716_d_n9;
        locals.var_xmp_dn10 = assign87720_e133716_d_n10;
        locals.var_xmp_dn11 = assign87720_e133716_d_n11;
        locals.var_xmp_dn14 = assign87720_e133716_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign87730_e133729,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign87730_e133729;
        locals.var_m0_rv = 0.0;

        let (assign87740_e133742,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87740_e133742;
        locals.var_mm_rv = 0.0;

        let (assign87750_e133755, assign87750_e133755_d_n0, assign87750_e133755_d_n2, assign87750_e133755_d_n4, assign87750_e133755_d_n5, assign87750_e133755_d_n6, assign87750_e133755_d_n7, assign87750_e133755_d_n8, assign87750_e133755_d_n9, assign87750_e133755_d_n10, assign87750_e133755_d_n11, assign87750_e133755_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign87750_e133755;
        locals.var_arg_dn0 = assign87750_e133755_d_n0;
        locals.var_arg_dn2 = assign87750_e133755_d_n2;
        locals.var_arg_dn4 = assign87750_e133755_d_n4;
        locals.var_arg_dn5 = assign87750_e133755_d_n5;
        locals.var_arg_dn6 = assign87750_e133755_d_n6;
        locals.var_arg_dn7 = assign87750_e133755_d_n7;
        locals.var_arg_dn8 = assign87750_e133755_d_n8;
        locals.var_arg_dn9 = assign87750_e133755_d_n9;
        locals.var_arg_dn10 = assign87750_e133755_d_n10;
        locals.var_arg_dn11 = assign87750_e133755_d_n11;
        locals.var_arg_dn14 = assign87750_e133755_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign87760_e133768, assign87760_e133768_d_n0, assign87760_e133768_d_n2, assign87760_e133768_d_n4, assign87760_e133768_d_n5, assign87760_e133768_d_n6, assign87760_e133768_d_n7, assign87760_e133768_d_n8, assign87760_e133768_d_n9, assign87760_e133768_d_n10, assign87760_e133768_d_n11, assign87760_e133768_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign87760_e133768;
        locals.var_dnm_dn0 = assign87760_e133768_d_n0;
        locals.var_dnm_dn2 = assign87760_e133768_d_n2;
        locals.var_dnm_dn4 = assign87760_e133768_d_n4;
        locals.var_dnm_dn5 = assign87760_e133768_d_n5;
        locals.var_dnm_dn6 = assign87760_e133768_d_n6;
        locals.var_dnm_dn7 = assign87760_e133768_d_n7;
        locals.var_dnm_dn8 = assign87760_e133768_d_n8;
        locals.var_dnm_dn9 = assign87760_e133768_d_n9;
        locals.var_dnm_dn10 = assign87760_e133768_d_n10;
        locals.var_dnm_dn11 = assign87760_e133768_d_n11;
        locals.var_dnm_dn14 = assign87760_e133768_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign87770_e133783, assign87770_e133783_d_n0, assign87770_e133783_d_n2, assign87770_e133783_d_n4, assign87770_e133783_d_n5, assign87770_e133783_d_n6, assign87770_e133783_d_n7, assign87770_e133783_d_n8, assign87770_e133783_d_n9, assign87770_e133783_d_n10, assign87770_e133783_d_n11, assign87770_e133783_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87770_e133781: f64 = (locals.var_xp * locals.var_x2);
        (assign87770_e133781, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign87770_e133783;
        locals.var_xp_dn0 = assign87770_e133783_d_n0;
        locals.var_xp_dn2 = assign87770_e133783_d_n2;
        locals.var_xp_dn4 = assign87770_e133783_d_n4;
        locals.var_xp_dn5 = assign87770_e133783_d_n5;
        locals.var_xp_dn6 = assign87770_e133783_d_n6;
        locals.var_xp_dn7 = assign87770_e133783_d_n7;
        locals.var_xp_dn8 = assign87770_e133783_d_n8;
        locals.var_xp_dn9 = assign87770_e133783_d_n9;
        locals.var_xp_dn10 = assign87770_e133783_d_n10;
        locals.var_xp_dn11 = assign87770_e133783_d_n11;
        locals.var_xp_dn14 = assign87770_e133783_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign87780_e133798, assign87780_e133798_d_n0, assign87780_e133798_d_n2, assign87780_e133798_d_n4, assign87780_e133798_d_n5, assign87780_e133798_d_n6, assign87780_e133798_d_n7, assign87780_e133798_d_n8, assign87780_e133798_d_n9, assign87780_e133798_d_n10, assign87780_e133798_d_n11, assign87780_e133798_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87780_e133796: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign87780_e133796, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign87780_e133798;
        locals.var_xmp_dn0 = assign87780_e133798_d_n0;
        locals.var_xmp_dn2 = assign87780_e133798_d_n2;
        locals.var_xmp_dn4 = assign87780_e133798_d_n4;
        locals.var_xmp_dn5 = assign87780_e133798_d_n5;
        locals.var_xmp_dn6 = assign87780_e133798_d_n6;
        locals.var_xmp_dn7 = assign87780_e133798_d_n7;
        locals.var_xmp_dn8 = assign87780_e133798_d_n8;
        locals.var_xmp_dn9 = assign87780_e133798_d_n9;
        locals.var_xmp_dn10 = assign87780_e133798_d_n10;
        locals.var_xmp_dn11 = assign87780_e133798_d_n11;
        locals.var_xmp_dn14 = assign87780_e133798_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign87790_e133813, assign87790_e133813_d_n0, assign87790_e133813_d_n2, assign87790_e133813_d_n4, assign87790_e133813_d_n5, assign87790_e133813_d_n6, assign87790_e133813_d_n7, assign87790_e133813_d_n8, assign87790_e133813_d_n9, assign87790_e133813_d_n10, assign87790_e133813_d_n11, assign87790_e133813_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87790_e133811: f64 = (locals.var_xp * locals.var_x2);
        (assign87790_e133811, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign87790_e133813;
        locals.var_xp_dn0 = assign87790_e133813_d_n0;
        locals.var_xp_dn2 = assign87790_e133813_d_n2;
        locals.var_xp_dn4 = assign87790_e133813_d_n4;
        locals.var_xp_dn5 = assign87790_e133813_d_n5;
        locals.var_xp_dn6 = assign87790_e133813_d_n6;
        locals.var_xp_dn7 = assign87790_e133813_d_n7;
        locals.var_xp_dn8 = assign87790_e133813_d_n8;
        locals.var_xp_dn9 = assign87790_e133813_d_n9;
        locals.var_xp_dn10 = assign87790_e133813_d_n10;
        locals.var_xp_dn11 = assign87790_e133813_d_n11;
        locals.var_xp_dn14 = assign87790_e133813_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign87800_e133828, assign87800_e133828_d_n0, assign87800_e133828_d_n2, assign87800_e133828_d_n4, assign87800_e133828_d_n5, assign87800_e133828_d_n6, assign87800_e133828_d_n7, assign87800_e133828_d_n8, assign87800_e133828_d_n9, assign87800_e133828_d_n10, assign87800_e133828_d_n11, assign87800_e133828_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87800_e133826: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign87800_e133826, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign87800_e133828;
        locals.var_xmp_dn0 = assign87800_e133828_d_n0;
        locals.var_xmp_dn2 = assign87800_e133828_d_n2;
        locals.var_xmp_dn4 = assign87800_e133828_d_n4;
        locals.var_xmp_dn5 = assign87800_e133828_d_n5;
        locals.var_xmp_dn6 = assign87800_e133828_d_n6;
        locals.var_xmp_dn7 = assign87800_e133828_d_n7;
        locals.var_xmp_dn8 = assign87800_e133828_d_n8;
        locals.var_xmp_dn9 = assign87800_e133828_d_n9;
        locals.var_xmp_dn10 = assign87800_e133828_d_n10;
        locals.var_xmp_dn11 = assign87800_e133828_d_n11;
        locals.var_xmp_dn14 = assign87800_e133828_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign87810_e133843, assign87810_e133843_d_n0, assign87810_e133843_d_n2, assign87810_e133843_d_n4, assign87810_e133843_d_n5, assign87810_e133843_d_n6, assign87810_e133843_d_n7, assign87810_e133843_d_n8, assign87810_e133843_d_n9, assign87810_e133843_d_n10, assign87810_e133843_d_n11, assign87810_e133843_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87810_e133841: f64 = (locals.var_xp + locals.var_xmp);
        (assign87810_e133841, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign87810_e133843;
        locals.var_arg_dn0 = assign87810_e133843_d_n0;
        locals.var_arg_dn2 = assign87810_e133843_d_n2;
        locals.var_arg_dn4 = assign87810_e133843_d_n4;
        locals.var_arg_dn5 = assign87810_e133843_d_n5;
        locals.var_arg_dn6 = assign87810_e133843_d_n6;
        locals.var_arg_dn7 = assign87810_e133843_d_n7;
        locals.var_arg_dn8 = assign87810_e133843_d_n8;
        locals.var_arg_dn9 = assign87810_e133843_d_n9;
        locals.var_arg_dn10 = assign87810_e133843_d_n10;
        locals.var_arg_dn11 = assign87810_e133843_d_n11;
        locals.var_arg_dn14 = assign87810_e133843_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign87820_e133856, assign87820_e133856_d_n0, assign87820_e133856_d_n2, assign87820_e133856_d_n4, assign87820_e133856_d_n5, assign87820_e133856_d_n6, assign87820_e133856_d_n7, assign87820_e133856_d_n8, assign87820_e133856_d_n9, assign87820_e133856_d_n10, assign87820_e133856_d_n11, assign87820_e133856_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign87820_e133856;
        locals.var_dnm_dn0 = assign87820_e133856_d_n0;
        locals.var_dnm_dn2 = assign87820_e133856_d_n2;
        locals.var_dnm_dn4 = assign87820_e133856_d_n4;
        locals.var_dnm_dn5 = assign87820_e133856_d_n5;
        locals.var_dnm_dn6 = assign87820_e133856_d_n6;
        locals.var_dnm_dn7 = assign87820_e133856_d_n7;
        locals.var_dnm_dn8 = assign87820_e133856_d_n8;
        locals.var_dnm_dn9 = assign87820_e133856_d_n9;
        locals.var_dnm_dn10 = assign87820_e133856_d_n10;
        locals.var_dnm_dn11 = assign87820_e133856_d_n11;
        locals.var_dnm_dn14 = assign87820_e133856_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign87830_e133871: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2038 = assign87830_e133871;
        locals.var_guard2038_rv = 0.0;

        let assign87840_e133874: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2039 = assign87840_e133874;
        locals.var_guard2039_rv = 0.0;

        let (assign87850_e133891,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 != 0.0)) && (locals.var_guard2039 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87850_e133891;
        locals.var_mm_rv = 0.0;

        let assign87860_e133894: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2040 = assign87860_e133894;
        locals.var_guard2040_rv = 0.0;

        let (assign87870_e133914,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 != 0.0)) && (locals.var_guard2039 == 0.0)) && (locals.var_guard2040 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87870_e133914;
        locals.var_mm_rv = 0.0;

        let assign87880_e133917: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2041 = assign87880_e133917;
        locals.var_guard2041_rv = 0.0;

        let (assign87890_e133940,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 != 0.0)) && (locals.var_guard2039 == 0.0)) && (locals.var_guard2040 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87890_e133940;
        locals.var_mm_rv = 0.0;

        let assign87900_e133943: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2042 = assign87900_e133943;
        locals.var_guard2042_rv = 0.0;

        let (assign87910_e133969,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 != 0.0)) && (locals.var_guard2039 == 0.0)) && (locals.var_guard2040 == 0.0)) && (locals.var_guard2041 == 0.0)) && (locals.var_guard2042 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87910_e133969;
        locals.var_mm_rv = 0.0;

        let (assign87920_e133984,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign87920_e133984;
        locals.var_m0_rv = 0.0;

        let mut assign87930_loop_guard: usize = 0;
        while {
            let assign87930_cond_e134000: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign87930_cond_e134000 != 0.0
        } {
            assign87930_loop_guard += 1;
            assert!(assign87930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign87930_body0_e134016, assign87930_body0_e134016_d_n0, assign87930_body0_e134016_d_n2, assign87930_body0_e134016_d_n4, assign87930_body0_e134016_d_n5, assign87930_body0_e134016_d_n6, assign87930_body0_e134016_d_n7, assign87930_body0_e134016_d_n8, assign87930_body0_e134016_d_n9, assign87930_body0_e134016_d_n10, assign87930_body0_e134016_d_n11, assign87930_body0_e134016_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 != 0.0)) {
        let assign87930_body0_e134014: f64 = (locals.var_dnm).sqrt();
        (assign87930_body0_e134014, (locals.var_dnm_dn0 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn2 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn4 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn5 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn6 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn7 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn8 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn9 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn10 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn11 / (2.0 * assign87930_body0_e134014)), (locals.var_dnm_dn14 / (2.0 * assign87930_body0_e134014)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign87930_body0_e134016;
            locals.var_dnm_dn0 = assign87930_body0_e134016_d_n0;
            locals.var_dnm_dn2 = assign87930_body0_e134016_d_n2;
            locals.var_dnm_dn4 = assign87930_body0_e134016_d_n4;
            locals.var_dnm_dn5 = assign87930_body0_e134016_d_n5;
            locals.var_dnm_dn6 = assign87930_body0_e134016_d_n6;
            locals.var_dnm_dn7 = assign87930_body0_e134016_d_n7;
            locals.var_dnm_dn8 = assign87930_body0_e134016_d_n8;
            locals.var_dnm_dn9 = assign87930_body0_e134016_d_n9;
            locals.var_dnm_dn10 = assign87930_body0_e134016_d_n10;
            locals.var_dnm_dn11 = assign87930_body0_e134016_d_n11;
            locals.var_dnm_dn14 = assign87930_body0_e134016_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign87930_body1_e134033,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 != 0.0)) {
        let assign87930_body1_e134031: f64 = (locals.var_m0 + 1.0);
        (assign87930_body1_e134031,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign87930_body1_e134033;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_337(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87940_e134060, assign87940_e134060_d_n0, assign87940_e134060_d_n2, assign87940_e134060_d_n4, assign87940_e134060_d_n5, assign87940_e134060_d_n6, assign87940_e134060_d_n7, assign87940_e134060_d_n8, assign87940_e134060_d_n9, assign87940_e134060_d_n10, assign87940_e134060_d_n11, assign87940_e134060_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) && (locals.var_guard2038 == 0.0)) {
        let (assign87940_e134058, assign87940_e134058_d_n0, assign87940_e134058_d_n2, assign87940_e134058_d_n4, assign87940_e134058_d_n5, assign87940_e134058_d_n6, assign87940_e134058_d_n7, assign87940_e134058_d_n8, assign87940_e134058_d_n9, assign87940_e134058_d_n10, assign87940_e134058_d_n11, assign87940_e134058_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign87940_e134055: f64 = (2.0 * 2.0);
                let assign87940_e134056: f64 = (1.0 / assign87940_e134055);
                let assign87940_e134057: f64 = (locals.var_dnm).powf(assign87940_e134056);
                (assign87940_e134057, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn0)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn2)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn4)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn5)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn6)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn7)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn8)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn9)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn10)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn11)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87940_e134056) as f64).is_finite() && ((assign87940_e134056) as f64).fract() == 0.0 { if assign87940_e134056 == 0.0 { 0.0 } else { (assign87940_e134056 * ((locals.var_dnm).powf(assign87940_e134056 - 1.0) * locals.var_dnm_dn14)) } } else { (assign87940_e134057 * (assign87940_e134056 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign87940_e134058, assign87940_e134058_d_n0, assign87940_e134058_d_n2, assign87940_e134058_d_n4, assign87940_e134058_d_n5, assign87940_e134058_d_n6, assign87940_e134058_d_n7, assign87940_e134058_d_n8, assign87940_e134058_d_n9, assign87940_e134058_d_n10, assign87940_e134058_d_n11, assign87940_e134058_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign87940_e134060;
        locals.var_dnm_dn0 = assign87940_e134060_d_n0;
        locals.var_dnm_dn2 = assign87940_e134060_d_n2;
        locals.var_dnm_dn4 = assign87940_e134060_d_n4;
        locals.var_dnm_dn5 = assign87940_e134060_d_n5;
        locals.var_dnm_dn6 = assign87940_e134060_d_n6;
        locals.var_dnm_dn7 = assign87940_e134060_d_n7;
        locals.var_dnm_dn8 = assign87940_e134060_d_n8;
        locals.var_dnm_dn9 = assign87940_e134060_d_n9;
        locals.var_dnm_dn10 = assign87940_e134060_d_n10;
        locals.var_dnm_dn11 = assign87940_e134060_d_n11;
        locals.var_dnm_dn14 = assign87940_e134060_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign87950_e134075, assign87950_e134075_d_n0, assign87950_e134075_d_n2, assign87950_e134075_d_n4, assign87950_e134075_d_n5, assign87950_e134075_d_n6, assign87950_e134075_d_n7, assign87950_e134075_d_n8, assign87950_e134075_d_n9, assign87950_e134075_d_n10, assign87950_e134075_d_n11, assign87950_e134075_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87950_e134073: f64 = (1.0 / locals.var_dnm);
        (assign87950_e134073, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign87950_e134075;
        locals.var_dnm_dn0 = assign87950_e134075_d_n0;
        locals.var_dnm_dn2 = assign87950_e134075_d_n2;
        locals.var_dnm_dn4 = assign87950_e134075_d_n4;
        locals.var_dnm_dn5 = assign87950_e134075_d_n5;
        locals.var_dnm_dn6 = assign87950_e134075_d_n6;
        locals.var_dnm_dn7 = assign87950_e134075_d_n7;
        locals.var_dnm_dn8 = assign87950_e134075_d_n8;
        locals.var_dnm_dn9 = assign87950_e134075_d_n9;
        locals.var_dnm_dn10 = assign87950_e134075_d_n10;
        locals.var_dnm_dn11 = assign87950_e134075_d_n11;
        locals.var_dnm_dn14 = assign87950_e134075_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign87960_e134094, assign87960_e134094_d_n0, assign87960_e134094_d_n2, assign87960_e134094_d_n4, assign87960_e134094_d_n5, assign87960_e134094_d_n6, assign87960_e134094_d_n7, assign87960_e134094_d_n8, assign87960_e134094_d_n9, assign87960_e134094_d_n10, assign87960_e134094_d_n11, assign87960_e134094_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87960_e134089: f64 = (0.2 * locals.var_chi_b);
        let assign87960_e134090: f64 = (locals.var_tmf1 * assign87960_e134089);
        let assign87960_e134092: f64 = (assign87960_e134090 * locals.var_dnm);
        (assign87960_e134092, ((((locals.var_tmf1_dn0 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn11))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign87960_e134089) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn14))) * locals.var_dnm) + (assign87960_e134090 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign87960_e134094;
        locals.var_tmf0_dn0 = assign87960_e134094_d_n0;
        locals.var_tmf0_dn2 = assign87960_e134094_d_n2;
        locals.var_tmf0_dn4 = assign87960_e134094_d_n4;
        locals.var_tmf0_dn5 = assign87960_e134094_d_n5;
        locals.var_tmf0_dn6 = assign87960_e134094_d_n6;
        locals.var_tmf0_dn7 = assign87960_e134094_d_n7;
        locals.var_tmf0_dn8 = assign87960_e134094_d_n8;
        locals.var_tmf0_dn9 = assign87960_e134094_d_n9;
        locals.var_tmf0_dn10 = assign87960_e134094_d_n10;
        locals.var_tmf0_dn11 = assign87960_e134094_d_n11;
        locals.var_tmf0_dn14 = assign87960_e134094_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign87970_e134115, assign87970_e134115_d_n0, assign87970_e134115_d_n2, assign87970_e134115_d_n4, assign87970_e134115_d_n5, assign87970_e134115_d_n6, assign87970_e134115_d_n7, assign87970_e134115_d_n8, assign87970_e134115_d_n9, assign87970_e134115_d_n10, assign87970_e134115_d_n11, assign87970_e134115_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87970_e134107: f64 = (0.2 * locals.var_chi_b);
        let assign87970_e134109: f64 = (assign87970_e134107 * locals.var_xmp);
        let assign87970_e134111: f64 = (assign87970_e134109 * locals.var_dnm);
        let assign87970_e134113: f64 = (assign87970_e134111 / locals.var_arg);
        (assign87970_e134113, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn0)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn2)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn4)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn5)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn6)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn7)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn8)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn9)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn10)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn11) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn11)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn14) * locals.var_xmp) + (assign87970_e134107 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign87970_e134109 * locals.var_dnm_dn14)) * locals.var_arg) - (assign87970_e134111 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign87970_e134115;
        locals.var_t1_dn0 = assign87970_e134115_d_n0;
        locals.var_t1_dn2 = assign87970_e134115_d_n2;
        locals.var_t1_dn4 = assign87970_e134115_d_n4;
        locals.var_t1_dn5 = assign87970_e134115_d_n5;
        locals.var_t1_dn6 = assign87970_e134115_d_n6;
        locals.var_t1_dn7 = assign87970_e134115_d_n7;
        locals.var_t1_dn8 = assign87970_e134115_d_n8;
        locals.var_t1_dn9 = assign87970_e134115_d_n9;
        locals.var_t1_dn10 = assign87970_e134115_d_n10;
        locals.var_t1_dn11 = assign87970_e134115_d_n11;
        locals.var_t1_dn14 = assign87970_e134115_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign87980_e134134, assign87980_e134134_d_n0, assign87980_e134134_d_n2, assign87980_e134134_d_n4, assign87980_e134134_d_n5, assign87980_e134134_d_n6, assign87980_e134134_d_n7, assign87980_e134134_d_n8, assign87980_e134134_d_n9, assign87980_e134134_d_n10, assign87980_e134134_d_n11, assign87980_e134134_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        let assign87980_e134129: f64 = (0.2 * locals.var_chi_b);
        let assign87980_e134130: f64 = (locals.var_chi_b - assign87980_e134129);
        let assign87980_e134132: f64 = (assign87980_e134130 + locals.var_tmf0);
        (assign87980_e134132, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn11 - (0.2 * locals.var_chi_b_dn11)) + locals.var_tmf0_dn11), ((locals.var_chi_b_dn14 - (0.2 * locals.var_chi_b_dn14)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign87980_e134134;
        locals.var_chi_dn0 = assign87980_e134134_d_n0;
        locals.var_chi_dn2 = assign87980_e134134_d_n2;
        locals.var_chi_dn4 = assign87980_e134134_d_n4;
        locals.var_chi_dn5 = assign87980_e134134_d_n5;
        locals.var_chi_dn6 = assign87980_e134134_d_n6;
        locals.var_chi_dn7 = assign87980_e134134_d_n7;
        locals.var_chi_dn8 = assign87980_e134134_d_n8;
        locals.var_chi_dn9 = assign87980_e134134_d_n9;
        locals.var_chi_dn10 = assign87980_e134134_d_n10;
        locals.var_chi_dn11 = assign87980_e134134_d_n11;
        locals.var_chi_dn14 = assign87980_e134134_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign87990_e134147, assign87990_e134147_d_n0, assign87990_e134147_d_n2, assign87990_e134147_d_n4, assign87990_e134147_d_n5, assign87990_e134147_d_n6, assign87990_e134147_d_n7, assign87990_e134147_d_n8, assign87990_e134147_d_n9, assign87990_e134147_d_n10, assign87990_e134147_d_n11, assign87990_e134147_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign87990_e134147;
        locals.var_t1_dn0 = assign87990_e134147_d_n0;
        locals.var_t1_dn2 = assign87990_e134147_d_n2;
        locals.var_t1_dn4 = assign87990_e134147_d_n4;
        locals.var_t1_dn5 = assign87990_e134147_d_n5;
        locals.var_t1_dn6 = assign87990_e134147_d_n6;
        locals.var_t1_dn7 = assign87990_e134147_d_n7;
        locals.var_t1_dn8 = assign87990_e134147_d_n8;
        locals.var_t1_dn9 = assign87990_e134147_d_n9;
        locals.var_t1_dn10 = assign87990_e134147_d_n10;
        locals.var_t1_dn11 = assign87990_e134147_d_n11;
        locals.var_t1_dn14 = assign87990_e134147_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign88000_e134161, assign88000_e134161_d_n0, assign88000_e134161_d_n2, assign88000_e134161_d_n4, assign88000_e134161_d_n5, assign88000_e134161_d_n6, assign88000_e134161_d_n7, assign88000_e134161_d_n8, assign88000_e134161_d_n9, assign88000_e134161_d_n10, assign88000_e134161_d_n11, assign88000_e134161_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign88000_e134161;
        locals.var_chi_dn0 = assign88000_e134161_d_n0;
        locals.var_chi_dn2 = assign88000_e134161_d_n2;
        locals.var_chi_dn4 = assign88000_e134161_d_n4;
        locals.var_chi_dn5 = assign88000_e134161_d_n5;
        locals.var_chi_dn6 = assign88000_e134161_d_n6;
        locals.var_chi_dn7 = assign88000_e134161_d_n7;
        locals.var_chi_dn8 = assign88000_e134161_d_n8;
        locals.var_chi_dn9 = assign88000_e134161_d_n9;
        locals.var_chi_dn10 = assign88000_e134161_d_n10;
        locals.var_chi_dn11 = assign88000_e134161_d_n11;
        locals.var_chi_dn14 = assign88000_e134161_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign88010_e134175, assign88010_e134175_d_n0, assign88010_e134175_d_n2, assign88010_e134175_d_n4, assign88010_e134175_d_n5, assign88010_e134175_d_n6, assign88010_e134175_d_n7, assign88010_e134175_d_n8, assign88010_e134175_d_n9, assign88010_e134175_d_n10, assign88010_e134175_d_n11, assign88010_e134175_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign88010_e134175;
        locals.var_t1_dn0 = assign88010_e134175_d_n0;
        locals.var_t1_dn2 = assign88010_e134175_d_n2;
        locals.var_t1_dn4 = assign88010_e134175_d_n4;
        locals.var_t1_dn5 = assign88010_e134175_d_n5;
        locals.var_t1_dn6 = assign88010_e134175_d_n6;
        locals.var_t1_dn7 = assign88010_e134175_d_n7;
        locals.var_t1_dn8 = assign88010_e134175_d_n8;
        locals.var_t1_dn9 = assign88010_e134175_d_n9;
        locals.var_t1_dn10 = assign88010_e134175_d_n10;
        locals.var_t1_dn11 = assign88010_e134175_d_n11;
        locals.var_t1_dn14 = assign88010_e134175_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign88020_e134192, assign88020_e134192_d_n0, assign88020_e134192_d_n2, assign88020_e134192_d_n4, assign88020_e134192_d_n5, assign88020_e134192_d_n6, assign88020_e134192_d_n7, assign88020_e134192_d_n8, assign88020_e134192_d_n9, assign88020_e134192_d_n10, assign88020_e134192_d_n11, assign88020_e134192_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2036 == 0.0)) {
        let (assign88020_e134190, assign88020_e134190_d_n0, assign88020_e134190_d_n2, assign88020_e134190_d_n4, assign88020_e134190_d_n5, assign88020_e134190_d_n6, assign88020_e134190_d_n7, assign88020_e134190_d_n8, assign88020_e134190_d_n9, assign88020_e134190_d_n10, assign88020_e134190_d_n11, assign88020_e134190_d_n14,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            }
        };
        (assign88020_e134190, assign88020_e134190_d_n0, assign88020_e134190_d_n2, assign88020_e134190_d_n4, assign88020_e134190_d_n5, assign88020_e134190_d_n6, assign88020_e134190_d_n7, assign88020_e134190_d_n8, assign88020_e134190_d_n9, assign88020_e134190_d_n10, assign88020_e134190_d_n11, assign88020_e134190_d_n14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign88020_e134192;
        locals.var_chi_dn0 = assign88020_e134192_d_n0;
        locals.var_chi_dn2 = assign88020_e134192_d_n2;
        locals.var_chi_dn4 = assign88020_e134192_d_n4;
        locals.var_chi_dn5 = assign88020_e134192_d_n5;
        locals.var_chi_dn6 = assign88020_e134192_d_n6;
        locals.var_chi_dn7 = assign88020_e134192_d_n7;
        locals.var_chi_dn8 = assign88020_e134192_d_n8;
        locals.var_chi_dn9 = assign88020_e134192_d_n9;
        locals.var_chi_dn10 = assign88020_e134192_d_n10;
        locals.var_chi_dn11 = assign88020_e134192_d_n11;
        locals.var_chi_dn14 = assign88020_e134192_d_n14;
        locals.var_chi_rv = 0.0;

        let assign88030_e134195: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2043 = assign88030_e134195;
        locals.var_guard2043_rv = 0.0;

        let (assign88040_e134208, assign88040_e134208_d_n0, assign88040_e134208_d_n2, assign88040_e134208_d_n4, assign88040_e134208_d_n5, assign88040_e134208_d_n6, assign88040_e134208_d_n7, assign88040_e134208_d_n8, assign88040_e134208_d_n9, assign88040_e134208_d_n10, assign88040_e134208_d_n11, assign88040_e134208_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88040_e134204: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign88040_e134206: f64 = (assign88040_e134204 - locals.var_vxbgmtcl);
        (assign88040_e134206, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign88040_e134208;
        locals.var_ps0ld_dn0 = assign88040_e134208_d_n0;
        locals.var_ps0ld_dn2 = assign88040_e134208_d_n2;
        locals.var_ps0ld_dn4 = assign88040_e134208_d_n4;
        locals.var_ps0ld_dn5 = assign88040_e134208_d_n5;
        locals.var_ps0ld_dn6 = assign88040_e134208_d_n6;
        locals.var_ps0ld_dn7 = assign88040_e134208_d_n7;
        locals.var_ps0ld_dn8 = assign88040_e134208_d_n8;
        locals.var_ps0ld_dn9 = assign88040_e134208_d_n9;
        locals.var_ps0ld_dn10 = assign88040_e134208_d_n10;
        locals.var_ps0ld_dn11 = assign88040_e134208_d_n11;
        locals.var_ps0ld_dn14 = assign88040_e134208_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign88050_e134211: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2044 = assign88050_e134211;
        locals.var_guard2044_rv = 0.0;

        let (assign88060_e134224, assign88060_e134224_d_n0, assign88060_e134224_d_n2, assign88060_e134224_d_n4, assign88060_e134224_d_n5, assign88060_e134224_d_n6, assign88060_e134224_d_n7, assign88060_e134224_d_n8, assign88060_e134224_d_n9, assign88060_e134224_d_n10, assign88060_e134224_d_n11, assign88060_e134224_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2044 != 0.0)) {
        let assign88060_e134222: f64 = (p.p334 - locals.var_wdep_func);
        (assign88060_e134222, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88060_e134224;
        locals.var_t2_dn0 = assign88060_e134224_d_n0;
        locals.var_t2_dn2 = assign88060_e134224_d_n2;
        locals.var_t2_dn4 = assign88060_e134224_d_n4;
        locals.var_t2_dn5 = assign88060_e134224_d_n5;
        locals.var_t2_dn6 = assign88060_e134224_d_n6;
        locals.var_t2_dn7 = assign88060_e134224_d_n7;
        locals.var_t2_dn8 = assign88060_e134224_d_n8;
        locals.var_t2_dn9 = assign88060_e134224_d_n9;
        locals.var_t2_dn10 = assign88060_e134224_d_n10;
        locals.var_t2_dn11 = assign88060_e134224_d_n11;
        locals.var_t2_dn14 = assign88060_e134224_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign88070_e134249, assign88070_e134249_d_n0, assign88070_e134249_d_n2, assign88070_e134249_d_n4, assign88070_e134249_d_n5, assign88070_e134249_d_n6, assign88070_e134249_d_n7, assign88070_e134249_d_n8, assign88070_e134249_d_n9, assign88070_e134249_d_n10, assign88070_e134249_d_n11, assign88070_e134249_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2044 == 0.0)) {
        let assign88070_e134236: f64 = (locals.var_vdsi + p.p137);
        let assign88070_e134239: f64 = (locals.var_vdsi + p.p137);
        let assign88070_e134240: f64 = (assign88070_e134236 * assign88070_e134239);
        let assign88070_e134243: f64 = (4.0 * 0.1);
        let assign88070_e134245: f64 = (assign88070_e134243 * 0.1);
        let assign88070_e134246: f64 = (assign88070_e134240 + assign88070_e134245);
        let assign88070_e134247: f64 = (assign88070_e134246).sqrt();
        (assign88070_e134247, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign88070_e134239) + (assign88070_e134236 * locals.var_vdsi_dn6)) / (2.0 * assign88070_e134247)), 0.0, (((locals.var_vdsi_dn8 * assign88070_e134239) + (assign88070_e134236 * locals.var_vdsi_dn8)) / (2.0 * assign88070_e134247)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign88070_e134249;
        locals.var_tmf2_dn0 = assign88070_e134249_d_n0;
        locals.var_tmf2_dn2 = assign88070_e134249_d_n2;
        locals.var_tmf2_dn4 = assign88070_e134249_d_n4;
        locals.var_tmf2_dn5 = assign88070_e134249_d_n5;
        locals.var_tmf2_dn6 = assign88070_e134249_d_n6;
        locals.var_tmf2_dn7 = assign88070_e134249_d_n7;
        locals.var_tmf2_dn8 = assign88070_e134249_d_n8;
        locals.var_tmf2_dn9 = assign88070_e134249_d_n9;
        locals.var_tmf2_dn10 = assign88070_e134249_d_n10;
        locals.var_tmf2_dn11 = assign88070_e134249_d_n11;
        locals.var_tmf2_dn14 = assign88070_e134249_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign88080_e134269, assign88080_e134269_d_n0, assign88080_e134269_d_n2, assign88080_e134269_d_n4, assign88080_e134269_d_n5, assign88080_e134269_d_n6, assign88080_e134269_d_n7, assign88080_e134269_d_n8, assign88080_e134269_d_n9, assign88080_e134269_d_n10, assign88080_e134269_d_n11, assign88080_e134269_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2044 == 0.0)) {
        let assign88080_e134263: f64 = (locals.var_vdsi + p.p137);
        let assign88080_e134265: f64 = (assign88080_e134263 / locals.var_tmf2);
        let assign88080_e134266: f64 = (1.0 + assign88080_e134265);
        let assign88080_e134267: f64 = (0.5 * assign88080_e134266);
        (assign88080_e134267, (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign88080_e134263 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign88080_e134263 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88080_e134263 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign88080_e134269;
        locals.var_t9_dn0 = assign88080_e134269_d_n0;
        locals.var_t9_dn2 = assign88080_e134269_d_n2;
        locals.var_t9_dn4 = assign88080_e134269_d_n4;
        locals.var_t9_dn5 = assign88080_e134269_d_n5;
        locals.var_t9_dn6 = assign88080_e134269_d_n6;
        locals.var_t9_dn7 = assign88080_e134269_d_n7;
        locals.var_t9_dn8 = assign88080_e134269_d_n8;
        locals.var_t9_dn9 = assign88080_e134269_d_n9;
        locals.var_t9_dn10 = assign88080_e134269_d_n10;
        locals.var_t9_dn11 = assign88080_e134269_d_n11;
        locals.var_t9_dn14 = assign88080_e134269_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign88090_e134287, assign88090_e134287_d_n0, assign88090_e134287_d_n2, assign88090_e134287_d_n4, assign88090_e134287_d_n5, assign88090_e134287_d_n6, assign88090_e134287_d_n7, assign88090_e134287_d_n8, assign88090_e134287_d_n9, assign88090_e134287_d_n10, assign88090_e134287_d_n11, assign88090_e134287_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2044 == 0.0)) {
        let assign88090_e134282: f64 = (locals.var_vdsi + p.p137);
        let assign88090_e134284: f64 = (assign88090_e134282 + locals.var_tmf2);
        let assign88090_e134285: f64 = (0.5 * assign88090_e134284);
        (assign88090_e134285, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88090_e134287;
        locals.var_t2_dn0 = assign88090_e134287_d_n0;
        locals.var_t2_dn2 = assign88090_e134287_d_n2;
        locals.var_t2_dn4 = assign88090_e134287_d_n4;
        locals.var_t2_dn5 = assign88090_e134287_d_n5;
        locals.var_t2_dn6 = assign88090_e134287_d_n6;
        locals.var_t2_dn7 = assign88090_e134287_d_n7;
        locals.var_t2_dn8 = assign88090_e134287_d_n8;
        locals.var_t2_dn9 = assign88090_e134287_d_n9;
        locals.var_t2_dn10 = assign88090_e134287_d_n10;
        locals.var_t2_dn11 = assign88090_e134287_d_n11;
        locals.var_t2_dn14 = assign88090_e134287_d_n14;
        locals.var_t2_rv = 0.0;

        let assign88100_e134290: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2045 = assign88100_e134290;
        locals.var_guard2045_rv = 0.0;

        let (assign88110_e134304, assign88110_e134304_d_n0, assign88110_e134304_d_n2, assign88110_e134304_d_n4, assign88110_e134304_d_n5, assign88110_e134304_d_n6, assign88110_e134304_d_n7, assign88110_e134304_d_n8, assign88110_e134304_d_n9, assign88110_e134304_d_n10, assign88110_e134304_d_n11, assign88110_e134304_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2044 == 0.0)) && (locals.var_guard2045 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88110_e134304;
        locals.var_t2_dn0 = assign88110_e134304_d_n0;
        locals.var_t2_dn2 = assign88110_e134304_d_n2;
        locals.var_t2_dn4 = assign88110_e134304_d_n4;
        locals.var_t2_dn5 = assign88110_e134304_d_n5;
        locals.var_t2_dn6 = assign88110_e134304_d_n6;
        locals.var_t2_dn7 = assign88110_e134304_d_n7;
        locals.var_t2_dn8 = assign88110_e134304_d_n8;
        locals.var_t2_dn9 = assign88110_e134304_d_n9;
        locals.var_t2_dn10 = assign88110_e134304_d_n10;
        locals.var_t2_dn11 = assign88110_e134304_d_n11;
        locals.var_t2_dn14 = assign88110_e134304_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign88120_e134318, assign88120_e134318_d_n0, assign88120_e134318_d_n2, assign88120_e134318_d_n4, assign88120_e134318_d_n5, assign88120_e134318_d_n6, assign88120_e134318_d_n7, assign88120_e134318_d_n8, assign88120_e134318_d_n9, assign88120_e134318_d_n10, assign88120_e134318_d_n11, assign88120_e134318_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2044 == 0.0)) && (locals.var_guard2045 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign88120_e134318;
        locals.var_t9_dn0 = assign88120_e134318_d_n0;
        locals.var_t9_dn2 = assign88120_e134318_d_n2;
        locals.var_t9_dn4 = assign88120_e134318_d_n4;
        locals.var_t9_dn5 = assign88120_e134318_d_n5;
        locals.var_t9_dn6 = assign88120_e134318_d_n6;
        locals.var_t9_dn7 = assign88120_e134318_d_n7;
        locals.var_t9_dn8 = assign88120_e134318_d_n8;
        locals.var_t9_dn9 = assign88120_e134318_d_n9;
        locals.var_t9_dn10 = assign88120_e134318_d_n10;
        locals.var_t9_dn11 = assign88120_e134318_d_n11;
        locals.var_t9_dn14 = assign88120_e134318_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign88130_e134335, assign88130_e134335_d_n0, assign88130_e134335_d_n2, assign88130_e134335_d_n4, assign88130_e134335_d_n5, assign88130_e134335_d_n6, assign88130_e134335_d_n7, assign88130_e134335_d_n8, assign88130_e134335_d_n9, assign88130_e134335_d_n10, assign88130_e134335_d_n11, assign88130_e134335_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2044 == 0.0)) {
        let assign88130_e134330: f64 = (locals.var_kjunc * locals.var_t2);
        let assign88130_e134331: f64 = (assign88130_e134330).sqrt();
        let assign88130_e134333: f64 = (assign88130_e134331 * p.p432);
        (assign88130_e134333, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign88130_e134331)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign88130_e134331)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign88130_e134335;
        locals.var_wjunc0_dn0 = assign88130_e134335_d_n0;
        locals.var_wjunc0_dn2 = assign88130_e134335_d_n2;
        locals.var_wjunc0_dn4 = assign88130_e134335_d_n4;
        locals.var_wjunc0_dn5 = assign88130_e134335_d_n5;
        locals.var_wjunc0_dn6 = assign88130_e134335_d_n6;
        locals.var_wjunc0_dn7 = assign88130_e134335_d_n7;
        locals.var_wjunc0_dn8 = assign88130_e134335_d_n8;
        locals.var_wjunc0_dn9 = assign88130_e134335_d_n9;
        locals.var_wjunc0_dn10 = assign88130_e134335_d_n10;
        locals.var_wjunc0_dn11 = assign88130_e134335_d_n11;
        locals.var_wjunc0_dn14 = assign88130_e134335_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign88140_e134349, assign88140_e134349_d_n0, assign88140_e134349_d_n2, assign88140_e134349_d_n4, assign88140_e134349_d_n5, assign88140_e134349_d_n6, assign88140_e134349_d_n7, assign88140_e134349_d_n8, assign88140_e134349_d_n9, assign88140_e134349_d_n10, assign88140_e134349_d_n11, assign88140_e134349_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2044 == 0.0)) {
        let assign88140_e134347: f64 = (p.p334 - locals.var_wjunc0);
        (assign88140_e134347, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88140_e134349;
        locals.var_t2_dn0 = assign88140_e134349_d_n0;
        locals.var_t2_dn2 = assign88140_e134349_d_n2;
        locals.var_t2_dn4 = assign88140_e134349_d_n4;
        locals.var_t2_dn5 = assign88140_e134349_d_n5;
        locals.var_t2_dn6 = assign88140_e134349_d_n6;
        locals.var_t2_dn7 = assign88140_e134349_d_n7;
        locals.var_t2_dn8 = assign88140_e134349_d_n8;
        locals.var_t2_dn9 = assign88140_e134349_d_n9;
        locals.var_t2_dn10 = assign88140_e134349_d_n10;
        locals.var_t2_dn11 = assign88140_e134349_d_n11;
        locals.var_t2_dn14 = assign88140_e134349_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign88150_e134371, assign88150_e134371_d_n0, assign88150_e134371_d_n2, assign88150_e134371_d_n4, assign88150_e134371_d_n5, assign88150_e134371_d_n6, assign88150_e134371_d_n7, assign88150_e134371_d_n8, assign88150_e134371_d_n9, assign88150_e134371_d_n10, assign88150_e134371_d_n11, assign88150_e134371_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88150_e134358: f64 = (locals.var_t2 * locals.var_t2);
        let assign88150_e134362: f64 = (p.p334 * 0.01);
        let assign88150_e134363: f64 = (4.0 * assign88150_e134362);
        let assign88150_e134366: f64 = (p.p334 * 0.01);
        let assign88150_e134367: f64 = (assign88150_e134363 * assign88150_e134366);
        let assign88150_e134368: f64 = (assign88150_e134358 + assign88150_e134367);
        let assign88150_e134369: f64 = (assign88150_e134368).sqrt();
        (assign88150_e134369, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign88150_e134369)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign88150_e134369)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign88150_e134371;
        locals.var_tmf2_dn0 = assign88150_e134371_d_n0;
        locals.var_tmf2_dn2 = assign88150_e134371_d_n2;
        locals.var_tmf2_dn4 = assign88150_e134371_d_n4;
        locals.var_tmf2_dn5 = assign88150_e134371_d_n5;
        locals.var_tmf2_dn6 = assign88150_e134371_d_n6;
        locals.var_tmf2_dn7 = assign88150_e134371_d_n7;
        locals.var_tmf2_dn8 = assign88150_e134371_d_n8;
        locals.var_tmf2_dn9 = assign88150_e134371_d_n9;
        locals.var_tmf2_dn10 = assign88150_e134371_d_n10;
        locals.var_tmf2_dn11 = assign88150_e134371_d_n11;
        locals.var_tmf2_dn14 = assign88150_e134371_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign88160_e134386, assign88160_e134386_d_n0, assign88160_e134386_d_n2, assign88160_e134386_d_n4, assign88160_e134386_d_n5, assign88160_e134386_d_n6, assign88160_e134386_d_n7, assign88160_e134386_d_n8, assign88160_e134386_d_n9, assign88160_e134386_d_n10, assign88160_e134386_d_n11, assign88160_e134386_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88160_e134382: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign88160_e134383: f64 = (1.0 + assign88160_e134382);
        let assign88160_e134384: f64 = (0.5 * assign88160_e134383);
        (assign88160_e134384, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign88160_e134386;
        locals.var_t9_dn0 = assign88160_e134386_d_n0;
        locals.var_t9_dn2 = assign88160_e134386_d_n2;
        locals.var_t9_dn4 = assign88160_e134386_d_n4;
        locals.var_t9_dn5 = assign88160_e134386_d_n5;
        locals.var_t9_dn6 = assign88160_e134386_d_n6;
        locals.var_t9_dn7 = assign88160_e134386_d_n7;
        locals.var_t9_dn8 = assign88160_e134386_d_n8;
        locals.var_t9_dn9 = assign88160_e134386_d_n9;
        locals.var_t9_dn10 = assign88160_e134386_d_n10;
        locals.var_t9_dn11 = assign88160_e134386_d_n11;
        locals.var_t9_dn14 = assign88160_e134386_d_n14;
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_338(
        locals: &mut StampLocals,
    ) {
        let (assign88170_e134399, assign88170_e134399_d_n0, assign88170_e134399_d_n2, assign88170_e134399_d_n4, assign88170_e134399_d_n5, assign88170_e134399_d_n6, assign88170_e134399_d_n7, assign88170_e134399_d_n8, assign88170_e134399_d_n9, assign88170_e134399_d_n10, assign88170_e134399_d_n11, assign88170_e134399_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88170_e134396: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign88170_e134397: f64 = (0.5 * assign88170_e134396);
        (assign88170_e134397, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88170_e134399;
        locals.var_t2_dn0 = assign88170_e134399_d_n0;
        locals.var_t2_dn2 = assign88170_e134399_d_n2;
        locals.var_t2_dn4 = assign88170_e134399_d_n4;
        locals.var_t2_dn5 = assign88170_e134399_d_n5;
        locals.var_t2_dn6 = assign88170_e134399_d_n6;
        locals.var_t2_dn7 = assign88170_e134399_d_n7;
        locals.var_t2_dn8 = assign88170_e134399_d_n8;
        locals.var_t2_dn9 = assign88170_e134399_d_n9;
        locals.var_t2_dn10 = assign88170_e134399_d_n10;
        locals.var_t2_dn11 = assign88170_e134399_d_n11;
        locals.var_t2_dn14 = assign88170_e134399_d_n14;
        locals.var_t2_rv = 0.0;

        let assign88180_e134402: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2046 = assign88180_e134402;
        locals.var_guard2046_rv = 0.0;

        let (assign88190_e134413, assign88190_e134413_d_n0, assign88190_e134413_d_n2, assign88190_e134413_d_n4, assign88190_e134413_d_n5, assign88190_e134413_d_n6, assign88190_e134413_d_n7, assign88190_e134413_d_n8, assign88190_e134413_d_n9, assign88190_e134413_d_n10, assign88190_e134413_d_n11, assign88190_e134413_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2046 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88190_e134413;
        locals.var_t2_dn0 = assign88190_e134413_d_n0;
        locals.var_t2_dn2 = assign88190_e134413_d_n2;
        locals.var_t2_dn4 = assign88190_e134413_d_n4;
        locals.var_t2_dn5 = assign88190_e134413_d_n5;
        locals.var_t2_dn6 = assign88190_e134413_d_n6;
        locals.var_t2_dn7 = assign88190_e134413_d_n7;
        locals.var_t2_dn8 = assign88190_e134413_d_n8;
        locals.var_t2_dn9 = assign88190_e134413_d_n9;
        locals.var_t2_dn10 = assign88190_e134413_d_n10;
        locals.var_t2_dn11 = assign88190_e134413_d_n11;
        locals.var_t2_dn14 = assign88190_e134413_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign88200_e134424, assign88200_e134424_d_n0, assign88200_e134424_d_n2, assign88200_e134424_d_n4, assign88200_e134424_d_n5, assign88200_e134424_d_n6, assign88200_e134424_d_n7, assign88200_e134424_d_n8, assign88200_e134424_d_n9, assign88200_e134424_d_n10, assign88200_e134424_d_n11, assign88200_e134424_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2046 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign88200_e134424;
        locals.var_t9_dn0 = assign88200_e134424_d_n0;
        locals.var_t9_dn2 = assign88200_e134424_d_n2;
        locals.var_t9_dn4 = assign88200_e134424_d_n4;
        locals.var_t9_dn5 = assign88200_e134424_d_n5;
        locals.var_t9_dn6 = assign88200_e134424_d_n6;
        locals.var_t9_dn7 = assign88200_e134424_d_n7;
        locals.var_t9_dn8 = assign88200_e134424_d_n8;
        locals.var_t9_dn9 = assign88200_e134424_d_n9;
        locals.var_t9_dn10 = assign88200_e134424_d_n10;
        locals.var_t9_dn11 = assign88200_e134424_d_n11;
        locals.var_t9_dn14 = assign88200_e134424_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign88210_e134433, assign88210_e134433_d_n0, assign88210_e134433_d_n2, assign88210_e134433_d_n4, assign88210_e134433_d_n5, assign88210_e134433_d_n6, assign88210_e134433_d_n7, assign88210_e134433_d_n8, assign88210_e134433_d_n9, assign88210_e134433_d_n10, assign88210_e134433_d_n11, assign88210_e134433_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign88210_e134433;
        locals.var_ddriftldc_dn0 = assign88210_e134433_d_n0;
        locals.var_ddriftldc_dn2 = assign88210_e134433_d_n2;
        locals.var_ddriftldc_dn4 = assign88210_e134433_d_n4;
        locals.var_ddriftldc_dn5 = assign88210_e134433_d_n5;
        locals.var_ddriftldc_dn6 = assign88210_e134433_d_n6;
        locals.var_ddriftldc_dn7 = assign88210_e134433_d_n7;
        locals.var_ddriftldc_dn8 = assign88210_e134433_d_n8;
        locals.var_ddriftldc_dn9 = assign88210_e134433_d_n9;
        locals.var_ddriftldc_dn10 = assign88210_e134433_d_n10;
        locals.var_ddriftldc_dn11 = assign88210_e134433_d_n11;
        locals.var_ddriftldc_dn14 = assign88210_e134433_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign88220_e134450, assign88220_e134450_d_n0, assign88220_e134450_d_n2, assign88220_e134450_d_n4, assign88220_e134450_d_n5, assign88220_e134450_d_n6, assign88220_e134450_d_n7, assign88220_e134450_d_n8, assign88220_e134450_d_n9, assign88220_e134450_d_n10, assign88220_e134450_d_n11, assign88220_e134450_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88220_e134442: f64 = (locals.var_q_nsubld__blk2006 * locals.var_ddriftldc);
        let assign88220_e134444: f64 = (assign88220_e134442 * locals.var_ddriftldc);
        let assign88220_e134446: f64 = (assign88220_e134444 / 2.0);
        let assign88220_e134448: f64 = (assign88220_e134446 / 1.034943e-10);
        (assign88220_e134448, (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign88220_e134442 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign88220_e134450;
        locals.var_dphi_sb_dn0 = assign88220_e134450_d_n0;
        locals.var_dphi_sb_dn2 = assign88220_e134450_d_n2;
        locals.var_dphi_sb_dn4 = assign88220_e134450_d_n4;
        locals.var_dphi_sb_dn5 = assign88220_e134450_d_n5;
        locals.var_dphi_sb_dn6 = assign88220_e134450_d_n6;
        locals.var_dphi_sb_dn7 = assign88220_e134450_d_n7;
        locals.var_dphi_sb_dn8 = assign88220_e134450_d_n8;
        locals.var_dphi_sb_dn9 = assign88220_e134450_d_n9;
        locals.var_dphi_sb_dn10 = assign88220_e134450_d_n10;
        locals.var_dphi_sb_dn11 = assign88220_e134450_d_n11;
        locals.var_dphi_sb_dn14 = assign88220_e134450_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign88230_e134464, assign88230_e134464_d_n0, assign88230_e134464_d_n2, assign88230_e134464_d_n4, assign88230_e134464_d_n5, assign88230_e134464_d_n6, assign88230_e134464_d_n7, assign88230_e134464_d_n8, assign88230_e134464_d_n9, assign88230_e134464_d_n10, assign88230_e134464_d_n11, assign88230_e134464_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88230_e134459: f64 = (2.0 * locals.var_beta);
        let assign88230_e134461: f64 = (assign88230_e134459 * locals.var_dphi_sb);
        let assign88230_e134462: f64 = (assign88230_e134461).sqrt();
        (assign88230_e134462, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn0)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn2)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn4)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn5)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn6)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn7)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn8)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn9)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn10)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn11)) / (2.0 * assign88230_e134462)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign88230_e134459 * locals.var_dphi_sb_dn14)) / (2.0 * assign88230_e134462)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign88230_e134464;
        locals.var_t0_dn0 = assign88230_e134464_d_n0;
        locals.var_t0_dn2 = assign88230_e134464_d_n2;
        locals.var_t0_dn4 = assign88230_e134464_d_n4;
        locals.var_t0_dn5 = assign88230_e134464_d_n5;
        locals.var_t0_dn6 = assign88230_e134464_d_n6;
        locals.var_t0_dn7 = assign88230_e134464_d_n7;
        locals.var_t0_dn8 = assign88230_e134464_d_n8;
        locals.var_t0_dn9 = assign88230_e134464_d_n9;
        locals.var_t0_dn10 = assign88230_e134464_d_n10;
        locals.var_t0_dn11 = assign88230_e134464_d_n11;
        locals.var_t0_dn14 = assign88230_e134464_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign88240_e134480, assign88240_e134480_d_n0, assign88240_e134480_d_n2, assign88240_e134480_d_n4, assign88240_e134480_d_n5, assign88240_e134480_d_n6, assign88240_e134480_d_n7, assign88240_e134480_d_n8, assign88240_e134480_d_n9, assign88240_e134480_d_n10, assign88240_e134480_d_n11, assign88240_e134480_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88240_e134472: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign88240_e134474: f64 = (-locals.var_t0);
        let assign88240_e134475: f64 = { let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign88240_e134476: f64 = (assign88240_e134472 + assign88240_e134475);
        let assign88240_e134478: f64 = (assign88240_e134476 / 2.0);
        (assign88240_e134478, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign88240_e134474; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign88240_e134480;
        locals.var_t1_dn0 = assign88240_e134480_d_n0;
        locals.var_t1_dn2 = assign88240_e134480_d_n2;
        locals.var_t1_dn4 = assign88240_e134480_d_n4;
        locals.var_t1_dn5 = assign88240_e134480_d_n5;
        locals.var_t1_dn6 = assign88240_e134480_d_n6;
        locals.var_t1_dn7 = assign88240_e134480_d_n7;
        locals.var_t1_dn8 = assign88240_e134480_d_n8;
        locals.var_t1_dn9 = assign88240_e134480_d_n9;
        locals.var_t1_dn10 = assign88240_e134480_d_n10;
        locals.var_t1_dn11 = assign88240_e134480_d_n11;
        locals.var_t1_dn14 = assign88240_e134480_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign88250_e134492, assign88250_e134492_d_n0, assign88250_e134492_d_n2, assign88250_e134492_d_n4, assign88250_e134492_d_n5, assign88250_e134492_d_n6, assign88250_e134492_d_n7, assign88250_e134492_d_n8, assign88250_e134492_d_n9, assign88250_e134492_d_n10, assign88250_e134492_d_n11, assign88250_e134492_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88250_e134488: f64 = (locals.var_t1).ln();
        let assign88250_e134490: f64 = (assign88250_e134488 / locals.var_dphi_sb);
        (assign88250_e134490, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign88250_e134488 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign88250_e134492;
        locals.var_c_sb_dn0 = assign88250_e134492_d_n0;
        locals.var_c_sb_dn2 = assign88250_e134492_d_n2;
        locals.var_c_sb_dn4 = assign88250_e134492_d_n4;
        locals.var_c_sb_dn5 = assign88250_e134492_d_n5;
        locals.var_c_sb_dn6 = assign88250_e134492_d_n6;
        locals.var_c_sb_dn7 = assign88250_e134492_d_n7;
        locals.var_c_sb_dn8 = assign88250_e134492_d_n8;
        locals.var_c_sb_dn9 = assign88250_e134492_d_n9;
        locals.var_c_sb_dn10 = assign88250_e134492_d_n10;
        locals.var_c_sb_dn11 = assign88250_e134492_d_n11;
        locals.var_c_sb_dn14 = assign88250_e134492_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign88260_e134503, assign88260_e134503_d_n0, assign88260_e134503_d_n2, assign88260_e134503_d_n4, assign88260_e134503_d_n5, assign88260_e134503_d_n6, assign88260_e134503_d_n7, assign88260_e134503_d_n8, assign88260_e134503_d_n9, assign88260_e134503_d_n10, assign88260_e134503_d_n11, assign88260_e134503_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88260_e134501: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign88260_e134501, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
        locals.var_ps0ld_vxb = assign88260_e134503;
        locals.var_ps0ld_vxb_dn0 = assign88260_e134503_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign88260_e134503_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign88260_e134503_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign88260_e134503_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign88260_e134503_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign88260_e134503_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign88260_e134503_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign88260_e134503_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign88260_e134503_d_n10;
        locals.var_ps0ld_vxb_dn11 = assign88260_e134503_d_n11;
        locals.var_ps0ld_vxb_dn14 = assign88260_e134503_d_n14;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign88270_e134516, assign88270_e134516_d_n0, assign88270_e134516_d_n2, assign88270_e134516_d_n4, assign88270_e134516_d_n5, assign88270_e134516_d_n6, assign88270_e134516_d_n7, assign88270_e134516_d_n8, assign88270_e134516_d_n9, assign88270_e134516_d_n10, assign88270_e134516_d_n11, assign88270_e134516_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88270_e134513: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign88270_e134514: f64 = (locals.var_c_sb * assign88270_e134513);
        (assign88270_e134514, ((locals.var_c_sb_dn0 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign88270_e134513) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign88270_e134516;
        locals.var_ty_dn0 = assign88270_e134516_d_n0;
        locals.var_ty_dn2 = assign88270_e134516_d_n2;
        locals.var_ty_dn4 = assign88270_e134516_d_n4;
        locals.var_ty_dn5 = assign88270_e134516_d_n5;
        locals.var_ty_dn6 = assign88270_e134516_d_n6;
        locals.var_ty_dn7 = assign88270_e134516_d_n7;
        locals.var_ty_dn8 = assign88270_e134516_d_n8;
        locals.var_ty_dn9 = assign88270_e134516_d_n9;
        locals.var_ty_dn10 = assign88270_e134516_d_n10;
        locals.var_ty_dn11 = assign88270_e134516_d_n11;
        locals.var_ty_dn14 = assign88270_e134516_d_n14;
        locals.var_ty_rv = 0.0;

        let assign88280_e134519: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard2047 = assign88280_e134519;
        locals.var_guard2047_rv = 0.0;

        let (assign88290_e134531, assign88290_e134531_d_n0, assign88290_e134531_d_n2, assign88290_e134531_d_n4, assign88290_e134531_d_n5, assign88290_e134531_d_n6, assign88290_e134531_d_n7, assign88290_e134531_d_n8, assign88290_e134531_d_n9, assign88290_e134531_d_n10, assign88290_e134531_d_n11, assign88290_e134531_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2047 != 0.0)) {
        let assign88290_e134529: f64 = (locals.var_ty).exp();
        (assign88290_e134529, (assign88290_e134529 * locals.var_ty_dn0), (assign88290_e134529 * locals.var_ty_dn2), (assign88290_e134529 * locals.var_ty_dn4), (assign88290_e134529 * locals.var_ty_dn5), (assign88290_e134529 * locals.var_ty_dn6), (assign88290_e134529 * locals.var_ty_dn7), (assign88290_e134529 * locals.var_ty_dn8), (assign88290_e134529 * locals.var_ty_dn9), (assign88290_e134529 * locals.var_ty_dn10), (assign88290_e134529 * locals.var_ty_dn11), (assign88290_e134529 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign88290_e134531;
        locals.var_t1_dn0 = assign88290_e134531_d_n0;
        locals.var_t1_dn2 = assign88290_e134531_d_n2;
        locals.var_t1_dn4 = assign88290_e134531_d_n4;
        locals.var_t1_dn5 = assign88290_e134531_d_n5;
        locals.var_t1_dn6 = assign88290_e134531_d_n6;
        locals.var_t1_dn7 = assign88290_e134531_d_n7;
        locals.var_t1_dn8 = assign88290_e134531_d_n8;
        locals.var_t1_dn9 = assign88290_e134531_d_n9;
        locals.var_t1_dn10 = assign88290_e134531_d_n10;
        locals.var_t1_dn11 = assign88290_e134531_d_n11;
        locals.var_t1_dn14 = assign88290_e134531_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign88300_e134546, assign88300_e134546_d_n0, assign88300_e134546_d_n2, assign88300_e134546_d_n4, assign88300_e134546_d_n5, assign88300_e134546_d_n6, assign88300_e134546_d_n7, assign88300_e134546_d_n8, assign88300_e134546_d_n9, assign88300_e134546_d_n10, assign88300_e134546_d_n11, assign88300_e134546_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2047 != 0.0)) {
        let assign88300_e134541: f64 = (-locals.var_c_sb);
        let assign88300_e134543: f64 = (assign88300_e134541 * locals.var_dphi_sb);
        let assign88300_e134544: f64 = (assign88300_e134543).exp();
        (assign88300_e134544, (assign88300_e134544 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn0))), (assign88300_e134544 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn2))), (assign88300_e134544 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn4))), (assign88300_e134544 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn5))), (assign88300_e134544 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn6))), (assign88300_e134544 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn7))), (assign88300_e134544 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn8))), (assign88300_e134544 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn9))), (assign88300_e134544 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn10))), (assign88300_e134544 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn11))), (assign88300_e134544 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign88300_e134541 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign88300_e134546;
        locals.var_t0_dn0 = assign88300_e134546_d_n0;
        locals.var_t0_dn2 = assign88300_e134546_d_n2;
        locals.var_t0_dn4 = assign88300_e134546_d_n4;
        locals.var_t0_dn5 = assign88300_e134546_d_n5;
        locals.var_t0_dn6 = assign88300_e134546_d_n6;
        locals.var_t0_dn7 = assign88300_e134546_d_n7;
        locals.var_t0_dn8 = assign88300_e134546_d_n8;
        locals.var_t0_dn9 = assign88300_e134546_d_n9;
        locals.var_t0_dn10 = assign88300_e134546_d_n10;
        locals.var_t0_dn11 = assign88300_e134546_d_n11;
        locals.var_t0_dn14 = assign88300_e134546_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign88310_e134559, assign88310_e134559_d_n0, assign88310_e134559_d_n2, assign88310_e134559_d_n4, assign88310_e134559_d_n5, assign88310_e134559_d_n6, assign88310_e134559_d_n7, assign88310_e134559_d_n8, assign88310_e134559_d_n9, assign88310_e134559_d_n10, assign88310_e134559_d_n11, assign88310_e134559_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2047 != 0.0)) {
        let assign88310_e134557: f64 = (locals.var_t1 - locals.var_t0);
        (assign88310_e134557, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88310_e134559;
        locals.var_t2_dn0 = assign88310_e134559_d_n0;
        locals.var_t2_dn2 = assign88310_e134559_d_n2;
        locals.var_t2_dn4 = assign88310_e134559_d_n4;
        locals.var_t2_dn5 = assign88310_e134559_d_n5;
        locals.var_t2_dn6 = assign88310_e134559_d_n6;
        locals.var_t2_dn7 = assign88310_e134559_d_n7;
        locals.var_t2_dn8 = assign88310_e134559_d_n8;
        locals.var_t2_dn9 = assign88310_e134559_d_n9;
        locals.var_t2_dn10 = assign88310_e134559_d_n10;
        locals.var_t2_dn11 = assign88310_e134559_d_n11;
        locals.var_t2_dn14 = assign88310_e134559_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign88320_e134575, assign88320_e134575_d_n0, assign88320_e134575_d_n2, assign88320_e134575_d_n4, assign88320_e134575_d_n5, assign88320_e134575_d_n6, assign88320_e134575_d_n7, assign88320_e134575_d_n8, assign88320_e134575_d_n9, assign88320_e134575_d_n10, assign88320_e134575_d_n11, assign88320_e134575_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2047 != 0.0)) {
        let assign88320_e134570: f64 = (1.0 + locals.var_t2);
        let assign88320_e134571: f64 = (assign88320_e134570).ln();
        let assign88320_e134573: f64 = (assign88320_e134571 / locals.var_c_sb);
        (assign88320_e134573, ((((locals.var_t2_dn0 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign88320_e134570) * locals.var_c_sb) - (assign88320_e134571 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign88320_e134575;
        locals.var_phi_b_dn0 = assign88320_e134575_d_n0;
        locals.var_phi_b_dn2 = assign88320_e134575_d_n2;
        locals.var_phi_b_dn4 = assign88320_e134575_d_n4;
        locals.var_phi_b_dn5 = assign88320_e134575_d_n5;
        locals.var_phi_b_dn6 = assign88320_e134575_d_n6;
        locals.var_phi_b_dn7 = assign88320_e134575_d_n7;
        locals.var_phi_b_dn8 = assign88320_e134575_d_n8;
        locals.var_phi_b_dn9 = assign88320_e134575_d_n9;
        locals.var_phi_b_dn10 = assign88320_e134575_d_n10;
        locals.var_phi_b_dn11 = assign88320_e134575_d_n11;
        locals.var_phi_b_dn14 = assign88320_e134575_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign88330_e134589, assign88330_e134589_d_n0, assign88330_e134589_d_n2, assign88330_e134589_d_n4, assign88330_e134589_d_n5, assign88330_e134589_d_n6, assign88330_e134589_d_n7, assign88330_e134589_d_n8, assign88330_e134589_d_n9, assign88330_e134589_d_n10, assign88330_e134589_d_n11, assign88330_e134589_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2047 == 0.0)) {
        let assign88330_e134587: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign88330_e134587, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign88330_e134589;
        locals.var_phi_b_dn0 = assign88330_e134589_d_n0;
        locals.var_phi_b_dn2 = assign88330_e134589_d_n2;
        locals.var_phi_b_dn4 = assign88330_e134589_d_n4;
        locals.var_phi_b_dn5 = assign88330_e134589_d_n5;
        locals.var_phi_b_dn6 = assign88330_e134589_d_n6;
        locals.var_phi_b_dn7 = assign88330_e134589_d_n7;
        locals.var_phi_b_dn8 = assign88330_e134589_d_n8;
        locals.var_phi_b_dn9 = assign88330_e134589_d_n9;
        locals.var_phi_b_dn10 = assign88330_e134589_d_n10;
        locals.var_phi_b_dn11 = assign88330_e134589_d_n11;
        locals.var_phi_b_dn14 = assign88330_e134589_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign88340_e134600, assign88340_e134600_d_n0, assign88340_e134600_d_n2, assign88340_e134600_d_n4, assign88340_e134600_d_n5, assign88340_e134600_d_n6, assign88340_e134600_d_n7, assign88340_e134600_d_n8, assign88340_e134600_d_n9, assign88340_e134600_d_n10, assign88340_e134600_d_n11, assign88340_e134600_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        let assign88340_e134598: f64 = (locals.var_beta * locals.var_phi_b);
        (assign88340_e134598, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
        locals.var_chib = assign88340_e134600;
        locals.var_chib_dn0 = assign88340_e134600_d_n0;
        locals.var_chib_dn2 = assign88340_e134600_d_n2;
        locals.var_chib_dn4 = assign88340_e134600_d_n4;
        locals.var_chib_dn5 = assign88340_e134600_d_n5;
        locals.var_chib_dn6 = assign88340_e134600_d_n6;
        locals.var_chib_dn7 = assign88340_e134600_d_n7;
        locals.var_chib_dn8 = assign88340_e134600_d_n8;
        locals.var_chib_dn9 = assign88340_e134600_d_n9;
        locals.var_chib_dn10 = assign88340_e134600_d_n10;
        locals.var_chib_dn11 = assign88340_e134600_d_n11;
        locals.var_chib_dn14 = assign88340_e134600_d_n14;
        locals.var_chib_rv = 0.0;

        let assign88350_e134604: f64 = (locals.var_chi / 100.0);
        let assign88350_e134609: f64 = if ((locals.var_chib > assign88350_e134604) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2048 = assign88350_e134609;
        locals.var_guard2048_rv = 0.0;

        let (assign88360_e134622,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2048 != 0.0)) {
        let assign88360_e134620: f64 = (locals.var_flg_fd_mode__blk2012 + 1.0);
        (assign88360_e134620,)
    } else {
        (locals.var_flg_fd_mode__blk2012,)
    }
};
        locals.var_flg_fd_mode__blk2012 = assign88360_e134622;
        locals.var_flg_fd_mode__blk2012_rv = 0.0;

        let (assign88370_e134633, assign88370_e134633_d_n0, assign88370_e134633_d_n2, assign88370_e134633_d_n4, assign88370_e134633_d_n5, assign88370_e134633_d_n6, assign88370_e134633_d_n7, assign88370_e134633_d_n8, assign88370_e134633_d_n9, assign88370_e134633_d_n10, assign88370_e134633_d_n11, assign88370_e134633_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2043 != 0.0)) && (locals.var_guard2048 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign88370_e134633;
        locals.var_chi_dn0 = assign88370_e134633_d_n0;
        locals.var_chi_dn2 = assign88370_e134633_d_n2;
        locals.var_chi_dn4 = assign88370_e134633_d_n4;
        locals.var_chi_dn5 = assign88370_e134633_d_n5;
        locals.var_chi_dn6 = assign88370_e134633_d_n6;
        locals.var_chi_dn7 = assign88370_e134633_d_n7;
        locals.var_chi_dn8 = assign88370_e134633_d_n8;
        locals.var_chi_dn9 = assign88370_e134633_d_n9;
        locals.var_chi_dn10 = assign88370_e134633_d_n10;
        locals.var_chi_dn11 = assign88370_e134633_d_n11;
        locals.var_chi_dn14 = assign88370_e134633_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign88380_e134644, assign88380_e134644_d_n0, assign88380_e134644_d_n2, assign88380_e134644_d_n4, assign88380_e134644_d_n5, assign88380_e134644_d_n6, assign88380_e134644_d_n7, assign88380_e134644_d_n8, assign88380_e134644_d_n9, assign88380_e134644_d_n10, assign88380_e134644_d_n11, assign88380_e134644_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) {
        let assign88380_e134640: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign88380_e134642: f64 = (assign88380_e134640 - locals.var_vxbgmtcl);
        (assign88380_e134642, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign88380_e134644;
        locals.var_ps0ld_dn0 = assign88380_e134644_d_n0;
        locals.var_ps0ld_dn2 = assign88380_e134644_d_n2;
        locals.var_ps0ld_dn4 = assign88380_e134644_d_n4;
        locals.var_ps0ld_dn5 = assign88380_e134644_d_n5;
        locals.var_ps0ld_dn6 = assign88380_e134644_d_n6;
        locals.var_ps0ld_dn7 = assign88380_e134644_d_n7;
        locals.var_ps0ld_dn8 = assign88380_e134644_d_n8;
        locals.var_ps0ld_dn9 = assign88380_e134644_d_n9;
        locals.var_ps0ld_dn10 = assign88380_e134644_d_n10;
        locals.var_ps0ld_dn11 = assign88380_e134644_d_n11;
        locals.var_ps0ld_dn14 = assign88380_e134644_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign88390_e134646: f64 = (locals.var_chi).abs();
        let assign88390_e134648: f64 = if assign88390_e134646 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard2049 = assign88390_e134648;
        locals.var_guard2049_rv = 0.0;

        let (assign88400_e134663, assign88400_e134663_d_n0, assign88400_e134663_d_n2, assign88400_e134663_d_n4, assign88400_e134663_d_n5, assign88400_e134663_d_n6, assign88400_e134663_d_n7, assign88400_e134663_d_n8, assign88400_e134663_d_n9, assign88400_e134663_d_n10, assign88400_e134663_d_n11, assign88400_e134663_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2049 != 0.0)) {
        let assign88400_e134657: f64 = (locals.var_chi - 1.0);
        let assign88400_e134659: f64 = (-locals.var_chi);
        let assign88400_e134660: f64 = (assign88400_e134659).exp();
        let assign88400_e134661: f64 = (assign88400_e134657 + assign88400_e134660);
        (assign88400_e134661, (locals.var_chi_dn0 + (assign88400_e134660 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign88400_e134660 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign88400_e134660 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign88400_e134660 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign88400_e134660 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign88400_e134660 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign88400_e134660 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign88400_e134660 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign88400_e134660 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign88400_e134660 * (-locals.var_chi_dn11))), (locals.var_chi_dn14 + (assign88400_e134660 * (-locals.var_chi_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign88400_e134663;
        locals.var_t1_dn0 = assign88400_e134663_d_n0;
        locals.var_t1_dn2 = assign88400_e134663_d_n2;
        locals.var_t1_dn4 = assign88400_e134663_d_n4;
        locals.var_t1_dn5 = assign88400_e134663_d_n5;
        locals.var_t1_dn6 = assign88400_e134663_d_n6;
        locals.var_t1_dn7 = assign88400_e134663_d_n7;
        locals.var_t1_dn8 = assign88400_e134663_d_n8;
        locals.var_t1_dn9 = assign88400_e134663_d_n9;
        locals.var_t1_dn10 = assign88400_e134663_d_n10;
        locals.var_t1_dn11 = assign88400_e134663_d_n11;
        locals.var_t1_dn14 = assign88400_e134663_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign88410_e134673, assign88410_e134673_d_n0, assign88410_e134673_d_n2, assign88410_e134673_d_n4, assign88410_e134673_d_n5, assign88410_e134673_d_n6, assign88410_e134673_d_n7, assign88410_e134673_d_n8, assign88410_e134673_d_n9, assign88410_e134673_d_n10, assign88410_e134673_d_n11, assign88410_e134673_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2049 != 0.0)) {
        let assign88410_e134671: f64 = (locals.var_t1).sqrt();
        (assign88410_e134671, (locals.var_t1_dn0 / (2.0 * assign88410_e134671)), (locals.var_t1_dn2 / (2.0 * assign88410_e134671)), (locals.var_t1_dn4 / (2.0 * assign88410_e134671)), (locals.var_t1_dn5 / (2.0 * assign88410_e134671)), (locals.var_t1_dn6 / (2.0 * assign88410_e134671)), (locals.var_t1_dn7 / (2.0 * assign88410_e134671)), (locals.var_t1_dn8 / (2.0 * assign88410_e134671)), (locals.var_t1_dn9 / (2.0 * assign88410_e134671)), (locals.var_t1_dn10 / (2.0 * assign88410_e134671)), (locals.var_t1_dn11 / (2.0 * assign88410_e134671)), (locals.var_t1_dn14 / (2.0 * assign88410_e134671)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88410_e134673;
        locals.var_t2_dn0 = assign88410_e134673_d_n0;
        locals.var_t2_dn2 = assign88410_e134673_d_n2;
        locals.var_t2_dn4 = assign88410_e134673_d_n4;
        locals.var_t2_dn5 = assign88410_e134673_d_n5;
        locals.var_t2_dn6 = assign88410_e134673_d_n6;
        locals.var_t2_dn7 = assign88410_e134673_d_n7;
        locals.var_t2_dn8 = assign88410_e134673_d_n8;
        locals.var_t2_dn9 = assign88410_e134673_d_n9;
        locals.var_t2_dn10 = assign88410_e134673_d_n10;
        locals.var_t2_dn11 = assign88410_e134673_d_n11;
        locals.var_t2_dn14 = assign88410_e134673_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign88430_e134704, assign88430_e134704_d_n0, assign88430_e134704_d_n2, assign88430_e134704_d_n4, assign88430_e134704_d_n5, assign88430_e134704_d_n6, assign88430_e134704_d_n7, assign88430_e134704_d_n8, assign88430_e134704_d_n9, assign88430_e134704_d_n10, assign88430_e134704_d_n11, assign88430_e134704_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2049 == 0.0)) {
        let assign88430_e134695: f64 = (0.7071067811865475 * locals.var_chi);
        let assign88430_e134699: f64 = (locals.var_chi * 0.3333333333333333);
        let assign88430_e134700: f64 = (1.0 - assign88430_e134699);
        let assign88430_e134701: f64 = (assign88430_e134700).sqrt();
        let assign88430_e134702: f64 = (assign88430_e134695 * assign88430_e134701);
        (assign88430_e134702, (((0.7071067811865475 * locals.var_chi_dn0) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn11) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn11 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))), (((0.7071067811865475 * locals.var_chi_dn14) * assign88430_e134701) + (assign88430_e134695 * ((-(locals.var_chi_dn14 * 0.3333333333333333)) / (2.0 * assign88430_e134701)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign88430_e134704;
        locals.var_t2_dn0 = assign88430_e134704_d_n0;
        locals.var_t2_dn2 = assign88430_e134704_d_n2;
        locals.var_t2_dn4 = assign88430_e134704_d_n4;
        locals.var_t2_dn5 = assign88430_e134704_d_n5;
        locals.var_t2_dn6 = assign88430_e134704_d_n6;
        locals.var_t2_dn7 = assign88430_e134704_d_n7;
        locals.var_t2_dn8 = assign88430_e134704_d_n8;
        locals.var_t2_dn9 = assign88430_e134704_d_n9;
        locals.var_t2_dn10 = assign88430_e134704_d_n10;
        locals.var_t2_dn11 = assign88430_e134704_d_n11;
        locals.var_t2_dn14 = assign88430_e134704_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_339(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign88440_e134713, assign88440_e134713_d_n0, assign88440_e134713_d_n2, assign88440_e134713_d_n4, assign88440_e134713_d_n5, assign88440_e134713_d_n6, assign88440_e134713_d_n7, assign88440_e134713_d_n8, assign88440_e134713_d_n9, assign88440_e134713_d_n10, assign88440_e134713_d_n11, assign88440_e134713_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) {
        let assign88440_e134711: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign88440_e134711, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign88440_e134713;
        locals.var_qbuld_dn0 = assign88440_e134713_d_n0;
        locals.var_qbuld_dn2 = assign88440_e134713_d_n2;
        locals.var_qbuld_dn4 = assign88440_e134713_d_n4;
        locals.var_qbuld_dn5 = assign88440_e134713_d_n5;
        locals.var_qbuld_dn6 = assign88440_e134713_d_n6;
        locals.var_qbuld_dn7 = assign88440_e134713_d_n7;
        locals.var_qbuld_dn8 = assign88440_e134713_d_n8;
        locals.var_qbuld_dn9 = assign88440_e134713_d_n9;
        locals.var_qbuld_dn10 = assign88440_e134713_d_n10;
        locals.var_qbuld_dn11 = assign88440_e134713_d_n11;
        locals.var_qbuld_dn14 = assign88440_e134713_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign88450_e134724, assign88450_e134724_d_n0, assign88450_e134724_d_n2, assign88450_e134724_d_n4, assign88450_e134724_d_n5, assign88450_e134724_d_n6, assign88450_e134724_d_n7, assign88450_e134724_d_n8, assign88450_e134724_d_n9, assign88450_e134724_d_n10, assign88450_e134724_d_n11, assign88450_e134724_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) {
        let assign88450_e134721: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign88450_e134722: f64 = (locals.var_cox0_func * assign88450_e134721);
        (assign88450_e134722, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (-locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn11)), (locals.var_cox0_func * (-locals.var_ps0ld_dn14)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign88450_e134724;
        locals.var_qsuld_dn0 = assign88450_e134724_d_n0;
        locals.var_qsuld_dn2 = assign88450_e134724_d_n2;
        locals.var_qsuld_dn4 = assign88450_e134724_d_n4;
        locals.var_qsuld_dn5 = assign88450_e134724_d_n5;
        locals.var_qsuld_dn6 = assign88450_e134724_d_n6;
        locals.var_qsuld_dn7 = assign88450_e134724_d_n7;
        locals.var_qsuld_dn8 = assign88450_e134724_d_n8;
        locals.var_qsuld_dn9 = assign88450_e134724_d_n9;
        locals.var_qsuld_dn10 = assign88450_e134724_d_n10;
        locals.var_qsuld_dn11 = assign88450_e134724_d_n11;
        locals.var_qsuld_dn14 = assign88450_e134724_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign88460_e134733, assign88460_e134733_d_n0, assign88460_e134733_d_n2, assign88460_e134733_d_n4, assign88460_e134733_d_n5, assign88460_e134733_d_n6, assign88460_e134733_d_n7, assign88460_e134733_d_n8, assign88460_e134733_d_n9, assign88460_e134733_d_n10, assign88460_e134733_d_n11, assign88460_e134733_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) {
        let assign88460_e134731: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk2006);
        (assign88460_e134731, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn11 / locals.var_q_nsubld__blk2006), (locals.var_qbuld_dn14 / locals.var_q_nsubld__blk2006),)
    } else {
        (locals.var_wdld0__blk2050, locals.var_wdld0__blk2050_dn0, locals.var_wdld0__blk2050_dn2, locals.var_wdld0__blk2050_dn4, locals.var_wdld0__blk2050_dn5, locals.var_wdld0__blk2050_dn6, locals.var_wdld0__blk2050_dn7, locals.var_wdld0__blk2050_dn8, locals.var_wdld0__blk2050_dn9, locals.var_wdld0__blk2050_dn10, locals.var_wdld0__blk2050_dn11, locals.var_wdld0__blk2050_dn14,)
    }
};
        locals.var_wdld0__blk2050 = assign88460_e134733;
        locals.var_wdld0__blk2050_dn0 = assign88460_e134733_d_n0;
        locals.var_wdld0__blk2050_dn2 = assign88460_e134733_d_n2;
        locals.var_wdld0__blk2050_dn4 = assign88460_e134733_d_n4;
        locals.var_wdld0__blk2050_dn5 = assign88460_e134733_d_n5;
        locals.var_wdld0__blk2050_dn6 = assign88460_e134733_d_n6;
        locals.var_wdld0__blk2050_dn7 = assign88460_e134733_d_n7;
        locals.var_wdld0__blk2050_dn8 = assign88460_e134733_d_n8;
        locals.var_wdld0__blk2050_dn9 = assign88460_e134733_d_n9;
        locals.var_wdld0__blk2050_dn10 = assign88460_e134733_d_n10;
        locals.var_wdld0__blk2050_dn11 = assign88460_e134733_d_n11;
        locals.var_wdld0__blk2050_dn14 = assign88460_e134733_d_n14;
        locals.var_wdld0__blk2050_rv = 0.0;

        let assign88470_e134736: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2052 = assign88470_e134736;
        locals.var_guard2052_rv = 0.0;

        let assign88480_e134741: f64 = (locals.var_ddriftldc * 0.1);
        let assign88480_e134742: f64 = (locals.var_ddriftldc - assign88480_e134741);
        let assign88480_e134746: f64 = (locals.var_ddriftldc * 0.1);
        let assign88480_e134749: f64 = if ((locals.var_wdld0__blk2050 > assign88480_e134742) && (assign88480_e134746 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2053 = assign88480_e134749;
        locals.var_guard2053_rv = 0.0;

        let (assign88490_e134766, assign88490_e134766_d_n0, assign88490_e134766_d_n2, assign88490_e134766_d_n4, assign88490_e134766_d_n5, assign88490_e134766_d_n6, assign88490_e134766_d_n7, assign88490_e134766_d_n8, assign88490_e134766_d_n9, assign88490_e134766_d_n10, assign88490_e134766_d_n11, assign88490_e134766_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88490_e134760: f64 = (locals.var_wdld0__blk2050 - locals.var_ddriftldc);
        let assign88490_e134763: f64 = (locals.var_ddriftldc * 0.1);
        let assign88490_e134764: f64 = (assign88490_e134760 + assign88490_e134763);
        (assign88490_e134764, ((locals.var_wdld0__blk2050_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk2050_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk2050_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk2050_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk2050_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk2050_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk2050_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk2050_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk2050_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk2050_dn11 - locals.var_ddriftldc_dn11) + (locals.var_ddriftldc_dn11 * 0.1)), ((locals.var_wdld0__blk2050_dn14 - locals.var_ddriftldc_dn14) + (locals.var_ddriftldc_dn14 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign88490_e134766;
        locals.var_tmf1_dn0 = assign88490_e134766_d_n0;
        locals.var_tmf1_dn2 = assign88490_e134766_d_n2;
        locals.var_tmf1_dn4 = assign88490_e134766_d_n4;
        locals.var_tmf1_dn5 = assign88490_e134766_d_n5;
        locals.var_tmf1_dn6 = assign88490_e134766_d_n6;
        locals.var_tmf1_dn7 = assign88490_e134766_d_n7;
        locals.var_tmf1_dn8 = assign88490_e134766_d_n8;
        locals.var_tmf1_dn9 = assign88490_e134766_d_n9;
        locals.var_tmf1_dn10 = assign88490_e134766_d_n10;
        locals.var_tmf1_dn11 = assign88490_e134766_d_n11;
        locals.var_tmf1_dn14 = assign88490_e134766_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign88500_e134779, assign88500_e134779_d_n0, assign88500_e134779_d_n2, assign88500_e134779_d_n4, assign88500_e134779_d_n5, assign88500_e134779_d_n6, assign88500_e134779_d_n7, assign88500_e134779_d_n8, assign88500_e134779_d_n9, assign88500_e134779_d_n10, assign88500_e134779_d_n11, assign88500_e134779_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88500_e134777: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign88500_e134777, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign88500_e134779;
        locals.var_x2_dn0 = assign88500_e134779_d_n0;
        locals.var_x2_dn2 = assign88500_e134779_d_n2;
        locals.var_x2_dn4 = assign88500_e134779_d_n4;
        locals.var_x2_dn5 = assign88500_e134779_d_n5;
        locals.var_x2_dn6 = assign88500_e134779_d_n6;
        locals.var_x2_dn7 = assign88500_e134779_d_n7;
        locals.var_x2_dn8 = assign88500_e134779_d_n8;
        locals.var_x2_dn9 = assign88500_e134779_d_n9;
        locals.var_x2_dn10 = assign88500_e134779_d_n10;
        locals.var_x2_dn11 = assign88500_e134779_d_n11;
        locals.var_x2_dn14 = assign88500_e134779_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign88510_e134796, assign88510_e134796_d_n0, assign88510_e134796_d_n2, assign88510_e134796_d_n4, assign88510_e134796_d_n5, assign88510_e134796_d_n6, assign88510_e134796_d_n7, assign88510_e134796_d_n8, assign88510_e134796_d_n9, assign88510_e134796_d_n10, assign88510_e134796_d_n11, assign88510_e134796_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88510_e134790: f64 = (locals.var_ddriftldc * 0.1);
        let assign88510_e134793: f64 = (locals.var_ddriftldc * 0.1);
        let assign88510_e134794: f64 = (assign88510_e134790 * assign88510_e134793);
        (assign88510_e134794, (((locals.var_ddriftldc_dn0 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn11 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn11 * 0.1))), (((locals.var_ddriftldc_dn14 * 0.1) * assign88510_e134793) + (assign88510_e134790 * (locals.var_ddriftldc_dn14 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign88510_e134796;
        locals.var_xmax2_dn0 = assign88510_e134796_d_n0;
        locals.var_xmax2_dn2 = assign88510_e134796_d_n2;
        locals.var_xmax2_dn4 = assign88510_e134796_d_n4;
        locals.var_xmax2_dn5 = assign88510_e134796_d_n5;
        locals.var_xmax2_dn6 = assign88510_e134796_d_n6;
        locals.var_xmax2_dn7 = assign88510_e134796_d_n7;
        locals.var_xmax2_dn8 = assign88510_e134796_d_n8;
        locals.var_xmax2_dn9 = assign88510_e134796_d_n9;
        locals.var_xmax2_dn10 = assign88510_e134796_d_n10;
        locals.var_xmax2_dn11 = assign88510_e134796_d_n11;
        locals.var_xmax2_dn14 = assign88510_e134796_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign88520_e134807, assign88520_e134807_d_n0, assign88520_e134807_d_n2, assign88520_e134807_d_n4, assign88520_e134807_d_n5, assign88520_e134807_d_n6, assign88520_e134807_d_n7, assign88520_e134807_d_n8, assign88520_e134807_d_n9, assign88520_e134807_d_n10, assign88520_e134807_d_n11, assign88520_e134807_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign88520_e134807;
        locals.var_xp_dn0 = assign88520_e134807_d_n0;
        locals.var_xp_dn2 = assign88520_e134807_d_n2;
        locals.var_xp_dn4 = assign88520_e134807_d_n4;
        locals.var_xp_dn5 = assign88520_e134807_d_n5;
        locals.var_xp_dn6 = assign88520_e134807_d_n6;
        locals.var_xp_dn7 = assign88520_e134807_d_n7;
        locals.var_xp_dn8 = assign88520_e134807_d_n8;
        locals.var_xp_dn9 = assign88520_e134807_d_n9;
        locals.var_xp_dn10 = assign88520_e134807_d_n10;
        locals.var_xp_dn11 = assign88520_e134807_d_n11;
        locals.var_xp_dn14 = assign88520_e134807_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign88530_e134818, assign88530_e134818_d_n0, assign88530_e134818_d_n2, assign88530_e134818_d_n4, assign88530_e134818_d_n5, assign88530_e134818_d_n6, assign88530_e134818_d_n7, assign88530_e134818_d_n8, assign88530_e134818_d_n9, assign88530_e134818_d_n10, assign88530_e134818_d_n11, assign88530_e134818_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign88530_e134818;
        locals.var_xmp_dn0 = assign88530_e134818_d_n0;
        locals.var_xmp_dn2 = assign88530_e134818_d_n2;
        locals.var_xmp_dn4 = assign88530_e134818_d_n4;
        locals.var_xmp_dn5 = assign88530_e134818_d_n5;
        locals.var_xmp_dn6 = assign88530_e134818_d_n6;
        locals.var_xmp_dn7 = assign88530_e134818_d_n7;
        locals.var_xmp_dn8 = assign88530_e134818_d_n8;
        locals.var_xmp_dn9 = assign88530_e134818_d_n9;
        locals.var_xmp_dn10 = assign88530_e134818_d_n10;
        locals.var_xmp_dn11 = assign88530_e134818_d_n11;
        locals.var_xmp_dn14 = assign88530_e134818_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign88540_e134829,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88540_e134829;
        locals.var_m0_rv = 0.0;

        let (assign88550_e134840,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88550_e134840;
        locals.var_mm_rv = 0.0;

        let (assign88560_e134851, assign88560_e134851_d_n0, assign88560_e134851_d_n2, assign88560_e134851_d_n4, assign88560_e134851_d_n5, assign88560_e134851_d_n6, assign88560_e134851_d_n7, assign88560_e134851_d_n8, assign88560_e134851_d_n9, assign88560_e134851_d_n10, assign88560_e134851_d_n11, assign88560_e134851_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign88560_e134851;
        locals.var_arg_dn0 = assign88560_e134851_d_n0;
        locals.var_arg_dn2 = assign88560_e134851_d_n2;
        locals.var_arg_dn4 = assign88560_e134851_d_n4;
        locals.var_arg_dn5 = assign88560_e134851_d_n5;
        locals.var_arg_dn6 = assign88560_e134851_d_n6;
        locals.var_arg_dn7 = assign88560_e134851_d_n7;
        locals.var_arg_dn8 = assign88560_e134851_d_n8;
        locals.var_arg_dn9 = assign88560_e134851_d_n9;
        locals.var_arg_dn10 = assign88560_e134851_d_n10;
        locals.var_arg_dn11 = assign88560_e134851_d_n11;
        locals.var_arg_dn14 = assign88560_e134851_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign88570_e134862, assign88570_e134862_d_n0, assign88570_e134862_d_n2, assign88570_e134862_d_n4, assign88570_e134862_d_n5, assign88570_e134862_d_n6, assign88570_e134862_d_n7, assign88570_e134862_d_n8, assign88570_e134862_d_n9, assign88570_e134862_d_n10, assign88570_e134862_d_n11, assign88570_e134862_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign88570_e134862;
        locals.var_dnm_dn0 = assign88570_e134862_d_n0;
        locals.var_dnm_dn2 = assign88570_e134862_d_n2;
        locals.var_dnm_dn4 = assign88570_e134862_d_n4;
        locals.var_dnm_dn5 = assign88570_e134862_d_n5;
        locals.var_dnm_dn6 = assign88570_e134862_d_n6;
        locals.var_dnm_dn7 = assign88570_e134862_d_n7;
        locals.var_dnm_dn8 = assign88570_e134862_d_n8;
        locals.var_dnm_dn9 = assign88570_e134862_d_n9;
        locals.var_dnm_dn10 = assign88570_e134862_d_n10;
        locals.var_dnm_dn11 = assign88570_e134862_d_n11;
        locals.var_dnm_dn14 = assign88570_e134862_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign88580_e134875, assign88580_e134875_d_n0, assign88580_e134875_d_n2, assign88580_e134875_d_n4, assign88580_e134875_d_n5, assign88580_e134875_d_n6, assign88580_e134875_d_n7, assign88580_e134875_d_n8, assign88580_e134875_d_n9, assign88580_e134875_d_n10, assign88580_e134875_d_n11, assign88580_e134875_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88580_e134873: f64 = (locals.var_xp * locals.var_x2);
        (assign88580_e134873, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign88580_e134875;
        locals.var_xp_dn0 = assign88580_e134875_d_n0;
        locals.var_xp_dn2 = assign88580_e134875_d_n2;
        locals.var_xp_dn4 = assign88580_e134875_d_n4;
        locals.var_xp_dn5 = assign88580_e134875_d_n5;
        locals.var_xp_dn6 = assign88580_e134875_d_n6;
        locals.var_xp_dn7 = assign88580_e134875_d_n7;
        locals.var_xp_dn8 = assign88580_e134875_d_n8;
        locals.var_xp_dn9 = assign88580_e134875_d_n9;
        locals.var_xp_dn10 = assign88580_e134875_d_n10;
        locals.var_xp_dn11 = assign88580_e134875_d_n11;
        locals.var_xp_dn14 = assign88580_e134875_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign88590_e134888, assign88590_e134888_d_n0, assign88590_e134888_d_n2, assign88590_e134888_d_n4, assign88590_e134888_d_n5, assign88590_e134888_d_n6, assign88590_e134888_d_n7, assign88590_e134888_d_n8, assign88590_e134888_d_n9, assign88590_e134888_d_n10, assign88590_e134888_d_n11, assign88590_e134888_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88590_e134886: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign88590_e134886, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign88590_e134888;
        locals.var_xmp_dn0 = assign88590_e134888_d_n0;
        locals.var_xmp_dn2 = assign88590_e134888_d_n2;
        locals.var_xmp_dn4 = assign88590_e134888_d_n4;
        locals.var_xmp_dn5 = assign88590_e134888_d_n5;
        locals.var_xmp_dn6 = assign88590_e134888_d_n6;
        locals.var_xmp_dn7 = assign88590_e134888_d_n7;
        locals.var_xmp_dn8 = assign88590_e134888_d_n8;
        locals.var_xmp_dn9 = assign88590_e134888_d_n9;
        locals.var_xmp_dn10 = assign88590_e134888_d_n10;
        locals.var_xmp_dn11 = assign88590_e134888_d_n11;
        locals.var_xmp_dn14 = assign88590_e134888_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign88600_e134901, assign88600_e134901_d_n0, assign88600_e134901_d_n2, assign88600_e134901_d_n4, assign88600_e134901_d_n5, assign88600_e134901_d_n6, assign88600_e134901_d_n7, assign88600_e134901_d_n8, assign88600_e134901_d_n9, assign88600_e134901_d_n10, assign88600_e134901_d_n11, assign88600_e134901_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88600_e134899: f64 = (locals.var_xp * locals.var_x2);
        (assign88600_e134899, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign88600_e134901;
        locals.var_xp_dn0 = assign88600_e134901_d_n0;
        locals.var_xp_dn2 = assign88600_e134901_d_n2;
        locals.var_xp_dn4 = assign88600_e134901_d_n4;
        locals.var_xp_dn5 = assign88600_e134901_d_n5;
        locals.var_xp_dn6 = assign88600_e134901_d_n6;
        locals.var_xp_dn7 = assign88600_e134901_d_n7;
        locals.var_xp_dn8 = assign88600_e134901_d_n8;
        locals.var_xp_dn9 = assign88600_e134901_d_n9;
        locals.var_xp_dn10 = assign88600_e134901_d_n10;
        locals.var_xp_dn11 = assign88600_e134901_d_n11;
        locals.var_xp_dn14 = assign88600_e134901_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign88610_e134914, assign88610_e134914_d_n0, assign88610_e134914_d_n2, assign88610_e134914_d_n4, assign88610_e134914_d_n5, assign88610_e134914_d_n6, assign88610_e134914_d_n7, assign88610_e134914_d_n8, assign88610_e134914_d_n9, assign88610_e134914_d_n10, assign88610_e134914_d_n11, assign88610_e134914_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88610_e134912: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign88610_e134912, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign88610_e134914;
        locals.var_xmp_dn0 = assign88610_e134914_d_n0;
        locals.var_xmp_dn2 = assign88610_e134914_d_n2;
        locals.var_xmp_dn4 = assign88610_e134914_d_n4;
        locals.var_xmp_dn5 = assign88610_e134914_d_n5;
        locals.var_xmp_dn6 = assign88610_e134914_d_n6;
        locals.var_xmp_dn7 = assign88610_e134914_d_n7;
        locals.var_xmp_dn8 = assign88610_e134914_d_n8;
        locals.var_xmp_dn9 = assign88610_e134914_d_n9;
        locals.var_xmp_dn10 = assign88610_e134914_d_n10;
        locals.var_xmp_dn11 = assign88610_e134914_d_n11;
        locals.var_xmp_dn14 = assign88610_e134914_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign88620_e134927, assign88620_e134927_d_n0, assign88620_e134927_d_n2, assign88620_e134927_d_n4, assign88620_e134927_d_n5, assign88620_e134927_d_n6, assign88620_e134927_d_n7, assign88620_e134927_d_n8, assign88620_e134927_d_n9, assign88620_e134927_d_n10, assign88620_e134927_d_n11, assign88620_e134927_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88620_e134925: f64 = (locals.var_xp + locals.var_xmp);
        (assign88620_e134925, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign88620_e134927;
        locals.var_arg_dn0 = assign88620_e134927_d_n0;
        locals.var_arg_dn2 = assign88620_e134927_d_n2;
        locals.var_arg_dn4 = assign88620_e134927_d_n4;
        locals.var_arg_dn5 = assign88620_e134927_d_n5;
        locals.var_arg_dn6 = assign88620_e134927_d_n6;
        locals.var_arg_dn7 = assign88620_e134927_d_n7;
        locals.var_arg_dn8 = assign88620_e134927_d_n8;
        locals.var_arg_dn9 = assign88620_e134927_d_n9;
        locals.var_arg_dn10 = assign88620_e134927_d_n10;
        locals.var_arg_dn11 = assign88620_e134927_d_n11;
        locals.var_arg_dn14 = assign88620_e134927_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign88630_e134938, assign88630_e134938_d_n0, assign88630_e134938_d_n2, assign88630_e134938_d_n4, assign88630_e134938_d_n5, assign88630_e134938_d_n6, assign88630_e134938_d_n7, assign88630_e134938_d_n8, assign88630_e134938_d_n9, assign88630_e134938_d_n10, assign88630_e134938_d_n11, assign88630_e134938_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign88630_e134938;
        locals.var_dnm_dn0 = assign88630_e134938_d_n0;
        locals.var_dnm_dn2 = assign88630_e134938_d_n2;
        locals.var_dnm_dn4 = assign88630_e134938_d_n4;
        locals.var_dnm_dn5 = assign88630_e134938_d_n5;
        locals.var_dnm_dn6 = assign88630_e134938_d_n6;
        locals.var_dnm_dn7 = assign88630_e134938_d_n7;
        locals.var_dnm_dn8 = assign88630_e134938_d_n8;
        locals.var_dnm_dn9 = assign88630_e134938_d_n9;
        locals.var_dnm_dn10 = assign88630_e134938_d_n10;
        locals.var_dnm_dn11 = assign88630_e134938_d_n11;
        locals.var_dnm_dn14 = assign88630_e134938_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign88640_e134953: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2054 = assign88640_e134953;
        locals.var_guard2054_rv = 0.0;

        let assign88650_e134956: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2055 = assign88650_e134956;
        locals.var_guard2055_rv = 0.0;

        let (assign88660_e134971,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 != 0.0)) && (locals.var_guard2055 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88660_e134971;
        locals.var_mm_rv = 0.0;

        let assign88670_e134974: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2056 = assign88670_e134974;
        locals.var_guard2056_rv = 0.0;

        let (assign88680_e134992,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 != 0.0)) && (locals.var_guard2055 == 0.0)) && (locals.var_guard2056 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88680_e134992;
        locals.var_mm_rv = 0.0;

        let assign88690_e134995: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2057 = assign88690_e134995;
        locals.var_guard2057_rv = 0.0;

        let (assign88700_e135016,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 != 0.0)) && (locals.var_guard2055 == 0.0)) && (locals.var_guard2056 == 0.0)) && (locals.var_guard2057 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88700_e135016;
        locals.var_mm_rv = 0.0;

        let assign88710_e135019: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2058 = assign88710_e135019;
        locals.var_guard2058_rv = 0.0;

        let (assign88720_e135043,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 != 0.0)) && (locals.var_guard2055 == 0.0)) && (locals.var_guard2056 == 0.0)) && (locals.var_guard2057 == 0.0)) && (locals.var_guard2058 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88720_e135043;
        locals.var_mm_rv = 0.0;

        let (assign88730_e135056,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88730_e135056;
        locals.var_m0_rv = 0.0;

        let mut assign88740_loop_guard: usize = 0;
        while {
            let assign88740_cond_e135070: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign88740_cond_e135070 != 0.0
        } {
            assign88740_loop_guard += 1;
            assert!(assign88740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign88740_body0_e135084, assign88740_body0_e135084_d_n0, assign88740_body0_e135084_d_n2, assign88740_body0_e135084_d_n4, assign88740_body0_e135084_d_n5, assign88740_body0_e135084_d_n6, assign88740_body0_e135084_d_n7, assign88740_body0_e135084_d_n8, assign88740_body0_e135084_d_n9, assign88740_body0_e135084_d_n10, assign88740_body0_e135084_d_n11, assign88740_body0_e135084_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 != 0.0)) {
        let assign88740_body0_e135082: f64 = (locals.var_dnm).sqrt();
        (assign88740_body0_e135082, (locals.var_dnm_dn0 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn2 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn4 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn5 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn6 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn7 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn8 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn9 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn10 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn11 / (2.0 * assign88740_body0_e135082)), (locals.var_dnm_dn14 / (2.0 * assign88740_body0_e135082)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign88740_body0_e135084;
            locals.var_dnm_dn0 = assign88740_body0_e135084_d_n0;
            locals.var_dnm_dn2 = assign88740_body0_e135084_d_n2;
            locals.var_dnm_dn4 = assign88740_body0_e135084_d_n4;
            locals.var_dnm_dn5 = assign88740_body0_e135084_d_n5;
            locals.var_dnm_dn6 = assign88740_body0_e135084_d_n6;
            locals.var_dnm_dn7 = assign88740_body0_e135084_d_n7;
            locals.var_dnm_dn8 = assign88740_body0_e135084_d_n8;
            locals.var_dnm_dn9 = assign88740_body0_e135084_d_n9;
            locals.var_dnm_dn10 = assign88740_body0_e135084_d_n10;
            locals.var_dnm_dn11 = assign88740_body0_e135084_d_n11;
            locals.var_dnm_dn14 = assign88740_body0_e135084_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign88740_body1_e135099,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 != 0.0)) {
        let assign88740_body1_e135097: f64 = (locals.var_m0 + 1.0);
        (assign88740_body1_e135097,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign88740_body1_e135099;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_340(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign88750_e135124, assign88750_e135124_d_n0, assign88750_e135124_d_n2, assign88750_e135124_d_n4, assign88750_e135124_d_n5, assign88750_e135124_d_n6, assign88750_e135124_d_n7, assign88750_e135124_d_n8, assign88750_e135124_d_n9, assign88750_e135124_d_n10, assign88750_e135124_d_n11, assign88750_e135124_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) && (locals.var_guard2054 == 0.0)) {
        let (assign88750_e135122, assign88750_e135122_d_n0, assign88750_e135122_d_n2, assign88750_e135122_d_n4, assign88750_e135122_d_n5, assign88750_e135122_d_n6, assign88750_e135122_d_n7, assign88750_e135122_d_n8, assign88750_e135122_d_n9, assign88750_e135122_d_n10, assign88750_e135122_d_n11, assign88750_e135122_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign88750_e135119: f64 = (2.0 * 2.0);
                let assign88750_e135120: f64 = (1.0 / assign88750_e135119);
                let assign88750_e135121: f64 = (locals.var_dnm).powf(assign88750_e135120);
                (assign88750_e135121, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn0)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn2)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn4)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn5)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn6)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn7)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn8)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn9)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn10)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn11)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88750_e135120) as f64).is_finite() && ((assign88750_e135120) as f64).fract() == 0.0 { if assign88750_e135120 == 0.0 { 0.0 } else { (assign88750_e135120 * ((locals.var_dnm).powf(assign88750_e135120 - 1.0) * locals.var_dnm_dn14)) } } else { (assign88750_e135121 * (assign88750_e135120 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign88750_e135122, assign88750_e135122_d_n0, assign88750_e135122_d_n2, assign88750_e135122_d_n4, assign88750_e135122_d_n5, assign88750_e135122_d_n6, assign88750_e135122_d_n7, assign88750_e135122_d_n8, assign88750_e135122_d_n9, assign88750_e135122_d_n10, assign88750_e135122_d_n11, assign88750_e135122_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign88750_e135124;
        locals.var_dnm_dn0 = assign88750_e135124_d_n0;
        locals.var_dnm_dn2 = assign88750_e135124_d_n2;
        locals.var_dnm_dn4 = assign88750_e135124_d_n4;
        locals.var_dnm_dn5 = assign88750_e135124_d_n5;
        locals.var_dnm_dn6 = assign88750_e135124_d_n6;
        locals.var_dnm_dn7 = assign88750_e135124_d_n7;
        locals.var_dnm_dn8 = assign88750_e135124_d_n8;
        locals.var_dnm_dn9 = assign88750_e135124_d_n9;
        locals.var_dnm_dn10 = assign88750_e135124_d_n10;
        locals.var_dnm_dn11 = assign88750_e135124_d_n11;
        locals.var_dnm_dn14 = assign88750_e135124_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign88760_e135137, assign88760_e135137_d_n0, assign88760_e135137_d_n2, assign88760_e135137_d_n4, assign88760_e135137_d_n5, assign88760_e135137_d_n6, assign88760_e135137_d_n7, assign88760_e135137_d_n8, assign88760_e135137_d_n9, assign88760_e135137_d_n10, assign88760_e135137_d_n11, assign88760_e135137_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88760_e135135: f64 = (1.0 / locals.var_dnm);
        (assign88760_e135135, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign88760_e135137;
        locals.var_dnm_dn0 = assign88760_e135137_d_n0;
        locals.var_dnm_dn2 = assign88760_e135137_d_n2;
        locals.var_dnm_dn4 = assign88760_e135137_d_n4;
        locals.var_dnm_dn5 = assign88760_e135137_d_n5;
        locals.var_dnm_dn6 = assign88760_e135137_d_n6;
        locals.var_dnm_dn7 = assign88760_e135137_d_n7;
        locals.var_dnm_dn8 = assign88760_e135137_d_n8;
        locals.var_dnm_dn9 = assign88760_e135137_d_n9;
        locals.var_dnm_dn10 = assign88760_e135137_d_n10;
        locals.var_dnm_dn11 = assign88760_e135137_d_n11;
        locals.var_dnm_dn14 = assign88760_e135137_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign88770_e135154, assign88770_e135154_d_n0, assign88770_e135154_d_n2, assign88770_e135154_d_n4, assign88770_e135154_d_n5, assign88770_e135154_d_n6, assign88770_e135154_d_n7, assign88770_e135154_d_n8, assign88770_e135154_d_n9, assign88770_e135154_d_n10, assign88770_e135154_d_n11, assign88770_e135154_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88770_e135149: f64 = (locals.var_ddriftldc * 0.1);
        let assign88770_e135150: f64 = (locals.var_tmf1 * assign88770_e135149);
        let assign88770_e135152: f64 = (assign88770_e135150 * locals.var_dnm);
        (assign88770_e135152, ((((locals.var_tmf1_dn0 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn11 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign88770_e135149) + (locals.var_tmf1 * (locals.var_ddriftldc_dn14 * 0.1))) * locals.var_dnm) + (assign88770_e135150 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign88770_e135154;
        locals.var_tmf0_dn0 = assign88770_e135154_d_n0;
        locals.var_tmf0_dn2 = assign88770_e135154_d_n2;
        locals.var_tmf0_dn4 = assign88770_e135154_d_n4;
        locals.var_tmf0_dn5 = assign88770_e135154_d_n5;
        locals.var_tmf0_dn6 = assign88770_e135154_d_n6;
        locals.var_tmf0_dn7 = assign88770_e135154_d_n7;
        locals.var_tmf0_dn8 = assign88770_e135154_d_n8;
        locals.var_tmf0_dn9 = assign88770_e135154_d_n9;
        locals.var_tmf0_dn10 = assign88770_e135154_d_n10;
        locals.var_tmf0_dn11 = assign88770_e135154_d_n11;
        locals.var_tmf0_dn14 = assign88770_e135154_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign88780_e135173, assign88780_e135173_d_n0, assign88780_e135173_d_n2, assign88780_e135173_d_n4, assign88780_e135173_d_n5, assign88780_e135173_d_n6, assign88780_e135173_d_n7, assign88780_e135173_d_n8, assign88780_e135173_d_n9, assign88780_e135173_d_n10, assign88780_e135173_d_n11, assign88780_e135173_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88780_e135165: f64 = (locals.var_ddriftldc * 0.1);
        let assign88780_e135167: f64 = (assign88780_e135165 * locals.var_xmp);
        let assign88780_e135169: f64 = (assign88780_e135167 * locals.var_dnm);
        let assign88780_e135171: f64 = (assign88780_e135169 / locals.var_arg);
        (assign88780_e135171, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn0)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn2)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn4)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn5)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn6)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn7)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn8)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn9)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn10)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn11 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn11)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn14 * 0.1) * locals.var_xmp) + (assign88780_e135165 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign88780_e135167 * locals.var_dnm_dn14)) * locals.var_arg) - (assign88780_e135169 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign88780_e135173;
        locals.var_t0_dn0 = assign88780_e135173_d_n0;
        locals.var_t0_dn2 = assign88780_e135173_d_n2;
        locals.var_t0_dn4 = assign88780_e135173_d_n4;
        locals.var_t0_dn5 = assign88780_e135173_d_n5;
        locals.var_t0_dn6 = assign88780_e135173_d_n6;
        locals.var_t0_dn7 = assign88780_e135173_d_n7;
        locals.var_t0_dn8 = assign88780_e135173_d_n8;
        locals.var_t0_dn9 = assign88780_e135173_d_n9;
        locals.var_t0_dn10 = assign88780_e135173_d_n10;
        locals.var_t0_dn11 = assign88780_e135173_d_n11;
        locals.var_t0_dn14 = assign88780_e135173_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign88790_e135190, assign88790_e135190_d_n0, assign88790_e135190_d_n2, assign88790_e135190_d_n4, assign88790_e135190_d_n5, assign88790_e135190_d_n6, assign88790_e135190_d_n7, assign88790_e135190_d_n8, assign88790_e135190_d_n9, assign88790_e135190_d_n10, assign88790_e135190_d_n11, assign88790_e135190_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        let assign88790_e135185: f64 = (locals.var_ddriftldc * 0.1);
        let assign88790_e135186: f64 = (locals.var_ddriftldc - assign88790_e135185);
        let assign88790_e135188: f64 = (assign88790_e135186 + locals.var_tmf0);
        (assign88790_e135188, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn11 - (locals.var_ddriftldc_dn11 * 0.1)) + locals.var_tmf0_dn11), ((locals.var_ddriftldc_dn14 - (locals.var_ddriftldc_dn14 * 0.1)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign88790_e135190;
        locals.var_t1_dn0 = assign88790_e135190_d_n0;
        locals.var_t1_dn2 = assign88790_e135190_d_n2;
        locals.var_t1_dn4 = assign88790_e135190_d_n4;
        locals.var_t1_dn5 = assign88790_e135190_d_n5;
        locals.var_t1_dn6 = assign88790_e135190_d_n6;
        locals.var_t1_dn7 = assign88790_e135190_d_n7;
        locals.var_t1_dn8 = assign88790_e135190_d_n8;
        locals.var_t1_dn9 = assign88790_e135190_d_n9;
        locals.var_t1_dn10 = assign88790_e135190_d_n10;
        locals.var_t1_dn11 = assign88790_e135190_d_n11;
        locals.var_t1_dn14 = assign88790_e135190_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign88800_e135201, assign88800_e135201_d_n0, assign88800_e135201_d_n2, assign88800_e135201_d_n4, assign88800_e135201_d_n5, assign88800_e135201_d_n6, assign88800_e135201_d_n7, assign88800_e135201_d_n8, assign88800_e135201_d_n9, assign88800_e135201_d_n10, assign88800_e135201_d_n11, assign88800_e135201_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign88800_e135201;
        locals.var_t0_dn0 = assign88800_e135201_d_n0;
        locals.var_t0_dn2 = assign88800_e135201_d_n2;
        locals.var_t0_dn4 = assign88800_e135201_d_n4;
        locals.var_t0_dn5 = assign88800_e135201_d_n5;
        locals.var_t0_dn6 = assign88800_e135201_d_n6;
        locals.var_t0_dn7 = assign88800_e135201_d_n7;
        locals.var_t0_dn8 = assign88800_e135201_d_n8;
        locals.var_t0_dn9 = assign88800_e135201_d_n9;
        locals.var_t0_dn10 = assign88800_e135201_d_n10;
        locals.var_t0_dn11 = assign88800_e135201_d_n11;
        locals.var_t0_dn14 = assign88800_e135201_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign88810_e135213, assign88810_e135213_d_n0, assign88810_e135213_d_n2, assign88810_e135213_d_n4, assign88810_e135213_d_n5, assign88810_e135213_d_n6, assign88810_e135213_d_n7, assign88810_e135213_d_n8, assign88810_e135213_d_n9, assign88810_e135213_d_n10, assign88810_e135213_d_n11, assign88810_e135213_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 == 0.0)) {
        (locals.var_wdld0__blk2050, locals.var_wdld0__blk2050_dn0, locals.var_wdld0__blk2050_dn2, locals.var_wdld0__blk2050_dn4, locals.var_wdld0__blk2050_dn5, locals.var_wdld0__blk2050_dn6, locals.var_wdld0__blk2050_dn7, locals.var_wdld0__blk2050_dn8, locals.var_wdld0__blk2050_dn9, locals.var_wdld0__blk2050_dn10, locals.var_wdld0__blk2050_dn11, locals.var_wdld0__blk2050_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign88810_e135213;
        locals.var_t1_dn0 = assign88810_e135213_d_n0;
        locals.var_t1_dn2 = assign88810_e135213_d_n2;
        locals.var_t1_dn4 = assign88810_e135213_d_n4;
        locals.var_t1_dn5 = assign88810_e135213_d_n5;
        locals.var_t1_dn6 = assign88810_e135213_d_n6;
        locals.var_t1_dn7 = assign88810_e135213_d_n7;
        locals.var_t1_dn8 = assign88810_e135213_d_n8;
        locals.var_t1_dn9 = assign88810_e135213_d_n9;
        locals.var_t1_dn10 = assign88810_e135213_d_n10;
        locals.var_t1_dn11 = assign88810_e135213_d_n11;
        locals.var_t1_dn14 = assign88810_e135213_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign88820_e135225, assign88820_e135225_d_n0, assign88820_e135225_d_n2, assign88820_e135225_d_n4, assign88820_e135225_d_n5, assign88820_e135225_d_n6, assign88820_e135225_d_n7, assign88820_e135225_d_n8, assign88820_e135225_d_n9, assign88820_e135225_d_n10, assign88820_e135225_d_n11, assign88820_e135225_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign88820_e135225;
        locals.var_t0_dn0 = assign88820_e135225_d_n0;
        locals.var_t0_dn2 = assign88820_e135225_d_n2;
        locals.var_t0_dn4 = assign88820_e135225_d_n4;
        locals.var_t0_dn5 = assign88820_e135225_d_n5;
        locals.var_t0_dn6 = assign88820_e135225_d_n6;
        locals.var_t0_dn7 = assign88820_e135225_d_n7;
        locals.var_t0_dn8 = assign88820_e135225_d_n8;
        locals.var_t0_dn9 = assign88820_e135225_d_n9;
        locals.var_t0_dn10 = assign88820_e135225_d_n10;
        locals.var_t0_dn11 = assign88820_e135225_d_n11;
        locals.var_t0_dn14 = assign88820_e135225_d_n14;
        locals.var_t0_rv = 0.0;

        let assign88830_e135228: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2059 = assign88830_e135228;
        locals.var_guard2059_rv = 0.0;

        let (assign88840_e135241,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2059 != 0.0)) {
        let assign88840_e135239: f64 = (locals.var_flg_fd_mode__blk2012 + 2.0);
        (assign88840_e135239,)
    } else {
        (locals.var_flg_fd_mode__blk2012,)
    }
};
        locals.var_flg_fd_mode__blk2012 = assign88840_e135241;
        locals.var_flg_fd_mode__blk2012_rv = 0.0;

        let (assign88850_e135256, assign88850_e135256_d_n0, assign88850_e135256_d_n2, assign88850_e135256_d_n4, assign88850_e135256_d_n5, assign88850_e135256_d_n6, assign88850_e135256_d_n7, assign88850_e135256_d_n8, assign88850_e135256_d_n9, assign88850_e135256_d_n10, assign88850_e135256_d_n11, assign88850_e135256_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 == 0.0)) {
        let (assign88850_e135254, assign88850_e135254_d_n0, assign88850_e135254_d_n2, assign88850_e135254_d_n4, assign88850_e135254_d_n5, assign88850_e135254_d_n6, assign88850_e135254_d_n7, assign88850_e135254_d_n8, assign88850_e135254_d_n9, assign88850_e135254_d_n10, assign88850_e135254_d_n11, assign88850_e135254_d_n14,) = {
            if (locals.var_wdld0__blk2050 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk2050, locals.var_wdld0__blk2050_dn0, locals.var_wdld0__blk2050_dn2, locals.var_wdld0__blk2050_dn4, locals.var_wdld0__blk2050_dn5, locals.var_wdld0__blk2050_dn6, locals.var_wdld0__blk2050_dn7, locals.var_wdld0__blk2050_dn8, locals.var_wdld0__blk2050_dn9, locals.var_wdld0__blk2050_dn10, locals.var_wdld0__blk2050_dn11, locals.var_wdld0__blk2050_dn14,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
            }
        };
        (assign88850_e135254, assign88850_e135254_d_n0, assign88850_e135254_d_n2, assign88850_e135254_d_n4, assign88850_e135254_d_n5, assign88850_e135254_d_n6, assign88850_e135254_d_n7, assign88850_e135254_d_n8, assign88850_e135254_d_n9, assign88850_e135254_d_n10, assign88850_e135254_d_n11, assign88850_e135254_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign88850_e135256;
        locals.var_t1_dn0 = assign88850_e135256_d_n0;
        locals.var_t1_dn2 = assign88850_e135256_d_n2;
        locals.var_t1_dn4 = assign88850_e135256_d_n4;
        locals.var_t1_dn5 = assign88850_e135256_d_n5;
        locals.var_t1_dn6 = assign88850_e135256_d_n6;
        locals.var_t1_dn7 = assign88850_e135256_d_n7;
        locals.var_t1_dn8 = assign88850_e135256_d_n8;
        locals.var_t1_dn9 = assign88850_e135256_d_n9;
        locals.var_t1_dn10 = assign88850_e135256_d_n10;
        locals.var_t1_dn11 = assign88850_e135256_d_n11;
        locals.var_t1_dn14 = assign88850_e135256_d_n14;
        locals.var_t1_rv = 0.0;

        let assign88860_e135259: f64 = if locals.var_wdld0__blk2050 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard2060 = assign88860_e135259;
        locals.var_guard2060_rv = 0.0;

        let (assign88870_e135273,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2052 == 0.0)) && (locals.var_guard2060 != 0.0)) {
        let assign88870_e135271: f64 = (locals.var_flg_fd_mode__blk2012 + 2.0);
        (assign88870_e135271,)
    } else {
        (locals.var_flg_fd_mode__blk2012,)
    }
};
        locals.var_flg_fd_mode__blk2012 = assign88870_e135273;
        locals.var_flg_fd_mode__blk2012_rv = 0.0;

        let assign88880_e135276: f64 = if locals.var_flg_fd_mode__blk2012 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2061 = assign88880_e135276;
        locals.var_guard2061_rv = 0.0;

        let (assign88890_e135285, assign88890_e135285_d_n0, assign88890_e135285_d_n2, assign88890_e135285_d_n4, assign88890_e135285_d_n5, assign88890_e135285_d_n6, assign88890_e135285_d_n7, assign88890_e135285_d_n8, assign88890_e135285_d_n9, assign88890_e135285_d_n10, assign88890_e135285_d_n11, assign88890_e135285_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_bef1__blk2051, locals.var_ps0ld_bef1__blk2051_dn0, locals.var_ps0ld_bef1__blk2051_dn2, locals.var_ps0ld_bef1__blk2051_dn4, locals.var_ps0ld_bef1__blk2051_dn5, locals.var_ps0ld_bef1__blk2051_dn6, locals.var_ps0ld_bef1__blk2051_dn7, locals.var_ps0ld_bef1__blk2051_dn8, locals.var_ps0ld_bef1__blk2051_dn9, locals.var_ps0ld_bef1__blk2051_dn10, locals.var_ps0ld_bef1__blk2051_dn11, locals.var_ps0ld_bef1__blk2051_dn14,)
    }
};
        locals.var_ps0ld_bef1__blk2051 = assign88890_e135285;
        locals.var_ps0ld_bef1__blk2051_dn0 = assign88890_e135285_d_n0;
        locals.var_ps0ld_bef1__blk2051_dn2 = assign88890_e135285_d_n2;
        locals.var_ps0ld_bef1__blk2051_dn4 = assign88890_e135285_d_n4;
        locals.var_ps0ld_bef1__blk2051_dn5 = assign88890_e135285_d_n5;
        locals.var_ps0ld_bef1__blk2051_dn6 = assign88890_e135285_d_n6;
        locals.var_ps0ld_bef1__blk2051_dn7 = assign88890_e135285_d_n7;
        locals.var_ps0ld_bef1__blk2051_dn8 = assign88890_e135285_d_n8;
        locals.var_ps0ld_bef1__blk2051_dn9 = assign88890_e135285_d_n9;
        locals.var_ps0ld_bef1__blk2051_dn10 = assign88890_e135285_d_n10;
        locals.var_ps0ld_bef1__blk2051_dn11 = assign88890_e135285_d_n11;
        locals.var_ps0ld_bef1__blk2051_dn14 = assign88890_e135285_d_n14;
        locals.var_ps0ld_bef1__blk2051_rv = 0.0;

        let (assign88900_e135296, assign88900_e135296_d_n0, assign88900_e135296_d_n2, assign88900_e135296_d_n4, assign88900_e135296_d_n5, assign88900_e135296_d_n6, assign88900_e135296_d_n7, assign88900_e135296_d_n8, assign88900_e135296_d_n9, assign88900_e135296_d_n10, assign88900_e135296_d_n11, assign88900_e135296_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign88900_e135294: f64 = (locals.var_t1 * locals.var_q_nsubld__blk2006);
        (assign88900_e135294, (locals.var_t1_dn0 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn2 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn4 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn5 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn6 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn7 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn8 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn9 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn10 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn11 * locals.var_q_nsubld__blk2006), (locals.var_t1_dn14 * locals.var_q_nsubld__blk2006),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign88900_e135296;
        locals.var_qbuld_dn0 = assign88900_e135296_d_n0;
        locals.var_qbuld_dn2 = assign88900_e135296_d_n2;
        locals.var_qbuld_dn4 = assign88900_e135296_d_n4;
        locals.var_qbuld_dn5 = assign88900_e135296_d_n5;
        locals.var_qbuld_dn6 = assign88900_e135296_d_n6;
        locals.var_qbuld_dn7 = assign88900_e135296_d_n7;
        locals.var_qbuld_dn8 = assign88900_e135296_d_n8;
        locals.var_qbuld_dn9 = assign88900_e135296_d_n9;
        locals.var_qbuld_dn10 = assign88900_e135296_d_n10;
        locals.var_qbuld_dn11 = assign88900_e135296_d_n11;
        locals.var_qbuld_dn14 = assign88900_e135296_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign88910_e135309, assign88910_e135309_d_n0, assign88910_e135309_d_n2, assign88910_e135309_d_n4, assign88910_e135309_d_n5, assign88910_e135309_d_n6, assign88910_e135309_d_n7, assign88910_e135309_d_n8, assign88910_e135309_d_n9, assign88910_e135309_d_n10, assign88910_e135309_d_n11, assign88910_e135309_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign88910_e135306: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign88910_e135307: f64 = (locals.var_vgpld - assign88910_e135306);
        (assign88910_e135307, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (-(locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (locals.var_vgpld_dn9 - (locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn11 / locals.var_cox0_func)), (-(locals.var_qbuld_dn14 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign88910_e135309;
        locals.var_ps0ld_dn0 = assign88910_e135309_d_n0;
        locals.var_ps0ld_dn2 = assign88910_e135309_d_n2;
        locals.var_ps0ld_dn4 = assign88910_e135309_d_n4;
        locals.var_ps0ld_dn5 = assign88910_e135309_d_n5;
        locals.var_ps0ld_dn6 = assign88910_e135309_d_n6;
        locals.var_ps0ld_dn7 = assign88910_e135309_d_n7;
        locals.var_ps0ld_dn8 = assign88910_e135309_d_n8;
        locals.var_ps0ld_dn9 = assign88910_e135309_d_n9;
        locals.var_ps0ld_dn10 = assign88910_e135309_d_n10;
        locals.var_ps0ld_dn11 = assign88910_e135309_d_n11;
        locals.var_ps0ld_dn14 = assign88910_e135309_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign88920_e135312: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2062 = assign88920_e135312;
        locals.var_guard2062_rv = 0.0;

        let assign88930_e135316: f64 = (locals.var_ps0ld_bef1__blk2051 - 0.1);
        let assign88930_e135321: f64 = if ((locals.var_ps0ld > assign88930_e135316) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2063 = assign88930_e135321;
        locals.var_guard2063_rv = 0.0;

        let (assign88940_e135338, assign88940_e135338_d_n0, assign88940_e135338_d_n2, assign88940_e135338_d_n4, assign88940_e135338_d_n5, assign88940_e135338_d_n6, assign88940_e135338_d_n7, assign88940_e135338_d_n8, assign88940_e135338_d_n9, assign88940_e135338_d_n10, assign88940_e135338_d_n11, assign88940_e135338_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign88940_e135334: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk2051);
        let assign88940_e135336: f64 = (assign88940_e135334 + 0.1);
        (assign88940_e135336, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk2051_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk2051_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk2051_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk2051_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk2051_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk2051_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk2051_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk2051_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk2051_dn10), (locals.var_ps0ld_dn11 - locals.var_ps0ld_bef1__blk2051_dn11), (locals.var_ps0ld_dn14 - locals.var_ps0ld_bef1__blk2051_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign88940_e135338;
        locals.var_tmf1_dn0 = assign88940_e135338_d_n0;
        locals.var_tmf1_dn2 = assign88940_e135338_d_n2;
        locals.var_tmf1_dn4 = assign88940_e135338_d_n4;
        locals.var_tmf1_dn5 = assign88940_e135338_d_n5;
        locals.var_tmf1_dn6 = assign88940_e135338_d_n6;
        locals.var_tmf1_dn7 = assign88940_e135338_d_n7;
        locals.var_tmf1_dn8 = assign88940_e135338_d_n8;
        locals.var_tmf1_dn9 = assign88940_e135338_d_n9;
        locals.var_tmf1_dn10 = assign88940_e135338_d_n10;
        locals.var_tmf1_dn11 = assign88940_e135338_d_n11;
        locals.var_tmf1_dn14 = assign88940_e135338_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign88950_e135353, assign88950_e135353_d_n0, assign88950_e135353_d_n2, assign88950_e135353_d_n4, assign88950_e135353_d_n5, assign88950_e135353_d_n6, assign88950_e135353_d_n7, assign88950_e135353_d_n8, assign88950_e135353_d_n9, assign88950_e135353_d_n10, assign88950_e135353_d_n11, assign88950_e135353_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign88950_e135351: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign88950_e135351, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign88950_e135353;
        locals.var_x2_dn0 = assign88950_e135353_d_n0;
        locals.var_x2_dn2 = assign88950_e135353_d_n2;
        locals.var_x2_dn4 = assign88950_e135353_d_n4;
        locals.var_x2_dn5 = assign88950_e135353_d_n5;
        locals.var_x2_dn6 = assign88950_e135353_d_n6;
        locals.var_x2_dn7 = assign88950_e135353_d_n7;
        locals.var_x2_dn8 = assign88950_e135353_d_n8;
        locals.var_x2_dn9 = assign88950_e135353_d_n9;
        locals.var_x2_dn10 = assign88950_e135353_d_n10;
        locals.var_x2_dn11 = assign88950_e135353_d_n11;
        locals.var_x2_dn14 = assign88950_e135353_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign88960_e135368, assign88960_e135368_d_n0, assign88960_e135368_d_n2, assign88960_e135368_d_n4, assign88960_e135368_d_n5, assign88960_e135368_d_n6, assign88960_e135368_d_n7, assign88960_e135368_d_n8, assign88960_e135368_d_n9, assign88960_e135368_d_n10, assign88960_e135368_d_n11, assign88960_e135368_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign88960_e135366: f64 = (0.1 * 0.1);
        (assign88960_e135366, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign88960_e135368;
        locals.var_xmax2_dn0 = assign88960_e135368_d_n0;
        locals.var_xmax2_dn2 = assign88960_e135368_d_n2;
        locals.var_xmax2_dn4 = assign88960_e135368_d_n4;
        locals.var_xmax2_dn5 = assign88960_e135368_d_n5;
        locals.var_xmax2_dn6 = assign88960_e135368_d_n6;
        locals.var_xmax2_dn7 = assign88960_e135368_d_n7;
        locals.var_xmax2_dn8 = assign88960_e135368_d_n8;
        locals.var_xmax2_dn9 = assign88960_e135368_d_n9;
        locals.var_xmax2_dn10 = assign88960_e135368_d_n10;
        locals.var_xmax2_dn11 = assign88960_e135368_d_n11;
        locals.var_xmax2_dn14 = assign88960_e135368_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign88970_e135381, assign88970_e135381_d_n0, assign88970_e135381_d_n2, assign88970_e135381_d_n4, assign88970_e135381_d_n5, assign88970_e135381_d_n6, assign88970_e135381_d_n7, assign88970_e135381_d_n8, assign88970_e135381_d_n9, assign88970_e135381_d_n10, assign88970_e135381_d_n11, assign88970_e135381_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign88970_e135381;
        locals.var_xp_dn0 = assign88970_e135381_d_n0;
        locals.var_xp_dn2 = assign88970_e135381_d_n2;
        locals.var_xp_dn4 = assign88970_e135381_d_n4;
        locals.var_xp_dn5 = assign88970_e135381_d_n5;
        locals.var_xp_dn6 = assign88970_e135381_d_n6;
        locals.var_xp_dn7 = assign88970_e135381_d_n7;
        locals.var_xp_dn8 = assign88970_e135381_d_n8;
        locals.var_xp_dn9 = assign88970_e135381_d_n9;
        locals.var_xp_dn10 = assign88970_e135381_d_n10;
        locals.var_xp_dn11 = assign88970_e135381_d_n11;
        locals.var_xp_dn14 = assign88970_e135381_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign88980_e135394, assign88980_e135394_d_n0, assign88980_e135394_d_n2, assign88980_e135394_d_n4, assign88980_e135394_d_n5, assign88980_e135394_d_n6, assign88980_e135394_d_n7, assign88980_e135394_d_n8, assign88980_e135394_d_n9, assign88980_e135394_d_n10, assign88980_e135394_d_n11, assign88980_e135394_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign88980_e135394;
        locals.var_xmp_dn0 = assign88980_e135394_d_n0;
        locals.var_xmp_dn2 = assign88980_e135394_d_n2;
        locals.var_xmp_dn4 = assign88980_e135394_d_n4;
        locals.var_xmp_dn5 = assign88980_e135394_d_n5;
        locals.var_xmp_dn6 = assign88980_e135394_d_n6;
        locals.var_xmp_dn7 = assign88980_e135394_d_n7;
        locals.var_xmp_dn8 = assign88980_e135394_d_n8;
        locals.var_xmp_dn9 = assign88980_e135394_d_n9;
        locals.var_xmp_dn10 = assign88980_e135394_d_n10;
        locals.var_xmp_dn11 = assign88980_e135394_d_n11;
        locals.var_xmp_dn14 = assign88980_e135394_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign88990_e135407,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88990_e135407;
        locals.var_m0_rv = 0.0;

        let (assign89000_e135420,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89000_e135420;
        locals.var_mm_rv = 0.0;

        let (assign89010_e135433, assign89010_e135433_d_n0, assign89010_e135433_d_n2, assign89010_e135433_d_n4, assign89010_e135433_d_n5, assign89010_e135433_d_n6, assign89010_e135433_d_n7, assign89010_e135433_d_n8, assign89010_e135433_d_n9, assign89010_e135433_d_n10, assign89010_e135433_d_n11, assign89010_e135433_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign89010_e135433;
        locals.var_arg_dn0 = assign89010_e135433_d_n0;
        locals.var_arg_dn2 = assign89010_e135433_d_n2;
        locals.var_arg_dn4 = assign89010_e135433_d_n4;
        locals.var_arg_dn5 = assign89010_e135433_d_n5;
        locals.var_arg_dn6 = assign89010_e135433_d_n6;
        locals.var_arg_dn7 = assign89010_e135433_d_n7;
        locals.var_arg_dn8 = assign89010_e135433_d_n8;
        locals.var_arg_dn9 = assign89010_e135433_d_n9;
        locals.var_arg_dn10 = assign89010_e135433_d_n10;
        locals.var_arg_dn11 = assign89010_e135433_d_n11;
        locals.var_arg_dn14 = assign89010_e135433_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign89020_e135446, assign89020_e135446_d_n0, assign89020_e135446_d_n2, assign89020_e135446_d_n4, assign89020_e135446_d_n5, assign89020_e135446_d_n6, assign89020_e135446_d_n7, assign89020_e135446_d_n8, assign89020_e135446_d_n9, assign89020_e135446_d_n10, assign89020_e135446_d_n11, assign89020_e135446_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign89020_e135446;
        locals.var_dnm_dn0 = assign89020_e135446_d_n0;
        locals.var_dnm_dn2 = assign89020_e135446_d_n2;
        locals.var_dnm_dn4 = assign89020_e135446_d_n4;
        locals.var_dnm_dn5 = assign89020_e135446_d_n5;
        locals.var_dnm_dn6 = assign89020_e135446_d_n6;
        locals.var_dnm_dn7 = assign89020_e135446_d_n7;
        locals.var_dnm_dn8 = assign89020_e135446_d_n8;
        locals.var_dnm_dn9 = assign89020_e135446_d_n9;
        locals.var_dnm_dn10 = assign89020_e135446_d_n10;
        locals.var_dnm_dn11 = assign89020_e135446_d_n11;
        locals.var_dnm_dn14 = assign89020_e135446_d_n14;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_341(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89030_e135461, assign89030_e135461_d_n0, assign89030_e135461_d_n2, assign89030_e135461_d_n4, assign89030_e135461_d_n5, assign89030_e135461_d_n6, assign89030_e135461_d_n7, assign89030_e135461_d_n8, assign89030_e135461_d_n9, assign89030_e135461_d_n10, assign89030_e135461_d_n11, assign89030_e135461_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89030_e135459: f64 = (locals.var_xp * locals.var_x2);
        (assign89030_e135459, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign89030_e135461;
        locals.var_xp_dn0 = assign89030_e135461_d_n0;
        locals.var_xp_dn2 = assign89030_e135461_d_n2;
        locals.var_xp_dn4 = assign89030_e135461_d_n4;
        locals.var_xp_dn5 = assign89030_e135461_d_n5;
        locals.var_xp_dn6 = assign89030_e135461_d_n6;
        locals.var_xp_dn7 = assign89030_e135461_d_n7;
        locals.var_xp_dn8 = assign89030_e135461_d_n8;
        locals.var_xp_dn9 = assign89030_e135461_d_n9;
        locals.var_xp_dn10 = assign89030_e135461_d_n10;
        locals.var_xp_dn11 = assign89030_e135461_d_n11;
        locals.var_xp_dn14 = assign89030_e135461_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign89040_e135476, assign89040_e135476_d_n0, assign89040_e135476_d_n2, assign89040_e135476_d_n4, assign89040_e135476_d_n5, assign89040_e135476_d_n6, assign89040_e135476_d_n7, assign89040_e135476_d_n8, assign89040_e135476_d_n9, assign89040_e135476_d_n10, assign89040_e135476_d_n11, assign89040_e135476_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89040_e135474: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign89040_e135474, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign89040_e135476;
        locals.var_xmp_dn0 = assign89040_e135476_d_n0;
        locals.var_xmp_dn2 = assign89040_e135476_d_n2;
        locals.var_xmp_dn4 = assign89040_e135476_d_n4;
        locals.var_xmp_dn5 = assign89040_e135476_d_n5;
        locals.var_xmp_dn6 = assign89040_e135476_d_n6;
        locals.var_xmp_dn7 = assign89040_e135476_d_n7;
        locals.var_xmp_dn8 = assign89040_e135476_d_n8;
        locals.var_xmp_dn9 = assign89040_e135476_d_n9;
        locals.var_xmp_dn10 = assign89040_e135476_d_n10;
        locals.var_xmp_dn11 = assign89040_e135476_d_n11;
        locals.var_xmp_dn14 = assign89040_e135476_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign89050_e135491, assign89050_e135491_d_n0, assign89050_e135491_d_n2, assign89050_e135491_d_n4, assign89050_e135491_d_n5, assign89050_e135491_d_n6, assign89050_e135491_d_n7, assign89050_e135491_d_n8, assign89050_e135491_d_n9, assign89050_e135491_d_n10, assign89050_e135491_d_n11, assign89050_e135491_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89050_e135489: f64 = (locals.var_xp * locals.var_x2);
        (assign89050_e135489, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign89050_e135491;
        locals.var_xp_dn0 = assign89050_e135491_d_n0;
        locals.var_xp_dn2 = assign89050_e135491_d_n2;
        locals.var_xp_dn4 = assign89050_e135491_d_n4;
        locals.var_xp_dn5 = assign89050_e135491_d_n5;
        locals.var_xp_dn6 = assign89050_e135491_d_n6;
        locals.var_xp_dn7 = assign89050_e135491_d_n7;
        locals.var_xp_dn8 = assign89050_e135491_d_n8;
        locals.var_xp_dn9 = assign89050_e135491_d_n9;
        locals.var_xp_dn10 = assign89050_e135491_d_n10;
        locals.var_xp_dn11 = assign89050_e135491_d_n11;
        locals.var_xp_dn14 = assign89050_e135491_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign89060_e135506, assign89060_e135506_d_n0, assign89060_e135506_d_n2, assign89060_e135506_d_n4, assign89060_e135506_d_n5, assign89060_e135506_d_n6, assign89060_e135506_d_n7, assign89060_e135506_d_n8, assign89060_e135506_d_n9, assign89060_e135506_d_n10, assign89060_e135506_d_n11, assign89060_e135506_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89060_e135504: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign89060_e135504, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign89060_e135506;
        locals.var_xmp_dn0 = assign89060_e135506_d_n0;
        locals.var_xmp_dn2 = assign89060_e135506_d_n2;
        locals.var_xmp_dn4 = assign89060_e135506_d_n4;
        locals.var_xmp_dn5 = assign89060_e135506_d_n5;
        locals.var_xmp_dn6 = assign89060_e135506_d_n6;
        locals.var_xmp_dn7 = assign89060_e135506_d_n7;
        locals.var_xmp_dn8 = assign89060_e135506_d_n8;
        locals.var_xmp_dn9 = assign89060_e135506_d_n9;
        locals.var_xmp_dn10 = assign89060_e135506_d_n10;
        locals.var_xmp_dn11 = assign89060_e135506_d_n11;
        locals.var_xmp_dn14 = assign89060_e135506_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign89070_e135521, assign89070_e135521_d_n0, assign89070_e135521_d_n2, assign89070_e135521_d_n4, assign89070_e135521_d_n5, assign89070_e135521_d_n6, assign89070_e135521_d_n7, assign89070_e135521_d_n8, assign89070_e135521_d_n9, assign89070_e135521_d_n10, assign89070_e135521_d_n11, assign89070_e135521_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89070_e135519: f64 = (locals.var_xp + locals.var_xmp);
        (assign89070_e135519, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign89070_e135521;
        locals.var_arg_dn0 = assign89070_e135521_d_n0;
        locals.var_arg_dn2 = assign89070_e135521_d_n2;
        locals.var_arg_dn4 = assign89070_e135521_d_n4;
        locals.var_arg_dn5 = assign89070_e135521_d_n5;
        locals.var_arg_dn6 = assign89070_e135521_d_n6;
        locals.var_arg_dn7 = assign89070_e135521_d_n7;
        locals.var_arg_dn8 = assign89070_e135521_d_n8;
        locals.var_arg_dn9 = assign89070_e135521_d_n9;
        locals.var_arg_dn10 = assign89070_e135521_d_n10;
        locals.var_arg_dn11 = assign89070_e135521_d_n11;
        locals.var_arg_dn14 = assign89070_e135521_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign89080_e135534, assign89080_e135534_d_n0, assign89080_e135534_d_n2, assign89080_e135534_d_n4, assign89080_e135534_d_n5, assign89080_e135534_d_n6, assign89080_e135534_d_n7, assign89080_e135534_d_n8, assign89080_e135534_d_n9, assign89080_e135534_d_n10, assign89080_e135534_d_n11, assign89080_e135534_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign89080_e135534;
        locals.var_dnm_dn0 = assign89080_e135534_d_n0;
        locals.var_dnm_dn2 = assign89080_e135534_d_n2;
        locals.var_dnm_dn4 = assign89080_e135534_d_n4;
        locals.var_dnm_dn5 = assign89080_e135534_d_n5;
        locals.var_dnm_dn6 = assign89080_e135534_d_n6;
        locals.var_dnm_dn7 = assign89080_e135534_d_n7;
        locals.var_dnm_dn8 = assign89080_e135534_d_n8;
        locals.var_dnm_dn9 = assign89080_e135534_d_n9;
        locals.var_dnm_dn10 = assign89080_e135534_d_n10;
        locals.var_dnm_dn11 = assign89080_e135534_d_n11;
        locals.var_dnm_dn14 = assign89080_e135534_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign89090_e135549: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2064 = assign89090_e135549;
        locals.var_guard2064_rv = 0.0;

        let assign89100_e135552: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2065 = assign89100_e135552;
        locals.var_guard2065_rv = 0.0;

        let (assign89110_e135569,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 != 0.0)) && (locals.var_guard2065 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89110_e135569;
        locals.var_mm_rv = 0.0;

        let assign89120_e135572: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2066 = assign89120_e135572;
        locals.var_guard2066_rv = 0.0;

        let (assign89130_e135592,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 != 0.0)) && (locals.var_guard2065 == 0.0)) && (locals.var_guard2066 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89130_e135592;
        locals.var_mm_rv = 0.0;

        let assign89140_e135595: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2067 = assign89140_e135595;
        locals.var_guard2067_rv = 0.0;

        let (assign89150_e135618,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 != 0.0)) && (locals.var_guard2065 == 0.0)) && (locals.var_guard2066 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89150_e135618;
        locals.var_mm_rv = 0.0;

        let assign89160_e135621: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2068 = assign89160_e135621;
        locals.var_guard2068_rv = 0.0;

        let (assign89170_e135647,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 != 0.0)) && (locals.var_guard2065 == 0.0)) && (locals.var_guard2066 == 0.0)) && (locals.var_guard2067 == 0.0)) && (locals.var_guard2068 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89170_e135647;
        locals.var_mm_rv = 0.0;

        let (assign89180_e135662,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign89180_e135662;
        locals.var_m0_rv = 0.0;

        let mut assign89190_loop_guard: usize = 0;
        while {
            let assign89190_cond_e135678: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign89190_cond_e135678 != 0.0
        } {
            assign89190_loop_guard += 1;
            assert!(assign89190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89190_body0_e135694, assign89190_body0_e135694_d_n0, assign89190_body0_e135694_d_n2, assign89190_body0_e135694_d_n4, assign89190_body0_e135694_d_n5, assign89190_body0_e135694_d_n6, assign89190_body0_e135694_d_n7, assign89190_body0_e135694_d_n8, assign89190_body0_e135694_d_n9, assign89190_body0_e135694_d_n10, assign89190_body0_e135694_d_n11, assign89190_body0_e135694_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 != 0.0)) {
        let assign89190_body0_e135692: f64 = (locals.var_dnm).sqrt();
        (assign89190_body0_e135692, (locals.var_dnm_dn0 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn2 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn4 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn5 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn6 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn7 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn8 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn9 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn10 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn11 / (2.0 * assign89190_body0_e135692)), (locals.var_dnm_dn14 / (2.0 * assign89190_body0_e135692)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign89190_body0_e135694;
            locals.var_dnm_dn0 = assign89190_body0_e135694_d_n0;
            locals.var_dnm_dn2 = assign89190_body0_e135694_d_n2;
            locals.var_dnm_dn4 = assign89190_body0_e135694_d_n4;
            locals.var_dnm_dn5 = assign89190_body0_e135694_d_n5;
            locals.var_dnm_dn6 = assign89190_body0_e135694_d_n6;
            locals.var_dnm_dn7 = assign89190_body0_e135694_d_n7;
            locals.var_dnm_dn8 = assign89190_body0_e135694_d_n8;
            locals.var_dnm_dn9 = assign89190_body0_e135694_d_n9;
            locals.var_dnm_dn10 = assign89190_body0_e135694_d_n10;
            locals.var_dnm_dn11 = assign89190_body0_e135694_d_n11;
            locals.var_dnm_dn14 = assign89190_body0_e135694_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign89190_body1_e135711,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 != 0.0)) {
        let assign89190_body1_e135709: f64 = (locals.var_m0 + 1.0);
        (assign89190_body1_e135709,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign89190_body1_e135711;
            locals.var_m0_rv = 0.0;
        }

        let (assign89200_e135738, assign89200_e135738_d_n0, assign89200_e135738_d_n2, assign89200_e135738_d_n4, assign89200_e135738_d_n5, assign89200_e135738_d_n6, assign89200_e135738_d_n7, assign89200_e135738_d_n8, assign89200_e135738_d_n9, assign89200_e135738_d_n10, assign89200_e135738_d_n11, assign89200_e135738_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) && (locals.var_guard2064 == 0.0)) {
        let (assign89200_e135736, assign89200_e135736_d_n0, assign89200_e135736_d_n2, assign89200_e135736_d_n4, assign89200_e135736_d_n5, assign89200_e135736_d_n6, assign89200_e135736_d_n7, assign89200_e135736_d_n8, assign89200_e135736_d_n9, assign89200_e135736_d_n10, assign89200_e135736_d_n11, assign89200_e135736_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89200_e135733: f64 = (2.0 * 2.0);
                let assign89200_e135734: f64 = (1.0 / assign89200_e135733);
                let assign89200_e135735: f64 = (locals.var_dnm).powf(assign89200_e135734);
                (assign89200_e135735, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn0)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn2)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn4)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn5)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn6)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn7)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn8)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn9)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn10)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn11)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89200_e135734) as f64).is_finite() && ((assign89200_e135734) as f64).fract() == 0.0 { if assign89200_e135734 == 0.0 { 0.0 } else { (assign89200_e135734 * ((locals.var_dnm).powf(assign89200_e135734 - 1.0) * locals.var_dnm_dn14)) } } else { (assign89200_e135735 * (assign89200_e135734 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign89200_e135736, assign89200_e135736_d_n0, assign89200_e135736_d_n2, assign89200_e135736_d_n4, assign89200_e135736_d_n5, assign89200_e135736_d_n6, assign89200_e135736_d_n7, assign89200_e135736_d_n8, assign89200_e135736_d_n9, assign89200_e135736_d_n10, assign89200_e135736_d_n11, assign89200_e135736_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign89200_e135738;
        locals.var_dnm_dn0 = assign89200_e135738_d_n0;
        locals.var_dnm_dn2 = assign89200_e135738_d_n2;
        locals.var_dnm_dn4 = assign89200_e135738_d_n4;
        locals.var_dnm_dn5 = assign89200_e135738_d_n5;
        locals.var_dnm_dn6 = assign89200_e135738_d_n6;
        locals.var_dnm_dn7 = assign89200_e135738_d_n7;
        locals.var_dnm_dn8 = assign89200_e135738_d_n8;
        locals.var_dnm_dn9 = assign89200_e135738_d_n9;
        locals.var_dnm_dn10 = assign89200_e135738_d_n10;
        locals.var_dnm_dn11 = assign89200_e135738_d_n11;
        locals.var_dnm_dn14 = assign89200_e135738_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign89210_e135753, assign89210_e135753_d_n0, assign89210_e135753_d_n2, assign89210_e135753_d_n4, assign89210_e135753_d_n5, assign89210_e135753_d_n6, assign89210_e135753_d_n7, assign89210_e135753_d_n8, assign89210_e135753_d_n9, assign89210_e135753_d_n10, assign89210_e135753_d_n11, assign89210_e135753_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89210_e135751: f64 = (1.0 / locals.var_dnm);
        (assign89210_e135751, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign89210_e135753;
        locals.var_dnm_dn0 = assign89210_e135753_d_n0;
        locals.var_dnm_dn2 = assign89210_e135753_d_n2;
        locals.var_dnm_dn4 = assign89210_e135753_d_n4;
        locals.var_dnm_dn5 = assign89210_e135753_d_n5;
        locals.var_dnm_dn6 = assign89210_e135753_d_n6;
        locals.var_dnm_dn7 = assign89210_e135753_d_n7;
        locals.var_dnm_dn8 = assign89210_e135753_d_n8;
        locals.var_dnm_dn9 = assign89210_e135753_d_n9;
        locals.var_dnm_dn10 = assign89210_e135753_d_n10;
        locals.var_dnm_dn11 = assign89210_e135753_d_n11;
        locals.var_dnm_dn14 = assign89210_e135753_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign89220_e135770, assign89220_e135770_d_n0, assign89220_e135770_d_n2, assign89220_e135770_d_n4, assign89220_e135770_d_n5, assign89220_e135770_d_n6, assign89220_e135770_d_n7, assign89220_e135770_d_n8, assign89220_e135770_d_n9, assign89220_e135770_d_n10, assign89220_e135770_d_n11, assign89220_e135770_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89220_e135766: f64 = (locals.var_tmf1 * 0.1);
        let assign89220_e135768: f64 = (assign89220_e135766 * locals.var_dnm);
        (assign89220_e135768, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign89220_e135766 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign89220_e135770;
        locals.var_tmf0_dn0 = assign89220_e135770_d_n0;
        locals.var_tmf0_dn2 = assign89220_e135770_d_n2;
        locals.var_tmf0_dn4 = assign89220_e135770_d_n4;
        locals.var_tmf0_dn5 = assign89220_e135770_d_n5;
        locals.var_tmf0_dn6 = assign89220_e135770_d_n6;
        locals.var_tmf0_dn7 = assign89220_e135770_d_n7;
        locals.var_tmf0_dn8 = assign89220_e135770_d_n8;
        locals.var_tmf0_dn9 = assign89220_e135770_d_n9;
        locals.var_tmf0_dn10 = assign89220_e135770_d_n10;
        locals.var_tmf0_dn11 = assign89220_e135770_d_n11;
        locals.var_tmf0_dn14 = assign89220_e135770_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign89230_e135789, assign89230_e135789_d_n0, assign89230_e135789_d_n2, assign89230_e135789_d_n4, assign89230_e135789_d_n5, assign89230_e135789_d_n6, assign89230_e135789_d_n7, assign89230_e135789_d_n8, assign89230_e135789_d_n9, assign89230_e135789_d_n10, assign89230_e135789_d_n11, assign89230_e135789_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89230_e135783: f64 = (0.1 * locals.var_xmp);
        let assign89230_e135785: f64 = (assign89230_e135783 * locals.var_dnm);
        let assign89230_e135787: f64 = (assign89230_e135785 / locals.var_arg);
        (assign89230_e135787, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn0)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn2)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn4)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn5)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn6)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn7)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn8)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn9)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn10)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn11)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign89230_e135783 * locals.var_dnm_dn14)) * locals.var_arg) - (assign89230_e135785 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign89230_e135789;
        locals.var_t0_dn0 = assign89230_e135789_d_n0;
        locals.var_t0_dn2 = assign89230_e135789_d_n2;
        locals.var_t0_dn4 = assign89230_e135789_d_n4;
        locals.var_t0_dn5 = assign89230_e135789_d_n5;
        locals.var_t0_dn6 = assign89230_e135789_d_n6;
        locals.var_t0_dn7 = assign89230_e135789_d_n7;
        locals.var_t0_dn8 = assign89230_e135789_d_n8;
        locals.var_t0_dn9 = assign89230_e135789_d_n9;
        locals.var_t0_dn10 = assign89230_e135789_d_n10;
        locals.var_t0_dn11 = assign89230_e135789_d_n11;
        locals.var_t0_dn14 = assign89230_e135789_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign89240_e135806, assign89240_e135806_d_n0, assign89240_e135806_d_n2, assign89240_e135806_d_n4, assign89240_e135806_d_n5, assign89240_e135806_d_n6, assign89240_e135806_d_n7, assign89240_e135806_d_n8, assign89240_e135806_d_n9, assign89240_e135806_d_n10, assign89240_e135806_d_n11, assign89240_e135806_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        let assign89240_e135802: f64 = (locals.var_ps0ld_bef1__blk2051 - 0.1);
        let assign89240_e135804: f64 = (assign89240_e135802 + locals.var_tmf0);
        (assign89240_e135804, (locals.var_ps0ld_bef1__blk2051_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk2051_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk2051_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk2051_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk2051_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk2051_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk2051_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk2051_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk2051_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk2051_dn11 + locals.var_tmf0_dn11), (locals.var_ps0ld_bef1__blk2051_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign89240_e135806;
        locals.var_ps0ld_dn0 = assign89240_e135806_d_n0;
        locals.var_ps0ld_dn2 = assign89240_e135806_d_n2;
        locals.var_ps0ld_dn4 = assign89240_e135806_d_n4;
        locals.var_ps0ld_dn5 = assign89240_e135806_d_n5;
        locals.var_ps0ld_dn6 = assign89240_e135806_d_n6;
        locals.var_ps0ld_dn7 = assign89240_e135806_d_n7;
        locals.var_ps0ld_dn8 = assign89240_e135806_d_n8;
        locals.var_ps0ld_dn9 = assign89240_e135806_d_n9;
        locals.var_ps0ld_dn10 = assign89240_e135806_d_n10;
        locals.var_ps0ld_dn11 = assign89240_e135806_d_n11;
        locals.var_ps0ld_dn14 = assign89240_e135806_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign89250_e135819, assign89250_e135819_d_n0, assign89250_e135819_d_n2, assign89250_e135819_d_n4, assign89250_e135819_d_n5, assign89250_e135819_d_n6, assign89250_e135819_d_n7, assign89250_e135819_d_n8, assign89250_e135819_d_n9, assign89250_e135819_d_n10, assign89250_e135819_d_n11, assign89250_e135819_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign89250_e135819;
        locals.var_t0_dn0 = assign89250_e135819_d_n0;
        locals.var_t0_dn2 = assign89250_e135819_d_n2;
        locals.var_t0_dn4 = assign89250_e135819_d_n4;
        locals.var_t0_dn5 = assign89250_e135819_d_n5;
        locals.var_t0_dn6 = assign89250_e135819_d_n6;
        locals.var_t0_dn7 = assign89250_e135819_d_n7;
        locals.var_t0_dn8 = assign89250_e135819_d_n8;
        locals.var_t0_dn9 = assign89250_e135819_d_n9;
        locals.var_t0_dn10 = assign89250_e135819_d_n10;
        locals.var_t0_dn11 = assign89250_e135819_d_n11;
        locals.var_t0_dn14 = assign89250_e135819_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign89260_e135833, assign89260_e135833_d_n0, assign89260_e135833_d_n2, assign89260_e135833_d_n4, assign89260_e135833_d_n5, assign89260_e135833_d_n6, assign89260_e135833_d_n7, assign89260_e135833_d_n8, assign89260_e135833_d_n9, assign89260_e135833_d_n10, assign89260_e135833_d_n11, assign89260_e135833_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign89260_e135833;
        locals.var_ps0ld_dn0 = assign89260_e135833_d_n0;
        locals.var_ps0ld_dn2 = assign89260_e135833_d_n2;
        locals.var_ps0ld_dn4 = assign89260_e135833_d_n4;
        locals.var_ps0ld_dn5 = assign89260_e135833_d_n5;
        locals.var_ps0ld_dn6 = assign89260_e135833_d_n6;
        locals.var_ps0ld_dn7 = assign89260_e135833_d_n7;
        locals.var_ps0ld_dn8 = assign89260_e135833_d_n8;
        locals.var_ps0ld_dn9 = assign89260_e135833_d_n9;
        locals.var_ps0ld_dn10 = assign89260_e135833_d_n10;
        locals.var_ps0ld_dn11 = assign89260_e135833_d_n11;
        locals.var_ps0ld_dn14 = assign89260_e135833_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign89270_e135847, assign89270_e135847_d_n0, assign89270_e135847_d_n2, assign89270_e135847_d_n4, assign89270_e135847_d_n5, assign89270_e135847_d_n6, assign89270_e135847_d_n7, assign89270_e135847_d_n8, assign89270_e135847_d_n9, assign89270_e135847_d_n10, assign89270_e135847_d_n11, assign89270_e135847_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign89270_e135847;
        locals.var_t0_dn0 = assign89270_e135847_d_n0;
        locals.var_t0_dn2 = assign89270_e135847_d_n2;
        locals.var_t0_dn4 = assign89270_e135847_d_n4;
        locals.var_t0_dn5 = assign89270_e135847_d_n5;
        locals.var_t0_dn6 = assign89270_e135847_d_n6;
        locals.var_t0_dn7 = assign89270_e135847_d_n7;
        locals.var_t0_dn8 = assign89270_e135847_d_n8;
        locals.var_t0_dn9 = assign89270_e135847_d_n9;
        locals.var_t0_dn10 = assign89270_e135847_d_n10;
        locals.var_t0_dn11 = assign89270_e135847_d_n11;
        locals.var_t0_dn14 = assign89270_e135847_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign89280_e135864, assign89280_e135864_d_n0, assign89280_e135864_d_n2, assign89280_e135864_d_n4, assign89280_e135864_d_n5, assign89280_e135864_d_n6, assign89280_e135864_d_n7, assign89280_e135864_d_n8, assign89280_e135864_d_n9, assign89280_e135864_d_n10, assign89280_e135864_d_n11, assign89280_e135864_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 == 0.0)) {
        let (assign89280_e135862, assign89280_e135862_d_n0, assign89280_e135862_d_n2, assign89280_e135862_d_n4, assign89280_e135862_d_n5, assign89280_e135862_d_n6, assign89280_e135862_d_n7, assign89280_e135862_d_n8, assign89280_e135862_d_n9, assign89280_e135862_d_n10, assign89280_e135862_d_n11, assign89280_e135862_d_n14,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk2051) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
            } else {
                (locals.var_ps0ld_bef1__blk2051, locals.var_ps0ld_bef1__blk2051_dn0, locals.var_ps0ld_bef1__blk2051_dn2, locals.var_ps0ld_bef1__blk2051_dn4, locals.var_ps0ld_bef1__blk2051_dn5, locals.var_ps0ld_bef1__blk2051_dn6, locals.var_ps0ld_bef1__blk2051_dn7, locals.var_ps0ld_bef1__blk2051_dn8, locals.var_ps0ld_bef1__blk2051_dn9, locals.var_ps0ld_bef1__blk2051_dn10, locals.var_ps0ld_bef1__blk2051_dn11, locals.var_ps0ld_bef1__blk2051_dn14,)
            }
        };
        (assign89280_e135862, assign89280_e135862_d_n0, assign89280_e135862_d_n2, assign89280_e135862_d_n4, assign89280_e135862_d_n5, assign89280_e135862_d_n6, assign89280_e135862_d_n7, assign89280_e135862_d_n8, assign89280_e135862_d_n9, assign89280_e135862_d_n10, assign89280_e135862_d_n11, assign89280_e135862_d_n14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign89280_e135864;
        locals.var_ps0ld_dn0 = assign89280_e135864_d_n0;
        locals.var_ps0ld_dn2 = assign89280_e135864_d_n2;
        locals.var_ps0ld_dn4 = assign89280_e135864_d_n4;
        locals.var_ps0ld_dn5 = assign89280_e135864_d_n5;
        locals.var_ps0ld_dn6 = assign89280_e135864_d_n6;
        locals.var_ps0ld_dn7 = assign89280_e135864_d_n7;
        locals.var_ps0ld_dn8 = assign89280_e135864_d_n8;
        locals.var_ps0ld_dn9 = assign89280_e135864_d_n9;
        locals.var_ps0ld_dn10 = assign89280_e135864_d_n10;
        locals.var_ps0ld_dn11 = assign89280_e135864_d_n11;
        locals.var_ps0ld_dn14 = assign89280_e135864_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign89290_e135871, assign89290_e135871_d_n0, assign89290_e135871_d_n2, assign89290_e135871_d_n4, assign89290_e135871_d_n5, assign89290_e135871_d_n6, assign89290_e135871_d_n7, assign89290_e135871_d_n8, assign89290_e135871_d_n9, assign89290_e135871_d_n10, assign89290_e135871_d_n11, assign89290_e135871_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk2013, locals.var_ps0ld_ini__blk2013_dn0, locals.var_ps0ld_ini__blk2013_dn2, locals.var_ps0ld_ini__blk2013_dn4, locals.var_ps0ld_ini__blk2013_dn5, locals.var_ps0ld_ini__blk2013_dn6, locals.var_ps0ld_ini__blk2013_dn7, locals.var_ps0ld_ini__blk2013_dn8, locals.var_ps0ld_ini__blk2013_dn9, locals.var_ps0ld_ini__blk2013_dn10, locals.var_ps0ld_ini__blk2013_dn11, locals.var_ps0ld_ini__blk2013_dn14,)
    }
};
        locals.var_ps0ld_ini__blk2013 = assign89290_e135871;
        locals.var_ps0ld_ini__blk2013_dn0 = assign89290_e135871_d_n0;
        locals.var_ps0ld_ini__blk2013_dn2 = assign89290_e135871_d_n2;
        locals.var_ps0ld_ini__blk2013_dn4 = assign89290_e135871_d_n4;
        locals.var_ps0ld_ini__blk2013_dn5 = assign89290_e135871_d_n5;
        locals.var_ps0ld_ini__blk2013_dn6 = assign89290_e135871_d_n6;
        locals.var_ps0ld_ini__blk2013_dn7 = assign89290_e135871_d_n7;
        locals.var_ps0ld_ini__blk2013_dn8 = assign89290_e135871_d_n8;
        locals.var_ps0ld_ini__blk2013_dn9 = assign89290_e135871_d_n9;
        locals.var_ps0ld_ini__blk2013_dn10 = assign89290_e135871_d_n10;
        locals.var_ps0ld_ini__blk2013_dn11 = assign89290_e135871_d_n11;
        locals.var_ps0ld_ini__blk2013_dn14 = assign89290_e135871_d_n14;
        locals.var_ps0ld_ini__blk2013_rv = 0.0;

        let assign89300_e135874: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2069 = assign89300_e135874;
        locals.var_guard2069_rv = 0.0;

        let (assign89310_e135883,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign89310_e135883;
        locals.var_flg_conv_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_342(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89320_e135899, assign89320_e135899_d_n0, assign89320_e135899_d_n2, assign89320_e135899_d_n4, assign89320_e135899_d_n5, assign89320_e135899_d_n6, assign89320_e135899_d_n7, assign89320_e135899_d_n8, assign89320_e135899_d_n9, assign89320_e135899_d_n10, assign89320_e135899_d_n11, assign89320_e135899_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89320_e135893: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2006);
        let assign89320_e135895: f64 = (assign89320_e135893 * locals.var_beta_inv);
        let assign89320_e135896: f64 = (2.0 * assign89320_e135895);
        let assign89320_e135897: f64 = (assign89320_e135896).sqrt();
        (assign89320_e135897, ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn0)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn2)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn4)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn5)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn6)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn7)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn8)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn9)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn10)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn11)) / (2.0 * assign89320_e135897)), ((2.0 * (assign89320_e135893 * locals.var_beta_inv_dn14)) / (2.0 * assign89320_e135897)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn11, locals.var_c_w_ld_dn14,)
    }
};
        locals.var_c_w_ld = assign89320_e135899;
        locals.var_c_w_ld_dn0 = assign89320_e135899_d_n0;
        locals.var_c_w_ld_dn2 = assign89320_e135899_d_n2;
        locals.var_c_w_ld_dn4 = assign89320_e135899_d_n4;
        locals.var_c_w_ld_dn5 = assign89320_e135899_d_n5;
        locals.var_c_w_ld_dn6 = assign89320_e135899_d_n6;
        locals.var_c_w_ld_dn7 = assign89320_e135899_d_n7;
        locals.var_c_w_ld_dn8 = assign89320_e135899_d_n8;
        locals.var_c_w_ld_dn9 = assign89320_e135899_d_n9;
        locals.var_c_w_ld_dn10 = assign89320_e135899_d_n10;
        locals.var_c_w_ld_dn11 = assign89320_e135899_d_n11;
        locals.var_c_w_ld_dn14 = assign89320_e135899_d_n14;
        locals.var_c_w_ld_rv = 0.0;

        let assign89330_e135902: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2070 = assign89330_e135902;
        locals.var_guard2070_rv = 0.0;

        let (assign89340_e135915, assign89340_e135915_d_n0, assign89340_e135915_d_n2, assign89340_e135915_d_n4, assign89340_e135915_d_n5, assign89340_e135915_d_n6, assign89340_e135915_d_n7, assign89340_e135915_d_n8, assign89340_e135915_d_n9, assign89340_e135915_d_n10, assign89340_e135915_d_n11, assign89340_e135915_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2070 != 0.0)) {
        let assign89340_e135913: f64 = (p.p334 - locals.var_wdep_func);
        (assign89340_e135913, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89340_e135915;
        locals.var_t2_dn0 = assign89340_e135915_d_n0;
        locals.var_t2_dn2 = assign89340_e135915_d_n2;
        locals.var_t2_dn4 = assign89340_e135915_d_n4;
        locals.var_t2_dn5 = assign89340_e135915_d_n5;
        locals.var_t2_dn6 = assign89340_e135915_d_n6;
        locals.var_t2_dn7 = assign89340_e135915_d_n7;
        locals.var_t2_dn8 = assign89340_e135915_d_n8;
        locals.var_t2_dn9 = assign89340_e135915_d_n9;
        locals.var_t2_dn10 = assign89340_e135915_d_n10;
        locals.var_t2_dn11 = assign89340_e135915_d_n11;
        locals.var_t2_dn14 = assign89340_e135915_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign89350_e135940, assign89350_e135940_d_n0, assign89350_e135940_d_n2, assign89350_e135940_d_n4, assign89350_e135940_d_n5, assign89350_e135940_d_n6, assign89350_e135940_d_n7, assign89350_e135940_d_n8, assign89350_e135940_d_n9, assign89350_e135940_d_n10, assign89350_e135940_d_n11, assign89350_e135940_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2070 == 0.0)) {
        let assign89350_e135927: f64 = (locals.var_vdsi + p.p137);
        let assign89350_e135930: f64 = (locals.var_vdsi + p.p137);
        let assign89350_e135931: f64 = (assign89350_e135927 * assign89350_e135930);
        let assign89350_e135934: f64 = (4.0 * 0.1);
        let assign89350_e135936: f64 = (assign89350_e135934 * 0.1);
        let assign89350_e135937: f64 = (assign89350_e135931 + assign89350_e135936);
        let assign89350_e135938: f64 = (assign89350_e135937).sqrt();
        (assign89350_e135938, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign89350_e135930) + (assign89350_e135927 * locals.var_vdsi_dn6)) / (2.0 * assign89350_e135938)), 0.0, (((locals.var_vdsi_dn8 * assign89350_e135930) + (assign89350_e135927 * locals.var_vdsi_dn8)) / (2.0 * assign89350_e135938)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign89350_e135940;
        locals.var_tmf2_dn0 = assign89350_e135940_d_n0;
        locals.var_tmf2_dn2 = assign89350_e135940_d_n2;
        locals.var_tmf2_dn4 = assign89350_e135940_d_n4;
        locals.var_tmf2_dn5 = assign89350_e135940_d_n5;
        locals.var_tmf2_dn6 = assign89350_e135940_d_n6;
        locals.var_tmf2_dn7 = assign89350_e135940_d_n7;
        locals.var_tmf2_dn8 = assign89350_e135940_d_n8;
        locals.var_tmf2_dn9 = assign89350_e135940_d_n9;
        locals.var_tmf2_dn10 = assign89350_e135940_d_n10;
        locals.var_tmf2_dn11 = assign89350_e135940_d_n11;
        locals.var_tmf2_dn14 = assign89350_e135940_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign89360_e135960, assign89360_e135960_d_n0, assign89360_e135960_d_n2, assign89360_e135960_d_n4, assign89360_e135960_d_n5, assign89360_e135960_d_n6, assign89360_e135960_d_n7, assign89360_e135960_d_n8, assign89360_e135960_d_n9, assign89360_e135960_d_n10, assign89360_e135960_d_n11, assign89360_e135960_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2070 == 0.0)) {
        let assign89360_e135954: f64 = (locals.var_vdsi + p.p137);
        let assign89360_e135956: f64 = (assign89360_e135954 / locals.var_tmf2);
        let assign89360_e135957: f64 = (1.0 + assign89360_e135956);
        let assign89360_e135958: f64 = (0.5 * assign89360_e135957);
        (assign89360_e135958, (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign89360_e135954 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign89360_e135954 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89360_e135954 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign89360_e135960;
        locals.var_t9_dn0 = assign89360_e135960_d_n0;
        locals.var_t9_dn2 = assign89360_e135960_d_n2;
        locals.var_t9_dn4 = assign89360_e135960_d_n4;
        locals.var_t9_dn5 = assign89360_e135960_d_n5;
        locals.var_t9_dn6 = assign89360_e135960_d_n6;
        locals.var_t9_dn7 = assign89360_e135960_d_n7;
        locals.var_t9_dn8 = assign89360_e135960_d_n8;
        locals.var_t9_dn9 = assign89360_e135960_d_n9;
        locals.var_t9_dn10 = assign89360_e135960_d_n10;
        locals.var_t9_dn11 = assign89360_e135960_d_n11;
        locals.var_t9_dn14 = assign89360_e135960_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign89370_e135978, assign89370_e135978_d_n0, assign89370_e135978_d_n2, assign89370_e135978_d_n4, assign89370_e135978_d_n5, assign89370_e135978_d_n6, assign89370_e135978_d_n7, assign89370_e135978_d_n8, assign89370_e135978_d_n9, assign89370_e135978_d_n10, assign89370_e135978_d_n11, assign89370_e135978_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2070 == 0.0)) {
        let assign89370_e135973: f64 = (locals.var_vdsi + p.p137);
        let assign89370_e135975: f64 = (assign89370_e135973 + locals.var_tmf2);
        let assign89370_e135976: f64 = (0.5 * assign89370_e135975);
        (assign89370_e135976, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89370_e135978;
        locals.var_t2_dn0 = assign89370_e135978_d_n0;
        locals.var_t2_dn2 = assign89370_e135978_d_n2;
        locals.var_t2_dn4 = assign89370_e135978_d_n4;
        locals.var_t2_dn5 = assign89370_e135978_d_n5;
        locals.var_t2_dn6 = assign89370_e135978_d_n6;
        locals.var_t2_dn7 = assign89370_e135978_d_n7;
        locals.var_t2_dn8 = assign89370_e135978_d_n8;
        locals.var_t2_dn9 = assign89370_e135978_d_n9;
        locals.var_t2_dn10 = assign89370_e135978_d_n10;
        locals.var_t2_dn11 = assign89370_e135978_d_n11;
        locals.var_t2_dn14 = assign89370_e135978_d_n14;
        locals.var_t2_rv = 0.0;

        let assign89380_e135981: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2071 = assign89380_e135981;
        locals.var_guard2071_rv = 0.0;

        let (assign89390_e135995, assign89390_e135995_d_n0, assign89390_e135995_d_n2, assign89390_e135995_d_n4, assign89390_e135995_d_n5, assign89390_e135995_d_n6, assign89390_e135995_d_n7, assign89390_e135995_d_n8, assign89390_e135995_d_n9, assign89390_e135995_d_n10, assign89390_e135995_d_n11, assign89390_e135995_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2070 == 0.0)) && (locals.var_guard2071 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89390_e135995;
        locals.var_t2_dn0 = assign89390_e135995_d_n0;
        locals.var_t2_dn2 = assign89390_e135995_d_n2;
        locals.var_t2_dn4 = assign89390_e135995_d_n4;
        locals.var_t2_dn5 = assign89390_e135995_d_n5;
        locals.var_t2_dn6 = assign89390_e135995_d_n6;
        locals.var_t2_dn7 = assign89390_e135995_d_n7;
        locals.var_t2_dn8 = assign89390_e135995_d_n8;
        locals.var_t2_dn9 = assign89390_e135995_d_n9;
        locals.var_t2_dn10 = assign89390_e135995_d_n10;
        locals.var_t2_dn11 = assign89390_e135995_d_n11;
        locals.var_t2_dn14 = assign89390_e135995_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign89400_e136009, assign89400_e136009_d_n0, assign89400_e136009_d_n2, assign89400_e136009_d_n4, assign89400_e136009_d_n5, assign89400_e136009_d_n6, assign89400_e136009_d_n7, assign89400_e136009_d_n8, assign89400_e136009_d_n9, assign89400_e136009_d_n10, assign89400_e136009_d_n11, assign89400_e136009_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2070 == 0.0)) && (locals.var_guard2071 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign89400_e136009;
        locals.var_t9_dn0 = assign89400_e136009_d_n0;
        locals.var_t9_dn2 = assign89400_e136009_d_n2;
        locals.var_t9_dn4 = assign89400_e136009_d_n4;
        locals.var_t9_dn5 = assign89400_e136009_d_n5;
        locals.var_t9_dn6 = assign89400_e136009_d_n6;
        locals.var_t9_dn7 = assign89400_e136009_d_n7;
        locals.var_t9_dn8 = assign89400_e136009_d_n8;
        locals.var_t9_dn9 = assign89400_e136009_d_n9;
        locals.var_t9_dn10 = assign89400_e136009_d_n10;
        locals.var_t9_dn11 = assign89400_e136009_d_n11;
        locals.var_t9_dn14 = assign89400_e136009_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign89410_e136026, assign89410_e136026_d_n0, assign89410_e136026_d_n2, assign89410_e136026_d_n4, assign89410_e136026_d_n5, assign89410_e136026_d_n6, assign89410_e136026_d_n7, assign89410_e136026_d_n8, assign89410_e136026_d_n9, assign89410_e136026_d_n10, assign89410_e136026_d_n11, assign89410_e136026_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2070 == 0.0)) {
        let assign89410_e136021: f64 = (locals.var_kjunc * locals.var_t2);
        let assign89410_e136022: f64 = (assign89410_e136021).sqrt();
        let assign89410_e136024: f64 = (assign89410_e136022 * p.p432);
        (assign89410_e136024, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign89410_e136022)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign89410_e136022)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign89410_e136026;
        locals.var_wjunc0_dn0 = assign89410_e136026_d_n0;
        locals.var_wjunc0_dn2 = assign89410_e136026_d_n2;
        locals.var_wjunc0_dn4 = assign89410_e136026_d_n4;
        locals.var_wjunc0_dn5 = assign89410_e136026_d_n5;
        locals.var_wjunc0_dn6 = assign89410_e136026_d_n6;
        locals.var_wjunc0_dn7 = assign89410_e136026_d_n7;
        locals.var_wjunc0_dn8 = assign89410_e136026_d_n8;
        locals.var_wjunc0_dn9 = assign89410_e136026_d_n9;
        locals.var_wjunc0_dn10 = assign89410_e136026_d_n10;
        locals.var_wjunc0_dn11 = assign89410_e136026_d_n11;
        locals.var_wjunc0_dn14 = assign89410_e136026_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign89420_e136040, assign89420_e136040_d_n0, assign89420_e136040_d_n2, assign89420_e136040_d_n4, assign89420_e136040_d_n5, assign89420_e136040_d_n6, assign89420_e136040_d_n7, assign89420_e136040_d_n8, assign89420_e136040_d_n9, assign89420_e136040_d_n10, assign89420_e136040_d_n11, assign89420_e136040_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2070 == 0.0)) {
        let assign89420_e136038: f64 = (p.p334 - locals.var_wjunc0);
        (assign89420_e136038, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89420_e136040;
        locals.var_t2_dn0 = assign89420_e136040_d_n0;
        locals.var_t2_dn2 = assign89420_e136040_d_n2;
        locals.var_t2_dn4 = assign89420_e136040_d_n4;
        locals.var_t2_dn5 = assign89420_e136040_d_n5;
        locals.var_t2_dn6 = assign89420_e136040_d_n6;
        locals.var_t2_dn7 = assign89420_e136040_d_n7;
        locals.var_t2_dn8 = assign89420_e136040_d_n8;
        locals.var_t2_dn9 = assign89420_e136040_d_n9;
        locals.var_t2_dn10 = assign89420_e136040_d_n10;
        locals.var_t2_dn11 = assign89420_e136040_d_n11;
        locals.var_t2_dn14 = assign89420_e136040_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign89430_e136062, assign89430_e136062_d_n0, assign89430_e136062_d_n2, assign89430_e136062_d_n4, assign89430_e136062_d_n5, assign89430_e136062_d_n6, assign89430_e136062_d_n7, assign89430_e136062_d_n8, assign89430_e136062_d_n9, assign89430_e136062_d_n10, assign89430_e136062_d_n11, assign89430_e136062_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89430_e136049: f64 = (locals.var_t2 * locals.var_t2);
        let assign89430_e136053: f64 = (p.p334 * 0.01);
        let assign89430_e136054: f64 = (4.0 * assign89430_e136053);
        let assign89430_e136057: f64 = (p.p334 * 0.01);
        let assign89430_e136058: f64 = (assign89430_e136054 * assign89430_e136057);
        let assign89430_e136059: f64 = (assign89430_e136049 + assign89430_e136058);
        let assign89430_e136060: f64 = (assign89430_e136059).sqrt();
        (assign89430_e136060, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign89430_e136060)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign89430_e136060)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign89430_e136062;
        locals.var_tmf2_dn0 = assign89430_e136062_d_n0;
        locals.var_tmf2_dn2 = assign89430_e136062_d_n2;
        locals.var_tmf2_dn4 = assign89430_e136062_d_n4;
        locals.var_tmf2_dn5 = assign89430_e136062_d_n5;
        locals.var_tmf2_dn6 = assign89430_e136062_d_n6;
        locals.var_tmf2_dn7 = assign89430_e136062_d_n7;
        locals.var_tmf2_dn8 = assign89430_e136062_d_n8;
        locals.var_tmf2_dn9 = assign89430_e136062_d_n9;
        locals.var_tmf2_dn10 = assign89430_e136062_d_n10;
        locals.var_tmf2_dn11 = assign89430_e136062_d_n11;
        locals.var_tmf2_dn14 = assign89430_e136062_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign89440_e136077, assign89440_e136077_d_n0, assign89440_e136077_d_n2, assign89440_e136077_d_n4, assign89440_e136077_d_n5, assign89440_e136077_d_n6, assign89440_e136077_d_n7, assign89440_e136077_d_n8, assign89440_e136077_d_n9, assign89440_e136077_d_n10, assign89440_e136077_d_n11, assign89440_e136077_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89440_e136073: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign89440_e136074: f64 = (1.0 + assign89440_e136073);
        let assign89440_e136075: f64 = (0.5 * assign89440_e136074);
        (assign89440_e136075, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign89440_e136077;
        locals.var_t9_dn0 = assign89440_e136077_d_n0;
        locals.var_t9_dn2 = assign89440_e136077_d_n2;
        locals.var_t9_dn4 = assign89440_e136077_d_n4;
        locals.var_t9_dn5 = assign89440_e136077_d_n5;
        locals.var_t9_dn6 = assign89440_e136077_d_n6;
        locals.var_t9_dn7 = assign89440_e136077_d_n7;
        locals.var_t9_dn8 = assign89440_e136077_d_n8;
        locals.var_t9_dn9 = assign89440_e136077_d_n9;
        locals.var_t9_dn10 = assign89440_e136077_d_n10;
        locals.var_t9_dn11 = assign89440_e136077_d_n11;
        locals.var_t9_dn14 = assign89440_e136077_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign89450_e136090, assign89450_e136090_d_n0, assign89450_e136090_d_n2, assign89450_e136090_d_n4, assign89450_e136090_d_n5, assign89450_e136090_d_n6, assign89450_e136090_d_n7, assign89450_e136090_d_n8, assign89450_e136090_d_n9, assign89450_e136090_d_n10, assign89450_e136090_d_n11, assign89450_e136090_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89450_e136087: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign89450_e136088: f64 = (0.5 * assign89450_e136087);
        (assign89450_e136088, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89450_e136090;
        locals.var_t2_dn0 = assign89450_e136090_d_n0;
        locals.var_t2_dn2 = assign89450_e136090_d_n2;
        locals.var_t2_dn4 = assign89450_e136090_d_n4;
        locals.var_t2_dn5 = assign89450_e136090_d_n5;
        locals.var_t2_dn6 = assign89450_e136090_d_n6;
        locals.var_t2_dn7 = assign89450_e136090_d_n7;
        locals.var_t2_dn8 = assign89450_e136090_d_n8;
        locals.var_t2_dn9 = assign89450_e136090_d_n9;
        locals.var_t2_dn10 = assign89450_e136090_d_n10;
        locals.var_t2_dn11 = assign89450_e136090_d_n11;
        locals.var_t2_dn14 = assign89450_e136090_d_n14;
        locals.var_t2_rv = 0.0;

        let assign89460_e136093: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2072 = assign89460_e136093;
        locals.var_guard2072_rv = 0.0;

        let (assign89470_e136104, assign89470_e136104_d_n0, assign89470_e136104_d_n2, assign89470_e136104_d_n4, assign89470_e136104_d_n5, assign89470_e136104_d_n6, assign89470_e136104_d_n7, assign89470_e136104_d_n8, assign89470_e136104_d_n9, assign89470_e136104_d_n10, assign89470_e136104_d_n11, assign89470_e136104_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89470_e136104;
        locals.var_t2_dn0 = assign89470_e136104_d_n0;
        locals.var_t2_dn2 = assign89470_e136104_d_n2;
        locals.var_t2_dn4 = assign89470_e136104_d_n4;
        locals.var_t2_dn5 = assign89470_e136104_d_n5;
        locals.var_t2_dn6 = assign89470_e136104_d_n6;
        locals.var_t2_dn7 = assign89470_e136104_d_n7;
        locals.var_t2_dn8 = assign89470_e136104_d_n8;
        locals.var_t2_dn9 = assign89470_e136104_d_n9;
        locals.var_t2_dn10 = assign89470_e136104_d_n10;
        locals.var_t2_dn11 = assign89470_e136104_d_n11;
        locals.var_t2_dn14 = assign89470_e136104_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign89480_e136115, assign89480_e136115_d_n0, assign89480_e136115_d_n2, assign89480_e136115_d_n4, assign89480_e136115_d_n5, assign89480_e136115_d_n6, assign89480_e136115_d_n7, assign89480_e136115_d_n8, assign89480_e136115_d_n9, assign89480_e136115_d_n10, assign89480_e136115_d_n11, assign89480_e136115_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign89480_e136115;
        locals.var_t9_dn0 = assign89480_e136115_d_n0;
        locals.var_t9_dn2 = assign89480_e136115_d_n2;
        locals.var_t9_dn4 = assign89480_e136115_d_n4;
        locals.var_t9_dn5 = assign89480_e136115_d_n5;
        locals.var_t9_dn6 = assign89480_e136115_d_n6;
        locals.var_t9_dn7 = assign89480_e136115_d_n7;
        locals.var_t9_dn8 = assign89480_e136115_d_n8;
        locals.var_t9_dn9 = assign89480_e136115_d_n9;
        locals.var_t9_dn10 = assign89480_e136115_d_n10;
        locals.var_t9_dn11 = assign89480_e136115_d_n11;
        locals.var_t9_dn14 = assign89480_e136115_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign89490_e136124, assign89490_e136124_d_n0, assign89490_e136124_d_n2, assign89490_e136124_d_n4, assign89490_e136124_d_n5, assign89490_e136124_d_n6, assign89490_e136124_d_n7, assign89490_e136124_d_n8, assign89490_e136124_d_n9, assign89490_e136124_d_n10, assign89490_e136124_d_n11, assign89490_e136124_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign89490_e136124;
        locals.var_ddriftldc_dn0 = assign89490_e136124_d_n0;
        locals.var_ddriftldc_dn2 = assign89490_e136124_d_n2;
        locals.var_ddriftldc_dn4 = assign89490_e136124_d_n4;
        locals.var_ddriftldc_dn5 = assign89490_e136124_d_n5;
        locals.var_ddriftldc_dn6 = assign89490_e136124_d_n6;
        locals.var_ddriftldc_dn7 = assign89490_e136124_d_n7;
        locals.var_ddriftldc_dn8 = assign89490_e136124_d_n8;
        locals.var_ddriftldc_dn9 = assign89490_e136124_d_n9;
        locals.var_ddriftldc_dn10 = assign89490_e136124_d_n10;
        locals.var_ddriftldc_dn11 = assign89490_e136124_d_n11;
        locals.var_ddriftldc_dn14 = assign89490_e136124_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign89500_e136141, assign89500_e136141_d_n0, assign89500_e136141_d_n2, assign89500_e136141_d_n4, assign89500_e136141_d_n5, assign89500_e136141_d_n6, assign89500_e136141_d_n7, assign89500_e136141_d_n8, assign89500_e136141_d_n9, assign89500_e136141_d_n10, assign89500_e136141_d_n11, assign89500_e136141_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89500_e136133: f64 = (locals.var_q_nsubld__blk2006 * locals.var_ddriftldc);
        let assign89500_e136135: f64 = (assign89500_e136133 * locals.var_ddriftldc);
        let assign89500_e136137: f64 = (assign89500_e136135 / 2.0);
        let assign89500_e136139: f64 = (assign89500_e136137 / 1.034943e-10);
        (assign89500_e136139, (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign89500_e136133 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign89500_e136141;
        locals.var_dphi_sb_dn0 = assign89500_e136141_d_n0;
        locals.var_dphi_sb_dn2 = assign89500_e136141_d_n2;
        locals.var_dphi_sb_dn4 = assign89500_e136141_d_n4;
        locals.var_dphi_sb_dn5 = assign89500_e136141_d_n5;
        locals.var_dphi_sb_dn6 = assign89500_e136141_d_n6;
        locals.var_dphi_sb_dn7 = assign89500_e136141_d_n7;
        locals.var_dphi_sb_dn8 = assign89500_e136141_d_n8;
        locals.var_dphi_sb_dn9 = assign89500_e136141_d_n9;
        locals.var_dphi_sb_dn10 = assign89500_e136141_d_n10;
        locals.var_dphi_sb_dn11 = assign89500_e136141_d_n11;
        locals.var_dphi_sb_dn14 = assign89500_e136141_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign89510_e136155, assign89510_e136155_d_n0, assign89510_e136155_d_n2, assign89510_e136155_d_n4, assign89510_e136155_d_n5, assign89510_e136155_d_n6, assign89510_e136155_d_n7, assign89510_e136155_d_n8, assign89510_e136155_d_n9, assign89510_e136155_d_n10, assign89510_e136155_d_n11, assign89510_e136155_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89510_e136150: f64 = (2.0 * locals.var_beta);
        let assign89510_e136152: f64 = (assign89510_e136150 * locals.var_dphi_sb);
        let assign89510_e136153: f64 = (assign89510_e136152).sqrt();
        (assign89510_e136153, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn0)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn2)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn4)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn5)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn6)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn7)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn8)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn9)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn10)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn11)) / (2.0 * assign89510_e136153)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign89510_e136150 * locals.var_dphi_sb_dn14)) / (2.0 * assign89510_e136153)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign89510_e136155;
        locals.var_t0_dn0 = assign89510_e136155_d_n0;
        locals.var_t0_dn2 = assign89510_e136155_d_n2;
        locals.var_t0_dn4 = assign89510_e136155_d_n4;
        locals.var_t0_dn5 = assign89510_e136155_d_n5;
        locals.var_t0_dn6 = assign89510_e136155_d_n6;
        locals.var_t0_dn7 = assign89510_e136155_d_n7;
        locals.var_t0_dn8 = assign89510_e136155_d_n8;
        locals.var_t0_dn9 = assign89510_e136155_d_n9;
        locals.var_t0_dn10 = assign89510_e136155_d_n10;
        locals.var_t0_dn11 = assign89510_e136155_d_n11;
        locals.var_t0_dn14 = assign89510_e136155_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign89520_e136171, assign89520_e136171_d_n0, assign89520_e136171_d_n2, assign89520_e136171_d_n4, assign89520_e136171_d_n5, assign89520_e136171_d_n6, assign89520_e136171_d_n7, assign89520_e136171_d_n8, assign89520_e136171_d_n9, assign89520_e136171_d_n10, assign89520_e136171_d_n11, assign89520_e136171_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89520_e136163: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89520_e136165: f64 = (-locals.var_t0);
        let assign89520_e136166: f64 = { let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89520_e136167: f64 = (assign89520_e136163 + assign89520_e136166);
        let assign89520_e136169: f64 = (assign89520_e136167 / 2.0);
        (assign89520_e136169, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign89520_e136165; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign89520_e136171;
        locals.var_t1_dn0 = assign89520_e136171_d_n0;
        locals.var_t1_dn2 = assign89520_e136171_d_n2;
        locals.var_t1_dn4 = assign89520_e136171_d_n4;
        locals.var_t1_dn5 = assign89520_e136171_d_n5;
        locals.var_t1_dn6 = assign89520_e136171_d_n6;
        locals.var_t1_dn7 = assign89520_e136171_d_n7;
        locals.var_t1_dn8 = assign89520_e136171_d_n8;
        locals.var_t1_dn9 = assign89520_e136171_d_n9;
        locals.var_t1_dn10 = assign89520_e136171_d_n10;
        locals.var_t1_dn11 = assign89520_e136171_d_n11;
        locals.var_t1_dn14 = assign89520_e136171_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign89530_e136183, assign89530_e136183_d_n0, assign89530_e136183_d_n2, assign89530_e136183_d_n4, assign89530_e136183_d_n5, assign89530_e136183_d_n6, assign89530_e136183_d_n7, assign89530_e136183_d_n8, assign89530_e136183_d_n9, assign89530_e136183_d_n10, assign89530_e136183_d_n11, assign89530_e136183_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89530_e136179: f64 = (locals.var_t1).ln();
        let assign89530_e136181: f64 = (assign89530_e136179 / locals.var_dphi_sb);
        (assign89530_e136181, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign89530_e136179 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign89530_e136183;
        locals.var_c_sb_dn0 = assign89530_e136183_d_n0;
        locals.var_c_sb_dn2 = assign89530_e136183_d_n2;
        locals.var_c_sb_dn4 = assign89530_e136183_d_n4;
        locals.var_c_sb_dn5 = assign89530_e136183_d_n5;
        locals.var_c_sb_dn6 = assign89530_e136183_d_n6;
        locals.var_c_sb_dn7 = assign89530_e136183_d_n7;
        locals.var_c_sb_dn8 = assign89530_e136183_d_n8;
        locals.var_c_sb_dn9 = assign89530_e136183_d_n9;
        locals.var_c_sb_dn10 = assign89530_e136183_d_n10;
        locals.var_c_sb_dn11 = assign89530_e136183_d_n11;
        locals.var_c_sb_dn14 = assign89530_e136183_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign89540_e136192,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign89540_e136192;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_343(
        locals: &mut StampLocals,
    ) {
        let mut assign89550_loop_guard: usize = 0;
        while {
            let assign89550_cond_e136202: f64 = (locals.var_lp_s0_max + 1.0);
            let assign89550_cond_e136204: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_lp_s0 <= assign89550_cond_e136202)) { 1.0 } else { 0.0 };
            assign89550_cond_e136204 != 0.0
        } {
            assign89550_loop_guard += 1;
            assert!(assign89550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89550_body3_e136240, assign89550_body3_e136240_d_n0, assign89550_body3_e136240_d_n2, assign89550_body3_e136240_d_n4, assign89550_body3_e136240_d_n5, assign89550_body3_e136240_d_n6, assign89550_body3_e136240_d_n7, assign89550_body3_e136240_d_n8, assign89550_body3_e136240_d_n9, assign89550_body3_e136240_d_n10, assign89550_body3_e136240_d_n11, assign89550_body3_e136240_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89550_body3_e136238: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign89550_body3_e136238, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign89550_body3_e136240;
            locals.var_ps0ld_vxb_dn0 = assign89550_body3_e136240_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign89550_body3_e136240_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign89550_body3_e136240_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign89550_body3_e136240_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign89550_body3_e136240_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign89550_body3_e136240_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign89550_body3_e136240_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign89550_body3_e136240_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign89550_body3_e136240_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign89550_body3_e136240_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign89550_body3_e136240_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign89550_body4_e136251, assign89550_body4_e136251_d_n0, assign89550_body4_e136251_d_n2, assign89550_body4_e136251_d_n4, assign89550_body4_e136251_d_n5, assign89550_body4_e136251_d_n6, assign89550_body4_e136251_d_n7, assign89550_body4_e136251_d_n8, assign89550_body4_e136251_d_n9, assign89550_body4_e136251_d_n10, assign89550_body4_e136251_d_n11, assign89550_body4_e136251_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89550_body4_e136249: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign89550_body4_e136249, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign89550_body4_e136251;
            locals.var_chi_dn0 = assign89550_body4_e136251_d_n0;
            locals.var_chi_dn2 = assign89550_body4_e136251_d_n2;
            locals.var_chi_dn4 = assign89550_body4_e136251_d_n4;
            locals.var_chi_dn5 = assign89550_body4_e136251_d_n5;
            locals.var_chi_dn6 = assign89550_body4_e136251_d_n6;
            locals.var_chi_dn7 = assign89550_body4_e136251_d_n7;
            locals.var_chi_dn8 = assign89550_body4_e136251_d_n8;
            locals.var_chi_dn9 = assign89550_body4_e136251_d_n9;
            locals.var_chi_dn10 = assign89550_body4_e136251_d_n10;
            locals.var_chi_dn11 = assign89550_body4_e136251_d_n11;
            locals.var_chi_dn14 = assign89550_body4_e136251_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign89550_body5_e136264, assign89550_body5_e136264_d_n0, assign89550_body5_e136264_d_n2, assign89550_body5_e136264_d_n4, assign89550_body5_e136264_d_n5, assign89550_body5_e136264_d_n6, assign89550_body5_e136264_d_n7, assign89550_body5_e136264_d_n8, assign89550_body5_e136264_d_n9, assign89550_body5_e136264_d_n10, assign89550_body5_e136264_d_n11, assign89550_body5_e136264_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89550_body5_e136261: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign89550_body5_e136262: f64 = (locals.var_c_sb * assign89550_body5_e136261);
        (assign89550_body5_e136262, ((locals.var_c_sb_dn0 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign89550_body5_e136261) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign89550_body5_e136264;
            locals.var_ty_dn0 = assign89550_body5_e136264_d_n0;
            locals.var_ty_dn2 = assign89550_body5_e136264_d_n2;
            locals.var_ty_dn4 = assign89550_body5_e136264_d_n4;
            locals.var_ty_dn5 = assign89550_body5_e136264_d_n5;
            locals.var_ty_dn6 = assign89550_body5_e136264_d_n6;
            locals.var_ty_dn7 = assign89550_body5_e136264_d_n7;
            locals.var_ty_dn8 = assign89550_body5_e136264_d_n8;
            locals.var_ty_dn9 = assign89550_body5_e136264_d_n9;
            locals.var_ty_dn10 = assign89550_body5_e136264_d_n10;
            locals.var_ty_dn11 = assign89550_body5_e136264_d_n11;
            locals.var_ty_dn14 = assign89550_body5_e136264_d_n14;
            locals.var_ty_rv = 0.0;
            let assign89550_body6_e136267: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2074 = assign89550_body6_e136267;
            locals.var_guard2074_rv = 0.0;
            let (assign89550_body7_e136279, assign89550_body7_e136279_d_n0, assign89550_body7_e136279_d_n2, assign89550_body7_e136279_d_n4, assign89550_body7_e136279_d_n5, assign89550_body7_e136279_d_n6, assign89550_body7_e136279_d_n7, assign89550_body7_e136279_d_n8, assign89550_body7_e136279_d_n9, assign89550_body7_e136279_d_n10, assign89550_body7_e136279_d_n11, assign89550_body7_e136279_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89550_body7_e136277: f64 = (locals.var_ty).exp();
        (assign89550_body7_e136277, (assign89550_body7_e136277 * locals.var_ty_dn0), (assign89550_body7_e136277 * locals.var_ty_dn2), (assign89550_body7_e136277 * locals.var_ty_dn4), (assign89550_body7_e136277 * locals.var_ty_dn5), (assign89550_body7_e136277 * locals.var_ty_dn6), (assign89550_body7_e136277 * locals.var_ty_dn7), (assign89550_body7_e136277 * locals.var_ty_dn8), (assign89550_body7_e136277 * locals.var_ty_dn9), (assign89550_body7_e136277 * locals.var_ty_dn10), (assign89550_body7_e136277 * locals.var_ty_dn11), (assign89550_body7_e136277 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89550_body7_e136279;
            locals.var_t1_dn0 = assign89550_body7_e136279_d_n0;
            locals.var_t1_dn2 = assign89550_body7_e136279_d_n2;
            locals.var_t1_dn4 = assign89550_body7_e136279_d_n4;
            locals.var_t1_dn5 = assign89550_body7_e136279_d_n5;
            locals.var_t1_dn6 = assign89550_body7_e136279_d_n6;
            locals.var_t1_dn7 = assign89550_body7_e136279_d_n7;
            locals.var_t1_dn8 = assign89550_body7_e136279_d_n8;
            locals.var_t1_dn9 = assign89550_body7_e136279_d_n9;
            locals.var_t1_dn10 = assign89550_body7_e136279_d_n10;
            locals.var_t1_dn11 = assign89550_body7_e136279_d_n11;
            locals.var_t1_dn14 = assign89550_body7_e136279_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89550_body8_e136294, assign89550_body8_e136294_d_n0, assign89550_body8_e136294_d_n2, assign89550_body8_e136294_d_n4, assign89550_body8_e136294_d_n5, assign89550_body8_e136294_d_n6, assign89550_body8_e136294_d_n7, assign89550_body8_e136294_d_n8, assign89550_body8_e136294_d_n9, assign89550_body8_e136294_d_n10, assign89550_body8_e136294_d_n11, assign89550_body8_e136294_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89550_body8_e136289: f64 = (-locals.var_c_sb);
        let assign89550_body8_e136291: f64 = (assign89550_body8_e136289 * locals.var_dphi_sb);
        let assign89550_body8_e136292: f64 = (assign89550_body8_e136291).exp();
        (assign89550_body8_e136292, (assign89550_body8_e136292 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn0))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn2))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn4))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn5))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn6))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn7))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn8))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn9))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn10))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn11))), (assign89550_body8_e136292 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign89550_body8_e136289 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89550_body8_e136294;
            locals.var_t0_dn0 = assign89550_body8_e136294_d_n0;
            locals.var_t0_dn2 = assign89550_body8_e136294_d_n2;
            locals.var_t0_dn4 = assign89550_body8_e136294_d_n4;
            locals.var_t0_dn5 = assign89550_body8_e136294_d_n5;
            locals.var_t0_dn6 = assign89550_body8_e136294_d_n6;
            locals.var_t0_dn7 = assign89550_body8_e136294_d_n7;
            locals.var_t0_dn8 = assign89550_body8_e136294_d_n8;
            locals.var_t0_dn9 = assign89550_body8_e136294_d_n9;
            locals.var_t0_dn10 = assign89550_body8_e136294_d_n10;
            locals.var_t0_dn11 = assign89550_body8_e136294_d_n11;
            locals.var_t0_dn14 = assign89550_body8_e136294_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89550_body9_e136307, assign89550_body9_e136307_d_n0, assign89550_body9_e136307_d_n2, assign89550_body9_e136307_d_n4, assign89550_body9_e136307_d_n5, assign89550_body9_e136307_d_n6, assign89550_body9_e136307_d_n7, assign89550_body9_e136307_d_n8, assign89550_body9_e136307_d_n9, assign89550_body9_e136307_d_n10, assign89550_body9_e136307_d_n11, assign89550_body9_e136307_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89550_body9_e136305: f64 = (locals.var_t1 - locals.var_t0);
        (assign89550_body9_e136305, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign89550_body9_e136307;
            locals.var_t2_dn0 = assign89550_body9_e136307_d_n0;
            locals.var_t2_dn2 = assign89550_body9_e136307_d_n2;
            locals.var_t2_dn4 = assign89550_body9_e136307_d_n4;
            locals.var_t2_dn5 = assign89550_body9_e136307_d_n5;
            locals.var_t2_dn6 = assign89550_body9_e136307_d_n6;
            locals.var_t2_dn7 = assign89550_body9_e136307_d_n7;
            locals.var_t2_dn8 = assign89550_body9_e136307_d_n8;
            locals.var_t2_dn9 = assign89550_body9_e136307_d_n9;
            locals.var_t2_dn10 = assign89550_body9_e136307_d_n10;
            locals.var_t2_dn11 = assign89550_body9_e136307_d_n11;
            locals.var_t2_dn14 = assign89550_body9_e136307_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign89550_body10_e136323, assign89550_body10_e136323_d_n0, assign89550_body10_e136323_d_n2, assign89550_body10_e136323_d_n4, assign89550_body10_e136323_d_n5, assign89550_body10_e136323_d_n6, assign89550_body10_e136323_d_n7, assign89550_body10_e136323_d_n8, assign89550_body10_e136323_d_n9, assign89550_body10_e136323_d_n10, assign89550_body10_e136323_d_n11, assign89550_body10_e136323_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89550_body10_e136318: f64 = (1.0 + locals.var_t2);
        let assign89550_body10_e136319: f64 = (assign89550_body10_e136318).ln();
        let assign89550_body10_e136321: f64 = (assign89550_body10_e136319 / locals.var_c_sb);
        (assign89550_body10_e136321, ((((locals.var_t2_dn0 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign89550_body10_e136318) * locals.var_c_sb) - (assign89550_body10_e136319 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign89550_body10_e136323;
            locals.var_phi_b_dn0 = assign89550_body10_e136323_d_n0;
            locals.var_phi_b_dn2 = assign89550_body10_e136323_d_n2;
            locals.var_phi_b_dn4 = assign89550_body10_e136323_d_n4;
            locals.var_phi_b_dn5 = assign89550_body10_e136323_d_n5;
            locals.var_phi_b_dn6 = assign89550_body10_e136323_d_n6;
            locals.var_phi_b_dn7 = assign89550_body10_e136323_d_n7;
            locals.var_phi_b_dn8 = assign89550_body10_e136323_d_n8;
            locals.var_phi_b_dn9 = assign89550_body10_e136323_d_n9;
            locals.var_phi_b_dn10 = assign89550_body10_e136323_d_n10;
            locals.var_phi_b_dn11 = assign89550_body10_e136323_d_n11;
            locals.var_phi_b_dn14 = assign89550_body10_e136323_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign89550_body11_e136338, assign89550_body11_e136338_d_n0, assign89550_body11_e136338_d_n2, assign89550_body11_e136338_d_n4, assign89550_body11_e136338_d_n5, assign89550_body11_e136338_d_n6, assign89550_body11_e136338_d_n7, assign89550_body11_e136338_d_n8, assign89550_body11_e136338_d_n9, assign89550_body11_e136338_d_n10, assign89550_body11_e136338_d_n11, assign89550_body11_e136338_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89550_body11_e136335: f64 = (1.0 + locals.var_t2);
        let assign89550_body11_e136336: f64 = (locals.var_t1 / assign89550_body11_e136335);
        (assign89550_body11_e136336, (((locals.var_t1_dn0 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn0)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn2 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn2)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn4 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn4)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn5 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn5)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn6 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn6)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn7 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn7)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn8 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn8)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn9 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn9)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn10 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn10)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn11 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn11)) / (assign89550_body11_e136335 * assign89550_body11_e136335)), (((locals.var_t1_dn14 * assign89550_body11_e136335) - (locals.var_t1 * locals.var_t2_dn14)) / (assign89550_body11_e136335 * assign89550_body11_e136335)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign89550_body11_e136338;
            locals.var_phi_b_dpss_dn0 = assign89550_body11_e136338_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89550_body11_e136338_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89550_body11_e136338_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89550_body11_e136338_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89550_body11_e136338_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89550_body11_e136338_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89550_body11_e136338_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89550_body11_e136338_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89550_body11_e136338_d_n10;
            locals.var_phi_b_dpss_dn11 = assign89550_body11_e136338_d_n11;
            locals.var_phi_b_dpss_dn14 = assign89550_body11_e136338_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89550_body13_e136366, assign89550_body13_e136366_d_n0, assign89550_body13_e136366_d_n2, assign89550_body13_e136366_d_n4, assign89550_body13_e136366_d_n5, assign89550_body13_e136366_d_n6, assign89550_body13_e136366_d_n7, assign89550_body13_e136366_d_n8, assign89550_body13_e136366_d_n9, assign89550_body13_e136366_d_n10, assign89550_body13_e136366_d_n11, assign89550_body13_e136366_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2074 == 0.0)) {
        let assign89550_body13_e136364: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign89550_body13_e136364, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign89550_body13_e136366;
            locals.var_phi_b_dn0 = assign89550_body13_e136366_d_n0;
            locals.var_phi_b_dn2 = assign89550_body13_e136366_d_n2;
            locals.var_phi_b_dn4 = assign89550_body13_e136366_d_n4;
            locals.var_phi_b_dn5 = assign89550_body13_e136366_d_n5;
            locals.var_phi_b_dn6 = assign89550_body13_e136366_d_n6;
            locals.var_phi_b_dn7 = assign89550_body13_e136366_d_n7;
            locals.var_phi_b_dn8 = assign89550_body13_e136366_d_n8;
            locals.var_phi_b_dn9 = assign89550_body13_e136366_d_n9;
            locals.var_phi_b_dn10 = assign89550_body13_e136366_d_n10;
            locals.var_phi_b_dn11 = assign89550_body13_e136366_d_n11;
            locals.var_phi_b_dn14 = assign89550_body13_e136366_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign89550_body14_e136378, assign89550_body14_e136378_d_n0, assign89550_body14_e136378_d_n2, assign89550_body14_e136378_d_n4, assign89550_body14_e136378_d_n5, assign89550_body14_e136378_d_n6, assign89550_body14_e136378_d_n7, assign89550_body14_e136378_d_n8, assign89550_body14_e136378_d_n9, assign89550_body14_e136378_d_n10, assign89550_body14_e136378_d_n11, assign89550_body14_e136378_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2074 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign89550_body14_e136378;
            locals.var_phi_b_dpss_dn0 = assign89550_body14_e136378_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89550_body14_e136378_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89550_body14_e136378_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89550_body14_e136378_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89550_body14_e136378_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89550_body14_e136378_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89550_body14_e136378_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89550_body14_e136378_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89550_body14_e136378_d_n10;
            locals.var_phi_b_dpss_dn11 = assign89550_body14_e136378_d_n11;
            locals.var_phi_b_dpss_dn14 = assign89550_body14_e136378_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89550_body15_e136389, assign89550_body15_e136389_d_n0, assign89550_body15_e136389_d_n2, assign89550_body15_e136389_d_n4, assign89550_body15_e136389_d_n5, assign89550_body15_e136389_d_n6, assign89550_body15_e136389_d_n7, assign89550_body15_e136389_d_n8, assign89550_body15_e136389_d_n9, assign89550_body15_e136389_d_n10, assign89550_body15_e136389_d_n11, assign89550_body15_e136389_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89550_body15_e136387: f64 = (locals.var_beta * locals.var_phi_b);
        (assign89550_body15_e136387, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign89550_body15_e136389;
            locals.var_chib_dn0 = assign89550_body15_e136389_d_n0;
            locals.var_chib_dn2 = assign89550_body15_e136389_d_n2;
            locals.var_chib_dn4 = assign89550_body15_e136389_d_n4;
            locals.var_chib_dn5 = assign89550_body15_e136389_d_n5;
            locals.var_chib_dn6 = assign89550_body15_e136389_d_n6;
            locals.var_chib_dn7 = assign89550_body15_e136389_d_n7;
            locals.var_chib_dn8 = assign89550_body15_e136389_d_n8;
            locals.var_chib_dn9 = assign89550_body15_e136389_d_n9;
            locals.var_chib_dn10 = assign89550_body15_e136389_d_n10;
            locals.var_chib_dn11 = assign89550_body15_e136389_d_n11;
            locals.var_chib_dn14 = assign89550_body15_e136389_d_n14;
            locals.var_chib_rv = 0.0;
            let assign89550_body16_e136392: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2075 = assign89550_body16_e136392;
            locals.var_guard2075_rv = 0.0;
            let (assign89550_body18_e136417, assign89550_body18_e136417_d_n0, assign89550_body18_e136417_d_n2, assign89550_body18_e136417_d_n4, assign89550_body18_e136417_d_n5, assign89550_body18_e136417_d_n6, assign89550_body18_e136417_d_n7, assign89550_body18_e136417_d_n8, assign89550_body18_e136417_d_n9, assign89550_body18_e136417_d_n10, assign89550_body18_e136417_d_n11, assign89550_body18_e136417_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 != 0.0)) {
        let assign89550_body18_e136415: f64 = (-0.7071067811865475);
        (assign89550_body18_e136415, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89550_body18_e136417;
            locals.var_t0_dn0 = assign89550_body18_e136417_d_n0;
            locals.var_t0_dn2 = assign89550_body18_e136417_d_n2;
            locals.var_t0_dn4 = assign89550_body18_e136417_d_n4;
            locals.var_t0_dn5 = assign89550_body18_e136417_d_n5;
            locals.var_t0_dn6 = assign89550_body18_e136417_d_n6;
            locals.var_t0_dn7 = assign89550_body18_e136417_d_n7;
            locals.var_t0_dn8 = assign89550_body18_e136417_d_n8;
            locals.var_t0_dn9 = assign89550_body18_e136417_d_n9;
            locals.var_t0_dn10 = assign89550_body18_e136417_d_n10;
            locals.var_t0_dn11 = assign89550_body18_e136417_d_n11;
            locals.var_t0_dn14 = assign89550_body18_e136417_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89550_body19_e136430, assign89550_body19_e136430_d_n0, assign89550_body19_e136430_d_n2, assign89550_body19_e136430_d_n4, assign89550_body19_e136430_d_n5, assign89550_body19_e136430_d_n6, assign89550_body19_e136430_d_n7, assign89550_body19_e136430_d_n8, assign89550_body19_e136430_d_n9, assign89550_body19_e136430_d_n10, assign89550_body19_e136430_d_n11, assign89550_body19_e136430_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 != 0.0)) {
        let assign89550_body19_e136428: f64 = (locals.var_chi * locals.var_t0);
        (assign89550_body19_e136428, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn14 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign89550_body19_e136430;
            locals.var_fb_dn0 = assign89550_body19_e136430_d_n0;
            locals.var_fb_dn2 = assign89550_body19_e136430_d_n2;
            locals.var_fb_dn4 = assign89550_body19_e136430_d_n4;
            locals.var_fb_dn5 = assign89550_body19_e136430_d_n5;
            locals.var_fb_dn6 = assign89550_body19_e136430_d_n6;
            locals.var_fb_dn7 = assign89550_body19_e136430_d_n7;
            locals.var_fb_dn8 = assign89550_body19_e136430_d_n8;
            locals.var_fb_dn9 = assign89550_body19_e136430_d_n9;
            locals.var_fb_dn10 = assign89550_body19_e136430_d_n10;
            locals.var_fb_dn11 = assign89550_body19_e136430_d_n11;
            locals.var_fb_dn14 = assign89550_body19_e136430_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign89550_body20_e136443, assign89550_body20_e136443_d_n0, assign89550_body20_e136443_d_n2, assign89550_body20_e136443_d_n4, assign89550_body20_e136443_d_n5, assign89550_body20_e136443_d_n6, assign89550_body20_e136443_d_n7, assign89550_body20_e136443_d_n8, assign89550_body20_e136443_d_n9, assign89550_body20_e136443_d_n10, assign89550_body20_e136443_d_n11, assign89550_body20_e136443_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 != 0.0)) {
        let assign89550_body20_e136441: f64 = (locals.var_beta * locals.var_t0);
        (assign89550_body20_e136441, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn11 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn11)), ((locals.var_beta_dn14 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign89550_body20_e136443;
            locals.var_fb_dpss_dn0 = assign89550_body20_e136443_d_n0;
            locals.var_fb_dpss_dn2 = assign89550_body20_e136443_d_n2;
            locals.var_fb_dpss_dn4 = assign89550_body20_e136443_d_n4;
            locals.var_fb_dpss_dn5 = assign89550_body20_e136443_d_n5;
            locals.var_fb_dpss_dn6 = assign89550_body20_e136443_d_n6;
            locals.var_fb_dpss_dn7 = assign89550_body20_e136443_d_n7;
            locals.var_fb_dpss_dn8 = assign89550_body20_e136443_d_n8;
            locals.var_fb_dpss_dn9 = assign89550_body20_e136443_d_n9;
            locals.var_fb_dpss_dn10 = assign89550_body20_e136443_d_n10;
            locals.var_fb_dpss_dn11 = assign89550_body20_e136443_d_n11;
            locals.var_fb_dpss_dn14 = assign89550_body20_e136443_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let assign89550_body21_e136446: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2076 = assign89550_body21_e136446;
            locals.var_guard2076_rv = 0.0;
            let (assign89550_body23_e136498, assign89550_body23_e136498_d_n0, assign89550_body23_e136498_d_n2, assign89550_body23_e136498_d_n4, assign89550_body23_e136498_d_n5, assign89550_body23_e136498_d_n6, assign89550_body23_e136498_d_n7, assign89550_body23_e136498_d_n8, assign89550_body23_e136498_d_n9, assign89550_body23_e136498_d_n10, assign89550_body23_e136498_d_n11, assign89550_body23_e136498_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89550_body23_e136476: f64 = (locals.var_chi * locals.var_chi);
        let assign89550_body23_e136478: f64 = (assign89550_body23_e136476 / 2.0);
        let assign89550_body23_e136482: f64 = (locals.var_chi / 3.0);
        let assign89550_body23_e136486: f64 = (locals.var_chi / 4.0);
        let assign89550_body23_e136490: f64 = (locals.var_chi / 5.0);
        let assign89550_body23_e136491: f64 = (1.0 - assign89550_body23_e136490);
        let assign89550_body23_e136492: f64 = (assign89550_body23_e136486 * assign89550_body23_e136491);
        let assign89550_body23_e136493: f64 = (1.0 - assign89550_body23_e136492);
        let assign89550_body23_e136494: f64 = (assign89550_body23_e136482 * assign89550_body23_e136493);
        let assign89550_body23_e136495: f64 = (1.0 - assign89550_body23_e136494);
        let assign89550_body23_e136496: f64 = (assign89550_body23_e136478 * assign89550_body23_e136495);
        (assign89550_body23_e136496, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn0 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn0 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn2 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn2 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn4 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn4 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn5 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn5 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn6 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn6 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn7 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn7 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn8 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn8 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn9 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn9 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn10 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn10 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn11 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn11 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign89550_body23_e136495) + (assign89550_body23_e136478 * (-(((locals.var_chi_dn14 / 3.0) * assign89550_body23_e136493) + (assign89550_body23_e136482 * (-(((locals.var_chi_dn14 / 4.0) * assign89550_body23_e136491) + (assign89550_body23_e136486 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89550_body23_e136498;
            locals.var_t0_dn0 = assign89550_body23_e136498_d_n0;
            locals.var_t0_dn2 = assign89550_body23_e136498_d_n2;
            locals.var_t0_dn4 = assign89550_body23_e136498_d_n4;
            locals.var_t0_dn5 = assign89550_body23_e136498_d_n5;
            locals.var_t0_dn6 = assign89550_body23_e136498_d_n6;
            locals.var_t0_dn7 = assign89550_body23_e136498_d_n7;
            locals.var_t0_dn8 = assign89550_body23_e136498_d_n8;
            locals.var_t0_dn9 = assign89550_body23_e136498_d_n9;
            locals.var_t0_dn10 = assign89550_body23_e136498_d_n10;
            locals.var_t0_dn11 = assign89550_body23_e136498_d_n11;
            locals.var_t0_dn14 = assign89550_body23_e136498_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89550_body24_e136530, assign89550_body24_e136530_d_n0, assign89550_body24_e136530_d_n2, assign89550_body24_e136530_d_n4, assign89550_body24_e136530_d_n5, assign89550_body24_e136530_d_n6, assign89550_body24_e136530_d_n7, assign89550_body24_e136530_d_n8, assign89550_body24_e136530_d_n9, assign89550_body24_e136530_d_n10, assign89550_body24_e136530_d_n11, assign89550_body24_e136530_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89550_body24_e136514: f64 = (locals.var_chi / 2.0);
        let assign89550_body24_e136518: f64 = (locals.var_chi / 3.0);
        let assign89550_body24_e136522: f64 = (locals.var_chi / 4.0);
        let assign89550_body24_e136523: f64 = (1.0 - assign89550_body24_e136522);
        let assign89550_body24_e136524: f64 = (assign89550_body24_e136518 * assign89550_body24_e136523);
        let assign89550_body24_e136525: f64 = (1.0 - assign89550_body24_e136524);
        let assign89550_body24_e136526: f64 = (assign89550_body24_e136514 * assign89550_body24_e136525);
        let assign89550_body24_e136527: f64 = (1.0 - assign89550_body24_e136526);
        let assign89550_body24_e136528: f64 = (locals.var_chi * assign89550_body24_e136527);
        (assign89550_body24_e136528, ((locals.var_chi_dn0 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn0 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn2 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn4 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn5 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn6 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn7 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn8 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn9 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn10 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn11 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign89550_body24_e136527) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign89550_body24_e136525) + (assign89550_body24_e136514 * (-(((locals.var_chi_dn14 / 3.0) * assign89550_body24_e136523) + (assign89550_body24_e136518 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89550_body24_e136530;
            locals.var_t1_dn0 = assign89550_body24_e136530_d_n0;
            locals.var_t1_dn2 = assign89550_body24_e136530_d_n2;
            locals.var_t1_dn4 = assign89550_body24_e136530_d_n4;
            locals.var_t1_dn5 = assign89550_body24_e136530_d_n5;
            locals.var_t1_dn6 = assign89550_body24_e136530_d_n6;
            locals.var_t1_dn7 = assign89550_body24_e136530_d_n7;
            locals.var_t1_dn8 = assign89550_body24_e136530_d_n8;
            locals.var_t1_dn9 = assign89550_body24_e136530_d_n9;
            locals.var_t1_dn10 = assign89550_body24_e136530_d_n10;
            locals.var_t1_dn11 = assign89550_body24_e136530_d_n11;
            locals.var_t1_dn14 = assign89550_body24_e136530_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89550_body25_e136566, assign89550_body25_e136566_d_n0, assign89550_body25_e136566_d_n2, assign89550_body25_e136566_d_n4, assign89550_body25_e136566_d_n5, assign89550_body25_e136566_d_n6, assign89550_body25_e136566_d_n7, assign89550_body25_e136566_d_n8, assign89550_body25_e136566_d_n9, assign89550_body25_e136566_d_n10, assign89550_body25_e136566_d_n11, assign89550_body25_e136566_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89550_body25_e136544: f64 = (locals.var_chib * locals.var_chib);
        let assign89550_body25_e136546: f64 = (assign89550_body25_e136544 / 2.0);
        let assign89550_body25_e136550: f64 = (locals.var_chib / 3.0);
        let assign89550_body25_e136554: f64 = (locals.var_chib / 4.0);
        let assign89550_body25_e136558: f64 = (locals.var_chib / 5.0);
        let assign89550_body25_e136559: f64 = (1.0 - assign89550_body25_e136558);
        let assign89550_body25_e136560: f64 = (assign89550_body25_e136554 * assign89550_body25_e136559);
        let assign89550_body25_e136561: f64 = (1.0 - assign89550_body25_e136560);
        let assign89550_body25_e136562: f64 = (assign89550_body25_e136550 * assign89550_body25_e136561);
        let assign89550_body25_e136563: f64 = (1.0 - assign89550_body25_e136562);
        let assign89550_body25_e136564: f64 = (assign89550_body25_e136546 * assign89550_body25_e136563);
        (assign89550_body25_e136564, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn0 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn0 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn2 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn2 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn4 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn4 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn5 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn5 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn6 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn6 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn7 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn7 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn8 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn8 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn9 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn9 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn10 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn10 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn11 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn11 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign89550_body25_e136563) + (assign89550_body25_e136546 * (-(((locals.var_chib_dn14 / 3.0) * assign89550_body25_e136561) + (assign89550_body25_e136550 * (-(((locals.var_chib_dn14 / 4.0) * assign89550_body25_e136559) + (assign89550_body25_e136554 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign89550_body25_e136566;
            locals.var_t2_dn0 = assign89550_body25_e136566_d_n0;
            locals.var_t2_dn2 = assign89550_body25_e136566_d_n2;
            locals.var_t2_dn4 = assign89550_body25_e136566_d_n4;
            locals.var_t2_dn5 = assign89550_body25_e136566_d_n5;
            locals.var_t2_dn6 = assign89550_body25_e136566_d_n6;
            locals.var_t2_dn7 = assign89550_body25_e136566_d_n7;
            locals.var_t2_dn8 = assign89550_body25_e136566_d_n8;
            locals.var_t2_dn9 = assign89550_body25_e136566_d_n9;
            locals.var_t2_dn10 = assign89550_body25_e136566_d_n10;
            locals.var_t2_dn11 = assign89550_body25_e136566_d_n11;
            locals.var_t2_dn14 = assign89550_body25_e136566_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign89550_body26_e136598, assign89550_body26_e136598_d_n0, assign89550_body26_e136598_d_n2, assign89550_body26_e136598_d_n4, assign89550_body26_e136598_d_n5, assign89550_body26_e136598_d_n6, assign89550_body26_e136598_d_n7, assign89550_body26_e136598_d_n8, assign89550_body26_e136598_d_n9, assign89550_body26_e136598_d_n10, assign89550_body26_e136598_d_n11, assign89550_body26_e136598_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89550_body26_e136582: f64 = (locals.var_chib / 2.0);
        let assign89550_body26_e136586: f64 = (locals.var_chib / 3.0);
        let assign89550_body26_e136590: f64 = (locals.var_chib / 4.0);
        let assign89550_body26_e136591: f64 = (1.0 - assign89550_body26_e136590);
        let assign89550_body26_e136592: f64 = (assign89550_body26_e136586 * assign89550_body26_e136591);
        let assign89550_body26_e136593: f64 = (1.0 - assign89550_body26_e136592);
        let assign89550_body26_e136594: f64 = (assign89550_body26_e136582 * assign89550_body26_e136593);
        let assign89550_body26_e136595: f64 = (1.0 - assign89550_body26_e136594);
        let assign89550_body26_e136596: f64 = (locals.var_chib * assign89550_body26_e136595);
        (assign89550_body26_e136596, ((locals.var_chib_dn0 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn0 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn2 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn4 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn5 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn6 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn7 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn8 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn9 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn10 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn11 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign89550_body26_e136595) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign89550_body26_e136593) + (assign89550_body26_e136582 * (-(((locals.var_chib_dn14 / 3.0) * assign89550_body26_e136591) + (assign89550_body26_e136586 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign89550_body26_e136598;
            locals.var_t3_dn0 = assign89550_body26_e136598_d_n0;
            locals.var_t3_dn2 = assign89550_body26_e136598_d_n2;
            locals.var_t3_dn4 = assign89550_body26_e136598_d_n4;
            locals.var_t3_dn5 = assign89550_body26_e136598_d_n5;
            locals.var_t3_dn6 = assign89550_body26_e136598_d_n6;
            locals.var_t3_dn7 = assign89550_body26_e136598_d_n7;
            locals.var_t3_dn8 = assign89550_body26_e136598_d_n8;
            locals.var_t3_dn9 = assign89550_body26_e136598_d_n9;
            locals.var_t3_dn10 = assign89550_body26_e136598_d_n10;
            locals.var_t3_dn11 = assign89550_body26_e136598_d_n11;
            locals.var_t3_dn14 = assign89550_body26_e136598_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign89550_body27_e136614, assign89550_body27_e136614_d_n0, assign89550_body27_e136614_d_n2, assign89550_body27_e136614_d_n4, assign89550_body27_e136614_d_n5, assign89550_body27_e136614_d_n6, assign89550_body27_e136614_d_n7, assign89550_body27_e136614_d_n8, assign89550_body27_e136614_d_n9, assign89550_body27_e136614_d_n10, assign89550_body27_e136614_d_n11, assign89550_body27_e136614_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89550_body27_e136612: f64 = (locals.var_t0 - locals.var_t2);
        (assign89550_body27_e136612, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign89550_body27_e136614;
            locals.var_t4_dn0 = assign89550_body27_e136614_d_n0;
            locals.var_t4_dn2 = assign89550_body27_e136614_d_n2;
            locals.var_t4_dn4 = assign89550_body27_e136614_d_n4;
            locals.var_t4_dn5 = assign89550_body27_e136614_d_n5;
            locals.var_t4_dn6 = assign89550_body27_e136614_d_n6;
            locals.var_t4_dn7 = assign89550_body27_e136614_d_n7;
            locals.var_t4_dn8 = assign89550_body27_e136614_d_n8;
            locals.var_t4_dn9 = assign89550_body27_e136614_d_n9;
            locals.var_t4_dn10 = assign89550_body27_e136614_d_n10;
            locals.var_t4_dn11 = assign89550_body27_e136614_d_n11;
            locals.var_t4_dn14 = assign89550_body27_e136614_d_n14;
            locals.var_t4_rv = 0.0;
            let assign89550_body28_e136617: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2077 = assign89550_body28_e136617;
            locals.var_guard2077_rv = 0.0;
            let (assign89550_body29_e136634, assign89550_body29_e136634_d_n0, assign89550_body29_e136634_d_n2, assign89550_body29_e136634_d_n4, assign89550_body29_e136634_d_n5, assign89550_body29_e136634_d_n6, assign89550_body29_e136634_d_n7, assign89550_body29_e136634_d_n8, assign89550_body29_e136634_d_n9, assign89550_body29_e136634_d_n10, assign89550_body29_e136634_d_n11, assign89550_body29_e136634_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        let assign89550_body29_e136632: f64 = (locals.var_t4).sqrt();
        (assign89550_body29_e136632, (locals.var_t4_dn0 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn2 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn4 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn5 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn6 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn7 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn8 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn9 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn10 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn11 / (2.0 * assign89550_body29_e136632)), (locals.var_t4_dn14 / (2.0 * assign89550_body29_e136632)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign89550_body29_e136634;
            locals.var_fb_dn0 = assign89550_body29_e136634_d_n0;
            locals.var_fb_dn2 = assign89550_body29_e136634_d_n2;
            locals.var_fb_dn4 = assign89550_body29_e136634_d_n4;
            locals.var_fb_dn5 = assign89550_body29_e136634_d_n5;
            locals.var_fb_dn6 = assign89550_body29_e136634_d_n6;
            locals.var_fb_dn7 = assign89550_body29_e136634_d_n7;
            locals.var_fb_dn8 = assign89550_body29_e136634_d_n8;
            locals.var_fb_dn9 = assign89550_body29_e136634_d_n9;
            locals.var_fb_dn10 = assign89550_body29_e136634_d_n10;
            locals.var_fb_dn11 = assign89550_body29_e136634_d_n11;
            locals.var_fb_dn14 = assign89550_body29_e136634_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign89550_body30_e136660, assign89550_body30_e136660_d_n0, assign89550_body30_e136660_d_n2, assign89550_body30_e136660_d_n4, assign89550_body30_e136660_d_n5, assign89550_body30_e136660_d_n6, assign89550_body30_e136660_d_n7, assign89550_body30_e136660_d_n8, assign89550_body30_e136660_d_n9, assign89550_body30_e136660_d_n10, assign89550_body30_e136660_d_n11, assign89550_body30_e136660_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        let assign89550_body30_e136650: f64 = (locals.var_beta * 0.5);
        let assign89550_body30_e136654: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign89550_body30_e136655: f64 = (locals.var_t1 - assign89550_body30_e136654);
        let assign89550_body30_e136656: f64 = (assign89550_body30_e136650 * assign89550_body30_e136655);
        let assign89550_body30_e136658: f64 = (assign89550_body30_e136656 / locals.var_fb);
        (assign89550_body30_e136658, ((((((locals.var_beta_dn0 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign89550_body30_e136655) + (assign89550_body30_e136650 * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))) * locals.var_fb) - (assign89550_body30_e136656 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign89550_body30_e136660;
            locals.var_fb_dpss_dn0 = assign89550_body30_e136660_d_n0;
            locals.var_fb_dpss_dn2 = assign89550_body30_e136660_d_n2;
            locals.var_fb_dpss_dn4 = assign89550_body30_e136660_d_n4;
            locals.var_fb_dpss_dn5 = assign89550_body30_e136660_d_n5;
            locals.var_fb_dpss_dn6 = assign89550_body30_e136660_d_n6;
            locals.var_fb_dpss_dn7 = assign89550_body30_e136660_d_n7;
            locals.var_fb_dpss_dn8 = assign89550_body30_e136660_d_n8;
            locals.var_fb_dpss_dn9 = assign89550_body30_e136660_d_n9;
            locals.var_fb_dpss_dn10 = assign89550_body30_e136660_d_n10;
            locals.var_fb_dpss_dn11 = assign89550_body30_e136660_d_n11;
            locals.var_fb_dpss_dn14 = assign89550_body30_e136660_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign89550_body32_e136696, assign89550_body32_e136696_d_n0, assign89550_body32_e136696_d_n2, assign89550_body32_e136696_d_n4, assign89550_body32_e136696_d_n5, assign89550_body32_e136696_d_n6, assign89550_body32_e136696_d_n7, assign89550_body32_e136696_d_n8, assign89550_body32_e136696_d_n9, assign89550_body32_e136696_d_n10, assign89550_body32_e136696_d_n11, assign89550_body32_e136696_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) && (locals.var_guard2077 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign89550_body32_e136696;
            locals.var_fb_dn0 = assign89550_body32_e136696_d_n0;
            locals.var_fb_dn2 = assign89550_body32_e136696_d_n2;
            locals.var_fb_dn4 = assign89550_body32_e136696_d_n4;
            locals.var_fb_dn5 = assign89550_body32_e136696_d_n5;
            locals.var_fb_dn6 = assign89550_body32_e136696_d_n6;
            locals.var_fb_dn7 = assign89550_body32_e136696_d_n7;
            locals.var_fb_dn8 = assign89550_body32_e136696_d_n8;
            locals.var_fb_dn9 = assign89550_body32_e136696_d_n9;
            locals.var_fb_dn10 = assign89550_body32_e136696_d_n10;
            locals.var_fb_dn11 = assign89550_body32_e136696_d_n11;
            locals.var_fb_dn14 = assign89550_body32_e136696_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign89550_body33_e136713, assign89550_body33_e136713_d_n0, assign89550_body33_e136713_d_n2, assign89550_body33_e136713_d_n4, assign89550_body33_e136713_d_n5, assign89550_body33_e136713_d_n6, assign89550_body33_e136713_d_n7, assign89550_body33_e136713_d_n8, assign89550_body33_e136713_d_n9, assign89550_body33_e136713_d_n10, assign89550_body33_e136713_d_n11, assign89550_body33_e136713_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 != 0.0)) && (locals.var_guard2077 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign89550_body33_e136713;
            locals.var_fb_dpss_dn0 = assign89550_body33_e136713_d_n0;
            locals.var_fb_dpss_dn2 = assign89550_body33_e136713_d_n2;
            locals.var_fb_dpss_dn4 = assign89550_body33_e136713_d_n4;
            locals.var_fb_dpss_dn5 = assign89550_body33_e136713_d_n5;
            locals.var_fb_dpss_dn6 = assign89550_body33_e136713_d_n6;
            locals.var_fb_dpss_dn7 = assign89550_body33_e136713_d_n7;
            locals.var_fb_dpss_dn8 = assign89550_body33_e136713_d_n8;
            locals.var_fb_dpss_dn9 = assign89550_body33_e136713_d_n9;
            locals.var_fb_dpss_dn10 = assign89550_body33_e136713_d_n10;
            locals.var_fb_dpss_dn11 = assign89550_body33_e136713_d_n11;
            locals.var_fb_dpss_dn14 = assign89550_body33_e136713_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign89550_body34_e136730, assign89550_body34_e136730_d_n0, assign89550_body34_e136730_d_n2, assign89550_body34_e136730_d_n4, assign89550_body34_e136730_d_n5, assign89550_body34_e136730_d_n6, assign89550_body34_e136730_d_n7, assign89550_body34_e136730_d_n8, assign89550_body34_e136730_d_n9, assign89550_body34_e136730_d_n10, assign89550_body34_e136730_d_n11, assign89550_body34_e136730_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 == 0.0)) {
        let assign89550_body34_e136727: f64 = (-locals.var_chi);
        let assign89550_body34_e136728: f64 = (assign89550_body34_e136727).exp();
        (assign89550_body34_e136728, (assign89550_body34_e136728 * (-locals.var_chi_dn0)), (assign89550_body34_e136728 * (-locals.var_chi_dn2)), (assign89550_body34_e136728 * (-locals.var_chi_dn4)), (assign89550_body34_e136728 * (-locals.var_chi_dn5)), (assign89550_body34_e136728 * (-locals.var_chi_dn6)), (assign89550_body34_e136728 * (-locals.var_chi_dn7)), (assign89550_body34_e136728 * (-locals.var_chi_dn8)), (assign89550_body34_e136728 * (-locals.var_chi_dn9)), (assign89550_body34_e136728 * (-locals.var_chi_dn10)), (assign89550_body34_e136728 * (-locals.var_chi_dn11)), (assign89550_body34_e136728 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89550_body34_e136730;
            locals.var_t0_dn0 = assign89550_body34_e136730_d_n0;
            locals.var_t0_dn2 = assign89550_body34_e136730_d_n2;
            locals.var_t0_dn4 = assign89550_body34_e136730_d_n4;
            locals.var_t0_dn5 = assign89550_body34_e136730_d_n5;
            locals.var_t0_dn6 = assign89550_body34_e136730_d_n6;
            locals.var_t0_dn7 = assign89550_body34_e136730_d_n7;
            locals.var_t0_dn8 = assign89550_body34_e136730_d_n8;
            locals.var_t0_dn9 = assign89550_body34_e136730_d_n9;
            locals.var_t0_dn10 = assign89550_body34_e136730_d_n10;
            locals.var_t0_dn11 = assign89550_body34_e136730_d_n11;
            locals.var_t0_dn14 = assign89550_body34_e136730_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89550_body35_e136747, assign89550_body35_e136747_d_n0, assign89550_body35_e136747_d_n2, assign89550_body35_e136747_d_n4, assign89550_body35_e136747_d_n5, assign89550_body35_e136747_d_n6, assign89550_body35_e136747_d_n7, assign89550_body35_e136747_d_n8, assign89550_body35_e136747_d_n9, assign89550_body35_e136747_d_n10, assign89550_body35_e136747_d_n11, assign89550_body35_e136747_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 == 0.0)) {
        let assign89550_body35_e136744: f64 = (-locals.var_chib);
        let assign89550_body35_e136745: f64 = (assign89550_body35_e136744).exp();
        (assign89550_body35_e136745, (assign89550_body35_e136745 * (-locals.var_chib_dn0)), (assign89550_body35_e136745 * (-locals.var_chib_dn2)), (assign89550_body35_e136745 * (-locals.var_chib_dn4)), (assign89550_body35_e136745 * (-locals.var_chib_dn5)), (assign89550_body35_e136745 * (-locals.var_chib_dn6)), (assign89550_body35_e136745 * (-locals.var_chib_dn7)), (assign89550_body35_e136745 * (-locals.var_chib_dn8)), (assign89550_body35_e136745 * (-locals.var_chib_dn9)), (assign89550_body35_e136745 * (-locals.var_chib_dn10)), (assign89550_body35_e136745 * (-locals.var_chib_dn11)), (assign89550_body35_e136745 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89550_body35_e136747;
            locals.var_t1_dn0 = assign89550_body35_e136747_d_n0;
            locals.var_t1_dn2 = assign89550_body35_e136747_d_n2;
            locals.var_t1_dn4 = assign89550_body35_e136747_d_n4;
            locals.var_t1_dn5 = assign89550_body35_e136747_d_n5;
            locals.var_t1_dn6 = assign89550_body35_e136747_d_n6;
            locals.var_t1_dn7 = assign89550_body35_e136747_d_n7;
            locals.var_t1_dn8 = assign89550_body35_e136747_d_n8;
            locals.var_t1_dn9 = assign89550_body35_e136747_d_n9;
            locals.var_t1_dn10 = assign89550_body35_e136747_d_n10;
            locals.var_t1_dn11 = assign89550_body35_e136747_d_n11;
            locals.var_t1_dn14 = assign89550_body35_e136747_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89550_body36_e136768, assign89550_body36_e136768_d_n0, assign89550_body36_e136768_d_n2, assign89550_body36_e136768_d_n4, assign89550_body36_e136768_d_n5, assign89550_body36_e136768_d_n6, assign89550_body36_e136768_d_n7, assign89550_body36_e136768_d_n8, assign89550_body36_e136768_d_n9, assign89550_body36_e136768_d_n10, assign89550_body36_e136768_d_n11, assign89550_body36_e136768_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 == 0.0)) {
        let assign89550_body36_e136762: f64 = (locals.var_chi - locals.var_chib);
        let assign89550_body36_e136765: f64 = (locals.var_t0 - locals.var_t1);
        let assign89550_body36_e136766: f64 = (assign89550_body36_e136762 + assign89550_body36_e136765);
        (assign89550_body36_e136766, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign89550_body36_e136768;
            locals.var_t4_dn0 = assign89550_body36_e136768_d_n0;
            locals.var_t4_dn2 = assign89550_body36_e136768_d_n2;
            locals.var_t4_dn4 = assign89550_body36_e136768_d_n4;
            locals.var_t4_dn5 = assign89550_body36_e136768_d_n5;
            locals.var_t4_dn6 = assign89550_body36_e136768_d_n6;
            locals.var_t4_dn7 = assign89550_body36_e136768_d_n7;
            locals.var_t4_dn8 = assign89550_body36_e136768_d_n8;
            locals.var_t4_dn9 = assign89550_body36_e136768_d_n9;
            locals.var_t4_dn10 = assign89550_body36_e136768_d_n10;
            locals.var_t4_dn11 = assign89550_body36_e136768_d_n11;
            locals.var_t4_dn14 = assign89550_body36_e136768_d_n14;
            locals.var_t4_rv = 0.0;
            let assign89550_body37_e136771: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2078 = assign89550_body37_e136771;
            locals.var_guard2078_rv = 0.0;
            let (assign89550_body38_e136789, assign89550_body38_e136789_d_n0, assign89550_body38_e136789_d_n2, assign89550_body38_e136789_d_n4, assign89550_body38_e136789_d_n5, assign89550_body38_e136789_d_n6, assign89550_body38_e136789_d_n7, assign89550_body38_e136789_d_n8, assign89550_body38_e136789_d_n9, assign89550_body38_e136789_d_n10, assign89550_body38_e136789_d_n11, assign89550_body38_e136789_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 == 0.0)) && (locals.var_guard2078 != 0.0)) {
        let assign89550_body38_e136787: f64 = (locals.var_t4).sqrt();
        (assign89550_body38_e136787, (locals.var_t4_dn0 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn2 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn4 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn5 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn6 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn7 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn8 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn9 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn10 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn11 / (2.0 * assign89550_body38_e136787)), (locals.var_t4_dn14 / (2.0 * assign89550_body38_e136787)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign89550_body38_e136789;
            locals.var_fb_dn0 = assign89550_body38_e136789_d_n0;
            locals.var_fb_dn2 = assign89550_body38_e136789_d_n2;
            locals.var_fb_dn4 = assign89550_body38_e136789_d_n4;
            locals.var_fb_dn5 = assign89550_body38_e136789_d_n5;
            locals.var_fb_dn6 = assign89550_body38_e136789_d_n6;
            locals.var_fb_dn7 = assign89550_body38_e136789_d_n7;
            locals.var_fb_dn8 = assign89550_body38_e136789_d_n8;
            locals.var_fb_dn9 = assign89550_body38_e136789_d_n9;
            locals.var_fb_dn10 = assign89550_body38_e136789_d_n10;
            locals.var_fb_dn11 = assign89550_body38_e136789_d_n11;
            locals.var_fb_dn14 = assign89550_body38_e136789_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign89550_body39_e136820, assign89550_body39_e136820_d_n0, assign89550_body39_e136820_d_n2, assign89550_body39_e136820_d_n4, assign89550_body39_e136820_d_n5, assign89550_body39_e136820_d_n6, assign89550_body39_e136820_d_n7, assign89550_body39_e136820_d_n8, assign89550_body39_e136820_d_n9, assign89550_body39_e136820_d_n10, assign89550_body39_e136820_d_n11, assign89550_body39_e136820_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 == 0.0)) && (locals.var_guard2078 != 0.0)) {
        let assign89550_body39_e136806: f64 = (locals.var_beta * 0.5);
        let assign89550_body39_e136809: f64 = (1.0 - locals.var_t0);
        let assign89550_body39_e136813: f64 = (1.0 - locals.var_t1);
        let assign89550_body39_e136814: f64 = (locals.var_phi_b_dpss * assign89550_body39_e136813);
        let assign89550_body39_e136815: f64 = (assign89550_body39_e136809 - assign89550_body39_e136814);
        let assign89550_body39_e136816: f64 = (assign89550_body39_e136806 * assign89550_body39_e136815);
        let assign89550_body39_e136818: f64 = (assign89550_body39_e136816 / locals.var_fb);
        (assign89550_body39_e136818, ((((((locals.var_beta_dn0 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign89550_body39_e136815) + (assign89550_body39_e136806 * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign89550_body39_e136813) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))) * locals.var_fb) - (assign89550_body39_e136816 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign89550_body39_e136820;
            locals.var_fb_dpss_dn0 = assign89550_body39_e136820_d_n0;
            locals.var_fb_dpss_dn2 = assign89550_body39_e136820_d_n2;
            locals.var_fb_dpss_dn4 = assign89550_body39_e136820_d_n4;
            locals.var_fb_dpss_dn5 = assign89550_body39_e136820_d_n5;
            locals.var_fb_dpss_dn6 = assign89550_body39_e136820_d_n6;
            locals.var_fb_dpss_dn7 = assign89550_body39_e136820_d_n7;
            locals.var_fb_dpss_dn8 = assign89550_body39_e136820_d_n8;
            locals.var_fb_dpss_dn9 = assign89550_body39_e136820_d_n9;
            locals.var_fb_dpss_dn10 = assign89550_body39_e136820_d_n10;
            locals.var_fb_dpss_dn11 = assign89550_body39_e136820_d_n11;
            locals.var_fb_dpss_dn14 = assign89550_body39_e136820_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign89550_body41_e136858, assign89550_body41_e136858_d_n0, assign89550_body41_e136858_d_n2, assign89550_body41_e136858_d_n4, assign89550_body41_e136858_d_n5, assign89550_body41_e136858_d_n6, assign89550_body41_e136858_d_n7, assign89550_body41_e136858_d_n8, assign89550_body41_e136858_d_n9, assign89550_body41_e136858_d_n10, assign89550_body41_e136858_d_n11, assign89550_body41_e136858_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 == 0.0)) && (locals.var_guard2078 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign89550_body41_e136858;
            locals.var_fb_dn0 = assign89550_body41_e136858_d_n0;
            locals.var_fb_dn2 = assign89550_body41_e136858_d_n2;
            locals.var_fb_dn4 = assign89550_body41_e136858_d_n4;
            locals.var_fb_dn5 = assign89550_body41_e136858_d_n5;
            locals.var_fb_dn6 = assign89550_body41_e136858_d_n6;
            locals.var_fb_dn7 = assign89550_body41_e136858_d_n7;
            locals.var_fb_dn8 = assign89550_body41_e136858_d_n8;
            locals.var_fb_dn9 = assign89550_body41_e136858_d_n9;
            locals.var_fb_dn10 = assign89550_body41_e136858_d_n10;
            locals.var_fb_dn11 = assign89550_body41_e136858_d_n11;
            locals.var_fb_dn14 = assign89550_body41_e136858_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign89550_body42_e136876, assign89550_body42_e136876_d_n0, assign89550_body42_e136876_d_n2, assign89550_body42_e136876_d_n4, assign89550_body42_e136876_d_n5, assign89550_body42_e136876_d_n6, assign89550_body42_e136876_d_n7, assign89550_body42_e136876_d_n8, assign89550_body42_e136876_d_n9, assign89550_body42_e136876_d_n10, assign89550_body42_e136876_d_n11, assign89550_body42_e136876_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2075 == 0.0)) && (locals.var_guard2076 == 0.0)) && (locals.var_guard2078 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign89550_body42_e136876;
            locals.var_fb_dpss_dn0 = assign89550_body42_e136876_d_n0;
            locals.var_fb_dpss_dn2 = assign89550_body42_e136876_d_n2;
            locals.var_fb_dpss_dn4 = assign89550_body42_e136876_d_n4;
            locals.var_fb_dpss_dn5 = assign89550_body42_e136876_d_n5;
            locals.var_fb_dpss_dn6 = assign89550_body42_e136876_d_n6;
            locals.var_fb_dpss_dn7 = assign89550_body42_e136876_d_n7;
            locals.var_fb_dpss_dn8 = assign89550_body42_e136876_d_n8;
            locals.var_fb_dpss_dn9 = assign89550_body42_e136876_d_n9;
            locals.var_fb_dpss_dn10 = assign89550_body42_e136876_d_n10;
            locals.var_fb_dpss_dn11 = assign89550_body42_e136876_d_n11;
            locals.var_fb_dpss_dn14 = assign89550_body42_e136876_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let assign89550_body43_e136879: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2079 = assign89550_body43_e136879;
            locals.var_guard2079_rv = 0.0;
            let (assign89550_body45_e136903, assign89550_body45_e136903_d_n0, assign89550_body45_e136903_d_n2, assign89550_body45_e136903_d_n4, assign89550_body45_e136903_d_n5, assign89550_body45_e136903_d_n6, assign89550_body45_e136903_d_n7, assign89550_body45_e136903_d_n8, assign89550_body45_e136903_d_n9, assign89550_body45_e136903_d_n10, assign89550_body45_e136903_d_n11, assign89550_body45_e136903_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89550_body45_e136903;
            locals.var_fs01_dn0 = assign89550_body45_e136903_d_n0;
            locals.var_fs01_dn2 = assign89550_body45_e136903_d_n2;
            locals.var_fs01_dn4 = assign89550_body45_e136903_d_n4;
            locals.var_fs01_dn5 = assign89550_body45_e136903_d_n5;
            locals.var_fs01_dn6 = assign89550_body45_e136903_d_n6;
            locals.var_fs01_dn7 = assign89550_body45_e136903_d_n7;
            locals.var_fs01_dn8 = assign89550_body45_e136903_d_n8;
            locals.var_fs01_dn9 = assign89550_body45_e136903_d_n9;
            locals.var_fs01_dn10 = assign89550_body45_e136903_d_n10;
            locals.var_fs01_dn11 = assign89550_body45_e136903_d_n11;
            locals.var_fs01_dn14 = assign89550_body45_e136903_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89550_body46_e136914, assign89550_body46_e136914_d_n0, assign89550_body46_e136914_d_n2, assign89550_body46_e136914_d_n4, assign89550_body46_e136914_d_n5, assign89550_body46_e136914_d_n6, assign89550_body46_e136914_d_n7, assign89550_body46_e136914_d_n8, assign89550_body46_e136914_d_n9, assign89550_body46_e136914_d_n10, assign89550_body46_e136914_d_n11, assign89550_body46_e136914_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89550_body46_e136914;
            locals.var_fs01_dps0_dn0 = assign89550_body46_e136914_d_n0;
            locals.var_fs01_dps0_dn2 = assign89550_body46_e136914_d_n2;
            locals.var_fs01_dps0_dn4 = assign89550_body46_e136914_d_n4;
            locals.var_fs01_dps0_dn5 = assign89550_body46_e136914_d_n5;
            locals.var_fs01_dps0_dn6 = assign89550_body46_e136914_d_n6;
            locals.var_fs01_dps0_dn7 = assign89550_body46_e136914_d_n7;
            locals.var_fs01_dps0_dn8 = assign89550_body46_e136914_d_n8;
            locals.var_fs01_dps0_dn9 = assign89550_body46_e136914_d_n9;
            locals.var_fs01_dps0_dn10 = assign89550_body46_e136914_d_n10;
            locals.var_fs01_dps0_dn11 = assign89550_body46_e136914_d_n11;
            locals.var_fs01_dps0_dn14 = assign89550_body46_e136914_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89550_body47_e136926, assign89550_body47_e136926_d_n0, assign89550_body47_e136926_d_n2, assign89550_body47_e136926_d_n4, assign89550_body47_e136926_d_n5, assign89550_body47_e136926_d_n6, assign89550_body47_e136926_d_n7, assign89550_body47_e136926_d_n8, assign89550_body47_e136926_d_n9, assign89550_body47_e136926_d_n10, assign89550_body47_e136926_d_n11, assign89550_body47_e136926_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89550_body47_e136924: f64 = (-locals.var_fb);
        (assign89550_body47_e136924, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89550_body47_e136926;
            locals.var_fs02_dn0 = assign89550_body47_e136926_d_n0;
            locals.var_fs02_dn2 = assign89550_body47_e136926_d_n2;
            locals.var_fs02_dn4 = assign89550_body47_e136926_d_n4;
            locals.var_fs02_dn5 = assign89550_body47_e136926_d_n5;
            locals.var_fs02_dn6 = assign89550_body47_e136926_d_n6;
            locals.var_fs02_dn7 = assign89550_body47_e136926_d_n7;
            locals.var_fs02_dn8 = assign89550_body47_e136926_d_n8;
            locals.var_fs02_dn9 = assign89550_body47_e136926_d_n9;
            locals.var_fs02_dn10 = assign89550_body47_e136926_d_n10;
            locals.var_fs02_dn11 = assign89550_body47_e136926_d_n11;
            locals.var_fs02_dn14 = assign89550_body47_e136926_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89550_body48_e136938, assign89550_body48_e136938_d_n0, assign89550_body48_e136938_d_n2, assign89550_body48_e136938_d_n4, assign89550_body48_e136938_d_n5, assign89550_body48_e136938_d_n6, assign89550_body48_e136938_d_n7, assign89550_body48_e136938_d_n8, assign89550_body48_e136938_d_n9, assign89550_body48_e136938_d_n10, assign89550_body48_e136938_d_n11, assign89550_body48_e136938_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89550_body48_e136936: f64 = (-locals.var_fb_dpss);
        (assign89550_body48_e136936, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89550_body48_e136938;
            locals.var_fs02_dps0_dn0 = assign89550_body48_e136938_d_n0;
            locals.var_fs02_dps0_dn2 = assign89550_body48_e136938_d_n2;
            locals.var_fs02_dps0_dn4 = assign89550_body48_e136938_d_n4;
            locals.var_fs02_dps0_dn5 = assign89550_body48_e136938_d_n5;
            locals.var_fs02_dps0_dn6 = assign89550_body48_e136938_d_n6;
            locals.var_fs02_dps0_dn7 = assign89550_body48_e136938_d_n7;
            locals.var_fs02_dps0_dn8 = assign89550_body48_e136938_d_n8;
            locals.var_fs02_dps0_dn9 = assign89550_body48_e136938_d_n9;
            locals.var_fs02_dps0_dn10 = assign89550_body48_e136938_d_n10;
            locals.var_fs02_dps0_dn11 = assign89550_body48_e136938_d_n11;
            locals.var_fs02_dps0_dn14 = assign89550_body48_e136938_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign89550_body49_e136941: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2080 = assign89550_body49_e136941;
            locals.var_guard2080_rv = 0.0;
            let assign89550_body50_e136944: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2081 = assign89550_body50_e136944;
            locals.var_guard2081_rv = 0.0;
            let (assign89550_body51_e136982, assign89550_body51_e136982_d_n0, assign89550_body51_e136982_d_n2, assign89550_body51_e136982_d_n4, assign89550_body51_e136982_d_n5, assign89550_body51_e136982_d_n6, assign89550_body51_e136982_d_n7, assign89550_body51_e136982_d_n8, assign89550_body51_e136982_d_n9, assign89550_body51_e136982_d_n10, assign89550_body51_e136982_d_n11, assign89550_body51_e136982_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 != 0.0)) && (locals.var_guard2081 != 0.0)) {
        let assign89550_body51_e136960: f64 = (locals.var_chi * locals.var_chi);
        let assign89550_body51_e136962: f64 = (assign89550_body51_e136960 / 2.0);
        let assign89550_body51_e136966: f64 = (locals.var_chi / 3.0);
        let assign89550_body51_e136970: f64 = (locals.var_chi / 4.0);
        let assign89550_body51_e136974: f64 = (locals.var_chi / 5.0);
        let assign89550_body51_e136975: f64 = (1.0 + assign89550_body51_e136974);
        let assign89550_body51_e136976: f64 = (assign89550_body51_e136970 * assign89550_body51_e136975);
        let assign89550_body51_e136977: f64 = (1.0 + assign89550_body51_e136976);
        let assign89550_body51_e136978: f64 = (assign89550_body51_e136966 * assign89550_body51_e136977);
        let assign89550_body51_e136979: f64 = (1.0 + assign89550_body51_e136978);
        let assign89550_body51_e136980: f64 = (assign89550_body51_e136962 * assign89550_body51_e136979);
        (assign89550_body51_e136980, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn0 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn0 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn2 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn2 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn4 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn4 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn5 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn5 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn6 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn6 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn7 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn7 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn8 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn8 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn9 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn9 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn10 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn10 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn11 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn11 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign89550_body51_e136979) + (assign89550_body51_e136962 * (((locals.var_chi_dn14 / 3.0) * assign89550_body51_e136977) + (assign89550_body51_e136966 * (((locals.var_chi_dn14 / 4.0) * assign89550_body51_e136975) + (assign89550_body51_e136970 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89550_body51_e136982;
            locals.var_t0_dn0 = assign89550_body51_e136982_d_n0;
            locals.var_t0_dn2 = assign89550_body51_e136982_d_n2;
            locals.var_t0_dn4 = assign89550_body51_e136982_d_n4;
            locals.var_t0_dn5 = assign89550_body51_e136982_d_n5;
            locals.var_t0_dn6 = assign89550_body51_e136982_d_n6;
            locals.var_t0_dn7 = assign89550_body51_e136982_d_n7;
            locals.var_t0_dn8 = assign89550_body51_e136982_d_n8;
            locals.var_t0_dn9 = assign89550_body51_e136982_d_n9;
            locals.var_t0_dn10 = assign89550_body51_e136982_d_n10;
            locals.var_t0_dn11 = assign89550_body51_e136982_d_n11;
            locals.var_t0_dn14 = assign89550_body51_e136982_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89550_body52_e137016, assign89550_body52_e137016_d_n0, assign89550_body52_e137016_d_n2, assign89550_body52_e137016_d_n4, assign89550_body52_e137016_d_n5, assign89550_body52_e137016_d_n6, assign89550_body52_e137016_d_n7, assign89550_body52_e137016_d_n8, assign89550_body52_e137016_d_n9, assign89550_body52_e137016_d_n10, assign89550_body52_e137016_d_n11, assign89550_body52_e137016_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 != 0.0)) && (locals.var_guard2081 != 0.0)) {
        let assign89550_body52_e137000: f64 = (locals.var_chi / 2.0);
        let assign89550_body52_e137004: f64 = (locals.var_chi / 3.0);
        let assign89550_body52_e137008: f64 = (locals.var_chi / 4.0);
        let assign89550_body52_e137009: f64 = (1.0 + assign89550_body52_e137008);
        let assign89550_body52_e137010: f64 = (assign89550_body52_e137004 * assign89550_body52_e137009);
        let assign89550_body52_e137011: f64 = (1.0 + assign89550_body52_e137010);
        let assign89550_body52_e137012: f64 = (assign89550_body52_e137000 * assign89550_body52_e137011);
        let assign89550_body52_e137013: f64 = (1.0 + assign89550_body52_e137012);
        let assign89550_body52_e137014: f64 = (locals.var_chi * assign89550_body52_e137013);
        (assign89550_body52_e137014, ((locals.var_chi_dn0 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn0 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn2 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn4 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn5 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn6 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn7 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn8 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn9 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn10 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn11 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign89550_body52_e137013) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign89550_body52_e137011) + (assign89550_body52_e137000 * (((locals.var_chi_dn14 / 3.0) * assign89550_body52_e137009) + (assign89550_body52_e137004 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89550_body52_e137016;
            locals.var_t1_dn0 = assign89550_body52_e137016_d_n0;
            locals.var_t1_dn2 = assign89550_body52_e137016_d_n2;
            locals.var_t1_dn4 = assign89550_body52_e137016_d_n4;
            locals.var_t1_dn5 = assign89550_body52_e137016_d_n5;
            locals.var_t1_dn6 = assign89550_body52_e137016_d_n6;
            locals.var_t1_dn7 = assign89550_body52_e137016_d_n7;
            locals.var_t1_dn8 = assign89550_body52_e137016_d_n8;
            locals.var_t1_dn9 = assign89550_body52_e137016_d_n9;
            locals.var_t1_dn10 = assign89550_body52_e137016_d_n10;
            locals.var_t1_dn11 = assign89550_body52_e137016_d_n11;
            locals.var_t1_dn14 = assign89550_body52_e137016_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89550_body53_e137034, assign89550_body53_e137034_d_n0, assign89550_body53_e137034_d_n2, assign89550_body53_e137034_d_n4, assign89550_body53_e137034_d_n5, assign89550_body53_e137034_d_n6, assign89550_body53_e137034_d_n7, assign89550_body53_e137034_d_n8, assign89550_body53_e137034_d_n9, assign89550_body53_e137034_d_n10, assign89550_body53_e137034_d_n11, assign89550_body53_e137034_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 != 0.0)) && (locals.var_guard2081 != 0.0)) {
        let assign89550_body53_e137032: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign89550_body53_e137032, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89550_body53_e137034;
            locals.var_fs01_dn0 = assign89550_body53_e137034_d_n0;
            locals.var_fs01_dn2 = assign89550_body53_e137034_d_n2;
            locals.var_fs01_dn4 = assign89550_body53_e137034_d_n4;
            locals.var_fs01_dn5 = assign89550_body53_e137034_d_n5;
            locals.var_fs01_dn6 = assign89550_body53_e137034_d_n6;
            locals.var_fs01_dn7 = assign89550_body53_e137034_d_n7;
            locals.var_fs01_dn8 = assign89550_body53_e137034_d_n8;
            locals.var_fs01_dn9 = assign89550_body53_e137034_d_n9;
            locals.var_fs01_dn10 = assign89550_body53_e137034_d_n10;
            locals.var_fs01_dn11 = assign89550_body53_e137034_d_n11;
            locals.var_fs01_dn14 = assign89550_body53_e137034_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89550_body54_e137054, assign89550_body54_e137054_d_n0, assign89550_body54_e137054_d_n2, assign89550_body54_e137054_d_n4, assign89550_body54_e137054_d_n5, assign89550_body54_e137054_d_n6, assign89550_body54_e137054_d_n7, assign89550_body54_e137054_d_n8, assign89550_body54_e137054_d_n9, assign89550_body54_e137054_d_n10, assign89550_body54_e137054_d_n11, assign89550_body54_e137054_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 != 0.0)) && (locals.var_guard2081 != 0.0)) {
        let assign89550_body54_e137050: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign89550_body54_e137052: f64 = (assign89550_body54_e137050 * locals.var_beta);
        (assign89550_body54_e137052, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign89550_body54_e137050 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89550_body54_e137054;
            locals.var_fs01_dps0_dn0 = assign89550_body54_e137054_d_n0;
            locals.var_fs01_dps0_dn2 = assign89550_body54_e137054_d_n2;
            locals.var_fs01_dps0_dn4 = assign89550_body54_e137054_d_n4;
            locals.var_fs01_dps0_dn5 = assign89550_body54_e137054_d_n5;
            locals.var_fs01_dps0_dn6 = assign89550_body54_e137054_d_n6;
            locals.var_fs01_dps0_dn7 = assign89550_body54_e137054_d_n7;
            locals.var_fs01_dps0_dn8 = assign89550_body54_e137054_d_n8;
            locals.var_fs01_dps0_dn9 = assign89550_body54_e137054_d_n9;
            locals.var_fs01_dps0_dn10 = assign89550_body54_e137054_d_n10;
            locals.var_fs01_dps0_dn11 = assign89550_body54_e137054_d_n11;
            locals.var_fs01_dps0_dn14 = assign89550_body54_e137054_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89550_body55_e137072, assign89550_body55_e137072_d_n0, assign89550_body55_e137072_d_n2, assign89550_body55_e137072_d_n4, assign89550_body55_e137072_d_n5, assign89550_body55_e137072_d_n6, assign89550_body55_e137072_d_n7, assign89550_body55_e137072_d_n8, assign89550_body55_e137072_d_n9, assign89550_body55_e137072_d_n10, assign89550_body55_e137072_d_n11, assign89550_body55_e137072_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89550_body55_e137070: f64 = (locals.var_chi).exp();
        (assign89550_body55_e137070, (assign89550_body55_e137070 * locals.var_chi_dn0), (assign89550_body55_e137070 * locals.var_chi_dn2), (assign89550_body55_e137070 * locals.var_chi_dn4), (assign89550_body55_e137070 * locals.var_chi_dn5), (assign89550_body55_e137070 * locals.var_chi_dn6), (assign89550_body55_e137070 * locals.var_chi_dn7), (assign89550_body55_e137070 * locals.var_chi_dn8), (assign89550_body55_e137070 * locals.var_chi_dn9), (assign89550_body55_e137070 * locals.var_chi_dn10), (assign89550_body55_e137070 * locals.var_chi_dn11), (assign89550_body55_e137070 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign89550_body55_e137072;
            locals.var_exp_chi_dn0 = assign89550_body55_e137072_d_n0;
            locals.var_exp_chi_dn2 = assign89550_body55_e137072_d_n2;
            locals.var_exp_chi_dn4 = assign89550_body55_e137072_d_n4;
            locals.var_exp_chi_dn5 = assign89550_body55_e137072_d_n5;
            locals.var_exp_chi_dn6 = assign89550_body55_e137072_d_n6;
            locals.var_exp_chi_dn7 = assign89550_body55_e137072_d_n7;
            locals.var_exp_chi_dn8 = assign89550_body55_e137072_d_n8;
            locals.var_exp_chi_dn9 = assign89550_body55_e137072_d_n9;
            locals.var_exp_chi_dn10 = assign89550_body55_e137072_d_n10;
            locals.var_exp_chi_dn11 = assign89550_body55_e137072_d_n11;
            locals.var_exp_chi_dn14 = assign89550_body55_e137072_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign89550_body56_e137091, assign89550_body56_e137091_d_n0, assign89550_body56_e137091_d_n2, assign89550_body56_e137091_d_n4, assign89550_body56_e137091_d_n5, assign89550_body56_e137091_d_n6, assign89550_body56_e137091_d_n7, assign89550_body56_e137091_d_n8, assign89550_body56_e137091_d_n9, assign89550_body56_e137091_d_n10, assign89550_body56_e137091_d_n11, assign89550_body56_e137091_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89550_body56_e137089: f64 = (locals.var_exp_chi - 1.0);
        (assign89550_body56_e137089, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89550_body56_e137091;
            locals.var_t1_dn0 = assign89550_body56_e137091_d_n0;
            locals.var_t1_dn2 = assign89550_body56_e137091_d_n2;
            locals.var_t1_dn4 = assign89550_body56_e137091_d_n4;
            locals.var_t1_dn5 = assign89550_body56_e137091_d_n5;
            locals.var_t1_dn6 = assign89550_body56_e137091_d_n6;
            locals.var_t1_dn7 = assign89550_body56_e137091_d_n7;
            locals.var_t1_dn8 = assign89550_body56_e137091_d_n8;
            locals.var_t1_dn9 = assign89550_body56_e137091_d_n9;
            locals.var_t1_dn10 = assign89550_body56_e137091_d_n10;
            locals.var_t1_dn11 = assign89550_body56_e137091_d_n11;
            locals.var_t1_dn14 = assign89550_body56_e137091_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89550_body57_e137112, assign89550_body57_e137112_d_n0, assign89550_body57_e137112_d_n2, assign89550_body57_e137112_d_n4, assign89550_body57_e137112_d_n5, assign89550_body57_e137112_d_n6, assign89550_body57_e137112_d_n7, assign89550_body57_e137112_d_n8, assign89550_body57_e137112_d_n9, assign89550_body57_e137112_d_n10, assign89550_body57_e137112_d_n11, assign89550_body57_e137112_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89550_body57_e137109: f64 = (locals.var_t1 - locals.var_chi);
        let assign89550_body57_e137110: f64 = (locals.var_cfs1 * assign89550_body57_e137109);
        (assign89550_body57_e137110, ((locals.var_cfs1_dn0 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign89550_body57_e137109) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89550_body57_e137112;
            locals.var_fs01_dn0 = assign89550_body57_e137112_d_n0;
            locals.var_fs01_dn2 = assign89550_body57_e137112_d_n2;
            locals.var_fs01_dn4 = assign89550_body57_e137112_d_n4;
            locals.var_fs01_dn5 = assign89550_body57_e137112_d_n5;
            locals.var_fs01_dn6 = assign89550_body57_e137112_d_n6;
            locals.var_fs01_dn7 = assign89550_body57_e137112_d_n7;
            locals.var_fs01_dn8 = assign89550_body57_e137112_d_n8;
            locals.var_fs01_dn9 = assign89550_body57_e137112_d_n9;
            locals.var_fs01_dn10 = assign89550_body57_e137112_d_n10;
            locals.var_fs01_dn11 = assign89550_body57_e137112_d_n11;
            locals.var_fs01_dn14 = assign89550_body57_e137112_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89550_body58_e137133, assign89550_body58_e137133_d_n0, assign89550_body58_e137133_d_n2, assign89550_body58_e137133_d_n4, assign89550_body58_e137133_d_n5, assign89550_body58_e137133_d_n6, assign89550_body58_e137133_d_n7, assign89550_body58_e137133_d_n8, assign89550_body58_e137133_d_n9, assign89550_body58_e137133_d_n10, assign89550_body58_e137133_d_n11, assign89550_body58_e137133_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89550_body58_e137129: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign89550_body58_e137131: f64 = (assign89550_body58_e137129 * locals.var_t1);
        (assign89550_body58_e137131, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign89550_body58_e137129 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89550_body58_e137133;
            locals.var_fs01_dps0_dn0 = assign89550_body58_e137133_d_n0;
            locals.var_fs01_dps0_dn2 = assign89550_body58_e137133_d_n2;
            locals.var_fs01_dps0_dn4 = assign89550_body58_e137133_d_n4;
            locals.var_fs01_dps0_dn5 = assign89550_body58_e137133_d_n5;
            locals.var_fs01_dps0_dn6 = assign89550_body58_e137133_d_n6;
            locals.var_fs01_dps0_dn7 = assign89550_body58_e137133_d_n7;
            locals.var_fs01_dps0_dn8 = assign89550_body58_e137133_d_n8;
            locals.var_fs01_dps0_dn9 = assign89550_body58_e137133_d_n9;
            locals.var_fs01_dps0_dn10 = assign89550_body58_e137133_d_n10;
            locals.var_fs01_dps0_dn11 = assign89550_body58_e137133_d_n11;
            locals.var_fs01_dps0_dn14 = assign89550_body58_e137133_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89550_body60_e137168, assign89550_body60_e137168_d_n0, assign89550_body60_e137168_d_n2, assign89550_body60_e137168_d_n4, assign89550_body60_e137168_d_n5, assign89550_body60_e137168_d_n6, assign89550_body60_e137168_d_n7, assign89550_body60_e137168_d_n8, assign89550_body60_e137168_d_n9, assign89550_body60_e137168_d_n10, assign89550_body60_e137168_d_n11, assign89550_body60_e137168_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 == 0.0)) {
        let assign89550_body60_e137165: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign89550_body60_e137166: f64 = (assign89550_body60_e137165).exp();
        (assign89550_body60_e137166, (assign89550_body60_e137166 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign89550_body60_e137166 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign89550_body60_e137166 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign89550_body60_e137166 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign89550_body60_e137166 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign89550_body60_e137166 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign89550_body60_e137166 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign89550_body60_e137166 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign89550_body60_e137166 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign89550_body60_e137166 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign89550_body60_e137166 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign89550_body60_e137168;
            locals.var_exp_bps0_dn0 = assign89550_body60_e137168_d_n0;
            locals.var_exp_bps0_dn2 = assign89550_body60_e137168_d_n2;
            locals.var_exp_bps0_dn4 = assign89550_body60_e137168_d_n4;
            locals.var_exp_bps0_dn5 = assign89550_body60_e137168_d_n5;
            locals.var_exp_bps0_dn6 = assign89550_body60_e137168_d_n6;
            locals.var_exp_bps0_dn7 = assign89550_body60_e137168_d_n7;
            locals.var_exp_bps0_dn8 = assign89550_body60_e137168_d_n8;
            locals.var_exp_bps0_dn9 = assign89550_body60_e137168_d_n9;
            locals.var_exp_bps0_dn10 = assign89550_body60_e137168_d_n10;
            locals.var_exp_bps0_dn11 = assign89550_body60_e137168_d_n11;
            locals.var_exp_bps0_dn14 = assign89550_body60_e137168_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign89550_body61_e137191, assign89550_body61_e137191_d_n0, assign89550_body61_e137191_d_n2, assign89550_body61_e137191_d_n4, assign89550_body61_e137191_d_n5, assign89550_body61_e137191_d_n6, assign89550_body61_e137191_d_n7, assign89550_body61_e137191_d_n8, assign89550_body61_e137191_d_n9, assign89550_body61_e137191_d_n10, assign89550_body61_e137191_d_n11, assign89550_body61_e137191_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 == 0.0)) {
        let assign89550_body61_e137186: f64 = (locals.var_chi + 1.0);
        let assign89550_body61_e137187: f64 = (locals.var_exp_bvbs * assign89550_body61_e137186);
        let assign89550_body61_e137188: f64 = (locals.var_exp_bps0 - assign89550_body61_e137187);
        let assign89550_body61_e137189: f64 = (locals.var_cnst1over * assign89550_body61_e137188);
        (assign89550_body61_e137189, ((locals.var_cnst1over_dn0 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign89550_body61_e137188) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign89550_body61_e137186) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89550_body61_e137191;
            locals.var_fs01_dn0 = assign89550_body61_e137191_d_n0;
            locals.var_fs01_dn2 = assign89550_body61_e137191_d_n2;
            locals.var_fs01_dn4 = assign89550_body61_e137191_d_n4;
            locals.var_fs01_dn5 = assign89550_body61_e137191_d_n5;
            locals.var_fs01_dn6 = assign89550_body61_e137191_d_n6;
            locals.var_fs01_dn7 = assign89550_body61_e137191_d_n7;
            locals.var_fs01_dn8 = assign89550_body61_e137191_d_n8;
            locals.var_fs01_dn9 = assign89550_body61_e137191_d_n9;
            locals.var_fs01_dn10 = assign89550_body61_e137191_d_n10;
            locals.var_fs01_dn11 = assign89550_body61_e137191_d_n11;
            locals.var_fs01_dn14 = assign89550_body61_e137191_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89550_body62_e137212, assign89550_body62_e137212_d_n0, assign89550_body62_e137212_d_n2, assign89550_body62_e137212_d_n4, assign89550_body62_e137212_d_n5, assign89550_body62_e137212_d_n6, assign89550_body62_e137212_d_n7, assign89550_body62_e137212_d_n8, assign89550_body62_e137212_d_n9, assign89550_body62_e137212_d_n10, assign89550_body62_e137212_d_n11, assign89550_body62_e137212_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2080 == 0.0)) {
        let assign89550_body62_e137206: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign89550_body62_e137209: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign89550_body62_e137210: f64 = (assign89550_body62_e137206 * assign89550_body62_e137209);
        (assign89550_body62_e137210, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign89550_body62_e137209) + (assign89550_body62_e137206 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89550_body62_e137212;
            locals.var_fs01_dps0_dn0 = assign89550_body62_e137212_d_n0;
            locals.var_fs01_dps0_dn2 = assign89550_body62_e137212_d_n2;
            locals.var_fs01_dps0_dn4 = assign89550_body62_e137212_d_n4;
            locals.var_fs01_dps0_dn5 = assign89550_body62_e137212_d_n5;
            locals.var_fs01_dps0_dn6 = assign89550_body62_e137212_d_n6;
            locals.var_fs01_dps0_dn7 = assign89550_body62_e137212_d_n7;
            locals.var_fs01_dps0_dn8 = assign89550_body62_e137212_d_n8;
            locals.var_fs01_dps0_dn9 = assign89550_body62_e137212_d_n9;
            locals.var_fs01_dps0_dn10 = assign89550_body62_e137212_d_n10;
            locals.var_fs01_dps0_dn11 = assign89550_body62_e137212_d_n11;
            locals.var_fs01_dps0_dn14 = assign89550_body62_e137212_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign89550_body63_e137215: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2082 = assign89550_body63_e137215;
            locals.var_guard2082_rv = 0.0;
            let (assign89550_body64_e137234, assign89550_body64_e137234_d_n0, assign89550_body64_e137234_d_n2, assign89550_body64_e137234_d_n4, assign89550_body64_e137234_d_n5, assign89550_body64_e137234_d_n6, assign89550_body64_e137234_d_n7, assign89550_body64_e137234_d_n8, assign89550_body64_e137234_d_n9, assign89550_body64_e137234_d_n10, assign89550_body64_e137234_d_n11, assign89550_body64_e137234_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2082 != 0.0)) {
        let assign89550_body64_e137229: f64 = (locals.var_fb * locals.var_fb);
        let assign89550_body64_e137231: f64 = (assign89550_body64_e137229 + locals.var_fs01);
        let assign89550_body64_e137232: f64 = (assign89550_body64_e137231).sqrt();
        (assign89550_body64_e137232, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign89550_body64_e137232)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign89550_body64_e137232)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89550_body64_e137234;
            locals.var_fs02_dn0 = assign89550_body64_e137234_d_n0;
            locals.var_fs02_dn2 = assign89550_body64_e137234_d_n2;
            locals.var_fs02_dn4 = assign89550_body64_e137234_d_n4;
            locals.var_fs02_dn5 = assign89550_body64_e137234_d_n5;
            locals.var_fs02_dn6 = assign89550_body64_e137234_d_n6;
            locals.var_fs02_dn7 = assign89550_body64_e137234_d_n7;
            locals.var_fs02_dn8 = assign89550_body64_e137234_d_n8;
            locals.var_fs02_dn9 = assign89550_body64_e137234_d_n9;
            locals.var_fs02_dn10 = assign89550_body64_e137234_d_n10;
            locals.var_fs02_dn11 = assign89550_body64_e137234_d_n11;
            locals.var_fs02_dn14 = assign89550_body64_e137234_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89550_body65_e137258, assign89550_body65_e137258_d_n0, assign89550_body65_e137258_d_n2, assign89550_body65_e137258_d_n4, assign89550_body65_e137258_d_n5, assign89550_body65_e137258_d_n6, assign89550_body65_e137258_d_n7, assign89550_body65_e137258_d_n8, assign89550_body65_e137258_d_n9, assign89550_body65_e137258_d_n10, assign89550_body65_e137258_d_n11, assign89550_body65_e137258_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2082 != 0.0)) {
        let assign89550_body65_e137249: f64 = (2.0 * locals.var_fb_dpss);
        let assign89550_body65_e137251: f64 = (assign89550_body65_e137249 * locals.var_fb);
        let assign89550_body65_e137253: f64 = (assign89550_body65_e137251 + locals.var_fs01_dps0);
        let assign89550_body65_e137254: f64 = (0.5 * assign89550_body65_e137253);
        let assign89550_body65_e137256: f64 = (assign89550_body65_e137254 / locals.var_fs02);
        (assign89550_body65_e137256, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn14) * locals.var_fb) + (assign89550_body65_e137249 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign89550_body65_e137254 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89550_body65_e137258;
            locals.var_fs02_dps0_dn0 = assign89550_body65_e137258_d_n0;
            locals.var_fs02_dps0_dn2 = assign89550_body65_e137258_d_n2;
            locals.var_fs02_dps0_dn4 = assign89550_body65_e137258_d_n4;
            locals.var_fs02_dps0_dn5 = assign89550_body65_e137258_d_n5;
            locals.var_fs02_dps0_dn6 = assign89550_body65_e137258_d_n6;
            locals.var_fs02_dps0_dn7 = assign89550_body65_e137258_d_n7;
            locals.var_fs02_dps0_dn8 = assign89550_body65_e137258_d_n8;
            locals.var_fs02_dps0_dn9 = assign89550_body65_e137258_d_n9;
            locals.var_fs02_dps0_dn10 = assign89550_body65_e137258_d_n10;
            locals.var_fs02_dps0_dn11 = assign89550_body65_e137258_d_n11;
            locals.var_fs02_dps0_dn14 = assign89550_body65_e137258_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89550_body67_e137290, assign89550_body67_e137290_d_n0, assign89550_body67_e137290_d_n2, assign89550_body67_e137290_d_n4, assign89550_body67_e137290_d_n5, assign89550_body67_e137290_d_n6, assign89550_body67_e137290_d_n7, assign89550_body67_e137290_d_n8, assign89550_body67_e137290_d_n9, assign89550_body67_e137290_d_n10, assign89550_body67_e137290_d_n11, assign89550_body67_e137290_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2082 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89550_body67_e137290;
            locals.var_fs02_dn0 = assign89550_body67_e137290_d_n0;
            locals.var_fs02_dn2 = assign89550_body67_e137290_d_n2;
            locals.var_fs02_dn4 = assign89550_body67_e137290_d_n4;
            locals.var_fs02_dn5 = assign89550_body67_e137290_d_n5;
            locals.var_fs02_dn6 = assign89550_body67_e137290_d_n6;
            locals.var_fs02_dn7 = assign89550_body67_e137290_d_n7;
            locals.var_fs02_dn8 = assign89550_body67_e137290_d_n8;
            locals.var_fs02_dn9 = assign89550_body67_e137290_d_n9;
            locals.var_fs02_dn10 = assign89550_body67_e137290_d_n10;
            locals.var_fs02_dn11 = assign89550_body67_e137290_d_n11;
            locals.var_fs02_dn14 = assign89550_body67_e137290_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89550_body68_e137305, assign89550_body68_e137305_d_n0, assign89550_body68_e137305_d_n2, assign89550_body68_e137305_d_n4, assign89550_body68_e137305_d_n5, assign89550_body68_e137305_d_n6, assign89550_body68_e137305_d_n7, assign89550_body68_e137305_d_n8, assign89550_body68_e137305_d_n9, assign89550_body68_e137305_d_n10, assign89550_body68_e137305_d_n11, assign89550_body68_e137305_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2082 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89550_body68_e137305;
            locals.var_fs02_dps0_dn0 = assign89550_body68_e137305_d_n0;
            locals.var_fs02_dps0_dn2 = assign89550_body68_e137305_d_n2;
            locals.var_fs02_dps0_dn4 = assign89550_body68_e137305_d_n4;
            locals.var_fs02_dps0_dn5 = assign89550_body68_e137305_d_n5;
            locals.var_fs02_dps0_dn6 = assign89550_body68_e137305_d_n6;
            locals.var_fs02_dps0_dn7 = assign89550_body68_e137305_d_n7;
            locals.var_fs02_dps0_dn8 = assign89550_body68_e137305_d_n8;
            locals.var_fs02_dps0_dn9 = assign89550_body68_e137305_d_n9;
            locals.var_fs02_dps0_dn10 = assign89550_body68_e137305_d_n10;
            locals.var_fs02_dps0_dn11 = assign89550_body68_e137305_d_n11;
            locals.var_fs02_dps0_dn14 = assign89550_body68_e137305_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89550_body69_e137321, assign89550_body69_e137321_d_n0, assign89550_body69_e137321_d_n2, assign89550_body69_e137321_d_n4, assign89550_body69_e137321_d_n5, assign89550_body69_e137321_d_n6, assign89550_body69_e137321_d_n7, assign89550_body69_e137321_d_n8, assign89550_body69_e137321_d_n9, assign89550_body69_e137321_d_n10, assign89550_body69_e137321_d_n11, assign89550_body69_e137321_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89550_body69_e137313: f64 = (-locals.var_vgpld);
        let assign89550_body69_e137315: f64 = (assign89550_body69_e137313 + locals.var_ps0ld);
        let assign89550_body69_e137318: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign89550_body69_e137319: f64 = (assign89550_body69_e137315 + assign89550_body69_e137318);
        (assign89550_body69_e137319, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign89550_body69_e137321;
            locals.var_fs0_dn0 = assign89550_body69_e137321_d_n0;
            locals.var_fs0_dn2 = assign89550_body69_e137321_d_n2;
            locals.var_fs0_dn4 = assign89550_body69_e137321_d_n4;
            locals.var_fs0_dn5 = assign89550_body69_e137321_d_n5;
            locals.var_fs0_dn6 = assign89550_body69_e137321_d_n6;
            locals.var_fs0_dn7 = assign89550_body69_e137321_d_n7;
            locals.var_fs0_dn8 = assign89550_body69_e137321_d_n8;
            locals.var_fs0_dn9 = assign89550_body69_e137321_d_n9;
            locals.var_fs0_dn10 = assign89550_body69_e137321_d_n10;
            locals.var_fs0_dn11 = assign89550_body69_e137321_d_n11;
            locals.var_fs0_dn14 = assign89550_body69_e137321_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign89550_body70_e137334, assign89550_body70_e137334_d_n0, assign89550_body70_e137334_d_n2, assign89550_body70_e137334_d_n4, assign89550_body70_e137334_d_n5, assign89550_body70_e137334_d_n6, assign89550_body70_e137334_d_n7, assign89550_body70_e137334_d_n8, assign89550_body70_e137334_d_n9, assign89550_body70_e137334_d_n10, assign89550_body70_e137334_d_n11, assign89550_body70_e137334_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89550_body70_e137331: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign89550_body70_e137332: f64 = (1.0 + assign89550_body70_e137331);
        (assign89550_body70_e137332, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign89550_body70_e137334;
            locals.var_fs0_dps0_dn0 = assign89550_body70_e137334_d_n0;
            locals.var_fs0_dps0_dn2 = assign89550_body70_e137334_d_n2;
            locals.var_fs0_dps0_dn4 = assign89550_body70_e137334_d_n4;
            locals.var_fs0_dps0_dn5 = assign89550_body70_e137334_d_n5;
            locals.var_fs0_dps0_dn6 = assign89550_body70_e137334_d_n6;
            locals.var_fs0_dps0_dn7 = assign89550_body70_e137334_d_n7;
            locals.var_fs0_dps0_dn8 = assign89550_body70_e137334_d_n8;
            locals.var_fs0_dps0_dn9 = assign89550_body70_e137334_d_n9;
            locals.var_fs0_dps0_dn10 = assign89550_body70_e137334_d_n10;
            locals.var_fs0_dps0_dn11 = assign89550_body70_e137334_d_n11;
            locals.var_fs0_dps0_dn14 = assign89550_body70_e137334_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign89550_body71_e137337: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard2083 = assign89550_body71_e137337;
            locals.var_guard2083_rv = 0.0;
            let (assign89550_body72_e137350,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2083 != 0.0)) {
        let assign89550_body72_e137348: f64 = (locals.var_lp_s0_max + 1.0);
        (assign89550_body72_e137348,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89550_body72_e137350;
            locals.var_lp_s0_rv = 0.0;
            let (assign89550_body73_e137365, assign89550_body73_e137365_d_n0, assign89550_body73_e137365_d_n2, assign89550_body73_e137365_d_n4, assign89550_body73_e137365_d_n5, assign89550_body73_e137365_d_n6, assign89550_body73_e137365_d_n7, assign89550_body73_e137365_d_n8, assign89550_body73_e137365_d_n9, assign89550_body73_e137365_d_n10, assign89550_body73_e137365_d_n11, assign89550_body73_e137365_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2083 == 0.0)) {
        let assign89550_body73_e137361: f64 = (-locals.var_fs0);
        let assign89550_body73_e137363: f64 = (assign89550_body73_e137361 / locals.var_fs0_dps0);
        (assign89550_body73_e137363, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign89550_body73_e137361 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign89550_body73_e137365;
            locals.var_dps0_dn0 = assign89550_body73_e137365_d_n0;
            locals.var_dps0_dn2 = assign89550_body73_e137365_d_n2;
            locals.var_dps0_dn4 = assign89550_body73_e137365_d_n4;
            locals.var_dps0_dn5 = assign89550_body73_e137365_d_n5;
            locals.var_dps0_dn6 = assign89550_body73_e137365_d_n6;
            locals.var_dps0_dn7 = assign89550_body73_e137365_d_n7;
            locals.var_dps0_dn8 = assign89550_body73_e137365_d_n8;
            locals.var_dps0_dn9 = assign89550_body73_e137365_d_n9;
            locals.var_dps0_dn10 = assign89550_body73_e137365_d_n10;
            locals.var_dps0_dn11 = assign89550_body73_e137365_d_n11;
            locals.var_dps0_dn14 = assign89550_body73_e137365_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign89550_body74_e137390, assign89550_body74_e137390_d_n0, assign89550_body74_e137390_d_n2, assign89550_body74_e137390_d_n4, assign89550_body74_e137390_d_n5, assign89550_body74_e137390_d_n6, assign89550_body74_e137390_d_n7, assign89550_body74_e137390_d_n8, assign89550_body74_e137390_d_n9, assign89550_body74_e137390_d_n10, assign89550_body74_e137390_d_n11, assign89550_body74_e137390_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2083 == 0.0)) {
        let assign89550_body74_e137377: f64 = (0.5 * 0.1);
        let assign89550_body74_e137381: f64 = (locals.var_ps0ld).abs();
        let (assign89550_body74_e137386, assign89550_body74_e137386_d_n0, assign89550_body74_e137386_d_n2, assign89550_body74_e137386_d_n4, assign89550_body74_e137386_d_n5, assign89550_body74_e137386_d_n6, assign89550_body74_e137386_d_n7, assign89550_body74_e137386_d_n8, assign89550_body74_e137386_d_n9, assign89550_body74_e137386_d_n10, assign89550_body74_e137386_d_n11, assign89550_body74_e137386_d_n14,) = {
            if (1.0 >= assign89550_body74_e137381) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89550_body74_e137385: f64 = (locals.var_ps0ld).abs();
                (assign89550_body74_e137385, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign89550_body74_e137387: f64 = (1.0 + assign89550_body74_e137386);
        let assign89550_body74_e137388: f64 = (assign89550_body74_e137377 * assign89550_body74_e137387);
        (assign89550_body74_e137388, (assign89550_body74_e137377 * assign89550_body74_e137386_d_n0), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n2), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n4), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n5), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n6), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n7), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n8), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n9), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n10), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n11), (assign89550_body74_e137377 * assign89550_body74_e137386_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign89550_body74_e137390;
            locals.var_dplim_dn0 = assign89550_body74_e137390_d_n0;
            locals.var_dplim_dn2 = assign89550_body74_e137390_d_n2;
            locals.var_dplim_dn4 = assign89550_body74_e137390_d_n4;
            locals.var_dplim_dn5 = assign89550_body74_e137390_d_n5;
            locals.var_dplim_dn6 = assign89550_body74_e137390_d_n6;
            locals.var_dplim_dn7 = assign89550_body74_e137390_d_n7;
            locals.var_dplim_dn8 = assign89550_body74_e137390_d_n8;
            locals.var_dplim_dn9 = assign89550_body74_e137390_d_n9;
            locals.var_dplim_dn10 = assign89550_body74_e137390_d_n10;
            locals.var_dplim_dn11 = assign89550_body74_e137390_d_n11;
            locals.var_dplim_dn14 = assign89550_body74_e137390_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign89550_body75_e137392: f64 = (locals.var_dps0).abs();
            let assign89550_body75_e137394: f64 = if assign89550_body75_e137392 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2084 = assign89550_body75_e137394;
            locals.var_guard2084_rv = 0.0;
            let (assign89550_body76_e137416, assign89550_body76_e137416_d_n0, assign89550_body76_e137416_d_n2, assign89550_body76_e137416_d_n4, assign89550_body76_e137416_d_n5, assign89550_body76_e137416_d_n6, assign89550_body76_e137416_d_n7, assign89550_body76_e137416_d_n8, assign89550_body76_e137416_d_n9, assign89550_body76_e137416_d_n10, assign89550_body76_e137416_d_n11, assign89550_body76_e137416_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2083 == 0.0)) && (locals.var_guard2084 != 0.0)) {
        let (assign89550_body76_e137413,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign89550_body76_e137412: f64 = (-1.0);
                (assign89550_body76_e137412,)
            }
        };
        let assign89550_body76_e137414: f64 = (locals.var_dplim * assign89550_body76_e137413);
        (assign89550_body76_e137414, (locals.var_dplim_dn0 * assign89550_body76_e137413), (locals.var_dplim_dn2 * assign89550_body76_e137413), (locals.var_dplim_dn4 * assign89550_body76_e137413), (locals.var_dplim_dn5 * assign89550_body76_e137413), (locals.var_dplim_dn6 * assign89550_body76_e137413), (locals.var_dplim_dn7 * assign89550_body76_e137413), (locals.var_dplim_dn8 * assign89550_body76_e137413), (locals.var_dplim_dn9 * assign89550_body76_e137413), (locals.var_dplim_dn10 * assign89550_body76_e137413), (locals.var_dplim_dn11 * assign89550_body76_e137413), (locals.var_dplim_dn14 * assign89550_body76_e137413),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign89550_body76_e137416;
            locals.var_dps0_dn0 = assign89550_body76_e137416_d_n0;
            locals.var_dps0_dn2 = assign89550_body76_e137416_d_n2;
            locals.var_dps0_dn4 = assign89550_body76_e137416_d_n4;
            locals.var_dps0_dn5 = assign89550_body76_e137416_d_n5;
            locals.var_dps0_dn6 = assign89550_body76_e137416_d_n6;
            locals.var_dps0_dn7 = assign89550_body76_e137416_d_n7;
            locals.var_dps0_dn8 = assign89550_body76_e137416_d_n8;
            locals.var_dps0_dn9 = assign89550_body76_e137416_d_n9;
            locals.var_dps0_dn10 = assign89550_body76_e137416_d_n10;
            locals.var_dps0_dn11 = assign89550_body76_e137416_d_n11;
            locals.var_dps0_dn14 = assign89550_body76_e137416_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign89550_body77_e137430, assign89550_body77_e137430_d_n0, assign89550_body77_e137430_d_n2, assign89550_body77_e137430_d_n4, assign89550_body77_e137430_d_n5, assign89550_body77_e137430_d_n6, assign89550_body77_e137430_d_n7, assign89550_body77_e137430_d_n8, assign89550_body77_e137430_d_n9, assign89550_body77_e137430_d_n10, assign89550_body77_e137430_d_n11, assign89550_body77_e137430_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2083 == 0.0)) {
        let assign89550_body77_e137428: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign89550_body77_e137428, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign89550_body77_e137430;
            locals.var_ps0ld_dn0 = assign89550_body77_e137430_d_n0;
            locals.var_ps0ld_dn2 = assign89550_body77_e137430_d_n2;
            locals.var_ps0ld_dn4 = assign89550_body77_e137430_d_n4;
            locals.var_ps0ld_dn5 = assign89550_body77_e137430_d_n5;
            locals.var_ps0ld_dn6 = assign89550_body77_e137430_d_n6;
            locals.var_ps0ld_dn7 = assign89550_body77_e137430_d_n7;
            locals.var_ps0ld_dn8 = assign89550_body77_e137430_d_n8;
            locals.var_ps0ld_dn9 = assign89550_body77_e137430_d_n9;
            locals.var_ps0ld_dn10 = assign89550_body77_e137430_d_n10;
            locals.var_ps0ld_dn11 = assign89550_body77_e137430_d_n11;
            locals.var_ps0ld_dn14 = assign89550_body77_e137430_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign89550_body78_e137432: f64 = (locals.var_dps0).abs();
            let assign89550_body78_e137436: f64 = (locals.var_fs0).abs();
            let assign89550_body78_e137439: f64 = if ((assign89550_body78_e137432 <= 1e-12) && (assign89550_body78_e137436 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2085 = assign89550_body78_e137439;
            locals.var_guard2085_rv = 0.0;
            let (assign89550_body79_e137453,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) && (locals.var_guard2083 == 0.0)) && (locals.var_guard2085 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign89550_body79_e137453;
            locals.var_flg_conv_rv = 0.0;
            let (assign89550_body80_e137464,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2030 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        let assign89550_body80_e137462: f64 = (locals.var_lp_s0 + 1.0);
        (assign89550_body80_e137462,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89550_body80_e137464;
            locals.var_lp_s0_rv = 0.0;
        }

    }
}
