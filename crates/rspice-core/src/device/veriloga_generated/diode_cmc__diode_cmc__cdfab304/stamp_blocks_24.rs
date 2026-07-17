#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_209(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let t0: f64 = (l.f6f3 * l.f6f3);let t1: f64 = (t0 + l.f6f7);let t2: f64 = (t1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2)), );l.f6fa = 0.0;let t3: f64 = (l.f6f3 + l.f6f7);let t4: f64 = (0.5 * t3);let t5: f64 = (l.f609 + t4);(l.f5f5, l.f5fe, l.f5ff, ) = (t5, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f600 = 0.0;let t6: f64 = (p.p85 - l.f5ed);let t7: f64 = (t6 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t7, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t8: f64 = (4.0 * p.p85);let t9: f64 = (t8 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t9, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {
            let (tb, tc, td,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let ta: f64 = (-l.f6f7);
        (ta, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tb, tc, td, );l.f6fa = 0.0;
        }
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let te: f64 = (l.f6f3 * l.f6f3);let tf: f64 = (te + l.f6f7);let t10: f64 = (tf).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t10, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t10)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t10)), );l.f6fa = 0.0;let t11: f64 = (l.f6f3 + l.f6f7);let t12: f64 = (0.5 * t11);let t13: f64 = (p.p85 - t12);(l.f5ed, l.f5ee, l.f5ef, ) = (t13, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t14: f64 = (l.f5ed - l.f609);let t15: f64 = (t14 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t15, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t16: f64 = (4.0 * l.f609);let t17: f64 = (t16 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t17, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {
            let (t19, t1a, t1b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t18: f64 = (-l.f6f7);
        (t18, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t19, t1a, t1b, );l.f6fa = 0.0;
        }
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let t1c: f64 = (l.f6f3 * l.f6f3);let t1d: f64 = (t1c + l.f6f7);let t1e: f64 = (t1d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1e)), );l.f6fa = 0.0;let t1f: f64 = (l.f6f3 + l.f6f7);let t20: f64 = (0.5 * t1f);let t21: f64 = (l.f609 + t20);(l.f5ed, l.f5ee, l.f5ef, ) = (t21, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((l.f48c != 0.0) && (l.f48e == 0.0)) {(l.f5f5, l.f5fe, l.f5ff, ) = (l.f609, 0.0, 0.0, );l.f600 = 0.0;(l.f5ed, l.f5ee, l.f5ef, ) = (l.f609, 0.0, 0.0, );l.f5f0 = 0.0;}
        if (l.f48c != 0.0) {(l.f79, l.f7e, l.f7f, ) = (l.f536, l.f537, l.f538, );l.f80 = 0.0;}
        let t22: f64 = (l.f743 - l.f741);let t23: f64 = (l.f745 - t22);let t24: f64 = if t23 > 0.0 { 1.0 } else { 0.0 };l.f490 = t24;l.f491 = 0.0;let t25: f64 = (l.f745 / l.f5f5);let t26: f64 = (l.f743 - l.f741);let t27: f64 = (t26 / l.f5f5);let t28: f64 = (t25 - t27);let t29: f64 = (l.f5f5 - l.f5ed);let t2a: f64 = (l.f743 * t29);let t2b: f64 = (l.f5ed * p.p85);let t2c: f64 = (t2a / t2b);let t2d: f64 = (t28 + t2c);let t2e: f64 = (l.f645 * t2d);let t2f: f64 = (t2e).abs();let t30: f64 = if t2f < 230.25850929940458 { 1.0 } else { 0.0 };l.f492 = t30;l.f493 = 0.0;
        if (((l.f48c != 0.0) && (l.f490 != 0.0)) && (l.f492 != 0.0)) {let t31: f64 = (l.f745 / l.f5f5);let t32: f64 = (l.f743 - l.f741);let t33: f64 = (t32 / l.f5f5);let t34: f64 = (t31 - t33);let t35: f64 = (l.f5f5 - l.f5ed);let t36: f64 = (l.f743 * t35);let t37: f64 = (l.f5ed * p.p85);let t38: f64 = (t36 / t37);let t39: f64 = (t34 + t38);let t3a: f64 = (l.f645 * t39);let t3b: f64 = (t3a).exp();(l.f81, l.f86, l.f87, ) = (t3b, (t3b * (l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t32 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t37) - (t36 * (l.f5ee * p.p85))) / (t37 * t37))))), (t3b * (l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t32 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t37) - (t36 * (l.f5ef * p.p85))) / (t37 * t37))))), );l.f88 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_210(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t3c: f64 = (l.f745 / l.f5f5);let t3d: f64 = (l.f743 - l.f741);let t3e: f64 = (t3d / l.f5f5);let t3f: f64 = (t3c - t3e);let t40: f64 = (l.f5f5 - l.f5ed);let t41: f64 = (l.f743 * t40);let t42: f64 = (l.f5ed * p.p85);let t43: f64 = (t41 / t42);let t44: f64 = (t3f + t43);let t45: f64 = (l.f645 * t44);let t46: f64 = (-230.25850929940458);let t47: f64 = if t45 < t46 { 1.0 } else { 0.0 };l.f494 = t47;l.f495 = 0.0;
        if ((((l.f48c != 0.0) && (l.f490 != 0.0)) && (l.f492 == 0.0)) && (l.f494 != 0.0)) {
            let t48: f64 = (-230.25850929940458);let t49: f64 = (l.f745 / l.f5f5);let t4a: f64 = (l.f743 - l.f741);let t4b: f64 = (t4a / l.f5f5);let t4c: f64 = (t49 - t4b);let t4d: f64 = (l.f5f5 - l.f5ed);let t4e: f64 = (l.f743 * t4d);let t4f: f64 = (l.f5ed * p.p85);let t50: f64 = (t4e / t4f);let t51: f64 = (t4c + t50);let t52: f64 = (l.f645 * t51);let t53: f64 = (t48 - t52);let t54: f64 = (-230.25850929940458);let t55: f64 = (l.f745 / l.f5f5);let t56: f64 = (l.f743 - l.f741);let t57: f64 = (t56 / l.f5f5);let t58: f64 = (t55 - t57);let t59: f64 = (l.f5f5 - l.f5ed);let t5a: f64 = (l.f743 * t59);let t5b: f64 = (l.f5ed * p.p85);let t5c: f64 = (t5a / t5b);let t5d: f64 = (t58 + t5c);let t5e: f64 = (l.f645 * t5d);let t5f: f64 = (t54 - t5e);let t60: f64 = (-230.25850929940458);let t61: f64 = (l.f745 / l.f5f5);let t62: f64 = (l.f743 - l.f741);let t63: f64 = (t62 / l.f5f5);let t64: f64 = (t61 - t63);let t65: f64 = (l.f5f5 - l.f5ed);let t66: f64 = (l.f743 * t65);let t67: f64 = (l.f5ed * p.p85);let t68: f64 = (t66 / t67);let t69: f64 = (t64 + t68);let t6a: f64 = (l.f645 * t69);let t6b: f64 = (t60 - t6a);let t6c: f64 = (t6b * 0.3333333333333333);let t6d: f64 = (1.0 + t6c);let t6e: f64 = (t5f * t6d);let t6f: f64 = (0.5 * t6e);let t70: f64 = (1.0 + t6f);let t71: f64 = (t53 * t70);let t72: f64 = (1.0 + t71);let t73: f64 = (1e-100 / t72);
            (l.f81, l.f86, l.f87, ) = (t73, (-((1e-100 * (((-(l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t4a * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t4f) - (t4e * (l.f5ee * p.p85))) / (t4f * t4f))))) * t70) + (t53 * (0.5 * (((-(l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t56 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t5b) - (t5a * (l.f5ee * p.p85))) / (t5b * t5b))))) * t6d) + (t5f * ((-(l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t62 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t67) - (t66 * (l.f5ee * p.p85))) / (t67 * t67))))) * 0.3333333333333333))))))) / (t72 * t72))), (-((1e-100 * (((-(l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t4a * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t4f) - (t4e * (l.f5ef * p.p85))) / (t4f * t4f))))) * t70) + (t53 * (0.5 * (((-(l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t56 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t5b) - (t5a * (l.f5ef * p.p85))) / (t5b * t5b))))) * t6d) + (t5f * ((-(l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t62 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t67) - (t66 * (l.f5ef * p.p85))) / (t67 * t67))))) * 0.3333333333333333))))))) / (t72 * t72))), );l.f88 = 0.0;
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_211(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        if ((((l.f48c != 0.0) && (l.f490 != 0.0)) && (l.f492 == 0.0)) && (l.f494 == 0.0)) {
            let t74: f64 = (l.f745 / l.f5f5);let t75: f64 = (l.f743 - l.f741);let t76: f64 = (t75 / l.f5f5);let t77: f64 = (t74 - t76);let t78: f64 = (l.f5f5 - l.f5ed);let t79: f64 = (l.f743 * t78);let t7a: f64 = (l.f5ed * p.p85);let t7b: f64 = (t79 / t7a);let t7c: f64 = (t77 + t7b);let t7d: f64 = (l.f645 * t7c);let t7e: f64 = (t7d - 230.25850929940458);let t7f: f64 = (l.f745 / l.f5f5);let t80: f64 = (l.f743 - l.f741);let t81: f64 = (t80 / l.f5f5);let t82: f64 = (t7f - t81);let t83: f64 = (l.f5f5 - l.f5ed);let t84: f64 = (l.f743 * t83);let t85: f64 = (l.f5ed * p.p85);let t86: f64 = (t84 / t85);let t87: f64 = (t82 + t86);let t88: f64 = (l.f645 * t87);let t89: f64 = (t88 - 230.25850929940458);let t8a: f64 = (l.f745 / l.f5f5);let t8b: f64 = (l.f743 - l.f741);let t8c: f64 = (t8b / l.f5f5);let t8d: f64 = (t8a - t8c);let t8e: f64 = (l.f5f5 - l.f5ed);let t8f: f64 = (l.f743 * t8e);let t90: f64 = (l.f5ed * p.p85);let t91: f64 = (t8f / t90);let t92: f64 = (t8d + t91);let t93: f64 = (l.f645 * t92);let t94: f64 = (t93 - 230.25850929940458);let t95: f64 = (t94 * 0.3333333333333333);let t96: f64 = (1.0 + t95);let t97: f64 = (t89 * t96);let t98: f64 = (0.5 * t97);let t99: f64 = (1.0 + t98);let t9a: f64 = (t7e * t99);let t9b: f64 = (1.0 + t9a);let t9c: f64 = (1e100 * t9b);
            (l.f81, l.f86, l.f87, ) = (t9c, (1e100 * (((l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t75 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t7a) - (t79 * (l.f5ee * p.p85))) / (t7a * t7a)))) * t99) + (t7e * (0.5 * (((l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t80 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t85) - (t84 * (l.f5ee * p.p85))) / (t85 * t85)))) * t96) + (t89 * ((l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t8b * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t90) - (t8f * (l.f5ee * p.p85))) / (t90 * t90)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t75 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t7a) - (t79 * (l.f5ef * p.p85))) / (t7a * t7a)))) * t99) + (t7e * (0.5 * (((l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t80 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t85) - (t84 * (l.f5ef * p.p85))) / (t85 * t85)))) * t96) + (t89 * ((l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t8b * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t90) - (t8f * (l.f5ef * p.p85))) / (t90 * t90)))) * 0.3333333333333333))))))), );l.f88 = 0.0;
        }
        if ((l.f48c != 0.0) && (l.f490 == 0.0)) {(l.f81, l.f86, l.f87, ) = (1.0, 0.0, 0.0, );l.f88 = 0.0;}
        let t9d: f64 = if ((p.p91 == 0.0) || (l.f745 < l.f741)) { 1.0 } else { 0.0 };l.f496 = t9d;l.f497 = 0.0;
        if ((l.f48c != 0.0) && (l.f496 != 0.0)) {let t9e: f64 = (l.f79 * p.p90);(l.f7a, l.f7b, l.f7c, ) = (t9e, (l.f7e * p.p90), (l.f7f * p.p90), );l.f7d = 0.0;}
        if ((l.f48c != 0.0) && (l.f496 == 0.0)) {let t9f: f64 = (l.f79 * p.p90);let ta0: f64 = (-p.p91);let ta1: f64 = (l.f745 - l.f741);let ta2: f64 = (ta0 * ta1);let ta3: f64 = (l.f745 - l.f741);let ta4: f64 = (ta2 * ta3);let ta5: f64 = (l.f6e9 / l.f6e7);let ta6: f64 = (ta5).ln();let ta7: f64 = (p.p98 * ta6);let ta8: f64 = (ta7).exp();let ta9: f64 = (ta4 * ta8);let taa: f64 = (ta9).exp();let tab: f64 = (t9f * taa);(l.f7a, l.f7b, l.f7c, ) = (tab, (((l.f7e * p.p90) * taa) + (t9f * (taa * ((((ta0 * l.f746) * ta3) + (ta2 * l.f746)) * ta8)))), (((l.f7f * p.p90) * taa) + (t9f * (taa * ((((ta0 * l.f747) * ta3) + (ta2 * l.f747)) * ta8)))), );l.f7d = 0.0;}
        if (l.f48c != 0.0) {
            let (tac, tad, tae,) = {
    if (l.f7a > p.p79) {
        (p.p79, 0.0, 0.0,)
    } else {
        (l.f7a, l.f7b, l.f7c,)
    }
};
            (l.f7a, l.f7b, l.f7c, ) = (tac, tad, tae, );l.f7d = 0.0;
        }
        if (l.f48c != 0.0) {let taf: f64 = (l.f64d * l.f7a);(l.f617, l.f618, l.f619, ) = (taf, (l.f64d * l.f7b), (l.f64d * l.f7c), );l.f61a = 0.0;let tb0: f64 = (1.6021918e-19 * l.f0);let tb1: f64 = (l.f617 - l.f64d);let tb2: f64 = (tb0 * tb1);(l.f66b, l.f66c, l.f66d, ) = (tb2, (tb0 * l.f618), (tb0 * l.f619), );l.f673 = 0.0;}
        let tb3: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };l.f498 = tb3;l.f499 = 0.0;
        if ((l.f48c != 0.0) && (l.f498 != 0.0)) {let tb4: f64 = (1e-23 / l.f669);let tb5: f64 = (l.f66b * tb4);(l.f67d, l.f67e, l.f67f, ) = (tb5, (l.f66c * tb4), (l.f66d * tb4), );l.f680 = 0.0;let tb6: f64 = (nv3 - 0.0);(l.f663, l.f664, ) = (tb6, 1.0, );l.f665 = 0.0;let tb7: f64 = (l.f663 - l.f67d);let tb8: f64 = (tb7 / p.p92);(l.f57c, l.f57d, l.f57e, l.f57f, ) = (tb8, ((-l.f67e) / p.p92), ((-l.f67f) / p.p92), (l.f664 / p.p92), );l.f580 = 0.0;let tb9: f64 = (1e-23 / l.f669);let tba: f64 = (l.f663 / tb9);(l.f66e, l.f66f, l.f670, l.f671, ) = (tba, 0.0, 0.0, (l.f664 / tb9), );l.f672 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_212(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        if ((l.f48c != 0.0) && (l.f498 == 0.0)) {(l.f67d, l.f67e, l.f67f, ) = (l.f66b, l.f66c, l.f66d, );l.f680 = 0.0;(l.f66e, l.f66f, l.f670, l.f671, ) = (l.f67d, l.f67e, l.f67f, 0.0, );l.f672 = 0.0;}
        let tbb: f64 = if ((p.p91 == 0.0) || (l.f745 < l.f743)) { 1.0 } else { 0.0 };l.f49a = tbb;l.f49b = 0.0;
        if ((l.f48c != 0.0) && (l.f49a != 0.0)) {let tbc: f64 = (l.f81 * p.p90);(l.f82, l.f83, l.f84, ) = (tbc, (l.f86 * p.p90), (l.f87 * p.p90), );l.f85 = 0.0;}
        if ((l.f48c != 0.0) && (l.f49a == 0.0)) {let tbd: f64 = (l.f81 * p.p90);let tbe: f64 = (-p.p91);let tbf: f64 = (l.f745 - l.f743);let tc0: f64 = (tbe * tbf);let tc1: f64 = (l.f745 - l.f743);let tc2: f64 = (tc0 * tc1);let tc3: f64 = (l.f6e9 / l.f6e7);let tc4: f64 = (tc3).ln();let tc5: f64 = (p.p98 * tc4);let tc6: f64 = (tc5).exp();let tc7: f64 = (tc2 * tc6);let tc8: f64 = (tc7).exp();let tc9: f64 = (tbd * tc8);(l.f82, l.f83, l.f84, ) = (tc9, (((l.f86 * p.p90) * tc8) + (tbd * (tc8 * ((((tbe * l.f746) * tc1) + (tc0 * l.f746)) * tc6)))), (((l.f87 * p.p90) * tc8) + (tbd * (tc8 * ((((tbe * l.f747) * tc1) + (tc0 * l.f747)) * tc6)))), );l.f85 = 0.0;}
        if (l.f48c != 0.0) {
            let (tca, tcb, tcc,) = {
    if (l.f82 > p.p79) {
        (p.p79, 0.0, 0.0,)
    } else {
        (l.f82, l.f83, l.f84,)
    }
};
            (l.f82, l.f83, l.f84, ) = (tca, tcb, tcc, );l.f85 = 0.0;
        }
        if (l.f48c != 0.0) {let tcd: f64 = (l.f64d * l.f82);(l.f61b, l.f61c, l.f61d, ) = (tcd, (l.f64d * l.f83), (l.f64d * l.f84), );l.f61e = 0.0;let tce: f64 = (1.6021918e-19 * l.f0);let tcf: f64 = (l.f61b - l.f64d);let td0: f64 = (tce * tcf);(l.f674, l.f675, l.f676, ) = (td0, (tce * l.f61c), (tce * l.f61d), );l.f67c = 0.0;}
        let td1: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };l.f49c = td1;l.f49d = 0.0;
        if ((l.f48c != 0.0) && (l.f49c != 0.0)) {let td2: f64 = (1e-23 / l.f669);let td3: f64 = (l.f674 * td2);(l.f681, l.f682, l.f683, ) = (td3, (l.f675 * td2), (l.f676 * td2), );l.f684 = 0.0;let td4: f64 = (nv4 - 0.0);(l.f666, l.f667, ) = (td4, 1.0, );l.f668 = 0.0;let td5: f64 = (l.f666 - l.f681);let td6: f64 = (td5 / p.p92);(l.f581, l.f582, l.f583, l.f584, ) = (td6, ((-l.f682) / p.p92), ((-l.f683) / p.p92), (l.f667 / p.p92), );l.f585 = 0.0;let td7: f64 = (1e-23 / l.f669);let td8: f64 = (l.f666 / td7);(l.f677, l.f678, l.f679, l.f67a, ) = (td8, 0.0, 0.0, (l.f667 / td7), );l.f67b = 0.0;}
        if ((l.f48c != 0.0) && (l.f49c == 0.0)) {(l.f681, l.f682, l.f683, ) = (l.f674, l.f675, l.f676, );l.f684 = 0.0;(l.f677, l.f678, l.f679, l.f67a, ) = (l.f681, l.f682, l.f683, 0.0, );l.f67b = 0.0;}
        if (l.f48c != 0.0) {let td9: f64 = (l.f61f - l.f745);(l.f7a7, l.f7a8, l.f7a9, ) = (td9, (-l.f746), (-l.f747), );l.f7aa = 0.0;let tda: f64 = (l.f7a7 * l.f7a7);let tdb: f64 = (4.0 * l.f5a3);let tdc: f64 = (tdb * l.f5a3);let tdd: f64 = (tda + tdc);let tde: f64 = (tdd).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tde, (((l.f7a8 * l.f7a7) + (l.f7a7 * l.f7a8)) / (2.0 * tde)), (((l.f7a9 * l.f7a7) + (l.f7a7 * l.f7a9)) / (2.0 * tde)), );l.f6fa = 0.0;let tdf: f64 = (l.f7a7 + l.f6f7);let te0: f64 = (0.5 * tdf);(l.f7a7, l.f7a8, l.f7a9, ) = (te0, (0.5 * (l.f7a8 + l.f6f8)), (0.5 * (l.f7a9 + l.f6f9)), );l.f7aa = 0.0;}
        let te1: f64 = if l.f7a7 < 0.0 { 1.0 } else { 0.0 };l.f49e = te1;l.f49f = 0.0;
        if ((l.f48c != 0.0) && (l.f49e != 0.0)) {(l.f7a7, l.f7a8, l.f7a9, ) = (0.0, 0.0, 0.0, );l.f7aa = 0.0;}
        if (l.f48c != 0.0) {let te2: f64 = (2.0 * l.f6b);let te3: f64 = (te2 * l.f7a7);let te4: f64 = (1.6021918e-19 * l.f5dd);let te5: f64 = (te3 / te4);let te6: f64 = (te5).sqrt();(l.f7bc, l.f7c1, l.f7c2, ) = (te6, (((te2 * l.f7a8) / te4) / (2.0 * te6)), (((te2 * l.f7a9) / te4) / (2.0 * te6)), );l.f7c8 = 0.0;let te7: f64 = (p.p94 - l.f7bc);let te8: f64 = (te7 - 1e-7);(l.f6f3, l.f6f4, l.f6f5, ) = (te8, (-l.f7c1), (-l.f7c2), );l.f6f6 = 0.0;let te9: f64 = (4.0 * p.p94);let tea: f64 = (te9 * 1e-7);(l.f6f7, l.f6f8, l.f6f9, ) = (tea, 0.0, 0.0, );l.f6fa = 0.0;}
        if (l.f48c != 0.0) {
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_213(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        if (l.f48c != 0.0) {let tef: f64 = (l.f6f3 * l.f6f3);let tf0: f64 = (tef + l.f6f7);let tf1: f64 = (tf0).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tf1, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tf1)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tf1)), );l.f6fa = 0.0;let tf2: f64 = (l.f6f3 + l.f6f7);let tf3: f64 = (0.5 * tf2);let tf4: f64 = (p.p94 - tf3);(l.f7bc, l.f7c1, l.f7c2, ) = (tf4, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f7c8 = 0.0;}
        let tf5: f64 = if p.p95 > 0.0 { 1.0 } else { 0.0 };l.f4a0 = tf5;l.f4a1 = 0.0;
        if ((l.f48c != 0.0) && (l.f4a0 != 0.0)) {let tf6: f64 = (1.0 / l.f7bd);let tf7: f64 = (l.f7bc * tf6);(l.f7cc, l.f7cd, l.f7ce, ) = (tf7, ((l.f7c1 * tf6) + (l.f7bc * (-(l.f7be / (l.f7bd * l.f7bd))))), ((l.f7c2 * tf6) + (l.f7bc * (-(l.f7bf / (l.f7bd * l.f7bd))))), );l.f7cf = 0.0;let tf8: f64 = (nv5 - 0.0);(l.f7c9, l.f7ca, ) = (tf8, 1.0, );l.f7cb = 0.0;let tf9: f64 = (l.f7c9 - l.f7cc);let tfa: f64 = (tf9 / p.p95);(l.f59e, l.f59f, l.f5a0, l.f5a1, ) = (tfa, ((-l.f7cd) / p.p95), ((-l.f7ce) / p.p95), (l.f7ca / p.p95), );l.f5a2 = 0.0;let tfb: f64 = (1.0 / l.f7bd);let tfc: f64 = (l.f7c9 / tfb);(l.f7c3, l.f7c4, l.f7c5, l.f7c6, ) = (tfc, (-((l.f7c9 * (-(l.f7be / (l.f7bd * l.f7bd)))) / (tfb * tfb))), (-((l.f7c9 * (-(l.f7bf / (l.f7bd * l.f7bd)))) / (tfb * tfb))), (l.f7ca / tfb), );l.f7c7 = 0.0;}
        if ((l.f48c != 0.0) && (l.f4a0 == 0.0)) {(l.f7cc, l.f7cd, l.f7ce, ) = (l.f7bc, l.f7c1, l.f7c2, );l.f7cf = 0.0;(l.f7c3, l.f7c4, l.f7c5, l.f7c6, ) = (l.f7cc, l.f7cd, l.f7ce, 0.0, );l.f7c7 = 0.0;}
        if (l.f48c != 0.0) {let tfd: f64 = (l.f5dd * l.f0);let tfe: f64 = (tfd * 1.6021918e-19);let tff: f64 = (-tfe);let t100: f64 = (tff * p.p94);l.f655 = t100;l.f656 = 0.0;let t101: f64 = (l.f5ad * l.f66e);let t102: f64 = (-p.p94);let t103: f64 = (t102 / l.f5ad);let t104: f64 = (t103).exp();let t105: f64 = (-l.f7c3);let t106: f64 = (t105 / l.f5ad);let t107: f64 = (t106).exp();let t108: f64 = (t104 - t107);let t109: f64 = (t101 * t108);(l.f657, l.f658, l.f659, l.f65a, l.f65b, ) = (t109, (((l.f5ad * l.f66f) * t108) + (t101 * (-(t107 * ((-l.f7c4) / l.f5ad))))), (((l.f5ad * l.f670) * t108) + (t101 * (-(t107 * ((-l.f7c5) / l.f5ad))))), ((l.f5ad * l.f671) * t108), (t101 * (-(t107 * ((-l.f7c6) / l.f5ad)))), );l.f65c = 0.0;let t10a: f64 = (l.f5ad * l.f677);let t10b: f64 = (p.p94 - l.f7c3);let t10c: f64 = (-t10b);let t10d: f64 = (t10c / l.f5ad);let t10e: f64 = (t10d).exp();let t10f: f64 = (t10e - 1.0);let t110: f64 = (t10a * t10f);(l.f65d, l.f65e, l.f65f, l.f660, l.f661, ) = (t110, (((l.f5ad * l.f678) * t10f) + (t10a * (t10e * ((-(-l.f7c4)) / l.f5ad)))), (((l.f5ad * l.f679) * t10f) + (t10a * (t10e * ((-(-l.f7c5)) / l.f5ad)))), ((l.f5ad * l.f67a) * t10f), (t10a * (t10e * ((-(-l.f7c6)) / l.f5ad))), );l.f662 = 0.0;let t111: f64 = (l.f655 + l.f657);let t112: f64 = (t111 + l.f65d);let t113: f64 = (-t112);(l.f6a4, l.f6a5, l.f6a6, l.f6a7, l.f6a8, l.f6a9, ) = (t113, (-(l.f658 + l.f65e)), (-(l.f659 + l.f65f)), (-l.f65a), (-l.f660), (-(l.f65b + l.f661)), );l.f6aa = 0.0;let t114: f64 = (l.f685 + l.f6a4);(l.f685, l.f686, l.f687, l.f688, l.f689, l.f68a, ) = (t114, (l.f686 + l.f6a5), (l.f687 + l.f6a6), (l.f688 + l.f6a7), (l.f689 + l.f6a8), (l.f68a + l.f6a9), );l.f68b = 0.0;l.f711 = 0.0;l.f712 = 0.0;}
        if (l.f48c == 0.0) {let t115: f64 = (l.f55e - l.f54e);let t116: f64 = (l.f711 * t115);(l.f6a4, l.f6a5, l.f6a6, l.f6a7, l.f6a8, l.f6a9, ) = (t116, (l.f711 * (l.f55f - l.f54f)), (l.f711 * (l.f560 - l.f550)), 0.0, 0.0, 0.0, );l.f6aa = 0.0;}
        let t117: f64 = if ((p.p84 > 0.0) && (p.p92 > 0.0)) { 1.0 } else { 0.0 };l.f4a3 = t117;l.f4a4 = 0.0;let t118: f64 = if ((p.p84 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };l.f4a5 = t118;l.f4a6 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let eq3_value: f64 = l.f55e;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq3_value),
            0,
            multiplicity * (l.f55f),
            2,
            multiplicity * (l.f560),
        );let eq4_e122: f64 = 0.0;let eq4_e124: f64 = (eq4_e122 * (nv0 - nv2));let eq4_value: f64 = eq4_e124;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq4_value),
            0,
            multiplicity * (eq4_e122),
            2,
            multiplicity * ((-eq4_e122)),
        );
        let (eq5_e130, eq5_e130_d_n1, eq5_e130_d_n2,) = {
    if (l.f4a2 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / l.f6af;let eq5_e128: f64 = (l.f7b9 * __rspice_inv_cse_0);let eq5_e128_d_n1: f64 = (l.f7ba * __rspice_inv_cse_0);let eq5_e128_d_n2: f64 = (l.f7bb * __rspice_inv_cse_0);
        (eq5_e128, eq5_e128_d_n1, eq5_e128_d_n2,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e130;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(1),
            multiplicity * (eq5_value),
            1,
            multiplicity * (eq5_e130_d_n1),
            2,
            multiplicity * (eq5_e130_d_n2),
        );
        let (eq6_e135,) = {
    if (l.f4a2 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e135;
        stamper.stamp_potential_const_local(
            0,
            eq6_value,
        );
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n2, eq7_e144_d_n3,) = {
    if (l.f4a3 != 0.0) {
        let eq7_e140: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, l.f663);let eq7_e141: f64 = (l.f57c + eq7_e140);let eq7_e141_d_n3: f64 = (l.f57f + (l.f664 * ddt_scale));let eq7_e142: f64 = (1e-12 * eq7_e141);let eq7_e142_d_n0: f64 = (1e-12 * l.f57d);let eq7_e142_d_n2: f64 = (1e-12 * l.f57e);let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n2, eq7_e142_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e144;
        stamper.stamp_current_node3_local(
            Some(3),
            None,
            multiplicity * (eq7_value),
            0,
            multiplicity * (eq7_e144_d_n0),
            2,
            multiplicity * (eq7_e144_d_n2),
            3,
            multiplicity * (eq7_e144_d_n3),
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n2, eq8_e153_d_n4,) = {
    if (l.f4a3 != 0.0) {
        let eq8_e149: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, l.f666);let eq8_e150: f64 = (l.f581 + eq8_e149);let eq8_e150_d_n4: f64 = (l.f584 + (l.f667 * ddt_scale));let eq8_e151: f64 = (1e-12 * eq8_e150);let eq8_e151_d_n0: f64 = (1e-12 * l.f582);let eq8_e151_d_n2: f64 = (1e-12 * l.f583);let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n2, eq8_e151_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e153;
        stamper.stamp_current_node3_local(
            Some(4),
            None,
            multiplicity * (eq8_value),
            0,
            multiplicity * (eq8_e153_d_n0),
            2,
            multiplicity * (eq8_e153_d_n2),
            4,
            multiplicity * (eq8_e153_d_n4),
        );
        let (eq9_e158,) = {
    if (l.f4a3 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e158;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );
        let (eq10_e163,) = {
    if (l.f4a3 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e163;
        stamper.stamp_potential_const_local(
            2,
            eq10_value,
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n2, eq11_e172_d_n5,) = {
    if (l.f4a5 != 0.0) {
        let eq11_e168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, l.f7c9);let eq11_e169: f64 = (l.f59e + eq11_e168);let eq11_e169_d_n5: f64 = (l.f5a1 + (l.f7ca * ddt_scale));let eq11_e170: f64 = (1e-13 * eq11_e169);let eq11_e170_d_n0: f64 = (1e-13 * l.f59f);let eq11_e170_d_n2: f64 = (1e-13 * l.f5a0);let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n2, eq11_e170_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e172;
        stamper.stamp_current_node3_local(
            Some(5),
            None,
            multiplicity * (eq11_value),
            0,
            multiplicity * (eq11_e172_d_n0),
            2,
            multiplicity * (eq11_e172_d_n2),
            5,
            multiplicity * (eq11_e172_d_n5),
        );
        let (eq12_e177,) = {
    if (l.f4a5 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e177;
        stamper.stamp_potential_const_local(
            3,
            eq12_value,
        );let eq13_e179: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, l.f685);let eq13_value: f64 = eq13_e179;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(0),
            Some(2),
            multiplicity * (eq13_value),
            [0, 2, 3, 4, 5],
            [multiplicity * ((l.f686 * ddt_scale)), multiplicity * ((l.f687 * ddt_scale)), multiplicity * ((l.f688 * ddt_scale)), multiplicity * ((l.f689 * ddt_scale)), multiplicity * ((l.f68a * ddt_scale))],
            [],
            [],
            1.0,
        );let eq14_e183: f64 = (l.f55e - l.f54e);let eq14_e183_d_n0: f64 = (l.f55f - l.f54f);let eq14_e183_d_n2: f64 = (l.f560 - l.f550);let eq14_e184: f64 = (l.f711 * eq14_e183);let eq14_e184_d_n0: f64 = (l.f711 * eq14_e183_d_n0);let eq14_e184_d_n2: f64 = (l.f711 * eq14_e183_d_n2);let eq14_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq14_e184);let eq14_value: f64 = eq14_e185;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq14_value),
            0,
            multiplicity * ((eq14_e184_d_n0 * ddt_scale)),
            2,
            multiplicity * ((eq14_e184_d_n2 * ddt_scale)),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_q, eq7_e144_q_d_n3,) = {
    if (l.f4a3 != 0.0) {
        let eq7_e140_q: f64 = l.f663;let eq7_e141: f64 = (l.f57c + l.f663);let eq7_e141_d_n3: f64 = (l.f57f + l.f664);let eq7_e141_q: f64 = eq7_e140_q;let eq7_e142: f64 = (1e-12 * eq7_e141);let eq7_e142_d_n0: f64 = (1e-12 * l.f57d);let eq7_e142_d_n2: f64 = (1e-12 * l.f57e);let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);let eq7_e142_q: f64 = (1e-12 * eq7_e141_q);let eq7_e142_q_d_n3: f64 = (1e-12 * l.f664);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_q, eq7_e142_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(3),
            None,
            3,
            multiplicity * (eq7_e144_q_d_n3),
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n2, eq8_e153_d_n4, eq8_e153_q, eq8_e153_q_d_n4,) = {
    if (l.f4a3 != 0.0) {
        let eq8_e149_q: f64 = l.f666;let eq8_e150: f64 = (l.f581 + l.f666);let eq8_e150_d_n4: f64 = (l.f584 + l.f667);let eq8_e150_q: f64 = eq8_e149_q;let eq8_e151: f64 = (1e-12 * eq8_e150);let eq8_e151_d_n0: f64 = (1e-12 * l.f582);let eq8_e151_d_n2: f64 = (1e-12 * l.f583);let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);let eq8_e151_q: f64 = (1e-12 * eq8_e150_q);let eq8_e151_q_d_n4: f64 = (1e-12 * l.f667);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n2, eq8_e151_d_n4, eq8_e151_q, eq8_e151_q_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            None,
            4,
            multiplicity * (eq8_e153_q_d_n4),
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n2, eq11_e172_d_n5, eq11_e172_q, eq11_e172_q_d_n5,) = {
    if (l.f4a5 != 0.0) {
        let eq11_e168_q: f64 = l.f7c9;let eq11_e169: f64 = (l.f59e + l.f7c9);let eq11_e169_d_n5: f64 = (l.f5a1 + l.f7ca);let eq11_e169_q: f64 = eq11_e168_q;let eq11_e170: f64 = (1e-13 * eq11_e169);let eq11_e170_d_n0: f64 = (1e-13 * l.f59f);let eq11_e170_d_n2: f64 = (1e-13 * l.f5a0);let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);let eq11_e170_q: f64 = (1e-13 * eq11_e169_q);let eq11_e170_q_d_n5: f64 = (1e-13 * l.f7ca);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n2, eq11_e170_d_n5, eq11_e170_q, eq11_e170_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(5),
            None,
            5,
            multiplicity * (eq11_e172_q_d_n5),
        );let eq13_e179_q: f64 = l.f685;let eq13_reactive_node_derivatives: [f64; 6] = [l.f686, 0.0, l.f687, l.f688, l.f689, l.f68a];let eq13_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(0),
            Some(2),
            &eq13_reactive_node_derivatives,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );let eq14_e183: f64 = (l.f55e - l.f54e);let eq14_e183_d_n0: f64 = (l.f55f - l.f54f);let eq14_e183_d_n2: f64 = (l.f560 - l.f550);let eq14_e184: f64 = (l.f711 * eq14_e183);let eq14_e184_d_n0: f64 = (l.f711 * eq14_e183_d_n0);let eq14_e184_d_n2: f64 = (l.f711 * eq14_e183_d_n2);let eq14_e185_q: f64 = eq14_e184;
        stamper.stamp_current_reactive_node2_local(
            Some(0),
            Some(2),
            0,
            multiplicity * (eq14_e184_d_n0),
            2,
            multiplicity * (eq14_e184_d_n2),
        );
    }
}
