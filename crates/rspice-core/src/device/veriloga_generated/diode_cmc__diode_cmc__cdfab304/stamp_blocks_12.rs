#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t0: f64 = (l.f6f3 + l.f6f7);let t1: f64 = (0.5 * t0);let t2: f64 = (l.f5e9 + t1);(l.f5f1, l.f5f2, l.f5f3, ) = (t2, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t3: f64 = (p.p85 - l.f5ed);let t4: f64 = (t3 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t4, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t5: f64 = (4.0 * p.p85);let t6: f64 = (t5 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t6, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {
            let (t8, t9, ta,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t7: f64 = (-l.f6f7);
        (t7, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t8, t9, ta, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let tb: f64 = (l.f6f3 * l.f6f3);let tc: f64 = (tb + l.f6f7);let td: f64 = (tc).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (td, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * td)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * td)), );l.f6fa = 0.0;let te: f64 = (l.f6f3 + l.f6f7);let tf: f64 = (0.5 * te);let t10: f64 = (p.p85 - tf);(l.f5ed, l.f5ee, l.f5ef, ) = (t10, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t11: f64 = (l.f5ed - l.f5e9);let t12: f64 = (t11 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t12, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t13: f64 = (4.0 * l.f5e9);let t14: f64 = (t13 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t14, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {
            let (t16, t17, t18,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t15: f64 = (-l.f6f7);
        (t15, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t16, t17, t18, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t19: f64 = (l.f6f3 * l.f6f3);let t1a: f64 = (t19 + l.f6f7);let t1b: f64 = (t1a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1b)), );l.f6fa = 0.0;let t1c: f64 = (l.f6f3 + l.f6f7);let t1d: f64 = (0.5 * t1c);let t1e: f64 = (l.f5e9 + t1d);(l.f5ed, l.f5ee, l.f5ef, ) = (t1e, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );l.f5f4 = 0.0;}
        let t1f: f64 = (l.f737 / l.f5f1);let t20: f64 = (l.f5f1 - l.f5ed);let t21: f64 = (l.f793 * t20);let t22: f64 = (l.f5ed * p.p85);let t23: f64 = (t21 / t22);let t24: f64 = (t1f + t23);let t25: f64 = (l.f645 * t24);let t26: f64 = (t25).abs();let t27: f64 = if t26 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4bd = t27;l.f4be = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bd != 0.0)) {let t28: f64 = (l.f737 / l.f5f1);let t29: f64 = (l.f5f1 - l.f5ed);let t2a: f64 = (l.f793 * t29);let t2b: f64 = (l.f5ed * p.p85);let t2c: f64 = (t2a / t2b);let t2d: f64 = (t28 + t2c);let t2e: f64 = (l.f645 * t2d);let t2f: f64 = (t2e).exp();(l.f53e, l.f53f, l.f540, ) = (t2f, (t2f * (l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2b) - (t2a * (l.f5ee * p.p85))) / (t2b * t2b))))), (t2f * (l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2b) - (t2a * (l.f5ef * p.p85))) / (t2b * t2b))))), );l.f541 = 0.0;}
        let t30: f64 = (l.f737 / l.f5f1);let t31: f64 = (l.f5f1 - l.f5ed);let t32: f64 = (l.f793 * t31);let t33: f64 = (l.f5ed * p.p85);let t34: f64 = (t32 / t33);let t35: f64 = (t30 + t34);let t36: f64 = (l.f645 * t35);let t37: f64 = (-230.25850929940458);let t38: f64 = if t36 < t37 { 1.0 } else { 0.0 };l.f4bf = t38;l.f4c0 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bd == 0.0)) && (l.f4bf != 0.0)) {let t39: f64 = (-230.25850929940458);let t3a: f64 = (l.f737 / l.f5f1);let t3b: f64 = (l.f5f1 - l.f5ed);let t3c: f64 = (l.f793 * t3b);let t3d: f64 = (l.f5ed * p.p85);let t3e: f64 = (t3c / t3d);let t3f: f64 = (t3a + t3e);let t40: f64 = (l.f645 * t3f);let t41: f64 = (t39 - t40);let t42: f64 = (-230.25850929940458);let t43: f64 = (l.f737 / l.f5f1);let t44: f64 = (l.f5f1 - l.f5ed);let t45: f64 = (l.f793 * t44);let t46: f64 = (l.f5ed * p.p85);let t47: f64 = (t45 / t46);let t48: f64 = (t43 + t47);let t49: f64 = (l.f645 * t48);let t4a: f64 = (t42 - t49);let t4b: f64 = (-230.25850929940458);let t4c: f64 = (l.f737 / l.f5f1);let t4d: f64 = (l.f5f1 - l.f5ed);let t4e: f64 = (l.f793 * t4d);let t4f: f64 = (l.f5ed * p.p85);let t50: f64 = (t4e / t4f);let t51: f64 = (t4c + t50);let t52: f64 = (l.f645 * t51);let t53: f64 = (t4b - t52);let t54: f64 = (t53 * 0.3333333333333333);let t55: f64 = (1.0 + t54);let t56: f64 = (t4a * t55);let t57: f64 = (0.5 * t56);let t58: f64 = (1.0 + t57);let t59: f64 = (t41 * t58);let t5a: f64 = (1.0 + t59);let t5b: f64 = (1e-100 / t5a);(l.f53e, l.f53f, l.f540, ) = (t5b, (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3d) - (t3c * (l.f5ee * p.p85))) / (t3d * t3d))))) * t58) + (t41 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t46) - (t45 * (l.f5ee * p.p85))) / (t46 * t46))))) * t55) + (t4a * ((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t4f) - (t4e * (l.f5ee * p.p85))) / (t4f * t4f))))) * 0.3333333333333333))))))) / (t5a * t5a))), (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3d) - (t3c * (l.f5ef * p.p85))) / (t3d * t3d))))) * t58) + (t41 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t46) - (t45 * (l.f5ef * p.p85))) / (t46 * t46))))) * t55) + (t4a * ((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t4f) - (t4e * (l.f5ef * p.p85))) / (t4f * t4f))))) * 0.3333333333333333))))))) / (t5a * t5a))), );l.f541 = 0.0;}
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bd == 0.0)) && (l.f4bf == 0.0)) {let t5c: f64 = (l.f737 / l.f5f1);let t5d: f64 = (l.f5f1 - l.f5ed);let t5e: f64 = (l.f793 * t5d);let t5f: f64 = (l.f5ed * p.p85);let t60: f64 = (t5e / t5f);let t61: f64 = (t5c + t60);let t62: f64 = (l.f645 * t61);let t63: f64 = (t62 - 230.25850929940458);let t64: f64 = (l.f737 / l.f5f1);let t65: f64 = (l.f5f1 - l.f5ed);let t66: f64 = (l.f793 * t65);let t67: f64 = (l.f5ed * p.p85);let t68: f64 = (t66 / t67);let t69: f64 = (t64 + t68);let t6a: f64 = (l.f645 * t69);let t6b: f64 = (t6a - 230.25850929940458);let t6c: f64 = (l.f737 / l.f5f1);let t6d: f64 = (l.f5f1 - l.f5ed);let t6e: f64 = (l.f793 * t6d);let t6f: f64 = (l.f5ed * p.p85);let t70: f64 = (t6e / t6f);let t71: f64 = (t6c + t70);let t72: f64 = (l.f645 * t71);let t73: f64 = (t72 - 230.25850929940458);let t74: f64 = (t73 * 0.3333333333333333);let t75: f64 = (1.0 + t74);let t76: f64 = (t6b * t75);let t77: f64 = (0.5 * t76);let t78: f64 = (1.0 + t77);let t79: f64 = (t63 * t78);let t7a: f64 = (1.0 + t79);let t7b: f64 = (1e100 * t7a);(l.f53e, l.f53f, l.f540, ) = (t7b, (1e100 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t5f) - (t5e * (l.f5ee * p.p85))) / (t5f * t5f)))) * t78) + (t63 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t67) - (t66 * (l.f5ee * p.p85))) / (t67 * t67)))) * t75) + (t6b * ((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t6f) - (t6e * (l.f5ee * p.p85))) / (t6f * t6f)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t5f) - (t5e * (l.f5ef * p.p85))) / (t5f * t5f)))) * t78) + (t63 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t67) - (t66 * (l.f5ef * p.p85))) / (t67 * t67)))) * t75) + (t6b * ((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t6f) - (t6e * (l.f5ef * p.p85))) / (t6f * t6f)))) * 0.3333333333333333))))))), );l.f541 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) {let t7c: f64 = (l.f5eb * l.f5eb);let t7d: f64 = (t7c / l.f5e1);l.f64f = t7d;l.f650 = 0.0;let t7e: f64 = (l.f5e7 / l.f645);let t7f: f64 = (l.f5e1 / l.f64f);let t80: f64 = (t7f).ln();let t81: f64 = (t7e * t80);l.f793 = t81;l.f794 = 0.0;}
        let t82: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f4c1 = t82;l.f4c2 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t83: f64 = (l.f737 - l.f793);let t84: f64 = (p.p86 * t83);let t85: f64 = (t84 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t85, 0.0, 0.0, );l.f604 = 0.0;let t86: f64 = (p.p86 * l.f793);let t87: f64 = (l.f5e7 - t86);(l.f5ed, l.f5ee, l.f5ef, ) = (t87, 0.0, 0.0, );l.f5f0 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t88: f64 = (p.p85 - l.f601);let t89: f64 = (t88 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t89, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t8a: f64 = (4.0 * p.p85);let t8b: f64 = (t8a * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t8b, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {
            let (t8d, t8e, t8f,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t8c: f64 = (-l.f6f7);
        (t8c, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t8d, t8e, t8f, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t90: f64 = (l.f6f3 * l.f6f3);let t91: f64 = (t90 + l.f6f7);let t92: f64 = (t91).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t92, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t92)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t92)), );l.f6fa = 0.0;let t93: f64 = (l.f6f3 + l.f6f7);let t94: f64 = (0.5 * t93);let t95: f64 = (p.p85 - t94);(l.f605, l.f606, l.f607, ) = (t95, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t96: f64 = (l.f605 - l.f5e7);let t97: f64 = (t96 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t97, l.f606, l.f607, );l.f6f6 = 0.0;let t98: f64 = (4.0 * l.f5e7);let t99: f64 = (t98 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t99, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {
            let (t9b, t9c, t9d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t9a: f64 = (-l.f6f7);
        (t9a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t9b, t9c, t9d, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t9e: f64 = (l.f6f3 * l.f6f3);let t9f: f64 = (t9e + l.f6f7);let ta0: f64 = (t9f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (ta0, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * ta0)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * ta0)), );l.f6fa = 0.0;let ta1: f64 = (l.f6f3 + l.f6f7);let ta2: f64 = (0.5 * ta1);let ta3: f64 = (l.f5e7 + ta2);(l.f5f1, l.f5f2, l.f5f3, ) = (ta3, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let ta4: f64 = (p.p85 - l.f5ed);let ta5: f64 = (ta4 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (ta5, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let ta6: f64 = (4.0 * p.p85);let ta7: f64 = (ta6 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (ta7, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {
            let (ta9, taa, tab,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let ta8: f64 = (-l.f6f7);
        (ta8, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (ta9, taa, tab, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let tac: f64 = (l.f6f3 * l.f6f3);let tad: f64 = (tac + l.f6f7);let tae: f64 = (tad).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tae, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tae)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tae)), );l.f6fa = 0.0;let taf: f64 = (l.f6f3 + l.f6f7);let tb0: f64 = (0.5 * taf);let tb1: f64 = (p.p85 - tb0);(l.f5ed, l.f5ee, l.f5ef, ) = (tb1, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let tb2: f64 = (l.f5ed - l.f5e7);let tb3: f64 = (tb2 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tb3, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let tb4: f64 = (4.0 * l.f5e7);let tb5: f64 = (tb4 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tb5, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {
            let (tb7, tb8, tb9,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tb6: f64 = (-l.f6f7);
        (tb6, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tb7, tb8, tb9, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let tba: f64 = (l.f6f3 * l.f6f3);let tbb: f64 = (tba + l.f6f7);let tbc: f64 = (tbb).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tbc, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tbc)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tbc)), );l.f6fa = 0.0;let tbd: f64 = (l.f6f3 + l.f6f7);let tbe: f64 = (0.5 * tbd);let tbf: f64 = (l.f5e7 + tbe);(l.f5ed, l.f5ee, l.f5ef, ) = (tbf, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );l.f5f4 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tc0: f64 = (l.f737 / l.f5f1);let tc1: f64 = (l.f5f1 - l.f5ed);let tc2: f64 = (l.f793 * tc1);let tc3: f64 = (l.f5ed * p.p85);let tc4: f64 = (tc2 / tc3);let tc5: f64 = (tc0 + tc4);let tc6: f64 = (l.f645 * tc5);let tc7: f64 = (tc6).abs();let tc8: f64 = if tc7 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4c3 = tc8;l.f4c4 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c3 != 0.0)) {let tc9: f64 = (l.f737 / l.f5f1);let tca: f64 = (l.f5f1 - l.f5ed);let tcb: f64 = (l.f793 * tca);let tcc: f64 = (l.f5ed * p.p85);let tcd: f64 = (tcb / tcc);let tce: f64 = (tc9 + tcd);let tcf: f64 = (l.f645 * tce);let td0: f64 = (tcf).exp();(l.f53a, l.f53b, l.f53c, ) = (td0, (td0 * (l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tcc) - (tcb * (l.f5ee * p.p85))) / (tcc * tcc))))), (td0 * (l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tcc) - (tcb * (l.f5ef * p.p85))) / (tcc * tcc))))), );l.f53d = 0.0;}
        let td1: f64 = (l.f737 / l.f5f1);let td2: f64 = (l.f5f1 - l.f5ed);let td3: f64 = (l.f793 * td2);let td4: f64 = (l.f5ed * p.p85);let td5: f64 = (td3 / td4);let td6: f64 = (td1 + td5);let td7: f64 = (l.f645 * td6);let td8: f64 = (-230.25850929940458);let td9: f64 = if td7 < td8 { 1.0 } else { 0.0 };l.f4c5 = td9;l.f4c6 = 0.0;
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c3 == 0.0)) && (l.f4c5 != 0.0)) {let tda: f64 = (-230.25850929940458);let tdb: f64 = (l.f737 / l.f5f1);let tdc: f64 = (l.f5f1 - l.f5ed);let tdd: f64 = (l.f793 * tdc);let tde: f64 = (l.f5ed * p.p85);let tdf: f64 = (tdd / tde);let te0: f64 = (tdb + tdf);let te1: f64 = (l.f645 * te0);let te2: f64 = (tda - te1);let te3: f64 = (-230.25850929940458);let te4: f64 = (l.f737 / l.f5f1);let te5: f64 = (l.f5f1 - l.f5ed);let te6: f64 = (l.f793 * te5);let te7: f64 = (l.f5ed * p.p85);let te8: f64 = (te6 / te7);let te9: f64 = (te4 + te8);let tea: f64 = (l.f645 * te9);let teb: f64 = (te3 - tea);let tec: f64 = (-230.25850929940458);let ted: f64 = (l.f737 / l.f5f1);let tee: f64 = (l.f5f1 - l.f5ed);let tef: f64 = (l.f793 * tee);let tf0: f64 = (l.f5ed * p.p85);let tf1: f64 = (tef / tf0);let tf2: f64 = (ted + tf1);let tf3: f64 = (l.f645 * tf2);let tf4: f64 = (tec - tf3);let tf5: f64 = (tf4 * 0.3333333333333333);let tf6: f64 = (1.0 + tf5);let tf7: f64 = (teb * tf6);let tf8: f64 = (0.5 * tf7);let tf9: f64 = (1.0 + tf8);let tfa: f64 = (te2 * tf9);let tfb: f64 = (1.0 + tfa);let tfc: f64 = (1e-100 / tfb);(l.f53a, l.f53b, l.f53c, ) = (tfc, (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tde) - (tdd * (l.f5ee * p.p85))) / (tde * tde))))) * tf9) + (te2 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * te7) - (te6 * (l.f5ee * p.p85))) / (te7 * te7))))) * tf6) + (teb * ((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tf0) - (tef * (l.f5ee * p.p85))) / (tf0 * tf0))))) * 0.3333333333333333))))))) / (tfb * tfb))), (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tde) - (tdd * (l.f5ef * p.p85))) / (tde * tde))))) * tf9) + (te2 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * te7) - (te6 * (l.f5ef * p.p85))) / (te7 * te7))))) * tf6) + (teb * ((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tf0) - (tef * (l.f5ef * p.p85))) / (tf0 * tf0))))) * 0.3333333333333333))))))) / (tfb * tfb))), );l.f53d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c3 == 0.0)) && (l.f4c5 == 0.0)) {let tfd: f64 = (l.f737 / l.f5f1);let tfe: f64 = (l.f5f1 - l.f5ed);let tff: f64 = (l.f793 * tfe);let t100: f64 = (l.f5ed * p.p85);let t101: f64 = (tff / t100);let t102: f64 = (tfd + t101);let t103: f64 = (l.f645 * t102);let t104: f64 = (t103 - 230.25850929940458);let t105: f64 = (l.f737 / l.f5f1);let t106: f64 = (l.f5f1 - l.f5ed);let t107: f64 = (l.f793 * t106);let t108: f64 = (l.f5ed * p.p85);let t109: f64 = (t107 / t108);let t10a: f64 = (t105 + t109);let t10b: f64 = (l.f645 * t10a);let t10c: f64 = (t10b - 230.25850929940458);let t10d: f64 = (l.f737 / l.f5f1);let t10e: f64 = (l.f5f1 - l.f5ed);let t10f: f64 = (l.f793 * t10e);let t110: f64 = (l.f5ed * p.p85);let t111: f64 = (t10f / t110);let t112: f64 = (t10d + t111);let t113: f64 = (l.f645 * t112);let t114: f64 = (t113 - 230.25850929940458);let t115: f64 = (t114 * 0.3333333333333333);let t116: f64 = (1.0 + t115);let t117: f64 = (t10c * t116);let t118: f64 = (0.5 * t117);let t119: f64 = (1.0 + t118);let t11a: f64 = (t104 * t119);let t11b: f64 = (1.0 + t11a);let t11c: f64 = (1e100 * t11b);(l.f53a, l.f53b, l.f53c, ) = (t11c, (1e100 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t100) - (tff * (l.f5ee * p.p85))) / (t100 * t100)))) * t119) + (t104 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t108) - (t107 * (l.f5ee * p.p85))) / (t108 * t108)))) * t116) + (t10c * ((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t110) - (t10f * (l.f5ee * p.p85))) / (t110 * t110)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t100) - (tff * (l.f5ef * p.p85))) / (t100 * t100)))) * t119) + (t104 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t108) - (t107 * (l.f5ef * p.p85))) / (t108 * t108)))) * t116) + (t10c * ((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t110) - (t10f * (l.f5ef * p.p85))) / (t110 * t110)))) * 0.3333333333333333))))))), );l.f53d = 0.0;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let t11d: f64 = (l.f737 - l.f7b1);let t11e: f64 = (t11d * l.f645);let t11f: f64 = (1.0 + t11e);let t120: f64 = (t11f * l.f89);let t121: f64 = (t120).sqrt();l.f825 = t121;l.f826 = 0.0;let t122: f64 = (l.f5eb * l.f5eb);let t123: f64 = (t122 / l.f5df);l.f64f = t123;l.f650 = 0.0;let t124: f64 = (l.f5e5 / l.f645);let t125: f64 = (l.f5df / l.f64f);let t126: f64 = (t125).ln();let t127: f64 = (t124 * t126);l.f793 = t127;l.f794 = 0.0;}
        let t128: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f4c7 = t128;l.f4c8 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t129: f64 = (l.f7b1 - l.f793);let t12a: f64 = (p.p86 * t129);let t12b: f64 = (t12a + l.f5e5);(l.f601, l.f602, l.f603, ) = (t12b, 0.0, 0.0, );l.f604 = 0.0;let t12c: f64 = (p.p86 * l.f793);let t12d: f64 = (l.f5e5 - t12c);(l.f5ed, l.f5ee, l.f5ef, ) = (t12d, 0.0, 0.0, );l.f5f0 = 0.0;let t12e: f64 = (p.p85 - l.f601);let t12f: f64 = (t12e - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t12f, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t130: f64 = (4.0 * p.p85);let t131: f64 = (t130 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t131, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {
            let (t133, t134, t135,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t132: f64 = (-l.f6f7);
        (t132, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t133, t134, t135, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t136: f64 = (l.f6f3 * l.f6f3);let t137: f64 = (t136 + l.f6f7);let t138: f64 = (t137).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t138, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t138)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t138)), );l.f6fa = 0.0;let t139: f64 = (l.f6f3 / l.f6f7);let t13a: f64 = (1.0 + t139);let t13b: f64 = (0.5 * t13a);(l.f55, l.f56, l.f57, ) = (t13b, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;let t13c: f64 = (l.f6f3 + l.f6f7);let t13d: f64 = (0.5 * t13c);let t13e: f64 = (p.p85 - t13d);(l.f605, l.f606, l.f607, ) = (t13e, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t13f: f64 = (l.f605 - l.f5e5);let t140: f64 = (t13f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t140, l.f606, l.f607, );l.f6f6 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t141: f64 = (4.0 * l.f5e5);let t142: f64 = (t141 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t142, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {
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
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t147: f64 = (l.f6f3 * l.f6f3);let t148: f64 = (t147 + l.f6f7);let t149: f64 = (t148).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t149, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t149)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t149)), );l.f6fa = 0.0;let t14a: f64 = (l.f6f3 / l.f6f7);let t14b: f64 = (1.0 + t14a);let t14c: f64 = (0.5 * t14b);(l.f51, l.f52, l.f53, ) = (t14c, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t14d: f64 = (l.f6f3 + l.f6f7);let t14e: f64 = (0.5 * t14d);let t14f: f64 = (l.f5e5 + t14e);(l.f5f1, l.f5f2, l.f5f3, ) = (t14f, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t150: f64 = (p.p85 - l.f5ed);let t151: f64 = (t150 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t151, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t152: f64 = (4.0 * p.p85);let t153: f64 = (t152 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t153, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {
            let (t155, t156, t157,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t154: f64 = (-l.f6f7);
        (t154, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t155, t156, t157, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t158: f64 = (l.f6f3 * l.f6f3);let t159: f64 = (t158 + l.f6f7);let t15a: f64 = (t159).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t15a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t15a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t15a)), );l.f6fa = 0.0;let t15b: f64 = (l.f6f3 + l.f6f7);let t15c: f64 = (0.5 * t15b);let t15d: f64 = (p.p85 - t15c);(l.f5ed, l.f5ee, l.f5ef, ) = (t15d, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t15e: f64 = (l.f5ed - l.f5e5);let t15f: f64 = (t15e - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t15f, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t160: f64 = (4.0 * l.f5e5);let t161: f64 = (t160 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t161, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {
            let (t163, t164, t165,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t162: f64 = (-l.f6f7);
        (t162, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t163, t164, t165, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t166: f64 = (l.f6f3 * l.f6f3);let t167: f64 = (t166 + l.f6f7);let t168: f64 = (t167).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t168, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t168)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t168)), );l.f6fa = 0.0;let t169: f64 = (l.f6f3 + l.f6f7);let t16a: f64 = (0.5 * t169);let t16b: f64 = (l.f5e5 + t16a);(l.f5ed, l.f5ee, l.f5ef, ) = (t16b, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t16c: f64 = (p.p86 * l.f55);let t16d: f64 = (t16c * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t16d, (((p.p86 * l.f56) * l.f51) + (t16c * l.f52)), (((p.p86 * l.f57) * l.f51) + (t16c * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
        let t16e: f64 = (l.f7b1 / l.f5f1);let t16f: f64 = (l.f5f1 - l.f5ed);let t170: f64 = (l.f793 * t16f);let t171: f64 = (l.f5ed * p.p85);let t172: f64 = (t170 / t171);let t173: f64 = (t16e + t172);let t174: f64 = (l.f645 * t173);let t175: f64 = (t174).abs();let t176: f64 = if t175 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4cb = t176;l.f4cc = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cb != 0.0)) {let t177: f64 = (l.f7b1 / l.f5f1);let t178: f64 = (l.f5f1 - l.f5ed);let t179: f64 = (l.f793 * t178);let t17a: f64 = (l.f5ed * p.p85);let t17b: f64 = (t179 / t17a);let t17c: f64 = (t177 + t17b);let t17d: f64 = (l.f645 * t17c);let t17e: f64 = (t17d).exp();(l.f8a, l.f8b, l.f8c, ) = (t17e, (t17e * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t17a) - (t179 * (l.f5ee * p.p85))) / (t17a * t17a))))), (t17e * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t17a) - (t179 * (l.f5ef * p.p85))) / (t17a * t17a))))), );l.f8d = 0.0;}
        let t17f: f64 = (l.f7b1 / l.f5f1);let t180: f64 = (l.f5f1 - l.f5ed);let t181: f64 = (l.f793 * t180);let t182: f64 = (l.f5ed * p.p85);let t183: f64 = (t181 / t182);let t184: f64 = (t17f + t183);let t185: f64 = (l.f645 * t184);let t186: f64 = (-230.25850929940458);let t187: f64 = if t185 < t186 { 1.0 } else { 0.0 };l.f4cd = t187;l.f4ce = 0.0;
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cb == 0.0)) && (l.f4cd != 0.0)) {let t188: f64 = (-230.25850929940458);let t189: f64 = (l.f7b1 / l.f5f1);let t18a: f64 = (l.f5f1 - l.f5ed);let t18b: f64 = (l.f793 * t18a);let t18c: f64 = (l.f5ed * p.p85);let t18d: f64 = (t18b / t18c);let t18e: f64 = (t189 + t18d);let t18f: f64 = (l.f645 * t18e);let t190: f64 = (t188 - t18f);let t191: f64 = (-230.25850929940458);let t192: f64 = (l.f7b1 / l.f5f1);let t193: f64 = (l.f5f1 - l.f5ed);let t194: f64 = (l.f793 * t193);let t195: f64 = (l.f5ed * p.p85);let t196: f64 = (t194 / t195);let t197: f64 = (t192 + t196);let t198: f64 = (l.f645 * t197);let t199: f64 = (t191 - t198);let t19a: f64 = (-230.25850929940458);let t19b: f64 = (l.f7b1 / l.f5f1);let t19c: f64 = (l.f5f1 - l.f5ed);let t19d: f64 = (l.f793 * t19c);let t19e: f64 = (l.f5ed * p.p85);let t19f: f64 = (t19d / t19e);let t1a0: f64 = (t19b + t19f);let t1a1: f64 = (l.f645 * t1a0);let t1a2: f64 = (t19a - t1a1);let t1a3: f64 = (t1a2 * 0.3333333333333333);let t1a4: f64 = (1.0 + t1a3);let t1a5: f64 = (t199 * t1a4);let t1a6: f64 = (0.5 * t1a5);let t1a7: f64 = (1.0 + t1a6);let t1a8: f64 = (t190 * t1a7);let t1a9: f64 = (1.0 + t1a8);let t1aa: f64 = (1e-100 / t1a9);(l.f8a, l.f8b, l.f8c, ) = (t1aa, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t18c) - (t18b * (l.f5ee * p.p85))) / (t18c * t18c))))) * t1a7) + (t190 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t195) - (t194 * (l.f5ee * p.p85))) / (t195 * t195))))) * t1a4) + (t199 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t19e) - (t19d * (l.f5ee * p.p85))) / (t19e * t19e))))) * 0.3333333333333333))))))) / (t1a9 * t1a9))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t18c) - (t18b * (l.f5ef * p.p85))) / (t18c * t18c))))) * t1a7) + (t190 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t195) - (t194 * (l.f5ef * p.p85))) / (t195 * t195))))) * t1a4) + (t199 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t19e) - (t19d * (l.f5ef * p.p85))) / (t19e * t19e))))) * 0.3333333333333333))))))) / (t1a9 * t1a9))), );l.f8d = 0.0;}
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cb == 0.0)) && (l.f4cd == 0.0)) {let t1ab: f64 = (l.f7b1 / l.f5f1);let t1ac: f64 = (l.f5f1 - l.f5ed);let t1ad: f64 = (l.f793 * t1ac);let t1ae: f64 = (l.f5ed * p.p85);let t1af: f64 = (t1ad / t1ae);let t1b0: f64 = (t1ab + t1af);let t1b1: f64 = (l.f645 * t1b0);let t1b2: f64 = (t1b1 - 230.25850929940458);let t1b3: f64 = (l.f7b1 / l.f5f1);let t1b4: f64 = (l.f5f1 - l.f5ed);let t1b5: f64 = (l.f793 * t1b4);let t1b6: f64 = (l.f5ed * p.p85);let t1b7: f64 = (t1b5 / t1b6);let t1b8: f64 = (t1b3 + t1b7);let t1b9: f64 = (l.f645 * t1b8);let t1ba: f64 = (t1b9 - 230.25850929940458);let t1bb: f64 = (l.f7b1 / l.f5f1);let t1bc: f64 = (l.f5f1 - l.f5ed);let t1bd: f64 = (l.f793 * t1bc);let t1be: f64 = (l.f5ed * p.p85);let t1bf: f64 = (t1bd / t1be);let t1c0: f64 = (t1bb + t1bf);let t1c1: f64 = (l.f645 * t1c0);let t1c2: f64 = (t1c1 - 230.25850929940458);let t1c3: f64 = (t1c2 * 0.3333333333333333);let t1c4: f64 = (1.0 + t1c3);let t1c5: f64 = (t1ba * t1c4);let t1c6: f64 = (0.5 * t1c5);let t1c7: f64 = (1.0 + t1c6);let t1c8: f64 = (t1b2 * t1c7);let t1c9: f64 = (1.0 + t1c8);let t1ca: f64 = (1e100 * t1c9);(l.f8a, l.f8b, l.f8c, ) = (t1ca, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1ae) - (t1ad * (l.f5ee * p.p85))) / (t1ae * t1ae)))) * t1c7) + (t1b2 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1b6) - (t1b5 * (l.f5ee * p.p85))) / (t1b6 * t1b6)))) * t1c4) + (t1ba * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1be) - (t1bd * (l.f5ee * p.p85))) / (t1be * t1be)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1ae) - (t1ad * (l.f5ef * p.p85))) / (t1ae * t1ae)))) * t1c7) + (t1b2 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1b6) - (t1b5 * (l.f5ef * p.p85))) / (t1b6 * t1b6)))) * t1c4) + (t1ba * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1be) - (t1bd * (l.f5ef * p.p85))) / (t1be * t1be)))) * 0.3333333333333333))))))), );l.f8d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let t1cb: f64 = (l.f7b1 * l.f5b);let t1cc: f64 = (l.f5f1 - t1cb);let t1cd: f64 = (l.f5f1 * l.f5f1);let t1ce: f64 = (t1cc / t1cd);let t1cf: f64 = (l.f793 * l.f5b);let t1d0: f64 = (l.f5ed * p.p85);let t1d1: f64 = (t1cf / t1d0);let t1d2: f64 = (t1ce + t1d1);let t1d3: f64 = (l.f645 * t1d2);(l.f61, l.f62, l.f63, ) = (t1d3, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t1cd) - (t1cc * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t1cd * t1cd)) + ((((l.f793 * l.f5c) * t1d0) - (t1cf * (l.f5ee * p.p85))) / (t1d0 * t1d0)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t1cd) - (t1cc * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t1cd * t1cd)) + ((((l.f793 * l.f5d) * t1d0) - (t1cf * (l.f5ef * p.p85))) / (t1d0 * t1d0)))), );l.f64 = 0.0;let t1d4: f64 = (l.f737 - l.f7b1);let t1d5: f64 = (t1d4 * l.f61);let t1d6: f64 = (1.0 + t1d5);let t1d7: f64 = (t1d6 * l.f8a);(l.f536, l.f537, l.f538, ) = (t1d7, (((t1d4 * l.f62) * l.f8a) + (t1d6 * l.f8b)), (((t1d4 * l.f63) * l.f8a) + (t1d6 * l.f8c)), );l.f539 = 0.0;let t1d8: f64 = (l.f5eb * l.f5eb);let t1d9: f64 = (t1d8 / l.f5e3);l.f64f = t1d9;l.f650 = 0.0;let t1da: f64 = (l.f5e9 / l.f645);let t1db: f64 = (l.f5e3 / l.f64f);let t1dc: f64 = (t1db).ln();let t1dd: f64 = (t1da * t1dc);l.f793 = t1dd;l.f794 = 0.0;}
        let t1de: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f4cf = t1de;l.f4d0 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t1df: f64 = (l.f7b1 - l.f793);let t1e0: f64 = (p.p86 * t1df);let t1e1: f64 = (t1e0 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t1e1, 0.0, 0.0, );l.f604 = 0.0;let t1e2: f64 = (p.p86 * l.f793);let t1e3: f64 = (l.f5e9 - t1e2);(l.f5ed, l.f5ee, l.f5ef, ) = (t1e3, 0.0, 0.0, );l.f5f0 = 0.0;let t1e4: f64 = (p.p85 - l.f601);let t1e5: f64 = (t1e4 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1e5, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t1e6: f64 = (4.0 * p.p85);let t1e7: f64 = (t1e6 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1e7, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {
            let (t1e9, t1ea, t1eb,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1e8: f64 = (-l.f6f7);
        (t1e8, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1e9, t1ea, t1eb, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t1ec: f64 = (l.f6f3 * l.f6f3);let t1ed: f64 = (t1ec + l.f6f7);let t1ee: f64 = (t1ed).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1ee, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1ee)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1ee)), );l.f6fa = 0.0;let t1ef: f64 = (l.f6f3 / l.f6f7);let t1f0: f64 = (1.0 + t1ef);let t1f1: f64 = (0.5 * t1f0);(l.f55, l.f56, l.f57, ) = (t1f1, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;let t1f2: f64 = (l.f6f3 + l.f6f7);let t1f3: f64 = (0.5 * t1f2);let t1f4: f64 = (p.p85 - t1f3);(l.f605, l.f606, l.f607, ) = (t1f4, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t1f5: f64 = (l.f605 - l.f5e9);let t1f6: f64 = (t1f5 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1f6, l.f606, l.f607, );l.f6f6 = 0.0;let t1f7: f64 = (4.0 * l.f5e9);let t1f8: f64 = (t1f7 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1f8, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {
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
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t1fd: f64 = (l.f6f3 * l.f6f3);let t1fe: f64 = (t1fd + l.f6f7);let t1ff: f64 = (t1fe).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1ff, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1ff)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1ff)), );l.f6fa = 0.0;let t200: f64 = (l.f6f3 / l.f6f7);let t201: f64 = (1.0 + t200);let t202: f64 = (0.5 * t201);(l.f51, l.f52, l.f53, ) = (t202, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t203: f64 = (l.f6f3 + l.f6f7);let t204: f64 = (0.5 * t203);let t205: f64 = (l.f5e9 + t204);(l.f5f1, l.f5f2, l.f5f3, ) = (t205, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t206: f64 = (p.p85 - l.f5ed);let t207: f64 = (t206 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t207, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t208: f64 = (4.0 * p.p85);let t209: f64 = (t208 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t209, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {
            let (t20b, t20c, t20d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t20a: f64 = (-l.f6f7);
        (t20a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t20b, t20c, t20d, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t20e: f64 = (l.f6f3 * l.f6f3);let t20f: f64 = (t20e + l.f6f7);let t210: f64 = (t20f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t210, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t210)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t210)), );l.f6fa = 0.0;let t211: f64 = (l.f6f3 + l.f6f7);let t212: f64 = (0.5 * t211);let t213: f64 = (p.p85 - t212);(l.f5ed, l.f5ee, l.f5ef, ) = (t213, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t214: f64 = (l.f5ed - l.f5e9);let t215: f64 = (t214 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t215, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t216: f64 = (4.0 * l.f5e9);let t217: f64 = (t216 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t217, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {
            let (t219, t21a, t21b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t218: f64 = (-l.f6f7);
        (t218, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t219, t21a, t21b, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t21c: f64 = (l.f6f3 * l.f6f3);let t21d: f64 = (t21c + l.f6f7);let t21e: f64 = (t21d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t21e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t21e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t21e)), );l.f6fa = 0.0;let t21f: f64 = (l.f6f3 + l.f6f7);let t220: f64 = (0.5 * t21f);let t221: f64 = (l.f5e9 + t220);(l.f5ed, l.f5ee, l.f5ef, ) = (t221, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t222: f64 = (p.p86 * l.f55);let t223: f64 = (t222 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t223, (((p.p86 * l.f56) * l.f51) + (t222 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t222 * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
        let t224: f64 = (l.f7b1 / l.f5f1);let t225: f64 = (l.f5f1 - l.f5ed);let t226: f64 = (l.f793 * t225);let t227: f64 = (l.f5ed * p.p85);let t228: f64 = (t226 / t227);let t229: f64 = (t224 + t228);let t22a: f64 = (l.f645 * t229);let t22b: f64 = (t22a).abs();let t22c: f64 = if t22b < 230.25850929940458 { 1.0 } else { 0.0 };l.f4d1 = t22c;l.f4d2 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d1 != 0.0)) {let t22d: f64 = (l.f7b1 / l.f5f1);let t22e: f64 = (l.f5f1 - l.f5ed);let t22f: f64 = (l.f793 * t22e);let t230: f64 = (l.f5ed * p.p85);let t231: f64 = (t22f / t230);let t232: f64 = (t22d + t231);let t233: f64 = (l.f645 * t232);let t234: f64 = (t233).exp();(l.f93, l.f94, l.f95, ) = (t234, (t234 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t230) - (t22f * (l.f5ee * p.p85))) / (t230 * t230))))), (t234 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t230) - (t22f * (l.f5ef * p.p85))) / (t230 * t230))))), );l.f96 = 0.0;}
        let t235: f64 = (l.f7b1 / l.f5f1);let t236: f64 = (l.f5f1 - l.f5ed);let t237: f64 = (l.f793 * t236);let t238: f64 = (l.f5ed * p.p85);let t239: f64 = (t237 / t238);let t23a: f64 = (t235 + t239);let t23b: f64 = (l.f645 * t23a);let t23c: f64 = (-230.25850929940458);let t23d: f64 = if t23b < t23c { 1.0 } else { 0.0 };l.f4d3 = t23d;l.f4d4 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d1 == 0.0)) && (l.f4d3 != 0.0)) {let t23e: f64 = (-230.25850929940458);let t23f: f64 = (l.f7b1 / l.f5f1);let t240: f64 = (l.f5f1 - l.f5ed);let t241: f64 = (l.f793 * t240);let t242: f64 = (l.f5ed * p.p85);let t243: f64 = (t241 / t242);let t244: f64 = (t23f + t243);let t245: f64 = (l.f645 * t244);let t246: f64 = (t23e - t245);let t247: f64 = (-230.25850929940458);let t248: f64 = (l.f7b1 / l.f5f1);let t249: f64 = (l.f5f1 - l.f5ed);let t24a: f64 = (l.f793 * t249);let t24b: f64 = (l.f5ed * p.p85);let t24c: f64 = (t24a / t24b);let t24d: f64 = (t248 + t24c);let t24e: f64 = (l.f645 * t24d);let t24f: f64 = (t247 - t24e);let t250: f64 = (-230.25850929940458);let t251: f64 = (l.f7b1 / l.f5f1);let t252: f64 = (l.f5f1 - l.f5ed);let t253: f64 = (l.f793 * t252);let t254: f64 = (l.f5ed * p.p85);let t255: f64 = (t253 / t254);let t256: f64 = (t251 + t255);let t257: f64 = (l.f645 * t256);let t258: f64 = (t250 - t257);let t259: f64 = (t258 * 0.3333333333333333);let t25a: f64 = (1.0 + t259);let t25b: f64 = (t24f * t25a);let t25c: f64 = (0.5 * t25b);let t25d: f64 = (1.0 + t25c);let t25e: f64 = (t246 * t25d);let t25f: f64 = (1.0 + t25e);let t260: f64 = (1e-100 / t25f);(l.f93, l.f94, l.f95, ) = (t260, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t242) - (t241 * (l.f5ee * p.p85))) / (t242 * t242))))) * t25d) + (t246 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t24b) - (t24a * (l.f5ee * p.p85))) / (t24b * t24b))))) * t25a) + (t24f * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t254) - (t253 * (l.f5ee * p.p85))) / (t254 * t254))))) * 0.3333333333333333))))))) / (t25f * t25f))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t242) - (t241 * (l.f5ef * p.p85))) / (t242 * t242))))) * t25d) + (t246 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t24b) - (t24a * (l.f5ef * p.p85))) / (t24b * t24b))))) * t25a) + (t24f * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t254) - (t253 * (l.f5ef * p.p85))) / (t254 * t254))))) * 0.3333333333333333))))))) / (t25f * t25f))), );l.f96 = 0.0;}
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d1 == 0.0)) && (l.f4d3 == 0.0)) {let t261: f64 = (l.f7b1 / l.f5f1);let t262: f64 = (l.f5f1 - l.f5ed);let t263: f64 = (l.f793 * t262);let t264: f64 = (l.f5ed * p.p85);let t265: f64 = (t263 / t264);let t266: f64 = (t261 + t265);let t267: f64 = (l.f645 * t266);let t268: f64 = (t267 - 230.25850929940458);let t269: f64 = (l.f7b1 / l.f5f1);let t26a: f64 = (l.f5f1 - l.f5ed);let t26b: f64 = (l.f793 * t26a);let t26c: f64 = (l.f5ed * p.p85);let t26d: f64 = (t26b / t26c);let t26e: f64 = (t269 + t26d);let t26f: f64 = (l.f645 * t26e);let t270: f64 = (t26f - 230.25850929940458);let t271: f64 = (l.f7b1 / l.f5f1);let t272: f64 = (l.f5f1 - l.f5ed);let t273: f64 = (l.f793 * t272);let t274: f64 = (l.f5ed * p.p85);let t275: f64 = (t273 / t274);let t276: f64 = (t271 + t275);let t277: f64 = (l.f645 * t276);let t278: f64 = (t277 - 230.25850929940458);let t279: f64 = (t278 * 0.3333333333333333);let t27a: f64 = (1.0 + t279);let t27b: f64 = (t270 * t27a);let t27c: f64 = (0.5 * t27b);let t27d: f64 = (1.0 + t27c);let t27e: f64 = (t268 * t27d);let t27f: f64 = (1.0 + t27e);let t280: f64 = (1e100 * t27f);(l.f93, l.f94, l.f95, ) = (t280, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t264) - (t263 * (l.f5ee * p.p85))) / (t264 * t264)))) * t27d) + (t268 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t26c) - (t26b * (l.f5ee * p.p85))) / (t26c * t26c)))) * t27a) + (t270 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t274) - (t273 * (l.f5ee * p.p85))) / (t274 * t274)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t264) - (t263 * (l.f5ef * p.p85))) / (t264 * t264)))) * t27d) + (t268 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t26c) - (t26b * (l.f5ef * p.p85))) / (t26c * t26c)))) * t27a) + (t270 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t274) - (t273 * (l.f5ef * p.p85))) / (t274 * t274)))) * 0.3333333333333333))))))), );l.f96 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let t281: f64 = (l.f7b1 * l.f5b);let t282: f64 = (l.f5f1 - t281);let t283: f64 = (l.f5f1 * l.f5f1);let t284: f64 = (t282 / t283);let t285: f64 = (l.f793 * l.f5b);let t286: f64 = (l.f5ed * p.p85);let t287: f64 = (t285 / t286);let t288: f64 = (t284 + t287);let t289: f64 = (l.f645 * t288);(l.f61, l.f62, l.f63, ) = (t289, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t283) - (t282 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t283 * t283)) + ((((l.f793 * l.f5c) * t286) - (t285 * (l.f5ee * p.p85))) / (t286 * t286)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t283) - (t282 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t283 * t283)) + ((((l.f793 * l.f5d) * t286) - (t285 * (l.f5ef * p.p85))) / (t286 * t286)))), );l.f64 = 0.0;let t28a: f64 = (l.f737 - l.f7b1);let t28b: f64 = (t28a * l.f61);let t28c: f64 = (1.0 + t28b);let t28d: f64 = (t28c * l.f93);(l.f53e, l.f53f, l.f540, ) = (t28d, (((t28a * l.f62) * l.f93) + (t28c * l.f94)), (((t28a * l.f63) * l.f93) + (t28c * l.f95)), );l.f541 = 0.0;let t28e: f64 = (l.f5eb * l.f5eb);let t28f: f64 = (t28e / l.f5e1);l.f64f = t28f;l.f650 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let t290: f64 = (l.f5e7 / l.f645);let t291: f64 = (l.f5e1 / l.f64f);let t292: f64 = (t291).ln();let t293: f64 = (t290 * t292);l.f793 = t293;l.f794 = 0.0;}
        let t294: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f4d5 = t294;l.f4d6 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t295: f64 = (l.f7b1 - l.f793);let t296: f64 = (p.p86 * t295);let t297: f64 = (t296 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t297, 0.0, 0.0, );l.f604 = 0.0;let t298: f64 = (p.p86 * l.f793);let t299: f64 = (l.f5e7 - t298);(l.f5ed, l.f5ee, l.f5ef, ) = (t299, 0.0, 0.0, );l.f5f0 = 0.0;let t29a: f64 = (p.p85 - l.f601);let t29b: f64 = (t29a - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t29b, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t29c: f64 = (4.0 * p.p85);let t29d: f64 = (t29c * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t29d, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {
            let (t29f, t2a0, t2a1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t29e: f64 = (-l.f6f7);
        (t29e, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t29f, t2a0, t2a1, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t2a2: f64 = (l.f6f3 * l.f6f3);let t2a3: f64 = (t2a2 + l.f6f7);let t2a4: f64 = (t2a3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2a4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2a4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2a4)), );l.f6fa = 0.0;let t2a5: f64 = (l.f6f3 / l.f6f7);let t2a6: f64 = (1.0 + t2a5);let t2a7: f64 = (0.5 * t2a6);(l.f55, l.f56, l.f57, ) = (t2a7, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;let t2a8: f64 = (l.f6f3 + l.f6f7);let t2a9: f64 = (0.5 * t2a8);let t2aa: f64 = (p.p85 - t2a9);(l.f605, l.f606, l.f607, ) = (t2aa, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t2ab: f64 = (l.f605 - l.f5e7);let t2ac: f64 = (t2ab - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2ac, l.f606, l.f607, );l.f6f6 = 0.0;let t2ad: f64 = (4.0 * l.f5e7);let t2ae: f64 = (t2ad * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2ae, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {
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
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t2b3: f64 = (l.f6f3 * l.f6f3);let t2b4: f64 = (t2b3 + l.f6f7);let t2b5: f64 = (t2b4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2b5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2b5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2b5)), );l.f6fa = 0.0;let t2b6: f64 = (l.f6f3 / l.f6f7);let t2b7: f64 = (1.0 + t2b6);let t2b8: f64 = (0.5 * t2b7);(l.f51, l.f52, l.f53, ) = (t2b8, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t2b9: f64 = (l.f6f3 + l.f6f7);let t2ba: f64 = (0.5 * t2b9);let t2bb: f64 = (l.f5e7 + t2ba);(l.f5f1, l.f5f2, l.f5f3, ) = (t2bb, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t2bc: f64 = (p.p85 - l.f5ed);let t2bd: f64 = (t2bc - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2bd, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t2be: f64 = (4.0 * p.p85);let t2bf: f64 = (t2be * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2bf, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {
            let (t2c1, t2c2, t2c3,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2c0: f64 = (-l.f6f7);
        (t2c0, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2c1, t2c2, t2c3, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t2c4: f64 = (l.f6f3 * l.f6f3);let t2c5: f64 = (t2c4 + l.f6f7);let t2c6: f64 = (t2c5).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2c6, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2c6)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2c6)), );l.f6fa = 0.0;let t2c7: f64 = (l.f6f3 + l.f6f7);let t2c8: f64 = (0.5 * t2c7);let t2c9: f64 = (p.p85 - t2c8);(l.f5ed, l.f5ee, l.f5ef, ) = (t2c9, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t2ca: f64 = (l.f5ed - l.f5e7);let t2cb: f64 = (t2ca - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2cb, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t2cc: f64 = (4.0 * l.f5e7);let t2cd: f64 = (t2cc * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2cd, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {
            let (t2cf, t2d0, t2d1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2ce: f64 = (-l.f6f7);
        (t2ce, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2cf, t2d0, t2d1, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t2d2: f64 = (l.f6f3 * l.f6f3);let t2d3: f64 = (t2d2 + l.f6f7);let t2d4: f64 = (t2d3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2d4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2d4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2d4)), );l.f6fa = 0.0;let t2d5: f64 = (l.f6f3 + l.f6f7);let t2d6: f64 = (0.5 * t2d5);let t2d7: f64 = (l.f5e7 + t2d6);(l.f5ed, l.f5ee, l.f5ef, ) = (t2d7, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t2d8: f64 = (p.p86 * l.f55);let t2d9: f64 = (t2d8 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t2d9, (((p.p86 * l.f56) * l.f51) + (t2d8 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t2d8 * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
        let t2da: f64 = (l.f7b1 / l.f5f1);let t2db: f64 = (l.f5f1 - l.f5ed);let t2dc: f64 = (l.f793 * t2db);let t2dd: f64 = (l.f5ed * p.p85);let t2de: f64 = (t2dc / t2dd);let t2df: f64 = (t2da + t2de);let t2e0: f64 = (l.f645 * t2df);let t2e1: f64 = (t2e0).abs();let t2e2: f64 = if t2e1 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4d7 = t2e2;l.f4d8 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d7 != 0.0)) {let t2e3: f64 = (l.f7b1 / l.f5f1);let t2e4: f64 = (l.f5f1 - l.f5ed);let t2e5: f64 = (l.f793 * t2e4);let t2e6: f64 = (l.f5ed * p.p85);let t2e7: f64 = (t2e5 / t2e6);let t2e8: f64 = (t2e3 + t2e7);let t2e9: f64 = (l.f645 * t2e8);let t2ea: f64 = (t2e9).exp();(l.f8e, l.f8f, l.f90, ) = (t2ea, (t2ea * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2e6) - (t2e5 * (l.f5ee * p.p85))) / (t2e6 * t2e6))))), (t2ea * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2e6) - (t2e5 * (l.f5ef * p.p85))) / (t2e6 * t2e6))))), );l.f91 = 0.0;}
        let t2eb: f64 = (l.f7b1 / l.f5f1);let t2ec: f64 = (l.f5f1 - l.f5ed);let t2ed: f64 = (l.f793 * t2ec);let t2ee: f64 = (l.f5ed * p.p85);let t2ef: f64 = (t2ed / t2ee);let t2f0: f64 = (t2eb + t2ef);let t2f1: f64 = (l.f645 * t2f0);let t2f2: f64 = (-230.25850929940458);let t2f3: f64 = if t2f1 < t2f2 { 1.0 } else { 0.0 };l.f4d9 = t2f3;l.f4da = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d7 == 0.0)) && (l.f4d9 != 0.0)) {let t2f4: f64 = (-230.25850929940458);let t2f5: f64 = (l.f7b1 / l.f5f1);let t2f6: f64 = (l.f5f1 - l.f5ed);let t2f7: f64 = (l.f793 * t2f6);let t2f8: f64 = (l.f5ed * p.p85);let t2f9: f64 = (t2f7 / t2f8);let t2fa: f64 = (t2f5 + t2f9);let t2fb: f64 = (l.f645 * t2fa);let t2fc: f64 = (t2f4 - t2fb);let t2fd: f64 = (-230.25850929940458);let t2fe: f64 = (l.f7b1 / l.f5f1);let t2ff: f64 = (l.f5f1 - l.f5ed);let t300: f64 = (l.f793 * t2ff);let t301: f64 = (l.f5ed * p.p85);let t302: f64 = (t300 / t301);let t303: f64 = (t2fe + t302);let t304: f64 = (l.f645 * t303);let t305: f64 = (t2fd - t304);let t306: f64 = (-230.25850929940458);let t307: f64 = (l.f7b1 / l.f5f1);let t308: f64 = (l.f5f1 - l.f5ed);let t309: f64 = (l.f793 * t308);let t30a: f64 = (l.f5ed * p.p85);let t30b: f64 = (t309 / t30a);let t30c: f64 = (t307 + t30b);let t30d: f64 = (l.f645 * t30c);let t30e: f64 = (t306 - t30d);let t30f: f64 = (t30e * 0.3333333333333333);let t310: f64 = (1.0 + t30f);let t311: f64 = (t305 * t310);let t312: f64 = (0.5 * t311);let t313: f64 = (1.0 + t312);let t314: f64 = (t2fc * t313);let t315: f64 = (1.0 + t314);let t316: f64 = (1e-100 / t315);(l.f8e, l.f8f, l.f90, ) = (t316, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2f8) - (t2f7 * (l.f5ee * p.p85))) / (t2f8 * t2f8))))) * t313) + (t2fc * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t301) - (t300 * (l.f5ee * p.p85))) / (t301 * t301))))) * t310) + (t305 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t30a) - (t309 * (l.f5ee * p.p85))) / (t30a * t30a))))) * 0.3333333333333333))))))) / (t315 * t315))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2f8) - (t2f7 * (l.f5ef * p.p85))) / (t2f8 * t2f8))))) * t313) + (t2fc * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t301) - (t300 * (l.f5ef * p.p85))) / (t301 * t301))))) * t310) + (t305 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t30a) - (t309 * (l.f5ef * p.p85))) / (t30a * t30a))))) * 0.3333333333333333))))))) / (t315 * t315))), );l.f91 = 0.0;}
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d7 == 0.0)) && (l.f4d9 == 0.0)) {let t317: f64 = (l.f7b1 / l.f5f1);let t318: f64 = (l.f5f1 - l.f5ed);let t319: f64 = (l.f793 * t318);let t31a: f64 = (l.f5ed * p.p85);let t31b: f64 = (t319 / t31a);let t31c: f64 = (t317 + t31b);let t31d: f64 = (l.f645 * t31c);let t31e: f64 = (t31d - 230.25850929940458);let t31f: f64 = (l.f7b1 / l.f5f1);let t320: f64 = (l.f5f1 - l.f5ed);let t321: f64 = (l.f793 * t320);let t322: f64 = (l.f5ed * p.p85);let t323: f64 = (t321 / t322);let t324: f64 = (t31f + t323);let t325: f64 = (l.f645 * t324);let t326: f64 = (t325 - 230.25850929940458);let t327: f64 = (l.f7b1 / l.f5f1);let t328: f64 = (l.f5f1 - l.f5ed);let t329: f64 = (l.f793 * t328);let t32a: f64 = (l.f5ed * p.p85);let t32b: f64 = (t329 / t32a);let t32c: f64 = (t327 + t32b);let t32d: f64 = (l.f645 * t32c);let t32e: f64 = (t32d - 230.25850929940458);let t32f: f64 = (t32e * 0.3333333333333333);let t330: f64 = (1.0 + t32f);let t331: f64 = (t326 * t330);let t332: f64 = (0.5 * t331);let t333: f64 = (1.0 + t332);let t334: f64 = (t31e * t333);let t335: f64 = (1.0 + t334);let t336: f64 = (1e100 * t335);(l.f8e, l.f8f, l.f90, ) = (t336, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t31a) - (t319 * (l.f5ee * p.p85))) / (t31a * t31a)))) * t333) + (t31e * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t322) - (t321 * (l.f5ee * p.p85))) / (t322 * t322)))) * t330) + (t326 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t32a) - (t329 * (l.f5ee * p.p85))) / (t32a * t32a)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t31a) - (t319 * (l.f5ef * p.p85))) / (t31a * t31a)))) * t333) + (t31e * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t322) - (t321 * (l.f5ef * p.p85))) / (t322 * t322)))) * t330) + (t326 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t32a) - (t329 * (l.f5ef * p.p85))) / (t32a * t32a)))) * 0.3333333333333333))))))), );l.f91 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let t337: f64 = (l.f7b1 * l.f5b);let t338: f64 = (l.f5f1 - t337);let t339: f64 = (l.f5f1 * l.f5f1);let t33a: f64 = (t338 / t339);let t33b: f64 = (l.f793 * l.f5b);let t33c: f64 = (l.f5ed * p.p85);let t33d: f64 = (t33b / t33c);let t33e: f64 = (t33a + t33d);let t33f: f64 = (l.f645 * t33e);(l.f61, l.f62, l.f63, ) = (t33f, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t339) - (t338 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t339 * t339)) + ((((l.f793 * l.f5c) * t33c) - (t33b * (l.f5ee * p.p85))) / (t33c * t33c)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t339) - (t338 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t339 * t339)) + ((((l.f793 * l.f5d) * t33c) - (t33b * (l.f5ef * p.p85))) / (t33c * t33c)))), );l.f64 = 0.0;let t340: f64 = (l.f737 - l.f7b1);let t341: f64 = (t340 * l.f61);let t342: f64 = (1.0 + t341);let t343: f64 = (t342 * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t343, (((t340 * l.f62) * l.f8e) + (t342 * l.f8f)), (((t340 * l.f63) * l.f8e) + (t342 * l.f90)), );l.f53d = 0.0;}
        if ((l.f29a != 0.0) && (l.f4ab != 0.0)) {let t344: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t344, l.f537, l.f538, );l.f539 = 0.0;let t345: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t345, l.f53f, l.f540, );l.f541 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        l: &mut StampLocals,
    ) {
        if ((l.f29a != 0.0) && (l.f4ab != 0.0)) {let t346: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t346, l.f53b, l.f53c, );l.f53d = 0.0;let t347: f64 = (1.0 / l.f825);l.f817 = t347;l.f818 = 0.0;}
        let t348: f64 = if l.f737 > 0.0 { 1.0 } else { 0.0 };l.f4db = t348;l.f4dc = 0.0;
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4db != 0.0)) {let t349: f64 = (2.0 + l.f817);let t34a: f64 = (l.f817 + 1.0);let t34b: f64 = (l.f817 + 3.0);let t34c: f64 = (t34a * t34b);let t34d: f64 = (t34c).sqrt();let t34e: f64 = (t349 + t34d);let t34f: f64 = (t34e).ln();let t350: f64 = (l.f643 * t34f);let t351: f64 = (2.0 * t350);l.f714 = t351;l.f715 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4db == 0.0)) {let t352: f64 = (-l.f737);let t353: f64 = (2.0 * l.f825);let t354: f64 = (t353 + 1.0);let t355: f64 = (1.0 + l.f825);let t356: f64 = (3.0 * l.f825);let t357: f64 = (1.0 + t356);let t358: f64 = (t355 * t357);let t359: f64 = (t358).sqrt();let t35a: f64 = (t354 + t359);let t35b: f64 = (t35a).ln();let t35c: f64 = (l.f643 * t35b);let t35d: f64 = (2.0 * t35c);let t35e: f64 = (t352 + t35d);l.f714 = t35e;l.f715 = 0.0;}
        if ((l.f29a != 0.0) && (l.f4ab != 0.0)) {let t35f: f64 = (l.f76f - l.f714);l.f79c = t35f;l.f79d = 0.0;let t360: f64 = (l.f737 + l.f79c);let t361: f64 = (l.f737 - l.f79c);let t362: f64 = (l.f737 - l.f79c);let t363: f64 = (t361 * t362);let t364: f64 = (4.0 * l.f643);let t365: f64 = (t364 * l.f643);let t366: f64 = (t363 + t365);let t367: f64 = (t366).sqrt();let t368: f64 = (t360 - t367);let t369: f64 = (0.5 * t368);l.f7a2 = t369;l.f7a3 = 0.0;let t36a: f64 = (l.f737 + l.f755);let t36b: f64 = (l.f737 - l.f755);let t36c: f64 = (l.f737 - l.f755);let t36d: f64 = (t36b * t36c);let t36e: f64 = (4.0 * l.f647);let t36f: f64 = (t36e * l.f647);let t370: f64 = (t36d + t36f);let t371: f64 = (t370).sqrt();let t372: f64 = (t36a - t371);let t373: f64 = (0.5 * t372);l.f750 = t373;l.f751 = 0.0;let t374: f64 = l.f737;let t375: f64 = l.f737;let t376: f64 = l.f737;let t377: f64 = (t375 * t376);let t378: f64 = (4.0 * 1e-6);let t379: f64 = (t378 * 1e-6);let t37a: f64 = (t377 + t379);let t37b: f64 = (t37a).sqrt();let t37c: f64 = (t374 - t37b);let t37d: f64 = (0.5 * t37c);l.f74a = t37d;l.f74b = 0.0;}
        if ((l.f29a != 0.0) && (l.f4ab == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );l.f539 = 0.0;(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );l.f541 = 0.0;(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f53d = 0.0;l.f714 = 0.0;l.f715 = 0.0;l.f796 = 0.0;l.f797 = 0.0;l.f825 = 0.0;l.f826 = 0.0;l.f7a2 = 0.0;l.f7a3 = 0.0;l.f750 = 0.0;l.f751 = 0.0;l.f74a = 0.0;l.f74b = 0.0;}
        let t37e: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f4dd = t37e;l.f4de = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        l: &mut StampLocals,
    ) {
        if ((l.f29a != 0.0) && (l.f4dd != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );l.f565 = 0.0;(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );l.f555 = 0.0;(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );l.f68f = 0.0;}
        let t37f: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.fdd = t37f;l.fde = 0.0;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdd != 0.0)) {let t380: f64 = (l.f796 * l.f769);let t381: f64 = (1.0 - t380);let t382: f64 = (t381).sqrt();l.f6fc = t382;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdd == 0.0)) {let t383: f64 = (l.f796 * l.f769);let t384: f64 = (1.0 - t383);let t385: f64 = (t384).powf(l.f60b);l.f6fc = t385;l.f6fd = 0.0;}
        if ((l.f29a != 0.0) && (l.f4dd == 0.0)) {let t386: f64 = (1.0 - l.f6fc);let t387: f64 = (l.f69e * t386);let t388: f64 = (l.f737 - l.f796);let t389: f64 = (l.f698 * t388);let t38a: f64 = (t387 + t389);(l.f68c, l.f68d, l.f68e, ) = (t38a, 0.0, 0.0, );l.f68f = 0.0;let t38b: f64 = (l.f542 * l.f536);(l.f52f, l.f530, l.f531, ) = (t38b, (l.f542 * l.f537), (l.f542 * l.f538), );l.f532 = 0.0;}
        let t38c: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.fdf = t38c;l.fe0 = 0.0;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf != 0.0)) {l.f758 = 0.0;l.f759 = 0.0;l.f7e9 = 0.0;l.f7ea = 0.0;l.f7d1 = 0.0;l.f7d2 = 0.0;l.f9 = 0.0;l.fa = 0.0;l.f593 = 0.0;l.f594 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) {let t38d: f64 = (l.f75d - l.f7a2);l.f758 = t38d;l.f759 = 0.0;let t38e: f64 = (l.f714 / l.f758);let t38f: f64 = (1.0 - t38e);let t390: f64 = (t38f).sqrt();let t391: f64 = (1.0 - t390);l.f7ef = t391;l.f7f0 = 0.0;}
        let t392: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.fe1 = t392;l.fe2 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) && (l.fe1 != 0.0)) {l.f66 = 0.0;l.f67 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) && (l.fe1 == 0.0)) {let t393: f64 = (l.f7ef * l.f7ef);let t394: f64 = (l.f7ef).ln();let t395: f64 = (t393 * t394);let t396: f64 = (1.0 - l.f7ef);let t397: f64 = (t395 / t396);let t398: f64 = (t397 + l.f7ef);let t399: f64 = (2.0 * l.f623);let t39a: f64 = (1.0 - t399);let t39b: f64 = (t398 * t39a);l.f66 = t39b;l.f67 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) {let t39c: f64 = (l.f7ef + l.f66);l.f7e9 = t39c;l.f7ea = 0.0;}
        let t39d: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.fe3 = t39d;l.fe4 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) && (l.fe3 != 0.0)) {let t39e: f64 = (l.f758 * l.f773);let t39f: f64 = (t39e).sqrt();l.f6fc = t39f;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) && (l.fe3 == 0.0)) {let t3a0: f64 = (l.f758 * l.f773);let t3a1: f64 = (t3a0).powf(l.f623);l.f6fc = t3a1;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) {let t3a2: f64 = (l.f7d6 * l.f6fc);l.f7d1 = t3a2;l.f7d2 = 0.0;let t3a3: f64 = (l.f825 - 1.0);let t3a4: f64 = (t3a3 * l.f7d1);let t3a5: f64 = (l.fc9 * t3a4);l.f9 = t3a5;l.fa = 0.0;let t3a6: f64 = (l.f9 * l.f7e9);let t3a7: f64 = (l.f39 * t3a6);l.f593 = t3a7;l.f594 = 0.0;}
        let t3a8: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.fe5 = t3a8;l.fe6 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 != 0.0)) {l.f599 = 0.0;l.f59a = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t3a9: f64 = (l.f7d1 * l.f60b);let t3aa: f64 = (t3a9 / l.f758);let t3ab: f64 = (l.f1e * t3aa);l.f19 = t3ab;l.f1a = 0.0;let t3ac: f64 = (0.666666666666667 * l.fe);let t3ad: f64 = (t3ac / l.f19);l.f71a = t3ad;l.f71b = 0.0;let t3ae: f64 = (l.f71a * l.f71a);l.f72c = t3ae;l.f72d = 0.0;let t3af: f64 = (l.f72c * l.f72c);let t3b0: f64 = (l.f72c * l.f72c);let t3b1: f64 = (t3b0 + 1.0);let t3b2: f64 = (t3af / t3b1);let t3b3: f64 = (t3b2).sqrt();l.f726 = t3b3;l.f727 = 0.0;let t3b4: f64 = (l.f726).abs();let t3b5: f64 = (t3b4).sqrt();l.f6c1 = t3b5;l.f6c2 = 0.0;let t3b6: f64 = (l.f726 * l.f6c1);l.f732 = t3b6;l.f733 = 0.0;}
        let t3b7: f64 = (-l.f623);let t3b8: f64 = (t3b7 * l.f611);let t3b9: f64 = (-1.0);let t3ba: f64 = if t3b8 == t3b9 { 1.0 } else { 0.0 };l.fe7 = t3ba;l.fe8 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fe7 != 0.0)) {let t3bb: f64 = (l.f19 * l.f732);let t3bc: f64 = (1.0 + t3bb);let t3bd: f64 = (1.0 / t3bc);l.f7e3 = t3bd;l.f7e4 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fe7 == 0.0)) {let t3be: f64 = (l.f19 * l.f732);let t3bf: f64 = (1.0 + t3be);let t3c0: f64 = (-l.f623);let t3c1: f64 = (t3c0 * l.f611);let t3c2: f64 = (t3bf).powf(t3c1);l.f7e3 = t3c2;l.f7e4 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t3c3: f64 = (l.f7e9 * l.f7e3);let t3c4: f64 = (l.f7e9 + l.f7e3);let t3c5: f64 = (t3c3 / t3c4);l.f7f5 = t3c5;l.f7f6 = 0.0;let t3c6: f64 = (l.f19 / l.f6c1);let t3c7: f64 = (0.375 * t3c6);let t3c8: f64 = (t3c7).sqrt();l.f5a8 = t3c8;l.f5a9 = 0.0;let t3c9: f64 = (l.f71a * l.f6c1);let t3ca: f64 = (2.0 * t3c9);let t3cb: f64 = (t3ca - l.f726);l.f5b4 = t3cb;l.f5b5 = 0.0;let t3cc: f64 = (l.fe * l.f71a);let t3cd: f64 = (t3cc * l.f6c1);let t3ce: f64 = (l.fe * l.f726);let t3cf: f64 = (t3cd - t3ce);let t3d0: f64 = (l.f19 * l.f732);let t3d1: f64 = (0.5 * t3d0);let t3d2: f64 = (t3cf + t3d1);l.f5d4 = t3d2;l.f5d5 = 0.0;let t3d3: f64 = (l.f5b4 - 1.0);let t3d4: f64 = (t3d3 * l.f5a8);l.f7fb = t3d4;l.f7fc = 0.0;let t3d5: f64 = (l.f7fb * l.f7fb);l.f811 = t3d5;l.f812 = 0.0;}
        let t3d6: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.fe9 = t3d6;l.fea = 0.0;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fe9 != 0.0)) {let t3d7: f64 = (l.f62b * l.f7fb);let t3d8: f64 = (1.0 + t3d7);let t3d9: f64 = (1.0 / t3d8);l.f6e2 = t3d9;l.f6e3 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fe9 == 0.0)) {let t3da: f64 = (l.f62b * l.f7fb);let t3db: f64 = (1.0 - t3da);let t3dc: f64 = (1.0 / t3db);l.f6e2 = t3dc;l.f6e3 = 0.0;}
        let t3dd: f64 = (-l.f811);let t3de: f64 = (t3dd + l.f5d4);let t3df: f64 = (-230.25850929940458);let t3e0: f64 = if t3de > t3df { 1.0 } else { 0.0 };l.feb = t3e0;l.fec = 0.0;
    }
}
