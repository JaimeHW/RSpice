#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f31e == 0.0)) && (l.f320 == 0.0)) {let t0: f64 = (-230.25850929940458);let t1: f64 = (t0 - l.f5d4);let t2: f64 = (-230.25850929940458);let t3: f64 = (t2 - l.f5d4);let t4: f64 = (-230.25850929940458);let t5: f64 = (t4 - l.f5d4);let t6: f64 = (t5 * 0.3333333333333333);let t7: f64 = (1.0 + t6);let t8: f64 = (t3 * t7);let t9: f64 = (0.5 * t8);let ta: f64 = (1.0 + t9);let tb: f64 = (t1 * ta);let tc: f64 = (1.0 + tb);let td: f64 = (1e-100 / tc);l.f6fc = td;}
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) && (l.f31e == 0.0)) {let te: f64 = (2.0 * l.f6fc);let tf: f64 = (te - l.f6e);l.f74 = tf;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f316 == 0.0)) {let t10: f64 = (1.772453850905516 * 0.5);let t11: f64 = (l.f10 * l.f74);let t12: f64 = (t11 / l.f5a8);let t13: f64 = (t10 * t12);l.fd6 = t13;let t14: f64 = (l.f9 * l.fd6);let t15: f64 = (t14 * l.f7f5);let t16: f64 = (l.f41 * t15);l.f599 = t16;}
        let t17: f64 = if l.f26 == 0.0 { 1.0 } else { 0.0 };l.f322 = t17;
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f322 != 0.0)) {l.f529 = 0.0;}
        let t18: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f324 = t18;
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f322 == 0.0)) && (l.f324 != 0.0)) {let t19: f64 = (l.f775 - l.f750);let t1a: f64 = (t19 * l.f777);let t1b: f64 = (t1a).sqrt();l.f6fc = t1b;}
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f322 == 0.0)) && (l.f324 == 0.0)) {let t1c: f64 = (l.f775 - l.f750);let t1d: f64 = (t1c * l.f777);let t1e: f64 = (t1d).powf(l.f62f);l.f6fc = t1e;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f322 == 0.0)) {let t1f: f64 = (l.f775 - l.f750);let t20: f64 = (t1f * l.f7dc);let t21: f64 = (t20 / l.f6fc);let t22: f64 = (l.f613 * t21);l.fb6 = t22;}
        let t23: f64 = (-l.fa3);let t24: f64 = (t23 / l.fb6);let t25: f64 = (t24).abs();let t26: f64 = if t25 < 230.25850929940458 { 1.0 } else { 0.0 };l.f326 = t26;
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f322 == 0.0)) && (l.f326 != 0.0)) {let t27: f64 = (-l.fa3);let t28: f64 = (t27 / l.fb6);let t29: f64 = (t28).exp();l.f6fc = t29;}
        let t2a: f64 = (-l.fa3);let t2b: f64 = (t2a / l.fb6);let t2c: f64 = (-230.25850929940458);let t2d: f64 = if t2b < t2c { 1.0 } else { 0.0 };l.f328 = t2d;
        if (((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f322 == 0.0)) && (l.f326 == 0.0)) && (l.f328 != 0.0)) {let t2e: f64 = (-230.25850929940458);let t2f: f64 = (-l.fa3);let t30: f64 = (t2f / l.fb6);let t31: f64 = (t2e - t30);let t32: f64 = (-230.25850929940458);let t33: f64 = (-l.fa3);let t34: f64 = (t33 / l.fb6);let t35: f64 = (t32 - t34);let t36: f64 = (-230.25850929940458);let t37: f64 = (-l.fa3);let t38: f64 = (t37 / l.fb6);let t39: f64 = (t36 - t38);let t3a: f64 = (t39 * 0.3333333333333333);let t3b: f64 = (1.0 + t3a);let t3c: f64 = (t35 * t3b);let t3d: f64 = (0.5 * t3c);let t3e: f64 = (1.0 + t3d);let t3f: f64 = (t31 * t3e);let t40: f64 = (1.0 + t3f);let t41: f64 = (1e-100 / t40);l.f6fc = t41;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f322 == 0.0)) && (l.f326 == 0.0)) && (l.f328 == 0.0)) {let t42: f64 = (-l.fa3);let t43: f64 = (t42 / l.fb6);let t44: f64 = (t43 - 230.25850929940458);let t45: f64 = (-l.fa3);let t46: f64 = (t45 / l.fb6);let t47: f64 = (t46 - 230.25850929940458);let t48: f64 = (-l.fa3);let t49: f64 = (t48 / l.fb6);let t4a: f64 = (t49 - 230.25850929940458);let t4b: f64 = (t4a * 0.3333333333333333);let t4c: f64 = (1.0 + t4b);let t4d: f64 = (t47 * t4c);let t4e: f64 = (0.5 * t4d);let t4f: f64 = (1.0 + t4e);let t50: f64 = (t44 * t4f);let t51: f64 = (1.0 + t50);let t52: f64 = (1e100 * t51);l.f6fc = t52;}
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f322 == 0.0)) {let t53: f64 = (l.f73d * l.fb6);let t54: f64 = (t53 * l.fb6);let t55: f64 = (t54 * l.f6fc);let t56: f64 = (l.f26 * t55);l.f529 = t56;}
        let t57: f64 = if ((l.f785 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f32a = t57;
        if (((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f32a != 0.0)) {l.fae = 1.0;}
        let t58: f64 = (-l.f2);let t59: f64 = (t58 * l.f785);let t5a: f64 = if l.f74a > t59 { 1.0 } else { 0.0 };l.f32c = t5a;let t5b: f64 = if l.f627 == 4.0 { 1.0 } else { 0.0 };l.f32e = t5b;
        if (((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f32a == 0.0)) && (l.f32c != 0.0)) && (l.f32e != 0.0)) {let t5c: f64 = (l.f74a * l.f789);let t5d: f64 = (t5c).abs();let t5e: f64 = (l.f74a * l.f789);let t5f: f64 = (t5e).abs();let t60: f64 = (t5d * t5f);let t61: f64 = (l.f74a * l.f789);let t62: f64 = (t61).abs();let t63: f64 = (t60 * t62);let t64: f64 = (l.f74a * l.f789);let t65: f64 = (t64).abs();let t66: f64 = (t63 * t65);l.f6fc = t66;}
        if (((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f32a == 0.0)) && (l.f32c != 0.0)) && (l.f32e == 0.0)) {let t67: f64 = (l.f74a * l.f789);let t68: f64 = (t67).abs();let t69: f64 = (t68).powf(l.f627);l.f6fc = t69;}
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f32a == 0.0)) && (l.f32c != 0.0)) {let t6a: f64 = (1.0 - l.f6fc);let t6b: f64 = (1.0 / t6a);l.fae = t6b;}
        if ((((l.f29a != 0.0) && (l.f30c == 0.0)) && (l.f32a == 0.0)) && (l.f32c == 0.0)) {let t6c: f64 = (l.f2 * l.f785);let t6d: f64 = (l.f74a + t6c);let t6e: f64 = (t6d * l.f6bc);let t6f: f64 = (l.fc5 + t6e);l.fae = t6f;}
        if ((l.f29a != 0.0) && (l.f30c == 0.0)) {let t70: f64 = (l.f52f + l.f593);let t71: f64 = (t70 + l.f599);let t72: f64 = (t71 + l.f529);let t73: f64 = (t72 * l.fae);(l.f56e, l.f56f, l.f570, ) = (t73, (l.f530 * l.fae), (l.f531 * l.fae), );let t74: f64 = (l.f593 + l.f599);let t75: f64 = (t74 + l.f529);let t76: f64 = (t75 * l.fae);(l.f556, l.f557, l.f558, ) = (t76, 0.0, 0.0, );}
        if (l.f29a != 0.0) {let t77: f64 = (l.f0 * l.f562);let t78: f64 = (l.f5b1 * l.f576);let t79: f64 = (t77 + t78);let t7a: f64 = (l.f5af * l.f56e);let t7b: f64 = (t79 + t7a);(l.f518, l.f51d, l.f51e, ) = (t7b, (((l.f0 * l.f563) + (l.f5b1 * l.f577)) + (l.f5af * l.f56f)), (((l.f0 * l.f564) + (l.f5b1 * l.f578)) + (l.f5af * l.f570)), );}
        let t7c: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f330 = t7c;
        if ((l.f29a != 0.0) && (l.f330 != 0.0)) {let t7d: f64 = (4.0 * l.f78f);let t7e: f64 = (t7d * l.f78f);l.f4e1 = t7e;let t7f: f64 = (l.f78f / l.f791);l.f4e5 = t7f;let t80: f64 = (l.f78f * l.f4e5);let t81: f64 = (l.f73f + t80);l.f4e9 = t81;let t82: f64 = (l.f791 + l.f4e9);l.f4ef = t82;let t83: f64 = (l.f791 - l.f4e9);l.f4f5 = t83;let t84: f64 = (l.f4f5 * l.f4f5);let t85: f64 = (t84 + l.f4e1);let t86: f64 = (t85).sqrt();l.f4fb = t86;let t87: f64 = (l.f73f * l.f791);let t88: f64 = (l.f4ef + l.f4fb);let t89: f64 = (t87 / t88);let t8a: f64 = (2.0 * t89);l.f796 = t8a;}
        let t8b: f64 = if l.f73f < l.f7b1 { 1.0 } else { 0.0 };l.f332 = t8b;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t8c: f64 = (l.f73f * l.f645);let t8d: f64 = (0.5 * t8c);let t8e: f64 = (t8d).abs();let t8f: f64 = if t8e < 230.25850929940458 { 1.0 } else { 0.0 };l.f334 = t8f;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f334 != 0.0)) {let t90: f64 = (l.f73f * l.f645);let t91: f64 = (0.5 * t90);let t92: f64 = (t91).exp();l.f825 = t92;}
        let t93: f64 = (l.f73f * l.f645);let t94: f64 = (0.5 * t93);let t95: f64 = (-230.25850929940458);let t96: f64 = if t94 < t95 { 1.0 } else { 0.0 };l.f336 = t96;
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f334 == 0.0)) && (l.f336 != 0.0)) {let t97: f64 = (-230.25850929940458);let t98: f64 = (l.f73f * l.f645);let t99: f64 = (0.5 * t98);let t9a: f64 = (t97 - t99);let t9b: f64 = (-230.25850929940458);let t9c: f64 = (l.f73f * l.f645);let t9d: f64 = (0.5 * t9c);let t9e: f64 = (t9b - t9d);let t9f: f64 = (-230.25850929940458);let ta0: f64 = (l.f73f * l.f645);let ta1: f64 = (0.5 * ta0);let ta2: f64 = (t9f - ta1);let ta3: f64 = (ta2 * 0.3333333333333333);let ta4: f64 = (1.0 + ta3);let ta5: f64 = (t9e * ta4);let ta6: f64 = (0.5 * ta5);let ta7: f64 = (1.0 + ta6);let ta8: f64 = (t9a * ta7);let ta9: f64 = (1.0 + ta8);let taa: f64 = (1e-100 / ta9);l.f825 = taa;}
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f334 == 0.0)) && (l.f336 == 0.0)) {let tab: f64 = (l.f73f * l.f645);let tac: f64 = (0.5 * tab);let tad: f64 = (tac - 230.25850929940458);let tae: f64 = (l.f73f * l.f645);let taf: f64 = (0.5 * tae);let tb0: f64 = (taf - 230.25850929940458);let tb1: f64 = (l.f73f * l.f645);let tb2: f64 = (0.5 * tb1);let tb3: f64 = (tb2 - 230.25850929940458);let tb4: f64 = (tb3 * 0.3333333333333333);let tb5: f64 = (1.0 + tb4);let tb6: f64 = (tb0 * tb5);let tb7: f64 = (0.5 * tb6);let tb8: f64 = (1.0 + tb7);let tb9: f64 = (tad * tb8);let tba: f64 = (1.0 + tb9);let tbb: f64 = (1e100 * tba);l.f825 = tbb;}
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) {let tbc: f64 = (l.f5eb * l.f5eb);let tbd: f64 = (tbc / l.f5df);l.f64f = tbd;let tbe: f64 = (l.f5e5 / l.f645);let tbf: f64 = (l.f5df / l.f64f);let tc0: f64 = (tbf).ln();let tc1: f64 = (tbe * tc0);l.f793 = tc1;}
        let tc2: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f338 = tc2;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {let tc3: f64 = (l.f73f - l.f793);let tc4: f64 = (p.p86 * tc3);let tc5: f64 = (tc4 + l.f5e5);(l.f601, l.f602, l.f603, ) = (tc5, 0.0, 0.0, );let tc6: f64 = (p.p86 * l.f793);let tc7: f64 = (l.f5e5 - tc6);(l.f5ed, l.f5ee, l.f5ef, ) = (tc7, 0.0, 0.0, );let tc8: f64 = (p.p85 - l.f601);let tc9: f64 = (tc8 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tc9, (-l.f602), (-l.f603), );let tca: f64 = (4.0 * p.p85);let tcb: f64 = (tca * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tcb, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {
            let (tcd, tce, tcf,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tcc: f64 = (-l.f6f7);
        (tcc, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tcd, tce, tcf, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {let td0: f64 = (l.f6f3 * l.f6f3);let td1: f64 = (td0 + l.f6f7);let td2: f64 = (td1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (td2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * td2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * td2)), );let td3: f64 = (l.f6f3 + l.f6f7);let td4: f64 = (0.5 * td3);let td5: f64 = (p.p85 - td4);(l.f605, l.f606, l.f607, ) = (td5, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let td6: f64 = (l.f605 - l.f5e5);let td7: f64 = (td6 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (td7, l.f606, l.f607, );let td8: f64 = (4.0 * l.f5e5);let td9: f64 = (td8 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (td9, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {
            let (tdb, tdc, tdd,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tda: f64 = (-l.f6f7);
        (tda, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tdb, tdc, tdd, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {let tde: f64 = (l.f6f3 * l.f6f3);let tdf: f64 = (tde + l.f6f7);let te0: f64 = (tdf).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (te0, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * te0)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * te0)), );let te1: f64 = (l.f6f3 + l.f6f7);let te2: f64 = (0.5 * te1);let te3: f64 = (l.f5e5 + te2);(l.f5f1, l.f5f2, l.f5f3, ) = (te3, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let te4: f64 = (p.p85 - l.f5ed);let te5: f64 = (te4 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (te5, (-l.f5ee), (-l.f5ef), );let te6: f64 = (4.0 * p.p85);let te7: f64 = (te6 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (te7, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {
            let (te9, tea, teb,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let te8: f64 = (-l.f6f7);
        (te8, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (te9, tea, teb, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {let tec: f64 = (l.f6f3 * l.f6f3);let ted: f64 = (tec + l.f6f7);let tee: f64 = (ted).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tee, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tee)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tee)), );let tef: f64 = (l.f6f3 + l.f6f7);let tf0: f64 = (0.5 * tef);let tf1: f64 = (p.p85 - tf0);(l.f5ed, l.f5ee, l.f5ef, ) = (tf1, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let tf2: f64 = (l.f5ed - l.f5e5);let tf3: f64 = (tf2 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tf3, l.f5ee, l.f5ef, );let tf4: f64 = (4.0 * l.f5e5);let tf5: f64 = (tf4 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tf5, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {
            let (tf7, tf8, tf9,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tf6: f64 = (-l.f6f7);
        (tf6, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tf7, tf8, tf9, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 != 0.0)) {let tfa: f64 = (l.f6f3 * l.f6f3);let tfb: f64 = (tfa + l.f6f7);let tfc: f64 = (tfb).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tfc, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tfc)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tfc)), );let tfd: f64 = (l.f6f3 + l.f6f7);let tfe: f64 = (0.5 * tfd);let tff: f64 = (l.f5e5 + tfe);(l.f5ed, l.f5ee, l.f5ef, ) = (tff, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f338 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );}
        let t100: f64 = (l.f73f / l.f5f1);let t101: f64 = (l.f5f1 - l.f5ed);let t102: f64 = (l.f793 * t101);let t103: f64 = (l.f5ed * p.p85);let t104: f64 = (t102 / t103);let t105: f64 = (t100 + t104);let t106: f64 = (l.f645 * t105);let t107: f64 = (t106).abs();let t108: f64 = if t107 < 230.25850929940458 { 1.0 } else { 0.0 };l.f33a = t108;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33a != 0.0)) {let t109: f64 = (l.f73f / l.f5f1);let t10a: f64 = (l.f5f1 - l.f5ed);let t10b: f64 = (l.f793 * t10a);let t10c: f64 = (l.f5ed * p.p85);let t10d: f64 = (t10b / t10c);let t10e: f64 = (t109 + t10d);let t10f: f64 = (l.f645 * t10e);let t110: f64 = (t10f).exp();(l.f536, l.f537, l.f538, ) = (t110, (t110 * (l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t10c) - (t10b * (l.f5ee * p.p85))) / (t10c * t10c))))), (t110 * (l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t10c) - (t10b * (l.f5ef * p.p85))) / (t10c * t10c))))), );}
        let t111: f64 = (l.f73f / l.f5f1);let t112: f64 = (l.f5f1 - l.f5ed);let t113: f64 = (l.f793 * t112);let t114: f64 = (l.f5ed * p.p85);let t115: f64 = (t113 / t114);let t116: f64 = (t111 + t115);let t117: f64 = (l.f645 * t116);let t118: f64 = (-230.25850929940458);let t119: f64 = if t117 < t118 { 1.0 } else { 0.0 };l.f33c = t119;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33a == 0.0)) && (l.f33c != 0.0)) {let t11a: f64 = (-230.25850929940458);let t11b: f64 = (l.f73f / l.f5f1);let t11c: f64 = (l.f5f1 - l.f5ed);let t11d: f64 = (l.f793 * t11c);let t11e: f64 = (l.f5ed * p.p85);let t11f: f64 = (t11d / t11e);let t120: f64 = (t11b + t11f);let t121: f64 = (l.f645 * t120);let t122: f64 = (t11a - t121);let t123: f64 = (-230.25850929940458);let t124: f64 = (l.f73f / l.f5f1);let t125: f64 = (l.f5f1 - l.f5ed);let t126: f64 = (l.f793 * t125);let t127: f64 = (l.f5ed * p.p85);let t128: f64 = (t126 / t127);let t129: f64 = (t124 + t128);let t12a: f64 = (l.f645 * t129);let t12b: f64 = (t123 - t12a);let t12c: f64 = (-230.25850929940458);let t12d: f64 = (l.f73f / l.f5f1);let t12e: f64 = (l.f5f1 - l.f5ed);let t12f: f64 = (l.f793 * t12e);let t130: f64 = (l.f5ed * p.p85);let t131: f64 = (t12f / t130);let t132: f64 = (t12d + t131);let t133: f64 = (l.f645 * t132);let t134: f64 = (t12c - t133);let t135: f64 = (t134 * 0.3333333333333333);let t136: f64 = (1.0 + t135);let t137: f64 = (t12b * t136);let t138: f64 = (0.5 * t137);let t139: f64 = (1.0 + t138);let t13a: f64 = (t122 * t139);let t13b: f64 = (1.0 + t13a);let t13c: f64 = (1e-100 / t13b);(l.f536, l.f537, l.f538, ) = (t13c, (-((1e-100 * (((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t11e) - (t11d * (l.f5ee * p.p85))) / (t11e * t11e))))) * t139) + (t122 * (0.5 * (((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t127) - (t126 * (l.f5ee * p.p85))) / (t127 * t127))))) * t136) + (t12b * ((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t130) - (t12f * (l.f5ee * p.p85))) / (t130 * t130))))) * 0.3333333333333333))))))) / (t13b * t13b))), (-((1e-100 * (((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t11e) - (t11d * (l.f5ef * p.p85))) / (t11e * t11e))))) * t139) + (t122 * (0.5 * (((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t127) - (t126 * (l.f5ef * p.p85))) / (t127 * t127))))) * t136) + (t12b * ((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t130) - (t12f * (l.f5ef * p.p85))) / (t130 * t130))))) * 0.3333333333333333))))))) / (t13b * t13b))), );}
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33a == 0.0)) && (l.f33c == 0.0)) {let t13d: f64 = (l.f73f / l.f5f1);let t13e: f64 = (l.f5f1 - l.f5ed);let t13f: f64 = (l.f793 * t13e);let t140: f64 = (l.f5ed * p.p85);let t141: f64 = (t13f / t140);let t142: f64 = (t13d + t141);let t143: f64 = (l.f645 * t142);let t144: f64 = (t143 - 230.25850929940458);let t145: f64 = (l.f73f / l.f5f1);let t146: f64 = (l.f5f1 - l.f5ed);let t147: f64 = (l.f793 * t146);let t148: f64 = (l.f5ed * p.p85);let t149: f64 = (t147 / t148);let t14a: f64 = (t145 + t149);let t14b: f64 = (l.f645 * t14a);let t14c: f64 = (t14b - 230.25850929940458);let t14d: f64 = (l.f73f / l.f5f1);let t14e: f64 = (l.f5f1 - l.f5ed);let t14f: f64 = (l.f793 * t14e);let t150: f64 = (l.f5ed * p.p85);let t151: f64 = (t14f / t150);let t152: f64 = (t14d + t151);let t153: f64 = (l.f645 * t152);let t154: f64 = (t153 - 230.25850929940458);let t155: f64 = (t154 * 0.3333333333333333);let t156: f64 = (1.0 + t155);let t157: f64 = (t14c * t156);let t158: f64 = (0.5 * t157);let t159: f64 = (1.0 + t158);let t15a: f64 = (t144 * t159);let t15b: f64 = (1.0 + t15a);let t15c: f64 = (1e100 * t15b);(l.f536, l.f537, l.f538, ) = (t15c, (1e100 * (((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t140) - (t13f * (l.f5ee * p.p85))) / (t140 * t140)))) * t159) + (t144 * (0.5 * (((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t148) - (t147 * (l.f5ee * p.p85))) / (t148 * t148)))) * t156) + (t14c * ((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t150) - (t14f * (l.f5ee * p.p85))) / (t150 * t150)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t140) - (t13f * (l.f5ef * p.p85))) / (t140 * t140)))) * t159) + (t144 * (0.5 * (((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t148) - (t147 * (l.f5ef * p.p85))) / (t148 * t148)))) * t156) + (t14c * ((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t150) - (t14f * (l.f5ef * p.p85))) / (t150 * t150)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) {let t15d: f64 = (l.f5eb * l.f5eb);let t15e: f64 = (t15d / l.f5e3);l.f64f = t15e;let t15f: f64 = (l.f5e9 / l.f645);let t160: f64 = (l.f5e3 / l.f64f);let t161: f64 = (t160).ln();let t162: f64 = (t15f * t161);l.f793 = t162;}
        let t163: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f33e = t163;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {let t164: f64 = (l.f73f - l.f793);let t165: f64 = (p.p86 * t164);let t166: f64 = (t165 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t166, 0.0, 0.0, );let t167: f64 = (p.p86 * l.f793);let t168: f64 = (l.f5e9 - t167);(l.f5ed, l.f5ee, l.f5ef, ) = (t168, 0.0, 0.0, );let t169: f64 = (p.p85 - l.f601);let t16a: f64 = (t169 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t16a, (-l.f602), (-l.f603), );let t16b: f64 = (4.0 * p.p85);let t16c: f64 = (t16b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t16c, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {
            let (t16e, t16f, t170,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t16d: f64 = (-l.f6f7);
        (t16d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t16e, t16f, t170, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {let t171: f64 = (l.f6f3 * l.f6f3);let t172: f64 = (t171 + l.f6f7);let t173: f64 = (t172).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t173, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t173)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t173)), );let t174: f64 = (l.f6f3 + l.f6f7);let t175: f64 = (0.5 * t174);let t176: f64 = (p.p85 - t175);(l.f605, l.f606, l.f607, ) = (t176, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t177: f64 = (l.f605 - l.f5e9);let t178: f64 = (t177 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t178, l.f606, l.f607, );let t179: f64 = (4.0 * l.f5e9);let t17a: f64 = (t179 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t17a, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {
            let (t17c, t17d, t17e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t17b: f64 = (-l.f6f7);
        (t17b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t17c, t17d, t17e, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {let t17f: f64 = (l.f6f3 * l.f6f3);let t180: f64 = (t17f + l.f6f7);let t181: f64 = (t180).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t181, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t181)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t181)), );let t182: f64 = (l.f6f3 + l.f6f7);let t183: f64 = (0.5 * t182);let t184: f64 = (l.f5e9 + t183);(l.f5f1, l.f5f2, l.f5f3, ) = (t184, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t185: f64 = (p.p85 - l.f5ed);let t186: f64 = (t185 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t186, (-l.f5ee), (-l.f5ef), );let t187: f64 = (4.0 * p.p85);let t188: f64 = (t187 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t188, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {
            let (t18a, t18b, t18c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t189: f64 = (-l.f6f7);
        (t189, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t18a, t18b, t18c, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {let t18d: f64 = (l.f6f3 * l.f6f3);let t18e: f64 = (t18d + l.f6f7);let t18f: f64 = (t18e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t18f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t18f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t18f)), );let t190: f64 = (l.f6f3 + l.f6f7);let t191: f64 = (0.5 * t190);let t192: f64 = (p.p85 - t191);(l.f5ed, l.f5ee, l.f5ef, ) = (t192, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t193: f64 = (l.f5ed - l.f5e9);let t194: f64 = (t193 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t194, l.f5ee, l.f5ef, );let t195: f64 = (4.0 * l.f5e9);let t196: f64 = (t195 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t196, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {
            let (t198, t199, t19a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t197: f64 = (-l.f6f7);
        (t197, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t198, t199, t19a, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e != 0.0)) {let t19b: f64 = (l.f6f3 * l.f6f3);let t19c: f64 = (t19b + l.f6f7);let t19d: f64 = (t19c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t19d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t19d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t19d)), );let t19e: f64 = (l.f6f3 + l.f6f7);let t19f: f64 = (0.5 * t19e);let t1a0: f64 = (l.f5e9 + t19f);(l.f5ed, l.f5ee, l.f5ef, ) = (t1a0, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f33e == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );}
        let t1a1: f64 = (l.f73f / l.f5f1);let t1a2: f64 = (l.f5f1 - l.f5ed);let t1a3: f64 = (l.f793 * t1a2);let t1a4: f64 = (l.f5ed * p.p85);let t1a5: f64 = (t1a3 / t1a4);let t1a6: f64 = (t1a1 + t1a5);let t1a7: f64 = (l.f645 * t1a6);let t1a8: f64 = (t1a7).abs();let t1a9: f64 = if t1a8 < 230.25850929940458 { 1.0 } else { 0.0 };l.f340 = t1a9;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f340 != 0.0)) {let t1aa: f64 = (l.f73f / l.f5f1);let t1ab: f64 = (l.f5f1 - l.f5ed);let t1ac: f64 = (l.f793 * t1ab);let t1ad: f64 = (l.f5ed * p.p85);let t1ae: f64 = (t1ac / t1ad);let t1af: f64 = (t1aa + t1ae);let t1b0: f64 = (l.f645 * t1af);let t1b1: f64 = (t1b0).exp();(l.f53e, l.f53f, l.f540, ) = (t1b1, (t1b1 * (l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1ad) - (t1ac * (l.f5ee * p.p85))) / (t1ad * t1ad))))), (t1b1 * (l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1ad) - (t1ac * (l.f5ef * p.p85))) / (t1ad * t1ad))))), );}
        let t1b2: f64 = (l.f73f / l.f5f1);let t1b3: f64 = (l.f5f1 - l.f5ed);let t1b4: f64 = (l.f793 * t1b3);let t1b5: f64 = (l.f5ed * p.p85);let t1b6: f64 = (t1b4 / t1b5);let t1b7: f64 = (t1b2 + t1b6);let t1b8: f64 = (l.f645 * t1b7);let t1b9: f64 = (-230.25850929940458);let t1ba: f64 = if t1b8 < t1b9 { 1.0 } else { 0.0 };l.f342 = t1ba;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f340 == 0.0)) && (l.f342 != 0.0)) {let t1bb: f64 = (-230.25850929940458);let t1bc: f64 = (l.f73f / l.f5f1);let t1bd: f64 = (l.f5f1 - l.f5ed);let t1be: f64 = (l.f793 * t1bd);let t1bf: f64 = (l.f5ed * p.p85);let t1c0: f64 = (t1be / t1bf);let t1c1: f64 = (t1bc + t1c0);let t1c2: f64 = (l.f645 * t1c1);let t1c3: f64 = (t1bb - t1c2);let t1c4: f64 = (-230.25850929940458);let t1c5: f64 = (l.f73f / l.f5f1);let t1c6: f64 = (l.f5f1 - l.f5ed);let t1c7: f64 = (l.f793 * t1c6);let t1c8: f64 = (l.f5ed * p.p85);let t1c9: f64 = (t1c7 / t1c8);let t1ca: f64 = (t1c5 + t1c9);let t1cb: f64 = (l.f645 * t1ca);let t1cc: f64 = (t1c4 - t1cb);let t1cd: f64 = (-230.25850929940458);let t1ce: f64 = (l.f73f / l.f5f1);let t1cf: f64 = (l.f5f1 - l.f5ed);let t1d0: f64 = (l.f793 * t1cf);let t1d1: f64 = (l.f5ed * p.p85);let t1d2: f64 = (t1d0 / t1d1);let t1d3: f64 = (t1ce + t1d2);let t1d4: f64 = (l.f645 * t1d3);let t1d5: f64 = (t1cd - t1d4);let t1d6: f64 = (t1d5 * 0.3333333333333333);let t1d7: f64 = (1.0 + t1d6);let t1d8: f64 = (t1cc * t1d7);let t1d9: f64 = (0.5 * t1d8);let t1da: f64 = (1.0 + t1d9);let t1db: f64 = (t1c3 * t1da);let t1dc: f64 = (1.0 + t1db);let t1dd: f64 = (1e-100 / t1dc);(l.f53e, l.f53f, l.f540, ) = (t1dd, (-((1e-100 * (((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1bf) - (t1be * (l.f5ee * p.p85))) / (t1bf * t1bf))))) * t1da) + (t1c3 * (0.5 * (((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1c8) - (t1c7 * (l.f5ee * p.p85))) / (t1c8 * t1c8))))) * t1d7) + (t1cc * ((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1d1) - (t1d0 * (l.f5ee * p.p85))) / (t1d1 * t1d1))))) * 0.3333333333333333))))))) / (t1dc * t1dc))), (-((1e-100 * (((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1bf) - (t1be * (l.f5ef * p.p85))) / (t1bf * t1bf))))) * t1da) + (t1c3 * (0.5 * (((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1c8) - (t1c7 * (l.f5ef * p.p85))) / (t1c8 * t1c8))))) * t1d7) + (t1cc * ((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1d1) - (t1d0 * (l.f5ef * p.p85))) / (t1d1 * t1d1))))) * 0.3333333333333333))))))) / (t1dc * t1dc))), );}
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f340 == 0.0)) && (l.f342 == 0.0)) {let t1de: f64 = (l.f73f / l.f5f1);let t1df: f64 = (l.f5f1 - l.f5ed);let t1e0: f64 = (l.f793 * t1df);let t1e1: f64 = (l.f5ed * p.p85);let t1e2: f64 = (t1e0 / t1e1);let t1e3: f64 = (t1de + t1e2);let t1e4: f64 = (l.f645 * t1e3);let t1e5: f64 = (t1e4 - 230.25850929940458);let t1e6: f64 = (l.f73f / l.f5f1);let t1e7: f64 = (l.f5f1 - l.f5ed);let t1e8: f64 = (l.f793 * t1e7);let t1e9: f64 = (l.f5ed * p.p85);let t1ea: f64 = (t1e8 / t1e9);let t1eb: f64 = (t1e6 + t1ea);let t1ec: f64 = (l.f645 * t1eb);let t1ed: f64 = (t1ec - 230.25850929940458);let t1ee: f64 = (l.f73f / l.f5f1);let t1ef: f64 = (l.f5f1 - l.f5ed);let t1f0: f64 = (l.f793 * t1ef);let t1f1: f64 = (l.f5ed * p.p85);let t1f2: f64 = (t1f0 / t1f1);let t1f3: f64 = (t1ee + t1f2);let t1f4: f64 = (l.f645 * t1f3);let t1f5: f64 = (t1f4 - 230.25850929940458);let t1f6: f64 = (t1f5 * 0.3333333333333333);let t1f7: f64 = (1.0 + t1f6);let t1f8: f64 = (t1ed * t1f7);let t1f9: f64 = (0.5 * t1f8);let t1fa: f64 = (1.0 + t1f9);let t1fb: f64 = (t1e5 * t1fa);let t1fc: f64 = (1.0 + t1fb);let t1fd: f64 = (1e100 * t1fc);(l.f53e, l.f53f, l.f540, ) = (t1fd, (1e100 * (((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e1) - (t1e0 * (l.f5ee * p.p85))) / (t1e1 * t1e1)))) * t1fa) + (t1e5 * (0.5 * (((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e9) - (t1e8 * (l.f5ee * p.p85))) / (t1e9 * t1e9)))) * t1f7) + (t1ed * ((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1f1) - (t1f0 * (l.f5ee * p.p85))) / (t1f1 * t1f1)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e1) - (t1e0 * (l.f5ef * p.p85))) / (t1e1 * t1e1)))) * t1fa) + (t1e5 * (0.5 * (((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e9) - (t1e8 * (l.f5ef * p.p85))) / (t1e9 * t1e9)))) * t1f7) + (t1ed * ((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1f1) - (t1f0 * (l.f5ef * p.p85))) / (t1f1 * t1f1)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) {let t1fe: f64 = (l.f5eb * l.f5eb);let t1ff: f64 = (t1fe / l.f5e1);l.f64f = t1ff;let t200: f64 = (l.f5e7 / l.f645);let t201: f64 = (l.f5e1 / l.f64f);let t202: f64 = (t201).ln();let t203: f64 = (t200 * t202);l.f793 = t203;}
        let t204: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f344 = t204;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {let t205: f64 = (l.f73f - l.f793);let t206: f64 = (p.p86 * t205);let t207: f64 = (t206 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t207, 0.0, 0.0, );let t208: f64 = (p.p86 * l.f793);let t209: f64 = (l.f5e7 - t208);(l.f5ed, l.f5ee, l.f5ef, ) = (t209, 0.0, 0.0, );let t20a: f64 = (p.p85 - l.f601);let t20b: f64 = (t20a - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t20b, (-l.f602), (-l.f603), );let t20c: f64 = (4.0 * p.p85);let t20d: f64 = (t20c * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t20d, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {
            let (t20f, t210, t211,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t20e: f64 = (-l.f6f7);
        (t20e, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t20f, t210, t211, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {let t212: f64 = (l.f6f3 * l.f6f3);let t213: f64 = (t212 + l.f6f7);let t214: f64 = (t213).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t214, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t214)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t214)), );let t215: f64 = (l.f6f3 + l.f6f7);let t216: f64 = (0.5 * t215);let t217: f64 = (p.p85 - t216);(l.f605, l.f606, l.f607, ) = (t217, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t218: f64 = (l.f605 - l.f5e7);let t219: f64 = (t218 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t219, l.f606, l.f607, );let t21a: f64 = (4.0 * l.f5e7);let t21b: f64 = (t21a * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t21b, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {
            let (t21d, t21e, t21f,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t21c: f64 = (-l.f6f7);
        (t21c, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t21d, t21e, t21f, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {let t220: f64 = (l.f6f3 * l.f6f3);let t221: f64 = (t220 + l.f6f7);let t222: f64 = (t221).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t222, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t222)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t222)), );let t223: f64 = (l.f6f3 + l.f6f7);let t224: f64 = (0.5 * t223);let t225: f64 = (l.f5e7 + t224);(l.f5f1, l.f5f2, l.f5f3, ) = (t225, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t226: f64 = (p.p85 - l.f5ed);let t227: f64 = (t226 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t227, (-l.f5ee), (-l.f5ef), );let t228: f64 = (4.0 * p.p85);let t229: f64 = (t228 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t229, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {
            let (t22b, t22c, t22d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t22a: f64 = (-l.f6f7);
        (t22a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t22b, t22c, t22d, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {let t22e: f64 = (l.f6f3 * l.f6f3);let t22f: f64 = (t22e + l.f6f7);let t230: f64 = (t22f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t230, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t230)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t230)), );let t231: f64 = (l.f6f3 + l.f6f7);let t232: f64 = (0.5 * t231);let t233: f64 = (p.p85 - t232);(l.f5ed, l.f5ee, l.f5ef, ) = (t233, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t234: f64 = (l.f5ed - l.f5e7);let t235: f64 = (t234 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t235, l.f5ee, l.f5ef, );let t236: f64 = (4.0 * l.f5e7);let t237: f64 = (t236 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t237, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {
            let (t239, t23a, t23b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t238: f64 = (-l.f6f7);
        (t238, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t239, t23a, t23b, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 != 0.0)) {let t23c: f64 = (l.f6f3 * l.f6f3);let t23d: f64 = (t23c + l.f6f7);let t23e: f64 = (t23d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t23e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t23e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t23e)), );let t23f: f64 = (l.f6f3 + l.f6f7);let t240: f64 = (0.5 * t23f);let t241: f64 = (l.f5e7 + t240);(l.f5ed, l.f5ee, l.f5ef, ) = (t241, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f344 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );}
        let t242: f64 = (l.f73f / l.f5f1);let t243: f64 = (l.f5f1 - l.f5ed);let t244: f64 = (l.f793 * t243);let t245: f64 = (l.f5ed * p.p85);let t246: f64 = (t244 / t245);let t247: f64 = (t242 + t246);let t248: f64 = (l.f645 * t247);let t249: f64 = (t248).abs();let t24a: f64 = if t249 < 230.25850929940458 { 1.0 } else { 0.0 };l.f346 = t24a;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f346 != 0.0)) {let t24b: f64 = (l.f73f / l.f5f1);let t24c: f64 = (l.f5f1 - l.f5ed);let t24d: f64 = (l.f793 * t24c);let t24e: f64 = (l.f5ed * p.p85);let t24f: f64 = (t24d / t24e);let t250: f64 = (t24b + t24f);let t251: f64 = (l.f645 * t250);let t252: f64 = (t251).exp();(l.f53a, l.f53b, l.f53c, ) = (t252, (t252 * (l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t24e) - (t24d * (l.f5ee * p.p85))) / (t24e * t24e))))), (t252 * (l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t24e) - (t24d * (l.f5ef * p.p85))) / (t24e * t24e))))), );}
        let t253: f64 = (l.f73f / l.f5f1);let t254: f64 = (l.f5f1 - l.f5ed);let t255: f64 = (l.f793 * t254);let t256: f64 = (l.f5ed * p.p85);let t257: f64 = (t255 / t256);let t258: f64 = (t253 + t257);let t259: f64 = (l.f645 * t258);let t25a: f64 = (-230.25850929940458);let t25b: f64 = if t259 < t25a { 1.0 } else { 0.0 };l.f348 = t25b;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f346 == 0.0)) && (l.f348 != 0.0)) {let t25c: f64 = (-230.25850929940458);let t25d: f64 = (l.f73f / l.f5f1);let t25e: f64 = (l.f5f1 - l.f5ed);let t25f: f64 = (l.f793 * t25e);let t260: f64 = (l.f5ed * p.p85);let t261: f64 = (t25f / t260);let t262: f64 = (t25d + t261);let t263: f64 = (l.f645 * t262);let t264: f64 = (t25c - t263);let t265: f64 = (-230.25850929940458);let t266: f64 = (l.f73f / l.f5f1);let t267: f64 = (l.f5f1 - l.f5ed);let t268: f64 = (l.f793 * t267);let t269: f64 = (l.f5ed * p.p85);let t26a: f64 = (t268 / t269);let t26b: f64 = (t266 + t26a);let t26c: f64 = (l.f645 * t26b);let t26d: f64 = (t265 - t26c);let t26e: f64 = (-230.25850929940458);let t26f: f64 = (l.f73f / l.f5f1);let t270: f64 = (l.f5f1 - l.f5ed);let t271: f64 = (l.f793 * t270);let t272: f64 = (l.f5ed * p.p85);let t273: f64 = (t271 / t272);let t274: f64 = (t26f + t273);let t275: f64 = (l.f645 * t274);let t276: f64 = (t26e - t275);let t277: f64 = (t276 * 0.3333333333333333);let t278: f64 = (1.0 + t277);let t279: f64 = (t26d * t278);let t27a: f64 = (0.5 * t279);let t27b: f64 = (1.0 + t27a);let t27c: f64 = (t264 * t27b);let t27d: f64 = (1.0 + t27c);let t27e: f64 = (1e-100 / t27d);(l.f53a, l.f53b, l.f53c, ) = (t27e, (-((1e-100 * (((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t260) - (t25f * (l.f5ee * p.p85))) / (t260 * t260))))) * t27b) + (t264 * (0.5 * (((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t269) - (t268 * (l.f5ee * p.p85))) / (t269 * t269))))) * t278) + (t26d * ((-(l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t272) - (t271 * (l.f5ee * p.p85))) / (t272 * t272))))) * 0.3333333333333333))))))) / (t27d * t27d))), (-((1e-100 * (((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t260) - (t25f * (l.f5ef * p.p85))) / (t260 * t260))))) * t27b) + (t264 * (0.5 * (((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t269) - (t268 * (l.f5ef * p.p85))) / (t269 * t269))))) * t278) + (t26d * ((-(l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t272) - (t271 * (l.f5ef * p.p85))) / (t272 * t272))))) * 0.3333333333333333))))))) / (t27d * t27d))), );}
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 != 0.0)) && (l.f346 == 0.0)) && (l.f348 == 0.0)) {let t27f: f64 = (l.f73f / l.f5f1);let t280: f64 = (l.f5f1 - l.f5ed);let t281: f64 = (l.f793 * t280);let t282: f64 = (l.f5ed * p.p85);let t283: f64 = (t281 / t282);let t284: f64 = (t27f + t283);let t285: f64 = (l.f645 * t284);let t286: f64 = (t285 - 230.25850929940458);let t287: f64 = (l.f73f / l.f5f1);let t288: f64 = (l.f5f1 - l.f5ed);let t289: f64 = (l.f793 * t288);let t28a: f64 = (l.f5ed * p.p85);let t28b: f64 = (t289 / t28a);let t28c: f64 = (t287 + t28b);let t28d: f64 = (l.f645 * t28c);let t28e: f64 = (t28d - 230.25850929940458);let t28f: f64 = (l.f73f / l.f5f1);let t290: f64 = (l.f5f1 - l.f5ed);let t291: f64 = (l.f793 * t290);let t292: f64 = (l.f5ed * p.p85);let t293: f64 = (t291 / t292);let t294: f64 = (t28f + t293);let t295: f64 = (l.f645 * t294);let t296: f64 = (t295 - 230.25850929940458);let t297: f64 = (t296 * 0.3333333333333333);let t298: f64 = (1.0 + t297);let t299: f64 = (t28e * t298);let t29a: f64 = (0.5 * t299);let t29b: f64 = (1.0 + t29a);let t29c: f64 = (t286 * t29b);let t29d: f64 = (1.0 + t29c);let t29e: f64 = (1e100 * t29d);(l.f53a, l.f53b, l.f53c, ) = (t29e, (1e100 * (((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t282) - (t281 * (l.f5ee * p.p85))) / (t282 * t282)))) * t29b) + (t286 * (0.5 * (((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t28a) - (t289 * (l.f5ee * p.p85))) / (t28a * t28a)))) * t298) + (t28e * ((l.f645 * ((-((l.f73f * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t292) - (t291 * (l.f5ee * p.p85))) / (t292 * t292)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t282) - (t281 * (l.f5ef * p.p85))) / (t282 * t282)))) * t29b) + (t286 * (0.5 * (((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t28a) - (t289 * (l.f5ef * p.p85))) / (t28a * t28a)))) * t298) + (t28e * ((l.f645 * ((-((l.f73f * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t292) - (t291 * (l.f5ef * p.p85))) / (t292 * t292)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) {let t29f: f64 = (l.f73f - l.f7b1);let t2a0: f64 = (t29f * l.f645);let t2a1: f64 = (1.0 + t2a0);let t2a2: f64 = (t2a1 * l.f89);let t2a3: f64 = (t2a2).sqrt();l.f825 = t2a3;let t2a4: f64 = (l.f5eb * l.f5eb);let t2a5: f64 = (t2a4 / l.f5df);l.f64f = t2a5;let t2a6: f64 = (l.f5e5 / l.f645);let t2a7: f64 = (l.f5df / l.f64f);let t2a8: f64 = (t2a7).ln();let t2a9: f64 = (t2a6 * t2a8);l.f793 = t2a9;}
        let t2aa: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f34a = t2aa;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {let t2ab: f64 = (l.f7b1 - l.f793);let t2ac: f64 = (p.p86 * t2ab);let t2ad: f64 = (t2ac + l.f5e5);(l.f601, l.f602, l.f603, ) = (t2ad, 0.0, 0.0, );let t2ae: f64 = (p.p86 * l.f793);let t2af: f64 = (l.f5e5 - t2ae);(l.f5ed, l.f5ee, l.f5ef, ) = (t2af, 0.0, 0.0, );let t2b0: f64 = (p.p85 - l.f601);let t2b1: f64 = (t2b0 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2b1, (-l.f602), (-l.f603), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {let t2b2: f64 = (4.0 * p.p85);let t2b3: f64 = (t2b2 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2b3, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {
            let (t2b5, t2b6, t2b7,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2b4: f64 = (-l.f6f7);
        (t2b4, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2b5, t2b6, t2b7, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {let t2b8: f64 = (l.f6f3 * l.f6f3);let t2b9: f64 = (t2b8 + l.f6f7);let t2ba: f64 = (t2b9).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2ba, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2ba)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2ba)), );let t2bb: f64 = (l.f6f3 / l.f6f7);let t2bc: f64 = (1.0 + t2bb);let t2bd: f64 = (0.5 * t2bc);(l.f55, l.f56, l.f57, ) = (t2bd, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t2be: f64 = (l.f6f3 + l.f6f7);let t2bf: f64 = (0.5 * t2be);let t2c0: f64 = (p.p85 - t2bf);(l.f605, l.f606, l.f607, ) = (t2c0, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2c1: f64 = (l.f605 - l.f5e5);let t2c2: f64 = (t2c1 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2c2, l.f606, l.f607, );let t2c3: f64 = (4.0 * l.f5e5);let t2c4: f64 = (t2c3 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2c4, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {
            let (t2c6, t2c7, t2c8,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2c5: f64 = (-l.f6f7);
        (t2c5, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2c6, t2c7, t2c8, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {let t2c9: f64 = (l.f6f3 * l.f6f3);let t2ca: f64 = (t2c9 + l.f6f7);let t2cb: f64 = (t2ca).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2cb, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2cb)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2cb)), );let t2cc: f64 = (l.f6f3 / l.f6f7);let t2cd: f64 = (1.0 + t2cc);let t2ce: f64 = (0.5 * t2cd);(l.f51, l.f52, l.f53, ) = (t2ce, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t2cf: f64 = (l.f6f3 + l.f6f7);let t2d0: f64 = (0.5 * t2cf);let t2d1: f64 = (l.f5e5 + t2d0);(l.f5f1, l.f5f2, l.f5f3, ) = (t2d1, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t2d2: f64 = (p.p85 - l.f5ed);let t2d3: f64 = (t2d2 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2d3, (-l.f5ee), (-l.f5ef), );let t2d4: f64 = (4.0 * p.p85);let t2d5: f64 = (t2d4 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2d5, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {
            let (t2d7, t2d8, t2d9,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2d6: f64 = (-l.f6f7);
        (t2d6, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2d7, t2d8, t2d9, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {let t2da: f64 = (l.f6f3 * l.f6f3);let t2db: f64 = (t2da + l.f6f7);let t2dc: f64 = (t2db).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2dc, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2dc)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2dc)), );let t2dd: f64 = (l.f6f3 + l.f6f7);let t2de: f64 = (0.5 * t2dd);let t2df: f64 = (p.p85 - t2de);(l.f5ed, l.f5ee, l.f5ef, ) = (t2df, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2e0: f64 = (l.f5ed - l.f5e5);let t2e1: f64 = (t2e0 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2e1, l.f5ee, l.f5ef, );let t2e2: f64 = (4.0 * l.f5e5);let t2e3: f64 = (t2e2 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2e3, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {
            let (t2e5, t2e6, t2e7,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2e4: f64 = (-l.f6f7);
        (t2e4, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2e5, t2e6, t2e7, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a != 0.0)) {let t2e8: f64 = (l.f6f3 * l.f6f3);let t2e9: f64 = (t2e8 + l.f6f7);let t2ea: f64 = (t2e9).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2ea, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2ea)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2ea)), );let t2eb: f64 = (l.f6f3 + l.f6f7);let t2ec: f64 = (0.5 * t2eb);let t2ed: f64 = (l.f5e5 + t2ec);(l.f5ed, l.f5ee, l.f5ef, ) = (t2ed, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t2ee: f64 = (p.p86 * l.f55);let t2ef: f64 = (t2ee * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t2ef, (((p.p86 * l.f56) * l.f51) + (t2ee * l.f52)), (((p.p86 * l.f57) * l.f51) + (t2ee * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34a == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t2f0: f64 = (l.f7b1 / l.f5f1);let t2f1: f64 = (l.f5f1 - l.f5ed);let t2f2: f64 = (l.f793 * t2f1);let t2f3: f64 = (l.f5ed * p.p85);let t2f4: f64 = (t2f2 / t2f3);let t2f5: f64 = (t2f0 + t2f4);let t2f6: f64 = (l.f645 * t2f5);let t2f7: f64 = (t2f6).abs();let t2f8: f64 = if t2f7 < 230.25850929940458 { 1.0 } else { 0.0 };l.f34c = t2f8;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_122(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34c != 0.0)) {let t2f9: f64 = (l.f7b1 / l.f5f1);let t2fa: f64 = (l.f5f1 - l.f5ed);let t2fb: f64 = (l.f793 * t2fa);let t2fc: f64 = (l.f5ed * p.p85);let t2fd: f64 = (t2fb / t2fc);let t2fe: f64 = (t2f9 + t2fd);let t2ff: f64 = (l.f645 * t2fe);let t300: f64 = (t2ff).exp();(l.f8a, l.f8b, l.f8c, ) = (t300, (t300 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2fc) - (t2fb * (l.f5ee * p.p85))) / (t2fc * t2fc))))), (t300 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2fc) - (t2fb * (l.f5ef * p.p85))) / (t2fc * t2fc))))), );}
        let t301: f64 = (l.f7b1 / l.f5f1);let t302: f64 = (l.f5f1 - l.f5ed);let t303: f64 = (l.f793 * t302);let t304: f64 = (l.f5ed * p.p85);let t305: f64 = (t303 / t304);let t306: f64 = (t301 + t305);let t307: f64 = (l.f645 * t306);let t308: f64 = (-230.25850929940458);let t309: f64 = if t307 < t308 { 1.0 } else { 0.0 };l.f34e = t309;
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34c == 0.0)) && (l.f34e != 0.0)) {let t30a: f64 = (-230.25850929940458);let t30b: f64 = (l.f7b1 / l.f5f1);let t30c: f64 = (l.f5f1 - l.f5ed);let t30d: f64 = (l.f793 * t30c);let t30e: f64 = (l.f5ed * p.p85);let t30f: f64 = (t30d / t30e);let t310: f64 = (t30b + t30f);let t311: f64 = (l.f645 * t310);let t312: f64 = (t30a - t311);let t313: f64 = (-230.25850929940458);let t314: f64 = (l.f7b1 / l.f5f1);let t315: f64 = (l.f5f1 - l.f5ed);let t316: f64 = (l.f793 * t315);let t317: f64 = (l.f5ed * p.p85);let t318: f64 = (t316 / t317);let t319: f64 = (t314 + t318);let t31a: f64 = (l.f645 * t319);let t31b: f64 = (t313 - t31a);let t31c: f64 = (-230.25850929940458);let t31d: f64 = (l.f7b1 / l.f5f1);let t31e: f64 = (l.f5f1 - l.f5ed);let t31f: f64 = (l.f793 * t31e);let t320: f64 = (l.f5ed * p.p85);let t321: f64 = (t31f / t320);let t322: f64 = (t31d + t321);let t323: f64 = (l.f645 * t322);let t324: f64 = (t31c - t323);let t325: f64 = (t324 * 0.3333333333333333);let t326: f64 = (1.0 + t325);let t327: f64 = (t31b * t326);let t328: f64 = (0.5 * t327);let t329: f64 = (1.0 + t328);let t32a: f64 = (t312 * t329);let t32b: f64 = (1.0 + t32a);let t32c: f64 = (1e-100 / t32b);(l.f8a, l.f8b, l.f8c, ) = (t32c, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t30e) - (t30d * (l.f5ee * p.p85))) / (t30e * t30e))))) * t329) + (t312 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t317) - (t316 * (l.f5ee * p.p85))) / (t317 * t317))))) * t326) + (t31b * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t320) - (t31f * (l.f5ee * p.p85))) / (t320 * t320))))) * 0.3333333333333333))))))) / (t32b * t32b))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t30e) - (t30d * (l.f5ef * p.p85))) / (t30e * t30e))))) * t329) + (t312 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t317) - (t316 * (l.f5ef * p.p85))) / (t317 * t317))))) * t326) + (t31b * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t320) - (t31f * (l.f5ef * p.p85))) / (t320 * t320))))) * 0.3333333333333333))))))) / (t32b * t32b))), );}
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f34c == 0.0)) && (l.f34e == 0.0)) {let t32d: f64 = (l.f7b1 / l.f5f1);let t32e: f64 = (l.f5f1 - l.f5ed);let t32f: f64 = (l.f793 * t32e);let t330: f64 = (l.f5ed * p.p85);let t331: f64 = (t32f / t330);let t332: f64 = (t32d + t331);let t333: f64 = (l.f645 * t332);let t334: f64 = (t333 - 230.25850929940458);let t335: f64 = (l.f7b1 / l.f5f1);let t336: f64 = (l.f5f1 - l.f5ed);let t337: f64 = (l.f793 * t336);let t338: f64 = (l.f5ed * p.p85);let t339: f64 = (t337 / t338);let t33a: f64 = (t335 + t339);let t33b: f64 = (l.f645 * t33a);let t33c: f64 = (t33b - 230.25850929940458);let t33d: f64 = (l.f7b1 / l.f5f1);let t33e: f64 = (l.f5f1 - l.f5ed);let t33f: f64 = (l.f793 * t33e);let t340: f64 = (l.f5ed * p.p85);let t341: f64 = (t33f / t340);let t342: f64 = (t33d + t341);let t343: f64 = (l.f645 * t342);let t344: f64 = (t343 - 230.25850929940458);let t345: f64 = (t344 * 0.3333333333333333);let t346: f64 = (1.0 + t345);let t347: f64 = (t33c * t346);let t348: f64 = (0.5 * t347);let t349: f64 = (1.0 + t348);let t34a: f64 = (t334 * t349);let t34b: f64 = (1.0 + t34a);let t34c: f64 = (1e100 * t34b);(l.f8a, l.f8b, l.f8c, ) = (t34c, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t330) - (t32f * (l.f5ee * p.p85))) / (t330 * t330)))) * t349) + (t334 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t338) - (t337 * (l.f5ee * p.p85))) / (t338 * t338)))) * t346) + (t33c * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t340) - (t33f * (l.f5ee * p.p85))) / (t340 * t340)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t330) - (t32f * (l.f5ef * p.p85))) / (t330 * t330)))) * t349) + (t334 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t338) - (t337 * (l.f5ef * p.p85))) / (t338 * t338)))) * t346) + (t33c * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t340) - (t33f * (l.f5ef * p.p85))) / (t340 * t340)))) * 0.3333333333333333))))))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_123(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) {let t34d: f64 = (l.f7b1 * l.f5b);let t34e: f64 = (l.f5f1 - t34d);let t34f: f64 = (l.f5f1 * l.f5f1);let t350: f64 = (t34e / t34f);let t351: f64 = (l.f793 * l.f5b);let t352: f64 = (l.f5ed * p.p85);let t353: f64 = (t351 / t352);let t354: f64 = (t350 + t353);let t355: f64 = (l.f645 * t354);(l.f61, l.f62, l.f63, ) = (t355, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t34f) - (t34e * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t34f * t34f)) + ((((l.f793 * l.f5c) * t352) - (t351 * (l.f5ee * p.p85))) / (t352 * t352)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t34f) - (t34e * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t34f * t34f)) + ((((l.f793 * l.f5d) * t352) - (t351 * (l.f5ef * p.p85))) / (t352 * t352)))), );let t356: f64 = (l.f73f - l.f7b1);let t357: f64 = (t356 * l.f61);let t358: f64 = (1.0 + t357);let t359: f64 = (t358 * l.f8a);(l.f536, l.f537, l.f538, ) = (t359, (((t356 * l.f62) * l.f8a) + (t358 * l.f8b)), (((t356 * l.f63) * l.f8a) + (t358 * l.f8c)), );let t35a: f64 = (l.f5eb * l.f5eb);let t35b: f64 = (t35a / l.f5e3);l.f64f = t35b;let t35c: f64 = (l.f5e9 / l.f645);let t35d: f64 = (l.f5e3 / l.f64f);let t35e: f64 = (t35d).ln();let t35f: f64 = (t35c * t35e);l.f793 = t35f;}
        let t360: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f352 = t360;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {let t361: f64 = (l.f7b1 - l.f793);let t362: f64 = (p.p86 * t361);let t363: f64 = (t362 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t363, 0.0, 0.0, );let t364: f64 = (p.p86 * l.f793);let t365: f64 = (l.f5e9 - t364);(l.f5ed, l.f5ee, l.f5ef, ) = (t365, 0.0, 0.0, );let t366: f64 = (p.p85 - l.f601);let t367: f64 = (t366 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t367, (-l.f602), (-l.f603), );let t368: f64 = (4.0 * p.p85);let t369: f64 = (t368 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t369, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {
            let (t36b, t36c, t36d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t36a: f64 = (-l.f6f7);
        (t36a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t36b, t36c, t36d, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {let t36e: f64 = (l.f6f3 * l.f6f3);let t36f: f64 = (t36e + l.f6f7);let t370: f64 = (t36f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t370, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t370)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t370)), );let t371: f64 = (l.f6f3 / l.f6f7);let t372: f64 = (1.0 + t371);let t373: f64 = (0.5 * t372);(l.f55, l.f56, l.f57, ) = (t373, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t374: f64 = (l.f6f3 + l.f6f7);let t375: f64 = (0.5 * t374);let t376: f64 = (p.p85 - t375);(l.f605, l.f606, l.f607, ) = (t376, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t377: f64 = (l.f605 - l.f5e9);let t378: f64 = (t377 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t378, l.f606, l.f607, );let t379: f64 = (4.0 * l.f5e9);let t37a: f64 = (t379 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t37a, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {
            let (t37c, t37d, t37e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t37b: f64 = (-l.f6f7);
        (t37b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t37c, t37d, t37e, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {let t37f: f64 = (l.f6f3 * l.f6f3);let t380: f64 = (t37f + l.f6f7);let t381: f64 = (t380).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t381, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t381)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t381)), );let t382: f64 = (l.f6f3 / l.f6f7);let t383: f64 = (1.0 + t382);let t384: f64 = (0.5 * t383);(l.f51, l.f52, l.f53, ) = (t384, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t385: f64 = (l.f6f3 + l.f6f7);let t386: f64 = (0.5 * t385);let t387: f64 = (l.f5e9 + t386);(l.f5f1, l.f5f2, l.f5f3, ) = (t387, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t388: f64 = (p.p85 - l.f5ed);let t389: f64 = (t388 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t389, (-l.f5ee), (-l.f5ef), );let t38a: f64 = (4.0 * p.p85);let t38b: f64 = (t38a * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t38b, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {
            let (t38d, t38e, t38f,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t38c: f64 = (-l.f6f7);
        (t38c, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t38d, t38e, t38f, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {let t390: f64 = (l.f6f3 * l.f6f3);let t391: f64 = (t390 + l.f6f7);let t392: f64 = (t391).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t392, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t392)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t392)), );let t393: f64 = (l.f6f3 + l.f6f7);let t394: f64 = (0.5 * t393);let t395: f64 = (p.p85 - t394);(l.f5ed, l.f5ee, l.f5ef, ) = (t395, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t396: f64 = (l.f5ed - l.f5e9);let t397: f64 = (t396 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t397, l.f5ee, l.f5ef, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_124(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {let t398: f64 = (4.0 * l.f5e9);let t399: f64 = (t398 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t399, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {
            let (t39b, t39c, t39d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t39a: f64 = (-l.f6f7);
        (t39a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t39b, t39c, t39d, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 != 0.0)) {let t39e: f64 = (l.f6f3 * l.f6f3);let t39f: f64 = (t39e + l.f6f7);let t3a0: f64 = (t39f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3a0, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3a0)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3a0)), );let t3a1: f64 = (l.f6f3 + l.f6f7);let t3a2: f64 = (0.5 * t3a1);let t3a3: f64 = (l.f5e9 + t3a2);(l.f5ed, l.f5ee, l.f5ef, ) = (t3a3, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t3a4: f64 = (p.p86 * l.f55);let t3a5: f64 = (t3a4 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t3a5, (((p.p86 * l.f56) * l.f51) + (t3a4 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t3a4 * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f352 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t3a6: f64 = (l.f7b1 / l.f5f1);let t3a7: f64 = (l.f5f1 - l.f5ed);let t3a8: f64 = (l.f793 * t3a7);let t3a9: f64 = (l.f5ed * p.p85);let t3aa: f64 = (t3a8 / t3a9);let t3ab: f64 = (t3a6 + t3aa);let t3ac: f64 = (l.f645 * t3ab);let t3ad: f64 = (t3ac).abs();let t3ae: f64 = if t3ad < 230.25850929940458 { 1.0 } else { 0.0 };l.f354 = t3ae;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f354 != 0.0)) {let t3af: f64 = (l.f7b1 / l.f5f1);let t3b0: f64 = (l.f5f1 - l.f5ed);let t3b1: f64 = (l.f793 * t3b0);let t3b2: f64 = (l.f5ed * p.p85);let t3b3: f64 = (t3b1 / t3b2);let t3b4: f64 = (t3af + t3b3);let t3b5: f64 = (l.f645 * t3b4);let t3b6: f64 = (t3b5).exp();(l.f93, l.f94, l.f95, ) = (t3b6, (t3b6 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3b2) - (t3b1 * (l.f5ee * p.p85))) / (t3b2 * t3b2))))), (t3b6 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3b2) - (t3b1 * (l.f5ef * p.p85))) / (t3b2 * t3b2))))), );}
        let t3b7: f64 = (l.f7b1 / l.f5f1);let t3b8: f64 = (l.f5f1 - l.f5ed);let t3b9: f64 = (l.f793 * t3b8);let t3ba: f64 = (l.f5ed * p.p85);let t3bb: f64 = (t3b9 / t3ba);let t3bc: f64 = (t3b7 + t3bb);let t3bd: f64 = (l.f645 * t3bc);let t3be: f64 = (-230.25850929940458);let t3bf: f64 = if t3bd < t3be { 1.0 } else { 0.0 };l.f356 = t3bf;
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f354 == 0.0)) && (l.f356 != 0.0)) {let t3c0: f64 = (-230.25850929940458);let t3c1: f64 = (l.f7b1 / l.f5f1);let t3c2: f64 = (l.f5f1 - l.f5ed);let t3c3: f64 = (l.f793 * t3c2);let t3c4: f64 = (l.f5ed * p.p85);let t3c5: f64 = (t3c3 / t3c4);let t3c6: f64 = (t3c1 + t3c5);let t3c7: f64 = (l.f645 * t3c6);let t3c8: f64 = (t3c0 - t3c7);let t3c9: f64 = (-230.25850929940458);let t3ca: f64 = (l.f7b1 / l.f5f1);let t3cb: f64 = (l.f5f1 - l.f5ed);let t3cc: f64 = (l.f793 * t3cb);let t3cd: f64 = (l.f5ed * p.p85);let t3ce: f64 = (t3cc / t3cd);let t3cf: f64 = (t3ca + t3ce);let t3d0: f64 = (l.f645 * t3cf);let t3d1: f64 = (t3c9 - t3d0);let t3d2: f64 = (-230.25850929940458);let t3d3: f64 = (l.f7b1 / l.f5f1);let t3d4: f64 = (l.f5f1 - l.f5ed);let t3d5: f64 = (l.f793 * t3d4);let t3d6: f64 = (l.f5ed * p.p85);let t3d7: f64 = (t3d5 / t3d6);let t3d8: f64 = (t3d3 + t3d7);let t3d9: f64 = (l.f645 * t3d8);let t3da: f64 = (t3d2 - t3d9);let t3db: f64 = (t3da * 0.3333333333333333);let t3dc: f64 = (1.0 + t3db);let t3dd: f64 = (t3d1 * t3dc);let t3de: f64 = (0.5 * t3dd);let t3df: f64 = (1.0 + t3de);let t3e0: f64 = (t3c8 * t3df);let t3e1: f64 = (1.0 + t3e0);let t3e2: f64 = (1e-100 / t3e1);(l.f93, l.f94, l.f95, ) = (t3e2, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3c4) - (t3c3 * (l.f5ee * p.p85))) / (t3c4 * t3c4))))) * t3df) + (t3c8 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3cd) - (t3cc * (l.f5ee * p.p85))) / (t3cd * t3cd))))) * t3dc) + (t3d1 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3d6) - (t3d5 * (l.f5ee * p.p85))) / (t3d6 * t3d6))))) * 0.3333333333333333))))))) / (t3e1 * t3e1))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3c4) - (t3c3 * (l.f5ef * p.p85))) / (t3c4 * t3c4))))) * t3df) + (t3c8 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3cd) - (t3cc * (l.f5ef * p.p85))) / (t3cd * t3cd))))) * t3dc) + (t3d1 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3d6) - (t3d5 * (l.f5ef * p.p85))) / (t3d6 * t3d6))))) * 0.3333333333333333))))))) / (t3e1 * t3e1))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_125(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f354 == 0.0)) && (l.f356 == 0.0)) {let t3e3: f64 = (l.f7b1 / l.f5f1);let t3e4: f64 = (l.f5f1 - l.f5ed);let t3e5: f64 = (l.f793 * t3e4);let t3e6: f64 = (l.f5ed * p.p85);let t3e7: f64 = (t3e5 / t3e6);let t3e8: f64 = (t3e3 + t3e7);let t3e9: f64 = (l.f645 * t3e8);let t3ea: f64 = (t3e9 - 230.25850929940458);let t3eb: f64 = (l.f7b1 / l.f5f1);let t3ec: f64 = (l.f5f1 - l.f5ed);let t3ed: f64 = (l.f793 * t3ec);let t3ee: f64 = (l.f5ed * p.p85);let t3ef: f64 = (t3ed / t3ee);let t3f0: f64 = (t3eb + t3ef);let t3f1: f64 = (l.f645 * t3f0);let t3f2: f64 = (t3f1 - 230.25850929940458);let t3f3: f64 = (l.f7b1 / l.f5f1);let t3f4: f64 = (l.f5f1 - l.f5ed);let t3f5: f64 = (l.f793 * t3f4);let t3f6: f64 = (l.f5ed * p.p85);let t3f7: f64 = (t3f5 / t3f6);let t3f8: f64 = (t3f3 + t3f7);let t3f9: f64 = (l.f645 * t3f8);let t3fa: f64 = (t3f9 - 230.25850929940458);let t3fb: f64 = (t3fa * 0.3333333333333333);let t3fc: f64 = (1.0 + t3fb);let t3fd: f64 = (t3f2 * t3fc);let t3fe: f64 = (0.5 * t3fd);let t3ff: f64 = (1.0 + t3fe);let t400: f64 = (t3ea * t3ff);let t401: f64 = (1.0 + t400);let t402: f64 = (1e100 * t401);(l.f93, l.f94, l.f95, ) = (t402, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3e6) - (t3e5 * (l.f5ee * p.p85))) / (t3e6 * t3e6)))) * t3ff) + (t3ea * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3ee) - (t3ed * (l.f5ee * p.p85))) / (t3ee * t3ee)))) * t3fc) + (t3f2 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3f6) - (t3f5 * (l.f5ee * p.p85))) / (t3f6 * t3f6)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3e6) - (t3e5 * (l.f5ef * p.p85))) / (t3e6 * t3e6)))) * t3ff) + (t3ea * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3ee) - (t3ed * (l.f5ef * p.p85))) / (t3ee * t3ee)))) * t3fc) + (t3f2 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3f6) - (t3f5 * (l.f5ef * p.p85))) / (t3f6 * t3f6)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) {let t403: f64 = (l.f7b1 * l.f5b);let t404: f64 = (l.f5f1 - t403);let t405: f64 = (l.f5f1 * l.f5f1);let t406: f64 = (t404 / t405);let t407: f64 = (l.f793 * l.f5b);let t408: f64 = (l.f5ed * p.p85);let t409: f64 = (t407 / t408);let t40a: f64 = (t406 + t409);let t40b: f64 = (l.f645 * t40a);(l.f61, l.f62, l.f63, ) = (t40b, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t405) - (t404 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t405 * t405)) + ((((l.f793 * l.f5c) * t408) - (t407 * (l.f5ee * p.p85))) / (t408 * t408)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t405) - (t404 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t405 * t405)) + ((((l.f793 * l.f5d) * t408) - (t407 * (l.f5ef * p.p85))) / (t408 * t408)))), );let t40c: f64 = (l.f73f - l.f7b1);let t40d: f64 = (t40c * l.f61);let t40e: f64 = (1.0 + t40d);let t40f: f64 = (t40e * l.f93);(l.f53e, l.f53f, l.f540, ) = (t40f, (((t40c * l.f62) * l.f93) + (t40e * l.f94)), (((t40c * l.f63) * l.f93) + (t40e * l.f95)), );let t410: f64 = (l.f5eb * l.f5eb);let t411: f64 = (t410 / l.f5e1);l.f64f = t411;let t412: f64 = (l.f5e7 / l.f645);let t413: f64 = (l.f5e1 / l.f64f);let t414: f64 = (t413).ln();let t415: f64 = (t412 * t414);l.f793 = t415;}
        let t416: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f358 = t416;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {let t417: f64 = (l.f7b1 - l.f793);let t418: f64 = (p.p86 * t417);let t419: f64 = (t418 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t419, 0.0, 0.0, );let t41a: f64 = (p.p86 * l.f793);let t41b: f64 = (l.f5e7 - t41a);(l.f5ed, l.f5ee, l.f5ef, ) = (t41b, 0.0, 0.0, );let t41c: f64 = (p.p85 - l.f601);let t41d: f64 = (t41c - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t41d, (-l.f602), (-l.f603), );let t41e: f64 = (4.0 * p.p85);let t41f: f64 = (t41e * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t41f, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {
            let (t421, t422, t423,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t420: f64 = (-l.f6f7);
        (t420, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t421, t422, t423, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {let t424: f64 = (l.f6f3 * l.f6f3);let t425: f64 = (t424 + l.f6f7);let t426: f64 = (t425).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t426, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t426)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t426)), );let t427: f64 = (l.f6f3 / l.f6f7);let t428: f64 = (1.0 + t427);let t429: f64 = (0.5 * t428);(l.f55, l.f56, l.f57, ) = (t429, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t42a: f64 = (l.f6f3 + l.f6f7);let t42b: f64 = (0.5 * t42a);let t42c: f64 = (p.p85 - t42b);(l.f605, l.f606, l.f607, ) = (t42c, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t42d: f64 = (l.f605 - l.f5e7);let t42e: f64 = (t42d - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t42e, l.f606, l.f607, );let t42f: f64 = (4.0 * l.f5e7);let t430: f64 = (t42f * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t430, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {
            let (t432, t433, t434,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t431: f64 = (-l.f6f7);
        (t431, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t432, t433, t434, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_126(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {let t435: f64 = (l.f6f3 * l.f6f3);let t436: f64 = (t435 + l.f6f7);let t437: f64 = (t436).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t437, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t437)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t437)), );let t438: f64 = (l.f6f3 / l.f6f7);let t439: f64 = (1.0 + t438);let t43a: f64 = (0.5 * t439);(l.f51, l.f52, l.f53, ) = (t43a, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t43b: f64 = (l.f6f3 + l.f6f7);let t43c: f64 = (0.5 * t43b);let t43d: f64 = (l.f5e7 + t43c);(l.f5f1, l.f5f2, l.f5f3, ) = (t43d, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t43e: f64 = (p.p85 - l.f5ed);let t43f: f64 = (t43e - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t43f, (-l.f5ee), (-l.f5ef), );let t440: f64 = (4.0 * p.p85);let t441: f64 = (t440 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t441, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {
            let (t443, t444, t445,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t442: f64 = (-l.f6f7);
        (t442, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t443, t444, t445, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {let t446: f64 = (l.f6f3 * l.f6f3);let t447: f64 = (t446 + l.f6f7);let t448: f64 = (t447).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t448, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t448)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t448)), );let t449: f64 = (l.f6f3 + l.f6f7);let t44a: f64 = (0.5 * t449);let t44b: f64 = (p.p85 - t44a);(l.f5ed, l.f5ee, l.f5ef, ) = (t44b, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t44c: f64 = (l.f5ed - l.f5e7);let t44d: f64 = (t44c - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t44d, l.f5ee, l.f5ef, );let t44e: f64 = (4.0 * l.f5e7);let t44f: f64 = (t44e * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t44f, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {
            let (t451, t452, t453,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t450: f64 = (-l.f6f7);
        (t450, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t451, t452, t453, );
        }
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 != 0.0)) {let t454: f64 = (l.f6f3 * l.f6f3);let t455: f64 = (t454 + l.f6f7);let t456: f64 = (t455).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t456, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t456)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t456)), );let t457: f64 = (l.f6f3 + l.f6f7);let t458: f64 = (0.5 * t457);let t459: f64 = (l.f5e7 + t458);(l.f5ed, l.f5ee, l.f5ef, ) = (t459, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t45a: f64 = (p.p86 * l.f55);let t45b: f64 = (t45a * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t45b, (((p.p86 * l.f56) * l.f51) + (t45a * l.f52)), (((p.p86 * l.f57) * l.f51) + (t45a * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f358 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t45c: f64 = (l.f7b1 / l.f5f1);let t45d: f64 = (l.f5f1 - l.f5ed);let t45e: f64 = (l.f793 * t45d);let t45f: f64 = (l.f5ed * p.p85);let t460: f64 = (t45e / t45f);let t461: f64 = (t45c + t460);let t462: f64 = (l.f645 * t461);let t463: f64 = (t462).abs();let t464: f64 = if t463 < 230.25850929940458 { 1.0 } else { 0.0 };l.f35a = t464;
        if ((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f35a != 0.0)) {let t465: f64 = (l.f7b1 / l.f5f1);let t466: f64 = (l.f5f1 - l.f5ed);let t467: f64 = (l.f793 * t466);let t468: f64 = (l.f5ed * p.p85);let t469: f64 = (t467 / t468);let t46a: f64 = (t465 + t469);let t46b: f64 = (l.f645 * t46a);let t46c: f64 = (t46b).exp();(l.f8e, l.f8f, l.f90, ) = (t46c, (t46c * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t468) - (t467 * (l.f5ee * p.p85))) / (t468 * t468))))), (t46c * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t468) - (t467 * (l.f5ef * p.p85))) / (t468 * t468))))), );}
        let t46d: f64 = (l.f7b1 / l.f5f1);let t46e: f64 = (l.f5f1 - l.f5ed);let t46f: f64 = (l.f793 * t46e);let t470: f64 = (l.f5ed * p.p85);let t471: f64 = (t46f / t470);let t472: f64 = (t46d + t471);let t473: f64 = (l.f645 * t472);let t474: f64 = (-230.25850929940458);let t475: f64 = if t473 < t474 { 1.0 } else { 0.0 };l.f35c = t475;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_127(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f35a == 0.0)) && (l.f35c != 0.0)) {let t476: f64 = (-230.25850929940458);let t477: f64 = (l.f7b1 / l.f5f1);let t478: f64 = (l.f5f1 - l.f5ed);let t479: f64 = (l.f793 * t478);let t47a: f64 = (l.f5ed * p.p85);let t47b: f64 = (t479 / t47a);let t47c: f64 = (t477 + t47b);let t47d: f64 = (l.f645 * t47c);let t47e: f64 = (t476 - t47d);let t47f: f64 = (-230.25850929940458);let t480: f64 = (l.f7b1 / l.f5f1);let t481: f64 = (l.f5f1 - l.f5ed);let t482: f64 = (l.f793 * t481);let t483: f64 = (l.f5ed * p.p85);let t484: f64 = (t482 / t483);let t485: f64 = (t480 + t484);let t486: f64 = (l.f645 * t485);let t487: f64 = (t47f - t486);let t488: f64 = (-230.25850929940458);let t489: f64 = (l.f7b1 / l.f5f1);let t48a: f64 = (l.f5f1 - l.f5ed);let t48b: f64 = (l.f793 * t48a);let t48c: f64 = (l.f5ed * p.p85);let t48d: f64 = (t48b / t48c);let t48e: f64 = (t489 + t48d);let t48f: f64 = (l.f645 * t48e);let t490: f64 = (t488 - t48f);let t491: f64 = (t490 * 0.3333333333333333);let t492: f64 = (1.0 + t491);let t493: f64 = (t487 * t492);let t494: f64 = (0.5 * t493);let t495: f64 = (1.0 + t494);let t496: f64 = (t47e * t495);let t497: f64 = (1.0 + t496);let t498: f64 = (1e-100 / t497);(l.f8e, l.f8f, l.f90, ) = (t498, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t47a) - (t479 * (l.f5ee * p.p85))) / (t47a * t47a))))) * t495) + (t47e * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t483) - (t482 * (l.f5ee * p.p85))) / (t483 * t483))))) * t492) + (t487 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t48c) - (t48b * (l.f5ee * p.p85))) / (t48c * t48c))))) * 0.3333333333333333))))))) / (t497 * t497))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t47a) - (t479 * (l.f5ef * p.p85))) / (t47a * t47a))))) * t495) + (t47e * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t483) - (t482 * (l.f5ef * p.p85))) / (t483 * t483))))) * t492) + (t487 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t48c) - (t48b * (l.f5ef * p.p85))) / (t48c * t48c))))) * 0.3333333333333333))))))) / (t497 * t497))), );}
        if (((((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) && (l.f35a == 0.0)) && (l.f35c == 0.0)) {let t499: f64 = (l.f7b1 / l.f5f1);let t49a: f64 = (l.f5f1 - l.f5ed);let t49b: f64 = (l.f793 * t49a);let t49c: f64 = (l.f5ed * p.p85);let t49d: f64 = (t49b / t49c);let t49e: f64 = (t499 + t49d);let t49f: f64 = (l.f645 * t49e);let t4a0: f64 = (t49f - 230.25850929940458);let t4a1: f64 = (l.f7b1 / l.f5f1);let t4a2: f64 = (l.f5f1 - l.f5ed);let t4a3: f64 = (l.f793 * t4a2);let t4a4: f64 = (l.f5ed * p.p85);let t4a5: f64 = (t4a3 / t4a4);let t4a6: f64 = (t4a1 + t4a5);let t4a7: f64 = (l.f645 * t4a6);let t4a8: f64 = (t4a7 - 230.25850929940458);let t4a9: f64 = (l.f7b1 / l.f5f1);let t4aa: f64 = (l.f5f1 - l.f5ed);let t4ab: f64 = (l.f793 * t4aa);let t4ac: f64 = (l.f5ed * p.p85);let t4ad: f64 = (t4ab / t4ac);let t4ae: f64 = (t4a9 + t4ad);let t4af: f64 = (l.f645 * t4ae);let t4b0: f64 = (t4af - 230.25850929940458);let t4b1: f64 = (t4b0 * 0.3333333333333333);let t4b2: f64 = (1.0 + t4b1);let t4b3: f64 = (t4a8 * t4b2);let t4b4: f64 = (0.5 * t4b3);let t4b5: f64 = (1.0 + t4b4);let t4b6: f64 = (t4a0 * t4b5);let t4b7: f64 = (1.0 + t4b6);let t4b8: f64 = (1e100 * t4b7);(l.f8e, l.f8f, l.f90, ) = (t4b8, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t49c) - (t49b * (l.f5ee * p.p85))) / (t49c * t49c)))) * t4b5) + (t4a0 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t4a4) - (t4a3 * (l.f5ee * p.p85))) / (t4a4 * t4a4)))) * t4b2) + (t4a8 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t4ac) - (t4ab * (l.f5ee * p.p85))) / (t4ac * t4ac)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t49c) - (t49b * (l.f5ef * p.p85))) / (t49c * t49c)))) * t4b5) + (t4a0 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t4a4) - (t4a3 * (l.f5ef * p.p85))) / (t4a4 * t4a4)))) * t4b2) + (t4a8 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t4ac) - (t4ab * (l.f5ef * p.p85))) / (t4ac * t4ac)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f332 == 0.0)) {let t4b9: f64 = (l.f7b1 * l.f5b);let t4ba: f64 = (l.f5f1 - t4b9);let t4bb: f64 = (l.f5f1 * l.f5f1);let t4bc: f64 = (t4ba / t4bb);let t4bd: f64 = (l.f793 * l.f5b);let t4be: f64 = (l.f5ed * p.p85);let t4bf: f64 = (t4bd / t4be);let t4c0: f64 = (t4bc + t4bf);let t4c1: f64 = (l.f645 * t4c0);(l.f61, l.f62, l.f63, ) = (t4c1, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t4bb) - (t4ba * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t4bb * t4bb)) + ((((l.f793 * l.f5c) * t4be) - (t4bd * (l.f5ee * p.p85))) / (t4be * t4be)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t4bb) - (t4ba * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t4bb * t4bb)) + ((((l.f793 * l.f5d) * t4be) - (t4bd * (l.f5ef * p.p85))) / (t4be * t4be)))), );let t4c2: f64 = (l.f73f - l.f7b1);let t4c3: f64 = (t4c2 * l.f61);let t4c4: f64 = (1.0 + t4c3);let t4c5: f64 = (t4c4 * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t4c5, (((t4c2 * l.f62) * l.f8e) + (t4c4 * l.f8f)), (((t4c2 * l.f63) * l.f8e) + (t4c4 * l.f90)), );}
        if ((l.f29a != 0.0) && (l.f330 != 0.0)) {let t4c6: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t4c6, l.f537, l.f538, );let t4c7: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t4c7, l.f53f, l.f540, );let t4c8: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t4c8, l.f53b, l.f53c, );let t4c9: f64 = (1.0 / l.f825);l.f817 = t4c9;}
        let t4ca: f64 = if l.f73f > 0.0 { 1.0 } else { 0.0 };l.f35e = t4ca;
    }
}
