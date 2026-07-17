#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_128(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f35e != 0.0)) {let t0: f64 = (2.0 + l.f817);let t1: f64 = (l.f817 + 1.0);let t2: f64 = (l.f817 + 3.0);let t3: f64 = (t1 * t2);let t4: f64 = (t3).sqrt();let t5: f64 = (t0 + t4);let t6: f64 = (t5).ln();let t7: f64 = (l.f643 * t6);let t8: f64 = (2.0 * t7);l.f714 = t8;}
        if (((l.f29a != 0.0) && (l.f330 != 0.0)) && (l.f35e == 0.0)) {let t9: f64 = (-l.f73f);let ta: f64 = (2.0 * l.f825);let tb: f64 = (ta + 1.0);let tc: f64 = (1.0 + l.f825);let td: f64 = (3.0 * l.f825);let te: f64 = (1.0 + td);let tf: f64 = (tc * te);let t10: f64 = (tf).sqrt();let t11: f64 = (tb + t10);let t12: f64 = (t11).ln();let t13: f64 = (l.f643 * t12);let t14: f64 = (2.0 * t13);let t15: f64 = (t9 + t14);l.f714 = t15;}
        if ((l.f29a != 0.0) && (l.f330 != 0.0)) {let t16: f64 = (l.f76f - l.f714);l.f79c = t16;let t17: f64 = (l.f73f + l.f79c);let t18: f64 = (l.f73f - l.f79c);let t19: f64 = (l.f73f - l.f79c);let t1a: f64 = (t18 * t19);let t1b: f64 = (4.0 * l.f643);let t1c: f64 = (t1b * l.f643);let t1d: f64 = (t1a + t1c);let t1e: f64 = (t1d).sqrt();let t1f: f64 = (t17 - t1e);let t20: f64 = (0.5 * t1f);l.f7a2 = t20;let t21: f64 = (l.f73f + l.f755);let t22: f64 = (l.f73f - l.f755);let t23: f64 = (l.f73f - l.f755);let t24: f64 = (t22 * t23);let t25: f64 = (4.0 * l.f647);let t26: f64 = (t25 * l.f647);let t27: f64 = (t24 + t26);let t28: f64 = (t27).sqrt();let t29: f64 = (t21 - t28);let t2a: f64 = (0.5 * t29);l.f750 = t2a;let t2b: f64 = l.f73f;let t2c: f64 = l.f73f;let t2d: f64 = l.f73f;let t2e: f64 = (t2c * t2d);let t2f: f64 = (4.0 * 1e-6);let t30: f64 = (t2f * 1e-6);let t31: f64 = (t2e + t30);let t32: f64 = (t31).sqrt();let t33: f64 = (t2b - t32);let t34: f64 = (0.5 * t33);l.f74a = t34;}
        if ((l.f29a != 0.0) && (l.f330 == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f714 = 0.0;l.f796 = 0.0;l.f825 = 0.0;l.f7a2 = 0.0;l.f750 = 0.0;l.f74a = 0.0;}
        let t35: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f360 = t35;
        if ((l.f29a != 0.0) && (l.f360 != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );}
        let t36: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f362 = t36;
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f362 != 0.0)) {let t37: f64 = (l.f796 * l.f769);let t38: f64 = (1.0 - t37);let t39: f64 = (t38).sqrt();l.f6fc = t39;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f362 == 0.0)) {let t3a: f64 = (l.f796 * l.f769);let t3b: f64 = (1.0 - t3a);let t3c: f64 = (t3b).powf(l.f60b);l.f6fc = t3c;}
        if ((l.f29a != 0.0) && (l.f360 == 0.0)) {let t3d: f64 = (1.0 - l.f6fc);let t3e: f64 = (l.f69e * t3d);let t3f: f64 = (l.f73f - l.f796);let t40: f64 = (l.f698 * t3f);let t41: f64 = (t3e + t40);(l.f68c, l.f68d, l.f68e, ) = (t41, 0.0, 0.0, );let t42: f64 = (l.f542 * l.f536);(l.f52f, l.f530, l.f531, ) = (t42, (l.f542 * l.f537), (l.f542 * l.f538), );}
        let t43: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.f364 = t43;
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_129(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 != 0.0)) {l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 == 0.0)) {let t44: f64 = (l.f75d - l.f7a2);l.f758 = t44;let t45: f64 = (l.f714 / l.f758);let t46: f64 = (1.0 - t45);let t47: f64 = (t46).sqrt();let t48: f64 = (1.0 - t47);l.f7ef = t48;}
        let t49: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f366 = t49;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 == 0.0)) && (l.f366 != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 == 0.0)) && (l.f366 == 0.0)) {let t4a: f64 = (l.f7ef * l.f7ef);let t4b: f64 = (l.f7ef).ln();let t4c: f64 = (t4a * t4b);let t4d: f64 = (1.0 - l.f7ef);let t4e: f64 = (t4c / t4d);let t4f: f64 = (t4e + l.f7ef);let t50: f64 = (2.0 * l.f623);let t51: f64 = (1.0 - t50);let t52: f64 = (t4f * t51);l.f66 = t52;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 == 0.0)) {let t53: f64 = (l.f7ef + l.f66);l.f7e9 = t53;}
        let t54: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f368 = t54;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 == 0.0)) && (l.f368 != 0.0)) {let t55: f64 = (l.f758 * l.f773);let t56: f64 = (t55).sqrt();l.f6fc = t56;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 == 0.0)) && (l.f368 == 0.0)) {let t57: f64 = (l.f758 * l.f773);let t58: f64 = (t57).powf(l.f623);l.f6fc = t58;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f364 == 0.0)) {let t59: f64 = (l.f7d6 * l.f6fc);l.f7d1 = t59;let t5a: f64 = (l.f825 - 1.0);let t5b: f64 = (t5a * l.f7d1);let t5c: f64 = (l.fc9 * t5b);l.f9 = t5c;let t5d: f64 = (l.f9 * l.f7e9);let t5e: f64 = (l.f39 * t5d);l.f593 = t5e;}
        let t5f: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.f36a = t5f;
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) {let t60: f64 = (l.f7d1 * l.f60b);let t61: f64 = (t60 / l.f758);let t62: f64 = (l.f1e * t61);l.f19 = t62;let t63: f64 = (0.666666666666667 * l.fe);let t64: f64 = (t63 / l.f19);l.f71a = t64;let t65: f64 = (l.f71a * l.f71a);l.f72c = t65;let t66: f64 = (l.f72c * l.f72c);let t67: f64 = (l.f72c * l.f72c);let t68: f64 = (t67 + 1.0);let t69: f64 = (t66 / t68);let t6a: f64 = (t69).sqrt();l.f726 = t6a;let t6b: f64 = (l.f726).abs();let t6c: f64 = (t6b).sqrt();l.f6c1 = t6c;let t6d: f64 = (l.f726 * l.f6c1);l.f732 = t6d;}
        let t6e: f64 = (-l.f623);let t6f: f64 = (t6e * l.f611);let t70: f64 = (-1.0);let t71: f64 = if t6f == t70 { 1.0 } else { 0.0 };l.f36c = t71;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f36c != 0.0)) {let t72: f64 = (l.f19 * l.f732);let t73: f64 = (1.0 + t72);let t74: f64 = (1.0 / t73);l.f7e3 = t74;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f36c == 0.0)) {let t75: f64 = (l.f19 * l.f732);let t76: f64 = (1.0 + t75);let t77: f64 = (-l.f623);let t78: f64 = (t77 * l.f611);let t79: f64 = (t76).powf(t78);l.f7e3 = t79;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) {let t7a: f64 = (l.f7e9 * l.f7e3);let t7b: f64 = (l.f7e9 + l.f7e3);let t7c: f64 = (t7a / t7b);l.f7f5 = t7c;let t7d: f64 = (l.f19 / l.f6c1);let t7e: f64 = (0.375 * t7d);let t7f: f64 = (t7e).sqrt();l.f5a8 = t7f;let t80: f64 = (l.f71a * l.f6c1);let t81: f64 = (2.0 * t80);let t82: f64 = (t81 - l.f726);l.f5b4 = t82;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_130(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) {let t83: f64 = (l.fe * l.f71a);let t84: f64 = (t83 * l.f6c1);let t85: f64 = (l.fe * l.f726);let t86: f64 = (t84 - t85);let t87: f64 = (l.f19 * l.f732);let t88: f64 = (0.5 * t87);let t89: f64 = (t86 + t88);l.f5d4 = t89;let t8a: f64 = (l.f5b4 - 1.0);let t8b: f64 = (t8a * l.f5a8);l.f7fb = t8b;let t8c: f64 = (l.f7fb * l.f7fb);l.f811 = t8c;}
        let t8d: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f36e = t8d;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f36e != 0.0)) {let t8e: f64 = (l.f62b * l.f7fb);let t8f: f64 = (1.0 + t8e);let t90: f64 = (1.0 / t8f);l.f6e2 = t90;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f36e == 0.0)) {let t91: f64 = (l.f62b * l.f7fb);let t92: f64 = (1.0 - t91);let t93: f64 = (1.0 / t92);l.f6e2 = t93;}
        let t94: f64 = (-l.f811);let t95: f64 = (t94 + l.f5d4);let t96: f64 = (-230.25850929940458);let t97: f64 = if t95 > t96 { 1.0 } else { 0.0 };l.f370 = t97;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f370 != 0.0)) {let t98: f64 = (-l.f811);let t99: f64 = (t98 + l.f5d4);let t9a: f64 = (t99).exp();l.f6fc = t9a;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f370 == 0.0)) {let t9b: f64 = (-230.25850929940458);let t9c: f64 = (-l.f811);let t9d: f64 = (t9c + l.f5d4);let t9e: f64 = (t9b - t9d);let t9f: f64 = (-230.25850929940458);let ta0: f64 = (-l.f811);let ta1: f64 = (ta0 + l.f5d4);let ta2: f64 = (t9f - ta1);let ta3: f64 = (-230.25850929940458);let ta4: f64 = (-l.f811);let ta5: f64 = (ta4 + l.f5d4);let ta6: f64 = (ta3 - ta5);let ta7: f64 = (ta6 * 0.3333333333333333);let ta8: f64 = (1.0 + ta7);let ta9: f64 = (ta2 * ta8);let taa: f64 = (0.5 * ta9);let tab: f64 = (1.0 + taa);let tac: f64 = (t9e * tab);let tad: f64 = (1.0 + tac);let tae: f64 = (1e-100 / tad);l.f6fc = tae;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) {let taf: f64 = (0.29214664 * l.f6e2);let tb0: f64 = (l.f6e2 * l.f6e2);let tb1: f64 = (l.f16 * tb0);let tb2: f64 = (taf + tb1);let tb3: f64 = (l.f6e2 * l.f6e2);let tb4: f64 = (tb3 * l.f6e2);let tb5: f64 = (l.f2a * tb4);let tb6: f64 = (tb2 + tb5);let tb7: f64 = (tb6 * l.f6fc);l.f6e = tb7;}
        let tb8: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f372 = tb8;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f372 != 0.0)) {l.f74 = l.f6e;}
        let tb9: f64 = (-230.25850929940458);let tba: f64 = if l.f5d4 > tb9 { 1.0 } else { 0.0 };l.f374 = tba;
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f372 == 0.0)) && (l.f374 != 0.0)) {let tbb: f64 = (l.f5d4).exp();l.f6fc = tbb;}
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f372 == 0.0)) && (l.f374 == 0.0)) {let tbc: f64 = (-230.25850929940458);let tbd: f64 = (tbc - l.f5d4);let tbe: f64 = (-230.25850929940458);let tbf: f64 = (tbe - l.f5d4);let tc0: f64 = (-230.25850929940458);let tc1: f64 = (tc0 - l.f5d4);let tc2: f64 = (tc1 * 0.3333333333333333);let tc3: f64 = (1.0 + tc2);let tc4: f64 = (tbf * tc3);let tc5: f64 = (0.5 * tc4);let tc6: f64 = (1.0 + tc5);let tc7: f64 = (tbd * tc6);let tc8: f64 = (1.0 + tc7);let tc9: f64 = (1e-100 / tc8);l.f6fc = tc9;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) && (l.f372 == 0.0)) {let tca: f64 = (2.0 * l.f6fc);let tcb: f64 = (tca - l.f6e);l.f74 = tcb;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) {let tcc: f64 = (1.772453850905516 * 0.5);let tcd: f64 = (l.fe * l.f74);let tce: f64 = (tcd / l.f5a8);let tcf: f64 = (tcc * tce);l.fd6 = tcf;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_131(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f36a == 0.0)) {let td0: f64 = (l.f9 * l.fd6);let td1: f64 = (td0 * l.f7f5);let td2: f64 = (l.f3f * td1);l.f599 = td2;}
        let td3: f64 = if l.f24 == 0.0 { 1.0 } else { 0.0 };l.f376 = td3;
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 != 0.0)) {l.f529 = 0.0;}
        let td4: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f378 = td4;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f378 != 0.0)) {let td5: f64 = (l.f771 - l.f750);let td6: f64 = (td5 * l.f773);let td7: f64 = (td6).sqrt();l.f6fc = td7;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f378 == 0.0)) {let td8: f64 = (l.f771 - l.f750);let td9: f64 = (td8 * l.f773);let tda: f64 = (td9).powf(l.f623);l.f6fc = tda;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) {let tdb: f64 = (l.f771 - l.f750);let tdc: f64 = (tdb * l.f7da);let tdd: f64 = (tdc / l.f6fc);let tde: f64 = (l.f611 * tdd);l.fb6 = tde;}
        let tdf: f64 = (-l.fa1);let te0: f64 = (tdf / l.fb6);let te1: f64 = (te0).abs();let te2: f64 = if te1 < 230.25850929940458 { 1.0 } else { 0.0 };l.f37a = te2;
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f37a != 0.0)) {let te3: f64 = (-l.fa1);let te4: f64 = (te3 / l.fb6);let te5: f64 = (te4).exp();l.f6fc = te5;}
        let te6: f64 = (-l.fa1);let te7: f64 = (te6 / l.fb6);let te8: f64 = (-230.25850929940458);let te9: f64 = if te7 < te8 { 1.0 } else { 0.0 };l.f37c = te9;
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f37a == 0.0)) && (l.f37c != 0.0)) {let tea: f64 = (-230.25850929940458);let teb: f64 = (-l.fa1);let tec: f64 = (teb / l.fb6);let ted: f64 = (tea - tec);let tee: f64 = (-230.25850929940458);let tef: f64 = (-l.fa1);let tf0: f64 = (tef / l.fb6);let tf1: f64 = (tee - tf0);let tf2: f64 = (-230.25850929940458);let tf3: f64 = (-l.fa1);let tf4: f64 = (tf3 / l.fb6);let tf5: f64 = (tf2 - tf4);let tf6: f64 = (tf5 * 0.3333333333333333);let tf7: f64 = (1.0 + tf6);let tf8: f64 = (tf1 * tf7);let tf9: f64 = (0.5 * tf8);let tfa: f64 = (1.0 + tf9);let tfb: f64 = (ted * tfa);let tfc: f64 = (1.0 + tfb);let tfd: f64 = (1e-100 / tfc);l.f6fc = tfd;}
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) && (l.f37a == 0.0)) && (l.f37c == 0.0)) {let tfe: f64 = (-l.fa1);let tff: f64 = (tfe / l.fb6);let t100: f64 = (tff - 230.25850929940458);let t101: f64 = (-l.fa1);let t102: f64 = (t101 / l.fb6);let t103: f64 = (t102 - 230.25850929940458);let t104: f64 = (-l.fa1);let t105: f64 = (t104 / l.fb6);let t106: f64 = (t105 - 230.25850929940458);let t107: f64 = (t106 * 0.3333333333333333);let t108: f64 = (1.0 + t107);let t109: f64 = (t103 * t108);let t10a: f64 = (0.5 * t109);let t10b: f64 = (1.0 + t10a);let t10c: f64 = (t100 * t10b);let t10d: f64 = (1.0 + t10c);let t10e: f64 = (1e100 * t10d);l.f6fc = t10e;}
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f376 == 0.0)) {let t10f: f64 = (l.f73f * l.fb6);let t110: f64 = (t10f * l.fb6);let t111: f64 = (t110 * l.f6fc);let t112: f64 = (l.f24 * t111);l.f529 = t112;}
        let t113: f64 = if ((l.f783 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f37e = t113;
        if (((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e != 0.0)) {l.fae = 1.0;}
        let t114: f64 = (-l.f2);let t115: f64 = (t114 * l.f783);let t116: f64 = if l.f74a > t115 { 1.0 } else { 0.0 };l.f380 = t116;let t117: f64 = if l.f625 == 4.0 { 1.0 } else { 0.0 };l.f382 = t117;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_132(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e == 0.0)) && (l.f380 != 0.0)) && (l.f382 != 0.0)) {let t118: f64 = (l.f74a * l.f787);let t119: f64 = (t118).abs();let t11a: f64 = (l.f74a * l.f787);let t11b: f64 = (t11a).abs();let t11c: f64 = (t119 * t11b);let t11d: f64 = (l.f74a * l.f787);let t11e: f64 = (t11d).abs();let t11f: f64 = (t11c * t11e);let t120: f64 = (l.f74a * l.f787);let t121: f64 = (t120).abs();let t122: f64 = (t11f * t121);l.f6fc = t122;}
        if (((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e == 0.0)) && (l.f380 != 0.0)) && (l.f382 == 0.0)) {let t123: f64 = (l.f74a * l.f787);let t124: f64 = (t123).abs();let t125: f64 = (t124).powf(l.f625);l.f6fc = t125;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e == 0.0)) && (l.f380 != 0.0)) {let t126: f64 = (1.0 - l.f6fc);let t127: f64 = (1.0 / t126);l.fae = t127;}
        if ((((l.f29a != 0.0) && (l.f360 == 0.0)) && (l.f37e == 0.0)) && (l.f380 == 0.0)) {let t128: f64 = (l.f2 * l.f783);let t129: f64 = (l.f74a + t128);let t12a: f64 = (t129 * l.f6ba);let t12b: f64 = (l.fc3 + t12a);l.fae = t12b;}
        if ((l.f29a != 0.0) && (l.f360 == 0.0)) {let t12c: f64 = (l.f52f + l.f593);let t12d: f64 = (t12c + l.f599);let t12e: f64 = (t12d + l.f529);let t12f: f64 = (t12e * l.fae);(l.f562, l.f563, l.f564, ) = (t12f, (l.f530 * l.fae), (l.f531 * l.fae), );let t130: f64 = (l.f593 + l.f599);let t131: f64 = (t130 + l.f529);let t132: f64 = (t131 * l.fae);(l.f552, l.f553, l.f554, ) = (t132, 0.0, 0.0, );}
        let t133: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f384 = t133;
        if ((l.f29a != 0.0) && (l.f384 != 0.0)) {(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );}
        let t134: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f386 = t134;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f386 != 0.0)) {let t135: f64 = (l.f796 * l.f76d);let t136: f64 = (1.0 - t135);let t137: f64 = (t136).sqrt();l.f6fc = t137;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f386 == 0.0)) {let t138: f64 = (l.f796 * l.f76d);let t139: f64 = (1.0 - t138);let t13a: f64 = (t139).powf(l.f60f);l.f6fc = t13a;}
        if ((l.f29a != 0.0) && (l.f384 == 0.0)) {let t13b: f64 = (1.0 - l.f6fc);let t13c: f64 = (l.f6a2 * t13b);let t13d: f64 = (l.f73f - l.f796);let t13e: f64 = (l.f69c * t13d);let t13f: f64 = (t13c + t13e);(l.f694, l.f695, l.f696, ) = (t13f, 0.0, 0.0, );let t140: f64 = (l.f54c * l.f53e);(l.f52f, l.f530, l.f531, ) = (t140, (l.f54c * l.f53f), (l.f54c * l.f540), );}
        let t141: f64 = if ((l.f3d == 0.0) && (l.f43 == 0.0)) { 1.0 } else { 0.0 };l.f388 = t141;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) {let t142: f64 = (l.f77d - l.f7a2);l.f758 = t142;let t143: f64 = (l.f714 / l.f758);let t144: f64 = (1.0 - t143);let t145: f64 = (t144).sqrt();let t146: f64 = (1.0 - t145);l.f7ef = t146;}
        let t147: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f38a = t147;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) && (l.f38a != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) && (l.f38a == 0.0)) {let t148: f64 = (l.f7ef * l.f7ef);let t149: f64 = (l.f7ef).ln();let t14a: f64 = (t148 * t149);let t14b: f64 = (1.0 - l.f7ef);let t14c: f64 = (t14a / t14b);let t14d: f64 = (t14c + l.f7ef);let t14e: f64 = (2.0 * l.f653);let t14f: f64 = (1.0 - t14e);let t150: f64 = (t14d * t14f);l.f66 = t150;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) {let t151: f64 = (l.f7ef + l.f66);l.f7e9 = t151;}
        let t152: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f38c = t152;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) && (l.f38c != 0.0)) {let t153: f64 = (l.f758 * l.f77b);let t154: f64 = (t153).sqrt();l.f6fc = t154;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) && (l.f38c == 0.0)) {let t155: f64 = (l.f758 * l.f77b);let t156: f64 = (t155).powf(l.f653);l.f6fc = t156;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) {let t157: f64 = (l.f7e0 * l.f6fc);l.f7d1 = t157;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_133(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f388 == 0.0)) {let t158: f64 = (l.f825 - 1.0);let t159: f64 = (t158 * l.f7d1);let t15a: f64 = (l.fd1 * t159);l.f9 = t15a;let t15b: f64 = (l.f9 * l.f7e9);let t15c: f64 = (l.f3d * t15b);l.f593 = t15c;}
        let t15d: f64 = if l.f43 == 0.0 { 1.0 } else { 0.0 };l.f38e = t15d;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let t15e: f64 = (l.f7d1 * l.f60f);let t15f: f64 = (t15e / l.f758);let t160: f64 = (l.f22 * t15f);l.f19 = t160;let t161: f64 = (0.666666666666667 * l.f12);let t162: f64 = (t161 / l.f19);l.f71a = t162;let t163: f64 = (l.f71a * l.f71a);l.f72c = t163;let t164: f64 = (l.f72c * l.f72c);let t165: f64 = (l.f72c * l.f72c);let t166: f64 = (t165 + 1.0);let t167: f64 = (t164 / t166);let t168: f64 = (t167).sqrt();l.f726 = t168;let t169: f64 = (l.f726).abs();let t16a: f64 = (t169).sqrt();l.f6c1 = t16a;let t16b: f64 = (l.f726 * l.f6c1);l.f732 = t16b;}
        let t16c: f64 = (-l.f653);let t16d: f64 = (t16c * l.f615);let t16e: f64 = (-1.0);let t16f: f64 = if t16d == t16e { 1.0 } else { 0.0 };l.f390 = t16f;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f390 != 0.0)) {let t170: f64 = (l.f19 * l.f732);let t171: f64 = (1.0 + t170);let t172: f64 = (1.0 / t171);l.f7e3 = t172;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f390 == 0.0)) {let t173: f64 = (l.f19 * l.f732);let t174: f64 = (1.0 + t173);let t175: f64 = (-l.f653);let t176: f64 = (t175 * l.f615);let t177: f64 = (t174).powf(t176);l.f7e3 = t177;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let t178: f64 = (l.f7e9 * l.f7e3);let t179: f64 = (l.f7e9 + l.f7e3);let t17a: f64 = (t178 / t179);l.f7f5 = t17a;let t17b: f64 = (l.f19 / l.f6c1);let t17c: f64 = (0.375 * t17b);let t17d: f64 = (t17c).sqrt();l.f5a8 = t17d;let t17e: f64 = (l.f71a * l.f6c1);let t17f: f64 = (2.0 * t17e);let t180: f64 = (t17f - l.f726);l.f5b4 = t180;let t181: f64 = (l.f12 * l.f71a);let t182: f64 = (t181 * l.f6c1);let t183: f64 = (l.f12 * l.f726);let t184: f64 = (t182 - t183);let t185: f64 = (l.f19 * l.f732);let t186: f64 = (0.5 * t185);let t187: f64 = (t184 + t186);l.f5d4 = t187;let t188: f64 = (l.f5b4 - 1.0);let t189: f64 = (t188 * l.f5a8);l.f7fb = t189;let t18a: f64 = (l.f7fb * l.f7fb);l.f811 = t18a;}
        let t18b: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f392 = t18b;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f392 != 0.0)) {let t18c: f64 = (l.f62b * l.f7fb);let t18d: f64 = (1.0 + t18c);let t18e: f64 = (1.0 / t18d);l.f6e2 = t18e;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f392 == 0.0)) {let t18f: f64 = (l.f62b * l.f7fb);let t190: f64 = (1.0 - t18f);let t191: f64 = (1.0 / t190);l.f6e2 = t191;}
        let t192: f64 = (-l.f811);let t193: f64 = (t192 + l.f5d4);let t194: f64 = (-230.25850929940458);let t195: f64 = if t193 > t194 { 1.0 } else { 0.0 };l.f394 = t195;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f394 != 0.0)) {let t196: f64 = (-l.f811);let t197: f64 = (t196 + l.f5d4);let t198: f64 = (t197).exp();l.f6fc = t198;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_134(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f394 == 0.0)) {let t199: f64 = (-230.25850929940458);let t19a: f64 = (-l.f811);let t19b: f64 = (t19a + l.f5d4);let t19c: f64 = (t199 - t19b);let t19d: f64 = (-230.25850929940458);let t19e: f64 = (-l.f811);let t19f: f64 = (t19e + l.f5d4);let t1a0: f64 = (t19d - t19f);let t1a1: f64 = (-230.25850929940458);let t1a2: f64 = (-l.f811);let t1a3: f64 = (t1a2 + l.f5d4);let t1a4: f64 = (t1a1 - t1a3);let t1a5: f64 = (t1a4 * 0.3333333333333333);let t1a6: f64 = (1.0 + t1a5);let t1a7: f64 = (t1a0 * t1a6);let t1a8: f64 = (0.5 * t1a7);let t1a9: f64 = (1.0 + t1a8);let t1aa: f64 = (t19c * t1a9);let t1ab: f64 = (1.0 + t1aa);let t1ac: f64 = (1e-100 / t1ab);l.f6fc = t1ac;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let t1ad: f64 = (0.29214664 * l.f6e2);let t1ae: f64 = (l.f6e2 * l.f6e2);let t1af: f64 = (l.f16 * t1ae);let t1b0: f64 = (t1ad + t1af);let t1b1: f64 = (l.f6e2 * l.f6e2);let t1b2: f64 = (t1b1 * l.f6e2);let t1b3: f64 = (l.f2a * t1b2);let t1b4: f64 = (t1b0 + t1b3);let t1b5: f64 = (t1b4 * l.f6fc);l.f6e = t1b5;}
        let t1b6: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f396 = t1b6;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f396 != 0.0)) {l.f74 = l.f6e;}
        let t1b7: f64 = (-230.25850929940458);let t1b8: f64 = if l.f5d4 > t1b7 { 1.0 } else { 0.0 };l.f398 = t1b8;
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f396 == 0.0)) && (l.f398 != 0.0)) {let t1b9: f64 = (l.f5d4).exp();l.f6fc = t1b9;}
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f396 == 0.0)) && (l.f398 == 0.0)) {let t1ba: f64 = (-230.25850929940458);let t1bb: f64 = (t1ba - l.f5d4);let t1bc: f64 = (-230.25850929940458);let t1bd: f64 = (t1bc - l.f5d4);let t1be: f64 = (-230.25850929940458);let t1bf: f64 = (t1be - l.f5d4);let t1c0: f64 = (t1bf * 0.3333333333333333);let t1c1: f64 = (1.0 + t1c0);let t1c2: f64 = (t1bd * t1c1);let t1c3: f64 = (0.5 * t1c2);let t1c4: f64 = (1.0 + t1c3);let t1c5: f64 = (t1bb * t1c4);let t1c6: f64 = (1.0 + t1c5);let t1c7: f64 = (1e-100 / t1c6);l.f6fc = t1c7;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) && (l.f396 == 0.0)) {let t1c8: f64 = (2.0 * l.f6fc);let t1c9: f64 = (t1c8 - l.f6e);l.f74 = t1c9;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f38e == 0.0)) {let t1ca: f64 = (1.772453850905516 * 0.5);let t1cb: f64 = (l.f12 * l.f74);let t1cc: f64 = (t1cb / l.f5a8);let t1cd: f64 = (t1ca * t1cc);l.fd6 = t1cd;let t1ce: f64 = (l.f9 * l.fd6);let t1cf: f64 = (t1ce * l.f7f5);let t1d0: f64 = (l.f43 * t1cf);l.f599 = t1d0;}
        let t1d1: f64 = if l.f28 == 0.0 { 1.0 } else { 0.0 };l.f39a = t1d1;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a != 0.0)) {l.f529 = 0.0;}
        let t1d2: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f39c = t1d2;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39c != 0.0)) {let t1d3: f64 = (l.f779 - l.f750);let t1d4: f64 = (t1d3 * l.f77b);let t1d5: f64 = (t1d4).sqrt();l.f6fc = t1d5;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39c == 0.0)) {let t1d6: f64 = (l.f779 - l.f750);let t1d7: f64 = (t1d6 * l.f77b);let t1d8: f64 = (t1d7).powf(l.f653);l.f6fc = t1d8;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) {let t1d9: f64 = (l.f779 - l.f750);let t1da: f64 = (t1d9 * l.f7de);let t1db: f64 = (t1da / l.f6fc);let t1dc: f64 = (l.f615 * t1db);l.fb6 = t1dc;}
        let t1dd: f64 = (-l.fab);let t1de: f64 = (t1dd / l.fb6);let t1df: f64 = (t1de).abs();let t1e0: f64 = if t1df < 230.25850929940458 { 1.0 } else { 0.0 };l.f39e = t1e0;
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39e != 0.0)) {let t1e1: f64 = (-l.fab);let t1e2: f64 = (t1e1 / l.fb6);let t1e3: f64 = (t1e2).exp();l.f6fc = t1e3;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_135(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t1e4: f64 = (-l.fab);let t1e5: f64 = (t1e4 / l.fb6);let t1e6: f64 = (-230.25850929940458);let t1e7: f64 = if t1e5 < t1e6 { 1.0 } else { 0.0 };l.f3a0 = t1e7;
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39e == 0.0)) && (l.f3a0 != 0.0)) {let t1e8: f64 = (-230.25850929940458);let t1e9: f64 = (-l.fab);let t1ea: f64 = (t1e9 / l.fb6);let t1eb: f64 = (t1e8 - t1ea);let t1ec: f64 = (-230.25850929940458);let t1ed: f64 = (-l.fab);let t1ee: f64 = (t1ed / l.fb6);let t1ef: f64 = (t1ec - t1ee);let t1f0: f64 = (-230.25850929940458);let t1f1: f64 = (-l.fab);let t1f2: f64 = (t1f1 / l.fb6);let t1f3: f64 = (t1f0 - t1f2);let t1f4: f64 = (t1f3 * 0.3333333333333333);let t1f5: f64 = (1.0 + t1f4);let t1f6: f64 = (t1ef * t1f5);let t1f7: f64 = (0.5 * t1f6);let t1f8: f64 = (1.0 + t1f7);let t1f9: f64 = (t1eb * t1f8);let t1fa: f64 = (1.0 + t1f9);let t1fb: f64 = (1e-100 / t1fa);l.f6fc = t1fb;}
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) && (l.f39e == 0.0)) && (l.f3a0 == 0.0)) {let t1fc: f64 = (-l.fab);let t1fd: f64 = (t1fc / l.fb6);let t1fe: f64 = (t1fd - 230.25850929940458);let t1ff: f64 = (-l.fab);let t200: f64 = (t1ff / l.fb6);let t201: f64 = (t200 - 230.25850929940458);let t202: f64 = (-l.fab);let t203: f64 = (t202 / l.fb6);let t204: f64 = (t203 - 230.25850929940458);let t205: f64 = (t204 * 0.3333333333333333);let t206: f64 = (1.0 + t205);let t207: f64 = (t201 * t206);let t208: f64 = (0.5 * t207);let t209: f64 = (1.0 + t208);let t20a: f64 = (t1fe * t209);let t20b: f64 = (1.0 + t20a);let t20c: f64 = (1e100 * t20b);l.f6fc = t20c;}
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f39a == 0.0)) {let t20d: f64 = (l.f73f * l.fb6);let t20e: f64 = (t20d * l.fb6);let t20f: f64 = (t20e * l.f6fc);let t210: f64 = (l.f28 * t20f);l.f529 = t210;}
        let t211: f64 = if ((l.f78d > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f3a2 = t211;
        if (((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 != 0.0)) {l.fae = 1.0;}
        let t212: f64 = (-l.f2);let t213: f64 = (t212 * l.f78d);let t214: f64 = if l.f74a > t213 { 1.0 } else { 0.0 };l.f3a4 = t214;let t215: f64 = if l.f629 == 4.0 { 1.0 } else { 0.0 };l.f3a6 = t215;
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 == 0.0)) && (l.f3a4 != 0.0)) && (l.f3a6 != 0.0)) {let t216: f64 = (l.f74a * l.f78b);let t217: f64 = (t216).abs();let t218: f64 = (l.f74a * l.f78b);let t219: f64 = (t218).abs();let t21a: f64 = (t217 * t219);let t21b: f64 = (l.f74a * l.f78b);let t21c: f64 = (t21b).abs();let t21d: f64 = (t21a * t21c);let t21e: f64 = (l.f74a * l.f78b);let t21f: f64 = (t21e).abs();let t220: f64 = (t21d * t21f);l.f6fc = t220;}
        if (((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 == 0.0)) && (l.f3a4 != 0.0)) && (l.f3a6 == 0.0)) {let t221: f64 = (l.f74a * l.f78b);let t222: f64 = (t221).abs();let t223: f64 = (t222).powf(l.f629);l.f6fc = t223;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 == 0.0)) && (l.f3a4 != 0.0)) {let t224: f64 = (1.0 - l.f6fc);let t225: f64 = (1.0 / t224);l.fae = t225;}
        if ((((l.f29a != 0.0) && (l.f384 == 0.0)) && (l.f3a2 == 0.0)) && (l.f3a4 == 0.0)) {let t226: f64 = (l.f2 * l.f78d);let t227: f64 = (l.f74a + t226);let t228: f64 = (t227 * l.f6be);let t229: f64 = (l.fc7 + t228);l.fae = t229;}
        if ((l.f29a != 0.0) && (l.f384 == 0.0)) {let t22a: f64 = (l.f52f + l.f593);let t22b: f64 = (t22a + l.f599);let t22c: f64 = (t22b + l.f529);let t22d: f64 = (t22c * l.fae);(l.f576, l.f577, l.f578, ) = (t22d, (l.f530 * l.fae), (l.f531 * l.fae), );let t22e: f64 = (l.f593 + l.f599);let t22f: f64 = (t22e + l.f529);let t230: f64 = (t22f * l.fae);(l.f55a, l.f55b, l.f55c, ) = (t230, 0.0, 0.0, );}
        let t231: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f3a8 = t231;
        if ((l.f29a != 0.0) && (l.f3a8 != 0.0)) {(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_136(
        l: &mut StampLocals,
    ) {
        let t232: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f3aa = t232;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3aa != 0.0)) {let t233: f64 = (l.f796 * l.f76b);let t234: f64 = (1.0 - t233);let t235: f64 = (t234).sqrt();l.f6fc = t235;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3aa == 0.0)) {let t236: f64 = (l.f796 * l.f76b);let t237: f64 = (1.0 - t236);let t238: f64 = (t237).powf(l.f60d);l.f6fc = t238;}
        if ((l.f29a != 0.0) && (l.f3a8 == 0.0)) {let t239: f64 = (1.0 - l.f6fc);let t23a: f64 = (l.f6a0 * t239);let t23b: f64 = (l.f73f - l.f796);let t23c: f64 = (l.f69a * t23b);let t23d: f64 = (t23a + t23c);(l.f690, l.f691, l.f692, ) = (t23d, 0.0, 0.0, );let t23e: f64 = (l.f544 * l.f53a);(l.f52f, l.f530, l.f531, ) = (t23e, (l.f544 * l.f53b), (l.f544 * l.f53c), );}
        let t23f: f64 = if ((l.f3b == 0.0) && (l.f41 == 0.0)) { 1.0 } else { 0.0 };l.f3ac = t23f;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) {let t240: f64 = (l.f763 - l.f7a2);l.f758 = t240;let t241: f64 = (l.f714 / l.f758);let t242: f64 = (1.0 - t241);let t243: f64 = (t242).sqrt();let t244: f64 = (1.0 - t243);l.f7ef = t244;}
        let t245: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f3ae = t245;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) && (l.f3ae != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) && (l.f3ae == 0.0)) {let t246: f64 = (l.f7ef * l.f7ef);let t247: f64 = (l.f7ef).ln();let t248: f64 = (t246 * t247);let t249: f64 = (1.0 - l.f7ef);let t24a: f64 = (t248 / t249);let t24b: f64 = (t24a + l.f7ef);let t24c: f64 = (2.0 * l.f62f);let t24d: f64 = (1.0 - t24c);let t24e: f64 = (t24b * t24d);l.f66 = t24e;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) {let t24f: f64 = (l.f7ef + l.f66);l.f7e9 = t24f;}
        let t250: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f3b0 = t250;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) && (l.f3b0 != 0.0)) {let t251: f64 = (l.f758 * l.f777);let t252: f64 = (t251).sqrt();l.f6fc = t252;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) && (l.f3b0 == 0.0)) {let t253: f64 = (l.f758 * l.f777);let t254: f64 = (t253).powf(l.f62f);l.f6fc = t254;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3ac == 0.0)) {let t255: f64 = (l.f7d8 * l.f6fc);l.f7d1 = t255;let t256: f64 = (l.f825 - 1.0);let t257: f64 = (t256 * l.f7d1);let t258: f64 = (l.fcd * t257);l.f9 = t258;let t259: f64 = (l.f9 * l.f7e9);let t25a: f64 = (l.f3b * t259);l.f593 = t25a;}
        let t25b: f64 = if l.f41 == 0.0 { 1.0 } else { 0.0 };l.f3b2 = t25b;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) {let t25c: f64 = (l.f7d1 * l.f60d);let t25d: f64 = (t25c / l.f758);let t25e: f64 = (l.f20 * t25d);l.f19 = t25e;let t25f: f64 = (0.666666666666667 * l.f10);let t260: f64 = (t25f / l.f19);l.f71a = t260;let t261: f64 = (l.f71a * l.f71a);l.f72c = t261;let t262: f64 = (l.f72c * l.f72c);let t263: f64 = (l.f72c * l.f72c);let t264: f64 = (t263 + 1.0);let t265: f64 = (t262 / t264);let t266: f64 = (t265).sqrt();l.f726 = t266;let t267: f64 = (l.f726).abs();let t268: f64 = (t267).sqrt();l.f6c1 = t268;let t269: f64 = (l.f726 * l.f6c1);l.f732 = t269;}
        let t26a: f64 = (-l.f62f);let t26b: f64 = (t26a * l.f613);let t26c: f64 = (-1.0);let t26d: f64 = if t26b == t26c { 1.0 } else { 0.0 };l.f3b4 = t26d;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b4 != 0.0)) {let t26e: f64 = (l.f19 * l.f732);let t26f: f64 = (1.0 + t26e);let t270: f64 = (1.0 / t26f);l.f7e3 = t270;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_137(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b4 == 0.0)) {let t271: f64 = (l.f19 * l.f732);let t272: f64 = (1.0 + t271);let t273: f64 = (-l.f62f);let t274: f64 = (t273 * l.f613);let t275: f64 = (t272).powf(t274);l.f7e3 = t275;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) {let t276: f64 = (l.f7e9 * l.f7e3);let t277: f64 = (l.f7e9 + l.f7e3);let t278: f64 = (t276 / t277);l.f7f5 = t278;let t279: f64 = (l.f19 / l.f6c1);let t27a: f64 = (0.375 * t279);let t27b: f64 = (t27a).sqrt();l.f5a8 = t27b;let t27c: f64 = (l.f71a * l.f6c1);let t27d: f64 = (2.0 * t27c);let t27e: f64 = (t27d - l.f726);l.f5b4 = t27e;let t27f: f64 = (l.f10 * l.f71a);let t280: f64 = (t27f * l.f6c1);let t281: f64 = (l.f10 * l.f726);let t282: f64 = (t280 - t281);let t283: f64 = (l.f19 * l.f732);let t284: f64 = (0.5 * t283);let t285: f64 = (t282 + t284);l.f5d4 = t285;let t286: f64 = (l.f5b4 - 1.0);let t287: f64 = (t286 * l.f5a8);l.f7fb = t287;let t288: f64 = (l.f7fb * l.f7fb);l.f811 = t288;}
        let t289: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f3b6 = t289;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b6 != 0.0)) {let t28a: f64 = (l.f62b * l.f7fb);let t28b: f64 = (1.0 + t28a);let t28c: f64 = (1.0 / t28b);l.f6e2 = t28c;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b6 == 0.0)) {let t28d: f64 = (l.f62b * l.f7fb);let t28e: f64 = (1.0 - t28d);let t28f: f64 = (1.0 / t28e);l.f6e2 = t28f;}
        let t290: f64 = (-l.f811);let t291: f64 = (t290 + l.f5d4);let t292: f64 = (-230.25850929940458);let t293: f64 = if t291 > t292 { 1.0 } else { 0.0 };l.f3b8 = t293;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b8 != 0.0)) {let t294: f64 = (-l.f811);let t295: f64 = (t294 + l.f5d4);let t296: f64 = (t295).exp();l.f6fc = t296;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3b8 == 0.0)) {let t297: f64 = (-230.25850929940458);let t298: f64 = (-l.f811);let t299: f64 = (t298 + l.f5d4);let t29a: f64 = (t297 - t299);let t29b: f64 = (-230.25850929940458);let t29c: f64 = (-l.f811);let t29d: f64 = (t29c + l.f5d4);let t29e: f64 = (t29b - t29d);let t29f: f64 = (-230.25850929940458);let t2a0: f64 = (-l.f811);let t2a1: f64 = (t2a0 + l.f5d4);let t2a2: f64 = (t29f - t2a1);let t2a3: f64 = (t2a2 * 0.3333333333333333);let t2a4: f64 = (1.0 + t2a3);let t2a5: f64 = (t29e * t2a4);let t2a6: f64 = (0.5 * t2a5);let t2a7: f64 = (1.0 + t2a6);let t2a8: f64 = (t29a * t2a7);let t2a9: f64 = (1.0 + t2a8);let t2aa: f64 = (1e-100 / t2a9);l.f6fc = t2aa;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) {let t2ab: f64 = (0.29214664 * l.f6e2);let t2ac: f64 = (l.f6e2 * l.f6e2);let t2ad: f64 = (l.f16 * t2ac);let t2ae: f64 = (t2ab + t2ad);let t2af: f64 = (l.f6e2 * l.f6e2);let t2b0: f64 = (t2af * l.f6e2);let t2b1: f64 = (l.f2a * t2b0);let t2b2: f64 = (t2ae + t2b1);let t2b3: f64 = (t2b2 * l.f6fc);l.f6e = t2b3;}
        let t2b4: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f3ba = t2b4;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3ba != 0.0)) {l.f74 = l.f6e;}
        let t2b5: f64 = (-230.25850929940458);let t2b6: f64 = if l.f5d4 > t2b5 { 1.0 } else { 0.0 };l.f3bc = t2b6;
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3ba == 0.0)) && (l.f3bc != 0.0)) {let t2b7: f64 = (l.f5d4).exp();l.f6fc = t2b7;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_138(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3ba == 0.0)) && (l.f3bc == 0.0)) {let t2b8: f64 = (-230.25850929940458);let t2b9: f64 = (t2b8 - l.f5d4);let t2ba: f64 = (-230.25850929940458);let t2bb: f64 = (t2ba - l.f5d4);let t2bc: f64 = (-230.25850929940458);let t2bd: f64 = (t2bc - l.f5d4);let t2be: f64 = (t2bd * 0.3333333333333333);let t2bf: f64 = (1.0 + t2be);let t2c0: f64 = (t2bb * t2bf);let t2c1: f64 = (0.5 * t2c0);let t2c2: f64 = (1.0 + t2c1);let t2c3: f64 = (t2b9 * t2c2);let t2c4: f64 = (1.0 + t2c3);let t2c5: f64 = (1e-100 / t2c4);l.f6fc = t2c5;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) && (l.f3ba == 0.0)) {let t2c6: f64 = (2.0 * l.f6fc);let t2c7: f64 = (t2c6 - l.f6e);l.f74 = t2c7;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3b2 == 0.0)) {let t2c8: f64 = (1.772453850905516 * 0.5);let t2c9: f64 = (l.f10 * l.f74);let t2ca: f64 = (t2c9 / l.f5a8);let t2cb: f64 = (t2c8 * t2ca);l.fd6 = t2cb;let t2cc: f64 = (l.f9 * l.fd6);let t2cd: f64 = (t2cc * l.f7f5);let t2ce: f64 = (l.f41 * t2cd);l.f599 = t2ce;}
        let t2cf: f64 = if l.f26 == 0.0 { 1.0 } else { 0.0 };l.f3be = t2cf;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be != 0.0)) {l.f529 = 0.0;}
        let t2d0: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f3c0 = t2d0;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c0 != 0.0)) {let t2d1: f64 = (l.f775 - l.f750);let t2d2: f64 = (t2d1 * l.f777);let t2d3: f64 = (t2d2).sqrt();l.f6fc = t2d3;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c0 == 0.0)) {let t2d4: f64 = (l.f775 - l.f750);let t2d5: f64 = (t2d4 * l.f777);let t2d6: f64 = (t2d5).powf(l.f62f);l.f6fc = t2d6;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) {let t2d7: f64 = (l.f775 - l.f750);let t2d8: f64 = (t2d7 * l.f7dc);let t2d9: f64 = (t2d8 / l.f6fc);let t2da: f64 = (l.f613 * t2d9);l.fb6 = t2da;}
        let t2db: f64 = (-l.fa3);let t2dc: f64 = (t2db / l.fb6);let t2dd: f64 = (t2dc).abs();let t2de: f64 = if t2dd < 230.25850929940458 { 1.0 } else { 0.0 };l.f3c2 = t2de;
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c2 != 0.0)) {let t2df: f64 = (-l.fa3);let t2e0: f64 = (t2df / l.fb6);let t2e1: f64 = (t2e0).exp();l.f6fc = t2e1;}
        let t2e2: f64 = (-l.fa3);let t2e3: f64 = (t2e2 / l.fb6);let t2e4: f64 = (-230.25850929940458);let t2e5: f64 = if t2e3 < t2e4 { 1.0 } else { 0.0 };l.f3c4 = t2e5;
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c2 == 0.0)) && (l.f3c4 != 0.0)) {let t2e6: f64 = (-230.25850929940458);let t2e7: f64 = (-l.fa3);let t2e8: f64 = (t2e7 / l.fb6);let t2e9: f64 = (t2e6 - t2e8);let t2ea: f64 = (-230.25850929940458);let t2eb: f64 = (-l.fa3);let t2ec: f64 = (t2eb / l.fb6);let t2ed: f64 = (t2ea - t2ec);let t2ee: f64 = (-230.25850929940458);let t2ef: f64 = (-l.fa3);let t2f0: f64 = (t2ef / l.fb6);let t2f1: f64 = (t2ee - t2f0);let t2f2: f64 = (t2f1 * 0.3333333333333333);let t2f3: f64 = (1.0 + t2f2);let t2f4: f64 = (t2ed * t2f3);let t2f5: f64 = (0.5 * t2f4);let t2f6: f64 = (1.0 + t2f5);let t2f7: f64 = (t2e9 * t2f6);let t2f8: f64 = (1.0 + t2f7);let t2f9: f64 = (1e-100 / t2f8);l.f6fc = t2f9;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_139(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) && (l.f3c2 == 0.0)) && (l.f3c4 == 0.0)) {let t2fa: f64 = (-l.fa3);let t2fb: f64 = (t2fa / l.fb6);let t2fc: f64 = (t2fb - 230.25850929940458);let t2fd: f64 = (-l.fa3);let t2fe: f64 = (t2fd / l.fb6);let t2ff: f64 = (t2fe - 230.25850929940458);let t300: f64 = (-l.fa3);let t301: f64 = (t300 / l.fb6);let t302: f64 = (t301 - 230.25850929940458);let t303: f64 = (t302 * 0.3333333333333333);let t304: f64 = (1.0 + t303);let t305: f64 = (t2ff * t304);let t306: f64 = (0.5 * t305);let t307: f64 = (1.0 + t306);let t308: f64 = (t2fc * t307);let t309: f64 = (1.0 + t308);let t30a: f64 = (1e100 * t309);l.f6fc = t30a;}
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3be == 0.0)) {let t30b: f64 = (l.f73f * l.fb6);let t30c: f64 = (t30b * l.fb6);let t30d: f64 = (t30c * l.f6fc);let t30e: f64 = (l.f26 * t30d);l.f529 = t30e;}
        let t30f: f64 = if ((l.f785 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f3c6 = t30f;
        if (((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 != 0.0)) {l.fae = 1.0;}
        let t310: f64 = (-l.f2);let t311: f64 = (t310 * l.f785);let t312: f64 = if l.f74a > t311 { 1.0 } else { 0.0 };l.f3c8 = t312;let t313: f64 = if l.f627 == 4.0 { 1.0 } else { 0.0 };l.f3ca = t313;
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 == 0.0)) && (l.f3c8 != 0.0)) && (l.f3ca != 0.0)) {let t314: f64 = (l.f74a * l.f789);let t315: f64 = (t314).abs();let t316: f64 = (l.f74a * l.f789);let t317: f64 = (t316).abs();let t318: f64 = (t315 * t317);let t319: f64 = (l.f74a * l.f789);let t31a: f64 = (t319).abs();let t31b: f64 = (t318 * t31a);let t31c: f64 = (l.f74a * l.f789);let t31d: f64 = (t31c).abs();let t31e: f64 = (t31b * t31d);l.f6fc = t31e;}
        if (((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 == 0.0)) && (l.f3c8 != 0.0)) && (l.f3ca == 0.0)) {let t31f: f64 = (l.f74a * l.f789);let t320: f64 = (t31f).abs();let t321: f64 = (t320).powf(l.f627);l.f6fc = t321;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 == 0.0)) && (l.f3c8 != 0.0)) {let t322: f64 = (1.0 - l.f6fc);let t323: f64 = (1.0 / t322);l.fae = t323;}
        if ((((l.f29a != 0.0) && (l.f3a8 == 0.0)) && (l.f3c6 == 0.0)) && (l.f3c8 == 0.0)) {let t324: f64 = (l.f2 * l.f785);let t325: f64 = (l.f74a + t324);let t326: f64 = (t325 * l.f6bc);let t327: f64 = (l.fc5 + t326);l.fae = t327;}
        if ((l.f29a != 0.0) && (l.f3a8 == 0.0)) {let t328: f64 = (l.f52f + l.f593);let t329: f64 = (t328 + l.f599);let t32a: f64 = (t329 + l.f529);let t32b: f64 = (t32a * l.fae);(l.f56e, l.f56f, l.f570, ) = (t32b, (l.f530 * l.fae), (l.f531 * l.fae), );let t32c: f64 = (l.f593 + l.f599);let t32d: f64 = (t32c + l.f529);let t32e: f64 = (t32d * l.fae);(l.f556, l.f557, l.f558, ) = (t32e, 0.0, 0.0, );}
        if (l.f29a != 0.0) {let t32f: f64 = (l.f0 * l.f562);let t330: f64 = (l.f5b1 * l.f576);let t331: f64 = (t32f + t330);let t332: f64 = (l.f5af * l.f56e);let t333: f64 = (t331 + t332);(l.f520, l.f525, l.f526, ) = (t333, (((l.f0 * l.f563) + (l.f5b1 * l.f577)) + (l.f5af * l.f56f)), (((l.f0 * l.f564) + (l.f5b1 * l.f578)) + (l.f5af * l.f570)), );l.f586 = l.f590;let t334: f64 = (l.f73d * l.f645);let t335: f64 = (t334 * l.f5c9);let t336: f64 = (t335).exp();let t337: f64 = (t336 - 1.0);let t338: f64 = (l.f586 * t337);let t339: f64 = (l.f518 - t338);(l.f519, l.f51a, l.f51b, ) = (t339, l.f51d, l.f51e, );let t33a: f64 = (l.f73f * l.f645);let t33b: f64 = (t33a * l.f5c9);let t33c: f64 = (t33b).exp();let t33d: f64 = (t33c - 1.0);let t33e: f64 = (l.f586 * t33d);let t33f: f64 = (l.f520 - t33e);(l.f521, l.f522, l.f523, ) = (t33f, l.f525, l.f526, );}
        let t340: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f3cc = t340;let t341: f64 = if ((l.f518 > 0.0) && (l.f520 > 0.0)) { 1.0 } else { 0.0 };l.f3ce = t341;let t342: f64 = (l.f519 / l.f518);let t343: f64 = (l.f521 / l.f520);let t344: f64 = if ((((t342 > 0.001) || (t343 > 0.001)) && (l.f519 > 0.0)) && (l.f521 > 0.0)) { 1.0 } else { 0.0 };l.f3d0 = t344;
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3ce != 0.0)) && (l.f3d0 != 0.0)) {let t345: f64 = (l.f519 / l.f521);(l.f4, l.f5, l.f6, ) = (t345, (((l.f51a * l.f521) - (l.f519 * l.f522)) / (l.f521 * l.f521)), (((l.f51b * l.f521) - (l.f519 * l.f523)) / (l.f521 * l.f521)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_140(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3ce != 0.0)) && (l.f3d0 != 0.0)) {let t346: f64 = (l.f4).ln();let t347: f64 = (l.f643 * t346);let t348: f64 = (l.f73d - l.f73f);let t349: f64 = (t347 / t348);(l.f5cb, l.f5cc, l.f5cd, ) = (t349, ((l.f643 * (l.f5 / l.f4)) / t348), ((l.f643 * (l.f6 / l.f4)) / t348), );let t34a: f64 = (l.f73d * l.f645);let t34b: f64 = (t34a * l.f5cb);let t34c: f64 = (t34b).exp();let t34d: f64 = (t34c - 1.0);let t34e: f64 = (l.f519 / t34d);(l.f588, l.f589, l.f58a, ) = (t34e, (((l.f51a * t34d) - (l.f519 * (t34c * (t34a * l.f5cc)))) / (t34d * t34d)), (((l.f51b * t34d) - (l.f519 * (t34c * (t34a * l.f5cd)))) / (t34d * t34d)), );}
        if ((l.f29a != 0.0) && (l.f3cc != 0.0)) {let t34f: f64 = (l.f737 * l.f645);let t350: f64 = (t34f * l.f5c9);let t351: f64 = (t350).exp();let t352: f64 = (t351 - 1.0);let t353: f64 = (l.f586 * t352);let t354: f64 = (l.f500 - t353);let t355: f64 = (l.f737 * l.f645);let t356: f64 = (t355 * l.f5cb);let t357: f64 = (t356).exp();let t358: f64 = (t357 - 1.0);let t359: f64 = (l.f588 * t358);let t35a: f64 = (t354 - t359);(l.f501, l.f502, l.f503, ) = (t35a, (l.f505 - ((l.f589 * t358) + (l.f588 * (t357 * (t355 * l.f5cc))))), (l.f506 - ((l.f58a * t358) + (l.f588 * (t357 * (t355 * l.f5cd))))), );let t35b: f64 = (l.f739 * l.f645);let t35c: f64 = (t35b * l.f5c9);let t35d: f64 = (t35c).exp();let t35e: f64 = (t35d - 1.0);let t35f: f64 = (l.f586 * t35e);let t360: f64 = (l.f508 - t35f);let t361: f64 = (l.f739 * l.f645);let t362: f64 = (t361 * l.f5cb);let t363: f64 = (t362).exp();let t364: f64 = (t363 - 1.0);let t365: f64 = (l.f588 * t364);let t366: f64 = (t360 - t365);(l.f509, l.f50a, l.f50b, ) = (t366, (l.f50d - ((l.f589 * t364) + (l.f588 * (t363 * (t361 * l.f5cc))))), (l.f50e - ((l.f58a * t364) + (l.f588 * (t363 * (t361 * l.f5cd))))), );let t367: f64 = (l.f73b * l.f645);let t368: f64 = (t367 * l.f5c9);let t369: f64 = (t368).exp();let t36a: f64 = (t369 - 1.0);let t36b: f64 = (l.f586 * t36a);let t36c: f64 = (l.f510 - t36b);let t36d: f64 = (l.f73b * l.f645);let t36e: f64 = (t36d * l.f5cb);let t36f: f64 = (t36e).exp();let t370: f64 = (t36f - 1.0);let t371: f64 = (l.f588 * t370);let t372: f64 = (t36c - t371);(l.f511, l.f512, l.f513, ) = (t372, (l.f515 - ((l.f589 * t370) + (l.f588 * (t36f * (t36d * l.f5cc))))), (l.f516 - ((l.f58a * t370) + (l.f588 * (t36f * (t36d * l.f5cd))))), );}
        let t373: f64 = if (((l.f500 < 0.0) && (l.f508 < 0.0)) && (l.f510 < 0.0)) { 1.0 } else { 0.0 };l.f3d2 = t373;let t374: f64 = (l.f501 / l.f500);let t375: f64 = (l.f509 / l.f508);let t376: f64 = (l.f511 / l.f510);let t377: f64 = if ((((((t374 > 0.001) || (t375 > 0.001)) || (t376 > 0.001)) && (l.f501 < 0.0)) && (l.f509 < 0.0)) && (l.f511 < 0.0)) { 1.0 } else { 0.0 };l.f3d4 = t377;
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t378: f64 = (l.f501 / l.f509);(l.f4, l.f5, l.f6, ) = (t378, (((l.f502 * l.f509) - (l.f501 * l.f50a)) / (l.f509 * l.f509)), (((l.f503 * l.f509) - (l.f501 * l.f50b)) / (l.f509 * l.f509)), );let t379: f64 = (-l.f643);let t37a: f64 = (l.f4).ln();let t37b: f64 = (t379 * t37a);let t37c: f64 = (l.f737 - l.f739);let t37d: f64 = (t37b / t37c);(l.f5b9, l.f5ba, l.f5bb, ) = (t37d, ((t379 * (l.f5 / l.f4)) / t37c), ((t379 * (l.f6 / l.f4)) / t37c), );let t37e: f64 = (l.f739 - l.f737);let t37f: f64 = (l.f739 / t37e);l.f707 = t37f;}
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t380: f64 = (l.f4 - 1.0);let t381: f64 = (l.f643 * t380);let t382: f64 = (l.f4).powf(l.f707);let t383: f64 = (t382 - 1.0);let t384: f64 = (t381 * t383);(l.f709, l.f70a, l.f70b, ) = (t384, (((l.f643 * l.f5) * t383) + (t381 * if 0.0 == 0.0 && ((l.f707) as f64).is_finite() && ((l.f707) as f64).fract() == 0.0 { if l.f707 == 0.0 { 0.0 } else { (l.f707 * ((l.f4).powf(l.f707 - 1.0) * l.f5)) } } else { (t382 * (l.f707 * (l.f5 / l.f4))) })), (((l.f643 * l.f6) * t383) + (t381 * if 0.0 == 0.0 && ((l.f707) as f64).is_finite() && ((l.f707) as f64).fract() == 0.0 { if l.f707 == 0.0 { 0.0 } else { (l.f707 * ((l.f4).powf(l.f707 - 1.0) * l.f6)) } } else { (t382 * (l.f707 * (l.f6 / l.f4))) })), );}
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t385: f64 = (l.f737 - l.f739);let t386: f64 = (l.f737 / t385);l.f707 = t386;}
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t387: f64 = (l.f4).powf(l.f707);let t388: f64 = (l.f739 - l.f737);let t389: f64 = (t387 * t388);let t38a: f64 = (l.f4 * l.f737);let t38b: f64 = (t389 + t38a);let t38c: f64 = (t38b - l.f739);(l.f70d, l.f70e, l.f70f, ) = (t38c, ((if 0.0 == 0.0 && ((l.f707) as f64).is_finite() && ((l.f707) as f64).fract() == 0.0 { if l.f707 == 0.0 { 0.0 } else { (l.f707 * ((l.f4).powf(l.f707 - 1.0) * l.f5)) } } else { (t387 * (l.f707 * (l.f5 / l.f4))) } * t388) + (l.f5 * l.f737)), ((if 0.0 == 0.0 && ((l.f707) as f64).is_finite() && ((l.f707) as f64).fract() == 0.0 { if l.f707 == 0.0 { 0.0 } else { (l.f707 * ((l.f4).powf(l.f707 - 1.0) * l.f6)) } } else { (t387 * (l.f707 * (l.f6 / l.f4))) } * t388) + (l.f6 * l.f737)), );}
        if ((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) {let t38d: f64 = (l.f709 / l.f70d);(l.f5bf, l.f5c0, l.f5c1, ) = (t38d, (((l.f70a * l.f70d) - (l.f709 * l.f70e)) / (l.f70d * l.f70d)), (((l.f70b * l.f70d) - (l.f709 * l.f70f)) / (l.f70d * l.f70d)), );let t38e: f64 = (l.f5b9 + l.f5bf);(l.f5cf, l.f5d0, l.f5d1, ) = (t38e, (l.f5ba + l.f5c0), (l.f5bb + l.f5c1), );}
        let t38f: f64 = (l.f73b * l.f645);let t390: f64 = (t38f * l.f5cf);let t391: f64 = (t390).abs();let t392: f64 = if t391 < 1e-6 { 1.0 } else { 0.0 };l.f3d6 = t392;
        if (((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) && (l.f3d6 != 0.0)) {l.f5bd = 1.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_141(
        ctx: &GeneratedEvalContext<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);
        if (((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) && (l.f3d6 != 0.0)) {let t393: f64 = (1.0 / l.f73b);let t394: f64 = (0.5 * l.f645);let t395: f64 = (t394 * l.f5cf);let t396: f64 = (t393 + t395);let t397: f64 = (l.f511 * t396);(l.f58c, l.f58d, l.f58e, ) = (t397, ((l.f512 * t396) + (l.f511 * (t394 * l.f5d0))), ((l.f513 * t396) + (l.f511 * (t394 * l.f5d1))), );let t398: f64 = (-0.5);let t399: f64 = (t398 * l.f511);let t39a: f64 = (t399 * l.f5cf);let t39b: f64 = (t39a * l.f645);let t39c: f64 = (t39b / l.f73b);(l.f5cf, l.f5d0, l.f5d1, ) = (t39c, (((((t398 * l.f512) * l.f5cf) + (t399 * l.f5d0)) * l.f645) / l.f73b), (((((t398 * l.f513) * l.f5cf) + (t399 * l.f5d1)) * l.f645) / l.f73b), );}
        if (((((l.f29a != 0.0) && (l.f3cc != 0.0)) && (l.f3d2 != 0.0)) && (l.f3d4 != 0.0)) && (l.f3d6 == 0.0)) {l.f5bd = 0.0;let t39d: f64 = (-l.f511);let t39e: f64 = (-l.f73b);let t39f: f64 = (t39e * l.f645);let t3a0: f64 = (t39f * l.f5cf);let t3a1: f64 = (t3a0).exp();let t3a2: f64 = (t3a1 - 1.0);let t3a3: f64 = (t39d / t3a2);(l.f58c, l.f58d, l.f58e, ) = (t3a3, ((((-l.f512) * t3a2) - (t39d * (t3a1 * (t39f * l.f5d0)))) / (t3a2 * t3a2)), ((((-l.f513) * t3a2) - (t39d * (t3a1 * (t39f * l.f5d1)))) / (t3a2 * t3a2)), );}
        if (l.f29a != 0.0) {let t3a4: f64 = (l.f0 * l.f2c);let t3a5: f64 = (l.f5b1 * l.f36);let t3a6: f64 = (t3a4 + t3a5);let t3a7: f64 = (l.f5af * l.f2e);let t3a8: f64 = (t3a6 + t3a7);let t3a9: f64 = (l.fb3 * t3a8);l.f822 = t3a9;}
        let t3aa: f64 = (l.f0 * l.f2c);let t3ab: f64 = if t3aa <= l.f822 { 1.0 } else { 0.0 };l.f3d8 = t3ab;
        if ((l.f29a != 0.0) && (l.f3d8 != 0.0)) {l.f81c = 0.0;}
        let t3ac: f64 = (l.f5b1 * l.f36);let t3ad: f64 = if t3ac <= l.f822 { 1.0 } else { 0.0 };l.f3da = t3ad;
        if ((l.f29a != 0.0) && (l.f3da != 0.0)) {l.f820 = 0.0;}
        let t3ae: f64 = (l.f5af * l.f2e);let t3af: f64 = if t3ae <= l.f822 { 1.0 } else { 0.0 };l.f3dc = t3af;
        if ((l.f29a != 0.0) && (l.f3dc != 0.0)) {l.f81e = 0.0;}
        let t3b0: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f3de = t3b0;
        if ((l.f29a != 0.0) && (l.f3de != 0.0)) {let t3b1: f64 = (0.5 * l.f57a);let t3b2: f64 = (l.f586 + 1e-21);let t3b3: f64 = (t3b1 / t3b2);let t3b4: f64 = (t3b3).ln();l.f800 = t3b4;let t3b5: f64 = (0.5 * l.f57a);let t3b6: f64 = (l.f588 + 1e-21);let t3b7: f64 = (t3b5 / t3b6);let t3b8: f64 = (t3b7).ln();(l.f802, l.f803, l.f804, ) = (t3b8, ((-((t3b5 * l.f589) / (t3b6 * t3b6))) / t3b7), ((-((t3b5 * l.f58a) / (t3b6 * t3b6))) / t3b7), );}
        if ((l.f29a != 0.0) && (l.f3de != 0.0)) {let t3b9: f64 = (0.5 * l.f57a);let t3ba: f64 = (l.f58c).abs();let t3bb: f64 = (t3ba + 1e-21);let t3bc: f64 = (t3b9 / t3bb);let t3bd: f64 = (t3bc).ln();(l.f806, l.f807, l.f808, ) = (t3bd, ((-((t3b9 * if l.f58c >= 0.0 { l.f58d } else { (-l.f58d) }) / (t3bb * t3bb))) / t3bc), ((-((t3b9 * if l.f58c >= 0.0 { l.f58e } else { (-l.f58e) }) / (t3bb * t3bb))) / t3bc), );}
        if (l.f29a != 0.0) {let t3be: f64 = (l.f800).min(230.25850929940458);l.f800 = t3be;let t3bf: f64 = (l.f800).exp();l.f97 = t3bf;}
        if (l.f29a != 0.0) {let t3c0: f64 = (l.f802).min(230.25850929940458);(l.f802, l.f803, l.f804, ) = (t3c0, if l.f802 <= 230.25850929940458 { l.f803 } else { 0.0 }, if l.f802 <= 230.25850929940458 { l.f804 } else { 0.0 }, );}
        if (l.f29a != 0.0) {let t3c1: f64 = (l.f802).exp();(l.f99, l.f9a, l.f9b, ) = (t3c1, (t3c1 * l.f803), (t3c1 * l.f804), );}
        if (l.f29a != 0.0) {let t3c2: f64 = (l.f806).min(230.25850929940458);(l.f806, l.f807, l.f808, ) = (t3c2, if l.f806 <= 230.25850929940458 { l.f807 } else { 0.0 }, if l.f806 <= 230.25850929940458 { l.f808 } else { 0.0 }, );}
        if (l.f29a != 0.0) {let t3c3: f64 = (l.f806).exp();(l.f9d, l.f9e, l.f9f, ) = (t3c3, (t3c3 * l.f807), (t3c3 * l.f808), );}
        (l.f745, l.f746, l.f747, ) = ((nv0 - nv2), 1.0, -1.0, );let t3c4: f64 = if l.f6d9 == 1.0 { 1.0 } else { 0.0 };l.f3e0 = t3c4;
        if (l.f3e0 != 0.0) {let t3c5: f64 = (l.f745 * l.f645);let t3c6: f64 = (t3c5 * l.f5c9);(l.f6eb, l.f6ec, l.f6ed, ) = (t3c6, ((l.f746 * l.f645) * l.f5c9), ((l.f747 * l.f645) * l.f5c9), );}
        if (l.f3e0 != 0.0) {
            let t3c7: f64 = (-230.25850929940458);
            let (t3d3, t3d4, t3d5,) = {
    if (l.f6eb < t3c7) {
        let t3c8: f64 = (-230.25850929940458);let t3c9: f64 = (t3c8 - l.f6eb);let t3ca: f64 = (t3c9 + 1.0);let t3cb: f64 = (1e-100 / t3ca);
        (t3cb, (-((1e-100 * (-l.f6ec)) / (t3ca * t3ca))), (-((1e-100 * (-l.f6ed)) / (t3ca * t3ca))),)
    } else {
        let (t3d0, t3d1, t3d2,) = {
            if (l.f6eb > l.f800) {
                let t3cc: f64 = (l.f6eb - l.f800);let t3cd: f64 = (t3cc + 1.0);let t3ce: f64 = (l.f97 * t3cd);
                (t3ce, (l.f97 * l.f6ec), (l.f97 * l.f6ed),)
            } else {
                let t3cf: f64 = (l.f6eb).exp();
                (t3cf, (t3cf * l.f6ec), (t3cf * l.f6ed),)
            }
        };
        (t3d0, t3d1, t3d2,)
    }
};
            (l.f6ef, l.f6f0, l.f6f1, ) = (t3d3, t3d4, t3d5, );
        }
        if (l.f3e0 != 0.0) {let t3d6: f64 = (l.f6ef - 1.0);let t3d7: f64 = (l.f586 * t3d6);(l.f566, l.f567, l.f568, ) = (t3d7, (l.f586 * l.f6f0), (l.f586 * l.f6f1), );let t3d8: f64 = (l.f745 * l.f645);let t3d9: f64 = (t3d8 * l.f5cb);(l.f6eb, l.f6ec, l.f6ed, ) = (t3d9, (((l.f746 * l.f645) * l.f5cb) + (t3d8 * l.f5cc)), (((l.f747 * l.f645) * l.f5cb) + (t3d8 * l.f5cd)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_142(
        l: &mut StampLocals,
    ) {
        if (l.f3e0 != 0.0) {
            let t3da: f64 = (-230.25850929940458);
            let (t3e6, t3e7, t3e8,) = {
    if (l.f6eb < t3da) {
        let t3db: f64 = (-230.25850929940458);let t3dc: f64 = (t3db - l.f6eb);let t3dd: f64 = (t3dc + 1.0);let t3de: f64 = (1e-100 / t3dd);
        (t3de, (-((1e-100 * (-l.f6ec)) / (t3dd * t3dd))), (-((1e-100 * (-l.f6ed)) / (t3dd * t3dd))),)
    } else {
        let (t3e3, t3e4, t3e5,) = {
            if (l.f6eb > l.f802) {
                let t3df: f64 = (l.f6eb - l.f802);let t3e0: f64 = (t3df + 1.0);let t3e1: f64 = (l.f99 * t3e0);
                (t3e1, ((l.f9a * t3e0) + (l.f99 * (l.f6ec - l.f803))), ((l.f9b * t3e0) + (l.f99 * (l.f6ed - l.f804))),)
            } else {
                let t3e2: f64 = (l.f6eb).exp();
                (t3e2, (t3e2 * l.f6ec), (t3e2 * l.f6ed),)
            }
        };
        (t3e3, t3e4, t3e5,)
    }
};
            (l.f6ef, l.f6f0, l.f6f1, ) = (t3e6, t3e7, t3e8, );
        }
        if (l.f3e0 != 0.0) {let t3e9: f64 = (l.f6ef - 1.0);let t3ea: f64 = (l.f588 * t3e9);(l.f56a, l.f56b, l.f56c, ) = (t3ea, ((l.f589 * t3e9) + (l.f588 * l.f6f0)), ((l.f58a * t3e9) + (l.f588 * l.f6f1)), );(l.f572, l.f573, l.f574, ) = (0.0, 0.0, 0.0, );}
        let t3eb: f64 = if l.f5bd > 0.0 { 1.0 } else { 0.0 };l.f3e2 = t3eb;
        if ((l.f3e0 != 0.0) && (l.f3e2 != 0.0)) {let t3ec: f64 = (l.f745 * l.f5cf);let t3ed: f64 = (l.f58c + t3ec);let t3ee: f64 = (l.f745 * t3ed);(l.f572, l.f573, l.f574, ) = (t3ee, ((l.f746 * t3ed) + (l.f745 * (l.f58d + ((l.f746 * l.f5cf) + (l.f745 * l.f5d0))))), ((l.f747 * t3ed) + (l.f745 * (l.f58e + ((l.f747 * l.f5cf) + (l.f745 * l.f5d1))))), );}
        if ((l.f3e0 != 0.0) && (l.f3e2 == 0.0)) {let t3ef: f64 = (-l.f745);let t3f0: f64 = (t3ef * l.f645);let t3f1: f64 = (t3f0 * l.f5cf);(l.f6eb, l.f6ec, l.f6ed, ) = (t3f1, ((((-l.f746) * l.f645) * l.f5cf) + (t3f0 * l.f5d0)), ((((-l.f747) * l.f645) * l.f5cf) + (t3f0 * l.f5d1)), );}
        if ((l.f3e0 != 0.0) && (l.f3e2 == 0.0)) {
            let t3f2: f64 = (-230.25850929940458);
            let (t3fe, t3ff, t400,) = {
    if (l.f6eb < t3f2) {
        let t3f3: f64 = (-230.25850929940458);let t3f4: f64 = (t3f3 - l.f6eb);let t3f5: f64 = (t3f4 + 1.0);let t3f6: f64 = (1e-100 / t3f5);
        (t3f6, (-((1e-100 * (-l.f6ec)) / (t3f5 * t3f5))), (-((1e-100 * (-l.f6ed)) / (t3f5 * t3f5))),)
    } else {
        let (t3fb, t3fc, t3fd,) = {
            if (l.f6eb > l.f806) {
                let t3f7: f64 = (l.f6eb - l.f806);let t3f8: f64 = (t3f7 + 1.0);let t3f9: f64 = (l.f9d * t3f8);
                (t3f9, ((l.f9e * t3f8) + (l.f9d * (l.f6ec - l.f807))), ((l.f9f * t3f8) + (l.f9d * (l.f6ed - l.f808))),)
            } else {
                let t3fa: f64 = (l.f6eb).exp();
                (t3fa, (t3fa * l.f6ec), (t3fa * l.f6ed),)
            }
        };
        (t3fb, t3fc, t3fd,)
    }
};
            (l.f6ef, l.f6f0, l.f6f1, ) = (t3fe, t3ff, t400, );
        }
        if ((l.f3e0 != 0.0) && (l.f3e2 == 0.0)) {let t401: f64 = (-l.f58c);let t402: f64 = (l.f6ef - 1.0);let t403: f64 = (t401 * t402);(l.f572, l.f573, l.f574, ) = (t403, (((-l.f58d) * t402) + (t401 * l.f6f0)), (((-l.f58e) * t402) + (t401 * l.f6f1)), );}
        if (l.f3e0 != 0.0) {let t404: f64 = (l.f566 + l.f56a);let t405: f64 = (t404 + l.f572);(l.f55e, l.f55f, l.f560, ) = (t405, ((l.f567 + l.f56b) + l.f573), ((l.f568 + l.f56c) + l.f574), );let t406: f64 = (l.f56a + l.f572);(l.f54e, l.f54f, l.f550, ) = (t406, (l.f56b + l.f573), (l.f56c + l.f574), );(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );let t407: f64 = (4.0 * l.f78f);let t408: f64 = (t407 * l.f78f);l.f4e0 = t408;let t409: f64 = (l.f78f / l.f791);l.f4e4 = t409;let t40a: f64 = (l.f78f * l.f4e4);let t40b: f64 = (l.f745 + t40a);(l.f4e8, l.f4eb, l.f4ec, ) = (t40b, l.f746, l.f747, );let t40c: f64 = (l.f791 + l.f4e8);(l.f4ee, l.f4f1, l.f4f2, ) = (t40c, l.f4eb, l.f4ec, );let t40d: f64 = (l.f791 - l.f4e8);(l.f4f4, l.f4f7, l.f4f8, ) = (t40d, (-l.f4eb), (-l.f4ec), );let t40e: f64 = (l.f4f4 * l.f4f4);let t40f: f64 = (t40e + l.f4e0);let t410: f64 = (t40f).sqrt();(l.f4fa, l.f4fd, l.f4fe, ) = (t410, (((l.f4f7 * l.f4f4) + (l.f4f4 * l.f4f7)) / (2.0 * t410)), (((l.f4f8 * l.f4f4) + (l.f4f4 * l.f4f8)) / (2.0 * t410)), );let t411: f64 = (l.f745 * l.f791);let t412: f64 = (l.f4ee + l.f4fa);let t413: f64 = (t411 / t412);let t414: f64 = (2.0 * t413);(l.f7ad, l.f7ae, l.f7af, ) = (t414, (2.0 * ((((l.f746 * l.f791) * t412) - (t411 * (l.f4f1 + l.f4fd))) / (t412 * t412))), (2.0 * ((((l.f747 * l.f791) * t412) - (t411 * (l.f4f2 + l.f4fe))) / (t412 * t412))), );}
        let t415: f64 = if l.f81c > 0.5 { 1.0 } else { 0.0 };l.f3e4 = t415;let t416: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f3e6 = t416;
        if (((l.f3e0 != 0.0) && (l.f3e4 != 0.0)) && (l.f3e6 != 0.0)) {let t417: f64 = (l.f7ad * l.f769);let t418: f64 = (1.0 - t417);let t419: f64 = (t418).sqrt();(l.f701, l.f702, l.f703, ) = (t419, ((-(l.f7ae * l.f769)) / (2.0 * t419)), ((-(l.f7af * l.f769)) / (2.0 * t419)), );}
        if (((l.f3e0 != 0.0) && (l.f3e4 != 0.0)) && (l.f3e6 == 0.0)) {let t41a: f64 = (l.f7ad * l.f769);let t41b: f64 = (1.0 - t41a);let t41c: f64 = (t41b).powf(l.f60b);(l.f701, l.f702, l.f703, ) = (t41c, if 0.0 == 0.0 && ((l.f60b) as f64).is_finite() && ((l.f60b) as f64).fract() == 0.0 { if l.f60b == 0.0 { 0.0 } else { (l.f60b * ((t41b).powf(l.f60b - 1.0) * (-(l.f7ae * l.f769)))) } } else { (t41c * (l.f60b * ((-(l.f7ae * l.f769)) / t41b))) }, if 0.0 == 0.0 && ((l.f60b) as f64).is_finite() && ((l.f60b) as f64).fract() == 0.0 { if l.f60b == 0.0 { 0.0 } else { (l.f60b * ((t41b).powf(l.f60b - 1.0) * (-(l.f7af * l.f769)))) } } else { (t41c * (l.f60b * ((-(l.f7af * l.f769)) / t41b))) }, );}
        if ((l.f3e0 != 0.0) && (l.f3e4 != 0.0)) {let t41d: f64 = (1.0 - l.f701);let t41e: f64 = (l.f69e * t41d);let t41f: f64 = (l.f745 - l.f7ad);let t420: f64 = (l.f698 * t41f);let t421: f64 = (t41e + t420);(l.f68c, l.f68d, l.f68e, ) = (t421, ((l.f69e * (-l.f702)) + (l.f698 * (l.f746 - l.f7ae))), ((l.f69e * (-l.f703)) + (l.f698 * (l.f747 - l.f7af))), );}
        if ((l.f3e0 != 0.0) && (l.f3e4 == 0.0)) {(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );}
        let t422: f64 = if l.f820 > 0.5 { 1.0 } else { 0.0 };l.f3e8 = t422;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_143(
        l: &mut StampLocals,
    ) {
        let t423: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f3ea = t423;
        if (((l.f3e0 != 0.0) && (l.f3e8 != 0.0)) && (l.f3ea != 0.0)) {let t424: f64 = (l.f7ad * l.f76d);let t425: f64 = (1.0 - t424);let t426: f64 = (t425).sqrt();(l.f701, l.f702, l.f703, ) = (t426, ((-(l.f7ae * l.f76d)) / (2.0 * t426)), ((-(l.f7af * l.f76d)) / (2.0 * t426)), );}
        if (((l.f3e0 != 0.0) && (l.f3e8 != 0.0)) && (l.f3ea == 0.0)) {let t427: f64 = (l.f7ad * l.f76d);let t428: f64 = (1.0 - t427);let t429: f64 = (t428).powf(l.f60f);(l.f701, l.f702, l.f703, ) = (t429, if 0.0 == 0.0 && ((l.f60f) as f64).is_finite() && ((l.f60f) as f64).fract() == 0.0 { if l.f60f == 0.0 { 0.0 } else { (l.f60f * ((t428).powf(l.f60f - 1.0) * (-(l.f7ae * l.f76d)))) } } else { (t429 * (l.f60f * ((-(l.f7ae * l.f76d)) / t428))) }, if 0.0 == 0.0 && ((l.f60f) as f64).is_finite() && ((l.f60f) as f64).fract() == 0.0 { if l.f60f == 0.0 { 0.0 } else { (l.f60f * ((t428).powf(l.f60f - 1.0) * (-(l.f7af * l.f76d)))) } } else { (t429 * (l.f60f * ((-(l.f7af * l.f76d)) / t428))) }, );}
        if ((l.f3e0 != 0.0) && (l.f3e8 != 0.0)) {let t42a: f64 = (1.0 - l.f701);let t42b: f64 = (l.f6a2 * t42a);let t42c: f64 = (l.f745 - l.f7ad);let t42d: f64 = (l.f69c * t42c);let t42e: f64 = (t42b + t42d);(l.f694, l.f695, l.f696, ) = (t42e, ((l.f6a2 * (-l.f702)) + (l.f69c * (l.f746 - l.f7ae))), ((l.f6a2 * (-l.f703)) + (l.f69c * (l.f747 - l.f7af))), );}
        if ((l.f3e0 != 0.0) && (l.f3e8 == 0.0)) {(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );}
        let t42f: f64 = if l.f81e > 0.5 { 1.0 } else { 0.0 };l.f3ec = t42f;let t430: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f3ee = t430;
        if (((l.f3e0 != 0.0) && (l.f3ec != 0.0)) && (l.f3ee != 0.0)) {let t431: f64 = (l.f7ad * l.f76b);let t432: f64 = (1.0 - t431);let t433: f64 = (t432).sqrt();(l.f701, l.f702, l.f703, ) = (t433, ((-(l.f7ae * l.f76b)) / (2.0 * t433)), ((-(l.f7af * l.f76b)) / (2.0 * t433)), );}
        if (((l.f3e0 != 0.0) && (l.f3ec != 0.0)) && (l.f3ee == 0.0)) {let t434: f64 = (l.f7ad * l.f76b);let t435: f64 = (1.0 - t434);let t436: f64 = (t435).powf(l.f60d);(l.f701, l.f702, l.f703, ) = (t436, if 0.0 == 0.0 && ((l.f60d) as f64).is_finite() && ((l.f60d) as f64).fract() == 0.0 { if l.f60d == 0.0 { 0.0 } else { (l.f60d * ((t435).powf(l.f60d - 1.0) * (-(l.f7ae * l.f76b)))) } } else { (t436 * (l.f60d * ((-(l.f7ae * l.f76b)) / t435))) }, if 0.0 == 0.0 && ((l.f60d) as f64).is_finite() && ((l.f60d) as f64).fract() == 0.0 { if l.f60d == 0.0 { 0.0 } else { (l.f60d * ((t435).powf(l.f60d - 1.0) * (-(l.f7af * l.f76b)))) } } else { (t436 * (l.f60d * ((-(l.f7af * l.f76b)) / t435))) }, );}
        if ((l.f3e0 != 0.0) && (l.f3ec != 0.0)) {let t437: f64 = (1.0 - l.f701);let t438: f64 = (l.f6a0 * t437);let t439: f64 = (l.f745 - l.f7ad);let t43a: f64 = (l.f69a * t439);let t43b: f64 = (t438 + t43a);(l.f690, l.f691, l.f692, ) = (t43b, ((l.f6a0 * (-l.f702)) + (l.f69a * (l.f746 - l.f7ae))), ((l.f6a0 * (-l.f703)) + (l.f69a * (l.f747 - l.f7af))), );}
        if ((l.f3e0 != 0.0) && (l.f3ec == 0.0)) {(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );}
        let t43c: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f3f0 = t43c;
        if ((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) {let t43d: f64 = (4.0 * l.f78f);let t43e: f64 = (t43d * l.f78f);l.f4e0 = t43e;let t43f: f64 = (l.f78f / l.f791);l.f4e4 = t43f;let t440: f64 = (l.f78f * l.f4e4);let t441: f64 = (l.f745 + t440);(l.f4e8, l.f4eb, l.f4ec, ) = (t441, l.f746, l.f747, );let t442: f64 = (l.f791 + l.f4e8);(l.f4ee, l.f4f1, l.f4f2, ) = (t442, l.f4eb, l.f4ec, );let t443: f64 = (l.f791 - l.f4e8);(l.f4f4, l.f4f7, l.f4f8, ) = (t443, (-l.f4eb), (-l.f4ec), );let t444: f64 = (l.f4f4 * l.f4f4);let t445: f64 = (t444 + l.f4e0);let t446: f64 = (t445).sqrt();(l.f4fa, l.f4fd, l.f4fe, ) = (t446, (((l.f4f7 * l.f4f4) + (l.f4f4 * l.f4f7)) / (2.0 * t446)), (((l.f4f8 * l.f4f4) + (l.f4f4 * l.f4f8)) / (2.0 * t446)), );let t447: f64 = (l.f745 * l.f791);let t448: f64 = (l.f4ee + l.f4fa);let t449: f64 = (t447 / t448);let t44a: f64 = (2.0 * t449);(l.f795, l.f798, l.f799, ) = (t44a, (2.0 * ((((l.f746 * l.f791) * t448) - (t447 * (l.f4f1 + l.f4fd))) / (t448 * t448))), (2.0 * ((((l.f747 * l.f791) * t448) - (t447 * (l.f4f2 + l.f4fe))) / (t448 * t448))), );}
        let t44b: f64 = if l.f745 < l.f7b1 { 1.0 } else { 0.0 };l.f3f2 = t44b;let t44c: f64 = (l.f745 * l.f645);let t44d: f64 = (0.5 * t44c);let t44e: f64 = (t44d).abs();let t44f: f64 = if t44e < 230.25850929940458 { 1.0 } else { 0.0 };l.f3f4 = t44f;
        if ((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f4 != 0.0)) {let t450: f64 = (l.f745 * l.f645);let t451: f64 = (0.5 * t450);let t452: f64 = (t451).exp();(l.f824, l.f827, l.f828, ) = (t452, (t452 * (0.5 * (l.f746 * l.f645))), (t452 * (0.5 * (l.f747 * l.f645))), );}
        let t453: f64 = (l.f745 * l.f645);let t454: f64 = (0.5 * t453);let t455: f64 = (-230.25850929940458);let t456: f64 = if t454 < t455 { 1.0 } else { 0.0 };l.f3f6 = t456;
        if (((((l.f3e0 == 0.0) && (l.f3f0 != 0.0)) && (l.f3f2 != 0.0)) && (l.f3f4 == 0.0)) && (l.f3f6 != 0.0)) {let t457: f64 = (-230.25850929940458);let t458: f64 = (l.f745 * l.f645);let t459: f64 = (0.5 * t458);let t45a: f64 = (t457 - t459);let t45b: f64 = (-230.25850929940458);let t45c: f64 = (l.f745 * l.f645);let t45d: f64 = (0.5 * t45c);let t45e: f64 = (t45b - t45d);let t45f: f64 = (-230.25850929940458);let t460: f64 = (l.f745 * l.f645);let t461: f64 = (0.5 * t460);let t462: f64 = (t45f - t461);let t463: f64 = (t462 * 0.3333333333333333);let t464: f64 = (1.0 + t463);let t465: f64 = (t45e * t464);let t466: f64 = (0.5 * t465);let t467: f64 = (1.0 + t466);let t468: f64 = (t45a * t467);let t469: f64 = (1.0 + t468);let t46a: f64 = (1e-100 / t469);(l.f824, l.f827, l.f828, ) = (t46a, (-((1e-100 * (((-(0.5 * (l.f746 * l.f645))) * t467) + (t45a * (0.5 * (((-(0.5 * (l.f746 * l.f645))) * t464) + (t45e * ((-(0.5 * (l.f746 * l.f645))) * 0.3333333333333333))))))) / (t469 * t469))), (-((1e-100 * (((-(0.5 * (l.f747 * l.f645))) * t467) + (t45a * (0.5 * (((-(0.5 * (l.f747 * l.f645))) * t464) + (t45e * ((-(0.5 * (l.f747 * l.f645))) * 0.3333333333333333))))))) / (t469 * t469))), );}
    }
}
