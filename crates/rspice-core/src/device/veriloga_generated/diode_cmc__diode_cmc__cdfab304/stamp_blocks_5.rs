#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_80(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f23b == 0.0)) && (l.f23d != 0.0)) && (l.f241 != 0.0)) {let t0: f64 = (l.f74a * l.f787);let t1: f64 = (t0).abs();let t2: f64 = (l.f74a * l.f787);let t3: f64 = (t2).abs();let t4: f64 = (t1 * t3);let t5: f64 = (l.f74a * l.f787);let t6: f64 = (t5).abs();let t7: f64 = (t4 * t6);let t8: f64 = (l.f74a * l.f787);let t9: f64 = (t8).abs();let ta: f64 = (t7 * t9);l.f6fc = ta;}
        if (((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f23b == 0.0)) && (l.f23d != 0.0)) && (l.f241 == 0.0)) {let tb: f64 = (l.f74a * l.f787);let tc: f64 = (tb).abs();let td: f64 = (tc).powf(l.f625);l.f6fc = td;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f23b == 0.0)) && (l.f23d != 0.0)) {let te: f64 = (1.0 - l.f6fc);let tf: f64 = (1.0 / te);l.fae = tf;}
        if ((((l.f29a != 0.0) && (l.f21c == 0.0)) && (l.f23b == 0.0)) && (l.f23d == 0.0)) {let t10: f64 = (l.f2 * l.f783);let t11: f64 = (l.f74a + t10);let t12: f64 = (t11 * l.f6ba);let t13: f64 = (l.fc3 + t12);l.fae = t13;}
        if ((l.f29a != 0.0) && (l.f21c == 0.0)) {let t14: f64 = (l.f52f + l.f593);let t15: f64 = (t14 + l.f599);let t16: f64 = (t15 + l.f529);let t17: f64 = (t16 * l.fae);(l.f562, l.f563, l.f564, ) = (t17, (l.f530 * l.fae), (l.f531 * l.fae), );let t18: f64 = (l.f593 + l.f599);let t19: f64 = (t18 + l.f529);let t1a: f64 = (t19 * l.fae);(l.f552, l.f553, l.f554, ) = (t1a, 0.0, 0.0, );}
        let t1b: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f243 = t1b;
        if ((l.f29a != 0.0) && (l.f243 != 0.0)) {(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );}
        let t1c: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f245 = t1c;
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f245 != 0.0)) {let t1d: f64 = (l.f796 * l.f76d);let t1e: f64 = (1.0 - t1d);let t1f: f64 = (t1e).sqrt();l.f6fc = t1f;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f245 == 0.0)) {let t20: f64 = (l.f796 * l.f76d);let t21: f64 = (1.0 - t20);let t22: f64 = (t21).powf(l.f60f);l.f6fc = t22;}
        if ((l.f29a != 0.0) && (l.f243 == 0.0)) {let t23: f64 = (1.0 - l.f6fc);let t24: f64 = (l.f6a2 * t23);let t25: f64 = (l.f73b - l.f796);let t26: f64 = (l.f69c * t25);let t27: f64 = (t24 + t26);(l.f694, l.f695, l.f696, ) = (t27, 0.0, 0.0, );let t28: f64 = (l.f54c * l.f53e);(l.f52f, l.f530, l.f531, ) = (t28, (l.f54c * l.f53f), (l.f54c * l.f540), );}
        let t29: f64 = if ((l.f3d == 0.0) && (l.f43 == 0.0)) { 1.0 } else { 0.0 };l.f247 = t29;
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 == 0.0)) {let t2a: f64 = (l.f77d - l.f7a2);l.f758 = t2a;let t2b: f64 = (l.f714 / l.f758);let t2c: f64 = (1.0 - t2b);let t2d: f64 = (t2c).sqrt();let t2e: f64 = (1.0 - t2d);l.f7ef = t2e;}
        let t2f: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f249 = t2f;
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 == 0.0)) && (l.f249 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 == 0.0)) && (l.f249 == 0.0)) {let t30: f64 = (l.f7ef * l.f7ef);let t31: f64 = (l.f7ef).ln();let t32: f64 = (t30 * t31);let t33: f64 = (1.0 - l.f7ef);let t34: f64 = (t32 / t33);let t35: f64 = (t34 + l.f7ef);let t36: f64 = (2.0 * l.f653);let t37: f64 = (1.0 - t36);let t38: f64 = (t35 * t37);l.f66 = t38;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 == 0.0)) {let t39: f64 = (l.f7ef + l.f66);l.f7e9 = t39;}
        let t3a: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f24b = t3a;
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 == 0.0)) && (l.f24b != 0.0)) {let t3b: f64 = (l.f758 * l.f77b);let t3c: f64 = (t3b).sqrt();l.f6fc = t3c;}
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 == 0.0)) && (l.f24b == 0.0)) {let t3d: f64 = (l.f758 * l.f77b);let t3e: f64 = (t3d).powf(l.f653);l.f6fc = t3e;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 == 0.0)) {let t3f: f64 = (l.f7e0 * l.f6fc);l.f7d1 = t3f;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_81(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f247 == 0.0)) {let t40: f64 = (l.f825 - 1.0);let t41: f64 = (t40 * l.f7d1);let t42: f64 = (l.fd1 * t41);l.f9 = t42;let t43: f64 = (l.f9 * l.f7e9);let t44: f64 = (l.f3d * t43);l.f593 = t44;}
        let t45: f64 = if l.f43 == 0.0 { 1.0 } else { 0.0 };l.f24d = t45;
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) {let t46: f64 = (l.f7d1 * l.f60f);let t47: f64 = (t46 / l.f758);let t48: f64 = (l.f22 * t47);l.f19 = t48;let t49: f64 = (0.666666666666667 * l.f12);let t4a: f64 = (t49 / l.f19);l.f71a = t4a;let t4b: f64 = (l.f71a * l.f71a);l.f72c = t4b;let t4c: f64 = (l.f72c * l.f72c);let t4d: f64 = (l.f72c * l.f72c);let t4e: f64 = (t4d + 1.0);let t4f: f64 = (t4c / t4e);let t50: f64 = (t4f).sqrt();l.f726 = t50;let t51: f64 = (l.f726).abs();let t52: f64 = (t51).sqrt();l.f6c1 = t52;let t53: f64 = (l.f726 * l.f6c1);l.f732 = t53;}
        let t54: f64 = (-l.f653);let t55: f64 = (t54 * l.f615);let t56: f64 = (-1.0);let t57: f64 = if t55 == t56 { 1.0 } else { 0.0 };l.f24f = t57;
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f24f != 0.0)) {let t58: f64 = (l.f19 * l.f732);let t59: f64 = (1.0 + t58);let t5a: f64 = (1.0 / t59);l.f7e3 = t5a;}
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f24f == 0.0)) {let t5b: f64 = (l.f19 * l.f732);let t5c: f64 = (1.0 + t5b);let t5d: f64 = (-l.f653);let t5e: f64 = (t5d * l.f615);let t5f: f64 = (t5c).powf(t5e);l.f7e3 = t5f;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) {let t60: f64 = (l.f7e9 * l.f7e3);let t61: f64 = (l.f7e9 + l.f7e3);let t62: f64 = (t60 / t61);l.f7f5 = t62;let t63: f64 = (l.f19 / l.f6c1);let t64: f64 = (0.375 * t63);let t65: f64 = (t64).sqrt();l.f5a8 = t65;let t66: f64 = (l.f71a * l.f6c1);let t67: f64 = (2.0 * t66);let t68: f64 = (t67 - l.f726);l.f5b4 = t68;let t69: f64 = (l.f12 * l.f71a);let t6a: f64 = (t69 * l.f6c1);let t6b: f64 = (l.f12 * l.f726);let t6c: f64 = (t6a - t6b);let t6d: f64 = (l.f19 * l.f732);let t6e: f64 = (0.5 * t6d);let t6f: f64 = (t6c + t6e);l.f5d4 = t6f;let t70: f64 = (l.f5b4 - 1.0);let t71: f64 = (t70 * l.f5a8);l.f7fb = t71;let t72: f64 = (l.f7fb * l.f7fb);l.f811 = t72;}
        let t73: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f251 = t73;
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f251 != 0.0)) {let t74: f64 = (l.f62b * l.f7fb);let t75: f64 = (1.0 + t74);let t76: f64 = (1.0 / t75);l.f6e2 = t76;}
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f251 == 0.0)) {let t77: f64 = (l.f62b * l.f7fb);let t78: f64 = (1.0 - t77);let t79: f64 = (1.0 / t78);l.f6e2 = t79;}
        let t7a: f64 = (-l.f811);let t7b: f64 = (t7a + l.f5d4);let t7c: f64 = (-230.25850929940458);let t7d: f64 = if t7b > t7c { 1.0 } else { 0.0 };l.f253 = t7d;
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f253 != 0.0)) {let t7e: f64 = (-l.f811);let t7f: f64 = (t7e + l.f5d4);let t80: f64 = (t7f).exp();l.f6fc = t80;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_82(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f253 == 0.0)) {let t81: f64 = (-230.25850929940458);let t82: f64 = (-l.f811);let t83: f64 = (t82 + l.f5d4);let t84: f64 = (t81 - t83);let t85: f64 = (-230.25850929940458);let t86: f64 = (-l.f811);let t87: f64 = (t86 + l.f5d4);let t88: f64 = (t85 - t87);let t89: f64 = (-230.25850929940458);let t8a: f64 = (-l.f811);let t8b: f64 = (t8a + l.f5d4);let t8c: f64 = (t89 - t8b);let t8d: f64 = (t8c * 0.3333333333333333);let t8e: f64 = (1.0 + t8d);let t8f: f64 = (t88 * t8e);let t90: f64 = (0.5 * t8f);let t91: f64 = (1.0 + t90);let t92: f64 = (t84 * t91);let t93: f64 = (1.0 + t92);let t94: f64 = (1e-100 / t93);l.f6fc = t94;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) {let t95: f64 = (0.29214664 * l.f6e2);let t96: f64 = (l.f6e2 * l.f6e2);let t97: f64 = (l.f16 * t96);let t98: f64 = (t95 + t97);let t99: f64 = (l.f6e2 * l.f6e2);let t9a: f64 = (t99 * l.f6e2);let t9b: f64 = (l.f2a * t9a);let t9c: f64 = (t98 + t9b);let t9d: f64 = (t9c * l.f6fc);l.f6e = t9d;}
        let t9e: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f257 = t9e;
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f257 != 0.0)) {l.f74 = l.f6e;}
        let t9f: f64 = (-230.25850929940458);let ta0: f64 = if l.f5d4 > t9f { 1.0 } else { 0.0 };l.f259 = ta0;
        if (((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f257 == 0.0)) && (l.f259 != 0.0)) {let ta1: f64 = (l.f5d4).exp();l.f6fc = ta1;}
        if (((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f257 == 0.0)) && (l.f259 == 0.0)) {let ta2: f64 = (-230.25850929940458);let ta3: f64 = (ta2 - l.f5d4);let ta4: f64 = (-230.25850929940458);let ta5: f64 = (ta4 - l.f5d4);let ta6: f64 = (-230.25850929940458);let ta7: f64 = (ta6 - l.f5d4);let ta8: f64 = (ta7 * 0.3333333333333333);let ta9: f64 = (1.0 + ta8);let taa: f64 = (ta5 * ta9);let tab: f64 = (0.5 * taa);let tac: f64 = (1.0 + tab);let tad: f64 = (ta3 * tac);let tae: f64 = (1.0 + tad);let taf: f64 = (1e-100 / tae);l.f6fc = taf;}
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) && (l.f257 == 0.0)) {let tb0: f64 = (2.0 * l.f6fc);let tb1: f64 = (tb0 - l.f6e);l.f74 = tb1;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f24d == 0.0)) {let tb2: f64 = (1.772453850905516 * 0.5);let tb3: f64 = (l.f12 * l.f74);let tb4: f64 = (tb3 / l.f5a8);let tb5: f64 = (tb2 * tb4);l.fd6 = tb5;let tb6: f64 = (l.f9 * l.fd6);let tb7: f64 = (tb6 * l.f7f5);let tb8: f64 = (l.f43 * tb7);l.f599 = tb8;}
        let tb9: f64 = if l.f28 == 0.0 { 1.0 } else { 0.0 };l.f25b = tb9;
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f25b != 0.0)) {l.f529 = 0.0;}
        let tba: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f25d = tba;
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f25b == 0.0)) && (l.f25d != 0.0)) {let tbb: f64 = (l.f779 - l.f750);let tbc: f64 = (tbb * l.f77b);let tbd: f64 = (tbc).sqrt();l.f6fc = tbd;}
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f25b == 0.0)) && (l.f25d == 0.0)) {let tbe: f64 = (l.f779 - l.f750);let tbf: f64 = (tbe * l.f77b);let tc0: f64 = (tbf).powf(l.f653);l.f6fc = tc0;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f25b == 0.0)) {let tc1: f64 = (l.f779 - l.f750);let tc2: f64 = (tc1 * l.f7de);let tc3: f64 = (tc2 / l.f6fc);let tc4: f64 = (l.f615 * tc3);l.fb6 = tc4;}
        let tc5: f64 = (-l.fab);let tc6: f64 = (tc5 / l.fb6);let tc7: f64 = (tc6).abs();let tc8: f64 = if tc7 < 230.25850929940458 { 1.0 } else { 0.0 };l.f25f = tc8;
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f25b == 0.0)) && (l.f25f != 0.0)) {let tc9: f64 = (-l.fab);let tca: f64 = (tc9 / l.fb6);let tcb: f64 = (tca).exp();l.f6fc = tcb;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_83(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tcc: f64 = (-l.fab);let tcd: f64 = (tcc / l.fb6);let tce: f64 = (-230.25850929940458);let tcf: f64 = if tcd < tce { 1.0 } else { 0.0 };l.f261 = tcf;
        if (((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f25b == 0.0)) && (l.f25f == 0.0)) && (l.f261 != 0.0)) {let td0: f64 = (-230.25850929940458);let td1: f64 = (-l.fab);let td2: f64 = (td1 / l.fb6);let td3: f64 = (td0 - td2);let td4: f64 = (-230.25850929940458);let td5: f64 = (-l.fab);let td6: f64 = (td5 / l.fb6);let td7: f64 = (td4 - td6);let td8: f64 = (-230.25850929940458);let td9: f64 = (-l.fab);let tda: f64 = (td9 / l.fb6);let tdb: f64 = (td8 - tda);let tdc: f64 = (tdb * 0.3333333333333333);let tdd: f64 = (1.0 + tdc);let tde: f64 = (td7 * tdd);let tdf: f64 = (0.5 * tde);let te0: f64 = (1.0 + tdf);let te1: f64 = (td3 * te0);let te2: f64 = (1.0 + te1);let te3: f64 = (1e-100 / te2);l.f6fc = te3;}
        if (((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f25b == 0.0)) && (l.f25f == 0.0)) && (l.f261 == 0.0)) {let te4: f64 = (-l.fab);let te5: f64 = (te4 / l.fb6);let te6: f64 = (te5 - 230.25850929940458);let te7: f64 = (-l.fab);let te8: f64 = (te7 / l.fb6);let te9: f64 = (te8 - 230.25850929940458);let tea: f64 = (-l.fab);let teb: f64 = (tea / l.fb6);let tec: f64 = (teb - 230.25850929940458);let ted: f64 = (tec * 0.3333333333333333);let tee: f64 = (1.0 + ted);let tef: f64 = (te9 * tee);let tf0: f64 = (0.5 * tef);let tf1: f64 = (1.0 + tf0);let tf2: f64 = (te6 * tf1);let tf3: f64 = (1.0 + tf2);let tf4: f64 = (1e100 * tf3);l.f6fc = tf4;}
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f25b == 0.0)) {let tf5: f64 = (l.f73b * l.fb6);let tf6: f64 = (tf5 * l.fb6);let tf7: f64 = (tf6 * l.f6fc);let tf8: f64 = (l.f28 * tf7);l.f529 = tf8;}
        let tf9: f64 = if ((l.f78d > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f263 = tf9;
        if (((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f263 != 0.0)) {l.fae = 1.0;}
        let tfa: f64 = (-l.f2);let tfb: f64 = (tfa * l.f78d);let tfc: f64 = if l.f74a > tfb { 1.0 } else { 0.0 };l.f265 = tfc;let tfd: f64 = if l.f629 == 4.0 { 1.0 } else { 0.0 };l.f267 = tfd;
        if (((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f263 == 0.0)) && (l.f265 != 0.0)) && (l.f267 != 0.0)) {let tfe: f64 = (l.f74a * l.f78b);let tff: f64 = (tfe).abs();let t100: f64 = (l.f74a * l.f78b);let t101: f64 = (t100).abs();let t102: f64 = (tff * t101);let t103: f64 = (l.f74a * l.f78b);let t104: f64 = (t103).abs();let t105: f64 = (t102 * t104);let t106: f64 = (l.f74a * l.f78b);let t107: f64 = (t106).abs();let t108: f64 = (t105 * t107);l.f6fc = t108;}
        if (((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f263 == 0.0)) && (l.f265 != 0.0)) && (l.f267 == 0.0)) {let t109: f64 = (l.f74a * l.f78b);let t10a: f64 = (t109).abs();let t10b: f64 = (t10a).powf(l.f629);l.f6fc = t10b;}
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f263 == 0.0)) && (l.f265 != 0.0)) {let t10c: f64 = (1.0 - l.f6fc);let t10d: f64 = (1.0 / t10c);l.fae = t10d;}
        if ((((l.f29a != 0.0) && (l.f243 == 0.0)) && (l.f263 == 0.0)) && (l.f265 == 0.0)) {let t10e: f64 = (l.f2 * l.f78d);let t10f: f64 = (l.f74a + t10e);let t110: f64 = (t10f * l.f6be);let t111: f64 = (l.fc7 + t110);l.fae = t111;}
        if ((l.f29a != 0.0) && (l.f243 == 0.0)) {let t112: f64 = (l.f52f + l.f593);let t113: f64 = (t112 + l.f599);let t114: f64 = (t113 + l.f529);let t115: f64 = (t114 * l.fae);(l.f576, l.f577, l.f578, ) = (t115, (l.f530 * l.fae), (l.f531 * l.fae), );let t116: f64 = (l.f593 + l.f599);let t117: f64 = (t116 + l.f529);let t118: f64 = (t117 * l.fae);(l.f55a, l.f55b, l.f55c, ) = (t118, 0.0, 0.0, );}
        let t119: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f269 = t119;
        if ((l.f29a != 0.0) && (l.f269 != 0.0)) {(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_84(
        l: &mut StampLocals,
    ) {
        let t11a: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f26d = t11a;
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26d != 0.0)) {let t11b: f64 = (l.f796 * l.f76b);let t11c: f64 = (1.0 - t11b);let t11d: f64 = (t11c).sqrt();l.f6fc = t11d;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26d == 0.0)) {let t11e: f64 = (l.f796 * l.f76b);let t11f: f64 = (1.0 - t11e);let t120: f64 = (t11f).powf(l.f60d);l.f6fc = t120;}
        if ((l.f29a != 0.0) && (l.f269 == 0.0)) {let t121: f64 = (1.0 - l.f6fc);let t122: f64 = (l.f6a0 * t121);let t123: f64 = (l.f73b - l.f796);let t124: f64 = (l.f69a * t123);let t125: f64 = (t122 + t124);(l.f690, l.f691, l.f692, ) = (t125, 0.0, 0.0, );let t126: f64 = (l.f544 * l.f53a);(l.f52f, l.f530, l.f531, ) = (t126, (l.f544 * l.f53b), (l.f544 * l.f53c), );}
        let t127: f64 = if ((l.f3b == 0.0) && (l.f41 == 0.0)) { 1.0 } else { 0.0 };l.f26f = t127;
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26f != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26f == 0.0)) {let t128: f64 = (l.f763 - l.f7a2);l.f758 = t128;let t129: f64 = (l.f714 / l.f758);let t12a: f64 = (1.0 - t129);let t12b: f64 = (t12a).sqrt();let t12c: f64 = (1.0 - t12b);l.f7ef = t12c;}
        let t12d: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f271 = t12d;
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26f == 0.0)) && (l.f271 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26f == 0.0)) && (l.f271 == 0.0)) {let t12e: f64 = (l.f7ef * l.f7ef);let t12f: f64 = (l.f7ef).ln();let t130: f64 = (t12e * t12f);let t131: f64 = (1.0 - l.f7ef);let t132: f64 = (t130 / t131);let t133: f64 = (t132 + l.f7ef);let t134: f64 = (2.0 * l.f62f);let t135: f64 = (1.0 - t134);let t136: f64 = (t133 * t135);l.f66 = t136;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26f == 0.0)) {let t137: f64 = (l.f7ef + l.f66);l.f7e9 = t137;}
        let t138: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f273 = t138;
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26f == 0.0)) && (l.f273 != 0.0)) {let t139: f64 = (l.f758 * l.f777);let t13a: f64 = (t139).sqrt();l.f6fc = t13a;}
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26f == 0.0)) && (l.f273 == 0.0)) {let t13b: f64 = (l.f758 * l.f777);let t13c: f64 = (t13b).powf(l.f62f);l.f6fc = t13c;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f26f == 0.0)) {let t13d: f64 = (l.f7d8 * l.f6fc);l.f7d1 = t13d;let t13e: f64 = (l.f825 - 1.0);let t13f: f64 = (t13e * l.f7d1);let t140: f64 = (l.fcd * t13f);l.f9 = t140;let t141: f64 = (l.f9 * l.f7e9);let t142: f64 = (l.f3b * t141);l.f593 = t142;}
        let t143: f64 = if l.f41 == 0.0 { 1.0 } else { 0.0 };l.f275 = t143;
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) {let t144: f64 = (l.f7d1 * l.f60d);let t145: f64 = (t144 / l.f758);let t146: f64 = (l.f20 * t145);l.f19 = t146;let t147: f64 = (0.666666666666667 * l.f10);let t148: f64 = (t147 / l.f19);l.f71a = t148;let t149: f64 = (l.f71a * l.f71a);l.f72c = t149;let t14a: f64 = (l.f72c * l.f72c);let t14b: f64 = (l.f72c * l.f72c);let t14c: f64 = (t14b + 1.0);let t14d: f64 = (t14a / t14c);let t14e: f64 = (t14d).sqrt();l.f726 = t14e;let t14f: f64 = (l.f726).abs();let t150: f64 = (t14f).sqrt();l.f6c1 = t150;let t151: f64 = (l.f726 * l.f6c1);l.f732 = t151;}
        let t152: f64 = (-l.f62f);let t153: f64 = (t152 * l.f613);let t154: f64 = (-1.0);let t155: f64 = if t153 == t154 { 1.0 } else { 0.0 };l.f277 = t155;
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f277 != 0.0)) {let t156: f64 = (l.f19 * l.f732);let t157: f64 = (1.0 + t156);let t158: f64 = (1.0 / t157);l.f7e3 = t158;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_85(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f277 == 0.0)) {let t159: f64 = (l.f19 * l.f732);let t15a: f64 = (1.0 + t159);let t15b: f64 = (-l.f62f);let t15c: f64 = (t15b * l.f613);let t15d: f64 = (t15a).powf(t15c);l.f7e3 = t15d;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) {let t15e: f64 = (l.f7e9 * l.f7e3);let t15f: f64 = (l.f7e9 + l.f7e3);let t160: f64 = (t15e / t15f);l.f7f5 = t160;let t161: f64 = (l.f19 / l.f6c1);let t162: f64 = (0.375 * t161);let t163: f64 = (t162).sqrt();l.f5a8 = t163;let t164: f64 = (l.f71a * l.f6c1);let t165: f64 = (2.0 * t164);let t166: f64 = (t165 - l.f726);l.f5b4 = t166;let t167: f64 = (l.f10 * l.f71a);let t168: f64 = (t167 * l.f6c1);let t169: f64 = (l.f10 * l.f726);let t16a: f64 = (t168 - t169);let t16b: f64 = (l.f19 * l.f732);let t16c: f64 = (0.5 * t16b);let t16d: f64 = (t16a + t16c);l.f5d4 = t16d;let t16e: f64 = (l.f5b4 - 1.0);let t16f: f64 = (t16e * l.f5a8);l.f7fb = t16f;let t170: f64 = (l.f7fb * l.f7fb);l.f811 = t170;}
        let t171: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f279 = t171;
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f279 != 0.0)) {let t172: f64 = (l.f62b * l.f7fb);let t173: f64 = (1.0 + t172);let t174: f64 = (1.0 / t173);l.f6e2 = t174;}
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f279 == 0.0)) {let t175: f64 = (l.f62b * l.f7fb);let t176: f64 = (1.0 - t175);let t177: f64 = (1.0 / t176);l.f6e2 = t177;}
        let t178: f64 = (-l.f811);let t179: f64 = (t178 + l.f5d4);let t17a: f64 = (-230.25850929940458);let t17b: f64 = if t179 > t17a { 1.0 } else { 0.0 };l.f27b = t17b;
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f27b != 0.0)) {let t17c: f64 = (-l.f811);let t17d: f64 = (t17c + l.f5d4);let t17e: f64 = (t17d).exp();l.f6fc = t17e;}
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f27b == 0.0)) {let t17f: f64 = (-230.25850929940458);let t180: f64 = (-l.f811);let t181: f64 = (t180 + l.f5d4);let t182: f64 = (t17f - t181);let t183: f64 = (-230.25850929940458);let t184: f64 = (-l.f811);let t185: f64 = (t184 + l.f5d4);let t186: f64 = (t183 - t185);let t187: f64 = (-230.25850929940458);let t188: f64 = (-l.f811);let t189: f64 = (t188 + l.f5d4);let t18a: f64 = (t187 - t189);let t18b: f64 = (t18a * 0.3333333333333333);let t18c: f64 = (1.0 + t18b);let t18d: f64 = (t186 * t18c);let t18e: f64 = (0.5 * t18d);let t18f: f64 = (1.0 + t18e);let t190: f64 = (t182 * t18f);let t191: f64 = (1.0 + t190);let t192: f64 = (1e-100 / t191);l.f6fc = t192;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) {let t193: f64 = (0.29214664 * l.f6e2);let t194: f64 = (l.f6e2 * l.f6e2);let t195: f64 = (l.f16 * t194);let t196: f64 = (t193 + t195);let t197: f64 = (l.f6e2 * l.f6e2);let t198: f64 = (t197 * l.f6e2);let t199: f64 = (l.f2a * t198);let t19a: f64 = (t196 + t199);let t19b: f64 = (t19a * l.f6fc);l.f6e = t19b;}
        let t19c: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f27d = t19c;
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f27d != 0.0)) {l.f74 = l.f6e;}
        let t19d: f64 = (-230.25850929940458);let t19e: f64 = if l.f5d4 > t19d { 1.0 } else { 0.0 };l.f27f = t19e;
        if (((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f27d == 0.0)) && (l.f27f != 0.0)) {let t19f: f64 = (l.f5d4).exp();l.f6fc = t19f;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_86(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f27d == 0.0)) && (l.f27f == 0.0)) {let t1a0: f64 = (-230.25850929940458);let t1a1: f64 = (t1a0 - l.f5d4);let t1a2: f64 = (-230.25850929940458);let t1a3: f64 = (t1a2 - l.f5d4);let t1a4: f64 = (-230.25850929940458);let t1a5: f64 = (t1a4 - l.f5d4);let t1a6: f64 = (t1a5 * 0.3333333333333333);let t1a7: f64 = (1.0 + t1a6);let t1a8: f64 = (t1a3 * t1a7);let t1a9: f64 = (0.5 * t1a8);let t1aa: f64 = (1.0 + t1a9);let t1ab: f64 = (t1a1 * t1aa);let t1ac: f64 = (1.0 + t1ab);let t1ad: f64 = (1e-100 / t1ac);l.f6fc = t1ad;}
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) && (l.f27d == 0.0)) {let t1ae: f64 = (2.0 * l.f6fc);let t1af: f64 = (t1ae - l.f6e);l.f74 = t1af;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f275 == 0.0)) {let t1b0: f64 = (1.772453850905516 * 0.5);let t1b1: f64 = (l.f10 * l.f74);let t1b2: f64 = (t1b1 / l.f5a8);let t1b3: f64 = (t1b0 * t1b2);l.fd6 = t1b3;let t1b4: f64 = (l.f9 * l.fd6);let t1b5: f64 = (t1b4 * l.f7f5);let t1b6: f64 = (l.f41 * t1b5);l.f599 = t1b6;}
        let t1b7: f64 = if l.f26 == 0.0 { 1.0 } else { 0.0 };l.f285 = t1b7;
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f285 != 0.0)) {l.f529 = 0.0;}
        let t1b8: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f287 = t1b8;
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f285 == 0.0)) && (l.f287 != 0.0)) {let t1b9: f64 = (l.f775 - l.f750);let t1ba: f64 = (t1b9 * l.f777);let t1bb: f64 = (t1ba).sqrt();l.f6fc = t1bb;}
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f285 == 0.0)) && (l.f287 == 0.0)) {let t1bc: f64 = (l.f775 - l.f750);let t1bd: f64 = (t1bc * l.f777);let t1be: f64 = (t1bd).powf(l.f62f);l.f6fc = t1be;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f285 == 0.0)) {let t1bf: f64 = (l.f775 - l.f750);let t1c0: f64 = (t1bf * l.f7dc);let t1c1: f64 = (t1c0 / l.f6fc);let t1c2: f64 = (l.f613 * t1c1);l.fb6 = t1c2;}
        let t1c3: f64 = (-l.fa3);let t1c4: f64 = (t1c3 / l.fb6);let t1c5: f64 = (t1c4).abs();let t1c6: f64 = if t1c5 < 230.25850929940458 { 1.0 } else { 0.0 };l.f289 = t1c6;
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f285 == 0.0)) && (l.f289 != 0.0)) {let t1c7: f64 = (-l.fa3);let t1c8: f64 = (t1c7 / l.fb6);let t1c9: f64 = (t1c8).exp();l.f6fc = t1c9;}
        let t1ca: f64 = (-l.fa3);let t1cb: f64 = (t1ca / l.fb6);let t1cc: f64 = (-230.25850929940458);let t1cd: f64 = if t1cb < t1cc { 1.0 } else { 0.0 };l.f28b = t1cd;
        if (((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f285 == 0.0)) && (l.f289 == 0.0)) && (l.f28b != 0.0)) {let t1ce: f64 = (-230.25850929940458);let t1cf: f64 = (-l.fa3);let t1d0: f64 = (t1cf / l.fb6);let t1d1: f64 = (t1ce - t1d0);let t1d2: f64 = (-230.25850929940458);let t1d3: f64 = (-l.fa3);let t1d4: f64 = (t1d3 / l.fb6);let t1d5: f64 = (t1d2 - t1d4);let t1d6: f64 = (-230.25850929940458);let t1d7: f64 = (-l.fa3);let t1d8: f64 = (t1d7 / l.fb6);let t1d9: f64 = (t1d6 - t1d8);let t1da: f64 = (t1d9 * 0.3333333333333333);let t1db: f64 = (1.0 + t1da);let t1dc: f64 = (t1d5 * t1db);let t1dd: f64 = (0.5 * t1dc);let t1de: f64 = (1.0 + t1dd);let t1df: f64 = (t1d1 * t1de);let t1e0: f64 = (1.0 + t1df);let t1e1: f64 = (1e-100 / t1e0);l.f6fc = t1e1;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_87(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f285 == 0.0)) && (l.f289 == 0.0)) && (l.f28b == 0.0)) {let t1e2: f64 = (-l.fa3);let t1e3: f64 = (t1e2 / l.fb6);let t1e4: f64 = (t1e3 - 230.25850929940458);let t1e5: f64 = (-l.fa3);let t1e6: f64 = (t1e5 / l.fb6);let t1e7: f64 = (t1e6 - 230.25850929940458);let t1e8: f64 = (-l.fa3);let t1e9: f64 = (t1e8 / l.fb6);let t1ea: f64 = (t1e9 - 230.25850929940458);let t1eb: f64 = (t1ea * 0.3333333333333333);let t1ec: f64 = (1.0 + t1eb);let t1ed: f64 = (t1e7 * t1ec);let t1ee: f64 = (0.5 * t1ed);let t1ef: f64 = (1.0 + t1ee);let t1f0: f64 = (t1e4 * t1ef);let t1f1: f64 = (1.0 + t1f0);let t1f2: f64 = (1e100 * t1f1);l.f6fc = t1f2;}
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f285 == 0.0)) {let t1f3: f64 = (l.f73b * l.fb6);let t1f4: f64 = (t1f3 * l.fb6);let t1f5: f64 = (t1f4 * l.f6fc);let t1f6: f64 = (l.f26 * t1f5);l.f529 = t1f6;}
        let t1f7: f64 = if ((l.f785 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f28d = t1f7;
        if (((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f28d != 0.0)) {l.fae = 1.0;}
        let t1f8: f64 = (-l.f2);let t1f9: f64 = (t1f8 * l.f785);let t1fa: f64 = if l.f74a > t1f9 { 1.0 } else { 0.0 };l.f28f = t1fa;let t1fb: f64 = if l.f627 == 4.0 { 1.0 } else { 0.0 };l.f291 = t1fb;
        if (((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f28d == 0.0)) && (l.f28f != 0.0)) && (l.f291 != 0.0)) {let t1fc: f64 = (l.f74a * l.f789);let t1fd: f64 = (t1fc).abs();let t1fe: f64 = (l.f74a * l.f789);let t1ff: f64 = (t1fe).abs();let t200: f64 = (t1fd * t1ff);let t201: f64 = (l.f74a * l.f789);let t202: f64 = (t201).abs();let t203: f64 = (t200 * t202);let t204: f64 = (l.f74a * l.f789);let t205: f64 = (t204).abs();let t206: f64 = (t203 * t205);l.f6fc = t206;}
        if (((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f28d == 0.0)) && (l.f28f != 0.0)) && (l.f291 == 0.0)) {let t207: f64 = (l.f74a * l.f789);let t208: f64 = (t207).abs();let t209: f64 = (t208).powf(l.f627);l.f6fc = t209;}
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f28d == 0.0)) && (l.f28f != 0.0)) {let t20a: f64 = (1.0 - l.f6fc);let t20b: f64 = (1.0 / t20a);l.fae = t20b;}
        if ((((l.f29a != 0.0) && (l.f269 == 0.0)) && (l.f28d == 0.0)) && (l.f28f == 0.0)) {let t20c: f64 = (l.f2 * l.f785);let t20d: f64 = (l.f74a + t20c);let t20e: f64 = (t20d * l.f6bc);let t20f: f64 = (l.fc5 + t20e);l.fae = t20f;}
        if ((l.f29a != 0.0) && (l.f269 == 0.0)) {let t210: f64 = (l.f52f + l.f593);let t211: f64 = (t210 + l.f599);let t212: f64 = (t211 + l.f529);let t213: f64 = (t212 * l.fae);(l.f56e, l.f56f, l.f570, ) = (t213, (l.f530 * l.fae), (l.f531 * l.fae), );let t214: f64 = (l.f593 + l.f599);let t215: f64 = (t214 + l.f529);let t216: f64 = (t215 * l.fae);(l.f556, l.f557, l.f558, ) = (t216, 0.0, 0.0, );}
        if (l.f29a != 0.0) {let t217: f64 = (l.f0 * l.f562);let t218: f64 = (l.f5b1 * l.f576);let t219: f64 = (t217 + t218);let t21a: f64 = (l.f5af * l.f56e);let t21b: f64 = (t219 + t21a);(l.f510, l.f515, l.f516, ) = (t21b, (((l.f0 * l.f563) + (l.f5b1 * l.f577)) + (l.f5af * l.f56f)), (((l.f0 * l.f564) + (l.f5b1 * l.f578)) + (l.f5af * l.f570)), );}
        let t21c: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f293 = t21c;
        if ((l.f29a != 0.0) && (l.f293 != 0.0)) {let t21d: f64 = (4.0 * l.f78f);let t21e: f64 = (t21d * l.f78f);l.f4e1 = t21e;let t21f: f64 = (l.f78f / l.f791);l.f4e5 = t21f;let t220: f64 = (l.f78f * l.f4e5);let t221: f64 = (l.f73d + t220);l.f4e9 = t221;let t222: f64 = (l.f791 + l.f4e9);l.f4ef = t222;let t223: f64 = (l.f791 - l.f4e9);l.f4f5 = t223;let t224: f64 = (l.f4f5 * l.f4f5);let t225: f64 = (t224 + l.f4e1);let t226: f64 = (t225).sqrt();l.f4fb = t226;let t227: f64 = (l.f73d * l.f791);let t228: f64 = (l.f4ef + l.f4fb);let t229: f64 = (t227 / t228);let t22a: f64 = (2.0 * t229);l.f796 = t22a;}
        let t22b: f64 = if l.f73d < l.f7b1 { 1.0 } else { 0.0 };l.f295 = t22b;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_88(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t22c: f64 = (l.f73d * l.f645);let t22d: f64 = (0.5 * t22c);let t22e: f64 = (t22d).abs();let t22f: f64 = if t22e < 230.25850929940458 { 1.0 } else { 0.0 };l.f297 = t22f;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f297 != 0.0)) {let t230: f64 = (l.f73d * l.f645);let t231: f64 = (0.5 * t230);let t232: f64 = (t231).exp();l.f825 = t232;}
        let t233: f64 = (l.f73d * l.f645);let t234: f64 = (0.5 * t233);let t235: f64 = (-230.25850929940458);let t236: f64 = if t234 < t235 { 1.0 } else { 0.0 };l.f29b = t236;
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f297 == 0.0)) && (l.f29b != 0.0)) {let t237: f64 = (-230.25850929940458);let t238: f64 = (l.f73d * l.f645);let t239: f64 = (0.5 * t238);let t23a: f64 = (t237 - t239);let t23b: f64 = (-230.25850929940458);let t23c: f64 = (l.f73d * l.f645);let t23d: f64 = (0.5 * t23c);let t23e: f64 = (t23b - t23d);let t23f: f64 = (-230.25850929940458);let t240: f64 = (l.f73d * l.f645);let t241: f64 = (0.5 * t240);let t242: f64 = (t23f - t241);let t243: f64 = (t242 * 0.3333333333333333);let t244: f64 = (1.0 + t243);let t245: f64 = (t23e * t244);let t246: f64 = (0.5 * t245);let t247: f64 = (1.0 + t246);let t248: f64 = (t23a * t247);let t249: f64 = (1.0 + t248);let t24a: f64 = (1e-100 / t249);l.f825 = t24a;}
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f297 == 0.0)) && (l.f29b == 0.0)) {let t24b: f64 = (l.f73d * l.f645);let t24c: f64 = (0.5 * t24b);let t24d: f64 = (t24c - 230.25850929940458);let t24e: f64 = (l.f73d * l.f645);let t24f: f64 = (0.5 * t24e);let t250: f64 = (t24f - 230.25850929940458);let t251: f64 = (l.f73d * l.f645);let t252: f64 = (0.5 * t251);let t253: f64 = (t252 - 230.25850929940458);let t254: f64 = (t253 * 0.3333333333333333);let t255: f64 = (1.0 + t254);let t256: f64 = (t250 * t255);let t257: f64 = (0.5 * t256);let t258: f64 = (1.0 + t257);let t259: f64 = (t24d * t258);let t25a: f64 = (1.0 + t259);let t25b: f64 = (1e100 * t25a);l.f825 = t25b;}
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) {let t25c: f64 = (l.f5eb * l.f5eb);let t25d: f64 = (t25c / l.f5df);l.f64f = t25d;let t25e: f64 = (l.f5e5 / l.f645);let t25f: f64 = (l.f5df / l.f64f);let t260: f64 = (t25f).ln();let t261: f64 = (t25e * t260);l.f793 = t261;}
        let t262: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f29d = t262;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {let t263: f64 = (l.f73d - l.f793);let t264: f64 = (p.p86 * t263);let t265: f64 = (t264 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t265, 0.0, 0.0, );let t266: f64 = (p.p86 * l.f793);let t267: f64 = (l.f5e5 - t266);(l.f5ed, l.f5ee, l.f5ef, ) = (t267, 0.0, 0.0, );let t268: f64 = (p.p85 - l.f601);let t269: f64 = (t268 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t269, (-l.f602), (-l.f603), );let t26a: f64 = (4.0 * p.p85);let t26b: f64 = (t26a * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t26b, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {
            let (t26d, t26e, t26f,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t26c: f64 = (-l.f6f7);
        (t26c, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t26d, t26e, t26f, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {let t270: f64 = (l.f6f3 * l.f6f3);let t271: f64 = (t270 + l.f6f7);let t272: f64 = (t271).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t272, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t272)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t272)), );let t273: f64 = (l.f6f3 + l.f6f7);let t274: f64 = (0.5 * t273);let t275: f64 = (p.p85 - t274);(l.f605, l.f606, l.f607, ) = (t275, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t276: f64 = (l.f605 - l.f5e5);let t277: f64 = (t276 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t277, l.f606, l.f607, );let t278: f64 = (4.0 * l.f5e5);let t279: f64 = (t278 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t279, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {
            let (t27b, t27c, t27d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t27a: f64 = (-l.f6f7);
        (t27a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t27b, t27c, t27d, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_89(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {let t27e: f64 = (l.f6f3 * l.f6f3);let t27f: f64 = (t27e + l.f6f7);let t280: f64 = (t27f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t280, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t280)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t280)), );let t281: f64 = (l.f6f3 + l.f6f7);let t282: f64 = (0.5 * t281);let t283: f64 = (l.f5e5 + t282);(l.f5f1, l.f5f2, l.f5f3, ) = (t283, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t284: f64 = (p.p85 - l.f5ed);let t285: f64 = (t284 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t285, (-l.f5ee), (-l.f5ef), );let t286: f64 = (4.0 * p.p85);let t287: f64 = (t286 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t287, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {
            let (t289, t28a, t28b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t288: f64 = (-l.f6f7);
        (t288, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t289, t28a, t28b, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {let t28c: f64 = (l.f6f3 * l.f6f3);let t28d: f64 = (t28c + l.f6f7);let t28e: f64 = (t28d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t28e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t28e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t28e)), );let t28f: f64 = (l.f6f3 + l.f6f7);let t290: f64 = (0.5 * t28f);let t291: f64 = (p.p85 - t290);(l.f5ed, l.f5ee, l.f5ef, ) = (t291, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t292: f64 = (l.f5ed - l.f5e5);let t293: f64 = (t292 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t293, l.f5ee, l.f5ef, );let t294: f64 = (4.0 * l.f5e5);let t295: f64 = (t294 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t295, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {
            let (t297, t298, t299,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t296: f64 = (-l.f6f7);
        (t296, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t297, t298, t299, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d != 0.0)) {let t29a: f64 = (l.f6f3 * l.f6f3);let t29b: f64 = (t29a + l.f6f7);let t29c: f64 = (t29b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t29c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t29c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t29c)), );let t29d: f64 = (l.f6f3 + l.f6f7);let t29e: f64 = (0.5 * t29d);let t29f: f64 = (l.f5e5 + t29e);(l.f5ed, l.f5ee, l.f5ef, ) = (t29f, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29d == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );}
        let t2a0: f64 = (l.f73d / l.f5f1);let t2a1: f64 = (l.f5f1 - l.f5ed);let t2a2: f64 = (l.f793 * t2a1);let t2a3: f64 = (l.f5ed * p.p85);let t2a4: f64 = (t2a2 / t2a3);let t2a5: f64 = (t2a0 + t2a4);let t2a6: f64 = (l.f645 * t2a5);let t2a7: f64 = (t2a6).abs();let t2a8: f64 = if t2a7 < 230.25850929940458 { 1.0 } else { 0.0 };l.f29f = t2a8;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29f != 0.0)) {let t2a9: f64 = (l.f73d / l.f5f1);let t2aa: f64 = (l.f5f1 - l.f5ed);let t2ab: f64 = (l.f793 * t2aa);let t2ac: f64 = (l.f5ed * p.p85);let t2ad: f64 = (t2ab / t2ac);let t2ae: f64 = (t2a9 + t2ad);let t2af: f64 = (l.f645 * t2ae);let t2b0: f64 = (t2af).exp();(l.f536, l.f537, l.f538, ) = (t2b0, (t2b0 * (l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2ac) - (t2ab * (l.f5ee * p.p85))) / (t2ac * t2ac))))), (t2b0 * (l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2ac) - (t2ab * (l.f5ef * p.p85))) / (t2ac * t2ac))))), );}
        let t2b1: f64 = (l.f73d / l.f5f1);let t2b2: f64 = (l.f5f1 - l.f5ed);let t2b3: f64 = (l.f793 * t2b2);let t2b4: f64 = (l.f5ed * p.p85);let t2b5: f64 = (t2b3 / t2b4);let t2b6: f64 = (t2b1 + t2b5);let t2b7: f64 = (l.f645 * t2b6);let t2b8: f64 = (-230.25850929940458);let t2b9: f64 = if t2b7 < t2b8 { 1.0 } else { 0.0 };l.f2a1 = t2b9;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_90(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29f == 0.0)) && (l.f2a1 != 0.0)) {let t2ba: f64 = (-230.25850929940458);let t2bb: f64 = (l.f73d / l.f5f1);let t2bc: f64 = (l.f5f1 - l.f5ed);let t2bd: f64 = (l.f793 * t2bc);let t2be: f64 = (l.f5ed * p.p85);let t2bf: f64 = (t2bd / t2be);let t2c0: f64 = (t2bb + t2bf);let t2c1: f64 = (l.f645 * t2c0);let t2c2: f64 = (t2ba - t2c1);let t2c3: f64 = (-230.25850929940458);let t2c4: f64 = (l.f73d / l.f5f1);let t2c5: f64 = (l.f5f1 - l.f5ed);let t2c6: f64 = (l.f793 * t2c5);let t2c7: f64 = (l.f5ed * p.p85);let t2c8: f64 = (t2c6 / t2c7);let t2c9: f64 = (t2c4 + t2c8);let t2ca: f64 = (l.f645 * t2c9);let t2cb: f64 = (t2c3 - t2ca);let t2cc: f64 = (-230.25850929940458);let t2cd: f64 = (l.f73d / l.f5f1);let t2ce: f64 = (l.f5f1 - l.f5ed);let t2cf: f64 = (l.f793 * t2ce);let t2d0: f64 = (l.f5ed * p.p85);let t2d1: f64 = (t2cf / t2d0);let t2d2: f64 = (t2cd + t2d1);let t2d3: f64 = (l.f645 * t2d2);let t2d4: f64 = (t2cc - t2d3);let t2d5: f64 = (t2d4 * 0.3333333333333333);let t2d6: f64 = (1.0 + t2d5);let t2d7: f64 = (t2cb * t2d6);let t2d8: f64 = (0.5 * t2d7);let t2d9: f64 = (1.0 + t2d8);let t2da: f64 = (t2c2 * t2d9);let t2db: f64 = (1.0 + t2da);let t2dc: f64 = (1e-100 / t2db);(l.f536, l.f537, l.f538, ) = (t2dc, (-((1e-100 * (((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2be) - (t2bd * (l.f5ee * p.p85))) / (t2be * t2be))))) * t2d9) + (t2c2 * (0.5 * (((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2c7) - (t2c6 * (l.f5ee * p.p85))) / (t2c7 * t2c7))))) * t2d6) + (t2cb * ((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2d0) - (t2cf * (l.f5ee * p.p85))) / (t2d0 * t2d0))))) * 0.3333333333333333))))))) / (t2db * t2db))), (-((1e-100 * (((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2be) - (t2bd * (l.f5ef * p.p85))) / (t2be * t2be))))) * t2d9) + (t2c2 * (0.5 * (((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2c7) - (t2c6 * (l.f5ef * p.p85))) / (t2c7 * t2c7))))) * t2d6) + (t2cb * ((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2d0) - (t2cf * (l.f5ef * p.p85))) / (t2d0 * t2d0))))) * 0.3333333333333333))))))) / (t2db * t2db))), );}
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f29f == 0.0)) && (l.f2a1 == 0.0)) {let t2dd: f64 = (l.f73d / l.f5f1);let t2de: f64 = (l.f5f1 - l.f5ed);let t2df: f64 = (l.f793 * t2de);let t2e0: f64 = (l.f5ed * p.p85);let t2e1: f64 = (t2df / t2e0);let t2e2: f64 = (t2dd + t2e1);let t2e3: f64 = (l.f645 * t2e2);let t2e4: f64 = (t2e3 - 230.25850929940458);let t2e5: f64 = (l.f73d / l.f5f1);let t2e6: f64 = (l.f5f1 - l.f5ed);let t2e7: f64 = (l.f793 * t2e6);let t2e8: f64 = (l.f5ed * p.p85);let t2e9: f64 = (t2e7 / t2e8);let t2ea: f64 = (t2e5 + t2e9);let t2eb: f64 = (l.f645 * t2ea);let t2ec: f64 = (t2eb - 230.25850929940458);let t2ed: f64 = (l.f73d / l.f5f1);let t2ee: f64 = (l.f5f1 - l.f5ed);let t2ef: f64 = (l.f793 * t2ee);let t2f0: f64 = (l.f5ed * p.p85);let t2f1: f64 = (t2ef / t2f0);let t2f2: f64 = (t2ed + t2f1);let t2f3: f64 = (l.f645 * t2f2);let t2f4: f64 = (t2f3 - 230.25850929940458);let t2f5: f64 = (t2f4 * 0.3333333333333333);let t2f6: f64 = (1.0 + t2f5);let t2f7: f64 = (t2ec * t2f6);let t2f8: f64 = (0.5 * t2f7);let t2f9: f64 = (1.0 + t2f8);let t2fa: f64 = (t2e4 * t2f9);let t2fb: f64 = (1.0 + t2fa);let t2fc: f64 = (1e100 * t2fb);(l.f536, l.f537, l.f538, ) = (t2fc, (1e100 * (((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2e0) - (t2df * (l.f5ee * p.p85))) / (t2e0 * t2e0)))) * t2f9) + (t2e4 * (0.5 * (((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2e8) - (t2e7 * (l.f5ee * p.p85))) / (t2e8 * t2e8)))) * t2f6) + (t2ec * ((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2f0) - (t2ef * (l.f5ee * p.p85))) / (t2f0 * t2f0)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2e0) - (t2df * (l.f5ef * p.p85))) / (t2e0 * t2e0)))) * t2f9) + (t2e4 * (0.5 * (((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2e8) - (t2e7 * (l.f5ef * p.p85))) / (t2e8 * t2e8)))) * t2f6) + (t2ec * ((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2f0) - (t2ef * (l.f5ef * p.p85))) / (t2f0 * t2f0)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) {let t2fd: f64 = (l.f5eb * l.f5eb);let t2fe: f64 = (t2fd / l.f5e3);l.f64f = t2fe;let t2ff: f64 = (l.f5e9 / l.f645);let t300: f64 = (l.f5e3 / l.f64f);let t301: f64 = (t300).ln();let t302: f64 = (t2ff * t301);l.f793 = t302;}
        let t303: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f2a3 = t303;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {let t304: f64 = (l.f73d - l.f793);let t305: f64 = (p.p86 * t304);let t306: f64 = (t305 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t306, 0.0, 0.0, );let t307: f64 = (p.p86 * l.f793);let t308: f64 = (l.f5e9 - t307);(l.f5ed, l.f5ee, l.f5ef, ) = (t308, 0.0, 0.0, );let t309: f64 = (p.p85 - l.f601);let t30a: f64 = (t309 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t30a, (-l.f602), (-l.f603), );let t30b: f64 = (4.0 * p.p85);let t30c: f64 = (t30b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t30c, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {
            let (t30e, t30f, t310,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t30d: f64 = (-l.f6f7);
        (t30d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t30e, t30f, t310, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_91(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {let t311: f64 = (l.f6f3 * l.f6f3);let t312: f64 = (t311 + l.f6f7);let t313: f64 = (t312).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t313, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t313)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t313)), );let t314: f64 = (l.f6f3 + l.f6f7);let t315: f64 = (0.5 * t314);let t316: f64 = (p.p85 - t315);(l.f605, l.f606, l.f607, ) = (t316, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t317: f64 = (l.f605 - l.f5e9);let t318: f64 = (t317 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t318, l.f606, l.f607, );let t319: f64 = (4.0 * l.f5e9);let t31a: f64 = (t319 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t31a, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {
            let (t31c, t31d, t31e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t31b: f64 = (-l.f6f7);
        (t31b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t31c, t31d, t31e, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {let t31f: f64 = (l.f6f3 * l.f6f3);let t320: f64 = (t31f + l.f6f7);let t321: f64 = (t320).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t321, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t321)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t321)), );let t322: f64 = (l.f6f3 + l.f6f7);let t323: f64 = (0.5 * t322);let t324: f64 = (l.f5e9 + t323);(l.f5f1, l.f5f2, l.f5f3, ) = (t324, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t325: f64 = (p.p85 - l.f5ed);let t326: f64 = (t325 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t326, (-l.f5ee), (-l.f5ef), );let t327: f64 = (4.0 * p.p85);let t328: f64 = (t327 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t328, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {
            let (t32a, t32b, t32c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t329: f64 = (-l.f6f7);
        (t329, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t32a, t32b, t32c, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {let t32d: f64 = (l.f6f3 * l.f6f3);let t32e: f64 = (t32d + l.f6f7);let t32f: f64 = (t32e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t32f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t32f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t32f)), );let t330: f64 = (l.f6f3 + l.f6f7);let t331: f64 = (0.5 * t330);let t332: f64 = (p.p85 - t331);(l.f5ed, l.f5ee, l.f5ef, ) = (t332, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t333: f64 = (l.f5ed - l.f5e9);let t334: f64 = (t333 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t334, l.f5ee, l.f5ef, );let t335: f64 = (4.0 * l.f5e9);let t336: f64 = (t335 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t336, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {
            let (t338, t339, t33a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t337: f64 = (-l.f6f7);
        (t337, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t338, t339, t33a, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 != 0.0)) {let t33b: f64 = (l.f6f3 * l.f6f3);let t33c: f64 = (t33b + l.f6f7);let t33d: f64 = (t33c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t33d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t33d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t33d)), );let t33e: f64 = (l.f6f3 + l.f6f7);let t33f: f64 = (0.5 * t33e);let t340: f64 = (l.f5e9 + t33f);(l.f5ed, l.f5ee, l.f5ef, ) = (t340, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a3 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );}
        let t341: f64 = (l.f73d / l.f5f1);let t342: f64 = (l.f5f1 - l.f5ed);let t343: f64 = (l.f793 * t342);let t344: f64 = (l.f5ed * p.p85);let t345: f64 = (t343 / t344);let t346: f64 = (t341 + t345);let t347: f64 = (l.f645 * t346);let t348: f64 = (t347).abs();let t349: f64 = if t348 < 230.25850929940458 { 1.0 } else { 0.0 };l.f2a5 = t349;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a5 != 0.0)) {let t34a: f64 = (l.f73d / l.f5f1);let t34b: f64 = (l.f5f1 - l.f5ed);let t34c: f64 = (l.f793 * t34b);let t34d: f64 = (l.f5ed * p.p85);let t34e: f64 = (t34c / t34d);let t34f: f64 = (t34a + t34e);let t350: f64 = (l.f645 * t34f);let t351: f64 = (t350).exp();(l.f53e, l.f53f, l.f540, ) = (t351, (t351 * (l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t34d) - (t34c * (l.f5ee * p.p85))) / (t34d * t34d))))), (t351 * (l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t34d) - (t34c * (l.f5ef * p.p85))) / (t34d * t34d))))), );}
        let t352: f64 = (l.f73d / l.f5f1);let t353: f64 = (l.f5f1 - l.f5ed);let t354: f64 = (l.f793 * t353);let t355: f64 = (l.f5ed * p.p85);let t356: f64 = (t354 / t355);let t357: f64 = (t352 + t356);let t358: f64 = (l.f645 * t357);let t359: f64 = (-230.25850929940458);let t35a: f64 = if t358 < t359 { 1.0 } else { 0.0 };l.f2a7 = t35a;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_92(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a5 == 0.0)) && (l.f2a7 != 0.0)) {let t35b: f64 = (-230.25850929940458);let t35c: f64 = (l.f73d / l.f5f1);let t35d: f64 = (l.f5f1 - l.f5ed);let t35e: f64 = (l.f793 * t35d);let t35f: f64 = (l.f5ed * p.p85);let t360: f64 = (t35e / t35f);let t361: f64 = (t35c + t360);let t362: f64 = (l.f645 * t361);let t363: f64 = (t35b - t362);let t364: f64 = (-230.25850929940458);let t365: f64 = (l.f73d / l.f5f1);let t366: f64 = (l.f5f1 - l.f5ed);let t367: f64 = (l.f793 * t366);let t368: f64 = (l.f5ed * p.p85);let t369: f64 = (t367 / t368);let t36a: f64 = (t365 + t369);let t36b: f64 = (l.f645 * t36a);let t36c: f64 = (t364 - t36b);let t36d: f64 = (-230.25850929940458);let t36e: f64 = (l.f73d / l.f5f1);let t36f: f64 = (l.f5f1 - l.f5ed);let t370: f64 = (l.f793 * t36f);let t371: f64 = (l.f5ed * p.p85);let t372: f64 = (t370 / t371);let t373: f64 = (t36e + t372);let t374: f64 = (l.f645 * t373);let t375: f64 = (t36d - t374);let t376: f64 = (t375 * 0.3333333333333333);let t377: f64 = (1.0 + t376);let t378: f64 = (t36c * t377);let t379: f64 = (0.5 * t378);let t37a: f64 = (1.0 + t379);let t37b: f64 = (t363 * t37a);let t37c: f64 = (1.0 + t37b);let t37d: f64 = (1e-100 / t37c);(l.f53e, l.f53f, l.f540, ) = (t37d, (-((1e-100 * (((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t35f) - (t35e * (l.f5ee * p.p85))) / (t35f * t35f))))) * t37a) + (t363 * (0.5 * (((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t368) - (t367 * (l.f5ee * p.p85))) / (t368 * t368))))) * t377) + (t36c * ((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t371) - (t370 * (l.f5ee * p.p85))) / (t371 * t371))))) * 0.3333333333333333))))))) / (t37c * t37c))), (-((1e-100 * (((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t35f) - (t35e * (l.f5ef * p.p85))) / (t35f * t35f))))) * t37a) + (t363 * (0.5 * (((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t368) - (t367 * (l.f5ef * p.p85))) / (t368 * t368))))) * t377) + (t36c * ((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t371) - (t370 * (l.f5ef * p.p85))) / (t371 * t371))))) * 0.3333333333333333))))))) / (t37c * t37c))), );}
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a5 == 0.0)) && (l.f2a7 == 0.0)) {let t37e: f64 = (l.f73d / l.f5f1);let t37f: f64 = (l.f5f1 - l.f5ed);let t380: f64 = (l.f793 * t37f);let t381: f64 = (l.f5ed * p.p85);let t382: f64 = (t380 / t381);let t383: f64 = (t37e + t382);let t384: f64 = (l.f645 * t383);let t385: f64 = (t384 - 230.25850929940458);let t386: f64 = (l.f73d / l.f5f1);let t387: f64 = (l.f5f1 - l.f5ed);let t388: f64 = (l.f793 * t387);let t389: f64 = (l.f5ed * p.p85);let t38a: f64 = (t388 / t389);let t38b: f64 = (t386 + t38a);let t38c: f64 = (l.f645 * t38b);let t38d: f64 = (t38c - 230.25850929940458);let t38e: f64 = (l.f73d / l.f5f1);let t38f: f64 = (l.f5f1 - l.f5ed);let t390: f64 = (l.f793 * t38f);let t391: f64 = (l.f5ed * p.p85);let t392: f64 = (t390 / t391);let t393: f64 = (t38e + t392);let t394: f64 = (l.f645 * t393);let t395: f64 = (t394 - 230.25850929940458);let t396: f64 = (t395 * 0.3333333333333333);let t397: f64 = (1.0 + t396);let t398: f64 = (t38d * t397);let t399: f64 = (0.5 * t398);let t39a: f64 = (1.0 + t399);let t39b: f64 = (t385 * t39a);let t39c: f64 = (1.0 + t39b);let t39d: f64 = (1e100 * t39c);(l.f53e, l.f53f, l.f540, ) = (t39d, (1e100 * (((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t381) - (t380 * (l.f5ee * p.p85))) / (t381 * t381)))) * t39a) + (t385 * (0.5 * (((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t389) - (t388 * (l.f5ee * p.p85))) / (t389 * t389)))) * t397) + (t38d * ((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t391) - (t390 * (l.f5ee * p.p85))) / (t391 * t391)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t381) - (t380 * (l.f5ef * p.p85))) / (t381 * t381)))) * t39a) + (t385 * (0.5 * (((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t389) - (t388 * (l.f5ef * p.p85))) / (t389 * t389)))) * t397) + (t38d * ((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t391) - (t390 * (l.f5ef * p.p85))) / (t391 * t391)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) {let t39e: f64 = (l.f5eb * l.f5eb);let t39f: f64 = (t39e / l.f5e1);l.f64f = t39f;let t3a0: f64 = (l.f5e7 / l.f645);let t3a1: f64 = (l.f5e1 / l.f64f);let t3a2: f64 = (t3a1).ln();let t3a3: f64 = (t3a0 * t3a2);l.f793 = t3a3;}
        let t3a4: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f2a9 = t3a4;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {let t3a5: f64 = (l.f73d - l.f793);let t3a6: f64 = (p.p86 * t3a5);let t3a7: f64 = (t3a6 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t3a7, 0.0, 0.0, );let t3a8: f64 = (p.p86 * l.f793);let t3a9: f64 = (l.f5e7 - t3a8);(l.f5ed, l.f5ee, l.f5ef, ) = (t3a9, 0.0, 0.0, );let t3aa: f64 = (p.p85 - l.f601);let t3ab: f64 = (t3aa - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3ab, (-l.f602), (-l.f603), );let t3ac: f64 = (4.0 * p.p85);let t3ad: f64 = (t3ac * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3ad, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {
            let (t3af, t3b0, t3b1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3ae: f64 = (-l.f6f7);
        (t3ae, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3af, t3b0, t3b1, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_93(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {let t3b2: f64 = (l.f6f3 * l.f6f3);let t3b3: f64 = (t3b2 + l.f6f7);let t3b4: f64 = (t3b3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3b4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3b4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3b4)), );let t3b5: f64 = (l.f6f3 + l.f6f7);let t3b6: f64 = (0.5 * t3b5);let t3b7: f64 = (p.p85 - t3b6);(l.f605, l.f606, l.f607, ) = (t3b7, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t3b8: f64 = (l.f605 - l.f5e7);let t3b9: f64 = (t3b8 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3b9, l.f606, l.f607, );let t3ba: f64 = (4.0 * l.f5e7);let t3bb: f64 = (t3ba * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3bb, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {
            let (t3bd, t3be, t3bf,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3bc: f64 = (-l.f6f7);
        (t3bc, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3bd, t3be, t3bf, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {let t3c0: f64 = (l.f6f3 * l.f6f3);let t3c1: f64 = (t3c0 + l.f6f7);let t3c2: f64 = (t3c1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3c2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3c2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3c2)), );let t3c3: f64 = (l.f6f3 + l.f6f7);let t3c4: f64 = (0.5 * t3c3);let t3c5: f64 = (l.f5e7 + t3c4);(l.f5f1, l.f5f2, l.f5f3, ) = (t3c5, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t3c6: f64 = (p.p85 - l.f5ed);let t3c7: f64 = (t3c6 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3c7, (-l.f5ee), (-l.f5ef), );let t3c8: f64 = (4.0 * p.p85);let t3c9: f64 = (t3c8 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3c9, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {
            let (t3cb, t3cc, t3cd,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3ca: f64 = (-l.f6f7);
        (t3ca, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3cb, t3cc, t3cd, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {let t3ce: f64 = (l.f6f3 * l.f6f3);let t3cf: f64 = (t3ce + l.f6f7);let t3d0: f64 = (t3cf).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3d0, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3d0)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3d0)), );let t3d1: f64 = (l.f6f3 + l.f6f7);let t3d2: f64 = (0.5 * t3d1);let t3d3: f64 = (p.p85 - t3d2);(l.f5ed, l.f5ee, l.f5ef, ) = (t3d3, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t3d4: f64 = (l.f5ed - l.f5e7);let t3d5: f64 = (t3d4 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3d5, l.f5ee, l.f5ef, );let t3d6: f64 = (4.0 * l.f5e7);let t3d7: f64 = (t3d6 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3d7, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {
            let (t3d9, t3da, t3db,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3d8: f64 = (-l.f6f7);
        (t3d8, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3d9, t3da, t3db, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 != 0.0)) {let t3dc: f64 = (l.f6f3 * l.f6f3);let t3dd: f64 = (t3dc + l.f6f7);let t3de: f64 = (t3dd).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3de, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3de)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3de)), );let t3df: f64 = (l.f6f3 + l.f6f7);let t3e0: f64 = (0.5 * t3df);let t3e1: f64 = (l.f5e7 + t3e0);(l.f5ed, l.f5ee, l.f5ef, ) = (t3e1, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2a9 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );}
        let t3e2: f64 = (l.f73d / l.f5f1);let t3e3: f64 = (l.f5f1 - l.f5ed);let t3e4: f64 = (l.f793 * t3e3);let t3e5: f64 = (l.f5ed * p.p85);let t3e6: f64 = (t3e4 / t3e5);let t3e7: f64 = (t3e2 + t3e6);let t3e8: f64 = (l.f645 * t3e7);let t3e9: f64 = (t3e8).abs();let t3ea: f64 = if t3e9 < 230.25850929940458 { 1.0 } else { 0.0 };l.f2ab = t3ea;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2ab != 0.0)) {let t3eb: f64 = (l.f73d / l.f5f1);let t3ec: f64 = (l.f5f1 - l.f5ed);let t3ed: f64 = (l.f793 * t3ec);let t3ee: f64 = (l.f5ed * p.p85);let t3ef: f64 = (t3ed / t3ee);let t3f0: f64 = (t3eb + t3ef);let t3f1: f64 = (l.f645 * t3f0);let t3f2: f64 = (t3f1).exp();(l.f53a, l.f53b, l.f53c, ) = (t3f2, (t3f2 * (l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3ee) - (t3ed * (l.f5ee * p.p85))) / (t3ee * t3ee))))), (t3f2 * (l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3ee) - (t3ed * (l.f5ef * p.p85))) / (t3ee * t3ee))))), );}
        let t3f3: f64 = (l.f73d / l.f5f1);let t3f4: f64 = (l.f5f1 - l.f5ed);let t3f5: f64 = (l.f793 * t3f4);let t3f6: f64 = (l.f5ed * p.p85);let t3f7: f64 = (t3f5 / t3f6);let t3f8: f64 = (t3f3 + t3f7);let t3f9: f64 = (l.f645 * t3f8);let t3fa: f64 = (-230.25850929940458);let t3fb: f64 = if t3f9 < t3fa { 1.0 } else { 0.0 };l.f2ad = t3fb;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_94(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2ab == 0.0)) && (l.f2ad != 0.0)) {let t3fc: f64 = (-230.25850929940458);let t3fd: f64 = (l.f73d / l.f5f1);let t3fe: f64 = (l.f5f1 - l.f5ed);let t3ff: f64 = (l.f793 * t3fe);let t400: f64 = (l.f5ed * p.p85);let t401: f64 = (t3ff / t400);let t402: f64 = (t3fd + t401);let t403: f64 = (l.f645 * t402);let t404: f64 = (t3fc - t403);let t405: f64 = (-230.25850929940458);let t406: f64 = (l.f73d / l.f5f1);let t407: f64 = (l.f5f1 - l.f5ed);let t408: f64 = (l.f793 * t407);let t409: f64 = (l.f5ed * p.p85);let t40a: f64 = (t408 / t409);let t40b: f64 = (t406 + t40a);let t40c: f64 = (l.f645 * t40b);let t40d: f64 = (t405 - t40c);let t40e: f64 = (-230.25850929940458);let t40f: f64 = (l.f73d / l.f5f1);let t410: f64 = (l.f5f1 - l.f5ed);let t411: f64 = (l.f793 * t410);let t412: f64 = (l.f5ed * p.p85);let t413: f64 = (t411 / t412);let t414: f64 = (t40f + t413);let t415: f64 = (l.f645 * t414);let t416: f64 = (t40e - t415);let t417: f64 = (t416 * 0.3333333333333333);let t418: f64 = (1.0 + t417);let t419: f64 = (t40d * t418);let t41a: f64 = (0.5 * t419);let t41b: f64 = (1.0 + t41a);let t41c: f64 = (t404 * t41b);let t41d: f64 = (1.0 + t41c);let t41e: f64 = (1e-100 / t41d);(l.f53a, l.f53b, l.f53c, ) = (t41e, (-((1e-100 * (((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t400) - (t3ff * (l.f5ee * p.p85))) / (t400 * t400))))) * t41b) + (t404 * (0.5 * (((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t409) - (t408 * (l.f5ee * p.p85))) / (t409 * t409))))) * t418) + (t40d * ((-(l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t412) - (t411 * (l.f5ee * p.p85))) / (t412 * t412))))) * 0.3333333333333333))))))) / (t41d * t41d))), (-((1e-100 * (((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t400) - (t3ff * (l.f5ef * p.p85))) / (t400 * t400))))) * t41b) + (t404 * (0.5 * (((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t409) - (t408 * (l.f5ef * p.p85))) / (t409 * t409))))) * t418) + (t40d * ((-(l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t412) - (t411 * (l.f5ef * p.p85))) / (t412 * t412))))) * 0.3333333333333333))))))) / (t41d * t41d))), );}
        if (((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 != 0.0)) && (l.f2ab == 0.0)) && (l.f2ad == 0.0)) {let t41f: f64 = (l.f73d / l.f5f1);let t420: f64 = (l.f5f1 - l.f5ed);let t421: f64 = (l.f793 * t420);let t422: f64 = (l.f5ed * p.p85);let t423: f64 = (t421 / t422);let t424: f64 = (t41f + t423);let t425: f64 = (l.f645 * t424);let t426: f64 = (t425 - 230.25850929940458);let t427: f64 = (l.f73d / l.f5f1);let t428: f64 = (l.f5f1 - l.f5ed);let t429: f64 = (l.f793 * t428);let t42a: f64 = (l.f5ed * p.p85);let t42b: f64 = (t429 / t42a);let t42c: f64 = (t427 + t42b);let t42d: f64 = (l.f645 * t42c);let t42e: f64 = (t42d - 230.25850929940458);let t42f: f64 = (l.f73d / l.f5f1);let t430: f64 = (l.f5f1 - l.f5ed);let t431: f64 = (l.f793 * t430);let t432: f64 = (l.f5ed * p.p85);let t433: f64 = (t431 / t432);let t434: f64 = (t42f + t433);let t435: f64 = (l.f645 * t434);let t436: f64 = (t435 - 230.25850929940458);let t437: f64 = (t436 * 0.3333333333333333);let t438: f64 = (1.0 + t437);let t439: f64 = (t42e * t438);let t43a: f64 = (0.5 * t439);let t43b: f64 = (1.0 + t43a);let t43c: f64 = (t426 * t43b);let t43d: f64 = (1.0 + t43c);let t43e: f64 = (1e100 * t43d);(l.f53a, l.f53b, l.f53c, ) = (t43e, (1e100 * (((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t422) - (t421 * (l.f5ee * p.p85))) / (t422 * t422)))) * t43b) + (t426 * (0.5 * (((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t42a) - (t429 * (l.f5ee * p.p85))) / (t42a * t42a)))) * t438) + (t42e * ((l.f645 * ((-((l.f73d * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t432) - (t431 * (l.f5ee * p.p85))) / (t432 * t432)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t422) - (t421 * (l.f5ef * p.p85))) / (t422 * t422)))) * t43b) + (t426 * (0.5 * (((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t42a) - (t429 * (l.f5ef * p.p85))) / (t42a * t42a)))) * t438) + (t42e * ((l.f645 * ((-((l.f73d * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t432) - (t431 * (l.f5ef * p.p85))) / (t432 * t432)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) {let t43f: f64 = (l.f73d - l.f7b1);let t440: f64 = (t43f * l.f645);let t441: f64 = (1.0 + t440);let t442: f64 = (t441 * l.f89);let t443: f64 = (t442).sqrt();l.f825 = t443;let t444: f64 = (l.f5eb * l.f5eb);let t445: f64 = (t444 / l.f5df);l.f64f = t445;let t446: f64 = (l.f5e5 / l.f645);let t447: f64 = (l.f5df / l.f64f);let t448: f64 = (t447).ln();let t449: f64 = (t446 * t448);l.f793 = t449;}
        let t44a: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f2b0 = t44a;
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {let t44b: f64 = (l.f7b1 - l.f793);let t44c: f64 = (p.p86 * t44b);let t44d: f64 = (t44c + l.f5e5);(l.f601, l.f602, l.f603, ) = (t44d, 0.0, 0.0, );let t44e: f64 = (p.p86 * l.f793);let t44f: f64 = (l.f5e5 - t44e);(l.f5ed, l.f5ee, l.f5ef, ) = (t44f, 0.0, 0.0, );let t450: f64 = (p.p85 - l.f601);let t451: f64 = (t450 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t451, (-l.f602), (-l.f603), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_95(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {let t452: f64 = (4.0 * p.p85);let t453: f64 = (t452 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t453, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {
            let (t455, t456, t457,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t454: f64 = (-l.f6f7);
        (t454, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t455, t456, t457, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {let t458: f64 = (l.f6f3 * l.f6f3);let t459: f64 = (t458 + l.f6f7);let t45a: f64 = (t459).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t45a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t45a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t45a)), );let t45b: f64 = (l.f6f3 / l.f6f7);let t45c: f64 = (1.0 + t45b);let t45d: f64 = (0.5 * t45c);(l.f55, l.f56, l.f57, ) = (t45d, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t45e: f64 = (l.f6f3 + l.f6f7);let t45f: f64 = (0.5 * t45e);let t460: f64 = (p.p85 - t45f);(l.f605, l.f606, l.f607, ) = (t460, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t461: f64 = (l.f605 - l.f5e5);let t462: f64 = (t461 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t462, l.f606, l.f607, );let t463: f64 = (4.0 * l.f5e5);let t464: f64 = (t463 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t464, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {
            let (t466, t467, t468,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t465: f64 = (-l.f6f7);
        (t465, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t466, t467, t468, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {let t469: f64 = (l.f6f3 * l.f6f3);let t46a: f64 = (t469 + l.f6f7);let t46b: f64 = (t46a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t46b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t46b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t46b)), );let t46c: f64 = (l.f6f3 / l.f6f7);let t46d: f64 = (1.0 + t46c);let t46e: f64 = (0.5 * t46d);(l.f51, l.f52, l.f53, ) = (t46e, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t46f: f64 = (l.f6f3 + l.f6f7);let t470: f64 = (0.5 * t46f);let t471: f64 = (l.f5e5 + t470);(l.f5f1, l.f5f2, l.f5f3, ) = (t471, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t472: f64 = (p.p85 - l.f5ed);let t473: f64 = (t472 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t473, (-l.f5ee), (-l.f5ef), );let t474: f64 = (4.0 * p.p85);let t475: f64 = (t474 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t475, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {
            let (t477, t478, t479,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t476: f64 = (-l.f6f7);
        (t476, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t477, t478, t479, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {let t47a: f64 = (l.f6f3 * l.f6f3);let t47b: f64 = (t47a + l.f6f7);let t47c: f64 = (t47b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t47c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t47c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t47c)), );let t47d: f64 = (l.f6f3 + l.f6f7);let t47e: f64 = (0.5 * t47d);let t47f: f64 = (p.p85 - t47e);(l.f5ed, l.f5ee, l.f5ef, ) = (t47f, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t480: f64 = (l.f5ed - l.f5e5);let t481: f64 = (t480 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t481, l.f5ee, l.f5ef, );let t482: f64 = (4.0 * l.f5e5);let t483: f64 = (t482 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t483, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {
            let (t485, t486, t487,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t484: f64 = (-l.f6f7);
        (t484, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t485, t486, t487, );
        }
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 != 0.0)) {let t488: f64 = (l.f6f3 * l.f6f3);let t489: f64 = (t488 + l.f6f7);let t48a: f64 = (t489).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t48a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t48a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t48a)), );let t48b: f64 = (l.f6f3 + l.f6f7);let t48c: f64 = (0.5 * t48b);let t48d: f64 = (l.f5e5 + t48c);(l.f5ed, l.f5ee, l.f5ef, ) = (t48d, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t48e: f64 = (p.p86 * l.f55);let t48f: f64 = (t48e * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t48f, (((p.p86 * l.f56) * l.f51) + (t48e * l.f52)), (((p.p86 * l.f57) * l.f51) + (t48e * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f293 != 0.0)) && (l.f295 == 0.0)) && (l.f2b0 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t490: f64 = (l.f7b1 / l.f5f1);let t491: f64 = (l.f5f1 - l.f5ed);let t492: f64 = (l.f793 * t491);let t493: f64 = (l.f5ed * p.p85);let t494: f64 = (t492 / t493);let t495: f64 = (t490 + t494);let t496: f64 = (l.f645 * t495);let t497: f64 = (t496).abs();let t498: f64 = if t497 < 230.25850929940458 { 1.0 } else { 0.0 };l.f2b2 = t498;
    }
}
