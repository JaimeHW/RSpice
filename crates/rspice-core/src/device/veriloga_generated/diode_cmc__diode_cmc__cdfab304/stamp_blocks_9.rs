#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_144(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f4 == 0.0)) && (l.f3f6 == 0.0)) {let t0: f64 = (l.f745 * l.f645);let t1: f64 = (0.5 * t0);let t2: f64 = (t1 - 230.25850929940458);let t3: f64 = (l.f745 * l.f645);let t4: f64 = (0.5 * t3);let t5: f64 = (t4 - 230.25850929940458);let t6: f64 = (l.f745 * l.f645);let t7: f64 = (0.5 * t6);let t8: f64 = (t7 - 230.25850929940458);let t9: f64 = (t8 * 0.3333333333333333);let ta: f64 = (1.0 + t9);let tb: f64 = (t5 * ta);let tc: f64 = (0.5 * tb);let td: f64 = (1.0 + tc);let te: f64 = (t2 * td);let tf: f64 = (1.0 + te);let t10: f64 = (1e100 * tf);(l.f824, l.f827, l.f828, ) = (t10, (1e100 * (((0.5 * (l.f746 * l.f645)) * td) + (t2 * (0.5 * (((0.5 * (l.f746 * l.f645)) * ta) + (t5 * ((0.5 * (l.f746 * l.f645)) * 0.3333333333333333))))))), (1e100 * (((0.5 * (l.f747 * l.f645)) * td) + (t2 * (0.5 * (((0.5 * (l.f747 * l.f645)) * ta) + (t5 * ((0.5 * (l.f747 * l.f645)) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) {let t11: f64 = (l.f5eb * l.f5eb);let t12: f64 = (t11 / l.f5df);l.f64f = t12;let t13: f64 = (l.f5e5 / l.f645);let t14: f64 = (l.f5df / l.f64f);let t15: f64 = (t14).ln();let t16: f64 = (t13 * t15);l.f793 = t16;}
        let t17: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f3f8 = t17;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t18: f64 = (l.f745 - l.f793);let t19: f64 = (p.p86 * t18);let t1a: f64 = (t19 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t1a, (p.p86 * l.f746), (p.p86 * l.f747), );let t1b: f64 = (p.p86 * l.f793);let t1c: f64 = (l.f5e5 - t1b);(l.f5ed, l.f5ee, l.f5ef, ) = (t1c, 0.0, 0.0, );let t1d: f64 = (p.p85 - l.f601);let t1e: f64 = (t1d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1e, (-l.f602), (-l.f603), );let t1f: f64 = (4.0 * p.p85);let t20: f64 = (t1f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t20, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {
            let (t22, t23, t24,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t21: f64 = (-l.f6f7);
        (t21, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t22, t23, t24, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t25: f64 = (l.f6f3 * l.f6f3);let t26: f64 = (t25 + l.f6f7);let t27: f64 = (t26).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t27, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t27)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t27)), );let t28: f64 = (l.f6f3 + l.f6f7);let t29: f64 = (0.5 * t28);let t2a: f64 = (p.p85 - t29);(l.f605, l.f606, l.f607, ) = (t2a, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2b: f64 = (l.f605 - l.f5e5);let t2c: f64 = (t2b - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2c, l.f606, l.f607, );let t2d: f64 = (4.0 * l.f5e5);let t2e: f64 = (t2d * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2e, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {
            let (t30, t31, t32,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2f: f64 = (-l.f6f7);
        (t2f, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t30, t31, t32, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t33: f64 = (l.f6f3 * l.f6f3);let t34: f64 = (t33 + l.f6f7);let t35: f64 = (t34).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t35, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t35)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t35)), );let t36: f64 = (l.f6f3 + l.f6f7);let t37: f64 = (0.5 * t36);let t38: f64 = (l.f5e5 + t37);(l.f5f1, l.f5f2, l.f5f3, ) = (t38, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t39: f64 = (p.p85 - l.f5ed);let t3a: f64 = (t39 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3a, (-l.f5ee), (-l.f5ef), );let t3b: f64 = (4.0 * p.p85);let t3c: f64 = (t3b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3c, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {
            let (t3e, t3f, t40,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3d: f64 = (-l.f6f7);
        (t3d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3e, t3f, t40, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t41: f64 = (l.f6f3 * l.f6f3);let t42: f64 = (t41 + l.f6f7);let t43: f64 = (t42).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t43, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t43)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t43)), );let t44: f64 = (l.f6f3 + l.f6f7);let t45: f64 = (0.5 * t44);let t46: f64 = (p.p85 - t45);(l.f5ed, l.f5ee, l.f5ef, ) = (t46, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t47: f64 = (l.f5ed - l.f5e5);let t48: f64 = (t47 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t48, l.f5ee, l.f5ef, );let t49: f64 = (4.0 * l.f5e5);let t4a: f64 = (t49 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t4a, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {
            let (t4c, t4d, t4e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t4b: f64 = (-l.f6f7);
        (t4b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t4c, t4d, t4e, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_145(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t4f: f64 = (l.f6f3 * l.f6f3);let t50: f64 = (t4f + l.f6f7);let t51: f64 = (t50).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t51, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t51)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t51)), );let t52: f64 = (l.f6f3 + l.f6f7);let t53: f64 = (0.5 * t52);let t54: f64 = (l.f5e5 + t53);(l.f5ed, l.f5ee, l.f5ef, ) = (t54, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );}
        let t55: f64 = (l.f745 / l.f5f1);let t56: f64 = (l.f5f1 - l.f5ed);let t57: f64 = (l.f793 * t56);let t58: f64 = (l.f5ed * p.p85);let t59: f64 = (t57 / t58);let t5a: f64 = (t55 + t59);let t5b: f64 = (l.f645 * t5a);let t5c: f64 = (t5b).abs();let t5d: f64 = if t5c < 230.25850929940458 { 1.0 } else { 0.0 };l.f3fa = t5d;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fa != 0.0)) {let t5e: f64 = (l.f745 / l.f5f1);let t5f: f64 = (l.f5f1 - l.f5ed);let t60: f64 = (l.f793 * t5f);let t61: f64 = (l.f5ed * p.p85);let t62: f64 = (t60 / t61);let t63: f64 = (t5e + t62);let t64: f64 = (l.f645 * t63);let t65: f64 = (t64).exp();(l.f536, l.f537, l.f538, ) = (t65, (t65 * (l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t61) - (t60 * (l.f5ee * p.p85))) / (t61 * t61))))), (t65 * (l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t61) - (t60 * (l.f5ef * p.p85))) / (t61 * t61))))), );}
        let t66: f64 = (l.f745 / l.f5f1);let t67: f64 = (l.f5f1 - l.f5ed);let t68: f64 = (l.f793 * t67);let t69: f64 = (l.f5ed * p.p85);let t6a: f64 = (t68 / t69);let t6b: f64 = (t66 + t6a);let t6c: f64 = (l.f645 * t6b);let t6d: f64 = (-230.25850929940458);let t6e: f64 = if t6c < t6d { 1.0 } else { 0.0 };l.f3fc = t6e;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fa == 0.0)) && (l.f3fc != 0.0)) {let t6f: f64 = (-230.25850929940458);let t70: f64 = (l.f745 / l.f5f1);let t71: f64 = (l.f5f1 - l.f5ed);let t72: f64 = (l.f793 * t71);let t73: f64 = (l.f5ed * p.p85);let t74: f64 = (t72 / t73);let t75: f64 = (t70 + t74);let t76: f64 = (l.f645 * t75);let t77: f64 = (t6f - t76);let t78: f64 = (-230.25850929940458);let t79: f64 = (l.f745 / l.f5f1);let t7a: f64 = (l.f5f1 - l.f5ed);let t7b: f64 = (l.f793 * t7a);let t7c: f64 = (l.f5ed * p.p85);let t7d: f64 = (t7b / t7c);let t7e: f64 = (t79 + t7d);let t7f: f64 = (l.f645 * t7e);let t80: f64 = (t78 - t7f);let t81: f64 = (-230.25850929940458);let t82: f64 = (l.f745 / l.f5f1);let t83: f64 = (l.f5f1 - l.f5ed);let t84: f64 = (l.f793 * t83);let t85: f64 = (l.f5ed * p.p85);let t86: f64 = (t84 / t85);let t87: f64 = (t82 + t86);let t88: f64 = (l.f645 * t87);let t89: f64 = (t81 - t88);let t8a: f64 = (t89 * 0.3333333333333333);let t8b: f64 = (1.0 + t8a);let t8c: f64 = (t80 * t8b);let t8d: f64 = (0.5 * t8c);let t8e: f64 = (1.0 + t8d);let t8f: f64 = (t77 * t8e);let t90: f64 = (1.0 + t8f);let t91: f64 = (1e-100 / t90);(l.f536, l.f537, l.f538, ) = (t91, (-((1e-100 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t73) - (t72 * (l.f5ee * p.p85))) / (t73 * t73))))) * t8e) + (t77 * (0.5 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t7c) - (t7b * (l.f5ee * p.p85))) / (t7c * t7c))))) * t8b) + (t80 * ((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t85) - (t84 * (l.f5ee * p.p85))) / (t85 * t85))))) * 0.3333333333333333))))))) / (t90 * t90))), (-((1e-100 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t73) - (t72 * (l.f5ef * p.p85))) / (t73 * t73))))) * t8e) + (t77 * (0.5 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t7c) - (t7b * (l.f5ef * p.p85))) / (t7c * t7c))))) * t8b) + (t80 * ((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t85) - (t84 * (l.f5ef * p.p85))) / (t85 * t85))))) * 0.3333333333333333))))))) / (t90 * t90))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_146(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fa == 0.0)) && (l.f3fc == 0.0)) {let t92: f64 = (l.f745 / l.f5f1);let t93: f64 = (l.f5f1 - l.f5ed);let t94: f64 = (l.f793 * t93);let t95: f64 = (l.f5ed * p.p85);let t96: f64 = (t94 / t95);let t97: f64 = (t92 + t96);let t98: f64 = (l.f645 * t97);let t99: f64 = (t98 - 230.25850929940458);let t9a: f64 = (l.f745 / l.f5f1);let t9b: f64 = (l.f5f1 - l.f5ed);let t9c: f64 = (l.f793 * t9b);let t9d: f64 = (l.f5ed * p.p85);let t9e: f64 = (t9c / t9d);let t9f: f64 = (t9a + t9e);let ta0: f64 = (l.f645 * t9f);let ta1: f64 = (ta0 - 230.25850929940458);let ta2: f64 = (l.f745 / l.f5f1);let ta3: f64 = (l.f5f1 - l.f5ed);let ta4: f64 = (l.f793 * ta3);let ta5: f64 = (l.f5ed * p.p85);let ta6: f64 = (ta4 / ta5);let ta7: f64 = (ta2 + ta6);let ta8: f64 = (l.f645 * ta7);let ta9: f64 = (ta8 - 230.25850929940458);let taa: f64 = (ta9 * 0.3333333333333333);let tab: f64 = (1.0 + taa);let tac: f64 = (ta1 * tab);let tad: f64 = (0.5 * tac);let tae: f64 = (1.0 + tad);let taf: f64 = (t99 * tae);let tb0: f64 = (1.0 + taf);let tb1: f64 = (1e100 * tb0);(l.f536, l.f537, l.f538, ) = (tb1, (1e100 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t95) - (t94 * (l.f5ee * p.p85))) / (t95 * t95)))) * tae) + (t99 * (0.5 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t9d) - (t9c * (l.f5ee * p.p85))) / (t9d * t9d)))) * tab) + (ta1 * ((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * ta5) - (ta4 * (l.f5ee * p.p85))) / (ta5 * ta5)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t95) - (t94 * (l.f5ef * p.p85))) / (t95 * t95)))) * tae) + (t99 * (0.5 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t9d) - (t9c * (l.f5ef * p.p85))) / (t9d * t9d)))) * tab) + (ta1 * ((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * ta5) - (ta4 * (l.f5ef * p.p85))) / (ta5 * ta5)))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) {let tb2: f64 = (l.f5eb * l.f5eb);let tb3: f64 = (tb2 / l.f5e3);l.f64f = tb3;let tb4: f64 = (l.f5e9 / l.f645);let tb5: f64 = (l.f5e3 / l.f64f);let tb6: f64 = (tb5).ln();let tb7: f64 = (tb4 * tb6);l.f793 = tb7;}
        let tb8: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f3fe = tb8;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let tb9: f64 = (l.f745 - l.f793);let tba: f64 = (p.p86 * tb9);let tbb: f64 = (tba + l.f5e9);(l.f601, l.f602, l.f603, ) = (tbb, (p.p86 * l.f746), (p.p86 * l.f747), );let tbc: f64 = (p.p86 * l.f793);let tbd: f64 = (l.f5e9 - tbc);(l.f5ed, l.f5ee, l.f5ef, ) = (tbd, 0.0, 0.0, );let tbe: f64 = (p.p85 - l.f601);let tbf: f64 = (tbe - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tbf, (-l.f602), (-l.f603), );let tc0: f64 = (4.0 * p.p85);let tc1: f64 = (tc0 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tc1, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {
            let (tc3, tc4, tc5,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tc2: f64 = (-l.f6f7);
        (tc2, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tc3, tc4, tc5, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let tc6: f64 = (l.f6f3 * l.f6f3);let tc7: f64 = (tc6 + l.f6f7);let tc8: f64 = (tc7).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tc8, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tc8)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tc8)), );let tc9: f64 = (l.f6f3 + l.f6f7);let tca: f64 = (0.5 * tc9);let tcb: f64 = (p.p85 - tca);(l.f605, l.f606, l.f607, ) = (tcb, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let tcc: f64 = (l.f605 - l.f5e9);let tcd: f64 = (tcc - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tcd, l.f606, l.f607, );let tce: f64 = (4.0 * l.f5e9);let tcf: f64 = (tce * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tcf, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {
            let (td1, td2, td3,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let td0: f64 = (-l.f6f7);
        (td0, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (td1, td2, td3, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let td4: f64 = (l.f6f3 * l.f6f3);let td5: f64 = (td4 + l.f6f7);let td6: f64 = (td5).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (td6, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * td6)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * td6)), );let td7: f64 = (l.f6f3 + l.f6f7);let td8: f64 = (0.5 * td7);let td9: f64 = (l.f5e9 + td8);(l.f5f1, l.f5f2, l.f5f3, ) = (td9, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let tda: f64 = (p.p85 - l.f5ed);let tdb: f64 = (tda - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tdb, (-l.f5ee), (-l.f5ef), );let tdc: f64 = (4.0 * p.p85);let tdd: f64 = (tdc * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tdd, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {
            let (tdf, te0, te1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tde: f64 = (-l.f6f7);
        (tde, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tdf, te0, te1, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_147(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let te2: f64 = (l.f6f3 * l.f6f3);let te3: f64 = (te2 + l.f6f7);let te4: f64 = (te3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (te4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * te4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * te4)), );let te5: f64 = (l.f6f3 + l.f6f7);let te6: f64 = (0.5 * te5);let te7: f64 = (p.p85 - te6);(l.f5ed, l.f5ee, l.f5ef, ) = (te7, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let te8: f64 = (l.f5ed - l.f5e9);let te9: f64 = (te8 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (te9, l.f5ee, l.f5ef, );let tea: f64 = (4.0 * l.f5e9);let teb: f64 = (tea * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (teb, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {
            let (ted, tee, tef,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tec: f64 = (-l.f6f7);
        (tec, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (ted, tee, tef, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let tf0: f64 = (l.f6f3 * l.f6f3);let tf1: f64 = (tf0 + l.f6f7);let tf2: f64 = (tf1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tf2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tf2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tf2)), );let tf3: f64 = (l.f6f3 + l.f6f7);let tf4: f64 = (0.5 * tf3);let tf5: f64 = (l.f5e9 + tf4);(l.f5ed, l.f5ee, l.f5ef, ) = (tf5, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );}
        let tf6: f64 = (l.f745 / l.f5f1);let tf7: f64 = (l.f5f1 - l.f5ed);let tf8: f64 = (l.f793 * tf7);let tf9: f64 = (l.f5ed * p.p85);let tfa: f64 = (tf8 / tf9);let tfb: f64 = (tf6 + tfa);let tfc: f64 = (l.f645 * tfb);let tfd: f64 = (tfc).abs();let tfe: f64 = if tfd < 230.25850929940458 { 1.0 } else { 0.0 };l.f400 = tfe;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f400 != 0.0)) {let tff: f64 = (l.f745 / l.f5f1);let t100: f64 = (l.f5f1 - l.f5ed);let t101: f64 = (l.f793 * t100);let t102: f64 = (l.f5ed * p.p85);let t103: f64 = (t101 / t102);let t104: f64 = (tff + t103);let t105: f64 = (l.f645 * t104);let t106: f64 = (t105).exp();(l.f53e, l.f53f, l.f540, ) = (t106, (t106 * (l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t102) - (t101 * (l.f5ee * p.p85))) / (t102 * t102))))), (t106 * (l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t102) - (t101 * (l.f5ef * p.p85))) / (t102 * t102))))), );}
        let t107: f64 = (l.f745 / l.f5f1);let t108: f64 = (l.f5f1 - l.f5ed);let t109: f64 = (l.f793 * t108);let t10a: f64 = (l.f5ed * p.p85);let t10b: f64 = (t109 / t10a);let t10c: f64 = (t107 + t10b);let t10d: f64 = (l.f645 * t10c);let t10e: f64 = (-230.25850929940458);let t10f: f64 = if t10d < t10e { 1.0 } else { 0.0 };l.f402 = t10f;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f400 == 0.0)) && (l.f402 != 0.0)) {let t110: f64 = (-230.25850929940458);let t111: f64 = (l.f745 / l.f5f1);let t112: f64 = (l.f5f1 - l.f5ed);let t113: f64 = (l.f793 * t112);let t114: f64 = (l.f5ed * p.p85);let t115: f64 = (t113 / t114);let t116: f64 = (t111 + t115);let t117: f64 = (l.f645 * t116);let t118: f64 = (t110 - t117);let t119: f64 = (-230.25850929940458);let t11a: f64 = (l.f745 / l.f5f1);let t11b: f64 = (l.f5f1 - l.f5ed);let t11c: f64 = (l.f793 * t11b);let t11d: f64 = (l.f5ed * p.p85);let t11e: f64 = (t11c / t11d);let t11f: f64 = (t11a + t11e);let t120: f64 = (l.f645 * t11f);let t121: f64 = (t119 - t120);let t122: f64 = (-230.25850929940458);let t123: f64 = (l.f745 / l.f5f1);let t124: f64 = (l.f5f1 - l.f5ed);let t125: f64 = (l.f793 * t124);let t126: f64 = (l.f5ed * p.p85);let t127: f64 = (t125 / t126);let t128: f64 = (t123 + t127);let t129: f64 = (l.f645 * t128);let t12a: f64 = (t122 - t129);let t12b: f64 = (t12a * 0.3333333333333333);let t12c: f64 = (1.0 + t12b);let t12d: f64 = (t121 * t12c);let t12e: f64 = (0.5 * t12d);let t12f: f64 = (1.0 + t12e);let t130: f64 = (t118 * t12f);let t131: f64 = (1.0 + t130);let t132: f64 = (1e-100 / t131);(l.f53e, l.f53f, l.f540, ) = (t132, (-((1e-100 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t114) - (t113 * (l.f5ee * p.p85))) / (t114 * t114))))) * t12f) + (t118 * (0.5 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t11d) - (t11c * (l.f5ee * p.p85))) / (t11d * t11d))))) * t12c) + (t121 * ((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t126) - (t125 * (l.f5ee * p.p85))) / (t126 * t126))))) * 0.3333333333333333))))))) / (t131 * t131))), (-((1e-100 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t114) - (t113 * (l.f5ef * p.p85))) / (t114 * t114))))) * t12f) + (t118 * (0.5 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t11d) - (t11c * (l.f5ef * p.p85))) / (t11d * t11d))))) * t12c) + (t121 * ((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t126) - (t125 * (l.f5ef * p.p85))) / (t126 * t126))))) * 0.3333333333333333))))))) / (t131 * t131))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_148(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f400 == 0.0)) && (l.f402 == 0.0)) {let t133: f64 = (l.f745 / l.f5f1);let t134: f64 = (l.f5f1 - l.f5ed);let t135: f64 = (l.f793 * t134);let t136: f64 = (l.f5ed * p.p85);let t137: f64 = (t135 / t136);let t138: f64 = (t133 + t137);let t139: f64 = (l.f645 * t138);let t13a: f64 = (t139 - 230.25850929940458);let t13b: f64 = (l.f745 / l.f5f1);let t13c: f64 = (l.f5f1 - l.f5ed);let t13d: f64 = (l.f793 * t13c);let t13e: f64 = (l.f5ed * p.p85);let t13f: f64 = (t13d / t13e);let t140: f64 = (t13b + t13f);let t141: f64 = (l.f645 * t140);let t142: f64 = (t141 - 230.25850929940458);let t143: f64 = (l.f745 / l.f5f1);let t144: f64 = (l.f5f1 - l.f5ed);let t145: f64 = (l.f793 * t144);let t146: f64 = (l.f5ed * p.p85);let t147: f64 = (t145 / t146);let t148: f64 = (t143 + t147);let t149: f64 = (l.f645 * t148);let t14a: f64 = (t149 - 230.25850929940458);let t14b: f64 = (t14a * 0.3333333333333333);let t14c: f64 = (1.0 + t14b);let t14d: f64 = (t142 * t14c);let t14e: f64 = (0.5 * t14d);let t14f: f64 = (1.0 + t14e);let t150: f64 = (t13a * t14f);let t151: f64 = (1.0 + t150);let t152: f64 = (1e100 * t151);(l.f53e, l.f53f, l.f540, ) = (t152, (1e100 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t136) - (t135 * (l.f5ee * p.p85))) / (t136 * t136)))) * t14f) + (t13a * (0.5 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t13e) - (t13d * (l.f5ee * p.p85))) / (t13e * t13e)))) * t14c) + (t142 * ((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t146) - (t145 * (l.f5ee * p.p85))) / (t146 * t146)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t136) - (t135 * (l.f5ef * p.p85))) / (t136 * t136)))) * t14f) + (t13a * (0.5 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t13e) - (t13d * (l.f5ef * p.p85))) / (t13e * t13e)))) * t14c) + (t142 * ((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t146) - (t145 * (l.f5ef * p.p85))) / (t146 * t146)))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) {let t153: f64 = (l.f5eb * l.f5eb);let t154: f64 = (t153 / l.f5e1);l.f64f = t154;let t155: f64 = (l.f5e7 / l.f645);let t156: f64 = (l.f5e1 / l.f64f);let t157: f64 = (t156).ln();let t158: f64 = (t155 * t157);l.f793 = t158;}
        let t159: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f404 = t159;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t15a: f64 = (l.f745 - l.f793);let t15b: f64 = (p.p86 * t15a);let t15c: f64 = (t15b + l.f5e7);(l.f601, l.f602, l.f603, ) = (t15c, (p.p86 * l.f746), (p.p86 * l.f747), );let t15d: f64 = (p.p86 * l.f793);let t15e: f64 = (l.f5e7 - t15d);(l.f5ed, l.f5ee, l.f5ef, ) = (t15e, 0.0, 0.0, );let t15f: f64 = (p.p85 - l.f601);let t160: f64 = (t15f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t160, (-l.f602), (-l.f603), );let t161: f64 = (4.0 * p.p85);let t162: f64 = (t161 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t162, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {
            let (t164, t165, t166,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t163: f64 = (-l.f6f7);
        (t163, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t164, t165, t166, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t167: f64 = (l.f6f3 * l.f6f3);let t168: f64 = (t167 + l.f6f7);let t169: f64 = (t168).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t169, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t169)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t169)), );let t16a: f64 = (l.f6f3 + l.f6f7);let t16b: f64 = (0.5 * t16a);let t16c: f64 = (p.p85 - t16b);(l.f605, l.f606, l.f607, ) = (t16c, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t16d: f64 = (l.f605 - l.f5e7);let t16e: f64 = (t16d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t16e, l.f606, l.f607, );let t16f: f64 = (4.0 * l.f5e7);let t170: f64 = (t16f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t170, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {
            let (t172, t173, t174,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t171: f64 = (-l.f6f7);
        (t171, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t172, t173, t174, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t175: f64 = (l.f6f3 * l.f6f3);let t176: f64 = (t175 + l.f6f7);let t177: f64 = (t176).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t177, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t177)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t177)), );let t178: f64 = (l.f6f3 + l.f6f7);let t179: f64 = (0.5 * t178);let t17a: f64 = (l.f5e7 + t179);(l.f5f1, l.f5f2, l.f5f3, ) = (t17a, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t17b: f64 = (p.p85 - l.f5ed);let t17c: f64 = (t17b - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t17c, (-l.f5ee), (-l.f5ef), );let t17d: f64 = (4.0 * p.p85);let t17e: f64 = (t17d * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t17e, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {
            let (t180, t181, t182,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t17f: f64 = (-l.f6f7);
        (t17f, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t180, t181, t182, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_149(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t183: f64 = (l.f6f3 * l.f6f3);let t184: f64 = (t183 + l.f6f7);let t185: f64 = (t184).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t185, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t185)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t185)), );let t186: f64 = (l.f6f3 + l.f6f7);let t187: f64 = (0.5 * t186);let t188: f64 = (p.p85 - t187);(l.f5ed, l.f5ee, l.f5ef, ) = (t188, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t189: f64 = (l.f5ed - l.f5e7);let t18a: f64 = (t189 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t18a, l.f5ee, l.f5ef, );let t18b: f64 = (4.0 * l.f5e7);let t18c: f64 = (t18b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t18c, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {
            let (t18e, t18f, t190,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t18d: f64 = (-l.f6f7);
        (t18d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t18e, t18f, t190, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t191: f64 = (l.f6f3 * l.f6f3);let t192: f64 = (t191 + l.f6f7);let t193: f64 = (t192).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t193, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t193)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t193)), );let t194: f64 = (l.f6f3 + l.f6f7);let t195: f64 = (0.5 * t194);let t196: f64 = (l.f5e7 + t195);(l.f5ed, l.f5ee, l.f5ef, ) = (t196, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );}
        let t197: f64 = (l.f745 / l.f5f1);let t198: f64 = (l.f5f1 - l.f5ed);let t199: f64 = (l.f793 * t198);let t19a: f64 = (l.f5ed * p.p85);let t19b: f64 = (t199 / t19a);let t19c: f64 = (t197 + t19b);let t19d: f64 = (l.f645 * t19c);let t19e: f64 = (t19d).abs();let t19f: f64 = if t19e < 230.25850929940458 { 1.0 } else { 0.0 };l.f406 = t19f;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f406 != 0.0)) {let t1a0: f64 = (l.f745 / l.f5f1);let t1a1: f64 = (l.f5f1 - l.f5ed);let t1a2: f64 = (l.f793 * t1a1);let t1a3: f64 = (l.f5ed * p.p85);let t1a4: f64 = (t1a2 / t1a3);let t1a5: f64 = (t1a0 + t1a4);let t1a6: f64 = (l.f645 * t1a5);let t1a7: f64 = (t1a6).exp();(l.f53a, l.f53b, l.f53c, ) = (t1a7, (t1a7 * (l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1a3) - (t1a2 * (l.f5ee * p.p85))) / (t1a3 * t1a3))))), (t1a7 * (l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1a3) - (t1a2 * (l.f5ef * p.p85))) / (t1a3 * t1a3))))), );}
        let t1a8: f64 = (l.f745 / l.f5f1);let t1a9: f64 = (l.f5f1 - l.f5ed);let t1aa: f64 = (l.f793 * t1a9);let t1ab: f64 = (l.f5ed * p.p85);let t1ac: f64 = (t1aa / t1ab);let t1ad: f64 = (t1a8 + t1ac);let t1ae: f64 = (l.f645 * t1ad);let t1af: f64 = (-230.25850929940458);let t1b0: f64 = if t1ae < t1af { 1.0 } else { 0.0 };l.f408 = t1b0;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f406 == 0.0)) && (l.f408 != 0.0)) {let t1b1: f64 = (-230.25850929940458);let t1b2: f64 = (l.f745 / l.f5f1);let t1b3: f64 = (l.f5f1 - l.f5ed);let t1b4: f64 = (l.f793 * t1b3);let t1b5: f64 = (l.f5ed * p.p85);let t1b6: f64 = (t1b4 / t1b5);let t1b7: f64 = (t1b2 + t1b6);let t1b8: f64 = (l.f645 * t1b7);let t1b9: f64 = (t1b1 - t1b8);let t1ba: f64 = (-230.25850929940458);let t1bb: f64 = (l.f745 / l.f5f1);let t1bc: f64 = (l.f5f1 - l.f5ed);let t1bd: f64 = (l.f793 * t1bc);let t1be: f64 = (l.f5ed * p.p85);let t1bf: f64 = (t1bd / t1be);let t1c0: f64 = (t1bb + t1bf);let t1c1: f64 = (l.f645 * t1c0);let t1c2: f64 = (t1ba - t1c1);let t1c3: f64 = (-230.25850929940458);let t1c4: f64 = (l.f745 / l.f5f1);let t1c5: f64 = (l.f5f1 - l.f5ed);let t1c6: f64 = (l.f793 * t1c5);let t1c7: f64 = (l.f5ed * p.p85);let t1c8: f64 = (t1c6 / t1c7);let t1c9: f64 = (t1c4 + t1c8);let t1ca: f64 = (l.f645 * t1c9);let t1cb: f64 = (t1c3 - t1ca);let t1cc: f64 = (t1cb * 0.3333333333333333);let t1cd: f64 = (1.0 + t1cc);let t1ce: f64 = (t1c2 * t1cd);let t1cf: f64 = (0.5 * t1ce);let t1d0: f64 = (1.0 + t1cf);let t1d1: f64 = (t1b9 * t1d0);let t1d2: f64 = (1.0 + t1d1);let t1d3: f64 = (1e-100 / t1d2);(l.f53a, l.f53b, l.f53c, ) = (t1d3, (-((1e-100 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1b5) - (t1b4 * (l.f5ee * p.p85))) / (t1b5 * t1b5))))) * t1d0) + (t1b9 * (0.5 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1be) - (t1bd * (l.f5ee * p.p85))) / (t1be * t1be))))) * t1cd) + (t1c2 * ((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1c7) - (t1c6 * (l.f5ee * p.p85))) / (t1c7 * t1c7))))) * 0.3333333333333333))))))) / (t1d2 * t1d2))), (-((1e-100 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1b5) - (t1b4 * (l.f5ef * p.p85))) / (t1b5 * t1b5))))) * t1d0) + (t1b9 * (0.5 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1be) - (t1bd * (l.f5ef * p.p85))) / (t1be * t1be))))) * t1cd) + (t1c2 * ((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1c7) - (t1c6 * (l.f5ef * p.p85))) / (t1c7 * t1c7))))) * 0.3333333333333333))))))) / (t1d2 * t1d2))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_150(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f406 == 0.0)) && (l.f408 == 0.0)) {let t1d4: f64 = (l.f745 / l.f5f1);let t1d5: f64 = (l.f5f1 - l.f5ed);let t1d6: f64 = (l.f793 * t1d5);let t1d7: f64 = (l.f5ed * p.p85);let t1d8: f64 = (t1d6 / t1d7);let t1d9: f64 = (t1d4 + t1d8);let t1da: f64 = (l.f645 * t1d9);let t1db: f64 = (t1da - 230.25850929940458);let t1dc: f64 = (l.f745 / l.f5f1);let t1dd: f64 = (l.f5f1 - l.f5ed);let t1de: f64 = (l.f793 * t1dd);let t1df: f64 = (l.f5ed * p.p85);let t1e0: f64 = (t1de / t1df);let t1e1: f64 = (t1dc + t1e0);let t1e2: f64 = (l.f645 * t1e1);let t1e3: f64 = (t1e2 - 230.25850929940458);let t1e4: f64 = (l.f745 / l.f5f1);let t1e5: f64 = (l.f5f1 - l.f5ed);let t1e6: f64 = (l.f793 * t1e5);let t1e7: f64 = (l.f5ed * p.p85);let t1e8: f64 = (t1e6 / t1e7);let t1e9: f64 = (t1e4 + t1e8);let t1ea: f64 = (l.f645 * t1e9);let t1eb: f64 = (t1ea - 230.25850929940458);let t1ec: f64 = (t1eb * 0.3333333333333333);let t1ed: f64 = (1.0 + t1ec);let t1ee: f64 = (t1e3 * t1ed);let t1ef: f64 = (0.5 * t1ee);let t1f0: f64 = (1.0 + t1ef);let t1f1: f64 = (t1db * t1f0);let t1f2: f64 = (1.0 + t1f1);let t1f3: f64 = (1e100 * t1f2);(l.f53a, l.f53b, l.f53c, ) = (t1f3, (1e100 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1d7) - (t1d6 * (l.f5ee * p.p85))) / (t1d7 * t1d7)))) * t1f0) + (t1db * (0.5 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1df) - (t1de * (l.f5ee * p.p85))) / (t1df * t1df)))) * t1ed) + (t1e3 * ((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e7) - (t1e6 * (l.f5ee * p.p85))) / (t1e7 * t1e7)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1d7) - (t1d6 * (l.f5ef * p.p85))) / (t1d7 * t1d7)))) * t1f0) + (t1db * (0.5 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1df) - (t1de * (l.f5ef * p.p85))) / (t1df * t1df)))) * t1ed) + (t1e3 * ((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e7) - (t1e6 * (l.f5ef * p.p85))) / (t1e7 * t1e7)))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) {let t1f4: f64 = (l.f745 - l.f7b1);let t1f5: f64 = (t1f4 * l.f645);let t1f6: f64 = (1.0 + t1f5);let t1f7: f64 = (t1f6 * l.f89);let t1f8: f64 = (t1f7).sqrt();(l.f824, l.f827, l.f828, ) = (t1f8, (((l.f746 * l.f645) * l.f89) / (2.0 * t1f8)), (((l.f747 * l.f645) * l.f89) / (2.0 * t1f8)), );let t1f9: f64 = (l.f5eb * l.f5eb);let t1fa: f64 = (t1f9 / l.f5df);l.f64f = t1fa;let t1fb: f64 = (l.f5e5 / l.f645);let t1fc: f64 = (l.f5df / l.f64f);let t1fd: f64 = (t1fc).ln();let t1fe: f64 = (t1fb * t1fd);l.f793 = t1fe;}
        let t1ff: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f40a = t1ff;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t200: f64 = (l.f7b1 - l.f793);let t201: f64 = (p.p86 * t200);let t202: f64 = (t201 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t202, 0.0, 0.0, );let t203: f64 = (p.p86 * l.f793);let t204: f64 = (l.f5e5 - t203);(l.f5ed, l.f5ee, l.f5ef, ) = (t204, 0.0, 0.0, );let t205: f64 = (p.p85 - l.f601);let t206: f64 = (t205 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t206, (-l.f602), (-l.f603), );let t207: f64 = (4.0 * p.p85);let t208: f64 = (t207 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t208, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {
            let (t20a, t20b, t20c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t209: f64 = (-l.f6f7);
        (t209, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t20a, t20b, t20c, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t20d: f64 = (l.f6f3 * l.f6f3);let t20e: f64 = (t20d + l.f6f7);let t20f: f64 = (t20e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t20f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t20f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t20f)), );let t210: f64 = (l.f6f3 / l.f6f7);let t211: f64 = (1.0 + t210);let t212: f64 = (0.5 * t211);(l.f55, l.f56, l.f57, ) = (t212, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t213: f64 = (l.f6f3 + l.f6f7);let t214: f64 = (0.5 * t213);let t215: f64 = (p.p85 - t214);(l.f605, l.f606, l.f607, ) = (t215, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t216: f64 = (l.f605 - l.f5e5);let t217: f64 = (t216 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t217, l.f606, l.f607, );let t218: f64 = (4.0 * l.f5e5);let t219: f64 = (t218 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t219, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {
            let (t21b, t21c, t21d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t21a: f64 = (-l.f6f7);
        (t21a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t21b, t21c, t21d, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t21e: f64 = (l.f6f3 * l.f6f3);let t21f: f64 = (t21e + l.f6f7);let t220: f64 = (t21f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t220, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t220)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t220)), );let t221: f64 = (l.f6f3 / l.f6f7);let t222: f64 = (1.0 + t221);let t223: f64 = (0.5 * t222);(l.f51, l.f52, l.f53, ) = (t223, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_151(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t224: f64 = (l.f6f3 + l.f6f7);let t225: f64 = (0.5 * t224);let t226: f64 = (l.f5e5 + t225);(l.f5f1, l.f5f2, l.f5f3, ) = (t226, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t227: f64 = (p.p85 - l.f5ed);let t228: f64 = (t227 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t228, (-l.f5ee), (-l.f5ef), );let t229: f64 = (4.0 * p.p85);let t22a: f64 = (t229 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t22a, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {
            let (t22c, t22d, t22e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t22b: f64 = (-l.f6f7);
        (t22b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t22c, t22d, t22e, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t22f: f64 = (l.f6f3 * l.f6f3);let t230: f64 = (t22f + l.f6f7);let t231: f64 = (t230).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t231, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t231)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t231)), );let t232: f64 = (l.f6f3 + l.f6f7);let t233: f64 = (0.5 * t232);let t234: f64 = (p.p85 - t233);(l.f5ed, l.f5ee, l.f5ef, ) = (t234, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t235: f64 = (l.f5ed - l.f5e5);let t236: f64 = (t235 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t236, l.f5ee, l.f5ef, );let t237: f64 = (4.0 * l.f5e5);let t238: f64 = (t237 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t238, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {
            let (t23a, t23b, t23c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t239: f64 = (-l.f6f7);
        (t239, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t23a, t23b, t23c, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t23d: f64 = (l.f6f3 * l.f6f3);let t23e: f64 = (t23d + l.f6f7);let t23f: f64 = (t23e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t23f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t23f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t23f)), );let t240: f64 = (l.f6f3 + l.f6f7);let t241: f64 = (0.5 * t240);let t242: f64 = (l.f5e5 + t241);(l.f5ed, l.f5ee, l.f5ef, ) = (t242, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t243: f64 = (p.p86 * l.f55);let t244: f64 = (t243 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t244, (((p.p86 * l.f56) * l.f51) + (t243 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t243 * l.f53)), );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t245: f64 = (l.f7b1 / l.f5f1);let t246: f64 = (l.f5f1 - l.f5ed);let t247: f64 = (l.f793 * t246);let t248: f64 = (l.f5ed * p.p85);let t249: f64 = (t247 / t248);let t24a: f64 = (t245 + t249);let t24b: f64 = (l.f645 * t24a);let t24c: f64 = (t24b).abs();let t24d: f64 = if t24c < 230.25850929940458 { 1.0 } else { 0.0 };l.f40c = t24d;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40c != 0.0)) {let t24e: f64 = (l.f7b1 / l.f5f1);let t24f: f64 = (l.f5f1 - l.f5ed);let t250: f64 = (l.f793 * t24f);let t251: f64 = (l.f5ed * p.p85);let t252: f64 = (t250 / t251);let t253: f64 = (t24e + t252);let t254: f64 = (l.f645 * t253);let t255: f64 = (t254).exp();(l.f8a, l.f8b, l.f8c, ) = (t255, (t255 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t251) - (t250 * (l.f5ee * p.p85))) / (t251 * t251))))), (t255 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t251) - (t250 * (l.f5ef * p.p85))) / (t251 * t251))))), );}
        let t256: f64 = (l.f7b1 / l.f5f1);let t257: f64 = (l.f5f1 - l.f5ed);let t258: f64 = (l.f793 * t257);let t259: f64 = (l.f5ed * p.p85);let t25a: f64 = (t258 / t259);let t25b: f64 = (t256 + t25a);let t25c: f64 = (l.f645 * t25b);let t25d: f64 = (-230.25850929940458);let t25e: f64 = if t25c < t25d { 1.0 } else { 0.0 };l.f40e = t25e;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_152(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40c == 0.0)) && (l.f40e != 0.0)) {let t25f: f64 = (-230.25850929940458);let t260: f64 = (l.f7b1 / l.f5f1);let t261: f64 = (l.f5f1 - l.f5ed);let t262: f64 = (l.f793 * t261);let t263: f64 = (l.f5ed * p.p85);let t264: f64 = (t262 / t263);let t265: f64 = (t260 + t264);let t266: f64 = (l.f645 * t265);let t267: f64 = (t25f - t266);let t268: f64 = (-230.25850929940458);let t269: f64 = (l.f7b1 / l.f5f1);let t26a: f64 = (l.f5f1 - l.f5ed);let t26b: f64 = (l.f793 * t26a);let t26c: f64 = (l.f5ed * p.p85);let t26d: f64 = (t26b / t26c);let t26e: f64 = (t269 + t26d);let t26f: f64 = (l.f645 * t26e);let t270: f64 = (t268 - t26f);let t271: f64 = (-230.25850929940458);let t272: f64 = (l.f7b1 / l.f5f1);let t273: f64 = (l.f5f1 - l.f5ed);let t274: f64 = (l.f793 * t273);let t275: f64 = (l.f5ed * p.p85);let t276: f64 = (t274 / t275);let t277: f64 = (t272 + t276);let t278: f64 = (l.f645 * t277);let t279: f64 = (t271 - t278);let t27a: f64 = (t279 * 0.3333333333333333);let t27b: f64 = (1.0 + t27a);let t27c: f64 = (t270 * t27b);let t27d: f64 = (0.5 * t27c);let t27e: f64 = (1.0 + t27d);let t27f: f64 = (t267 * t27e);let t280: f64 = (1.0 + t27f);let t281: f64 = (1e-100 / t280);(l.f8a, l.f8b, l.f8c, ) = (t281, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t263) - (t262 * (l.f5ee * p.p85))) / (t263 * t263))))) * t27e) + (t267 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t26c) - (t26b * (l.f5ee * p.p85))) / (t26c * t26c))))) * t27b) + (t270 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t275) - (t274 * (l.f5ee * p.p85))) / (t275 * t275))))) * 0.3333333333333333))))))) / (t280 * t280))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t263) - (t262 * (l.f5ef * p.p85))) / (t263 * t263))))) * t27e) + (t267 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t26c) - (t26b * (l.f5ef * p.p85))) / (t26c * t26c))))) * t27b) + (t270 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t275) - (t274 * (l.f5ef * p.p85))) / (t275 * t275))))) * 0.3333333333333333))))))) / (t280 * t280))), );}
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40c == 0.0)) && (l.f40e == 0.0)) {let t282: f64 = (l.f7b1 / l.f5f1);let t283: f64 = (l.f5f1 - l.f5ed);let t284: f64 = (l.f793 * t283);let t285: f64 = (l.f5ed * p.p85);let t286: f64 = (t284 / t285);let t287: f64 = (t282 + t286);let t288: f64 = (l.f645 * t287);let t289: f64 = (t288 - 230.25850929940458);let t28a: f64 = (l.f7b1 / l.f5f1);let t28b: f64 = (l.f5f1 - l.f5ed);let t28c: f64 = (l.f793 * t28b);let t28d: f64 = (l.f5ed * p.p85);let t28e: f64 = (t28c / t28d);let t28f: f64 = (t28a + t28e);let t290: f64 = (l.f645 * t28f);let t291: f64 = (t290 - 230.25850929940458);let t292: f64 = (l.f7b1 / l.f5f1);let t293: f64 = (l.f5f1 - l.f5ed);let t294: f64 = (l.f793 * t293);let t295: f64 = (l.f5ed * p.p85);let t296: f64 = (t294 / t295);let t297: f64 = (t292 + t296);let t298: f64 = (l.f645 * t297);let t299: f64 = (t298 - 230.25850929940458);let t29a: f64 = (t299 * 0.3333333333333333);let t29b: f64 = (1.0 + t29a);let t29c: f64 = (t291 * t29b);let t29d: f64 = (0.5 * t29c);let t29e: f64 = (1.0 + t29d);let t29f: f64 = (t289 * t29e);let t2a0: f64 = (1.0 + t29f);let t2a1: f64 = (1e100 * t2a0);(l.f8a, l.f8b, l.f8c, ) = (t2a1, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t285) - (t284 * (l.f5ee * p.p85))) / (t285 * t285)))) * t29e) + (t289 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t28d) - (t28c * (l.f5ee * p.p85))) / (t28d * t28d)))) * t29b) + (t291 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t295) - (t294 * (l.f5ee * p.p85))) / (t295 * t295)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t285) - (t284 * (l.f5ef * p.p85))) / (t285 * t285)))) * t29e) + (t289 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t28d) - (t28c * (l.f5ef * p.p85))) / (t28d * t28d)))) * t29b) + (t291 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t295) - (t294 * (l.f5ef * p.p85))) / (t295 * t295)))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) {let t2a2: f64 = (l.f7b1 * l.f5b);let t2a3: f64 = (l.f5f1 - t2a2);let t2a4: f64 = (l.f5f1 * l.f5f1);let t2a5: f64 = (t2a3 / t2a4);let t2a6: f64 = (l.f793 * l.f5b);let t2a7: f64 = (l.f5ed * p.p85);let t2a8: f64 = (t2a6 / t2a7);let t2a9: f64 = (t2a5 + t2a8);let t2aa: f64 = (l.f645 * t2a9);(l.f61, l.f62, l.f63, ) = (t2aa, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t2a4) - (t2a3 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t2a4 * t2a4)) + ((((l.f793 * l.f5c) * t2a7) - (t2a6 * (l.f5ee * p.p85))) / (t2a7 * t2a7)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t2a4) - (t2a3 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t2a4 * t2a4)) + ((((l.f793 * l.f5d) * t2a7) - (t2a6 * (l.f5ef * p.p85))) / (t2a7 * t2a7)))), );let t2ab: f64 = (l.f745 - l.f7b1);let t2ac: f64 = (t2ab * l.f61);let t2ad: f64 = (1.0 + t2ac);let t2ae: f64 = (t2ad * l.f8a);(l.f536, l.f537, l.f538, ) = (t2ae, ((((l.f746 * l.f61) + (t2ab * l.f62)) * l.f8a) + (t2ad * l.f8b)), ((((l.f747 * l.f61) + (t2ab * l.f63)) * l.f8a) + (t2ad * l.f8c)), );let t2af: f64 = (l.f5eb * l.f5eb);let t2b0: f64 = (t2af / l.f5e3);l.f64f = t2b0;let t2b1: f64 = (l.f5e9 / l.f645);let t2b2: f64 = (l.f5e3 / l.f64f);let t2b3: f64 = (t2b2).ln();let t2b4: f64 = (t2b1 * t2b3);l.f793 = t2b4;}
        let t2b5: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f410 = t2b5;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_153(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t2b6: f64 = (l.f7b1 - l.f793);let t2b7: f64 = (p.p86 * t2b6);let t2b8: f64 = (t2b7 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t2b8, 0.0, 0.0, );let t2b9: f64 = (p.p86 * l.f793);let t2ba: f64 = (l.f5e9 - t2b9);(l.f5ed, l.f5ee, l.f5ef, ) = (t2ba, 0.0, 0.0, );let t2bb: f64 = (p.p85 - l.f601);let t2bc: f64 = (t2bb - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2bc, (-l.f602), (-l.f603), );let t2bd: f64 = (4.0 * p.p85);let t2be: f64 = (t2bd * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2be, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {
            let (t2c0, t2c1, t2c2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2bf: f64 = (-l.f6f7);
        (t2bf, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2c0, t2c1, t2c2, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t2c3: f64 = (l.f6f3 * l.f6f3);let t2c4: f64 = (t2c3 + l.f6f7);let t2c5: f64 = (t2c4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2c5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2c5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2c5)), );let t2c6: f64 = (l.f6f3 / l.f6f7);let t2c7: f64 = (1.0 + t2c6);let t2c8: f64 = (0.5 * t2c7);(l.f55, l.f56, l.f57, ) = (t2c8, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t2c9: f64 = (l.f6f3 + l.f6f7);let t2ca: f64 = (0.5 * t2c9);let t2cb: f64 = (p.p85 - t2ca);(l.f605, l.f606, l.f607, ) = (t2cb, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2cc: f64 = (l.f605 - l.f5e9);let t2cd: f64 = (t2cc - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2cd, l.f606, l.f607, );let t2ce: f64 = (4.0 * l.f5e9);let t2cf: f64 = (t2ce * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2cf, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {
            let (t2d1, t2d2, t2d3,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2d0: f64 = (-l.f6f7);
        (t2d0, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2d1, t2d2, t2d3, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t2d4: f64 = (l.f6f3 * l.f6f3);let t2d5: f64 = (t2d4 + l.f6f7);let t2d6: f64 = (t2d5).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2d6, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2d6)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2d6)), );let t2d7: f64 = (l.f6f3 / l.f6f7);let t2d8: f64 = (1.0 + t2d7);let t2d9: f64 = (0.5 * t2d8);(l.f51, l.f52, l.f53, ) = (t2d9, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t2da: f64 = (l.f6f3 + l.f6f7);let t2db: f64 = (0.5 * t2da);let t2dc: f64 = (l.f5e9 + t2db);(l.f5f1, l.f5f2, l.f5f3, ) = (t2dc, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t2dd: f64 = (p.p85 - l.f5ed);let t2de: f64 = (t2dd - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2de, (-l.f5ee), (-l.f5ef), );let t2df: f64 = (4.0 * p.p85);let t2e0: f64 = (t2df * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2e0, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {
            let (t2e2, t2e3, t2e4,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2e1: f64 = (-l.f6f7);
        (t2e1, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2e2, t2e3, t2e4, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t2e5: f64 = (l.f6f3 * l.f6f3);let t2e6: f64 = (t2e5 + l.f6f7);let t2e7: f64 = (t2e6).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2e7, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2e7)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2e7)), );let t2e8: f64 = (l.f6f3 + l.f6f7);let t2e9: f64 = (0.5 * t2e8);let t2ea: f64 = (p.p85 - t2e9);(l.f5ed, l.f5ee, l.f5ef, ) = (t2ea, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2eb: f64 = (l.f5ed - l.f5e9);let t2ec: f64 = (t2eb - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2ec, l.f5ee, l.f5ef, );let t2ed: f64 = (4.0 * l.f5e9);let t2ee: f64 = (t2ed * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2ee, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {
            let (t2f0, t2f1, t2f2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2ef: f64 = (-l.f6f7);
        (t2ef, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2f0, t2f1, t2f2, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t2f3: f64 = (l.f6f3 * l.f6f3);let t2f4: f64 = (t2f3 + l.f6f7);let t2f5: f64 = (t2f4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2f5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2f5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2f5)), );let t2f6: f64 = (l.f6f3 + l.f6f7);let t2f7: f64 = (0.5 * t2f6);let t2f8: f64 = (l.f5e9 + t2f7);(l.f5ed, l.f5ee, l.f5ef, ) = (t2f8, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t2f9: f64 = (p.p86 * l.f55);let t2fa: f64 = (t2f9 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t2fa, (((p.p86 * l.f56) * l.f51) + (t2f9 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t2f9 * l.f53)), );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_154(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t2fb: f64 = (l.f7b1 / l.f5f1);let t2fc: f64 = (l.f5f1 - l.f5ed);let t2fd: f64 = (l.f793 * t2fc);let t2fe: f64 = (l.f5ed * p.p85);let t2ff: f64 = (t2fd / t2fe);let t300: f64 = (t2fb + t2ff);let t301: f64 = (l.f645 * t300);let t302: f64 = (t301).abs();let t303: f64 = if t302 < 230.25850929940458 { 1.0 } else { 0.0 };l.f412 = t303;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f412 != 0.0)) {let t304: f64 = (l.f7b1 / l.f5f1);let t305: f64 = (l.f5f1 - l.f5ed);let t306: f64 = (l.f793 * t305);let t307: f64 = (l.f5ed * p.p85);let t308: f64 = (t306 / t307);let t309: f64 = (t304 + t308);let t30a: f64 = (l.f645 * t309);let t30b: f64 = (t30a).exp();(l.f93, l.f94, l.f95, ) = (t30b, (t30b * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t307) - (t306 * (l.f5ee * p.p85))) / (t307 * t307))))), (t30b * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t307) - (t306 * (l.f5ef * p.p85))) / (t307 * t307))))), );}
        let t30c: f64 = (l.f7b1 / l.f5f1);let t30d: f64 = (l.f5f1 - l.f5ed);let t30e: f64 = (l.f793 * t30d);let t30f: f64 = (l.f5ed * p.p85);let t310: f64 = (t30e / t30f);let t311: f64 = (t30c + t310);let t312: f64 = (l.f645 * t311);let t313: f64 = (-230.25850929940458);let t314: f64 = if t312 < t313 { 1.0 } else { 0.0 };l.f414 = t314;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f412 == 0.0)) && (l.f414 != 0.0)) {let t315: f64 = (-230.25850929940458);let t316: f64 = (l.f7b1 / l.f5f1);let t317: f64 = (l.f5f1 - l.f5ed);let t318: f64 = (l.f793 * t317);let t319: f64 = (l.f5ed * p.p85);let t31a: f64 = (t318 / t319);let t31b: f64 = (t316 + t31a);let t31c: f64 = (l.f645 * t31b);let t31d: f64 = (t315 - t31c);let t31e: f64 = (-230.25850929940458);let t31f: f64 = (l.f7b1 / l.f5f1);let t320: f64 = (l.f5f1 - l.f5ed);let t321: f64 = (l.f793 * t320);let t322: f64 = (l.f5ed * p.p85);let t323: f64 = (t321 / t322);let t324: f64 = (t31f + t323);let t325: f64 = (l.f645 * t324);let t326: f64 = (t31e - t325);let t327: f64 = (-230.25850929940458);let t328: f64 = (l.f7b1 / l.f5f1);let t329: f64 = (l.f5f1 - l.f5ed);let t32a: f64 = (l.f793 * t329);let t32b: f64 = (l.f5ed * p.p85);let t32c: f64 = (t32a / t32b);let t32d: f64 = (t328 + t32c);let t32e: f64 = (l.f645 * t32d);let t32f: f64 = (t327 - t32e);let t330: f64 = (t32f * 0.3333333333333333);let t331: f64 = (1.0 + t330);let t332: f64 = (t326 * t331);let t333: f64 = (0.5 * t332);let t334: f64 = (1.0 + t333);let t335: f64 = (t31d * t334);let t336: f64 = (1.0 + t335);let t337: f64 = (1e-100 / t336);(l.f93, l.f94, l.f95, ) = (t337, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t319) - (t318 * (l.f5ee * p.p85))) / (t319 * t319))))) * t334) + (t31d * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t322) - (t321 * (l.f5ee * p.p85))) / (t322 * t322))))) * t331) + (t326 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t32b) - (t32a * (l.f5ee * p.p85))) / (t32b * t32b))))) * 0.3333333333333333))))))) / (t336 * t336))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t319) - (t318 * (l.f5ef * p.p85))) / (t319 * t319))))) * t334) + (t31d * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t322) - (t321 * (l.f5ef * p.p85))) / (t322 * t322))))) * t331) + (t326 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t32b) - (t32a * (l.f5ef * p.p85))) / (t32b * t32b))))) * 0.3333333333333333))))))) / (t336 * t336))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_155(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f412 == 0.0)) && (l.f414 == 0.0)) {let t338: f64 = (l.f7b1 / l.f5f1);let t339: f64 = (l.f5f1 - l.f5ed);let t33a: f64 = (l.f793 * t339);let t33b: f64 = (l.f5ed * p.p85);let t33c: f64 = (t33a / t33b);let t33d: f64 = (t338 + t33c);let t33e: f64 = (l.f645 * t33d);let t33f: f64 = (t33e - 230.25850929940458);let t340: f64 = (l.f7b1 / l.f5f1);let t341: f64 = (l.f5f1 - l.f5ed);let t342: f64 = (l.f793 * t341);let t343: f64 = (l.f5ed * p.p85);let t344: f64 = (t342 / t343);let t345: f64 = (t340 + t344);let t346: f64 = (l.f645 * t345);let t347: f64 = (t346 - 230.25850929940458);let t348: f64 = (l.f7b1 / l.f5f1);let t349: f64 = (l.f5f1 - l.f5ed);let t34a: f64 = (l.f793 * t349);let t34b: f64 = (l.f5ed * p.p85);let t34c: f64 = (t34a / t34b);let t34d: f64 = (t348 + t34c);let t34e: f64 = (l.f645 * t34d);let t34f: f64 = (t34e - 230.25850929940458);let t350: f64 = (t34f * 0.3333333333333333);let t351: f64 = (1.0 + t350);let t352: f64 = (t347 * t351);let t353: f64 = (0.5 * t352);let t354: f64 = (1.0 + t353);let t355: f64 = (t33f * t354);let t356: f64 = (1.0 + t355);let t357: f64 = (1e100 * t356);(l.f93, l.f94, l.f95, ) = (t357, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t33b) - (t33a * (l.f5ee * p.p85))) / (t33b * t33b)))) * t354) + (t33f * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t343) - (t342 * (l.f5ee * p.p85))) / (t343 * t343)))) * t351) + (t347 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t34b) - (t34a * (l.f5ee * p.p85))) / (t34b * t34b)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t33b) - (t33a * (l.f5ef * p.p85))) / (t33b * t33b)))) * t354) + (t33f * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t343) - (t342 * (l.f5ef * p.p85))) / (t343 * t343)))) * t351) + (t347 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t34b) - (t34a * (l.f5ef * p.p85))) / (t34b * t34b)))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) {let t358: f64 = (l.f7b1 * l.f5b);let t359: f64 = (l.f5f1 - t358);let t35a: f64 = (l.f5f1 * l.f5f1);let t35b: f64 = (t359 / t35a);let t35c: f64 = (l.f793 * l.f5b);let t35d: f64 = (l.f5ed * p.p85);let t35e: f64 = (t35c / t35d);let t35f: f64 = (t35b + t35e);let t360: f64 = (l.f645 * t35f);(l.f61, l.f62, l.f63, ) = (t360, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t35a) - (t359 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t35a * t35a)) + ((((l.f793 * l.f5c) * t35d) - (t35c * (l.f5ee * p.p85))) / (t35d * t35d)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t35a) - (t359 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t35a * t35a)) + ((((l.f793 * l.f5d) * t35d) - (t35c * (l.f5ef * p.p85))) / (t35d * t35d)))), );let t361: f64 = (l.f745 - l.f7b1);let t362: f64 = (t361 * l.f61);let t363: f64 = (1.0 + t362);let t364: f64 = (t363 * l.f93);(l.f53e, l.f53f, l.f540, ) = (t364, ((((l.f746 * l.f61) + (t361 * l.f62)) * l.f93) + (t363 * l.f94)), ((((l.f747 * l.f61) + (t361 * l.f63)) * l.f93) + (t363 * l.f95)), );let t365: f64 = (l.f5eb * l.f5eb);let t366: f64 = (t365 / l.f5e1);l.f64f = t366;let t367: f64 = (l.f5e7 / l.f645);let t368: f64 = (l.f5e1 / l.f64f);let t369: f64 = (t368).ln();let t36a: f64 = (t367 * t369);l.f793 = t36a;}
        let t36b: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f416 = t36b;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t36c: f64 = (l.f7b1 - l.f793);let t36d: f64 = (p.p86 * t36c);let t36e: f64 = (t36d + l.f5e7);(l.f601, l.f602, l.f603, ) = (t36e, 0.0, 0.0, );let t36f: f64 = (p.p86 * l.f793);let t370: f64 = (l.f5e7 - t36f);(l.f5ed, l.f5ee, l.f5ef, ) = (t370, 0.0, 0.0, );let t371: f64 = (p.p85 - l.f601);let t372: f64 = (t371 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t372, (-l.f602), (-l.f603), );let t373: f64 = (4.0 * p.p85);let t374: f64 = (t373 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t374, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {
            let (t376, t377, t378,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t375: f64 = (-l.f6f7);
        (t375, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t376, t377, t378, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t379: f64 = (l.f6f3 * l.f6f3);let t37a: f64 = (t379 + l.f6f7);let t37b: f64 = (t37a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t37b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t37b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t37b)), );let t37c: f64 = (l.f6f3 / l.f6f7);let t37d: f64 = (1.0 + t37c);let t37e: f64 = (0.5 * t37d);(l.f55, l.f56, l.f57, ) = (t37e, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t37f: f64 = (l.f6f3 + l.f6f7);let t380: f64 = (0.5 * t37f);let t381: f64 = (p.p85 - t380);(l.f605, l.f606, l.f607, ) = (t381, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t382: f64 = (l.f605 - l.f5e7);let t383: f64 = (t382 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t383, l.f606, l.f607, );let t384: f64 = (4.0 * l.f5e7);let t385: f64 = (t384 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t385, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {
            let (t387, t388, t389,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t386: f64 = (-l.f6f7);
        (t386, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t387, t388, t389, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_156(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t38a: f64 = (l.f6f3 * l.f6f3);let t38b: f64 = (t38a + l.f6f7);let t38c: f64 = (t38b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t38c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t38c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t38c)), );let t38d: f64 = (l.f6f3 / l.f6f7);let t38e: f64 = (1.0 + t38d);let t38f: f64 = (0.5 * t38e);(l.f51, l.f52, l.f53, ) = (t38f, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t390: f64 = (l.f6f3 + l.f6f7);let t391: f64 = (0.5 * t390);let t392: f64 = (l.f5e7 + t391);(l.f5f1, l.f5f2, l.f5f3, ) = (t392, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t393: f64 = (p.p85 - l.f5ed);let t394: f64 = (t393 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t394, (-l.f5ee), (-l.f5ef), );let t395: f64 = (4.0 * p.p85);let t396: f64 = (t395 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t396, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {
            let (t398, t399, t39a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t397: f64 = (-l.f6f7);
        (t397, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t398, t399, t39a, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t39b: f64 = (l.f6f3 * l.f6f3);let t39c: f64 = (t39b + l.f6f7);let t39d: f64 = (t39c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t39d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t39d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t39d)), );let t39e: f64 = (l.f6f3 + l.f6f7);let t39f: f64 = (0.5 * t39e);let t3a0: f64 = (p.p85 - t39f);(l.f5ed, l.f5ee, l.f5ef, ) = (t3a0, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t3a1: f64 = (l.f5ed - l.f5e7);let t3a2: f64 = (t3a1 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3a2, l.f5ee, l.f5ef, );let t3a3: f64 = (4.0 * l.f5e7);let t3a4: f64 = (t3a3 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3a4, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {
            let (t3a6, t3a7, t3a8,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3a5: f64 = (-l.f6f7);
        (t3a5, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3a6, t3a7, t3a8, );
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t3a9: f64 = (l.f6f3 * l.f6f3);let t3aa: f64 = (t3a9 + l.f6f7);let t3ab: f64 = (t3aa).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3ab, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3ab)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3ab)), );let t3ac: f64 = (l.f6f3 + l.f6f7);let t3ad: f64 = (0.5 * t3ac);let t3ae: f64 = (l.f5e7 + t3ad);(l.f5ed, l.f5ee, l.f5ef, ) = (t3ae, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t3af: f64 = (p.p86 * l.f55);let t3b0: f64 = (t3af * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t3b0, (((p.p86 * l.f56) * l.f51) + (t3af * l.f52)), (((p.p86 * l.f57) * l.f51) + (t3af * l.f53)), );}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t3b1: f64 = (l.f7b1 / l.f5f1);let t3b2: f64 = (l.f5f1 - l.f5ed);let t3b3: f64 = (l.f793 * t3b2);let t3b4: f64 = (l.f5ed * p.p85);let t3b5: f64 = (t3b3 / t3b4);let t3b6: f64 = (t3b1 + t3b5);let t3b7: f64 = (l.f645 * t3b6);let t3b8: f64 = (t3b7).abs();let t3b9: f64 = if t3b8 < 230.25850929940458 { 1.0 } else { 0.0 };l.f418 = t3b9;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f418 != 0.0)) {let t3ba: f64 = (l.f7b1 / l.f5f1);let t3bb: f64 = (l.f5f1 - l.f5ed);let t3bc: f64 = (l.f793 * t3bb);let t3bd: f64 = (l.f5ed * p.p85);let t3be: f64 = (t3bc / t3bd);let t3bf: f64 = (t3ba + t3be);let t3c0: f64 = (l.f645 * t3bf);let t3c1: f64 = (t3c0).exp();(l.f8e, l.f8f, l.f90, ) = (t3c1, (t3c1 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3bd) - (t3bc * (l.f5ee * p.p85))) / (t3bd * t3bd))))), (t3c1 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3bd) - (t3bc * (l.f5ef * p.p85))) / (t3bd * t3bd))))), );}
        let t3c2: f64 = (l.f7b1 / l.f5f1);let t3c3: f64 = (l.f5f1 - l.f5ed);let t3c4: f64 = (l.f793 * t3c3);let t3c5: f64 = (l.f5ed * p.p85);let t3c6: f64 = (t3c4 / t3c5);let t3c7: f64 = (t3c2 + t3c6);let t3c8: f64 = (l.f645 * t3c7);let t3c9: f64 = (-230.25850929940458);let t3ca: f64 = if t3c8 < t3c9 { 1.0 } else { 0.0 };l.f41c = t3ca;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_157(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f418 == 0.0)) && (l.f41c != 0.0)) {let t3cb: f64 = (-230.25850929940458);let t3cc: f64 = (l.f7b1 / l.f5f1);let t3cd: f64 = (l.f5f1 - l.f5ed);let t3ce: f64 = (l.f793 * t3cd);let t3cf: f64 = (l.f5ed * p.p85);let t3d0: f64 = (t3ce / t3cf);let t3d1: f64 = (t3cc + t3d0);let t3d2: f64 = (l.f645 * t3d1);let t3d3: f64 = (t3cb - t3d2);let t3d4: f64 = (-230.25850929940458);let t3d5: f64 = (l.f7b1 / l.f5f1);let t3d6: f64 = (l.f5f1 - l.f5ed);let t3d7: f64 = (l.f793 * t3d6);let t3d8: f64 = (l.f5ed * p.p85);let t3d9: f64 = (t3d7 / t3d8);let t3da: f64 = (t3d5 + t3d9);let t3db: f64 = (l.f645 * t3da);let t3dc: f64 = (t3d4 - t3db);let t3dd: f64 = (-230.25850929940458);let t3de: f64 = (l.f7b1 / l.f5f1);let t3df: f64 = (l.f5f1 - l.f5ed);let t3e0: f64 = (l.f793 * t3df);let t3e1: f64 = (l.f5ed * p.p85);let t3e2: f64 = (t3e0 / t3e1);let t3e3: f64 = (t3de + t3e2);let t3e4: f64 = (l.f645 * t3e3);let t3e5: f64 = (t3dd - t3e4);let t3e6: f64 = (t3e5 * 0.3333333333333333);let t3e7: f64 = (1.0 + t3e6);let t3e8: f64 = (t3dc * t3e7);let t3e9: f64 = (0.5 * t3e8);let t3ea: f64 = (1.0 + t3e9);let t3eb: f64 = (t3d3 * t3ea);let t3ec: f64 = (1.0 + t3eb);let t3ed: f64 = (1e-100 / t3ec);(l.f8e, l.f8f, l.f90, ) = (t3ed, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3cf) - (t3ce * (l.f5ee * p.p85))) / (t3cf * t3cf))))) * t3ea) + (t3d3 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3d8) - (t3d7 * (l.f5ee * p.p85))) / (t3d8 * t3d8))))) * t3e7) + (t3dc * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3e1) - (t3e0 * (l.f5ee * p.p85))) / (t3e1 * t3e1))))) * 0.3333333333333333))))))) / (t3ec * t3ec))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3cf) - (t3ce * (l.f5ef * p.p85))) / (t3cf * t3cf))))) * t3ea) + (t3d3 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3d8) - (t3d7 * (l.f5ef * p.p85))) / (t3d8 * t3d8))))) * t3e7) + (t3dc * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3e1) - (t3e0 * (l.f5ef * p.p85))) / (t3e1 * t3e1))))) * 0.3333333333333333))))))) / (t3ec * t3ec))), );}
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f418 == 0.0)) && (l.f41c == 0.0)) {let t3ee: f64 = (l.f7b1 / l.f5f1);let t3ef: f64 = (l.f5f1 - l.f5ed);let t3f0: f64 = (l.f793 * t3ef);let t3f1: f64 = (l.f5ed * p.p85);let t3f2: f64 = (t3f0 / t3f1);let t3f3: f64 = (t3ee + t3f2);let t3f4: f64 = (l.f645 * t3f3);let t3f5: f64 = (t3f4 - 230.25850929940458);let t3f6: f64 = (l.f7b1 / l.f5f1);let t3f7: f64 = (l.f5f1 - l.f5ed);let t3f8: f64 = (l.f793 * t3f7);let t3f9: f64 = (l.f5ed * p.p85);let t3fa: f64 = (t3f8 / t3f9);let t3fb: f64 = (t3f6 + t3fa);let t3fc: f64 = (l.f645 * t3fb);let t3fd: f64 = (t3fc - 230.25850929940458);let t3fe: f64 = (l.f7b1 / l.f5f1);let t3ff: f64 = (l.f5f1 - l.f5ed);let t400: f64 = (l.f793 * t3ff);let t401: f64 = (l.f5ed * p.p85);let t402: f64 = (t400 / t401);let t403: f64 = (t3fe + t402);let t404: f64 = (l.f645 * t403);let t405: f64 = (t404 - 230.25850929940458);let t406: f64 = (t405 * 0.3333333333333333);let t407: f64 = (1.0 + t406);let t408: f64 = (t3fd * t407);let t409: f64 = (0.5 * t408);let t40a: f64 = (1.0 + t409);let t40b: f64 = (t3f5 * t40a);let t40c: f64 = (1.0 + t40b);let t40d: f64 = (1e100 * t40c);(l.f8e, l.f8f, l.f90, ) = (t40d, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3f1) - (t3f0 * (l.f5ee * p.p85))) / (t3f1 * t3f1)))) * t40a) + (t3f5 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3f9) - (t3f8 * (l.f5ee * p.p85))) / (t3f9 * t3f9)))) * t407) + (t3fd * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t401) - (t400 * (l.f5ee * p.p85))) / (t401 * t401)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3f1) - (t3f0 * (l.f5ef * p.p85))) / (t3f1 * t3f1)))) * t40a) + (t3f5 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3f9) - (t3f8 * (l.f5ef * p.p85))) / (t3f9 * t3f9)))) * t407) + (t3fd * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t401) - (t400 * (l.f5ef * p.p85))) / (t401 * t401)))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) {let t40e: f64 = (l.f7b1 * l.f5b);let t40f: f64 = (l.f5f1 - t40e);let t410: f64 = (l.f5f1 * l.f5f1);let t411: f64 = (t40f / t410);let t412: f64 = (l.f793 * l.f5b);let t413: f64 = (l.f5ed * p.p85);let t414: f64 = (t412 / t413);let t415: f64 = (t411 + t414);let t416: f64 = (l.f645 * t415);(l.f61, l.f62, l.f63, ) = (t416, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t410) - (t40f * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t410 * t410)) + ((((l.f793 * l.f5c) * t413) - (t412 * (l.f5ee * p.p85))) / (t413 * t413)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t410) - (t40f * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t410 * t410)) + ((((l.f793 * l.f5d) * t413) - (t412 * (l.f5ef * p.p85))) / (t413 * t413)))), );let t417: f64 = (l.f745 - l.f7b1);let t418: f64 = (t417 * l.f61);let t419: f64 = (1.0 + t418);let t41a: f64 = (t419 * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t41a, ((((l.f746 * l.f61) + (t417 * l.f62)) * l.f8e) + (t419 * l.f8f)), ((((l.f747 * l.f61) + (t417 * l.f63)) * l.f8e) + (t419 * l.f90)), );}
        if ((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) {let t41b: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t41b, l.f537, l.f538, );let t41c: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t41c, l.f53f, l.f540, );let t41d: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t41d, l.f53b, l.f53c, );let t41e: f64 = (1.0 / l.f824);(l.f816, l.f819, l.f81a, ) = (t41e, (-(l.f827 / (l.f824 * l.f824))), (-(l.f828 / (l.f824 * l.f824))), );}
        let t41f: f64 = if l.f745 > 0.0 { 1.0 } else { 0.0 };l.f41e = t41f;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_158(
        l: &mut StampLocals,
    ) {
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f41e != 0.0)) {let t420: f64 = (2.0 + l.f816);let t421: f64 = (l.f816 + 1.0);let t422: f64 = (l.f816 + 3.0);let t423: f64 = (t421 * t422);let t424: f64 = (t423).sqrt();let t425: f64 = (t420 + t424);let t426: f64 = (t425).ln();let t427: f64 = (l.f643 * t426);let t428: f64 = (2.0 * t427);(l.f713, l.f716, l.f717, ) = (t428, (2.0 * (l.f643 * ((l.f819 + (((l.f819 * t422) + (t421 * l.f819)) / (2.0 * t424))) / t425))), (2.0 * (l.f643 * ((l.f81a + (((l.f81a * t422) + (t421 * l.f81a)) / (2.0 * t424))) / t425))), );}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f41e == 0.0)) {let t429: f64 = (-l.f745);let t42a: f64 = (2.0 * l.f824);let t42b: f64 = (t42a + 1.0);let t42c: f64 = (1.0 + l.f824);let t42d: f64 = (3.0 * l.f824);let t42e: f64 = (1.0 + t42d);let t42f: f64 = (t42c * t42e);let t430: f64 = (t42f).sqrt();let t431: f64 = (t42b + t430);let t432: f64 = (t431).ln();let t433: f64 = (l.f643 * t432);let t434: f64 = (2.0 * t433);let t435: f64 = (t429 + t434);(l.f713, l.f716, l.f717, ) = (t435, ((-l.f746) + (2.0 * (l.f643 * (((2.0 * l.f827) + (((l.f827 * t42e) + (t42c * (3.0 * l.f827))) / (2.0 * t430))) / t431)))), ((-l.f747) + (2.0 * (l.f643 * (((2.0 * l.f828) + (((l.f828 * t42e) + (t42c * (3.0 * l.f828))) / (2.0 * t430))) / t431)))), );}
        if ((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) {let t436: f64 = (l.f76f - l.f713);(l.f79b, l.f79e, l.f79f, ) = (t436, (-l.f716), (-l.f717), );let t437: f64 = (l.f745 + l.f79b);let t438: f64 = (l.f745 - l.f79b);let t439: f64 = (l.f745 - l.f79b);let t43a: f64 = (t438 * t439);let t43b: f64 = (4.0 * l.f643);let t43c: f64 = (t43b * l.f643);let t43d: f64 = (t43a + t43c);let t43e: f64 = (t43d).sqrt();let t43f: f64 = (t437 - t43e);let t440: f64 = (0.5 * t43f);(l.f7a1, l.f7a4, l.f7a5, ) = (t440, (0.5 * ((l.f746 + l.f79e) - ((((l.f746 - l.f79e) * t439) + (t438 * (l.f746 - l.f79e))) / (2.0 * t43e)))), (0.5 * ((l.f747 + l.f79f) - ((((l.f747 - l.f79f) * t439) + (t438 * (l.f747 - l.f79f))) / (2.0 * t43e)))), );let t441: f64 = (l.f745 + l.f755);let t442: f64 = (l.f745 - l.f755);let t443: f64 = (l.f745 - l.f755);let t444: f64 = (t442 * t443);let t445: f64 = (4.0 * l.f647);let t446: f64 = (t445 * l.f647);let t447: f64 = (t444 + t446);let t448: f64 = (t447).sqrt();let t449: f64 = (t441 - t448);let t44a: f64 = (0.5 * t449);(l.f74f, l.f752, l.f753, ) = (t44a, (0.5 * (l.f746 - (((l.f746 * t443) + (t442 * l.f746)) / (2.0 * t448)))), (0.5 * (l.f747 - (((l.f747 * t443) + (t442 * l.f747)) / (2.0 * t448)))), );let t44b: f64 = l.f745;let t44c: f64 = l.f745;let t44d: f64 = l.f745;let t44e: f64 = (t44c * t44d);let t44f: f64 = (4.0 * 1e-6);let t450: f64 = (t44f * 1e-6);let t451: f64 = (t44e + t450);let t452: f64 = (t451).sqrt();let t453: f64 = (t44b - t452);let t454: f64 = (0.5 * t453);(l.f749, l.f74c, l.f74d, ) = (t454, (0.5 * (l.f746 - (((l.f746 * t44d) + (t44c * l.f746)) / (2.0 * t452)))), (0.5 * (l.f747 - (((l.f747 * t44d) + (t44c * l.f747)) / (2.0 * t452)))), );}
        if ((l.f3e0 == 0.0) && (l.f3f0 == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );(l.f713, l.f716, l.f717, ) = (0.0, 0.0, 0.0, );(l.f795, l.f798, l.f799, ) = (0.0, 0.0, 0.0, );(l.f824, l.f827, l.f828, ) = (0.0, 0.0, 0.0, );(l.f7a1, l.f7a4, l.f7a5, ) = (0.0, 0.0, 0.0, );(l.f74f, l.f752, l.f753, ) = (0.0, 0.0, 0.0, );(l.f749, l.f74c, l.f74d, ) = (0.0, 0.0, 0.0, );}
        let t455: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f420 = t455;
        if ((l.f3e0 == 0.0) && (l.f420 != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );}
        let t456: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f422 = t456;
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f422 != 0.0)) {let t457: f64 = (l.f795 * l.f769);let t458: f64 = (1.0 - t457);let t459: f64 = (t458).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (t459, ((-(l.f798 * l.f769)) / (2.0 * t459)), ((-(l.f799 * l.f769)) / (2.0 * t459)), );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f422 == 0.0)) {let t45a: f64 = (l.f795 * l.f769);let t45b: f64 = (1.0 - t45a);let t45c: f64 = (t45b).powf(l.f60b);(l.f6fb, l.f6fe, l.f6ff, ) = (t45c, if 0.0 == 0.0 && ((l.f60b) as f64).is_finite() && ((l.f60b) as f64).fract() == 0.0 { if l.f60b == 0.0 { 0.0 } else { (l.f60b * ((t45b).powf(l.f60b - 1.0) * (-(l.f798 * l.f769)))) } } else { (t45c * (l.f60b * ((-(l.f798 * l.f769)) / t45b))) }, if 0.0 == 0.0 && ((l.f60b) as f64).is_finite() && ((l.f60b) as f64).fract() == 0.0 { if l.f60b == 0.0 { 0.0 } else { (l.f60b * ((t45b).powf(l.f60b - 1.0) * (-(l.f799 * l.f769)))) } } else { (t45c * (l.f60b * ((-(l.f799 * l.f769)) / t45b))) }, );}
        if ((l.f3e0 == 0.0) && (l.f420 == 0.0)) {let t45d: f64 = (1.0 - l.f6fb);let t45e: f64 = (l.f69e * t45d);let t45f: f64 = (l.f745 - l.f795);let t460: f64 = (l.f698 * t45f);let t461: f64 = (t45e + t460);(l.f68c, l.f68d, l.f68e, ) = (t461, ((l.f69e * (-l.f6fe)) + (l.f698 * (l.f746 - l.f798))), ((l.f69e * (-l.f6ff)) + (l.f698 * (l.f747 - l.f799))), );let t462: f64 = (l.f542 * l.f536);(l.f52e, l.f533, l.f534, ) = (t462, (l.f542 * l.f537), (l.f542 * l.f538), );}
        let t463: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.f424 = t463;
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 != 0.0)) {(l.f757, l.f75a, l.f75b, ) = (0.0, 0.0, 0.0, );(l.f7e8, l.f7eb, l.f7ec, ) = (0.0, 0.0, 0.0, );(l.f7d0, l.f7d3, l.f7d4, ) = (0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_159(
        l: &mut StampLocals,
    ) {
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 != 0.0)) {(l.f8, l.fb, l.fc, ) = (0.0, 0.0, 0.0, );(l.f592, l.f595, l.f596, ) = (0.0, 0.0, 0.0, );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 == 0.0)) {let t464: f64 = (l.f75d - l.f7a1);(l.f757, l.f75a, l.f75b, ) = (t464, (-l.f7a4), (-l.f7a5), );let t465: f64 = (l.f713 / l.f757);let t466: f64 = (1.0 - t465);let t467: f64 = (t466).sqrt();let t468: f64 = (1.0 - t467);(l.f7ee, l.f7f1, l.f7f2, ) = (t468, (-((-(((l.f716 * l.f757) - (l.f713 * l.f75a)) / (l.f757 * l.f757))) / (2.0 * t467))), (-((-(((l.f717 * l.f757) - (l.f713 * l.f75b)) / (l.f757 * l.f757))) / (2.0 * t467))), );}
        let t469: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f426 = t469;
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 == 0.0)) && (l.f426 != 0.0)) {(l.f65, l.f68, l.f69, ) = (0.0, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 == 0.0)) && (l.f426 == 0.0)) {let t46a: f64 = (l.f7ee * l.f7ee);let t46b: f64 = (l.f7ee).ln();let t46c: f64 = (t46a * t46b);let t46d: f64 = (1.0 - l.f7ee);let t46e: f64 = (t46c / t46d);let t46f: f64 = (t46e + l.f7ee);let t470: f64 = (2.0 * l.f623);let t471: f64 = (1.0 - t470);let t472: f64 = (t46f * t471);(l.f65, l.f68, l.f69, ) = (t472, (((((((((l.f7f1 * l.f7ee) + (l.f7ee * l.f7f1)) * t46b) + (t46a * (l.f7f1 / l.f7ee))) * t46d) - (t46c * (-l.f7f1))) / (t46d * t46d)) + l.f7f1) * t471), (((((((((l.f7f2 * l.f7ee) + (l.f7ee * l.f7f2)) * t46b) + (t46a * (l.f7f2 / l.f7ee))) * t46d) - (t46c * (-l.f7f2))) / (t46d * t46d)) + l.f7f2) * t471), );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 == 0.0)) {let t473: f64 = (l.f7ee + l.f65);(l.f7e8, l.f7eb, l.f7ec, ) = (t473, (l.f7f1 + l.f68), (l.f7f2 + l.f69), );}
        let t474: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f428 = t474;
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 == 0.0)) && (l.f428 != 0.0)) {let t475: f64 = (l.f757 * l.f773);let t476: f64 = (t475).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (t476, ((l.f75a * l.f773) / (2.0 * t476)), ((l.f75b * l.f773) / (2.0 * t476)), );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 == 0.0)) && (l.f428 == 0.0)) {let t477: f64 = (l.f757 * l.f773);let t478: f64 = (t477).powf(l.f623);(l.f6fb, l.f6fe, l.f6ff, ) = (t478, if 0.0 == 0.0 && ((l.f623) as f64).is_finite() && ((l.f623) as f64).fract() == 0.0 { if l.f623 == 0.0 { 0.0 } else { (l.f623 * ((t477).powf(l.f623 - 1.0) * (l.f75a * l.f773))) } } else { (t478 * (l.f623 * ((l.f75a * l.f773) / t477))) }, if 0.0 == 0.0 && ((l.f623) as f64).is_finite() && ((l.f623) as f64).fract() == 0.0 { if l.f623 == 0.0 { 0.0 } else { (l.f623 * ((t477).powf(l.f623 - 1.0) * (l.f75b * l.f773))) } } else { (t478 * (l.f623 * ((l.f75b * l.f773) / t477))) }, );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f424 == 0.0)) {let t479: f64 = (l.f7d6 * l.f6fb);(l.f7d0, l.f7d3, l.f7d4, ) = (t479, (l.f7d6 * l.f6fe), (l.f7d6 * l.f6ff), );let t47a: f64 = (l.f824 - 1.0);let t47b: f64 = (t47a * l.f7d0);let t47c: f64 = (l.fc9 * t47b);(l.f8, l.fb, l.fc, ) = (t47c, (l.fc9 * ((l.f827 * l.f7d0) + (t47a * l.f7d3))), (l.fc9 * ((l.f828 * l.f7d0) + (t47a * l.f7d4))), );let t47d: f64 = (l.f8 * l.f7e8);let t47e: f64 = (l.f39 * t47d);(l.f592, l.f595, l.f596, ) = (t47e, (l.f39 * ((l.fb * l.f7e8) + (l.f8 * l.f7eb))), (l.f39 * ((l.fc * l.f7e8) + (l.f8 * l.f7ec))), );}
        let t47f: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.f42a = t47f;
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a != 0.0)) {(l.f598, l.f59b, l.f59c, ) = (0.0, 0.0, 0.0, );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) {let t480: f64 = (l.f7d0 * l.f60b);let t481: f64 = (t480 / l.f757);let t482: f64 = (l.f1e * t481);(l.f18, l.f1b, l.f1c, ) = (t482, (l.f1e * ((((l.f7d3 * l.f60b) * l.f757) - (t480 * l.f75a)) / (l.f757 * l.f757))), (l.f1e * ((((l.f7d4 * l.f60b) * l.f757) - (t480 * l.f75b)) / (l.f757 * l.f757))), );let t483: f64 = (0.666666666666667 * l.fe);let t484: f64 = (t483 / l.f18);(l.f719, l.f71c, l.f71d, ) = (t484, (-((t483 * l.f1b) / (l.f18 * l.f18))), (-((t483 * l.f1c) / (l.f18 * l.f18))), );let t485: f64 = (l.f719 * l.f719);(l.f72b, l.f72e, l.f72f, ) = (t485, ((l.f71c * l.f719) + (l.f719 * l.f71c)), ((l.f71d * l.f719) + (l.f719 * l.f71d)), );let t486: f64 = (l.f72b * l.f72b);let t487: f64 = (l.f72b * l.f72b);let t488: f64 = (t487 + 1.0);let t489: f64 = (t486 / t488);let t48a: f64 = (t489).sqrt();(l.f725, l.f728, l.f729, ) = (t48a, ((((((l.f72e * l.f72b) + (l.f72b * l.f72e)) * t488) - (t486 * ((l.f72e * l.f72b) + (l.f72b * l.f72e)))) / (t488 * t488)) / (2.0 * t48a)), ((((((l.f72f * l.f72b) + (l.f72b * l.f72f)) * t488) - (t486 * ((l.f72f * l.f72b) + (l.f72b * l.f72f)))) / (t488 * t488)) / (2.0 * t48a)), );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) {let t48b: f64 = (l.f725).abs();let t48c: f64 = (t48b).sqrt();(l.f6c0, l.f6c3, l.f6c4, ) = (t48c, (if l.f725 >= 0.0 { l.f728 } else { (-l.f728) } / (2.0 * t48c)), (if l.f725 >= 0.0 { l.f729 } else { (-l.f729) } / (2.0 * t48c)), );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) {let t48d: f64 = (l.f725 * l.f6c0);(l.f731, l.f734, l.f735, ) = (t48d, ((l.f728 * l.f6c0) + (l.f725 * l.f6c3)), ((l.f729 * l.f6c0) + (l.f725 * l.f6c4)), );}
        let t48e: f64 = (-l.f623);let t48f: f64 = (t48e * l.f611);let t490: f64 = (-1.0);let t491: f64 = if t48f == t490 { 1.0 } else { 0.0 };l.f42c = t491;
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f42c != 0.0)) {let t492: f64 = (l.f18 * l.f731);let t493: f64 = (1.0 + t492);let t494: f64 = (1.0 / t493);(l.f7e2, l.f7e5, l.f7e6, ) = (t494, (-(((l.f1b * l.f731) + (l.f18 * l.f734)) / (t493 * t493))), (-(((l.f1c * l.f731) + (l.f18 * l.f735)) / (t493 * t493))), );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f42c == 0.0)) {let t495: f64 = (l.f18 * l.f731);let t496: f64 = (1.0 + t495);let t497: f64 = (-l.f623);let t498: f64 = (t497 * l.f611);let t499: f64 = (t496).powf(t498);(l.f7e2, l.f7e5, l.f7e6, ) = (t499, if 0.0 == 0.0 && ((t498) as f64).is_finite() && ((t498) as f64).fract() == 0.0 { if t498 == 0.0 { 0.0 } else { (t498 * ((t496).powf(t498 - 1.0) * ((l.f1b * l.f731) + (l.f18 * l.f734)))) } } else { (t499 * (t498 * (((l.f1b * l.f731) + (l.f18 * l.f734)) / t496))) }, if 0.0 == 0.0 && ((t498) as f64).is_finite() && ((t498) as f64).fract() == 0.0 { if t498 == 0.0 { 0.0 } else { (t498 * ((t496).powf(t498 - 1.0) * ((l.f1c * l.f731) + (l.f18 * l.f735)))) } } else { (t499 * (t498 * (((l.f1c * l.f731) + (l.f18 * l.f735)) / t496))) }, );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) {let t49a: f64 = (l.f7e8 * l.f7e2);let t49b: f64 = (l.f7e8 + l.f7e2);let t49c: f64 = (t49a / t49b);(l.f7f4, l.f7f7, l.f7f8, ) = (t49c, (((((l.f7eb * l.f7e2) + (l.f7e8 * l.f7e5)) * t49b) - (t49a * (l.f7eb + l.f7e5))) / (t49b * t49b)), (((((l.f7ec * l.f7e2) + (l.f7e8 * l.f7e6)) * t49b) - (t49a * (l.f7ec + l.f7e6))) / (t49b * t49b)), );let t49d: f64 = (l.f18 / l.f6c0);let t49e: f64 = (0.375 * t49d);let t49f: f64 = (t49e).sqrt();(l.f5a7, l.f5aa, l.f5ab, ) = (t49f, ((0.375 * (((l.f1b * l.f6c0) - (l.f18 * l.f6c3)) / (l.f6c0 * l.f6c0))) / (2.0 * t49f)), ((0.375 * (((l.f1c * l.f6c0) - (l.f18 * l.f6c4)) / (l.f6c0 * l.f6c0))) / (2.0 * t49f)), );let t4a0: f64 = (l.f719 * l.f6c0);let t4a1: f64 = (2.0 * t4a0);let t4a2: f64 = (t4a1 - l.f725);(l.f5b3, l.f5b6, l.f5b7, ) = (t4a2, ((2.0 * ((l.f71c * l.f6c0) + (l.f719 * l.f6c3))) - l.f728), ((2.0 * ((l.f71d * l.f6c0) + (l.f719 * l.f6c4))) - l.f729), );}
    }
}
