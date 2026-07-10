#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_81(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t0: f64 = (l.f73b / l.f5f1);let t1: f64 = (l.f5f1 - l.f5ed);let t2: f64 = (l.f793 * t1);let t3: f64 = (l.f5ed * p.p85);let t4: f64 = (t2 / t3);let t5: f64 = (t0 + t4);let t6: f64 = (l.f645 * t5);let t7: f64 = (t6).abs();let t8: f64 = if t7 < 230.25850929940458 { 1.0 } else { 0.0 };l.f1fc = t8;l.f1fd = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fc != 0.0)) {let t9: f64 = (l.f73b / l.f5f1);let ta: f64 = (l.f5f1 - l.f5ed);let tb: f64 = (l.f793 * ta);let tc: f64 = (l.f5ed * p.p85);let td: f64 = (tb / tc);let te: f64 = (t9 + td);let tf: f64 = (l.f645 * te);let t10: f64 = (tf).exp();(l.f53e, l.f53f, l.f540, ) = (t10, (t10 * (l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tc) - (tb * (l.f5ee * p.p85))) / (tc * tc))))), (t10 * (l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tc) - (tb * (l.f5ef * p.p85))) / (tc * tc))))), );l.f541 = 0.0;}
        let t11: f64 = (l.f73b / l.f5f1);let t12: f64 = (l.f5f1 - l.f5ed);let t13: f64 = (l.f793 * t12);let t14: f64 = (l.f5ed * p.p85);let t15: f64 = (t13 / t14);let t16: f64 = (t11 + t15);let t17: f64 = (l.f645 * t16);let t18: f64 = (-230.25850929940458);let t19: f64 = if t17 < t18 { 1.0 } else { 0.0 };l.f1fe = t19;l.f1ff = 0.0;
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fc == 0.0)) && (l.f1fe != 0.0)) {let t1a: f64 = (-230.25850929940458);let t1b: f64 = (l.f73b / l.f5f1);let t1c: f64 = (l.f5f1 - l.f5ed);let t1d: f64 = (l.f793 * t1c);let t1e: f64 = (l.f5ed * p.p85);let t1f: f64 = (t1d / t1e);let t20: f64 = (t1b + t1f);let t21: f64 = (l.f645 * t20);let t22: f64 = (t1a - t21);let t23: f64 = (-230.25850929940458);let t24: f64 = (l.f73b / l.f5f1);let t25: f64 = (l.f5f1 - l.f5ed);let t26: f64 = (l.f793 * t25);let t27: f64 = (l.f5ed * p.p85);let t28: f64 = (t26 / t27);let t29: f64 = (t24 + t28);let t2a: f64 = (l.f645 * t29);let t2b: f64 = (t23 - t2a);let t2c: f64 = (-230.25850929940458);let t2d: f64 = (l.f73b / l.f5f1);let t2e: f64 = (l.f5f1 - l.f5ed);let t2f: f64 = (l.f793 * t2e);let t30: f64 = (l.f5ed * p.p85);let t31: f64 = (t2f / t30);let t32: f64 = (t2d + t31);let t33: f64 = (l.f645 * t32);let t34: f64 = (t2c - t33);let t35: f64 = (t34 * 0.3333333333333333);let t36: f64 = (1.0 + t35);let t37: f64 = (t2b * t36);let t38: f64 = (0.5 * t37);let t39: f64 = (1.0 + t38);let t3a: f64 = (t22 * t39);let t3b: f64 = (1.0 + t3a);let t3c: f64 = (1e-100 / t3b);(l.f53e, l.f53f, l.f540, ) = (t3c, (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e) - (t1d * (l.f5ee * p.p85))) / (t1e * t1e))))) * t39) + (t22 * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t27) - (t26 * (l.f5ee * p.p85))) / (t27 * t27))))) * t36) + (t2b * ((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t30) - (t2f * (l.f5ee * p.p85))) / (t30 * t30))))) * 0.3333333333333333))))))) / (t3b * t3b))), (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e) - (t1d * (l.f5ef * p.p85))) / (t1e * t1e))))) * t39) + (t22 * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t27) - (t26 * (l.f5ef * p.p85))) / (t27 * t27))))) * t36) + (t2b * ((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t30) - (t2f * (l.f5ef * p.p85))) / (t30 * t30))))) * 0.3333333333333333))))))) / (t3b * t3b))), );l.f541 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_82(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fc == 0.0)) && (l.f1fe == 0.0)) {let t3d: f64 = (l.f73b / l.f5f1);let t3e: f64 = (l.f5f1 - l.f5ed);let t3f: f64 = (l.f793 * t3e);let t40: f64 = (l.f5ed * p.p85);let t41: f64 = (t3f / t40);let t42: f64 = (t3d + t41);let t43: f64 = (l.f645 * t42);let t44: f64 = (t43 - 230.25850929940458);let t45: f64 = (l.f73b / l.f5f1);let t46: f64 = (l.f5f1 - l.f5ed);let t47: f64 = (l.f793 * t46);let t48: f64 = (l.f5ed * p.p85);let t49: f64 = (t47 / t48);let t4a: f64 = (t45 + t49);let t4b: f64 = (l.f645 * t4a);let t4c: f64 = (t4b - 230.25850929940458);let t4d: f64 = (l.f73b / l.f5f1);let t4e: f64 = (l.f5f1 - l.f5ed);let t4f: f64 = (l.f793 * t4e);let t50: f64 = (l.f5ed * p.p85);let t51: f64 = (t4f / t50);let t52: f64 = (t4d + t51);let t53: f64 = (l.f645 * t52);let t54: f64 = (t53 - 230.25850929940458);let t55: f64 = (t54 * 0.3333333333333333);let t56: f64 = (1.0 + t55);let t57: f64 = (t4c * t56);let t58: f64 = (0.5 * t57);let t59: f64 = (1.0 + t58);let t5a: f64 = (t44 * t59);let t5b: f64 = (1.0 + t5a);let t5c: f64 = (1e100 * t5b);(l.f53e, l.f53f, l.f540, ) = (t5c, (1e100 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t40) - (t3f * (l.f5ee * p.p85))) / (t40 * t40)))) * t59) + (t44 * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t48) - (t47 * (l.f5ee * p.p85))) / (t48 * t48)))) * t56) + (t4c * ((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t50) - (t4f * (l.f5ee * p.p85))) / (t50 * t50)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t40) - (t3f * (l.f5ef * p.p85))) / (t40 * t40)))) * t59) + (t44 * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t48) - (t47 * (l.f5ef * p.p85))) / (t48 * t48)))) * t56) + (t4c * ((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t50) - (t4f * (l.f5ef * p.p85))) / (t50 * t50)))) * 0.3333333333333333))))))), );l.f541 = 0.0;}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) {let t5d: f64 = (l.f5eb * l.f5eb);let t5e: f64 = (t5d / l.f5e1);l.f64f = t5e;l.f650 = 0.0;let t5f: f64 = (l.f5e7 / l.f645);let t60: f64 = (l.f5e1 / l.f64f);let t61: f64 = (t60).ln();let t62: f64 = (t5f * t61);l.f793 = t62;l.f794 = 0.0;}
        let t63: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f201 = t63;l.f202 = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t64: f64 = (l.f73b - l.f793);let t65: f64 = (p.p86 * t64);let t66: f64 = (t65 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t66, 0.0, 0.0, );l.f604 = 0.0;let t67: f64 = (p.p86 * l.f793);let t68: f64 = (l.f5e7 - t67);(l.f5ed, l.f5ee, l.f5ef, ) = (t68, 0.0, 0.0, );l.f5f0 = 0.0;let t69: f64 = (p.p85 - l.f601);let t6a: f64 = (t69 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t6a, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t6b: f64 = (4.0 * p.p85);let t6c: f64 = (t6b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t6c, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {
            let (t6e, t6f, t70,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t6d: f64 = (-l.f6f7);
        (t6d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t6e, t6f, t70, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t71: f64 = (l.f6f3 * l.f6f3);let t72: f64 = (t71 + l.f6f7);let t73: f64 = (t72).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t73, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t73)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t73)), );l.f6fa = 0.0;let t74: f64 = (l.f6f3 + l.f6f7);let t75: f64 = (0.5 * t74);let t76: f64 = (p.p85 - t75);(l.f605, l.f606, l.f607, ) = (t76, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t77: f64 = (l.f605 - l.f5e7);let t78: f64 = (t77 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t78, l.f606, l.f607, );l.f6f6 = 0.0;let t79: f64 = (4.0 * l.f5e7);let t7a: f64 = (t79 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t7a, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {
            let (t7c, t7d, t7e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t7b: f64 = (-l.f6f7);
        (t7b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t7c, t7d, t7e, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t7f: f64 = (l.f6f3 * l.f6f3);let t80: f64 = (t7f + l.f6f7);let t81: f64 = (t80).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t81, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t81)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t81)), );l.f6fa = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_83(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t82: f64 = (l.f6f3 + l.f6f7);let t83: f64 = (0.5 * t82);let t84: f64 = (l.f5e7 + t83);(l.f5f1, l.f5f2, l.f5f3, ) = (t84, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t85: f64 = (p.p85 - l.f5ed);let t86: f64 = (t85 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t86, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t87: f64 = (4.0 * p.p85);let t88: f64 = (t87 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t88, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {
            let (t8a, t8b, t8c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t89: f64 = (-l.f6f7);
        (t89, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t8a, t8b, t8c, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t8d: f64 = (l.f6f3 * l.f6f3);let t8e: f64 = (t8d + l.f6f7);let t8f: f64 = (t8e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t8f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t8f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t8f)), );l.f6fa = 0.0;let t90: f64 = (l.f6f3 + l.f6f7);let t91: f64 = (0.5 * t90);let t92: f64 = (p.p85 - t91);(l.f5ed, l.f5ee, l.f5ef, ) = (t92, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t93: f64 = (l.f5ed - l.f5e7);let t94: f64 = (t93 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t94, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t95: f64 = (4.0 * l.f5e7);let t96: f64 = (t95 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t96, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {
            let (t98, t99, t9a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t97: f64 = (-l.f6f7);
        (t97, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t98, t99, t9a, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t9b: f64 = (l.f6f3 * l.f6f3);let t9c: f64 = (t9b + l.f6f7);let t9d: f64 = (t9c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t9d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t9d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t9d)), );l.f6fa = 0.0;let t9e: f64 = (l.f6f3 + l.f6f7);let t9f: f64 = (0.5 * t9e);let ta0: f64 = (l.f5e7 + t9f);(l.f5ed, l.f5ee, l.f5ef, ) = (ta0, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );l.f5f4 = 0.0;}
        let ta1: f64 = (l.f73b / l.f5f1);let ta2: f64 = (l.f5f1 - l.f5ed);let ta3: f64 = (l.f793 * ta2);let ta4: f64 = (l.f5ed * p.p85);let ta5: f64 = (ta3 / ta4);let ta6: f64 = (ta1 + ta5);let ta7: f64 = (l.f645 * ta6);let ta8: f64 = (ta7).abs();let ta9: f64 = if ta8 < 230.25850929940458 { 1.0 } else { 0.0 };l.f203 = ta9;l.f204 = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f203 != 0.0)) {let taa: f64 = (l.f73b / l.f5f1);let tab: f64 = (l.f5f1 - l.f5ed);let tac: f64 = (l.f793 * tab);let tad: f64 = (l.f5ed * p.p85);let tae: f64 = (tac / tad);let taf: f64 = (taa + tae);let tb0: f64 = (l.f645 * taf);let tb1: f64 = (tb0).exp();(l.f53a, l.f53b, l.f53c, ) = (tb1, (tb1 * (l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tad) - (tac * (l.f5ee * p.p85))) / (tad * tad))))), (tb1 * (l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tad) - (tac * (l.f5ef * p.p85))) / (tad * tad))))), );l.f53d = 0.0;}
        let tb2: f64 = (l.f73b / l.f5f1);let tb3: f64 = (l.f5f1 - l.f5ed);let tb4: f64 = (l.f793 * tb3);let tb5: f64 = (l.f5ed * p.p85);let tb6: f64 = (tb4 / tb5);let tb7: f64 = (tb2 + tb6);let tb8: f64 = (l.f645 * tb7);let tb9: f64 = (-230.25850929940458);let tba: f64 = if tb8 < tb9 { 1.0 } else { 0.0 };l.f205 = tba;l.f206 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_84(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f203 == 0.0)) && (l.f205 != 0.0)) {let tbb: f64 = (-230.25850929940458);let tbc: f64 = (l.f73b / l.f5f1);let tbd: f64 = (l.f5f1 - l.f5ed);let tbe: f64 = (l.f793 * tbd);let tbf: f64 = (l.f5ed * p.p85);let tc0: f64 = (tbe / tbf);let tc1: f64 = (tbc + tc0);let tc2: f64 = (l.f645 * tc1);let tc3: f64 = (tbb - tc2);let tc4: f64 = (-230.25850929940458);let tc5: f64 = (l.f73b / l.f5f1);let tc6: f64 = (l.f5f1 - l.f5ed);let tc7: f64 = (l.f793 * tc6);let tc8: f64 = (l.f5ed * p.p85);let tc9: f64 = (tc7 / tc8);let tca: f64 = (tc5 + tc9);let tcb: f64 = (l.f645 * tca);let tcc: f64 = (tc4 - tcb);let tcd: f64 = (-230.25850929940458);let tce: f64 = (l.f73b / l.f5f1);let tcf: f64 = (l.f5f1 - l.f5ed);let td0: f64 = (l.f793 * tcf);let td1: f64 = (l.f5ed * p.p85);let td2: f64 = (td0 / td1);let td3: f64 = (tce + td2);let td4: f64 = (l.f645 * td3);let td5: f64 = (tcd - td4);let td6: f64 = (td5 * 0.3333333333333333);let td7: f64 = (1.0 + td6);let td8: f64 = (tcc * td7);let td9: f64 = (0.5 * td8);let tda: f64 = (1.0 + td9);let tdb: f64 = (tc3 * tda);let tdc: f64 = (1.0 + tdb);let tdd: f64 = (1e-100 / tdc);(l.f53a, l.f53b, l.f53c, ) = (tdd, (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tbf) - (tbe * (l.f5ee * p.p85))) / (tbf * tbf))))) * tda) + (tc3 * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tc8) - (tc7 * (l.f5ee * p.p85))) / (tc8 * tc8))))) * td7) + (tcc * ((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * td1) - (td0 * (l.f5ee * p.p85))) / (td1 * td1))))) * 0.3333333333333333))))))) / (tdc * tdc))), (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tbf) - (tbe * (l.f5ef * p.p85))) / (tbf * tbf))))) * tda) + (tc3 * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tc8) - (tc7 * (l.f5ef * p.p85))) / (tc8 * tc8))))) * td7) + (tcc * ((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * td1) - (td0 * (l.f5ef * p.p85))) / (td1 * td1))))) * 0.3333333333333333))))))) / (tdc * tdc))), );l.f53d = 0.0;}
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f203 == 0.0)) && (l.f205 == 0.0)) {let tde: f64 = (l.f73b / l.f5f1);let tdf: f64 = (l.f5f1 - l.f5ed);let te0: f64 = (l.f793 * tdf);let te1: f64 = (l.f5ed * p.p85);let te2: f64 = (te0 / te1);let te3: f64 = (tde + te2);let te4: f64 = (l.f645 * te3);let te5: f64 = (te4 - 230.25850929940458);let te6: f64 = (l.f73b / l.f5f1);let te7: f64 = (l.f5f1 - l.f5ed);let te8: f64 = (l.f793 * te7);let te9: f64 = (l.f5ed * p.p85);let tea: f64 = (te8 / te9);let teb: f64 = (te6 + tea);let tec: f64 = (l.f645 * teb);let ted: f64 = (tec - 230.25850929940458);let tee: f64 = (l.f73b / l.f5f1);let tef: f64 = (l.f5f1 - l.f5ed);let tf0: f64 = (l.f793 * tef);let tf1: f64 = (l.f5ed * p.p85);let tf2: f64 = (tf0 / tf1);let tf3: f64 = (tee + tf2);let tf4: f64 = (l.f645 * tf3);let tf5: f64 = (tf4 - 230.25850929940458);let tf6: f64 = (tf5 * 0.3333333333333333);let tf7: f64 = (1.0 + tf6);let tf8: f64 = (ted * tf7);let tf9: f64 = (0.5 * tf8);let tfa: f64 = (1.0 + tf9);let tfb: f64 = (te5 * tfa);let tfc: f64 = (1.0 + tfb);let tfd: f64 = (1e100 * tfc);(l.f53a, l.f53b, l.f53c, ) = (tfd, (1e100 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * te1) - (te0 * (l.f5ee * p.p85))) / (te1 * te1)))) * tfa) + (te5 * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * te9) - (te8 * (l.f5ee * p.p85))) / (te9 * te9)))) * tf7) + (ted * ((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tf1) - (tf0 * (l.f5ee * p.p85))) / (tf1 * tf1)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * te1) - (te0 * (l.f5ef * p.p85))) / (te1 * te1)))) * tfa) + (te5 * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * te9) - (te8 * (l.f5ef * p.p85))) / (te9 * te9)))) * tf7) + (ted * ((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tf1) - (tf0 * (l.f5ef * p.p85))) / (tf1 * tf1)))) * 0.3333333333333333))))))), );l.f53d = 0.0;}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) {let tfe: f64 = (l.f73b - l.f7b1);let tff: f64 = (tfe * l.f645);let t100: f64 = (1.0 + tff);let t101: f64 = (t100 * l.f89);let t102: f64 = (t101).sqrt();l.f825 = t102;l.f826 = 0.0;let t103: f64 = (l.f5eb * l.f5eb);let t104: f64 = (t103 / l.f5df);l.f64f = t104;l.f650 = 0.0;let t105: f64 = (l.f5e5 / l.f645);let t106: f64 = (l.f5df / l.f64f);let t107: f64 = (t106).ln();let t108: f64 = (t105 * t107);l.f793 = t108;l.f794 = 0.0;}
        let t109: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f207 = t109;l.f208 = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t10a: f64 = (l.f7b1 - l.f793);let t10b: f64 = (p.p86 * t10a);let t10c: f64 = (t10b + l.f5e5);(l.f601, l.f602, l.f603, ) = (t10c, 0.0, 0.0, );l.f604 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_85(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t10d: f64 = (p.p86 * l.f793);let t10e: f64 = (l.f5e5 - t10d);(l.f5ed, l.f5ee, l.f5ef, ) = (t10e, 0.0, 0.0, );l.f5f0 = 0.0;let t10f: f64 = (p.p85 - l.f601);let t110: f64 = (t10f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t110, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t111: f64 = (4.0 * p.p85);let t112: f64 = (t111 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t112, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {
            let (t114, t115, t116,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t113: f64 = (-l.f6f7);
        (t113, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t114, t115, t116, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t117: f64 = (l.f6f3 * l.f6f3);let t118: f64 = (t117 + l.f6f7);let t119: f64 = (t118).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t119, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t119)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t119)), );l.f6fa = 0.0;let t11a: f64 = (l.f6f3 / l.f6f7);let t11b: f64 = (1.0 + t11a);let t11c: f64 = (0.5 * t11b);(l.f55, l.f56, l.f57, ) = (t11c, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;let t11d: f64 = (l.f6f3 + l.f6f7);let t11e: f64 = (0.5 * t11d);let t11f: f64 = (p.p85 - t11e);(l.f605, l.f606, l.f607, ) = (t11f, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t120: f64 = (l.f605 - l.f5e5);let t121: f64 = (t120 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t121, l.f606, l.f607, );l.f6f6 = 0.0;let t122: f64 = (4.0 * l.f5e5);let t123: f64 = (t122 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t123, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {
            let (t125, t126, t127,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t124: f64 = (-l.f6f7);
        (t124, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t125, t126, t127, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t128: f64 = (l.f6f3 * l.f6f3);let t129: f64 = (t128 + l.f6f7);let t12a: f64 = (t129).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t12a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t12a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t12a)), );l.f6fa = 0.0;let t12b: f64 = (l.f6f3 / l.f6f7);let t12c: f64 = (1.0 + t12b);let t12d: f64 = (0.5 * t12c);(l.f51, l.f52, l.f53, ) = (t12d, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t12e: f64 = (l.f6f3 + l.f6f7);let t12f: f64 = (0.5 * t12e);let t130: f64 = (l.f5e5 + t12f);(l.f5f1, l.f5f2, l.f5f3, ) = (t130, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t131: f64 = (p.p85 - l.f5ed);let t132: f64 = (t131 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t132, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t133: f64 = (4.0 * p.p85);let t134: f64 = (t133 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t134, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {
            let (t136, t137, t138,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t135: f64 = (-l.f6f7);
        (t135, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t136, t137, t138, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t139: f64 = (l.f6f3 * l.f6f3);let t13a: f64 = (t139 + l.f6f7);let t13b: f64 = (t13a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t13b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t13b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t13b)), );l.f6fa = 0.0;let t13c: f64 = (l.f6f3 + l.f6f7);let t13d: f64 = (0.5 * t13c);let t13e: f64 = (p.p85 - t13d);(l.f5ed, l.f5ee, l.f5ef, ) = (t13e, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t13f: f64 = (l.f5ed - l.f5e5);let t140: f64 = (t13f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t140, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t141: f64 = (4.0 * l.f5e5);let t142: f64 = (t141 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t142, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {
            let (t144, t145, t146,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t143: f64 = (-l.f6f7);
        (t143, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t144, t145, t146, );l.f6fa = 0.0;
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_86(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t147: f64 = (l.f6f3 * l.f6f3);let t148: f64 = (t147 + l.f6f7);let t149: f64 = (t148).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t149, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t149)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t149)), );l.f6fa = 0.0;let t14a: f64 = (l.f6f3 + l.f6f7);let t14b: f64 = (0.5 * t14a);let t14c: f64 = (l.f5e5 + t14b);(l.f5ed, l.f5ee, l.f5ef, ) = (t14c, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t14d: f64 = (p.p86 * l.f55);let t14e: f64 = (t14d * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t14e, (((p.p86 * l.f56) * l.f51) + (t14d * l.f52)), (((p.p86 * l.f57) * l.f51) + (t14d * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
        let t14f: f64 = (l.f7b1 / l.f5f1);let t150: f64 = (l.f5f1 - l.f5ed);let t151: f64 = (l.f793 * t150);let t152: f64 = (l.f5ed * p.p85);let t153: f64 = (t151 / t152);let t154: f64 = (t14f + t153);let t155: f64 = (l.f645 * t154);let t156: f64 = (t155).abs();let t157: f64 = if t156 < 230.25850929940458 { 1.0 } else { 0.0 };l.f209 = t157;l.f20a = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f209 != 0.0)) {let t158: f64 = (l.f7b1 / l.f5f1);let t159: f64 = (l.f5f1 - l.f5ed);let t15a: f64 = (l.f793 * t159);let t15b: f64 = (l.f5ed * p.p85);let t15c: f64 = (t15a / t15b);let t15d: f64 = (t158 + t15c);let t15e: f64 = (l.f645 * t15d);let t15f: f64 = (t15e).exp();(l.f8a, l.f8b, l.f8c, ) = (t15f, (t15f * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t15b) - (t15a * (l.f5ee * p.p85))) / (t15b * t15b))))), (t15f * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t15b) - (t15a * (l.f5ef * p.p85))) / (t15b * t15b))))), );l.f8d = 0.0;}
        let t160: f64 = (l.f7b1 / l.f5f1);let t161: f64 = (l.f5f1 - l.f5ed);let t162: f64 = (l.f793 * t161);let t163: f64 = (l.f5ed * p.p85);let t164: f64 = (t162 / t163);let t165: f64 = (t160 + t164);let t166: f64 = (l.f645 * t165);let t167: f64 = (-230.25850929940458);let t168: f64 = if t166 < t167 { 1.0 } else { 0.0 };l.f20b = t168;l.f20c = 0.0;
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f209 == 0.0)) && (l.f20b != 0.0)) {let t169: f64 = (-230.25850929940458);let t16a: f64 = (l.f7b1 / l.f5f1);let t16b: f64 = (l.f5f1 - l.f5ed);let t16c: f64 = (l.f793 * t16b);let t16d: f64 = (l.f5ed * p.p85);let t16e: f64 = (t16c / t16d);let t16f: f64 = (t16a + t16e);let t170: f64 = (l.f645 * t16f);let t171: f64 = (t169 - t170);let t172: f64 = (-230.25850929940458);let t173: f64 = (l.f7b1 / l.f5f1);let t174: f64 = (l.f5f1 - l.f5ed);let t175: f64 = (l.f793 * t174);let t176: f64 = (l.f5ed * p.p85);let t177: f64 = (t175 / t176);let t178: f64 = (t173 + t177);let t179: f64 = (l.f645 * t178);let t17a: f64 = (t172 - t179);let t17b: f64 = (-230.25850929940458);let t17c: f64 = (l.f7b1 / l.f5f1);let t17d: f64 = (l.f5f1 - l.f5ed);let t17e: f64 = (l.f793 * t17d);let t17f: f64 = (l.f5ed * p.p85);let t180: f64 = (t17e / t17f);let t181: f64 = (t17c + t180);let t182: f64 = (l.f645 * t181);let t183: f64 = (t17b - t182);let t184: f64 = (t183 * 0.3333333333333333);let t185: f64 = (1.0 + t184);let t186: f64 = (t17a * t185);let t187: f64 = (0.5 * t186);let t188: f64 = (1.0 + t187);let t189: f64 = (t171 * t188);let t18a: f64 = (1.0 + t189);let t18b: f64 = (1e-100 / t18a);(l.f8a, l.f8b, l.f8c, ) = (t18b, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t16d) - (t16c * (l.f5ee * p.p85))) / (t16d * t16d))))) * t188) + (t171 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t176) - (t175 * (l.f5ee * p.p85))) / (t176 * t176))))) * t185) + (t17a * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t17f) - (t17e * (l.f5ee * p.p85))) / (t17f * t17f))))) * 0.3333333333333333))))))) / (t18a * t18a))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t16d) - (t16c * (l.f5ef * p.p85))) / (t16d * t16d))))) * t188) + (t171 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t176) - (t175 * (l.f5ef * p.p85))) / (t176 * t176))))) * t185) + (t17a * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t17f) - (t17e * (l.f5ef * p.p85))) / (t17f * t17f))))) * 0.3333333333333333))))))) / (t18a * t18a))), );l.f8d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_87(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f209 == 0.0)) && (l.f20b == 0.0)) {let t18c: f64 = (l.f7b1 / l.f5f1);let t18d: f64 = (l.f5f1 - l.f5ed);let t18e: f64 = (l.f793 * t18d);let t18f: f64 = (l.f5ed * p.p85);let t190: f64 = (t18e / t18f);let t191: f64 = (t18c + t190);let t192: f64 = (l.f645 * t191);let t193: f64 = (t192 - 230.25850929940458);let t194: f64 = (l.f7b1 / l.f5f1);let t195: f64 = (l.f5f1 - l.f5ed);let t196: f64 = (l.f793 * t195);let t197: f64 = (l.f5ed * p.p85);let t198: f64 = (t196 / t197);let t199: f64 = (t194 + t198);let t19a: f64 = (l.f645 * t199);let t19b: f64 = (t19a - 230.25850929940458);let t19c: f64 = (l.f7b1 / l.f5f1);let t19d: f64 = (l.f5f1 - l.f5ed);let t19e: f64 = (l.f793 * t19d);let t19f: f64 = (l.f5ed * p.p85);let t1a0: f64 = (t19e / t19f);let t1a1: f64 = (t19c + t1a0);let t1a2: f64 = (l.f645 * t1a1);let t1a3: f64 = (t1a2 - 230.25850929940458);let t1a4: f64 = (t1a3 * 0.3333333333333333);let t1a5: f64 = (1.0 + t1a4);let t1a6: f64 = (t19b * t1a5);let t1a7: f64 = (0.5 * t1a6);let t1a8: f64 = (1.0 + t1a7);let t1a9: f64 = (t193 * t1a8);let t1aa: f64 = (1.0 + t1a9);let t1ab: f64 = (1e100 * t1aa);(l.f8a, l.f8b, l.f8c, ) = (t1ab, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t18f) - (t18e * (l.f5ee * p.p85))) / (t18f * t18f)))) * t1a8) + (t193 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t197) - (t196 * (l.f5ee * p.p85))) / (t197 * t197)))) * t1a5) + (t19b * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t19f) - (t19e * (l.f5ee * p.p85))) / (t19f * t19f)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t18f) - (t18e * (l.f5ef * p.p85))) / (t18f * t18f)))) * t1a8) + (t193 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t197) - (t196 * (l.f5ef * p.p85))) / (t197 * t197)))) * t1a5) + (t19b * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t19f) - (t19e * (l.f5ef * p.p85))) / (t19f * t19f)))) * 0.3333333333333333))))))), );l.f8d = 0.0;}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) {let t1ac: f64 = (l.f7b1 * l.f5b);let t1ad: f64 = (l.f5f1 - t1ac);let t1ae: f64 = (l.f5f1 * l.f5f1);let t1af: f64 = (t1ad / t1ae);let t1b0: f64 = (l.f793 * l.f5b);let t1b1: f64 = (l.f5ed * p.p85);let t1b2: f64 = (t1b0 / t1b1);let t1b3: f64 = (t1af + t1b2);let t1b4: f64 = (l.f645 * t1b3);(l.f61, l.f62, l.f63, ) = (t1b4, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t1ae) - (t1ad * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t1ae * t1ae)) + ((((l.f793 * l.f5c) * t1b1) - (t1b0 * (l.f5ee * p.p85))) / (t1b1 * t1b1)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t1ae) - (t1ad * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t1ae * t1ae)) + ((((l.f793 * l.f5d) * t1b1) - (t1b0 * (l.f5ef * p.p85))) / (t1b1 * t1b1)))), );l.f64 = 0.0;let t1b5: f64 = (l.f73b - l.f7b1);let t1b6: f64 = (t1b5 * l.f61);let t1b7: f64 = (1.0 + t1b6);let t1b8: f64 = (t1b7 * l.f8a);(l.f536, l.f537, l.f538, ) = (t1b8, (((t1b5 * l.f62) * l.f8a) + (t1b7 * l.f8b)), (((t1b5 * l.f63) * l.f8a) + (t1b7 * l.f8c)), );l.f539 = 0.0;let t1b9: f64 = (l.f5eb * l.f5eb);let t1ba: f64 = (t1b9 / l.f5e3);l.f64f = t1ba;l.f650 = 0.0;let t1bb: f64 = (l.f5e9 / l.f645);let t1bc: f64 = (l.f5e3 / l.f64f);let t1bd: f64 = (t1bc).ln();let t1be: f64 = (t1bb * t1bd);l.f793 = t1be;l.f794 = 0.0;}
        let t1bf: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f20d = t1bf;l.f20e = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t1c0: f64 = (l.f7b1 - l.f793);let t1c1: f64 = (p.p86 * t1c0);let t1c2: f64 = (t1c1 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t1c2, 0.0, 0.0, );l.f604 = 0.0;let t1c3: f64 = (p.p86 * l.f793);let t1c4: f64 = (l.f5e9 - t1c3);(l.f5ed, l.f5ee, l.f5ef, ) = (t1c4, 0.0, 0.0, );l.f5f0 = 0.0;let t1c5: f64 = (p.p85 - l.f601);let t1c6: f64 = (t1c5 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1c6, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t1c7: f64 = (4.0 * p.p85);let t1c8: f64 = (t1c7 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1c8, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {
            let (t1ca, t1cb, t1cc,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1c9: f64 = (-l.f6f7);
        (t1c9, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1ca, t1cb, t1cc, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t1cd: f64 = (l.f6f3 * l.f6f3);let t1ce: f64 = (t1cd + l.f6f7);let t1cf: f64 = (t1ce).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1cf, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1cf)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1cf)), );l.f6fa = 0.0;let t1d0: f64 = (l.f6f3 / l.f6f7);let t1d1: f64 = (1.0 + t1d0);let t1d2: f64 = (0.5 * t1d1);(l.f55, l.f56, l.f57, ) = (t1d2, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_88(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t1d3: f64 = (l.f6f3 + l.f6f7);let t1d4: f64 = (0.5 * t1d3);let t1d5: f64 = (p.p85 - t1d4);(l.f605, l.f606, l.f607, ) = (t1d5, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t1d6: f64 = (l.f605 - l.f5e9);let t1d7: f64 = (t1d6 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1d7, l.f606, l.f607, );l.f6f6 = 0.0;let t1d8: f64 = (4.0 * l.f5e9);let t1d9: f64 = (t1d8 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1d9, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {
            let (t1db, t1dc, t1dd,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1da: f64 = (-l.f6f7);
        (t1da, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1db, t1dc, t1dd, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t1de: f64 = (l.f6f3 * l.f6f3);let t1df: f64 = (t1de + l.f6f7);let t1e0: f64 = (t1df).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1e0, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1e0)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1e0)), );l.f6fa = 0.0;let t1e1: f64 = (l.f6f3 / l.f6f7);let t1e2: f64 = (1.0 + t1e1);let t1e3: f64 = (0.5 * t1e2);(l.f51, l.f52, l.f53, ) = (t1e3, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t1e4: f64 = (l.f6f3 + l.f6f7);let t1e5: f64 = (0.5 * t1e4);let t1e6: f64 = (l.f5e9 + t1e5);(l.f5f1, l.f5f2, l.f5f3, ) = (t1e6, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t1e7: f64 = (p.p85 - l.f5ed);let t1e8: f64 = (t1e7 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1e8, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t1e9: f64 = (4.0 * p.p85);let t1ea: f64 = (t1e9 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1ea, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {
            let (t1ec, t1ed, t1ee,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1eb: f64 = (-l.f6f7);
        (t1eb, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1ec, t1ed, t1ee, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t1ef: f64 = (l.f6f3 * l.f6f3);let t1f0: f64 = (t1ef + l.f6f7);let t1f1: f64 = (t1f0).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1f1, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1f1)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1f1)), );l.f6fa = 0.0;let t1f2: f64 = (l.f6f3 + l.f6f7);let t1f3: f64 = (0.5 * t1f2);let t1f4: f64 = (p.p85 - t1f3);(l.f5ed, l.f5ee, l.f5ef, ) = (t1f4, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t1f5: f64 = (l.f5ed - l.f5e9);let t1f6: f64 = (t1f5 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1f6, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t1f7: f64 = (4.0 * l.f5e9);let t1f8: f64 = (t1f7 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1f8, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {
            let (t1fa, t1fb, t1fc,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1f9: f64 = (-l.f6f7);
        (t1f9, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1fa, t1fb, t1fc, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t1fd: f64 = (l.f6f3 * l.f6f3);let t1fe: f64 = (t1fd + l.f6f7);let t1ff: f64 = (t1fe).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1ff, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1ff)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1ff)), );l.f6fa = 0.0;let t200: f64 = (l.f6f3 + l.f6f7);let t201: f64 = (0.5 * t200);let t202: f64 = (l.f5e9 + t201);(l.f5ed, l.f5ee, l.f5ef, ) = (t202, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t203: f64 = (p.p86 * l.f55);let t204: f64 = (t203 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t204, (((p.p86 * l.f56) * l.f51) + (t203 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t203 * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_89(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t205: f64 = (l.f7b1 / l.f5f1);let t206: f64 = (l.f5f1 - l.f5ed);let t207: f64 = (l.f793 * t206);let t208: f64 = (l.f5ed * p.p85);let t209: f64 = (t207 / t208);let t20a: f64 = (t205 + t209);let t20b: f64 = (l.f645 * t20a);let t20c: f64 = (t20b).abs();let t20d: f64 = if t20c < 230.25850929940458 { 1.0 } else { 0.0 };l.f20f = t20d;l.f210 = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20f != 0.0)) {let t20e: f64 = (l.f7b1 / l.f5f1);let t20f: f64 = (l.f5f1 - l.f5ed);let t210: f64 = (l.f793 * t20f);let t211: f64 = (l.f5ed * p.p85);let t212: f64 = (t210 / t211);let t213: f64 = (t20e + t212);let t214: f64 = (l.f645 * t213);let t215: f64 = (t214).exp();(l.f93, l.f94, l.f95, ) = (t215, (t215 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t211) - (t210 * (l.f5ee * p.p85))) / (t211 * t211))))), (t215 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t211) - (t210 * (l.f5ef * p.p85))) / (t211 * t211))))), );l.f96 = 0.0;}
        let t216: f64 = (l.f7b1 / l.f5f1);let t217: f64 = (l.f5f1 - l.f5ed);let t218: f64 = (l.f793 * t217);let t219: f64 = (l.f5ed * p.p85);let t21a: f64 = (t218 / t219);let t21b: f64 = (t216 + t21a);let t21c: f64 = (l.f645 * t21b);let t21d: f64 = (-230.25850929940458);let t21e: f64 = if t21c < t21d { 1.0 } else { 0.0 };l.f211 = t21e;l.f212 = 0.0;
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20f == 0.0)) && (l.f211 != 0.0)) {let t21f: f64 = (-230.25850929940458);let t220: f64 = (l.f7b1 / l.f5f1);let t221: f64 = (l.f5f1 - l.f5ed);let t222: f64 = (l.f793 * t221);let t223: f64 = (l.f5ed * p.p85);let t224: f64 = (t222 / t223);let t225: f64 = (t220 + t224);let t226: f64 = (l.f645 * t225);let t227: f64 = (t21f - t226);let t228: f64 = (-230.25850929940458);let t229: f64 = (l.f7b1 / l.f5f1);let t22a: f64 = (l.f5f1 - l.f5ed);let t22b: f64 = (l.f793 * t22a);let t22c: f64 = (l.f5ed * p.p85);let t22d: f64 = (t22b / t22c);let t22e: f64 = (t229 + t22d);let t22f: f64 = (l.f645 * t22e);let t230: f64 = (t228 - t22f);let t231: f64 = (-230.25850929940458);let t232: f64 = (l.f7b1 / l.f5f1);let t233: f64 = (l.f5f1 - l.f5ed);let t234: f64 = (l.f793 * t233);let t235: f64 = (l.f5ed * p.p85);let t236: f64 = (t234 / t235);let t237: f64 = (t232 + t236);let t238: f64 = (l.f645 * t237);let t239: f64 = (t231 - t238);let t23a: f64 = (t239 * 0.3333333333333333);let t23b: f64 = (1.0 + t23a);let t23c: f64 = (t230 * t23b);let t23d: f64 = (0.5 * t23c);let t23e: f64 = (1.0 + t23d);let t23f: f64 = (t227 * t23e);let t240: f64 = (1.0 + t23f);let t241: f64 = (1e-100 / t240);(l.f93, l.f94, l.f95, ) = (t241, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t223) - (t222 * (l.f5ee * p.p85))) / (t223 * t223))))) * t23e) + (t227 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t22c) - (t22b * (l.f5ee * p.p85))) / (t22c * t22c))))) * t23b) + (t230 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t235) - (t234 * (l.f5ee * p.p85))) / (t235 * t235))))) * 0.3333333333333333))))))) / (t240 * t240))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t223) - (t222 * (l.f5ef * p.p85))) / (t223 * t223))))) * t23e) + (t227 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t22c) - (t22b * (l.f5ef * p.p85))) / (t22c * t22c))))) * t23b) + (t230 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t235) - (t234 * (l.f5ef * p.p85))) / (t235 * t235))))) * 0.3333333333333333))))))) / (t240 * t240))), );l.f96 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_90(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20f == 0.0)) && (l.f211 == 0.0)) {let t242: f64 = (l.f7b1 / l.f5f1);let t243: f64 = (l.f5f1 - l.f5ed);let t244: f64 = (l.f793 * t243);let t245: f64 = (l.f5ed * p.p85);let t246: f64 = (t244 / t245);let t247: f64 = (t242 + t246);let t248: f64 = (l.f645 * t247);let t249: f64 = (t248 - 230.25850929940458);let t24a: f64 = (l.f7b1 / l.f5f1);let t24b: f64 = (l.f5f1 - l.f5ed);let t24c: f64 = (l.f793 * t24b);let t24d: f64 = (l.f5ed * p.p85);let t24e: f64 = (t24c / t24d);let t24f: f64 = (t24a + t24e);let t250: f64 = (l.f645 * t24f);let t251: f64 = (t250 - 230.25850929940458);let t252: f64 = (l.f7b1 / l.f5f1);let t253: f64 = (l.f5f1 - l.f5ed);let t254: f64 = (l.f793 * t253);let t255: f64 = (l.f5ed * p.p85);let t256: f64 = (t254 / t255);let t257: f64 = (t252 + t256);let t258: f64 = (l.f645 * t257);let t259: f64 = (t258 - 230.25850929940458);let t25a: f64 = (t259 * 0.3333333333333333);let t25b: f64 = (1.0 + t25a);let t25c: f64 = (t251 * t25b);let t25d: f64 = (0.5 * t25c);let t25e: f64 = (1.0 + t25d);let t25f: f64 = (t249 * t25e);let t260: f64 = (1.0 + t25f);let t261: f64 = (1e100 * t260);(l.f93, l.f94, l.f95, ) = (t261, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t245) - (t244 * (l.f5ee * p.p85))) / (t245 * t245)))) * t25e) + (t249 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t24d) - (t24c * (l.f5ee * p.p85))) / (t24d * t24d)))) * t25b) + (t251 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t255) - (t254 * (l.f5ee * p.p85))) / (t255 * t255)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t245) - (t244 * (l.f5ef * p.p85))) / (t245 * t245)))) * t25e) + (t249 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t24d) - (t24c * (l.f5ef * p.p85))) / (t24d * t24d)))) * t25b) + (t251 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t255) - (t254 * (l.f5ef * p.p85))) / (t255 * t255)))) * 0.3333333333333333))))))), );l.f96 = 0.0;}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) {let t262: f64 = (l.f7b1 * l.f5b);let t263: f64 = (l.f5f1 - t262);let t264: f64 = (l.f5f1 * l.f5f1);let t265: f64 = (t263 / t264);let t266: f64 = (l.f793 * l.f5b);let t267: f64 = (l.f5ed * p.p85);let t268: f64 = (t266 / t267);let t269: f64 = (t265 + t268);let t26a: f64 = (l.f645 * t269);(l.f61, l.f62, l.f63, ) = (t26a, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t264) - (t263 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t264 * t264)) + ((((l.f793 * l.f5c) * t267) - (t266 * (l.f5ee * p.p85))) / (t267 * t267)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t264) - (t263 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t264 * t264)) + ((((l.f793 * l.f5d) * t267) - (t266 * (l.f5ef * p.p85))) / (t267 * t267)))), );l.f64 = 0.0;let t26b: f64 = (l.f73b - l.f7b1);let t26c: f64 = (t26b * l.f61);let t26d: f64 = (1.0 + t26c);let t26e: f64 = (t26d * l.f93);(l.f53e, l.f53f, l.f540, ) = (t26e, (((t26b * l.f62) * l.f93) + (t26d * l.f94)), (((t26b * l.f63) * l.f93) + (t26d * l.f95)), );l.f541 = 0.0;let t26f: f64 = (l.f5eb * l.f5eb);let t270: f64 = (t26f / l.f5e1);l.f64f = t270;l.f650 = 0.0;let t271: f64 = (l.f5e7 / l.f645);let t272: f64 = (l.f5e1 / l.f64f);let t273: f64 = (t272).ln();let t274: f64 = (t271 * t273);l.f793 = t274;l.f794 = 0.0;}
        let t275: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f213 = t275;l.f214 = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t276: f64 = (l.f7b1 - l.f793);let t277: f64 = (p.p86 * t276);let t278: f64 = (t277 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t278, 0.0, 0.0, );l.f604 = 0.0;let t279: f64 = (p.p86 * l.f793);let t27a: f64 = (l.f5e7 - t279);(l.f5ed, l.f5ee, l.f5ef, ) = (t27a, 0.0, 0.0, );l.f5f0 = 0.0;let t27b: f64 = (p.p85 - l.f601);let t27c: f64 = (t27b - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t27c, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t27d: f64 = (4.0 * p.p85);let t27e: f64 = (t27d * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t27e, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {
            let (t280, t281, t282,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t27f: f64 = (-l.f6f7);
        (t27f, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t280, t281, t282, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t283: f64 = (l.f6f3 * l.f6f3);let t284: f64 = (t283 + l.f6f7);let t285: f64 = (t284).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t285, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t285)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t285)), );l.f6fa = 0.0;let t286: f64 = (l.f6f3 / l.f6f7);let t287: f64 = (1.0 + t286);let t288: f64 = (0.5 * t287);(l.f55, l.f56, l.f57, ) = (t288, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_91(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t289: f64 = (l.f6f3 + l.f6f7);let t28a: f64 = (0.5 * t289);let t28b: f64 = (p.p85 - t28a);(l.f605, l.f606, l.f607, ) = (t28b, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t28c: f64 = (l.f605 - l.f5e7);let t28d: f64 = (t28c - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t28d, l.f606, l.f607, );l.f6f6 = 0.0;let t28e: f64 = (4.0 * l.f5e7);let t28f: f64 = (t28e * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t28f, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {
            let (t291, t292, t293,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t290: f64 = (-l.f6f7);
        (t290, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t291, t292, t293, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t294: f64 = (l.f6f3 * l.f6f3);let t295: f64 = (t294 + l.f6f7);let t296: f64 = (t295).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t296, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t296)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t296)), );l.f6fa = 0.0;let t297: f64 = (l.f6f3 / l.f6f7);let t298: f64 = (1.0 + t297);let t299: f64 = (0.5 * t298);(l.f51, l.f52, l.f53, ) = (t299, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t29a: f64 = (l.f6f3 + l.f6f7);let t29b: f64 = (0.5 * t29a);let t29c: f64 = (l.f5e7 + t29b);(l.f5f1, l.f5f2, l.f5f3, ) = (t29c, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t29d: f64 = (p.p85 - l.f5ed);let t29e: f64 = (t29d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t29e, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t29f: f64 = (4.0 * p.p85);let t2a0: f64 = (t29f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2a0, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {
            let (t2a2, t2a3, t2a4,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2a1: f64 = (-l.f6f7);
        (t2a1, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2a2, t2a3, t2a4, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t2a5: f64 = (l.f6f3 * l.f6f3);let t2a6: f64 = (t2a5 + l.f6f7);let t2a7: f64 = (t2a6).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2a7, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2a7)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2a7)), );l.f6fa = 0.0;let t2a8: f64 = (l.f6f3 + l.f6f7);let t2a9: f64 = (0.5 * t2a8);let t2aa: f64 = (p.p85 - t2a9);(l.f5ed, l.f5ee, l.f5ef, ) = (t2aa, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t2ab: f64 = (l.f5ed - l.f5e7);let t2ac: f64 = (t2ab - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2ac, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t2ad: f64 = (4.0 * l.f5e7);let t2ae: f64 = (t2ad * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2ae, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {
            let (t2b0, t2b1, t2b2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2af: f64 = (-l.f6f7);
        (t2af, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2b0, t2b1, t2b2, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t2b3: f64 = (l.f6f3 * l.f6f3);let t2b4: f64 = (t2b3 + l.f6f7);let t2b5: f64 = (t2b4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2b5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2b5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2b5)), );l.f6fa = 0.0;let t2b6: f64 = (l.f6f3 + l.f6f7);let t2b7: f64 = (0.5 * t2b6);let t2b8: f64 = (l.f5e7 + t2b7);(l.f5ed, l.f5ee, l.f5ef, ) = (t2b8, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t2b9: f64 = (p.p86 * l.f55);let t2ba: f64 = (t2b9 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t2ba, (((p.p86 * l.f56) * l.f51) + (t2b9 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t2b9 * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_92(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t2bb: f64 = (l.f7b1 / l.f5f1);let t2bc: f64 = (l.f5f1 - l.f5ed);let t2bd: f64 = (l.f793 * t2bc);let t2be: f64 = (l.f5ed * p.p85);let t2bf: f64 = (t2bd / t2be);let t2c0: f64 = (t2bb + t2bf);let t2c1: f64 = (l.f645 * t2c0);let t2c2: f64 = (t2c1).abs();let t2c3: f64 = if t2c2 < 230.25850929940458 { 1.0 } else { 0.0 };l.f216 = t2c3;l.f217 = 0.0;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f216 != 0.0)) {let t2c4: f64 = (l.f7b1 / l.f5f1);let t2c5: f64 = (l.f5f1 - l.f5ed);let t2c6: f64 = (l.f793 * t2c5);let t2c7: f64 = (l.f5ed * p.p85);let t2c8: f64 = (t2c6 / t2c7);let t2c9: f64 = (t2c4 + t2c8);let t2ca: f64 = (l.f645 * t2c9);let t2cb: f64 = (t2ca).exp();(l.f8e, l.f8f, l.f90, ) = (t2cb, (t2cb * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2c7) - (t2c6 * (l.f5ee * p.p85))) / (t2c7 * t2c7))))), (t2cb * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2c7) - (t2c6 * (l.f5ef * p.p85))) / (t2c7 * t2c7))))), );l.f91 = 0.0;}
        let t2cc: f64 = (l.f7b1 / l.f5f1);let t2cd: f64 = (l.f5f1 - l.f5ed);let t2ce: f64 = (l.f793 * t2cd);let t2cf: f64 = (l.f5ed * p.p85);let t2d0: f64 = (t2ce / t2cf);let t2d1: f64 = (t2cc + t2d0);let t2d2: f64 = (l.f645 * t2d1);let t2d3: f64 = (-230.25850929940458);let t2d4: f64 = if t2d2 < t2d3 { 1.0 } else { 0.0 };l.f218 = t2d4;l.f219 = 0.0;
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f216 == 0.0)) && (l.f218 != 0.0)) {let t2d5: f64 = (-230.25850929940458);let t2d6: f64 = (l.f7b1 / l.f5f1);let t2d7: f64 = (l.f5f1 - l.f5ed);let t2d8: f64 = (l.f793 * t2d7);let t2d9: f64 = (l.f5ed * p.p85);let t2da: f64 = (t2d8 / t2d9);let t2db: f64 = (t2d6 + t2da);let t2dc: f64 = (l.f645 * t2db);let t2dd: f64 = (t2d5 - t2dc);let t2de: f64 = (-230.25850929940458);let t2df: f64 = (l.f7b1 / l.f5f1);let t2e0: f64 = (l.f5f1 - l.f5ed);let t2e1: f64 = (l.f793 * t2e0);let t2e2: f64 = (l.f5ed * p.p85);let t2e3: f64 = (t2e1 / t2e2);let t2e4: f64 = (t2df + t2e3);let t2e5: f64 = (l.f645 * t2e4);let t2e6: f64 = (t2de - t2e5);let t2e7: f64 = (-230.25850929940458);let t2e8: f64 = (l.f7b1 / l.f5f1);let t2e9: f64 = (l.f5f1 - l.f5ed);let t2ea: f64 = (l.f793 * t2e9);let t2eb: f64 = (l.f5ed * p.p85);let t2ec: f64 = (t2ea / t2eb);let t2ed: f64 = (t2e8 + t2ec);let t2ee: f64 = (l.f645 * t2ed);let t2ef: f64 = (t2e7 - t2ee);let t2f0: f64 = (t2ef * 0.3333333333333333);let t2f1: f64 = (1.0 + t2f0);let t2f2: f64 = (t2e6 * t2f1);let t2f3: f64 = (0.5 * t2f2);let t2f4: f64 = (1.0 + t2f3);let t2f5: f64 = (t2dd * t2f4);let t2f6: f64 = (1.0 + t2f5);let t2f7: f64 = (1e-100 / t2f6);(l.f8e, l.f8f, l.f90, ) = (t2f7, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2d9) - (t2d8 * (l.f5ee * p.p85))) / (t2d9 * t2d9))))) * t2f4) + (t2dd * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2e2) - (t2e1 * (l.f5ee * p.p85))) / (t2e2 * t2e2))))) * t2f1) + (t2e6 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2eb) - (t2ea * (l.f5ee * p.p85))) / (t2eb * t2eb))))) * 0.3333333333333333))))))) / (t2f6 * t2f6))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2d9) - (t2d8 * (l.f5ef * p.p85))) / (t2d9 * t2d9))))) * t2f4) + (t2dd * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2e2) - (t2e1 * (l.f5ef * p.p85))) / (t2e2 * t2e2))))) * t2f1) + (t2e6 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2eb) - (t2ea * (l.f5ef * p.p85))) / (t2eb * t2eb))))) * 0.3333333333333333))))))) / (t2f6 * t2f6))), );l.f91 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_93(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f216 == 0.0)) && (l.f218 == 0.0)) {let t2f8: f64 = (l.f7b1 / l.f5f1);let t2f9: f64 = (l.f5f1 - l.f5ed);let t2fa: f64 = (l.f793 * t2f9);let t2fb: f64 = (l.f5ed * p.p85);let t2fc: f64 = (t2fa / t2fb);let t2fd: f64 = (t2f8 + t2fc);let t2fe: f64 = (l.f645 * t2fd);let t2ff: f64 = (t2fe - 230.25850929940458);let t300: f64 = (l.f7b1 / l.f5f1);let t301: f64 = (l.f5f1 - l.f5ed);let t302: f64 = (l.f793 * t301);let t303: f64 = (l.f5ed * p.p85);let t304: f64 = (t302 / t303);let t305: f64 = (t300 + t304);let t306: f64 = (l.f645 * t305);let t307: f64 = (t306 - 230.25850929940458);let t308: f64 = (l.f7b1 / l.f5f1);let t309: f64 = (l.f5f1 - l.f5ed);let t30a: f64 = (l.f793 * t309);let t30b: f64 = (l.f5ed * p.p85);let t30c: f64 = (t30a / t30b);let t30d: f64 = (t308 + t30c);let t30e: f64 = (l.f645 * t30d);let t30f: f64 = (t30e - 230.25850929940458);let t310: f64 = (t30f * 0.3333333333333333);let t311: f64 = (1.0 + t310);let t312: f64 = (t307 * t311);let t313: f64 = (0.5 * t312);let t314: f64 = (1.0 + t313);let t315: f64 = (t2ff * t314);let t316: f64 = (1.0 + t315);let t317: f64 = (1e100 * t316);(l.f8e, l.f8f, l.f90, ) = (t317, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2fb) - (t2fa * (l.f5ee * p.p85))) / (t2fb * t2fb)))) * t314) + (t2ff * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t303) - (t302 * (l.f5ee * p.p85))) / (t303 * t303)))) * t311) + (t307 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t30b) - (t30a * (l.f5ee * p.p85))) / (t30b * t30b)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2fb) - (t2fa * (l.f5ef * p.p85))) / (t2fb * t2fb)))) * t314) + (t2ff * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t303) - (t302 * (l.f5ef * p.p85))) / (t303 * t303)))) * t311) + (t307 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t30b) - (t30a * (l.f5ef * p.p85))) / (t30b * t30b)))) * 0.3333333333333333))))))), );l.f91 = 0.0;}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) {let t318: f64 = (l.f7b1 * l.f5b);let t319: f64 = (l.f5f1 - t318);let t31a: f64 = (l.f5f1 * l.f5f1);let t31b: f64 = (t319 / t31a);let t31c: f64 = (l.f793 * l.f5b);let t31d: f64 = (l.f5ed * p.p85);let t31e: f64 = (t31c / t31d);let t31f: f64 = (t31b + t31e);let t320: f64 = (l.f645 * t31f);(l.f61, l.f62, l.f63, ) = (t320, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t31a) - (t319 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t31a * t31a)) + ((((l.f793 * l.f5c) * t31d) - (t31c * (l.f5ee * p.p85))) / (t31d * t31d)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t31a) - (t319 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t31a * t31a)) + ((((l.f793 * l.f5d) * t31d) - (t31c * (l.f5ef * p.p85))) / (t31d * t31d)))), );l.f64 = 0.0;let t321: f64 = (l.f73b - l.f7b1);let t322: f64 = (t321 * l.f61);let t323: f64 = (1.0 + t322);let t324: f64 = (t323 * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t324, (((t321 * l.f62) * l.f8e) + (t323 * l.f8f)), (((t321 * l.f63) * l.f8e) + (t323 * l.f90)), );l.f53d = 0.0;}
        if ((l.f29a != 0.0) && (l.f1ec != 0.0)) {let t325: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t325, l.f537, l.f538, );l.f539 = 0.0;let t326: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t326, l.f53f, l.f540, );l.f541 = 0.0;let t327: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t327, l.f53b, l.f53c, );l.f53d = 0.0;let t328: f64 = (1.0 / l.f825);l.f817 = t328;l.f818 = 0.0;}
        let t329: f64 = if l.f73b > 0.0 { 1.0 } else { 0.0 };l.f21a = t329;l.f21b = 0.0;
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f21a != 0.0)) {let t32a: f64 = (2.0 + l.f817);let t32b: f64 = (l.f817 + 1.0);let t32c: f64 = (l.f817 + 3.0);let t32d: f64 = (t32b * t32c);let t32e: f64 = (t32d).sqrt();let t32f: f64 = (t32a + t32e);let t330: f64 = (t32f).ln();let t331: f64 = (l.f643 * t330);let t332: f64 = (2.0 * t331);l.f714 = t332;l.f715 = 0.0;}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f21a == 0.0)) {let t333: f64 = (-l.f73b);let t334: f64 = (2.0 * l.f825);let t335: f64 = (t334 + 1.0);let t336: f64 = (1.0 + l.f825);let t337: f64 = (3.0 * l.f825);let t338: f64 = (1.0 + t337);let t339: f64 = (t336 * t338);let t33a: f64 = (t339).sqrt();let t33b: f64 = (t335 + t33a);let t33c: f64 = (t33b).ln();let t33d: f64 = (l.f643 * t33c);let t33e: f64 = (2.0 * t33d);let t33f: f64 = (t333 + t33e);l.f714 = t33f;l.f715 = 0.0;}
        if ((l.f29a != 0.0) && (l.f1ec != 0.0)) {let t340: f64 = (l.f76f - l.f714);l.f79c = t340;l.f79d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_94(
        l: &mut StampLocals,
    ) {
        if ((l.f29a != 0.0) && (l.f1ec != 0.0)) {let t341: f64 = (l.f73b + l.f79c);let t342: f64 = (l.f73b - l.f79c);let t343: f64 = (l.f73b - l.f79c);let t344: f64 = (t342 * t343);let t345: f64 = (4.0 * l.f643);let t346: f64 = (t345 * l.f643);let t347: f64 = (t344 + t346);let t348: f64 = (t347).sqrt();let t349: f64 = (t341 - t348);let t34a: f64 = (0.5 * t349);l.f7a2 = t34a;l.f7a3 = 0.0;let t34b: f64 = (l.f73b + l.f755);let t34c: f64 = (l.f73b - l.f755);let t34d: f64 = (l.f73b - l.f755);let t34e: f64 = (t34c * t34d);let t34f: f64 = (4.0 * l.f647);let t350: f64 = (t34f * l.f647);let t351: f64 = (t34e + t350);let t352: f64 = (t351).sqrt();let t353: f64 = (t34b - t352);let t354: f64 = (0.5 * t353);l.f750 = t354;l.f751 = 0.0;let t355: f64 = l.f73b;let t356: f64 = l.f73b;let t357: f64 = l.f73b;let t358: f64 = (t356 * t357);let t359: f64 = (4.0 * 1e-6);let t35a: f64 = (t359 * 1e-6);let t35b: f64 = (t358 + t35a);let t35c: f64 = (t35b).sqrt();let t35d: f64 = (t355 - t35c);let t35e: f64 = (0.5 * t35d);l.f74a = t35e;l.f74b = 0.0;}
        if ((l.f29a != 0.0) && (l.f1ec == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );l.f539 = 0.0;(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );l.f541 = 0.0;(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f53d = 0.0;l.f714 = 0.0;l.f715 = 0.0;l.f796 = 0.0;l.f797 = 0.0;l.f825 = 0.0;l.f826 = 0.0;l.f7a2 = 0.0;l.f7a3 = 0.0;l.f750 = 0.0;l.f751 = 0.0;l.f74a = 0.0;l.f74b = 0.0;}
        let t35f: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f21c = t35f;l.f21d = 0.0;
        if ((l.f29a != 0.0) && (l.f21c != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );l.f565 = 0.0;(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );l.f555 = 0.0;(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );l.f68f = 0.0;}
        let t360: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f21e = t360;l.f21f = 0.0;
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f21e != 0.0)) {let t361: f64 = (l.f796 * l.f769);let t362: f64 = (1.0 - t361);let t363: f64 = (t362).sqrt();l.f6fc = t363;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f21e == 0.0)) {let t364: f64 = (l.f796 * l.f769);let t365: f64 = (1.0 - t364);let t366: f64 = (t365).powf(l.f60b);l.f6fc = t366;l.f6fd = 0.0;}
        if ((l.f29a != 0.0) && (l.f21c == 0.0)) {let t367: f64 = (1.0 - l.f6fc);let t368: f64 = (l.f69e * t367);let t369: f64 = (l.f73b - l.f796);let t36a: f64 = (l.f698 * t369);let t36b: f64 = (t368 + t36a);(l.f68c, l.f68d, l.f68e, ) = (t36b, 0.0, 0.0, );l.f68f = 0.0;let t36c: f64 = (l.f542 * l.f536);(l.f52f, l.f530, l.f531, ) = (t36c, (l.f542 * l.f537), (l.f542 * l.f538), );l.f532 = 0.0;}
        let t36d: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.f220 = t36d;l.f221 = 0.0;
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 != 0.0)) {l.f758 = 0.0;l.f759 = 0.0;l.f7e9 = 0.0;l.f7ea = 0.0;l.f7d1 = 0.0;l.f7d2 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_95(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 != 0.0)) {l.f9 = 0.0;l.fa = 0.0;l.f593 = 0.0;l.f594 = 0.0;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) {let t36e: f64 = (l.f75d - l.f7a2);l.f758 = t36e;l.f759 = 0.0;let t36f: f64 = (l.f714 / l.f758);let t370: f64 = (1.0 - t36f);let t371: f64 = (t370).sqrt();let t372: f64 = (1.0 - t371);l.f7ef = t372;l.f7f0 = 0.0;}
        let t373: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f222 = t373;l.f223 = 0.0;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) && (l.f222 != 0.0)) {l.f66 = 0.0;l.f67 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) && (l.f222 == 0.0)) {let t374: f64 = (l.f7ef * l.f7ef);let t375: f64 = (l.f7ef).ln();let t376: f64 = (t374 * t375);let t377: f64 = (1.0 - l.f7ef);let t378: f64 = (t376 / t377);let t379: f64 = (t378 + l.f7ef);let t37a: f64 = (2.0 * l.f623);let t37b: f64 = (1.0 - t37a);let t37c: f64 = (t379 * t37b);l.f66 = t37c;l.f67 = 0.0;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) {let t37d: f64 = (l.f7ef + l.f66);l.f7e9 = t37d;l.f7ea = 0.0;}
        let t37e: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f224 = t37e;l.f225 = 0.0;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) && (l.f224 != 0.0)) {let t37f: f64 = (l.f758 * l.f773);let t380: f64 = (t37f).sqrt();l.f6fc = t380;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) && (l.f224 == 0.0)) {let t381: f64 = (l.f758 * l.f773);let t382: f64 = (t381).powf(l.f623);l.f6fc = t382;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) {let t383: f64 = (l.f7d6 * l.f6fc);l.f7d1 = t383;l.f7d2 = 0.0;let t384: f64 = (l.f825 - 1.0);let t385: f64 = (t384 * l.f7d1);let t386: f64 = (l.fc9 * t385);l.f9 = t386;l.fa = 0.0;let t387: f64 = (l.f9 * l.f7e9);let t388: f64 = (l.f39 * t387);l.f593 = t388;l.f594 = 0.0;}
        let t389: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.f226 = t389;l.f227 = 0.0;
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 != 0.0)) {l.f599 = 0.0;l.f59a = 0.0;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) {let t38a: f64 = (l.f7d1 * l.f60b);let t38b: f64 = (t38a / l.f758);let t38c: f64 = (l.f1e * t38b);l.f19 = t38c;l.f1a = 0.0;let t38d: f64 = (0.666666666666667 * l.fe);let t38e: f64 = (t38d / l.f19);l.f71a = t38e;l.f71b = 0.0;let t38f: f64 = (l.f71a * l.f71a);l.f72c = t38f;l.f72d = 0.0;let t390: f64 = (l.f72c * l.f72c);let t391: f64 = (l.f72c * l.f72c);let t392: f64 = (t391 + 1.0);let t393: f64 = (t390 / t392);let t394: f64 = (t393).sqrt();l.f726 = t394;l.f727 = 0.0;let t395: f64 = (l.f726).abs();let t396: f64 = (t395).sqrt();l.f6c1 = t396;l.f6c2 = 0.0;let t397: f64 = (l.f726 * l.f6c1);l.f732 = t397;l.f733 = 0.0;}
        let t398: f64 = (-l.f623);let t399: f64 = (t398 * l.f611);let t39a: f64 = (-1.0);let t39b: f64 = if t399 == t39a { 1.0 } else { 0.0 };l.f228 = t39b;l.f229 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_96(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f228 != 0.0)) {let t39c: f64 = (l.f19 * l.f732);let t39d: f64 = (1.0 + t39c);let t39e: f64 = (1.0 / t39d);l.f7e3 = t39e;l.f7e4 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f228 == 0.0)) {let t39f: f64 = (l.f19 * l.f732);let t3a0: f64 = (1.0 + t39f);let t3a1: f64 = (-l.f623);let t3a2: f64 = (t3a1 * l.f611);let t3a3: f64 = (t3a0).powf(t3a2);l.f7e3 = t3a3;l.f7e4 = 0.0;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) {let t3a4: f64 = (l.f7e9 * l.f7e3);let t3a5: f64 = (l.f7e9 + l.f7e3);let t3a6: f64 = (t3a4 / t3a5);l.f7f5 = t3a6;l.f7f6 = 0.0;let t3a7: f64 = (l.f19 / l.f6c1);let t3a8: f64 = (0.375 * t3a7);let t3a9: f64 = (t3a8).sqrt();l.f5a8 = t3a9;l.f5a9 = 0.0;let t3aa: f64 = (l.f71a * l.f6c1);let t3ab: f64 = (2.0 * t3aa);let t3ac: f64 = (t3ab - l.f726);l.f5b4 = t3ac;l.f5b5 = 0.0;let t3ad: f64 = (l.fe * l.f71a);let t3ae: f64 = (t3ad * l.f6c1);let t3af: f64 = (l.fe * l.f726);let t3b0: f64 = (t3ae - t3af);let t3b1: f64 = (l.f19 * l.f732);let t3b2: f64 = (0.5 * t3b1);let t3b3: f64 = (t3b0 + t3b2);l.f5d4 = t3b3;l.f5d5 = 0.0;let t3b4: f64 = (l.f5b4 - 1.0);let t3b5: f64 = (t3b4 * l.f5a8);l.f7fb = t3b5;l.f7fc = 0.0;let t3b6: f64 = (l.f7fb * l.f7fb);l.f811 = t3b6;l.f812 = 0.0;}
        let t3b7: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f22b = t3b7;l.f22c = 0.0;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22b != 0.0)) {let t3b8: f64 = (l.f62b * l.f7fb);let t3b9: f64 = (1.0 + t3b8);let t3ba: f64 = (1.0 / t3b9);l.f6e2 = t3ba;l.f6e3 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22b == 0.0)) {let t3bb: f64 = (l.f62b * l.f7fb);let t3bc: f64 = (1.0 - t3bb);let t3bd: f64 = (1.0 / t3bc);l.f6e2 = t3bd;l.f6e3 = 0.0;}
        let t3be: f64 = (-l.f811);let t3bf: f64 = (t3be + l.f5d4);let t3c0: f64 = (-230.25850929940458);let t3c1: f64 = if t3bf > t3c0 { 1.0 } else { 0.0 };l.f22d = t3c1;l.f22e = 0.0;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22d != 0.0)) {let t3c2: f64 = (-l.f811);let t3c3: f64 = (t3c2 + l.f5d4);let t3c4: f64 = (t3c3).exp();l.f6fc = t3c4;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22d == 0.0)) {let t3c5: f64 = (-230.25850929940458);let t3c6: f64 = (-l.f811);let t3c7: f64 = (t3c6 + l.f5d4);let t3c8: f64 = (t3c5 - t3c7);let t3c9: f64 = (-230.25850929940458);let t3ca: f64 = (-l.f811);let t3cb: f64 = (t3ca + l.f5d4);let t3cc: f64 = (t3c9 - t3cb);let t3cd: f64 = (-230.25850929940458);let t3ce: f64 = (-l.f811);let t3cf: f64 = (t3ce + l.f5d4);let t3d0: f64 = (t3cd - t3cf);let t3d1: f64 = (t3d0 * 0.3333333333333333);let t3d2: f64 = (1.0 + t3d1);let t3d3: f64 = (t3cc * t3d2);let t3d4: f64 = (0.5 * t3d3);let t3d5: f64 = (1.0 + t3d4);let t3d6: f64 = (t3c8 * t3d5);let t3d7: f64 = (1.0 + t3d6);let t3d8: f64 = (1e-100 / t3d7);l.f6fc = t3d8;l.f6fd = 0.0;}
    }
}
