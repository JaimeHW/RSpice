#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_177(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) {let t0: f64 = (l.f791 + l.f4e8);(l.f4ee, l.f4f1, l.f4f2, ) = (t0, l.f4eb, l.f4ec, );l.f4f3 = 0.0;let t1: f64 = (l.f791 - l.f4e8);(l.f4f4, l.f4f7, l.f4f8, ) = (t1, (-l.f4eb), (-l.f4ec), );l.f4f9 = 0.0;let t2: f64 = (l.f4f4 * l.f4f4);let t3: f64 = (t2 + l.f4e0);let t4: f64 = (t3).sqrt();(l.f4fa, l.f4fd, l.f4fe, ) = (t4, (((l.f4f7 * l.f4f4) + (l.f4f4 * l.f4f7)) / (2.0 * t4)), (((l.f4f8 * l.f4f4) + (l.f4f4 * l.f4f8)) / (2.0 * t4)), );l.f4ff = 0.0;let t5: f64 = (l.f745 * l.f791);let t6: f64 = (l.f4ee + l.f4fa);let t7: f64 = (t5 / t6);let t8: f64 = (2.0 * t7);(l.f795, l.f798, l.f799, ) = (t8, (2.0 * ((((l.f746 * l.f791) * t6) - (t5 * (l.f4f1 + l.f4fd))) / (t6 * t6))), (2.0 * ((((l.f747 * l.f791) * t6) - (t5 * (l.f4f2 + l.f4fe))) / (t6 * t6))), );l.f79a = 0.0;}
        let t9: f64 = if l.f745 < l.f7b1 { 1.0 } else { 0.0 };l.f3f2 = t9;l.f3f3 = 0.0;let ta: f64 = (l.f745 * l.f645);let tb: f64 = (0.5 * ta);let tc: f64 = (tb).abs();let td: f64 = if tc < 230.25850929940458 { 1.0 } else { 0.0 };l.f3f4 = td;l.f3f5 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f4 != 0.0)) {let te: f64 = (l.f745 * l.f645);let tf: f64 = (0.5 * te);let t10: f64 = (tf).exp();(l.f824, l.f827, l.f828, ) = (t10, (t10 * (0.5 * (l.f746 * l.f645))), (t10 * (0.5 * (l.f747 * l.f645))), );l.f829 = 0.0;}
        let t11: f64 = (l.f745 * l.f645);let t12: f64 = (0.5 * t11);let t13: f64 = (-230.25850929940458);let t14: f64 = if t12 < t13 { 1.0 } else { 0.0 };l.f3f6 = t14;l.f3f7 = 0.0;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f4 == 0.0)) && (l.f3f6 != 0.0)) {let t15: f64 = (-230.25850929940458);let t16: f64 = (l.f745 * l.f645);let t17: f64 = (0.5 * t16);let t18: f64 = (t15 - t17);let t19: f64 = (-230.25850929940458);let t1a: f64 = (l.f745 * l.f645);let t1b: f64 = (0.5 * t1a);let t1c: f64 = (t19 - t1b);let t1d: f64 = (-230.25850929940458);let t1e: f64 = (l.f745 * l.f645);let t1f: f64 = (0.5 * t1e);let t20: f64 = (t1d - t1f);let t21: f64 = (t20 * 0.3333333333333333);let t22: f64 = (1.0 + t21);let t23: f64 = (t1c * t22);let t24: f64 = (0.5 * t23);let t25: f64 = (1.0 + t24);let t26: f64 = (t18 * t25);let t27: f64 = (1.0 + t26);let t28: f64 = (1e-100 / t27);(l.f824, l.f827, l.f828, ) = (t28, (-((1e-100 * (((-(0.5 * (l.f746 * l.f645))) * t25) + (t18 * (0.5 * (((-(0.5 * (l.f746 * l.f645))) * t22) + (t1c * ((-(0.5 * (l.f746 * l.f645))) * 0.3333333333333333))))))) / (t27 * t27))), (-((1e-100 * (((-(0.5 * (l.f747 * l.f645))) * t25) + (t18 * (0.5 * (((-(0.5 * (l.f747 * l.f645))) * t22) + (t1c * ((-(0.5 * (l.f747 * l.f645))) * 0.3333333333333333))))))) / (t27 * t27))), );l.f829 = 0.0;}
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f4 == 0.0)) && (l.f3f6 == 0.0)) {let t29: f64 = (l.f745 * l.f645);let t2a: f64 = (0.5 * t29);let t2b: f64 = (t2a - 230.25850929940458);let t2c: f64 = (l.f745 * l.f645);let t2d: f64 = (0.5 * t2c);let t2e: f64 = (t2d - 230.25850929940458);let t2f: f64 = (l.f745 * l.f645);let t30: f64 = (0.5 * t2f);let t31: f64 = (t30 - 230.25850929940458);let t32: f64 = (t31 * 0.3333333333333333);let t33: f64 = (1.0 + t32);let t34: f64 = (t2e * t33);let t35: f64 = (0.5 * t34);let t36: f64 = (1.0 + t35);let t37: f64 = (t2b * t36);let t38: f64 = (1.0 + t37);let t39: f64 = (1e100 * t38);(l.f824, l.f827, l.f828, ) = (t39, (1e100 * (((0.5 * (l.f746 * l.f645)) * t36) + (t2b * (0.5 * (((0.5 * (l.f746 * l.f645)) * t33) + (t2e * ((0.5 * (l.f746 * l.f645)) * 0.3333333333333333))))))), (1e100 * (((0.5 * (l.f747 * l.f645)) * t36) + (t2b * (0.5 * (((0.5 * (l.f747 * l.f645)) * t33) + (t2e * ((0.5 * (l.f747 * l.f645)) * 0.3333333333333333))))))), );l.f829 = 0.0;}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) {let t3a: f64 = (l.f5eb * l.f5eb);let t3b: f64 = (t3a / l.f5df);l.f64f = t3b;l.f650 = 0.0;let t3c: f64 = (l.f5e5 / l.f645);let t3d: f64 = (l.f5df / l.f64f);let t3e: f64 = (t3d).ln();let t3f: f64 = (t3c * t3e);l.f793 = t3f;l.f794 = 0.0;}
        let t40: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f3f8 = t40;l.f3f9 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t41: f64 = (l.f745 - l.f793);let t42: f64 = (p.p86 * t41);let t43: f64 = (t42 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t43, (p.p86 * l.f746), (p.p86 * l.f747), );l.f604 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_178(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t44: f64 = (p.p86 * l.f793);let t45: f64 = (l.f5e5 - t44);(l.f5ed, l.f5ee, l.f5ef, ) = (t45, 0.0, 0.0, );l.f5f0 = 0.0;let t46: f64 = (p.p85 - l.f601);let t47: f64 = (t46 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t47, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t48: f64 = (4.0 * p.p85);let t49: f64 = (t48 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t49, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {
            let (t4b, t4c, t4d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t4a: f64 = (-l.f6f7);
        (t4a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t4b, t4c, t4d, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t4e: f64 = (l.f6f3 * l.f6f3);let t4f: f64 = (t4e + l.f6f7);let t50: f64 = (t4f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t50, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t50)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t50)), );l.f6fa = 0.0;let t51: f64 = (l.f6f3 + l.f6f7);let t52: f64 = (0.5 * t51);let t53: f64 = (p.p85 - t52);(l.f605, l.f606, l.f607, ) = (t53, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t54: f64 = (l.f605 - l.f5e5);let t55: f64 = (t54 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t55, l.f606, l.f607, );l.f6f6 = 0.0;let t56: f64 = (4.0 * l.f5e5);let t57: f64 = (t56 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t57, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {
            let (t59, t5a, t5b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t58: f64 = (-l.f6f7);
        (t58, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t59, t5a, t5b, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t5c: f64 = (l.f6f3 * l.f6f3);let t5d: f64 = (t5c + l.f6f7);let t5e: f64 = (t5d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t5e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t5e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t5e)), );l.f6fa = 0.0;let t5f: f64 = (l.f6f3 + l.f6f7);let t60: f64 = (0.5 * t5f);let t61: f64 = (l.f5e5 + t60);(l.f5f1, l.f5f2, l.f5f3, ) = (t61, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t62: f64 = (p.p85 - l.f5ed);let t63: f64 = (t62 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t63, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t64: f64 = (4.0 * p.p85);let t65: f64 = (t64 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t65, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {
            let (t67, t68, t69,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t66: f64 = (-l.f6f7);
        (t66, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t67, t68, t69, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t6a: f64 = (l.f6f3 * l.f6f3);let t6b: f64 = (t6a + l.f6f7);let t6c: f64 = (t6b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t6c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t6c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t6c)), );l.f6fa = 0.0;let t6d: f64 = (l.f6f3 + l.f6f7);let t6e: f64 = (0.5 * t6d);let t6f: f64 = (p.p85 - t6e);(l.f5ed, l.f5ee, l.f5ef, ) = (t6f, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t70: f64 = (l.f5ed - l.f5e5);let t71: f64 = (t70 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t71, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t72: f64 = (4.0 * l.f5e5);let t73: f64 = (t72 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t73, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {
            let (t75, t76, t77,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t74: f64 = (-l.f6f7);
        (t74, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t75, t76, t77, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 != 0.0)) {let t78: f64 = (l.f6f3 * l.f6f3);let t79: f64 = (t78 + l.f6f7);let t7a: f64 = (t79).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t7a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t7a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t7a)), );l.f6fa = 0.0;let t7b: f64 = (l.f6f3 + l.f6f7);let t7c: f64 = (0.5 * t7b);let t7d: f64 = (l.f5e5 + t7c);(l.f5ed, l.f5ee, l.f5ef, ) = (t7d, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f8 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );l.f5f4 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_179(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t7e: f64 = (l.f745 / l.f5f1);let t7f: f64 = (l.f5f1 - l.f5ed);let t80: f64 = (l.f793 * t7f);let t81: f64 = (l.f5ed * p.p85);let t82: f64 = (t80 / t81);let t83: f64 = (t7e + t82);let t84: f64 = (l.f645 * t83);let t85: f64 = (t84).abs();let t86: f64 = if t85 < 230.25850929940458 { 1.0 } else { 0.0 };l.f3fa = t86;l.f3fb = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fa != 0.0)) {let t87: f64 = (l.f745 / l.f5f1);let t88: f64 = (l.f5f1 - l.f5ed);let t89: f64 = (l.f793 * t88);let t8a: f64 = (l.f5ed * p.p85);let t8b: f64 = (t89 / t8a);let t8c: f64 = (t87 + t8b);let t8d: f64 = (l.f645 * t8c);let t8e: f64 = (t8d).exp();(l.f536, l.f537, l.f538, ) = (t8e, (t8e * (l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t8a) - (t89 * (l.f5ee * p.p85))) / (t8a * t8a))))), (t8e * (l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t8a) - (t89 * (l.f5ef * p.p85))) / (t8a * t8a))))), );l.f539 = 0.0;}
        let t8f: f64 = (l.f745 / l.f5f1);let t90: f64 = (l.f5f1 - l.f5ed);let t91: f64 = (l.f793 * t90);let t92: f64 = (l.f5ed * p.p85);let t93: f64 = (t91 / t92);let t94: f64 = (t8f + t93);let t95: f64 = (l.f645 * t94);let t96: f64 = (-230.25850929940458);let t97: f64 = if t95 < t96 { 1.0 } else { 0.0 };l.f3fc = t97;l.f3fd = 0.0;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fa == 0.0)) && (l.f3fc != 0.0)) {let t98: f64 = (-230.25850929940458);let t99: f64 = (l.f745 / l.f5f1);let t9a: f64 = (l.f5f1 - l.f5ed);let t9b: f64 = (l.f793 * t9a);let t9c: f64 = (l.f5ed * p.p85);let t9d: f64 = (t9b / t9c);let t9e: f64 = (t99 + t9d);let t9f: f64 = (l.f645 * t9e);let ta0: f64 = (t98 - t9f);let ta1: f64 = (-230.25850929940458);let ta2: f64 = (l.f745 / l.f5f1);let ta3: f64 = (l.f5f1 - l.f5ed);let ta4: f64 = (l.f793 * ta3);let ta5: f64 = (l.f5ed * p.p85);let ta6: f64 = (ta4 / ta5);let ta7: f64 = (ta2 + ta6);let ta8: f64 = (l.f645 * ta7);let ta9: f64 = (ta1 - ta8);let taa: f64 = (-230.25850929940458);let tab: f64 = (l.f745 / l.f5f1);let tac: f64 = (l.f5f1 - l.f5ed);let tad: f64 = (l.f793 * tac);let tae: f64 = (l.f5ed * p.p85);let taf: f64 = (tad / tae);let tb0: f64 = (tab + taf);let tb1: f64 = (l.f645 * tb0);let tb2: f64 = (taa - tb1);let tb3: f64 = (tb2 * 0.3333333333333333);let tb4: f64 = (1.0 + tb3);let tb5: f64 = (ta9 * tb4);let tb6: f64 = (0.5 * tb5);let tb7: f64 = (1.0 + tb6);let tb8: f64 = (ta0 * tb7);let tb9: f64 = (1.0 + tb8);let tba: f64 = (1e-100 / tb9);(l.f536, l.f537, l.f538, ) = (tba, (-((1e-100 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t9c) - (t9b * (l.f5ee * p.p85))) / (t9c * t9c))))) * tb7) + (ta0 * (0.5 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * ta5) - (ta4 * (l.f5ee * p.p85))) / (ta5 * ta5))))) * tb4) + (ta9 * ((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tae) - (tad * (l.f5ee * p.p85))) / (tae * tae))))) * 0.3333333333333333))))))) / (tb9 * tb9))), (-((1e-100 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t9c) - (t9b * (l.f5ef * p.p85))) / (t9c * t9c))))) * tb7) + (ta0 * (0.5 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * ta5) - (ta4 * (l.f5ef * p.p85))) / (ta5 * ta5))))) * tb4) + (ta9 * ((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tae) - (tad * (l.f5ef * p.p85))) / (tae * tae))))) * 0.3333333333333333))))))) / (tb9 * tb9))), );l.f539 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_180(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fa == 0.0)) && (l.f3fc == 0.0)) {let tbb: f64 = (l.f745 / l.f5f1);let tbc: f64 = (l.f5f1 - l.f5ed);let tbd: f64 = (l.f793 * tbc);let tbe: f64 = (l.f5ed * p.p85);let tbf: f64 = (tbd / tbe);let tc0: f64 = (tbb + tbf);let tc1: f64 = (l.f645 * tc0);let tc2: f64 = (tc1 - 230.25850929940458);let tc3: f64 = (l.f745 / l.f5f1);let tc4: f64 = (l.f5f1 - l.f5ed);let tc5: f64 = (l.f793 * tc4);let tc6: f64 = (l.f5ed * p.p85);let tc7: f64 = (tc5 / tc6);let tc8: f64 = (tc3 + tc7);let tc9: f64 = (l.f645 * tc8);let tca: f64 = (tc9 - 230.25850929940458);let tcb: f64 = (l.f745 / l.f5f1);let tcc: f64 = (l.f5f1 - l.f5ed);let tcd: f64 = (l.f793 * tcc);let tce: f64 = (l.f5ed * p.p85);let tcf: f64 = (tcd / tce);let td0: f64 = (tcb + tcf);let td1: f64 = (l.f645 * td0);let td2: f64 = (td1 - 230.25850929940458);let td3: f64 = (td2 * 0.3333333333333333);let td4: f64 = (1.0 + td3);let td5: f64 = (tca * td4);let td6: f64 = (0.5 * td5);let td7: f64 = (1.0 + td6);let td8: f64 = (tc2 * td7);let td9: f64 = (1.0 + td8);let tda: f64 = (1e100 * td9);(l.f536, l.f537, l.f538, ) = (tda, (1e100 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tbe) - (tbd * (l.f5ee * p.p85))) / (tbe * tbe)))) * td7) + (tc2 * (0.5 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tc6) - (tc5 * (l.f5ee * p.p85))) / (tc6 * tc6)))) * td4) + (tca * ((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tce) - (tcd * (l.f5ee * p.p85))) / (tce * tce)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tbe) - (tbd * (l.f5ef * p.p85))) / (tbe * tbe)))) * td7) + (tc2 * (0.5 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tc6) - (tc5 * (l.f5ef * p.p85))) / (tc6 * tc6)))) * td4) + (tca * ((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tce) - (tcd * (l.f5ef * p.p85))) / (tce * tce)))) * 0.3333333333333333))))))), );l.f539 = 0.0;}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) {let tdb: f64 = (l.f5eb * l.f5eb);let tdc: f64 = (tdb / l.f5e3);l.f64f = tdc;l.f650 = 0.0;let tdd: f64 = (l.f5e9 / l.f645);let tde: f64 = (l.f5e3 / l.f64f);let tdf: f64 = (tde).ln();let te0: f64 = (tdd * tdf);l.f793 = te0;l.f794 = 0.0;}
        let te1: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f3fe = te1;l.f3ff = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let te2: f64 = (l.f745 - l.f793);let te3: f64 = (p.p86 * te2);let te4: f64 = (te3 + l.f5e9);(l.f601, l.f602, l.f603, ) = (te4, (p.p86 * l.f746), (p.p86 * l.f747), );l.f604 = 0.0;let te5: f64 = (p.p86 * l.f793);let te6: f64 = (l.f5e9 - te5);(l.f5ed, l.f5ee, l.f5ef, ) = (te6, 0.0, 0.0, );l.f5f0 = 0.0;let te7: f64 = (p.p85 - l.f601);let te8: f64 = (te7 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (te8, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let te9: f64 = (4.0 * p.p85);let tea: f64 = (te9 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tea, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {
            let (tec, ted, tee,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let teb: f64 = (-l.f6f7);
        (teb, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tec, ted, tee, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let tef: f64 = (l.f6f3 * l.f6f3);let tf0: f64 = (tef + l.f6f7);let tf1: f64 = (tf0).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tf1, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tf1)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tf1)), );l.f6fa = 0.0;let tf2: f64 = (l.f6f3 + l.f6f7);let tf3: f64 = (0.5 * tf2);let tf4: f64 = (p.p85 - tf3);(l.f605, l.f606, l.f607, ) = (tf4, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let tf5: f64 = (l.f605 - l.f5e9);let tf6: f64 = (tf5 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tf6, l.f606, l.f607, );l.f6f6 = 0.0;let tf7: f64 = (4.0 * l.f5e9);let tf8: f64 = (tf7 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tf8, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {
            let (tfa, tfb, tfc,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tf9: f64 = (-l.f6f7);
        (tf9, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tfa, tfb, tfc, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let tfd: f64 = (l.f6f3 * l.f6f3);let tfe: f64 = (tfd + l.f6f7);let tff: f64 = (tfe).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tff, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tff)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tff)), );l.f6fa = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_181(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let t100: f64 = (l.f6f3 + l.f6f7);let t101: f64 = (0.5 * t100);let t102: f64 = (l.f5e9 + t101);(l.f5f1, l.f5f2, l.f5f3, ) = (t102, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t103: f64 = (p.p85 - l.f5ed);let t104: f64 = (t103 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t104, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t105: f64 = (4.0 * p.p85);let t106: f64 = (t105 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t106, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {
            let (t108, t109, t10a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t107: f64 = (-l.f6f7);
        (t107, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t108, t109, t10a, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let t10b: f64 = (l.f6f3 * l.f6f3);let t10c: f64 = (t10b + l.f6f7);let t10d: f64 = (t10c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t10d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t10d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t10d)), );l.f6fa = 0.0;let t10e: f64 = (l.f6f3 + l.f6f7);let t10f: f64 = (0.5 * t10e);let t110: f64 = (p.p85 - t10f);(l.f5ed, l.f5ee, l.f5ef, ) = (t110, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t111: f64 = (l.f5ed - l.f5e9);let t112: f64 = (t111 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t112, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t113: f64 = (4.0 * l.f5e9);let t114: f64 = (t113 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t114, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {
            let (t116, t117, t118,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t115: f64 = (-l.f6f7);
        (t115, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t116, t117, t118, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe != 0.0)) {let t119: f64 = (l.f6f3 * l.f6f3);let t11a: f64 = (t119 + l.f6f7);let t11b: f64 = (t11a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t11b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t11b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t11b)), );l.f6fa = 0.0;let t11c: f64 = (l.f6f3 + l.f6f7);let t11d: f64 = (0.5 * t11c);let t11e: f64 = (l.f5e9 + t11d);(l.f5ed, l.f5ee, l.f5ef, ) = (t11e, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3fe == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );l.f5f4 = 0.0;}
        let t11f: f64 = (l.f745 / l.f5f1);let t120: f64 = (l.f5f1 - l.f5ed);let t121: f64 = (l.f793 * t120);let t122: f64 = (l.f5ed * p.p85);let t123: f64 = (t121 / t122);let t124: f64 = (t11f + t123);let t125: f64 = (l.f645 * t124);let t126: f64 = (t125).abs();let t127: f64 = if t126 < 230.25850929940458 { 1.0 } else { 0.0 };l.f400 = t127;l.f401 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f400 != 0.0)) {let t128: f64 = (l.f745 / l.f5f1);let t129: f64 = (l.f5f1 - l.f5ed);let t12a: f64 = (l.f793 * t129);let t12b: f64 = (l.f5ed * p.p85);let t12c: f64 = (t12a / t12b);let t12d: f64 = (t128 + t12c);let t12e: f64 = (l.f645 * t12d);let t12f: f64 = (t12e).exp();(l.f53e, l.f53f, l.f540, ) = (t12f, (t12f * (l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t12b) - (t12a * (l.f5ee * p.p85))) / (t12b * t12b))))), (t12f * (l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t12b) - (t12a * (l.f5ef * p.p85))) / (t12b * t12b))))), );l.f541 = 0.0;}
        let t130: f64 = (l.f745 / l.f5f1);let t131: f64 = (l.f5f1 - l.f5ed);let t132: f64 = (l.f793 * t131);let t133: f64 = (l.f5ed * p.p85);let t134: f64 = (t132 / t133);let t135: f64 = (t130 + t134);let t136: f64 = (l.f645 * t135);let t137: f64 = (-230.25850929940458);let t138: f64 = if t136 < t137 { 1.0 } else { 0.0 };l.f402 = t138;l.f403 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_182(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f400 == 0.0)) && (l.f402 != 0.0)) {let t139: f64 = (-230.25850929940458);let t13a: f64 = (l.f745 / l.f5f1);let t13b: f64 = (l.f5f1 - l.f5ed);let t13c: f64 = (l.f793 * t13b);let t13d: f64 = (l.f5ed * p.p85);let t13e: f64 = (t13c / t13d);let t13f: f64 = (t13a + t13e);let t140: f64 = (l.f645 * t13f);let t141: f64 = (t139 - t140);let t142: f64 = (-230.25850929940458);let t143: f64 = (l.f745 / l.f5f1);let t144: f64 = (l.f5f1 - l.f5ed);let t145: f64 = (l.f793 * t144);let t146: f64 = (l.f5ed * p.p85);let t147: f64 = (t145 / t146);let t148: f64 = (t143 + t147);let t149: f64 = (l.f645 * t148);let t14a: f64 = (t142 - t149);let t14b: f64 = (-230.25850929940458);let t14c: f64 = (l.f745 / l.f5f1);let t14d: f64 = (l.f5f1 - l.f5ed);let t14e: f64 = (l.f793 * t14d);let t14f: f64 = (l.f5ed * p.p85);let t150: f64 = (t14e / t14f);let t151: f64 = (t14c + t150);let t152: f64 = (l.f645 * t151);let t153: f64 = (t14b - t152);let t154: f64 = (t153 * 0.3333333333333333);let t155: f64 = (1.0 + t154);let t156: f64 = (t14a * t155);let t157: f64 = (0.5 * t156);let t158: f64 = (1.0 + t157);let t159: f64 = (t141 * t158);let t15a: f64 = (1.0 + t159);let t15b: f64 = (1e-100 / t15a);(l.f53e, l.f53f, l.f540, ) = (t15b, (-((1e-100 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t13d) - (t13c * (l.f5ee * p.p85))) / (t13d * t13d))))) * t158) + (t141 * (0.5 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t146) - (t145 * (l.f5ee * p.p85))) / (t146 * t146))))) * t155) + (t14a * ((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t14f) - (t14e * (l.f5ee * p.p85))) / (t14f * t14f))))) * 0.3333333333333333))))))) / (t15a * t15a))), (-((1e-100 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t13d) - (t13c * (l.f5ef * p.p85))) / (t13d * t13d))))) * t158) + (t141 * (0.5 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t146) - (t145 * (l.f5ef * p.p85))) / (t146 * t146))))) * t155) + (t14a * ((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t14f) - (t14e * (l.f5ef * p.p85))) / (t14f * t14f))))) * 0.3333333333333333))))))) / (t15a * t15a))), );l.f541 = 0.0;}
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f400 == 0.0)) && (l.f402 == 0.0)) {let t15c: f64 = (l.f745 / l.f5f1);let t15d: f64 = (l.f5f1 - l.f5ed);let t15e: f64 = (l.f793 * t15d);let t15f: f64 = (l.f5ed * p.p85);let t160: f64 = (t15e / t15f);let t161: f64 = (t15c + t160);let t162: f64 = (l.f645 * t161);let t163: f64 = (t162 - 230.25850929940458);let t164: f64 = (l.f745 / l.f5f1);let t165: f64 = (l.f5f1 - l.f5ed);let t166: f64 = (l.f793 * t165);let t167: f64 = (l.f5ed * p.p85);let t168: f64 = (t166 / t167);let t169: f64 = (t164 + t168);let t16a: f64 = (l.f645 * t169);let t16b: f64 = (t16a - 230.25850929940458);let t16c: f64 = (l.f745 / l.f5f1);let t16d: f64 = (l.f5f1 - l.f5ed);let t16e: f64 = (l.f793 * t16d);let t16f: f64 = (l.f5ed * p.p85);let t170: f64 = (t16e / t16f);let t171: f64 = (t16c + t170);let t172: f64 = (l.f645 * t171);let t173: f64 = (t172 - 230.25850929940458);let t174: f64 = (t173 * 0.3333333333333333);let t175: f64 = (1.0 + t174);let t176: f64 = (t16b * t175);let t177: f64 = (0.5 * t176);let t178: f64 = (1.0 + t177);let t179: f64 = (t163 * t178);let t17a: f64 = (1.0 + t179);let t17b: f64 = (1e100 * t17a);(l.f53e, l.f53f, l.f540, ) = (t17b, (1e100 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t15f) - (t15e * (l.f5ee * p.p85))) / (t15f * t15f)))) * t178) + (t163 * (0.5 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t167) - (t166 * (l.f5ee * p.p85))) / (t167 * t167)))) * t175) + (t16b * ((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t16f) - (t16e * (l.f5ee * p.p85))) / (t16f * t16f)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t15f) - (t15e * (l.f5ef * p.p85))) / (t15f * t15f)))) * t178) + (t163 * (0.5 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t167) - (t166 * (l.f5ef * p.p85))) / (t167 * t167)))) * t175) + (t16b * ((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t16f) - (t16e * (l.f5ef * p.p85))) / (t16f * t16f)))) * 0.3333333333333333))))))), );l.f541 = 0.0;}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) {let t17c: f64 = (l.f5eb * l.f5eb);let t17d: f64 = (t17c / l.f5e1);l.f64f = t17d;l.f650 = 0.0;let t17e: f64 = (l.f5e7 / l.f645);let t17f: f64 = (l.f5e1 / l.f64f);let t180: f64 = (t17f).ln();let t181: f64 = (t17e * t180);l.f793 = t181;l.f794 = 0.0;}
        let t182: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f404 = t182;l.f405 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t183: f64 = (l.f745 - l.f793);let t184: f64 = (p.p86 * t183);let t185: f64 = (t184 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t185, (p.p86 * l.f746), (p.p86 * l.f747), );l.f604 = 0.0;let t186: f64 = (p.p86 * l.f793);let t187: f64 = (l.f5e7 - t186);(l.f5ed, l.f5ee, l.f5ef, ) = (t187, 0.0, 0.0, );l.f5f0 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_183(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t188: f64 = (p.p85 - l.f601);let t189: f64 = (t188 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t189, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t18a: f64 = (4.0 * p.p85);let t18b: f64 = (t18a * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t18b, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {
            let (t18d, t18e, t18f,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t18c: f64 = (-l.f6f7);
        (t18c, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t18d, t18e, t18f, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t190: f64 = (l.f6f3 * l.f6f3);let t191: f64 = (t190 + l.f6f7);let t192: f64 = (t191).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t192, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t192)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t192)), );l.f6fa = 0.0;let t193: f64 = (l.f6f3 + l.f6f7);let t194: f64 = (0.5 * t193);let t195: f64 = (p.p85 - t194);(l.f605, l.f606, l.f607, ) = (t195, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t196: f64 = (l.f605 - l.f5e7);let t197: f64 = (t196 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t197, l.f606, l.f607, );l.f6f6 = 0.0;let t198: f64 = (4.0 * l.f5e7);let t199: f64 = (t198 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t199, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {
            let (t19b, t19c, t19d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t19a: f64 = (-l.f6f7);
        (t19a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t19b, t19c, t19d, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t19e: f64 = (l.f6f3 * l.f6f3);let t19f: f64 = (t19e + l.f6f7);let t1a0: f64 = (t19f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1a0, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1a0)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1a0)), );l.f6fa = 0.0;let t1a1: f64 = (l.f6f3 + l.f6f7);let t1a2: f64 = (0.5 * t1a1);let t1a3: f64 = (l.f5e7 + t1a2);(l.f5f1, l.f5f2, l.f5f3, ) = (t1a3, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t1a4: f64 = (p.p85 - l.f5ed);let t1a5: f64 = (t1a4 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1a5, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t1a6: f64 = (4.0 * p.p85);let t1a7: f64 = (t1a6 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1a7, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {
            let (t1a9, t1aa, t1ab,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1a8: f64 = (-l.f6f7);
        (t1a8, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1a9, t1aa, t1ab, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t1ac: f64 = (l.f6f3 * l.f6f3);let t1ad: f64 = (t1ac + l.f6f7);let t1ae: f64 = (t1ad).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1ae, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1ae)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1ae)), );l.f6fa = 0.0;let t1af: f64 = (l.f6f3 + l.f6f7);let t1b0: f64 = (0.5 * t1af);let t1b1: f64 = (p.p85 - t1b0);(l.f5ed, l.f5ee, l.f5ef, ) = (t1b1, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t1b2: f64 = (l.f5ed - l.f5e7);let t1b3: f64 = (t1b2 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1b3, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t1b4: f64 = (4.0 * l.f5e7);let t1b5: f64 = (t1b4 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1b5, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {
            let (t1b7, t1b8, t1b9,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1b6: f64 = (-l.f6f7);
        (t1b6, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1b7, t1b8, t1b9, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 != 0.0)) {let t1ba: f64 = (l.f6f3 * l.f6f3);let t1bb: f64 = (t1ba + l.f6f7);let t1bc: f64 = (t1bb).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1bc, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1bc)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1bc)), );l.f6fa = 0.0;let t1bd: f64 = (l.f6f3 + l.f6f7);let t1be: f64 = (0.5 * t1bd);let t1bf: f64 = (l.f5e7 + t1be);(l.f5ed, l.f5ee, l.f5ef, ) = (t1bf, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f404 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );l.f5f4 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_184(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t1c0: f64 = (l.f745 / l.f5f1);let t1c1: f64 = (l.f5f1 - l.f5ed);let t1c2: f64 = (l.f793 * t1c1);let t1c3: f64 = (l.f5ed * p.p85);let t1c4: f64 = (t1c2 / t1c3);let t1c5: f64 = (t1c0 + t1c4);let t1c6: f64 = (l.f645 * t1c5);let t1c7: f64 = (t1c6).abs();let t1c8: f64 = if t1c7 < 230.25850929940458 { 1.0 } else { 0.0 };l.f406 = t1c8;l.f407 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f406 != 0.0)) {let t1c9: f64 = (l.f745 / l.f5f1);let t1ca: f64 = (l.f5f1 - l.f5ed);let t1cb: f64 = (l.f793 * t1ca);let t1cc: f64 = (l.f5ed * p.p85);let t1cd: f64 = (t1cb / t1cc);let t1ce: f64 = (t1c9 + t1cd);let t1cf: f64 = (l.f645 * t1ce);let t1d0: f64 = (t1cf).exp();(l.f53a, l.f53b, l.f53c, ) = (t1d0, (t1d0 * (l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1cc) - (t1cb * (l.f5ee * p.p85))) / (t1cc * t1cc))))), (t1d0 * (l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1cc) - (t1cb * (l.f5ef * p.p85))) / (t1cc * t1cc))))), );l.f53d = 0.0;}
        let t1d1: f64 = (l.f745 / l.f5f1);let t1d2: f64 = (l.f5f1 - l.f5ed);let t1d3: f64 = (l.f793 * t1d2);let t1d4: f64 = (l.f5ed * p.p85);let t1d5: f64 = (t1d3 / t1d4);let t1d6: f64 = (t1d1 + t1d5);let t1d7: f64 = (l.f645 * t1d6);let t1d8: f64 = (-230.25850929940458);let t1d9: f64 = if t1d7 < t1d8 { 1.0 } else { 0.0 };l.f408 = t1d9;l.f409 = 0.0;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f406 == 0.0)) && (l.f408 != 0.0)) {let t1da: f64 = (-230.25850929940458);let t1db: f64 = (l.f745 / l.f5f1);let t1dc: f64 = (l.f5f1 - l.f5ed);let t1dd: f64 = (l.f793 * t1dc);let t1de: f64 = (l.f5ed * p.p85);let t1df: f64 = (t1dd / t1de);let t1e0: f64 = (t1db + t1df);let t1e1: f64 = (l.f645 * t1e0);let t1e2: f64 = (t1da - t1e1);let t1e3: f64 = (-230.25850929940458);let t1e4: f64 = (l.f745 / l.f5f1);let t1e5: f64 = (l.f5f1 - l.f5ed);let t1e6: f64 = (l.f793 * t1e5);let t1e7: f64 = (l.f5ed * p.p85);let t1e8: f64 = (t1e6 / t1e7);let t1e9: f64 = (t1e4 + t1e8);let t1ea: f64 = (l.f645 * t1e9);let t1eb: f64 = (t1e3 - t1ea);let t1ec: f64 = (-230.25850929940458);let t1ed: f64 = (l.f745 / l.f5f1);let t1ee: f64 = (l.f5f1 - l.f5ed);let t1ef: f64 = (l.f793 * t1ee);let t1f0: f64 = (l.f5ed * p.p85);let t1f1: f64 = (t1ef / t1f0);let t1f2: f64 = (t1ed + t1f1);let t1f3: f64 = (l.f645 * t1f2);let t1f4: f64 = (t1ec - t1f3);let t1f5: f64 = (t1f4 * 0.3333333333333333);let t1f6: f64 = (1.0 + t1f5);let t1f7: f64 = (t1eb * t1f6);let t1f8: f64 = (0.5 * t1f7);let t1f9: f64 = (1.0 + t1f8);let t1fa: f64 = (t1e2 * t1f9);let t1fb: f64 = (1.0 + t1fa);let t1fc: f64 = (1e-100 / t1fb);(l.f53a, l.f53b, l.f53c, ) = (t1fc, (-((1e-100 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1de) - (t1dd * (l.f5ee * p.p85))) / (t1de * t1de))))) * t1f9) + (t1e2 * (0.5 * (((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e7) - (t1e6 * (l.f5ee * p.p85))) / (t1e7 * t1e7))))) * t1f6) + (t1eb * ((-(l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1f0) - (t1ef * (l.f5ee * p.p85))) / (t1f0 * t1f0))))) * 0.3333333333333333))))))) / (t1fb * t1fb))), (-((1e-100 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1de) - (t1dd * (l.f5ef * p.p85))) / (t1de * t1de))))) * t1f9) + (t1e2 * (0.5 * (((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e7) - (t1e6 * (l.f5ef * p.p85))) / (t1e7 * t1e7))))) * t1f6) + (t1eb * ((-(l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1f0) - (t1ef * (l.f5ef * p.p85))) / (t1f0 * t1f0))))) * 0.3333333333333333))))))) / (t1fb * t1fb))), );l.f53d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_185(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f406 == 0.0)) && (l.f408 == 0.0)) {let t1fd: f64 = (l.f745 / l.f5f1);let t1fe: f64 = (l.f5f1 - l.f5ed);let t1ff: f64 = (l.f793 * t1fe);let t200: f64 = (l.f5ed * p.p85);let t201: f64 = (t1ff / t200);let t202: f64 = (t1fd + t201);let t203: f64 = (l.f645 * t202);let t204: f64 = (t203 - 230.25850929940458);let t205: f64 = (l.f745 / l.f5f1);let t206: f64 = (l.f5f1 - l.f5ed);let t207: f64 = (l.f793 * t206);let t208: f64 = (l.f5ed * p.p85);let t209: f64 = (t207 / t208);let t20a: f64 = (t205 + t209);let t20b: f64 = (l.f645 * t20a);let t20c: f64 = (t20b - 230.25850929940458);let t20d: f64 = (l.f745 / l.f5f1);let t20e: f64 = (l.f5f1 - l.f5ed);let t20f: f64 = (l.f793 * t20e);let t210: f64 = (l.f5ed * p.p85);let t211: f64 = (t20f / t210);let t212: f64 = (t20d + t211);let t213: f64 = (l.f645 * t212);let t214: f64 = (t213 - 230.25850929940458);let t215: f64 = (t214 * 0.3333333333333333);let t216: f64 = (1.0 + t215);let t217: f64 = (t20c * t216);let t218: f64 = (0.5 * t217);let t219: f64 = (1.0 + t218);let t21a: f64 = (t204 * t219);let t21b: f64 = (1.0 + t21a);let t21c: f64 = (1e100 * t21b);(l.f53a, l.f53b, l.f53c, ) = (t21c, (1e100 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t200) - (t1ff * (l.f5ee * p.p85))) / (t200 * t200)))) * t219) + (t204 * (0.5 * (((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t208) - (t207 * (l.f5ee * p.p85))) / (t208 * t208)))) * t216) + (t20c * ((l.f645 * ((((l.f746 * l.f5f1) - (l.f745 * l.f5f2)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t210) - (t20f * (l.f5ee * p.p85))) / (t210 * t210)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t200) - (t1ff * (l.f5ef * p.p85))) / (t200 * t200)))) * t219) + (t204 * (0.5 * (((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t208) - (t207 * (l.f5ef * p.p85))) / (t208 * t208)))) * t216) + (t20c * ((l.f645 * ((((l.f747 * l.f5f1) - (l.f745 * l.f5f3)) / (l.f5f1 * l.f5f1)) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t210) - (t20f * (l.f5ef * p.p85))) / (t210 * t210)))) * 0.3333333333333333))))))), );l.f53d = 0.0;}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) {let t21d: f64 = (l.f745 - l.f7b1);let t21e: f64 = (t21d * l.f645);let t21f: f64 = (1.0 + t21e);let t220: f64 = (t21f * l.f89);let t221: f64 = (t220).sqrt();(l.f824, l.f827, l.f828, ) = (t221, (((l.f746 * l.f645) * l.f89) / (2.0 * t221)), (((l.f747 * l.f645) * l.f89) / (2.0 * t221)), );l.f829 = 0.0;let t222: f64 = (l.f5eb * l.f5eb);let t223: f64 = (t222 / l.f5df);l.f64f = t223;l.f650 = 0.0;let t224: f64 = (l.f5e5 / l.f645);let t225: f64 = (l.f5df / l.f64f);let t226: f64 = (t225).ln();let t227: f64 = (t224 * t226);l.f793 = t227;l.f794 = 0.0;}
        let t228: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f40a = t228;l.f40b = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t229: f64 = (l.f7b1 - l.f793);let t22a: f64 = (p.p86 * t229);let t22b: f64 = (t22a + l.f5e5);(l.f601, l.f602, l.f603, ) = (t22b, 0.0, 0.0, );l.f604 = 0.0;let t22c: f64 = (p.p86 * l.f793);let t22d: f64 = (l.f5e5 - t22c);(l.f5ed, l.f5ee, l.f5ef, ) = (t22d, 0.0, 0.0, );l.f5f0 = 0.0;let t22e: f64 = (p.p85 - l.f601);let t22f: f64 = (t22e - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t22f, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t230: f64 = (4.0 * p.p85);let t231: f64 = (t230 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t231, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {
            let (t233, t234, t235,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t232: f64 = (-l.f6f7);
        (t232, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t233, t234, t235, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t236: f64 = (l.f6f3 * l.f6f3);let t237: f64 = (t236 + l.f6f7);let t238: f64 = (t237).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t238, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t238)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t238)), );l.f6fa = 0.0;let t239: f64 = (l.f6f3 / l.f6f7);let t23a: f64 = (1.0 + t239);let t23b: f64 = (0.5 * t23a);(l.f55, l.f56, l.f57, ) = (t23b, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;let t23c: f64 = (l.f6f3 + l.f6f7);let t23d: f64 = (0.5 * t23c);let t23e: f64 = (p.p85 - t23d);(l.f605, l.f606, l.f607, ) = (t23e, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t23f: f64 = (l.f605 - l.f5e5);let t240: f64 = (t23f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t240, l.f606, l.f607, );l.f6f6 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_186(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t241: f64 = (4.0 * l.f5e5);let t242: f64 = (t241 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t242, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {
            let (t244, t245, t246,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t243: f64 = (-l.f6f7);
        (t243, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t244, t245, t246, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t247: f64 = (l.f6f3 * l.f6f3);let t248: f64 = (t247 + l.f6f7);let t249: f64 = (t248).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t249, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t249)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t249)), );l.f6fa = 0.0;let t24a: f64 = (l.f6f3 / l.f6f7);let t24b: f64 = (1.0 + t24a);let t24c: f64 = (0.5 * t24b);(l.f51, l.f52, l.f53, ) = (t24c, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t24d: f64 = (l.f6f3 + l.f6f7);let t24e: f64 = (0.5 * t24d);let t24f: f64 = (l.f5e5 + t24e);(l.f5f1, l.f5f2, l.f5f3, ) = (t24f, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t250: f64 = (p.p85 - l.f5ed);let t251: f64 = (t250 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t251, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t252: f64 = (4.0 * p.p85);let t253: f64 = (t252 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t253, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {
            let (t255, t256, t257,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t254: f64 = (-l.f6f7);
        (t254, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t255, t256, t257, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t258: f64 = (l.f6f3 * l.f6f3);let t259: f64 = (t258 + l.f6f7);let t25a: f64 = (t259).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t25a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t25a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t25a)), );l.f6fa = 0.0;let t25b: f64 = (l.f6f3 + l.f6f7);let t25c: f64 = (0.5 * t25b);let t25d: f64 = (p.p85 - t25c);(l.f5ed, l.f5ee, l.f5ef, ) = (t25d, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t25e: f64 = (l.f5ed - l.f5e5);let t25f: f64 = (t25e - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t25f, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t260: f64 = (4.0 * l.f5e5);let t261: f64 = (t260 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t261, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {
            let (t263, t264, t265,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t262: f64 = (-l.f6f7);
        (t262, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t263, t264, t265, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a != 0.0)) {let t266: f64 = (l.f6f3 * l.f6f3);let t267: f64 = (t266 + l.f6f7);let t268: f64 = (t267).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t268, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t268)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t268)), );l.f6fa = 0.0;let t269: f64 = (l.f6f3 + l.f6f7);let t26a: f64 = (0.5 * t269);let t26b: f64 = (l.f5e5 + t26a);(l.f5ed, l.f5ee, l.f5ef, ) = (t26b, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t26c: f64 = (p.p86 * l.f55);let t26d: f64 = (t26c * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t26d, (((p.p86 * l.f56) * l.f51) + (t26c * l.f52)), (((p.p86 * l.f57) * l.f51) + (t26c * l.f53)), );l.f5e = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40a == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
        let t26e: f64 = (l.f7b1 / l.f5f1);let t26f: f64 = (l.f5f1 - l.f5ed);let t270: f64 = (l.f793 * t26f);let t271: f64 = (l.f5ed * p.p85);let t272: f64 = (t270 / t271);let t273: f64 = (t26e + t272);let t274: f64 = (l.f645 * t273);let t275: f64 = (t274).abs();let t276: f64 = if t275 < 230.25850929940458 { 1.0 } else { 0.0 };l.f40c = t276;l.f40d = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_187(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40c != 0.0)) {let t277: f64 = (l.f7b1 / l.f5f1);let t278: f64 = (l.f5f1 - l.f5ed);let t279: f64 = (l.f793 * t278);let t27a: f64 = (l.f5ed * p.p85);let t27b: f64 = (t279 / t27a);let t27c: f64 = (t277 + t27b);let t27d: f64 = (l.f645 * t27c);let t27e: f64 = (t27d).exp();(l.f8a, l.f8b, l.f8c, ) = (t27e, (t27e * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t27a) - (t279 * (l.f5ee * p.p85))) / (t27a * t27a))))), (t27e * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t27a) - (t279 * (l.f5ef * p.p85))) / (t27a * t27a))))), );l.f8d = 0.0;}
        let t27f: f64 = (l.f7b1 / l.f5f1);let t280: f64 = (l.f5f1 - l.f5ed);let t281: f64 = (l.f793 * t280);let t282: f64 = (l.f5ed * p.p85);let t283: f64 = (t281 / t282);let t284: f64 = (t27f + t283);let t285: f64 = (l.f645 * t284);let t286: f64 = (-230.25850929940458);let t287: f64 = if t285 < t286 { 1.0 } else { 0.0 };l.f40e = t287;l.f40f = 0.0;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40c == 0.0)) && (l.f40e != 0.0)) {let t288: f64 = (-230.25850929940458);let t289: f64 = (l.f7b1 / l.f5f1);let t28a: f64 = (l.f5f1 - l.f5ed);let t28b: f64 = (l.f793 * t28a);let t28c: f64 = (l.f5ed * p.p85);let t28d: f64 = (t28b / t28c);let t28e: f64 = (t289 + t28d);let t28f: f64 = (l.f645 * t28e);let t290: f64 = (t288 - t28f);let t291: f64 = (-230.25850929940458);let t292: f64 = (l.f7b1 / l.f5f1);let t293: f64 = (l.f5f1 - l.f5ed);let t294: f64 = (l.f793 * t293);let t295: f64 = (l.f5ed * p.p85);let t296: f64 = (t294 / t295);let t297: f64 = (t292 + t296);let t298: f64 = (l.f645 * t297);let t299: f64 = (t291 - t298);let t29a: f64 = (-230.25850929940458);let t29b: f64 = (l.f7b1 / l.f5f1);let t29c: f64 = (l.f5f1 - l.f5ed);let t29d: f64 = (l.f793 * t29c);let t29e: f64 = (l.f5ed * p.p85);let t29f: f64 = (t29d / t29e);let t2a0: f64 = (t29b + t29f);let t2a1: f64 = (l.f645 * t2a0);let t2a2: f64 = (t29a - t2a1);let t2a3: f64 = (t2a2 * 0.3333333333333333);let t2a4: f64 = (1.0 + t2a3);let t2a5: f64 = (t299 * t2a4);let t2a6: f64 = (0.5 * t2a5);let t2a7: f64 = (1.0 + t2a6);let t2a8: f64 = (t290 * t2a7);let t2a9: f64 = (1.0 + t2a8);let t2aa: f64 = (1e-100 / t2a9);(l.f8a, l.f8b, l.f8c, ) = (t2aa, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t28c) - (t28b * (l.f5ee * p.p85))) / (t28c * t28c))))) * t2a7) + (t290 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t295) - (t294 * (l.f5ee * p.p85))) / (t295 * t295))))) * t2a4) + (t299 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t29e) - (t29d * (l.f5ee * p.p85))) / (t29e * t29e))))) * 0.3333333333333333))))))) / (t2a9 * t2a9))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t28c) - (t28b * (l.f5ef * p.p85))) / (t28c * t28c))))) * t2a7) + (t290 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t295) - (t294 * (l.f5ef * p.p85))) / (t295 * t295))))) * t2a4) + (t299 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t29e) - (t29d * (l.f5ef * p.p85))) / (t29e * t29e))))) * 0.3333333333333333))))))) / (t2a9 * t2a9))), );l.f8d = 0.0;}
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f40c == 0.0)) && (l.f40e == 0.0)) {let t2ab: f64 = (l.f7b1 / l.f5f1);let t2ac: f64 = (l.f5f1 - l.f5ed);let t2ad: f64 = (l.f793 * t2ac);let t2ae: f64 = (l.f5ed * p.p85);let t2af: f64 = (t2ad / t2ae);let t2b0: f64 = (t2ab + t2af);let t2b1: f64 = (l.f645 * t2b0);let t2b2: f64 = (t2b1 - 230.25850929940458);let t2b3: f64 = (l.f7b1 / l.f5f1);let t2b4: f64 = (l.f5f1 - l.f5ed);let t2b5: f64 = (l.f793 * t2b4);let t2b6: f64 = (l.f5ed * p.p85);let t2b7: f64 = (t2b5 / t2b6);let t2b8: f64 = (t2b3 + t2b7);let t2b9: f64 = (l.f645 * t2b8);let t2ba: f64 = (t2b9 - 230.25850929940458);let t2bb: f64 = (l.f7b1 / l.f5f1);let t2bc: f64 = (l.f5f1 - l.f5ed);let t2bd: f64 = (l.f793 * t2bc);let t2be: f64 = (l.f5ed * p.p85);let t2bf: f64 = (t2bd / t2be);let t2c0: f64 = (t2bb + t2bf);let t2c1: f64 = (l.f645 * t2c0);let t2c2: f64 = (t2c1 - 230.25850929940458);let t2c3: f64 = (t2c2 * 0.3333333333333333);let t2c4: f64 = (1.0 + t2c3);let t2c5: f64 = (t2ba * t2c4);let t2c6: f64 = (0.5 * t2c5);let t2c7: f64 = (1.0 + t2c6);let t2c8: f64 = (t2b2 * t2c7);let t2c9: f64 = (1.0 + t2c8);let t2ca: f64 = (1e100 * t2c9);(l.f8a, l.f8b, l.f8c, ) = (t2ca, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2ae) - (t2ad * (l.f5ee * p.p85))) / (t2ae * t2ae)))) * t2c7) + (t2b2 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2b6) - (t2b5 * (l.f5ee * p.p85))) / (t2b6 * t2b6)))) * t2c4) + (t2ba * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2be) - (t2bd * (l.f5ee * p.p85))) / (t2be * t2be)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2ae) - (t2ad * (l.f5ef * p.p85))) / (t2ae * t2ae)))) * t2c7) + (t2b2 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2b6) - (t2b5 * (l.f5ef * p.p85))) / (t2b6 * t2b6)))) * t2c4) + (t2ba * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2be) - (t2bd * (l.f5ef * p.p85))) / (t2be * t2be)))) * 0.3333333333333333))))))), );l.f8d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_188(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) {let t2cb: f64 = (l.f7b1 * l.f5b);let t2cc: f64 = (l.f5f1 - t2cb);let t2cd: f64 = (l.f5f1 * l.f5f1);let t2ce: f64 = (t2cc / t2cd);let t2cf: f64 = (l.f793 * l.f5b);let t2d0: f64 = (l.f5ed * p.p85);let t2d1: f64 = (t2cf / t2d0);let t2d2: f64 = (t2ce + t2d1);let t2d3: f64 = (l.f645 * t2d2);(l.f61, l.f62, l.f63, ) = (t2d3, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t2cd) - (t2cc * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t2cd * t2cd)) + ((((l.f793 * l.f5c) * t2d0) - (t2cf * (l.f5ee * p.p85))) / (t2d0 * t2d0)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t2cd) - (t2cc * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t2cd * t2cd)) + ((((l.f793 * l.f5d) * t2d0) - (t2cf * (l.f5ef * p.p85))) / (t2d0 * t2d0)))), );l.f64 = 0.0;let t2d4: f64 = (l.f745 - l.f7b1);let t2d5: f64 = (t2d4 * l.f61);let t2d6: f64 = (1.0 + t2d5);let t2d7: f64 = (t2d6 * l.f8a);(l.f536, l.f537, l.f538, ) = (t2d7, ((((l.f746 * l.f61) + (t2d4 * l.f62)) * l.f8a) + (t2d6 * l.f8b)), ((((l.f747 * l.f61) + (t2d4 * l.f63)) * l.f8a) + (t2d6 * l.f8c)), );l.f539 = 0.0;let t2d8: f64 = (l.f5eb * l.f5eb);let t2d9: f64 = (t2d8 / l.f5e3);l.f64f = t2d9;l.f650 = 0.0;let t2da: f64 = (l.f5e9 / l.f645);let t2db: f64 = (l.f5e3 / l.f64f);let t2dc: f64 = (t2db).ln();let t2dd: f64 = (t2da * t2dc);l.f793 = t2dd;l.f794 = 0.0;}
        let t2de: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f410 = t2de;l.f411 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t2df: f64 = (l.f7b1 - l.f793);let t2e0: f64 = (p.p86 * t2df);let t2e1: f64 = (t2e0 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t2e1, 0.0, 0.0, );l.f604 = 0.0;let t2e2: f64 = (p.p86 * l.f793);let t2e3: f64 = (l.f5e9 - t2e2);(l.f5ed, l.f5ee, l.f5ef, ) = (t2e3, 0.0, 0.0, );l.f5f0 = 0.0;let t2e4: f64 = (p.p85 - l.f601);let t2e5: f64 = (t2e4 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2e5, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t2e6: f64 = (4.0 * p.p85);let t2e7: f64 = (t2e6 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2e7, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {
            let (t2e9, t2ea, t2eb,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2e8: f64 = (-l.f6f7);
        (t2e8, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2e9, t2ea, t2eb, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t2ec: f64 = (l.f6f3 * l.f6f3);let t2ed: f64 = (t2ec + l.f6f7);let t2ee: f64 = (t2ed).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2ee, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2ee)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2ee)), );l.f6fa = 0.0;let t2ef: f64 = (l.f6f3 / l.f6f7);let t2f0: f64 = (1.0 + t2ef);let t2f1: f64 = (0.5 * t2f0);(l.f55, l.f56, l.f57, ) = (t2f1, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;let t2f2: f64 = (l.f6f3 + l.f6f7);let t2f3: f64 = (0.5 * t2f2);let t2f4: f64 = (p.p85 - t2f3);(l.f605, l.f606, l.f607, ) = (t2f4, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t2f5: f64 = (l.f605 - l.f5e9);let t2f6: f64 = (t2f5 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2f6, l.f606, l.f607, );l.f6f6 = 0.0;let t2f7: f64 = (4.0 * l.f5e9);let t2f8: f64 = (t2f7 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2f8, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {
            let (t2fa, t2fb, t2fc,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2f9: f64 = (-l.f6f7);
        (t2f9, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2fa, t2fb, t2fc, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t2fd: f64 = (l.f6f3 * l.f6f3);let t2fe: f64 = (t2fd + l.f6f7);let t2ff: f64 = (t2fe).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2ff, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2ff)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2ff)), );l.f6fa = 0.0;let t300: f64 = (l.f6f3 / l.f6f7);let t301: f64 = (1.0 + t300);let t302: f64 = (0.5 * t301);(l.f51, l.f52, l.f53, ) = (t302, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t303: f64 = (l.f6f3 + l.f6f7);let t304: f64 = (0.5 * t303);let t305: f64 = (l.f5e9 + t304);(l.f5f1, l.f5f2, l.f5f3, ) = (t305, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_189(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t306: f64 = (p.p85 - l.f5ed);let t307: f64 = (t306 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t307, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t308: f64 = (4.0 * p.p85);let t309: f64 = (t308 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t309, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {
            let (t30b, t30c, t30d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t30a: f64 = (-l.f6f7);
        (t30a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t30b, t30c, t30d, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t30e: f64 = (l.f6f3 * l.f6f3);let t30f: f64 = (t30e + l.f6f7);let t310: f64 = (t30f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t310, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t310)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t310)), );l.f6fa = 0.0;let t311: f64 = (l.f6f3 + l.f6f7);let t312: f64 = (0.5 * t311);let t313: f64 = (p.p85 - t312);(l.f5ed, l.f5ee, l.f5ef, ) = (t313, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t314: f64 = (l.f5ed - l.f5e9);let t315: f64 = (t314 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t315, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t316: f64 = (4.0 * l.f5e9);let t317: f64 = (t316 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t317, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {
            let (t319, t31a, t31b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t318: f64 = (-l.f6f7);
        (t318, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t319, t31a, t31b, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 != 0.0)) {let t31c: f64 = (l.f6f3 * l.f6f3);let t31d: f64 = (t31c + l.f6f7);let t31e: f64 = (t31d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t31e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t31e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t31e)), );l.f6fa = 0.0;let t31f: f64 = (l.f6f3 + l.f6f7);let t320: f64 = (0.5 * t31f);let t321: f64 = (l.f5e9 + t320);(l.f5ed, l.f5ee, l.f5ef, ) = (t321, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t322: f64 = (p.p86 * l.f55);let t323: f64 = (t322 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t323, (((p.p86 * l.f56) * l.f51) + (t322 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t322 * l.f53)), );l.f5e = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f410 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
        let t324: f64 = (l.f7b1 / l.f5f1);let t325: f64 = (l.f5f1 - l.f5ed);let t326: f64 = (l.f793 * t325);let t327: f64 = (l.f5ed * p.p85);let t328: f64 = (t326 / t327);let t329: f64 = (t324 + t328);let t32a: f64 = (l.f645 * t329);let t32b: f64 = (t32a).abs();let t32c: f64 = if t32b < 230.25850929940458 { 1.0 } else { 0.0 };l.f412 = t32c;l.f413 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f412 != 0.0)) {let t32d: f64 = (l.f7b1 / l.f5f1);let t32e: f64 = (l.f5f1 - l.f5ed);let t32f: f64 = (l.f793 * t32e);let t330: f64 = (l.f5ed * p.p85);let t331: f64 = (t32f / t330);let t332: f64 = (t32d + t331);let t333: f64 = (l.f645 * t332);let t334: f64 = (t333).exp();(l.f93, l.f94, l.f95, ) = (t334, (t334 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t330) - (t32f * (l.f5ee * p.p85))) / (t330 * t330))))), (t334 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t330) - (t32f * (l.f5ef * p.p85))) / (t330 * t330))))), );l.f96 = 0.0;}
        let t335: f64 = (l.f7b1 / l.f5f1);let t336: f64 = (l.f5f1 - l.f5ed);let t337: f64 = (l.f793 * t336);let t338: f64 = (l.f5ed * p.p85);let t339: f64 = (t337 / t338);let t33a: f64 = (t335 + t339);let t33b: f64 = (l.f645 * t33a);let t33c: f64 = (-230.25850929940458);let t33d: f64 = if t33b < t33c { 1.0 } else { 0.0 };l.f414 = t33d;l.f415 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_190(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f412 == 0.0)) && (l.f414 != 0.0)) {let t33e: f64 = (-230.25850929940458);let t33f: f64 = (l.f7b1 / l.f5f1);let t340: f64 = (l.f5f1 - l.f5ed);let t341: f64 = (l.f793 * t340);let t342: f64 = (l.f5ed * p.p85);let t343: f64 = (t341 / t342);let t344: f64 = (t33f + t343);let t345: f64 = (l.f645 * t344);let t346: f64 = (t33e - t345);let t347: f64 = (-230.25850929940458);let t348: f64 = (l.f7b1 / l.f5f1);let t349: f64 = (l.f5f1 - l.f5ed);let t34a: f64 = (l.f793 * t349);let t34b: f64 = (l.f5ed * p.p85);let t34c: f64 = (t34a / t34b);let t34d: f64 = (t348 + t34c);let t34e: f64 = (l.f645 * t34d);let t34f: f64 = (t347 - t34e);let t350: f64 = (-230.25850929940458);let t351: f64 = (l.f7b1 / l.f5f1);let t352: f64 = (l.f5f1 - l.f5ed);let t353: f64 = (l.f793 * t352);let t354: f64 = (l.f5ed * p.p85);let t355: f64 = (t353 / t354);let t356: f64 = (t351 + t355);let t357: f64 = (l.f645 * t356);let t358: f64 = (t350 - t357);let t359: f64 = (t358 * 0.3333333333333333);let t35a: f64 = (1.0 + t359);let t35b: f64 = (t34f * t35a);let t35c: f64 = (0.5 * t35b);let t35d: f64 = (1.0 + t35c);let t35e: f64 = (t346 * t35d);let t35f: f64 = (1.0 + t35e);let t360: f64 = (1e-100 / t35f);(l.f93, l.f94, l.f95, ) = (t360, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t342) - (t341 * (l.f5ee * p.p85))) / (t342 * t342))))) * t35d) + (t346 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t34b) - (t34a * (l.f5ee * p.p85))) / (t34b * t34b))))) * t35a) + (t34f * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t354) - (t353 * (l.f5ee * p.p85))) / (t354 * t354))))) * 0.3333333333333333))))))) / (t35f * t35f))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t342) - (t341 * (l.f5ef * p.p85))) / (t342 * t342))))) * t35d) + (t346 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t34b) - (t34a * (l.f5ef * p.p85))) / (t34b * t34b))))) * t35a) + (t34f * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t354) - (t353 * (l.f5ef * p.p85))) / (t354 * t354))))) * 0.3333333333333333))))))) / (t35f * t35f))), );l.f96 = 0.0;}
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f412 == 0.0)) && (l.f414 == 0.0)) {let t361: f64 = (l.f7b1 / l.f5f1);let t362: f64 = (l.f5f1 - l.f5ed);let t363: f64 = (l.f793 * t362);let t364: f64 = (l.f5ed * p.p85);let t365: f64 = (t363 / t364);let t366: f64 = (t361 + t365);let t367: f64 = (l.f645 * t366);let t368: f64 = (t367 - 230.25850929940458);let t369: f64 = (l.f7b1 / l.f5f1);let t36a: f64 = (l.f5f1 - l.f5ed);let t36b: f64 = (l.f793 * t36a);let t36c: f64 = (l.f5ed * p.p85);let t36d: f64 = (t36b / t36c);let t36e: f64 = (t369 + t36d);let t36f: f64 = (l.f645 * t36e);let t370: f64 = (t36f - 230.25850929940458);let t371: f64 = (l.f7b1 / l.f5f1);let t372: f64 = (l.f5f1 - l.f5ed);let t373: f64 = (l.f793 * t372);let t374: f64 = (l.f5ed * p.p85);let t375: f64 = (t373 / t374);let t376: f64 = (t371 + t375);let t377: f64 = (l.f645 * t376);let t378: f64 = (t377 - 230.25850929940458);let t379: f64 = (t378 * 0.3333333333333333);let t37a: f64 = (1.0 + t379);let t37b: f64 = (t370 * t37a);let t37c: f64 = (0.5 * t37b);let t37d: f64 = (1.0 + t37c);let t37e: f64 = (t368 * t37d);let t37f: f64 = (1.0 + t37e);let t380: f64 = (1e100 * t37f);(l.f93, l.f94, l.f95, ) = (t380, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t364) - (t363 * (l.f5ee * p.p85))) / (t364 * t364)))) * t37d) + (t368 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t36c) - (t36b * (l.f5ee * p.p85))) / (t36c * t36c)))) * t37a) + (t370 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t374) - (t373 * (l.f5ee * p.p85))) / (t374 * t374)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t364) - (t363 * (l.f5ef * p.p85))) / (t364 * t364)))) * t37d) + (t368 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t36c) - (t36b * (l.f5ef * p.p85))) / (t36c * t36c)))) * t37a) + (t370 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t374) - (t373 * (l.f5ef * p.p85))) / (t374 * t374)))) * 0.3333333333333333))))))), );l.f96 = 0.0;}
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) {let t381: f64 = (l.f7b1 * l.f5b);let t382: f64 = (l.f5f1 - t381);let t383: f64 = (l.f5f1 * l.f5f1);let t384: f64 = (t382 / t383);let t385: f64 = (l.f793 * l.f5b);let t386: f64 = (l.f5ed * p.p85);let t387: f64 = (t385 / t386);let t388: f64 = (t384 + t387);let t389: f64 = (l.f645 * t388);(l.f61, l.f62, l.f63, ) = (t389, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t383) - (t382 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t383 * t383)) + ((((l.f793 * l.f5c) * t386) - (t385 * (l.f5ee * p.p85))) / (t386 * t386)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t383) - (t382 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t383 * t383)) + ((((l.f793 * l.f5d) * t386) - (t385 * (l.f5ef * p.p85))) / (t386 * t386)))), );l.f64 = 0.0;let t38a: f64 = (l.f745 - l.f7b1);let t38b: f64 = (t38a * l.f61);let t38c: f64 = (1.0 + t38b);let t38d: f64 = (t38c * l.f93);(l.f53e, l.f53f, l.f540, ) = (t38d, ((((l.f746 * l.f61) + (t38a * l.f62)) * l.f93) + (t38c * l.f94)), ((((l.f747 * l.f61) + (t38a * l.f63)) * l.f93) + (t38c * l.f95)), );l.f541 = 0.0;let t38e: f64 = (l.f5eb * l.f5eb);let t38f: f64 = (t38e / l.f5e1);l.f64f = t38f;l.f650 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_191(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) {let t390: f64 = (l.f5e7 / l.f645);let t391: f64 = (l.f5e1 / l.f64f);let t392: f64 = (t391).ln();let t393: f64 = (t390 * t392);l.f793 = t393;l.f794 = 0.0;}
        let t394: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f416 = t394;l.f417 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t395: f64 = (l.f7b1 - l.f793);let t396: f64 = (p.p86 * t395);let t397: f64 = (t396 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t397, 0.0, 0.0, );l.f604 = 0.0;let t398: f64 = (p.p86 * l.f793);let t399: f64 = (l.f5e7 - t398);(l.f5ed, l.f5ee, l.f5ef, ) = (t399, 0.0, 0.0, );l.f5f0 = 0.0;let t39a: f64 = (p.p85 - l.f601);let t39b: f64 = (t39a - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t39b, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t39c: f64 = (4.0 * p.p85);let t39d: f64 = (t39c * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t39d, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {
            let (t39f, t3a0, t3a1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t39e: f64 = (-l.f6f7);
        (t39e, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t39f, t3a0, t3a1, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t3a2: f64 = (l.f6f3 * l.f6f3);let t3a3: f64 = (t3a2 + l.f6f7);let t3a4: f64 = (t3a3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3a4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3a4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3a4)), );l.f6fa = 0.0;let t3a5: f64 = (l.f6f3 / l.f6f7);let t3a6: f64 = (1.0 + t3a5);let t3a7: f64 = (0.5 * t3a6);(l.f55, l.f56, l.f57, ) = (t3a7, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;let t3a8: f64 = (l.f6f3 + l.f6f7);let t3a9: f64 = (0.5 * t3a8);let t3aa: f64 = (p.p85 - t3a9);(l.f605, l.f606, l.f607, ) = (t3aa, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t3ab: f64 = (l.f605 - l.f5e7);let t3ac: f64 = (t3ab - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3ac, l.f606, l.f607, );l.f6f6 = 0.0;let t3ad: f64 = (4.0 * l.f5e7);let t3ae: f64 = (t3ad * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3ae, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {
            let (t3b0, t3b1, t3b2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3af: f64 = (-l.f6f7);
        (t3af, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3b0, t3b1, t3b2, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t3b3: f64 = (l.f6f3 * l.f6f3);let t3b4: f64 = (t3b3 + l.f6f7);let t3b5: f64 = (t3b4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3b5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3b5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3b5)), );l.f6fa = 0.0;let t3b6: f64 = (l.f6f3 / l.f6f7);let t3b7: f64 = (1.0 + t3b6);let t3b8: f64 = (0.5 * t3b7);(l.f51, l.f52, l.f53, ) = (t3b8, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t3b9: f64 = (l.f6f3 + l.f6f7);let t3ba: f64 = (0.5 * t3b9);let t3bb: f64 = (l.f5e7 + t3ba);(l.f5f1, l.f5f2, l.f5f3, ) = (t3bb, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t3bc: f64 = (p.p85 - l.f5ed);let t3bd: f64 = (t3bc - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3bd, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t3be: f64 = (4.0 * p.p85);let t3bf: f64 = (t3be * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3bf, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {
            let (t3c1, t3c2, t3c3,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3c0: f64 = (-l.f6f7);
        (t3c0, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3c1, t3c2, t3c3, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t3c4: f64 = (l.f6f3 * l.f6f3);let t3c5: f64 = (t3c4 + l.f6f7);let t3c6: f64 = (t3c5).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3c6, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3c6)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3c6)), );l.f6fa = 0.0;let t3c7: f64 = (l.f6f3 + l.f6f7);let t3c8: f64 = (0.5 * t3c7);let t3c9: f64 = (p.p85 - t3c8);(l.f5ed, l.f5ee, l.f5ef, ) = (t3c9, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_192(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t3ca: f64 = (l.f5ed - l.f5e7);let t3cb: f64 = (t3ca - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3cb, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t3cc: f64 = (4.0 * l.f5e7);let t3cd: f64 = (t3cc * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3cd, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {
            let (t3cf, t3d0, t3d1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3ce: f64 = (-l.f6f7);
        (t3ce, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3cf, t3d0, t3d1, );l.f6fa = 0.0;
        }
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 != 0.0)) {let t3d2: f64 = (l.f6f3 * l.f6f3);let t3d3: f64 = (t3d2 + l.f6f7);let t3d4: f64 = (t3d3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3d4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3d4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3d4)), );l.f6fa = 0.0;let t3d5: f64 = (l.f6f3 + l.f6f7);let t3d6: f64 = (0.5 * t3d5);let t3d7: f64 = (l.f5e7 + t3d6);(l.f5ed, l.f5ee, l.f5ef, ) = (t3d7, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t3d8: f64 = (p.p86 * l.f55);let t3d9: f64 = (t3d8 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t3d9, (((p.p86 * l.f56) * l.f51) + (t3d8 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t3d8 * l.f53)), );l.f5e = 0.0;}
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f416 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
        let t3da: f64 = (l.f7b1 / l.f5f1);let t3db: f64 = (l.f5f1 - l.f5ed);let t3dc: f64 = (l.f793 * t3db);let t3dd: f64 = (l.f5ed * p.p85);let t3de: f64 = (t3dc / t3dd);let t3df: f64 = (t3da + t3de);let t3e0: f64 = (l.f645 * t3df);let t3e1: f64 = (t3e0).abs();let t3e2: f64 = if t3e1 < 230.25850929940458 { 1.0 } else { 0.0 };l.f418 = t3e2;l.f419 = 0.0;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 == 0.0)) && (l.f418 != 0.0)) {let t3e3: f64 = (l.f7b1 / l.f5f1);let t3e4: f64 = (l.f5f1 - l.f5ed);let t3e5: f64 = (l.f793 * t3e4);let t3e6: f64 = (l.f5ed * p.p85);let t3e7: f64 = (t3e5 / t3e6);let t3e8: f64 = (t3e3 + t3e7);let t3e9: f64 = (l.f645 * t3e8);let t3ea: f64 = (t3e9).exp();(l.f8e, l.f8f, l.f90, ) = (t3ea, (t3ea * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3e6) - (t3e5 * (l.f5ee * p.p85))) / (t3e6 * t3e6))))), (t3ea * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3e6) - (t3e5 * (l.f5ef * p.p85))) / (t3e6 * t3e6))))), );l.f91 = 0.0;}
        let t3eb: f64 = (l.f7b1 / l.f5f1);let t3ec: f64 = (l.f5f1 - l.f5ed);let t3ed: f64 = (l.f793 * t3ec);let t3ee: f64 = (l.f5ed * p.p85);let t3ef: f64 = (t3ed / t3ee);let t3f0: f64 = (t3eb + t3ef);let t3f1: f64 = (l.f645 * t3f0);let t3f2: f64 = (-230.25850929940458);let t3f3: f64 = if t3f1 < t3f2 { 1.0 } else { 0.0 };l.f41c = t3f3;l.f41d = 0.0;
    }
}
