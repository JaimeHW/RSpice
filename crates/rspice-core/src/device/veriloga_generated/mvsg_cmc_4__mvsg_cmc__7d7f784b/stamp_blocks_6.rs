#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1f6a != 0.0) {let t0: f64 = (2.0 * l.ff95);let t1: f64 = (t0 * l.ffd9);let t2: f64 = (t1 * l.ffc3);let t3: f64 = (t2 / l.ffc0);let t4: f64 = (1.0 - l.ff95);let t5: f64 = (t4 * l.f1062);let t6: f64 = (t3 + t5);(l.f1069, l.f106a, l.f106d, l.f106f, l.f106b, l.f106c, l.f106e, ) = (t6, ((((((2.0 * l.ff96) * l.ffd9) * l.ffc3) + (t1 * l.ffc4)) / l.ffc0) + (((-l.ff96) * l.f1062) + (t4 * l.f1063))), ((((((2.0 * l.ff99) * l.ffd9) * l.ffc3) + (t1 * l.ffc7)) / l.ffc0) + (((-l.ff99) * l.f1062) + (t4 * l.f1066))), (((((((2.0 * l.ff9b) * l.ffd9) + (t0 * l.ffda)) * l.ffc3) + (t1 * l.ffc9)) / l.ffc0) + (((-l.ff9b) * l.f1062) + (t4 * l.f1068))), ((((((2.0 * l.ff97) * l.ffd9) * l.ffc3) + (t1 * l.ffc5)) / l.ffc0) + (((-l.ff97) * l.f1062) + (t4 * l.f1064))), ((((((2.0 * l.ff98) * l.ffd9) * l.ffc3) + (t1 * l.ffc6)) / l.ffc0) + (((-l.ff98) * l.f1062) + (t4 * l.f1065))), ((((((2.0 * l.ff9a) * l.ffd9) * l.ffc3) + (t1 * l.ffc8)) / l.ffc0) + (((-l.ff9a) * l.f1062) + (t4 * l.f1067))), );let t7: f64 = (l.f1062 * l.ffc0);let t8: f64 = (t7 / l.ffc3);(l.f101b, l.f1023, l.f1026, l.f1028, l.f1024, l.f1025, l.f1027, ) = (t8, ((((l.f1063 * l.ffc0) * l.ffc3) - (t7 * l.ffc4)) / (l.ffc3 * l.ffc3)), ((((l.f1066 * l.ffc0) * l.ffc3) - (t7 * l.ffc7)) / (l.ffc3 * l.ffc3)), ((((l.f1068 * l.ffc0) * l.ffc3) - (t7 * l.ffc9)) / (l.ffc3 * l.ffc3)), ((((l.f1064 * l.ffc0) * l.ffc3) - (t7 * l.ffc5)) / (l.ffc3 * l.ffc3)), ((((l.f1065 * l.ffc0) * l.ffc3) - (t7 * l.ffc6)) / (l.ffc3 * l.ffc3)), ((((l.f1067 * l.ffc0) * l.ffc3) - (t7 * l.ffc8)) / (l.ffc3 * l.ffc3)), );let t9: f64 = (2.0 * l.ffeb);let ta: f64 = (t9 / l.ff60);let tb: f64 = (ta / l.f101b);let tc: f64 = (1.0 + tb);let td: f64 = (tc).sqrt();let te: f64 = (l.f101b * td);let tf: f64 = (te - l.f101b);(l.f101c, l.f101d, l.f1020, l.f1022, l.f101e, l.f101f, l.f1021, ) = (tf, (((l.f1023 * td) + (l.f101b * ((((((2.0 * l.ffec) / l.ff60) * l.f101b) - (ta * l.f1023)) / (l.f101b * l.f101b)) / (2.0 * td)))) - l.f1023), (((l.f1026 * td) + (l.f101b * ((((((2.0 * l.ffef) / l.ff60) * l.f101b) - (ta * l.f1026)) / (l.f101b * l.f101b)) / (2.0 * td)))) - l.f1026), (((l.f1028 * td) + (l.f101b * ((((((2.0 * l.fff1) / l.ff60) * l.f101b) - (ta * l.f1028)) / (l.f101b * l.f101b)) / (2.0 * td)))) - l.f1028), (((l.f1024 * td) + (l.f101b * ((((((2.0 * l.ffed) / l.ff60) * l.f101b) - (ta * l.f1024)) / (l.f101b * l.f101b)) / (2.0 * td)))) - l.f1024), (((l.f1025 * td) + (l.f101b * ((((((2.0 * l.ffee) / l.ff60) * l.f101b) - (ta * l.f1025)) / (l.f101b * l.f101b)) / (2.0 * td)))) - l.f1025), (((l.f1027 * td) + (l.f101b * ((((((2.0 * l.fff0) / l.ff60) * l.f101b) - (ta * l.f1027)) / (l.f101b * l.f101b)) / (2.0 * td)))) - l.f1027), );}
        if (l.f1f6a != 0.0) {let t10: f64 = (1.0 - l.ff95);let t11: f64 = (l.f101b * t10);let t12: f64 = (l.f1004 * l.ff95);let t13: f64 = (t11 + t12);(l.f100d, l.f1015, l.f1018, l.f101a, l.f1016, l.f1017, l.f1019, ) = (t13, (((l.f1023 * t10) + (l.f101b * (-l.ff96))) + (l.f1004 * l.ff96)), (((l.f1026 * t10) + (l.f101b * (-l.ff99))) + (l.f1004 * l.ff99)), (((l.f1028 * t10) + (l.f101b * (-l.ff9b))) + ((l.f1009 * l.ff95) + (l.f1004 * l.ff9b))), (((l.f1024 * t10) + (l.f101b * (-l.ff97))) + ((l.f1007 * l.ff95) + (l.f1004 * l.ff97))), (((l.f1025 * t10) + (l.f101b * (-l.ff98))) + ((l.f1008 * l.ff95) + (l.f1004 * l.ff98))), (((l.f1027 * t10) + (l.f101b * (-l.ff9a))) + (l.f1004 * l.ff9a)), );let t14: f64 = (1.0 - l.ff95);let t15: f64 = (l.f101c * t14);let t16: f64 = (l.f1004 * l.ff95);let t17: f64 = (t15 + t16);(l.f100e, l.f100f, l.f1012, l.f1014, l.f1010, l.f1011, l.f1013, ) = (t17, (((l.f101d * t14) + (l.f101c * (-l.ff96))) + (l.f1004 * l.ff96)), (((l.f1020 * t14) + (l.f101c * (-l.ff99))) + (l.f1004 * l.ff99)), (((l.f1022 * t14) + (l.f101c * (-l.ff9b))) + ((l.f1009 * l.ff95) + (l.f1004 * l.ff9b))), (((l.f101e * t14) + (l.f101c * (-l.ff97))) + ((l.f1007 * l.ff95) + (l.f1004 * l.ff97))), (((l.f101f * t14) + (l.f101c * (-l.ff98))) + ((l.f1008 * l.ff95) + (l.f1004 * l.ff98))), (((l.f1021 * t14) + (l.f101c * (-l.ff9a))) + (l.f1004 * l.ff9a)), );}
        if (l.f1f6a != 0.0) {
            let (t36, t37, t3a, t3c, t38, t39, t3b,) = {
    if (p.p52 != 0.0) {
        let t18: f64 = (l.f1030 / l.f100e);let t19: f64 = t18;let t1a: f64 = (l.f1030 / l.f100e);let t1b: f64 = (-t1a);let t1c: f64 = (0.001 / p.p53);let t1d: f64 = (l.f1030 / l.f100e);let t1e: f64 = (-t1d);let t1f: f64 = (t1c * t1e);let t20: f64 = (t1f).tanh();let t21: f64 = (t1b * t20);let t22: f64 = (t19 + t21);let t23: f64 = (0.5 * t22);
        (t23, (0.5 * ((-((l.f1030 * l.f100f) / (l.f100e * l.f100e))) + (((-(-((l.f1030 * l.f100f) / (l.f100e * l.f100e)))) * t20) + (t1b * ((t1c * (-(-((l.f1030 * l.f100f) / (l.f100e * l.f100e))))) / ((t1f).cosh() * (t1f).cosh())))))), (0.5 * ((-((l.f1030 * l.f1012) / (l.f100e * l.f100e))) + (((-(-((l.f1030 * l.f1012) / (l.f100e * l.f100e)))) * t20) + (t1b * ((t1c * (-(-((l.f1030 * l.f1012) / (l.f100e * l.f100e))))) / ((t1f).cosh() * (t1f).cosh())))))), (0.5 * ((-((l.f1030 * l.f1014) / (l.f100e * l.f100e))) + (((-(-((l.f1030 * l.f1014) / (l.f100e * l.f100e)))) * t20) + (t1b * ((t1c * (-(-((l.f1030 * l.f1014) / (l.f100e * l.f100e))))) / ((t1f).cosh() * (t1f).cosh())))))), (0.5 * ((((l.f1031 * l.f100e) - (l.f1030 * l.f1010)) / (l.f100e * l.f100e)) + (((-(((l.f1031 * l.f100e) - (l.f1030 * l.f1010)) / (l.f100e * l.f100e))) * t20) + (t1b * ((t1c * (-(((l.f1031 * l.f100e) - (l.f1030 * l.f1010)) / (l.f100e * l.f100e)))) / ((t1f).cosh() * (t1f).cosh())))))), (0.5 * ((((l.f1032 * l.f100e) - (l.f1030 * l.f1011)) / (l.f100e * l.f100e)) + (((-(((l.f1032 * l.f100e) - (l.f1030 * l.f1011)) / (l.f100e * l.f100e))) * t20) + (t1b * ((t1c * (-(((l.f1032 * l.f100e) - (l.f1030 * l.f1011)) / (l.f100e * l.f100e)))) / ((t1f).cosh() * (t1f).cosh())))))), (0.5 * ((-((l.f1030 * l.f1013) / (l.f100e * l.f100e))) + (((-(-((l.f1030 * l.f1013) / (l.f100e * l.f100e)))) * t20) + (t1b * ((t1c * (-(-((l.f1030 * l.f1013) / (l.f100e * l.f100e))))) / ((t1f).cosh() * (t1f).cosh())))))),)
    } else {
        let (t2f, t30, t33, t35, t31, t32, t34,) = {
            if (p.p52 == 0.0) {
                let t24: f64 = (l.f1030 / l.f100e);let t25: f64 = t24;let t26: f64 = (l.f1030 / l.f100e);let t27: f64 = (-t26);let t28: f64 = (l.f1030 / l.f100e);let t29: f64 = (-t28);let t2a: f64 = (t27 * t29);let t2b: f64 = (t2a + p.p53);let t2c: f64 = (t2b).sqrt();let t2d: f64 = (t25 + t2c);let t2e: f64 = (0.5 * t2d);
                (t2e, (0.5 * ((-((l.f1030 * l.f100f) / (l.f100e * l.f100e))) + ((((-(-((l.f1030 * l.f100f) / (l.f100e * l.f100e)))) * t29) + (t27 * (-(-((l.f1030 * l.f100f) / (l.f100e * l.f100e)))))) / (2.0 * t2c)))), (0.5 * ((-((l.f1030 * l.f1012) / (l.f100e * l.f100e))) + ((((-(-((l.f1030 * l.f1012) / (l.f100e * l.f100e)))) * t29) + (t27 * (-(-((l.f1030 * l.f1012) / (l.f100e * l.f100e)))))) / (2.0 * t2c)))), (0.5 * ((-((l.f1030 * l.f1014) / (l.f100e * l.f100e))) + ((((-(-((l.f1030 * l.f1014) / (l.f100e * l.f100e)))) * t29) + (t27 * (-(-((l.f1030 * l.f1014) / (l.f100e * l.f100e)))))) / (2.0 * t2c)))), (0.5 * ((((l.f1031 * l.f100e) - (l.f1030 * l.f1010)) / (l.f100e * l.f100e)) + ((((-(((l.f1031 * l.f100e) - (l.f1030 * l.f1010)) / (l.f100e * l.f100e))) * t29) + (t27 * (-(((l.f1031 * l.f100e) - (l.f1030 * l.f1010)) / (l.f100e * l.f100e))))) / (2.0 * t2c)))), (0.5 * ((((l.f1032 * l.f100e) - (l.f1030 * l.f1011)) / (l.f100e * l.f100e)) + ((((-(((l.f1032 * l.f100e) - (l.f1030 * l.f1011)) / (l.f100e * l.f100e))) * t29) + (t27 * (-(((l.f1032 * l.f100e) - (l.f1030 * l.f1011)) / (l.f100e * l.f100e))))) / (2.0 * t2c)))), (0.5 * ((-((l.f1030 * l.f1013) / (l.f100e * l.f100e))) + ((((-(-((l.f1030 * l.f1013) / (l.f100e * l.f100e)))) * t29) + (t27 * (-(-((l.f1030 * l.f1013) / (l.f100e * l.f100e)))))) / (2.0 * t2c)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2f, t30, t33, t35, t31, t32, t34,)
    }
};
            let t3d: f64 = (t36).powf(l.ff5f);let t3e: f64 = (1.0 + t3d);let t3f: f64 = (1.0 / l.ff5f);let t40: f64 = (t3e).powf(t3f);let t41: f64 = (1.0 / t40);
            (l.ffb1, l.ffb2, l.ffb5, l.ffb7, l.ffb3, l.ffb4, l.ffb6, ) = (t41, (-(if 0.0 == 0.0 && ((t3f) as f64).is_finite() && ((t3f) as f64).fract() == 0.0 { if t3f == 0.0 { 0.0 } else { (t3f * ((t3e).powf(t3f - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t37)) } } else { (t3d * (l.ff5f * (t37 / t36))) })) } } else { (t40 * (t3f * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t37)) } } else { (t3d * (l.ff5f * (t37 / t36))) } / t3e))) } / (t40 * t40))), (-(if 0.0 == 0.0 && ((t3f) as f64).is_finite() && ((t3f) as f64).fract() == 0.0 { if t3f == 0.0 { 0.0 } else { (t3f * ((t3e).powf(t3f - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t3a)) } } else { (t3d * (l.ff5f * (t3a / t36))) })) } } else { (t40 * (t3f * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t3a)) } } else { (t3d * (l.ff5f * (t3a / t36))) } / t3e))) } / (t40 * t40))), (-(if 0.0 == 0.0 && ((t3f) as f64).is_finite() && ((t3f) as f64).fract() == 0.0 { if t3f == 0.0 { 0.0 } else { (t3f * ((t3e).powf(t3f - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t3c)) } } else { (t3d * (l.ff5f * (t3c / t36))) })) } } else { (t40 * (t3f * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t3c)) } } else { (t3d * (l.ff5f * (t3c / t36))) } / t3e))) } / (t40 * t40))), (-(if 0.0 == 0.0 && ((t3f) as f64).is_finite() && ((t3f) as f64).fract() == 0.0 { if t3f == 0.0 { 0.0 } else { (t3f * ((t3e).powf(t3f - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t38)) } } else { (t3d * (l.ff5f * (t38 / t36))) })) } } else { (t40 * (t3f * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t38)) } } else { (t3d * (l.ff5f * (t38 / t36))) } / t3e))) } / (t40 * t40))), (-(if 0.0 == 0.0 && ((t3f) as f64).is_finite() && ((t3f) as f64).fract() == 0.0 { if t3f == 0.0 { 0.0 } else { (t3f * ((t3e).powf(t3f - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t39)) } } else { (t3d * (l.ff5f * (t39 / t36))) })) } } else { (t40 * (t3f * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t39)) } } else { (t3d * (l.ff5f * (t39 / t36))) } / t3e))) } / (t40 * t40))), (-(if 0.0 == 0.0 && ((t3f) as f64).is_finite() && ((t3f) as f64).fract() == 0.0 { if t3f == 0.0 { 0.0 } else { (t3f * ((t3e).powf(t3f - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t3b)) } } else { (t3d * (l.ff5f * (t3b / t36))) })) } } else { (t40 * (t3f * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t36).powf(l.ff5f - 1.0) * t3b)) } } else { (t3d * (l.ff5f * (t3b / t36))) } / t3e))) } / (t40 * t40))), );
        }
        if (l.f1f6a != 0.0) {let t42: f64 = (l.f1030 * l.ffb1);(l.f1033, l.f1034, l.f1037, l.f1039, l.f1035, l.f1036, l.f1038, ) = (t42, (l.f1030 * l.ffb2), (l.f1030 * l.ffb5), (l.f1030 * l.ffb7), ((l.f1031 * l.ffb1) + (l.f1030 * l.ffb3)), ((l.f1032 * l.ffb1) + (l.f1030 * l.ffb4)), (l.f1030 * l.ffb6), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1f6a != 0.0) {
            let (t67, t68, t6b, t6d, t69, t6a, t6c,) = {
    if (p.p52 != 0.0) {
        let t43: f64 = (-l.f1030);let t44: f64 = (t43 / l.f100e);let t45: f64 = t44;let t46: f64 = (-l.f1030);let t47: f64 = (t46 / l.f100e);let t48: f64 = (-t47);let t49: f64 = (0.001 / p.p53);let t4a: f64 = (-l.f1030);let t4b: f64 = (t4a / l.f100e);let t4c: f64 = (-t4b);let t4d: f64 = (t49 * t4c);let t4e: f64 = (t4d).tanh();let t4f: f64 = (t48 * t4e);let t50: f64 = (t45 + t4f);let t51: f64 = (0.5 * t50);
        (t51, (0.5 * ((-((t43 * l.f100f) / (l.f100e * l.f100e))) + (((-(-((t46 * l.f100f) / (l.f100e * l.f100e)))) * t4e) + (t48 * ((t49 * (-(-((t4a * l.f100f) / (l.f100e * l.f100e))))) / ((t4d).cosh() * (t4d).cosh())))))), (0.5 * ((-((t43 * l.f1012) / (l.f100e * l.f100e))) + (((-(-((t46 * l.f1012) / (l.f100e * l.f100e)))) * t4e) + (t48 * ((t49 * (-(-((t4a * l.f1012) / (l.f100e * l.f100e))))) / ((t4d).cosh() * (t4d).cosh())))))), (0.5 * ((-((t43 * l.f1014) / (l.f100e * l.f100e))) + (((-(-((t46 * l.f1014) / (l.f100e * l.f100e)))) * t4e) + (t48 * ((t49 * (-(-((t4a * l.f1014) / (l.f100e * l.f100e))))) / ((t4d).cosh() * (t4d).cosh())))))), (0.5 * (((((-l.f1031) * l.f100e) - (t43 * l.f1010)) / (l.f100e * l.f100e)) + (((-((((-l.f1031) * l.f100e) - (t46 * l.f1010)) / (l.f100e * l.f100e))) * t4e) + (t48 * ((t49 * (-((((-l.f1031) * l.f100e) - (t4a * l.f1010)) / (l.f100e * l.f100e)))) / ((t4d).cosh() * (t4d).cosh())))))), (0.5 * (((((-l.f1032) * l.f100e) - (t43 * l.f1011)) / (l.f100e * l.f100e)) + (((-((((-l.f1032) * l.f100e) - (t46 * l.f1011)) / (l.f100e * l.f100e))) * t4e) + (t48 * ((t49 * (-((((-l.f1032) * l.f100e) - (t4a * l.f1011)) / (l.f100e * l.f100e)))) / ((t4d).cosh() * (t4d).cosh())))))), (0.5 * ((-((t43 * l.f1013) / (l.f100e * l.f100e))) + (((-(-((t46 * l.f1013) / (l.f100e * l.f100e)))) * t4e) + (t48 * ((t49 * (-(-((t4a * l.f1013) / (l.f100e * l.f100e))))) / ((t4d).cosh() * (t4d).cosh())))))),)
    } else {
        let (t60, t61, t64, t66, t62, t63, t65,) = {
            if (p.p52 == 0.0) {
                let t52: f64 = (-l.f1030);let t53: f64 = (t52 / l.f100e);let t54: f64 = t53;let t55: f64 = (-l.f1030);let t56: f64 = (t55 / l.f100e);let t57: f64 = (-t56);let t58: f64 = (-l.f1030);let t59: f64 = (t58 / l.f100e);let t5a: f64 = (-t59);let t5b: f64 = (t57 * t5a);let t5c: f64 = (t5b + p.p53);let t5d: f64 = (t5c).sqrt();let t5e: f64 = (t54 + t5d);let t5f: f64 = (0.5 * t5e);
                (t5f, (0.5 * ((-((t52 * l.f100f) / (l.f100e * l.f100e))) + ((((-(-((t55 * l.f100f) / (l.f100e * l.f100e)))) * t5a) + (t57 * (-(-((t58 * l.f100f) / (l.f100e * l.f100e)))))) / (2.0 * t5d)))), (0.5 * ((-((t52 * l.f1012) / (l.f100e * l.f100e))) + ((((-(-((t55 * l.f1012) / (l.f100e * l.f100e)))) * t5a) + (t57 * (-(-((t58 * l.f1012) / (l.f100e * l.f100e)))))) / (2.0 * t5d)))), (0.5 * ((-((t52 * l.f1014) / (l.f100e * l.f100e))) + ((((-(-((t55 * l.f1014) / (l.f100e * l.f100e)))) * t5a) + (t57 * (-(-((t58 * l.f1014) / (l.f100e * l.f100e)))))) / (2.0 * t5d)))), (0.5 * (((((-l.f1031) * l.f100e) - (t52 * l.f1010)) / (l.f100e * l.f100e)) + ((((-((((-l.f1031) * l.f100e) - (t55 * l.f1010)) / (l.f100e * l.f100e))) * t5a) + (t57 * (-((((-l.f1031) * l.f100e) - (t58 * l.f1010)) / (l.f100e * l.f100e))))) / (2.0 * t5d)))), (0.5 * (((((-l.f1032) * l.f100e) - (t52 * l.f1011)) / (l.f100e * l.f100e)) + ((((-((((-l.f1032) * l.f100e) - (t55 * l.f1011)) / (l.f100e * l.f100e))) * t5a) + (t57 * (-((((-l.f1032) * l.f100e) - (t58 * l.f1011)) / (l.f100e * l.f100e))))) / (2.0 * t5d)))), (0.5 * ((-((t52 * l.f1013) / (l.f100e * l.f100e))) + ((((-(-((t55 * l.f1013) / (l.f100e * l.f100e)))) * t5a) + (t57 * (-(-((t58 * l.f1013) / (l.f100e * l.f100e)))))) / (2.0 * t5d)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t60, t61, t64, t66, t62, t63, t65,)
    }
};
            let t6e: f64 = (t67).powf(l.ff5f);let t6f: f64 = (1.0 + t6e);let t70: f64 = (1.0 / l.ff5f);let t71: f64 = (t6f).powf(t70);let t72: f64 = (1.0 / t71);
            (l.ff8e, l.ff8f, l.ff92, l.ff94, l.ff90, l.ff91, l.ff93, ) = (t72, (-(if 0.0 == 0.0 && ((t70) as f64).is_finite() && ((t70) as f64).fract() == 0.0 { if t70 == 0.0 { 0.0 } else { (t70 * ((t6f).powf(t70 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t68)) } } else { (t6e * (l.ff5f * (t68 / t67))) })) } } else { (t71 * (t70 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t68)) } } else { (t6e * (l.ff5f * (t68 / t67))) } / t6f))) } / (t71 * t71))), (-(if 0.0 == 0.0 && ((t70) as f64).is_finite() && ((t70) as f64).fract() == 0.0 { if t70 == 0.0 { 0.0 } else { (t70 * ((t6f).powf(t70 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t6b)) } } else { (t6e * (l.ff5f * (t6b / t67))) })) } } else { (t71 * (t70 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t6b)) } } else { (t6e * (l.ff5f * (t6b / t67))) } / t6f))) } / (t71 * t71))), (-(if 0.0 == 0.0 && ((t70) as f64).is_finite() && ((t70) as f64).fract() == 0.0 { if t70 == 0.0 { 0.0 } else { (t70 * ((t6f).powf(t70 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t6d)) } } else { (t6e * (l.ff5f * (t6d / t67))) })) } } else { (t71 * (t70 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t6d)) } } else { (t6e * (l.ff5f * (t6d / t67))) } / t6f))) } / (t71 * t71))), (-(if 0.0 == 0.0 && ((t70) as f64).is_finite() && ((t70) as f64).fract() == 0.0 { if t70 == 0.0 { 0.0 } else { (t70 * ((t6f).powf(t70 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t69)) } } else { (t6e * (l.ff5f * (t69 / t67))) })) } } else { (t71 * (t70 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t69)) } } else { (t6e * (l.ff5f * (t69 / t67))) } / t6f))) } / (t71 * t71))), (-(if 0.0 == 0.0 && ((t70) as f64).is_finite() && ((t70) as f64).fract() == 0.0 { if t70 == 0.0 { 0.0 } else { (t70 * ((t6f).powf(t70 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t6a)) } } else { (t6e * (l.ff5f * (t6a / t67))) })) } } else { (t71 * (t70 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t6a)) } } else { (t6e * (l.ff5f * (t6a / t67))) } / t6f))) } / (t71 * t71))), (-(if 0.0 == 0.0 && ((t70) as f64).is_finite() && ((t70) as f64).fract() == 0.0 { if t70 == 0.0 { 0.0 } else { (t70 * ((t6f).powf(t70 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t6c)) } } else { (t6e * (l.ff5f * (t6c / t67))) })) } } else { (t71 * (t70 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((t67).powf(l.ff5f - 1.0) * t6c)) } } else { (t6e * (l.ff5f * (t6c / t67))) } / t6f))) } / (t71 * t71))), );
        }
        if (l.f1f6a != 0.0) {let t73: f64 = (-l.f1030);let t74: f64 = (t73 * l.ff8e);(l.f1052, l.f1053, l.f1056, l.f1058, l.f1054, l.f1055, l.f1057, ) = (t74, (t73 * l.ff8f), (t73 * l.ff92), (t73 * l.ff94), (((-l.f1031) * l.ff8e) + (t73 * l.ff90)), (((-l.f1032) * l.ff8e) + (t73 * l.ff91)), (t73 * l.ff93), );let t75: f64 = (l.f1049 - l.ffca);let t76: f64 = (t75 / l.ff5d);(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (t76, ((l.f104a - l.ffcb) / l.ff5d), ((l.f104c - l.ffce) / l.ff5d), ((((l.f104e - l.ffd0) * l.ff5d) - (t75 * l.ff5e)) / (l.ff5d * l.ff5d)), ((l.f104b - l.ffcc) / l.ff5d), ((-l.ffcd) / l.ff5d), ((l.f104d - l.ffcf) / l.ff5d), );}
        let t77: f64 = if l.ff87 > 50.0 { 1.0 } else { 0.0 };l.f1f72 = t77;
        if ((l.f1f6a != 0.0) && (l.f1f72 != 0.0)) {(l.ffa3, l.ffa4, l.ffa7, l.ffa9, l.ffa5, l.ffa6, l.ffa8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t78: f64 = (-50.0);let t79: f64 = if l.ff87 < t78 { 1.0 } else { 0.0 };l.f1f73 = t79;
        if (((l.f1f6a != 0.0) && (l.f1f72 == 0.0)) && (l.f1f73 != 0.0)) {(l.ffa3, l.ffa4, l.ffa7, l.ffa9, l.ffa5, l.ffa6, l.ffa8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f1f6a != 0.0) && (l.f1f72 == 0.0)) && (l.f1f73 == 0.0)) {let t7a: f64 = (l.ff87).exp();let t7b: f64 = (1.0 + t7a);let t7c: f64 = (1.0 / t7b);(l.ffa3, l.ffa4, l.ffa7, l.ffa9, l.ffa5, l.ffa6, l.ffa8, ) = (t7c, (-((t7a * l.ff88) / (t7b * t7b))), (-((t7a * l.ff8b) / (t7b * t7b))), (-((t7a * l.ff8d) / (t7b * t7b))), (-((t7a * l.ff89) / (t7b * t7b))), (-((t7a * l.ff8a) / (t7b * t7b))), (-((t7a * l.ff8c) / (t7b * t7b))), );}
        if (l.f1f6a != 0.0) {let t7d: f64 = (l.f1042 - l.f1052);let t7e: f64 = (p.p51 * 0.1);let t7f: f64 = (t7e * l.ff5d);let t80: f64 = (t7f * l.ffa3);let t81: f64 = (l.f1059 - t80);let t82: f64 = (t7d - t81);let t83: f64 = (t82 / l.f1004);(l.ff80, l.ff81, l.ff84, l.ff86, l.ff82, l.ff83, l.ff85, ) = (t83, (((l.f1043 - l.f1053) - (-(t7f * l.ffa4))) / l.f1004), (((l.f1046 - l.f1056) - (-(t7f * l.ffa7))) / l.f1004), (((((l.f1048 - l.f1058) - (l.f105c - (((t7e * l.ff5e) * l.ffa3) + (t7f * l.ffa9)))) * l.f1004) - (t82 * l.f1009)) / (l.f1004 * l.f1004)), (((((l.f1044 - l.f1054) - (l.f105a - (t7f * l.ffa5))) * l.f1004) - (t82 * l.f1007)) / (l.f1004 * l.f1004)), (((((l.f1045 - l.f1055) - (l.f105b - (t7f * l.ffa6))) * l.f1004) - (t82 * l.f1008)) / (l.f1004 * l.f1004)), (((l.f1047 - l.f1057) - (-(t7f * l.ffa8))) / l.f1004), );}
        let t84: f64 = if l.ff80 > 50.0 { 1.0 } else { 0.0 };l.f1f74 = t84;
        if ((l.f1f6a != 0.0) && (l.f1f74 != 0.0)) {let t85: f64 = (l.fff2 * l.ff80);(l.ffe4, l.ffe5, l.ffe8, l.ffea, l.ffe6, l.ffe7, l.ffe9, ) = (t85, (l.fff2 * l.ff81), (l.fff2 * l.ff84), ((l.fff5 * l.ff80) + (l.fff2 * l.ff86)), ((l.fff3 * l.ff80) + (l.fff2 * l.ff82)), ((l.fff4 * l.ff80) + (l.fff2 * l.ff83)), (l.fff2 * l.ff85), );}
        let t86: f64 = (-50.0);let t87: f64 = if l.ff80 < t86 { 1.0 } else { 0.0 };l.f1f75 = t87;
        if (((l.f1f6a != 0.0) && (l.f1f74 == 0.0)) && (l.f1f75 != 0.0)) {let t88: f64 = (l.ff80).exp();let t89: f64 = (l.fff2 * t88);(l.ffe4, l.ffe5, l.ffe8, l.ffea, l.ffe6, l.ffe7, l.ffe9, ) = (t89, (l.fff2 * (t88 * l.ff81)), (l.fff2 * (t88 * l.ff84)), ((l.fff5 * t88) + (l.fff2 * (t88 * l.ff86))), ((l.fff3 * t88) + (l.fff2 * (t88 * l.ff82))), ((l.fff4 * t88) + (l.fff2 * (t88 * l.ff83))), (l.fff2 * (t88 * l.ff85)), );}
        if (((l.f1f6a != 0.0) && (l.f1f74 == 0.0)) && (l.f1f75 == 0.0)) {let t8a: f64 = (l.ff80).exp();let t8b: f64 = (1.0 + t8a);let t8c: f64 = (t8b).ln();let t8d: f64 = (l.fff2 * t8c);(l.ffe4, l.ffe5, l.ffe8, l.ffea, l.ffe6, l.ffe7, l.ffe9, ) = (t8d, (l.fff2 * ((t8a * l.ff81) / t8b)), (l.fff2 * ((t8a * l.ff84) / t8b)), ((l.fff5 * t8c) + (l.fff2 * ((t8a * l.ff86) / t8b))), ((l.fff3 * t8c) + (l.fff2 * ((t8a * l.ff82) / t8b))), ((l.fff4 * t8c) + (l.fff2 * ((t8a * l.ff83) / t8b))), (l.fff2 * ((t8a * l.ff85) / t8b)), );}
        if (l.f1f6a != 0.0) {let t8e: f64 = (l.f1042 - l.ffca);let t8f: f64 = (t8e / l.ff5d);(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (t8f, ((l.f1043 - l.ffcb) / l.ff5d), ((l.f1046 - l.ffce) / l.ff5d), ((((l.f1048 - l.ffd0) * l.ff5d) - (t8e * l.ff5e)) / (l.ff5d * l.ff5d)), ((l.f1044 - l.ffcc) / l.ff5d), ((l.f1045 - l.ffcd) / l.ff5d), ((l.f1047 - l.ffcf) / l.ff5d), );}
        let t90: f64 = if l.ff87 > 50.0 { 1.0 } else { 0.0 };l.f1f76 = t90;
        if ((l.f1f6a != 0.0) && (l.f1f76 != 0.0)) {(l.ff9c, l.ff9d, l.ffa0, l.ffa2, l.ff9e, l.ff9f, l.ffa1, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t91: f64 = (-50.0);let t92: f64 = if l.ff87 < t91 { 1.0 } else { 0.0 };l.f1f77 = t92;
        if (((l.f1f6a != 0.0) && (l.f1f76 == 0.0)) && (l.f1f77 != 0.0)) {(l.ff9c, l.ff9d, l.ffa0, l.ffa2, l.ff9e, l.ff9f, l.ffa1, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f1f6a != 0.0) && (l.f1f76 == 0.0)) && (l.f1f77 == 0.0)) {let t93: f64 = (l.ff87).exp();let t94: f64 = (1.0 + t93);let t95: f64 = (1.0 / t94);(l.ff9c, l.ff9d, l.ffa0, l.ffa2, l.ff9e, l.ff9f, l.ffa1, ) = (t95, (-((t93 * l.ff88) / (t94 * t94))), (-((t93 * l.ff8b) / (t94 * t94))), (-((t93 * l.ff8d) / (t94 * t94))), (-((t93 * l.ff89) / (t94 * t94))), (-((t93 * l.ff8a) / (t94 * t94))), (-((t93 * l.ff8c) / (t94 * t94))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1f6a != 0.0) {let t96: f64 = (l.f1049 - l.f1033);let t97: f64 = (p.p51 * 0.1);let t98: f64 = (t97 * l.ff5d);let t99: f64 = (t98 * l.ff9c);let t9a: f64 = (l.f1059 - t99);let t9b: f64 = (t96 - t9a);let t9c: f64 = (t9b / l.f1004);(l.ff73, l.ff74, l.ff77, l.ff79, l.ff75, l.ff76, l.ff78, ) = (t9c, (((l.f104a - l.f1034) - (-(t98 * l.ff9d))) / l.f1004), (((l.f104c - l.f1037) - (-(t98 * l.ffa0))) / l.f1004), (((((l.f104e - l.f1039) - (l.f105c - (((t97 * l.ff5e) * l.ff9c) + (t98 * l.ffa2)))) * l.f1004) - (t9b * l.f1009)) / (l.f1004 * l.f1004)), (((((l.f104b - l.f1035) - (l.f105a - (t98 * l.ff9e))) * l.f1004) - (t9b * l.f1007)) / (l.f1004 * l.f1004)), (((((-l.f1036) - (l.f105b - (t98 * l.ff9f))) * l.f1004) - (t9b * l.f1008)) / (l.f1004 * l.f1004)), (((l.f104d - l.f1038) - (-(t98 * l.ffa1))) / l.f1004), );}
        let t9d: f64 = if l.ff73 > 50.0 { 1.0 } else { 0.0 };l.f1f7a = t9d;
        if ((l.f1f6a != 0.0) && (l.f1f7a != 0.0)) {let t9e: f64 = (l.fff2 * l.ff73);(l.ffdd, l.ffde, l.ffe1, l.ffe3, l.ffdf, l.ffe0, l.ffe2, ) = (t9e, (l.fff2 * l.ff74), (l.fff2 * l.ff77), ((l.fff5 * l.ff73) + (l.fff2 * l.ff79)), ((l.fff3 * l.ff73) + (l.fff2 * l.ff75)), ((l.fff4 * l.ff73) + (l.fff2 * l.ff76)), (l.fff2 * l.ff78), );}
        let t9f: f64 = (-50.0);let ta0: f64 = if l.ff73 < t9f { 1.0 } else { 0.0 };l.f1f7b = ta0;
        if (((l.f1f6a != 0.0) && (l.f1f7a == 0.0)) && (l.f1f7b != 0.0)) {let ta1: f64 = (l.ff73).exp();let ta2: f64 = (l.fff2 * ta1);(l.ffdd, l.ffde, l.ffe1, l.ffe3, l.ffdf, l.ffe0, l.ffe2, ) = (ta2, (l.fff2 * (ta1 * l.ff74)), (l.fff2 * (ta1 * l.ff77)), ((l.fff5 * ta1) + (l.fff2 * (ta1 * l.ff79))), ((l.fff3 * ta1) + (l.fff2 * (ta1 * l.ff75))), ((l.fff4 * ta1) + (l.fff2 * (ta1 * l.ff76))), (l.fff2 * (ta1 * l.ff78)), );}
        if (((l.f1f6a != 0.0) && (l.f1f7a == 0.0)) && (l.f1f7b == 0.0)) {let ta3: f64 = (l.ff73).exp();let ta4: f64 = (1.0 + ta3);let ta5: f64 = (ta4).ln();let ta6: f64 = (l.fff2 * ta5);(l.ffdd, l.ffde, l.ffe1, l.ffe3, l.ffdf, l.ffe0, l.ffe2, ) = (ta6, (l.fff2 * ((ta3 * l.ff74) / ta4)), (l.fff2 * ((ta3 * l.ff77) / ta4)), ((l.fff5 * ta5) + (l.fff2 * ((ta3 * l.ff79) / ta4))), ((l.fff3 * ta5) + (l.fff2 * ((ta3 * l.ff75) / ta4))), ((l.fff4 * ta5) + (l.fff2 * ((ta3 * l.ff76) / ta4))), (l.fff2 * ((ta3 * l.ff78) / ta4)), );}
        if (l.f1f6a != 0.0) {let ta7: f64 = (l.ffe4 - l.ffdd);let ta8: f64 = (ta7 / l.ff60);(l.f1029, l.f102a, l.f102d, l.f102f, l.f102b, l.f102c, l.f102e, ) = (ta8, ((l.ffe5 - l.ffde) / l.ff60), ((l.ffe8 - l.ffe1) / l.ff60), ((l.ffea - l.ffe3) / l.ff60), ((l.ffe6 - l.ffdf) / l.ff60), ((l.ffe7 - l.ffe0) / l.ff60), ((l.ffe9 - l.ffe2) / l.ff60), );let ta9: f64 = (l.f1029 / l.f100d);(l.ffca, l.ffcb, l.ffce, l.ffd0, l.ffcc, l.ffcd, l.ffcf, ) = (ta9, (((l.f102a * l.f100d) - (l.f1029 * l.f1015)) / (l.f100d * l.f100d)), (((l.f102d * l.f100d) - (l.f1029 * l.f1018)) / (l.f100d * l.f100d)), (((l.f102f * l.f100d) - (l.f1029 * l.f101a)) / (l.f100d * l.f100d)), (((l.f102b * l.f100d) - (l.f1029 * l.f1016)) / (l.f100d * l.f100d)), (((l.f102c * l.f100d) - (l.f1029 * l.f1017)) / (l.f100d * l.f100d)), (((l.f102e * l.f100d) - (l.f1029 * l.f1019)) / (l.f100d * l.f100d)), );}
        if (l.f1f6a != 0.0) {
            let (tb8, tb9, tbc, tbe, tba, tbb, tbd,) = {
    if (p.p52 != 0.0) {
        let taa: f64 = (0.001 / p.p53);let tab: f64 = (taa * l.ffca);let tac: f64 = (tab).tanh();let tad: f64 = (l.ffca * tac);
        (tad, ((l.ffcb * tac) + (l.ffca * ((taa * l.ffcb) / ((tab).cosh() * (tab).cosh())))), ((l.ffce * tac) + (l.ffca * ((taa * l.ffce) / ((tab).cosh() * (tab).cosh())))), ((l.ffd0 * tac) + (l.ffca * ((taa * l.ffd0) / ((tab).cosh() * (tab).cosh())))), ((l.ffcc * tac) + (l.ffca * ((taa * l.ffcc) / ((tab).cosh() * (tab).cosh())))), ((l.ffcd * tac) + (l.ffca * ((taa * l.ffcd) / ((tab).cosh() * (tab).cosh())))), ((l.ffcf * tac) + (l.ffca * ((taa * l.ffcf) / ((tab).cosh() * (tab).cosh())))),)
    } else {
        let (tb1, tb2, tb5, tb7, tb3, tb4, tb6,) = {
            if (p.p52 == 0.0) {
                let tae: f64 = (l.ffca * l.ffca);let taf: f64 = (tae + p.p53);let tb0: f64 = (taf).sqrt();
                (tb0, (((l.ffcb * l.ffca) + (l.ffca * l.ffcb)) / (2.0 * tb0)), (((l.ffce * l.ffca) + (l.ffca * l.ffce)) / (2.0 * tb0)), (((l.ffd0 * l.ffca) + (l.ffca * l.ffd0)) / (2.0 * tb0)), (((l.ffcc * l.ffca) + (l.ffca * l.ffcc)) / (2.0 * tb0)), (((l.ffcd * l.ffca) + (l.ffca * l.ffcd)) / (2.0 * tb0)), (((l.ffcf * l.ffca) + (l.ffca * l.ffcf)) / (2.0 * tb0)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (tb1, tb2, tb5, tb7, tb3, tb4, tb6,)
    }
};
            let tbf: f64 = (tb8).powf(l.ff5f);let tc0: f64 = (1.0 + tbf);let tc1: f64 = (1.0 / l.ff5f);let tc2: f64 = (tc0).powf(tc1);let tc3: f64 = (l.ffca / tc2);
            (l.ffaa, l.ffab, l.ffae, l.ffb0, l.ffac, l.ffad, l.ffaf, ) = (tc3, (((l.ffcb * tc2) - (l.ffca * if 0.0 == 0.0 && ((tc1) as f64).is_finite() && ((tc1) as f64).fract() == 0.0 { if tc1 == 0.0 { 0.0 } else { (tc1 * ((tc0).powf(tc1 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tb9)) } } else { (tbf * (l.ff5f * (tb9 / tb8))) })) } } else { (tc2 * (tc1 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tb9)) } } else { (tbf * (l.ff5f * (tb9 / tb8))) } / tc0))) })) / (tc2 * tc2)), (((l.ffce * tc2) - (l.ffca * if 0.0 == 0.0 && ((tc1) as f64).is_finite() && ((tc1) as f64).fract() == 0.0 { if tc1 == 0.0 { 0.0 } else { (tc1 * ((tc0).powf(tc1 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tbc)) } } else { (tbf * (l.ff5f * (tbc / tb8))) })) } } else { (tc2 * (tc1 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tbc)) } } else { (tbf * (l.ff5f * (tbc / tb8))) } / tc0))) })) / (tc2 * tc2)), (((l.ffd0 * tc2) - (l.ffca * if 0.0 == 0.0 && ((tc1) as f64).is_finite() && ((tc1) as f64).fract() == 0.0 { if tc1 == 0.0 { 0.0 } else { (tc1 * ((tc0).powf(tc1 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tbe)) } } else { (tbf * (l.ff5f * (tbe / tb8))) })) } } else { (tc2 * (tc1 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tbe)) } } else { (tbf * (l.ff5f * (tbe / tb8))) } / tc0))) })) / (tc2 * tc2)), (((l.ffcc * tc2) - (l.ffca * if 0.0 == 0.0 && ((tc1) as f64).is_finite() && ((tc1) as f64).fract() == 0.0 { if tc1 == 0.0 { 0.0 } else { (tc1 * ((tc0).powf(tc1 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tba)) } } else { (tbf * (l.ff5f * (tba / tb8))) })) } } else { (tc2 * (tc1 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tba)) } } else { (tbf * (l.ff5f * (tba / tb8))) } / tc0))) })) / (tc2 * tc2)), (((l.ffcd * tc2) - (l.ffca * if 0.0 == 0.0 && ((tc1) as f64).is_finite() && ((tc1) as f64).fract() == 0.0 { if tc1 == 0.0 { 0.0 } else { (tc1 * ((tc0).powf(tc1 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tbb)) } } else { (tbf * (l.ff5f * (tbb / tb8))) })) } } else { (tc2 * (tc1 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tbb)) } } else { (tbf * (l.ff5f * (tbb / tb8))) } / tc0))) })) / (tc2 * tc2)), (((l.ffcf * tc2) - (l.ffca * if 0.0 == 0.0 && ((tc1) as f64).is_finite() && ((tc1) as f64).fract() == 0.0 { if tc1 == 0.0 { 0.0 } else { (tc1 * ((tc0).powf(tc1 - 1.0) * if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tbd)) } } else { (tbf * (l.ff5f * (tbd / tb8))) })) } } else { (tc2 * (tc1 * (if 0.0 == 0.0 && ((l.ff5f) as f64).is_finite() && ((l.ff5f) as f64).fract() == 0.0 { if l.ff5f == 0.0 { 0.0 } else { (l.ff5f * ((tb8).powf(l.ff5f - 1.0) * tbd)) } } else { (tbf * (l.ff5f * (tbd / tb8))) } / tc0))) })) / (tc2 * tc2)), );
        }
        if (l.f1f6a != 0.0) {let tc4: f64 = (l.f1069 * l.ffaa);(l.f103a, l.f103c, l.f103f, l.f1041, l.f103d, l.f103e, l.f1040, ) = (tc4, ((l.f106a * l.ffaa) + (l.f1069 * l.ffab)), ((l.f106d * l.ffaa) + (l.f1069 * l.ffae)), ((l.f106f * l.ffaa) + (l.f1069 * l.ffb0)), ((l.f106b * l.ffaa) + (l.f1069 * l.ffac)), ((l.f106c * l.ffaa) + (l.f1069 * l.ffad)), ((l.f106e * l.ffaa) + (l.f1069 * l.ffaf)), );let tc5: f64 = (l.f100a * l.f1071);let tc6: f64 = (tc5 * l.ffd8);let tc7: f64 = (tc6 * 0.5);let tc8: f64 = (l.ffe4 + l.ffdd);let tc9: f64 = (tc7 * tc8);let tca: f64 = (tc9 * l.f103a);let tcb: f64 = (tca * l.f1003);(l.ffb8, l.ffb9, l.ffbc, l.ffbe, l.ffba, l.ffbb, l.ffbd, ) = (tcb, ((((tc7 * (l.ffe5 + l.ffde)) * l.f103a) + (tc9 * l.f103c)) * l.f1003), ((((tc7 * (l.ffe8 + l.ffe1)) * l.f103a) + (tc9 * l.f103f)) * l.f1003), ((((tc7 * (l.ffea + l.ffe3)) * l.f103a) + (tc9 * l.f1041)) * l.f1003), ((((tc7 * (l.ffe6 + l.ffdf)) * l.f103a) + (tc9 * l.f103d)) * l.f1003), ((((tc7 * (l.ffe7 + l.ffe0)) * l.f103a) + (tc9 * l.f103e)) * l.f1003), ((((tc7 * (l.ffe9 + l.ffe2)) * l.f103a) + (tc9 * l.f1040)) * l.f1003), );let tcc: f64 = (2.302585092994046 * l.ffd9);let tcd: f64 = (l.fffd / tcc);(l.ffd2, l.ffd3, ) = (tcd, (-((l.fffd * (2.302585092994046 * l.ffda)) / (tcc * tcc))), );let tce: f64 = (2.0 * l.ffd2);let tcf: f64 = (tce * l.ffd9);(l.f1005, l.f1006, ) = (tcf, (((2.0 * l.ffd3) * l.ffd9) + (tce * l.ffda)), );}
        let td0: f64 = if l.ffdb == 1.0 { 1.0 } else { 0.0 };l.f1f7e = td0;
        if ((l.f1f6a != 0.0) && (l.f1f7e != 0.0)) {let td1: f64 = (p.p51 * 0.5);let td2: f64 = (td1 * l.ff5d);let td3: f64 = (l.f105f - td2);let td4: f64 = (l.f100c - td3);let td5: f64 = (td4 / l.f1005);(l.ff71, l.ff72, ) = (td5, ((((-(l.f1060 - (td1 * l.ff5e))) * l.f1005) - (td4 * l.f1006)) / (l.f1005 * l.f1005)), );}
        let td6: f64 = if l.ff71 > 50.0 { 1.0 } else { 0.0 };l.f1f7f = td6;
        if (((l.f1f6a != 0.0) && (l.f1f7e != 0.0)) && (l.f1f7f != 0.0)) {(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (l.ff71, 0.0, 0.0, l.ff72, 0.0, 0.0, 0.0, );}
        let td7: f64 = (-50.0);let td8: f64 = if l.ff71 < td7 { 1.0 } else { 0.0 };l.f1f80 = td8;
        if ((((l.f1f6a != 0.0) && (l.f1f7e != 0.0)) && (l.f1f7f == 0.0)) && (l.f1f80 != 0.0)) {let td9: f64 = (l.ff71).exp();(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (td9, 0.0, 0.0, (td9 * l.ff72), 0.0, 0.0, 0.0, );}
        if ((((l.f1f6a != 0.0) && (l.f1f7e != 0.0)) && (l.f1f7f == 0.0)) && (l.f1f80 == 0.0)) {let tda: f64 = (l.ff71).exp();let tdb: f64 = (1.0 + tda);let tdc: f64 = (tdb).ln();(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (tdc, 0.0, 0.0, ((tda * l.ff72) / tdb), 0.0, 0.0, 0.0, );}
        if ((l.f1f6a != 0.0) && (l.f1f7e != 0.0)) {let tdd: f64 = (p.p51 * 0.5);let tde: f64 = (tdd * l.ff5d);let tdf: f64 = (l.f105f - tde);let te0: f64 = (l.f100b - tdf);let te1: f64 = (te0 / l.f1005);(l.ff6f, l.ff70, ) = (te1, ((((-(l.f1060 - (tdd * l.ff5e))) * l.f1005) - (te0 * l.f1006)) / (l.f1005 * l.f1005)), );}
        let te2: f64 = if l.ff6f > 50.0 { 1.0 } else { 0.0 };l.f1f81 = te2;
        if (((l.f1f6a != 0.0) && (l.f1f7e != 0.0)) && (l.f1f81 != 0.0)) {(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (l.ff6f, 0.0, 0.0, l.ff70, 0.0, 0.0, 0.0, );}
        let te3: f64 = (-50.0);let te4: f64 = if l.ff6f < te3 { 1.0 } else { 0.0 };l.f1f82 = te4;
        if ((((l.f1f6a != 0.0) && (l.f1f7e != 0.0)) && (l.f1f81 == 0.0)) && (l.f1f82 != 0.0)) {let te5: f64 = (l.ff6f).exp();(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (te5, 0.0, 0.0, (te5 * l.ff70), 0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f1f6a != 0.0) && (l.f1f7e != 0.0)) && (l.f1f81 == 0.0)) && (l.f1f82 == 0.0)) {let te6: f64 = (l.ff6f).exp();let te7: f64 = (1.0 + te6);let te8: f64 = (te7).ln();(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (te8, 0.0, 0.0, ((te6 * l.ff70) / te7), 0.0, 0.0, 0.0, );}
        let te9: f64 = if l.ffdc == 1.0 { 1.0 } else { 0.0 };l.f1f83 = te9;
        if ((l.f1f6a != 0.0) && (l.f1f83 != 0.0)) {let tea: f64 = (p.p51 * 0.5);let teb: f64 = (tea * l.ff5d);let tec: f64 = (l.f105f - teb);let ted: f64 = (l.f1049 - tec);let tee: f64 = (ted / l.f1005);(l.ff7a, l.ff7b, l.ff7d, l.ff7f, l.ff7c, l.ff7e, ) = (tee, (l.f104a / l.f1005), (l.f104c / l.f1005), ((((l.f104e - (l.f1060 - (tea * l.ff5e))) * l.f1005) - (ted * l.f1006)) / (l.f1005 * l.f1005)), (l.f104b / l.f1005), (l.f104d / l.f1005), );}
        let tef: f64 = if l.ff7a > 50.0 { 1.0 } else { 0.0 };l.f1f86 = tef;
        if (((l.f1f6a != 0.0) && (l.f1f83 != 0.0)) && (l.f1f86 != 0.0)) {(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (l.ff7a, l.ff7b, l.ff7d, l.ff7f, l.ff7c, 0.0, l.ff7e, );}
        let tf0: f64 = (-50.0);let tf1: f64 = if l.ff7a < tf0 { 1.0 } else { 0.0 };l.f1f87 = tf1;
        if ((((l.f1f6a != 0.0) && (l.f1f83 != 0.0)) && (l.f1f86 == 0.0)) && (l.f1f87 != 0.0)) {let tf2: f64 = (l.ff7a).exp();(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (tf2, (tf2 * l.ff7b), (tf2 * l.ff7d), (tf2 * l.ff7f), (tf2 * l.ff7c), 0.0, (tf2 * l.ff7e), );}
        if ((((l.f1f6a != 0.0) && (l.f1f83 != 0.0)) && (l.f1f86 == 0.0)) && (l.f1f87 == 0.0)) {let tf3: f64 = (l.ff7a).exp();let tf4: f64 = (1.0 + tf3);let tf5: f64 = (tf4).ln();(l.ff87, l.ff88, l.ff8b, l.ff8d, l.ff89, l.ff8a, l.ff8c, ) = (tf5, ((tf3 * l.ff7b) / tf4), ((tf3 * l.ff7d) / tf4), ((tf3 * l.ff7f) / tf4), ((tf3 * l.ff7c) / tf4), 0.0, ((tf3 * l.ff7e) / tf4), );}
        if (l.f1f6a != 0.0) {(l.fff6, l.fff7, l.fffa, l.fffc, l.fff8, l.fff9, l.fffb, ) = (l.ffb8, l.ffb9, l.ffbc, l.ffbe, l.ffba, l.ffbb, l.ffbd, );(l.f2106, l.f2107, l.f210a, l.f210c, l.f2108, l.f2109, l.f210b, ) = (l.ffb8, l.ffb9, l.ffbc, l.ffbe, l.ffba, l.ffbb, l.ffbd, );(l.f2106, l.f2107, l.f210a, l.f210c, l.f2108, l.f2109, l.f210b, ) = (l.fff6, l.fff7, l.fffa, l.fffc, l.fff8, l.fff9, l.fffb, );}
        (l.f11b6, l.f11bb, l.f11bc, l.f11bd, l.f11be, l.f11b7, l.f11b8, l.f11b9, l.f11ba, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1109, l.f110e, l.f110f, l.f1110, l.f1111, l.f110a, l.f110b, l.f110c, l.f110d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1162, l.f1167, l.f1168, l.f1169, l.f116a, l.f1163, l.f1164, l.f1165, l.f1166, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1156, l.f115b, l.f115c, l.f115d, l.f115e, l.f1157, l.f1158, l.f1159, l.f115a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f123c, l.f123d, l.f123e, l.f123f, ) = (0.0, 0.0, 0.0, 0.0, );(l.f11df, l.f11e6, l.f11e7, l.f11e8, l.f11e9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1228, l.f1229, l.f122a, ) = (l.f23a5, l.f23a6, l.f23a7, );(l.f120b, l.f120c, l.f120d, ) = (l.f236d, l.f236e, l.f236f, );l.f113c = 0.0;l.f11dc = 0.0;l.f11da = 0.0;l.f1160 = 0.0;(l.f11c2, l.f11c3, ) = (l.f22ef, l.f22f0, );l.f11c8 = l.f22f4;(l.f1139, l.f113a, ) = (l.f215b, l.f215c, );l.f125b = p.p0;l.f1115 = p.p1;(l.f107d, l.f107e, ) = (l.f48, l.f49, );l.f1243 = p.p35;l.f11c0 = p.p36;l.f1081 = p.p37;l.f1083 = p.p38;l.f1135 = p.p40;l.f1076 = p.p41;l.f121c = p.p32;l.f1119 = p.p33;l.f107b = p.p34;l.f1117 = p.p44;l.f1241 = p.p43;l.f1248 = p.p46;l.f1088 = p.p39;l.f108a = p.p47;l.f1259 = p.p45;l.f1113 = p.p42;l.f1137 = p.p2;l.f11d8 = p.p6;(l.f11ca, l.f11cb, l.f11cc, l.f11cd, l.f11ce, ) = (l.f4b, l.f4c, l.f4d, l.f4e, l.f4f, );(l.f1077, l.f1078, ) = (0.0, 0.0, );(l.f1080, l.f1085, l.f1086, ) = (0.0, 0.0, 0.0, );(l.f112d, l.f1131, l.f1132, l.f1133, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1245, l.f1246, ) = (0.0, 0.0, );(l.f122c, l.f122d, l.f122e, ) = (0.0, 0.0, 0.0, );(l.f10eb, l.f10f2, l.f10f3, l.f10f4, l.f10f5, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11d0, l.f11d4, l.f11d5, l.f11d6, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1196, l.f119a, l.f119b, l.f119c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f10af, l.f10b6, l.f10b7, l.f10b8, l.f10b9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f117e, l.f1185, l.f1186, l.f1187, l.f1188, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f111b, l.f111f, l.f1120, l.f1121, l.f1122, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f124a, l.f124e, l.f124f, l.f1250, l.f1251, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1253, l.f1254, l.f1255, l.f1256, l.f1257, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f112e, l.f112f, ) = (0.0, 0.0, );(l.f10ec, l.f10ed, l.f10ee, l.f10ef, l.f10f0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11d1, l.f11d2, ) = (0.0, 0.0, );(l.f1197, l.f1198, ) = (0.0, 0.0, );(l.f10b0, l.f10b1, l.f10b2, l.f10b3, l.f10b4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f117f, l.f1180, l.f1181, l.f1182, l.f1183, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f111c, l.f111d, ) = (0.0, 0.0, );(l.f124b, l.f124c, ) = (0.0, 0.0, );(l.f11c5, l.f11c6, ) = (0.0, 0.0, );(l.f10d3, l.f10da, l.f10db, l.f10dc, l.f10dd, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f108c, l.f1093, l.f1094, l.f1095, l.f1096, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f118a, l.f1191, l.f1192, l.f1193, l.f1194, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f10d4, l.f10d5, l.f10d6, l.f10d7, l.f10d8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f108d, l.f108e, l.f108f, l.f1090, l.f1091, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f118b, l.f118c, l.f118d, l.f118e, l.f118f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11f0, l.f1200, l.f1201, l.f1202, l.f1203, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11f4, l.f11fb, l.f11fc, l.f11fd, l.f11fe, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11de, l.f11eb, l.f11ec, l.f11ed, l.f11ee, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f10fd, l.f1104, l.f1105, l.f1106, l.f1107, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        (l.f120f, l.f1216, l.f1217, l.f1218, l.f1219, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f10c7, l.f10ce, l.f10cf, l.f10d0, l.f10d1, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1230, l.f1237, l.f1238, l.f1239, l.f123a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f10df, l.f10e6, l.f10e7, l.f10e8, l.f10e9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f109e, l.f10a5, l.f10a6, l.f10a7, l.f10a8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f116c, l.f1173, l.f1174, l.f1175, l.f1176, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1205, l.f1206, l.f1207, l.f1208, l.f1209, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f10f7, l.f10f8, l.f10f9, l.f10fa, l.f10fb, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f121b, l.f121e, l.f121f, l.f1220, l.f1221, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11f1, l.f11f2, ) = (0.0, 0.0, );(l.f11f5, l.f11f6, l.f11f7, l.f11f8, l.f11f9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11e0, l.f11e1, l.f11e2, l.f11e3, l.f11e4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f10fe, l.f10ff, l.f1100, l.f1101, l.f1102, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1210, l.f1211, l.f1212, l.f1213, l.f1214, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f10c8, l.f10c9, l.f10ca, l.f10cb, l.f10cc, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1231, l.f1232, l.f1233, l.f1234, l.f1235, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f10e0, l.f10e1, l.f10e2, l.f10e3, l.f10e4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f109f, l.f10a0, l.f10a1, l.f10a2, l.f10a3, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f116d, l.f116e, l.f116f, l.f1170, l.f1171, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f119f, l.f11a0, l.f11a1, l.f11a2, l.f11a3, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11a5, l.f11a6, l.f11a7, l.f11a8, l.f11a9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1145, l.f1146, l.f1147, l.f1148, l.f1149, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f114b, l.f114c, l.f114d, l.f114e, l.f114f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f11b0, l.f11b1, l.f11b2, l.f11b3, l.f11b4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1178, l.f1179, l.f117a, l.f117b, l.f117c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f113f, l.f1140, l.f1141, l.f1142, l.f1143, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f119e, l.f11ab, l.f11ac, l.f11ad, l.f11ae, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f113e, l.f1151, l.f1152, l.f1153, l.f1154, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f109b, l.f109c, ) = (0.0, 0.0, );(l.f1098, l.f1099, ) = (0.0, 0.0, );(l.f10aa, l.f10ab, l.f10ac, l.f10ad, ) = (0.0, 0.0, 0.0, 0.0, );(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1124, l.f1128, l.f1129, l.f112a, l.f112b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1072, l.f1073, l.f1074, ) = (0.0, 0.0, 0.0, );(l.f1223, l.f1224, l.f1225, l.f1226, ) = (0.0, 0.0, 0.0, 0.0, );(l.f10bc, l.f10bd, l.f10be, l.f10bf, l.f10c0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1125, l.f1126, ) = (0.0, 0.0, );
        let (t100, t101, t102,) = {
    if (p.p52 != 0.0) {
        let tf6: f64 = (0.001 / p.p53);let tf7: f64 = (tf6 * l.f120b);let tf8: f64 = (tf7).tanh();let tf9: f64 = (l.f120b * tf8);
        (tf9, ((l.f120c * tf8) + (l.f120b * ((tf6 * l.f120c) / ((tf7).cosh() * (tf7).cosh())))), ((l.f120d * tf8) + (l.f120b * ((tf6 * l.f120d) / ((tf7).cosh() * (tf7).cosh())))),)
    } else {
        let (tfd, tfe, tff,) = {
            if (p.p52 == 0.0) {
                let tfa: f64 = (l.f120b * l.f120b);let tfb: f64 = (tfa + p.p53);let tfc: f64 = (tfb).sqrt();
                (tfc, (((l.f120c * l.f120b) + (l.f120b * l.f120c)) / (2.0 * tfc)), (((l.f120d * l.f120b) + (l.f120b * l.f120d)) / (2.0 * tfc)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (tfd, tfe, tff,)
    }
};
        (l.f1072, l.f1073, l.f1074, ) = (t100, t101, t102, );let t103: f64 = (l.f1228 - l.f120b);(l.f1223, l.f1224, l.f1225, l.f1226, ) = (t103, (-l.f120c), l.f1229, (l.f122a - l.f120d), );let t104: f64 = (l.f1076 * l.f1139);(l.f1077, l.f1078, ) = (t104, (l.f1076 * l.f113a), );let t105: f64 = (2.302585092994046 * l.f1139);let t106: f64 = (l.f11c0 / t105);let t107: f64 = (l.f1135 * l.f1072);let t108: f64 = (t106 + t107);(l.f112d, l.f1131, l.f1132, l.f1133, ) = (t108, (-((l.f11c0 * (2.302585092994046 * l.f113a)) / (t105 * t105))), (l.f1135 * l.f1073), (l.f1135 * l.f1074), );let t109: f64 = (l.f11c2 - l.f11c8);let t10a: f64 = (l.f1248 * t109);let t10b: f64 = (l.f1243 + t10a);(l.f1245, l.f1246, ) = (t10b, (l.f1248 * l.f11c3), );let t10c: f64 = (l.f11c2 / l.f11c8);let t10d: f64 = (t10c).powf(l.f108a);(l.f11c5, l.f11c6, ) = (t10d, if 0.0 == 0.0 && ((l.f108a) as f64).is_finite() && ((l.f108a) as f64).fract() == 0.0 { if l.f108a == 0.0 { 0.0 } else { (l.f108a * ((t10c).powf(l.f108a - 1.0) * (l.f11c3 / l.f11c8))) } } else { (t10d * (l.f108a * ((l.f11c3 / l.f11c8) / t10c))) }, );let t10e: f64 = if l.f1088 != 0.0 { 1.0 } else { 0.0 };l.f1f88 = t10e;
        if (l.f1f88 != 0.0) {let t10f: f64 = (l.f1072 / l.f1088);let t110: f64 = (t10f).powf(l.f107b);let t111: f64 = (1.0 + t110);let t112: f64 = (1.0 / l.f107b);let t113: f64 = (t111).powf(t112);let t114: f64 = (l.f1072 / t113);(l.f122c, l.f122d, l.f122e, ) = (t114, (((l.f1073 * t113) - (l.f1072 * if 0.0 == 0.0 && ((t112) as f64).is_finite() && ((t112) as f64).fract() == 0.0 { if t112 == 0.0 { 0.0 } else { (t112 * ((t111).powf(t112 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t10f).powf(l.f107b - 1.0) * (l.f1073 / l.f1088))) } } else { (t110 * (l.f107b * ((l.f1073 / l.f1088) / t10f))) })) } } else { (t113 * (t112 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t10f).powf(l.f107b - 1.0) * (l.f1073 / l.f1088))) } } else { (t110 * (l.f107b * ((l.f1073 / l.f1088) / t10f))) } / t111))) })) / (t113 * t113)), (((l.f1074 * t113) - (l.f1072 * if 0.0 == 0.0 && ((t112) as f64).is_finite() && ((t112) as f64).fract() == 0.0 { if t112 == 0.0 { 0.0 } else { (t112 * ((t111).powf(t112 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t10f).powf(l.f107b - 1.0) * (l.f1074 / l.f1088))) } } else { (t110 * (l.f107b * ((l.f1074 / l.f1088) / t10f))) })) } } else { (t113 * (t112 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t10f).powf(l.f107b - 1.0) * (l.f1074 / l.f1088))) } } else { (t110 * (l.f107b * ((l.f1074 / l.f1088) / t10f))) } / t111))) })) / (t113 * t113)), );}
        if (l.f1f88 == 0.0) {(l.f122c, l.f122d, l.f122e, ) = (0.0, 0.0, 0.0, );}
        let t115: f64 = (l.f122c * l.f1083);let t116: f64 = (l.f1081 - t115);let t117: f64 = (t116 * l.f1072);(l.f1080, l.f1085, l.f1086, ) = (t117, (((-(l.f122d * l.f1083)) * l.f1072) + (t116 * l.f1073)), (((-(l.f122e * l.f1083)) * l.f1072) + (t116 * l.f1074)), );let t118: f64 = (l.f1245 - l.f1080);(l.f123c, l.f123d, l.f123e, l.f123f, ) = (t118, l.f1246, (-l.f1085), (-l.f1086), );let t119: f64 = (2.0 * l.f112d);let t11a: f64 = (t119 * l.f1139);(l.f11d0, l.f11d4, l.f11d5, l.f11d6, ) = (t11a, (((2.0 * l.f1131) * l.f1139) + (t119 * l.f113a)), ((2.0 * l.f1132) * l.f1139), ((2.0 * l.f1133) * l.f1139), );let t11b: f64 = (l.f107d * l.f11d0);(l.f1196, l.f119a, l.f119b, l.f119c, ) = (t11b, ((l.f107e * l.f11d0) + (l.f107d * l.f11d4)), (l.f107d * l.f11d5), (l.f107d * l.f11d6), );let t11c: f64 = (p.p51 * l.f1077);let t11d: f64 = (t11c / 2.0);let t11e: f64 = (l.f123c - t11d);(l.f1124, l.f1128, l.f1129, l.f112a, l.f112b, ) = (t11e, (l.f123d - ((p.p51 * l.f1078) / 2.0)), l.f123e, 0.0, l.f123f, );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let (t134, t135, t136, t137,) = {
    if (p.p52 != 0.0) {
        let t11f: f64 = (l.f1228 + l.f1223);let t120: f64 = (l.f1228 - l.f1223);let t121: f64 = (0.001 / p.p53);let t122: f64 = (l.f1228 - l.f1223);let t123: f64 = (t121 * t122);let t124: f64 = (t123).tanh();let t125: f64 = (t120 * t124);let t126: f64 = (t11f + t125);let t127: f64 = (0.5 * t126);
        (t127, (0.5 * (l.f1224 + (((-l.f1224) * t124) + (t120 * ((t121 * (-l.f1224)) / ((t123).cosh() * (t123).cosh())))))), (0.5 * ((l.f1229 + l.f1225) + (((l.f1229 - l.f1225) * t124) + (t120 * ((t121 * (l.f1229 - l.f1225)) / ((t123).cosh() * (t123).cosh())))))), (0.5 * ((l.f122a + l.f1226) + (((l.f122a - l.f1226) * t124) + (t120 * ((t121 * (l.f122a - l.f1226)) / ((t123).cosh() * (t123).cosh())))))),)
    } else {
        let (t130, t131, t132, t133,) = {
            if (p.p52 == 0.0) {
                let t128: f64 = (l.f1228 + l.f1223);let t129: f64 = (l.f1228 - l.f1223);let t12a: f64 = (l.f1228 - l.f1223);let t12b: f64 = (t129 * t12a);let t12c: f64 = (t12b + p.p53);let t12d: f64 = (t12c).sqrt();let t12e: f64 = (t128 + t12d);let t12f: f64 = (0.5 * t12e);
                (t12f, (0.5 * (l.f1224 + ((((-l.f1224) * t12a) + (t129 * (-l.f1224))) / (2.0 * t12d)))), (0.5 * ((l.f1229 + l.f1225) + ((((l.f1229 - l.f1225) * t12a) + (t129 * (l.f1229 - l.f1225))) / (2.0 * t12d)))), (0.5 * ((l.f122a + l.f1226) + ((((l.f122a - l.f1226) * t12a) + (t129 * (l.f122a - l.f1226))) / (2.0 * t12d)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t130, t131, t132, t133,)
    }
};
        let t138: f64 = (t134 - l.f1124);let t139: f64 = (t138 / l.f1077);(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t139, ((((-l.f1128) * l.f1077) - (t138 * l.f1078)) / (l.f1077 * l.f1077)), ((t135 - l.f1129) / l.f1077), ((t136 - l.f112a) / l.f1077), ((t137 - l.f112b) / l.f1077), );let t13a: f64 = if l.f10bb > 50.0 { 1.0 } else { 0.0 };l.f1f8a = t13a;
        if (l.f1f8a != 0.0) {(l.f10d3, l.f10da, l.f10db, l.f10dc, l.f10dd, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t13b: f64 = (-50.0);let t13c: f64 = if l.f10bb < t13b { 1.0 } else { 0.0 };l.f1f8c = t13c;
        if ((l.f1f8a == 0.0) && (l.f1f8c != 0.0)) {(l.f10d3, l.f10da, l.f10db, l.f10dc, l.f10dd, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((l.f1f8a == 0.0) && (l.f1f8c == 0.0)) {let t13d: f64 = (l.f10bb).exp();let t13e: f64 = (1.0 + t13d);let t13f: f64 = (1.0 / t13e);(l.f10d3, l.f10da, l.f10db, l.f10dc, l.f10dd, ) = (t13f, (-((t13d * l.f10c2) / (t13e * t13e))), (-((t13d * l.f10c3) / (t13e * t13e))), (-((t13d * l.f10c4) / (t13e * t13e))), (-((t13d * l.f10c5) / (t13e * t13e))), );}
        let (t155, t156, t157, t158,) = {
    if (p.p52 != 0.0) {
        let t140: f64 = (l.f1228 + l.f1223);let t141: f64 = (l.f1228 - l.f1223);let t142: f64 = (0.001 / p.p53);let t143: f64 = (l.f1228 - l.f1223);let t144: f64 = (t142 * t143);let t145: f64 = (t144).tanh();let t146: f64 = (t141 * t145);let t147: f64 = (t140 + t146);let t148: f64 = (0.5 * t147);
        (t148, (0.5 * (l.f1224 + (((-l.f1224) * t145) + (t141 * ((t142 * (-l.f1224)) / ((t144).cosh() * (t144).cosh())))))), (0.5 * ((l.f1229 + l.f1225) + (((l.f1229 - l.f1225) * t145) + (t141 * ((t142 * (l.f1229 - l.f1225)) / ((t144).cosh() * (t144).cosh())))))), (0.5 * ((l.f122a + l.f1226) + (((l.f122a - l.f1226) * t145) + (t141 * ((t142 * (l.f122a - l.f1226)) / ((t144).cosh() * (t144).cosh())))))),)
    } else {
        let (t151, t152, t153, t154,) = {
            if (p.p52 == 0.0) {
                let t149: f64 = (l.f1228 + l.f1223);let t14a: f64 = (l.f1228 - l.f1223);let t14b: f64 = (l.f1228 - l.f1223);let t14c: f64 = (t14a * t14b);let t14d: f64 = (t14c + p.p53);let t14e: f64 = (t14d).sqrt();let t14f: f64 = (t149 + t14e);let t150: f64 = (0.5 * t14f);
                (t150, (0.5 * (l.f1224 + ((((-l.f1224) * t14b) + (t14a * (-l.f1224))) / (2.0 * t14e)))), (0.5 * ((l.f1229 + l.f1225) + ((((l.f1229 - l.f1225) * t14b) + (t14a * (l.f1229 - l.f1225))) / (2.0 * t14e)))), (0.5 * ((l.f122a + l.f1226) + ((((l.f122a - l.f1226) * t14b) + (t14a * (l.f122a - l.f1226))) / (2.0 * t14e)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t151, t152, t153, t154,)
    }
};
        let t159: f64 = (p.p51 * 0.1);let t15a: f64 = (t159 * l.f1077);let t15b: f64 = (t15a * l.f10d3);let t15c: f64 = (l.f123c - t15b);let t15d: f64 = (t155 - t15c);let t15e: f64 = (t15d / l.f11d0);(l.f108c, l.f1093, l.f1094, l.f1095, l.f1096, ) = (t15e, ((((-(l.f123d - (((t159 * l.f1078) * l.f10d3) + (t15a * l.f10da)))) * l.f11d0) - (t15d * l.f11d4)) / (l.f11d0 * l.f11d0)), ((((t156 - (l.f123e - (t15a * l.f10db))) * l.f11d0) - (t15d * l.f11d5)) / (l.f11d0 * l.f11d0)), ((t157 - (-(t15a * l.f10dc))) / l.f11d0), ((((t158 - (l.f123f - (t15a * l.f10dd))) * l.f11d0) - (t15d * l.f11d6)) / (l.f11d0 * l.f11d0)), );let t15f: f64 = if l.f108c > 50.0 { 1.0 } else { 0.0 };l.f1f8e = t15f;
        if (l.f1f8e != 0.0) {let t160: f64 = (l.f1196 * l.f108c);(l.f118a, l.f1191, l.f1192, l.f1193, l.f1194, ) = (t160, ((l.f119a * l.f108c) + (l.f1196 * l.f1093)), ((l.f119b * l.f108c) + (l.f1196 * l.f1094)), (l.f1196 * l.f1095), ((l.f119c * l.f108c) + (l.f1196 * l.f1096)), );}
        let t161: f64 = (-50.0);let t162: f64 = if l.f108c < t161 { 1.0 } else { 0.0 };l.f1f90 = t162;
        if ((l.f1f8e == 0.0) && (l.f1f90 != 0.0)) {let t163: f64 = (l.f108c).exp();let t164: f64 = (l.f1196 * t163);(l.f118a, l.f1191, l.f1192, l.f1193, l.f1194, ) = (t164, ((l.f119a * t163) + (l.f1196 * (t163 * l.f1093))), ((l.f119b * t163) + (l.f1196 * (t163 * l.f1094))), (l.f1196 * (t163 * l.f1095)), ((l.f119c * t163) + (l.f1196 * (t163 * l.f1096))), );}
        if ((l.f1f8e == 0.0) && (l.f1f90 == 0.0)) {let t165: f64 = (l.f108c).exp();let t166: f64 = (1.0 + t165);let t167: f64 = (t166).ln();let t168: f64 = (l.f1196 * t167);(l.f118a, l.f1191, l.f1192, l.f1193, l.f1194, ) = (t168, ((l.f119a * t167) + (l.f1196 * ((t165 * l.f1093) / t166))), ((l.f119b * t167) + (l.f1196 * ((t165 * l.f1094) / t166))), (l.f1196 * ((t165 * l.f1095) / t166)), ((l.f119c * t167) + (l.f1196 * ((t165 * l.f1096) / t166))), );}
        let t169: f64 = (l.f1117 * l.f118a);let t16a: f64 = (t169 / l.f107d);let t16b: f64 = (1.0 + t16a);let t16c: f64 = (l.f11c5 * t16b);let t16d: f64 = (l.f1119 / t16c);(l.f111b, l.f111f, l.f1120, l.f1121, l.f1122, ) = (t16d, (-((l.f1119 * ((l.f11c6 * t16b) + (l.f11c5 * ((((l.f1117 * l.f1191) * l.f107d) - (t169 * l.f107e)) / (l.f107d * l.f107d))))) / (t16c * t16c))), (-((l.f1119 * (l.f11c5 * ((l.f1117 * l.f1192) / l.f107d))) / (t16c * t16c))), (-((l.f1119 * (l.f11c5 * ((l.f1117 * l.f1193) / l.f107d))) / (t16c * t16c))), (-((l.f1119 * (l.f11c5 * ((l.f1117 * l.f1194) / l.f107d))) / (t16c * t16c))), );let t16e: f64 = (l.f1259 * l.f11c8);let t16f: f64 = (1.0 + t16e);let t170: f64 = (l.f1259 * l.f11c2);let t171: f64 = (1.0 + t170);let t172: f64 = (t16f / t171);let t173: f64 = (l.f121c * t172);let t174: f64 = (l.f1113 * l.f1072);let t175: f64 = (t174 / l.f1115);let t176: f64 = (1.0 + t175);let t177: f64 = (t173 * t176);let t178: f64 = (l.f1241 * l.f118a);let t179: f64 = (t178 / l.f107d);let t17a: f64 = (1.0 + t179);let t17b: f64 = (t177 / t17a);(l.f124a, l.f124e, l.f124f, l.f1250, l.f1251, ) = (t17b, (((((l.f121c * (-((t16f * (l.f1259 * l.f11c3)) / (t171 * t171)))) * t176) * t17a) - (t177 * ((((l.f1241 * l.f1191) * l.f107d) - (t178 * l.f107e)) / (l.f107d * l.f107d)))) / (t17a * t17a)), ((((t173 * ((l.f1113 * l.f1073) / l.f1115)) * t17a) - (t177 * ((l.f1241 * l.f1192) / l.f107d))) / (t17a * t17a)), (-((t177 * ((l.f1241 * l.f1193) / l.f107d)) / (t17a * t17a))), ((((t173 * ((l.f1113 * l.f1074) / l.f1115)) * t17a) - (t177 * ((l.f1241 * l.f1194) / l.f107d))) / (t17a * t17a)), );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t17c: f64 = (2.0 * l.f10d3);let t17d: f64 = (t17c * l.f1139);let t17e: f64 = (t17d * l.f111b);let t17f: f64 = (t17e / l.f1115);let t180: f64 = (1.0 - l.f10d3);let t181: f64 = (t180 * l.f124a);let t182: f64 = (t17f + t181);(l.f1253, l.f1254, l.f1255, l.f1256, l.f1257, ) = (t182, (((((((2.0 * l.f10da) * l.f1139) + (t17c * l.f113a)) * l.f111b) + (t17d * l.f111f)) / l.f1115) + (((-l.f10da) * l.f124a) + (t180 * l.f124e))), ((((((2.0 * l.f10db) * l.f1139) * l.f111b) + (t17d * l.f1120)) / l.f1115) + (((-l.f10db) * l.f124a) + (t180 * l.f124f))), ((((((2.0 * l.f10dc) * l.f1139) * l.f111b) + (t17d * l.f1121)) / l.f1115) + (((-l.f10dc) * l.f124a) + (t180 * l.f1250))), ((((((2.0 * l.f10dd) * l.f1139) * l.f111b) + (t17d * l.f1122)) / l.f1115) + (((-l.f10dd) * l.f124a) + (t180 * l.f1251))), );let t183: f64 = (l.f124a * l.f1115);let t184: f64 = (t183 / l.f111b);(l.f11f0, l.f1200, l.f1201, l.f1202, l.f1203, ) = (t184, ((((l.f124e * l.f1115) * l.f111b) - (t183 * l.f111f)) / (l.f111b * l.f111b)), ((((l.f124f * l.f1115) * l.f111b) - (t183 * l.f1120)) / (l.f111b * l.f111b)), ((((l.f1250 * l.f1115) * l.f111b) - (t183 * l.f1121)) / (l.f111b * l.f111b)), ((((l.f1251 * l.f1115) * l.f111b) - (t183 * l.f1122)) / (l.f111b * l.f111b)), );let t185: f64 = (2.0 * l.f118a);let t186: f64 = (t185 / l.f107d);let t187: f64 = (t186 / l.f11f0);let t188: f64 = (1.0 + t187);let t189: f64 = (t188).sqrt();let t18a: f64 = (l.f11f0 * t189);let t18b: f64 = (t18a - l.f11f0);(l.f11f4, l.f11fb, l.f11fc, l.f11fd, l.f11fe, ) = (t18b, (((l.f1200 * t189) + (l.f11f0 * ((((((((2.0 * l.f1191) * l.f107d) - (t185 * l.f107e)) / (l.f107d * l.f107d)) * l.f11f0) - (t186 * l.f1200)) / (l.f11f0 * l.f11f0)) / (2.0 * t189)))) - l.f1200), (((l.f1201 * t189) + (l.f11f0 * ((((((2.0 * l.f1192) / l.f107d) * l.f11f0) - (t186 * l.f1201)) / (l.f11f0 * l.f11f0)) / (2.0 * t189)))) - l.f1201), (((l.f1202 * t189) + (l.f11f0 * ((((((2.0 * l.f1193) / l.f107d) * l.f11f0) - (t186 * l.f1202)) / (l.f11f0 * l.f11f0)) / (2.0 * t189)))) - l.f1202), (((l.f1203 * t189) + (l.f11f0 * ((((((2.0 * l.f1194) / l.f107d) * l.f11f0) - (t186 * l.f1203)) / (l.f11f0 * l.f11f0)) / (2.0 * t189)))) - l.f1203), );let t18c: f64 = (1.0 - l.f10d3);let t18d: f64 = (l.f11f0 * t18c);let t18e: f64 = (l.f11d0 * l.f10d3);let t18f: f64 = (t18d + t18e);(l.f11de, l.f11eb, l.f11ec, l.f11ed, l.f11ee, ) = (t18f, (((l.f1200 * t18c) + (l.f11f0 * (-l.f10da))) + ((l.f11d4 * l.f10d3) + (l.f11d0 * l.f10da))), (((l.f1201 * t18c) + (l.f11f0 * (-l.f10db))) + ((l.f11d5 * l.f10d3) + (l.f11d0 * l.f10db))), (((l.f1202 * t18c) + (l.f11f0 * (-l.f10dc))) + (l.f11d0 * l.f10dc)), (((l.f1203 * t18c) + (l.f11f0 * (-l.f10dd))) + ((l.f11d6 * l.f10d3) + (l.f11d0 * l.f10dd))), );let t190: f64 = (1.0 - l.f10d3);let t191: f64 = (l.f11f4 * t190);let t192: f64 = (l.f11d0 * l.f10d3);let t193: f64 = (t191 + t192);
        (l.f11df, l.f11e6, l.f11e7, l.f11e8, l.f11e9, ) = (t193, (((l.f11fb * t190) + (l.f11f4 * (-l.f10da))) + ((l.f11d4 * l.f10d3) + (l.f11d0 * l.f10da))), (((l.f11fc * t190) + (l.f11f4 * (-l.f10db))) + ((l.f11d5 * l.f10d3) + (l.f11d0 * l.f10db))), (((l.f11fd * t190) + (l.f11f4 * (-l.f10dc))) + (l.f11d0 * l.f10dc)), (((l.f11fe * t190) + (l.f11f4 * (-l.f10dd))) + ((l.f11d6 * l.f10d3) + (l.f11d0 * l.f10dd))), );
        let (t1b0, t1b1, t1b2, t1b3, t1b4,) = {
    if (p.p52 != 0.0) {
        let t194: f64 = (l.f120b / l.f11df);let t195: f64 = t194;let t196: f64 = (l.f120b / l.f11df);let t197: f64 = (-t196);let t198: f64 = (0.001 / p.p53);let t199: f64 = (l.f120b / l.f11df);let t19a: f64 = (-t199);let t19b: f64 = (t198 * t19a);let t19c: f64 = (t19b).tanh();let t19d: f64 = (t197 * t19c);let t19e: f64 = (t195 + t19d);let t19f: f64 = (0.5 * t19e);
        (t19f, (0.5 * ((-((l.f120b * l.f11e6) / (l.f11df * l.f11df))) + (((-(-((l.f120b * l.f11e6) / (l.f11df * l.f11df)))) * t19c) + (t197 * ((t198 * (-(-((l.f120b * l.f11e6) / (l.f11df * l.f11df))))) / ((t19b).cosh() * (t19b).cosh())))))), (0.5 * ((((l.f120c * l.f11df) - (l.f120b * l.f11e7)) / (l.f11df * l.f11df)) + (((-(((l.f120c * l.f11df) - (l.f120b * l.f11e7)) / (l.f11df * l.f11df))) * t19c) + (t197 * ((t198 * (-(((l.f120c * l.f11df) - (l.f120b * l.f11e7)) / (l.f11df * l.f11df)))) / ((t19b).cosh() * (t19b).cosh())))))), (0.5 * ((-((l.f120b * l.f11e8) / (l.f11df * l.f11df))) + (((-(-((l.f120b * l.f11e8) / (l.f11df * l.f11df)))) * t19c) + (t197 * ((t198 * (-(-((l.f120b * l.f11e8) / (l.f11df * l.f11df))))) / ((t19b).cosh() * (t19b).cosh())))))), (0.5 * ((((l.f120d * l.f11df) - (l.f120b * l.f11e9)) / (l.f11df * l.f11df)) + (((-(((l.f120d * l.f11df) - (l.f120b * l.f11e9)) / (l.f11df * l.f11df))) * t19c) + (t197 * ((t198 * (-(((l.f120d * l.f11df) - (l.f120b * l.f11e9)) / (l.f11df * l.f11df)))) / ((t19b).cosh() * (t19b).cosh())))))),)
    } else {
        let (t1ab, t1ac, t1ad, t1ae, t1af,) = {
            if (p.p52 == 0.0) {
                let t1a0: f64 = (l.f120b / l.f11df);let t1a1: f64 = t1a0;let t1a2: f64 = (l.f120b / l.f11df);let t1a3: f64 = (-t1a2);let t1a4: f64 = (l.f120b / l.f11df);let t1a5: f64 = (-t1a4);let t1a6: f64 = (t1a3 * t1a5);let t1a7: f64 = (t1a6 + p.p53);let t1a8: f64 = (t1a7).sqrt();let t1a9: f64 = (t1a1 + t1a8);let t1aa: f64 = (0.5 * t1a9);
                (t1aa, (0.5 * ((-((l.f120b * l.f11e6) / (l.f11df * l.f11df))) + ((((-(-((l.f120b * l.f11e6) / (l.f11df * l.f11df)))) * t1a5) + (t1a3 * (-(-((l.f120b * l.f11e6) / (l.f11df * l.f11df)))))) / (2.0 * t1a8)))), (0.5 * ((((l.f120c * l.f11df) - (l.f120b * l.f11e7)) / (l.f11df * l.f11df)) + ((((-(((l.f120c * l.f11df) - (l.f120b * l.f11e7)) / (l.f11df * l.f11df))) * t1a5) + (t1a3 * (-(((l.f120c * l.f11df) - (l.f120b * l.f11e7)) / (l.f11df * l.f11df))))) / (2.0 * t1a8)))), (0.5 * ((-((l.f120b * l.f11e8) / (l.f11df * l.f11df))) + ((((-(-((l.f120b * l.f11e8) / (l.f11df * l.f11df)))) * t1a5) + (t1a3 * (-(-((l.f120b * l.f11e8) / (l.f11df * l.f11df)))))) / (2.0 * t1a8)))), (0.5 * ((((l.f120d * l.f11df) - (l.f120b * l.f11e9)) / (l.f11df * l.f11df)) + ((((-(((l.f120d * l.f11df) - (l.f120b * l.f11e9)) / (l.f11df * l.f11df))) * t1a5) + (t1a3 * (-(((l.f120d * l.f11df) - (l.f120b * l.f11e9)) / (l.f11df * l.f11df))))) / (2.0 * t1a8)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t1ab, t1ac, t1ad, t1ae, t1af,)
    }
};
        let t1b5: f64 = (t1b0).powf(l.f107b);let t1b6: f64 = (1.0 + t1b5);let t1b7: f64 = (1.0 / l.f107b);let t1b8: f64 = (t1b6).powf(t1b7);let t1b9: f64 = (1.0 / t1b8);
        (l.f10fd, l.f1104, l.f1105, l.f1106, l.f1107, ) = (t1b9, (-(if 0.0 == 0.0 && ((t1b7) as f64).is_finite() && ((t1b7) as f64).fract() == 0.0 { if t1b7 == 0.0 { 0.0 } else { (t1b7 * ((t1b6).powf(t1b7 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1b0).powf(l.f107b - 1.0) * t1b1)) } } else { (t1b5 * (l.f107b * (t1b1 / t1b0))) })) } } else { (t1b8 * (t1b7 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1b0).powf(l.f107b - 1.0) * t1b1)) } } else { (t1b5 * (l.f107b * (t1b1 / t1b0))) } / t1b6))) } / (t1b8 * t1b8))), (-(if 0.0 == 0.0 && ((t1b7) as f64).is_finite() && ((t1b7) as f64).fract() == 0.0 { if t1b7 == 0.0 { 0.0 } else { (t1b7 * ((t1b6).powf(t1b7 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1b0).powf(l.f107b - 1.0) * t1b2)) } } else { (t1b5 * (l.f107b * (t1b2 / t1b0))) })) } } else { (t1b8 * (t1b7 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1b0).powf(l.f107b - 1.0) * t1b2)) } } else { (t1b5 * (l.f107b * (t1b2 / t1b0))) } / t1b6))) } / (t1b8 * t1b8))), (-(if 0.0 == 0.0 && ((t1b7) as f64).is_finite() && ((t1b7) as f64).fract() == 0.0 { if t1b7 == 0.0 { 0.0 } else { (t1b7 * ((t1b6).powf(t1b7 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1b0).powf(l.f107b - 1.0) * t1b3)) } } else { (t1b5 * (l.f107b * (t1b3 / t1b0))) })) } } else { (t1b8 * (t1b7 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1b0).powf(l.f107b - 1.0) * t1b3)) } } else { (t1b5 * (l.f107b * (t1b3 / t1b0))) } / t1b6))) } / (t1b8 * t1b8))), (-(if 0.0 == 0.0 && ((t1b7) as f64).is_finite() && ((t1b7) as f64).fract() == 0.0 { if t1b7 == 0.0 { 0.0 } else { (t1b7 * ((t1b6).powf(t1b7 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1b0).powf(l.f107b - 1.0) * t1b4)) } } else { (t1b5 * (l.f107b * (t1b4 / t1b0))) })) } } else { (t1b8 * (t1b7 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1b0).powf(l.f107b - 1.0) * t1b4)) } } else { (t1b5 * (l.f107b * (t1b4 / t1b0))) } / t1b6))) } / (t1b8 * t1b8))), );let t1ba: f64 = (l.f120b * l.f10fd);
        (l.f120f, l.f1216, l.f1217, l.f1218, l.f1219, ) = (t1ba, (l.f120b * l.f1104), ((l.f120c * l.f10fd) + (l.f120b * l.f1105)), (l.f120b * l.f1106), ((l.f120d * l.f10fd) + (l.f120b * l.f1107)), );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let (t1dd, t1de, t1df, t1e0, t1e1,) = {
    if (p.p52 != 0.0) {
        let t1bb: f64 = (-l.f120b);let t1bc: f64 = (t1bb / l.f11df);let t1bd: f64 = t1bc;let t1be: f64 = (-l.f120b);let t1bf: f64 = (t1be / l.f11df);let t1c0: f64 = (-t1bf);let t1c1: f64 = (0.001 / p.p53);let t1c2: f64 = (-l.f120b);let t1c3: f64 = (t1c2 / l.f11df);let t1c4: f64 = (-t1c3);let t1c5: f64 = (t1c1 * t1c4);let t1c6: f64 = (t1c5).tanh();let t1c7: f64 = (t1c0 * t1c6);let t1c8: f64 = (t1bd + t1c7);let t1c9: f64 = (0.5 * t1c8);
        (t1c9, (0.5 * ((-((t1bb * l.f11e6) / (l.f11df * l.f11df))) + (((-(-((t1be * l.f11e6) / (l.f11df * l.f11df)))) * t1c6) + (t1c0 * ((t1c1 * (-(-((t1c2 * l.f11e6) / (l.f11df * l.f11df))))) / ((t1c5).cosh() * (t1c5).cosh())))))), (0.5 * (((((-l.f120c) * l.f11df) - (t1bb * l.f11e7)) / (l.f11df * l.f11df)) + (((-((((-l.f120c) * l.f11df) - (t1be * l.f11e7)) / (l.f11df * l.f11df))) * t1c6) + (t1c0 * ((t1c1 * (-((((-l.f120c) * l.f11df) - (t1c2 * l.f11e7)) / (l.f11df * l.f11df)))) / ((t1c5).cosh() * (t1c5).cosh())))))), (0.5 * ((-((t1bb * l.f11e8) / (l.f11df * l.f11df))) + (((-(-((t1be * l.f11e8) / (l.f11df * l.f11df)))) * t1c6) + (t1c0 * ((t1c1 * (-(-((t1c2 * l.f11e8) / (l.f11df * l.f11df))))) / ((t1c5).cosh() * (t1c5).cosh())))))), (0.5 * (((((-l.f120d) * l.f11df) - (t1bb * l.f11e9)) / (l.f11df * l.f11df)) + (((-((((-l.f120d) * l.f11df) - (t1be * l.f11e9)) / (l.f11df * l.f11df))) * t1c6) + (t1c0 * ((t1c1 * (-((((-l.f120d) * l.f11df) - (t1c2 * l.f11e9)) / (l.f11df * l.f11df)))) / ((t1c5).cosh() * (t1c5).cosh())))))),)
    } else {
        let (t1d8, t1d9, t1da, t1db, t1dc,) = {
            if (p.p52 == 0.0) {
                let t1ca: f64 = (-l.f120b);let t1cb: f64 = (t1ca / l.f11df);let t1cc: f64 = t1cb;let t1cd: f64 = (-l.f120b);let t1ce: f64 = (t1cd / l.f11df);let t1cf: f64 = (-t1ce);let t1d0: f64 = (-l.f120b);let t1d1: f64 = (t1d0 / l.f11df);let t1d2: f64 = (-t1d1);let t1d3: f64 = (t1cf * t1d2);let t1d4: f64 = (t1d3 + p.p53);let t1d5: f64 = (t1d4).sqrt();let t1d6: f64 = (t1cc + t1d5);let t1d7: f64 = (0.5 * t1d6);
                (t1d7, (0.5 * ((-((t1ca * l.f11e6) / (l.f11df * l.f11df))) + ((((-(-((t1cd * l.f11e6) / (l.f11df * l.f11df)))) * t1d2) + (t1cf * (-(-((t1d0 * l.f11e6) / (l.f11df * l.f11df)))))) / (2.0 * t1d5)))), (0.5 * (((((-l.f120c) * l.f11df) - (t1ca * l.f11e7)) / (l.f11df * l.f11df)) + ((((-((((-l.f120c) * l.f11df) - (t1cd * l.f11e7)) / (l.f11df * l.f11df))) * t1d2) + (t1cf * (-((((-l.f120c) * l.f11df) - (t1d0 * l.f11e7)) / (l.f11df * l.f11df))))) / (2.0 * t1d5)))), (0.5 * ((-((t1ca * l.f11e8) / (l.f11df * l.f11df))) + ((((-(-((t1cd * l.f11e8) / (l.f11df * l.f11df)))) * t1d2) + (t1cf * (-(-((t1d0 * l.f11e8) / (l.f11df * l.f11df)))))) / (2.0 * t1d5)))), (0.5 * (((((-l.f120d) * l.f11df) - (t1ca * l.f11e9)) / (l.f11df * l.f11df)) + ((((-((((-l.f120d) * l.f11df) - (t1cd * l.f11e9)) / (l.f11df * l.f11df))) * t1d2) + (t1cf * (-((((-l.f120d) * l.f11df) - (t1d0 * l.f11e9)) / (l.f11df * l.f11df))))) / (2.0 * t1d5)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t1d8, t1d9, t1da, t1db, t1dc,)
    }
};
        let t1e2: f64 = (t1dd).powf(l.f107b);let t1e3: f64 = (1.0 + t1e2);let t1e4: f64 = (1.0 / l.f107b);let t1e5: f64 = (t1e3).powf(t1e4);let t1e6: f64 = (1.0 / t1e5);
        (l.f10c7, l.f10ce, l.f10cf, l.f10d0, l.f10d1, ) = (t1e6, (-(if 0.0 == 0.0 && ((t1e4) as f64).is_finite() && ((t1e4) as f64).fract() == 0.0 { if t1e4 == 0.0 { 0.0 } else { (t1e4 * ((t1e3).powf(t1e4 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1dd).powf(l.f107b - 1.0) * t1de)) } } else { (t1e2 * (l.f107b * (t1de / t1dd))) })) } } else { (t1e5 * (t1e4 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1dd).powf(l.f107b - 1.0) * t1de)) } } else { (t1e2 * (l.f107b * (t1de / t1dd))) } / t1e3))) } / (t1e5 * t1e5))), (-(if 0.0 == 0.0 && ((t1e4) as f64).is_finite() && ((t1e4) as f64).fract() == 0.0 { if t1e4 == 0.0 { 0.0 } else { (t1e4 * ((t1e3).powf(t1e4 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1dd).powf(l.f107b - 1.0) * t1df)) } } else { (t1e2 * (l.f107b * (t1df / t1dd))) })) } } else { (t1e5 * (t1e4 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1dd).powf(l.f107b - 1.0) * t1df)) } } else { (t1e2 * (l.f107b * (t1df / t1dd))) } / t1e3))) } / (t1e5 * t1e5))), (-(if 0.0 == 0.0 && ((t1e4) as f64).is_finite() && ((t1e4) as f64).fract() == 0.0 { if t1e4 == 0.0 { 0.0 } else { (t1e4 * ((t1e3).powf(t1e4 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1dd).powf(l.f107b - 1.0) * t1e0)) } } else { (t1e2 * (l.f107b * (t1e0 / t1dd))) })) } } else { (t1e5 * (t1e4 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1dd).powf(l.f107b - 1.0) * t1e0)) } } else { (t1e2 * (l.f107b * (t1e0 / t1dd))) } / t1e3))) } / (t1e5 * t1e5))), (-(if 0.0 == 0.0 && ((t1e4) as f64).is_finite() && ((t1e4) as f64).fract() == 0.0 { if t1e4 == 0.0 { 0.0 } else { (t1e4 * ((t1e3).powf(t1e4 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1dd).powf(l.f107b - 1.0) * t1e1)) } } else { (t1e2 * (l.f107b * (t1e1 / t1dd))) })) } } else { (t1e5 * (t1e4 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t1dd).powf(l.f107b - 1.0) * t1e1)) } } else { (t1e2 * (l.f107b * (t1e1 / t1dd))) } / t1e3))) } / (t1e5 * t1e5))), );let t1e7: f64 = (-l.f120b);let t1e8: f64 = (t1e7 * l.f10c7);
        (l.f1230, l.f1237, l.f1238, l.f1239, l.f123a, ) = (t1e8, (t1e7 * l.f10ce), (((-l.f120c) * l.f10c7) + (t1e7 * l.f10cf)), (t1e7 * l.f10d0), (((-l.f120d) * l.f10c7) + (t1e7 * l.f10d1)), );let t1e9: f64 = (l.f1228 - l.f1124);let t1ea: f64 = (t1e9 / l.f1077);(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t1ea, ((((-l.f1128) * l.f1077) - (t1e9 * l.f1078)) / (l.f1077 * l.f1077)), ((-l.f1129) / l.f1077), ((l.f1229 - l.f112a) / l.f1077), ((l.f122a - l.f112b) / l.f1077), );let t1eb: f64 = if l.f10bb > 50.0 { 1.0 } else { 0.0 };l.f1f92 = t1eb;
        if (l.f1f92 != 0.0) {(l.f10eb, l.f10f2, l.f10f3, l.f10f4, l.f10f5, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t1ec: f64 = (-50.0);let t1ed: f64 = if l.f10bb < t1ec { 1.0 } else { 0.0 };l.f1f94 = t1ed;
        if ((l.f1f92 == 0.0) && (l.f1f94 != 0.0)) {(l.f10eb, l.f10f2, l.f10f3, l.f10f4, l.f10f5, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((l.f1f92 == 0.0) && (l.f1f94 == 0.0)) {let t1ee: f64 = (l.f10bb).exp();let t1ef: f64 = (1.0 + t1ee);let t1f0: f64 = (1.0 / t1ef);(l.f10eb, l.f10f2, l.f10f3, l.f10f4, l.f10f5, ) = (t1f0, (-((t1ee * l.f10c2) / (t1ef * t1ef))), (-((t1ee * l.f10c3) / (t1ef * t1ef))), (-((t1ee * l.f10c4) / (t1ef * t1ef))), (-((t1ee * l.f10c5) / (t1ef * t1ef))), );}
        let t1f1: f64 = (l.f1223 - l.f1230);let t1f2: f64 = (p.p51 * 0.1);let t1f3: f64 = (t1f2 * l.f1077);let t1f4: f64 = (t1f3 * l.f10eb);let t1f5: f64 = (l.f123c - t1f4);let t1f6: f64 = (t1f1 - t1f5);let t1f7: f64 = (t1f6 / l.f11d0);(l.f10af, l.f10b6, l.f10b7, l.f10b8, l.f10b9, ) = (t1f7, (((((-l.f1237) - (l.f123d - (((t1f2 * l.f1078) * l.f10eb) + (t1f3 * l.f10f2)))) * l.f11d0) - (t1f6 * l.f11d4)) / (l.f11d0 * l.f11d0)), (((((l.f1224 - l.f1238) - (l.f123e - (t1f3 * l.f10f3))) * l.f11d0) - (t1f6 * l.f11d5)) / (l.f11d0 * l.f11d0)), (((l.f1225 - l.f1239) - (-(t1f3 * l.f10f4))) / l.f11d0), (((((l.f1226 - l.f123a) - (l.f123f - (t1f3 * l.f10f5))) * l.f11d0) - (t1f6 * l.f11d6)) / (l.f11d0 * l.f11d0)), );let t1f8: f64 = if l.f10af > 50.0 { 1.0 } else { 0.0 };l.f1f98 = t1f8;
        if (l.f1f98 != 0.0) {let t1f9: f64 = (l.f1196 * l.f10af);(l.f117e, l.f1185, l.f1186, l.f1187, l.f1188, ) = (t1f9, ((l.f119a * l.f10af) + (l.f1196 * l.f10b6)), ((l.f119b * l.f10af) + (l.f1196 * l.f10b7)), (l.f1196 * l.f10b8), ((l.f119c * l.f10af) + (l.f1196 * l.f10b9)), );}
        let t1fa: f64 = (-50.0);let t1fb: f64 = if l.f10af < t1fa { 1.0 } else { 0.0 };l.f1f9a = t1fb;
        if ((l.f1f98 == 0.0) && (l.f1f9a != 0.0)) {let t1fc: f64 = (l.f10af).exp();let t1fd: f64 = (l.f1196 * t1fc);(l.f117e, l.f1185, l.f1186, l.f1187, l.f1188, ) = (t1fd, ((l.f119a * t1fc) + (l.f1196 * (t1fc * l.f10b6))), ((l.f119b * t1fc) + (l.f1196 * (t1fc * l.f10b7))), (l.f1196 * (t1fc * l.f10b8)), ((l.f119c * t1fc) + (l.f1196 * (t1fc * l.f10b9))), );}
        if ((l.f1f98 == 0.0) && (l.f1f9a == 0.0)) {let t1fe: f64 = (l.f10af).exp();let t1ff: f64 = (1.0 + t1fe);let t200: f64 = (t1ff).ln();let t201: f64 = (l.f1196 * t200);(l.f117e, l.f1185, l.f1186, l.f1187, l.f1188, ) = (t201, ((l.f119a * t200) + (l.f1196 * ((t1fe * l.f10b6) / t1ff))), ((l.f119b * t200) + (l.f1196 * ((t1fe * l.f10b7) / t1ff))), (l.f1196 * ((t1fe * l.f10b8) / t1ff)), ((l.f119c * t200) + (l.f1196 * ((t1fe * l.f10b9) / t1ff))), );}
        let t202: f64 = (l.f1223 - l.f1124);let t203: f64 = (t202 / l.f1077);(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t203, ((((-l.f1128) * l.f1077) - (t202 * l.f1078)) / (l.f1077 * l.f1077)), ((l.f1224 - l.f1129) / l.f1077), ((l.f1225 - l.f112a) / l.f1077), ((l.f1226 - l.f112b) / l.f1077), );let t204: f64 = if l.f10bb > 50.0 { 1.0 } else { 0.0 };l.f1f9c = t204;
        if (l.f1f9c != 0.0) {(l.f10df, l.f10e6, l.f10e7, l.f10e8, l.f10e9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t205: f64 = (-50.0);let t206: f64 = if l.f10bb < t205 { 1.0 } else { 0.0 };l.f1f9e = t206;
        if ((l.f1f9c == 0.0) && (l.f1f9e != 0.0)) {(l.f10df, l.f10e6, l.f10e7, l.f10e8, l.f10e9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((l.f1f9c == 0.0) && (l.f1f9e == 0.0)) {let t207: f64 = (l.f10bb).exp();let t208: f64 = (1.0 + t207);let t209: f64 = (1.0 / t208);(l.f10df, l.f10e6, l.f10e7, l.f10e8, l.f10e9, ) = (t209, (-((t207 * l.f10c2) / (t208 * t208))), (-((t207 * l.f10c3) / (t208 * t208))), (-((t207 * l.f10c4) / (t208 * t208))), (-((t207 * l.f10c5) / (t208 * t208))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t20a: f64 = (l.f1228 - l.f120f);let t20b: f64 = (p.p51 * 0.1);let t20c: f64 = (t20b * l.f1077);let t20d: f64 = (t20c * l.f10df);let t20e: f64 = (l.f123c - t20d);let t20f: f64 = (t20a - t20e);let t210: f64 = (t20f / l.f11d0);(l.f109e, l.f10a5, l.f10a6, l.f10a7, l.f10a8, ) = (t210, (((((-l.f1216) - (l.f123d - (((t20b * l.f1078) * l.f10df) + (t20c * l.f10e6)))) * l.f11d0) - (t20f * l.f11d4)) / (l.f11d0 * l.f11d0)), (((((-l.f1217) - (l.f123e - (t20c * l.f10e7))) * l.f11d0) - (t20f * l.f11d5)) / (l.f11d0 * l.f11d0)), (((l.f1229 - l.f1218) - (-(t20c * l.f10e8))) / l.f11d0), (((((l.f122a - l.f1219) - (l.f123f - (t20c * l.f10e9))) * l.f11d0) - (t20f * l.f11d6)) / (l.f11d0 * l.f11d0)), );let t211: f64 = if l.f109e > 50.0 { 1.0 } else { 0.0 };l.f1fa0 = t211;
        if (l.f1fa0 != 0.0) {let t212: f64 = (l.f1196 * l.f109e);(l.f116c, l.f1173, l.f1174, l.f1175, l.f1176, ) = (t212, ((l.f119a * l.f109e) + (l.f1196 * l.f10a5)), ((l.f119b * l.f109e) + (l.f1196 * l.f10a6)), (l.f1196 * l.f10a7), ((l.f119c * l.f109e) + (l.f1196 * l.f10a8)), );}
        let t213: f64 = (-50.0);let t214: f64 = if l.f109e < t213 { 1.0 } else { 0.0 };l.f1fa2 = t214;
        if ((l.f1fa0 == 0.0) && (l.f1fa2 != 0.0)) {let t215: f64 = (l.f109e).exp();let t216: f64 = (l.f1196 * t215);(l.f116c, l.f1173, l.f1174, l.f1175, l.f1176, ) = (t216, ((l.f119a * t215) + (l.f1196 * (t215 * l.f10a5))), ((l.f119b * t215) + (l.f1196 * (t215 * l.f10a6))), (l.f1196 * (t215 * l.f10a7)), ((l.f119c * t215) + (l.f1196 * (t215 * l.f10a8))), );}
        if ((l.f1fa0 == 0.0) && (l.f1fa2 == 0.0)) {let t217: f64 = (l.f109e).exp();let t218: f64 = (1.0 + t217);let t219: f64 = (t218).ln();let t21a: f64 = (l.f1196 * t219);(l.f116c, l.f1173, l.f1174, l.f1175, l.f1176, ) = (t21a, ((l.f119a * t219) + (l.f1196 * ((t217 * l.f10a5) / t218))), ((l.f119b * t219) + (l.f1196 * ((t217 * l.f10a6) / t218))), (l.f1196 * ((t217 * l.f10a7) / t218)), ((l.f119c * t219) + (l.f1196 * ((t217 * l.f10a8) / t218))), );}
        let t21b: f64 = (l.f117e - l.f116c);let t21c: f64 = (t21b / l.f107d);(l.f1205, l.f1206, l.f1207, l.f1208, l.f1209, ) = (t21c, ((((l.f1185 - l.f1173) * l.f107d) - (t21b * l.f107e)) / (l.f107d * l.f107d)), ((l.f1186 - l.f1174) / l.f107d), ((l.f1187 - l.f1175) / l.f107d), ((l.f1188 - l.f1176) / l.f107d), );let t21d: f64 = (l.f1205 / l.f11de);(l.f1124, l.f1128, l.f1129, l.f112a, l.f112b, ) = (t21d, (((l.f1206 * l.f11de) - (l.f1205 * l.f11eb)) / (l.f11de * l.f11de)), (((l.f1207 * l.f11de) - (l.f1205 * l.f11ec)) / (l.f11de * l.f11de)), (((l.f1208 * l.f11de) - (l.f1205 * l.f11ed)) / (l.f11de * l.f11de)), (((l.f1209 * l.f11de) - (l.f1205 * l.f11ee)) / (l.f11de * l.f11de)), );
        let (t22a, t22b, t22c, t22d, t22e,) = {
    if (p.p52 != 0.0) {
        let t21e: f64 = (0.001 / p.p53);let t21f: f64 = (t21e * l.f1124);let t220: f64 = (t21f).tanh();let t221: f64 = (l.f1124 * t220);
        (t221, ((l.f1128 * t220) + (l.f1124 * ((t21e * l.f1128) / ((t21f).cosh() * (t21f).cosh())))), ((l.f1129 * t220) + (l.f1124 * ((t21e * l.f1129) / ((t21f).cosh() * (t21f).cosh())))), ((l.f112a * t220) + (l.f1124 * ((t21e * l.f112a) / ((t21f).cosh() * (t21f).cosh())))), ((l.f112b * t220) + (l.f1124 * ((t21e * l.f112b) / ((t21f).cosh() * (t21f).cosh())))),)
    } else {
        let (t225, t226, t227, t228, t229,) = {
            if (p.p52 == 0.0) {
                let t222: f64 = (l.f1124 * l.f1124);let t223: f64 = (t222 + p.p53);let t224: f64 = (t223).sqrt();
                (t224, (((l.f1128 * l.f1124) + (l.f1124 * l.f1128)) / (2.0 * t224)), (((l.f1129 * l.f1124) + (l.f1124 * l.f1129)) / (2.0 * t224)), (((l.f112a * l.f1124) + (l.f1124 * l.f112a)) / (2.0 * t224)), (((l.f112b * l.f1124) + (l.f1124 * l.f112b)) / (2.0 * t224)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t225, t226, t227, t228, t229,)
    }
};
        let t22f: f64 = (t22a).powf(l.f107b);let t230: f64 = (1.0 + t22f);let t231: f64 = (1.0 / l.f107b);let t232: f64 = (t230).powf(t231);let t233: f64 = (l.f1124 / t232);
        (l.f10f7, l.f10f8, l.f10f9, l.f10fa, l.f10fb, ) = (t233, (((l.f1128 * t232) - (l.f1124 * if 0.0 == 0.0 && ((t231) as f64).is_finite() && ((t231) as f64).fract() == 0.0 { if t231 == 0.0 { 0.0 } else { (t231 * ((t230).powf(t231 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t22a).powf(l.f107b - 1.0) * t22b)) } } else { (t22f * (l.f107b * (t22b / t22a))) })) } } else { (t232 * (t231 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t22a).powf(l.f107b - 1.0) * t22b)) } } else { (t22f * (l.f107b * (t22b / t22a))) } / t230))) })) / (t232 * t232)), (((l.f1129 * t232) - (l.f1124 * if 0.0 == 0.0 && ((t231) as f64).is_finite() && ((t231) as f64).fract() == 0.0 { if t231 == 0.0 { 0.0 } else { (t231 * ((t230).powf(t231 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t22a).powf(l.f107b - 1.0) * t22c)) } } else { (t22f * (l.f107b * (t22c / t22a))) })) } } else { (t232 * (t231 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t22a).powf(l.f107b - 1.0) * t22c)) } } else { (t22f * (l.f107b * (t22c / t22a))) } / t230))) })) / (t232 * t232)), (((l.f112a * t232) - (l.f1124 * if 0.0 == 0.0 && ((t231) as f64).is_finite() && ((t231) as f64).fract() == 0.0 { if t231 == 0.0 { 0.0 } else { (t231 * ((t230).powf(t231 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t22a).powf(l.f107b - 1.0) * t22d)) } } else { (t22f * (l.f107b * (t22d / t22a))) })) } } else { (t232 * (t231 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t22a).powf(l.f107b - 1.0) * t22d)) } } else { (t22f * (l.f107b * (t22d / t22a))) } / t230))) })) / (t232 * t232)), (((l.f112b * t232) - (l.f1124 * if 0.0 == 0.0 && ((t231) as f64).is_finite() && ((t231) as f64).fract() == 0.0 { if t231 == 0.0 { 0.0 } else { (t231 * ((t230).powf(t231 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t22a).powf(l.f107b - 1.0) * t22e)) } } else { (t22f * (l.f107b * (t22e / t22a))) })) } } else { (t232 * (t231 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t22a).powf(l.f107b - 1.0) * t22e)) } } else { (t22f * (l.f107b * (t22e / t22a))) } / t230))) })) / (t232 * t232)), );
        let t234: f64 = (l.f1253 * l.f10f7);(l.f121b, l.f121e, l.f121f, l.f1220, l.f1221, ) = (t234, ((l.f1254 * l.f10f7) + (l.f1253 * l.f10f8)), ((l.f1255 * l.f10f7) + (l.f1253 * l.f10f9)), ((l.f1256 * l.f10f7) + (l.f1253 * l.f10fa)), ((l.f1257 * l.f10f7) + (l.f1253 * l.f10fb)), );let t235: f64 = (l.f11d8 * l.f125b);let t236: f64 = (t235 * l.f1137);let t237: f64 = (t236 * 0.5);let t238: f64 = (l.f117e + l.f116c);let t239: f64 = (t237 * t238);let t23a: f64 = (t239 * l.f121b);let t23b: f64 = (t23a * l.f11ca);(l.f1109, l.f110e, l.f110f, l.f1110, l.f1111, l.f110a, l.f110b, l.f110c, l.f110d, ) = (t23b, ((((t237 * (l.f1185 + l.f1173)) * l.f121b) + (t239 * l.f121e)) * l.f11ca), ((((t237 * (l.f1186 + l.f1174)) * l.f121b) + (t239 * l.f121f)) * l.f11ca), ((((t237 * (l.f1187 + l.f1175)) * l.f121b) + (t239 * l.f1220)) * l.f11ca), ((((t237 * (l.f1188 + l.f1176)) * l.f121b) + (t239 * l.f1221)) * l.f11ca), (t23a * l.f11cb), (t23a * l.f11cc), (t23a * l.f11cd), (t23a * l.f11ce), );let t23c: f64 = (2.302585092994046 * l.f1139);let t23d: f64 = (l.f11c0 / t23c);(l.f112e, l.f112f, ) = (t23d, (-((l.f11c0 * (2.302585092994046 * l.f113a)) / (t23c * t23c))), );let t23e: f64 = (2.0 * l.f112e);let t23f: f64 = (t23e * l.f1139);(l.f11d1, l.f11d2, ) = (t23f, (((2.0 * l.f112f) * l.f1139) + (t23e * l.f113a)), );let t240: f64 = (l.f107d * l.f11d1);(l.f1197, l.f1198, ) = (t240, ((l.f107e * l.f11d1) + (l.f107d * l.f11d2)), );let t241: f64 = (p.p51 * l.f1077);let t242: f64 = (t241 / 2.0);let t243: f64 = (l.f1245 - t242);(l.f1125, l.f1126, ) = (t243, (l.f1246 - ((p.p51 * l.f1078) / 2.0)), );
        let (t259, t25a, t25b, t25c,) = {
    if (p.p52 != 0.0) {
        let t244: f64 = (l.f1228 + l.f1223);let t245: f64 = (l.f1228 - l.f1223);let t246: f64 = (0.001 / p.p53);let t247: f64 = (l.f1228 - l.f1223);let t248: f64 = (t246 * t247);let t249: f64 = (t248).tanh();let t24a: f64 = (t245 * t249);let t24b: f64 = (t244 + t24a);let t24c: f64 = (0.5 * t24b);
        (t24c, (0.5 * (l.f1224 + (((-l.f1224) * t249) + (t245 * ((t246 * (-l.f1224)) / ((t248).cosh() * (t248).cosh())))))), (0.5 * ((l.f1229 + l.f1225) + (((l.f1229 - l.f1225) * t249) + (t245 * ((t246 * (l.f1229 - l.f1225)) / ((t248).cosh() * (t248).cosh())))))), (0.5 * ((l.f122a + l.f1226) + (((l.f122a - l.f1226) * t249) + (t245 * ((t246 * (l.f122a - l.f1226)) / ((t248).cosh() * (t248).cosh())))))),)
    } else {
        let (t255, t256, t257, t258,) = {
            if (p.p52 == 0.0) {
                let t24d: f64 = (l.f1228 + l.f1223);let t24e: f64 = (l.f1228 - l.f1223);let t24f: f64 = (l.f1228 - l.f1223);let t250: f64 = (t24e * t24f);let t251: f64 = (t250 + p.p53);let t252: f64 = (t251).sqrt();let t253: f64 = (t24d + t252);let t254: f64 = (0.5 * t253);
                (t254, (0.5 * (l.f1224 + ((((-l.f1224) * t24f) + (t24e * (-l.f1224))) / (2.0 * t252)))), (0.5 * ((l.f1229 + l.f1225) + ((((l.f1229 - l.f1225) * t24f) + (t24e * (l.f1229 - l.f1225))) / (2.0 * t252)))), (0.5 * ((l.f122a + l.f1226) + ((((l.f122a - l.f1226) * t24f) + (t24e * (l.f122a - l.f1226))) / (2.0 * t252)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t255, t256, t257, t258,)
    }
};
        let t25d: f64 = (t259 - l.f1125);let t25e: f64 = (t25d / l.f1077);(l.f10bc, l.f10bd, l.f10be, l.f10bf, l.f10c0, ) = (t25e, ((((-l.f1126) * l.f1077) - (t25d * l.f1078)) / (l.f1077 * l.f1077)), (t25a / l.f1077), (t25b / l.f1077), (t25c / l.f1077), );let t25f: f64 = if l.f10bc > 50.0 { 1.0 } else { 0.0 };l.f1fa4 = t25f;
        if (l.f1fa4 != 0.0) {(l.f10d4, l.f10d5, l.f10d6, l.f10d7, l.f10d8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t260: f64 = (-50.0);let t261: f64 = if l.f10bc < t260 { 1.0 } else { 0.0 };l.f1fa6 = t261;
        if ((l.f1fa4 == 0.0) && (l.f1fa6 != 0.0)) {(l.f10d4, l.f10d5, l.f10d6, l.f10d7, l.f10d8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f1fa4 == 0.0) && (l.f1fa6 == 0.0)) {let t262: f64 = (l.f10bc).exp();let t263: f64 = (1.0 + t262);let t264: f64 = (1.0 / t263);(l.f10d4, l.f10d5, l.f10d6, l.f10d7, l.f10d8, ) = (t264, (-((t262 * l.f10bd) / (t263 * t263))), (-((t262 * l.f10be) / (t263 * t263))), (-((t262 * l.f10bf) / (t263 * t263))), (-((t262 * l.f10c0) / (t263 * t263))), );}
        let (t27a, t27b, t27c, t27d,) = {
    if (p.p52 != 0.0) {
        let t265: f64 = (l.f1228 + l.f1223);let t266: f64 = (l.f1228 - l.f1223);let t267: f64 = (0.001 / p.p53);let t268: f64 = (l.f1228 - l.f1223);let t269: f64 = (t267 * t268);let t26a: f64 = (t269).tanh();let t26b: f64 = (t266 * t26a);let t26c: f64 = (t265 + t26b);let t26d: f64 = (0.5 * t26c);
        (t26d, (0.5 * (l.f1224 + (((-l.f1224) * t26a) + (t266 * ((t267 * (-l.f1224)) / ((t269).cosh() * (t269).cosh())))))), (0.5 * ((l.f1229 + l.f1225) + (((l.f1229 - l.f1225) * t26a) + (t266 * ((t267 * (l.f1229 - l.f1225)) / ((t269).cosh() * (t269).cosh())))))), (0.5 * ((l.f122a + l.f1226) + (((l.f122a - l.f1226) * t26a) + (t266 * ((t267 * (l.f122a - l.f1226)) / ((t269).cosh() * (t269).cosh())))))),)
    } else {
        let (t276, t277, t278, t279,) = {
            if (p.p52 == 0.0) {
                let t26e: f64 = (l.f1228 + l.f1223);let t26f: f64 = (l.f1228 - l.f1223);let t270: f64 = (l.f1228 - l.f1223);let t271: f64 = (t26f * t270);let t272: f64 = (t271 + p.p53);let t273: f64 = (t272).sqrt();let t274: f64 = (t26e + t273);let t275: f64 = (0.5 * t274);
                (t275, (0.5 * (l.f1224 + ((((-l.f1224) * t270) + (t26f * (-l.f1224))) / (2.0 * t273)))), (0.5 * ((l.f1229 + l.f1225) + ((((l.f1229 - l.f1225) * t270) + (t26f * (l.f1229 - l.f1225))) / (2.0 * t273)))), (0.5 * ((l.f122a + l.f1226) + ((((l.f122a - l.f1226) * t270) + (t26f * (l.f122a - l.f1226))) / (2.0 * t273)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t276, t277, t278, t279,)
    }
};
        let t27e: f64 = (p.p51 * 0.1);let t27f: f64 = (t27e * l.f1077);let t280: f64 = (t27f * l.f10d4);let t281: f64 = (l.f1245 - t280);let t282: f64 = (t27a - t281);let t283: f64 = (t282 / l.f11d1);(l.f108d, l.f108e, l.f108f, l.f1090, l.f1091, ) = (t283, ((((-(l.f1246 - (((t27e * l.f1078) * l.f10d4) + (t27f * l.f10d5)))) * l.f11d1) - (t282 * l.f11d2)) / (l.f11d1 * l.f11d1)), ((t27b - (-(t27f * l.f10d6))) / l.f11d1), ((t27c - (-(t27f * l.f10d7))) / l.f11d1), ((t27d - (-(t27f * l.f10d8))) / l.f11d1), );let t284: f64 = if l.f108d > 50.0 { 1.0 } else { 0.0 };l.f1fa8 = t284;
        if (l.f1fa8 != 0.0) {let t285: f64 = (l.f1197 * l.f108d);(l.f118b, l.f118c, l.f118d, l.f118e, l.f118f, ) = (t285, ((l.f1198 * l.f108d) + (l.f1197 * l.f108e)), (l.f1197 * l.f108f), (l.f1197 * l.f1090), (l.f1197 * l.f1091), );}
        let t286: f64 = (-50.0);let t287: f64 = if l.f108d < t286 { 1.0 } else { 0.0 };l.f1faa = t287;
        if ((l.f1fa8 == 0.0) && (l.f1faa != 0.0)) {let t288: f64 = (l.f108d).exp();let t289: f64 = (l.f1197 * t288);(l.f118b, l.f118c, l.f118d, l.f118e, l.f118f, ) = (t289, ((l.f1198 * t288) + (l.f1197 * (t288 * l.f108e))), (l.f1197 * (t288 * l.f108f)), (l.f1197 * (t288 * l.f1090)), (l.f1197 * (t288 * l.f1091)), );}
        if ((l.f1fa8 == 0.0) && (l.f1faa == 0.0)) {let t28a: f64 = (l.f108d).exp();let t28b: f64 = (1.0 + t28a);let t28c: f64 = (t28b).ln();let t28d: f64 = (l.f1197 * t28c);(l.f118b, l.f118c, l.f118d, l.f118e, l.f118f, ) = (t28d, ((l.f1198 * t28c) + (l.f1197 * ((t28a * l.f108e) / t28b))), (l.f1197 * ((t28a * l.f108f) / t28b)), (l.f1197 * ((t28a * l.f1090) / t28b)), (l.f1197 * ((t28a * l.f1091) / t28b)), );}
        let t28e: f64 = (l.f1119 / l.f11c5);(l.f111c, l.f111d, ) = (t28e, (-((l.f1119 * l.f11c6) / (l.f11c5 * l.f11c5))), );let t28f: f64 = (l.f1259 * l.f11c8);let t290: f64 = (1.0 + t28f);let t291: f64 = (l.f1259 * l.f11c2);let t292: f64 = (1.0 + t291);let t293: f64 = (t290 / t292);let t294: f64 = (l.f121c * t293);(l.f124b, l.f124c, ) = (t294, (l.f121c * (-((t290 * (l.f1259 * l.f11c3)) / (t292 * t292)))), );let t295: f64 = (l.f124b * l.f1115);let t296: f64 = (t295 / l.f111c);(l.f11f1, l.f11f2, ) = (t296, ((((l.f124c * l.f1115) * l.f111c) - (t295 * l.f111d)) / (l.f111c * l.f111c)), );let t297: f64 = (2.0 * l.f118b);let t298: f64 = (t297 / l.f107d);let t299: f64 = (t298 / l.f11f1);let t29a: f64 = (1.0 + t299);let t29b: f64 = (t29a).sqrt();let t29c: f64 = (l.f11f1 * t29b);let t29d: f64 = (t29c - l.f11f1);(l.f11f5, l.f11f6, l.f11f7, l.f11f8, l.f11f9, ) = (t29d, (((l.f11f2 * t29b) + (l.f11f1 * ((((((((2.0 * l.f118c) * l.f107d) - (t297 * l.f107e)) / (l.f107d * l.f107d)) * l.f11f1) - (t298 * l.f11f2)) / (l.f11f1 * l.f11f1)) / (2.0 * t29b)))) - l.f11f2), (l.f11f1 * ((((2.0 * l.f118d) / l.f107d) / l.f11f1) / (2.0 * t29b))), (l.f11f1 * ((((2.0 * l.f118e) / l.f107d) / l.f11f1) / (2.0 * t29b))), (l.f11f1 * ((((2.0 * l.f118f) / l.f107d) / l.f11f1) / (2.0 * t29b))), );let t29e: f64 = (1.0 - l.f10d4);let t29f: f64 = (l.f11f5 * t29e);let t2a0: f64 = (l.f11d1 * l.f10d4);let t2a1: f64 = (t29f + t2a0);(l.f11e0, l.f11e1, l.f11e2, l.f11e3, l.f11e4, ) = (t2a1, (((l.f11f6 * t29e) + (l.f11f5 * (-l.f10d5))) + ((l.f11d2 * l.f10d4) + (l.f11d1 * l.f10d5))), (((l.f11f7 * t29e) + (l.f11f5 * (-l.f10d6))) + (l.f11d1 * l.f10d6)), (((l.f11f8 * t29e) + (l.f11f5 * (-l.f10d7))) + (l.f11d1 * l.f10d7)), (((l.f11f9 * t29e) + (l.f11f5 * (-l.f10d8))) + (l.f11d1 * l.f10d8)), );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let (t2be, t2bf, t2c0, t2c1, t2c2,) = {
    if (p.p52 != 0.0) {
        let t2a2: f64 = (l.f120b / l.f11e0);let t2a3: f64 = t2a2;let t2a4: f64 = (l.f120b / l.f11e0);let t2a5: f64 = (-t2a4);let t2a6: f64 = (0.001 / p.p53);let t2a7: f64 = (l.f120b / l.f11e0);let t2a8: f64 = (-t2a7);let t2a9: f64 = (t2a6 * t2a8);let t2aa: f64 = (t2a9).tanh();let t2ab: f64 = (t2a5 * t2aa);let t2ac: f64 = (t2a3 + t2ab);let t2ad: f64 = (0.5 * t2ac);
        (t2ad, (0.5 * ((-((l.f120b * l.f11e1) / (l.f11e0 * l.f11e0))) + (((-(-((l.f120b * l.f11e1) / (l.f11e0 * l.f11e0)))) * t2aa) + (t2a5 * ((t2a6 * (-(-((l.f120b * l.f11e1) / (l.f11e0 * l.f11e0))))) / ((t2a9).cosh() * (t2a9).cosh())))))), (0.5 * ((((l.f120c * l.f11e0) - (l.f120b * l.f11e2)) / (l.f11e0 * l.f11e0)) + (((-(((l.f120c * l.f11e0) - (l.f120b * l.f11e2)) / (l.f11e0 * l.f11e0))) * t2aa) + (t2a5 * ((t2a6 * (-(((l.f120c * l.f11e0) - (l.f120b * l.f11e2)) / (l.f11e0 * l.f11e0)))) / ((t2a9).cosh() * (t2a9).cosh())))))), (0.5 * ((-((l.f120b * l.f11e3) / (l.f11e0 * l.f11e0))) + (((-(-((l.f120b * l.f11e3) / (l.f11e0 * l.f11e0)))) * t2aa) + (t2a5 * ((t2a6 * (-(-((l.f120b * l.f11e3) / (l.f11e0 * l.f11e0))))) / ((t2a9).cosh() * (t2a9).cosh())))))), (0.5 * ((((l.f120d * l.f11e0) - (l.f120b * l.f11e4)) / (l.f11e0 * l.f11e0)) + (((-(((l.f120d * l.f11e0) - (l.f120b * l.f11e4)) / (l.f11e0 * l.f11e0))) * t2aa) + (t2a5 * ((t2a6 * (-(((l.f120d * l.f11e0) - (l.f120b * l.f11e4)) / (l.f11e0 * l.f11e0)))) / ((t2a9).cosh() * (t2a9).cosh())))))),)
    } else {
        let (t2b9, t2ba, t2bb, t2bc, t2bd,) = {
            if (p.p52 == 0.0) {
                let t2ae: f64 = (l.f120b / l.f11e0);let t2af: f64 = t2ae;let t2b0: f64 = (l.f120b / l.f11e0);let t2b1: f64 = (-t2b0);let t2b2: f64 = (l.f120b / l.f11e0);let t2b3: f64 = (-t2b2);let t2b4: f64 = (t2b1 * t2b3);let t2b5: f64 = (t2b4 + p.p53);let t2b6: f64 = (t2b5).sqrt();let t2b7: f64 = (t2af + t2b6);let t2b8: f64 = (0.5 * t2b7);
                (t2b8, (0.5 * ((-((l.f120b * l.f11e1) / (l.f11e0 * l.f11e0))) + ((((-(-((l.f120b * l.f11e1) / (l.f11e0 * l.f11e0)))) * t2b3) + (t2b1 * (-(-((l.f120b * l.f11e1) / (l.f11e0 * l.f11e0)))))) / (2.0 * t2b6)))), (0.5 * ((((l.f120c * l.f11e0) - (l.f120b * l.f11e2)) / (l.f11e0 * l.f11e0)) + ((((-(((l.f120c * l.f11e0) - (l.f120b * l.f11e2)) / (l.f11e0 * l.f11e0))) * t2b3) + (t2b1 * (-(((l.f120c * l.f11e0) - (l.f120b * l.f11e2)) / (l.f11e0 * l.f11e0))))) / (2.0 * t2b6)))), (0.5 * ((-((l.f120b * l.f11e3) / (l.f11e0 * l.f11e0))) + ((((-(-((l.f120b * l.f11e3) / (l.f11e0 * l.f11e0)))) * t2b3) + (t2b1 * (-(-((l.f120b * l.f11e3) / (l.f11e0 * l.f11e0)))))) / (2.0 * t2b6)))), (0.5 * ((((l.f120d * l.f11e0) - (l.f120b * l.f11e4)) / (l.f11e0 * l.f11e0)) + ((((-(((l.f120d * l.f11e0) - (l.f120b * l.f11e4)) / (l.f11e0 * l.f11e0))) * t2b3) + (t2b1 * (-(((l.f120d * l.f11e0) - (l.f120b * l.f11e4)) / (l.f11e0 * l.f11e0))))) / (2.0 * t2b6)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2b9, t2ba, t2bb, t2bc, t2bd,)
    }
};
        let t2c3: f64 = (t2be).powf(l.f107b);let t2c4: f64 = (1.0 + t2c3);let t2c5: f64 = (1.0 / l.f107b);let t2c6: f64 = (t2c4).powf(t2c5);let t2c7: f64 = (1.0 / t2c6);
        (l.f10fe, l.f10ff, l.f1100, l.f1101, l.f1102, ) = (t2c7, (-(if 0.0 == 0.0 && ((t2c5) as f64).is_finite() && ((t2c5) as f64).fract() == 0.0 { if t2c5 == 0.0 { 0.0 } else { (t2c5 * ((t2c4).powf(t2c5 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2be).powf(l.f107b - 1.0) * t2bf)) } } else { (t2c3 * (l.f107b * (t2bf / t2be))) })) } } else { (t2c6 * (t2c5 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2be).powf(l.f107b - 1.0) * t2bf)) } } else { (t2c3 * (l.f107b * (t2bf / t2be))) } / t2c4))) } / (t2c6 * t2c6))), (-(if 0.0 == 0.0 && ((t2c5) as f64).is_finite() && ((t2c5) as f64).fract() == 0.0 { if t2c5 == 0.0 { 0.0 } else { (t2c5 * ((t2c4).powf(t2c5 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2be).powf(l.f107b - 1.0) * t2c0)) } } else { (t2c3 * (l.f107b * (t2c0 / t2be))) })) } } else { (t2c6 * (t2c5 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2be).powf(l.f107b - 1.0) * t2c0)) } } else { (t2c3 * (l.f107b * (t2c0 / t2be))) } / t2c4))) } / (t2c6 * t2c6))), (-(if 0.0 == 0.0 && ((t2c5) as f64).is_finite() && ((t2c5) as f64).fract() == 0.0 { if t2c5 == 0.0 { 0.0 } else { (t2c5 * ((t2c4).powf(t2c5 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2be).powf(l.f107b - 1.0) * t2c1)) } } else { (t2c3 * (l.f107b * (t2c1 / t2be))) })) } } else { (t2c6 * (t2c5 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2be).powf(l.f107b - 1.0) * t2c1)) } } else { (t2c3 * (l.f107b * (t2c1 / t2be))) } / t2c4))) } / (t2c6 * t2c6))), (-(if 0.0 == 0.0 && ((t2c5) as f64).is_finite() && ((t2c5) as f64).fract() == 0.0 { if t2c5 == 0.0 { 0.0 } else { (t2c5 * ((t2c4).powf(t2c5 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2be).powf(l.f107b - 1.0) * t2c2)) } } else { (t2c3 * (l.f107b * (t2c2 / t2be))) })) } } else { (t2c6 * (t2c5 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2be).powf(l.f107b - 1.0) * t2c2)) } } else { (t2c3 * (l.f107b * (t2c2 / t2be))) } / t2c4))) } / (t2c6 * t2c6))), );let t2c8: f64 = (l.f120b * l.f10fe);
        (l.f1210, l.f1211, l.f1212, l.f1213, l.f1214, ) = (t2c8, (l.f120b * l.f10ff), ((l.f120c * l.f10fe) + (l.f120b * l.f1100)), (l.f120b * l.f1101), ((l.f120d * l.f10fe) + (l.f120b * l.f1102)), );
        let (t2eb, t2ec, t2ed, t2ee, t2ef,) = {
    if (p.p52 != 0.0) {
        let t2c9: f64 = (-l.f120b);let t2ca: f64 = (t2c9 / l.f11e0);let t2cb: f64 = t2ca;let t2cc: f64 = (-l.f120b);let t2cd: f64 = (t2cc / l.f11e0);let t2ce: f64 = (-t2cd);let t2cf: f64 = (0.001 / p.p53);let t2d0: f64 = (-l.f120b);let t2d1: f64 = (t2d0 / l.f11e0);let t2d2: f64 = (-t2d1);let t2d3: f64 = (t2cf * t2d2);let t2d4: f64 = (t2d3).tanh();let t2d5: f64 = (t2ce * t2d4);let t2d6: f64 = (t2cb + t2d5);let t2d7: f64 = (0.5 * t2d6);
        (t2d7, (0.5 * ((-((t2c9 * l.f11e1) / (l.f11e0 * l.f11e0))) + (((-(-((t2cc * l.f11e1) / (l.f11e0 * l.f11e0)))) * t2d4) + (t2ce * ((t2cf * (-(-((t2d0 * l.f11e1) / (l.f11e0 * l.f11e0))))) / ((t2d3).cosh() * (t2d3).cosh())))))), (0.5 * (((((-l.f120c) * l.f11e0) - (t2c9 * l.f11e2)) / (l.f11e0 * l.f11e0)) + (((-((((-l.f120c) * l.f11e0) - (t2cc * l.f11e2)) / (l.f11e0 * l.f11e0))) * t2d4) + (t2ce * ((t2cf * (-((((-l.f120c) * l.f11e0) - (t2d0 * l.f11e2)) / (l.f11e0 * l.f11e0)))) / ((t2d3).cosh() * (t2d3).cosh())))))), (0.5 * ((-((t2c9 * l.f11e3) / (l.f11e0 * l.f11e0))) + (((-(-((t2cc * l.f11e3) / (l.f11e0 * l.f11e0)))) * t2d4) + (t2ce * ((t2cf * (-(-((t2d0 * l.f11e3) / (l.f11e0 * l.f11e0))))) / ((t2d3).cosh() * (t2d3).cosh())))))), (0.5 * (((((-l.f120d) * l.f11e0) - (t2c9 * l.f11e4)) / (l.f11e0 * l.f11e0)) + (((-((((-l.f120d) * l.f11e0) - (t2cc * l.f11e4)) / (l.f11e0 * l.f11e0))) * t2d4) + (t2ce * ((t2cf * (-((((-l.f120d) * l.f11e0) - (t2d0 * l.f11e4)) / (l.f11e0 * l.f11e0)))) / ((t2d3).cosh() * (t2d3).cosh())))))),)
    } else {
        let (t2e6, t2e7, t2e8, t2e9, t2ea,) = {
            if (p.p52 == 0.0) {
                let t2d8: f64 = (-l.f120b);let t2d9: f64 = (t2d8 / l.f11e0);let t2da: f64 = t2d9;let t2db: f64 = (-l.f120b);let t2dc: f64 = (t2db / l.f11e0);let t2dd: f64 = (-t2dc);let t2de: f64 = (-l.f120b);let t2df: f64 = (t2de / l.f11e0);let t2e0: f64 = (-t2df);let t2e1: f64 = (t2dd * t2e0);let t2e2: f64 = (t2e1 + p.p53);let t2e3: f64 = (t2e2).sqrt();let t2e4: f64 = (t2da + t2e3);let t2e5: f64 = (0.5 * t2e4);
                (t2e5, (0.5 * ((-((t2d8 * l.f11e1) / (l.f11e0 * l.f11e0))) + ((((-(-((t2db * l.f11e1) / (l.f11e0 * l.f11e0)))) * t2e0) + (t2dd * (-(-((t2de * l.f11e1) / (l.f11e0 * l.f11e0)))))) / (2.0 * t2e3)))), (0.5 * (((((-l.f120c) * l.f11e0) - (t2d8 * l.f11e2)) / (l.f11e0 * l.f11e0)) + ((((-((((-l.f120c) * l.f11e0) - (t2db * l.f11e2)) / (l.f11e0 * l.f11e0))) * t2e0) + (t2dd * (-((((-l.f120c) * l.f11e0) - (t2de * l.f11e2)) / (l.f11e0 * l.f11e0))))) / (2.0 * t2e3)))), (0.5 * ((-((t2d8 * l.f11e3) / (l.f11e0 * l.f11e0))) + ((((-(-((t2db * l.f11e3) / (l.f11e0 * l.f11e0)))) * t2e0) + (t2dd * (-(-((t2de * l.f11e3) / (l.f11e0 * l.f11e0)))))) / (2.0 * t2e3)))), (0.5 * (((((-l.f120d) * l.f11e0) - (t2d8 * l.f11e4)) / (l.f11e0 * l.f11e0)) + ((((-((((-l.f120d) * l.f11e0) - (t2db * l.f11e4)) / (l.f11e0 * l.f11e0))) * t2e0) + (t2dd * (-((((-l.f120d) * l.f11e0) - (t2de * l.f11e4)) / (l.f11e0 * l.f11e0))))) / (2.0 * t2e3)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2e6, t2e7, t2e8, t2e9, t2ea,)
    }
};
        let t2f0: f64 = (t2eb).powf(l.f107b);let t2f1: f64 = (1.0 + t2f0);let t2f2: f64 = (1.0 / l.f107b);let t2f3: f64 = (t2f1).powf(t2f2);let t2f4: f64 = (1.0 / t2f3);
        (l.f10c8, l.f10c9, l.f10ca, l.f10cb, l.f10cc, ) = (t2f4, (-(if 0.0 == 0.0 && ((t2f2) as f64).is_finite() && ((t2f2) as f64).fract() == 0.0 { if t2f2 == 0.0 { 0.0 } else { (t2f2 * ((t2f1).powf(t2f2 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2eb).powf(l.f107b - 1.0) * t2ec)) } } else { (t2f0 * (l.f107b * (t2ec / t2eb))) })) } } else { (t2f3 * (t2f2 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2eb).powf(l.f107b - 1.0) * t2ec)) } } else { (t2f0 * (l.f107b * (t2ec / t2eb))) } / t2f1))) } / (t2f3 * t2f3))), (-(if 0.0 == 0.0 && ((t2f2) as f64).is_finite() && ((t2f2) as f64).fract() == 0.0 { if t2f2 == 0.0 { 0.0 } else { (t2f2 * ((t2f1).powf(t2f2 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2eb).powf(l.f107b - 1.0) * t2ed)) } } else { (t2f0 * (l.f107b * (t2ed / t2eb))) })) } } else { (t2f3 * (t2f2 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2eb).powf(l.f107b - 1.0) * t2ed)) } } else { (t2f0 * (l.f107b * (t2ed / t2eb))) } / t2f1))) } / (t2f3 * t2f3))), (-(if 0.0 == 0.0 && ((t2f2) as f64).is_finite() && ((t2f2) as f64).fract() == 0.0 { if t2f2 == 0.0 { 0.0 } else { (t2f2 * ((t2f1).powf(t2f2 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2eb).powf(l.f107b - 1.0) * t2ee)) } } else { (t2f0 * (l.f107b * (t2ee / t2eb))) })) } } else { (t2f3 * (t2f2 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2eb).powf(l.f107b - 1.0) * t2ee)) } } else { (t2f0 * (l.f107b * (t2ee / t2eb))) } / t2f1))) } / (t2f3 * t2f3))), (-(if 0.0 == 0.0 && ((t2f2) as f64).is_finite() && ((t2f2) as f64).fract() == 0.0 { if t2f2 == 0.0 { 0.0 } else { (t2f2 * ((t2f1).powf(t2f2 - 1.0) * if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2eb).powf(l.f107b - 1.0) * t2ef)) } } else { (t2f0 * (l.f107b * (t2ef / t2eb))) })) } } else { (t2f3 * (t2f2 * (if 0.0 == 0.0 && ((l.f107b) as f64).is_finite() && ((l.f107b) as f64).fract() == 0.0 { if l.f107b == 0.0 { 0.0 } else { (l.f107b * ((t2eb).powf(l.f107b - 1.0) * t2ef)) } } else { (t2f0 * (l.f107b * (t2ef / t2eb))) } / t2f1))) } / (t2f3 * t2f3))), );let t2f5: f64 = (-l.f120b);let t2f6: f64 = (t2f5 * l.f10c8);
        (l.f1231, l.f1232, l.f1233, l.f1234, l.f1235, ) = (t2f6, (t2f5 * l.f10c9), (((-l.f120c) * l.f10c8) + (t2f5 * l.f10ca)), (t2f5 * l.f10cb), (((-l.f120d) * l.f10c8) + (t2f5 * l.f10cc)), );let t2f7: f64 = (l.f1228 - l.f1125);let t2f8: f64 = (t2f7 / l.f1077);(l.f10bc, l.f10bd, l.f10be, l.f10bf, l.f10c0, ) = (t2f8, ((((-l.f1126) * l.f1077) - (t2f7 * l.f1078)) / (l.f1077 * l.f1077)), 0.0, (l.f1229 / l.f1077), (l.f122a / l.f1077), );let t2f9: f64 = if l.f10bc > 50.0 { 1.0 } else { 0.0 };l.f1fb0 = t2f9;
        if (l.f1fb0 != 0.0) {(l.f10ec, l.f10ed, l.f10ee, l.f10ef, l.f10f0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t2fa: f64 = (-50.0);let t2fb: f64 = if l.f10bc < t2fa { 1.0 } else { 0.0 };l.f1fb2 = t2fb;
        if ((l.f1fb0 == 0.0) && (l.f1fb2 != 0.0)) {(l.f10ec, l.f10ed, l.f10ee, l.f10ef, l.f10f0, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((l.f1fb0 == 0.0) && (l.f1fb2 == 0.0)) {let t2fc: f64 = (l.f10bc).exp();let t2fd: f64 = (1.0 + t2fc);let t2fe: f64 = (1.0 / t2fd);(l.f10ec, l.f10ed, l.f10ee, l.f10ef, l.f10f0, ) = (t2fe, (-((t2fc * l.f10bd) / (t2fd * t2fd))), (-((t2fc * l.f10be) / (t2fd * t2fd))), (-((t2fc * l.f10bf) / (t2fd * t2fd))), (-((t2fc * l.f10c0) / (t2fd * t2fd))), );}
        let t2ff: f64 = (l.f1223 - l.f1231);let t300: f64 = (p.p51 * 0.1);let t301: f64 = (t300 * l.f1077);let t302: f64 = (t301 * l.f10ec);let t303: f64 = (l.f1245 - t302);let t304: f64 = (t2ff - t303);let t305: f64 = (t304 / l.f11d1);(l.f10b0, l.f10b1, l.f10b2, l.f10b3, l.f10b4, ) = (t305, (((((-l.f1232) - (l.f1246 - (((t300 * l.f1078) * l.f10ec) + (t301 * l.f10ed)))) * l.f11d1) - (t304 * l.f11d2)) / (l.f11d1 * l.f11d1)), (((l.f1224 - l.f1233) - (-(t301 * l.f10ee))) / l.f11d1), (((l.f1225 - l.f1234) - (-(t301 * l.f10ef))) / l.f11d1), (((l.f1226 - l.f1235) - (-(t301 * l.f10f0))) / l.f11d1), );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_107(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t306: f64 = if l.f10b0 > 50.0 { 1.0 } else { 0.0 };l.f1fb4 = t306;
        if (l.f1fb4 != 0.0) {let t307: f64 = (l.f1197 * l.f10b0);(l.f117f, l.f1180, l.f1181, l.f1182, l.f1183, ) = (t307, ((l.f1198 * l.f10b0) + (l.f1197 * l.f10b1)), (l.f1197 * l.f10b2), (l.f1197 * l.f10b3), (l.f1197 * l.f10b4), );}
        let t308: f64 = (-50.0);let t309: f64 = if l.f10b0 < t308 { 1.0 } else { 0.0 };l.f1fb6 = t309;
        if ((l.f1fb4 == 0.0) && (l.f1fb6 != 0.0)) {let t30a: f64 = (l.f10b0).exp();let t30b: f64 = (l.f1197 * t30a);(l.f117f, l.f1180, l.f1181, l.f1182, l.f1183, ) = (t30b, ((l.f1198 * t30a) + (l.f1197 * (t30a * l.f10b1))), (l.f1197 * (t30a * l.f10b2)), (l.f1197 * (t30a * l.f10b3)), (l.f1197 * (t30a * l.f10b4)), );}
        if ((l.f1fb4 == 0.0) && (l.f1fb6 == 0.0)) {let t30c: f64 = (l.f10b0).exp();let t30d: f64 = (1.0 + t30c);let t30e: f64 = (t30d).ln();let t30f: f64 = (l.f1197 * t30e);(l.f117f, l.f1180, l.f1181, l.f1182, l.f1183, ) = (t30f, ((l.f1198 * t30e) + (l.f1197 * ((t30c * l.f10b1) / t30d))), (l.f1197 * ((t30c * l.f10b2) / t30d)), (l.f1197 * ((t30c * l.f10b3) / t30d)), (l.f1197 * ((t30c * l.f10b4) / t30d)), );}
        let t310: f64 = (l.f1223 - l.f1125);let t311: f64 = (t310 / l.f1077);(l.f10bc, l.f10bd, l.f10be, l.f10bf, l.f10c0, ) = (t311, ((((-l.f1126) * l.f1077) - (t310 * l.f1078)) / (l.f1077 * l.f1077)), (l.f1224 / l.f1077), (l.f1225 / l.f1077), (l.f1226 / l.f1077), );let t312: f64 = if l.f10bc > 50.0 { 1.0 } else { 0.0 };l.f1fb8 = t312;
        if (l.f1fb8 != 0.0) {(l.f10e0, l.f10e1, l.f10e2, l.f10e3, l.f10e4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t313: f64 = (-50.0);let t314: f64 = if l.f10bc < t313 { 1.0 } else { 0.0 };l.f1fba = t314;
        if ((l.f1fb8 == 0.0) && (l.f1fba != 0.0)) {(l.f10e0, l.f10e1, l.f10e2, l.f10e3, l.f10e4, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((l.f1fb8 == 0.0) && (l.f1fba == 0.0)) {let t315: f64 = (l.f10bc).exp();let t316: f64 = (1.0 + t315);let t317: f64 = (1.0 / t316);(l.f10e0, l.f10e1, l.f10e2, l.f10e3, l.f10e4, ) = (t317, (-((t315 * l.f10bd) / (t316 * t316))), (-((t315 * l.f10be) / (t316 * t316))), (-((t315 * l.f10bf) / (t316 * t316))), (-((t315 * l.f10c0) / (t316 * t316))), );}
        let t318: f64 = (l.f1228 - l.f1210);let t319: f64 = (p.p51 * 0.1);let t31a: f64 = (t319 * l.f1077);let t31b: f64 = (t31a * l.f10e0);let t31c: f64 = (l.f1245 - t31b);let t31d: f64 = (t318 - t31c);let t31e: f64 = (t31d / l.f11d1);(l.f109f, l.f10a0, l.f10a1, l.f10a2, l.f10a3, ) = (t31e, (((((-l.f1211) - (l.f1246 - (((t319 * l.f1078) * l.f10e0) + (t31a * l.f10e1)))) * l.f11d1) - (t31d * l.f11d2)) / (l.f11d1 * l.f11d1)), (((-l.f1212) - (-(t31a * l.f10e2))) / l.f11d1), (((l.f1229 - l.f1213) - (-(t31a * l.f10e3))) / l.f11d1), (((l.f122a - l.f1214) - (-(t31a * l.f10e4))) / l.f11d1), );let t31f: f64 = if l.f109f > 50.0 { 1.0 } else { 0.0 };l.f1fbc = t31f;
        if (l.f1fbc != 0.0) {let t320: f64 = (l.f1197 * l.f109f);(l.f116d, l.f116e, l.f116f, l.f1170, l.f1171, ) = (t320, ((l.f1198 * l.f109f) + (l.f1197 * l.f10a0)), (l.f1197 * l.f10a1), (l.f1197 * l.f10a2), (l.f1197 * l.f10a3), );}
        let t321: f64 = (-50.0);let t322: f64 = if l.f109f < t321 { 1.0 } else { 0.0 };l.f1fbe = t322;
        if ((l.f1fbc == 0.0) && (l.f1fbe != 0.0)) {let t323: f64 = (l.f109f).exp();let t324: f64 = (l.f1197 * t323);(l.f116d, l.f116e, l.f116f, l.f1170, l.f1171, ) = (t324, ((l.f1198 * t323) + (l.f1197 * (t323 * l.f10a0))), (l.f1197 * (t323 * l.f10a1)), (l.f1197 * (t323 * l.f10a2)), (l.f1197 * (t323 * l.f10a3)), );}
        if ((l.f1fbc == 0.0) && (l.f1fbe == 0.0)) {let t325: f64 = (l.f109f).exp();let t326: f64 = (1.0 + t325);let t327: f64 = (t326).ln();let t328: f64 = (l.f1197 * t327);(l.f116d, l.f116e, l.f116f, l.f1170, l.f1171, ) = (t328, ((l.f1198 * t327) + (l.f1197 * ((t325 * l.f10a0) / t326))), (l.f1197 * ((t325 * l.f10a1) / t326)), (l.f1197 * ((t325 * l.f10a2) / t326)), (l.f1197 * ((t325 * l.f10a3) / t326)), );}
        let t329: f64 = (l.f117f * l.f117f);let t32a: f64 = (t329 + 1e-38);(l.f119f, l.f11a0, l.f11a1, l.f11a2, l.f11a3, ) = (t32a, ((l.f1180 * l.f117f) + (l.f117f * l.f1180)), ((l.f1181 * l.f117f) + (l.f117f * l.f1181)), ((l.f1182 * l.f117f) + (l.f117f * l.f1182)), ((l.f1183 * l.f117f) + (l.f117f * l.f1183)), );let t32b: f64 = (l.f119f * l.f117f);let t32c: f64 = (t32b + 1e-57);(l.f11a5, l.f11a6, l.f11a7, l.f11a8, l.f11a9, ) = (t32c, ((l.f11a0 * l.f117f) + (l.f119f * l.f1180)), ((l.f11a1 * l.f117f) + (l.f119f * l.f1181)), ((l.f11a2 * l.f117f) + (l.f119f * l.f1182)), ((l.f11a3 * l.f117f) + (l.f119f * l.f1183)), );let t32d: f64 = (l.f116d * l.f116d);let t32e: f64 = (t32d + 1e-38);(l.f1145, l.f1146, l.f1147, l.f1148, l.f1149, ) = (t32e, ((l.f116e * l.f116d) + (l.f116d * l.f116e)), ((l.f116f * l.f116d) + (l.f116d * l.f116f)), ((l.f1170 * l.f116d) + (l.f116d * l.f1170)), ((l.f1171 * l.f116d) + (l.f116d * l.f1171)), );let t32f: f64 = (l.f1145 * l.f116d);let t330: f64 = (t32f + 1e-57);(l.f114b, l.f114c, l.f114d, l.f114e, l.f114f, ) = (t330, ((l.f1146 * l.f116d) + (l.f1145 * l.f116e)), ((l.f1147 * l.f116d) + (l.f1145 * l.f116f)), ((l.f1148 * l.f116d) + (l.f1145 * l.f1170)), ((l.f1149 * l.f116d) + (l.f1145 * l.f1171)), );let t331: f64 = (l.f117f * l.f116d);let t332: f64 = (t331 + 1e-38);(l.f11b0, l.f11b1, l.f11b2, l.f11b3, l.f11b4, ) = (t332, ((l.f1180 * l.f116d) + (l.f117f * l.f116e)), ((l.f1181 * l.f116d) + (l.f117f * l.f116f)), ((l.f1182 * l.f116d) + (l.f117f * l.f1170)), ((l.f1183 * l.f116d) + (l.f117f * l.f1171)), );let t333: f64 = (2.0 / 3.0);let t334: f64 = (l.f119f + l.f1145);let t335: f64 = (t334 + l.f11b0);let t336: f64 = (t333 * t335);let t337: f64 = (l.f117f + l.f116d);let t338: f64 = (t337 + 2e-19);let t339: f64 = (t336 / t338);(l.f1178, l.f1179, l.f117a, l.f117b, l.f117c, ) = (t339, ((((t333 * ((l.f11a0 + l.f1146) + l.f11b1)) * t338) - (t336 * (l.f1180 + l.f116e))) / (t338 * t338)), ((((t333 * ((l.f11a1 + l.f1147) + l.f11b2)) * t338) - (t336 * (l.f1181 + l.f116f))) / (t338 * t338)), ((((t333 * ((l.f11a2 + l.f1148) + l.f11b3)) * t338) - (t336 * (l.f1182 + l.f1170))) / (t338 * t338)), ((((t333 * ((l.f11a3 + l.f1149) + l.f11b4)) * t338) - (t336 * (l.f1183 + l.f1171))) / (t338 * t338)), );let t33a: f64 = (2.0 * l.f11a5);let t33b: f64 = (3.0 * l.f114b);let t33c: f64 = (t33a + t33b);let t33d: f64 = (4.0 * l.f119f);let t33e: f64 = (t33d * l.f116d);let t33f: f64 = (t33c + t33e);let t340: f64 = (6.0 * l.f1145);let t341: f64 = (t340 * l.f117f);let t342: f64 = (t33f + t341);let t343: f64 = (2.0 * t342);let t344: f64 = (l.f119f + l.f1145);let t345: f64 = (2.0 * l.f11b0);let t346: f64 = (t344 + t345);let t347: f64 = (15.0 * t346);let t348: f64 = (t343 / t347);
        (l.f113f, l.f1140, l.f1141, l.f1142, l.f1143, ) = (t348, ((((2.0 * ((((2.0 * l.f11a6) + (3.0 * l.f114c)) + (((4.0 * l.f11a0) * l.f116d) + (t33d * l.f116e))) + (((6.0 * l.f1146) * l.f117f) + (t340 * l.f1180)))) * t347) - (t343 * (15.0 * ((l.f11a0 + l.f1146) + (2.0 * l.f11b1))))) / (t347 * t347)), ((((2.0 * ((((2.0 * l.f11a7) + (3.0 * l.f114d)) + (((4.0 * l.f11a1) * l.f116d) + (t33d * l.f116f))) + (((6.0 * l.f1147) * l.f117f) + (t340 * l.f1181)))) * t347) - (t343 * (15.0 * ((l.f11a1 + l.f1147) + (2.0 * l.f11b2))))) / (t347 * t347)), ((((2.0 * ((((2.0 * l.f11a8) + (3.0 * l.f114e)) + (((4.0 * l.f11a2) * l.f116d) + (t33d * l.f1170))) + (((6.0 * l.f1148) * l.f117f) + (t340 * l.f1182)))) * t347) - (t343 * (15.0 * ((l.f11a2 + l.f1148) + (2.0 * l.f11b3))))) / (t347 * t347)), ((((2.0 * ((((2.0 * l.f11a9) + (3.0 * l.f114f)) + (((4.0 * l.f11a3) * l.f116d) + (t33d * l.f1171))) + (((6.0 * l.f1149) * l.f117f) + (t340 * l.f1183)))) * t347) - (t343 * (15.0 * ((l.f11a3 + l.f1149) + (2.0 * l.f11b4))))) / (t347 * t347)), );let t349: f64 = (l.f1178 - l.f113f);(l.f119e, l.f11ab, l.f11ac, l.f11ad, l.f11ae, ) = (t349, (l.f1179 - l.f1140), (l.f117a - l.f1141), (l.f117b - l.f1142), (l.f117c - l.f1143), );(l.f113e, l.f1151, l.f1152, l.f1153, l.f1154, ) = (l.f113f, l.f1140, l.f1141, l.f1142, l.f1143, );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_108(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv13 = ctx.node_voltage(nodes[13]);let nv29 = ctx.node_voltage(nodes[29]);let t34a: f64 = (l.f125b * l.f1137);let t34b: f64 = (t34a * l.f1115);let t34c: f64 = (t34b * l.f11d8);let t34d: f64 = (t34c * l.f119e);let t34e: f64 = (t34d * l.f11ca);(l.f1162, l.f1167, l.f1168, l.f1169, l.f116a, l.f1163, l.f1164, l.f1165, l.f1166, ) = (t34e, ((t34c * l.f11ab) * l.f11ca), ((t34c * l.f11ac) * l.f11ca), ((t34c * l.f11ad) * l.f11ca), ((t34c * l.f11ae) * l.f11ca), (t34d * l.f11cb), (t34d * l.f11cc), (t34d * l.f11cd), (t34d * l.f11ce), );let t34f: f64 = (l.f125b * l.f1137);let t350: f64 = (t34f * l.f1115);let t351: f64 = (t350 * l.f11d8);let t352: f64 = (t351 * l.f113e);let t353: f64 = (t352 * l.f11ca);(l.f1156, l.f115b, l.f115c, l.f115d, l.f115e, l.f1157, l.f1158, l.f1159, l.f115a, ) = (t353, ((t351 * l.f1151) * l.f11ca), ((t351 * l.f1152) * l.f11ca), ((t351 * l.f1153) * l.f11ca), ((t351 * l.f1154) * l.f11ca), (t352 * l.f11cb), (t352 * l.f11cc), (t352 * l.f11cd), (t352 * l.f11ce), );let t354: f64 = if l.f113c == 1.0 { 1.0 } else { 0.0 };l.f1fc0 = t354;
        if (l.f1fc0 != 0.0) {let t355: f64 = (p.p51 * 0.5);let t356: f64 = (t355 * l.f1077);let t357: f64 = (l.f1245 - t356);let t358: f64 = (l.f11dc - t357);let t359: f64 = (t358 / l.f11d1);(l.f109b, l.f109c, ) = (t359, ((((-(l.f1246 - (t355 * l.f1078))) * l.f11d1) - (t358 * l.f11d2)) / (l.f11d1 * l.f11d1)), );}
        let t35a: f64 = if l.f109b > 50.0 { 1.0 } else { 0.0 };l.f1fc2 = t35a;
        if ((l.f1fc0 != 0.0) && (l.f1fc2 != 0.0)) {(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (l.f109b, l.f109c, 0.0, 0.0, 0.0, );}
        let t35b: f64 = (-50.0);let t35c: f64 = if l.f109b < t35b { 1.0 } else { 0.0 };l.f1fc6 = t35c;
        if (((l.f1fc0 != 0.0) && (l.f1fc2 == 0.0)) && (l.f1fc6 != 0.0)) {let t35d: f64 = (l.f109b).exp();(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t35d, (t35d * l.f109c), 0.0, 0.0, 0.0, );}
        if (((l.f1fc0 != 0.0) && (l.f1fc2 == 0.0)) && (l.f1fc6 == 0.0)) {let t35e: f64 = (l.f109b).exp();let t35f: f64 = (1.0 + t35e);let t360: f64 = (t35f).ln();(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t360, ((t35e * l.f109c) / t35f), 0.0, 0.0, 0.0, );}
        if (l.f1fc0 != 0.0) {let t361: f64 = (p.p51 * 0.5);let t362: f64 = (t361 * l.f1077);let t363: f64 = (l.f1245 - t362);let t364: f64 = (l.f11da - t363);let t365: f64 = (t364 / l.f11d1);(l.f1098, l.f1099, ) = (t365, ((((-(l.f1246 - (t361 * l.f1078))) * l.f11d1) - (t364 * l.f11d2)) / (l.f11d1 * l.f11d1)), );}
        let t366: f64 = if l.f1098 > 50.0 { 1.0 } else { 0.0 };l.f1fc8 = t366;
        if ((l.f1fc0 != 0.0) && (l.f1fc8 != 0.0)) {(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (l.f1098, l.f1099, 0.0, 0.0, 0.0, );}
        let t367: f64 = (-50.0);let t368: f64 = if l.f1098 < t367 { 1.0 } else { 0.0 };l.f1fca = t368;
        if (((l.f1fc0 != 0.0) && (l.f1fc8 == 0.0)) && (l.f1fca != 0.0)) {let t369: f64 = (l.f1098).exp();(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t369, (t369 * l.f1099), 0.0, 0.0, 0.0, );}
        if (((l.f1fc0 != 0.0) && (l.f1fc8 == 0.0)) && (l.f1fca == 0.0)) {let t36a: f64 = (l.f1098).exp();let t36b: f64 = (1.0 + t36a);let t36c: f64 = (t36b).ln();(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t36c, ((t36a * l.f1099) / t36b), 0.0, 0.0, 0.0, );}
        let t36d: f64 = if l.f1160 == 1.0 { 1.0 } else { 0.0 };l.f1fcc = t36d;
        if (l.f1fcc != 0.0) {let t36e: f64 = (p.p51 * 0.5);let t36f: f64 = (t36e * l.f1077);let t370: f64 = (l.f1245 - t36f);let t371: f64 = (l.f1228 - t370);let t372: f64 = (t371 / l.f11d1);(l.f10aa, l.f10ab, l.f10ac, l.f10ad, ) = (t372, ((((-(l.f1246 - (t36e * l.f1078))) * l.f11d1) - (t371 * l.f11d2)) / (l.f11d1 * l.f11d1)), (l.f1229 / l.f11d1), (l.f122a / l.f11d1), );}
        let t373: f64 = if l.f10aa > 50.0 { 1.0 } else { 0.0 };l.f1fce = t373;
        if ((l.f1fcc != 0.0) && (l.f1fce != 0.0)) {(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (l.f10aa, l.f10ab, 0.0, l.f10ac, l.f10ad, );}
        let t374: f64 = (-50.0);let t375: f64 = if l.f10aa < t374 { 1.0 } else { 0.0 };l.f1fd0 = t375;
        if (((l.f1fcc != 0.0) && (l.f1fce == 0.0)) && (l.f1fd0 != 0.0)) {let t376: f64 = (l.f10aa).exp();(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t376, (t376 * l.f10ab), 0.0, (t376 * l.f10ac), (t376 * l.f10ad), );}
        if (((l.f1fcc != 0.0) && (l.f1fce == 0.0)) && (l.f1fd0 == 0.0)) {let t377: f64 = (l.f10aa).exp();let t378: f64 = (1.0 + t377);let t379: f64 = (t378).ln();(l.f10bb, l.f10c2, l.f10c3, l.f10c4, l.f10c5, ) = (t379, ((t377 * l.f10ab) / t378), 0.0, ((t377 * l.f10ac) / t378), ((t377 * l.f10ad) / t378), );}
        (l.f11b6, l.f11bb, l.f11bc, l.f11bd, l.f11be, l.f11b7, l.f11b8, l.f11b9, l.f11ba, ) = (l.f1109, l.f110e, l.f110f, l.f1110, l.f1111, l.f110a, l.f110b, l.f110c, l.f110d, );(l.f20bc, l.f20c1, l.f20c2, l.f20c3, l.f20c4, l.f20bd, l.f20be, l.f20bf, l.f20c0, ) = (l.f1109, l.f110e, l.f110f, l.f1110, l.f1111, l.f110a, l.f110b, l.f110c, l.f110d, );(l.f2220, l.f2225, l.f2226, l.f2227, l.f2228, l.f2221, l.f2222, l.f2223, l.f2224, ) = (l.f1162, l.f1167, l.f1168, l.f1169, l.f116a, l.f1163, l.f1164, l.f1165, l.f1166, );(l.f21de, l.f21e3, l.f21e4, l.f21e5, l.f21e6, l.f21df, l.f21e0, l.f21e1, l.f21e2, ) = (l.f1156, l.f115b, l.f115c, l.f115d, l.f115e, l.f1157, l.f1158, l.f1159, l.f115a, );(l.f20bc, l.f20c1, l.f20c2, l.f20c3, l.f20c4, l.f20bd, l.f20be, l.f20bf, l.f20c0, ) = (l.f11b6, l.f11bb, l.f11bc, l.f11bd, l.f11be, l.f11b7, l.f11b8, l.f11b9, l.f11ba, );(l.f210d, l.f210e, ) = ((nv29 - 0.0), 1.0, );let t37a: f64 = if p.p322 == 0.0 { 1.0 } else { 0.0 };l.f1fd2 = t37a;(l.f2133, l.f213d, l.f213e, l.f213c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f211c, l.f2126, l.f2127, l.f2125, ) = (0.0, 0.0, 0.0, 0.0, );(l.f2134, l.f2136, l.f2137, l.f2135, ) = (0.0, 0.0, 0.0, 0.0, );(l.f211d, l.f211f, l.f2120, l.f211e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f213f, l.f2140, l.f2141, l.f2142, ) = (0.0, 0.0, 0.0, 0.0, );(l.f2128, l.f2129, l.f212a, l.f212b, ) = (0.0, 0.0, 0.0, 0.0, );(l.f2138, l.f2139, l.f213a, l.f213b, ) = (0.0, 0.0, 0.0, 0.0, );(l.f2121, l.f2122, l.f2123, l.f2124, ) = (0.0, 0.0, 0.0, 0.0, );let t37b: f64 = if p.p254 == 1.0 { 1.0 } else { 0.0 };l.f1fd4 = t37b;
        if (l.f1fd4 != 0.0) {(l.f12ce, l.f12d0, l.f12d1, l.f12cf, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12c1, l.f12c2, ) = (0.0, 0.0, );(l.f12c3, l.f12c4, ) = (0.0, 0.0, );let t37c: f64 = (p.p6 * (nv8 - nv13));(l.f12d8, l.f12da, l.f12d9, ) = (t37c, p.p6, (-p.p6), );(l.f12cc, l.f12cd, ) = (l.f215b, l.f215c, );l.f12db = p.p260;l.f125f = p.p262;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_109(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1fd4 != 0.0) {l.f129b = p.p261;l.f12c9 = p.p258;l.f12c7 = p.p278;l.f12d7 = p.p277;(l.f12d4, l.f12d5, ) = (l.f22f2, l.f22f3, );l.f12de = p.p0;l.f12c6 = p.p2;let t37d: f64 = (1.0 - p.p255);let t37e: f64 = (t37d * p.p259);l.f12bf = t37e;l.f12c5 = p.p276;l.f12dc = p.p270;l.f1260 = p.p271;let t37f: f64 = (1.0 - p.p255);let t380: f64 = (t37f * p.p269);l.f12c0 = t380;l.f12cb = p.p268;l.f12c8 = p.p257;l.f12dd = p.p256;l.f12d6 = p.p6;(l.f12bb, l.f12bd, l.f12be, l.f12bc, ) = (0.0, 0.0, 0.0, 0.0, );(l.f125d, l.f125e, ) = (0.0, 0.0, );(l.f12d2, l.f12d3, ) = (0.0, 0.0, );(l.f1297, l.f1299, l.f129a, l.f1298, ) = (0.0, 0.0, 0.0, 0.0, );(l.f129f, l.f12a1, l.f12a2, l.f12a0, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12a5, l.f12a7, l.f12a8, l.f12a6, ) = (0.0, 0.0, 0.0, 0.0, );(l.f129c, l.f129e, l.f129d, ) = (0.0, 0.0, 0.0, );(l.f12b7, l.f12b9, l.f12ba, l.f12b8, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1269, l.f126b, l.f126c, l.f126a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f126f, l.f1270, ) = (0.0, 0.0, );(l.f1261, l.f1263, l.f1264, l.f1262, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1267, l.f1268, ) = (0.0, 0.0, );(l.f1295, l.f1296, ) = (0.0, 0.0, );(l.f1271, l.f1273, l.f1274, l.f1272, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1281, l.f1283, l.f1284, l.f1282, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1275, l.f1277, l.f1278, l.f1276, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1291, l.f1293, l.f1294, l.f1292, ) = (0.0, 0.0, 0.0, 0.0, );(l.f128d, l.f128f, l.f1290, l.f128e, ) = (0.0, 0.0, 0.0, 0.0, );l.f12ca = 0.0;(l.f126d, l.f126e, ) = (0.0, 0.0, );(l.f1265, l.f1266, ) = (0.0, 0.0, );(l.f12a3, l.f12a4, ) = (0.0, 0.0, );(l.f128b, l.f128c, ) = (0.0, 0.0, );(l.f127f, l.f1280, ) = (0.0, 0.0, );(l.f12b5, l.f12b6, ) = (0.0, 0.0, );(l.f12b1, l.f12b3, l.f12b4, l.f12b2, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1289, l.f128a, ) = (0.0, 0.0, );(l.f127d, l.f127e, ) = (0.0, 0.0, );(l.f12af, l.f12b0, ) = (0.0, 0.0, );(l.f1285, l.f1287, l.f1288, l.f1286, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1279, l.f127b, l.f127c, l.f127a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12ad, l.f12ae, ) = (0.0, 0.0, );(l.f12a9, l.f12ab, l.f12ac, l.f12aa, ) = (0.0, 0.0, 0.0, 0.0, );let t381: f64 = (l.f12c8 / l.f12cc);let t382: f64 = (-l.f12dd);let t383: f64 = (t381 * t382);(l.f1295, l.f1296, ) = (t383, ((-((l.f12c8 * l.f12cd) / (l.f12cc * l.f12cc))) * t382), );}
        if (l.f1fd4 != 0.0) {
            let t384: f64 = (-50.0);
            let (t391, t392,) = {
    if ((!(l.f1295 > 50.0)) && (!(l.f1295 < t384))) {
        let t385: f64 = (l.f1295).exp();
        (t385, (t385 * l.f1296),)
    } else {
        let t386: f64 = (-50.0);
        let (t38f, t390,) = {
            if ((!(l.f1295 > 50.0)) && (l.f1295 < t386)) {
                let t387: f64 = (-50.0);let t388: f64 = (t387).exp();
                (t388, 0.0,)
            } else {
                let (t38d, t38e,) = {
                    if (l.f1295 > 50.0) {
                        let t389: f64 = (50.0_f64).exp();let t38a: f64 = (l.f1295 - 50.0);let t38b: f64 = (1.0 + t38a);let t38c: f64 = (t389 * t38b);
                        (t38c, (t389 * l.f1296),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t38d, t38e,)
            }
        };
        (t38f, t390,)
    }
};
            (l.f12d2, l.f12d3, ) = (t391, t392, );
        }
        if (l.f1fd4 != 0.0) {let t393: f64 = (-l.f12d8);let t394: f64 = (t393 - l.f12d7);let t395: f64 = (l.f12c7 * t394);let t396: f64 = (t395 + l.f1295);(l.f1269, l.f126b, l.f126c, l.f126a, ) = (t396, l.f1296, (l.f12c7 * (-l.f12da)), (l.f12c7 * (-l.f12d9)), );let t397: f64 = (-l.f12c7);let t398: f64 = (t397 * l.f12d7);let t399: f64 = (t398 + l.f1295);(l.f126f, l.f1270, ) = (t399, l.f1296, );}
        if (l.f1fd4 != 0.0) {
            let t39a: f64 = (-50.0);
            let (t3ab, t3ad, t3ae, t3ac,) = {
    if ((!(l.f1269 > 50.0)) && (!(l.f1269 < t39a))) {
        let t39b: f64 = (l.f1269).exp();
        (t39b, (t39b * l.f126b), (t39b * l.f126c), (t39b * l.f126a),)
    } else {
        let t39c: f64 = (-50.0);
        let (t3a7, t3a9, t3aa, t3a8,) = {
            if ((!(l.f1269 > 50.0)) && (l.f1269 < t39c)) {
                let t39d: f64 = (-50.0);let t39e: f64 = (t39d).exp();
                (t39e, 0.0, 0.0, 0.0,)
            } else {
                let (t3a3, t3a5, t3a6, t3a4,) = {
                    if (l.f1269 > 50.0) {
                        let t39f: f64 = (50.0_f64).exp();let t3a0: f64 = (l.f1269 - 50.0);let t3a1: f64 = (1.0 + t3a0);let t3a2: f64 = (t39f * t3a1);
                        (t3a2, (t39f * l.f126b), (t39f * l.f126c), (t39f * l.f126a),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t3a3, t3a5, t3a6, t3a4,)
            }
        };
        (t3a7, t3a9, t3aa, t3a8,)
    }
};
            (l.f1261, l.f1263, l.f1264, l.f1262, ) = (t3ab, t3ad, t3ae, t3ac, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_110(
        l: &mut StampLocals,
    ) {
        if (l.f1fd4 != 0.0) {
            let t3af: f64 = (-50.0);
            let (t3bc, t3bd,) = {
    if ((!(l.f126f > 50.0)) && (!(l.f126f < t3af))) {
        let t3b0: f64 = (l.f126f).exp();
        (t3b0, (t3b0 * l.f1270),)
    } else {
        let t3b1: f64 = (-50.0);
        let (t3ba, t3bb,) = {
            if ((!(l.f126f > 50.0)) && (l.f126f < t3b1)) {
                let t3b2: f64 = (-50.0);let t3b3: f64 = (t3b2).exp();
                (t3b3, 0.0,)
            } else {
                let (t3b8, t3b9,) = {
                    if (l.f126f > 50.0) {
                        let t3b4: f64 = (50.0_f64).exp();let t3b5: f64 = (l.f126f - 50.0);let t3b6: f64 = (1.0 + t3b5);let t3b7: f64 = (t3b4 * t3b6);
                        (t3b7, (t3b4 * l.f1270),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t3b8, t3b9,)
            }
        };
        (t3ba, t3bb,)
    }
};
            (l.f1267, l.f1268, ) = (t3bc, t3bd, );
        }
        if (l.f1fd4 != 0.0) {let t3be: f64 = (l.f1261 - l.f1267);(l.f129f, l.f12a1, l.f12a2, l.f12a0, ) = (t3be, (l.f1263 - l.f1268), l.f1264, l.f1262, );let t3bf: f64 = (l.f12d6 * l.f12de);let t3c0: f64 = (t3bf * l.f12c6);let t3c1: f64 = (t3c0 * l.f12bf);let t3c2: f64 = (t3c1 * l.f12d4);(l.f12c1, l.f12c2, ) = (t3c2, (t3c1 * l.f12d5), );let t3c3: f64 = (l.f12c9 / l.f12cc);let t3c4: f64 = (t3c3 * l.f12d8);let t3c5: f64 = (t3c4 + l.f1295);(l.f1281, l.f1283, l.f1284, l.f1282, ) = (t3c5, (((-((l.f12c9 * l.f12cd) / (l.f12cc * l.f12cc))) * l.f12d8) + l.f1296), (t3c3 * l.f12da), (t3c3 * l.f12d9), );}
        if (l.f1fd4 != 0.0) {
            let t3c6: f64 = (-50.0);
            let (t3d7, t3d9, t3da, t3d8,) = {
    if ((!(l.f1281 > 50.0)) && (!(l.f1281 < t3c6))) {
        let t3c7: f64 = (l.f1281).exp();
        (t3c7, (t3c7 * l.f1283), (t3c7 * l.f1284), (t3c7 * l.f1282),)
    } else {
        let t3c8: f64 = (-50.0);
        let (t3d3, t3d5, t3d6, t3d4,) = {
            if ((!(l.f1281 > 50.0)) && (l.f1281 < t3c8)) {
                let t3c9: f64 = (-50.0);let t3ca: f64 = (t3c9).exp();
                (t3ca, 0.0, 0.0, 0.0,)
            } else {
                let (t3cf, t3d1, t3d2, t3d0,) = {
                    if (l.f1281 > 50.0) {
                        let t3cb: f64 = (50.0_f64).exp();let t3cc: f64 = (l.f1281 - 50.0);let t3cd: f64 = (1.0 + t3cc);let t3ce: f64 = (t3cb * t3cd);
                        (t3ce, (t3cb * l.f1283), (t3cb * l.f1284), (t3cb * l.f1282),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t3cf, t3d1, t3d2, t3d0,)
            }
        };
        (t3d3, t3d5, t3d6, t3d4,)
    }
};
            (l.f1275, l.f1277, l.f1278, l.f1276, ) = (t3d7, t3d9, t3da, t3d8, );
        }
        let t3db: f64 = if l.f129b == 1.0 { 1.0 } else { 0.0 };l.f1fd5 = t3db;
        if ((l.f1fd4 != 0.0) && (l.f1fd5 != 0.0)) {let t3dc: f64 = (l.f12c5 * l.f129f);let t3dd: f64 = (l.f1275 - t3dc);let t3de: f64 = (t3dd - l.f12d2);let t3df: f64 = (l.f12c1 * t3de);(l.f12a5, l.f12a7, l.f12a8, l.f12a6, ) = (t3df, ((l.f12c2 * t3de) + (l.f12c1 * ((l.f1277 - (l.f12c5 * l.f12a1)) - l.f12d3))), (l.f12c1 * (l.f1278 - (l.f12c5 * l.f12a2))), (l.f12c1 * (l.f1276 - (l.f12c5 * l.f12a0))), );}
        if ((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) {let t3e0: f64 = (-l.f12db);let t3e1: f64 = (t3e0 - l.f12d7);let t3e2: f64 = (l.f12c7 * t3e1);let t3e3: f64 = (t3e2 + l.f1295);(l.f126d, l.f126e, ) = (t3e3, l.f1296, );}
        if ((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) {
            let t3e4: f64 = (-50.0);
            let (t3f1, t3f2,) = {
    if ((!(l.f126d > 50.0)) && (!(l.f126d < t3e4))) {
        let t3e5: f64 = (l.f126d).exp();
        (t3e5, (t3e5 * l.f126e),)
    } else {
        let t3e6: f64 = (-50.0);
        let (t3ef, t3f0,) = {
            if ((!(l.f126d > 50.0)) && (l.f126d < t3e6)) {
                let t3e7: f64 = (-50.0);let t3e8: f64 = (t3e7).exp();
                (t3e8, 0.0,)
            } else {
                let (t3ed, t3ee,) = {
                    if (l.f126d > 50.0) {
                        let t3e9: f64 = (50.0_f64).exp();let t3ea: f64 = (l.f126d - 50.0);let t3eb: f64 = (1.0 + t3ea);let t3ec: f64 = (t3e9 * t3eb);
                        (t3ec, (t3e9 * l.f126e),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t3ed, t3ee,)
            }
        };
        (t3ef, t3f0,)
    }
};
            (l.f1265, l.f1266, ) = (t3f1, t3f2, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) {let t3f3: f64 = (l.f1265 - l.f1267);(l.f12a3, l.f12a4, ) = (t3f3, (l.f1266 - l.f1268), );let t3f4: f64 = (l.f12c9 / l.f12cc);let t3f5: f64 = (t3f4 * l.f12db);let t3f6: f64 = (t3f5 + l.f1295);(l.f128b, l.f128c, ) = (t3f6, (((-((l.f12c9 * l.f12cd) / (l.f12cc * l.f12cc))) * l.f12db) + l.f1296), );}
        if ((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) {
            let t3f7: f64 = (-50.0);
            let (t404, t405,) = {
    if ((!(l.f128b > 50.0)) && (!(l.f128b < t3f7))) {
        let t3f8: f64 = (l.f128b).exp();
        (t3f8, (t3f8 * l.f128c),)
    } else {
        let t3f9: f64 = (-50.0);
        let (t402, t403,) = {
            if ((!(l.f128b > 50.0)) && (l.f128b < t3f9)) {
                let t3fa: f64 = (-50.0);let t3fb: f64 = (t3fa).exp();
                (t3fb, 0.0,)
            } else {
                let (t400, t401,) = {
                    if (l.f128b > 50.0) {
                        let t3fc: f64 = (50.0_f64).exp();let t3fd: f64 = (l.f128b - 50.0);let t3fe: f64 = (1.0 + t3fd);let t3ff: f64 = (t3fc * t3fe);
                        (t3ff, (t3fc * l.f128c),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t400, t401,)
            }
        };
        (t402, t403,)
    }
};
            (l.f127f, l.f1280, ) = (t404, t405, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) {let t406: f64 = (l.f12c5 * l.f12a3);let t407: f64 = (l.f127f - t406);let t408: f64 = (t407 - l.f12d2);(l.f12b5, l.f12b6, ) = (t408, ((l.f1280 - (l.f12c5 * l.f12a4)) - l.f12d3), );let t409: f64 = (l.f12c5 * l.f129f);let t40a: f64 = (l.f1275 - t409);let t40b: f64 = (t40a - l.f12d2);let t40c: f64 = (l.f12c1 * t40b);(l.f12b1, l.f12b3, l.f12b4, l.f12b2, ) = (t40c, ((l.f12c2 * t40b) + (l.f12c1 * ((l.f1277 - (l.f12c5 * l.f12a1)) - l.f12d3))), (l.f12c1 * (l.f1278 - (l.f12c5 * l.f12a2))), (l.f12c1 * (l.f1276 - (l.f12c5 * l.f12a0))), );}
        let t40d: f64 = if l.f129b > 0.0 { 1.0 } else { 0.0 };l.f1fd8 = t40d;
        if (((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd8 != 0.0)) {let t40e: f64 = (l.f129b * l.f12c9);l.f12ca = t40e;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_111(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd8 != 0.0)) {let t40f: f64 = (l.f12ca / l.f12cc);let t410: f64 = (t40f * l.f12db);let t411: f64 = (t410 + l.f1295);(l.f1289, l.f128a, ) = (t411, (((-((l.f12ca * l.f12cd) / (l.f12cc * l.f12cc))) * l.f12db) + l.f1296), );}
        if (((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd8 != 0.0)) {
            let t412: f64 = (-50.0);
            let (t41f, t420,) = {
    if ((!(l.f1289 > 50.0)) && (!(l.f1289 < t412))) {
        let t413: f64 = (l.f1289).exp();
        (t413, (t413 * l.f128a),)
    } else {
        let t414: f64 = (-50.0);
        let (t41d, t41e,) = {
            if ((!(l.f1289 > 50.0)) && (l.f1289 < t414)) {
                let t415: f64 = (-50.0);let t416: f64 = (t415).exp();
                (t416, 0.0,)
            } else {
                let (t41b, t41c,) = {
                    if (l.f1289 > 50.0) {
                        let t417: f64 = (50.0_f64).exp();let t418: f64 = (l.f1289 - 50.0);let t419: f64 = (1.0 + t418);let t41a: f64 = (t417 * t419);
                        (t41a, (t417 * l.f128a),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t41b, t41c,)
            }
        };
        (t41d, t41e,)
    }
};
            (l.f127d, l.f127e, ) = (t41f, t420, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd8 != 0.0)) {let t421: f64 = (l.f12c5 * l.f12a3);let t422: f64 = (l.f127d - t421);let t423: f64 = (t422 - l.f12d2);(l.f12af, l.f12b0, ) = (t423, ((l.f127e - (l.f12c5 * l.f12a4)) - l.f12d3), );let t424: f64 = (l.f12ca / l.f12cc);let t425: f64 = (t424 * l.f12d8);let t426: f64 = (t425 + l.f1295);(l.f1285, l.f1287, l.f1288, l.f1286, ) = (t426, (((-((l.f12ca * l.f12cd) / (l.f12cc * l.f12cc))) * l.f12d8) + l.f1296), (t424 * l.f12da), (t424 * l.f12d9), );}
        if (((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd8 != 0.0)) {
            let t427: f64 = (-50.0);
            let (t438, t43a, t43b, t439,) = {
    if ((!(l.f1285 > 50.0)) && (!(l.f1285 < t427))) {
        let t428: f64 = (l.f1285).exp();
        (t428, (t428 * l.f1287), (t428 * l.f1288), (t428 * l.f1286),)
    } else {
        let t429: f64 = (-50.0);
        let (t434, t436, t437, t435,) = {
            if ((!(l.f1285 > 50.0)) && (l.f1285 < t429)) {
                let t42a: f64 = (-50.0);let t42b: f64 = (t42a).exp();
                (t42b, 0.0, 0.0, 0.0,)
            } else {
                let (t430, t432, t433, t431,) = {
                    if (l.f1285 > 50.0) {
                        let t42c: f64 = (50.0_f64).exp();let t42d: f64 = (l.f1285 - 50.0);let t42e: f64 = (1.0 + t42d);let t42f: f64 = (t42c * t42e);
                        (t42f, (t42c * l.f1287), (t42c * l.f1288), (t42c * l.f1286),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t430, t432, t433, t431,)
            }
        };
        (t434, t436, t437, t435,)
    }
};
            (l.f1279, l.f127b, l.f127c, l.f127a, ) = (t438, t43a, t43b, t439, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd8 != 0.0)) {let t43c: f64 = (l.f12c1 * l.f12b5);let t43d: f64 = (t43c / l.f12af);(l.f12ad, l.f12ae, ) = (t43d, (((((l.f12c2 * l.f12b5) + (l.f12c1 * l.f12b6)) * l.f12af) - (t43c * l.f12b0)) / (l.f12af * l.f12af)), );let t43e: f64 = (l.f12c5 * l.f129f);let t43f: f64 = (l.f1279 - t43e);let t440: f64 = (t43f - l.f12d2);let t441: f64 = (l.f12ad * t440);(l.f12a9, l.f12ab, l.f12ac, l.f12aa, ) = (t441, ((l.f12ae * t440) + (l.f12ad * ((l.f127b - (l.f12c5 * l.f12a1)) - l.f12d3))), (l.f12ad * (l.f127c - (l.f12c5 * l.f12a2))), (l.f12ad * (l.f127a - (l.f12c5 * l.f12a0))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd8 == 0.0)) {let t442: f64 = (l.f12c1 * l.f12b5);(l.f12a9, l.f12ab, l.f12ac, l.f12aa, ) = (t442, ((l.f12c2 * l.f12b5) + (l.f12c1 * l.f12b6)), 0.0, 0.0, );}
        if ((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) {let t443: f64 = (l.f125f * l.f125f);let t444: f64 = (t443 * l.f12cc);(l.f125d, l.f125e, ) = (t444, (t443 * l.f12cd), );let t445: f64 = (l.f125d / 2.0);let t446: f64 = (l.f12db - t445);let t447: f64 = (l.f12d8 - t446);let t448: f64 = (t447 / l.f125d);(l.f1271, l.f1273, l.f1274, l.f1272, ) = (t448, ((((-(-(l.f125e / 2.0))) * l.f125d) - (t447 * l.f125e)) / (l.f125d * l.f125d)), (l.f12da / l.f125d), (l.f12d9 / l.f125d), );}
        let t449: f64 = if l.f1271 > 50.0 { 1.0 } else { 0.0 };l.f1fd9 = t449;
        if (((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd9 != 0.0)) {(l.f1297, l.f1299, l.f129a, l.f1298, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t44a: f64 = (-50.0);let t44b: f64 = if l.f1271 < t44a { 1.0 } else { 0.0 };l.f1fda = t44b;
        if ((((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd9 == 0.0)) && (l.f1fda != 0.0)) {(l.f1297, l.f1299, l.f129a, l.f1298, ) = (1.0, 0.0, 0.0, 0.0, );}
        if ((((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) && (l.f1fd9 == 0.0)) && (l.f1fda == 0.0)) {let t44c: f64 = (l.f1271).exp();let t44d: f64 = (1.0 + t44c);let t44e: f64 = (1.0 / t44d);(l.f1297, l.f1299, l.f129a, l.f1298, ) = (t44e, (-((t44c * l.f1273) / (t44d * t44d))), (-((t44c * l.f1274) / (t44d * t44d))), (-((t44c * l.f1272) / (t44d * t44d))), );}
        if ((l.f1fd4 != 0.0) && (l.f1fd5 == 0.0)) {let t44f: f64 = (l.f1297 * l.f12b1);let t450: f64 = (1.0 - l.f1297);let t451: f64 = (t450 * l.f12a9);let t452: f64 = (t44f + t451);(l.f12a5, l.f12a7, l.f12a8, l.f12a6, ) = (t452, (((l.f1299 * l.f12b1) + (l.f1297 * l.f12b3)) + (((-l.f1299) * l.f12a9) + (t450 * l.f12ab))), (((l.f129a * l.f12b1) + (l.f1297 * l.f12b4)) + (((-l.f129a) * l.f12a9) + (t450 * l.f12ac))), (((l.f1298 * l.f12b1) + (l.f1297 * l.f12b2)) + (((-l.f1298) * l.f12a9) + (t450 * l.f12aa))), );}
        if (l.f1fd4 != 0.0) {
            let t453: f64 = (-l.f12d8);
            let (t462, t464, t463,) = {
    if (p.p52 != 0.0) {
        let t454: f64 = (l.f12d8 / l.f12dc);let t455: f64 = (0.001 / p.p53);let t456: f64 = (l.f12d8 / l.f12dc);let t457: f64 = (t455 * t456);let t458: f64 = (t457).tanh();let t459: f64 = (t454 * t458);
        (t459, (((l.f12da / l.f12dc) * t458) + (t454 * ((t455 * (l.f12da / l.f12dc)) / ((t457).cosh() * (t457).cosh())))), (((l.f12d9 / l.f12dc) * t458) + (t454 * ((t455 * (l.f12d9 / l.f12dc)) / ((t457).cosh() * (t457).cosh())))),)
    } else {
        let (t45f, t461, t460,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f12dc;let t45a: f64 = (l.f12d8 * __rspice_inv_cse_0);let t45b: f64 = (l.f12d8 * __rspice_inv_cse_0);let t45c: f64 = (t45a * t45b);let t45d: f64 = (t45c + p.p53);let t45e: f64 = (t45d).sqrt();
                (t45e, ((((l.f12da / l.f12dc) * t45b) + (t45a * (l.f12da / l.f12dc))) / (2.0 * t45e)), ((((l.f12d9 / l.f12dc) * t45b) + (t45a * (l.f12d9 / l.f12dc))) / (2.0 * t45e)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t45f, t461, t460,)
    }
};
            let t465: f64 = (t462).powf(l.f1260);let t466: f64 = (1.0 + t465);let t467: f64 = (1.0 / l.f1260);let t468: f64 = (t466).powf(t467);let t469: f64 = (t453 / t468);(l.f129c, l.f129e, l.f129d, ) = (t469, ((((-l.f12da) * t468) - (t453 * if 0.0 == 0.0 && ((t467) as f64).is_finite() && ((t467) as f64).fract() == 0.0 { if t467 == 0.0 { 0.0 } else { (t467 * ((t466).powf(t467 - 1.0) * if 0.0 == 0.0 && ((l.f1260) as f64).is_finite() && ((l.f1260) as f64).fract() == 0.0 { if l.f1260 == 0.0 { 0.0 } else { (l.f1260 * ((t462).powf(l.f1260 - 1.0) * t464)) } } else { (t465 * (l.f1260 * (t464 / t462))) })) } } else { (t468 * (t467 * (if 0.0 == 0.0 && ((l.f1260) as f64).is_finite() && ((l.f1260) as f64).fract() == 0.0 { if l.f1260 == 0.0 { 0.0 } else { (l.f1260 * ((t462).powf(l.f1260 - 1.0) * t464)) } } else { (t465 * (l.f1260 * (t464 / t462))) } / t466))) })) / (t468 * t468)), ((((-l.f12d9) * t468) - (t453 * if 0.0 == 0.0 && ((t467) as f64).is_finite() && ((t467) as f64).fract() == 0.0 { if t467 == 0.0 { 0.0 } else { (t467 * ((t466).powf(t467 - 1.0) * if 0.0 == 0.0 && ((l.f1260) as f64).is_finite() && ((l.f1260) as f64).fract() == 0.0 { if l.f1260 == 0.0 { 0.0 } else { (l.f1260 * ((t462).powf(l.f1260 - 1.0) * t463)) } } else { (t465 * (l.f1260 * (t463 / t462))) })) } } else { (t468 * (t467 * (if 0.0 == 0.0 && ((l.f1260) as f64).is_finite() && ((l.f1260) as f64).fract() == 0.0 { if l.f1260 == 0.0 { 0.0 } else { (l.f1260 * ((t462).powf(l.f1260 - 1.0) * t463)) } } else { (t465 * (l.f1260 * (t463 / t462))) } / t466))) })) / (t468 * t468)), );
        }
    }
}
