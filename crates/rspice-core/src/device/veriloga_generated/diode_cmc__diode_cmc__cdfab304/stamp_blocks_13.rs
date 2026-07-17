#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.feb != 0.0)) {let t0: f64 = (-l.f811);let t1: f64 = (t0 + l.f5d4);let t2: f64 = (t1).exp();l.f6fc = t2;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.feb == 0.0)) {let t3: f64 = (-230.25850929940458);let t4: f64 = (-l.f811);let t5: f64 = (t4 + l.f5d4);let t6: f64 = (t3 - t5);let t7: f64 = (-230.25850929940458);let t8: f64 = (-l.f811);let t9: f64 = (t8 + l.f5d4);let ta: f64 = (t7 - t9);let tb: f64 = (-230.25850929940458);let tc: f64 = (-l.f811);let td: f64 = (tc + l.f5d4);let te: f64 = (tb - td);let tf: f64 = (te * 0.3333333333333333);let t10: f64 = (1.0 + tf);let t11: f64 = (ta * t10);let t12: f64 = (0.5 * t11);let t13: f64 = (1.0 + t12);let t14: f64 = (t6 * t13);let t15: f64 = (1.0 + t14);let t16: f64 = (1e-100 / t15);l.f6fc = t16;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t17: f64 = (0.29214664 * l.f6e2);let t18: f64 = (l.f6e2 * l.f6e2);let t19: f64 = (l.f16 * t18);let t1a: f64 = (t17 + t19);let t1b: f64 = (l.f6e2 * l.f6e2);let t1c: f64 = (t1b * l.f6e2);let t1d: f64 = (l.f2a * t1c);let t1e: f64 = (t1a + t1d);let t1f: f64 = (t1e * l.f6fc);l.f6e = t1f;l.f6f = 0.0;}
        let t20: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.fed = t20;l.fee = 0.0;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fed != 0.0)) {l.f74 = l.f6e;l.f75 = 0.0;}
        let t21: f64 = (-230.25850929940458);let t22: f64 = if l.f5d4 > t21 { 1.0 } else { 0.0 };l.fef = t22;l.ff0 = 0.0;
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fed == 0.0)) && (l.fef != 0.0)) {let t23: f64 = (l.f5d4).exp();l.f6fc = t23;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fed == 0.0)) && (l.fef == 0.0)) {let t24: f64 = (-230.25850929940458);let t25: f64 = (t24 - l.f5d4);let t26: f64 = (-230.25850929940458);let t27: f64 = (t26 - l.f5d4);let t28: f64 = (-230.25850929940458);let t29: f64 = (t28 - l.f5d4);let t2a: f64 = (t29 * 0.3333333333333333);let t2b: f64 = (1.0 + t2a);let t2c: f64 = (t27 * t2b);let t2d: f64 = (0.5 * t2c);let t2e: f64 = (1.0 + t2d);let t2f: f64 = (t25 * t2e);let t30: f64 = (1.0 + t2f);let t31: f64 = (1e-100 / t30);l.f6fc = t31;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) && (l.fed == 0.0)) {let t32: f64 = (2.0 * l.f6fc);let t33: f64 = (t32 - l.f6e);l.f74 = t33;l.f75 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.fe5 == 0.0)) {let t34: f64 = (1.772453850905516 * 0.5);let t35: f64 = (l.fe * l.f74);let t36: f64 = (t35 / l.f5a8);let t37: f64 = (t34 * t36);l.fd6 = t37;l.fd7 = 0.0;let t38: f64 = (l.f9 * l.fd6);let t39: f64 = (t38 * l.f7f5);let t3a: f64 = (l.f3f * t39);l.f599 = t3a;l.f59a = 0.0;}
        let t3b: f64 = if l.f24 == 0.0 { 1.0 } else { 0.0 };l.ff3 = t3b;l.ff4 = 0.0;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 != 0.0)) {l.f529 = 0.0;l.f52a = 0.0;}
        let t3c: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.ff5 = t3c;l.ff6 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff5 != 0.0)) {let t3d: f64 = (l.f771 - l.f750);let t3e: f64 = (t3d * l.f773);let t3f: f64 = (t3e).sqrt();l.f6fc = t3f;l.f6fd = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff5 == 0.0)) {let t40: f64 = (l.f771 - l.f750);let t41: f64 = (t40 * l.f773);let t42: f64 = (t41).powf(l.f623);l.f6fc = t42;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) {let t43: f64 = (l.f771 - l.f750);let t44: f64 = (t43 * l.f7da);let t45: f64 = (t44 / l.f6fc);let t46: f64 = (l.f611 * t45);l.fb6 = t46;l.fb7 = 0.0;}
        let t47: f64 = (-l.fa1);let t48: f64 = (t47 / l.fb6);let t49: f64 = (t48).abs();let t4a: f64 = if t49 < 230.25850929940458 { 1.0 } else { 0.0 };l.ff7 = t4a;l.ff8 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff7 != 0.0)) {let t4b: f64 = (-l.fa1);let t4c: f64 = (t4b / l.fb6);let t4d: f64 = (t4c).exp();l.f6fc = t4d;l.f6fd = 0.0;}
        let t4e: f64 = (-l.fa1);let t4f: f64 = (t4e / l.fb6);let t50: f64 = (-230.25850929940458);let t51: f64 = if t4f < t50 { 1.0 } else { 0.0 };l.ff9 = t51;l.ffa = 0.0;
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff7 == 0.0)) && (l.ff9 != 0.0)) {let t52: f64 = (-230.25850929940458);let t53: f64 = (-l.fa1);let t54: f64 = (t53 / l.fb6);let t55: f64 = (t52 - t54);let t56: f64 = (-230.25850929940458);let t57: f64 = (-l.fa1);let t58: f64 = (t57 / l.fb6);let t59: f64 = (t56 - t58);let t5a: f64 = (-230.25850929940458);let t5b: f64 = (-l.fa1);let t5c: f64 = (t5b / l.fb6);let t5d: f64 = (t5a - t5c);let t5e: f64 = (t5d * 0.3333333333333333);let t5f: f64 = (1.0 + t5e);let t60: f64 = (t59 * t5f);let t61: f64 = (0.5 * t60);let t62: f64 = (1.0 + t61);let t63: f64 = (t55 * t62);let t64: f64 = (1.0 + t63);let t65: f64 = (1e-100 / t64);l.f6fc = t65;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) && (l.ff7 == 0.0)) && (l.ff9 == 0.0)) {let t66: f64 = (-l.fa1);let t67: f64 = (t66 / l.fb6);let t68: f64 = (t67 - 230.25850929940458);let t69: f64 = (-l.fa1);let t6a: f64 = (t69 / l.fb6);let t6b: f64 = (t6a - 230.25850929940458);let t6c: f64 = (-l.fa1);let t6d: f64 = (t6c / l.fb6);let t6e: f64 = (t6d - 230.25850929940458);let t6f: f64 = (t6e * 0.3333333333333333);let t70: f64 = (1.0 + t6f);let t71: f64 = (t6b * t70);let t72: f64 = (0.5 * t71);let t73: f64 = (1.0 + t72);let t74: f64 = (t68 * t73);let t75: f64 = (1.0 + t74);let t76: f64 = (1e100 * t75);l.f6fc = t76;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ff3 == 0.0)) {let t77: f64 = (l.f737 * l.fb6);let t78: f64 = (t77 * l.fb6);let t79: f64 = (t78 * l.f6fc);let t7a: f64 = (l.f24 * t79);l.f529 = t7a;l.f52a = 0.0;}
        let t7b: f64 = if ((l.f783 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.ffb = t7b;l.ffc = 0.0;
        if (((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb != 0.0)) {l.fae = 1.0;l.faf = 0.0;}
        let t7c: f64 = (-l.f2);let t7d: f64 = (t7c * l.f783);let t7e: f64 = if l.f74a > t7d { 1.0 } else { 0.0 };l.ffd = t7e;l.ffe = 0.0;let t7f: f64 = if l.f625 == 4.0 { 1.0 } else { 0.0 };l.fff = t7f;l.f100 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb == 0.0)) && (l.ffd != 0.0)) && (l.fff != 0.0)) {let t80: f64 = (l.f74a * l.f787);let t81: f64 = (t80).abs();let t82: f64 = (l.f74a * l.f787);let t83: f64 = (t82).abs();let t84: f64 = (t81 * t83);let t85: f64 = (l.f74a * l.f787);let t86: f64 = (t85).abs();let t87: f64 = (t84 * t86);let t88: f64 = (l.f74a * l.f787);let t89: f64 = (t88).abs();let t8a: f64 = (t87 * t89);l.f6fc = t8a;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb == 0.0)) && (l.ffd != 0.0)) && (l.fff == 0.0)) {let t8b: f64 = (l.f74a * l.f787);let t8c: f64 = (t8b).abs();let t8d: f64 = (t8c).powf(l.f625);l.f6fc = t8d;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb == 0.0)) && (l.ffd != 0.0)) {let t8e: f64 = (1.0 - l.f6fc);let t8f: f64 = (1.0 / t8e);l.fae = t8f;l.faf = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4dd == 0.0)) && (l.ffb == 0.0)) && (l.ffd == 0.0)) {let t90: f64 = (l.f2 * l.f783);let t91: f64 = (l.f74a + t90);let t92: f64 = (t91 * l.f6ba);let t93: f64 = (l.fc3 + t92);l.fae = t93;l.faf = 0.0;}
        if ((l.f29a != 0.0) && (l.f4dd == 0.0)) {let t94: f64 = (l.f52f + l.f593);let t95: f64 = (t94 + l.f599);let t96: f64 = (t95 + l.f529);let t97: f64 = (t96 * l.fae);(l.f562, l.f563, l.f564, ) = (t97, (l.f530 * l.fae), (l.f531 * l.fae), );l.f565 = 0.0;let t98: f64 = (l.f593 + l.f599);let t99: f64 = (t98 + l.f529);let t9a: f64 = (t99 * l.fae);(l.f552, l.f553, l.f554, ) = (t9a, 0.0, 0.0, );l.f555 = 0.0;}
        let t9b: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f101 = t9b;l.f102 = 0.0;
        if ((l.f29a != 0.0) && (l.f101 != 0.0)) {(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );l.f579 = 0.0;(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );l.f55d = 0.0;(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );l.f697 = 0.0;}
        let t9c: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f103 = t9c;l.f104 = 0.0;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f103 != 0.0)) {let t9d: f64 = (l.f796 * l.f76d);let t9e: f64 = (1.0 - t9d);let t9f: f64 = (t9e).sqrt();l.f6fc = t9f;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f103 == 0.0)) {let ta0: f64 = (l.f796 * l.f76d);let ta1: f64 = (1.0 - ta0);let ta2: f64 = (ta1).powf(l.f60f);l.f6fc = ta2;l.f6fd = 0.0;}
        if ((l.f29a != 0.0) && (l.f101 == 0.0)) {let ta3: f64 = (1.0 - l.f6fc);let ta4: f64 = (l.f6a2 * ta3);let ta5: f64 = (l.f737 - l.f796);let ta6: f64 = (l.f69c * ta5);let ta7: f64 = (ta4 + ta6);(l.f694, l.f695, l.f696, ) = (ta7, 0.0, 0.0, );l.f697 = 0.0;let ta8: f64 = (l.f54c * l.f53e);(l.f52f, l.f530, l.f531, ) = (ta8, (l.f54c * l.f53f), (l.f54c * l.f540), );l.f532 = 0.0;}
        let ta9: f64 = if ((l.f3d == 0.0) && (l.f43 == 0.0)) { 1.0 } else { 0.0 };l.f105 = ta9;l.f106 = 0.0;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 != 0.0)) {l.f758 = 0.0;l.f759 = 0.0;l.f7e9 = 0.0;l.f7ea = 0.0;l.f7d1 = 0.0;l.f7d2 = 0.0;l.f9 = 0.0;l.fa = 0.0;l.f593 = 0.0;l.f594 = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) {let taa: f64 = (l.f77d - l.f7a2);l.f758 = taa;l.f759 = 0.0;let tab: f64 = (l.f714 / l.f758);let tac: f64 = (1.0 - tab);let tad: f64 = (tac).sqrt();let tae: f64 = (1.0 - tad);l.f7ef = tae;l.f7f0 = 0.0;}
        let taf: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f109 = taf;l.f10a = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) && (l.f109 != 0.0)) {l.f66 = 0.0;l.f67 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) && (l.f109 == 0.0)) {let tb0: f64 = (l.f7ef * l.f7ef);let tb1: f64 = (l.f7ef).ln();let tb2: f64 = (tb0 * tb1);let tb3: f64 = (1.0 - l.f7ef);let tb4: f64 = (tb2 / tb3);let tb5: f64 = (tb4 + l.f7ef);let tb6: f64 = (2.0 * l.f653);let tb7: f64 = (1.0 - tb6);let tb8: f64 = (tb5 * tb7);l.f66 = tb8;l.f67 = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) {let tb9: f64 = (l.f7ef + l.f66);l.f7e9 = tb9;l.f7ea = 0.0;}
        let tba: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f10b = tba;l.f10c = 0.0;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) && (l.f10b != 0.0)) {let tbb: f64 = (l.f758 * l.f77b);let tbc: f64 = (tbb).sqrt();l.f6fc = tbc;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) && (l.f10b == 0.0)) {let tbd: f64 = (l.f758 * l.f77b);let tbe: f64 = (tbd).powf(l.f653);l.f6fc = tbe;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f105 == 0.0)) {let tbf: f64 = (l.f7e0 * l.f6fc);l.f7d1 = tbf;l.f7d2 = 0.0;let tc0: f64 = (l.f825 - 1.0);let tc1: f64 = (tc0 * l.f7d1);let tc2: f64 = (l.fd1 * tc1);l.f9 = tc2;l.fa = 0.0;let tc3: f64 = (l.f9 * l.f7e9);let tc4: f64 = (l.f3d * tc3);l.f593 = tc4;l.f594 = 0.0;}
        let tc5: f64 = if l.f43 == 0.0 { 1.0 } else { 0.0 };l.f10d = tc5;l.f10e = 0.0;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d != 0.0)) {l.f599 = 0.0;l.f59a = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let tc6: f64 = (l.f7d1 * l.f60f);let tc7: f64 = (tc6 / l.f758);let tc8: f64 = (l.f22 * tc7);l.f19 = tc8;l.f1a = 0.0;let tc9: f64 = (0.666666666666667 * l.f12);let tca: f64 = (tc9 / l.f19);l.f71a = tca;l.f71b = 0.0;let tcb: f64 = (l.f71a * l.f71a);l.f72c = tcb;l.f72d = 0.0;let tcc: f64 = (l.f72c * l.f72c);let tcd: f64 = (l.f72c * l.f72c);let tce: f64 = (tcd + 1.0);let tcf: f64 = (tcc / tce);let td0: f64 = (tcf).sqrt();l.f726 = td0;l.f727 = 0.0;let td1: f64 = (l.f726).abs();let td2: f64 = (td1).sqrt();l.f6c1 = td2;l.f6c2 = 0.0;let td3: f64 = (l.f726 * l.f6c1);l.f732 = td3;l.f733 = 0.0;}
        let td4: f64 = (-l.f653);let td5: f64 = (td4 * l.f615);let td6: f64 = (-1.0);let td7: f64 = if td5 == td6 { 1.0 } else { 0.0 };l.f10f = td7;l.f110 = 0.0;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f10f != 0.0)) {let td8: f64 = (l.f19 * l.f732);let td9: f64 = (1.0 + td8);let tda: f64 = (1.0 / td9);l.f7e3 = tda;l.f7e4 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f10f == 0.0)) {let tdb: f64 = (l.f19 * l.f732);let tdc: f64 = (1.0 + tdb);let tdd: f64 = (-l.f653);let tde: f64 = (tdd * l.f615);let tdf: f64 = (tdc).powf(tde);l.f7e3 = tdf;l.f7e4 = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let te0: f64 = (l.f7e9 * l.f7e3);let te1: f64 = (l.f7e9 + l.f7e3);let te2: f64 = (te0 / te1);l.f7f5 = te2;l.f7f6 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let te3: f64 = (l.f19 / l.f6c1);let te4: f64 = (0.375 * te3);let te5: f64 = (te4).sqrt();l.f5a8 = te5;l.f5a9 = 0.0;let te6: f64 = (l.f71a * l.f6c1);let te7: f64 = (2.0 * te6);let te8: f64 = (te7 - l.f726);l.f5b4 = te8;l.f5b5 = 0.0;let te9: f64 = (l.f12 * l.f71a);let tea: f64 = (te9 * l.f6c1);let teb: f64 = (l.f12 * l.f726);let tec: f64 = (tea - teb);let ted: f64 = (l.f19 * l.f732);let tee: f64 = (0.5 * ted);let tef: f64 = (tec + tee);l.f5d4 = tef;l.f5d5 = 0.0;let tf0: f64 = (l.f5b4 - 1.0);let tf1: f64 = (tf0 * l.f5a8);l.f7fb = tf1;l.f7fc = 0.0;let tf2: f64 = (l.f7fb * l.f7fb);l.f811 = tf2;l.f812 = 0.0;}
        let tf3: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f111 = tf3;l.f112 = 0.0;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f111 != 0.0)) {let tf4: f64 = (l.f62b * l.f7fb);let tf5: f64 = (1.0 + tf4);let tf6: f64 = (1.0 / tf5);l.f6e2 = tf6;l.f6e3 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f111 == 0.0)) {let tf7: f64 = (l.f62b * l.f7fb);let tf8: f64 = (1.0 - tf7);let tf9: f64 = (1.0 / tf8);l.f6e2 = tf9;l.f6e3 = 0.0;}
        let tfa: f64 = (-l.f811);let tfb: f64 = (tfa + l.f5d4);let tfc: f64 = (-230.25850929940458);let tfd: f64 = if tfb > tfc { 1.0 } else { 0.0 };l.f113 = tfd;l.f114 = 0.0;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f113 != 0.0)) {let tfe: f64 = (-l.f811);let tff: f64 = (tfe + l.f5d4);let t100: f64 = (tff).exp();l.f6fc = t100;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f113 == 0.0)) {let t101: f64 = (-230.25850929940458);let t102: f64 = (-l.f811);let t103: f64 = (t102 + l.f5d4);let t104: f64 = (t101 - t103);let t105: f64 = (-230.25850929940458);let t106: f64 = (-l.f811);let t107: f64 = (t106 + l.f5d4);let t108: f64 = (t105 - t107);let t109: f64 = (-230.25850929940458);let t10a: f64 = (-l.f811);let t10b: f64 = (t10a + l.f5d4);let t10c: f64 = (t109 - t10b);let t10d: f64 = (t10c * 0.3333333333333333);let t10e: f64 = (1.0 + t10d);let t10f: f64 = (t108 * t10e);let t110: f64 = (0.5 * t10f);let t111: f64 = (1.0 + t110);let t112: f64 = (t104 * t111);let t113: f64 = (1.0 + t112);let t114: f64 = (1e-100 / t113);l.f6fc = t114;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let t115: f64 = (0.29214664 * l.f6e2);let t116: f64 = (l.f6e2 * l.f6e2);let t117: f64 = (l.f16 * t116);let t118: f64 = (t115 + t117);let t119: f64 = (l.f6e2 * l.f6e2);let t11a: f64 = (t119 * l.f6e2);let t11b: f64 = (l.f2a * t11a);let t11c: f64 = (t118 + t11b);let t11d: f64 = (t11c * l.f6fc);l.f6e = t11d;l.f6f = 0.0;}
        let t11e: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f115 = t11e;l.f116 = 0.0;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f115 != 0.0)) {l.f74 = l.f6e;l.f75 = 0.0;}
        let t11f: f64 = (-230.25850929940458);let t120: f64 = if l.f5d4 > t11f { 1.0 } else { 0.0 };l.f117 = t120;l.f118 = 0.0;
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f115 == 0.0)) && (l.f117 != 0.0)) {let t121: f64 = (l.f5d4).exp();l.f6fc = t121;l.f6fd = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f115 == 0.0)) && (l.f117 == 0.0)) {let t122: f64 = (-230.25850929940458);let t123: f64 = (t122 - l.f5d4);let t124: f64 = (-230.25850929940458);let t125: f64 = (t124 - l.f5d4);let t126: f64 = (-230.25850929940458);let t127: f64 = (t126 - l.f5d4);let t128: f64 = (t127 * 0.3333333333333333);let t129: f64 = (1.0 + t128);let t12a: f64 = (t125 * t129);let t12b: f64 = (0.5 * t12a);let t12c: f64 = (1.0 + t12b);let t12d: f64 = (t123 * t12c);let t12e: f64 = (1.0 + t12d);let t12f: f64 = (1e-100 / t12e);l.f6fc = t12f;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) && (l.f115 == 0.0)) {let t130: f64 = (2.0 * l.f6fc);let t131: f64 = (t130 - l.f6e);l.f74 = t131;l.f75 = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f10d == 0.0)) {let t132: f64 = (1.772453850905516 * 0.5);let t133: f64 = (l.f12 * l.f74);let t134: f64 = (t133 / l.f5a8);let t135: f64 = (t132 * t134);l.fd6 = t135;l.fd7 = 0.0;let t136: f64 = (l.f9 * l.fd6);let t137: f64 = (t136 * l.f7f5);let t138: f64 = (l.f43 * t137);l.f599 = t138;l.f59a = 0.0;}
        let t139: f64 = if l.f28 == 0.0 { 1.0 } else { 0.0 };l.f119 = t139;l.f11a = 0.0;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 != 0.0)) {l.f529 = 0.0;l.f52a = 0.0;}
        let t13a: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f11b = t13a;l.f11c = 0.0;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11b != 0.0)) {let t13b: f64 = (l.f779 - l.f750);let t13c: f64 = (t13b * l.f77b);let t13d: f64 = (t13c).sqrt();l.f6fc = t13d;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11b == 0.0)) {let t13e: f64 = (l.f779 - l.f750);let t13f: f64 = (t13e * l.f77b);let t140: f64 = (t13f).powf(l.f653);l.f6fc = t140;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) {let t141: f64 = (l.f779 - l.f750);let t142: f64 = (t141 * l.f7de);let t143: f64 = (t142 / l.f6fc);let t144: f64 = (l.f615 * t143);l.fb6 = t144;l.fb7 = 0.0;}
        let t145: f64 = (-l.fab);let t146: f64 = (t145 / l.fb6);let t147: f64 = (t146).abs();let t148: f64 = if t147 < 230.25850929940458 { 1.0 } else { 0.0 };l.f11f = t148;l.f120 = 0.0;
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11f != 0.0)) {let t149: f64 = (-l.fab);let t14a: f64 = (t149 / l.fb6);let t14b: f64 = (t14a).exp();l.f6fc = t14b;l.f6fd = 0.0;}
        let t14c: f64 = (-l.fab);let t14d: f64 = (t14c / l.fb6);let t14e: f64 = (-230.25850929940458);let t14f: f64 = if t14d < t14e { 1.0 } else { 0.0 };l.f121 = t14f;l.f122 = 0.0;
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11f == 0.0)) && (l.f121 != 0.0)) {let t150: f64 = (-230.25850929940458);let t151: f64 = (-l.fab);let t152: f64 = (t151 / l.fb6);let t153: f64 = (t150 - t152);let t154: f64 = (-230.25850929940458);let t155: f64 = (-l.fab);let t156: f64 = (t155 / l.fb6);let t157: f64 = (t154 - t156);let t158: f64 = (-230.25850929940458);let t159: f64 = (-l.fab);let t15a: f64 = (t159 / l.fb6);let t15b: f64 = (t158 - t15a);let t15c: f64 = (t15b * 0.3333333333333333);let t15d: f64 = (1.0 + t15c);let t15e: f64 = (t157 * t15d);let t15f: f64 = (0.5 * t15e);let t160: f64 = (1.0 + t15f);let t161: f64 = (t153 * t160);let t162: f64 = (1.0 + t161);let t163: f64 = (1e-100 / t162);l.f6fc = t163;l.f6fd = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) && (l.f11f == 0.0)) && (l.f121 == 0.0)) {let t164: f64 = (-l.fab);let t165: f64 = (t164 / l.fb6);let t166: f64 = (t165 - 230.25850929940458);let t167: f64 = (-l.fab);let t168: f64 = (t167 / l.fb6);let t169: f64 = (t168 - 230.25850929940458);let t16a: f64 = (-l.fab);let t16b: f64 = (t16a / l.fb6);let t16c: f64 = (t16b - 230.25850929940458);let t16d: f64 = (t16c * 0.3333333333333333);let t16e: f64 = (1.0 + t16d);let t16f: f64 = (t169 * t16e);let t170: f64 = (0.5 * t16f);let t171: f64 = (1.0 + t170);let t172: f64 = (t166 * t171);let t173: f64 = (1.0 + t172);let t174: f64 = (1e100 * t173);l.f6fc = t174;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f119 == 0.0)) {let t175: f64 = (l.f737 * l.fb6);let t176: f64 = (t175 * l.fb6);let t177: f64 = (t176 * l.f6fc);let t178: f64 = (l.f28 * t177);l.f529 = t178;l.f52a = 0.0;}
        let t179: f64 = if ((l.f78d > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f123 = t179;l.f124 = 0.0;
        if (((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 != 0.0)) {l.fae = 1.0;l.faf = 0.0;}
        let t17a: f64 = (-l.f2);let t17b: f64 = (t17a * l.f78d);let t17c: f64 = if l.f74a > t17b { 1.0 } else { 0.0 };l.f125 = t17c;l.f126 = 0.0;let t17d: f64 = if l.f629 == 4.0 { 1.0 } else { 0.0 };l.f127 = t17d;l.f128 = 0.0;
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 == 0.0)) && (l.f125 != 0.0)) && (l.f127 != 0.0)) {let t17e: f64 = (l.f74a * l.f78b);let t17f: f64 = (t17e).abs();let t180: f64 = (l.f74a * l.f78b);let t181: f64 = (t180).abs();let t182: f64 = (t17f * t181);let t183: f64 = (l.f74a * l.f78b);let t184: f64 = (t183).abs();let t185: f64 = (t182 * t184);let t186: f64 = (l.f74a * l.f78b);let t187: f64 = (t186).abs();let t188: f64 = (t185 * t187);l.f6fc = t188;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 == 0.0)) && (l.f125 != 0.0)) && (l.f127 == 0.0)) {let t189: f64 = (l.f74a * l.f78b);let t18a: f64 = (t189).abs();let t18b: f64 = (t18a).powf(l.f629);l.f6fc = t18b;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 == 0.0)) && (l.f125 != 0.0)) {let t18c: f64 = (1.0 - l.f6fc);let t18d: f64 = (1.0 / t18c);l.fae = t18d;l.faf = 0.0;}
        if ((((l.f29a != 0.0) && (l.f101 == 0.0)) && (l.f123 == 0.0)) && (l.f125 == 0.0)) {let t18e: f64 = (l.f2 * l.f78d);let t18f: f64 = (l.f74a + t18e);let t190: f64 = (t18f * l.f6be);let t191: f64 = (l.fc7 + t190);l.fae = t191;l.faf = 0.0;}
        if ((l.f29a != 0.0) && (l.f101 == 0.0)) {let t192: f64 = (l.f52f + l.f593);let t193: f64 = (t192 + l.f599);let t194: f64 = (t193 + l.f529);let t195: f64 = (t194 * l.fae);(l.f576, l.f577, l.f578, ) = (t195, (l.f530 * l.fae), (l.f531 * l.fae), );l.f579 = 0.0;let t196: f64 = (l.f593 + l.f599);let t197: f64 = (t196 + l.f529);let t198: f64 = (t197 * l.fae);(l.f55a, l.f55b, l.f55c, ) = (t198, 0.0, 0.0, );l.f55d = 0.0;}
        let t199: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f129 = t199;l.f12a = 0.0;
        if ((l.f29a != 0.0) && (l.f129 != 0.0)) {(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );l.f571 = 0.0;(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );l.f559 = 0.0;(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );l.f693 = 0.0;}
        let t19a: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f12b = t19a;l.f12c = 0.0;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12b != 0.0)) {let t19b: f64 = (l.f796 * l.f76b);let t19c: f64 = (1.0 - t19b);let t19d: f64 = (t19c).sqrt();l.f6fc = t19d;l.f6fd = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12b == 0.0)) {let t19e: f64 = (l.f796 * l.f76b);let t19f: f64 = (1.0 - t19e);let t1a0: f64 = (t19f).powf(l.f60d);l.f6fc = t1a0;l.f6fd = 0.0;}
        if ((l.f29a != 0.0) && (l.f129 == 0.0)) {let t1a1: f64 = (1.0 - l.f6fc);let t1a2: f64 = (l.f6a0 * t1a1);let t1a3: f64 = (l.f737 - l.f796);let t1a4: f64 = (l.f69a * t1a3);let t1a5: f64 = (t1a2 + t1a4);(l.f690, l.f691, l.f692, ) = (t1a5, 0.0, 0.0, );l.f693 = 0.0;let t1a6: f64 = (l.f544 * l.f53a);(l.f52f, l.f530, l.f531, ) = (t1a6, (l.f544 * l.f53b), (l.f544 * l.f53c), );l.f532 = 0.0;}
        let t1a7: f64 = if ((l.f3b == 0.0) && (l.f41 == 0.0)) { 1.0 } else { 0.0 };l.f12d = t1a7;l.f12e = 0.0;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d != 0.0)) {l.f758 = 0.0;l.f759 = 0.0;l.f7e9 = 0.0;l.f7ea = 0.0;l.f7d1 = 0.0;l.f7d2 = 0.0;l.f9 = 0.0;l.fa = 0.0;l.f593 = 0.0;l.f594 = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) {let t1a8: f64 = (l.f763 - l.f7a2);l.f758 = t1a8;l.f759 = 0.0;let t1a9: f64 = (l.f714 / l.f758);let t1aa: f64 = (1.0 - t1a9);let t1ab: f64 = (t1aa).sqrt();let t1ac: f64 = (1.0 - t1ab);l.f7ef = t1ac;l.f7f0 = 0.0;}
        let t1ad: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f12f = t1ad;l.f130 = 0.0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) && (l.f12f != 0.0)) {l.f66 = 0.0;l.f67 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) && (l.f12f == 0.0)) {let t1ae: f64 = (l.f7ef * l.f7ef);let t1af: f64 = (l.f7ef).ln();let t1b0: f64 = (t1ae * t1af);let t1b1: f64 = (1.0 - l.f7ef);let t1b2: f64 = (t1b0 / t1b1);let t1b3: f64 = (t1b2 + l.f7ef);let t1b4: f64 = (2.0 * l.f62f);let t1b5: f64 = (1.0 - t1b4);let t1b6: f64 = (t1b3 * t1b5);l.f66 = t1b6;l.f67 = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) {let t1b7: f64 = (l.f7ef + l.f66);l.f7e9 = t1b7;l.f7ea = 0.0;}
        let t1b8: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f131 = t1b8;l.f132 = 0.0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) && (l.f131 != 0.0)) {let t1b9: f64 = (l.f758 * l.f777);let t1ba: f64 = (t1b9).sqrt();l.f6fc = t1ba;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) && (l.f131 == 0.0)) {let t1bb: f64 = (l.f758 * l.f777);let t1bc: f64 = (t1bb).powf(l.f62f);l.f6fc = t1bc;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) {let t1bd: f64 = (l.f7d8 * l.f6fc);l.f7d1 = t1bd;l.f7d2 = 0.0;let t1be: f64 = (l.f825 - 1.0);let t1bf: f64 = (t1be * l.f7d1);let t1c0: f64 = (l.fcd * t1bf);l.f9 = t1c0;l.fa = 0.0;let t1c1: f64 = (l.f9 * l.f7e9);let t1c2: f64 = (l.f3b * t1c1);l.f593 = t1c2;l.f594 = 0.0;}
        let t1c3: f64 = if l.f41 == 0.0 { 1.0 } else { 0.0 };l.f134 = t1c3;l.f135 = 0.0;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 != 0.0)) {l.f599 = 0.0;l.f59a = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t1c4: f64 = (l.f7d1 * l.f60d);let t1c5: f64 = (t1c4 / l.f758);let t1c6: f64 = (l.f20 * t1c5);l.f19 = t1c6;l.f1a = 0.0;let t1c7: f64 = (0.666666666666667 * l.f10);let t1c8: f64 = (t1c7 / l.f19);l.f71a = t1c8;l.f71b = 0.0;let t1c9: f64 = (l.f71a * l.f71a);l.f72c = t1c9;l.f72d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t1ca: f64 = (l.f72c * l.f72c);let t1cb: f64 = (l.f72c * l.f72c);let t1cc: f64 = (t1cb + 1.0);let t1cd: f64 = (t1ca / t1cc);let t1ce: f64 = (t1cd).sqrt();l.f726 = t1ce;l.f727 = 0.0;let t1cf: f64 = (l.f726).abs();let t1d0: f64 = (t1cf).sqrt();l.f6c1 = t1d0;l.f6c2 = 0.0;let t1d1: f64 = (l.f726 * l.f6c1);l.f732 = t1d1;l.f733 = 0.0;}
        let t1d2: f64 = (-l.f62f);let t1d3: f64 = (t1d2 * l.f613);let t1d4: f64 = (-1.0);let t1d5: f64 = if t1d3 == t1d4 { 1.0 } else { 0.0 };l.f136 = t1d5;l.f137 = 0.0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f136 != 0.0)) {let t1d6: f64 = (l.f19 * l.f732);let t1d7: f64 = (1.0 + t1d6);let t1d8: f64 = (1.0 / t1d7);l.f7e3 = t1d8;l.f7e4 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f136 == 0.0)) {let t1d9: f64 = (l.f19 * l.f732);let t1da: f64 = (1.0 + t1d9);let t1db: f64 = (-l.f62f);let t1dc: f64 = (t1db * l.f613);let t1dd: f64 = (t1da).powf(t1dc);l.f7e3 = t1dd;l.f7e4 = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t1de: f64 = (l.f7e9 * l.f7e3);let t1df: f64 = (l.f7e9 + l.f7e3);let t1e0: f64 = (t1de / t1df);l.f7f5 = t1e0;l.f7f6 = 0.0;let t1e1: f64 = (l.f19 / l.f6c1);let t1e2: f64 = (0.375 * t1e1);let t1e3: f64 = (t1e2).sqrt();l.f5a8 = t1e3;l.f5a9 = 0.0;let t1e4: f64 = (l.f71a * l.f6c1);let t1e5: f64 = (2.0 * t1e4);let t1e6: f64 = (t1e5 - l.f726);l.f5b4 = t1e6;l.f5b5 = 0.0;let t1e7: f64 = (l.f10 * l.f71a);let t1e8: f64 = (t1e7 * l.f6c1);let t1e9: f64 = (l.f10 * l.f726);let t1ea: f64 = (t1e8 - t1e9);let t1eb: f64 = (l.f19 * l.f732);let t1ec: f64 = (0.5 * t1eb);let t1ed: f64 = (t1ea + t1ec);l.f5d4 = t1ed;l.f5d5 = 0.0;let t1ee: f64 = (l.f5b4 - 1.0);let t1ef: f64 = (t1ee * l.f5a8);l.f7fb = t1ef;l.f7fc = 0.0;let t1f0: f64 = (l.f7fb * l.f7fb);l.f811 = t1f0;l.f812 = 0.0;}
        let t1f1: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f138 = t1f1;l.f139 = 0.0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f138 != 0.0)) {let t1f2: f64 = (l.f62b * l.f7fb);let t1f3: f64 = (1.0 + t1f2);let t1f4: f64 = (1.0 / t1f3);l.f6e2 = t1f4;l.f6e3 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f138 == 0.0)) {let t1f5: f64 = (l.f62b * l.f7fb);let t1f6: f64 = (1.0 - t1f5);let t1f7: f64 = (1.0 / t1f6);l.f6e2 = t1f7;l.f6e3 = 0.0;}
        let t1f8: f64 = (-l.f811);let t1f9: f64 = (t1f8 + l.f5d4);let t1fa: f64 = (-230.25850929940458);let t1fb: f64 = if t1f9 > t1fa { 1.0 } else { 0.0 };l.f13a = t1fb;l.f13b = 0.0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13a != 0.0)) {let t1fc: f64 = (-l.f811);let t1fd: f64 = (t1fc + l.f5d4);let t1fe: f64 = (t1fd).exp();l.f6fc = t1fe;l.f6fd = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13a == 0.0)) {let t1ff: f64 = (-230.25850929940458);let t200: f64 = (-l.f811);let t201: f64 = (t200 + l.f5d4);let t202: f64 = (t1ff - t201);let t203: f64 = (-230.25850929940458);let t204: f64 = (-l.f811);let t205: f64 = (t204 + l.f5d4);let t206: f64 = (t203 - t205);let t207: f64 = (-230.25850929940458);let t208: f64 = (-l.f811);let t209: f64 = (t208 + l.f5d4);let t20a: f64 = (t207 - t209);let t20b: f64 = (t20a * 0.3333333333333333);let t20c: f64 = (1.0 + t20b);let t20d: f64 = (t206 * t20c);let t20e: f64 = (0.5 * t20d);let t20f: f64 = (1.0 + t20e);let t210: f64 = (t202 * t20f);let t211: f64 = (1.0 + t210);let t212: f64 = (1e-100 / t211);l.f6fc = t212;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t213: f64 = (0.29214664 * l.f6e2);let t214: f64 = (l.f6e2 * l.f6e2);let t215: f64 = (l.f16 * t214);let t216: f64 = (t213 + t215);let t217: f64 = (l.f6e2 * l.f6e2);let t218: f64 = (t217 * l.f6e2);let t219: f64 = (l.f2a * t218);let t21a: f64 = (t216 + t219);let t21b: f64 = (t21a * l.f6fc);l.f6e = t21b;l.f6f = 0.0;}
        let t21c: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f13c = t21c;l.f13d = 0.0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13c != 0.0)) {l.f74 = l.f6e;l.f75 = 0.0;}
        let t21d: f64 = (-230.25850929940458);let t21e: f64 = if l.f5d4 > t21d { 1.0 } else { 0.0 };l.f13e = t21e;l.f13f = 0.0;
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13c == 0.0)) && (l.f13e != 0.0)) {let t21f: f64 = (l.f5d4).exp();l.f6fc = t21f;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13c == 0.0)) && (l.f13e == 0.0)) {let t220: f64 = (-230.25850929940458);let t221: f64 = (t220 - l.f5d4);let t222: f64 = (-230.25850929940458);let t223: f64 = (t222 - l.f5d4);let t224: f64 = (-230.25850929940458);let t225: f64 = (t224 - l.f5d4);let t226: f64 = (t225 * 0.3333333333333333);let t227: f64 = (1.0 + t226);let t228: f64 = (t223 * t227);let t229: f64 = (0.5 * t228);let t22a: f64 = (1.0 + t229);let t22b: f64 = (t221 * t22a);let t22c: f64 = (1.0 + t22b);let t22d: f64 = (1e-100 / t22c);l.f6fc = t22d;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13c == 0.0)) {let t22e: f64 = (2.0 * l.f6fc);let t22f: f64 = (t22e - l.f6e);l.f74 = t22f;l.f75 = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t230: f64 = (1.772453850905516 * 0.5);let t231: f64 = (l.f10 * l.f74);let t232: f64 = (t231 / l.f5a8);let t233: f64 = (t230 * t232);l.fd6 = t233;l.fd7 = 0.0;let t234: f64 = (l.f9 * l.fd6);let t235: f64 = (t234 * l.f7f5);let t236: f64 = (l.f41 * t235);l.f599 = t236;l.f59a = 0.0;}
        let t237: f64 = if l.f26 == 0.0 { 1.0 } else { 0.0 };l.f140 = t237;l.f141 = 0.0;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 != 0.0)) {l.f529 = 0.0;l.f52a = 0.0;}
        let t238: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f142 = t238;l.f143 = 0.0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f142 != 0.0)) {let t239: f64 = (l.f775 - l.f750);let t23a: f64 = (t239 * l.f777);let t23b: f64 = (t23a).sqrt();l.f6fc = t23b;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f142 == 0.0)) {let t23c: f64 = (l.f775 - l.f750);let t23d: f64 = (t23c * l.f777);let t23e: f64 = (t23d).powf(l.f62f);l.f6fc = t23e;l.f6fd = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) {let t23f: f64 = (l.f775 - l.f750);let t240: f64 = (t23f * l.f7dc);let t241: f64 = (t240 / l.f6fc);let t242: f64 = (l.f613 * t241);l.fb6 = t242;l.fb7 = 0.0;}
        let t243: f64 = (-l.fa3);let t244: f64 = (t243 / l.fb6);let t245: f64 = (t244).abs();let t246: f64 = if t245 < 230.25850929940458 { 1.0 } else { 0.0 };l.f144 = t246;l.f145 = 0.0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f144 != 0.0)) {let t247: f64 = (-l.fa3);let t248: f64 = (t247 / l.fb6);let t249: f64 = (t248).exp();l.f6fc = t249;l.f6fd = 0.0;}
        let t24a: f64 = (-l.fa3);let t24b: f64 = (t24a / l.fb6);let t24c: f64 = (-230.25850929940458);let t24d: f64 = if t24b < t24c { 1.0 } else { 0.0 };l.f146 = t24d;l.f147 = 0.0;
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f144 == 0.0)) && (l.f146 != 0.0)) {let t24e: f64 = (-230.25850929940458);let t24f: f64 = (-l.fa3);let t250: f64 = (t24f / l.fb6);let t251: f64 = (t24e - t250);let t252: f64 = (-230.25850929940458);let t253: f64 = (-l.fa3);let t254: f64 = (t253 / l.fb6);let t255: f64 = (t252 - t254);let t256: f64 = (-230.25850929940458);let t257: f64 = (-l.fa3);let t258: f64 = (t257 / l.fb6);let t259: f64 = (t256 - t258);let t25a: f64 = (t259 * 0.3333333333333333);let t25b: f64 = (1.0 + t25a);let t25c: f64 = (t255 * t25b);let t25d: f64 = (0.5 * t25c);let t25e: f64 = (1.0 + t25d);let t25f: f64 = (t251 * t25e);let t260: f64 = (1.0 + t25f);let t261: f64 = (1e-100 / t260);l.f6fc = t261;l.f6fd = 0.0;}
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f144 == 0.0)) && (l.f146 == 0.0)) {let t267: f64 = (-l.fa3);let t268: f64 = (t267 / l.fb6);let t269: f64 = (t268 - 230.25850929940458);let t26a: f64 = (-l.fa3);let t26b: f64 = (t26a / l.fb6);let t26c: f64 = (t26b - 230.25850929940458);let t26d: f64 = (-l.fa3);let t26e: f64 = (t26d / l.fb6);let t26f: f64 = (t26e - 230.25850929940458);let t270: f64 = (t26f * 0.3333333333333333);let t271: f64 = (1.0 + t270);let t272: f64 = (t26c * t271);let t262: f64 = (0.5 * t272);let t263: f64 = (1.0 + t262);let t264: f64 = (t269 * t263);let t265: f64 = (1.0 + t264);let t266: f64 = (1e100 * t265);l.f6fc = t266;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) {let t273: f64 = (l.f737 * l.fb6);let t274: f64 = (t273 * l.fb6);let t275: f64 = (t274 * l.f6fc);let t276: f64 = (l.f26 * t275);l.f529 = t276;l.f52a = 0.0;}
        let t277: f64 = if ((l.f785 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f148 = t277;l.f149 = 0.0;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 != 0.0)) {l.fae = 1.0;l.faf = 0.0;}
        let t278: f64 = (-l.f2);let t279: f64 = (t278 * l.f785);let t27a: f64 = if l.f74a > t279 { 1.0 } else { 0.0 };l.f14a = t27a;l.f14b = 0.0;let t27b: f64 = if l.f627 == 4.0 { 1.0 } else { 0.0 };l.f14c = t27b;l.f14d = 0.0;
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 == 0.0)) && (l.f14a != 0.0)) && (l.f14c != 0.0)) {let t27c: f64 = (l.f74a * l.f789);let t27d: f64 = (t27c).abs();let t27e: f64 = (l.f74a * l.f789);let t27f: f64 = (t27e).abs();let t280: f64 = (t27d * t27f);let t281: f64 = (l.f74a * l.f789);let t282: f64 = (t281).abs();let t283: f64 = (t280 * t282);let t284: f64 = (l.f74a * l.f789);let t285: f64 = (t284).abs();let t286: f64 = (t283 * t285);l.f6fc = t286;l.f6fd = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 == 0.0)) && (l.f14a != 0.0)) && (l.f14c == 0.0)) {let t287: f64 = (l.f74a * l.f789);let t288: f64 = (t287).abs();let t289: f64 = (t288).powf(l.f627);l.f6fc = t289;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 == 0.0)) && (l.f14a != 0.0)) {let t28a: f64 = (1.0 - l.f6fc);let t28b: f64 = (1.0 / t28a);l.fae = t28b;l.faf = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 == 0.0)) && (l.f14a == 0.0)) {let t28c: f64 = (l.f2 * l.f785);let t28d: f64 = (l.f74a + t28c);let t28e: f64 = (t28d * l.f6bc);let t28f: f64 = (l.fc5 + t28e);l.fae = t28f;l.faf = 0.0;}
        if ((l.f29a != 0.0) && (l.f129 == 0.0)) {let t290: f64 = (l.f52f + l.f593);let t291: f64 = (t290 + l.f599);let t292: f64 = (t291 + l.f529);let t293: f64 = (t292 * l.fae);(l.f56e, l.f56f, l.f570, ) = (t293, (l.f530 * l.fae), (l.f531 * l.fae), );l.f571 = 0.0;let t294: f64 = (l.f593 + l.f599);let t295: f64 = (t294 + l.f529);let t296: f64 = (t295 * l.fae);(l.f556, l.f557, l.f558, ) = (t296, 0.0, 0.0, );l.f559 = 0.0;}
        if (l.f29a != 0.0) {let t297: f64 = (l.f0 * l.f562);let t298: f64 = (l.f5b1 * l.f576);let t299: f64 = (t297 + t298);let t29a: f64 = (l.f5af * l.f56e);let t29b: f64 = (t299 + t29a);(l.f500, l.f505, l.f506, ) = (t29b, (((l.f0 * l.f563) + (l.f5b1 * l.f577)) + (l.f5af * l.f56f)), (((l.f0 * l.f564) + (l.f5b1 * l.f578)) + (l.f5af * l.f570)), );l.f507 = 0.0;}
        let t29c: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f14e = t29c;l.f14f = 0.0;
        if ((l.f29a != 0.0) && (l.f14e != 0.0)) {let t29d: f64 = (4.0 * l.f78f);let t29e: f64 = (t29d * l.f78f);l.f4e1 = t29e;l.f4e2 = 0.0;let t29f: f64 = (l.f78f / l.f791);l.f4e5 = t29f;l.f4e6 = 0.0;let t2a0: f64 = (l.f78f * l.f4e5);let t2a1: f64 = (l.f739 + t2a0);l.f4e9 = t2a1;l.f4ea = 0.0;let t2a2: f64 = (l.f791 + l.f4e9);l.f4ef = t2a2;l.f4f0 = 0.0;let t2a3: f64 = (l.f791 - l.f4e9);l.f4f5 = t2a3;l.f4f6 = 0.0;let t2a4: f64 = (l.f4f5 * l.f4f5);let t2a5: f64 = (t2a4 + l.f4e1);let t2a6: f64 = (t2a5).sqrt();l.f4fb = t2a6;l.f4fc = 0.0;let t2a7: f64 = (l.f739 * l.f791);let t2a8: f64 = (l.f4ef + l.f4fb);let t2a9: f64 = (t2a7 / t2a8);let t2aa: f64 = (2.0 * t2a9);l.f796 = t2aa;l.f797 = 0.0;}
        let t2ab: f64 = if l.f739 < l.f7b1 { 1.0 } else { 0.0 };l.f150 = t2ab;l.f151 = 0.0;let t2ac: f64 = (l.f739 * l.f645);let t2ad: f64 = (0.5 * t2ac);let t2ae: f64 = (t2ad).abs();let t2af: f64 = if t2ae < 230.25850929940458 { 1.0 } else { 0.0 };l.f152 = t2af;l.f153 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f152 != 0.0)) {let t2b0: f64 = (l.f739 * l.f645);let t2b1: f64 = (0.5 * t2b0);let t2b2: f64 = (t2b1).exp();l.f825 = t2b2;l.f826 = 0.0;}
        let t2b3: f64 = (l.f739 * l.f645);let t2b4: f64 = (0.5 * t2b3);let t2b5: f64 = (-230.25850929940458);let t2b6: f64 = if t2b4 < t2b5 { 1.0 } else { 0.0 };l.f154 = t2b6;l.f155 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f152 == 0.0)) && (l.f154 != 0.0)) {let t2b7: f64 = (-230.25850929940458);let t2b8: f64 = (l.f739 * l.f645);let t2b9: f64 = (0.5 * t2b8);let t2ba: f64 = (t2b7 - t2b9);let t2bb: f64 = (-230.25850929940458);let t2bc: f64 = (l.f739 * l.f645);let t2bd: f64 = (0.5 * t2bc);let t2be: f64 = (t2bb - t2bd);let t2bf: f64 = (-230.25850929940458);let t2c0: f64 = (l.f739 * l.f645);let t2c1: f64 = (0.5 * t2c0);let t2c2: f64 = (t2bf - t2c1);let t2c3: f64 = (t2c2 * 0.3333333333333333);let t2c4: f64 = (1.0 + t2c3);let t2c5: f64 = (t2be * t2c4);let t2c6: f64 = (0.5 * t2c5);let t2c7: f64 = (1.0 + t2c6);let t2c8: f64 = (t2ba * t2c7);let t2c9: f64 = (1.0 + t2c8);let t2ca: f64 = (1e-100 / t2c9);l.f825 = t2ca;l.f826 = 0.0;}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f152 == 0.0)) && (l.f154 == 0.0)) {let t2cb: f64 = (l.f739 * l.f645);let t2cc: f64 = (0.5 * t2cb);let t2cd: f64 = (t2cc - 230.25850929940458);let t2ce: f64 = (l.f739 * l.f645);let t2cf: f64 = (0.5 * t2ce);let t2d0: f64 = (t2cf - 230.25850929940458);let t2d1: f64 = (l.f739 * l.f645);let t2d2: f64 = (0.5 * t2d1);let t2d3: f64 = (t2d2 - 230.25850929940458);let t2d4: f64 = (t2d3 * 0.3333333333333333);let t2d5: f64 = (1.0 + t2d4);let t2d6: f64 = (t2d0 * t2d5);let t2d7: f64 = (0.5 * t2d6);let t2d8: f64 = (1.0 + t2d7);let t2d9: f64 = (t2cd * t2d8);let t2da: f64 = (1.0 + t2d9);let t2db: f64 = (1e100 * t2da);l.f825 = t2db;l.f826 = 0.0;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) {let t2dc: f64 = (l.f5eb * l.f5eb);let t2dd: f64 = (t2dc / l.f5df);l.f64f = t2dd;l.f650 = 0.0;let t2de: f64 = (l.f5e5 / l.f645);let t2df: f64 = (l.f5df / l.f64f);let t2e0: f64 = (t2df).ln();let t2e1: f64 = (t2de * t2e0);l.f793 = t2e1;l.f794 = 0.0;}
        let t2e2: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f156 = t2e2;l.f157 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t2e3: f64 = (l.f739 - l.f793);let t2e4: f64 = (p.p86 * t2e3);let t2e5: f64 = (t2e4 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t2e5, 0.0, 0.0, );l.f604 = 0.0;let t2e6: f64 = (p.p86 * l.f793);let t2e7: f64 = (l.f5e5 - t2e6);(l.f5ed, l.f5ee, l.f5ef, ) = (t2e7, 0.0, 0.0, );l.f5f0 = 0.0;let t2e8: f64 = (p.p85 - l.f601);let t2e9: f64 = (t2e8 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2e9, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t2ea: f64 = (4.0 * p.p85);let t2eb: f64 = (t2ea * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2eb, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {
            let (t2ed, t2ee, t2ef,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2ec: f64 = (-l.f6f7);
        (t2ec, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2ed, t2ee, t2ef, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t2f0: f64 = (l.f6f3 * l.f6f3);let t2f1: f64 = (t2f0 + l.f6f7);let t2f2: f64 = (t2f1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2f2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2f2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2f2)), );l.f6fa = 0.0;let t2f3: f64 = (l.f6f3 + l.f6f7);let t2f4: f64 = (0.5 * t2f3);let t2f5: f64 = (p.p85 - t2f4);(l.f605, l.f606, l.f607, ) = (t2f5, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t2f6: f64 = (l.f605 - l.f5e5);let t2f7: f64 = (t2f6 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2f7, l.f606, l.f607, );l.f6f6 = 0.0;let t2f8: f64 = (4.0 * l.f5e5);let t2f9: f64 = (t2f8 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2f9, 0.0, 0.0, );l.f6fa = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {
            let (t2fb, t2fc, t2fd,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2fa: f64 = (-l.f6f7);
        (t2fa, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2fb, t2fc, t2fd, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t2fe: f64 = (l.f6f3 * l.f6f3);let t2ff: f64 = (t2fe + l.f6f7);let t300: f64 = (t2ff).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t300, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t300)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t300)), );l.f6fa = 0.0;let t301: f64 = (l.f6f3 + l.f6f7);let t302: f64 = (0.5 * t301);let t303: f64 = (l.f5e5 + t302);(l.f5f1, l.f5f2, l.f5f3, ) = (t303, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t304: f64 = (p.p85 - l.f5ed);let t305: f64 = (t304 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t305, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t306: f64 = (4.0 * p.p85);let t307: f64 = (t306 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t307, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {
            let (t309, t30a, t30b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t308: f64 = (-l.f6f7);
        (t308, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t309, t30a, t30b, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t30c: f64 = (l.f6f3 * l.f6f3);let t30d: f64 = (t30c + l.f6f7);let t30e: f64 = (t30d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t30e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t30e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t30e)), );l.f6fa = 0.0;let t30f: f64 = (l.f6f3 + l.f6f7);let t310: f64 = (0.5 * t30f);let t311: f64 = (p.p85 - t310);(l.f5ed, l.f5ee, l.f5ef, ) = (t311, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t312: f64 = (l.f5ed - l.f5e5);let t313: f64 = (t312 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t313, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t314: f64 = (4.0 * l.f5e5);let t315: f64 = (t314 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t315, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {
            let (t317, t318, t319,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t316: f64 = (-l.f6f7);
        (t316, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t317, t318, t319, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t31a: f64 = (l.f6f3 * l.f6f3);let t31b: f64 = (t31a + l.f6f7);let t31c: f64 = (t31b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t31c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t31c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t31c)), );l.f6fa = 0.0;let t31d: f64 = (l.f6f3 + l.f6f7);let t31e: f64 = (0.5 * t31d);let t31f: f64 = (l.f5e5 + t31e);(l.f5ed, l.f5ee, l.f5ef, ) = (t31f, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );l.f5f4 = 0.0;}
        let t320: f64 = (l.f739 / l.f5f1);let t321: f64 = (l.f5f1 - l.f5ed);let t322: f64 = (l.f793 * t321);let t323: f64 = (l.f5ed * p.p85);let t324: f64 = (t322 / t323);let t325: f64 = (t320 + t324);let t326: f64 = (l.f645 * t325);let t327: f64 = (t326).abs();let t328: f64 = if t327 < 230.25850929940458 { 1.0 } else { 0.0 };l.f158 = t328;l.f159 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f158 != 0.0)) {let t329: f64 = (l.f739 / l.f5f1);let t32a: f64 = (l.f5f1 - l.f5ed);let t32b: f64 = (l.f793 * t32a);let t32c: f64 = (l.f5ed * p.p85);let t32d: f64 = (t32b / t32c);let t32e: f64 = (t329 + t32d);let t32f: f64 = (l.f645 * t32e);let t330: f64 = (t32f).exp();(l.f536, l.f537, l.f538, ) = (t330, (t330 * (l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t32c) - (t32b * (l.f5ee * p.p85))) / (t32c * t32c))))), (t330 * (l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t32c) - (t32b * (l.f5ef * p.p85))) / (t32c * t32c))))), );l.f539 = 0.0;}
        let t331: f64 = (l.f739 / l.f5f1);let t332: f64 = (l.f5f1 - l.f5ed);let t333: f64 = (l.f793 * t332);let t334: f64 = (l.f5ed * p.p85);let t335: f64 = (t333 / t334);let t336: f64 = (t331 + t335);let t337: f64 = (l.f645 * t336);let t338: f64 = (-230.25850929940458);let t339: f64 = if t337 < t338 { 1.0 } else { 0.0 };l.f15a = t339;l.f15b = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f158 == 0.0)) && (l.f15a != 0.0)) {let t33a: f64 = (-230.25850929940458);let t33b: f64 = (l.f739 / l.f5f1);let t33c: f64 = (l.f5f1 - l.f5ed);let t33d: f64 = (l.f793 * t33c);let t33e: f64 = (l.f5ed * p.p85);let t33f: f64 = (t33d / t33e);let t340: f64 = (t33b + t33f);let t341: f64 = (l.f645 * t340);let t342: f64 = (t33a - t341);let t343: f64 = (-230.25850929940458);let t344: f64 = (l.f739 / l.f5f1);let t345: f64 = (l.f5f1 - l.f5ed);let t346: f64 = (l.f793 * t345);let t347: f64 = (l.f5ed * p.p85);let t348: f64 = (t346 / t347);let t349: f64 = (t344 + t348);let t34a: f64 = (l.f645 * t349);let t34b: f64 = (t343 - t34a);let t34c: f64 = (-230.25850929940458);let t34d: f64 = (l.f739 / l.f5f1);let t34e: f64 = (l.f5f1 - l.f5ed);let t34f: f64 = (l.f793 * t34e);let t350: f64 = (l.f5ed * p.p85);let t351: f64 = (t34f / t350);let t352: f64 = (t34d + t351);let t353: f64 = (l.f645 * t352);let t354: f64 = (t34c - t353);let t355: f64 = (t354 * 0.3333333333333333);let t356: f64 = (1.0 + t355);let t357: f64 = (t34b * t356);let t358: f64 = (0.5 * t357);let t359: f64 = (1.0 + t358);let t35a: f64 = (t342 * t359);let t35b: f64 = (1.0 + t35a);let t35c: f64 = (1e-100 / t35b);(l.f536, l.f537, l.f538, ) = (t35c, (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t33e) - (t33d * (l.f5ee * p.p85))) / (t33e * t33e))))) * t359) + (t342 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t347) - (t346 * (l.f5ee * p.p85))) / (t347 * t347))))) * t356) + (t34b * ((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t350) - (t34f * (l.f5ee * p.p85))) / (t350 * t350))))) * 0.3333333333333333))))))) / (t35b * t35b))), (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t33e) - (t33d * (l.f5ef * p.p85))) / (t33e * t33e))))) * t359) + (t342 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t347) - (t346 * (l.f5ef * p.p85))) / (t347 * t347))))) * t356) + (t34b * ((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t350) - (t34f * (l.f5ef * p.p85))) / (t350 * t350))))) * 0.3333333333333333))))))) / (t35b * t35b))), );l.f539 = 0.0;}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f158 == 0.0)) && (l.f15a == 0.0)) {let t35d: f64 = (l.f739 / l.f5f1);let t35e: f64 = (l.f5f1 - l.f5ed);let t35f: f64 = (l.f793 * t35e);let t360: f64 = (l.f5ed * p.p85);let t361: f64 = (t35f / t360);let t362: f64 = (t35d + t361);let t363: f64 = (l.f645 * t362);let t364: f64 = (t363 - 230.25850929940458);let t365: f64 = (l.f739 / l.f5f1);let t366: f64 = (l.f5f1 - l.f5ed);let t367: f64 = (l.f793 * t366);let t368: f64 = (l.f5ed * p.p85);let t369: f64 = (t367 / t368);let t36a: f64 = (t365 + t369);let t36b: f64 = (l.f645 * t36a);let t36c: f64 = (t36b - 230.25850929940458);let t36d: f64 = (l.f739 / l.f5f1);let t36e: f64 = (l.f5f1 - l.f5ed);let t36f: f64 = (l.f793 * t36e);let t370: f64 = (l.f5ed * p.p85);let t371: f64 = (t36f / t370);let t372: f64 = (t36d + t371);let t373: f64 = (l.f645 * t372);let t374: f64 = (t373 - 230.25850929940458);let t375: f64 = (t374 * 0.3333333333333333);let t376: f64 = (1.0 + t375);let t377: f64 = (t36c * t376);let t378: f64 = (0.5 * t377);let t379: f64 = (1.0 + t378);let t37a: f64 = (t364 * t379);let t37b: f64 = (1.0 + t37a);let t37c: f64 = (1e100 * t37b);(l.f536, l.f537, l.f538, ) = (t37c, (1e100 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t360) - (t35f * (l.f5ee * p.p85))) / (t360 * t360)))) * t379) + (t364 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t368) - (t367 * (l.f5ee * p.p85))) / (t368 * t368)))) * t376) + (t36c * ((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t370) - (t36f * (l.f5ee * p.p85))) / (t370 * t370)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t360) - (t35f * (l.f5ef * p.p85))) / (t360 * t360)))) * t379) + (t364 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t368) - (t367 * (l.f5ef * p.p85))) / (t368 * t368)))) * t376) + (t36c * ((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t370) - (t36f * (l.f5ef * p.p85))) / (t370 * t370)))) * 0.3333333333333333))))))), );l.f539 = 0.0;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) {let t37d: f64 = (l.f5eb * l.f5eb);let t37e: f64 = (t37d / l.f5e3);l.f64f = t37e;l.f650 = 0.0;let t37f: f64 = (l.f5e9 / l.f645);let t380: f64 = (l.f5e3 / l.f64f);let t381: f64 = (t380).ln();let t382: f64 = (t37f * t381);l.f793 = t382;l.f794 = 0.0;}
        let t383: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f15c = t383;l.f15d = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t384: f64 = (l.f739 - l.f793);let t385: f64 = (p.p86 * t384);let t386: f64 = (t385 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t386, 0.0, 0.0, );l.f604 = 0.0;let t387: f64 = (p.p86 * l.f793);let t388: f64 = (l.f5e9 - t387);(l.f5ed, l.f5ee, l.f5ef, ) = (t388, 0.0, 0.0, );l.f5f0 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t389: f64 = (p.p85 - l.f601);let t38a: f64 = (t389 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t38a, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t38b: f64 = (4.0 * p.p85);let t38c: f64 = (t38b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t38c, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {
            let (t38e, t38f, t390,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t38d: f64 = (-l.f6f7);
        (t38d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t38e, t38f, t390, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t391: f64 = (l.f6f3 * l.f6f3);let t392: f64 = (t391 + l.f6f7);let t393: f64 = (t392).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t393, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t393)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t393)), );l.f6fa = 0.0;let t394: f64 = (l.f6f3 + l.f6f7);let t395: f64 = (0.5 * t394);let t396: f64 = (p.p85 - t395);(l.f605, l.f606, l.f607, ) = (t396, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t397: f64 = (l.f605 - l.f5e9);let t398: f64 = (t397 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t398, l.f606, l.f607, );l.f6f6 = 0.0;let t399: f64 = (4.0 * l.f5e9);let t39a: f64 = (t399 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t39a, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {
            let (t39c, t39d, t39e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t39b: f64 = (-l.f6f7);
        (t39b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t39c, t39d, t39e, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t39f: f64 = (l.f6f3 * l.f6f3);let t3a0: f64 = (t39f + l.f6f7);let t3a1: f64 = (t3a0).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3a1, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3a1)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3a1)), );l.f6fa = 0.0;let t3a2: f64 = (l.f6f3 + l.f6f7);let t3a3: f64 = (0.5 * t3a2);let t3a4: f64 = (l.f5e9 + t3a3);(l.f5f1, l.f5f2, l.f5f3, ) = (t3a4, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t3a5: f64 = (p.p85 - l.f5ed);let t3a6: f64 = (t3a5 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3a6, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t3a7: f64 = (4.0 * p.p85);let t3a8: f64 = (t3a7 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3a8, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {
            let (t3aa, t3ab, t3ac,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3a9: f64 = (-l.f6f7);
        (t3a9, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3aa, t3ab, t3ac, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t3ad: f64 = (l.f6f3 * l.f6f3);let t3ae: f64 = (t3ad + l.f6f7);let t3af: f64 = (t3ae).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3af, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3af)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3af)), );l.f6fa = 0.0;let t3b0: f64 = (l.f6f3 + l.f6f7);let t3b1: f64 = (0.5 * t3b0);let t3b2: f64 = (p.p85 - t3b1);(l.f5ed, l.f5ee, l.f5ef, ) = (t3b2, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t3b3: f64 = (l.f5ed - l.f5e9);let t3b4: f64 = (t3b3 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3b4, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t3b5: f64 = (4.0 * l.f5e9);let t3b6: f64 = (t3b5 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3b6, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {
            let (t3b8, t3b9, t3ba,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3b7: f64 = (-l.f6f7);
        (t3b7, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3b8, t3b9, t3ba, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t3bb: f64 = (l.f6f3 * l.f6f3);let t3bc: f64 = (t3bb + l.f6f7);let t3bd: f64 = (t3bc).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3bd, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3bd)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3bd)), );l.f6fa = 0.0;let t3be: f64 = (l.f6f3 + l.f6f7);let t3bf: f64 = (0.5 * t3be);let t3c0: f64 = (l.f5e9 + t3bf);(l.f5ed, l.f5ee, l.f5ef, ) = (t3c0, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );l.f5f4 = 0.0;}
    }
}
