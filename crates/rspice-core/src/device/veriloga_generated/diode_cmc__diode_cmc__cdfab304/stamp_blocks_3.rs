#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t0: f64 = (l.f6f3 * l.f6f3);let t1: f64 = (t0 + l.f6f7);let t2: f64 = (t1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2)), );let t3: f64 = (l.f6f3 / l.f6f7);let t4: f64 = (1.0 + t3);let t5: f64 = (0.5 * t4);(l.f51, l.f52, l.f53, ) = (t5, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t6: f64 = (l.f6f3 + l.f6f7);let t7: f64 = (0.5 * t6);let t8: f64 = (l.f5e7 + t7);(l.f5f1, l.f5f2, l.f5f3, ) = (t8, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t9: f64 = (p.p85 - l.f5ed);let ta: f64 = (t9 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (ta, (-l.f5ee), (-l.f5ef), );let tb: f64 = (4.0 * p.p85);let tc: f64 = (tb * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tc, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {
            let (te, tf, t10,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let td: f64 = (-l.f6f7);
        (td, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (te, tf, t10, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t11: f64 = (l.f6f3 * l.f6f3);let t12: f64 = (t11 + l.f6f7);let t13: f64 = (t12).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t13, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t13)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t13)), );let t14: f64 = (l.f6f3 + l.f6f7);let t15: f64 = (0.5 * t14);let t16: f64 = (p.p85 - t15);(l.f5ed, l.f5ee, l.f5ef, ) = (t16, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t17: f64 = (l.f5ed - l.f5e7);let t18: f64 = (t17 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t18, l.f5ee, l.f5ef, );let t19: f64 = (4.0 * l.f5e7);let t1a: f64 = (t19 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1a, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {
            let (t1c, t1d, t1e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1b: f64 = (-l.f6f7);
        (t1b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1c, t1d, t1e, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t1f: f64 = (l.f6f3 * l.f6f3);let t20: f64 = (t1f + l.f6f7);let t21: f64 = (t20).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t21, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t21)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t21)), );let t22: f64 = (l.f6f3 + l.f6f7);let t23: f64 = (0.5 * t22);let t24: f64 = (l.f5e7 + t23);(l.f5ed, l.f5ee, l.f5ef, ) = (t24, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t25: f64 = (p.p86 * l.f55);let t26: f64 = (t25 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t26, (((p.p86 * l.f56) * l.f51) + (t25 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t25 * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t27: f64 = (l.f7b1 / l.f5f1);let t28: f64 = (l.f5f1 - l.f5ed);let t29: f64 = (l.f793 * t28);let t2a: f64 = (l.f5ed * p.p85);let t2b: f64 = (t29 / t2a);let t2c: f64 = (t27 + t2b);let t2d: f64 = (l.f645 * t2c);let t2e: f64 = (t2d).abs();let t2f: f64 = if t2e < 230.25850929940458 { 1.0 } else { 0.0 };l.f176 = t2f;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f176 != 0.0)) {let t30: f64 = (l.f7b1 / l.f5f1);let t31: f64 = (l.f5f1 - l.f5ed);let t32: f64 = (l.f793 * t31);let t33: f64 = (l.f5ed * p.p85);let t34: f64 = (t32 / t33);let t35: f64 = (t30 + t34);let t36: f64 = (l.f645 * t35);let t37: f64 = (t36).exp();(l.f8e, l.f8f, l.f90, ) = (t37, (t37 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t33) - (t32 * (l.f5ee * p.p85))) / (t33 * t33))))), (t37 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t33) - (t32 * (l.f5ef * p.p85))) / (t33 * t33))))), );}
        let t38: f64 = (l.f7b1 / l.f5f1);let t39: f64 = (l.f5f1 - l.f5ed);let t3a: f64 = (l.f793 * t39);let t3b: f64 = (l.f5ed * p.p85);let t3c: f64 = (t3a / t3b);let t3d: f64 = (t38 + t3c);let t3e: f64 = (l.f645 * t3d);let t3f: f64 = (-230.25850929940458);let t40: f64 = if t3e < t3f { 1.0 } else { 0.0 };l.f178 = t40;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f176 == 0.0)) && (l.f178 != 0.0)) {let t41: f64 = (-230.25850929940458);let t42: f64 = (l.f7b1 / l.f5f1);let t43: f64 = (l.f5f1 - l.f5ed);let t44: f64 = (l.f793 * t43);let t45: f64 = (l.f5ed * p.p85);let t46: f64 = (t44 / t45);let t47: f64 = (t42 + t46);let t48: f64 = (l.f645 * t47);let t49: f64 = (t41 - t48);let t4a: f64 = (-230.25850929940458);let t4b: f64 = (l.f7b1 / l.f5f1);let t4c: f64 = (l.f5f1 - l.f5ed);let t4d: f64 = (l.f793 * t4c);let t4e: f64 = (l.f5ed * p.p85);let t4f: f64 = (t4d / t4e);let t50: f64 = (t4b + t4f);let t51: f64 = (l.f645 * t50);let t52: f64 = (t4a - t51);let t53: f64 = (-230.25850929940458);let t54: f64 = (l.f7b1 / l.f5f1);let t55: f64 = (l.f5f1 - l.f5ed);let t56: f64 = (l.f793 * t55);let t57: f64 = (l.f5ed * p.p85);let t58: f64 = (t56 / t57);let t59: f64 = (t54 + t58);let t5a: f64 = (l.f645 * t59);let t5b: f64 = (t53 - t5a);let t5c: f64 = (t5b * 0.3333333333333333);let t5d: f64 = (1.0 + t5c);let t5e: f64 = (t52 * t5d);let t5f: f64 = (0.5 * t5e);let t60: f64 = (1.0 + t5f);let t61: f64 = (t49 * t60);let t62: f64 = (1.0 + t61);let t63: f64 = (1e-100 / t62);(l.f8e, l.f8f, l.f90, ) = (t63, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t45) - (t44 * (l.f5ee * p.p85))) / (t45 * t45))))) * t60) + (t49 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t4e) - (t4d * (l.f5ee * p.p85))) / (t4e * t4e))))) * t5d) + (t52 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t57) - (t56 * (l.f5ee * p.p85))) / (t57 * t57))))) * 0.3333333333333333))))))) / (t62 * t62))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t45) - (t44 * (l.f5ef * p.p85))) / (t45 * t45))))) * t60) + (t49 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t4e) - (t4d * (l.f5ef * p.p85))) / (t4e * t4e))))) * t5d) + (t52 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t57) - (t56 * (l.f5ef * p.p85))) / (t57 * t57))))) * 0.3333333333333333))))))) / (t62 * t62))), );}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f176 == 0.0)) && (l.f178 == 0.0)) {let t64: f64 = (l.f7b1 / l.f5f1);let t65: f64 = (l.f5f1 - l.f5ed);let t66: f64 = (l.f793 * t65);let t67: f64 = (l.f5ed * p.p85);let t68: f64 = (t66 / t67);let t69: f64 = (t64 + t68);let t6a: f64 = (l.f645 * t69);let t6b: f64 = (t6a - 230.25850929940458);let t6c: f64 = (l.f7b1 / l.f5f1);let t6d: f64 = (l.f5f1 - l.f5ed);let t6e: f64 = (l.f793 * t6d);let t6f: f64 = (l.f5ed * p.p85);let t70: f64 = (t6e / t6f);let t71: f64 = (t6c + t70);let t72: f64 = (l.f645 * t71);let t73: f64 = (t72 - 230.25850929940458);let t74: f64 = (l.f7b1 / l.f5f1);let t75: f64 = (l.f5f1 - l.f5ed);let t76: f64 = (l.f793 * t75);let t77: f64 = (l.f5ed * p.p85);let t78: f64 = (t76 / t77);let t79: f64 = (t74 + t78);let t7a: f64 = (l.f645 * t79);let t7b: f64 = (t7a - 230.25850929940458);let t7c: f64 = (t7b * 0.3333333333333333);let t7d: f64 = (1.0 + t7c);let t7e: f64 = (t73 * t7d);let t7f: f64 = (0.5 * t7e);let t80: f64 = (1.0 + t7f);let t81: f64 = (t6b * t80);let t82: f64 = (1.0 + t81);let t83: f64 = (1e100 * t82);(l.f8e, l.f8f, l.f90, ) = (t83, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t67) - (t66 * (l.f5ee * p.p85))) / (t67 * t67)))) * t80) + (t6b * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t6f) - (t6e * (l.f5ee * p.p85))) / (t6f * t6f)))) * t7d) + (t73 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t77) - (t76 * (l.f5ee * p.p85))) / (t77 * t77)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t67) - (t66 * (l.f5ef * p.p85))) / (t67 * t67)))) * t80) + (t6b * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t6f) - (t6e * (l.f5ef * p.p85))) / (t6f * t6f)))) * t7d) + (t73 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t77) - (t76 * (l.f5ef * p.p85))) / (t77 * t77)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) {let t84: f64 = (l.f7b1 * l.f5b);let t85: f64 = (l.f5f1 - t84);let t86: f64 = (l.f5f1 * l.f5f1);let t87: f64 = (t85 / t86);let t88: f64 = (l.f793 * l.f5b);let t89: f64 = (l.f5ed * p.p85);let t8a: f64 = (t88 / t89);let t8b: f64 = (t87 + t8a);let t8c: f64 = (l.f645 * t8b);(l.f61, l.f62, l.f63, ) = (t8c, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t86) - (t85 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t86 * t86)) + ((((l.f793 * l.f5c) * t89) - (t88 * (l.f5ee * p.p85))) / (t89 * t89)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t86) - (t85 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t86 * t86)) + ((((l.f793 * l.f5d) * t89) - (t88 * (l.f5ef * p.p85))) / (t89 * t89)))), );let t8d: f64 = (l.f739 - l.f7b1);let t8e: f64 = (t8d * l.f61);let t8f: f64 = (1.0 + t8e);let t90: f64 = (t8f * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t90, (((t8d * l.f62) * l.f8e) + (t8f * l.f8f)), (((t8d * l.f63) * l.f8e) + (t8f * l.f90)), );}
        if ((l.f29a != 0.0) && (l.f14e != 0.0)) {let t91: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t91, l.f537, l.f538, );let t92: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t92, l.f53f, l.f540, );let t93: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t93, l.f53b, l.f53c, );let t94: f64 = (1.0 / l.f825);l.f817 = t94;}
        let t95: f64 = if l.f739 > 0.0 { 1.0 } else { 0.0 };l.f17a = t95;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f17a != 0.0)) {let t96: f64 = (2.0 + l.f817);let t97: f64 = (l.f817 + 1.0);let t98: f64 = (l.f817 + 3.0);let t99: f64 = (t97 * t98);let t9a: f64 = (t99).sqrt();let t9b: f64 = (t96 + t9a);let t9c: f64 = (t9b).ln();let t9d: f64 = (l.f643 * t9c);let t9e: f64 = (2.0 * t9d);l.f714 = t9e;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f17a == 0.0)) {let t9f: f64 = (-l.f739);let ta0: f64 = (2.0 * l.f825);let ta1: f64 = (ta0 + 1.0);let ta2: f64 = (1.0 + l.f825);let ta3: f64 = (3.0 * l.f825);let ta4: f64 = (1.0 + ta3);let ta5: f64 = (ta2 * ta4);let ta6: f64 = (ta5).sqrt();let ta7: f64 = (ta1 + ta6);let ta8: f64 = (ta7).ln();let ta9: f64 = (l.f643 * ta8);let taa: f64 = (2.0 * ta9);let tab: f64 = (t9f + taa);l.f714 = tab;}
        if ((l.f29a != 0.0) && (l.f14e != 0.0)) {let tac: f64 = (l.f76f - l.f714);l.f79c = tac;let tad: f64 = (l.f739 + l.f79c);let tae: f64 = (l.f739 - l.f79c);let taf: f64 = (l.f739 - l.f79c);let tb0: f64 = (tae * taf);let tb1: f64 = (4.0 * l.f643);let tb2: f64 = (tb1 * l.f643);let tb3: f64 = (tb0 + tb2);let tb4: f64 = (tb3).sqrt();let tb5: f64 = (tad - tb4);let tb6: f64 = (0.5 * tb5);l.f7a2 = tb6;let tb7: f64 = (l.f739 + l.f755);let tb8: f64 = (l.f739 - l.f755);let tb9: f64 = (l.f739 - l.f755);let tba: f64 = (tb8 * tb9);let tbb: f64 = (4.0 * l.f647);let tbc: f64 = (tbb * l.f647);let tbd: f64 = (tba + tbc);let tbe: f64 = (tbd).sqrt();let tbf: f64 = (tb7 - tbe);let tc0: f64 = (0.5 * tbf);l.f750 = tc0;let tc1: f64 = l.f739;let tc2: f64 = l.f739;let tc3: f64 = l.f739;let tc4: f64 = (tc2 * tc3);let tc5: f64 = (4.0 * 1e-6);let tc6: f64 = (tc5 * 1e-6);let tc7: f64 = (tc4 + tc6);let tc8: f64 = (tc7).sqrt();let tc9: f64 = (tc1 - tc8);let tca: f64 = (0.5 * tc9);l.f74a = tca;}
        if ((l.f29a != 0.0) && (l.f14e == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f714 = 0.0;l.f796 = 0.0;l.f825 = 0.0;l.f7a2 = 0.0;l.f750 = 0.0;l.f74a = 0.0;}
        let tcb: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f17c = tcb;
        if ((l.f29a != 0.0) && (l.f17c != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );}
        let tcc: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f17e = tcc;
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f17e != 0.0)) {let tcd: f64 = (l.f796 * l.f769);let tce: f64 = (1.0 - tcd);let tcf: f64 = (tce).sqrt();l.f6fc = tcf;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f17e == 0.0)) {let td0: f64 = (l.f796 * l.f769);let td1: f64 = (1.0 - td0);let td2: f64 = (td1).powf(l.f60b);l.f6fc = td2;}
        if ((l.f29a != 0.0) && (l.f17c == 0.0)) {let td3: f64 = (1.0 - l.f6fc);let td4: f64 = (l.f69e * td3);let td5: f64 = (l.f739 - l.f796);let td6: f64 = (l.f698 * td5);let td7: f64 = (td4 + td6);(l.f68c, l.f68d, l.f68e, ) = (td7, 0.0, 0.0, );let td8: f64 = (l.f542 * l.f536);(l.f52f, l.f530, l.f531, ) = (td8, (l.f542 * l.f537), (l.f542 * l.f538), );}
        let td9: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.f180 = td9;
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 != 0.0)) {l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) {let tda: f64 = (l.f75d - l.f7a2);l.f758 = tda;let tdb: f64 = (l.f714 / l.f758);let tdc: f64 = (1.0 - tdb);let tdd: f64 = (tdc).sqrt();let tde: f64 = (1.0 - tdd);l.f7ef = tde;}
        let tdf: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f182 = tdf;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) && (l.f182 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) && (l.f182 == 0.0)) {let te0: f64 = (l.f7ef * l.f7ef);let te1: f64 = (l.f7ef).ln();let te2: f64 = (te0 * te1);let te3: f64 = (1.0 - l.f7ef);let te4: f64 = (te2 / te3);let te5: f64 = (te4 + l.f7ef);let te6: f64 = (2.0 * l.f623);let te7: f64 = (1.0 - te6);let te8: f64 = (te5 * te7);l.f66 = te8;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) {let te9: f64 = (l.f7ef + l.f66);l.f7e9 = te9;}
        let tea: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f184 = tea;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) && (l.f184 != 0.0)) {let teb: f64 = (l.f758 * l.f773);let tec: f64 = (teb).sqrt();l.f6fc = tec;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) && (l.f184 == 0.0)) {let ted: f64 = (l.f758 * l.f773);let tee: f64 = (ted).powf(l.f623);l.f6fc = tee;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) {let tef: f64 = (l.f7d6 * l.f6fc);l.f7d1 = tef;let tf0: f64 = (l.f825 - 1.0);let tf1: f64 = (tf0 * l.f7d1);let tf2: f64 = (l.fc9 * tf1);l.f9 = tf2;let tf3: f64 = (l.f9 * l.f7e9);let tf4: f64 = (l.f39 * tf3);l.f593 = tf4;}
        let tf5: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.f186 = tf5;
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) {let tf6: f64 = (l.f7d1 * l.f60b);let tf7: f64 = (tf6 / l.f758);let tf8: f64 = (l.f1e * tf7);l.f19 = tf8;let tf9: f64 = (0.666666666666667 * l.fe);let tfa: f64 = (tf9 / l.f19);l.f71a = tfa;let tfb: f64 = (l.f71a * l.f71a);l.f72c = tfb;let tfc: f64 = (l.f72c * l.f72c);let tfd: f64 = (l.f72c * l.f72c);let tfe: f64 = (tfd + 1.0);let tff: f64 = (tfc / tfe);let t100: f64 = (tff).sqrt();l.f726 = t100;let t101: f64 = (l.f726).abs();let t102: f64 = (t101).sqrt();l.f6c1 = t102;let t103: f64 = (l.f726 * l.f6c1);l.f732 = t103;}
        let t104: f64 = (-l.f623);let t105: f64 = (t104 * l.f611);let t106: f64 = (-1.0);let t107: f64 = if t105 == t106 { 1.0 } else { 0.0 };l.f188 = t107;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f188 != 0.0)) {let t108: f64 = (l.f19 * l.f732);let t109: f64 = (1.0 + t108);let t10a: f64 = (1.0 / t109);l.f7e3 = t10a;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f188 == 0.0)) {let t10b: f64 = (l.f19 * l.f732);let t10c: f64 = (1.0 + t10b);let t10d: f64 = (-l.f623);let t10e: f64 = (t10d * l.f611);let t10f: f64 = (t10c).powf(t10e);l.f7e3 = t10f;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) {let t110: f64 = (l.f7e9 * l.f7e3);let t111: f64 = (l.f7e9 + l.f7e3);let t112: f64 = (t110 / t111);l.f7f5 = t112;let t113: f64 = (l.f19 / l.f6c1);let t114: f64 = (0.375 * t113);let t115: f64 = (t114).sqrt();l.f5a8 = t115;let t116: f64 = (l.f71a * l.f6c1);let t117: f64 = (2.0 * t116);let t118: f64 = (t117 - l.f726);l.f5b4 = t118;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) {let t119: f64 = (l.fe * l.f71a);let t11a: f64 = (t119 * l.f6c1);let t11b: f64 = (l.fe * l.f726);let t11c: f64 = (t11a - t11b);let t11d: f64 = (l.f19 * l.f732);let t11e: f64 = (0.5 * t11d);let t11f: f64 = (t11c + t11e);l.f5d4 = t11f;let t120: f64 = (l.f5b4 - 1.0);let t121: f64 = (t120 * l.f5a8);l.f7fb = t121;let t122: f64 = (l.f7fb * l.f7fb);l.f811 = t122;}
        let t123: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f18a = t123;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18a != 0.0)) {let t124: f64 = (l.f62b * l.f7fb);let t125: f64 = (1.0 + t124);let t126: f64 = (1.0 / t125);l.f6e2 = t126;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18a == 0.0)) {let t127: f64 = (l.f62b * l.f7fb);let t128: f64 = (1.0 - t127);let t129: f64 = (1.0 / t128);l.f6e2 = t129;}
        let t12a: f64 = (-l.f811);let t12b: f64 = (t12a + l.f5d4);let t12c: f64 = (-230.25850929940458);let t12d: f64 = if t12b > t12c { 1.0 } else { 0.0 };l.f18c = t12d;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18c != 0.0)) {let t12e: f64 = (-l.f811);let t12f: f64 = (t12e + l.f5d4);let t130: f64 = (t12f).exp();l.f6fc = t130;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18c == 0.0)) {let t131: f64 = (-230.25850929940458);let t132: f64 = (-l.f811);let t133: f64 = (t132 + l.f5d4);let t134: f64 = (t131 - t133);let t135: f64 = (-230.25850929940458);let t136: f64 = (-l.f811);let t137: f64 = (t136 + l.f5d4);let t138: f64 = (t135 - t137);let t139: f64 = (-230.25850929940458);let t13a: f64 = (-l.f811);let t13b: f64 = (t13a + l.f5d4);let t13c: f64 = (t139 - t13b);let t13d: f64 = (t13c * 0.3333333333333333);let t13e: f64 = (1.0 + t13d);let t13f: f64 = (t138 * t13e);let t140: f64 = (0.5 * t13f);let t141: f64 = (1.0 + t140);let t142: f64 = (t134 * t141);let t143: f64 = (1.0 + t142);let t144: f64 = (1e-100 / t143);l.f6fc = t144;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) {let t145: f64 = (0.29214664 * l.f6e2);let t146: f64 = (l.f6e2 * l.f6e2);let t147: f64 = (l.f16 * t146);let t148: f64 = (t145 + t147);let t149: f64 = (l.f6e2 * l.f6e2);let t14a: f64 = (t149 * l.f6e2);let t14b: f64 = (l.f2a * t14a);let t14c: f64 = (t148 + t14b);let t14d: f64 = (t14c * l.f6fc);l.f6e = t14d;}
        let t14e: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f18e = t14e;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18e != 0.0)) {l.f74 = l.f6e;}
        let t14f: f64 = (-230.25850929940458);let t150: f64 = if l.f5d4 > t14f { 1.0 } else { 0.0 };l.f190 = t150;
        if (((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18e == 0.0)) && (l.f190 != 0.0)) {let t151: f64 = (l.f5d4).exp();l.f6fc = t151;}
        if (((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18e == 0.0)) && (l.f190 == 0.0)) {let t152: f64 = (-230.25850929940458);let t153: f64 = (t152 - l.f5d4);let t154: f64 = (-230.25850929940458);let t155: f64 = (t154 - l.f5d4);let t156: f64 = (-230.25850929940458);let t157: f64 = (t156 - l.f5d4);let t158: f64 = (t157 * 0.3333333333333333);let t159: f64 = (1.0 + t158);let t15a: f64 = (t155 * t159);let t15b: f64 = (0.5 * t15a);let t15c: f64 = (1.0 + t15b);let t15d: f64 = (t153 * t15c);let t15e: f64 = (1.0 + t15d);let t15f: f64 = (1e-100 / t15e);l.f6fc = t15f;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18e == 0.0)) {let t160: f64 = (2.0 * l.f6fc);let t161: f64 = (t160 - l.f6e);l.f74 = t161;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) {let t162: f64 = (1.772453850905516 * 0.5);let t163: f64 = (l.fe * l.f74);let t164: f64 = (t163 / l.f5a8);let t165: f64 = (t162 * t164);l.fd6 = t165;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) {let t166: f64 = (l.f9 * l.fd6);let t167: f64 = (t166 * l.f7f5);let t168: f64 = (l.f3f * t167);l.f599 = t168;}
        let t169: f64 = if l.f24 == 0.0 { 1.0 } else { 0.0 };l.f192 = t169;
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f192 != 0.0)) {l.f529 = 0.0;}
        let t16a: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f194 = t16a;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f192 == 0.0)) && (l.f194 != 0.0)) {let t16b: f64 = (l.f771 - l.f750);let t16c: f64 = (t16b * l.f773);let t16d: f64 = (t16c).sqrt();l.f6fc = t16d;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f192 == 0.0)) && (l.f194 == 0.0)) {let t16e: f64 = (l.f771 - l.f750);let t16f: f64 = (t16e * l.f773);let t170: f64 = (t16f).powf(l.f623);l.f6fc = t170;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f192 == 0.0)) {let t171: f64 = (l.f771 - l.f750);let t172: f64 = (t171 * l.f7da);let t173: f64 = (t172 / l.f6fc);let t174: f64 = (l.f611 * t173);l.fb6 = t174;}
        let t175: f64 = (-l.fa1);let t176: f64 = (t175 / l.fb6);let t177: f64 = (t176).abs();let t178: f64 = if t177 < 230.25850929940458 { 1.0 } else { 0.0 };l.f196 = t178;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f192 == 0.0)) && (l.f196 != 0.0)) {let t179: f64 = (-l.fa1);let t17a: f64 = (t179 / l.fb6);let t17b: f64 = (t17a).exp();l.f6fc = t17b;}
        let t17c: f64 = (-l.fa1);let t17d: f64 = (t17c / l.fb6);let t17e: f64 = (-230.25850929940458);let t17f: f64 = if t17d < t17e { 1.0 } else { 0.0 };l.f198 = t17f;
        if (((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f192 == 0.0)) && (l.f196 == 0.0)) && (l.f198 != 0.0)) {let t180: f64 = (-230.25850929940458);let t181: f64 = (-l.fa1);let t182: f64 = (t181 / l.fb6);let t183: f64 = (t180 - t182);let t184: f64 = (-230.25850929940458);let t185: f64 = (-l.fa1);let t186: f64 = (t185 / l.fb6);let t187: f64 = (t184 - t186);let t188: f64 = (-230.25850929940458);let t189: f64 = (-l.fa1);let t18a: f64 = (t189 / l.fb6);let t18b: f64 = (t188 - t18a);let t18c: f64 = (t18b * 0.3333333333333333);let t18d: f64 = (1.0 + t18c);let t18e: f64 = (t187 * t18d);let t18f: f64 = (0.5 * t18e);let t190: f64 = (1.0 + t18f);let t191: f64 = (t183 * t190);let t192: f64 = (1.0 + t191);let t193: f64 = (1e-100 / t192);l.f6fc = t193;}
        if (((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f192 == 0.0)) && (l.f196 == 0.0)) && (l.f198 == 0.0)) {let t194: f64 = (-l.fa1);let t195: f64 = (t194 / l.fb6);let t196: f64 = (t195 - 230.25850929940458);let t197: f64 = (-l.fa1);let t198: f64 = (t197 / l.fb6);let t199: f64 = (t198 - 230.25850929940458);let t19a: f64 = (-l.fa1);let t19b: f64 = (t19a / l.fb6);let t19c: f64 = (t19b - 230.25850929940458);let t19d: f64 = (t19c * 0.3333333333333333);let t19e: f64 = (1.0 + t19d);let t19f: f64 = (t199 * t19e);let t1a0: f64 = (0.5 * t19f);let t1a1: f64 = (1.0 + t1a0);let t1a2: f64 = (t196 * t1a1);let t1a3: f64 = (1.0 + t1a2);let t1a4: f64 = (1e100 * t1a3);l.f6fc = t1a4;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f192 == 0.0)) {let t1a5: f64 = (l.f739 * l.fb6);let t1a6: f64 = (t1a5 * l.fb6);let t1a7: f64 = (t1a6 * l.f6fc);let t1a8: f64 = (l.f24 * t1a7);l.f529 = t1a8;}
        let t1a9: f64 = if ((l.f783 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f19a = t1a9;
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f19a != 0.0)) {l.fae = 1.0;}
        let t1aa: f64 = (-l.f2);let t1ab: f64 = (t1aa * l.f783);let t1ac: f64 = if l.f74a > t1ab { 1.0 } else { 0.0 };l.f19c = t1ac;let t1ad: f64 = if l.f625 == 4.0 { 1.0 } else { 0.0 };l.f19e = t1ad;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f19a == 0.0)) && (l.f19c != 0.0)) && (l.f19e != 0.0)) {let t1ae: f64 = (l.f74a * l.f787);let t1af: f64 = (t1ae).abs();let t1b0: f64 = (l.f74a * l.f787);let t1b1: f64 = (t1b0).abs();let t1b2: f64 = (t1af * t1b1);let t1b3: f64 = (l.f74a * l.f787);let t1b4: f64 = (t1b3).abs();let t1b5: f64 = (t1b2 * t1b4);let t1b6: f64 = (l.f74a * l.f787);let t1b7: f64 = (t1b6).abs();let t1b8: f64 = (t1b5 * t1b7);l.f6fc = t1b8;}
        if (((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f19a == 0.0)) && (l.f19c != 0.0)) && (l.f19e == 0.0)) {let t1b9: f64 = (l.f74a * l.f787);let t1ba: f64 = (t1b9).abs();let t1bb: f64 = (t1ba).powf(l.f625);l.f6fc = t1bb;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f19a == 0.0)) && (l.f19c != 0.0)) {let t1bc: f64 = (1.0 - l.f6fc);let t1bd: f64 = (1.0 / t1bc);l.fae = t1bd;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f19a == 0.0)) && (l.f19c == 0.0)) {let t1be: f64 = (l.f2 * l.f783);let t1bf: f64 = (l.f74a + t1be);let t1c0: f64 = (t1bf * l.f6ba);let t1c1: f64 = (l.fc3 + t1c0);l.fae = t1c1;}
        if ((l.f29a != 0.0) && (l.f17c == 0.0)) {let t1c2: f64 = (l.f52f + l.f593);let t1c3: f64 = (t1c2 + l.f599);let t1c4: f64 = (t1c3 + l.f529);let t1c5: f64 = (t1c4 * l.fae);(l.f562, l.f563, l.f564, ) = (t1c5, (l.f530 * l.fae), (l.f531 * l.fae), );let t1c6: f64 = (l.f593 + l.f599);let t1c7: f64 = (t1c6 + l.f529);let t1c8: f64 = (t1c7 * l.fae);(l.f552, l.f553, l.f554, ) = (t1c8, 0.0, 0.0, );}
        let t1c9: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f1a0 = t1c9;
        if ((l.f29a != 0.0) && (l.f1a0 != 0.0)) {(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );}
        let t1ca: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f1a2 = t1ca;
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a2 != 0.0)) {let t1cb: f64 = (l.f796 * l.f76d);let t1cc: f64 = (1.0 - t1cb);let t1cd: f64 = (t1cc).sqrt();l.f6fc = t1cd;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a2 == 0.0)) {let t1ce: f64 = (l.f796 * l.f76d);let t1cf: f64 = (1.0 - t1ce);let t1d0: f64 = (t1cf).powf(l.f60f);l.f6fc = t1d0;}
        if ((l.f29a != 0.0) && (l.f1a0 == 0.0)) {let t1d1: f64 = (1.0 - l.f6fc);let t1d2: f64 = (l.f6a2 * t1d1);let t1d3: f64 = (l.f739 - l.f796);let t1d4: f64 = (l.f69c * t1d3);let t1d5: f64 = (t1d2 + t1d4);(l.f694, l.f695, l.f696, ) = (t1d5, 0.0, 0.0, );let t1d6: f64 = (l.f54c * l.f53e);(l.f52f, l.f530, l.f531, ) = (t1d6, (l.f54c * l.f53f), (l.f54c * l.f540), );}
        let t1d7: f64 = if ((l.f3d == 0.0) && (l.f43 == 0.0)) { 1.0 } else { 0.0 };l.f1a4 = t1d7;
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 == 0.0)) {let t1d8: f64 = (l.f77d - l.f7a2);l.f758 = t1d8;let t1d9: f64 = (l.f714 / l.f758);let t1da: f64 = (1.0 - t1d9);let t1db: f64 = (t1da).sqrt();let t1dc: f64 = (1.0 - t1db);l.f7ef = t1dc;}
        let t1dd: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f1a6 = t1dd;
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 == 0.0)) && (l.f1a6 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 == 0.0)) && (l.f1a6 == 0.0)) {let t1de: f64 = (l.f7ef * l.f7ef);let t1df: f64 = (l.f7ef).ln();let t1e0: f64 = (t1de * t1df);let t1e1: f64 = (1.0 - l.f7ef);let t1e2: f64 = (t1e0 / t1e1);let t1e3: f64 = (t1e2 + l.f7ef);let t1e4: f64 = (2.0 * l.f653);let t1e5: f64 = (1.0 - t1e4);let t1e6: f64 = (t1e3 * t1e5);l.f66 = t1e6;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 == 0.0)) {let t1e7: f64 = (l.f7ef + l.f66);l.f7e9 = t1e7;}
        let t1e8: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f1a8 = t1e8;
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 == 0.0)) && (l.f1a8 != 0.0)) {let t1e9: f64 = (l.f758 * l.f77b);let t1ea: f64 = (t1e9).sqrt();l.f6fc = t1ea;}
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 == 0.0)) && (l.f1a8 == 0.0)) {let t1eb: f64 = (l.f758 * l.f77b);let t1ec: f64 = (t1eb).powf(l.f653);l.f6fc = t1ec;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 == 0.0)) {let t1ed: f64 = (l.f7e0 * l.f6fc);l.f7d1 = t1ed;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1a4 == 0.0)) {let t1ee: f64 = (l.f825 - 1.0);let t1ef: f64 = (t1ee * l.f7d1);let t1f0: f64 = (l.fd1 * t1ef);l.f9 = t1f0;let t1f1: f64 = (l.f9 * l.f7e9);let t1f2: f64 = (l.f3d * t1f1);l.f593 = t1f2;}
        let t1f3: f64 = if l.f43 == 0.0 { 1.0 } else { 0.0 };l.f1aa = t1f3;
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) {let t1f4: f64 = (l.f7d1 * l.f60f);let t1f5: f64 = (t1f4 / l.f758);let t1f6: f64 = (l.f22 * t1f5);l.f19 = t1f6;let t1f7: f64 = (0.666666666666667 * l.f12);let t1f8: f64 = (t1f7 / l.f19);l.f71a = t1f8;let t1f9: f64 = (l.f71a * l.f71a);l.f72c = t1f9;let t1fa: f64 = (l.f72c * l.f72c);let t1fb: f64 = (l.f72c * l.f72c);let t1fc: f64 = (t1fb + 1.0);let t1fd: f64 = (t1fa / t1fc);let t1fe: f64 = (t1fd).sqrt();l.f726 = t1fe;let t1ff: f64 = (l.f726).abs();let t200: f64 = (t1ff).sqrt();l.f6c1 = t200;let t201: f64 = (l.f726 * l.f6c1);l.f732 = t201;}
        let t202: f64 = (-l.f653);let t203: f64 = (t202 * l.f615);let t204: f64 = (-1.0);let t205: f64 = if t203 == t204 { 1.0 } else { 0.0 };l.f1ae = t205;
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1ae != 0.0)) {let t206: f64 = (l.f19 * l.f732);let t207: f64 = (1.0 + t206);let t208: f64 = (1.0 / t207);l.f7e3 = t208;}
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1ae == 0.0)) {let t209: f64 = (l.f19 * l.f732);let t20a: f64 = (1.0 + t209);let t20b: f64 = (-l.f653);let t20c: f64 = (t20b * l.f615);let t20d: f64 = (t20a).powf(t20c);l.f7e3 = t20d;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) {let t20e: f64 = (l.f7e9 * l.f7e3);let t20f: f64 = (l.f7e9 + l.f7e3);let t210: f64 = (t20e / t20f);l.f7f5 = t210;let t211: f64 = (l.f19 / l.f6c1);let t212: f64 = (0.375 * t211);let t213: f64 = (t212).sqrt();l.f5a8 = t213;let t214: f64 = (l.f71a * l.f6c1);let t215: f64 = (2.0 * t214);let t216: f64 = (t215 - l.f726);l.f5b4 = t216;let t217: f64 = (l.f12 * l.f71a);let t218: f64 = (t217 * l.f6c1);let t219: f64 = (l.f12 * l.f726);let t21a: f64 = (t218 - t219);let t21b: f64 = (l.f19 * l.f732);let t21c: f64 = (0.5 * t21b);let t21d: f64 = (t21a + t21c);l.f5d4 = t21d;let t21e: f64 = (l.f5b4 - 1.0);let t21f: f64 = (t21e * l.f5a8);l.f7fb = t21f;let t220: f64 = (l.f7fb * l.f7fb);l.f811 = t220;}
        let t221: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f1b0 = t221;
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1b0 != 0.0)) {let t222: f64 = (l.f62b * l.f7fb);let t223: f64 = (1.0 + t222);let t224: f64 = (1.0 / t223);l.f6e2 = t224;}
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1b0 == 0.0)) {let t225: f64 = (l.f62b * l.f7fb);let t226: f64 = (1.0 - t225);let t227: f64 = (1.0 / t226);l.f6e2 = t227;}
        let t228: f64 = (-l.f811);let t229: f64 = (t228 + l.f5d4);let t22a: f64 = (-230.25850929940458);let t22b: f64 = if t229 > t22a { 1.0 } else { 0.0 };l.f1b2 = t22b;
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1b2 != 0.0)) {let t22c: f64 = (-l.f811);let t22d: f64 = (t22c + l.f5d4);let t22e: f64 = (t22d).exp();l.f6fc = t22e;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1b2 == 0.0)) {let t22f: f64 = (-230.25850929940458);let t230: f64 = (-l.f811);let t231: f64 = (t230 + l.f5d4);let t232: f64 = (t22f - t231);let t233: f64 = (-230.25850929940458);let t234: f64 = (-l.f811);let t235: f64 = (t234 + l.f5d4);let t236: f64 = (t233 - t235);let t237: f64 = (-230.25850929940458);let t238: f64 = (-l.f811);let t239: f64 = (t238 + l.f5d4);let t23a: f64 = (t237 - t239);let t23b: f64 = (t23a * 0.3333333333333333);let t23c: f64 = (1.0 + t23b);let t23d: f64 = (t236 * t23c);let t23e: f64 = (0.5 * t23d);let t23f: f64 = (1.0 + t23e);let t240: f64 = (t232 * t23f);let t241: f64 = (1.0 + t240);let t242: f64 = (1e-100 / t241);l.f6fc = t242;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) {let t243: f64 = (0.29214664 * l.f6e2);let t244: f64 = (l.f6e2 * l.f6e2);let t245: f64 = (l.f16 * t244);let t246: f64 = (t243 + t245);let t247: f64 = (l.f6e2 * l.f6e2);let t248: f64 = (t247 * l.f6e2);let t249: f64 = (l.f2a * t248);let t24a: f64 = (t246 + t249);let t24b: f64 = (t24a * l.f6fc);l.f6e = t24b;}
        let t24c: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f1b4 = t24c;
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1b4 != 0.0)) {l.f74 = l.f6e;}
        let t24d: f64 = (-230.25850929940458);let t24e: f64 = if l.f5d4 > t24d { 1.0 } else { 0.0 };l.f1b6 = t24e;
        if (((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1b4 == 0.0)) && (l.f1b6 != 0.0)) {let t24f: f64 = (l.f5d4).exp();l.f6fc = t24f;}
        if (((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1b4 == 0.0)) && (l.f1b6 == 0.0)) {let t250: f64 = (-230.25850929940458);let t251: f64 = (t250 - l.f5d4);let t252: f64 = (-230.25850929940458);let t253: f64 = (t252 - l.f5d4);let t254: f64 = (-230.25850929940458);let t255: f64 = (t254 - l.f5d4);let t256: f64 = (t255 * 0.3333333333333333);let t257: f64 = (1.0 + t256);let t258: f64 = (t253 * t257);let t259: f64 = (0.5 * t258);let t25a: f64 = (1.0 + t259);let t25b: f64 = (t251 * t25a);let t25c: f64 = (1.0 + t25b);let t25d: f64 = (1e-100 / t25c);l.f6fc = t25d;}
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) && (l.f1b4 == 0.0)) {let t25e: f64 = (2.0 * l.f6fc);let t25f: f64 = (t25e - l.f6e);l.f74 = t25f;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1aa == 0.0)) {let t260: f64 = (1.772453850905516 * 0.5);let t261: f64 = (l.f12 * l.f74);let t262: f64 = (t261 / l.f5a8);let t263: f64 = (t260 * t262);l.fd6 = t263;let t264: f64 = (l.f9 * l.fd6);let t265: f64 = (t264 * l.f7f5);let t266: f64 = (l.f43 * t265);l.f599 = t266;}
        let t267: f64 = if l.f28 == 0.0 { 1.0 } else { 0.0 };l.f1b8 = t267;
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1b8 != 0.0)) {l.f529 = 0.0;}
        let t268: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f1ba = t268;
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1b8 == 0.0)) && (l.f1ba != 0.0)) {let t269: f64 = (l.f779 - l.f750);let t26a: f64 = (t269 * l.f77b);let t26b: f64 = (t26a).sqrt();l.f6fc = t26b;}
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1b8 == 0.0)) && (l.f1ba == 0.0)) {let t26c: f64 = (l.f779 - l.f750);let t26d: f64 = (t26c * l.f77b);let t26e: f64 = (t26d).powf(l.f653);l.f6fc = t26e;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1b8 == 0.0)) {let t26f: f64 = (l.f779 - l.f750);let t270: f64 = (t26f * l.f7de);let t271: f64 = (t270 / l.f6fc);let t272: f64 = (l.f615 * t271);l.fb6 = t272;}
        let t273: f64 = (-l.fab);let t274: f64 = (t273 / l.fb6);let t275: f64 = (t274).abs();let t276: f64 = if t275 < 230.25850929940458 { 1.0 } else { 0.0 };l.f1bc = t276;
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1b8 == 0.0)) && (l.f1bc != 0.0)) {let t277: f64 = (-l.fab);let t278: f64 = (t277 / l.fb6);let t279: f64 = (t278).exp();l.f6fc = t279;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t27a: f64 = (-l.fab);let t27b: f64 = (t27a / l.fb6);let t27c: f64 = (-230.25850929940458);let t27d: f64 = if t27b < t27c { 1.0 } else { 0.0 };l.f1be = t27d;
        if (((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1b8 == 0.0)) && (l.f1bc == 0.0)) && (l.f1be != 0.0)) {let t27e: f64 = (-230.25850929940458);let t27f: f64 = (-l.fab);let t280: f64 = (t27f / l.fb6);let t281: f64 = (t27e - t280);let t282: f64 = (-230.25850929940458);let t283: f64 = (-l.fab);let t284: f64 = (t283 / l.fb6);let t285: f64 = (t282 - t284);let t286: f64 = (-230.25850929940458);let t287: f64 = (-l.fab);let t288: f64 = (t287 / l.fb6);let t289: f64 = (t286 - t288);let t28a: f64 = (t289 * 0.3333333333333333);let t28b: f64 = (1.0 + t28a);let t28c: f64 = (t285 * t28b);let t28d: f64 = (0.5 * t28c);let t28e: f64 = (1.0 + t28d);let t28f: f64 = (t281 * t28e);let t290: f64 = (1.0 + t28f);let t291: f64 = (1e-100 / t290);l.f6fc = t291;}
        if (((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1b8 == 0.0)) && (l.f1bc == 0.0)) && (l.f1be == 0.0)) {let t292: f64 = (-l.fab);let t293: f64 = (t292 / l.fb6);let t294: f64 = (t293 - 230.25850929940458);let t295: f64 = (-l.fab);let t296: f64 = (t295 / l.fb6);let t297: f64 = (t296 - 230.25850929940458);let t298: f64 = (-l.fab);let t299: f64 = (t298 / l.fb6);let t29a: f64 = (t299 - 230.25850929940458);let t29b: f64 = (t29a * 0.3333333333333333);let t29c: f64 = (1.0 + t29b);let t29d: f64 = (t297 * t29c);let t29e: f64 = (0.5 * t29d);let t29f: f64 = (1.0 + t29e);let t2a0: f64 = (t294 * t29f);let t2a1: f64 = (1.0 + t2a0);let t2a2: f64 = (1e100 * t2a1);l.f6fc = t2a2;}
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1b8 == 0.0)) {let t2a3: f64 = (l.f739 * l.fb6);let t2a4: f64 = (t2a3 * l.fb6);let t2a5: f64 = (t2a4 * l.f6fc);let t2a6: f64 = (l.f28 * t2a5);l.f529 = t2a6;}
        let t2a7: f64 = if ((l.f78d > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f1c0 = t2a7;
        if (((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1c0 != 0.0)) {l.fae = 1.0;}
        let t2a8: f64 = (-l.f2);let t2a9: f64 = (t2a8 * l.f78d);let t2aa: f64 = if l.f74a > t2a9 { 1.0 } else { 0.0 };l.f1c2 = t2aa;let t2ab: f64 = if l.f629 == 4.0 { 1.0 } else { 0.0 };l.f1c4 = t2ab;
        if (((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1c0 == 0.0)) && (l.f1c2 != 0.0)) && (l.f1c4 != 0.0)) {let t2ac: f64 = (l.f74a * l.f78b);let t2ad: f64 = (t2ac).abs();let t2ae: f64 = (l.f74a * l.f78b);let t2af: f64 = (t2ae).abs();let t2b0: f64 = (t2ad * t2af);let t2b1: f64 = (l.f74a * l.f78b);let t2b2: f64 = (t2b1).abs();let t2b3: f64 = (t2b0 * t2b2);let t2b4: f64 = (l.f74a * l.f78b);let t2b5: f64 = (t2b4).abs();let t2b6: f64 = (t2b3 * t2b5);l.f6fc = t2b6;}
        if (((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1c0 == 0.0)) && (l.f1c2 != 0.0)) && (l.f1c4 == 0.0)) {let t2b7: f64 = (l.f74a * l.f78b);let t2b8: f64 = (t2b7).abs();let t2b9: f64 = (t2b8).powf(l.f629);l.f6fc = t2b9;}
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1c0 == 0.0)) && (l.f1c2 != 0.0)) {let t2ba: f64 = (1.0 - l.f6fc);let t2bb: f64 = (1.0 / t2ba);l.fae = t2bb;}
        if ((((l.f29a != 0.0) && (l.f1a0 == 0.0)) && (l.f1c0 == 0.0)) && (l.f1c2 == 0.0)) {let t2bc: f64 = (l.f2 * l.f78d);let t2bd: f64 = (l.f74a + t2bc);let t2be: f64 = (t2bd * l.f6be);let t2bf: f64 = (l.fc7 + t2be);l.fae = t2bf;}
        if ((l.f29a != 0.0) && (l.f1a0 == 0.0)) {let t2c0: f64 = (l.f52f + l.f593);let t2c1: f64 = (t2c0 + l.f599);let t2c2: f64 = (t2c1 + l.f529);let t2c3: f64 = (t2c2 * l.fae);(l.f576, l.f577, l.f578, ) = (t2c3, (l.f530 * l.fae), (l.f531 * l.fae), );let t2c4: f64 = (l.f593 + l.f599);let t2c5: f64 = (t2c4 + l.f529);let t2c6: f64 = (t2c5 * l.fae);(l.f55a, l.f55b, l.f55c, ) = (t2c6, 0.0, 0.0, );}
        let t2c7: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f1c6 = t2c7;
        if ((l.f29a != 0.0) && (l.f1c6 != 0.0)) {(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        l: &mut StampLocals,
    ) {
        let t2c8: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f1c8 = t2c8;
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1c8 != 0.0)) {let t2c9: f64 = (l.f796 * l.f76b);let t2ca: f64 = (1.0 - t2c9);let t2cb: f64 = (t2ca).sqrt();l.f6fc = t2cb;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1c8 == 0.0)) {let t2cc: f64 = (l.f796 * l.f76b);let t2cd: f64 = (1.0 - t2cc);let t2ce: f64 = (t2cd).powf(l.f60d);l.f6fc = t2ce;}
        if ((l.f29a != 0.0) && (l.f1c6 == 0.0)) {let t2cf: f64 = (1.0 - l.f6fc);let t2d0: f64 = (l.f6a0 * t2cf);let t2d1: f64 = (l.f739 - l.f796);let t2d2: f64 = (l.f69a * t2d1);let t2d3: f64 = (t2d0 + t2d2);(l.f690, l.f691, l.f692, ) = (t2d3, 0.0, 0.0, );let t2d4: f64 = (l.f544 * l.f53a);(l.f52f, l.f530, l.f531, ) = (t2d4, (l.f544 * l.f53b), (l.f544 * l.f53c), );}
        let t2d5: f64 = if ((l.f3b == 0.0) && (l.f41 == 0.0)) { 1.0 } else { 0.0 };l.f1ca = t2d5;
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1ca != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1ca == 0.0)) {let t2d6: f64 = (l.f763 - l.f7a2);l.f758 = t2d6;let t2d7: f64 = (l.f714 / l.f758);let t2d8: f64 = (1.0 - t2d7);let t2d9: f64 = (t2d8).sqrt();let t2da: f64 = (1.0 - t2d9);l.f7ef = t2da;}
        let t2db: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f1cc = t2db;
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1ca == 0.0)) && (l.f1cc != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1ca == 0.0)) && (l.f1cc == 0.0)) {let t2dc: f64 = (l.f7ef * l.f7ef);let t2dd: f64 = (l.f7ef).ln();let t2de: f64 = (t2dc * t2dd);let t2df: f64 = (1.0 - l.f7ef);let t2e0: f64 = (t2de / t2df);let t2e1: f64 = (t2e0 + l.f7ef);let t2e2: f64 = (2.0 * l.f62f);let t2e3: f64 = (1.0 - t2e2);let t2e4: f64 = (t2e1 * t2e3);l.f66 = t2e4;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1ca == 0.0)) {let t2e5: f64 = (l.f7ef + l.f66);l.f7e9 = t2e5;}
        let t2e6: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f1ce = t2e6;
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1ca == 0.0)) && (l.f1ce != 0.0)) {let t2e7: f64 = (l.f758 * l.f777);let t2e8: f64 = (t2e7).sqrt();l.f6fc = t2e8;}
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1ca == 0.0)) && (l.f1ce == 0.0)) {let t2e9: f64 = (l.f758 * l.f777);let t2ea: f64 = (t2e9).powf(l.f62f);l.f6fc = t2ea;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1ca == 0.0)) {let t2eb: f64 = (l.f7d8 * l.f6fc);l.f7d1 = t2eb;let t2ec: f64 = (l.f825 - 1.0);let t2ed: f64 = (t2ec * l.f7d1);let t2ee: f64 = (l.fcd * t2ed);l.f9 = t2ee;let t2ef: f64 = (l.f9 * l.f7e9);let t2f0: f64 = (l.f3b * t2ef);l.f593 = t2f0;}
        let t2f1: f64 = if l.f41 == 0.0 { 1.0 } else { 0.0 };l.f1d0 = t2f1;
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) {let t2f2: f64 = (l.f7d1 * l.f60d);let t2f3: f64 = (t2f2 / l.f758);let t2f4: f64 = (l.f20 * t2f3);l.f19 = t2f4;let t2f5: f64 = (0.666666666666667 * l.f10);let t2f6: f64 = (t2f5 / l.f19);l.f71a = t2f6;let t2f7: f64 = (l.f71a * l.f71a);l.f72c = t2f7;let t2f8: f64 = (l.f72c * l.f72c);let t2f9: f64 = (l.f72c * l.f72c);let t2fa: f64 = (t2f9 + 1.0);let t2fb: f64 = (t2f8 / t2fa);let t2fc: f64 = (t2fb).sqrt();l.f726 = t2fc;let t2fd: f64 = (l.f726).abs();let t2fe: f64 = (t2fd).sqrt();l.f6c1 = t2fe;let t2ff: f64 = (l.f726 * l.f6c1);l.f732 = t2ff;}
        let t300: f64 = (-l.f62f);let t301: f64 = (t300 * l.f613);let t302: f64 = (-1.0);let t303: f64 = if t301 == t302 { 1.0 } else { 0.0 };l.f1d2 = t303;
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d2 != 0.0)) {let t304: f64 = (l.f19 * l.f732);let t305: f64 = (1.0 + t304);let t306: f64 = (1.0 / t305);l.f7e3 = t306;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d2 == 0.0)) {let t307: f64 = (l.f19 * l.f732);let t308: f64 = (1.0 + t307);let t309: f64 = (-l.f62f);let t30a: f64 = (t309 * l.f613);let t30b: f64 = (t308).powf(t30a);l.f7e3 = t30b;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) {let t30c: f64 = (l.f7e9 * l.f7e3);let t30d: f64 = (l.f7e9 + l.f7e3);let t30e: f64 = (t30c / t30d);l.f7f5 = t30e;let t30f: f64 = (l.f19 / l.f6c1);let t310: f64 = (0.375 * t30f);let t311: f64 = (t310).sqrt();l.f5a8 = t311;let t312: f64 = (l.f71a * l.f6c1);let t313: f64 = (2.0 * t312);let t314: f64 = (t313 - l.f726);l.f5b4 = t314;let t315: f64 = (l.f10 * l.f71a);let t316: f64 = (t315 * l.f6c1);let t317: f64 = (l.f10 * l.f726);let t318: f64 = (t316 - t317);let t319: f64 = (l.f19 * l.f732);let t31a: f64 = (0.5 * t319);let t31b: f64 = (t318 + t31a);l.f5d4 = t31b;let t31c: f64 = (l.f5b4 - 1.0);let t31d: f64 = (t31c * l.f5a8);l.f7fb = t31d;let t31e: f64 = (l.f7fb * l.f7fb);l.f811 = t31e;}
        let t31f: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f1d4 = t31f;
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d4 != 0.0)) {let t320: f64 = (l.f62b * l.f7fb);let t321: f64 = (1.0 + t320);let t322: f64 = (1.0 / t321);l.f6e2 = t322;}
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d4 == 0.0)) {let t323: f64 = (l.f62b * l.f7fb);let t324: f64 = (1.0 - t323);let t325: f64 = (1.0 / t324);l.f6e2 = t325;}
        let t326: f64 = (-l.f811);let t327: f64 = (t326 + l.f5d4);let t328: f64 = (-230.25850929940458);let t329: f64 = if t327 > t328 { 1.0 } else { 0.0 };l.f1d7 = t329;
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d7 != 0.0)) {let t32a: f64 = (-l.f811);let t32b: f64 = (t32a + l.f5d4);let t32c: f64 = (t32b).exp();l.f6fc = t32c;}
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d7 == 0.0)) {let t32d: f64 = (-230.25850929940458);let t32e: f64 = (-l.f811);let t32f: f64 = (t32e + l.f5d4);let t330: f64 = (t32d - t32f);let t331: f64 = (-230.25850929940458);let t332: f64 = (-l.f811);let t333: f64 = (t332 + l.f5d4);let t334: f64 = (t331 - t333);let t335: f64 = (-230.25850929940458);let t336: f64 = (-l.f811);let t337: f64 = (t336 + l.f5d4);let t338: f64 = (t335 - t337);let t339: f64 = (t338 * 0.3333333333333333);let t33a: f64 = (1.0 + t339);let t33b: f64 = (t334 * t33a);let t33c: f64 = (0.5 * t33b);let t33d: f64 = (1.0 + t33c);let t33e: f64 = (t330 * t33d);let t33f: f64 = (1.0 + t33e);let t340: f64 = (1e-100 / t33f);l.f6fc = t340;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) {let t341: f64 = (0.29214664 * l.f6e2);let t342: f64 = (l.f6e2 * l.f6e2);let t343: f64 = (l.f16 * t342);let t344: f64 = (t341 + t343);let t345: f64 = (l.f6e2 * l.f6e2);let t346: f64 = (t345 * l.f6e2);let t347: f64 = (l.f2a * t346);let t348: f64 = (t344 + t347);let t349: f64 = (t348 * l.f6fc);l.f6e = t349;}
        let t34a: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f1d9 = t34a;
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d9 != 0.0)) {l.f74 = l.f6e;}
        let t34b: f64 = (-230.25850929940458);let t34c: f64 = if l.f5d4 > t34b { 1.0 } else { 0.0 };l.f1db = t34c;
        if (((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d9 == 0.0)) && (l.f1db != 0.0)) {let t34d: f64 = (l.f5d4).exp();l.f6fc = t34d;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d9 == 0.0)) && (l.f1db == 0.0)) {let t34e: f64 = (-230.25850929940458);let t34f: f64 = (t34e - l.f5d4);let t350: f64 = (-230.25850929940458);let t351: f64 = (t350 - l.f5d4);let t352: f64 = (-230.25850929940458);let t353: f64 = (t352 - l.f5d4);let t354: f64 = (t353 * 0.3333333333333333);let t355: f64 = (1.0 + t354);let t356: f64 = (t351 * t355);let t357: f64 = (0.5 * t356);let t358: f64 = (1.0 + t357);let t359: f64 = (t34f * t358);let t35a: f64 = (1.0 + t359);let t35b: f64 = (1e-100 / t35a);l.f6fc = t35b;}
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) && (l.f1d9 == 0.0)) {let t35c: f64 = (2.0 * l.f6fc);let t35d: f64 = (t35c - l.f6e);l.f74 = t35d;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1d0 == 0.0)) {let t35e: f64 = (1.772453850905516 * 0.5);let t35f: f64 = (l.f10 * l.f74);let t360: f64 = (t35f / l.f5a8);let t361: f64 = (t35e * t360);l.fd6 = t361;let t362: f64 = (l.f9 * l.fd6);let t363: f64 = (t362 * l.f7f5);let t364: f64 = (l.f41 * t363);l.f599 = t364;}
        let t365: f64 = if l.f26 == 0.0 { 1.0 } else { 0.0 };l.f1dd = t365;
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1dd != 0.0)) {l.f529 = 0.0;}
        let t366: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f1df = t366;
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1dd == 0.0)) && (l.f1df != 0.0)) {let t367: f64 = (l.f775 - l.f750);let t368: f64 = (t367 * l.f777);let t369: f64 = (t368).sqrt();l.f6fc = t369;}
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1dd == 0.0)) && (l.f1df == 0.0)) {let t36a: f64 = (l.f775 - l.f750);let t36b: f64 = (t36a * l.f777);let t36c: f64 = (t36b).powf(l.f62f);l.f6fc = t36c;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1dd == 0.0)) {let t36d: f64 = (l.f775 - l.f750);let t36e: f64 = (t36d * l.f7dc);let t36f: f64 = (t36e / l.f6fc);let t370: f64 = (l.f613 * t36f);l.fb6 = t370;}
        let t371: f64 = (-l.fa3);let t372: f64 = (t371 / l.fb6);let t373: f64 = (t372).abs();let t374: f64 = if t373 < 230.25850929940458 { 1.0 } else { 0.0 };l.f1e1 = t374;
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1dd == 0.0)) && (l.f1e1 != 0.0)) {let t375: f64 = (-l.fa3);let t376: f64 = (t375 / l.fb6);let t377: f64 = (t376).exp();l.f6fc = t377;}
        let t378: f64 = (-l.fa3);let t379: f64 = (t378 / l.fb6);let t37a: f64 = (-230.25850929940458);let t37b: f64 = if t379 < t37a { 1.0 } else { 0.0 };l.f1e3 = t37b;
        if (((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1dd == 0.0)) && (l.f1e1 == 0.0)) && (l.f1e3 != 0.0)) {let t37c: f64 = (-230.25850929940458);let t37d: f64 = (-l.fa3);let t37e: f64 = (t37d / l.fb6);let t37f: f64 = (t37c - t37e);let t380: f64 = (-230.25850929940458);let t381: f64 = (-l.fa3);let t382: f64 = (t381 / l.fb6);let t383: f64 = (t380 - t382);let t384: f64 = (-230.25850929940458);let t385: f64 = (-l.fa3);let t386: f64 = (t385 / l.fb6);let t387: f64 = (t384 - t386);let t388: f64 = (t387 * 0.3333333333333333);let t389: f64 = (1.0 + t388);let t38a: f64 = (t383 * t389);let t38b: f64 = (0.5 * t38a);let t38c: f64 = (1.0 + t38b);let t38d: f64 = (t37f * t38c);let t38e: f64 = (1.0 + t38d);let t38f: f64 = (1e-100 / t38e);l.f6fc = t38f;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1dd == 0.0)) && (l.f1e1 == 0.0)) && (l.f1e3 == 0.0)) {let t390: f64 = (-l.fa3);let t391: f64 = (t390 / l.fb6);let t392: f64 = (t391 - 230.25850929940458);let t393: f64 = (-l.fa3);let t394: f64 = (t393 / l.fb6);let t395: f64 = (t394 - 230.25850929940458);let t396: f64 = (-l.fa3);let t397: f64 = (t396 / l.fb6);let t398: f64 = (t397 - 230.25850929940458);let t399: f64 = (t398 * 0.3333333333333333);let t39a: f64 = (1.0 + t399);let t39b: f64 = (t395 * t39a);let t39c: f64 = (0.5 * t39b);let t39d: f64 = (1.0 + t39c);let t39e: f64 = (t392 * t39d);let t39f: f64 = (1.0 + t39e);let t3a0: f64 = (1e100 * t39f);l.f6fc = t3a0;}
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1dd == 0.0)) {let t3a1: f64 = (l.f739 * l.fb6);let t3a2: f64 = (t3a1 * l.fb6);let t3a3: f64 = (t3a2 * l.f6fc);let t3a4: f64 = (l.f26 * t3a3);l.f529 = t3a4;}
        let t3a5: f64 = if ((l.f785 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f1e5 = t3a5;
        if (((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1e5 != 0.0)) {l.fae = 1.0;}
        let t3a6: f64 = (-l.f2);let t3a7: f64 = (t3a6 * l.f785);let t3a8: f64 = if l.f74a > t3a7 { 1.0 } else { 0.0 };l.f1e7 = t3a8;let t3a9: f64 = if l.f627 == 4.0 { 1.0 } else { 0.0 };l.f1e9 = t3a9;
        if (((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1e5 == 0.0)) && (l.f1e7 != 0.0)) && (l.f1e9 != 0.0)) {let t3aa: f64 = (l.f74a * l.f789);let t3ab: f64 = (t3aa).abs();let t3ac: f64 = (l.f74a * l.f789);let t3ad: f64 = (t3ac).abs();let t3ae: f64 = (t3ab * t3ad);let t3af: f64 = (l.f74a * l.f789);let t3b0: f64 = (t3af).abs();let t3b1: f64 = (t3ae * t3b0);let t3b2: f64 = (l.f74a * l.f789);let t3b3: f64 = (t3b2).abs();let t3b4: f64 = (t3b1 * t3b3);l.f6fc = t3b4;}
        if (((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1e5 == 0.0)) && (l.f1e7 != 0.0)) && (l.f1e9 == 0.0)) {let t3b5: f64 = (l.f74a * l.f789);let t3b6: f64 = (t3b5).abs();let t3b7: f64 = (t3b6).powf(l.f627);l.f6fc = t3b7;}
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1e5 == 0.0)) && (l.f1e7 != 0.0)) {let t3b8: f64 = (1.0 - l.f6fc);let t3b9: f64 = (1.0 / t3b8);l.fae = t3b9;}
        if ((((l.f29a != 0.0) && (l.f1c6 == 0.0)) && (l.f1e5 == 0.0)) && (l.f1e7 == 0.0)) {let t3ba: f64 = (l.f2 * l.f785);let t3bb: f64 = (l.f74a + t3ba);let t3bc: f64 = (t3bb * l.f6bc);let t3bd: f64 = (l.fc5 + t3bc);l.fae = t3bd;}
        if ((l.f29a != 0.0) && (l.f1c6 == 0.0)) {let t3be: f64 = (l.f52f + l.f593);let t3bf: f64 = (t3be + l.f599);let t3c0: f64 = (t3bf + l.f529);let t3c1: f64 = (t3c0 * l.fae);(l.f56e, l.f56f, l.f570, ) = (t3c1, (l.f530 * l.fae), (l.f531 * l.fae), );let t3c2: f64 = (l.f593 + l.f599);let t3c3: f64 = (t3c2 + l.f529);let t3c4: f64 = (t3c3 * l.fae);(l.f556, l.f557, l.f558, ) = (t3c4, 0.0, 0.0, );}
        if (l.f29a != 0.0) {let t3c5: f64 = (l.f0 * l.f562);let t3c6: f64 = (l.f5b1 * l.f576);let t3c7: f64 = (t3c5 + t3c6);let t3c8: f64 = (l.f5af * l.f56e);let t3c9: f64 = (t3c7 + t3c8);(l.f508, l.f50d, l.f50e, ) = (t3c9, (((l.f0 * l.f563) + (l.f5b1 * l.f577)) + (l.f5af * l.f56f)), (((l.f0 * l.f564) + (l.f5b1 * l.f578)) + (l.f5af * l.f570)), );}
        let t3ca: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f1ec = t3ca;
        if ((l.f29a != 0.0) && (l.f1ec != 0.0)) {let t3cb: f64 = (4.0 * l.f78f);let t3cc: f64 = (t3cb * l.f78f);l.f4e1 = t3cc;let t3cd: f64 = (l.f78f / l.f791);l.f4e5 = t3cd;let t3ce: f64 = (l.f78f * l.f4e5);let t3cf: f64 = (l.f73b + t3ce);l.f4e9 = t3cf;let t3d0: f64 = (l.f791 + l.f4e9);l.f4ef = t3d0;let t3d1: f64 = (l.f791 - l.f4e9);l.f4f5 = t3d1;let t3d2: f64 = (l.f4f5 * l.f4f5);let t3d3: f64 = (t3d2 + l.f4e1);let t3d4: f64 = (t3d3).sqrt();l.f4fb = t3d4;let t3d5: f64 = (l.f73b * l.f791);let t3d6: f64 = (l.f4ef + l.f4fb);let t3d7: f64 = (t3d5 / t3d6);let t3d8: f64 = (2.0 * t3d7);l.f796 = t3d8;}
        let t3d9: f64 = if l.f73b < l.f7b1 { 1.0 } else { 0.0 };l.f1ee = t3d9;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t3da: f64 = (l.f73b * l.f645);let t3db: f64 = (0.5 * t3da);let t3dc: f64 = (t3db).abs();let t3dd: f64 = if t3dc < 230.25850929940458 { 1.0 } else { 0.0 };l.f1f0 = t3dd;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f0 != 0.0)) {let t3de: f64 = (l.f73b * l.f645);let t3df: f64 = (0.5 * t3de);let t3e0: f64 = (t3df).exp();l.f825 = t3e0;}
        let t3e1: f64 = (l.f73b * l.f645);let t3e2: f64 = (0.5 * t3e1);let t3e3: f64 = (-230.25850929940458);let t3e4: f64 = if t3e2 < t3e3 { 1.0 } else { 0.0 };l.f1f2 = t3e4;
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f0 == 0.0)) && (l.f1f2 != 0.0)) {let t3e5: f64 = (-230.25850929940458);let t3e6: f64 = (l.f73b * l.f645);let t3e7: f64 = (0.5 * t3e6);let t3e8: f64 = (t3e5 - t3e7);let t3e9: f64 = (-230.25850929940458);let t3ea: f64 = (l.f73b * l.f645);let t3eb: f64 = (0.5 * t3ea);let t3ec: f64 = (t3e9 - t3eb);let t3ed: f64 = (-230.25850929940458);let t3ee: f64 = (l.f73b * l.f645);let t3ef: f64 = (0.5 * t3ee);let t3f0: f64 = (t3ed - t3ef);let t3f1: f64 = (t3f0 * 0.3333333333333333);let t3f2: f64 = (1.0 + t3f1);let t3f3: f64 = (t3ec * t3f2);let t3f4: f64 = (0.5 * t3f3);let t3f5: f64 = (1.0 + t3f4);let t3f6: f64 = (t3e8 * t3f5);let t3f7: f64 = (1.0 + t3f6);let t3f8: f64 = (1e-100 / t3f7);l.f825 = t3f8;}
        if (((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f0 == 0.0)) && (l.f1f2 == 0.0)) {let t3f9: f64 = (l.f73b * l.f645);let t3fa: f64 = (0.5 * t3f9);let t3fb: f64 = (t3fa - 230.25850929940458);let t3fc: f64 = (l.f73b * l.f645);let t3fd: f64 = (0.5 * t3fc);let t3fe: f64 = (t3fd - 230.25850929940458);let t3ff: f64 = (l.f73b * l.f645);let t400: f64 = (0.5 * t3ff);let t401: f64 = (t400 - 230.25850929940458);let t402: f64 = (t401 * 0.3333333333333333);let t403: f64 = (1.0 + t402);let t404: f64 = (t3fe * t403);let t405: f64 = (0.5 * t404);let t406: f64 = (1.0 + t405);let t407: f64 = (t3fb * t406);let t408: f64 = (1.0 + t407);let t409: f64 = (1e100 * t408);l.f825 = t409;}
        if (((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) {let t40a: f64 = (l.f5eb * l.f5eb);let t40b: f64 = (t40a / l.f5df);l.f64f = t40b;let t40c: f64 = (l.f5e5 / l.f645);let t40d: f64 = (l.f5df / l.f64f);let t40e: f64 = (t40d).ln();let t40f: f64 = (t40c * t40e);l.f793 = t40f;}
        let t410: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f1f4 = t410;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {let t411: f64 = (l.f73b - l.f793);let t412: f64 = (p.p86 * t411);let t413: f64 = (t412 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t413, 0.0, 0.0, );let t414: f64 = (p.p86 * l.f793);let t415: f64 = (l.f5e5 - t414);(l.f5ed, l.f5ee, l.f5ef, ) = (t415, 0.0, 0.0, );let t416: f64 = (p.p85 - l.f601);let t417: f64 = (t416 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t417, (-l.f602), (-l.f603), );let t418: f64 = (4.0 * p.p85);let t419: f64 = (t418 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t419, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {
            let (t41b, t41c, t41d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t41a: f64 = (-l.f6f7);
        (t41a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t41b, t41c, t41d, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {let t41e: f64 = (l.f6f3 * l.f6f3);let t41f: f64 = (t41e + l.f6f7);let t420: f64 = (t41f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t420, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t420)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t420)), );let t421: f64 = (l.f6f3 + l.f6f7);let t422: f64 = (0.5 * t421);let t423: f64 = (p.p85 - t422);(l.f605, l.f606, l.f607, ) = (t423, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t424: f64 = (l.f605 - l.f5e5);let t425: f64 = (t424 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t425, l.f606, l.f607, );let t426: f64 = (4.0 * l.f5e5);let t427: f64 = (t426 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t427, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {
            let (t429, t42a, t42b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t428: f64 = (-l.f6f7);
        (t428, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t429, t42a, t42b, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {let t42c: f64 = (l.f6f3 * l.f6f3);let t42d: f64 = (t42c + l.f6f7);let t42e: f64 = (t42d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t42e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t42e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t42e)), );let t42f: f64 = (l.f6f3 + l.f6f7);let t430: f64 = (0.5 * t42f);let t431: f64 = (l.f5e5 + t430);(l.f5f1, l.f5f2, l.f5f3, ) = (t431, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t432: f64 = (p.p85 - l.f5ed);let t433: f64 = (t432 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t433, (-l.f5ee), (-l.f5ef), );let t434: f64 = (4.0 * p.p85);let t435: f64 = (t434 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t435, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {
            let (t437, t438, t439,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t436: f64 = (-l.f6f7);
        (t436, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t437, t438, t439, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {let t43a: f64 = (l.f6f3 * l.f6f3);let t43b: f64 = (t43a + l.f6f7);let t43c: f64 = (t43b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t43c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t43c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t43c)), );let t43d: f64 = (l.f6f3 + l.f6f7);let t43e: f64 = (0.5 * t43d);let t43f: f64 = (p.p85 - t43e);(l.f5ed, l.f5ee, l.f5ef, ) = (t43f, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t440: f64 = (l.f5ed - l.f5e5);let t441: f64 = (t440 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t441, l.f5ee, l.f5ef, );let t442: f64 = (4.0 * l.f5e5);let t443: f64 = (t442 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t443, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {
            let (t445, t446, t447,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t444: f64 = (-l.f6f7);
        (t444, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t445, t446, t447, );
        }
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 != 0.0)) {let t448: f64 = (l.f6f3 * l.f6f3);let t449: f64 = (t448 + l.f6f7);let t44a: f64 = (t449).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t44a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t44a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t44a)), );let t44b: f64 = (l.f6f3 + l.f6f7);let t44c: f64 = (0.5 * t44b);let t44d: f64 = (l.f5e5 + t44c);(l.f5ed, l.f5ee, l.f5ef, ) = (t44d, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f4 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );}
        let t44e: f64 = (l.f73b / l.f5f1);let t44f: f64 = (l.f5f1 - l.f5ed);let t450: f64 = (l.f793 * t44f);let t451: f64 = (l.f5ed * p.p85);let t452: f64 = (t450 / t451);let t453: f64 = (t44e + t452);let t454: f64 = (l.f645 * t453);let t455: f64 = (t454).abs();let t456: f64 = if t455 < 230.25850929940458 { 1.0 } else { 0.0 };l.f1f6 = t456;
        if ((((l.f29a != 0.0) && (l.f1ec != 0.0)) && (l.f1ee != 0.0)) && (l.f1f6 != 0.0)) {let t457: f64 = (l.f73b / l.f5f1);let t458: f64 = (l.f5f1 - l.f5ed);let t459: f64 = (l.f793 * t458);let t45a: f64 = (l.f5ed * p.p85);let t45b: f64 = (t459 / t45a);let t45c: f64 = (t457 + t45b);let t45d: f64 = (l.f645 * t45c);let t45e: f64 = (t45d).exp();(l.f536, l.f537, l.f538, ) = (t45e, (t45e * (l.f645 * ((-((l.f73b * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t45a) - (t459 * (l.f5ee * p.p85))) / (t45a * t45a))))), (t45e * (l.f645 * ((-((l.f73b * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t45a) - (t459 * (l.f5ef * p.p85))) / (t45a * t45a))))), );}
        let t45f: f64 = (l.f73b / l.f5f1);let t460: f64 = (l.f5f1 - l.f5ed);let t461: f64 = (l.f793 * t460);let t462: f64 = (l.f5ed * p.p85);let t463: f64 = (t461 / t462);let t464: f64 = (t45f + t463);let t465: f64 = (l.f645 * t464);let t466: f64 = (-230.25850929940458);let t467: f64 = if t465 < t466 { 1.0 } else { 0.0 };l.f1f8 = t467;
    }
}
