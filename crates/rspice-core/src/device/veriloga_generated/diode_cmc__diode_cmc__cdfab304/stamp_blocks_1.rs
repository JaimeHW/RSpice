#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c3 == 0.0)) && (l.f4c5 == 0.0)) {let t0: f64 = (l.f737 / l.f5f1);let t1: f64 = (l.f5f1 - l.f5ed);let t2: f64 = (l.f793 * t1);let t3: f64 = (l.f5ed * p.p85);let t4: f64 = (t2 / t3);let t5: f64 = (t0 + t4);let t6: f64 = (l.f645 * t5);let t7: f64 = (t6 - 230.25850929940458);let t8: f64 = (l.f737 / l.f5f1);let t9: f64 = (l.f5f1 - l.f5ed);let ta: f64 = (l.f793 * t9);let tb: f64 = (l.f5ed * p.p85);let tc: f64 = (ta / tb);let td: f64 = (t8 + tc);let te: f64 = (l.f645 * td);let tf: f64 = (te - 230.25850929940458);let t10: f64 = (l.f737 / l.f5f1);let t11: f64 = (l.f5f1 - l.f5ed);let t12: f64 = (l.f793 * t11);let t13: f64 = (l.f5ed * p.p85);let t14: f64 = (t12 / t13);let t15: f64 = (t10 + t14);let t16: f64 = (l.f645 * t15);let t17: f64 = (t16 - 230.25850929940458);let t18: f64 = (t17 * 0.3333333333333333);let t19: f64 = (1.0 + t18);let t1a: f64 = (tf * t19);let t1b: f64 = (0.5 * t1a);let t1c: f64 = (1.0 + t1b);let t1d: f64 = (t7 * t1c);let t1e: f64 = (1.0 + t1d);let t1f: f64 = (1e100 * t1e);(l.f53a, l.f53b, l.f53c, ) = (t1f, (1e100 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3) - (t2 * (l.f5ee * p.p85))) / (t3 * t3)))) * t1c) + (t7 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tb) - (ta * (l.f5ee * p.p85))) / (tb * tb)))) * t19) + (tf * ((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t13) - (t12 * (l.f5ee * p.p85))) / (t13 * t13)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3) - (t2 * (l.f5ef * p.p85))) / (t3 * t3)))) * t1c) + (t7 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tb) - (ta * (l.f5ef * p.p85))) / (tb * tb)))) * t19) + (tf * ((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t13) - (t12 * (l.f5ef * p.p85))) / (t13 * t13)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let t20: f64 = (l.f737 - l.f7b1);let t21: f64 = (t20 * l.f645);let t22: f64 = (1.0 + t21);let t23: f64 = (t22 * l.f89);let t24: f64 = (t23).sqrt();l.f825 = t24;let t25: f64 = (l.f5eb * l.f5eb);let t26: f64 = (t25 / l.f5df);l.f64f = t26;let t27: f64 = (l.f5e5 / l.f645);let t28: f64 = (l.f5df / l.f64f);let t29: f64 = (t28).ln();let t2a: f64 = (t27 * t29);l.f793 = t2a;}
        let t2b: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f4c7 = t2b;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t2c: f64 = (l.f7b1 - l.f793);let t2d: f64 = (p.p86 * t2c);let t2e: f64 = (t2d + l.f5e5);(l.f601, l.f602, l.f603, ) = (t2e, 0.0, 0.0, );let t2f: f64 = (p.p86 * l.f793);let t30: f64 = (l.f5e5 - t2f);(l.f5ed, l.f5ee, l.f5ef, ) = (t30, 0.0, 0.0, );let t31: f64 = (p.p85 - l.f601);let t32: f64 = (t31 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t32, (-l.f602), (-l.f603), );let t33: f64 = (4.0 * p.p85);let t34: f64 = (t33 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t34, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {
            let (t36, t37, t38,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t35: f64 = (-l.f6f7);
        (t35, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t36, t37, t38, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t39: f64 = (l.f6f3 * l.f6f3);let t3a: f64 = (t39 + l.f6f7);let t3b: f64 = (t3a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3b)), );let t3c: f64 = (l.f6f3 / l.f6f7);let t3d: f64 = (1.0 + t3c);let t3e: f64 = (0.5 * t3d);(l.f55, l.f56, l.f57, ) = (t3e, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t3f: f64 = (l.f6f3 + l.f6f7);let t40: f64 = (0.5 * t3f);let t41: f64 = (p.p85 - t40);(l.f605, l.f606, l.f607, ) = (t41, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t42: f64 = (l.f605 - l.f5e5);let t43: f64 = (t42 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t43, l.f606, l.f607, );let t44: f64 = (4.0 * l.f5e5);let t45: f64 = (t44 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t45, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {
            let (t47, t48, t49,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t46: f64 = (-l.f6f7);
        (t46, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t47, t48, t49, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t4a: f64 = (l.f6f3 * l.f6f3);let t4b: f64 = (t4a + l.f6f7);let t4c: f64 = (t4b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t4c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t4c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t4c)), );let t4d: f64 = (l.f6f3 / l.f6f7);let t4e: f64 = (1.0 + t4d);let t4f: f64 = (0.5 * t4e);(l.f51, l.f52, l.f53, ) = (t4f, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t50: f64 = (l.f6f3 + l.f6f7);let t51: f64 = (0.5 * t50);let t52: f64 = (l.f5e5 + t51);(l.f5f1, l.f5f2, l.f5f3, ) = (t52, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t53: f64 = (p.p85 - l.f5ed);let t54: f64 = (t53 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t54, (-l.f5ee), (-l.f5ef), );let t55: f64 = (4.0 * p.p85);let t56: f64 = (t55 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t56, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {
            let (t58, t59, t5a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t57: f64 = (-l.f6f7);
        (t57, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t58, t59, t5a, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t5b: f64 = (l.f6f3 * l.f6f3);let t5c: f64 = (t5b + l.f6f7);let t5d: f64 = (t5c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t5d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t5d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t5d)), );let t5e: f64 = (l.f6f3 + l.f6f7);let t5f: f64 = (0.5 * t5e);let t60: f64 = (p.p85 - t5f);(l.f5ed, l.f5ee, l.f5ef, ) = (t60, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t61: f64 = (l.f5ed - l.f5e5);let t62: f64 = (t61 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t62, l.f5ee, l.f5ef, );let t63: f64 = (4.0 * l.f5e5);let t64: f64 = (t63 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t64, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {
            let (t66, t67, t68,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t65: f64 = (-l.f6f7);
        (t65, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t66, t67, t68, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 != 0.0)) {let t69: f64 = (l.f6f3 * l.f6f3);let t6a: f64 = (t69 + l.f6f7);let t6b: f64 = (t6a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t6b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t6b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t6b)), );let t6c: f64 = (l.f6f3 + l.f6f7);let t6d: f64 = (0.5 * t6c);let t6e: f64 = (l.f5e5 + t6d);(l.f5ed, l.f5ee, l.f5ef, ) = (t6e, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t6f: f64 = (p.p86 * l.f55);let t70: f64 = (t6f * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t70, (((p.p86 * l.f56) * l.f51) + (t6f * l.f52)), (((p.p86 * l.f57) * l.f51) + (t6f * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4c7 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t71: f64 = (l.f7b1 / l.f5f1);let t72: f64 = (l.f5f1 - l.f5ed);let t73: f64 = (l.f793 * t72);let t74: f64 = (l.f5ed * p.p85);let t75: f64 = (t73 / t74);let t76: f64 = (t71 + t75);let t77: f64 = (l.f645 * t76);let t78: f64 = (t77).abs();let t79: f64 = if t78 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4cb = t79;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cb != 0.0)) {let t7a: f64 = (l.f7b1 / l.f5f1);let t7b: f64 = (l.f5f1 - l.f5ed);let t7c: f64 = (l.f793 * t7b);let t7d: f64 = (l.f5ed * p.p85);let t7e: f64 = (t7c / t7d);let t7f: f64 = (t7a + t7e);let t80: f64 = (l.f645 * t7f);let t81: f64 = (t80).exp();(l.f8a, l.f8b, l.f8c, ) = (t81, (t81 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t7d) - (t7c * (l.f5ee * p.p85))) / (t7d * t7d))))), (t81 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t7d) - (t7c * (l.f5ef * p.p85))) / (t7d * t7d))))), );}
        let t82: f64 = (l.f7b1 / l.f5f1);let t83: f64 = (l.f5f1 - l.f5ed);let t84: f64 = (l.f793 * t83);let t85: f64 = (l.f5ed * p.p85);let t86: f64 = (t84 / t85);let t87: f64 = (t82 + t86);let t88: f64 = (l.f645 * t87);let t89: f64 = (-230.25850929940458);let t8a: f64 = if t88 < t89 { 1.0 } else { 0.0 };l.f4cd = t8a;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cb == 0.0)) && (l.f4cd != 0.0)) {let t8b: f64 = (-230.25850929940458);let t8c: f64 = (l.f7b1 / l.f5f1);let t8d: f64 = (l.f5f1 - l.f5ed);let t8e: f64 = (l.f793 * t8d);let t8f: f64 = (l.f5ed * p.p85);let t90: f64 = (t8e / t8f);let t91: f64 = (t8c + t90);let t92: f64 = (l.f645 * t91);let t93: f64 = (t8b - t92);let t94: f64 = (-230.25850929940458);let t95: f64 = (l.f7b1 / l.f5f1);let t96: f64 = (l.f5f1 - l.f5ed);let t97: f64 = (l.f793 * t96);let t98: f64 = (l.f5ed * p.p85);let t99: f64 = (t97 / t98);let t9a: f64 = (t95 + t99);let t9b: f64 = (l.f645 * t9a);let t9c: f64 = (t94 - t9b);let t9d: f64 = (-230.25850929940458);let t9e: f64 = (l.f7b1 / l.f5f1);let t9f: f64 = (l.f5f1 - l.f5ed);let ta0: f64 = (l.f793 * t9f);let ta1: f64 = (l.f5ed * p.p85);let ta2: f64 = (ta0 / ta1);let ta3: f64 = (t9e + ta2);let ta4: f64 = (l.f645 * ta3);let ta5: f64 = (t9d - ta4);let ta6: f64 = (ta5 * 0.3333333333333333);let ta7: f64 = (1.0 + ta6);let ta8: f64 = (t9c * ta7);let ta9: f64 = (0.5 * ta8);let taa: f64 = (1.0 + ta9);let tab: f64 = (t93 * taa);let tac: f64 = (1.0 + tab);let tad: f64 = (1e-100 / tac);(l.f8a, l.f8b, l.f8c, ) = (tad, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t8f) - (t8e * (l.f5ee * p.p85))) / (t8f * t8f))))) * taa) + (t93 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t98) - (t97 * (l.f5ee * p.p85))) / (t98 * t98))))) * ta7) + (t9c * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * ta1) - (ta0 * (l.f5ee * p.p85))) / (ta1 * ta1))))) * 0.3333333333333333))))))) / (tac * tac))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t8f) - (t8e * (l.f5ef * p.p85))) / (t8f * t8f))))) * taa) + (t93 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t98) - (t97 * (l.f5ef * p.p85))) / (t98 * t98))))) * ta7) + (t9c * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * ta1) - (ta0 * (l.f5ef * p.p85))) / (ta1 * ta1))))) * 0.3333333333333333))))))) / (tac * tac))), );}
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cb == 0.0)) && (l.f4cd == 0.0)) {let tae: f64 = (l.f7b1 / l.f5f1);let taf: f64 = (l.f5f1 - l.f5ed);let tb0: f64 = (l.f793 * taf);let tb1: f64 = (l.f5ed * p.p85);let tb2: f64 = (tb0 / tb1);let tb3: f64 = (tae + tb2);let tb4: f64 = (l.f645 * tb3);let tb5: f64 = (tb4 - 230.25850929940458);let tb6: f64 = (l.f7b1 / l.f5f1);let tb7: f64 = (l.f5f1 - l.f5ed);let tb8: f64 = (l.f793 * tb7);let tb9: f64 = (l.f5ed * p.p85);let tba: f64 = (tb8 / tb9);let tbb: f64 = (tb6 + tba);let tbc: f64 = (l.f645 * tbb);let tbd: f64 = (tbc - 230.25850929940458);let tbe: f64 = (l.f7b1 / l.f5f1);let tbf: f64 = (l.f5f1 - l.f5ed);let tc0: f64 = (l.f793 * tbf);let tc1: f64 = (l.f5ed * p.p85);let tc2: f64 = (tc0 / tc1);let tc3: f64 = (tbe + tc2);let tc4: f64 = (l.f645 * tc3);let tc5: f64 = (tc4 - 230.25850929940458);let tc6: f64 = (tc5 * 0.3333333333333333);let tc7: f64 = (1.0 + tc6);let tc8: f64 = (tbd * tc7);let tc9: f64 = (0.5 * tc8);let tca: f64 = (1.0 + tc9);let tcb: f64 = (tb5 * tca);let tcc: f64 = (1.0 + tcb);let tcd: f64 = (1e100 * tcc);(l.f8a, l.f8b, l.f8c, ) = (tcd, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tb1) - (tb0 * (l.f5ee * p.p85))) / (tb1 * tb1)))) * tca) + (tb5 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tb9) - (tb8 * (l.f5ee * p.p85))) / (tb9 * tb9)))) * tc7) + (tbd * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tc1) - (tc0 * (l.f5ee * p.p85))) / (tc1 * tc1)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tb1) - (tb0 * (l.f5ef * p.p85))) / (tb1 * tb1)))) * tca) + (tb5 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tb9) - (tb8 * (l.f5ef * p.p85))) / (tb9 * tb9)))) * tc7) + (tbd * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tc1) - (tc0 * (l.f5ef * p.p85))) / (tc1 * tc1)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let tce: f64 = (l.f7b1 * l.f5b);let tcf: f64 = (l.f5f1 - tce);let td0: f64 = (l.f5f1 * l.f5f1);let td1: f64 = (tcf / td0);let td2: f64 = (l.f793 * l.f5b);let td3: f64 = (l.f5ed * p.p85);let td4: f64 = (td2 / td3);let td5: f64 = (td1 + td4);let td6: f64 = (l.f645 * td5);(l.f61, l.f62, l.f63, ) = (td6, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * td0) - (tcf * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (td0 * td0)) + ((((l.f793 * l.f5c) * td3) - (td2 * (l.f5ee * p.p85))) / (td3 * td3)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * td0) - (tcf * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (td0 * td0)) + ((((l.f793 * l.f5d) * td3) - (td2 * (l.f5ef * p.p85))) / (td3 * td3)))), );let td7: f64 = (l.f737 - l.f7b1);let td8: f64 = (td7 * l.f61);let td9: f64 = (1.0 + td8);let tda: f64 = (td9 * l.f8a);(l.f536, l.f537, l.f538, ) = (tda, (((td7 * l.f62) * l.f8a) + (td9 * l.f8b)), (((td7 * l.f63) * l.f8a) + (td9 * l.f8c)), );let tdb: f64 = (l.f5eb * l.f5eb);let tdc: f64 = (tdb / l.f5e3);l.f64f = tdc;let tdd: f64 = (l.f5e9 / l.f645);let tde: f64 = (l.f5e3 / l.f64f);let tdf: f64 = (tde).ln();let te0: f64 = (tdd * tdf);l.f793 = te0;}
        let te1: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f4cf = te1;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let te2: f64 = (l.f7b1 - l.f793);let te3: f64 = (p.p86 * te2);let te4: f64 = (te3 + l.f5e9);(l.f601, l.f602, l.f603, ) = (te4, 0.0, 0.0, );let te5: f64 = (p.p86 * l.f793);let te6: f64 = (l.f5e9 - te5);(l.f5ed, l.f5ee, l.f5ef, ) = (te6, 0.0, 0.0, );let te7: f64 = (p.p85 - l.f601);let te8: f64 = (te7 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (te8, (-l.f602), (-l.f603), );let te9: f64 = (4.0 * p.p85);let tea: f64 = (te9 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tea, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {
            let (tec, ted, tee,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let teb: f64 = (-l.f6f7);
        (teb, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tec, ted, tee, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let tef: f64 = (l.f6f3 * l.f6f3);let tf0: f64 = (tef + l.f6f7);let tf1: f64 = (tf0).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tf1, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tf1)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tf1)), );let tf2: f64 = (l.f6f3 / l.f6f7);let tf3: f64 = (1.0 + tf2);let tf4: f64 = (0.5 * tf3);(l.f55, l.f56, l.f57, ) = (tf4, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let tf5: f64 = (l.f6f3 + l.f6f7);let tf6: f64 = (0.5 * tf5);let tf7: f64 = (p.p85 - tf6);(l.f605, l.f606, l.f607, ) = (tf7, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let tf8: f64 = (l.f605 - l.f5e9);let tf9: f64 = (tf8 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tf9, l.f606, l.f607, );let tfa: f64 = (4.0 * l.f5e9);let tfb: f64 = (tfa * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tfb, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {
            let (tfd, tfe, tff,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tfc: f64 = (-l.f6f7);
        (tfc, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tfd, tfe, tff, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t100: f64 = (l.f6f3 * l.f6f3);let t101: f64 = (t100 + l.f6f7);let t102: f64 = (t101).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t102, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t102)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t102)), );let t103: f64 = (l.f6f3 / l.f6f7);let t104: f64 = (1.0 + t103);let t105: f64 = (0.5 * t104);(l.f51, l.f52, l.f53, ) = (t105, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t106: f64 = (l.f6f3 + l.f6f7);let t107: f64 = (0.5 * t106);let t108: f64 = (l.f5e9 + t107);(l.f5f1, l.f5f2, l.f5f3, ) = (t108, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t109: f64 = (p.p85 - l.f5ed);let t10a: f64 = (t109 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t10a, (-l.f5ee), (-l.f5ef), );let t10b: f64 = (4.0 * p.p85);let t10c: f64 = (t10b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t10c, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {
            let (t10e, t10f, t110,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t10d: f64 = (-l.f6f7);
        (t10d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t10e, t10f, t110, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t111: f64 = (l.f6f3 * l.f6f3);let t112: f64 = (t111 + l.f6f7);let t113: f64 = (t112).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t113, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t113)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t113)), );let t114: f64 = (l.f6f3 + l.f6f7);let t115: f64 = (0.5 * t114);let t116: f64 = (p.p85 - t115);(l.f5ed, l.f5ee, l.f5ef, ) = (t116, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t117: f64 = (l.f5ed - l.f5e9);let t118: f64 = (t117 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t118, l.f5ee, l.f5ef, );let t119: f64 = (4.0 * l.f5e9);let t11a: f64 = (t119 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t11a, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {
            let (t11c, t11d, t11e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t11b: f64 = (-l.f6f7);
        (t11b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t11c, t11d, t11e, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf != 0.0)) {let t11f: f64 = (l.f6f3 * l.f6f3);let t120: f64 = (t11f + l.f6f7);let t121: f64 = (t120).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t121, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t121)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t121)), );let t122: f64 = (l.f6f3 + l.f6f7);let t123: f64 = (0.5 * t122);let t124: f64 = (l.f5e9 + t123);(l.f5ed, l.f5ee, l.f5ef, ) = (t124, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t125: f64 = (p.p86 * l.f55);let t126: f64 = (t125 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t126, (((p.p86 * l.f56) * l.f51) + (t125 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t125 * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4cf == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t127: f64 = (l.f7b1 / l.f5f1);let t128: f64 = (l.f5f1 - l.f5ed);let t129: f64 = (l.f793 * t128);let t12a: f64 = (l.f5ed * p.p85);let t12b: f64 = (t129 / t12a);let t12c: f64 = (t127 + t12b);let t12d: f64 = (l.f645 * t12c);let t12e: f64 = (t12d).abs();let t12f: f64 = if t12e < 230.25850929940458 { 1.0 } else { 0.0 };l.f4d1 = t12f;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d1 != 0.0)) {let t130: f64 = (l.f7b1 / l.f5f1);let t131: f64 = (l.f5f1 - l.f5ed);let t132: f64 = (l.f793 * t131);let t133: f64 = (l.f5ed * p.p85);let t134: f64 = (t132 / t133);let t135: f64 = (t130 + t134);let t136: f64 = (l.f645 * t135);let t137: f64 = (t136).exp();(l.f93, l.f94, l.f95, ) = (t137, (t137 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t133) - (t132 * (l.f5ee * p.p85))) / (t133 * t133))))), (t137 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t133) - (t132 * (l.f5ef * p.p85))) / (t133 * t133))))), );}
        let t138: f64 = (l.f7b1 / l.f5f1);let t139: f64 = (l.f5f1 - l.f5ed);let t13a: f64 = (l.f793 * t139);let t13b: f64 = (l.f5ed * p.p85);let t13c: f64 = (t13a / t13b);let t13d: f64 = (t138 + t13c);let t13e: f64 = (l.f645 * t13d);let t13f: f64 = (-230.25850929940458);let t140: f64 = if t13e < t13f { 1.0 } else { 0.0 };l.f4d3 = t140;
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d1 == 0.0)) && (l.f4d3 != 0.0)) {let t141: f64 = (-230.25850929940458);let t142: f64 = (l.f7b1 / l.f5f1);let t143: f64 = (l.f5f1 - l.f5ed);let t144: f64 = (l.f793 * t143);let t145: f64 = (l.f5ed * p.p85);let t146: f64 = (t144 / t145);let t147: f64 = (t142 + t146);let t148: f64 = (l.f645 * t147);let t149: f64 = (t141 - t148);let t14a: f64 = (-230.25850929940458);let t14b: f64 = (l.f7b1 / l.f5f1);let t14c: f64 = (l.f5f1 - l.f5ed);let t14d: f64 = (l.f793 * t14c);let t14e: f64 = (l.f5ed * p.p85);let t14f: f64 = (t14d / t14e);let t150: f64 = (t14b + t14f);let t151: f64 = (l.f645 * t150);let t152: f64 = (t14a - t151);let t153: f64 = (-230.25850929940458);let t154: f64 = (l.f7b1 / l.f5f1);let t155: f64 = (l.f5f1 - l.f5ed);let t156: f64 = (l.f793 * t155);let t157: f64 = (l.f5ed * p.p85);let t158: f64 = (t156 / t157);let t159: f64 = (t154 + t158);let t15a: f64 = (l.f645 * t159);let t15b: f64 = (t153 - t15a);let t15c: f64 = (t15b * 0.3333333333333333);let t15d: f64 = (1.0 + t15c);let t15e: f64 = (t152 * t15d);let t15f: f64 = (0.5 * t15e);let t160: f64 = (1.0 + t15f);let t161: f64 = (t149 * t160);let t162: f64 = (1.0 + t161);let t163: f64 = (1e-100 / t162);(l.f93, l.f94, l.f95, ) = (t163, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t145) - (t144 * (l.f5ee * p.p85))) / (t145 * t145))))) * t160) + (t149 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t14e) - (t14d * (l.f5ee * p.p85))) / (t14e * t14e))))) * t15d) + (t152 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t157) - (t156 * (l.f5ee * p.p85))) / (t157 * t157))))) * 0.3333333333333333))))))) / (t162 * t162))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t145) - (t144 * (l.f5ef * p.p85))) / (t145 * t145))))) * t160) + (t149 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t14e) - (t14d * (l.f5ef * p.p85))) / (t14e * t14e))))) * t15d) + (t152 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t157) - (t156 * (l.f5ef * p.p85))) / (t157 * t157))))) * 0.3333333333333333))))))) / (t162 * t162))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d1 == 0.0)) && (l.f4d3 == 0.0)) {let t164: f64 = (l.f7b1 / l.f5f1);let t165: f64 = (l.f5f1 - l.f5ed);let t166: f64 = (l.f793 * t165);let t167: f64 = (l.f5ed * p.p85);let t168: f64 = (t166 / t167);let t169: f64 = (t164 + t168);let t16a: f64 = (l.f645 * t169);let t16b: f64 = (t16a - 230.25850929940458);let t16c: f64 = (l.f7b1 / l.f5f1);let t16d: f64 = (l.f5f1 - l.f5ed);let t16e: f64 = (l.f793 * t16d);let t16f: f64 = (l.f5ed * p.p85);let t170: f64 = (t16e / t16f);let t171: f64 = (t16c + t170);let t172: f64 = (l.f645 * t171);let t173: f64 = (t172 - 230.25850929940458);let t174: f64 = (l.f7b1 / l.f5f1);let t175: f64 = (l.f5f1 - l.f5ed);let t176: f64 = (l.f793 * t175);let t177: f64 = (l.f5ed * p.p85);let t178: f64 = (t176 / t177);let t179: f64 = (t174 + t178);let t17a: f64 = (l.f645 * t179);let t17b: f64 = (t17a - 230.25850929940458);let t17c: f64 = (t17b * 0.3333333333333333);let t17d: f64 = (1.0 + t17c);let t17e: f64 = (t173 * t17d);let t17f: f64 = (0.5 * t17e);let t180: f64 = (1.0 + t17f);let t181: f64 = (t16b * t180);let t182: f64 = (1.0 + t181);let t183: f64 = (1e100 * t182);(l.f93, l.f94, l.f95, ) = (t183, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t167) - (t166 * (l.f5ee * p.p85))) / (t167 * t167)))) * t180) + (t16b * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t16f) - (t16e * (l.f5ee * p.p85))) / (t16f * t16f)))) * t17d) + (t173 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t177) - (t176 * (l.f5ee * p.p85))) / (t177 * t177)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t167) - (t166 * (l.f5ef * p.p85))) / (t167 * t167)))) * t180) + (t16b * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t16f) - (t16e * (l.f5ef * p.p85))) / (t16f * t16f)))) * t17d) + (t173 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t177) - (t176 * (l.f5ef * p.p85))) / (t177 * t177)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let t184: f64 = (l.f7b1 * l.f5b);let t185: f64 = (l.f5f1 - t184);let t186: f64 = (l.f5f1 * l.f5f1);let t187: f64 = (t185 / t186);let t188: f64 = (l.f793 * l.f5b);let t189: f64 = (l.f5ed * p.p85);let t18a: f64 = (t188 / t189);let t18b: f64 = (t187 + t18a);let t18c: f64 = (l.f645 * t18b);(l.f61, l.f62, l.f63, ) = (t18c, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t186) - (t185 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t186 * t186)) + ((((l.f793 * l.f5c) * t189) - (t188 * (l.f5ee * p.p85))) / (t189 * t189)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t186) - (t185 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t186 * t186)) + ((((l.f793 * l.f5d) * t189) - (t188 * (l.f5ef * p.p85))) / (t189 * t189)))), );let t18d: f64 = (l.f737 - l.f7b1);let t18e: f64 = (t18d * l.f61);let t18f: f64 = (1.0 + t18e);let t190: f64 = (t18f * l.f93);(l.f53e, l.f53f, l.f540, ) = (t190, (((t18d * l.f62) * l.f93) + (t18f * l.f94)), (((t18d * l.f63) * l.f93) + (t18f * l.f95)), );let t191: f64 = (l.f5eb * l.f5eb);let t192: f64 = (t191 / l.f5e1);l.f64f = t192;let t193: f64 = (l.f5e7 / l.f645);let t194: f64 = (l.f5e1 / l.f64f);let t195: f64 = (t194).ln();let t196: f64 = (t193 * t195);l.f793 = t196;}
        let t197: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f4d5 = t197;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t198: f64 = (l.f7b1 - l.f793);let t199: f64 = (p.p86 * t198);let t19a: f64 = (t199 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t19a, 0.0, 0.0, );let t19b: f64 = (p.p86 * l.f793);let t19c: f64 = (l.f5e7 - t19b);(l.f5ed, l.f5ee, l.f5ef, ) = (t19c, 0.0, 0.0, );let t19d: f64 = (p.p85 - l.f601);let t19e: f64 = (t19d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t19e, (-l.f602), (-l.f603), );let t19f: f64 = (4.0 * p.p85);let t1a0: f64 = (t19f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1a0, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {
            let (t1a2, t1a3, t1a4,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1a1: f64 = (-l.f6f7);
        (t1a1, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1a2, t1a3, t1a4, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t1a5: f64 = (l.f6f3 * l.f6f3);let t1a6: f64 = (t1a5 + l.f6f7);let t1a7: f64 = (t1a6).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1a7, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1a7)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1a7)), );let t1a8: f64 = (l.f6f3 / l.f6f7);let t1a9: f64 = (1.0 + t1a8);let t1aa: f64 = (0.5 * t1a9);(l.f55, l.f56, l.f57, ) = (t1aa, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t1ab: f64 = (l.f6f3 + l.f6f7);let t1ac: f64 = (0.5 * t1ab);let t1ad: f64 = (p.p85 - t1ac);(l.f605, l.f606, l.f607, ) = (t1ad, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t1ae: f64 = (l.f605 - l.f5e7);let t1af: f64 = (t1ae - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1af, l.f606, l.f607, );let t1b0: f64 = (4.0 * l.f5e7);let t1b1: f64 = (t1b0 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1b1, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {
            let (t1b3, t1b4, t1b5,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1b2: f64 = (-l.f6f7);
        (t1b2, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1b3, t1b4, t1b5, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t1b6: f64 = (l.f6f3 * l.f6f3);let t1b7: f64 = (t1b6 + l.f6f7);let t1b8: f64 = (t1b7).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1b8, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1b8)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1b8)), );let t1b9: f64 = (l.f6f3 / l.f6f7);let t1ba: f64 = (1.0 + t1b9);let t1bb: f64 = (0.5 * t1ba);(l.f51, l.f52, l.f53, ) = (t1bb, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t1bc: f64 = (l.f6f3 + l.f6f7);let t1bd: f64 = (0.5 * t1bc);let t1be: f64 = (l.f5e7 + t1bd);(l.f5f1, l.f5f2, l.f5f3, ) = (t1be, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t1bf: f64 = (p.p85 - l.f5ed);let t1c0: f64 = (t1bf - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1c0, (-l.f5ee), (-l.f5ef), );let t1c1: f64 = (4.0 * p.p85);let t1c2: f64 = (t1c1 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1c2, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {
            let (t1c4, t1c5, t1c6,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1c3: f64 = (-l.f6f7);
        (t1c3, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1c4, t1c5, t1c6, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t1c7: f64 = (l.f6f3 * l.f6f3);let t1c8: f64 = (t1c7 + l.f6f7);let t1c9: f64 = (t1c8).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1c9, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1c9)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1c9)), );let t1ca: f64 = (l.f6f3 + l.f6f7);let t1cb: f64 = (0.5 * t1ca);let t1cc: f64 = (p.p85 - t1cb);(l.f5ed, l.f5ee, l.f5ef, ) = (t1cc, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t1cd: f64 = (l.f5ed - l.f5e7);let t1ce: f64 = (t1cd - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1ce, l.f5ee, l.f5ef, );let t1cf: f64 = (4.0 * l.f5e7);let t1d0: f64 = (t1cf * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1d0, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {
            let (t1d2, t1d3, t1d4,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1d1: f64 = (-l.f6f7);
        (t1d1, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1d2, t1d3, t1d4, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 != 0.0)) {let t1d5: f64 = (l.f6f3 * l.f6f3);let t1d6: f64 = (t1d5 + l.f6f7);let t1d7: f64 = (t1d6).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1d7, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1d7)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1d7)), );let t1d8: f64 = (l.f6f3 + l.f6f7);let t1d9: f64 = (0.5 * t1d8);let t1da: f64 = (l.f5e7 + t1d9);(l.f5ed, l.f5ee, l.f5ef, ) = (t1da, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t1db: f64 = (p.p86 * l.f55);let t1dc: f64 = (t1db * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t1dc, (((p.p86 * l.f56) * l.f51) + (t1db * l.f52)), (((p.p86 * l.f57) * l.f51) + (t1db * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d5 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t1dd: f64 = (l.f7b1 / l.f5f1);let t1de: f64 = (l.f5f1 - l.f5ed);let t1df: f64 = (l.f793 * t1de);let t1e0: f64 = (l.f5ed * p.p85);let t1e1: f64 = (t1df / t1e0);let t1e2: f64 = (t1dd + t1e1);let t1e3: f64 = (l.f645 * t1e2);let t1e4: f64 = (t1e3).abs();let t1e5: f64 = if t1e4 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4d7 = t1e5;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d7 != 0.0)) {let t1e6: f64 = (l.f7b1 / l.f5f1);let t1e7: f64 = (l.f5f1 - l.f5ed);let t1e8: f64 = (l.f793 * t1e7);let t1e9: f64 = (l.f5ed * p.p85);let t1ea: f64 = (t1e8 / t1e9);let t1eb: f64 = (t1e6 + t1ea);let t1ec: f64 = (l.f645 * t1eb);let t1ed: f64 = (t1ec).exp();(l.f8e, l.f8f, l.f90, ) = (t1ed, (t1ed * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e9) - (t1e8 * (l.f5ee * p.p85))) / (t1e9 * t1e9))))), (t1ed * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e9) - (t1e8 * (l.f5ef * p.p85))) / (t1e9 * t1e9))))), );}
        let t1ee: f64 = (l.f7b1 / l.f5f1);let t1ef: f64 = (l.f5f1 - l.f5ed);let t1f0: f64 = (l.f793 * t1ef);let t1f1: f64 = (l.f5ed * p.p85);let t1f2: f64 = (t1f0 / t1f1);let t1f3: f64 = (t1ee + t1f2);let t1f4: f64 = (l.f645 * t1f3);let t1f5: f64 = (-230.25850929940458);let t1f6: f64 = if t1f4 < t1f5 { 1.0 } else { 0.0 };l.f4d9 = t1f6;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d7 == 0.0)) && (l.f4d9 != 0.0)) {let t1f7: f64 = (-230.25850929940458);let t1f8: f64 = (l.f7b1 / l.f5f1);let t1f9: f64 = (l.f5f1 - l.f5ed);let t1fa: f64 = (l.f793 * t1f9);let t1fb: f64 = (l.f5ed * p.p85);let t1fc: f64 = (t1fa / t1fb);let t1fd: f64 = (t1f8 + t1fc);let t1fe: f64 = (l.f645 * t1fd);let t1ff: f64 = (t1f7 - t1fe);let t200: f64 = (-230.25850929940458);let t201: f64 = (l.f7b1 / l.f5f1);let t202: f64 = (l.f5f1 - l.f5ed);let t203: f64 = (l.f793 * t202);let t204: f64 = (l.f5ed * p.p85);let t205: f64 = (t203 / t204);let t206: f64 = (t201 + t205);let t207: f64 = (l.f645 * t206);let t208: f64 = (t200 - t207);let t209: f64 = (-230.25850929940458);let t20a: f64 = (l.f7b1 / l.f5f1);let t20b: f64 = (l.f5f1 - l.f5ed);let t20c: f64 = (l.f793 * t20b);let t20d: f64 = (l.f5ed * p.p85);let t20e: f64 = (t20c / t20d);let t20f: f64 = (t20a + t20e);let t210: f64 = (l.f645 * t20f);let t211: f64 = (t209 - t210);let t212: f64 = (t211 * 0.3333333333333333);let t213: f64 = (1.0 + t212);let t214: f64 = (t208 * t213);let t215: f64 = (0.5 * t214);let t216: f64 = (1.0 + t215);let t217: f64 = (t1ff * t216);let t218: f64 = (1.0 + t217);let t219: f64 = (1e-100 / t218);(l.f8e, l.f8f, l.f90, ) = (t219, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1fb) - (t1fa * (l.f5ee * p.p85))) / (t1fb * t1fb))))) * t216) + (t1ff * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t204) - (t203 * (l.f5ee * p.p85))) / (t204 * t204))))) * t213) + (t208 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t20d) - (t20c * (l.f5ee * p.p85))) / (t20d * t20d))))) * 0.3333333333333333))))))) / (t218 * t218))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1fb) - (t1fa * (l.f5ef * p.p85))) / (t1fb * t1fb))))) * t216) + (t1ff * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t204) - (t203 * (l.f5ef * p.p85))) / (t204 * t204))))) * t213) + (t208 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t20d) - (t20c * (l.f5ef * p.p85))) / (t20d * t20d))))) * 0.3333333333333333))))))) / (t218 * t218))), );}
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) && (l.f4d7 == 0.0)) && (l.f4d9 == 0.0)) {let t21a: f64 = (l.f7b1 / l.f5f1);let t21b: f64 = (l.f5f1 - l.f5ed);let t21c: f64 = (l.f793 * t21b);let t21d: f64 = (l.f5ed * p.p85);let t21e: f64 = (t21c / t21d);let t21f: f64 = (t21a + t21e);let t220: f64 = (l.f645 * t21f);let t221: f64 = (t220 - 230.25850929940458);let t222: f64 = (l.f7b1 / l.f5f1);let t223: f64 = (l.f5f1 - l.f5ed);let t224: f64 = (l.f793 * t223);let t225: f64 = (l.f5ed * p.p85);let t226: f64 = (t224 / t225);let t227: f64 = (t222 + t226);let t228: f64 = (l.f645 * t227);let t229: f64 = (t228 - 230.25850929940458);let t22a: f64 = (l.f7b1 / l.f5f1);let t22b: f64 = (l.f5f1 - l.f5ed);let t22c: f64 = (l.f793 * t22b);let t22d: f64 = (l.f5ed * p.p85);let t22e: f64 = (t22c / t22d);let t22f: f64 = (t22a + t22e);let t230: f64 = (l.f645 * t22f);let t231: f64 = (t230 - 230.25850929940458);let t232: f64 = (t231 * 0.3333333333333333);let t233: f64 = (1.0 + t232);let t234: f64 = (t229 * t233);let t235: f64 = (0.5 * t234);let t236: f64 = (1.0 + t235);let t237: f64 = (t221 * t236);let t238: f64 = (1.0 + t237);let t239: f64 = (1e100 * t238);(l.f8e, l.f8f, l.f90, ) = (t239, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t21d) - (t21c * (l.f5ee * p.p85))) / (t21d * t21d)))) * t236) + (t221 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t225) - (t224 * (l.f5ee * p.p85))) / (t225 * t225)))) * t233) + (t229 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t22d) - (t22c * (l.f5ee * p.p85))) / (t22d * t22d)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t21d) - (t21c * (l.f5ef * p.p85))) / (t21d * t21d)))) * t236) + (t221 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t225) - (t224 * (l.f5ef * p.p85))) / (t225 * t225)))) * t233) + (t229 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t22d) - (t22c * (l.f5ef * p.p85))) / (t22d * t22d)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad == 0.0)) {let t23a: f64 = (l.f7b1 * l.f5b);let t23b: f64 = (l.f5f1 - t23a);let t23c: f64 = (l.f5f1 * l.f5f1);let t23d: f64 = (t23b / t23c);let t23e: f64 = (l.f793 * l.f5b);let t23f: f64 = (l.f5ed * p.p85);let t240: f64 = (t23e / t23f);let t241: f64 = (t23d + t240);let t242: f64 = (l.f645 * t241);(l.f61, l.f62, l.f63, ) = (t242, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t23c) - (t23b * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t23c * t23c)) + ((((l.f793 * l.f5c) * t23f) - (t23e * (l.f5ee * p.p85))) / (t23f * t23f)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t23c) - (t23b * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t23c * t23c)) + ((((l.f793 * l.f5d) * t23f) - (t23e * (l.f5ef * p.p85))) / (t23f * t23f)))), );let t243: f64 = (l.f737 - l.f7b1);let t244: f64 = (t243 * l.f61);let t245: f64 = (1.0 + t244);let t246: f64 = (t245 * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t246, (((t243 * l.f62) * l.f8e) + (t245 * l.f8f)), (((t243 * l.f63) * l.f8e) + (t245 * l.f90)), );}
        if ((l.f29a != 0.0) && (l.f4ab != 0.0)) {let t247: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t247, l.f537, l.f538, );let t248: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t248, l.f53f, l.f540, );let t249: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t249, l.f53b, l.f53c, );let t24a: f64 = (1.0 / l.f825);l.f817 = t24a;}
        let t24b: f64 = if l.f737 > 0.0 { 1.0 } else { 0.0 };l.f4db = t24b;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4db != 0.0)) {let t24c: f64 = (2.0 + l.f817);let t24d: f64 = (l.f817 + 1.0);let t24e: f64 = (l.f817 + 3.0);let t24f: f64 = (t24d * t24e);let t250: f64 = (t24f).sqrt();let t251: f64 = (t24c + t250);let t252: f64 = (t251).ln();let t253: f64 = (l.f643 * t252);let t254: f64 = (2.0 * t253);l.f714 = t254;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4db == 0.0)) {let t255: f64 = (-l.f737);let t256: f64 = (2.0 * l.f825);let t257: f64 = (t256 + 1.0);let t258: f64 = (1.0 + l.f825);let t259: f64 = (3.0 * l.f825);let t25a: f64 = (1.0 + t259);let t25b: f64 = (t258 * t25a);let t25c: f64 = (t25b).sqrt();let t25d: f64 = (t257 + t25c);let t25e: f64 = (t25d).ln();let t25f: f64 = (l.f643 * t25e);let t260: f64 = (2.0 * t25f);let t261: f64 = (t255 + t260);l.f714 = t261;}
        if ((l.f29a != 0.0) && (l.f4ab != 0.0)) {let t262: f64 = (l.f76f - l.f714);l.f79c = t262;let t263: f64 = (l.f737 + l.f79c);let t264: f64 = (l.f737 - l.f79c);let t265: f64 = (l.f737 - l.f79c);let t266: f64 = (t264 * t265);let t267: f64 = (4.0 * l.f643);let t268: f64 = (t267 * l.f643);let t269: f64 = (t266 + t268);let t26a: f64 = (t269).sqrt();let t26b: f64 = (t263 - t26a);let t26c: f64 = (0.5 * t26b);l.f7a2 = t26c;let t26d: f64 = (l.f737 + l.f755);let t26e: f64 = (l.f737 - l.f755);let t26f: f64 = (l.f737 - l.f755);let t270: f64 = (t26e * t26f);let t271: f64 = (4.0 * l.f647);let t272: f64 = (t271 * l.f647);let t273: f64 = (t270 + t272);let t274: f64 = (t273).sqrt();let t275: f64 = (t26d - t274);let t276: f64 = (0.5 * t275);l.f750 = t276;let t277: f64 = l.f737;let t278: f64 = l.f737;let t279: f64 = l.f737;let t27a: f64 = (t278 * t279);let t27b: f64 = (4.0 * 1e-6);let t27c: f64 = (t27b * 1e-6);let t27d: f64 = (t27a + t27c);let t27e: f64 = (t27d).sqrt();let t27f: f64 = (t277 - t27e);let t280: f64 = (0.5 * t27f);l.f74a = t280;}
        if ((l.f29a != 0.0) && (l.f4ab == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f714 = 0.0;l.f796 = 0.0;l.f825 = 0.0;l.f7a2 = 0.0;l.f750 = 0.0;l.f74a = 0.0;}
        let t281: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f4dd = t281;
        if ((l.f29a != 0.0) && (l.f4dd != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );}
        let t282: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.fdd = t282;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdd != 0.0)) {let t283: f64 = (l.f796 * l.f769);let t284: f64 = (1.0 - t283);let t285: f64 = (t284).sqrt();l.f6fc = t285;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdd == 0.0)) {let t286: f64 = (l.f796 * l.f769);let t287: f64 = (1.0 - t286);let t288: f64 = (t287).powf(l.f60b);l.f6fc = t288;}
        if ((l.f29a != 0.0) && (l.f4dd == 0.0)) {let t289: f64 = (1.0 - l.f6fc);let t28a: f64 = (l.f69e * t289);let t28b: f64 = (l.f737 - l.f796);let t28c: f64 = (l.f698 * t28b);let t28d: f64 = (t28a + t28c);(l.f68c, l.f68d, l.f68e, ) = (t28d, 0.0, 0.0, );let t28e: f64 = (l.f542 * l.f536);(l.f52f, l.f530, l.f531, ) = (t28e, (l.f542 * l.f537), (l.f542 * l.f538), );}
        let t28f: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.fdf = t28f;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf != 0.0)) {l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) {let t290: f64 = (l.f75d - l.f7a2);l.f758 = t290;let t291: f64 = (l.f714 / l.f758);let t292: f64 = (1.0 - t291);let t293: f64 = (t292).sqrt();let t294: f64 = (1.0 - t293);l.f7ef = t294;}
        let t295: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.fe1 = t295;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) && (l.fe1 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) && (l.fe1 == 0.0)) {let t296: f64 = (l.f7ef * l.f7ef);let t297: f64 = (l.f7ef).ln();let t298: f64 = (t296 * t297);let t299: f64 = (1.0 - l.f7ef);let t29a: f64 = (t298 / t299);let t29b: f64 = (t29a + l.f7ef);let t29c: f64 = (2.0 * l.f623);let t29d: f64 = (1.0 - t29c);let t29e: f64 = (t29b * t29d);l.f66 = t29e;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) {let t29f: f64 = (l.f7ef + l.f66);l.f7e9 = t29f;}
        let t2a0: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.fe3 = t2a0;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) && (l.fe3 != 0.0)) {let t2a1: f64 = (l.f758 * l.f773);let t2a2: f64 = (t2a1).sqrt();l.f6fc = t2a2;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) && (l.fe3 == 0.0)) {let t2a3: f64 = (l.f758 * l.f773);let t2a4: f64 = (t2a3).powf(l.f623);l.f6fc = t2a4;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fdf == 0.0)) {let t2a5: f64 = (l.f7d6 * l.f6fc);l.f7d1 = t2a5;let t2a6: f64 = (l.f825 - 1.0);let t2a7: f64 = (t2a6 * l.f7d1);let t2a8: f64 = (l.fc9 * t2a7);l.f9 = t2a8;let t2a9: f64 = (l.f9 * l.f7e9);let t2aa: f64 = (l.f39 * t2a9);l.f593 = t2aa;}
        let t2ab: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.fe5 = t2ab;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t2ac: f64 = (l.f7d1 * l.f60b);let t2ad: f64 = (t2ac / l.f758);let t2ae: f64 = (l.f1e * t2ad);l.f19 = t2ae;let t2af: f64 = (0.666666666666667 * l.fe);let t2b0: f64 = (t2af / l.f19);l.f71a = t2b0;let t2b1: f64 = (l.f71a * l.f71a);l.f72c = t2b1;let t2b2: f64 = (l.f72c * l.f72c);let t2b3: f64 = (l.f72c * l.f72c);let t2b4: f64 = (t2b3 + 1.0);let t2b5: f64 = (t2b2 / t2b4);let t2b6: f64 = (t2b5).sqrt();l.f726 = t2b6;let t2b7: f64 = (l.f726).abs();let t2b8: f64 = (t2b7).sqrt();l.f6c1 = t2b8;let t2b9: f64 = (l.f726 * l.f6c1);l.f732 = t2b9;}
        let t2ba: f64 = (-l.f623);let t2bb: f64 = (t2ba * l.f611);let t2bc: f64 = (-1.0);let t2bd: f64 = if t2bb == t2bc { 1.0 } else { 0.0 };l.fe7 = t2bd;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fe7 != 0.0)) {let t2be: f64 = (l.f19 * l.f732);let t2bf: f64 = (1.0 + t2be);let t2c0: f64 = (1.0 / t2bf);l.f7e3 = t2c0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fe7 == 0.0)) {let t2c1: f64 = (l.f19 * l.f732);let t2c2: f64 = (1.0 + t2c1);let t2c3: f64 = (-l.f623);let t2c4: f64 = (t2c3 * l.f611);let t2c5: f64 = (t2c2).powf(t2c4);l.f7e3 = t2c5;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t2c6: f64 = (l.f7e9 * l.f7e3);let t2c7: f64 = (l.f7e9 + l.f7e3);let t2c8: f64 = (t2c6 / t2c7);l.f7f5 = t2c8;let t2c9: f64 = (l.f19 / l.f6c1);let t2ca: f64 = (0.375 * t2c9);let t2cb: f64 = (t2ca).sqrt();l.f5a8 = t2cb;let t2cc: f64 = (l.f71a * l.f6c1);let t2cd: f64 = (2.0 * t2cc);let t2ce: f64 = (t2cd - l.f726);l.f5b4 = t2ce;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t2cf: f64 = (l.fe * l.f71a);let t2d0: f64 = (t2cf * l.f6c1);let t2d1: f64 = (l.fe * l.f726);let t2d2: f64 = (t2d0 - t2d1);let t2d3: f64 = (l.f19 * l.f732);let t2d4: f64 = (0.5 * t2d3);let t2d5: f64 = (t2d2 + t2d4);l.f5d4 = t2d5;let t2d6: f64 = (l.f5b4 - 1.0);let t2d7: f64 = (t2d6 * l.f5a8);l.f7fb = t2d7;let t2d8: f64 = (l.f7fb * l.f7fb);l.f811 = t2d8;}
        let t2d9: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.fe9 = t2d9;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fe9 != 0.0)) {let t2da: f64 = (l.f62b * l.f7fb);let t2db: f64 = (1.0 + t2da);let t2dc: f64 = (1.0 / t2db);l.f6e2 = t2dc;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fe9 == 0.0)) {let t2dd: f64 = (l.f62b * l.f7fb);let t2de: f64 = (1.0 - t2dd);let t2df: f64 = (1.0 / t2de);l.f6e2 = t2df;}
        let t2e0: f64 = (-l.f811);let t2e1: f64 = (t2e0 + l.f5d4);let t2e2: f64 = (-230.25850929940458);let t2e3: f64 = if t2e1 > t2e2 { 1.0 } else { 0.0 };l.feb = t2e3;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.feb != 0.0)) {let t2e4: f64 = (-l.f811);let t2e5: f64 = (t2e4 + l.f5d4);let t2e6: f64 = (t2e5).exp();l.f6fc = t2e6;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.feb == 0.0)) {let t2e7: f64 = (-230.25850929940458);let t2e8: f64 = (-l.f811);let t2e9: f64 = (t2e8 + l.f5d4);let t2ea: f64 = (t2e7 - t2e9);let t2eb: f64 = (-230.25850929940458);let t2ec: f64 = (-l.f811);let t2ed: f64 = (t2ec + l.f5d4);let t2ee: f64 = (t2eb - t2ed);let t2ef: f64 = (-230.25850929940458);let t2f0: f64 = (-l.f811);let t2f1: f64 = (t2f0 + l.f5d4);let t2f2: f64 = (t2ef - t2f1);let t2f3: f64 = (t2f2 * 0.3333333333333333);let t2f4: f64 = (1.0 + t2f3);let t2f5: f64 = (t2ee * t2f4);let t2f6: f64 = (0.5 * t2f5);let t2f7: f64 = (1.0 + t2f6);let t2f8: f64 = (t2ea * t2f7);let t2f9: f64 = (1.0 + t2f8);let t2fa: f64 = (1e-100 / t2f9);l.f6fc = t2fa;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t2fb: f64 = (0.29214664 * l.f6e2);let t2fc: f64 = (l.f6e2 * l.f6e2);let t2fd: f64 = (l.f16 * t2fc);let t2fe: f64 = (t2fb + t2fd);let t2ff: f64 = (l.f6e2 * l.f6e2);let t300: f64 = (t2ff * l.f6e2);let t301: f64 = (l.f2a * t300);let t302: f64 = (t2fe + t301);let t303: f64 = (t302 * l.f6fc);l.f6e = t303;}
        let t304: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.fed = t304;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fed != 0.0)) {l.f74 = l.f6e;}
        let t305: f64 = (-230.25850929940458);let t306: f64 = if l.f5d4 > t305 { 1.0 } else { 0.0 };l.fef = t306;
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fed == 0.0)) && (l.fef != 0.0)) {let t307: f64 = (l.f5d4).exp();l.f6fc = t307;}
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fed == 0.0)) && (l.fef == 0.0)) {let t308: f64 = (-230.25850929940458);let t309: f64 = (t308 - l.f5d4);let t30a: f64 = (-230.25850929940458);let t30b: f64 = (t30a - l.f5d4);let t30c: f64 = (-230.25850929940458);let t30d: f64 = (t30c - l.f5d4);let t30e: f64 = (t30d * 0.3333333333333333);let t30f: f64 = (1.0 + t30e);let t310: f64 = (t30b * t30f);let t311: f64 = (0.5 * t310);let t312: f64 = (1.0 + t311);let t313: f64 = (t309 * t312);let t314: f64 = (1.0 + t313);let t315: f64 = (1e-100 / t314);l.f6fc = t315;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fed == 0.0)) {let t316: f64 = (2.0 * l.f6fc);let t317: f64 = (t316 - l.f6e);l.f74 = t317;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t318: f64 = (1.772453850905516 * 0.5);let t319: f64 = (l.fe * l.f74);let t31a: f64 = (t319 / l.f5a8);let t31b: f64 = (t318 * t31a);l.fd6 = t31b;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t31c: f64 = (l.f9 * l.fd6);let t31d: f64 = (t31c * l.f7f5);let t31e: f64 = (l.f3f * t31d);l.f599 = t31e;}
        let t31f: f64 = if l.f24 == 0.0 { 1.0 } else { 0.0 };l.ff3 = t31f;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 != 0.0)) {l.f529 = 0.0;}
        let t320: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.ff5 = t320;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff5 != 0.0)) {let t321: f64 = (l.f771 - l.f750);let t322: f64 = (t321 * l.f773);let t323: f64 = (t322).sqrt();l.f6fc = t323;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff5 == 0.0)) {let t324: f64 = (l.f771 - l.f750);let t325: f64 = (t324 * l.f773);let t326: f64 = (t325).powf(l.f623);l.f6fc = t326;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) {let t327: f64 = (l.f771 - l.f750);let t328: f64 = (t327 * l.f7da);let t329: f64 = (t328 / l.f6fc);let t32a: f64 = (l.f611 * t329);l.fb6 = t32a;}
        let t32b: f64 = (-l.fa1);let t32c: f64 = (t32b / l.fb6);let t32d: f64 = (t32c).abs();let t32e: f64 = if t32d < 230.25850929940458 { 1.0 } else { 0.0 };l.ff7 = t32e;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff7 != 0.0)) {let t32f: f64 = (-l.fa1);let t330: f64 = (t32f / l.fb6);let t331: f64 = (t330).exp();l.f6fc = t331;}
        let t332: f64 = (-l.fa1);let t333: f64 = (t332 / l.fb6);let t334: f64 = (-230.25850929940458);let t335: f64 = if t333 < t334 { 1.0 } else { 0.0 };l.ff9 = t335;
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff7 == 0.0)) && (l.ff9 != 0.0)) {let t336: f64 = (-230.25850929940458);let t337: f64 = (-l.fa1);let t338: f64 = (t337 / l.fb6);let t339: f64 = (t336 - t338);let t33a: f64 = (-230.25850929940458);let t33b: f64 = (-l.fa1);let t33c: f64 = (t33b / l.fb6);let t33d: f64 = (t33a - t33c);let t33e: f64 = (-230.25850929940458);let t33f: f64 = (-l.fa1);let t340: f64 = (t33f / l.fb6);let t341: f64 = (t33e - t340);let t342: f64 = (t341 * 0.3333333333333333);let t343: f64 = (1.0 + t342);let t344: f64 = (t33d * t343);let t345: f64 = (0.5 * t344);let t346: f64 = (1.0 + t345);let t347: f64 = (t339 * t346);let t348: f64 = (1.0 + t347);let t349: f64 = (1e-100 / t348);l.f6fc = t349;}
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff7 == 0.0)) && (l.ff9 == 0.0)) {let t34a: f64 = (-l.fa1);let t34b: f64 = (t34a / l.fb6);let t34c: f64 = (t34b - 230.25850929940458);let t34d: f64 = (-l.fa1);let t34e: f64 = (t34d / l.fb6);let t34f: f64 = (t34e - 230.25850929940458);let t350: f64 = (-l.fa1);let t351: f64 = (t350 / l.fb6);let t352: f64 = (t351 - 230.25850929940458);let t353: f64 = (t352 * 0.3333333333333333);let t354: f64 = (1.0 + t353);let t355: f64 = (t34f * t354);let t356: f64 = (0.5 * t355);let t357: f64 = (1.0 + t356);let t358: f64 = (t34c * t357);let t359: f64 = (1.0 + t358);let t35a: f64 = (1e100 * t359);l.f6fc = t35a;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) {let t35b: f64 = (l.f737 * l.fb6);let t35c: f64 = (t35b * l.fb6);let t35d: f64 = (t35c * l.f6fc);let t35e: f64 = (l.f24 * t35d);l.f529 = t35e;}
        let t35f: f64 = if ((l.f783 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.ffb = t35f;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb != 0.0)) {l.fae = 1.0;}
        let t360: f64 = (-l.f2);let t361: f64 = (t360 * l.f783);let t362: f64 = if l.f74a > t361 { 1.0 } else { 0.0 };l.ffd = t362;let t363: f64 = if l.f625 == 4.0 { 1.0 } else { 0.0 };l.fff = t363;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb == 0.0)) && (l.ffd != 0.0)) && (l.fff != 0.0)) {let t364: f64 = (l.f74a * l.f787);let t365: f64 = (t364).abs();let t366: f64 = (l.f74a * l.f787);let t367: f64 = (t366).abs();let t368: f64 = (t365 * t367);let t369: f64 = (l.f74a * l.f787);let t36a: f64 = (t369).abs();let t36b: f64 = (t368 * t36a);let t36c: f64 = (l.f74a * l.f787);let t36d: f64 = (t36c).abs();let t36e: f64 = (t36b * t36d);l.f6fc = t36e;}
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb == 0.0)) && (l.ffd != 0.0)) && (l.fff == 0.0)) {let t36f: f64 = (l.f74a * l.f787);let t370: f64 = (t36f).abs();let t371: f64 = (t370).powf(l.f625);l.f6fc = t371;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb == 0.0)) && (l.ffd != 0.0)) {let t372: f64 = (1.0 - l.f6fc);let t373: f64 = (1.0 / t372);l.fae = t373;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb == 0.0)) && (l.ffd == 0.0)) {let t374: f64 = (l.f2 * l.f783);let t375: f64 = (l.f74a + t374);let t376: f64 = (t375 * l.f6ba);let t377: f64 = (l.fc3 + t376);l.fae = t377;}
        if ((l.f29a != 0.0) && (l.f4dd == 0.0)) {let t378: f64 = (l.f52f + l.f593);let t379: f64 = (t378 + l.f599);let t37a: f64 = (t379 + l.f529);let t37b: f64 = (t37a * l.fae);(l.f562, l.f563, l.f564, ) = (t37b, (l.f530 * l.fae), (l.f531 * l.fae), );let t37c: f64 = (l.f593 + l.f599);let t37d: f64 = (t37c + l.f529);let t37e: f64 = (t37d * l.fae);(l.f552, l.f553, l.f554, ) = (t37e, 0.0, 0.0, );}
        let t37f: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f101 = t37f;
        if ((l.f29a != 0.0) && (l.f101 != 0.0)) {(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );}
        let t380: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f103 = t380;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f103 != 0.0)) {let t381: f64 = (l.f796 * l.f76d);let t382: f64 = (1.0 - t381);let t383: f64 = (t382).sqrt();l.f6fc = t383;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f103 == 0.0)) {let t384: f64 = (l.f796 * l.f76d);let t385: f64 = (1.0 - t384);let t386: f64 = (t385).powf(l.f60f);l.f6fc = t386;}
        if ((l.f29a != 0.0) && (l.f101 == 0.0)) {let t387: f64 = (1.0 - l.f6fc);let t388: f64 = (l.f6a2 * t387);let t389: f64 = (l.f737 - l.f796);let t38a: f64 = (l.f69c * t389);let t38b: f64 = (t388 + t38a);(l.f694, l.f695, l.f696, ) = (t38b, 0.0, 0.0, );let t38c: f64 = (l.f54c * l.f53e);(l.f52f, l.f530, l.f531, ) = (t38c, (l.f54c * l.f53f), (l.f54c * l.f540), );}
        let t38d: f64 = if ((l.f3d == 0.0) && (l.f43 == 0.0)) { 1.0 } else { 0.0 };l.f105 = t38d;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) {let t38e: f64 = (l.f77d - l.f7a2);l.f758 = t38e;let t38f: f64 = (l.f714 / l.f758);let t390: f64 = (1.0 - t38f);let t391: f64 = (t390).sqrt();let t392: f64 = (1.0 - t391);l.f7ef = t392;}
        let t393: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f109 = t393;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) && (l.f109 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) && (l.f109 == 0.0)) {let t394: f64 = (l.f7ef * l.f7ef);let t395: f64 = (l.f7ef).ln();let t396: f64 = (t394 * t395);let t397: f64 = (1.0 - l.f7ef);let t398: f64 = (t396 / t397);let t399: f64 = (t398 + l.f7ef);let t39a: f64 = (2.0 * l.f653);let t39b: f64 = (1.0 - t39a);let t39c: f64 = (t399 * t39b);l.f66 = t39c;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) {let t39d: f64 = (l.f7ef + l.f66);l.f7e9 = t39d;}
        let t39e: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f10b = t39e;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) && (l.f10b != 0.0)) {let t39f: f64 = (l.f758 * l.f77b);let t3a0: f64 = (t39f).sqrt();l.f6fc = t3a0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) && (l.f10b == 0.0)) {let t3a1: f64 = (l.f758 * l.f77b);let t3a2: f64 = (t3a1).powf(l.f653);l.f6fc = t3a2;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) {let t3a3: f64 = (l.f7e0 * l.f6fc);l.f7d1 = t3a3;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) {let t3a4: f64 = (l.f825 - 1.0);let t3a5: f64 = (t3a4 * l.f7d1);let t3a6: f64 = (l.fd1 * t3a5);l.f9 = t3a6;let t3a7: f64 = (l.f9 * l.f7e9);let t3a8: f64 = (l.f3d * t3a7);l.f593 = t3a8;}
        let t3a9: f64 = if l.f43 == 0.0 { 1.0 } else { 0.0 };l.f10d = t3a9;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let t3aa: f64 = (l.f7d1 * l.f60f);let t3ab: f64 = (t3aa / l.f758);let t3ac: f64 = (l.f22 * t3ab);l.f19 = t3ac;let t3ad: f64 = (0.666666666666667 * l.f12);let t3ae: f64 = (t3ad / l.f19);l.f71a = t3ae;let t3af: f64 = (l.f71a * l.f71a);l.f72c = t3af;let t3b0: f64 = (l.f72c * l.f72c);let t3b1: f64 = (l.f72c * l.f72c);let t3b2: f64 = (t3b1 + 1.0);let t3b3: f64 = (t3b0 / t3b2);let t3b4: f64 = (t3b3).sqrt();l.f726 = t3b4;let t3b5: f64 = (l.f726).abs();let t3b6: f64 = (t3b5).sqrt();l.f6c1 = t3b6;let t3b7: f64 = (l.f726 * l.f6c1);l.f732 = t3b7;}
        let t3b8: f64 = (-l.f653);let t3b9: f64 = (t3b8 * l.f615);let t3ba: f64 = (-1.0);let t3bb: f64 = if t3b9 == t3ba { 1.0 } else { 0.0 };l.f10f = t3bb;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f10f != 0.0)) {let t3bc: f64 = (l.f19 * l.f732);let t3bd: f64 = (1.0 + t3bc);let t3be: f64 = (1.0 / t3bd);l.f7e3 = t3be;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f10f == 0.0)) {let t3bf: f64 = (l.f19 * l.f732);let t3c0: f64 = (1.0 + t3bf);let t3c1: f64 = (-l.f653);let t3c2: f64 = (t3c1 * l.f615);let t3c3: f64 = (t3c0).powf(t3c2);l.f7e3 = t3c3;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let t3c4: f64 = (l.f7e9 * l.f7e3);let t3c5: f64 = (l.f7e9 + l.f7e3);let t3c6: f64 = (t3c4 / t3c5);l.f7f5 = t3c6;let t3c7: f64 = (l.f19 / l.f6c1);let t3c8: f64 = (0.375 * t3c7);let t3c9: f64 = (t3c8).sqrt();l.f5a8 = t3c9;let t3ca: f64 = (l.f71a * l.f6c1);let t3cb: f64 = (2.0 * t3ca);let t3cc: f64 = (t3cb - l.f726);l.f5b4 = t3cc;let t3cd: f64 = (l.f12 * l.f71a);let t3ce: f64 = (t3cd * l.f6c1);let t3cf: f64 = (l.f12 * l.f726);let t3d0: f64 = (t3ce - t3cf);let t3d1: f64 = (l.f19 * l.f732);let t3d2: f64 = (0.5 * t3d1);let t3d3: f64 = (t3d0 + t3d2);l.f5d4 = t3d3;let t3d4: f64 = (l.f5b4 - 1.0);let t3d5: f64 = (t3d4 * l.f5a8);l.f7fb = t3d5;let t3d6: f64 = (l.f7fb * l.f7fb);l.f811 = t3d6;}
        let t3d7: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f111 = t3d7;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f111 != 0.0)) {let t3d8: f64 = (l.f62b * l.f7fb);let t3d9: f64 = (1.0 + t3d8);let t3da: f64 = (1.0 / t3d9);l.f6e2 = t3da;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f111 == 0.0)) {let t3db: f64 = (l.f62b * l.f7fb);let t3dc: f64 = (1.0 - t3db);let t3dd: f64 = (1.0 / t3dc);l.f6e2 = t3dd;}
        let t3de: f64 = (-l.f811);let t3df: f64 = (t3de + l.f5d4);let t3e0: f64 = (-230.25850929940458);let t3e1: f64 = if t3df > t3e0 { 1.0 } else { 0.0 };l.f113 = t3e1;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f113 != 0.0)) {let t3e2: f64 = (-l.f811);let t3e3: f64 = (t3e2 + l.f5d4);let t3e4: f64 = (t3e3).exp();l.f6fc = t3e4;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f113 == 0.0)) {let t3e5: f64 = (-230.25850929940458);let t3e6: f64 = (-l.f811);let t3e7: f64 = (t3e6 + l.f5d4);let t3e8: f64 = (t3e5 - t3e7);let t3e9: f64 = (-230.25850929940458);let t3ea: f64 = (-l.f811);let t3eb: f64 = (t3ea + l.f5d4);let t3ec: f64 = (t3e9 - t3eb);let t3ed: f64 = (-230.25850929940458);let t3ee: f64 = (-l.f811);let t3ef: f64 = (t3ee + l.f5d4);let t3f0: f64 = (t3ed - t3ef);let t3f1: f64 = (t3f0 * 0.3333333333333333);let t3f2: f64 = (1.0 + t3f1);let t3f3: f64 = (t3ec * t3f2);let t3f4: f64 = (0.5 * t3f3);let t3f5: f64 = (1.0 + t3f4);let t3f6: f64 = (t3e8 * t3f5);let t3f7: f64 = (1.0 + t3f6);let t3f8: f64 = (1e-100 / t3f7);l.f6fc = t3f8;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let t3f9: f64 = (0.29214664 * l.f6e2);let t3fa: f64 = (l.f6e2 * l.f6e2);let t3fb: f64 = (l.f16 * t3fa);let t3fc: f64 = (t3f9 + t3fb);let t3fd: f64 = (l.f6e2 * l.f6e2);let t3fe: f64 = (t3fd * l.f6e2);let t3ff: f64 = (l.f2a * t3fe);let t400: f64 = (t3fc + t3ff);let t401: f64 = (t400 * l.f6fc);l.f6e = t401;}
        let t402: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f115 = t402;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f115 != 0.0)) {l.f74 = l.f6e;}
        let t403: f64 = (-230.25850929940458);let t404: f64 = if l.f5d4 > t403 { 1.0 } else { 0.0 };l.f117 = t404;
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f115 == 0.0)) && (l.f117 != 0.0)) {let t405: f64 = (l.f5d4).exp();l.f6fc = t405;}
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f115 == 0.0)) && (l.f117 == 0.0)) {let t406: f64 = (-230.25850929940458);let t407: f64 = (t406 - l.f5d4);let t408: f64 = (-230.25850929940458);let t409: f64 = (t408 - l.f5d4);let t40a: f64 = (-230.25850929940458);let t40b: f64 = (t40a - l.f5d4);let t40c: f64 = (t40b * 0.3333333333333333);let t40d: f64 = (1.0 + t40c);let t40e: f64 = (t409 * t40d);let t40f: f64 = (0.5 * t40e);let t410: f64 = (1.0 + t40f);let t411: f64 = (t407 * t410);let t412: f64 = (1.0 + t411);let t413: f64 = (1e-100 / t412);l.f6fc = t413;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f115 == 0.0)) {let t414: f64 = (2.0 * l.f6fc);let t415: f64 = (t414 - l.f6e);l.f74 = t415;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let t416: f64 = (1.772453850905516 * 0.5);let t417: f64 = (l.f12 * l.f74);let t418: f64 = (t417 / l.f5a8);let t419: f64 = (t416 * t418);l.fd6 = t419;let t41a: f64 = (l.f9 * l.fd6);let t41b: f64 = (t41a * l.f7f5);let t41c: f64 = (l.f43 * t41b);l.f599 = t41c;}
        let t41d: f64 = if l.f28 == 0.0 { 1.0 } else { 0.0 };l.f119 = t41d;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 != 0.0)) {l.f529 = 0.0;}
        let t41e: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f11b = t41e;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11b != 0.0)) {let t41f: f64 = (l.f779 - l.f750);let t420: f64 = (t41f * l.f77b);let t421: f64 = (t420).sqrt();l.f6fc = t421;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11b == 0.0)) {let t422: f64 = (l.f779 - l.f750);let t423: f64 = (t422 * l.f77b);let t424: f64 = (t423).powf(l.f653);l.f6fc = t424;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) {let t425: f64 = (l.f779 - l.f750);let t426: f64 = (t425 * l.f7de);let t427: f64 = (t426 / l.f6fc);let t428: f64 = (l.f615 * t427);l.fb6 = t428;}
        let t429: f64 = (-l.fab);let t42a: f64 = (t429 / l.fb6);let t42b: f64 = (t42a).abs();let t42c: f64 = if t42b < 230.25850929940458 { 1.0 } else { 0.0 };l.f11f = t42c;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11f != 0.0)) {let t42d: f64 = (-l.fab);let t42e: f64 = (t42d / l.fb6);let t42f: f64 = (t42e).exp();l.f6fc = t42f;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t430: f64 = (-l.fab);let t431: f64 = (t430 / l.fb6);let t432: f64 = (-230.25850929940458);let t433: f64 = if t431 < t432 { 1.0 } else { 0.0 };l.f121 = t433;
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11f == 0.0)) && (l.f121 != 0.0)) {let t434: f64 = (-230.25850929940458);let t435: f64 = (-l.fab);let t436: f64 = (t435 / l.fb6);let t437: f64 = (t434 - t436);let t438: f64 = (-230.25850929940458);let t439: f64 = (-l.fab);let t43a: f64 = (t439 / l.fb6);let t43b: f64 = (t438 - t43a);let t43c: f64 = (-230.25850929940458);let t43d: f64 = (-l.fab);let t43e: f64 = (t43d / l.fb6);let t43f: f64 = (t43c - t43e);let t440: f64 = (t43f * 0.3333333333333333);let t441: f64 = (1.0 + t440);let t442: f64 = (t43b * t441);let t443: f64 = (0.5 * t442);let t444: f64 = (1.0 + t443);let t445: f64 = (t437 * t444);let t446: f64 = (1.0 + t445);let t447: f64 = (1e-100 / t446);l.f6fc = t447;}
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11f == 0.0)) && (l.f121 == 0.0)) {let t448: f64 = (-l.fab);let t449: f64 = (t448 / l.fb6);let t44a: f64 = (t449 - 230.25850929940458);let t44b: f64 = (-l.fab);let t44c: f64 = (t44b / l.fb6);let t44d: f64 = (t44c - 230.25850929940458);let t44e: f64 = (-l.fab);let t44f: f64 = (t44e / l.fb6);let t450: f64 = (t44f - 230.25850929940458);let t451: f64 = (t450 * 0.3333333333333333);let t452: f64 = (1.0 + t451);let t453: f64 = (t44d * t452);let t454: f64 = (0.5 * t453);let t455: f64 = (1.0 + t454);let t456: f64 = (t44a * t455);let t457: f64 = (1.0 + t456);let t458: f64 = (1e100 * t457);l.f6fc = t458;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) {let t459: f64 = (l.f737 * l.fb6);let t45a: f64 = (t459 * l.fb6);let t45b: f64 = (t45a * l.f6fc);let t45c: f64 = (l.f28 * t45b);l.f529 = t45c;}
        let t45d: f64 = if ((l.f78d > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f123 = t45d;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 != 0.0)) {l.fae = 1.0;}
        let t45e: f64 = (-l.f2);let t45f: f64 = (t45e * l.f78d);let t460: f64 = if l.f74a > t45f { 1.0 } else { 0.0 };l.f125 = t460;let t461: f64 = if l.f629 == 4.0 { 1.0 } else { 0.0 };l.f127 = t461;
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 == 0.0)) && (l.f125 != 0.0)) && (l.f127 != 0.0)) {let t462: f64 = (l.f74a * l.f78b);let t463: f64 = (t462).abs();let t464: f64 = (l.f74a * l.f78b);let t465: f64 = (t464).abs();let t466: f64 = (t463 * t465);let t467: f64 = (l.f74a * l.f78b);let t468: f64 = (t467).abs();let t469: f64 = (t466 * t468);let t46a: f64 = (l.f74a * l.f78b);let t46b: f64 = (t46a).abs();let t46c: f64 = (t469 * t46b);l.f6fc = t46c;}
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 == 0.0)) && (l.f125 != 0.0)) && (l.f127 == 0.0)) {let t46d: f64 = (l.f74a * l.f78b);let t46e: f64 = (t46d).abs();let t46f: f64 = (t46e).powf(l.f629);l.f6fc = t46f;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 == 0.0)) && (l.f125 != 0.0)) {let t470: f64 = (1.0 - l.f6fc);let t471: f64 = (1.0 / t470);l.fae = t471;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 == 0.0)) && (l.f125 == 0.0)) {let t472: f64 = (l.f2 * l.f78d);let t473: f64 = (l.f74a + t472);let t474: f64 = (t473 * l.f6be);let t475: f64 = (l.fc7 + t474);l.fae = t475;}
        if ((l.f29a != 0.0) && (l.f101 == 0.0)) {let t476: f64 = (l.f52f + l.f593);let t477: f64 = (t476 + l.f599);let t478: f64 = (t477 + l.f529);let t479: f64 = (t478 * l.fae);(l.f576, l.f577, l.f578, ) = (t479, (l.f530 * l.fae), (l.f531 * l.fae), );let t47a: f64 = (l.f593 + l.f599);let t47b: f64 = (t47a + l.f529);let t47c: f64 = (t47b * l.fae);(l.f55a, l.f55b, l.f55c, ) = (t47c, 0.0, 0.0, );}
        let t47d: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f129 = t47d;
        if ((l.f29a != 0.0) && (l.f129 != 0.0)) {(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );}
    }
}
