#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b2 != 0.0)) {let t0: f64 = (l.f7b1 / l.f5f1);let t1: f64 = (l.f5f1 - l.f5ed);let t2: f64 = (l.f793 * t1);let t3: f64 = (l.f5ed * p.p85);let t4: f64 = (t2 / t3);let t5: f64 = (t0 + t4);let t6: f64 = (l.f645 * t5);let t7: f64 = (t6).exp();(l.f8a, l.f8b, l.f8c, ) = (t7, (t7 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3) - (t2 * (l.f5ee * p.p85))) / (t3 * t3))))), (t7 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3) - (t2 * (l.f5ef * p.p85))) / (t3 * t3))))), );}
        let t8: f64 = (l.f7b1 / l.f5f1);let t9: f64 = (l.f5f1 - l.f5ed);let ta: f64 = (l.f793 * t9);let tb: f64 = (l.f5ed * p.p85);let tc: f64 = (ta / tb);let td: f64 = (t8 + tc);let te: f64 = (l.f645 * td);let tf: f64 = (-230.25850929940458);let t10: f64 = if te < tf { 1.0 } else { 0.0 };l.f2b4 = t10;
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b2 == 0.0)) && (l.f2b4 != 0.0)) {let t11: f64 = (-230.25850929940458);let t12: f64 = (l.f7b1 / l.f5f1);let t13: f64 = (l.f5f1 - l.f5ed);let t14: f64 = (l.f793 * t13);let t15: f64 = (l.f5ed * p.p85);let t16: f64 = (t14 / t15);let t17: f64 = (t12 + t16);let t18: f64 = (l.f645 * t17);let t19: f64 = (t11 - t18);let t1a: f64 = (-230.25850929940458);let t1b: f64 = (l.f7b1 / l.f5f1);let t1c: f64 = (l.f5f1 - l.f5ed);let t1d: f64 = (l.f793 * t1c);let t1e: f64 = (l.f5ed * p.p85);let t1f: f64 = (t1d / t1e);let t20: f64 = (t1b + t1f);let t21: f64 = (l.f645 * t20);let t22: f64 = (t1a - t21);let t23: f64 = (-230.25850929940458);let t24: f64 = (l.f7b1 / l.f5f1);let t25: f64 = (l.f5f1 - l.f5ed);let t26: f64 = (l.f793 * t25);let t27: f64 = (l.f5ed * p.p85);let t28: f64 = (t26 / t27);let t29: f64 = (t24 + t28);let t2a: f64 = (l.f645 * t29);let t2b: f64 = (t23 - t2a);let t2c: f64 = (t2b * 0.3333333333333333);let t2d: f64 = (1.0 + t2c);let t2e: f64 = (t22 * t2d);let t2f: f64 = (0.5 * t2e);let t30: f64 = (1.0 + t2f);let t31: f64 = (t19 * t30);let t32: f64 = (1.0 + t31);let t33: f64 = (1e-100 / t32);(l.f8a, l.f8b, l.f8c, ) = (t33, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t15) - (t14 * (l.f5ee * p.p85))) / (t15 * t15))))) * t30) + (t19 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e) - (t1d * (l.f5ee * p.p85))) / (t1e * t1e))))) * t2d) + (t22 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t27) - (t26 * (l.f5ee * p.p85))) / (t27 * t27))))) * 0.3333333333333333))))))) / (t32 * t32))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t15) - (t14 * (l.f5ef * p.p85))) / (t15 * t15))))) * t30) + (t19 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e) - (t1d * (l.f5ef * p.p85))) / (t1e * t1e))))) * t2d) + (t22 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t27) - (t26 * (l.f5ef * p.p85))) / (t27 * t27))))) * 0.3333333333333333))))))) / (t32 * t32))), );}
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b2 == 0.0)) && (l.f2b4 == 0.0)) {let t34: f64 = (l.f7b1 / l.f5f1);let t35: f64 = (l.f5f1 - l.f5ed);let t36: f64 = (l.f793 * t35);let t37: f64 = (l.f5ed * p.p85);let t38: f64 = (t36 / t37);let t39: f64 = (t34 + t38);let t3a: f64 = (l.f645 * t39);let t3b: f64 = (t3a - 230.25850929940458);let t3c: f64 = (l.f7b1 / l.f5f1);let t3d: f64 = (l.f5f1 - l.f5ed);let t3e: f64 = (l.f793 * t3d);let t3f: f64 = (l.f5ed * p.p85);let t40: f64 = (t3e / t3f);let t41: f64 = (t3c + t40);let t42: f64 = (l.f645 * t41);let t43: f64 = (t42 - 230.25850929940458);let t44: f64 = (l.f7b1 / l.f5f1);let t45: f64 = (l.f5f1 - l.f5ed);let t46: f64 = (l.f793 * t45);let t47: f64 = (l.f5ed * p.p85);let t48: f64 = (t46 / t47);let t49: f64 = (t44 + t48);let t4a: f64 = (l.f645 * t49);let t4b: f64 = (t4a - 230.25850929940458);let t4c: f64 = (t4b * 0.3333333333333333);let t4d: f64 = (1.0 + t4c);let t4e: f64 = (t43 * t4d);let t4f: f64 = (0.5 * t4e);let t50: f64 = (1.0 + t4f);let t51: f64 = (t3b * t50);let t52: f64 = (1.0 + t51);let t53: f64 = (1e100 * t52);(l.f8a, l.f8b, l.f8c, ) = (t53, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t37) - (t36 * (l.f5ee * p.p85))) / (t37 * t37)))) * t50) + (t3b * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3f) - (t3e * (l.f5ee * p.p85))) / (t3f * t3f)))) * t4d) + (t43 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t47) - (t46 * (l.f5ee * p.p85))) / (t47 * t47)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t37) - (t36 * (l.f5ef * p.p85))) / (t37 * t37)))) * t50) + (t3b * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3f) - (t3e * (l.f5ef * p.p85))) / (t3f * t3f)))) * t4d) + (t43 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t47) - (t46 * (l.f5ef * p.p85))) / (t47 * t47)))) * 0.3333333333333333))))))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) {let t54: f64 = (l.f7b1 * l.f5b);let t55: f64 = (l.f5f1 - t54);let t56: f64 = (l.f5f1 * l.f5f1);let t57: f64 = (t55 / t56);let t58: f64 = (l.f793 * l.f5b);let t59: f64 = (l.f5ed * p.p85);let t5a: f64 = (t58 / t59);let t5b: f64 = (t57 + t5a);let t5c: f64 = (l.f645 * t5b);(l.f61, l.f62, l.f63, ) = (t5c, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t56) - (t55 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t56 * t56)) + ((((l.f793 * l.f5c) * t59) - (t58 * (l.f5ee * p.p85))) / (t59 * t59)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t56) - (t55 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t56 * t56)) + ((((l.f793 * l.f5d) * t59) - (t58 * (l.f5ef * p.p85))) / (t59 * t59)))), );let t5d: f64 = (l.f73d - l.f7b1);let t5e: f64 = (t5d * l.f61);let t5f: f64 = (1.0 + t5e);let t60: f64 = (t5f * l.f8a);(l.f536, l.f537, l.f538, ) = (t60, (((t5d * l.f62) * l.f8a) + (t5f * l.f8b)), (((t5d * l.f63) * l.f8a) + (t5f * l.f8c)), );let t61: f64 = (l.f5eb * l.f5eb);let t62: f64 = (t61 / l.f5e3);l.f64f = t62;let t63: f64 = (l.f5e9 / l.f645);let t64: f64 = (l.f5e3 / l.f64f);let t65: f64 = (t64).ln();let t66: f64 = (t63 * t65);l.f793 = t66;}
        let t67: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f2b6 = t67;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {let t68: f64 = (l.f7b1 - l.f793);let t69: f64 = (p.p86 * t68);let t6a: f64 = (t69 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t6a, 0.0, 0.0, );let t6b: f64 = (p.p86 * l.f793);let t6c: f64 = (l.f5e9 - t6b);(l.f5ed, l.f5ee, l.f5ef, ) = (t6c, 0.0, 0.0, );let t6d: f64 = (p.p85 - l.f601);let t6e: f64 = (t6d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t6e, (-l.f602), (-l.f603), );let t6f: f64 = (4.0 * p.p85);let t70: f64 = (t6f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t70, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {
            let (t72, t73, t74,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t71: f64 = (-l.f6f7);
        (t71, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t72, t73, t74, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {let t75: f64 = (l.f6f3 * l.f6f3);let t76: f64 = (t75 + l.f6f7);let t77: f64 = (t76).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t77, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t77)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t77)), );let t78: f64 = (l.f6f3 / l.f6f7);let t79: f64 = (1.0 + t78);let t7a: f64 = (0.5 * t79);(l.f55, l.f56, l.f57, ) = (t7a, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t7b: f64 = (l.f6f3 + l.f6f7);let t7c: f64 = (0.5 * t7b);let t7d: f64 = (p.p85 - t7c);(l.f605, l.f606, l.f607, ) = (t7d, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t7e: f64 = (l.f605 - l.f5e9);let t7f: f64 = (t7e - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t7f, l.f606, l.f607, );let t80: f64 = (4.0 * l.f5e9);let t81: f64 = (t80 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t81, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {
            let (t83, t84, t85,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t82: f64 = (-l.f6f7);
        (t82, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t83, t84, t85, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {let t86: f64 = (l.f6f3 * l.f6f3);let t87: f64 = (t86 + l.f6f7);let t88: f64 = (t87).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t88, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t88)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t88)), );let t89: f64 = (l.f6f3 / l.f6f7);let t8a: f64 = (1.0 + t89);let t8b: f64 = (0.5 * t8a);(l.f51, l.f52, l.f53, ) = (t8b, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t8c: f64 = (l.f6f3 + l.f6f7);let t8d: f64 = (0.5 * t8c);let t8e: f64 = (l.f5e9 + t8d);(l.f5f1, l.f5f2, l.f5f3, ) = (t8e, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t8f: f64 = (p.p85 - l.f5ed);let t90: f64 = (t8f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t90, (-l.f5ee), (-l.f5ef), );let t91: f64 = (4.0 * p.p85);let t92: f64 = (t91 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t92, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {
            let (t94, t95, t96,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t93: f64 = (-l.f6f7);
        (t93, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t94, t95, t96, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {let t97: f64 = (l.f6f3 * l.f6f3);let t98: f64 = (t97 + l.f6f7);let t99: f64 = (t98).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t99, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t99)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t99)), );let t9a: f64 = (l.f6f3 + l.f6f7);let t9b: f64 = (0.5 * t9a);let t9c: f64 = (p.p85 - t9b);(l.f5ed, l.f5ee, l.f5ef, ) = (t9c, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t9d: f64 = (l.f5ed - l.f5e9);let t9e: f64 = (t9d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t9e, l.f5ee, l.f5ef, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {let t9f: f64 = (4.0 * l.f5e9);let ta0: f64 = (t9f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (ta0, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {
            let (ta2, ta3, ta4,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let ta1: f64 = (-l.f6f7);
        (ta1, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (ta2, ta3, ta4, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 != 0.0)) {let ta5: f64 = (l.f6f3 * l.f6f3);let ta6: f64 = (ta5 + l.f6f7);let ta7: f64 = (ta6).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (ta7, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * ta7)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * ta7)), );let ta8: f64 = (l.f6f3 + l.f6f7);let ta9: f64 = (0.5 * ta8);let taa: f64 = (l.f5e9 + ta9);(l.f5ed, l.f5ee, l.f5ef, ) = (taa, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let tab: f64 = (p.p86 * l.f55);let tac: f64 = (tab * l.f51);(l.f5b, l.f5c, l.f5d, ) = (tac, (((p.p86 * l.f56) * l.f51) + (tab * l.f52)), (((p.p86 * l.f57) * l.f51) + (tab * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b6 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let tad: f64 = (l.f7b1 / l.f5f1);let tae: f64 = (l.f5f1 - l.f5ed);let taf: f64 = (l.f793 * tae);let tb0: f64 = (l.f5ed * p.p85);let tb1: f64 = (taf / tb0);let tb2: f64 = (tad + tb1);let tb3: f64 = (l.f645 * tb2);let tb4: f64 = (tb3).abs();let tb5: f64 = if tb4 < 230.25850929940458 { 1.0 } else { 0.0 };l.f2b8 = tb5;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b8 != 0.0)) {let tb6: f64 = (l.f7b1 / l.f5f1);let tb7: f64 = (l.f5f1 - l.f5ed);let tb8: f64 = (l.f793 * tb7);let tb9: f64 = (l.f5ed * p.p85);let tba: f64 = (tb8 / tb9);let tbb: f64 = (tb6 + tba);let tbc: f64 = (l.f645 * tbb);let tbd: f64 = (tbc).exp();(l.f93, l.f94, l.f95, ) = (tbd, (tbd * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tb9) - (tb8 * (l.f5ee * p.p85))) / (tb9 * tb9))))), (tbd * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tb9) - (tb8 * (l.f5ef * p.p85))) / (tb9 * tb9))))), );}
        let tbe: f64 = (l.f7b1 / l.f5f1);let tbf: f64 = (l.f5f1 - l.f5ed);let tc0: f64 = (l.f793 * tbf);let tc1: f64 = (l.f5ed * p.p85);let tc2: f64 = (tc0 / tc1);let tc3: f64 = (tbe + tc2);let tc4: f64 = (l.f645 * tc3);let tc5: f64 = (-230.25850929940458);let tc6: f64 = if tc4 < tc5 { 1.0 } else { 0.0 };l.f2ba = tc6;
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b8 == 0.0)) && (l.f2ba != 0.0)) {let tc7: f64 = (-230.25850929940458);let tc8: f64 = (l.f7b1 / l.f5f1);let tc9: f64 = (l.f5f1 - l.f5ed);let tca: f64 = (l.f793 * tc9);let tcb: f64 = (l.f5ed * p.p85);let tcc: f64 = (tca / tcb);let tcd: f64 = (tc8 + tcc);let tce: f64 = (l.f645 * tcd);let tcf: f64 = (tc7 - tce);let td0: f64 = (-230.25850929940458);let td1: f64 = (l.f7b1 / l.f5f1);let td2: f64 = (l.f5f1 - l.f5ed);let td3: f64 = (l.f793 * td2);let td4: f64 = (l.f5ed * p.p85);let td5: f64 = (td3 / td4);let td6: f64 = (td1 + td5);let td7: f64 = (l.f645 * td6);let td8: f64 = (td0 - td7);let td9: f64 = (-230.25850929940458);let tda: f64 = (l.f7b1 / l.f5f1);let tdb: f64 = (l.f5f1 - l.f5ed);let tdc: f64 = (l.f793 * tdb);let tdd: f64 = (l.f5ed * p.p85);let tde: f64 = (tdc / tdd);let tdf: f64 = (tda + tde);let te0: f64 = (l.f645 * tdf);let te1: f64 = (td9 - te0);let te2: f64 = (te1 * 0.3333333333333333);let te3: f64 = (1.0 + te2);let te4: f64 = (td8 * te3);let te5: f64 = (0.5 * te4);let te6: f64 = (1.0 + te5);let te7: f64 = (tcf * te6);let te8: f64 = (1.0 + te7);let te9: f64 = (1e-100 / te8);(l.f93, l.f94, l.f95, ) = (te9, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tcb) - (tca * (l.f5ee * p.p85))) / (tcb * tcb))))) * te6) + (tcf * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * td4) - (td3 * (l.f5ee * p.p85))) / (td4 * td4))))) * te3) + (td8 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tdd) - (tdc * (l.f5ee * p.p85))) / (tdd * tdd))))) * 0.3333333333333333))))))) / (te8 * te8))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tcb) - (tca * (l.f5ef * p.p85))) / (tcb * tcb))))) * te6) + (tcf * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * td4) - (td3 * (l.f5ef * p.p85))) / (td4 * td4))))) * te3) + (td8 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tdd) - (tdc * (l.f5ef * p.p85))) / (tdd * tdd))))) * 0.3333333333333333))))))) / (te8 * te8))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b8 == 0.0)) && (l.f2ba == 0.0)) {let tea: f64 = (l.f7b1 / l.f5f1);let teb: f64 = (l.f5f1 - l.f5ed);let tec: f64 = (l.f793 * teb);let ted: f64 = (l.f5ed * p.p85);let tee: f64 = (tec / ted);let tef: f64 = (tea + tee);let tf0: f64 = (l.f645 * tef);let tf1: f64 = (tf0 - 230.25850929940458);let tf2: f64 = (l.f7b1 / l.f5f1);let tf3: f64 = (l.f5f1 - l.f5ed);let tf4: f64 = (l.f793 * tf3);let tf5: f64 = (l.f5ed * p.p85);let tf6: f64 = (tf4 / tf5);let tf7: f64 = (tf2 + tf6);let tf8: f64 = (l.f645 * tf7);let tf9: f64 = (tf8 - 230.25850929940458);let tfa: f64 = (l.f7b1 / l.f5f1);let tfb: f64 = (l.f5f1 - l.f5ed);let tfc: f64 = (l.f793 * tfb);let tfd: f64 = (l.f5ed * p.p85);let tfe: f64 = (tfc / tfd);let tff: f64 = (tfa + tfe);let t100: f64 = (l.f645 * tff);let t101: f64 = (t100 - 230.25850929940458);let t102: f64 = (t101 * 0.3333333333333333);let t103: f64 = (1.0 + t102);let t104: f64 = (tf9 * t103);let t105: f64 = (0.5 * t104);let t106: f64 = (1.0 + t105);let t107: f64 = (tf1 * t106);let t108: f64 = (1.0 + t107);let t109: f64 = (1e100 * t108);(l.f93, l.f94, l.f95, ) = (t109, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * ted) - (tec * (l.f5ee * p.p85))) / (ted * ted)))) * t106) + (tf1 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tf5) - (tf4 * (l.f5ee * p.p85))) / (tf5 * tf5)))) * t103) + (tf9 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tfd) - (tfc * (l.f5ee * p.p85))) / (tfd * tfd)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * ted) - (tec * (l.f5ef * p.p85))) / (ted * ted)))) * t106) + (tf1 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tf5) - (tf4 * (l.f5ef * p.p85))) / (tf5 * tf5)))) * t103) + (tf9 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tfd) - (tfc * (l.f5ef * p.p85))) / (tfd * tfd)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) {let t10a: f64 = (l.f7b1 * l.f5b);let t10b: f64 = (l.f5f1 - t10a);let t10c: f64 = (l.f5f1 * l.f5f1);let t10d: f64 = (t10b / t10c);let t10e: f64 = (l.f793 * l.f5b);let t10f: f64 = (l.f5ed * p.p85);let t110: f64 = (t10e / t10f);let t111: f64 = (t10d + t110);let t112: f64 = (l.f645 * t111);(l.f61, l.f62, l.f63, ) = (t112, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t10c) - (t10b * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t10c * t10c)) + ((((l.f793 * l.f5c) * t10f) - (t10e * (l.f5ee * p.p85))) / (t10f * t10f)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t10c) - (t10b * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t10c * t10c)) + ((((l.f793 * l.f5d) * t10f) - (t10e * (l.f5ef * p.p85))) / (t10f * t10f)))), );let t113: f64 = (l.f73d - l.f7b1);let t114: f64 = (t113 * l.f61);let t115: f64 = (1.0 + t114);let t116: f64 = (t115 * l.f93);(l.f53e, l.f53f, l.f540, ) = (t116, (((t113 * l.f62) * l.f93) + (t115 * l.f94)), (((t113 * l.f63) * l.f93) + (t115 * l.f95)), );let t117: f64 = (l.f5eb * l.f5eb);let t118: f64 = (t117 / l.f5e1);l.f64f = t118;let t119: f64 = (l.f5e7 / l.f645);let t11a: f64 = (l.f5e1 / l.f64f);let t11b: f64 = (t11a).ln();let t11c: f64 = (t119 * t11b);l.f793 = t11c;}
        let t11d: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f2bc = t11d;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {let t11e: f64 = (l.f7b1 - l.f793);let t11f: f64 = (p.p86 * t11e);let t120: f64 = (t11f + l.f5e7);(l.f601, l.f602, l.f603, ) = (t120, 0.0, 0.0, );let t121: f64 = (p.p86 * l.f793);let t122: f64 = (l.f5e7 - t121);(l.f5ed, l.f5ee, l.f5ef, ) = (t122, 0.0, 0.0, );let t123: f64 = (p.p85 - l.f601);let t124: f64 = (t123 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t124, (-l.f602), (-l.f603), );let t125: f64 = (4.0 * p.p85);let t126: f64 = (t125 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t126, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {
            let (t128, t129, t12a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t127: f64 = (-l.f6f7);
        (t127, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t128, t129, t12a, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {let t12b: f64 = (l.f6f3 * l.f6f3);let t12c: f64 = (t12b + l.f6f7);let t12d: f64 = (t12c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t12d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t12d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t12d)), );let t12e: f64 = (l.f6f3 / l.f6f7);let t12f: f64 = (1.0 + t12e);let t130: f64 = (0.5 * t12f);(l.f55, l.f56, l.f57, ) = (t130, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t131: f64 = (l.f6f3 + l.f6f7);let t132: f64 = (0.5 * t131);let t133: f64 = (p.p85 - t132);(l.f605, l.f606, l.f607, ) = (t133, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t134: f64 = (l.f605 - l.f5e7);let t135: f64 = (t134 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t135, l.f606, l.f607, );let t136: f64 = (4.0 * l.f5e7);let t137: f64 = (t136 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t137, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {
            let (t139, t13a, t13b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t138: f64 = (-l.f6f7);
        (t138, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t139, t13a, t13b, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {let t13c: f64 = (l.f6f3 * l.f6f3);let t13d: f64 = (t13c + l.f6f7);let t13e: f64 = (t13d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t13e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t13e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t13e)), );let t13f: f64 = (l.f6f3 / l.f6f7);let t140: f64 = (1.0 + t13f);let t141: f64 = (0.5 * t140);(l.f51, l.f52, l.f53, ) = (t141, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t142: f64 = (l.f6f3 + l.f6f7);let t143: f64 = (0.5 * t142);let t144: f64 = (l.f5e7 + t143);(l.f5f1, l.f5f2, l.f5f3, ) = (t144, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t145: f64 = (p.p85 - l.f5ed);let t146: f64 = (t145 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t146, (-l.f5ee), (-l.f5ef), );let t147: f64 = (4.0 * p.p85);let t148: f64 = (t147 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t148, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {
            let (t14a, t14b, t14c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t149: f64 = (-l.f6f7);
        (t149, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t14a, t14b, t14c, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {let t14d: f64 = (l.f6f3 * l.f6f3);let t14e: f64 = (t14d + l.f6f7);let t14f: f64 = (t14e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t14f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t14f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t14f)), );let t150: f64 = (l.f6f3 + l.f6f7);let t151: f64 = (0.5 * t150);let t152: f64 = (p.p85 - t151);(l.f5ed, l.f5ee, l.f5ef, ) = (t152, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t153: f64 = (l.f5ed - l.f5e7);let t154: f64 = (t153 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t154, l.f5ee, l.f5ef, );let t155: f64 = (4.0 * l.f5e7);let t156: f64 = (t155 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t156, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {
            let (t158, t159, t15a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t157: f64 = (-l.f6f7);
        (t157, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t158, t159, t15a, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc != 0.0)) {let t15b: f64 = (l.f6f3 * l.f6f3);let t15c: f64 = (t15b + l.f6f7);let t15d: f64 = (t15c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t15d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t15d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t15d)), );let t15e: f64 = (l.f6f3 + l.f6f7);let t15f: f64 = (0.5 * t15e);let t160: f64 = (l.f5e7 + t15f);(l.f5ed, l.f5ee, l.f5ef, ) = (t160, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t161: f64 = (p.p86 * l.f55);let t162: f64 = (t161 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t162, (((p.p86 * l.f56) * l.f51) + (t161 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t161 * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2bc == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t163: f64 = (l.f7b1 / l.f5f1);let t164: f64 = (l.f5f1 - l.f5ed);let t165: f64 = (l.f793 * t164);let t166: f64 = (l.f5ed * p.p85);let t167: f64 = (t165 / t166);let t168: f64 = (t163 + t167);let t169: f64 = (l.f645 * t168);let t16a: f64 = (t169).abs();let t16b: f64 = if t16a < 230.25850929940458 { 1.0 } else { 0.0 };l.f2be = t16b;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2be != 0.0)) {let t16c: f64 = (l.f7b1 / l.f5f1);let t16d: f64 = (l.f5f1 - l.f5ed);let t16e: f64 = (l.f793 * t16d);let t16f: f64 = (l.f5ed * p.p85);let t170: f64 = (t16e / t16f);let t171: f64 = (t16c + t170);let t172: f64 = (l.f645 * t171);let t173: f64 = (t172).exp();(l.f8e, l.f8f, l.f90, ) = (t173, (t173 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t16f) - (t16e * (l.f5ee * p.p85))) / (t16f * t16f))))), (t173 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t16f) - (t16e * (l.f5ef * p.p85))) / (t16f * t16f))))), );}
        let t174: f64 = (l.f7b1 / l.f5f1);let t175: f64 = (l.f5f1 - l.f5ed);let t176: f64 = (l.f793 * t175);let t177: f64 = (l.f5ed * p.p85);let t178: f64 = (t176 / t177);let t179: f64 = (t174 + t178);let t17a: f64 = (l.f645 * t179);let t17b: f64 = (-230.25850929940458);let t17c: f64 = if t17a < t17b { 1.0 } else { 0.0 };l.f2c0 = t17c;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2be == 0.0)) && (l.f2c0 != 0.0)) {let t17d: f64 = (-230.25850929940458);let t17e: f64 = (l.f7b1 / l.f5f1);let t17f: f64 = (l.f5f1 - l.f5ed);let t180: f64 = (l.f793 * t17f);let t181: f64 = (l.f5ed * p.p85);let t182: f64 = (t180 / t181);let t183: f64 = (t17e + t182);let t184: f64 = (l.f645 * t183);let t185: f64 = (t17d - t184);let t186: f64 = (-230.25850929940458);let t187: f64 = (l.f7b1 / l.f5f1);let t188: f64 = (l.f5f1 - l.f5ed);let t189: f64 = (l.f793 * t188);let t18a: f64 = (l.f5ed * p.p85);let t18b: f64 = (t189 / t18a);let t18c: f64 = (t187 + t18b);let t18d: f64 = (l.f645 * t18c);let t18e: f64 = (t186 - t18d);let t18f: f64 = (-230.25850929940458);let t190: f64 = (l.f7b1 / l.f5f1);let t191: f64 = (l.f5f1 - l.f5ed);let t192: f64 = (l.f793 * t191);let t193: f64 = (l.f5ed * p.p85);let t194: f64 = (t192 / t193);let t195: f64 = (t190 + t194);let t196: f64 = (l.f645 * t195);let t197: f64 = (t18f - t196);let t198: f64 = (t197 * 0.3333333333333333);let t199: f64 = (1.0 + t198);let t19a: f64 = (t18e * t199);let t19b: f64 = (0.5 * t19a);let t19c: f64 = (1.0 + t19b);let t19d: f64 = (t185 * t19c);let t19e: f64 = (1.0 + t19d);let t19f: f64 = (1e-100 / t19e);(l.f8e, l.f8f, l.f90, ) = (t19f, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t181) - (t180 * (l.f5ee * p.p85))) / (t181 * t181))))) * t19c) + (t185 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t18a) - (t189 * (l.f5ee * p.p85))) / (t18a * t18a))))) * t199) + (t18e * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t193) - (t192 * (l.f5ee * p.p85))) / (t193 * t193))))) * 0.3333333333333333))))))) / (t19e * t19e))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t181) - (t180 * (l.f5ef * p.p85))) / (t181 * t181))))) * t19c) + (t185 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t18a) - (t189 * (l.f5ef * p.p85))) / (t18a * t18a))))) * t199) + (t18e * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t193) - (t192 * (l.f5ef * p.p85))) / (t193 * t193))))) * 0.3333333333333333))))))) / (t19e * t19e))), );}
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2be == 0.0)) && (l.f2c0 == 0.0)) {let t1a0: f64 = (l.f7b1 / l.f5f1);let t1a1: f64 = (l.f5f1 - l.f5ed);let t1a2: f64 = (l.f793 * t1a1);let t1a3: f64 = (l.f5ed * p.p85);let t1a4: f64 = (t1a2 / t1a3);let t1a5: f64 = (t1a0 + t1a4);let t1a6: f64 = (l.f645 * t1a5);let t1a7: f64 = (t1a6 - 230.25850929940458);let t1a8: f64 = (l.f7b1 / l.f5f1);let t1a9: f64 = (l.f5f1 - l.f5ed);let t1aa: f64 = (l.f793 * t1a9);let t1ab: f64 = (l.f5ed * p.p85);let t1ac: f64 = (t1aa / t1ab);let t1ad: f64 = (t1a8 + t1ac);let t1ae: f64 = (l.f645 * t1ad);let t1af: f64 = (t1ae - 230.25850929940458);let t1b0: f64 = (l.f7b1 / l.f5f1);let t1b1: f64 = (l.f5f1 - l.f5ed);let t1b2: f64 = (l.f793 * t1b1);let t1b3: f64 = (l.f5ed * p.p85);let t1b4: f64 = (t1b2 / t1b3);let t1b5: f64 = (t1b0 + t1b4);let t1b6: f64 = (l.f645 * t1b5);let t1b7: f64 = (t1b6 - 230.25850929940458);let t1b8: f64 = (t1b7 * 0.3333333333333333);let t1b9: f64 = (1.0 + t1b8);let t1ba: f64 = (t1af * t1b9);let t1bb: f64 = (0.5 * t1ba);let t1bc: f64 = (1.0 + t1bb);let t1bd: f64 = (t1a7 * t1bc);let t1be: f64 = (1.0 + t1bd);let t1bf: f64 = (1e100 * t1be);(l.f8e, l.f8f, l.f90, ) = (t1bf, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1a3) - (t1a2 * (l.f5ee * p.p85))) / (t1a3 * t1a3)))) * t1bc) + (t1a7 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1ab) - (t1aa * (l.f5ee * p.p85))) / (t1ab * t1ab)))) * t1b9) + (t1af * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1b3) - (t1b2 * (l.f5ee * p.p85))) / (t1b3 * t1b3)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1a3) - (t1a2 * (l.f5ef * p.p85))) / (t1a3 * t1a3)))) * t1bc) + (t1a7 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1ab) - (t1aa * (l.f5ef * p.p85))) / (t1ab * t1ab)))) * t1b9) + (t1af * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1b3) - (t1b2 * (l.f5ef * p.p85))) / (t1b3 * t1b3)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) {let t1c0: f64 = (l.f7b1 * l.f5b);let t1c1: f64 = (l.f5f1 - t1c0);let t1c2: f64 = (l.f5f1 * l.f5f1);let t1c3: f64 = (t1c1 / t1c2);let t1c4: f64 = (l.f793 * l.f5b);let t1c5: f64 = (l.f5ed * p.p85);let t1c6: f64 = (t1c4 / t1c5);let t1c7: f64 = (t1c3 + t1c6);let t1c8: f64 = (l.f645 * t1c7);(l.f61, l.f62, l.f63, ) = (t1c8, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t1c2) - (t1c1 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t1c2 * t1c2)) + ((((l.f793 * l.f5c) * t1c5) - (t1c4 * (l.f5ee * p.p85))) / (t1c5 * t1c5)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t1c2) - (t1c1 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t1c2 * t1c2)) + ((((l.f793 * l.f5d) * t1c5) - (t1c4 * (l.f5ef * p.p85))) / (t1c5 * t1c5)))), );let t1c9: f64 = (l.f73d - l.f7b1);let t1ca: f64 = (t1c9 * l.f61);let t1cb: f64 = (1.0 + t1ca);let t1cc: f64 = (t1cb * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t1cc, (((t1c9 * l.f62) * l.f8e) + (t1cb * l.f8f)), (((t1c9 * l.f63) * l.f8e) + (t1cb * l.f90)), );}
        if ((l.f29a != 0.0) && (l.f293 != 0.0)) {let t1cd: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t1cd, l.f537, l.f538, );let t1ce: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t1ce, l.f53f, l.f540, );let t1cf: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t1cf, l.f53b, l.f53c, );let t1d0: f64 = (1.0 / l.f825);l.f817 = t1d0;}
        let t1d1: f64 = if l.f73d > 0.0 { 1.0 } else { 0.0 };l.f2c2 = t1d1;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f2c2 != 0.0)) {let t1d2: f64 = (2.0 + l.f817);let t1d3: f64 = (l.f817 + 1.0);let t1d4: f64 = (l.f817 + 3.0);let t1d5: f64 = (t1d3 * t1d4);let t1d6: f64 = (t1d5).sqrt();let t1d7: f64 = (t1d2 + t1d6);let t1d8: f64 = (t1d7).ln();let t1d9: f64 = (l.f643 * t1d8);let t1da: f64 = (2.0 * t1d9);l.f714 = t1da;}
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f2c2 == 0.0)) {let t1db: f64 = (-l.f73d);let t1dc: f64 = (2.0 * l.f825);let t1dd: f64 = (t1dc + 1.0);let t1de: f64 = (1.0 + l.f825);let t1df: f64 = (3.0 * l.f825);let t1e0: f64 = (1.0 + t1df);let t1e1: f64 = (t1de * t1e0);let t1e2: f64 = (t1e1).sqrt();let t1e3: f64 = (t1dd + t1e2);let t1e4: f64 = (t1e3).ln();let t1e5: f64 = (l.f643 * t1e4);let t1e6: f64 = (2.0 * t1e5);let t1e7: f64 = (t1db + t1e6);l.f714 = t1e7;}
        if ((l.f29a != 0.0) && (l.f293 != 0.0)) {let t1e8: f64 = (l.f76f - l.f714);l.f79c = t1e8;let t1e9: f64 = (l.f73d + l.f79c);let t1ea: f64 = (l.f73d - l.f79c);let t1eb: f64 = (l.f73d - l.f79c);let t1ec: f64 = (t1ea * t1eb);let t1ed: f64 = (4.0 * l.f643);let t1ee: f64 = (t1ed * l.f643);let t1ef: f64 = (t1ec + t1ee);let t1f0: f64 = (t1ef).sqrt();let t1f1: f64 = (t1e9 - t1f0);let t1f2: f64 = (0.5 * t1f1);l.f7a2 = t1f2;let t1f3: f64 = (l.f73d + l.f755);let t1f4: f64 = (l.f73d - l.f755);let t1f5: f64 = (l.f73d - l.f755);let t1f6: f64 = (t1f4 * t1f5);let t1f7: f64 = (4.0 * l.f647);let t1f8: f64 = (t1f7 * l.f647);let t1f9: f64 = (t1f6 + t1f8);let t1fa: f64 = (t1f9).sqrt();let t1fb: f64 = (t1f3 - t1fa);let t1fc: f64 = (0.5 * t1fb);l.f750 = t1fc;let t1fd: f64 = l.f73d;let t1fe: f64 = l.f73d;let t1ff: f64 = l.f73d;let t200: f64 = (t1fe * t1ff);let t201: f64 = (4.0 * 1e-6);let t202: f64 = (t201 * 1e-6);let t203: f64 = (t200 + t202);let t204: f64 = (t203).sqrt();let t205: f64 = (t1fd - t204);let t206: f64 = (0.5 * t205);l.f74a = t206;}
        if ((l.f29a != 0.0) && (l.f293 == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f714 = 0.0;l.f796 = 0.0;l.f825 = 0.0;l.f7a2 = 0.0;l.f750 = 0.0;l.f74a = 0.0;}
        let t207: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f2c4 = t207;
        if ((l.f29a != 0.0) && (l.f2c4 != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );}
        let t208: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f2c6 = t208;
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c6 != 0.0)) {let t209: f64 = (l.f796 * l.f769);let t20a: f64 = (1.0 - t209);let t20b: f64 = (t20a).sqrt();l.f6fc = t20b;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c6 == 0.0)) {let t20c: f64 = (l.f796 * l.f769);let t20d: f64 = (1.0 - t20c);let t20e: f64 = (t20d).powf(l.f60b);l.f6fc = t20e;}
        if ((l.f29a != 0.0) && (l.f2c4 == 0.0)) {let t20f: f64 = (1.0 - l.f6fc);let t210: f64 = (l.f69e * t20f);let t211: f64 = (l.f73d - l.f796);let t212: f64 = (l.f698 * t211);let t213: f64 = (t210 + t212);(l.f68c, l.f68d, l.f68e, ) = (t213, 0.0, 0.0, );let t214: f64 = (l.f542 * l.f536);(l.f52f, l.f530, l.f531, ) = (t214, (l.f542 * l.f537), (l.f542 * l.f538), );}
        let t215: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.f2c8 = t215;
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 != 0.0)) {l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 == 0.0)) {let t216: f64 = (l.f75d - l.f7a2);l.f758 = t216;let t217: f64 = (l.f714 / l.f758);let t218: f64 = (1.0 - t217);let t219: f64 = (t218).sqrt();let t21a: f64 = (1.0 - t219);l.f7ef = t21a;}
        let t21b: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f2ca = t21b;
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 == 0.0)) && (l.f2ca != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 == 0.0)) && (l.f2ca == 0.0)) {let t21c: f64 = (l.f7ef * l.f7ef);let t21d: f64 = (l.f7ef).ln();let t21e: f64 = (t21c * t21d);let t21f: f64 = (1.0 - l.f7ef);let t220: f64 = (t21e / t21f);let t221: f64 = (t220 + l.f7ef);let t222: f64 = (2.0 * l.f623);let t223: f64 = (1.0 - t222);let t224: f64 = (t221 * t223);l.f66 = t224;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 == 0.0)) {let t225: f64 = (l.f7ef + l.f66);l.f7e9 = t225;}
        let t226: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f2cc = t226;
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 == 0.0)) && (l.f2cc != 0.0)) {let t227: f64 = (l.f758 * l.f773);let t228: f64 = (t227).sqrt();l.f6fc = t228;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 == 0.0)) && (l.f2cc == 0.0)) {let t229: f64 = (l.f758 * l.f773);let t22a: f64 = (t229).powf(l.f623);l.f6fc = t22a;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2c8 == 0.0)) {let t22b: f64 = (l.f7d6 * l.f6fc);l.f7d1 = t22b;let t22c: f64 = (l.f825 - 1.0);let t22d: f64 = (t22c * l.f7d1);let t22e: f64 = (l.fc9 * t22d);l.f9 = t22e;let t22f: f64 = (l.f9 * l.f7e9);let t230: f64 = (l.f39 * t22f);l.f593 = t230;}
        let t231: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.f2ce = t231;
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) {let t232: f64 = (l.f7d1 * l.f60b);let t233: f64 = (t232 / l.f758);let t234: f64 = (l.f1e * t233);l.f19 = t234;let t235: f64 = (0.666666666666667 * l.fe);let t236: f64 = (t235 / l.f19);l.f71a = t236;let t237: f64 = (l.f71a * l.f71a);l.f72c = t237;let t238: f64 = (l.f72c * l.f72c);let t239: f64 = (l.f72c * l.f72c);let t23a: f64 = (t239 + 1.0);let t23b: f64 = (t238 / t23a);let t23c: f64 = (t23b).sqrt();l.f726 = t23c;let t23d: f64 = (l.f726).abs();let t23e: f64 = (t23d).sqrt();l.f6c1 = t23e;let t23f: f64 = (l.f726 * l.f6c1);l.f732 = t23f;}
        let t240: f64 = (-l.f623);let t241: f64 = (t240 * l.f611);let t242: f64 = (-1.0);let t243: f64 = if t241 == t242 { 1.0 } else { 0.0 };l.f2d0 = t243;
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d0 != 0.0)) {let t244: f64 = (l.f19 * l.f732);let t245: f64 = (1.0 + t244);let t246: f64 = (1.0 / t245);l.f7e3 = t246;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d0 == 0.0)) {let t247: f64 = (l.f19 * l.f732);let t248: f64 = (1.0 + t247);let t249: f64 = (-l.f623);let t24a: f64 = (t249 * l.f611);let t24b: f64 = (t248).powf(t24a);l.f7e3 = t24b;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) {let t24c: f64 = (l.f7e9 * l.f7e3);let t24d: f64 = (l.f7e9 + l.f7e3);let t24e: f64 = (t24c / t24d);l.f7f5 = t24e;let t24f: f64 = (l.f19 / l.f6c1);let t250: f64 = (0.375 * t24f);let t251: f64 = (t250).sqrt();l.f5a8 = t251;let t252: f64 = (l.f71a * l.f6c1);let t253: f64 = (2.0 * t252);let t254: f64 = (t253 - l.f726);l.f5b4 = t254;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) {let t255: f64 = (l.fe * l.f71a);let t256: f64 = (t255 * l.f6c1);let t257: f64 = (l.fe * l.f726);let t258: f64 = (t256 - t257);let t259: f64 = (l.f19 * l.f732);let t25a: f64 = (0.5 * t259);let t25b: f64 = (t258 + t25a);l.f5d4 = t25b;let t25c: f64 = (l.f5b4 - 1.0);let t25d: f64 = (t25c * l.f5a8);l.f7fb = t25d;let t25e: f64 = (l.f7fb * l.f7fb);l.f811 = t25e;}
        let t25f: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f2d2 = t25f;
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d2 != 0.0)) {let t260: f64 = (l.f62b * l.f7fb);let t261: f64 = (1.0 + t260);let t262: f64 = (1.0 / t261);l.f6e2 = t262;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d2 == 0.0)) {let t263: f64 = (l.f62b * l.f7fb);let t264: f64 = (1.0 - t263);let t265: f64 = (1.0 / t264);l.f6e2 = t265;}
        let t266: f64 = (-l.f811);let t267: f64 = (t266 + l.f5d4);let t268: f64 = (-230.25850929940458);let t269: f64 = if t267 > t268 { 1.0 } else { 0.0 };l.f2d4 = t269;
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d4 != 0.0)) {let t26a: f64 = (-l.f811);let t26b: f64 = (t26a + l.f5d4);let t26c: f64 = (t26b).exp();l.f6fc = t26c;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d4 == 0.0)) {let t26d: f64 = (-230.25850929940458);let t26e: f64 = (-l.f811);let t26f: f64 = (t26e + l.f5d4);let t270: f64 = (t26d - t26f);let t271: f64 = (-230.25850929940458);let t272: f64 = (-l.f811);let t273: f64 = (t272 + l.f5d4);let t274: f64 = (t271 - t273);let t275: f64 = (-230.25850929940458);let t276: f64 = (-l.f811);let t277: f64 = (t276 + l.f5d4);let t278: f64 = (t275 - t277);let t279: f64 = (t278 * 0.3333333333333333);let t27a: f64 = (1.0 + t279);let t27b: f64 = (t274 * t27a);let t27c: f64 = (0.5 * t27b);let t27d: f64 = (1.0 + t27c);let t27e: f64 = (t270 * t27d);let t27f: f64 = (1.0 + t27e);let t280: f64 = (1e-100 / t27f);l.f6fc = t280;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) {let t281: f64 = (0.29214664 * l.f6e2);let t282: f64 = (l.f6e2 * l.f6e2);let t283: f64 = (l.f16 * t282);let t284: f64 = (t281 + t283);let t285: f64 = (l.f6e2 * l.f6e2);let t286: f64 = (t285 * l.f6e2);let t287: f64 = (l.f2a * t286);let t288: f64 = (t284 + t287);let t289: f64 = (t288 * l.f6fc);l.f6e = t289;}
        let t28a: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f2d6 = t28a;
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d6 != 0.0)) {l.f74 = l.f6e;}
        let t28b: f64 = (-230.25850929940458);let t28c: f64 = if l.f5d4 > t28b { 1.0 } else { 0.0 };l.f2d8 = t28c;
        if (((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d6 == 0.0)) && (l.f2d8 != 0.0)) {let t28d: f64 = (l.f5d4).exp();l.f6fc = t28d;}
        if (((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d6 == 0.0)) && (l.f2d8 == 0.0)) {let t28e: f64 = (-230.25850929940458);let t28f: f64 = (t28e - l.f5d4);let t290: f64 = (-230.25850929940458);let t291: f64 = (t290 - l.f5d4);let t292: f64 = (-230.25850929940458);let t293: f64 = (t292 - l.f5d4);let t294: f64 = (t293 * 0.3333333333333333);let t295: f64 = (1.0 + t294);let t296: f64 = (t291 * t295);let t297: f64 = (0.5 * t296);let t298: f64 = (1.0 + t297);let t299: f64 = (t28f * t298);let t29a: f64 = (1.0 + t299);let t29b: f64 = (1e-100 / t29a);l.f6fc = t29b;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) && (l.f2d6 == 0.0)) {let t29c: f64 = (2.0 * l.f6fc);let t29d: f64 = (t29c - l.f6e);l.f74 = t29d;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) {let t29e: f64 = (1.772453850905516 * 0.5);let t29f: f64 = (l.fe * l.f74);let t2a0: f64 = (t29f / l.f5a8);let t2a1: f64 = (t29e * t2a0);l.fd6 = t2a1;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2ce == 0.0)) {let t2a2: f64 = (l.f9 * l.fd6);let t2a3: f64 = (t2a2 * l.f7f5);let t2a4: f64 = (l.f3f * t2a3);l.f599 = t2a4;}
        let t2a5: f64 = if l.f24 == 0.0 { 1.0 } else { 0.0 };l.f2da = t2a5;
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2da != 0.0)) {l.f529 = 0.0;}
        let t2a6: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f2dc = t2a6;
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2da == 0.0)) && (l.f2dc != 0.0)) {let t2a7: f64 = (l.f771 - l.f750);let t2a8: f64 = (t2a7 * l.f773);let t2a9: f64 = (t2a8).sqrt();l.f6fc = t2a9;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2da == 0.0)) && (l.f2dc == 0.0)) {let t2aa: f64 = (l.f771 - l.f750);let t2ab: f64 = (t2aa * l.f773);let t2ac: f64 = (t2ab).powf(l.f623);l.f6fc = t2ac;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2da == 0.0)) {let t2ad: f64 = (l.f771 - l.f750);let t2ae: f64 = (t2ad * l.f7da);let t2af: f64 = (t2ae / l.f6fc);let t2b0: f64 = (l.f611 * t2af);l.fb6 = t2b0;}
        let t2b1: f64 = (-l.fa1);let t2b2: f64 = (t2b1 / l.fb6);let t2b3: f64 = (t2b2).abs();let t2b4: f64 = if t2b3 < 230.25850929940458 { 1.0 } else { 0.0 };l.f2de = t2b4;
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2da == 0.0)) && (l.f2de != 0.0)) {let t2b5: f64 = (-l.fa1);let t2b6: f64 = (t2b5 / l.fb6);let t2b7: f64 = (t2b6).exp();l.f6fc = t2b7;}
        let t2b8: f64 = (-l.fa1);let t2b9: f64 = (t2b8 / l.fb6);let t2ba: f64 = (-230.25850929940458);let t2bb: f64 = if t2b9 < t2ba { 1.0 } else { 0.0 };l.f2e0 = t2bb;
        if (((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2da == 0.0)) && (l.f2de == 0.0)) && (l.f2e0 != 0.0)) {let t2bc: f64 = (-230.25850929940458);let t2bd: f64 = (-l.fa1);let t2be: f64 = (t2bd / l.fb6);let t2bf: f64 = (t2bc - t2be);let t2c0: f64 = (-230.25850929940458);let t2c1: f64 = (-l.fa1);let t2c2: f64 = (t2c1 / l.fb6);let t2c3: f64 = (t2c0 - t2c2);let t2c4: f64 = (-230.25850929940458);let t2c5: f64 = (-l.fa1);let t2c6: f64 = (t2c5 / l.fb6);let t2c7: f64 = (t2c4 - t2c6);let t2c8: f64 = (t2c7 * 0.3333333333333333);let t2c9: f64 = (1.0 + t2c8);let t2ca: f64 = (t2c3 * t2c9);let t2cb: f64 = (0.5 * t2ca);let t2cc: f64 = (1.0 + t2cb);let t2cd: f64 = (t2bf * t2cc);let t2ce: f64 = (1.0 + t2cd);let t2cf: f64 = (1e-100 / t2ce);l.f6fc = t2cf;}
        if (((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2da == 0.0)) && (l.f2de == 0.0)) && (l.f2e0 == 0.0)) {let t2d0: f64 = (-l.fa1);let t2d1: f64 = (t2d0 / l.fb6);let t2d2: f64 = (t2d1 - 230.25850929940458);let t2d3: f64 = (-l.fa1);let t2d4: f64 = (t2d3 / l.fb6);let t2d5: f64 = (t2d4 - 230.25850929940458);let t2d6: f64 = (-l.fa1);let t2d7: f64 = (t2d6 / l.fb6);let t2d8: f64 = (t2d7 - 230.25850929940458);let t2d9: f64 = (t2d8 * 0.3333333333333333);let t2da: f64 = (1.0 + t2d9);let t2db: f64 = (t2d5 * t2da);let t2dc: f64 = (0.5 * t2db);let t2dd: f64 = (1.0 + t2dc);let t2de: f64 = (t2d2 * t2dd);let t2df: f64 = (1.0 + t2de);let t2e0: f64 = (1e100 * t2df);l.f6fc = t2e0;}
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2da == 0.0)) {let t2e1: f64 = (l.f73d * l.fb6);let t2e2: f64 = (t2e1 * l.fb6);let t2e3: f64 = (t2e2 * l.f6fc);let t2e4: f64 = (l.f24 * t2e3);l.f529 = t2e4;}
        let t2e5: f64 = if ((l.f783 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f2e2 = t2e5;
        if (((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2e2 != 0.0)) {l.fae = 1.0;}
        let t2e6: f64 = (-l.f2);let t2e7: f64 = (t2e6 * l.f783);let t2e8: f64 = if l.f74a > t2e7 { 1.0 } else { 0.0 };l.f2e4 = t2e8;let t2e9: f64 = if l.f625 == 4.0 { 1.0 } else { 0.0 };l.f2e6 = t2e9;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2e2 == 0.0)) && (l.f2e4 != 0.0)) && (l.f2e6 != 0.0)) {let t2ea: f64 = (l.f74a * l.f787);let t2eb: f64 = (t2ea).abs();let t2ec: f64 = (l.f74a * l.f787);let t2ed: f64 = (t2ec).abs();let t2ee: f64 = (t2eb * t2ed);let t2ef: f64 = (l.f74a * l.f787);let t2f0: f64 = (t2ef).abs();let t2f1: f64 = (t2ee * t2f0);let t2f2: f64 = (l.f74a * l.f787);let t2f3: f64 = (t2f2).abs();let t2f4: f64 = (t2f1 * t2f3);l.f6fc = t2f4;}
        if (((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2e2 == 0.0)) && (l.f2e4 != 0.0)) && (l.f2e6 == 0.0)) {let t2f5: f64 = (l.f74a * l.f787);let t2f6: f64 = (t2f5).abs();let t2f7: f64 = (t2f6).powf(l.f625);l.f6fc = t2f7;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2e2 == 0.0)) && (l.f2e4 != 0.0)) {let t2f8: f64 = (1.0 - l.f6fc);let t2f9: f64 = (1.0 / t2f8);l.fae = t2f9;}
        if ((((l.f29a != 0.0) && (l.f2c4 == 0.0)) && (l.f2e2 == 0.0)) && (l.f2e4 == 0.0)) {let t2fa: f64 = (l.f2 * l.f783);let t2fb: f64 = (l.f74a + t2fa);let t2fc: f64 = (t2fb * l.f6ba);let t2fd: f64 = (l.fc3 + t2fc);l.fae = t2fd;}
        if ((l.f29a != 0.0) && (l.f2c4 == 0.0)) {let t2fe: f64 = (l.f52f + l.f593);let t2ff: f64 = (t2fe + l.f599);let t300: f64 = (t2ff + l.f529);let t301: f64 = (t300 * l.fae);(l.f562, l.f563, l.f564, ) = (t301, (l.f530 * l.fae), (l.f531 * l.fae), );let t302: f64 = (l.f593 + l.f599);let t303: f64 = (t302 + l.f529);let t304: f64 = (t303 * l.fae);(l.f552, l.f553, l.f554, ) = (t304, 0.0, 0.0, );}
        let t305: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f2e8 = t305;
        if ((l.f29a != 0.0) && (l.f2e8 != 0.0)) {(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );}
        let t306: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f2ea = t306;
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ea != 0.0)) {let t307: f64 = (l.f796 * l.f76d);let t308: f64 = (1.0 - t307);let t309: f64 = (t308).sqrt();l.f6fc = t309;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ea == 0.0)) {let t30a: f64 = (l.f796 * l.f76d);let t30b: f64 = (1.0 - t30a);let t30c: f64 = (t30b).powf(l.f60f);l.f6fc = t30c;}
        if ((l.f29a != 0.0) && (l.f2e8 == 0.0)) {let t30d: f64 = (1.0 - l.f6fc);let t30e: f64 = (l.f6a2 * t30d);let t30f: f64 = (l.f73d - l.f796);let t310: f64 = (l.f69c * t30f);let t311: f64 = (t30e + t310);(l.f694, l.f695, l.f696, ) = (t311, 0.0, 0.0, );let t312: f64 = (l.f54c * l.f53e);(l.f52f, l.f530, l.f531, ) = (t312, (l.f54c * l.f53f), (l.f54c * l.f540), );}
        let t313: f64 = if ((l.f3d == 0.0) && (l.f43 == 0.0)) { 1.0 } else { 0.0 };l.f2ec = t313;
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec == 0.0)) {let t314: f64 = (l.f77d - l.f7a2);l.f758 = t314;let t315: f64 = (l.f714 / l.f758);let t316: f64 = (1.0 - t315);let t317: f64 = (t316).sqrt();let t318: f64 = (1.0 - t317);l.f7ef = t318;}
        let t319: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f2ee = t319;
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec == 0.0)) && (l.f2ee != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec == 0.0)) && (l.f2ee == 0.0)) {let t31a: f64 = (l.f7ef * l.f7ef);let t31b: f64 = (l.f7ef).ln();let t31c: f64 = (t31a * t31b);let t31d: f64 = (1.0 - l.f7ef);let t31e: f64 = (t31c / t31d);let t31f: f64 = (t31e + l.f7ef);let t320: f64 = (2.0 * l.f653);let t321: f64 = (1.0 - t320);let t322: f64 = (t31f * t321);l.f66 = t322;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec == 0.0)) {let t323: f64 = (l.f7ef + l.f66);l.f7e9 = t323;}
        let t324: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f2f0 = t324;
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec == 0.0)) && (l.f2f0 != 0.0)) {let t325: f64 = (l.f758 * l.f77b);let t326: f64 = (t325).sqrt();l.f6fc = t326;}
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec == 0.0)) && (l.f2f0 == 0.0)) {let t327: f64 = (l.f758 * l.f77b);let t328: f64 = (t327).powf(l.f653);l.f6fc = t328;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec == 0.0)) {let t329: f64 = (l.f7e0 * l.f6fc);l.f7d1 = t329;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_107(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2ec == 0.0)) {let t32a: f64 = (l.f825 - 1.0);let t32b: f64 = (t32a * l.f7d1);let t32c: f64 = (l.fd1 * t32b);l.f9 = t32c;let t32d: f64 = (l.f9 * l.f7e9);let t32e: f64 = (l.f3d * t32d);l.f593 = t32e;}
        let t32f: f64 = if l.f43 == 0.0 { 1.0 } else { 0.0 };l.f2f2 = t32f;
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) {let t330: f64 = (l.f7d1 * l.f60f);let t331: f64 = (t330 / l.f758);let t332: f64 = (l.f22 * t331);l.f19 = t332;let t333: f64 = (0.666666666666667 * l.f12);let t334: f64 = (t333 / l.f19);l.f71a = t334;let t335: f64 = (l.f71a * l.f71a);l.f72c = t335;let t336: f64 = (l.f72c * l.f72c);let t337: f64 = (l.f72c * l.f72c);let t338: f64 = (t337 + 1.0);let t339: f64 = (t336 / t338);let t33a: f64 = (t339).sqrt();l.f726 = t33a;let t33b: f64 = (l.f726).abs();let t33c: f64 = (t33b).sqrt();l.f6c1 = t33c;let t33d: f64 = (l.f726 * l.f6c1);l.f732 = t33d;}
        let t33e: f64 = (-l.f653);let t33f: f64 = (t33e * l.f615);let t340: f64 = (-1.0);let t341: f64 = if t33f == t340 { 1.0 } else { 0.0 };l.f2f4 = t341;
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2f4 != 0.0)) {let t342: f64 = (l.f19 * l.f732);let t343: f64 = (1.0 + t342);let t344: f64 = (1.0 / t343);l.f7e3 = t344;}
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2f4 == 0.0)) {let t345: f64 = (l.f19 * l.f732);let t346: f64 = (1.0 + t345);let t347: f64 = (-l.f653);let t348: f64 = (t347 * l.f615);let t349: f64 = (t346).powf(t348);l.f7e3 = t349;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) {let t34a: f64 = (l.f7e9 * l.f7e3);let t34b: f64 = (l.f7e9 + l.f7e3);let t34c: f64 = (t34a / t34b);l.f7f5 = t34c;let t34d: f64 = (l.f19 / l.f6c1);let t34e: f64 = (0.375 * t34d);let t34f: f64 = (t34e).sqrt();l.f5a8 = t34f;let t350: f64 = (l.f71a * l.f6c1);let t351: f64 = (2.0 * t350);let t352: f64 = (t351 - l.f726);l.f5b4 = t352;let t353: f64 = (l.f12 * l.f71a);let t354: f64 = (t353 * l.f6c1);let t355: f64 = (l.f12 * l.f726);let t356: f64 = (t354 - t355);let t357: f64 = (l.f19 * l.f732);let t358: f64 = (0.5 * t357);let t359: f64 = (t356 + t358);l.f5d4 = t359;let t35a: f64 = (l.f5b4 - 1.0);let t35b: f64 = (t35a * l.f5a8);l.f7fb = t35b;let t35c: f64 = (l.f7fb * l.f7fb);l.f811 = t35c;}
        let t35d: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f2f6 = t35d;
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2f6 != 0.0)) {let t35e: f64 = (l.f62b * l.f7fb);let t35f: f64 = (1.0 + t35e);let t360: f64 = (1.0 / t35f);l.f6e2 = t360;}
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2f6 == 0.0)) {let t361: f64 = (l.f62b * l.f7fb);let t362: f64 = (1.0 - t361);let t363: f64 = (1.0 / t362);l.f6e2 = t363;}
        let t364: f64 = (-l.f811);let t365: f64 = (t364 + l.f5d4);let t366: f64 = (-230.25850929940458);let t367: f64 = if t365 > t366 { 1.0 } else { 0.0 };l.f2f8 = t367;
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2f8 != 0.0)) {let t368: f64 = (-l.f811);let t369: f64 = (t368 + l.f5d4);let t36a: f64 = (t369).exp();l.f6fc = t36a;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_108(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2f8 == 0.0)) {let t36b: f64 = (-230.25850929940458);let t36c: f64 = (-l.f811);let t36d: f64 = (t36c + l.f5d4);let t36e: f64 = (t36b - t36d);let t36f: f64 = (-230.25850929940458);let t370: f64 = (-l.f811);let t371: f64 = (t370 + l.f5d4);let t372: f64 = (t36f - t371);let t373: f64 = (-230.25850929940458);let t374: f64 = (-l.f811);let t375: f64 = (t374 + l.f5d4);let t376: f64 = (t373 - t375);let t377: f64 = (t376 * 0.3333333333333333);let t378: f64 = (1.0 + t377);let t379: f64 = (t372 * t378);let t37a: f64 = (0.5 * t379);let t37b: f64 = (1.0 + t37a);let t37c: f64 = (t36e * t37b);let t37d: f64 = (1.0 + t37c);let t37e: f64 = (1e-100 / t37d);l.f6fc = t37e;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) {let t37f: f64 = (0.29214664 * l.f6e2);let t380: f64 = (l.f6e2 * l.f6e2);let t381: f64 = (l.f16 * t380);let t382: f64 = (t37f + t381);let t383: f64 = (l.f6e2 * l.f6e2);let t384: f64 = (t383 * l.f6e2);let t385: f64 = (l.f2a * t384);let t386: f64 = (t382 + t385);let t387: f64 = (t386 * l.f6fc);l.f6e = t387;}
        let t388: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f2fa = t388;
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2fa != 0.0)) {l.f74 = l.f6e;}
        let t389: f64 = (-230.25850929940458);let t38a: f64 = if l.f5d4 > t389 { 1.0 } else { 0.0 };l.f2fc = t38a;
        if (((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2fa == 0.0)) && (l.f2fc != 0.0)) {let t38b: f64 = (l.f5d4).exp();l.f6fc = t38b;}
        if (((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2fa == 0.0)) && (l.f2fc == 0.0)) {let t38c: f64 = (-230.25850929940458);let t38d: f64 = (t38c - l.f5d4);let t38e: f64 = (-230.25850929940458);let t38f: f64 = (t38e - l.f5d4);let t390: f64 = (-230.25850929940458);let t391: f64 = (t390 - l.f5d4);let t392: f64 = (t391 * 0.3333333333333333);let t393: f64 = (1.0 + t392);let t394: f64 = (t38f * t393);let t395: f64 = (0.5 * t394);let t396: f64 = (1.0 + t395);let t397: f64 = (t38d * t396);let t398: f64 = (1.0 + t397);let t399: f64 = (1e-100 / t398);l.f6fc = t399;}
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) && (l.f2fa == 0.0)) {let t39a: f64 = (2.0 * l.f6fc);let t39b: f64 = (t39a - l.f6e);l.f74 = t39b;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2f2 == 0.0)) {let t39c: f64 = (1.772453850905516 * 0.5);let t39d: f64 = (l.f12 * l.f74);let t39e: f64 = (t39d / l.f5a8);let t39f: f64 = (t39c * t39e);l.fd6 = t39f;let t3a0: f64 = (l.f9 * l.fd6);let t3a1: f64 = (t3a0 * l.f7f5);let t3a2: f64 = (l.f43 * t3a1);l.f599 = t3a2;}
        let t3a3: f64 = if l.f28 == 0.0 { 1.0 } else { 0.0 };l.f2fe = t3a3;
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2fe != 0.0)) {l.f529 = 0.0;}
        let t3a4: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f300 = t3a4;
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2fe == 0.0)) && (l.f300 != 0.0)) {let t3a5: f64 = (l.f779 - l.f750);let t3a6: f64 = (t3a5 * l.f77b);let t3a7: f64 = (t3a6).sqrt();l.f6fc = t3a7;}
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2fe == 0.0)) && (l.f300 == 0.0)) {let t3a8: f64 = (l.f779 - l.f750);let t3a9: f64 = (t3a8 * l.f77b);let t3aa: f64 = (t3a9).powf(l.f653);l.f6fc = t3aa;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2fe == 0.0)) {let t3ab: f64 = (l.f779 - l.f750);let t3ac: f64 = (t3ab * l.f7de);let t3ad: f64 = (t3ac / l.f6fc);let t3ae: f64 = (l.f615 * t3ad);l.fb6 = t3ae;}
        let t3af: f64 = (-l.fab);let t3b0: f64 = (t3af / l.fb6);let t3b1: f64 = (t3b0).abs();let t3b2: f64 = if t3b1 < 230.25850929940458 { 1.0 } else { 0.0 };l.f302 = t3b2;
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2fe == 0.0)) && (l.f302 != 0.0)) {let t3b3: f64 = (-l.fab);let t3b4: f64 = (t3b3 / l.fb6);let t3b5: f64 = (t3b4).exp();l.f6fc = t3b5;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_109(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t3b6: f64 = (-l.fab);let t3b7: f64 = (t3b6 / l.fb6);let t3b8: f64 = (-230.25850929940458);let t3b9: f64 = if t3b7 < t3b8 { 1.0 } else { 0.0 };l.f304 = t3b9;
        if (((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2fe == 0.0)) && (l.f302 == 0.0)) && (l.f304 != 0.0)) {let t3ba: f64 = (-230.25850929940458);let t3bb: f64 = (-l.fab);let t3bc: f64 = (t3bb / l.fb6);let t3bd: f64 = (t3ba - t3bc);let t3be: f64 = (-230.25850929940458);let t3bf: f64 = (-l.fab);let t3c0: f64 = (t3bf / l.fb6);let t3c1: f64 = (t3be - t3c0);let t3c2: f64 = (-230.25850929940458);let t3c3: f64 = (-l.fab);let t3c4: f64 = (t3c3 / l.fb6);let t3c5: f64 = (t3c2 - t3c4);let t3c6: f64 = (t3c5 * 0.3333333333333333);let t3c7: f64 = (1.0 + t3c6);let t3c8: f64 = (t3c1 * t3c7);let t3c9: f64 = (0.5 * t3c8);let t3ca: f64 = (1.0 + t3c9);let t3cb: f64 = (t3bd * t3ca);let t3cc: f64 = (1.0 + t3cb);let t3cd: f64 = (1e-100 / t3cc);l.f6fc = t3cd;}
        if (((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2fe == 0.0)) && (l.f302 == 0.0)) && (l.f304 == 0.0)) {let t3ce: f64 = (-l.fab);let t3cf: f64 = (t3ce / l.fb6);let t3d0: f64 = (t3cf - 230.25850929940458);let t3d1: f64 = (-l.fab);let t3d2: f64 = (t3d1 / l.fb6);let t3d3: f64 = (t3d2 - 230.25850929940458);let t3d4: f64 = (-l.fab);let t3d5: f64 = (t3d4 / l.fb6);let t3d6: f64 = (t3d5 - 230.25850929940458);let t3d7: f64 = (t3d6 * 0.3333333333333333);let t3d8: f64 = (1.0 + t3d7);let t3d9: f64 = (t3d3 * t3d8);let t3da: f64 = (0.5 * t3d9);let t3db: f64 = (1.0 + t3da);let t3dc: f64 = (t3d0 * t3db);let t3dd: f64 = (1.0 + t3dc);let t3de: f64 = (1e100 * t3dd);l.f6fc = t3de;}
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f2fe == 0.0)) {let t3df: f64 = (l.f73d * l.fb6);let t3e0: f64 = (t3df * l.fb6);let t3e1: f64 = (t3e0 * l.f6fc);let t3e2: f64 = (l.f28 * t3e1);l.f529 = t3e2;}
        let t3e3: f64 = if ((l.f78d > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f306 = t3e3;
        if (((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f306 != 0.0)) {l.fae = 1.0;}
        let t3e4: f64 = (-l.f2);let t3e5: f64 = (t3e4 * l.f78d);let t3e6: f64 = if l.f74a > t3e5 { 1.0 } else { 0.0 };l.f308 = t3e6;let t3e7: f64 = if l.f629 == 4.0 { 1.0 } else { 0.0 };l.f30a = t3e7;
        if (((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f306 == 0.0)) && (l.f308 != 0.0)) && (l.f30a != 0.0)) {let t3e8: f64 = (l.f74a * l.f78b);let t3e9: f64 = (t3e8).abs();let t3ea: f64 = (l.f74a * l.f78b);let t3eb: f64 = (t3ea).abs();let t3ec: f64 = (t3e9 * t3eb);let t3ed: f64 = (l.f74a * l.f78b);let t3ee: f64 = (t3ed).abs();let t3ef: f64 = (t3ec * t3ee);let t3f0: f64 = (l.f74a * l.f78b);let t3f1: f64 = (t3f0).abs();let t3f2: f64 = (t3ef * t3f1);l.f6fc = t3f2;}
        if (((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f306 == 0.0)) && (l.f308 != 0.0)) && (l.f30a == 0.0)) {let t3f3: f64 = (l.f74a * l.f78b);let t3f4: f64 = (t3f3).abs();let t3f5: f64 = (t3f4).powf(l.f629);l.f6fc = t3f5;}
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f306 == 0.0)) && (l.f308 != 0.0)) {let t3f6: f64 = (1.0 - l.f6fc);let t3f7: f64 = (1.0 / t3f6);l.fae = t3f7;}
        if ((((l.f29a != 0.0) && (l.f2e8 == 0.0)) && (l.f306 == 0.0)) && (l.f308 == 0.0)) {let t3f8: f64 = (l.f2 * l.f78d);let t3f9: f64 = (l.f74a + t3f8);let t3fa: f64 = (t3f9 * l.f6be);let t3fb: f64 = (l.fc7 + t3fa);l.fae = t3fb;}
        if ((l.f29a != 0.0) && (l.f2e8 == 0.0)) {let t3fc: f64 = (l.f52f + l.f593);let t3fd: f64 = (t3fc + l.f599);let t3fe: f64 = (t3fd + l.f529);let t3ff: f64 = (t3fe * l.fae);(l.f576, l.f577, l.f578, ) = (t3ff, (l.f530 * l.fae), (l.f531 * l.fae), );let t400: f64 = (l.f593 + l.f599);let t401: f64 = (t400 + l.f529);let t402: f64 = (t401 * l.fae);(l.f55a, l.f55b, l.f55c, ) = (t402, 0.0, 0.0, );}
        let t403: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f30c = t403;
        if ((l.f29a != 0.0) && (l.f30c != 0.0)) {(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_110(
        l: &mut StampLocals,
    ) {
        let t404: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f30e = t404;
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f30e != 0.0)) {let t405: f64 = (l.f796 * l.f76b);let t406: f64 = (1.0 - t405);let t407: f64 = (t406).sqrt();l.f6fc = t407;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f30e == 0.0)) {let t408: f64 = (l.f796 * l.f76b);let t409: f64 = (1.0 - t408);let t40a: f64 = (t409).powf(l.f60d);l.f6fc = t40a;}
        if ((l.f29a != 0.0) && (l.f30c == 0.0)) {let t40b: f64 = (1.0 - l.f6fc);let t40c: f64 = (l.f6a0 * t40b);let t40d: f64 = (l.f73d - l.f796);let t40e: f64 = (l.f69a * t40d);let t40f: f64 = (t40c + t40e);(l.f690, l.f691, l.f692, ) = (t40f, 0.0, 0.0, );let t410: f64 = (l.f544 * l.f53a);(l.f52f, l.f530, l.f531, ) = (t410, (l.f544 * l.f53b), (l.f544 * l.f53c), );}
        let t411: f64 = if ((l.f3b == 0.0) && (l.f41 == 0.0)) { 1.0 } else { 0.0 };l.f310 = t411;
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f310 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f310 == 0.0)) {let t412: f64 = (l.f763 - l.f7a2);l.f758 = t412;let t413: f64 = (l.f714 / l.f758);let t414: f64 = (1.0 - t413);let t415: f64 = (t414).sqrt();let t416: f64 = (1.0 - t415);l.f7ef = t416;}
        let t417: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f312 = t417;
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f310 == 0.0)) && (l.f312 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f310 == 0.0)) && (l.f312 == 0.0)) {let t418: f64 = (l.f7ef * l.f7ef);let t419: f64 = (l.f7ef).ln();let t41a: f64 = (t418 * t419);let t41b: f64 = (1.0 - l.f7ef);let t41c: f64 = (t41a / t41b);let t41d: f64 = (t41c + l.f7ef);let t41e: f64 = (2.0 * l.f62f);let t41f: f64 = (1.0 - t41e);let t420: f64 = (t41d * t41f);l.f66 = t420;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f310 == 0.0)) {let t421: f64 = (l.f7ef + l.f66);l.f7e9 = t421;}
        let t422: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f314 = t422;
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f310 == 0.0)) && (l.f314 != 0.0)) {let t423: f64 = (l.f758 * l.f777);let t424: f64 = (t423).sqrt();l.f6fc = t424;}
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f310 == 0.0)) && (l.f314 == 0.0)) {let t425: f64 = (l.f758 * l.f777);let t426: f64 = (t425).powf(l.f62f);l.f6fc = t426;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f310 == 0.0)) {let t427: f64 = (l.f7d8 * l.f6fc);l.f7d1 = t427;let t428: f64 = (l.f825 - 1.0);let t429: f64 = (t428 * l.f7d1);let t42a: f64 = (l.fcd * t429);l.f9 = t42a;let t42b: f64 = (l.f9 * l.f7e9);let t42c: f64 = (l.f3b * t42b);l.f593 = t42c;}
        let t42d: f64 = if l.f41 == 0.0 { 1.0 } else { 0.0 };l.f316 = t42d;
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) {let t42e: f64 = (l.f7d1 * l.f60d);let t42f: f64 = (t42e / l.f758);let t430: f64 = (l.f20 * t42f);l.f19 = t430;let t431: f64 = (0.666666666666667 * l.f10);let t432: f64 = (t431 / l.f19);l.f71a = t432;let t433: f64 = (l.f71a * l.f71a);l.f72c = t433;let t434: f64 = (l.f72c * l.f72c);let t435: f64 = (l.f72c * l.f72c);let t436: f64 = (t435 + 1.0);let t437: f64 = (t434 / t436);let t438: f64 = (t437).sqrt();l.f726 = t438;let t439: f64 = (l.f726).abs();let t43a: f64 = (t439).sqrt();l.f6c1 = t43a;let t43b: f64 = (l.f726 * l.f6c1);l.f732 = t43b;}
        let t43c: f64 = (-l.f62f);let t43d: f64 = (t43c * l.f613);let t43e: f64 = (-1.0);let t43f: f64 = if t43d == t43e { 1.0 } else { 0.0 };l.f318 = t43f;
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f318 != 0.0)) {let t440: f64 = (l.f19 * l.f732);let t441: f64 = (1.0 + t440);let t442: f64 = (1.0 / t441);l.f7e3 = t442;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_111(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f318 == 0.0)) {let t443: f64 = (l.f19 * l.f732);let t444: f64 = (1.0 + t443);let t445: f64 = (-l.f62f);let t446: f64 = (t445 * l.f613);let t447: f64 = (t444).powf(t446);l.f7e3 = t447;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) {let t448: f64 = (l.f7e9 * l.f7e3);let t449: f64 = (l.f7e9 + l.f7e3);let t44a: f64 = (t448 / t449);l.f7f5 = t44a;let t44b: f64 = (l.f19 / l.f6c1);let t44c: f64 = (0.375 * t44b);let t44d: f64 = (t44c).sqrt();l.f5a8 = t44d;let t44e: f64 = (l.f71a * l.f6c1);let t44f: f64 = (2.0 * t44e);let t450: f64 = (t44f - l.f726);l.f5b4 = t450;let t451: f64 = (l.f10 * l.f71a);let t452: f64 = (t451 * l.f6c1);let t453: f64 = (l.f10 * l.f726);let t454: f64 = (t452 - t453);let t455: f64 = (l.f19 * l.f732);let t456: f64 = (0.5 * t455);let t457: f64 = (t454 + t456);l.f5d4 = t457;let t458: f64 = (l.f5b4 - 1.0);let t459: f64 = (t458 * l.f5a8);l.f7fb = t459;let t45a: f64 = (l.f7fb * l.f7fb);l.f811 = t45a;}
        let t45b: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f31a = t45b;
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f31a != 0.0)) {let t45c: f64 = (l.f62b * l.f7fb);let t45d: f64 = (1.0 + t45c);let t45e: f64 = (1.0 / t45d);l.f6e2 = t45e;}
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f31a == 0.0)) {let t45f: f64 = (l.f62b * l.f7fb);let t460: f64 = (1.0 - t45f);let t461: f64 = (1.0 / t460);l.f6e2 = t461;}
        let t462: f64 = (-l.f811);let t463: f64 = (t462 + l.f5d4);let t464: f64 = (-230.25850929940458);let t465: f64 = if t463 > t464 { 1.0 } else { 0.0 };l.f31c = t465;
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f31c != 0.0)) {let t466: f64 = (-l.f811);let t467: f64 = (t466 + l.f5d4);let t468: f64 = (t467).exp();l.f6fc = t468;}
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f31c == 0.0)) {let t469: f64 = (-230.25850929940458);let t46a: f64 = (-l.f811);let t46b: f64 = (t46a + l.f5d4);let t46c: f64 = (t469 - t46b);let t46d: f64 = (-230.25850929940458);let t46e: f64 = (-l.f811);let t46f: f64 = (t46e + l.f5d4);let t470: f64 = (t46d - t46f);let t471: f64 = (-230.25850929940458);let t472: f64 = (-l.f811);let t473: f64 = (t472 + l.f5d4);let t474: f64 = (t471 - t473);let t475: f64 = (t474 * 0.3333333333333333);let t476: f64 = (1.0 + t475);let t477: f64 = (t470 * t476);let t478: f64 = (0.5 * t477);let t479: f64 = (1.0 + t478);let t47a: f64 = (t46c * t479);let t47b: f64 = (1.0 + t47a);let t47c: f64 = (1e-100 / t47b);l.f6fc = t47c;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) {let t47d: f64 = (0.29214664 * l.f6e2);let t47e: f64 = (l.f6e2 * l.f6e2);let t47f: f64 = (l.f16 * t47e);let t480: f64 = (t47d + t47f);let t481: f64 = (l.f6e2 * l.f6e2);let t482: f64 = (t481 * l.f6e2);let t483: f64 = (l.f2a * t482);let t484: f64 = (t480 + t483);let t485: f64 = (t484 * l.f6fc);l.f6e = t485;}
        let t486: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f31e = t486;
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f31e != 0.0)) {l.f74 = l.f6e;}
        let t487: f64 = (-230.25850929940458);let t488: f64 = if l.f5d4 > t487 { 1.0 } else { 0.0 };l.f320 = t488;
        if (((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f31e == 0.0)) && (l.f320 != 0.0)) {let t489: f64 = (l.f5d4).exp();l.f6fc = t489;}
    }
}
