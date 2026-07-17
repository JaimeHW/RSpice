#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f6 == 0.0)) && (l.f1f8 != 0.0)) {let t0: f64 = (-230.25850929940458);let t1: f64 = (l.f73b / l.f5f1);let t2: f64 = (l.f5f1 - l.f5ed);let t3: f64 = (l.f793 * t2);let t4: f64 = (l.f5ed * p.p85);let t5: f64 = (t3 / t4);let t6: f64 = (t1 + t5);let t7: f64 = (l.f645 * t6);let t8: f64 = (t0 - t7);let t9: f64 = (-230.25850929940458);let ta: f64 = (l.f73b / l.f5f1);let tb: f64 = (l.f5f1 - l.f5ed);let tc: f64 = (l.f793 * tb);let td: f64 = (l.f5ed * p.p85);let te: f64 = (tc / td);let tf: f64 = (ta + te);let t10: f64 = (l.f645 * tf);let t11: f64 = (t9 - t10);let t12: f64 = (-230.25850929940458);let t13: f64 = (l.f73b / l.f5f1);let t14: f64 = (l.f5f1 - l.f5ed);let t15: f64 = (l.f793 * t14);let t16: f64 = (l.f5ed * p.p85);let t17: f64 = (t15 / t16);let t18: f64 = (t13 + t17);let t19: f64 = (l.f645 * t18);let t1a: f64 = (t12 - t19);let t1b: f64 = (t1a * 0.3333333333333333);let t1c: f64 = (1.0 + t1b);let t1d: f64 = (t11 * t1c);let t1e: f64 = (0.5 * t1d);let t1f: f64 = (1.0 + t1e);let t20: f64 = (t8 * t1f);let t21: f64 = (1.0 + t20);let t22: f64 = (1e-100 / t21);(l.f536, l.f537, l.f538, ) = (t22, (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t4) - (t3 * (l.f5ee * p.p85))) / (t4 * t4))))) * t1f) + (t8 * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * td) - (tc * (l.f5ee * p.p85))) / (td * td))))) * t1c) + (t11 * ((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t16) - (t15 * (l.f5ee * p.p85))) / (t16 * t16))))) * 0.3333333333333333))))))) / (t21 * t21))), (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t4) - (t3 * (l.f5ef * p.p85))) / (t4 * t4))))) * t1f) + (t8 * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * td) - (tc * (l.f5ef * p.p85))) / (td * td))))) * t1c) + (t11 * ((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t16) - (t15 * (l.f5ef * p.p85))) / (t16 * t16))))) * 0.3333333333333333))))))) / (t21 * t21))), );}
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f6 == 0.0)) && (l.f1f8 == 0.0)) {let t23: f64 = (l.f73b / l.f5f1);let t24: f64 = (l.f5f1 - l.f5ed);let t25: f64 = (l.f793 * t24);let t26: f64 = (l.f5ed * p.p85);let t27: f64 = (t25 / t26);let t28: f64 = (t23 + t27);let t29: f64 = (l.f645 * t28);let t2a: f64 = (t29 - 230.25850929940458);let t2b: f64 = (l.f73b / l.f5f1);let t2c: f64 = (l.f5f1 - l.f5ed);let t2d: f64 = (l.f793 * t2c);let t2e: f64 = (l.f5ed * p.p85);let t2f: f64 = (t2d / t2e);let t30: f64 = (t2b + t2f);let t31: f64 = (l.f645 * t30);let t32: f64 = (t31 - 230.25850929940458);let t33: f64 = (l.f73b / l.f5f1);let t34: f64 = (l.f5f1 - l.f5ed);let t35: f64 = (l.f793 * t34);let t36: f64 = (l.f5ed * p.p85);let t37: f64 = (t35 / t36);let t38: f64 = (t33 + t37);let t39: f64 = (l.f645 * t38);let t3a: f64 = (t39 - 230.25850929940458);let t3b: f64 = (t3a * 0.3333333333333333);let t3c: f64 = (1.0 + t3b);let t3d: f64 = (t32 * t3c);let t3e: f64 = (0.5 * t3d);let t3f: f64 = (1.0 + t3e);let t40: f64 = (t2a * t3f);let t41: f64 = (1.0 + t40);let t42: f64 = (1e100 * t41);(l.f536, l.f537, l.f538, ) = (t42, (1e100 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t26) - (t25 * (l.f5ee * p.p85))) / (t26 * t26)))) * t3f) + (t2a * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2e) - (t2d * (l.f5ee * p.p85))) / (t2e * t2e)))) * t3c) + (t32 * ((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t36) - (t35 * (l.f5ee * p.p85))) / (t36 * t36)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t26) - (t25 * (l.f5ef * p.p85))) / (t26 * t26)))) * t3f) + (t2a * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2e) - (t2d * (l.f5ef * p.p85))) / (t2e * t2e)))) * t3c) + (t32 * ((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t36) - (t35 * (l.f5ef * p.p85))) / (t36 * t36)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) {let t43: f64 = (l.f5eb * l.f5eb);let t44: f64 = (t43 / l.f5e3);l.f64f = t44;let t45: f64 = (l.f5e9 / l.f645);let t46: f64 = (l.f5e3 / l.f64f);let t47: f64 = (t46).ln();let t48: f64 = (t45 * t47);l.f793 = t48;}
        let t49: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f1fa = t49;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {let t4a: f64 = (l.f73b - l.f793);let t4b: f64 = (p.p86 * t4a);let t4c: f64 = (t4b + l.f5e9);(l.f601, l.f602, l.f603, ) = (t4c, 0.0, 0.0, );let t4d: f64 = (p.p86 * l.f793);let t4e: f64 = (l.f5e9 - t4d);(l.f5ed, l.f5ee, l.f5ef, ) = (t4e, 0.0, 0.0, );let t4f: f64 = (p.p85 - l.f601);let t50: f64 = (t4f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t50, (-l.f602), (-l.f603), );let t51: f64 = (4.0 * p.p85);let t52: f64 = (t51 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t52, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {
            let (t54, t55, t56,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t53: f64 = (-l.f6f7);
        (t53, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t54, t55, t56, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {let t57: f64 = (l.f6f3 * l.f6f3);let t58: f64 = (t57 + l.f6f7);let t59: f64 = (t58).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t59, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t59)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t59)), );let t5a: f64 = (l.f6f3 + l.f6f7);let t5b: f64 = (0.5 * t5a);let t5c: f64 = (p.p85 - t5b);(l.f605, l.f606, l.f607, ) = (t5c, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t5d: f64 = (l.f605 - l.f5e9);let t5e: f64 = (t5d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t5e, l.f606, l.f607, );let t5f: f64 = (4.0 * l.f5e9);let t60: f64 = (t5f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t60, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {
            let (t62, t63, t64,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t61: f64 = (-l.f6f7);
        (t61, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t62, t63, t64, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {let t65: f64 = (l.f6f3 * l.f6f3);let t66: f64 = (t65 + l.f6f7);let t67: f64 = (t66).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t67, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t67)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t67)), );let t68: f64 = (l.f6f3 + l.f6f7);let t69: f64 = (0.5 * t68);let t6a: f64 = (l.f5e9 + t69);(l.f5f1, l.f5f2, l.f5f3, ) = (t6a, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t6b: f64 = (p.p85 - l.f5ed);let t6c: f64 = (t6b - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t6c, (-l.f5ee), (-l.f5ef), );let t6d: f64 = (4.0 * p.p85);let t6e: f64 = (t6d * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t6e, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {
            let (t70, t71, t72,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t6f: f64 = (-l.f6f7);
        (t6f, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t70, t71, t72, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {let t73: f64 = (l.f6f3 * l.f6f3);let t74: f64 = (t73 + l.f6f7);let t75: f64 = (t74).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t75, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t75)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t75)), );let t76: f64 = (l.f6f3 + l.f6f7);let t77: f64 = (0.5 * t76);let t78: f64 = (p.p85 - t77);(l.f5ed, l.f5ee, l.f5ef, ) = (t78, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t79: f64 = (l.f5ed - l.f5e9);let t7a: f64 = (t79 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t7a, l.f5ee, l.f5ef, );let t7b: f64 = (4.0 * l.f5e9);let t7c: f64 = (t7b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t7c, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {
            let (t7e, t7f, t80,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t7d: f64 = (-l.f6f7);
        (t7d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t7e, t7f, t80, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa != 0.0)) {let t81: f64 = (l.f6f3 * l.f6f3);let t82: f64 = (t81 + l.f6f7);let t83: f64 = (t82).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t83, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t83)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t83)), );let t84: f64 = (l.f6f3 + l.f6f7);let t85: f64 = (0.5 * t84);let t86: f64 = (l.f5e9 + t85);(l.f5ed, l.f5ee, l.f5ef, ) = (t86, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fa == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );}
        let t87: f64 = (l.f73b / l.f5f1);let t88: f64 = (l.f5f1 - l.f5ed);let t89: f64 = (l.f793 * t88);let t8a: f64 = (l.f5ed * p.p85);let t8b: f64 = (t89 / t8a);let t8c: f64 = (t87 + t8b);let t8d: f64 = (l.f645 * t8c);let t8e: f64 = (t8d).abs();let t8f: f64 = if t8e < 230.25850929940458 { 1.0 } else { 0.0 };l.f1fc = t8f;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fc != 0.0)) {let t90: f64 = (l.f73b / l.f5f1);let t91: f64 = (l.f5f1 - l.f5ed);let t92: f64 = (l.f793 * t91);let t93: f64 = (l.f5ed * p.p85);let t94: f64 = (t92 / t93);let t95: f64 = (t90 + t94);let t96: f64 = (l.f645 * t95);let t97: f64 = (t96).exp();(l.f53e, l.f53f, l.f540, ) = (t97, (t97 * (l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t93) - (t92 * (l.f5ee * p.p85))) / (t93 * t93))))), (t97 * (l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t93) - (t92 * (l.f5ef * p.p85))) / (t93 * t93))))), );}
        let t98: f64 = (l.f73b / l.f5f1);let t99: f64 = (l.f5f1 - l.f5ed);let t9a: f64 = (l.f793 * t99);let t9b: f64 = (l.f5ed * p.p85);let t9c: f64 = (t9a / t9b);let t9d: f64 = (t98 + t9c);let t9e: f64 = (l.f645 * t9d);let t9f: f64 = (-230.25850929940458);let ta0: f64 = if t9e < t9f { 1.0 } else { 0.0 };l.f1fe = ta0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fc == 0.0)) && (l.f1fe != 0.0)) {let ta1: f64 = (-230.25850929940458);let ta2: f64 = (l.f73b / l.f5f1);let ta3: f64 = (l.f5f1 - l.f5ed);let ta4: f64 = (l.f793 * ta3);let ta5: f64 = (l.f5ed * p.p85);let ta6: f64 = (ta4 / ta5);let ta7: f64 = (ta2 + ta6);let ta8: f64 = (l.f645 * ta7);let ta9: f64 = (ta1 - ta8);let taa: f64 = (-230.25850929940458);let tab: f64 = (l.f73b / l.f5f1);let tac: f64 = (l.f5f1 - l.f5ed);let tad: f64 = (l.f793 * tac);let tae: f64 = (l.f5ed * p.p85);let taf: f64 = (tad / tae);let tb0: f64 = (tab + taf);let tb1: f64 = (l.f645 * tb0);let tb2: f64 = (taa - tb1);let tb3: f64 = (-230.25850929940458);let tb4: f64 = (l.f73b / l.f5f1);let tb5: f64 = (l.f5f1 - l.f5ed);let tb6: f64 = (l.f793 * tb5);let tb7: f64 = (l.f5ed * p.p85);let tb8: f64 = (tb6 / tb7);let tb9: f64 = (tb4 + tb8);let tba: f64 = (l.f645 * tb9);let tbb: f64 = (tb3 - tba);let tbc: f64 = (tbb * 0.3333333333333333);let tbd: f64 = (1.0 + tbc);let tbe: f64 = (tb2 * tbd);let tbf: f64 = (0.5 * tbe);let tc0: f64 = (1.0 + tbf);let tc1: f64 = (ta9 * tc0);let tc2: f64 = (1.0 + tc1);let tc3: f64 = (1e-100 / tc2);(l.f53e, l.f53f, l.f540, ) = (tc3, (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * ta5) - (ta4 * (l.f5ee * p.p85))) / (ta5 * ta5))))) * tc0) + (ta9 * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tae) - (tad * (l.f5ee * p.p85))) / (tae * tae))))) * tbd) + (tb2 * ((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tb7) - (tb6 * (l.f5ee * p.p85))) / (tb7 * tb7))))) * 0.3333333333333333))))))) / (tc2 * tc2))), (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * ta5) - (ta4 * (l.f5ef * p.p85))) / (ta5 * ta5))))) * tc0) + (ta9 * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tae) - (tad * (l.f5ef * p.p85))) / (tae * tae))))) * tbd) + (tb2 * ((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tb7) - (tb6 * (l.f5ef * p.p85))) / (tb7 * tb7))))) * 0.3333333333333333))))))) / (tc2 * tc2))), );}
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1fc == 0.0)) && (l.f1fe == 0.0)) {let tc4: f64 = (l.f73b / l.f5f1);let tc5: f64 = (l.f5f1 - l.f5ed);let tc6: f64 = (l.f793 * tc5);let tc7: f64 = (l.f5ed * p.p85);let tc8: f64 = (tc6 / tc7);let tc9: f64 = (tc4 + tc8);let tca: f64 = (l.f645 * tc9);let tcb: f64 = (tca - 230.25850929940458);let tcc: f64 = (l.f73b / l.f5f1);let tcd: f64 = (l.f5f1 - l.f5ed);let tce: f64 = (l.f793 * tcd);let tcf: f64 = (l.f5ed * p.p85);let td0: f64 = (tce / tcf);let td1: f64 = (tcc + td0);let td2: f64 = (l.f645 * td1);let td3: f64 = (td2 - 230.25850929940458);let td4: f64 = (l.f73b / l.f5f1);let td5: f64 = (l.f5f1 - l.f5ed);let td6: f64 = (l.f793 * td5);let td7: f64 = (l.f5ed * p.p85);let td8: f64 = (td6 / td7);let td9: f64 = (td4 + td8);let tda: f64 = (l.f645 * td9);let tdb: f64 = (tda - 230.25850929940458);let tdc: f64 = (tdb * 0.3333333333333333);let tdd: f64 = (1.0 + tdc);let tde: f64 = (td3 * tdd);let tdf: f64 = (0.5 * tde);let te0: f64 = (1.0 + tdf);let te1: f64 = (tcb * te0);let te2: f64 = (1.0 + te1);let te3: f64 = (1e100 * te2);(l.f53e, l.f53f, l.f540, ) = (te3, (1e100 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tc7) - (tc6 * (l.f5ee * p.p85))) / (tc7 * tc7)))) * te0) + (tcb * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tcf) - (tce * (l.f5ee * p.p85))) / (tcf * tcf)))) * tdd) + (td3 * ((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * td7) - (td6 * (l.f5ee * p.p85))) / (td7 * td7)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tc7) - (tc6 * (l.f5ef * p.p85))) / (tc7 * tc7)))) * te0) + (tcb * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tcf) - (tce * (l.f5ef * p.p85))) / (tcf * tcf)))) * tdd) + (td3 * ((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * td7) - (td6 * (l.f5ef * p.p85))) / (td7 * td7)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) {let te4: f64 = (l.f5eb * l.f5eb);let te5: f64 = (te4 / l.f5e1);l.f64f = te5;let te6: f64 = (l.f5e7 / l.f645);let te7: f64 = (l.f5e1 / l.f64f);let te8: f64 = (te7).ln();let te9: f64 = (te6 * te8);l.f793 = te9;}
        let tea: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f201 = tea;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let teb: f64 = (l.f73b - l.f793);let tec: f64 = (p.p86 * teb);let ted: f64 = (tec + l.f5e7);(l.f601, l.f602, l.f603, ) = (ted, 0.0, 0.0, );let tee: f64 = (p.p86 * l.f793);let tef: f64 = (l.f5e7 - tee);(l.f5ed, l.f5ee, l.f5ef, ) = (tef, 0.0, 0.0, );let tf0: f64 = (p.p85 - l.f601);let tf1: f64 = (tf0 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tf1, (-l.f602), (-l.f603), );let tf2: f64 = (4.0 * p.p85);let tf3: f64 = (tf2 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tf3, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {
            let (tf5, tf6, tf7,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tf4: f64 = (-l.f6f7);
        (tf4, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tf5, tf6, tf7, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let tf8: f64 = (l.f6f3 * l.f6f3);let tf9: f64 = (tf8 + l.f6f7);let tfa: f64 = (tf9).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tfa, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tfa)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tfa)), );let tfb: f64 = (l.f6f3 + l.f6f7);let tfc: f64 = (0.5 * tfb);let tfd: f64 = (p.p85 - tfc);(l.f605, l.f606, l.f607, ) = (tfd, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let tfe: f64 = (l.f605 - l.f5e7);let tff: f64 = (tfe - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tff, l.f606, l.f607, );let t100: f64 = (4.0 * l.f5e7);let t101: f64 = (t100 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t101, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {
            let (t103, t104, t105,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t102: f64 = (-l.f6f7);
        (t102, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t103, t104, t105, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t106: f64 = (l.f6f3 * l.f6f3);let t107: f64 = (t106 + l.f6f7);let t108: f64 = (t107).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t108, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t108)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t108)), );let t109: f64 = (l.f6f3 + l.f6f7);let t10a: f64 = (0.5 * t109);let t10b: f64 = (l.f5e7 + t10a);(l.f5f1, l.f5f2, l.f5f3, ) = (t10b, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t10c: f64 = (p.p85 - l.f5ed);let t10d: f64 = (t10c - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t10d, (-l.f5ee), (-l.f5ef), );let t10e: f64 = (4.0 * p.p85);let t10f: f64 = (t10e * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t10f, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {
            let (t111, t112, t113,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t110: f64 = (-l.f6f7);
        (t110, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t111, t112, t113, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t114: f64 = (l.f6f3 * l.f6f3);let t115: f64 = (t114 + l.f6f7);let t116: f64 = (t115).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t116, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t116)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t116)), );let t117: f64 = (l.f6f3 + l.f6f7);let t118: f64 = (0.5 * t117);let t119: f64 = (p.p85 - t118);(l.f5ed, l.f5ee, l.f5ef, ) = (t119, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t11a: f64 = (l.f5ed - l.f5e7);let t11b: f64 = (t11a - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t11b, l.f5ee, l.f5ef, );let t11c: f64 = (4.0 * l.f5e7);let t11d: f64 = (t11c * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t11d, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {
            let (t11f, t120, t121,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t11e: f64 = (-l.f6f7);
        (t11e, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t11f, t120, t121, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 != 0.0)) {let t122: f64 = (l.f6f3 * l.f6f3);let t123: f64 = (t122 + l.f6f7);let t124: f64 = (t123).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t124, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t124)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t124)), );let t125: f64 = (l.f6f3 + l.f6f7);let t126: f64 = (0.5 * t125);let t127: f64 = (l.f5e7 + t126);(l.f5ed, l.f5ee, l.f5ef, ) = (t127, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f201 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );}
        let t128: f64 = (l.f73b / l.f5f1);let t129: f64 = (l.f5f1 - l.f5ed);let t12a: f64 = (l.f793 * t129);let t12b: f64 = (l.f5ed * p.p85);let t12c: f64 = (t12a / t12b);let t12d: f64 = (t128 + t12c);let t12e: f64 = (l.f645 * t12d);let t12f: f64 = (t12e).abs();let t130: f64 = if t12f < 230.25850929940458 { 1.0 } else { 0.0 };l.f203 = t130;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f203 != 0.0)) {let t131: f64 = (l.f73b / l.f5f1);let t132: f64 = (l.f5f1 - l.f5ed);let t133: f64 = (l.f793 * t132);let t134: f64 = (l.f5ed * p.p85);let t135: f64 = (t133 / t134);let t136: f64 = (t131 + t135);let t137: f64 = (l.f645 * t136);let t138: f64 = (t137).exp();(l.f53a, l.f53b, l.f53c, ) = (t138, (t138 * (l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t134) - (t133 * (l.f5ee * p.p85))) / (t134 * t134))))), (t138 * (l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t134) - (t133 * (l.f5ef * p.p85))) / (t134 * t134))))), );}
        let t139: f64 = (l.f73b / l.f5f1);let t13a: f64 = (l.f5f1 - l.f5ed);let t13b: f64 = (l.f793 * t13a);let t13c: f64 = (l.f5ed * p.p85);let t13d: f64 = (t13b / t13c);let t13e: f64 = (t139 + t13d);let t13f: f64 = (l.f645 * t13e);let t140: f64 = (-230.25850929940458);let t141: f64 = if t13f < t140 { 1.0 } else { 0.0 };l.f205 = t141;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f203 == 0.0)) && (l.f205 != 0.0)) {let t142: f64 = (-230.25850929940458);let t143: f64 = (l.f73b / l.f5f1);let t144: f64 = (l.f5f1 - l.f5ed);let t145: f64 = (l.f793 * t144);let t146: f64 = (l.f5ed * p.p85);let t147: f64 = (t145 / t146);let t148: f64 = (t143 + t147);let t149: f64 = (l.f645 * t148);let t14a: f64 = (t142 - t149);let t14b: f64 = (-230.25850929940458);let t14c: f64 = (l.f73b / l.f5f1);let t14d: f64 = (l.f5f1 - l.f5ed);let t14e: f64 = (l.f793 * t14d);let t14f: f64 = (l.f5ed * p.p85);let t150: f64 = (t14e / t14f);let t151: f64 = (t14c + t150);let t152: f64 = (l.f645 * t151);let t153: f64 = (t14b - t152);let t154: f64 = (-230.25850929940458);let t155: f64 = (l.f73b / l.f5f1);let t156: f64 = (l.f5f1 - l.f5ed);let t157: f64 = (l.f793 * t156);let t158: f64 = (l.f5ed * p.p85);let t159: f64 = (t157 / t158);let t15a: f64 = (t155 + t159);let t15b: f64 = (l.f645 * t15a);let t15c: f64 = (t154 - t15b);let t15d: f64 = (t15c * 0.3333333333333333);let t15e: f64 = (1.0 + t15d);let t15f: f64 = (t153 * t15e);let t160: f64 = (0.5 * t15f);let t161: f64 = (1.0 + t160);let t162: f64 = (t14a * t161);let t163: f64 = (1.0 + t162);let t164: f64 = (1e-100 / t163);(l.f53a, l.f53b, l.f53c, ) = (t164, (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t146) - (t145 * (l.f5ee * p.p85))) / (t146 * t146))))) * t161) + (t14a * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t14f) - (t14e * (l.f5ee * p.p85))) / (t14f * t14f))))) * t15e) + (t153 * ((-(l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t158) - (t157 * (l.f5ee * p.p85))) / (t158 * t158))))) * 0.3333333333333333))))))) / (t163 * t163))), (-((1e-100 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t146) - (t145 * (l.f5ef * p.p85))) / (t146 * t146))))) * t161) + (t14a * (0.5 * (((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t14f) - (t14e * (l.f5ef * p.p85))) / (t14f * t14f))))) * t15e) + (t153 * ((-(l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t158) - (t157 * (l.f5ef * p.p85))) / (t158 * t158))))) * 0.3333333333333333))))))) / (t163 * t163))), );}
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f203 == 0.0)) && (l.f205 == 0.0)) {let t165: f64 = (l.f73b / l.f5f1);let t166: f64 = (l.f5f1 - l.f5ed);let t167: f64 = (l.f793 * t166);let t168: f64 = (l.f5ed * p.p85);let t169: f64 = (t167 / t168);let t16a: f64 = (t165 + t169);let t16b: f64 = (l.f645 * t16a);let t16c: f64 = (t16b - 230.25850929940458);let t16d: f64 = (l.f73b / l.f5f1);let t16e: f64 = (l.f5f1 - l.f5ed);let t16f: f64 = (l.f793 * t16e);let t170: f64 = (l.f5ed * p.p85);let t171: f64 = (t16f / t170);let t172: f64 = (t16d + t171);let t173: f64 = (l.f645 * t172);let t174: f64 = (t173 - 230.25850929940458);let t175: f64 = (l.f73b / l.f5f1);let t176: f64 = (l.f5f1 - l.f5ed);let t177: f64 = (l.f793 * t176);let t178: f64 = (l.f5ed * p.p85);let t179: f64 = (t177 / t178);let t17a: f64 = (t175 + t179);let t17b: f64 = (l.f645 * t17a);let t17c: f64 = (t17b - 230.25850929940458);let t17d: f64 = (t17c * 0.3333333333333333);let t17e: f64 = (1.0 + t17d);let t17f: f64 = (t174 * t17e);let t180: f64 = (0.5 * t17f);let t181: f64 = (1.0 + t180);let t182: f64 = (t16c * t181);let t183: f64 = (1.0 + t182);let t184: f64 = (1e100 * t183);(l.f53a, l.f53b, l.f53c, ) = (t184, (1e100 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t168) - (t167 * (l.f5ee * p.p85))) / (t168 * t168)))) * t181) + (t16c * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t170) - (t16f * (l.f5ee * p.p85))) / (t170 * t170)))) * t17e) + (t174 * ((l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t178) - (t177 * (l.f5ee * p.p85))) / (t178 * t178)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t168) - (t167 * (l.f5ef * p.p85))) / (t168 * t168)))) * t181) + (t16c * (0.5 * (((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t170) - (t16f * (l.f5ef * p.p85))) / (t170 * t170)))) * t17e) + (t174 * ((l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t178) - (t177 * (l.f5ef * p.p85))) / (t178 * t178)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) {let t185: f64 = (l.f73b - l.f7b1);let t186: f64 = (t185 * l.f645);let t187: f64 = (1.0 + t186);let t188: f64 = (t187 * l.f89);let t189: f64 = (t188).sqrt();l.f825 = t189;let t18a: f64 = (l.f5eb * l.f5eb);let t18b: f64 = (t18a / l.f5df);l.f64f = t18b;let t18c: f64 = (l.f5e5 / l.f645);let t18d: f64 = (l.f5df / l.f64f);let t18e: f64 = (t18d).ln();let t18f: f64 = (t18c * t18e);l.f793 = t18f;}
        let t190: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f207 = t190;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t191: f64 = (l.f7b1 - l.f793);let t192: f64 = (p.p86 * t191);let t193: f64 = (t192 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t193, 0.0, 0.0, );let t194: f64 = (p.p86 * l.f793);let t195: f64 = (l.f5e5 - t194);(l.f5ed, l.f5ee, l.f5ef, ) = (t195, 0.0, 0.0, );let t196: f64 = (p.p85 - l.f601);let t197: f64 = (t196 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t197, (-l.f602), (-l.f603), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t198: f64 = (4.0 * p.p85);let t199: f64 = (t198 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t199, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {
            let (t19b, t19c, t19d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t19a: f64 = (-l.f6f7);
        (t19a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t19b, t19c, t19d, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t19e: f64 = (l.f6f3 * l.f6f3);let t19f: f64 = (t19e + l.f6f7);let t1a0: f64 = (t19f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1a0, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1a0)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1a0)), );let t1a1: f64 = (l.f6f3 / l.f6f7);let t1a2: f64 = (1.0 + t1a1);let t1a3: f64 = (0.5 * t1a2);(l.f55, l.f56, l.f57, ) = (t1a3, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t1a4: f64 = (l.f6f3 + l.f6f7);let t1a5: f64 = (0.5 * t1a4);let t1a6: f64 = (p.p85 - t1a5);(l.f605, l.f606, l.f607, ) = (t1a6, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t1a7: f64 = (l.f605 - l.f5e5);let t1a8: f64 = (t1a7 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1a8, l.f606, l.f607, );let t1a9: f64 = (4.0 * l.f5e5);let t1aa: f64 = (t1a9 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1aa, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {
            let (t1ac, t1ad, t1ae,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1ab: f64 = (-l.f6f7);
        (t1ab, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1ac, t1ad, t1ae, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t1af: f64 = (l.f6f3 * l.f6f3);let t1b0: f64 = (t1af + l.f6f7);let t1b1: f64 = (t1b0).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1b1, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1b1)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1b1)), );let t1b2: f64 = (l.f6f3 / l.f6f7);let t1b3: f64 = (1.0 + t1b2);let t1b4: f64 = (0.5 * t1b3);(l.f51, l.f52, l.f53, ) = (t1b4, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t1b5: f64 = (l.f6f3 + l.f6f7);let t1b6: f64 = (0.5 * t1b5);let t1b7: f64 = (l.f5e5 + t1b6);(l.f5f1, l.f5f2, l.f5f3, ) = (t1b7, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t1b8: f64 = (p.p85 - l.f5ed);let t1b9: f64 = (t1b8 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1b9, (-l.f5ee), (-l.f5ef), );let t1ba: f64 = (4.0 * p.p85);let t1bb: f64 = (t1ba * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1bb, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {
            let (t1bd, t1be, t1bf,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1bc: f64 = (-l.f6f7);
        (t1bc, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1bd, t1be, t1bf, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t1c0: f64 = (l.f6f3 * l.f6f3);let t1c1: f64 = (t1c0 + l.f6f7);let t1c2: f64 = (t1c1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1c2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1c2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1c2)), );let t1c3: f64 = (l.f6f3 + l.f6f7);let t1c4: f64 = (0.5 * t1c3);let t1c5: f64 = (p.p85 - t1c4);(l.f5ed, l.f5ee, l.f5ef, ) = (t1c5, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t1c6: f64 = (l.f5ed - l.f5e5);let t1c7: f64 = (t1c6 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1c7, l.f5ee, l.f5ef, );let t1c8: f64 = (4.0 * l.f5e5);let t1c9: f64 = (t1c8 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1c9, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {
            let (t1cb, t1cc, t1cd,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1ca: f64 = (-l.f6f7);
        (t1ca, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1cb, t1cc, t1cd, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 != 0.0)) {let t1ce: f64 = (l.f6f3 * l.f6f3);let t1cf: f64 = (t1ce + l.f6f7);let t1d0: f64 = (t1cf).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1d0, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1d0)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1d0)), );let t1d1: f64 = (l.f6f3 + l.f6f7);let t1d2: f64 = (0.5 * t1d1);let t1d3: f64 = (l.f5e5 + t1d2);(l.f5ed, l.f5ee, l.f5ef, ) = (t1d3, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t1d4: f64 = (p.p86 * l.f55);let t1d5: f64 = (t1d4 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t1d5, (((p.p86 * l.f56) * l.f51) + (t1d4 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t1d4 * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f207 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t1d6: f64 = (l.f7b1 / l.f5f1);let t1d7: f64 = (l.f5f1 - l.f5ed);let t1d8: f64 = (l.f793 * t1d7);let t1d9: f64 = (l.f5ed * p.p85);let t1da: f64 = (t1d8 / t1d9);let t1db: f64 = (t1d6 + t1da);let t1dc: f64 = (l.f645 * t1db);let t1dd: f64 = (t1dc).abs();let t1de: f64 = if t1dd < 230.25850929940458 { 1.0 } else { 0.0 };l.f209 = t1de;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f209 != 0.0)) {let t1df: f64 = (l.f7b1 / l.f5f1);let t1e0: f64 = (l.f5f1 - l.f5ed);let t1e1: f64 = (l.f793 * t1e0);let t1e2: f64 = (l.f5ed * p.p85);let t1e3: f64 = (t1e1 / t1e2);let t1e4: f64 = (t1df + t1e3);let t1e5: f64 = (l.f645 * t1e4);let t1e6: f64 = (t1e5).exp();(l.f8a, l.f8b, l.f8c, ) = (t1e6, (t1e6 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e2) - (t1e1 * (l.f5ee * p.p85))) / (t1e2 * t1e2))))), (t1e6 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e2) - (t1e1 * (l.f5ef * p.p85))) / (t1e2 * t1e2))))), );}
        let t1e7: f64 = (l.f7b1 / l.f5f1);let t1e8: f64 = (l.f5f1 - l.f5ed);let t1e9: f64 = (l.f793 * t1e8);let t1ea: f64 = (l.f5ed * p.p85);let t1eb: f64 = (t1e9 / t1ea);let t1ec: f64 = (t1e7 + t1eb);let t1ed: f64 = (l.f645 * t1ec);let t1ee: f64 = (-230.25850929940458);let t1ef: f64 = if t1ed < t1ee { 1.0 } else { 0.0 };l.f20b = t1ef;
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f209 == 0.0)) && (l.f20b != 0.0)) {let t1f0: f64 = (-230.25850929940458);let t1f1: f64 = (l.f7b1 / l.f5f1);let t1f2: f64 = (l.f5f1 - l.f5ed);let t1f3: f64 = (l.f793 * t1f2);let t1f4: f64 = (l.f5ed * p.p85);let t1f5: f64 = (t1f3 / t1f4);let t1f6: f64 = (t1f1 + t1f5);let t1f7: f64 = (l.f645 * t1f6);let t1f8: f64 = (t1f0 - t1f7);let t1f9: f64 = (-230.25850929940458);let t1fa: f64 = (l.f7b1 / l.f5f1);let t1fb: f64 = (l.f5f1 - l.f5ed);let t1fc: f64 = (l.f793 * t1fb);let t1fd: f64 = (l.f5ed * p.p85);let t1fe: f64 = (t1fc / t1fd);let t1ff: f64 = (t1fa + t1fe);let t200: f64 = (l.f645 * t1ff);let t201: f64 = (t1f9 - t200);let t202: f64 = (-230.25850929940458);let t203: f64 = (l.f7b1 / l.f5f1);let t204: f64 = (l.f5f1 - l.f5ed);let t205: f64 = (l.f793 * t204);let t206: f64 = (l.f5ed * p.p85);let t207: f64 = (t205 / t206);let t208: f64 = (t203 + t207);let t209: f64 = (l.f645 * t208);let t20a: f64 = (t202 - t209);let t20b: f64 = (t20a * 0.3333333333333333);let t20c: f64 = (1.0 + t20b);let t20d: f64 = (t201 * t20c);let t20e: f64 = (0.5 * t20d);let t20f: f64 = (1.0 + t20e);let t210: f64 = (t1f8 * t20f);let t211: f64 = (1.0 + t210);let t212: f64 = (1e-100 / t211);(l.f8a, l.f8b, l.f8c, ) = (t212, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1f4) - (t1f3 * (l.f5ee * p.p85))) / (t1f4 * t1f4))))) * t20f) + (t1f8 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1fd) - (t1fc * (l.f5ee * p.p85))) / (t1fd * t1fd))))) * t20c) + (t201 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t206) - (t205 * (l.f5ee * p.p85))) / (t206 * t206))))) * 0.3333333333333333))))))) / (t211 * t211))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1f4) - (t1f3 * (l.f5ef * p.p85))) / (t1f4 * t1f4))))) * t20f) + (t1f8 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1fd) - (t1fc * (l.f5ef * p.p85))) / (t1fd * t1fd))))) * t20c) + (t201 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t206) - (t205 * (l.f5ef * p.p85))) / (t206 * t206))))) * 0.3333333333333333))))))) / (t211 * t211))), );}
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f209 == 0.0)) && (l.f20b == 0.0)) {let t213: f64 = (l.f7b1 / l.f5f1);let t214: f64 = (l.f5f1 - l.f5ed);let t215: f64 = (l.f793 * t214);let t216: f64 = (l.f5ed * p.p85);let t217: f64 = (t215 / t216);let t218: f64 = (t213 + t217);let t219: f64 = (l.f645 * t218);let t21a: f64 = (t219 - 230.25850929940458);let t21b: f64 = (l.f7b1 / l.f5f1);let t21c: f64 = (l.f5f1 - l.f5ed);let t21d: f64 = (l.f793 * t21c);let t21e: f64 = (l.f5ed * p.p85);let t21f: f64 = (t21d / t21e);let t220: f64 = (t21b + t21f);let t221: f64 = (l.f645 * t220);let t222: f64 = (t221 - 230.25850929940458);let t223: f64 = (l.f7b1 / l.f5f1);let t224: f64 = (l.f5f1 - l.f5ed);let t225: f64 = (l.f793 * t224);let t226: f64 = (l.f5ed * p.p85);let t227: f64 = (t225 / t226);let t228: f64 = (t223 + t227);let t229: f64 = (l.f645 * t228);let t22a: f64 = (t229 - 230.25850929940458);let t22b: f64 = (t22a * 0.3333333333333333);let t22c: f64 = (1.0 + t22b);let t22d: f64 = (t222 * t22c);let t22e: f64 = (0.5 * t22d);let t22f: f64 = (1.0 + t22e);let t230: f64 = (t21a * t22f);let t231: f64 = (1.0 + t230);let t232: f64 = (1e100 * t231);(l.f8a, l.f8b, l.f8c, ) = (t232, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t216) - (t215 * (l.f5ee * p.p85))) / (t216 * t216)))) * t22f) + (t21a * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t21e) - (t21d * (l.f5ee * p.p85))) / (t21e * t21e)))) * t22c) + (t222 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t226) - (t225 * (l.f5ee * p.p85))) / (t226 * t226)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t216) - (t215 * (l.f5ef * p.p85))) / (t216 * t216)))) * t22f) + (t21a * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t21e) - (t21d * (l.f5ef * p.p85))) / (t21e * t21e)))) * t22c) + (t222 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t226) - (t225 * (l.f5ef * p.p85))) / (t226 * t226)))) * 0.3333333333333333))))))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) {let t233: f64 = (l.f7b1 * l.f5b);let t234: f64 = (l.f5f1 - t233);let t235: f64 = (l.f5f1 * l.f5f1);let t236: f64 = (t234 / t235);let t237: f64 = (l.f793 * l.f5b);let t238: f64 = (l.f5ed * p.p85);let t239: f64 = (t237 / t238);let t23a: f64 = (t236 + t239);let t23b: f64 = (l.f645 * t23a);(l.f61, l.f62, l.f63, ) = (t23b, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t235) - (t234 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t235 * t235)) + ((((l.f793 * l.f5c) * t238) - (t237 * (l.f5ee * p.p85))) / (t238 * t238)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t235) - (t234 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t235 * t235)) + ((((l.f793 * l.f5d) * t238) - (t237 * (l.f5ef * p.p85))) / (t238 * t238)))), );let t23c: f64 = (l.f73b - l.f7b1);let t23d: f64 = (t23c * l.f61);let t23e: f64 = (1.0 + t23d);let t23f: f64 = (t23e * l.f8a);(l.f536, l.f537, l.f538, ) = (t23f, (((t23c * l.f62) * l.f8a) + (t23e * l.f8b)), (((t23c * l.f63) * l.f8a) + (t23e * l.f8c)), );let t240: f64 = (l.f5eb * l.f5eb);let t241: f64 = (t240 / l.f5e3);l.f64f = t241;let t242: f64 = (l.f5e9 / l.f645);let t243: f64 = (l.f5e3 / l.f64f);let t244: f64 = (t243).ln();let t245: f64 = (t242 * t244);l.f793 = t245;}
        let t246: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f20d = t246;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t247: f64 = (l.f7b1 - l.f793);let t248: f64 = (p.p86 * t247);let t249: f64 = (t248 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t249, 0.0, 0.0, );let t24a: f64 = (p.p86 * l.f793);let t24b: f64 = (l.f5e9 - t24a);(l.f5ed, l.f5ee, l.f5ef, ) = (t24b, 0.0, 0.0, );let t24c: f64 = (p.p85 - l.f601);let t24d: f64 = (t24c - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t24d, (-l.f602), (-l.f603), );let t24e: f64 = (4.0 * p.p85);let t24f: f64 = (t24e * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t24f, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {
            let (t251, t252, t253,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t250: f64 = (-l.f6f7);
        (t250, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t251, t252, t253, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t254: f64 = (l.f6f3 * l.f6f3);let t255: f64 = (t254 + l.f6f7);let t256: f64 = (t255).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t256, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t256)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t256)), );let t257: f64 = (l.f6f3 / l.f6f7);let t258: f64 = (1.0 + t257);let t259: f64 = (0.5 * t258);(l.f55, l.f56, l.f57, ) = (t259, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t25a: f64 = (l.f6f3 + l.f6f7);let t25b: f64 = (0.5 * t25a);let t25c: f64 = (p.p85 - t25b);(l.f605, l.f606, l.f607, ) = (t25c, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t25d: f64 = (l.f605 - l.f5e9);let t25e: f64 = (t25d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t25e, l.f606, l.f607, );let t25f: f64 = (4.0 * l.f5e9);let t260: f64 = (t25f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t260, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {
            let (t262, t263, t264,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t261: f64 = (-l.f6f7);
        (t261, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t262, t263, t264, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t265: f64 = (l.f6f3 * l.f6f3);let t266: f64 = (t265 + l.f6f7);let t267: f64 = (t266).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t267, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t267)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t267)), );let t268: f64 = (l.f6f3 / l.f6f7);let t269: f64 = (1.0 + t268);let t26a: f64 = (0.5 * t269);(l.f51, l.f52, l.f53, ) = (t26a, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t26b: f64 = (l.f6f3 + l.f6f7);let t26c: f64 = (0.5 * t26b);let t26d: f64 = (l.f5e9 + t26c);(l.f5f1, l.f5f2, l.f5f3, ) = (t26d, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t26e: f64 = (p.p85 - l.f5ed);let t26f: f64 = (t26e - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t26f, (-l.f5ee), (-l.f5ef), );let t270: f64 = (4.0 * p.p85);let t271: f64 = (t270 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t271, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {
            let (t273, t274, t275,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t272: f64 = (-l.f6f7);
        (t272, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t273, t274, t275, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t276: f64 = (l.f6f3 * l.f6f3);let t277: f64 = (t276 + l.f6f7);let t278: f64 = (t277).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t278, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t278)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t278)), );let t279: f64 = (l.f6f3 + l.f6f7);let t27a: f64 = (0.5 * t279);let t27b: f64 = (p.p85 - t27a);(l.f5ed, l.f5ee, l.f5ef, ) = (t27b, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t27c: f64 = (l.f5ed - l.f5e9);let t27d: f64 = (t27c - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t27d, l.f5ee, l.f5ef, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t27e: f64 = (4.0 * l.f5e9);let t27f: f64 = (t27e * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t27f, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {
            let (t281, t282, t283,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t280: f64 = (-l.f6f7);
        (t280, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t281, t282, t283, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d != 0.0)) {let t284: f64 = (l.f6f3 * l.f6f3);let t285: f64 = (t284 + l.f6f7);let t286: f64 = (t285).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t286, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t286)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t286)), );let t287: f64 = (l.f6f3 + l.f6f7);let t288: f64 = (0.5 * t287);let t289: f64 = (l.f5e9 + t288);(l.f5ed, l.f5ee, l.f5ef, ) = (t289, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t28a: f64 = (p.p86 * l.f55);let t28b: f64 = (t28a * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t28b, (((p.p86 * l.f56) * l.f51) + (t28a * l.f52)), (((p.p86 * l.f57) * l.f51) + (t28a * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20d == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t28c: f64 = (l.f7b1 / l.f5f1);let t28d: f64 = (l.f5f1 - l.f5ed);let t28e: f64 = (l.f793 * t28d);let t28f: f64 = (l.f5ed * p.p85);let t290: f64 = (t28e / t28f);let t291: f64 = (t28c + t290);let t292: f64 = (l.f645 * t291);let t293: f64 = (t292).abs();let t294: f64 = if t293 < 230.25850929940458 { 1.0 } else { 0.0 };l.f20f = t294;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20f != 0.0)) {let t295: f64 = (l.f7b1 / l.f5f1);let t296: f64 = (l.f5f1 - l.f5ed);let t297: f64 = (l.f793 * t296);let t298: f64 = (l.f5ed * p.p85);let t299: f64 = (t297 / t298);let t29a: f64 = (t295 + t299);let t29b: f64 = (l.f645 * t29a);let t29c: f64 = (t29b).exp();(l.f93, l.f94, l.f95, ) = (t29c, (t29c * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t298) - (t297 * (l.f5ee * p.p85))) / (t298 * t298))))), (t29c * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t298) - (t297 * (l.f5ef * p.p85))) / (t298 * t298))))), );}
        let t29d: f64 = (l.f7b1 / l.f5f1);let t29e: f64 = (l.f5f1 - l.f5ed);let t29f: f64 = (l.f793 * t29e);let t2a0: f64 = (l.f5ed * p.p85);let t2a1: f64 = (t29f / t2a0);let t2a2: f64 = (t29d + t2a1);let t2a3: f64 = (l.f645 * t2a2);let t2a4: f64 = (-230.25850929940458);let t2a5: f64 = if t2a3 < t2a4 { 1.0 } else { 0.0 };l.f211 = t2a5;
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20f == 0.0)) && (l.f211 != 0.0)) {let t2a6: f64 = (-230.25850929940458);let t2a7: f64 = (l.f7b1 / l.f5f1);let t2a8: f64 = (l.f5f1 - l.f5ed);let t2a9: f64 = (l.f793 * t2a8);let t2aa: f64 = (l.f5ed * p.p85);let t2ab: f64 = (t2a9 / t2aa);let t2ac: f64 = (t2a7 + t2ab);let t2ad: f64 = (l.f645 * t2ac);let t2ae: f64 = (t2a6 - t2ad);let t2af: f64 = (-230.25850929940458);let t2b0: f64 = (l.f7b1 / l.f5f1);let t2b1: f64 = (l.f5f1 - l.f5ed);let t2b2: f64 = (l.f793 * t2b1);let t2b3: f64 = (l.f5ed * p.p85);let t2b4: f64 = (t2b2 / t2b3);let t2b5: f64 = (t2b0 + t2b4);let t2b6: f64 = (l.f645 * t2b5);let t2b7: f64 = (t2af - t2b6);let t2b8: f64 = (-230.25850929940458);let t2b9: f64 = (l.f7b1 / l.f5f1);let t2ba: f64 = (l.f5f1 - l.f5ed);let t2bb: f64 = (l.f793 * t2ba);let t2bc: f64 = (l.f5ed * p.p85);let t2bd: f64 = (t2bb / t2bc);let t2be: f64 = (t2b9 + t2bd);let t2bf: f64 = (l.f645 * t2be);let t2c0: f64 = (t2b8 - t2bf);let t2c1: f64 = (t2c0 * 0.3333333333333333);let t2c2: f64 = (1.0 + t2c1);let t2c3: f64 = (t2b7 * t2c2);let t2c4: f64 = (0.5 * t2c3);let t2c5: f64 = (1.0 + t2c4);let t2c6: f64 = (t2ae * t2c5);let t2c7: f64 = (1.0 + t2c6);let t2c8: f64 = (1e-100 / t2c7);(l.f93, l.f94, l.f95, ) = (t2c8, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2aa) - (t2a9 * (l.f5ee * p.p85))) / (t2aa * t2aa))))) * t2c5) + (t2ae * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2b3) - (t2b2 * (l.f5ee * p.p85))) / (t2b3 * t2b3))))) * t2c2) + (t2b7 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2bc) - (t2bb * (l.f5ee * p.p85))) / (t2bc * t2bc))))) * 0.3333333333333333))))))) / (t2c7 * t2c7))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2aa) - (t2a9 * (l.f5ef * p.p85))) / (t2aa * t2aa))))) * t2c5) + (t2ae * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2b3) - (t2b2 * (l.f5ef * p.p85))) / (t2b3 * t2b3))))) * t2c2) + (t2b7 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2bc) - (t2bb * (l.f5ef * p.p85))) / (t2bc * t2bc))))) * 0.3333333333333333))))))) / (t2c7 * t2c7))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f20f == 0.0)) && (l.f211 == 0.0)) {let t2c9: f64 = (l.f7b1 / l.f5f1);let t2ca: f64 = (l.f5f1 - l.f5ed);let t2cb: f64 = (l.f793 * t2ca);let t2cc: f64 = (l.f5ed * p.p85);let t2cd: f64 = (t2cb / t2cc);let t2ce: f64 = (t2c9 + t2cd);let t2cf: f64 = (l.f645 * t2ce);let t2d0: f64 = (t2cf - 230.25850929940458);let t2d1: f64 = (l.f7b1 / l.f5f1);let t2d2: f64 = (l.f5f1 - l.f5ed);let t2d3: f64 = (l.f793 * t2d2);let t2d4: f64 = (l.f5ed * p.p85);let t2d5: f64 = (t2d3 / t2d4);let t2d6: f64 = (t2d1 + t2d5);let t2d7: f64 = (l.f645 * t2d6);let t2d8: f64 = (t2d7 - 230.25850929940458);let t2d9: f64 = (l.f7b1 / l.f5f1);let t2da: f64 = (l.f5f1 - l.f5ed);let t2db: f64 = (l.f793 * t2da);let t2dc: f64 = (l.f5ed * p.p85);let t2dd: f64 = (t2db / t2dc);let t2de: f64 = (t2d9 + t2dd);let t2df: f64 = (l.f645 * t2de);let t2e0: f64 = (t2df - 230.25850929940458);let t2e1: f64 = (t2e0 * 0.3333333333333333);let t2e2: f64 = (1.0 + t2e1);let t2e3: f64 = (t2d8 * t2e2);let t2e4: f64 = (0.5 * t2e3);let t2e5: f64 = (1.0 + t2e4);let t2e6: f64 = (t2d0 * t2e5);let t2e7: f64 = (1.0 + t2e6);let t2e8: f64 = (1e100 * t2e7);(l.f93, l.f94, l.f95, ) = (t2e8, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2cc) - (t2cb * (l.f5ee * p.p85))) / (t2cc * t2cc)))) * t2e5) + (t2d0 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2d4) - (t2d3 * (l.f5ee * p.p85))) / (t2d4 * t2d4)))) * t2e2) + (t2d8 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2dc) - (t2db * (l.f5ee * p.p85))) / (t2dc * t2dc)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2cc) - (t2cb * (l.f5ef * p.p85))) / (t2cc * t2cc)))) * t2e5) + (t2d0 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2d4) - (t2d3 * (l.f5ef * p.p85))) / (t2d4 * t2d4)))) * t2e2) + (t2d8 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2dc) - (t2db * (l.f5ef * p.p85))) / (t2dc * t2dc)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) {let t2e9: f64 = (l.f7b1 * l.f5b);let t2ea: f64 = (l.f5f1 - t2e9);let t2eb: f64 = (l.f5f1 * l.f5f1);let t2ec: f64 = (t2ea / t2eb);let t2ed: f64 = (l.f793 * l.f5b);let t2ee: f64 = (l.f5ed * p.p85);let t2ef: f64 = (t2ed / t2ee);let t2f0: f64 = (t2ec + t2ef);let t2f1: f64 = (l.f645 * t2f0);(l.f61, l.f62, l.f63, ) = (t2f1, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t2eb) - (t2ea * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t2eb * t2eb)) + ((((l.f793 * l.f5c) * t2ee) - (t2ed * (l.f5ee * p.p85))) / (t2ee * t2ee)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t2eb) - (t2ea * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t2eb * t2eb)) + ((((l.f793 * l.f5d) * t2ee) - (t2ed * (l.f5ef * p.p85))) / (t2ee * t2ee)))), );let t2f2: f64 = (l.f73b - l.f7b1);let t2f3: f64 = (t2f2 * l.f61);let t2f4: f64 = (1.0 + t2f3);let t2f5: f64 = (t2f4 * l.f93);(l.f53e, l.f53f, l.f540, ) = (t2f5, (((t2f2 * l.f62) * l.f93) + (t2f4 * l.f94)), (((t2f2 * l.f63) * l.f93) + (t2f4 * l.f95)), );let t2f6: f64 = (l.f5eb * l.f5eb);let t2f7: f64 = (t2f6 / l.f5e1);l.f64f = t2f7;let t2f8: f64 = (l.f5e7 / l.f645);let t2f9: f64 = (l.f5e1 / l.f64f);let t2fa: f64 = (t2f9).ln();let t2fb: f64 = (t2f8 * t2fa);l.f793 = t2fb;}
        let t2fc: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f213 = t2fc;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t2fd: f64 = (l.f7b1 - l.f793);let t2fe: f64 = (p.p86 * t2fd);let t2ff: f64 = (t2fe + l.f5e7);(l.f601, l.f602, l.f603, ) = (t2ff, 0.0, 0.0, );let t300: f64 = (p.p86 * l.f793);let t301: f64 = (l.f5e7 - t300);(l.f5ed, l.f5ee, l.f5ef, ) = (t301, 0.0, 0.0, );let t302: f64 = (p.p85 - l.f601);let t303: f64 = (t302 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t303, (-l.f602), (-l.f603), );let t304: f64 = (4.0 * p.p85);let t305: f64 = (t304 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t305, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {
            let (t307, t308, t309,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t306: f64 = (-l.f6f7);
        (t306, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t307, t308, t309, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t30a: f64 = (l.f6f3 * l.f6f3);let t30b: f64 = (t30a + l.f6f7);let t30c: f64 = (t30b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t30c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t30c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t30c)), );let t30d: f64 = (l.f6f3 / l.f6f7);let t30e: f64 = (1.0 + t30d);let t30f: f64 = (0.5 * t30e);(l.f55, l.f56, l.f57, ) = (t30f, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t310: f64 = (l.f6f3 + l.f6f7);let t311: f64 = (0.5 * t310);let t312: f64 = (p.p85 - t311);(l.f605, l.f606, l.f607, ) = (t312, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t313: f64 = (l.f605 - l.f5e7);let t314: f64 = (t313 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t314, l.f606, l.f607, );let t315: f64 = (4.0 * l.f5e7);let t316: f64 = (t315 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t316, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {
            let (t318, t319, t31a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t317: f64 = (-l.f6f7);
        (t317, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t318, t319, t31a, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t31b: f64 = (l.f6f3 * l.f6f3);let t31c: f64 = (t31b + l.f6f7);let t31d: f64 = (t31c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t31d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t31d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t31d)), );let t31e: f64 = (l.f6f3 / l.f6f7);let t31f: f64 = (1.0 + t31e);let t320: f64 = (0.5 * t31f);(l.f51, l.f52, l.f53, ) = (t320, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t321: f64 = (l.f6f3 + l.f6f7);let t322: f64 = (0.5 * t321);let t323: f64 = (l.f5e7 + t322);(l.f5f1, l.f5f2, l.f5f3, ) = (t323, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t324: f64 = (p.p85 - l.f5ed);let t325: f64 = (t324 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t325, (-l.f5ee), (-l.f5ef), );let t326: f64 = (4.0 * p.p85);let t327: f64 = (t326 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t327, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {
            let (t329, t32a, t32b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t328: f64 = (-l.f6f7);
        (t328, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t329, t32a, t32b, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t32c: f64 = (l.f6f3 * l.f6f3);let t32d: f64 = (t32c + l.f6f7);let t32e: f64 = (t32d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t32e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t32e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t32e)), );let t32f: f64 = (l.f6f3 + l.f6f7);let t330: f64 = (0.5 * t32f);let t331: f64 = (p.p85 - t330);(l.f5ed, l.f5ee, l.f5ef, ) = (t331, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t332: f64 = (l.f5ed - l.f5e7);let t333: f64 = (t332 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t333, l.f5ee, l.f5ef, );let t334: f64 = (4.0 * l.f5e7);let t335: f64 = (t334 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t335, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {
            let (t337, t338, t339,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t336: f64 = (-l.f6f7);
        (t336, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t337, t338, t339, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 != 0.0)) {let t33a: f64 = (l.f6f3 * l.f6f3);let t33b: f64 = (t33a + l.f6f7);let t33c: f64 = (t33b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t33c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t33c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t33c)), );let t33d: f64 = (l.f6f3 + l.f6f7);let t33e: f64 = (0.5 * t33d);let t33f: f64 = (l.f5e7 + t33e);(l.f5ed, l.f5ee, l.f5ef, ) = (t33f, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t340: f64 = (p.p86 * l.f55);let t341: f64 = (t340 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t341, (((p.p86 * l.f56) * l.f51) + (t340 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t340 * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f213 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t342: f64 = (l.f7b1 / l.f5f1);let t343: f64 = (l.f5f1 - l.f5ed);let t344: f64 = (l.f793 * t343);let t345: f64 = (l.f5ed * p.p85);let t346: f64 = (t344 / t345);let t347: f64 = (t342 + t346);let t348: f64 = (l.f645 * t347);let t349: f64 = (t348).abs();let t34a: f64 = if t349 < 230.25850929940458 { 1.0 } else { 0.0 };l.f216 = t34a;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f216 != 0.0)) {let t34b: f64 = (l.f7b1 / l.f5f1);let t34c: f64 = (l.f5f1 - l.f5ed);let t34d: f64 = (l.f793 * t34c);let t34e: f64 = (l.f5ed * p.p85);let t34f: f64 = (t34d / t34e);let t350: f64 = (t34b + t34f);let t351: f64 = (l.f645 * t350);let t352: f64 = (t351).exp();(l.f8e, l.f8f, l.f90, ) = (t352, (t352 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t34e) - (t34d * (l.f5ee * p.p85))) / (t34e * t34e))))), (t352 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t34e) - (t34d * (l.f5ef * p.p85))) / (t34e * t34e))))), );}
        let t353: f64 = (l.f7b1 / l.f5f1);let t354: f64 = (l.f5f1 - l.f5ed);let t355: f64 = (l.f793 * t354);let t356: f64 = (l.f5ed * p.p85);let t357: f64 = (t355 / t356);let t358: f64 = (t353 + t357);let t359: f64 = (l.f645 * t358);let t35a: f64 = (-230.25850929940458);let t35b: f64 = if t359 < t35a { 1.0 } else { 0.0 };l.f218 = t35b;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f216 == 0.0)) && (l.f218 != 0.0)) {let t35c: f64 = (-230.25850929940458);let t35d: f64 = (l.f7b1 / l.f5f1);let t35e: f64 = (l.f5f1 - l.f5ed);let t35f: f64 = (l.f793 * t35e);let t360: f64 = (l.f5ed * p.p85);let t361: f64 = (t35f / t360);let t362: f64 = (t35d + t361);let t363: f64 = (l.f645 * t362);let t364: f64 = (t35c - t363);let t365: f64 = (-230.25850929940458);let t366: f64 = (l.f7b1 / l.f5f1);let t367: f64 = (l.f5f1 - l.f5ed);let t368: f64 = (l.f793 * t367);let t369: f64 = (l.f5ed * p.p85);let t36a: f64 = (t368 / t369);let t36b: f64 = (t366 + t36a);let t36c: f64 = (l.f645 * t36b);let t36d: f64 = (t365 - t36c);let t36e: f64 = (-230.25850929940458);let t36f: f64 = (l.f7b1 / l.f5f1);let t370: f64 = (l.f5f1 - l.f5ed);let t371: f64 = (l.f793 * t370);let t372: f64 = (l.f5ed * p.p85);let t373: f64 = (t371 / t372);let t374: f64 = (t36f + t373);let t375: f64 = (l.f645 * t374);let t376: f64 = (t36e - t375);let t377: f64 = (t376 * 0.3333333333333333);let t378: f64 = (1.0 + t377);let t379: f64 = (t36d * t378);let t37a: f64 = (0.5 * t379);let t37b: f64 = (1.0 + t37a);let t37c: f64 = (t364 * t37b);let t37d: f64 = (1.0 + t37c);let t37e: f64 = (1e-100 / t37d);(l.f8e, l.f8f, l.f90, ) = (t37e, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t360) - (t35f * (l.f5ee * p.p85))) / (t360 * t360))))) * t37b) + (t364 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t369) - (t368 * (l.f5ee * p.p85))) / (t369 * t369))))) * t378) + (t36d * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t372) - (t371 * (l.f5ee * p.p85))) / (t372 * t372))))) * 0.3333333333333333))))))) / (t37d * t37d))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t360) - (t35f * (l.f5ef * p.p85))) / (t360 * t360))))) * t37b) + (t364 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t369) - (t368 * (l.f5ef * p.p85))) / (t369 * t369))))) * t378) + (t36d * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t372) - (t371 * (l.f5ef * p.p85))) / (t372 * t372))))) * 0.3333333333333333))))))) / (t37d * t37d))), );}
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) && (l.f216 == 0.0)) && (l.f218 == 0.0)) {let t37f: f64 = (l.f7b1 / l.f5f1);let t380: f64 = (l.f5f1 - l.f5ed);let t381: f64 = (l.f793 * t380);let t382: f64 = (l.f5ed * p.p85);let t383: f64 = (t381 / t382);let t384: f64 = (t37f + t383);let t385: f64 = (l.f645 * t384);let t386: f64 = (t385 - 230.25850929940458);let t387: f64 = (l.f7b1 / l.f5f1);let t388: f64 = (l.f5f1 - l.f5ed);let t389: f64 = (l.f793 * t388);let t38a: f64 = (l.f5ed * p.p85);let t38b: f64 = (t389 / t38a);let t38c: f64 = (t387 + t38b);let t38d: f64 = (l.f645 * t38c);let t38e: f64 = (t38d - 230.25850929940458);let t38f: f64 = (l.f7b1 / l.f5f1);let t390: f64 = (l.f5f1 - l.f5ed);let t391: f64 = (l.f793 * t390);let t392: f64 = (l.f5ed * p.p85);let t393: f64 = (t391 / t392);let t394: f64 = (t38f + t393);let t395: f64 = (l.f645 * t394);let t396: f64 = (t395 - 230.25850929940458);let t397: f64 = (t396 * 0.3333333333333333);let t398: f64 = (1.0 + t397);let t399: f64 = (t38e * t398);let t39a: f64 = (0.5 * t399);let t39b: f64 = (1.0 + t39a);let t39c: f64 = (t386 * t39b);let t39d: f64 = (1.0 + t39c);let t39e: f64 = (1e100 * t39d);(l.f8e, l.f8f, l.f90, ) = (t39e, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t382) - (t381 * (l.f5ee * p.p85))) / (t382 * t382)))) * t39b) + (t386 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t38a) - (t389 * (l.f5ee * p.p85))) / (t38a * t38a)))) * t398) + (t38e * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t392) - (t391 * (l.f5ee * p.p85))) / (t392 * t392)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t382) - (t381 * (l.f5ef * p.p85))) / (t382 * t382)))) * t39b) + (t386 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t38a) - (t389 * (l.f5ef * p.p85))) / (t38a * t38a)))) * t398) + (t38e * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t392) - (t391 * (l.f5ef * p.p85))) / (t392 * t392)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee == 0.0)) {let t39f: f64 = (l.f7b1 * l.f5b);let t3a0: f64 = (l.f5f1 - t39f);let t3a1: f64 = (l.f5f1 * l.f5f1);let t3a2: f64 = (t3a0 / t3a1);let t3a3: f64 = (l.f793 * l.f5b);let t3a4: f64 = (l.f5ed * p.p85);let t3a5: f64 = (t3a3 / t3a4);let t3a6: f64 = (t3a2 + t3a5);let t3a7: f64 = (l.f645 * t3a6);(l.f61, l.f62, l.f63, ) = (t3a7, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t3a1) - (t3a0 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t3a1 * t3a1)) + ((((l.f793 * l.f5c) * t3a4) - (t3a3 * (l.f5ee * p.p85))) / (t3a4 * t3a4)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t3a1) - (t3a0 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t3a1 * t3a1)) + ((((l.f793 * l.f5d) * t3a4) - (t3a3 * (l.f5ef * p.p85))) / (t3a4 * t3a4)))), );let t3a8: f64 = (l.f73b - l.f7b1);let t3a9: f64 = (t3a8 * l.f61);let t3aa: f64 = (1.0 + t3a9);let t3ab: f64 = (t3aa * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t3ab, (((t3a8 * l.f62) * l.f8e) + (t3aa * l.f8f)), (((t3a8 * l.f63) * l.f8e) + (t3aa * l.f90)), );}
        if ((l.f29a != 0.0) && (l.f1ec != 0.0)) {let t3ac: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t3ac, l.f537, l.f538, );let t3ad: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t3ad, l.f53f, l.f540, );let t3ae: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t3ae, l.f53b, l.f53c, );let t3af: f64 = (1.0 / l.f825);l.f817 = t3af;}
        let t3b0: f64 = if l.f73b > 0.0 { 1.0 } else { 0.0 };l.f21a = t3b0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f21a != 0.0)) {let t3b1: f64 = (2.0 + l.f817);let t3b2: f64 = (l.f817 + 1.0);let t3b3: f64 = (l.f817 + 3.0);let t3b4: f64 = (t3b2 * t3b3);let t3b5: f64 = (t3b4).sqrt();let t3b6: f64 = (t3b1 + t3b5);let t3b7: f64 = (t3b6).ln();let t3b8: f64 = (l.f643 * t3b7);let t3b9: f64 = (2.0 * t3b8);l.f714 = t3b9;}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f21a == 0.0)) {let t3ba: f64 = (-l.f73b);let t3bb: f64 = (2.0 * l.f825);let t3bc: f64 = (t3bb + 1.0);let t3bd: f64 = (1.0 + l.f825);let t3be: f64 = (3.0 * l.f825);let t3bf: f64 = (1.0 + t3be);let t3c0: f64 = (t3bd * t3bf);let t3c1: f64 = (t3c0).sqrt();let t3c2: f64 = (t3bc + t3c1);let t3c3: f64 = (t3c2).ln();let t3c4: f64 = (l.f643 * t3c3);let t3c5: f64 = (2.0 * t3c4);let t3c6: f64 = (t3ba + t3c5);l.f714 = t3c6;}
        if ((l.f29a != 0.0) && (l.f1ec != 0.0)) {let t3c7: f64 = (l.f76f - l.f714);l.f79c = t3c7;let t3c8: f64 = (l.f73b + l.f79c);let t3c9: f64 = (l.f73b - l.f79c);let t3ca: f64 = (l.f73b - l.f79c);let t3cb: f64 = (t3c9 * t3ca);let t3cc: f64 = (4.0 * l.f643);let t3cd: f64 = (t3cc * l.f643);let t3ce: f64 = (t3cb + t3cd);let t3cf: f64 = (t3ce).sqrt();let t3d0: f64 = (t3c8 - t3cf);let t3d1: f64 = (0.5 * t3d0);l.f7a2 = t3d1;let t3d2: f64 = (l.f73b + l.f755);let t3d3: f64 = (l.f73b - l.f755);let t3d4: f64 = (l.f73b - l.f755);let t3d5: f64 = (t3d3 * t3d4);let t3d6: f64 = (4.0 * l.f647);let t3d7: f64 = (t3d6 * l.f647);let t3d8: f64 = (t3d5 + t3d7);let t3d9: f64 = (t3d8).sqrt();let t3da: f64 = (t3d2 - t3d9);let t3db: f64 = (0.5 * t3da);l.f750 = t3db;let t3dc: f64 = l.f73b;let t3dd: f64 = l.f73b;let t3de: f64 = l.f73b;let t3df: f64 = (t3dd * t3de);let t3e0: f64 = (4.0 * 1e-6);let t3e1: f64 = (t3e0 * 1e-6);let t3e2: f64 = (t3df + t3e1);let t3e3: f64 = (t3e2).sqrt();let t3e4: f64 = (t3dc - t3e3);let t3e5: f64 = (0.5 * t3e4);l.f74a = t3e5;}
        if ((l.f29a != 0.0) && (l.f1ec == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f714 = 0.0;l.f796 = 0.0;l.f825 = 0.0;l.f7a2 = 0.0;l.f750 = 0.0;l.f74a = 0.0;}
        let t3e6: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f21c = t3e6;
        if ((l.f29a != 0.0) && (l.f21c != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );}
        let t3e7: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f21e = t3e7;
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f21e != 0.0)) {let t3e8: f64 = (l.f796 * l.f769);let t3e9: f64 = (1.0 - t3e8);let t3ea: f64 = (t3e9).sqrt();l.f6fc = t3ea;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f21e == 0.0)) {let t3eb: f64 = (l.f796 * l.f769);let t3ec: f64 = (1.0 - t3eb);let t3ed: f64 = (t3ec).powf(l.f60b);l.f6fc = t3ed;}
        if ((l.f29a != 0.0) && (l.f21c == 0.0)) {let t3ee: f64 = (1.0 - l.f6fc);let t3ef: f64 = (l.f69e * t3ee);let t3f0: f64 = (l.f73b - l.f796);let t3f1: f64 = (l.f698 * t3f0);let t3f2: f64 = (t3ef + t3f1);(l.f68c, l.f68d, l.f68e, ) = (t3f2, 0.0, 0.0, );let t3f3: f64 = (l.f542 * l.f536);(l.f52f, l.f530, l.f531, ) = (t3f3, (l.f542 * l.f537), (l.f542 * l.f538), );}
        let t3f4: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.f220 = t3f4;
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 != 0.0)) {l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) {let t3f5: f64 = (l.f75d - l.f7a2);l.f758 = t3f5;let t3f6: f64 = (l.f714 / l.f758);let t3f7: f64 = (1.0 - t3f6);let t3f8: f64 = (t3f7).sqrt();let t3f9: f64 = (1.0 - t3f8);l.f7ef = t3f9;}
        let t3fa: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f222 = t3fa;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) && (l.f222 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) && (l.f222 == 0.0)) {let t3fb: f64 = (l.f7ef * l.f7ef);let t3fc: f64 = (l.f7ef).ln();let t3fd: f64 = (t3fb * t3fc);let t3fe: f64 = (1.0 - l.f7ef);let t3ff: f64 = (t3fd / t3fe);let t400: f64 = (t3ff + l.f7ef);let t401: f64 = (2.0 * l.f623);let t402: f64 = (1.0 - t401);let t403: f64 = (t400 * t402);l.f66 = t403;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) {let t404: f64 = (l.f7ef + l.f66);l.f7e9 = t404;}
        let t405: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f224 = t405;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) && (l.f224 != 0.0)) {let t406: f64 = (l.f758 * l.f773);let t407: f64 = (t406).sqrt();l.f6fc = t407;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) && (l.f224 == 0.0)) {let t408: f64 = (l.f758 * l.f773);let t409: f64 = (t408).powf(l.f623);l.f6fc = t409;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f220 == 0.0)) {let t40a: f64 = (l.f7d6 * l.f6fc);l.f7d1 = t40a;let t40b: f64 = (l.f825 - 1.0);let t40c: f64 = (t40b * l.f7d1);let t40d: f64 = (l.fc9 * t40c);l.f9 = t40d;let t40e: f64 = (l.f9 * l.f7e9);let t40f: f64 = (l.f39 * t40e);l.f593 = t40f;}
        let t410: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.f226 = t410;
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) {let t411: f64 = (l.f7d1 * l.f60b);let t412: f64 = (t411 / l.f758);let t413: f64 = (l.f1e * t412);l.f19 = t413;let t414: f64 = (0.666666666666667 * l.fe);let t415: f64 = (t414 / l.f19);l.f71a = t415;let t416: f64 = (l.f71a * l.f71a);l.f72c = t416;let t417: f64 = (l.f72c * l.f72c);let t418: f64 = (l.f72c * l.f72c);let t419: f64 = (t418 + 1.0);let t41a: f64 = (t417 / t419);let t41b: f64 = (t41a).sqrt();l.f726 = t41b;let t41c: f64 = (l.f726).abs();let t41d: f64 = (t41c).sqrt();l.f6c1 = t41d;let t41e: f64 = (l.f726 * l.f6c1);l.f732 = t41e;}
        let t41f: f64 = (-l.f623);let t420: f64 = (t41f * l.f611);let t421: f64 = (-1.0);let t422: f64 = if t420 == t421 { 1.0 } else { 0.0 };l.f228 = t422;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f228 != 0.0)) {let t423: f64 = (l.f19 * l.f732);let t424: f64 = (1.0 + t423);let t425: f64 = (1.0 / t424);l.f7e3 = t425;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f228 == 0.0)) {let t426: f64 = (l.f19 * l.f732);let t427: f64 = (1.0 + t426);let t428: f64 = (-l.f623);let t429: f64 = (t428 * l.f611);let t42a: f64 = (t427).powf(t429);l.f7e3 = t42a;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) {let t42b: f64 = (l.f7e9 * l.f7e3);let t42c: f64 = (l.f7e9 + l.f7e3);let t42d: f64 = (t42b / t42c);l.f7f5 = t42d;let t42e: f64 = (l.f19 / l.f6c1);let t42f: f64 = (0.375 * t42e);let t430: f64 = (t42f).sqrt();l.f5a8 = t430;let t431: f64 = (l.f71a * l.f6c1);let t432: f64 = (2.0 * t431);let t433: f64 = (t432 - l.f726);l.f5b4 = t433;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) {let t434: f64 = (l.fe * l.f71a);let t435: f64 = (t434 * l.f6c1);let t436: f64 = (l.fe * l.f726);let t437: f64 = (t435 - t436);let t438: f64 = (l.f19 * l.f732);let t439: f64 = (0.5 * t438);let t43a: f64 = (t437 + t439);l.f5d4 = t43a;let t43b: f64 = (l.f5b4 - 1.0);let t43c: f64 = (t43b * l.f5a8);l.f7fb = t43c;let t43d: f64 = (l.f7fb * l.f7fb);l.f811 = t43d;}
        let t43e: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f22b = t43e;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22b != 0.0)) {let t43f: f64 = (l.f62b * l.f7fb);let t440: f64 = (1.0 + t43f);let t441: f64 = (1.0 / t440);l.f6e2 = t441;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22b == 0.0)) {let t442: f64 = (l.f62b * l.f7fb);let t443: f64 = (1.0 - t442);let t444: f64 = (1.0 / t443);l.f6e2 = t444;}
        let t445: f64 = (-l.f811);let t446: f64 = (t445 + l.f5d4);let t447: f64 = (-230.25850929940458);let t448: f64 = if t446 > t447 { 1.0 } else { 0.0 };l.f22d = t448;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22d != 0.0)) {let t449: f64 = (-l.f811);let t44a: f64 = (t449 + l.f5d4);let t44b: f64 = (t44a).exp();l.f6fc = t44b;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22d == 0.0)) {let t44c: f64 = (-230.25850929940458);let t44d: f64 = (-l.f811);let t44e: f64 = (t44d + l.f5d4);let t44f: f64 = (t44c - t44e);let t450: f64 = (-230.25850929940458);let t451: f64 = (-l.f811);let t452: f64 = (t451 + l.f5d4);let t453: f64 = (t450 - t452);let t454: f64 = (-230.25850929940458);let t455: f64 = (-l.f811);let t456: f64 = (t455 + l.f5d4);let t457: f64 = (t454 - t456);let t458: f64 = (t457 * 0.3333333333333333);let t459: f64 = (1.0 + t458);let t45a: f64 = (t453 * t459);let t45b: f64 = (0.5 * t45a);let t45c: f64 = (1.0 + t45b);let t45d: f64 = (t44f * t45c);let t45e: f64 = (1.0 + t45d);let t45f: f64 = (1e-100 / t45e);l.f6fc = t45f;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) {let t460: f64 = (0.29214664 * l.f6e2);let t461: f64 = (l.f6e2 * l.f6e2);let t462: f64 = (l.f16 * t461);let t463: f64 = (t460 + t462);let t464: f64 = (l.f6e2 * l.f6e2);let t465: f64 = (t464 * l.f6e2);let t466: f64 = (l.f2a * t465);let t467: f64 = (t463 + t466);let t468: f64 = (t467 * l.f6fc);l.f6e = t468;}
        let t469: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f22f = t469;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22f != 0.0)) {l.f74 = l.f6e;}
        let t46a: f64 = (-230.25850929940458);let t46b: f64 = if l.f5d4 > t46a { 1.0 } else { 0.0 };l.f231 = t46b;
        if (((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22f == 0.0)) && (l.f231 != 0.0)) {let t46c: f64 = (l.f5d4).exp();l.f6fc = t46c;}
        if (((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22f == 0.0)) && (l.f231 == 0.0)) {let t46d: f64 = (-230.25850929940458);let t46e: f64 = (t46d - l.f5d4);let t46f: f64 = (-230.25850929940458);let t470: f64 = (t46f - l.f5d4);let t471: f64 = (-230.25850929940458);let t472: f64 = (t471 - l.f5d4);let t473: f64 = (t472 * 0.3333333333333333);let t474: f64 = (1.0 + t473);let t475: f64 = (t470 * t474);let t476: f64 = (0.5 * t475);let t477: f64 = (1.0 + t476);let t478: f64 = (t46e * t477);let t479: f64 = (1.0 + t478);let t47a: f64 = (1e-100 / t479);l.f6fc = t47a;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) && (l.f22f == 0.0)) {let t47b: f64 = (2.0 * l.f6fc);let t47c: f64 = (t47b - l.f6e);l.f74 = t47c;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) {let t47d: f64 = (1.772453850905516 * 0.5);let t47e: f64 = (l.fe * l.f74);let t47f: f64 = (t47e / l.f5a8);let t480: f64 = (t47d * t47f);l.fd6 = t480;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f226 == 0.0)) {let t481: f64 = (l.f9 * l.fd6);let t482: f64 = (t481 * l.f7f5);let t483: f64 = (l.f3f * t482);l.f599 = t483;}
        let t484: f64 = if l.f24 == 0.0 { 1.0 } else { 0.0 };l.f233 = t484;
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f233 != 0.0)) {l.f529 = 0.0;}
        let t485: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f235 = t485;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f233 == 0.0)) && (l.f235 != 0.0)) {let t486: f64 = (l.f771 - l.f750);let t487: f64 = (t486 * l.f773);let t488: f64 = (t487).sqrt();l.f6fc = t488;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f233 == 0.0)) && (l.f235 == 0.0)) {let t489: f64 = (l.f771 - l.f750);let t48a: f64 = (t489 * l.f773);let t48b: f64 = (t48a).powf(l.f623);l.f6fc = t48b;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f233 == 0.0)) {let t48c: f64 = (l.f771 - l.f750);let t48d: f64 = (t48c * l.f7da);let t48e: f64 = (t48d / l.f6fc);let t48f: f64 = (l.f611 * t48e);l.fb6 = t48f;}
        let t490: f64 = (-l.fa1);let t491: f64 = (t490 / l.fb6);let t492: f64 = (t491).abs();let t493: f64 = if t492 < 230.25850929940458 { 1.0 } else { 0.0 };l.f237 = t493;
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f233 == 0.0)) && (l.f237 != 0.0)) {let t494: f64 = (-l.fa1);let t495: f64 = (t494 / l.fb6);let t496: f64 = (t495).exp();l.f6fc = t496;}
        let t497: f64 = (-l.fa1);let t498: f64 = (t497 / l.fb6);let t499: f64 = (-230.25850929940458);let t49a: f64 = if t498 < t499 { 1.0 } else { 0.0 };l.f239 = t49a;
        if (((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f233 == 0.0)) && (l.f237 == 0.0)) && (l.f239 != 0.0)) {let t49b: f64 = (-230.25850929940458);let t49c: f64 = (-l.fa1);let t49d: f64 = (t49c / l.fb6);let t49e: f64 = (t49b - t49d);let t49f: f64 = (-230.25850929940458);let t4a0: f64 = (-l.fa1);let t4a1: f64 = (t4a0 / l.fb6);let t4a2: f64 = (t49f - t4a1);let t4a3: f64 = (-230.25850929940458);let t4a4: f64 = (-l.fa1);let t4a5: f64 = (t4a4 / l.fb6);let t4a6: f64 = (t4a3 - t4a5);let t4a7: f64 = (t4a6 * 0.3333333333333333);let t4a8: f64 = (1.0 + t4a7);let t4a9: f64 = (t4a2 * t4a8);let t4aa: f64 = (0.5 * t4a9);let t4ab: f64 = (1.0 + t4aa);let t4ac: f64 = (t49e * t4ab);let t4ad: f64 = (1.0 + t4ac);let t4ae: f64 = (1e-100 / t4ad);l.f6fc = t4ae;}
        if (((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f233 == 0.0)) && (l.f237 == 0.0)) && (l.f239 == 0.0)) {let t4af: f64 = (-l.fa1);let t4b0: f64 = (t4af / l.fb6);let t4b1: f64 = (t4b0 - 230.25850929940458);let t4b2: f64 = (-l.fa1);let t4b3: f64 = (t4b2 / l.fb6);let t4b4: f64 = (t4b3 - 230.25850929940458);let t4b5: f64 = (-l.fa1);let t4b6: f64 = (t4b5 / l.fb6);let t4b7: f64 = (t4b6 - 230.25850929940458);let t4b8: f64 = (t4b7 * 0.3333333333333333);let t4b9: f64 = (1.0 + t4b8);let t4ba: f64 = (t4b4 * t4b9);let t4bb: f64 = (0.5 * t4ba);let t4bc: f64 = (1.0 + t4bb);let t4bd: f64 = (t4b1 * t4bc);let t4be: f64 = (1.0 + t4bd);let t4bf: f64 = (1e100 * t4be);l.f6fc = t4bf;}
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f233 == 0.0)) {let t4c0: f64 = (l.f73b * l.fb6);let t4c1: f64 = (t4c0 * l.fb6);let t4c2: f64 = (t4c1 * l.f6fc);let t4c3: f64 = (l.f24 * t4c2);l.f529 = t4c3;}
        let t4c4: f64 = if ((l.f783 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f23b = t4c4;
        if (((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f23b != 0.0)) {l.fae = 1.0;}
        let t4c5: f64 = (-l.f2);let t4c6: f64 = (t4c5 * l.f783);let t4c7: f64 = if l.f74a > t4c6 { 1.0 } else { 0.0 };l.f23d = t4c7;let t4c8: f64 = if l.f625 == 4.0 { 1.0 } else { 0.0 };l.f241 = t4c8;
    }
}
