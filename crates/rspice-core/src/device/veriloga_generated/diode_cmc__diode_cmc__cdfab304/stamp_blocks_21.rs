#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_161(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) {let t0: f64 = (0.29214664 * l.f6e2);let t1: f64 = (l.f6e2 * l.f6e2);let t2: f64 = (l.f16 * t1);let t3: f64 = (t0 + t2);let t4: f64 = (l.f6e2 * l.f6e2);let t5: f64 = (t4 * l.f6e2);let t6: f64 = (l.f2a * t5);let t7: f64 = (t3 + t6);let t8: f64 = (t7 * l.f6fc);l.f6e = t8;l.f6f = 0.0;}
        let t9: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f372 = t9;l.f373 = 0.0;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f372 != 0.0)) {l.f74 = l.f6e;l.f75 = 0.0;}
        let ta: f64 = (-230.25850929940458);let tb: f64 = if l.f5d4 > ta { 1.0 } else { 0.0 };l.f374 = tb;l.f375 = 0.0;
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f372 == 0.0)) && (l.f374 != 0.0)) {let tc: f64 = (l.f5d4).exp();l.f6fc = tc;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f372 == 0.0)) && (l.f374 == 0.0)) {let td: f64 = (-230.25850929940458);let te: f64 = (td - l.f5d4);let tf: f64 = (-230.25850929940458);let t10: f64 = (tf - l.f5d4);let t11: f64 = (-230.25850929940458);let t12: f64 = (t11 - l.f5d4);let t13: f64 = (t12 * 0.3333333333333333);let t14: f64 = (1.0 + t13);let t15: f64 = (t10 * t14);let t16: f64 = (0.5 * t15);let t17: f64 = (1.0 + t16);let t18: f64 = (te * t17);let t19: f64 = (1.0 + t18);let t1a: f64 = (1e-100 / t19);l.f6fc = t1a;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f372 == 0.0)) {let t1b: f64 = (2.0 * l.f6fc);let t1c: f64 = (t1b - l.f6e);l.f74 = t1c;l.f75 = 0.0;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) {let t1d: f64 = (1.772453850905516 * 0.5);let t1e: f64 = (l.fe * l.f74);let t1f: f64 = (t1e / l.f5a8);let t20: f64 = (t1d * t1f);l.fd6 = t20;l.fd7 = 0.0;let t21: f64 = (l.f9 * l.fd6);let t22: f64 = (t21 * l.f7f5);let t23: f64 = (l.f3f * t22);l.f599 = t23;l.f59a = 0.0;}
        let t24: f64 = if l.f24 == 0.0 { 1.0 } else { 0.0 };l.f376 = t24;l.f377 = 0.0;
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 != 0.0)) {l.f529 = 0.0;l.f52a = 0.0;}
        let t25: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f378 = t25;l.f379 = 0.0;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f378 != 0.0)) {let t26: f64 = (l.f771 - l.f750);let t27: f64 = (t26 * l.f773);let t28: f64 = (t27).sqrt();l.f6fc = t28;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f378 == 0.0)) {let t29: f64 = (l.f771 - l.f750);let t2a: f64 = (t29 * l.f773);let t2b: f64 = (t2a).powf(l.f623);l.f6fc = t2b;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) {let t2c: f64 = (l.f771 - l.f750);let t2d: f64 = (t2c * l.f7da);let t2e: f64 = (t2d / l.f6fc);let t2f: f64 = (l.f611 * t2e);l.fb6 = t2f;l.fb7 = 0.0;}
        let t30: f64 = (-l.fa1);let t31: f64 = (t30 / l.fb6);let t32: f64 = (t31).abs();let t33: f64 = if t32 < 230.25850929940458 { 1.0 } else { 0.0 };l.f37a = t33;l.f37b = 0.0;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f37a != 0.0)) {let t34: f64 = (-l.fa1);let t35: f64 = (t34 / l.fb6);let t36: f64 = (t35).exp();l.f6fc = t36;l.f6fd = 0.0;}
        let t37: f64 = (-l.fa1);let t38: f64 = (t37 / l.fb6);let t39: f64 = (-230.25850929940458);let t3a: f64 = if t38 < t39 { 1.0 } else { 0.0 };l.f37c = t3a;l.f37d = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_162(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f37a == 0.0)) && (l.f37c != 0.0)) {let t3b: f64 = (-230.25850929940458);let t3c: f64 = (-l.fa1);let t3d: f64 = (t3c / l.fb6);let t3e: f64 = (t3b - t3d);let t3f: f64 = (-230.25850929940458);let t40: f64 = (-l.fa1);let t41: f64 = (t40 / l.fb6);let t42: f64 = (t3f - t41);let t43: f64 = (-230.25850929940458);let t44: f64 = (-l.fa1);let t45: f64 = (t44 / l.fb6);let t46: f64 = (t43 - t45);let t47: f64 = (t46 * 0.3333333333333333);let t48: f64 = (1.0 + t47);let t49: f64 = (t42 * t48);let t4a: f64 = (0.5 * t49);let t4b: f64 = (1.0 + t4a);let t4c: f64 = (t3e * t4b);let t4d: f64 = (1.0 + t4c);let t4e: f64 = (1e-100 / t4d);l.f6fc = t4e;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f37a == 0.0)) && (l.f37c == 0.0)) {let t4f: f64 = (-l.fa1);let t50: f64 = (t4f / l.fb6);let t51: f64 = (t50 - 230.25850929940458);let t52: f64 = (-l.fa1);let t53: f64 = (t52 / l.fb6);let t54: f64 = (t53 - 230.25850929940458);let t55: f64 = (-l.fa1);let t56: f64 = (t55 / l.fb6);let t57: f64 = (t56 - 230.25850929940458);let t58: f64 = (t57 * 0.3333333333333333);let t59: f64 = (1.0 + t58);let t5a: f64 = (t54 * t59);let t5b: f64 = (0.5 * t5a);let t5c: f64 = (1.0 + t5b);let t5d: f64 = (t51 * t5c);let t5e: f64 = (1.0 + t5d);let t5f: f64 = (1e100 * t5e);l.f6fc = t5f;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) {let t60: f64 = (l.f73f * l.fb6);let t61: f64 = (t60 * l.fb6);let t62: f64 = (t61 * l.f6fc);let t63: f64 = (l.f24 * t62);l.f529 = t63;l.f52a = 0.0;}
        let t64: f64 = if ((l.f783 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f37e = t64;l.f37f = 0.0;
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e != 0.0)) {l.fae = 1.0;l.faf = 0.0;}
        let t65: f64 = (-l.f2);let t66: f64 = (t65 * l.f783);let t67: f64 = if l.f74a > t66 { 1.0 } else { 0.0 };l.f380 = t67;l.f381 = 0.0;let t68: f64 = if l.f625 == 4.0 { 1.0 } else { 0.0 };l.f382 = t68;l.f383 = 0.0;
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e == 0.0)) && (l.f380 != 0.0)) && (l.f382 != 0.0)) {let t69: f64 = (l.f74a * l.f787);let t6a: f64 = (t69).abs();let t6b: f64 = (l.f74a * l.f787);let t6c: f64 = (t6b).abs();let t6d: f64 = (t6a * t6c);let t6e: f64 = (l.f74a * l.f787);let t6f: f64 = (t6e).abs();let t70: f64 = (t6d * t6f);let t71: f64 = (l.f74a * l.f787);let t72: f64 = (t71).abs();let t73: f64 = (t70 * t72);l.f6fc = t73;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e == 0.0)) && (l.f380 != 0.0)) && (l.f382 == 0.0)) {let t74: f64 = (l.f74a * l.f787);let t75: f64 = (t74).abs();let t76: f64 = (t75).powf(l.f625);l.f6fc = t76;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e == 0.0)) && (l.f380 != 0.0)) {let t77: f64 = (1.0 - l.f6fc);let t78: f64 = (1.0 / t77);l.fae = t78;l.faf = 0.0;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e == 0.0)) && (l.f380 == 0.0)) {let t79: f64 = (l.f2 * l.f783);let t7a: f64 = (l.f74a + t79);let t7b: f64 = (t7a * l.f6ba);let t7c: f64 = (l.fc3 + t7b);l.fae = t7c;l.faf = 0.0;}
        if ((l.f29a != 0.0) && (l.f360 == 0.0)) {let t7d: f64 = (l.f52f + l.f593);let t7e: f64 = (t7d + l.f599);let t7f: f64 = (t7e + l.f529);let t80: f64 = (t7f * l.fae);(l.f562, l.f563, l.f564, ) = (t80, (l.f530 * l.fae), (l.f531 * l.fae), );l.f565 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_163(
        l: &mut StampLocals,
    ) {
        if ((l.f29a != 0.0) && (l.f360 == 0.0)) {let t81: f64 = (l.f593 + l.f599);let t82: f64 = (t81 + l.f529);let t83: f64 = (t82 * l.fae);(l.f552, l.f553, l.f554, ) = (t83, 0.0, 0.0, );l.f555 = 0.0;}
        let t84: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f384 = t84;l.f385 = 0.0;
        if ((l.f29a != 0.0) && (l.f384 != 0.0)) {(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );l.f579 = 0.0;(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );l.f55d = 0.0;(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );l.f697 = 0.0;}
        let t85: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f386 = t85;l.f387 = 0.0;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f386 != 0.0)) {let t86: f64 = (l.f796 * l.f76d);let t87: f64 = (1.0 - t86);let t88: f64 = (t87).sqrt();l.f6fc = t88;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f386 == 0.0)) {let t89: f64 = (l.f796 * l.f76d);let t8a: f64 = (1.0 - t89);let t8b: f64 = (t8a).powf(l.f60f);l.f6fc = t8b;l.f6fd = 0.0;}
        if ((l.f29a != 0.0) && (l.f384 == 0.0)) {let t8c: f64 = (1.0 - l.f6fc);let t8d: f64 = (l.f6a2 * t8c);let t8e: f64 = (l.f73f - l.f796);let t8f: f64 = (l.f69c * t8e);let t90: f64 = (t8d + t8f);(l.f694, l.f695, l.f696, ) = (t90, 0.0, 0.0, );l.f697 = 0.0;let t91: f64 = (l.f54c * l.f53e);(l.f52f, l.f530, l.f531, ) = (t91, (l.f54c * l.f53f), (l.f54c * l.f540), );l.f532 = 0.0;}
        let t92: f64 = if ((l.f3d == 0.0) && (l.f43 == 0.0)) { 1.0 } else { 0.0 };l.f388 = t92;l.f389 = 0.0;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 != 0.0)) {l.f758 = 0.0;l.f759 = 0.0;l.f7e9 = 0.0;l.f7ea = 0.0;l.f7d1 = 0.0;l.f7d2 = 0.0;l.f9 = 0.0;l.fa = 0.0;l.f593 = 0.0;l.f594 = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) {let t93: f64 = (l.f77d - l.f7a2);l.f758 = t93;l.f759 = 0.0;let t94: f64 = (l.f714 / l.f758);let t95: f64 = (1.0 - t94);let t96: f64 = (t95).sqrt();let t97: f64 = (1.0 - t96);l.f7ef = t97;l.f7f0 = 0.0;}
        let t98: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f38a = t98;l.f38b = 0.0;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) && (l.f38a != 0.0)) {l.f66 = 0.0;l.f67 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) && (l.f38a == 0.0)) {let t99: f64 = (l.f7ef * l.f7ef);let t9a: f64 = (l.f7ef).ln();let t9b: f64 = (t99 * t9a);let t9c: f64 = (1.0 - l.f7ef);let t9d: f64 = (t9b / t9c);let t9e: f64 = (t9d + l.f7ef);let t9f: f64 = (2.0 * l.f653);let ta0: f64 = (1.0 - t9f);let ta1: f64 = (t9e * ta0);l.f66 = ta1;l.f67 = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) {let ta2: f64 = (l.f7ef + l.f66);l.f7e9 = ta2;l.f7ea = 0.0;}
        let ta3: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f38c = ta3;l.f38d = 0.0;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) && (l.f38c != 0.0)) {let ta4: f64 = (l.f758 * l.f77b);let ta5: f64 = (ta4).sqrt();l.f6fc = ta5;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) && (l.f38c == 0.0)) {let ta6: f64 = (l.f758 * l.f77b);let ta7: f64 = (ta6).powf(l.f653);l.f6fc = ta7;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) {let ta8: f64 = (l.f7e0 * l.f6fc);l.f7d1 = ta8;l.f7d2 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_164(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) {let ta9: f64 = (l.f825 - 1.0);let taa: f64 = (ta9 * l.f7d1);let tab: f64 = (l.fd1 * taa);l.f9 = tab;l.fa = 0.0;let tac: f64 = (l.f9 * l.f7e9);let tad: f64 = (l.f3d * tac);l.f593 = tad;l.f594 = 0.0;}
        let tae: f64 = if l.f43 == 0.0 { 1.0 } else { 0.0 };l.f38e = tae;l.f38f = 0.0;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e != 0.0)) {l.f599 = 0.0;l.f59a = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let taf: f64 = (l.f7d1 * l.f60f);let tb0: f64 = (taf / l.f758);let tb1: f64 = (l.f22 * tb0);l.f19 = tb1;l.f1a = 0.0;let tb2: f64 = (0.666666666666667 * l.f12);let tb3: f64 = (tb2 / l.f19);l.f71a = tb3;l.f71b = 0.0;let tb4: f64 = (l.f71a * l.f71a);l.f72c = tb4;l.f72d = 0.0;let tb5: f64 = (l.f72c * l.f72c);let tb6: f64 = (l.f72c * l.f72c);let tb7: f64 = (tb6 + 1.0);let tb8: f64 = (tb5 / tb7);let tb9: f64 = (tb8).sqrt();l.f726 = tb9;l.f727 = 0.0;let tba: f64 = (l.f726).abs();let tbb: f64 = (tba).sqrt();l.f6c1 = tbb;l.f6c2 = 0.0;let tbc: f64 = (l.f726 * l.f6c1);l.f732 = tbc;l.f733 = 0.0;}
        let tbd: f64 = (-l.f653);let tbe: f64 = (tbd * l.f615);let tbf: f64 = (-1.0);let tc0: f64 = if tbe == tbf { 1.0 } else { 0.0 };l.f390 = tc0;l.f391 = 0.0;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f390 != 0.0)) {let tc1: f64 = (l.f19 * l.f732);let tc2: f64 = (1.0 + tc1);let tc3: f64 = (1.0 / tc2);l.f7e3 = tc3;l.f7e4 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f390 == 0.0)) {let tc4: f64 = (l.f19 * l.f732);let tc5: f64 = (1.0 + tc4);let tc6: f64 = (-l.f653);let tc7: f64 = (tc6 * l.f615);let tc8: f64 = (tc5).powf(tc7);l.f7e3 = tc8;l.f7e4 = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let tc9: f64 = (l.f7e9 * l.f7e3);let tca: f64 = (l.f7e9 + l.f7e3);let tcb: f64 = (tc9 / tca);l.f7f5 = tcb;l.f7f6 = 0.0;let tcc: f64 = (l.f19 / l.f6c1);let tcd: f64 = (0.375 * tcc);let tce: f64 = (tcd).sqrt();l.f5a8 = tce;l.f5a9 = 0.0;let tcf: f64 = (l.f71a * l.f6c1);let td0: f64 = (2.0 * tcf);let td1: f64 = (td0 - l.f726);l.f5b4 = td1;l.f5b5 = 0.0;let td2: f64 = (l.f12 * l.f71a);let td3: f64 = (td2 * l.f6c1);let td4: f64 = (l.f12 * l.f726);let td5: f64 = (td3 - td4);let td6: f64 = (l.f19 * l.f732);let td7: f64 = (0.5 * td6);let td8: f64 = (td5 + td7);l.f5d4 = td8;l.f5d5 = 0.0;let td9: f64 = (l.f5b4 - 1.0);let tda: f64 = (td9 * l.f5a8);l.f7fb = tda;l.f7fc = 0.0;let tdb: f64 = (l.f7fb * l.f7fb);l.f811 = tdb;l.f812 = 0.0;}
        let tdc: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f392 = tdc;l.f393 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_165(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f392 != 0.0)) {let tdd: f64 = (l.f62b * l.f7fb);let tde: f64 = (1.0 + tdd);let tdf: f64 = (1.0 / tde);l.f6e2 = tdf;l.f6e3 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f392 == 0.0)) {let te0: f64 = (l.f62b * l.f7fb);let te1: f64 = (1.0 - te0);let te2: f64 = (1.0 / te1);l.f6e2 = te2;l.f6e3 = 0.0;}
        let te3: f64 = (-l.f811);let te4: f64 = (te3 + l.f5d4);let te5: f64 = (-230.25850929940458);let te6: f64 = if te4 > te5 { 1.0 } else { 0.0 };l.f394 = te6;l.f395 = 0.0;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f394 != 0.0)) {let te7: f64 = (-l.f811);let te8: f64 = (te7 + l.f5d4);let te9: f64 = (te8).exp();l.f6fc = te9;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f394 == 0.0)) {let tea: f64 = (-230.25850929940458);let teb: f64 = (-l.f811);let tec: f64 = (teb + l.f5d4);let ted: f64 = (tea - tec);let tee: f64 = (-230.25850929940458);let tef: f64 = (-l.f811);let tf0: f64 = (tef + l.f5d4);let tf1: f64 = (tee - tf0);let tf2: f64 = (-230.25850929940458);let tf3: f64 = (-l.f811);let tf4: f64 = (tf3 + l.f5d4);let tf5: f64 = (tf2 - tf4);let tf6: f64 = (tf5 * 0.3333333333333333);let tf7: f64 = (1.0 + tf6);let tf8: f64 = (tf1 * tf7);let tf9: f64 = (0.5 * tf8);let tfa: f64 = (1.0 + tf9);let tfb: f64 = (ted * tfa);let tfc: f64 = (1.0 + tfb);let tfd: f64 = (1e-100 / tfc);l.f6fc = tfd;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let tfe: f64 = (0.29214664 * l.f6e2);let tff: f64 = (l.f6e2 * l.f6e2);let t100: f64 = (l.f16 * tff);let t101: f64 = (tfe + t100);let t102: f64 = (l.f6e2 * l.f6e2);let t103: f64 = (t102 * l.f6e2);let t104: f64 = (l.f2a * t103);let t105: f64 = (t101 + t104);let t106: f64 = (t105 * l.f6fc);l.f6e = t106;l.f6f = 0.0;}
        let t107: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f396 = t107;l.f397 = 0.0;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f396 != 0.0)) {l.f74 = l.f6e;l.f75 = 0.0;}
        let t108: f64 = (-230.25850929940458);let t109: f64 = if l.f5d4 > t108 { 1.0 } else { 0.0 };l.f398 = t109;l.f399 = 0.0;
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f396 == 0.0)) && (l.f398 != 0.0)) {let t10a: f64 = (l.f5d4).exp();l.f6fc = t10a;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f396 == 0.0)) && (l.f398 == 0.0)) {let t10b: f64 = (-230.25850929940458);let t10c: f64 = (t10b - l.f5d4);let t10d: f64 = (-230.25850929940458);let t10e: f64 = (t10d - l.f5d4);let t10f: f64 = (-230.25850929940458);let t110: f64 = (t10f - l.f5d4);let t111: f64 = (t110 * 0.3333333333333333);let t112: f64 = (1.0 + t111);let t113: f64 = (t10e * t112);let t114: f64 = (0.5 * t113);let t115: f64 = (1.0 + t114);let t116: f64 = (t10c * t115);let t117: f64 = (1.0 + t116);let t118: f64 = (1e-100 / t117);l.f6fc = t118;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f396 == 0.0)) {let t119: f64 = (2.0 * l.f6fc);let t11a: f64 = (t119 - l.f6e);l.f74 = t11a;l.f75 = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let t11b: f64 = (1.772453850905516 * 0.5);let t11c: f64 = (l.f12 * l.f74);let t11d: f64 = (t11c / l.f5a8);let t11e: f64 = (t11b * t11d);l.fd6 = t11e;l.fd7 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_166(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let t11f: f64 = (l.f9 * l.fd6);let t120: f64 = (t11f * l.f7f5);let t121: f64 = (l.f43 * t120);l.f599 = t121;l.f59a = 0.0;}
        let t122: f64 = if l.f28 == 0.0 { 1.0 } else { 0.0 };l.f39a = t122;l.f39b = 0.0;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a != 0.0)) {l.f529 = 0.0;l.f52a = 0.0;}
        let t123: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f39c = t123;l.f39d = 0.0;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39c != 0.0)) {let t124: f64 = (l.f779 - l.f750);let t125: f64 = (t124 * l.f77b);let t126: f64 = (t125).sqrt();l.f6fc = t126;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39c == 0.0)) {let t127: f64 = (l.f779 - l.f750);let t128: f64 = (t127 * l.f77b);let t129: f64 = (t128).powf(l.f653);l.f6fc = t129;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) {let t12a: f64 = (l.f779 - l.f750);let t12b: f64 = (t12a * l.f7de);let t12c: f64 = (t12b / l.f6fc);let t12d: f64 = (l.f615 * t12c);l.fb6 = t12d;l.fb7 = 0.0;}
        let t12e: f64 = (-l.fab);let t12f: f64 = (t12e / l.fb6);let t130: f64 = (t12f).abs();let t131: f64 = if t130 < 230.25850929940458 { 1.0 } else { 0.0 };l.f39e = t131;l.f39f = 0.0;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39e != 0.0)) {let t132: f64 = (-l.fab);let t133: f64 = (t132 / l.fb6);let t134: f64 = (t133).exp();l.f6fc = t134;l.f6fd = 0.0;}
        let t135: f64 = (-l.fab);let t136: f64 = (t135 / l.fb6);let t137: f64 = (-230.25850929940458);let t138: f64 = if t136 < t137 { 1.0 } else { 0.0 };l.f3a0 = t138;l.f3a1 = 0.0;
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39e == 0.0)) && (l.f3a0 != 0.0)) {let t139: f64 = (-230.25850929940458);let t13a: f64 = (-l.fab);let t13b: f64 = (t13a / l.fb6);let t13c: f64 = (t139 - t13b);let t13d: f64 = (-230.25850929940458);let t13e: f64 = (-l.fab);let t13f: f64 = (t13e / l.fb6);let t140: f64 = (t13d - t13f);let t141: f64 = (-230.25850929940458);let t142: f64 = (-l.fab);let t143: f64 = (t142 / l.fb6);let t144: f64 = (t141 - t143);let t145: f64 = (t144 * 0.3333333333333333);let t146: f64 = (1.0 + t145);let t147: f64 = (t140 * t146);let t148: f64 = (0.5 * t147);let t149: f64 = (1.0 + t148);let t14a: f64 = (t13c * t149);let t14b: f64 = (1.0 + t14a);let t14c: f64 = (1e-100 / t14b);l.f6fc = t14c;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39e == 0.0)) && (l.f3a0 == 0.0)) {let t14d: f64 = (-l.fab);let t14e: f64 = (t14d / l.fb6);let t14f: f64 = (t14e - 230.25850929940458);let t150: f64 = (-l.fab);let t151: f64 = (t150 / l.fb6);let t152: f64 = (t151 - 230.25850929940458);let t153: f64 = (-l.fab);let t154: f64 = (t153 / l.fb6);let t155: f64 = (t154 - 230.25850929940458);let t156: f64 = (t155 * 0.3333333333333333);let t157: f64 = (1.0 + t156);let t158: f64 = (t152 * t157);let t159: f64 = (0.5 * t158);let t15a: f64 = (1.0 + t159);let t15b: f64 = (t14f * t15a);let t15c: f64 = (1.0 + t15b);let t15d: f64 = (1e100 * t15c);l.f6fc = t15d;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) {let t15e: f64 = (l.f73f * l.fb6);let t15f: f64 = (t15e * l.fb6);let t160: f64 = (t15f * l.f6fc);let t161: f64 = (l.f28 * t160);l.f529 = t161;l.f52a = 0.0;}
        let t162: f64 = if ((l.f78d > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f3a2 = t162;l.f3a3 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_167(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 != 0.0)) {l.fae = 1.0;l.faf = 0.0;}
        let t163: f64 = (-l.f2);let t164: f64 = (t163 * l.f78d);let t165: f64 = if l.f74a > t164 { 1.0 } else { 0.0 };l.f3a4 = t165;l.f3a5 = 0.0;let t166: f64 = if l.f629 == 4.0 { 1.0 } else { 0.0 };l.f3a6 = t166;l.f3a7 = 0.0;
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 == 0.0)) && (l.f3a4 != 0.0)) && (l.f3a6 != 0.0)) {let t167: f64 = (l.f74a * l.f78b);let t168: f64 = (t167).abs();let t169: f64 = (l.f74a * l.f78b);let t16a: f64 = (t169).abs();let t16b: f64 = (t168 * t16a);let t16c: f64 = (l.f74a * l.f78b);let t16d: f64 = (t16c).abs();let t16e: f64 = (t16b * t16d);let t16f: f64 = (l.f74a * l.f78b);let t170: f64 = (t16f).abs();let t171: f64 = (t16e * t170);l.f6fc = t171;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 == 0.0)) && (l.f3a4 != 0.0)) && (l.f3a6 == 0.0)) {let t172: f64 = (l.f74a * l.f78b);let t173: f64 = (t172).abs();let t174: f64 = (t173).powf(l.f629);l.f6fc = t174;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 == 0.0)) && (l.f3a4 != 0.0)) {let t175: f64 = (1.0 - l.f6fc);let t176: f64 = (1.0 / t175);l.fae = t176;l.faf = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 == 0.0)) && (l.f3a4 == 0.0)) {let t177: f64 = (l.f2 * l.f78d);let t178: f64 = (l.f74a + t177);let t179: f64 = (t178 * l.f6be);let t17a: f64 = (l.fc7 + t179);l.fae = t17a;l.faf = 0.0;}
        if ((l.f29a != 0.0) && (l.f384 == 0.0)) {let t17b: f64 = (l.f52f + l.f593);let t17c: f64 = (t17b + l.f599);let t17d: f64 = (t17c + l.f529);let t17e: f64 = (t17d * l.fae);(l.f576, l.f577, l.f578, ) = (t17e, (l.f530 * l.fae), (l.f531 * l.fae), );l.f579 = 0.0;let t17f: f64 = (l.f593 + l.f599);let t180: f64 = (t17f + l.f529);let t181: f64 = (t180 * l.fae);(l.f55a, l.f55b, l.f55c, ) = (t181, 0.0, 0.0, );l.f55d = 0.0;}
        let t182: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f3a8 = t182;l.f3a9 = 0.0;
        if ((l.f29a != 0.0) && (l.f3a8 != 0.0)) {(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );l.f571 = 0.0;(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );l.f559 = 0.0;(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );l.f693 = 0.0;}
        let t183: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f3aa = t183;l.f3ab = 0.0;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3aa != 0.0)) {let t184: f64 = (l.f796 * l.f76b);let t185: f64 = (1.0 - t184);let t186: f64 = (t185).sqrt();l.f6fc = t186;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3aa == 0.0)) {let t187: f64 = (l.f796 * l.f76b);let t188: f64 = (1.0 - t187);let t189: f64 = (t188).powf(l.f60d);l.f6fc = t189;l.f6fd = 0.0;}
        if ((l.f29a != 0.0) && (l.f3a8 == 0.0)) {let t18a: f64 = (1.0 - l.f6fc);let t18b: f64 = (l.f6a0 * t18a);let t18c: f64 = (l.f73f - l.f796);let t18d: f64 = (l.f69a * t18c);let t18e: f64 = (t18b + t18d);(l.f690, l.f691, l.f692, ) = (t18e, 0.0, 0.0, );l.f693 = 0.0;let t18f: f64 = (l.f544 * l.f53a);(l.f52f, l.f530, l.f531, ) = (t18f, (l.f544 * l.f53b), (l.f544 * l.f53c), );l.f532 = 0.0;}
        let t190: f64 = if ((l.f3b == 0.0) && (l.f41 == 0.0)) { 1.0 } else { 0.0 };l.f3ac = t190;l.f3ad = 0.0;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac != 0.0)) {l.f758 = 0.0;l.f759 = 0.0;l.f7e9 = 0.0;l.f7ea = 0.0;l.f7d1 = 0.0;l.f7d2 = 0.0;l.f9 = 0.0;l.fa = 0.0;l.f593 = 0.0;l.f594 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_168(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) {let t191: f64 = (l.f763 - l.f7a2);l.f758 = t191;l.f759 = 0.0;let t192: f64 = (l.f714 / l.f758);let t193: f64 = (1.0 - t192);let t194: f64 = (t193).sqrt();let t195: f64 = (1.0 - t194);l.f7ef = t195;l.f7f0 = 0.0;}
        let t196: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f3ae = t196;l.f3af = 0.0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) && (l.f3ae != 0.0)) {l.f66 = 0.0;l.f67 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) && (l.f3ae == 0.0)) {let t197: f64 = (l.f7ef * l.f7ef);let t198: f64 = (l.f7ef).ln();let t199: f64 = (t197 * t198);let t19a: f64 = (1.0 - l.f7ef);let t19b: f64 = (t199 / t19a);let t19c: f64 = (t19b + l.f7ef);let t19d: f64 = (2.0 * l.f62f);let t19e: f64 = (1.0 - t19d);let t19f: f64 = (t19c * t19e);l.f66 = t19f;l.f67 = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) {let t1a0: f64 = (l.f7ef + l.f66);l.f7e9 = t1a0;l.f7ea = 0.0;}
        let t1a1: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f3b0 = t1a1;l.f3b1 = 0.0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) && (l.f3b0 != 0.0)) {let t1a2: f64 = (l.f758 * l.f777);let t1a3: f64 = (t1a2).sqrt();l.f6fc = t1a3;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) && (l.f3b0 == 0.0)) {let t1a4: f64 = (l.f758 * l.f777);let t1a5: f64 = (t1a4).powf(l.f62f);l.f6fc = t1a5;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) {let t1a6: f64 = (l.f7d8 * l.f6fc);l.f7d1 = t1a6;l.f7d2 = 0.0;let t1a7: f64 = (l.f825 - 1.0);let t1a8: f64 = (t1a7 * l.f7d1);let t1a9: f64 = (l.fcd * t1a8);l.f9 = t1a9;l.fa = 0.0;let t1aa: f64 = (l.f9 * l.f7e9);let t1ab: f64 = (l.f3b * t1aa);l.f593 = t1ab;l.f594 = 0.0;}
        let t1ac: f64 = if l.f41 == 0.0 { 1.0 } else { 0.0 };l.f3b2 = t1ac;l.f3b3 = 0.0;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 != 0.0)) {l.f599 = 0.0;l.f59a = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) {let t1ad: f64 = (l.f7d1 * l.f60d);let t1ae: f64 = (t1ad / l.f758);let t1af: f64 = (l.f20 * t1ae);l.f19 = t1af;l.f1a = 0.0;let t1b0: f64 = (0.666666666666667 * l.f10);let t1b1: f64 = (t1b0 / l.f19);l.f71a = t1b1;l.f71b = 0.0;let t1b2: f64 = (l.f71a * l.f71a);l.f72c = t1b2;l.f72d = 0.0;let t1b3: f64 = (l.f72c * l.f72c);let t1b4: f64 = (l.f72c * l.f72c);let t1b5: f64 = (t1b4 + 1.0);let t1b6: f64 = (t1b3 / t1b5);let t1b7: f64 = (t1b6).sqrt();l.f726 = t1b7;l.f727 = 0.0;let t1b8: f64 = (l.f726).abs();let t1b9: f64 = (t1b8).sqrt();l.f6c1 = t1b9;l.f6c2 = 0.0;let t1ba: f64 = (l.f726 * l.f6c1);l.f732 = t1ba;l.f733 = 0.0;}
        let t1bb: f64 = (-l.f62f);let t1bc: f64 = (t1bb * l.f613);let t1bd: f64 = (-1.0);let t1be: f64 = if t1bc == t1bd { 1.0 } else { 0.0 };l.f3b4 = t1be;l.f3b5 = 0.0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b4 != 0.0)) {let t1bf: f64 = (l.f19 * l.f732);let t1c0: f64 = (1.0 + t1bf);let t1c1: f64 = (1.0 / t1c0);l.f7e3 = t1c1;l.f7e4 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_169(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b4 == 0.0)) {let t1c2: f64 = (l.f19 * l.f732);let t1c3: f64 = (1.0 + t1c2);let t1c4: f64 = (-l.f62f);let t1c5: f64 = (t1c4 * l.f613);let t1c6: f64 = (t1c3).powf(t1c5);l.f7e3 = t1c6;l.f7e4 = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) {let t1c7: f64 = (l.f7e9 * l.f7e3);let t1c8: f64 = (l.f7e9 + l.f7e3);let t1c9: f64 = (t1c7 / t1c8);l.f7f5 = t1c9;l.f7f6 = 0.0;let t1ca: f64 = (l.f19 / l.f6c1);let t1cb: f64 = (0.375 * t1ca);let t1cc: f64 = (t1cb).sqrt();l.f5a8 = t1cc;l.f5a9 = 0.0;let t1cd: f64 = (l.f71a * l.f6c1);let t1ce: f64 = (2.0 * t1cd);let t1cf: f64 = (t1ce - l.f726);l.f5b4 = t1cf;l.f5b5 = 0.0;let t1d0: f64 = (l.f10 * l.f71a);let t1d1: f64 = (t1d0 * l.f6c1);let t1d2: f64 = (l.f10 * l.f726);let t1d3: f64 = (t1d1 - t1d2);let t1d4: f64 = (l.f19 * l.f732);let t1d5: f64 = (0.5 * t1d4);let t1d6: f64 = (t1d3 + t1d5);l.f5d4 = t1d6;l.f5d5 = 0.0;let t1d7: f64 = (l.f5b4 - 1.0);let t1d8: f64 = (t1d7 * l.f5a8);l.f7fb = t1d8;l.f7fc = 0.0;let t1d9: f64 = (l.f7fb * l.f7fb);l.f811 = t1d9;l.f812 = 0.0;}
        let t1da: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f3b6 = t1da;l.f3b7 = 0.0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b6 != 0.0)) {let t1db: f64 = (l.f62b * l.f7fb);let t1dc: f64 = (1.0 + t1db);let t1dd: f64 = (1.0 / t1dc);l.f6e2 = t1dd;l.f6e3 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b6 == 0.0)) {let t1de: f64 = (l.f62b * l.f7fb);let t1df: f64 = (1.0 - t1de);let t1e0: f64 = (1.0 / t1df);l.f6e2 = t1e0;l.f6e3 = 0.0;}
        let t1e1: f64 = (-l.f811);let t1e2: f64 = (t1e1 + l.f5d4);let t1e3: f64 = (-230.25850929940458);let t1e4: f64 = if t1e2 > t1e3 { 1.0 } else { 0.0 };l.f3b8 = t1e4;l.f3b9 = 0.0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b8 != 0.0)) {let t1e5: f64 = (-l.f811);let t1e6: f64 = (t1e5 + l.f5d4);let t1e7: f64 = (t1e6).exp();l.f6fc = t1e7;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b8 == 0.0)) {let t1e8: f64 = (-230.25850929940458);let t1e9: f64 = (-l.f811);let t1ea: f64 = (t1e9 + l.f5d4);let t1eb: f64 = (t1e8 - t1ea);let t1ec: f64 = (-230.25850929940458);let t1ed: f64 = (-l.f811);let t1ee: f64 = (t1ed + l.f5d4);let t1ef: f64 = (t1ec - t1ee);let t1f0: f64 = (-230.25850929940458);let t1f1: f64 = (-l.f811);let t1f2: f64 = (t1f1 + l.f5d4);let t1f3: f64 = (t1f0 - t1f2);let t1f4: f64 = (t1f3 * 0.3333333333333333);let t1f5: f64 = (1.0 + t1f4);let t1f6: f64 = (t1ef * t1f5);let t1f7: f64 = (0.5 * t1f6);let t1f8: f64 = (1.0 + t1f7);let t1f9: f64 = (t1eb * t1f8);let t1fa: f64 = (1.0 + t1f9);let t1fb: f64 = (1e-100 / t1fa);l.f6fc = t1fb;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) {let t1fc: f64 = (0.29214664 * l.f6e2);let t1fd: f64 = (l.f6e2 * l.f6e2);let t1fe: f64 = (l.f16 * t1fd);let t1ff: f64 = (t1fc + t1fe);let t200: f64 = (l.f6e2 * l.f6e2);let t201: f64 = (t200 * l.f6e2);let t202: f64 = (l.f2a * t201);let t203: f64 = (t1ff + t202);let t204: f64 = (t203 * l.f6fc);l.f6e = t204;l.f6f = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_170(
        l: &mut StampLocals,
    ) {
        let t205: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f3ba = t205;l.f3bb = 0.0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3ba != 0.0)) {l.f74 = l.f6e;l.f75 = 0.0;}
        let t206: f64 = (-230.25850929940458);let t207: f64 = if l.f5d4 > t206 { 1.0 } else { 0.0 };l.f3bc = t207;l.f3bd = 0.0;
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3ba == 0.0)) && (l.f3bc != 0.0)) {let t208: f64 = (l.f5d4).exp();l.f6fc = t208;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3ba == 0.0)) && (l.f3bc == 0.0)) {let t209: f64 = (-230.25850929940458);let t20a: f64 = (t209 - l.f5d4);let t20b: f64 = (-230.25850929940458);let t20c: f64 = (t20b - l.f5d4);let t20d: f64 = (-230.25850929940458);let t20e: f64 = (t20d - l.f5d4);let t20f: f64 = (t20e * 0.3333333333333333);let t210: f64 = (1.0 + t20f);let t211: f64 = (t20c * t210);let t212: f64 = (0.5 * t211);let t213: f64 = (1.0 + t212);let t214: f64 = (t20a * t213);let t215: f64 = (1.0 + t214);let t216: f64 = (1e-100 / t215);l.f6fc = t216;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3ba == 0.0)) {let t217: f64 = (2.0 * l.f6fc);let t218: f64 = (t217 - l.f6e);l.f74 = t218;l.f75 = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) {let t219: f64 = (1.772453850905516 * 0.5);let t21a: f64 = (l.f10 * l.f74);let t21b: f64 = (t21a / l.f5a8);let t21c: f64 = (t219 * t21b);l.fd6 = t21c;l.fd7 = 0.0;let t21d: f64 = (l.f9 * l.fd6);let t21e: f64 = (t21d * l.f7f5);let t21f: f64 = (l.f41 * t21e);l.f599 = t21f;l.f59a = 0.0;}
        let t220: f64 = if l.f26 == 0.0 { 1.0 } else { 0.0 };l.f3be = t220;l.f3bf = 0.0;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be != 0.0)) {l.f529 = 0.0;l.f52a = 0.0;}
        let t221: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f3c0 = t221;l.f3c1 = 0.0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c0 != 0.0)) {let t222: f64 = (l.f775 - l.f750);let t223: f64 = (t222 * l.f777);let t224: f64 = (t223).sqrt();l.f6fc = t224;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c0 == 0.0)) {let t225: f64 = (l.f775 - l.f750);let t226: f64 = (t225 * l.f777);let t227: f64 = (t226).powf(l.f62f);l.f6fc = t227;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) {let t228: f64 = (l.f775 - l.f750);let t229: f64 = (t228 * l.f7dc);let t22a: f64 = (t229 / l.f6fc);let t22b: f64 = (l.f613 * t22a);l.fb6 = t22b;l.fb7 = 0.0;}
        let t22c: f64 = (-l.fa3);let t22d: f64 = (t22c / l.fb6);let t22e: f64 = (t22d).abs();let t22f: f64 = if t22e < 230.25850929940458 { 1.0 } else { 0.0 };l.f3c2 = t22f;l.f3c3 = 0.0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c2 != 0.0)) {let t230: f64 = (-l.fa3);let t231: f64 = (t230 / l.fb6);let t232: f64 = (t231).exp();l.f6fc = t232;l.f6fd = 0.0;}
        let t233: f64 = (-l.fa3);let t234: f64 = (t233 / l.fb6);let t235: f64 = (-230.25850929940458);let t236: f64 = if t234 < t235 { 1.0 } else { 0.0 };l.f3c4 = t236;l.f3c5 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_171(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c2 == 0.0)) && (l.f3c4 != 0.0)) {let t237: f64 = (-230.25850929940458);let t238: f64 = (-l.fa3);let t239: f64 = (t238 / l.fb6);let t23a: f64 = (t237 - t239);let t23b: f64 = (-230.25850929940458);let t23c: f64 = (-l.fa3);let t23d: f64 = (t23c / l.fb6);let t23e: f64 = (t23b - t23d);let t23f: f64 = (-230.25850929940458);let t240: f64 = (-l.fa3);let t241: f64 = (t240 / l.fb6);let t242: f64 = (t23f - t241);let t243: f64 = (t242 * 0.3333333333333333);let t244: f64 = (1.0 + t243);let t245: f64 = (t23e * t244);let t246: f64 = (0.5 * t245);let t247: f64 = (1.0 + t246);let t248: f64 = (t23a * t247);let t249: f64 = (1.0 + t248);let t24a: f64 = (1e-100 / t249);l.f6fc = t24a;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c2 == 0.0)) && (l.f3c4 == 0.0)) {let t24b: f64 = (-l.fa3);let t24c: f64 = (t24b / l.fb6);let t24d: f64 = (t24c - 230.25850929940458);let t24e: f64 = (-l.fa3);let t24f: f64 = (t24e / l.fb6);let t250: f64 = (t24f - 230.25850929940458);let t251: f64 = (-l.fa3);let t252: f64 = (t251 / l.fb6);let t253: f64 = (t252 - 230.25850929940458);let t254: f64 = (t253 * 0.3333333333333333);let t255: f64 = (1.0 + t254);let t256: f64 = (t250 * t255);let t257: f64 = (0.5 * t256);let t258: f64 = (1.0 + t257);let t259: f64 = (t24d * t258);let t25a: f64 = (1.0 + t259);let t25b: f64 = (1e100 * t25a);l.f6fc = t25b;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) {let t25c: f64 = (l.f73f * l.fb6);let t25d: f64 = (t25c * l.fb6);let t25e: f64 = (t25d * l.f6fc);let t25f: f64 = (l.f26 * t25e);l.f529 = t25f;l.f52a = 0.0;}
        let t260: f64 = if ((l.f785 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f3c6 = t260;l.f3c7 = 0.0;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 != 0.0)) {l.fae = 1.0;l.faf = 0.0;}
        let t261: f64 = (-l.f2);let t262: f64 = (t261 * l.f785);let t263: f64 = if l.f74a > t262 { 1.0 } else { 0.0 };l.f3c8 = t263;l.f3c9 = 0.0;let t264: f64 = if l.f627 == 4.0 { 1.0 } else { 0.0 };l.f3ca = t264;l.f3cb = 0.0;
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 == 0.0)) && (l.f3c8 != 0.0)) && (l.f3ca != 0.0)) {let t265: f64 = (l.f74a * l.f789);let t266: f64 = (t265).abs();let t267: f64 = (l.f74a * l.f789);let t268: f64 = (t267).abs();let t269: f64 = (t266 * t268);let t26a: f64 = (l.f74a * l.f789);let t26b: f64 = (t26a).abs();let t26c: f64 = (t269 * t26b);let t26d: f64 = (l.f74a * l.f789);let t26e: f64 = (t26d).abs();let t26f: f64 = (t26c * t26e);l.f6fc = t26f;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 == 0.0)) && (l.f3c8 != 0.0)) && (l.f3ca == 0.0)) {let t270: f64 = (l.f74a * l.f789);let t271: f64 = (t270).abs();let t272: f64 = (t271).powf(l.f627);l.f6fc = t272;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 == 0.0)) && (l.f3c8 != 0.0)) {let t273: f64 = (1.0 - l.f6fc);let t274: f64 = (1.0 / t273);l.fae = t274;l.faf = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 == 0.0)) && (l.f3c8 == 0.0)) {let t275: f64 = (l.f2 * l.f785);let t276: f64 = (l.f74a + t275);let t277: f64 = (t276 * l.f6bc);let t278: f64 = (l.fc5 + t277);l.fae = t278;l.faf = 0.0;}
        if ((l.f29a != 0.0) && (l.f3a8 == 0.0)) {let t279: f64 = (l.f52f + l.f593);let t27a: f64 = (t279 + l.f599);let t27b: f64 = (t27a + l.f529);let t27c: f64 = (t27b * l.fae);(l.f56e, l.f56f, l.f570, ) = (t27c, (l.f530 * l.fae), (l.f531 * l.fae), );l.f571 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_172(
        l: &mut StampLocals,
    ) {
        if ((l.f29a != 0.0) && (l.f3a8 == 0.0)) {let t27d: f64 = (l.f593 + l.f599);let t27e: f64 = (t27d + l.f529);let t27f: f64 = (t27e * l.fae);(l.f556, l.f557, l.f558, ) = (t27f, 0.0, 0.0, );l.f559 = 0.0;}
        if (l.f29a != 0.0) {let t280: f64 = (l.f0 * l.f562);let t281: f64 = (l.f5b1 * l.f576);let t282: f64 = (t280 + t281);let t283: f64 = (l.f5af * l.f56e);let t284: f64 = (t282 + t283);(l.f520, l.f525, l.f526, ) = (t284, (((l.f0 * l.f563) + (l.f5b1 * l.f577)) + (l.f5af * l.f56f)), (((l.f0 * l.f564) + (l.f5b1 * l.f578)) + (l.f5af * l.f570)), );l.f527 = 0.0;l.f586 = l.f590;l.f587 = 0.0;let t285: f64 = (l.f73d * l.f645);let t286: f64 = (t285 * l.f5c9);let t287: f64 = (t286).exp();let t288: f64 = (t287 - 1.0);let t289: f64 = (l.f586 * t288);let t28a: f64 = (l.f518 - t289);(l.f519, l.f51a, l.f51b, ) = (t28a, l.f51d, l.f51e, );l.f51c = 0.0;let t28b: f64 = (l.f73f * l.f645);let t28c: f64 = (t28b * l.f5c9);let t28d: f64 = (t28c).exp();let t28e: f64 = (t28d - 1.0);let t28f: f64 = (l.f586 * t28e);let t290: f64 = (l.f520 - t28f);(l.f521, l.f522, l.f523, ) = (t290, l.f525, l.f526, );l.f524 = 0.0;}
        let t291: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f3cc = t291;l.f3cd = 0.0;let t292: f64 = if ((l.f518 > 0.0) && (l.f520 > 0.0)) { 1.0 } else { 0.0 };l.f3ce = t292;l.f3cf = 0.0;let t293: f64 = (l.f519 / l.f518);let t294: f64 = (l.f521 / l.f520);let t295: f64 = if ((((t293 > 0.001) || (t294 > 0.001)) && (l.f519 > 0.0)) && (l.f521 > 0.0)) { 1.0 } else { 0.0 };l.f3d0 = t295;l.f3d1 = 0.0;
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3ce != 0.0)) && (l.f3d0 != 0.0)) {let t296: f64 = (l.f519 / l.f521);(l.f4, l.f5, l.f6, ) = (t296, (((l.f51a * l.f521) - (l.f519 * l.f522)) / (l.f521 * l.f521)), (((l.f51b * l.f521) - (l.f519 * l.f523)) / (l.f521 * l.f521)), );l.f7 = 0.0;let t297: f64 = (l.f4).ln();let t298: f64 = (l.f643 * t297);let t299: f64 = (l.f73d - l.f73f);let t29a: f64 = (t298 / t299);(l.f5cb, l.f5cc, l.f5cd, ) = (t29a, ((l.f643 * (l.f5 / l.f4)) / t299), ((l.f643 * (l.f6 / l.f4)) / t299), );l.f5ce = 0.0;let t29b: f64 = (l.f73d * l.f645);let t29c: f64 = (t29b * l.f5cb);let t29d: f64 = (t29c).exp();let t29e: f64 = (t29d - 1.0);let t29f: f64 = (l.f519 / t29e);(l.f588, l.f589, l.f58a, ) = (t29f, (((l.f51a * t29e) - (l.f519 * (t29d * (t29b * l.f5cc)))) / (t29e * t29e)), (((l.f51b * t29e) - (l.f519 * (t29d * (t29b * l.f5cd)))) / (t29e * t29e)), );l.f58b = 0.0;}
        if ((l.f29a != 0.0) && (l.f3cc != 0.0)) {let t2a0: f64 = (l.f737 * l.f645);let t2a1: f64 = (t2a0 * l.f5c9);let t2a2: f64 = (t2a1).exp();let t2a3: f64 = (t2a2 - 1.0);let t2a4: f64 = (l.f586 * t2a3);let t2a5: f64 = (l.f500 - t2a4);let t2a6: f64 = (l.f737 * l.f645);let t2a7: f64 = (t2a6 * l.f5cb);let t2a8: f64 = (t2a7).exp();let t2a9: f64 = (t2a8 - 1.0);let t2aa: f64 = (l.f588 * t2a9);let t2ab: f64 = (t2a5 - t2aa);(l.f501, l.f502, l.f503, ) = (t2ab, (l.f505 - ((l.f589 * t2a9) + (l.f588 * (t2a8 * (t2a6 * l.f5cc))))), (l.f506 - ((l.f58a * t2a9) + (l.f588 * (t2a8 * (t2a6 * l.f5cd))))), );l.f504 = 0.0;let t2ac: f64 = (l.f739 * l.f645);let t2ad: f64 = (t2ac * l.f5c9);let t2ae: f64 = (t2ad).exp();let t2af: f64 = (t2ae - 1.0);let t2b0: f64 = (l.f586 * t2af);let t2b1: f64 = (l.f508 - t2b0);let t2b2: f64 = (l.f739 * l.f645);let t2b3: f64 = (t2b2 * l.f5cb);let t2b4: f64 = (t2b3).exp();let t2b5: f64 = (t2b4 - 1.0);let t2b6: f64 = (l.f588 * t2b5);let t2b7: f64 = (t2b1 - t2b6);(l.f509, l.f50a, l.f50b, ) = (t2b7, (l.f50d - ((l.f589 * t2b5) + (l.f588 * (t2b4 * (t2b2 * l.f5cc))))), (l.f50e - ((l.f58a * t2b5) + (l.f588 * (t2b4 * (t2b2 * l.f5cd))))), );l.f50c = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_173(
        l: &mut StampLocals,
    ) {
        if ((l.f29a != 0.0) && (l.f3cc != 0.0)) {let t2b8: f64 = (l.f73b * l.f645);let t2b9: f64 = (t2b8 * l.f5c9);let t2ba: f64 = (t2b9).exp();let t2bb: f64 = (t2ba - 1.0);let t2bc: f64 = (l.f586 * t2bb);let t2bd: f64 = (l.f510 - t2bc);let t2be: f64 = (l.f73b * l.f645);let t2bf: f64 = (t2be * l.f5cb);let t2c0: f64 = (t2bf).exp();let t2c1: f64 = (t2c0 - 1.0);let t2c2: f64 = (l.f588 * t2c1);let t2c3: f64 = (t2bd - t2c2);(l.f511, l.f512, l.f513, ) = (t2c3, (l.f515 - ((l.f589 * t2c1) + (l.f588 * (t2c0 * (t2be * l.f5cc))))), (l.f516 - ((l.f58a * t2c1) + (l.f588 * (t2c0 * (t2be * l.f5cd))))), );l.f514 = 0.0;}
        let t2c4: f64 = if (((l.f500 < 0.0) && (l.f508 < 0.0)) && (l.f510 < 0.0)) { 1.0 } else { 0.0 };l.f3d2 = t2c4;l.f3d3 = 0.0;let t2c5: f64 = (l.f501 / l.f500);let t2c6: f64 = (l.f509 / l.f508);let t2c7: f64 = (l.f511 / l.f510);let t2c8: f64 = if ((((((t2c5 > 0.001) || (t2c6 > 0.001)) || (t2c7 > 0.001)) && (l.f501 < 0.0)) && (l.f509 < 0.0)) && (l.f511 < 0.0)) { 1.0 } else { 0.0 };l.f3d4 = t2c8;l.f3d5 = 0.0;
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t2c9: f64 = (l.f501 / l.f509);(l.f4, l.f5, l.f6, ) = (t2c9, (((l.f502 * l.f509) - (l.f501 * l.f50a)) / (l.f509 * l.f509)), (((l.f503 * l.f509) - (l.f501 * l.f50b)) / (l.f509 * l.f509)), );l.f7 = 0.0;let t2ca: f64 = (-l.f643);let t2cb: f64 = (l.f4).ln();let t2cc: f64 = (t2ca * t2cb);let t2cd: f64 = (l.f737 - l.f739);let t2ce: f64 = (t2cc / t2cd);(l.f5b9, l.f5ba, l.f5bb, ) = (t2ce, ((t2ca * (l.f5 / l.f4)) / t2cd), ((t2ca * (l.f6 / l.f4)) / t2cd), );l.f5bc = 0.0;let t2cf: f64 = (l.f739 - l.f737);let t2d0: f64 = (l.f739 / t2cf);l.f707 = t2d0;l.f708 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t2d1: f64 = (l.f4 - 1.0);let t2d2: f64 = (l.f643 * t2d1);let t2d3: f64 = (l.f4).powf(l.f707);let t2d4: f64 = (t2d3 - 1.0);let t2d5: f64 = (t2d2 * t2d4);(l.f709, l.f70a, l.f70b, ) = (t2d5, (((l.f643 * l.f5) * t2d4) + (t2d2 * if 0.0 == 0.0 && ((l.f707) as f64).is_finite() && ((l.f707) as f64).fract() == 0.0 { if l.f707 == 0.0 { 0.0 } else { (l.f707 * ((l.f4).powf(l.f707 - 1.0) * l.f5)) } } else { (t2d3 * (l.f707 * (l.f5 / l.f4))) })), (((l.f643 * l.f6) * t2d4) + (t2d2 * if 0.0 == 0.0 && ((l.f707) as f64).is_finite() && ((l.f707) as f64).fract() == 0.0 { if l.f707 == 0.0 { 0.0 } else { (l.f707 * ((l.f4).powf(l.f707 - 1.0) * l.f6)) } } else { (t2d3 * (l.f707 * (l.f6 / l.f4))) })), );l.f70c = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t2d6: f64 = (l.f737 - l.f739);let t2d7: f64 = (l.f737 / t2d6);l.f707 = t2d7;l.f708 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t2d8: f64 = (l.f4).powf(l.f707);let t2d9: f64 = (l.f739 - l.f737);let t2da: f64 = (t2d8 * t2d9);let t2db: f64 = (l.f4 * l.f737);let t2dc: f64 = (t2da + t2db);let t2dd: f64 = (t2dc - l.f739);(l.f70d, l.f70e, l.f70f, ) = (t2dd, ((if 0.0 == 0.0 && ((l.f707) as f64).is_finite() && ((l.f707) as f64).fract() == 0.0 { if l.f707 == 0.0 { 0.0 } else { (l.f707 * ((l.f4).powf(l.f707 - 1.0) * l.f5)) } } else { (t2d8 * (l.f707 * (l.f5 / l.f4))) } * t2d9) + (l.f5 * l.f737)), ((if 0.0 == 0.0 && ((l.f707) as f64).is_finite() && ((l.f707) as f64).fract() == 0.0 { if l.f707 == 0.0 { 0.0 } else { (l.f707 * ((l.f4).powf(l.f707 - 1.0) * l.f6)) } } else { (t2d8 * (l.f707 * (l.f6 / l.f4))) } * t2d9) + (l.f6 * l.f737)), );l.f710 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t2de: f64 = (l.f709 / l.f70d);(l.f5bf, l.f5c0, l.f5c1, ) = (t2de, (((l.f70a * l.f70d) - (l.f709 * l.f70e)) / (l.f70d * l.f70d)), (((l.f70b * l.f70d) - (l.f709 * l.f70f)) / (l.f70d * l.f70d)), );l.f5c2 = 0.0;let t2df: f64 = (l.f5b9 + l.f5bf);(l.f5cf, l.f5d0, l.f5d1, ) = (t2df, (l.f5ba + l.f5c0), (l.f5bb + l.f5c1), );l.f5d2 = 0.0;}
        let t2e0: f64 = (l.f73b * l.f645);let t2e1: f64 = (t2e0 * l.f5cf);let t2e2: f64 = (t2e1).abs();let t2e3: f64 = if t2e2 < 1e-6 { 1.0 } else { 0.0 };l.f3d6 = t2e3;l.f3d7 = 0.0;
        if (((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) && (l.f3d6 != 0.0)) {l.f5bd = 1.0;l.f5be = 0.0;let t2e4: f64 = (1.0 / l.f73b);let t2e5: f64 = (0.5 * l.f645);let t2e6: f64 = (t2e5 * l.f5cf);let t2e7: f64 = (t2e4 + t2e6);let t2e8: f64 = (l.f511 * t2e7);(l.f58c, l.f58d, l.f58e, ) = (t2e8, ((l.f512 * t2e7) + (l.f511 * (t2e5 * l.f5d0))), ((l.f513 * t2e7) + (l.f511 * (t2e5 * l.f5d1))), );l.f58f = 0.0;let t2e9: f64 = (-0.5);let t2ea: f64 = (t2e9 * l.f511);let t2eb: f64 = (t2ea * l.f5cf);let t2ec: f64 = (t2eb * l.f645);let t2ed: f64 = (t2ec / l.f73b);(l.f5cf, l.f5d0, l.f5d1, ) = (t2ed, (((((t2e9 * l.f512) * l.f5cf) + (t2ea * l.f5d0)) * l.f645) / l.f73b), (((((t2e9 * l.f513) * l.f5cf) + (t2ea * l.f5d1)) * l.f645) / l.f73b), );l.f5d2 = 0.0;}
        if (((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) && (l.f3d6 == 0.0)) {l.f5bd = 0.0;l.f5be = 0.0;let t2ee: f64 = (-l.f511);let t2ef: f64 = (-l.f73b);let t2f0: f64 = (t2ef * l.f645);let t2f1: f64 = (t2f0 * l.f5cf);let t2f2: f64 = (t2f1).exp();let t2f3: f64 = (t2f2 - 1.0);let t2f4: f64 = (t2ee / t2f3);(l.f58c, l.f58d, l.f58e, ) = (t2f4, ((((-l.f512) * t2f3) - (t2ee * (t2f2 * (t2f0 * l.f5d0)))) / (t2f3 * t2f3)), ((((-l.f513) * t2f3) - (t2ee * (t2f2 * (t2f0 * l.f5d1)))) / (t2f3 * t2f3)), );l.f58f = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_174(
        ctx: &GeneratedEvalContext<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);
        if (l.f29a != 0.0) {let t2f5: f64 = (l.f0 * l.f2c);let t2f6: f64 = (l.f5b1 * l.f36);let t2f7: f64 = (t2f5 + t2f6);let t2f8: f64 = (l.f5af * l.f2e);let t2f9: f64 = (t2f7 + t2f8);let t2fa: f64 = (l.fb3 * t2f9);l.f822 = t2fa;l.f823 = 0.0;}
        let t2fb: f64 = (l.f0 * l.f2c);let t2fc: f64 = if t2fb <= l.f822 { 1.0 } else { 0.0 };l.f3d8 = t2fc;l.f3d9 = 0.0;
        if ((l.f29a != 0.0) && (l.f3d8 != 0.0)) {l.f81c = 0.0;l.f81d = 0.0;}
        let t2fd: f64 = (l.f5b1 * l.f36);let t2fe: f64 = if t2fd <= l.f822 { 1.0 } else { 0.0 };l.f3da = t2fe;l.f3db = 0.0;
        if ((l.f29a != 0.0) && (l.f3da != 0.0)) {l.f820 = 0.0;l.f821 = 0.0;}
        let t2ff: f64 = (l.f5af * l.f2e);let t300: f64 = if t2ff <= l.f822 { 1.0 } else { 0.0 };l.f3dc = t300;l.f3dd = 0.0;
        if ((l.f29a != 0.0) && (l.f3dc != 0.0)) {l.f81e = 0.0;l.f81f = 0.0;}
        let t301: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f3de = t301;l.f3df = 0.0;
        if ((l.f29a != 0.0) && (l.f3de != 0.0)) {let t302: f64 = (0.5 * l.f57a);let t303: f64 = (l.f586 + 1e-21);let t304: f64 = (t302 / t303);let t305: f64 = (t304).ln();l.f800 = t305;l.f801 = 0.0;let t306: f64 = (0.5 * l.f57a);let t307: f64 = (l.f588 + 1e-21);let t308: f64 = (t306 / t307);let t309: f64 = (t308).ln();(l.f802, l.f803, l.f804, ) = (t309, ((-((t306 * l.f589) / (t307 * t307))) / t308), ((-((t306 * l.f58a) / (t307 * t307))) / t308), );l.f805 = 0.0;}
        if ((l.f29a != 0.0) && (l.f3de != 0.0)) {let t30a: f64 = (0.5 * l.f57a);let t30b: f64 = (l.f58c).abs();let t30c: f64 = (t30b + 1e-21);let t30d: f64 = (t30a / t30c);let t30e: f64 = (t30d).ln();(l.f806, l.f807, l.f808, ) = (t30e, ((-((t30a * if l.f58c >= 0.0 { l.f58d } else { (-l.f58d) }) / (t30c * t30c))) / t30d), ((-((t30a * if l.f58c >= 0.0 { l.f58e } else { (-l.f58e) }) / (t30c * t30c))) / t30d), );l.f809 = 0.0;}
        if (l.f29a != 0.0) {let t30f: f64 = (l.f800).min(230.25850929940458);l.f800 = t30f;l.f801 = 0.0;let t310: f64 = (l.f800).exp();l.f97 = t310;l.f98 = 0.0;}
        if (l.f29a != 0.0) {let t311: f64 = (l.f802).min(230.25850929940458);(l.f802, l.f803, l.f804, ) = (t311, if l.f802 <= 230.25850929940458 { l.f803 } else { 0.0 }, if l.f802 <= 230.25850929940458 { l.f804 } else { 0.0 }, );l.f805 = 0.0;}
        if (l.f29a != 0.0) {let t312: f64 = (l.f802).exp();(l.f99, l.f9a, l.f9b, ) = (t312, (t312 * l.f803), (t312 * l.f804), );l.f9c = 0.0;}
        if (l.f29a != 0.0) {let t313: f64 = (l.f806).min(230.25850929940458);(l.f806, l.f807, l.f808, ) = (t313, if l.f806 <= 230.25850929940458 { l.f807 } else { 0.0 }, if l.f806 <= 230.25850929940458 { l.f808 } else { 0.0 }, );l.f809 = 0.0;}
        if (l.f29a != 0.0) {let t314: f64 = (l.f806).exp();(l.f9d, l.f9e, l.f9f, ) = (t314, (t314 * l.f807), (t314 * l.f808), );l.fa0 = 0.0;}
        (l.f745, l.f746, l.f747, ) = ((nv0 - nv2), 1.0, -1.0, );l.f748 = 0.0;let t315: f64 = if l.f6d9 == 1.0 { 1.0 } else { 0.0 };l.f3e0 = t315;l.f3e1 = 0.0;
        if (l.f3e0 != 0.0) {let t316: f64 = (l.f745 * l.f645);let t317: f64 = (t316 * l.f5c9);(l.f6eb, l.f6ec, l.f6ed, ) = (t317, ((l.f746 * l.f645) * l.f5c9), ((l.f747 * l.f645) * l.f5c9), );l.f6ee = 0.0;}
        if (l.f3e0 != 0.0) {
            let t318: f64 = (-230.25850929940458);
            let (t324, t325, t326,) = {
    if (l.f6eb < t318) {
        let t319: f64 = (-230.25850929940458);let t31a: f64 = (t319 - l.f6eb);let t31b: f64 = (t31a + 1.0);let t31c: f64 = (1e-100 / t31b);
        (t31c, (-((1e-100 * (-l.f6ec)) / (t31b * t31b))), (-((1e-100 * (-l.f6ed)) / (t31b * t31b))),)
    } else {
        let (t321, t322, t323,) = {
            if (l.f6eb > l.f800) {
                let t31d: f64 = (l.f6eb - l.f800);let t31e: f64 = (t31d + 1.0);let t31f: f64 = (l.f97 * t31e);
                (t31f, (l.f97 * l.f6ec), (l.f97 * l.f6ed),)
            } else {
                let t320: f64 = (l.f6eb).exp();
                (t320, (t320 * l.f6ec), (t320 * l.f6ed),)
            }
        };
        (t321, t322, t323,)
    }
};
            (l.f6ef, l.f6f0, l.f6f1, ) = (t324, t325, t326, );l.f6f2 = 0.0;
        }
        if (l.f3e0 != 0.0) {let t327: f64 = (l.f6ef - 1.0);let t328: f64 = (l.f586 * t327);(l.f566, l.f567, l.f568, ) = (t328, (l.f586 * l.f6f0), (l.f586 * l.f6f1), );l.f569 = 0.0;let t329: f64 = (l.f745 * l.f645);let t32a: f64 = (t329 * l.f5cb);(l.f6eb, l.f6ec, l.f6ed, ) = (t32a, (((l.f746 * l.f645) * l.f5cb) + (t329 * l.f5cc)), (((l.f747 * l.f645) * l.f5cb) + (t329 * l.f5cd)), );l.f6ee = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_175(
        l: &mut StampLocals,
    ) {
        if (l.f3e0 != 0.0) {
            let t32b: f64 = (-230.25850929940458);
            let (t337, t338, t339,) = {
    if (l.f6eb < t32b) {
        let t32c: f64 = (-230.25850929940458);let t32d: f64 = (t32c - l.f6eb);let t32e: f64 = (t32d + 1.0);let t32f: f64 = (1e-100 / t32e);
        (t32f, (-((1e-100 * (-l.f6ec)) / (t32e * t32e))), (-((1e-100 * (-l.f6ed)) / (t32e * t32e))),)
    } else {
        let (t334, t335, t336,) = {
            if (l.f6eb > l.f802) {
                let t330: f64 = (l.f6eb - l.f802);let t331: f64 = (t330 + 1.0);let t332: f64 = (l.f99 * t331);
                (t332, ((l.f9a * t331) + (l.f99 * (l.f6ec - l.f803))), ((l.f9b * t331) + (l.f99 * (l.f6ed - l.f804))),)
            } else {
                let t333: f64 = (l.f6eb).exp();
                (t333, (t333 * l.f6ec), (t333 * l.f6ed),)
            }
        };
        (t334, t335, t336,)
    }
};
            (l.f6ef, l.f6f0, l.f6f1, ) = (t337, t338, t339, );l.f6f2 = 0.0;
        }
        if (l.f3e0 != 0.0) {let t33a: f64 = (l.f6ef - 1.0);let t33b: f64 = (l.f588 * t33a);(l.f56a, l.f56b, l.f56c, ) = (t33b, ((l.f589 * t33a) + (l.f588 * l.f6f0)), ((l.f58a * t33a) + (l.f588 * l.f6f1)), );l.f56d = 0.0;(l.f572, l.f573, l.f574, ) = (0.0, 0.0, 0.0, );l.f575 = 0.0;}
        let t33c: f64 = if l.f5bd > 0.0 { 1.0 } else { 0.0 };l.f3e2 = t33c;l.f3e3 = 0.0;
        if ((l.f3e0 != 0.0) && (l.f3e2 != 0.0)) {let t33d: f64 = (l.f745 * l.f5cf);let t33e: f64 = (l.f58c + t33d);let t33f: f64 = (l.f745 * t33e);(l.f572, l.f573, l.f574, ) = (t33f, ((l.f746 * t33e) + (l.f745 * (l.f58d + ((l.f746 * l.f5cf) + (l.f745 * l.f5d0))))), ((l.f747 * t33e) + (l.f745 * (l.f58e + ((l.f747 * l.f5cf) + (l.f745 * l.f5d1))))), );l.f575 = 0.0;}
        if ((l.f3e0 != 0.0) && (l.f3e2 == 0.0)) {let t340: f64 = (-l.f745);let t341: f64 = (t340 * l.f645);let t342: f64 = (t341 * l.f5cf);(l.f6eb, l.f6ec, l.f6ed, ) = (t342, ((((-l.f746) * l.f645) * l.f5cf) + (t341 * l.f5d0)), ((((-l.f747) * l.f645) * l.f5cf) + (t341 * l.f5d1)), );l.f6ee = 0.0;}
        if ((l.f3e0 != 0.0) && (l.f3e2 == 0.0)) {
            let t343: f64 = (-230.25850929940458);
            let (t34f, t350, t351,) = {
    if (l.f6eb < t343) {
        let t344: f64 = (-230.25850929940458);let t345: f64 = (t344 - l.f6eb);let t346: f64 = (t345 + 1.0);let t347: f64 = (1e-100 / t346);
        (t347, (-((1e-100 * (-l.f6ec)) / (t346 * t346))), (-((1e-100 * (-l.f6ed)) / (t346 * t346))),)
    } else {
        let (t34c, t34d, t34e,) = {
            if (l.f6eb > l.f806) {
                let t348: f64 = (l.f6eb - l.f806);let t349: f64 = (t348 + 1.0);let t34a: f64 = (l.f9d * t349);
                (t34a, ((l.f9e * t349) + (l.f9d * (l.f6ec - l.f807))), ((l.f9f * t349) + (l.f9d * (l.f6ed - l.f808))),)
            } else {
                let t34b: f64 = (l.f6eb).exp();
                (t34b, (t34b * l.f6ec), (t34b * l.f6ed),)
            }
        };
        (t34c, t34d, t34e,)
    }
};
            (l.f6ef, l.f6f0, l.f6f1, ) = (t34f, t350, t351, );l.f6f2 = 0.0;
        }
        if ((l.f3e0 != 0.0) && (l.f3e2 == 0.0)) {let t352: f64 = (-l.f58c);let t353: f64 = (l.f6ef - 1.0);let t354: f64 = (t352 * t353);(l.f572, l.f573, l.f574, ) = (t354, (((-l.f58d) * t353) + (t352 * l.f6f0)), (((-l.f58e) * t353) + (t352 * l.f6f1)), );l.f575 = 0.0;}
        if (l.f3e0 != 0.0) {let t355: f64 = (l.f566 + l.f56a);let t356: f64 = (t355 + l.f572);(l.f55e, l.f55f, l.f560, ) = (t356, ((l.f567 + l.f56b) + l.f573), ((l.f568 + l.f56c) + l.f574), );l.f561 = 0.0;let t357: f64 = (l.f56a + l.f572);(l.f54e, l.f54f, l.f550, ) = (t357, (l.f56b + l.f573), (l.f56c + l.f574), );l.f551 = 0.0;(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );l.f565 = 0.0;(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );l.f579 = 0.0;(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );l.f571 = 0.0;(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );l.f555 = 0.0;(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );l.f55d = 0.0;(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );l.f559 = 0.0;let t358: f64 = (4.0 * l.f78f);let t359: f64 = (t358 * l.f78f);l.f4e0 = t359;l.f4e3 = 0.0;let t35a: f64 = (l.f78f / l.f791);l.f4e4 = t35a;l.f4e7 = 0.0;let t35b: f64 = (l.f78f * l.f4e4);let t35c: f64 = (l.f745 + t35b);(l.f4e8, l.f4eb, l.f4ec, ) = (t35c, l.f746, l.f747, );l.f4ed = 0.0;let t35d: f64 = (l.f791 + l.f4e8);(l.f4ee, l.f4f1, l.f4f2, ) = (t35d, l.f4eb, l.f4ec, );l.f4f3 = 0.0;let t35e: f64 = (l.f791 - l.f4e8);(l.f4f4, l.f4f7, l.f4f8, ) = (t35e, (-l.f4eb), (-l.f4ec), );l.f4f9 = 0.0;let t35f: f64 = (l.f4f4 * l.f4f4);let t360: f64 = (t35f + l.f4e0);let t361: f64 = (t360).sqrt();(l.f4fa, l.f4fd, l.f4fe, ) = (t361, (((l.f4f7 * l.f4f4) + (l.f4f4 * l.f4f7)) / (2.0 * t361)), (((l.f4f8 * l.f4f4) + (l.f4f4 * l.f4f8)) / (2.0 * t361)), );l.f4ff = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_176(
        l: &mut StampLocals,
    ) {
        if (l.f3e0 != 0.0) {let t362: f64 = (l.f745 * l.f791);let t363: f64 = (l.f4ee + l.f4fa);let t364: f64 = (t362 / t363);let t365: f64 = (2.0 * t364);(l.f7ad, l.f7ae, l.f7af, ) = (t365, (2.0 * ((((l.f746 * l.f791) * t363) - (t362 * (l.f4f1 + l.f4fd))) / (t363 * t363))), (2.0 * ((((l.f747 * l.f791) * t363) - (t362 * (l.f4f2 + l.f4fe))) / (t363 * t363))), );l.f7b0 = 0.0;}
        let t366: f64 = if l.f81c > 0.5 { 1.0 } else { 0.0 };l.f3e4 = t366;l.f3e5 = 0.0;let t367: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f3e6 = t367;l.f3e7 = 0.0;
        if (((l.f3e0 != 0.0) && (l.f3e4 != 0.0)) && (l.f3e6 != 0.0)) {let t368: f64 = (l.f7ad * l.f769);let t369: f64 = (1.0 - t368);let t36a: f64 = (t369).sqrt();(l.f701, l.f702, l.f703, ) = (t36a, ((-(l.f7ae * l.f769)) / (2.0 * t36a)), ((-(l.f7af * l.f769)) / (2.0 * t36a)), );l.f704 = 0.0;}
        if (((l.f3e0 != 0.0) && (l.f3e4 != 0.0)) && (l.f3e6 == 0.0)) {let t36b: f64 = (l.f7ad * l.f769);let t36c: f64 = (1.0 - t36b);let t36d: f64 = (t36c).powf(l.f60b);(l.f701, l.f702, l.f703, ) = (t36d, if 0.0 == 0.0 && ((l.f60b) as f64).is_finite() && ((l.f60b) as f64).fract() == 0.0 { if l.f60b == 0.0 { 0.0 } else { (l.f60b * ((t36c).powf(l.f60b - 1.0) * (-(l.f7ae * l.f769)))) } } else { (t36d * (l.f60b * ((-(l.f7ae * l.f769)) / t36c))) }, if 0.0 == 0.0 && ((l.f60b) as f64).is_finite() && ((l.f60b) as f64).fract() == 0.0 { if l.f60b == 0.0 { 0.0 } else { (l.f60b * ((t36c).powf(l.f60b - 1.0) * (-(l.f7af * l.f769)))) } } else { (t36d * (l.f60b * ((-(l.f7af * l.f769)) / t36c))) }, );l.f704 = 0.0;}
        if ((l.f3e0 != 0.0) && (l.f3e4 != 0.0)) {let t36e: f64 = (1.0 - l.f701);let t36f: f64 = (l.f69e * t36e);let t370: f64 = (l.f745 - l.f7ad);let t371: f64 = (l.f698 * t370);let t372: f64 = (t36f + t371);(l.f68c, l.f68d, l.f68e, ) = (t372, ((l.f69e * (-l.f702)) + (l.f698 * (l.f746 - l.f7ae))), ((l.f69e * (-l.f703)) + (l.f698 * (l.f747 - l.f7af))), );l.f68f = 0.0;}
        if ((l.f3e0 != 0.0) && (l.f3e4 == 0.0)) {(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );l.f68f = 0.0;}
        let t373: f64 = if l.f820 > 0.5 { 1.0 } else { 0.0 };l.f3e8 = t373;l.f3e9 = 0.0;let t374: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f3ea = t374;l.f3eb = 0.0;
        if (((l.f3e0 != 0.0) && (l.f3e8 != 0.0)) && (l.f3ea != 0.0)) {let t375: f64 = (l.f7ad * l.f76d);let t376: f64 = (1.0 - t375);let t377: f64 = (t376).sqrt();(l.f701, l.f702, l.f703, ) = (t377, ((-(l.f7ae * l.f76d)) / (2.0 * t377)), ((-(l.f7af * l.f76d)) / (2.0 * t377)), );l.f704 = 0.0;}
        if (((l.f3e0 != 0.0) && (l.f3e8 != 0.0)) && (l.f3ea == 0.0)) {let t378: f64 = (l.f7ad * l.f76d);let t379: f64 = (1.0 - t378);let t37a: f64 = (t379).powf(l.f60f);(l.f701, l.f702, l.f703, ) = (t37a, if 0.0 == 0.0 && ((l.f60f) as f64).is_finite() && ((l.f60f) as f64).fract() == 0.0 { if l.f60f == 0.0 { 0.0 } else { (l.f60f * ((t379).powf(l.f60f - 1.0) * (-(l.f7ae * l.f76d)))) } } else { (t37a * (l.f60f * ((-(l.f7ae * l.f76d)) / t379))) }, if 0.0 == 0.0 && ((l.f60f) as f64).is_finite() && ((l.f60f) as f64).fract() == 0.0 { if l.f60f == 0.0 { 0.0 } else { (l.f60f * ((t379).powf(l.f60f - 1.0) * (-(l.f7af * l.f76d)))) } } else { (t37a * (l.f60f * ((-(l.f7af * l.f76d)) / t379))) }, );l.f704 = 0.0;}
        if ((l.f3e0 != 0.0) && (l.f3e8 != 0.0)) {let t37b: f64 = (1.0 - l.f701);let t37c: f64 = (l.f6a2 * t37b);let t37d: f64 = (l.f745 - l.f7ad);let t37e: f64 = (l.f69c * t37d);let t37f: f64 = (t37c + t37e);(l.f694, l.f695, l.f696, ) = (t37f, ((l.f6a2 * (-l.f702)) + (l.f69c * (l.f746 - l.f7ae))), ((l.f6a2 * (-l.f703)) + (l.f69c * (l.f747 - l.f7af))), );l.f697 = 0.0;}
        if ((l.f3e0 != 0.0) && (l.f3e8 == 0.0)) {(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );l.f697 = 0.0;}
        let t380: f64 = if l.f81e > 0.5 { 1.0 } else { 0.0 };l.f3ec = t380;l.f3ed = 0.0;let t381: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f3ee = t381;l.f3ef = 0.0;
        if (((l.f3e0 != 0.0) && (l.f3ec != 0.0)) && (l.f3ee != 0.0)) {let t382: f64 = (l.f7ad * l.f76b);let t383: f64 = (1.0 - t382);let t384: f64 = (t383).sqrt();(l.f701, l.f702, l.f703, ) = (t384, ((-(l.f7ae * l.f76b)) / (2.0 * t384)), ((-(l.f7af * l.f76b)) / (2.0 * t384)), );l.f704 = 0.0;}
        if (((l.f3e0 != 0.0) && (l.f3ec != 0.0)) && (l.f3ee == 0.0)) {let t385: f64 = (l.f7ad * l.f76b);let t386: f64 = (1.0 - t385);let t387: f64 = (t386).powf(l.f60d);(l.f701, l.f702, l.f703, ) = (t387, if 0.0 == 0.0 && ((l.f60d) as f64).is_finite() && ((l.f60d) as f64).fract() == 0.0 { if l.f60d == 0.0 { 0.0 } else { (l.f60d * ((t386).powf(l.f60d - 1.0) * (-(l.f7ae * l.f76b)))) } } else { (t387 * (l.f60d * ((-(l.f7ae * l.f76b)) / t386))) }, if 0.0 == 0.0 && ((l.f60d) as f64).is_finite() && ((l.f60d) as f64).fract() == 0.0 { if l.f60d == 0.0 { 0.0 } else { (l.f60d * ((t386).powf(l.f60d - 1.0) * (-(l.f7af * l.f76b)))) } } else { (t387 * (l.f60d * ((-(l.f7af * l.f76b)) / t386))) }, );l.f704 = 0.0;}
        if ((l.f3e0 != 0.0) && (l.f3ec != 0.0)) {let t388: f64 = (1.0 - l.f701);let t389: f64 = (l.f6a0 * t388);let t38a: f64 = (l.f745 - l.f7ad);let t38b: f64 = (l.f69a * t38a);let t38c: f64 = (t389 + t38b);(l.f690, l.f691, l.f692, ) = (t38c, ((l.f6a0 * (-l.f702)) + (l.f69a * (l.f746 - l.f7ae))), ((l.f6a0 * (-l.f703)) + (l.f69a * (l.f747 - l.f7af))), );l.f693 = 0.0;}
        if ((l.f3e0 != 0.0) && (l.f3ec == 0.0)) {(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );l.f693 = 0.0;}
        let t38d: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f3f0 = t38d;l.f3f1 = 0.0;
        if ((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) {let t38e: f64 = (4.0 * l.f78f);let t38f: f64 = (t38e * l.f78f);l.f4e0 = t38f;l.f4e3 = 0.0;let t390: f64 = (l.f78f / l.f791);l.f4e4 = t390;l.f4e7 = 0.0;let t391: f64 = (l.f78f * l.f4e4);let t392: f64 = (l.f745 + t391);(l.f4e8, l.f4eb, l.f4ec, ) = (t392, l.f746, l.f747, );l.f4ed = 0.0;}
    }
}
