#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f704 != 0.0) {l.f18b = l.f183;l.f466 = l.f462;l.f199 = l.f195;l.f16d = l.f169;}
        let t0: f64 = (8.8541878176e-12 * l.f3b3);l.f3b1 = t0;let t1: f64 = (l.f3b1 / l.f1600);l.f1c7 = t1;let t2: f64 = (l.f1600 * l.f1600);l.f1604 = t2;let t3: f64 = (l.f1c7 / 1.6021918e-19);l.f1b9 = t3;let t4: f64 = (l.f420 * l.fed3);l.fed7 = t4;
        let (t6,) = {
    if (l.fed7 > 1e20) {
        let (t5,) = {
            if (l.fed7 < 1e26) {
                (l.fed7,)
            } else {
                (1e26,)
            }
        };
        (t5,)
    } else {
        (1e20,)
    }
};
        l.fed7 = t6;l.f127a = 0.0;let t7: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };l.f71a = t7;
        if (l.f71a != 0.0) {let t8: f64 = (0.4 * 5.951993);let t9: f64 = (t8 * p.p51);let ta: f64 = (l.f1c7).powf(0.6666666666666666);let tb: f64 = (t9 * ta);l.f127a = tb;}
        let tc: f64 = (-1.0);let td: f64 = if l.f193 == tc { 1.0 } else { 0.0 };l.f730 = td;
        if ((l.f71a != 0.0) && (l.f730 != 0.0)) {let te: f64 = (7.448711 / 5.951993);let tf: f64 = (te * l.f127a);l.f127a = tf;}
        let t10: f64 = (1e-8 * l.f1c7);let t11: f64 = (t10 / l.f3b7);l.f37d = t11;let t12: f64 = (0.5 * l.f474);l.f3e3 = t12;l.f3e4 = 0.5;let t13: f64 = (-1.0);let t14: f64 = if l.f193 == t13 { 1.0 } else { 0.0 };l.f746 = t14;
        if (l.f746 != 0.0) {let t15: f64 = (0.3333333333333333 * l.f474);l.f3e3 = t15;l.f3e4 = 0.3333333333333333;}
        let t16: f64 = (-2.0);let t17: f64 = (t16 / l.fe2);let t18: f64 = (t17 + 1.0);let t19: f64 = (2.0_f64).powf(t18);let t1a: f64 = (t19 - 1.0);l.f151c = t1a;let t1b: f64 = (l.f151c - 1.0);let t1c: f64 = (l.f151c - 1.0);let t1d: f64 = (t1b * t1c);let t1e: f64 = (4.0 * l.f151c);
        let (t20,) = {
    if (t1e > 0.0001) {
        let t1f: f64 = (4.0 * l.f151c);
        (t1f,)
    } else {
        (0.0001,)
    }
};
        let t21: f64 = (t1d / t20);l.fa4 = t21;let t22: f64 = (-2.0);let t23: f64 = (t22 / l.fe6);let t24: f64 = (t23 + 1.0);let t25: f64 = (2.0_f64).powf(t24);let t26: f64 = (t25 - 1.0);l.f151c = t26;let t27: f64 = (l.f151c - 1.0);let t28: f64 = (l.f151c - 1.0);let t29: f64 = (t27 * t28);let t2a: f64 = (4.0 * l.f151c);
        let (t2c,) = {
    if (t2a > 0.0001) {
        let t2b: f64 = (4.0 * l.f151c);
        (t2b,)
    } else {
        (0.0001,)
    }
};
        let t2d: f64 = (t29 / t2c);l.fa6 = t2d;let t2e: f64 = (1.0 / l.f1862);l.fd75 = t2e;let t2f: f64 = (l.f3b1 / l.f1606);l.f1c3 = t2f;let t30: f64 = (l.f3b1 / l.f160a);l.f1c4 = t30;let t31: f64 = (2.0 * 1.6021918e-19);let t32: f64 = (t31 * l.fee0);let t33: f64 = (t32 * l.f3b7);let t34: f64 = (t33 * l.fd73);let t35: f64 = (t34).sqrt();let t36: f64 = (t35 / l.f1c3);l.f5c5 = t36;let t37: f64 = (2.0 * 1.6021918e-19);let t38: f64 = (t37 * l.fee4);let t39: f64 = (t38 * l.f3b7);let t3a: f64 = (t39 * l.fd73);let t3b: f64 = (t3a).sqrt();let t3c: f64 = (t3b / l.f1c4);l.f5c3 = t3c;let t3d: f64 = (l.f5c5 * l.f5c5);l.f5c1 = t3d;let t3e: f64 = (l.f5c3 * l.f5c3);l.f5bf = t3e;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        l: &mut StampLocals,
    ) {
        let t3f: f64 = (l.f187 * 0.005);let t40: f64 = (t3f * l.fd73);let t41: f64 = (t40).exp();let t42: f64 = (t41 - 1.0);let t43: f64 = (t42).ln();let t44: f64 = (t43 / l.f187);let t45: f64 = (0.005 * l.fd73);let t46: f64 = (t45).exp();let t47: f64 = (t46 - 1.0);let t48: f64 = (t47).ln();let t49: f64 = (t44 - t48);l.f375 = t49;let t4a: f64 = (0.5 * l.f5c5);let t4b: f64 = (t4a).ln();let t4c: f64 = (t4b + l.f375);l.f373 = t4c;let t4d: f64 = (0.5 * l.f5c3);let t4e: f64 = (t4d).ln();let t4f: f64 = (t4e + l.f375);l.f371 = t4f;let t50: f64 = (1.0 / l.f5c5);l.fd57 = t50;let t51: f64 = (3.1 * l.f5c5);let t52: f64 = (t51 + 8.5);l.f133d = t52;let t53: f64 = (l.f133d * l.f133d);l.f1340 = t53;let t54: f64 = (0.5 * l.f133d);l.f1337 = t54;let t55: f64 = if l.fd57 < 0.06 { 1.0 } else { 0.0 };l.f75c = t55;
        if (l.f75c != 0.0) {let t56: f64 = (64.0 * l.fd57);l.f1335 = t56;}
        let t57: f64 = if l.fd57 <= 0.45 { 1.0 } else { 0.0 };l.f772 = t57;
        if ((l.f75c == 0.0) && (l.f772 != 0.0)) {let t58: f64 = (22.0 * l.fd57);let t59: f64 = (t58 + 3.0);l.f1335 = t59;}
        let t5a: f64 = if l.fd57 <= 1.6 { 1.0 } else { 0.0 };l.f77e = t5a;
        if (((l.f75c == 0.0) && (l.f772 == 0.0)) && (l.f77e != 0.0)) {let t5b: f64 = (-7.2);let t5c: f64 = (t5b * l.fd57);let t5d: f64 = (t5c + 15.5);l.f1335 = t5d;}
        if (((l.f75c == 0.0) && (l.f772 == 0.0)) && (l.f77e == 0.0)) {l.f1335 = l.f5c5;}
        let t5e: f64 = (l.f5c1 * 0.5);let t5f: f64 = (l.f1337 + t5e);let t60: f64 = (l.f5c1 * 0.25);let t61: f64 = (l.f1337 + t60);let t62: f64 = (t61 + l.f1335);let t63: f64 = (t62).sqrt();let t64: f64 = (l.f5c5 * t63);let t65: f64 = (t5f - t64);l.f133a = t65;let t66: f64 = (1.0 / l.f5c3);l.fd57 = t66;let t67: f64 = (3.1 * l.f5c3);let t68: f64 = (t67 + 8.5);l.f133d = t68;let t69: f64 = (l.f133d * l.f133d);l.f133e = t69;let t6a: f64 = (0.5 * l.f133d);l.f1337 = t6a;let t6b: f64 = if l.fd57 < 0.06 { 1.0 } else { 0.0 };l.f780 = t6b;
        if (l.f780 != 0.0) {let t6c: f64 = (64.0 * l.fd57);l.f1333 = t6c;}
        let t6d: f64 = if l.fd57 <= 0.45 { 1.0 } else { 0.0 };l.f782 = t6d;
        if ((l.f780 == 0.0) && (l.f782 != 0.0)) {let t6e: f64 = (22.0 * l.fd57);let t6f: f64 = (t6e + 3.0);l.f1333 = t6f;}
        let t70: f64 = if l.fd57 <= 1.6 { 1.0 } else { 0.0 };l.f784 = t70;
        if (((l.f780 == 0.0) && (l.f782 == 0.0)) && (l.f784 != 0.0)) {let t71: f64 = (-7.2);let t72: f64 = (t71 * l.fd57);let t73: f64 = (t72 + 15.5);l.f1333 = t73;}
        if (((l.f780 == 0.0) && (l.f782 == 0.0)) && (l.f784 == 0.0)) {l.f1333 = l.f5c3;}
        let t74: f64 = (l.f5bf * 0.5);let t75: f64 = (l.f1337 + t74);let t76: f64 = (l.f5bf * 0.25);let t77: f64 = (l.f1337 + t76);let t78: f64 = (t77 + l.f1333);let t79: f64 = (t78).sqrt();let t7a: f64 = (l.f5c3 * t79);let t7b: f64 = (t75 - t7a);l.f1338 = t7b;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t7c: f64 = (l.f3a3 + l.f2da);let t7d: f64 = (2.0 * l.ff9e);let t7e: f64 = (-0.75);let t7f: f64 = (l.ff72).powf(t7e);let t80: f64 = (l.fed3 * t7f);let t81: f64 = (t80 * 4e-26);let t82: f64 = (t81).ln();let t83: f64 = (t7d * t82);let t84: f64 = (t7c + t83);l.ff6d = t84;
        if (!(l.ff6d > 0.05)) {l.ff6d = 0.05;}
        let t85: f64 = (2.0 * 1.6021918e-19);let t86: f64 = (t85 * l.fed3);let t87: f64 = (t86 * l.f3b7);let t88: f64 = (t87 * l.fd59);let t89: f64 = (t88).sqrt();let t8a: f64 = (t89 / l.f1c7);l.f4f2 = t8a;l.fdf6 = 0.0;l.fee8 = 0.0;let t8b: f64 = if l.fee9 > 0.0 { 1.0 } else { 0.0 };l.f786 = t8b;
        if (l.f786 != 0.0) {let t8c: f64 = (80000000.0 / l.f1604);l.fae = t8c;}
        if (l.f786 != 0.0) {
            let (t8d,) = {
    if (l.fee9 > l.fae) {
        (l.fee9,)
    } else {
        (l.fae,)
    }
};
            l.fee8 = t8d;
        }
        if (l.f786 != 0.0) {
            let (t8e,) = {
    if (5e24 > l.fee8) {
        (5e24,)
    } else {
        (l.fee8,)
    }
};
            l.fee8 = t8e;
        }
        if (l.f786 != 0.0) {let t8f: f64 = (2.0 * l.f1c7);let t90: f64 = (t8f * l.f1c7);let t91: f64 = (t90 * l.ff9e);let t92: f64 = (1.6021918e-19 * l.fee8);let t93: f64 = (t92 * l.f3b7);let t94: f64 = (t91 / t93);l.fdf6 = t94;}
        let t95: f64 = (100.0 * l.ff9e);let t96: f64 = (t95 * l.ff9e);l.f1252 = t96;let t97: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };l.f792 = t97;
        if (l.f792 != 0.0) {let t98: f64 = (l.ff9e * l.f4f2);let t99: f64 = (t98 * l.f4f2);let t9a: f64 = (t99 * l.ff6d);let t9b: f64 = (t9a).sqrt();l.f1099 = t9b;let t9c: f64 = (0.75 * l.f127a);let t9d: f64 = (l.f1099).powf(0.6666666666666666);let t9e: f64 = (t9c * t9d);l.f2e2 = t9e;let t9f: f64 = (l.ff6d + l.f2e2);l.ff6d = t9f;let ta0: f64 = (2.0 * 0.6666666666666666);let ta1: f64 = (ta0 * l.f2e2);let ta2: f64 = (ta1 / l.f1099);let ta3: f64 = (1.0 + ta2);let ta4: f64 = (l.f4f2 * ta3);l.f4f2 = ta4;}
        let ta5: f64 = (l.ff6d).sqrt();l.f149c = ta5;let ta6: f64 = (0.95 * l.ff6d);l.ffe2 = ta6;let ta7: f64 = (0.0025 * l.ff6d);let ta8: f64 = (ta7 * l.ff6d);l.f9f = ta8;l.f128 = l.f9f;let ta9: f64 = (l.f128).sqrt();let taa: f64 = (0.5 * ta9);l.ffdc = taa;let tab: f64 = (l.ffe2 - l.ffdc);let tac: f64 = tab;let tad: f64 = (l.ffe2 - l.ffdc);let tae: f64 = tad;let taf: f64 = (l.ffe2 - l.ffdc);let tb0: f64 = taf;let tb1: f64 = (tae * tb0);let tb2: f64 = (tb1 + l.f9f);let tb3: f64 = (tb2).sqrt();let tb4: f64 = (tac - tb3);let tb5: f64 = (0.5 * tb4);l.ffd8 = tb5;let tb6: f64 = (l.ff6d + l.f3a3);let tb7: f64 = (0.5 * tb6);l.f60 = tb7;let tb8: f64 = (l.f1866 + l.ff6d);let tb9: f64 = (tb8).sqrt();let tba: f64 = (tb9 - l.f149c);l.f1693 = tba;let tbb: f64 = (l.f1866 + l.f367);let tbc: f64 = (tbb + l.ff6d);let tbd: f64 = (tbc).sqrt();let tbe: f64 = (tbd - l.f149c);let tbf: f64 = (tbe - l.f1693);l.f1695 = tbf;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tc0: f64 = (l.f3a3 + l.f2da);let tc1: f64 = (tc0 + l.f29b);let tc2: f64 = (2.0 * l.ff9e);let tc3: f64 = (-0.75);let tc4: f64 = (l.ff72).powf(tc3);let tc5: f64 = (l.fed7 * tc4);let tc6: f64 = (tc5 * 4e-26);let tc7: f64 = (tc6).ln();let tc8: f64 = (tc2 * tc7);let tc9: f64 = (tc1 + tc8);l.ff6b = tc9;
        if (!(l.ff6b > 0.05)) {l.ff6b = 0.05;}
        let tca: f64 = (2.0 * 1.6021918e-19);let tcb: f64 = (tca * l.fed7);let tcc: f64 = (tcb * l.f3b7);let tcd: f64 = (tcc * l.fd59);let tce: f64 = (tcd).sqrt();let tcf: f64 = (tce / l.f1c7);l.f4f0 = tcf;let td0: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };l.f7a8 = td0;
        if (l.f7a8 != 0.0) {let td1: f64 = (l.ff9e * l.f4f0);let td2: f64 = (td1 * l.f4f0);let td3: f64 = (td2 * l.ff6b);let td4: f64 = (td3).sqrt();l.f1099 = td4;let td5: f64 = (0.75 * l.f127a);let td6: f64 = (l.f1099).powf(0.6666666666666666);let td7: f64 = (td5 * td6);l.f2e2 = td7;let td8: f64 = (l.ff6b + l.f2e2);l.ff6b = td8;let td9: f64 = (2.0 * 0.6666666666666666);let tda: f64 = (td9 * l.f2e2);let tdb: f64 = (tda / l.f1099);let tdc: f64 = (1.0 + tdb);let tdd: f64 = (l.f4f0 * tdc);l.f4f0 = tdd;}
        let tde: f64 = (0.95 * l.ff6b);l.ffe0 = tde;let tdf: f64 = (0.0025 * l.ff6b);let te0: f64 = (tdf * l.ff6b);l.f9d = te0;l.f126 = l.f9d;let te1: f64 = (l.f126).sqrt();let te2: f64 = (0.5 * te1);l.ffdc = te2;let te3: f64 = (l.ffe0 - l.ffdc);let te4: f64 = te3;let te5: f64 = (l.ffe0 - l.ffdc);let te6: f64 = te5;let te7: f64 = (l.ffe0 - l.ffdc);let te8: f64 = te7;let te9: f64 = (te6 * te8);let tea: f64 = (te9 + l.f9d);let teb: f64 = (tea).sqrt();let tec: f64 = (te4 - teb);let ted: f64 = (0.5 * tec);l.ffd6 = ted;let tee: f64 = (l.f1500 * l.f23f);let tef: f64 = (l.f14c2 * l.f23f);let tf0: f64 = (1.0 + tef);let tf1: f64 = (tee * tf0);let tf2: f64 = (l.f17a9 + tf1);let tf3: f64 = (tf2 + l.f29f);l.f17ad = tf3;let tf4: f64 = (l.f14de * l.fe2b);let tf5: f64 = (tf4).exp();l.f1550 = tf5;let tf6: f64 = (l.f1e1 * l.f1550);l.f1e5 = tf6;let tf7: f64 = (l.f1f5 / l.f12d9);l.f1f9 = tf7;let tf8: f64 = (l.f14ca * l.fe2b);let tf9: f64 = (tf8).exp();l.f154a = tf9;let tfa: f64 = (l.f101 * l.f154a);l.f105 = tfa;let tfb: f64 = (l.f436 * l.f105);let tfc: f64 = (tfb * l.f1c7);l.ffd = tfc;let tfd: f64 = (l.f14f8 * l.fe2b);let tfe: f64 = (tfd).exp();let tff: f64 = (l.f1562 * tfe);l.f1566 = tff;let t100: f64 = (l.f14ec * l.fe2b);let t101: f64 = (t100).exp();l.f1554 = t101;let t102: f64 = (l.feb9 * l.f1554);l.febd = t102;let t103: f64 = (l.f14f4 * l.fe2b);let t104: f64 = (t103).exp();let t105: f64 = (l.f155c * t104);l.f1560 = t105;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t106: f64 = (l.f14da * l.fe2b);let t107: f64 = (t106).exp();l.f154e = t107;let t108: f64 = (l.f1c9 * l.f154e);l.f1cd = t108;let t109: f64 = (l.f1508 * l.fe2b);let t10a: f64 = (t109).exp();l.f155a = t10a;let t10b: f64 = (l.f19ba * l.f155a);l.f19be = t10b;let t10c: f64 = (l.f14f0 * l.fe2b);let t10d: f64 = (t10c).exp();l.f1556 = t10d;let t10e: f64 = (l.f12c5 * l.f1556);l.f12c9 = t10e;let t10f: f64 = (2.0 * l.ffd);let t110: f64 = (t10f * l.f12c9);l.f1568 = t110;let t111: f64 = (l.f14fc * l.fe2b);let t112: f64 = (t111).exp();l.f1558 = t112;let t113: f64 = (l.f1587 * l.f1558);l.f158b = t113;let t114: f64 = (l.f158d * l.f1558);l.f1591 = t114;let t115: f64 = (-l.f14c6);let t116: f64 = (t115 * l.fe2b);let t117: f64 = (t116).exp();let t118: f64 = (l.f4 * t117);l.f8 = t118;let t119: f64 = (l.f4a5 * 4.0);let t11a: f64 = (t119 * 1.3806505e-23);let t11b: f64 = (t11a * l.f15c7);l.ff00 = t11b;let t11c: f64 = if ((p.p46 != 0.0) && (l.f107 > 0.0)) { 1.0 } else { 0.0 };l.f7bd = t11c;
        if (l.f7bd != 0.0) {let t11d: f64 = (l.f1504 * l.f23f);let t11e: f64 = (l.f17af + t11d);let t11f: f64 = (t11e + l.f2a1);l.f17b3 = t11f;let t120: f64 = (l.f14ce * l.fe2b);let t121: f64 = (t120).exp();l.f154c = t121;let t122: f64 = (l.f107 * l.f154c);l.f10b = t122;let t123: f64 = (l.f438 * l.f10b);let t124: f64 = (t123 * l.f1c7);l.fff = t124;let t125: f64 = (l.f1f1 * l.f12d9);let t126: f64 = (1.0 + t125);let t127: f64 = (l.ff9e * t126);l.ff9f = t127;let t128: f64 = (l.f3a3 + l.f2de);let t129: f64 = (2.0 * l.ff9f);let t12a: f64 = (-0.75);let t12b: f64 = (l.ff72).powf(t12a);let t12c: f64 = (l.fed9 * t12b);let t12d: f64 = (t12c * 4e-26);let t12e: f64 = (t12d).ln();let t12f: f64 = (t129 * t12e);let t130: f64 = (t128 + t12f);l.ff70 = t130;}
        if (l.f7bd != 0.0) {
            let (t131,) = {
    if (l.ff70 > 0.05) {
        (l.ff70,)
    } else {
        (0.05,)
    }
};
            l.ff70 = t131;
        }
        if (l.f7bd != 0.0) {let t132: f64 = (2.0 * 1.6021918e-19);let t133: f64 = (t132 * l.fed9);let t134: f64 = (t133 * l.f3b7);let t135: f64 = (t134 * l.fd59);let t136: f64 = (t135).sqrt();let t137: f64 = (t136 / l.f1c7);l.f569 = t137;let t138: f64 = (l.f569 * l.f569);l.f56a = t138;let t139: f64 = (l.f56a).ln();l.fe2d = t139;let t13a: f64 = (0.95 * l.ff70);l.ffe4 = t13a;let t13b: f64 = (0.0025 * l.ff70);let t13c: f64 = (t13b * l.ff70);l.fa2 = t13c;l.f12a = l.fa2;let t13d: f64 = (l.f12a).sqrt();let t13e: f64 = (0.5 * t13d);l.ffde = t13e;let t13f: f64 = (l.ffe4 - l.ffde);let t140: f64 = t13f;let t141: f64 = (l.ffe4 - l.ffde);let t142: f64 = t141;let t143: f64 = (l.ffe4 - l.ffde);let t144: f64 = t143;let t145: f64 = (t142 * t144);let t146: f64 = (t145 + l.fa2);let t147: f64 = (t146).sqrt();let t148: f64 = (t140 - t147);let t149: f64 = (0.5 * t148);l.ffda = t149;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        l: &mut StampLocals,
    ) {
        if (l.f7bd == 0.0) {l.f17b3 = 0.0;l.f154c = 1.0;l.f10b = 0.0;l.fff = 0.0;l.ff9f = l.ff9e;l.ff70 = 0.0;l.f569 = 1.0;l.f56a = 1.0;l.fe2d = 0.0;l.ffe4 = 0.0;l.fa2 = 0.0;l.f12a = 0.0;l.ffde = 0.0;l.ffda = 0.0;}
        let t14a: f64 = (1.0 / l.f18f);l.fd3d = t14a;let t14b: f64 = (4.0 * 0.3333333333333333);let t14c: f64 = (2.0 * 1.6021918e-19);let t14d: f64 = (t14c * 9.1093826e-31);let t14e: f64 = (t14d * l.f18f);let t14f: f64 = (t14e).sqrt();let t150: f64 = (t14b * t14f);let t151: f64 = (t150 / 1.05457168e-34);l.ff2 = t151;let t152: f64 = (l.ff2 * l.f1600);l.ff6 = t152;let t153: f64 = (l.ff2 * l.f1606);l.f122 = t153;let t154: f64 = (l.ff2 * l.f160a);l.f123 = t154;l.f523 = 0.0;let t155: f64 = if l.f513 < 0.0 { 1.0 } else { 0.0 };l.f7d1 = t155;
        if (l.f7d1 != 0.0) {let t156: f64 = (-0.495);let t157: f64 = (t156 * l.f507);let t158: f64 = (t157 / l.f513);l.f523 = t158;}
        l.f525 = 0.0;let t159: f64 = if l.f517 < 0.0 { 1.0 } else { 0.0 };l.f7e6 = t159;
        if (l.f7e6 != 0.0) {let t15a: f64 = (-0.495);let t15b: f64 = (t15a * l.f50b);let t15c: f64 = (t15b / l.f517);l.f525 = t15c;}
        let t15d: f64 = if l.f51b < 0.0 { 1.0 } else { 0.0 };l.f7fa = t15d;
        if (l.f7fa != 0.0) {let t15e: f64 = (-0.495);let t15f: f64 = (t15e * l.f50f);let t160: f64 = (t15f / l.f51b);l.f527 = t160;}
        let t161: f64 = (l.f12d7).powf(l.f14e8);l.f1552 = t161;let t162: f64 = (l.fcb6 * l.f1552);l.fcb6 = t162;let t163: f64 = (l.fcba * l.f1552);l.fcba = t163;let t164: f64 = (l.fcbe * l.f1552);l.fcbe = t164;let t165: f64 = (l.f27 * 4e-18);let t166: f64 = (l.f1606 * l.f1606);let t167: f64 = (t165 / t166);l.f30 = t167;let t168: f64 = (l.f2b * 4e-18);let t169: f64 = (l.f160a * l.f160a);let t16a: f64 = (t168 / t169);l.f2f = t16a;let t16b: f64 = (l.f14d2 * l.f241);let t16c: f64 = (1.0 + t16b);
        let (t16f,) = {
    if (t16c > 0.0) {
        let t16d: f64 = (l.f14d2 * l.f241);let t16e: f64 = (1.0 + t16d);
        (t16e,)
    } else {
        (0.0,)
    }
};
        l.ff2 = t16f;let t170: f64 = (l.f112 * l.ff2);l.f116 = t170;let t171: f64 = (l.f116 * l.f1606);let t172: f64 = (t171 * 500000000.0);l.f120 = t172;let t173: f64 = (l.f14d6 * l.f241);let t174: f64 = (1.0 + t173);
        let (t177,) = {
    if (t174 > 0.0) {
        let t175: f64 = (l.f14d6 * l.f241);let t176: f64 = (1.0 + t175);
        (t176,)
    } else {
        (0.0,)
    }
};
        l.ff2 = t177;let t178: f64 = (l.f118 * l.ff2);l.f11c = t178;let t179: f64 = (l.f11c * l.f160a);let t17a: f64 = (t179 * 500000000.0);l.f11e = t17a;l.f17f9 = 0.0;let t17b: f64 = if l.f46a > 1e-10 { 1.0 } else { 0.0 };l.f80d = t17b;
        if (l.f80d != 0.0) {let t17c: f64 = (0.75 / l.f46a);l.f17f9 = t17c;}
        let t17d: f64 = (l.fee * l.fee);l.f31 = t17d;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t17e: f64 = (9.1093826e-31 * 1000000000.0);let t17f: f64 = (t17e * l.f4a9);l.f41f = t17f;let t180: f64 = if l.f1297 > 0.0 { 1.0 } else { 0.0 };l.f823 = t180;
        if (l.f823 != 0.0) {let t181: f64 = (1.0 / l.f1297);l.f56d = t181;}
        if (l.f823 == 0.0) {l.f56d = 0.0;}
        let t182: f64 = if l.f12cf > 0.0 { 1.0 } else { 0.0 };l.f838 = t182;
        if (l.f838 != 0.0) {let t183: f64 = (1.0 / l.f12cf);l.f5e3 = t183;}
        if (l.f838 == 0.0) {l.f5e3 = 0.0;}
        let t184: f64 = if l.f1295 > 0.0 { 1.0 } else { 0.0 };l.f84a = t184;
        if (l.f84a != 0.0) {let t185: f64 = (1.0 / l.f1295);l.f535 = t185;}
        if (l.f84a == 0.0) {l.f535 = 0.0;}
        let t186: f64 = if l.f1293 > 0.0 { 1.0 } else { 0.0 };l.f85f = t186;
        if (l.f85f != 0.0) {let t187: f64 = (1.0 / l.f1293);l.f506 = t187;}
        if (l.f85f == 0.0) {l.f506 = 0.0;}
        let t188: f64 = if l.f12c3 > 0.0 { 1.0 } else { 0.0 };l.f871 = t188;
        if (l.f871 != 0.0) {let t189: f64 = (1.0 / l.f12c3);l.f56f = t189;}
        if (l.f871 == 0.0) {l.f56f = 0.0;}
        let t18a: f64 = if l.f12c1 > 0.0 { 1.0 } else { 0.0 };l.f885 = t18a;
        if (l.f885 != 0.0) {let t18b: f64 = (1.0 / l.f12c1);l.f56e = t18b;}
        if (l.f885 == 0.0) {l.f56e = 0.0;}
        let t18c: f64 = if l.f12db > 0.0 { 1.0 } else { 0.0 };l.f898 = t18c;
        if (l.f898 != 0.0) {let t18d: f64 = (1.0 / l.f12db);l.fbeb = t18d;}
        if (l.f898 == 0.0) {l.fbeb = 0.0;}
        let t18e: f64 = (l.f1a * l.fd8e);l.f18 = t18e;let t18f: f64 = (l.fe3b * l.fd8e);l.fe39 = t18f;let t190: f64 = (l.fe29 * l.fd8e);l.fe27 = t190;let t191: f64 = (l.f16 * l.fd8e);l.f14 = t191;let t192: f64 = (l.fe37 * l.fd8e);l.fe35 = t192;let t193: f64 = (l.fe25 * l.fd8e);l.fe23 = t193;l.fdcc = 0.0;let t194: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };l.f8ab = t194;
        if (l.f8ab != 0.0) {l.fdcc = 1.0;}
        l.fdce = l.f18e8;let t195: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };l.f8b5 = t195;
        if (l.f8b5 != 0.0) {
            let (t196,) = {
    if (l.fdca > 0.0) {
        (l.fdca,)
    } else {
        (0.0,)
    }
};
            l.fdce = t196;
        }
        let t197: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };l.f8b7 = t197;
        if (l.f8b7 != 0.0) {let t198: f64 = (l.fba * l.fd8e);l.f18 = t198;let t199: f64 = (l.f100b * l.fd8e);let t19a: f64 = (l.fdcc * l.fdce);let t19b: f64 = (t199 - t19a);l.fe39 = t19b;l.fe27 = l.fdce;let t19c: f64 = (l.f1c * l.fd8e);l.f14 = t19c;let t19d: f64 = (l.ff5c * l.fd8e);let t19e: f64 = (l.fdcc * l.fdce);let t19f: f64 = (t19d - t19e);l.fe35 = t19f;l.fe23 = l.fdce;}
        let t1a0: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };l.f8b9 = t1a0;
        if (l.f8b9 != 0.0) {
            let (t1a1,) = {
    if (l.f18 > 0.0) {
        (l.f18,)
    } else {
        (0.0,)
    }
};
            l.f1a = t1a1;
        }
        if (l.f8b9 != 0.0) {
            let (t1a2,) = {
    if (l.fe39 > 0.0) {
        (l.fe39,)
    } else {
        (0.0,)
    }
};
            l.fe3b = t1a2;
        }
        if (l.f8b9 != 0.0) {
            let (t1a3,) = {
    if (l.fe27 > 0.0) {
        (l.fe27,)
    } else {
        (0.0,)
    }
};
            l.fe29 = t1a3;
        }
        if (l.f8b9 != 0.0) {
            let (t1a4,) = {
    if (l.f14 > 0.0) {
        (l.f14,)
    } else {
        (0.0,)
    }
};
            l.f16 = t1a4;
        }
        if (l.f8b9 != 0.0) {
            let (t1a5,) = {
    if (l.fe35 > 0.0) {
        (l.fe35,)
    } else {
        (0.0,)
    }
};
            l.fe37 = t1a5;
        }
        if (l.f8b9 != 0.0) {
            let (t1a6,) = {
    if (l.fe23 > 0.0) {
        (l.fe23,)
    } else {
        (0.0,)
    }
};
            l.fe25 = t1a6;
        }
        if (l.f8b9 == 0.0) {l.f1a = 0.0;l.fe3b = 0.0;l.fe29 = 0.0;l.f16 = 0.0;l.fe37 = 0.0;l.fe25 = 0.0;}
        l.f1720 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        l.f171e = 0.0;l.f17b7 = 0.0;l.f17b5 = 0.0;l.f1768 = 0.0;l.f1766 = 0.0;l.f16f1 = 0.0;l.f16ef = 0.0;l.f182c = 0.0;l.f182a = 0.0;l.f407 = 0.0;l.f405 = 0.0;l.fd99 = 0.0;l.fd98 = 0.0;l.fe77 = 1.0;l.fe76 = 1.0;(l.fd9f, l.fda0, l.fda1, l.fda2, l.fda3, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fd9a, l.fd9b, l.fd9c, l.fd9d, l.fd9e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe7d, l.fe7e, l.fe7f, l.fe80, l.fe81, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );(l.fe78, l.fe79, l.fe7a, l.fe7b, l.fe7c, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );(l.fda9, l.fdaa, l.fdab, l.fdac, l.fdad, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fda4, l.fda5, l.fda6, l.fda7, l.fda8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fea7, l.fea8, l.fea9, l.feaa, l.feab, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );(l.fea2, l.fea3, l.fea4, l.fea5, l.fea6, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );l.fe52 = 0.0;l.fe51 = 0.0;l.f1a67 = 0.0;l.f1a66 = 0.0;l.f40a = 0.0;l.f409 = 0.0;(l.f1a6d, l.f1a6e, l.f1a6f, l.f1a70, l.f1a71, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1a68, l.f1a69, l.f1a6a, l.f1a6b, l.f1a6c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f410, l.f411, l.f412, l.f413, l.f414, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f40b, l.f40c, l.f40d, l.f40e, l.f40f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1a77, l.f1a78, l.f1a79, l.f1a7a, l.f1a7b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1a72, l.f1a73, l.f1a74, l.f1a75, l.f1a76, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f41a, l.f41b, l.f41c, l.f41d, l.f41e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f415, l.f416, l.f417, l.f418, l.f419, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f1bba = 1.0;l.f1bb8 = 1.0;l.f1bc2 = 1.0;l.f1bc0 = 1.0;l.f1bbe = 1.0;l.f1bbc = 1.0;(l.fe4c, l.fe4d, l.fe4e, l.fe4f, l.fe50, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe6b, l.fe6c, l.fe6d, l.fe6e, l.fe6f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fc1a, l.fc1b, l.fc1c, l.fc1d, l.fc1e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fc24, l.fc25, l.fc26, l.fc27, l.fc28, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fc2e, l.fc2f, l.fc30, l.fc31, l.fc32, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fc38, l.fc39, l.fc3a, l.fc3b, l.fc3c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fc42, l.fc43, l.fc44, l.fc45, l.fc46, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f1613 = 0.0;(l.f1614, l.f1615, l.f1616, l.f1617, l.f1618, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1619, l.f161a, l.f161b, l.f161c, l.f161d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f1bc4 = 0.0;(l.f75, l.f76, l.f77, l.f78, l.f79, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );let t1a7: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };l.f8bb = t1a7;let t1a8: f64 = (l.fc8b * l.f1a);let t1a9: f64 = if t1a8 > 0.0 { 1.0 } else { 0.0 };l.f8c8 = t1a9;
        if ((l.f8bb != 0.0) && (l.f8c8 != 0.0)) {let t1aa: f64 = (l.fc8b * l.f1a);let t1ab: f64 = (p.p815 / t1aa);let t1ac: f64 = (t1ab + 1.0);let t1ad: f64 = (t1ac).ln();let t1ae: f64 = (l.ffce * t1ad);l.f182e = t1ae;}
        if ((l.f8bb != 0.0) && (l.f8c8 == 0.0)) {l.f182e = 100000000.0;}
        let t1af: f64 = (l.fc99 * l.fe3b);let t1b0: f64 = if t1af > 0.0 { 1.0 } else { 0.0 };l.f8ca = t1b0;
        if ((l.f8bb != 0.0) && (l.f8ca != 0.0)) {let t1b1: f64 = (l.fc99 * l.fe3b);let t1b2: f64 = (p.p815 / t1b1);let t1b3: f64 = (t1b2 + 1.0);let t1b4: f64 = (t1b3).ln();let t1b5: f64 = (l.ffce * t1b4);l.f1832 = t1b5;}
        if ((l.f8bb != 0.0) && (l.f8ca == 0.0)) {l.f1832 = 100000000.0;}
        let t1b6: f64 = (l.fc8f * l.fe29);let t1b7: f64 = if t1b6 > 0.0 { 1.0 } else { 0.0 };l.f8cc = t1b7;
        if ((l.f8bb != 0.0) && (l.f8cc != 0.0)) {let t1b8: f64 = (l.fc8f * l.fe29);let t1b9: f64 = (p.p815 / t1b8);let t1ba: f64 = (t1b9 + 1.0);let t1bb: f64 = (t1ba).ln();let t1bc: f64 = (l.ffce * t1bb);l.f1830 = t1bc;}
        if ((l.f8bb != 0.0) && (l.f8cc == 0.0)) {l.f1830 = 100000000.0;}
        if (l.f8bb != 0.0) {let t1bd: f64 = (l.f182e).min(l.f1832);let t1be: f64 = (t1bd).min(l.f1830);l.f182c = t1be;}
        let t1bf: f64 = (l.f182c * l.ffd0);let t1c0: f64 = (t1bf).abs();let t1c1: f64 = if t1c0 < 230.25850929940458 { 1.0 } else { 0.0 };l.f8ce = t1c1;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f8bb != 0.0) && (l.f8ce != 0.0)) {let t1c2: f64 = (l.f182c * l.ffd0);let t1c3: f64 = (t1c2).exp();l.f407 = t1c3;}
        let t1c4: f64 = (l.f182c * l.ffd0);let t1c5: f64 = if t1c4 < 0.0 { 1.0 } else { 0.0 };l.f8d0 = t1c5;
        if (((l.f8bb != 0.0) && (l.f8ce == 0.0)) && (l.f8d0 != 0.0)) {let t1c6: f64 = (-230.25850929940458);let t1c7: f64 = (l.f182c * l.ffd0);let t1c8: f64 = (t1c6 - t1c7);let t1c9: f64 = (-230.25850929940458);let t1ca: f64 = (l.f182c * l.ffd0);let t1cb: f64 = (t1c9 - t1ca);let t1cc: f64 = (-230.25850929940458);let t1cd: f64 = (l.f182c * l.ffd0);let t1ce: f64 = (t1cc - t1cd);let t1cf: f64 = (t1ce * 0.3333333333333333);let t1d0: f64 = (1.0 + t1cf);let t1d1: f64 = (t1cb * t1d0);let t1d2: f64 = (0.5 * t1d1);let t1d3: f64 = (1.0 + t1d2);let t1d4: f64 = (t1c8 * t1d3);let t1d5: f64 = (1.0 + t1d4);let t1d6: f64 = (1e-100 / t1d5);l.f407 = t1d6;}
        if (((l.f8bb != 0.0) && (l.f8ce == 0.0)) && (l.f8d0 == 0.0)) {let t1d7: f64 = (l.f182c * l.ffd0);let t1d8: f64 = (t1d7 - 230.25850929940458);let t1d9: f64 = (l.f182c * l.ffd0);let t1da: f64 = (t1d9 - 230.25850929940458);let t1db: f64 = (l.f182c * l.ffd0);let t1dc: f64 = (t1db - 230.25850929940458);let t1dd: f64 = (t1dc * 0.3333333333333333);let t1de: f64 = (1.0 + t1dd);let t1df: f64 = (t1da * t1de);let t1e0: f64 = (0.5 * t1df);let t1e1: f64 = (1.0 + t1e0);let t1e2: f64 = (t1d8 * t1e1);let t1e3: f64 = (1.0 + t1e2);let t1e4: f64 = (1e100 * t1e3);l.f407 = t1e4;}
        if (l.f8bb != 0.0) {l.f16fb = l.f16fa;l.f1739 = l.f1738;l.f1703 = l.f1702;l.ff3b = p.p824;l.f102c = p.p825;l.ff60 = p.p826;l.f16fd = p.p821;l.f173b = p.p822;l.f1709 = p.p823;}
        let t1e5: f64 = if l.f1a == 0.0 { 1.0 } else { 0.0 };l.f8d2 = t1e5;
        if ((l.f8bb != 0.0) && (l.f8d2 != 0.0)) {let t1e6: f64 = (l.f1738 + l.f1702);l.f16fb = t1e6;let t1e7: f64 = (p.p825).min(p.p826);let t1e8: f64 = (0.9 * t1e7);l.ff3b = t1e8;let t1e9: f64 = (p.p822 + p.p823);l.f16fd = t1e9;}
        let t1ea: f64 = if l.fe3b == 0.0 { 1.0 } else { 0.0 };l.f8d4 = t1ea;
        if ((l.f8bb != 0.0) && (l.f8d4 != 0.0)) {let t1eb: f64 = (l.f16fa + l.f1702);l.f1739 = t1eb;let t1ec: f64 = (p.p824).min(p.p826);let t1ed: f64 = (0.9 * t1ec);l.f102c = t1ed;let t1ee: f64 = (p.p821 + p.p823);l.f173b = t1ee;}
        let t1ef: f64 = if l.fe29 == 0.0 { 1.0 } else { 0.0 };l.f8d6 = t1ef;
        if ((l.f8bb != 0.0) && (l.f8d6 != 0.0)) {let t1f0: f64 = (l.f16fa + l.f1738);l.f1703 = t1f0;let t1f1: f64 = (p.p824).min(p.p825);let t1f2: f64 = (0.9 * t1f1);l.ff60 = t1f2;let t1f3: f64 = (p.p821 + p.p822);l.f1709 = t1f3;}
        if (l.f8bb != 0.0) {let t1f4: f64 = (l.f16fb).min(l.f1739);let t1f5: f64 = (t1f4).min(l.f1703);l.f1720 = t1f5;let t1f6: f64 = (l.f1720 * 0.1);l.f1768 = t1f6;let t1f7: f64 = (l.ff3b).max(l.f102c);let t1f8: f64 = (t1f7).max(l.ff60);l.fff6 = t1f8;let t1f9: f64 = (-1.0);let t1fa: f64 = (t1f9 / l.fff6);let t1fb: f64 = (2.0_f64).powf(t1fa);let t1fc: f64 = (1.0 - t1fb);let t1fd: f64 = (l.f1720 * t1fc);l.f17b7 = t1fd;let t1fe: f64 = (l.f16fd).min(l.f173b);let t1ff: f64 = (t1fe).min(l.f1709);let t200: f64 = (t1ff - 0.05);l.f16f1 = t200;}
        let t201: f64 = (l.fc8c * l.f16);let t202: f64 = if t201 > 0.0 { 1.0 } else { 0.0 };l.f8d8 = t202;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f8bb != 0.0) && (l.f8d8 != 0.0)) {let t203: f64 = (l.fc8c * l.f16);let t204: f64 = (p.p815 / t203);let t205: f64 = (t204 + 1.0);let t206: f64 = (t205).ln();let t207: f64 = (l.ffce * t206);l.f182e = t207;}
        if ((l.f8bb != 0.0) && (l.f8d8 == 0.0)) {l.f182e = 100000000.0;}
        let t208: f64 = (l.fc9a * l.fe37);let t209: f64 = if t208 > 0.0 { 1.0 } else { 0.0 };l.f8da = t209;
        if ((l.f8bb != 0.0) && (l.f8da != 0.0)) {let t20a: f64 = (l.fc9a * l.fe37);let t20b: f64 = (p.p815 / t20a);let t20c: f64 = (t20b + 1.0);let t20d: f64 = (t20c).ln();let t20e: f64 = (l.ffce * t20d);l.f1832 = t20e;}
        if ((l.f8bb != 0.0) && (l.f8da == 0.0)) {l.f1832 = 100000000.0;}
        let t20f: f64 = (l.fc90 * l.fe25);let t210: f64 = if t20f > 0.0 { 1.0 } else { 0.0 };l.f8dc = t210;
        if ((l.f8bb != 0.0) && (l.f8dc != 0.0)) {let t211: f64 = (l.fc90 * l.fe25);let t212: f64 = (p.p815 / t211);let t213: f64 = (t212 + 1.0);let t214: f64 = (t213).ln();let t215: f64 = (l.ffce * t214);l.f1830 = t215;}
        if ((l.f8bb != 0.0) && (l.f8dc == 0.0)) {l.f1830 = 100000000.0;}
        if (l.f8bb != 0.0) {let t216: f64 = (l.f182e).min(l.f1832);let t217: f64 = (t216).min(l.f1830);l.f182a = t217;}
        let t218: f64 = (l.f182a * l.ffd0);let t219: f64 = (t218).abs();let t21a: f64 = if t219 < 230.25850929940458 { 1.0 } else { 0.0 };l.f8de = t21a;
        if ((l.f8bb != 0.0) && (l.f8de != 0.0)) {let t21b: f64 = (l.f182a * l.ffd0);let t21c: f64 = (t21b).exp();l.f405 = t21c;}
        let t21d: f64 = (l.f182a * l.ffd0);let t21e: f64 = if t21d < 0.0 { 1.0 } else { 0.0 };l.f8e0 = t21e;
        if (((l.f8bb != 0.0) && (l.f8de == 0.0)) && (l.f8e0 != 0.0)) {let t21f: f64 = (-230.25850929940458);let t220: f64 = (l.f182a * l.ffd0);let t221: f64 = (t21f - t220);let t222: f64 = (-230.25850929940458);let t223: f64 = (l.f182a * l.ffd0);let t224: f64 = (t222 - t223);let t225: f64 = (-230.25850929940458);let t226: f64 = (l.f182a * l.ffd0);let t227: f64 = (t225 - t226);let t228: f64 = (t227 * 0.3333333333333333);let t229: f64 = (1.0 + t228);let t22a: f64 = (t224 * t229);let t22b: f64 = (0.5 * t22a);let t22c: f64 = (1.0 + t22b);let t22d: f64 = (t221 * t22c);let t22e: f64 = (1.0 + t22d);let t22f: f64 = (1e-100 / t22e);l.f405 = t22f;}
        if (((l.f8bb != 0.0) && (l.f8de == 0.0)) && (l.f8e0 == 0.0)) {let t230: f64 = (l.f182a * l.ffd0);let t231: f64 = (t230 - 230.25850929940458);let t232: f64 = (l.f182a * l.ffd0);let t233: f64 = (t232 - 230.25850929940458);let t234: f64 = (l.f182a * l.ffd0);let t235: f64 = (t234 - 230.25850929940458);let t236: f64 = (t235 * 0.3333333333333333);let t237: f64 = (1.0 + t236);let t238: f64 = (t233 * t237);let t239: f64 = (0.5 * t238);let t23a: f64 = (1.0 + t239);let t23b: f64 = (t231 * t23a);let t23c: f64 = (1.0 + t23b);let t23d: f64 = (1e100 * t23c);l.f405 = t23d;}
        if (l.f8bb != 0.0) {l.f16fb = l.f16ff;l.f1739 = l.f173d;l.f1703 = l.f170b;l.ff3b = l.ff3d;l.f102c = l.f102e;l.ff60 = l.ff66;l.f16fd = l.f1722;l.f173b = l.f1732;l.f1709 = l.f172c;}
        let t23e: f64 = if l.f16 == 0.0 { 1.0 } else { 0.0 };l.f8e2 = t23e;
        if ((l.f8bb != 0.0) && (l.f8e2 != 0.0)) {let t23f: f64 = (l.f173d + l.f170b);l.f16fb = t23f;let t240: f64 = (l.f102e).min(l.ff66);let t241: f64 = (0.9 * t240);l.ff3b = t241;let t242: f64 = (l.f1732 + l.f172c);l.f16fd = t242;}
        let t243: f64 = if l.fe37 == 0.0 { 1.0 } else { 0.0 };l.f8e4 = t243;
        if ((l.f8bb != 0.0) && (l.f8e4 != 0.0)) {let t244: f64 = (l.f16ff + l.f170b);l.f1739 = t244;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f8bb != 0.0) && (l.f8e4 != 0.0)) {let t245: f64 = (l.ff3d).min(l.ff66);let t246: f64 = (0.9 * t245);l.f102c = t246;let t247: f64 = (l.f1722 + l.f172c);l.f173b = t247;}
        let t248: f64 = if l.fe25 == 0.0 { 1.0 } else { 0.0 };l.f8e6 = t248;
        if ((l.f8bb != 0.0) && (l.f8e6 != 0.0)) {let t249: f64 = (l.f16ff + l.f173d);l.f1703 = t249;let t24a: f64 = (l.ff3d).min(l.f102e);let t24b: f64 = (0.9 * t24a);l.ff60 = t24b;let t24c: f64 = (l.f1722 + l.f1732);l.f1709 = t24c;}
        if (l.f8bb != 0.0) {let t24d: f64 = (l.f16fb).min(l.f1739);let t24e: f64 = (t24d).min(l.f1703);l.f171e = t24e;let t24f: f64 = (l.f171e * 0.1);l.f1766 = t24f;let t250: f64 = (l.ff3b).max(l.f102c);let t251: f64 = (t250).max(l.ff60);l.fff6 = t251;let t252: f64 = (-1.0);let t253: f64 = (t252 / l.fff6);let t254: f64 = (2.0_f64).powf(t253);let t255: f64 = (1.0 - t254);let t256: f64 = (l.f171e * t255);l.f17b5 = t256;let t257: f64 = (l.f16fd).min(l.f173b);let t258: f64 = (t257).min(l.f1709);let t259: f64 = (t258 - 0.05);l.f16ef = t259;}
        let t25a: f64 = if l.f1510 == 1.0 { 1.0 } else { 0.0 };l.f8e8 = t25a;
        if ((l.f8bb != 0.0) && (l.f8e8 != 0.0)) {(l.f1b98, l.f1ba1, l.f1ba2, l.f1ba3, l.f1ba4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f153e, l.f1546, l.f1547, l.f1548, l.f1549, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f3b9, l.f3c1, l.f3c2, l.f3c3, l.f3c4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.fc84 = 0.0;l.f1ba5 = 0.0;l.f1bcc = 0.0;l.f161e = 0.0;l.f1801 = 0.0;l.f1808 = 0.0;l.f16e8 = 0.0;l.f16e1 = 0.0;(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.fc83 = 0.0;(l.fdae, l.fdb6, l.fdb7, l.fdb8, l.fdb9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f16f3 = 0.0;l.f190c = 0.0;l.f36b = 0.0;l.f1906 = 0.0;(l.f18c3, l.f18cc, l.f18cd, l.f18ce, l.f18cf, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fc8, l.fd0, l.fd1, l.fd2, l.fd3, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fdba, l.fdc2, l.fdc3, l.fdc4, l.fdc5, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f12c, l.f135, l.f136, l.f137, l.f138, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1625, l.f162e, l.f162f, l.f1630, l.f1631, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1678, l.f1681, l.f1682, l.f1683, l.f1684, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f166b, l.f1674, l.f1675, l.f1676, l.f1677, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f149e, l.f14a7, l.f14a8, l.f14a9, l.f14aa, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1685, l.f168e, l.f168f, l.f1690, l.f1691, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f18ee, l.f18f6, l.f18f7, l.f18f8, l.f18f9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1912, l.f191a, l.f191b, l.f191c, l.f191d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fdfc, l.fe05, l.fe06, l.fe07, l.fe08, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe3d, l.fe46, l.fe47, l.fe48, l.fe49, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.feac, l.feb5, l.feb6, l.feb7, l.feb8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f19d5, l.f19de, l.f19df, l.f19e0, l.f19e1, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f3c5, l.f3cd, l.f3ce, l.f3cf, l.f3d0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f4fa, l.f502, l.f503, l.f504, l.f505, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fc72, l.fc7a, l.fc7b, l.fc7c, l.fc7d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f498, l.f4a1, l.f4a2, l.f4a3, l.f4a4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f456, l.f45e, l.f45f, l.f460, l.f461, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f4b6 = 0.4;l.f4b7 = 0.65;l.f4b5 = 0.8;let t25b: f64 = (-l.f4b6);let t25c: f64 = (t25b * p.p921);l.f16ae = t25c;let t25d: f64 = (-l.f4b7);let t25e: f64 = (t25d * p.p921);l.f16af = t25e;let t25f: f64 = (-l.f4b5);let t260: f64 = (t25f * p.p921);l.f16b0 = t260;l.f16b1 = 0.1;l.f16b2 = 0.2;l.f16e8 = 0.0;l.f161e = 0.0;}
        let t261: f64 = if (!(((l.f1a == 0.0) && (l.fe3b == 0.0)) && (l.fe29 == 0.0))) { 1.0 } else { 0.0 };l.f8ec = t261;let t262: f64 = if l.f16ae < l.f182c { 1.0 } else { 0.0 };l.f8ed = t262;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        l: &mut StampLocals,
    ) {
        let t263: f64 = (-0.5);let t264: f64 = (l.f16ae * l.ffd0);let t265: f64 = (t263 * t264);let t266: f64 = (t265).abs();let t267: f64 = if t266 < 230.25850929940458 { 1.0 } else { 0.0 };l.f8ee = t267;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) && (l.f8ed != 0.0)) && (l.f8ee != 0.0)) {let t268: f64 = (-0.5);let t269: f64 = (l.f16ae * l.ffd0);let t26a: f64 = (t268 * t269);let t26b: f64 = (t26a).exp();l.f1ba5 = t26b;}
        let t26c: f64 = (-0.5);let t26d: f64 = (l.f16ae * l.ffd0);let t26e: f64 = (t26c * t26d);let t26f: f64 = if t26e < 0.0 { 1.0 } else { 0.0 };l.f8ef = t26f;
        if ((((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) && (l.f8ed != 0.0)) && (l.f8ee == 0.0)) && (l.f8ef != 0.0)) {let t270: f64 = (-230.25850929940458);let t271: f64 = (-0.5);let t272: f64 = (l.f16ae * l.ffd0);let t273: f64 = (t271 * t272);let t274: f64 = (t270 - t273);let t275: f64 = (-230.25850929940458);let t276: f64 = (-0.5);let t277: f64 = (l.f16ae * l.ffd0);let t278: f64 = (t276 * t277);let t279: f64 = (t275 - t278);let t27a: f64 = (-230.25850929940458);let t27b: f64 = (-0.5);let t27c: f64 = (l.f16ae * l.ffd0);let t27d: f64 = (t27b * t27c);let t27e: f64 = (t27a - t27d);let t27f: f64 = (t27e * 0.3333333333333333);let t280: f64 = (1.0 + t27f);let t281: f64 = (t279 * t280);let t282: f64 = (0.5 * t281);let t283: f64 = (1.0 + t282);let t284: f64 = (t274 * t283);let t285: f64 = (1.0 + t284);let t286: f64 = (1e-100 / t285);l.f1ba5 = t286;}
        if ((((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) && (l.f8ed != 0.0)) && (l.f8ee == 0.0)) && (l.f8ef == 0.0)) {let t287: f64 = (-0.5);let t288: f64 = (l.f16ae * l.ffd0);let t289: f64 = (t287 * t288);let t28a: f64 = (t289 - 230.25850929940458);let t28b: f64 = (-0.5);let t28c: f64 = (l.f16ae * l.ffd0);let t28d: f64 = (t28b * t28c);let t28e: f64 = (t28d - 230.25850929940458);let t28f: f64 = (-0.5);let t290: f64 = (l.f16ae * l.ffd0);let t291: f64 = (t28f * t290);let t292: f64 = (t291 - 230.25850929940458);let t293: f64 = (t292 * 0.3333333333333333);let t294: f64 = (1.0 + t293);let t295: f64 = (t28e * t294);let t296: f64 = (0.5 * t295);let t297: f64 = (1.0 + t296);let t298: f64 = (t28a * t297);let t299: f64 = (1.0 + t298);let t29a: f64 = (1e100 * t299);l.f1ba5 = t29a;}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) && (l.f8ed != 0.0)) {let t29b: f64 = (1.0 / l.f1ba5);l.f1bcc = t29b;let t29c: f64 = (l.f1bcc * l.f1bcc);l.fc84 = t29c;}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) && (l.f8ed == 0.0)) {let t29d: f64 = (l.f16ae - l.f182c);let t29e: f64 = (t29d * l.ffd0);let t29f: f64 = (1.0 + t29e);let t2a0: f64 = (t29f * l.f407);l.fc84 = t2a0;let t2a1: f64 = (l.fc84).sqrt();l.f1bcc = t2a1;let t2a2: f64 = (1.0 / l.f1bcc);l.f1ba5 = t2a2;}
        if (((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) {let t2a3: f64 = (l.fc84 - 1.0);l.fc84 = t2a3;}
        let t2a4: f64 = if l.f16ae > 0.0 { 1.0 } else { 0.0 };l.f8f0 = t2a4;
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) && (l.f8f0 != 0.0)) {let t2a5: f64 = (2.0 + l.f1ba5);let t2a6: f64 = (l.f1ba5 + 1.0);let t2a7: f64 = (l.f1ba5 + 3.0);let t2a8: f64 = (t2a6 * t2a7);let t2a9: f64 = (t2a8).sqrt();let t2aa: f64 = (t2a5 + t2a9);let t2ab: f64 = (t2aa).ln();let t2ac: f64 = (l.ffce * t2ab);let t2ad: f64 = (2.0 * t2ac);l.f161e = t2ad;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) && (l.f8f0 == 0.0)) {let t2ae: f64 = (-l.f16ae);let t2af: f64 = (2.0 * l.f1bcc);let t2b0: f64 = (t2af + 1.0);let t2b1: f64 = (1.0 + l.f1bcc);let t2b2: f64 = (3.0 * l.f1bcc);let t2b3: f64 = (1.0 + t2b2);let t2b4: f64 = (t2b1 * t2b3);let t2b5: f64 = (t2b4).sqrt();let t2b6: f64 = (t2b0 + t2b5);let t2b7: f64 = (t2b6).ln();let t2b8: f64 = (l.ffce * t2b7);let t2b9: f64 = (2.0 * t2b8);let t2ba: f64 = (t2ae + t2b9);l.f161e = t2ba;}
        if (((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8ec != 0.0)) {let t2bb: f64 = (l.f1720 - l.f161e);l.f1801 = t2bb;let t2bc: f64 = (l.f16ae + l.f1801);let t2bd: f64 = (l.f16ae - l.f1801);let t2be: f64 = (l.f16ae - l.f1801);let t2bf: f64 = (t2bd * t2be);let t2c0: f64 = (4.0 * l.ffce);let t2c1: f64 = (t2c0 * l.ffce);let t2c2: f64 = (t2bf + t2c1);let t2c3: f64 = (t2c2).sqrt();let t2c4: f64 = (t2bc - t2c3);let t2c5: f64 = (0.5 * t2c4);l.f1808 = t2c5;let t2c6: f64 = (l.f16ae + l.f16f1);let t2c7: f64 = (l.f16ae - l.f16f1);let t2c8: f64 = (l.f16ae - l.f16f1);let t2c9: f64 = (t2c7 * t2c8);let t2ca: f64 = (4.0 * l.ffd2);let t2cb: f64 = (t2ca * l.ffd2);let t2cc: f64 = (t2c9 + t2cb);let t2cd: f64 = (t2cc).sqrt();let t2ce: f64 = (t2c6 - t2cd);let t2cf: f64 = (0.5 * t2ce);l.f16e8 = t2cf;let t2d0: f64 = l.f16ae;let t2d1: f64 = l.f16ae;let t2d2: f64 = l.f16ae;let t2d3: f64 = (t2d1 * t2d2);let t2d4: f64 = (4.0 * 1e-6);let t2d5: f64 = (t2d4 * 1e-6);let t2d6: f64 = (t2d3 + t2d5);let t2d7: f64 = (t2d6).sqrt();let t2d8: f64 = (t2d0 - t2d7);let t2d9: f64 = (0.5 * t2d8);l.f16e1 = t2d9;}
        let t2da: f64 = if l.f1a == 0.0 { 1.0 } else { 0.0 };l.f8f1 = t2da;
        if (((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 != 0.0)) {(l.fce5, l.fced, l.fcee, l.fcef, l.fcf0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) {let t2db: f64 = (l.fc8b * l.fc84);l.fc83 = t2db;}
        let t2dc: f64 = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };l.f8f2 = t2dc;
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f2 != 0.0)) {(l.fdae, l.fdb6, l.fdb7, l.fdb8, l.fdb9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f2 == 0.0)) {let t2dd: f64 = (l.f16fa - l.f1808);l.f16f3 = t2dd;let t2de: f64 = (l.f161e / l.f16f3);let t2df: f64 = (1.0 - t2de);let t2e0: f64 = (t2df).sqrt();let t2e1: f64 = (1.0 - t2e0);l.f190c = t2e1;}
        let t2e2: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };l.f8f3 = t2e2;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f2 == 0.0)) && (l.f8f3 != 0.0)) {l.f36b = 0.0;}
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f2 == 0.0)) && (l.f8f3 == 0.0)) {let t2e3: f64 = (l.f190c * l.f190c);let t2e4: f64 = (l.f190c).ln();let t2e5: f64 = (t2e3 * t2e4);let t2e6: f64 = (1.0 - l.f190c);let t2e7: f64 = (t2e5 / t2e6);let t2e8: f64 = (t2e7 + l.f190c);let t2e9: f64 = (2.0 * p.p824);let t2ea: f64 = (1.0 - t2e9);let t2eb: f64 = (t2e8 * t2ea);l.f36b = t2eb;}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f2 == 0.0)) {let t2ec: f64 = (l.f190c + l.f36b);l.f1906 = t2ec;}
        let t2ed: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };l.f8f4 = t2ed;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f2 == 0.0)) && (l.f8f4 != 0.0)) {let t2ee: f64 = (l.f16f3 * l.f1724);let t2ef: f64 = (t2ee).sqrt();(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t2ef, 0.0, 0.0, 0.0, 0.0, );}
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f2 == 0.0)) && (l.f8f4 == 0.0)) {let t2f0: f64 = (l.f16f3 * l.f1724);let t2f1: f64 = (t2f0).powf(p.p824);(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t2f1, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f2 == 0.0)) {let t2f2: f64 = (l.f18d0 * l.f15e7);(l.f18c3, l.f18cc, l.f18cd, l.f18ce, l.f18cf, ) = (t2f2, (l.f18d0 * l.f15f0), (l.f18d0 * l.f15f1), (l.f18d0 * l.f15f2), (l.f18d0 * l.f15f3), );let t2f3: f64 = (l.f1bcc - 1.0);let t2f4: f64 = (t2f3 * l.f18c3);let t2f5: f64 = (l.f4db * t2f4);(l.fc8, l.fd0, l.fd1, l.fd2, l.fd3, ) = (t2f5, (l.f4db * (t2f3 * l.f18cc)), (l.f4db * (t2f3 * l.f18cd)), (l.f4db * (t2f3 * l.f18ce)), (l.f4db * (t2f3 * l.f18cf)), );let t2f6: f64 = (l.fc8 * l.f1906);let t2f7: f64 = (p.p833 * t2f6);(l.fdae, l.fdb6, l.fdb7, l.fdb8, l.fdb9, ) = (t2f7, (p.p833 * (l.fd0 * l.f1906)), (p.p833 * (l.fd1 * l.f1906)), (p.p833 * (l.fd2 * l.f1906)), (p.p833 * (l.fd3 * l.f1906)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t2f8: f64 = if p.p838 == 0.0 { 1.0 } else { 0.0 };l.f8f5 = t2f8;
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 != 0.0)) {(l.fdba, l.fdc2, l.fdc3, l.fdc4, l.fdc5, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) {let t2f9: f64 = (l.f18c3 * l.ff0f);let t2fa: f64 = (t2f9 / l.f16f3);let t2fb: f64 = (l.f139 * t2fa);(l.f12c, l.f135, l.f136, l.f137, l.f138, ) = (t2fb, (l.f139 * ((l.f18cc * l.ff0f) / l.f16f3)), (l.f139 * ((l.f18cd * l.ff0f) / l.f16f3)), (l.f139 * ((l.f18ce * l.ff0f) / l.f16f3)), (l.f139 * ((l.f18cf * l.ff0f) / l.f16f3)), );let t2fc: f64 = (0.666666666666667 * l.fd4);let t2fd: f64 = (t2fc / l.f12c);(l.f1625, l.f162e, l.f162f, l.f1630, l.f1631, ) = (t2fd, (-((t2fc * l.f135) / (l.f12c * l.f12c))), (-((t2fc * l.f136) / (l.f12c * l.f12c))), (-((t2fc * l.f137) / (l.f12c * l.f12c))), (-((t2fc * l.f138) / (l.f12c * l.f12c))), );let t2fe: f64 = (l.f1625 * l.f1625);(l.f1678, l.f1681, l.f1682, l.f1683, l.f1684, ) = (t2fe, ((l.f162e * l.f1625) + (l.f1625 * l.f162e)), ((l.f162f * l.f1625) + (l.f1625 * l.f162f)), ((l.f1630 * l.f1625) + (l.f1625 * l.f1630)), ((l.f1631 * l.f1625) + (l.f1625 * l.f1631)), );let t2ff: f64 = (l.f1678 * l.f1678);let t300: f64 = (l.f1678 * l.f1678);let t301: f64 = (t300 + 1.0);let t302: f64 = (t2ff / t301);let t303: f64 = (t302).sqrt();(l.f166b, l.f1674, l.f1675, l.f1676, l.f1677, ) = (t303, ((((((l.f1681 * l.f1678) + (l.f1678 * l.f1681)) * t301) - (t2ff * ((l.f1681 * l.f1678) + (l.f1678 * l.f1681)))) / (t301 * t301)) / (2.0 * t303)), ((((((l.f1682 * l.f1678) + (l.f1678 * l.f1682)) * t301) - (t2ff * ((l.f1682 * l.f1678) + (l.f1678 * l.f1682)))) / (t301 * t301)) / (2.0 * t303)), ((((((l.f1683 * l.f1678) + (l.f1678 * l.f1683)) * t301) - (t2ff * ((l.f1683 * l.f1678) + (l.f1678 * l.f1683)))) / (t301 * t301)) / (2.0 * t303)), ((((((l.f1684 * l.f1678) + (l.f1678 * l.f1684)) * t301) - (t2ff * ((l.f1684 * l.f1678) + (l.f1678 * l.f1684)))) / (t301 * t301)) / (2.0 * t303)), );let t304: f64 = (l.f166b).sqrt();(l.f149e, l.f14a7, l.f14a8, l.f14a9, l.f14aa, ) = (t304, (l.f1674 / (2.0 * t304)), (l.f1675 / (2.0 * t304)), (l.f1676 / (2.0 * t304)), (l.f1677 / (2.0 * t304)), );let t305: f64 = (l.f166b * l.f149e);(l.f1685, l.f168e, l.f168f, l.f1690, l.f1691, ) = (t305, ((l.f1674 * l.f149e) + (l.f166b * l.f14a7)), ((l.f1675 * l.f149e) + (l.f166b * l.f14a8)), ((l.f1676 * l.f149e) + (l.f166b * l.f14a9)), ((l.f1677 * l.f149e) + (l.f166b * l.f14aa)), );}
        let t306: f64 = (-p.p824);let t307: f64 = (t306 * l.ff1f);let t308: f64 = (-1.0);let t309: f64 = if t307 == t308 { 1.0 } else { 0.0 };l.f8f6 = t309;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f6 != 0.0)) {let t30a: f64 = (l.f12c * l.f1685);let t30b: f64 = (1.0 + t30a);let t30c: f64 = (1.0 / t30b);(l.f18ee, l.f18f6, l.f18f7, l.f18f8, l.f18f9, ) = (t30c, (-(((l.f135 * l.f1685) + (l.f12c * l.f168e)) / (t30b * t30b))), (-(((l.f136 * l.f1685) + (l.f12c * l.f168f)) / (t30b * t30b))), (-(((l.f137 * l.f1685) + (l.f12c * l.f1690)) / (t30b * t30b))), (-(((l.f138 * l.f1685) + (l.f12c * l.f1691)) / (t30b * t30b))), );}
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f6 == 0.0)) {let t30d: f64 = (l.f12c * l.f1685);let t30e: f64 = (1.0 + t30d);let t30f: f64 = (-p.p824);let t310: f64 = (t30f * l.ff1f);let t311: f64 = (t30e).powf(t310);(l.f18ee, l.f18f6, l.f18f7, l.f18f8, l.f18f9, ) = (t311, if 0.0 == 0.0 && ((t310) as f64).is_finite() && ((t310) as f64).fract() == 0.0 { if t310 == 0.0 { 0.0 } else { (t310 * ((t30e).powf(t310 - 1.0) * ((l.f135 * l.f1685) + (l.f12c * l.f168e)))) } } else { (t311 * (t310 * (((l.f135 * l.f1685) + (l.f12c * l.f168e)) / t30e))) }, if 0.0 == 0.0 && ((t310) as f64).is_finite() && ((t310) as f64).fract() == 0.0 { if t310 == 0.0 { 0.0 } else { (t310 * ((t30e).powf(t310 - 1.0) * ((l.f136 * l.f1685) + (l.f12c * l.f168f)))) } } else { (t311 * (t310 * (((l.f136 * l.f1685) + (l.f12c * l.f168f)) / t30e))) }, if 0.0 == 0.0 && ((t310) as f64).is_finite() && ((t310) as f64).fract() == 0.0 { if t310 == 0.0 { 0.0 } else { (t310 * ((t30e).powf(t310 - 1.0) * ((l.f137 * l.f1685) + (l.f12c * l.f1690)))) } } else { (t311 * (t310 * (((l.f137 * l.f1685) + (l.f12c * l.f1690)) / t30e))) }, if 0.0 == 0.0 && ((t310) as f64).is_finite() && ((t310) as f64).fract() == 0.0 { if t310 == 0.0 { 0.0 } else { (t310 * ((t30e).powf(t310 - 1.0) * ((l.f138 * l.f1685) + (l.f12c * l.f1691)))) } } else { (t311 * (t310 * (((l.f138 * l.f1685) + (l.f12c * l.f1691)) / t30e))) }, );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) {let t312: f64 = (l.f1906 * l.f18ee);let t313: f64 = (l.f1906 + l.f18ee);let t314: f64 = (t312 / t313);(l.f1912, l.f191a, l.f191b, l.f191c, l.f191d, ) = (t314, ((((l.f1906 * l.f18f6) * t313) - (t312 * l.f18f6)) / (t313 * t313)), ((((l.f1906 * l.f18f7) * t313) - (t312 * l.f18f7)) / (t313 * t313)), ((((l.f1906 * l.f18f8) * t313) - (t312 * l.f18f8)) / (t313 * t313)), ((((l.f1906 * l.f18f9) * t313) - (t312 * l.f18f9)) / (t313 * t313)), );let t315: f64 = (l.f12c / l.f149e);let t316: f64 = (0.375 * t315);let t317: f64 = (t316).sqrt();(l.fdfc, l.fe05, l.fe06, l.fe07, l.fe08, ) = (t317, ((0.375 * (((l.f135 * l.f149e) - (l.f12c * l.f14a7)) / (l.f149e * l.f149e))) / (2.0 * t317)), ((0.375 * (((l.f136 * l.f149e) - (l.f12c * l.f14a8)) / (l.f149e * l.f149e))) / (2.0 * t317)), ((0.375 * (((l.f137 * l.f149e) - (l.f12c * l.f14a9)) / (l.f149e * l.f149e))) / (2.0 * t317)), ((0.375 * (((l.f138 * l.f149e) - (l.f12c * l.f14aa)) / (l.f149e * l.f149e))) / (2.0 * t317)), );let t318: f64 = (l.f1625 * l.f149e);let t319: f64 = (2.0 * t318);let t31a: f64 = (t319 - l.f166b);(l.fe3d, l.fe46, l.fe47, l.fe48, l.fe49, ) = (t31a, ((2.0 * ((l.f162e * l.f149e) + (l.f1625 * l.f14a7))) - l.f1674), ((2.0 * ((l.f162f * l.f149e) + (l.f1625 * l.f14a8))) - l.f1675), ((2.0 * ((l.f1630 * l.f149e) + (l.f1625 * l.f14a9))) - l.f1676), ((2.0 * ((l.f1631 * l.f149e) + (l.f1625 * l.f14aa))) - l.f1677), );let t31b: f64 = (l.fd4 * l.f1625);let t31c: f64 = (t31b * l.f149e);let t31d: f64 = (l.fd4 * l.f166b);let t31e: f64 = (t31c - t31d);let t31f: f64 = (l.f12c * l.f1685);let t320: f64 = (0.5 * t31f);let t321: f64 = (t31e + t320);(l.feac, l.feb5, l.feb6, l.feb7, l.feb8, ) = (t321, (((((l.fd4 * l.f162e) * l.f149e) + (t31b * l.f14a7)) - (l.fd4 * l.f1674)) + (0.5 * ((l.f135 * l.f1685) + (l.f12c * l.f168e)))), (((((l.fd4 * l.f162f) * l.f149e) + (t31b * l.f14a8)) - (l.fd4 * l.f1675)) + (0.5 * ((l.f136 * l.f1685) + (l.f12c * l.f168f)))), (((((l.fd4 * l.f1630) * l.f149e) + (t31b * l.f14a9)) - (l.fd4 * l.f1676)) + (0.5 * ((l.f137 * l.f1685) + (l.f12c * l.f1690)))), (((((l.fd4 * l.f1631) * l.f149e) + (t31b * l.f14aa)) - (l.fd4 * l.f1677)) + (0.5 * ((l.f138 * l.f1685) + (l.f12c * l.f1691)))), );let t322: f64 = (l.fe3d - 1.0);let t323: f64 = (t322 * l.fdfc);(l.f19d5, l.f19de, l.f19df, l.f19e0, l.f19e1, ) = (t323, ((l.fe46 * l.fdfc) + (t322 * l.fe05)), ((l.fe47 * l.fdfc) + (t322 * l.fe06)), ((l.fe48 * l.fdfc) + (t322 * l.fe07)), ((l.fe49 * l.fdfc) + (t322 * l.fe08)), );let t324: f64 = (l.f19d5 * l.f19d5);(l.f1b98, l.f1ba1, l.f1ba2, l.f1ba3, l.f1ba4, ) = (t324, ((l.f19de * l.f19d5) + (l.f19d5 * l.f19de)), ((l.f19df * l.f19d5) + (l.f19d5 * l.f19df)), ((l.f19e0 * l.f19d5) + (l.f19d5 * l.f19e0)), ((l.f19e1 * l.f19d5) + (l.f19d5 * l.f19e1)), );}
        let t325: f64 = if l.f19d5 > 0.0 { 1.0 } else { 0.0 };l.f8f7 = t325;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f7 != 0.0)) {let t326: f64 = (l.ff5f * l.f19d5);let t327: f64 = (1.0 + t326);let t328: f64 = (1.0 / t327);(l.f153e, l.f1546, l.f1547, l.f1548, l.f1549, ) = (t328, (-((l.ff5f * l.f19de) / (t327 * t327))), (-((l.ff5f * l.f19df) / (t327 * t327))), (-((l.ff5f * l.f19e0) / (t327 * t327))), (-((l.ff5f * l.f19e1) / (t327 * t327))), );}
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f7 == 0.0)) {let t329: f64 = (l.ff5f * l.f19d5);let t32a: f64 = (1.0 - t329);let t32b: f64 = (1.0 / t32a);(l.f153e, l.f1546, l.f1547, l.f1548, l.f1549, ) = (t32b, (-((-(l.ff5f * l.f19de)) / (t32a * t32a))), (-((-(l.ff5f * l.f19df)) / (t32a * t32a))), (-((-(l.ff5f * l.f19e0)) / (t32a * t32a))), (-((-(l.ff5f * l.f19e1)) / (t32a * t32a))), );}
        let t32c: f64 = (-l.f1b98);let t32d: f64 = (t32c + l.feac);let t32e: f64 = (-230.25850929940458);let t32f: f64 = if t32d > t32e { 1.0 } else { 0.0 };l.f8f8 = t32f;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f8 != 0.0)) {let t330: f64 = (-l.f1b98);let t331: f64 = (t330 + l.feac);let t332: f64 = (t331).exp();(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t332, (t332 * ((-l.f1ba1) + l.feb5)), (t332 * ((-l.f1ba2) + l.feb6)), (t332 * ((-l.f1ba3) + l.feb7)), (t332 * ((-l.f1ba4) + l.feb8)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f8 == 0.0)) {let t333: f64 = (-230.25850929940458);let t334: f64 = (-l.f1b98);let t335: f64 = (t334 + l.feac);let t336: f64 = (t333 - t335);let t337: f64 = (-230.25850929940458);let t338: f64 = (-l.f1b98);let t339: f64 = (t338 + l.feac);let t33a: f64 = (t337 - t339);let t33b: f64 = (-230.25850929940458);let t33c: f64 = (-l.f1b98);let t33d: f64 = (t33c + l.feac);let t33e: f64 = (t33b - t33d);let t33f: f64 = (t33e * 0.3333333333333333);let t340: f64 = (1.0 + t33f);let t341: f64 = (t33a * t340);let t342: f64 = (0.5 * t341);let t343: f64 = (1.0 + t342);let t344: f64 = (t336 * t343);let t345: f64 = (1.0 + t344);let t346: f64 = (1e-100 / t345);(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t346, (-((1e-100 * (((-((-l.f1ba1) + l.feb5)) * t343) + (t336 * (0.5 * (((-((-l.f1ba1) + l.feb5)) * t340) + (t33a * ((-((-l.f1ba1) + l.feb5)) * 0.3333333333333333))))))) / (t345 * t345))), (-((1e-100 * (((-((-l.f1ba2) + l.feb6)) * t343) + (t336 * (0.5 * (((-((-l.f1ba2) + l.feb6)) * t340) + (t33a * ((-((-l.f1ba2) + l.feb6)) * 0.3333333333333333))))))) / (t345 * t345))), (-((1e-100 * (((-((-l.f1ba3) + l.feb7)) * t343) + (t336 * (0.5 * (((-((-l.f1ba3) + l.feb7)) * t340) + (t33a * ((-((-l.f1ba3) + l.feb7)) * 0.3333333333333333))))))) / (t345 * t345))), (-((1e-100 * (((-((-l.f1ba4) + l.feb8)) * t343) + (t336 * (0.5 * (((-((-l.f1ba4) + l.feb8)) * t340) + (t33a * ((-((-l.f1ba4) + l.feb8)) * 0.3333333333333333))))))) / (t345 * t345))), );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) {let t347: f64 = (0.29214664 * l.f153e);let t348: f64 = (l.f153e * l.f153e);let t349: f64 = (l.ffc * t348);let t34a: f64 = (t347 + t349);let t34b: f64 = (l.f153e * l.f153e);let t34c: f64 = (t34b * l.f153e);let t34d: f64 = (l.f150 * t34c);let t34e: f64 = (t34a + t34d);let t34f: f64 = (t34e * l.f15e7);(l.f3b9, l.f3c1, l.f3c2, l.f3c3, l.f3c4, ) = (t34f, (((((0.29214664 * l.f1546) + (l.ffc * ((l.f1546 * l.f153e) + (l.f153e * l.f1546)))) + (l.f150 * ((((l.f1546 * l.f153e) + (l.f153e * l.f1546)) * l.f153e) + (t34b * l.f1546)))) * l.f15e7) + (t34e * l.f15f0)), (((((0.29214664 * l.f1547) + (l.ffc * ((l.f1547 * l.f153e) + (l.f153e * l.f1547)))) + (l.f150 * ((((l.f1547 * l.f153e) + (l.f153e * l.f1547)) * l.f153e) + (t34b * l.f1547)))) * l.f15e7) + (t34e * l.f15f1)), (((((0.29214664 * l.f1548) + (l.ffc * ((l.f1548 * l.f153e) + (l.f153e * l.f1548)))) + (l.f150 * ((((l.f1548 * l.f153e) + (l.f153e * l.f1548)) * l.f153e) + (t34b * l.f1548)))) * l.f15e7) + (t34e * l.f15f2)), (((((0.29214664 * l.f1549) + (l.ffc * ((l.f1549 * l.f153e) + (l.f153e * l.f1549)))) + (l.f150 * ((((l.f1549 * l.f153e) + (l.f153e * l.f1549)) * l.f153e) + (t34b * l.f1549)))) * l.f15e7) + (t34e * l.f15f3)), );}
        let t350: f64 = if l.f19d5 > 0.0 { 1.0 } else { 0.0 };l.f8f9 = t350;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f9 != 0.0)) {(l.f3c5, l.f3cd, l.f3ce, l.f3cf, l.f3d0, ) = (l.f3b9, l.f3c1, l.f3c2, l.f3c3, l.f3c4, );}
        let t351: f64 = (-230.25850929940458);let t352: f64 = if l.feac > t351 { 1.0 } else { 0.0 };l.f8fa = t352;
        if ((((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f9 == 0.0)) && (l.f8fa != 0.0)) {let t353: f64 = (l.feac).exp();(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t353, (t353 * l.feb5), (t353 * l.feb6), (t353 * l.feb7), (t353 * l.feb8), );}
        if ((((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f9 == 0.0)) && (l.f8fa == 0.0)) {let t354: f64 = (-230.25850929940458);let t355: f64 = (t354 - l.feac);let t356: f64 = (-230.25850929940458);let t357: f64 = (t356 - l.feac);let t358: f64 = (-230.25850929940458);let t359: f64 = (t358 - l.feac);let t35a: f64 = (t359 * 0.3333333333333333);let t35b: f64 = (1.0 + t35a);let t35c: f64 = (t357 * t35b);let t35d: f64 = (0.5 * t35c);let t35e: f64 = (1.0 + t35d);let t35f: f64 = (t355 * t35e);let t360: f64 = (1.0 + t35f);let t361: f64 = (1e-100 / t360);(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t361, (-((1e-100 * (((-l.feb5) * t35e) + (t355 * (0.5 * (((-l.feb5) * t35b) + (t357 * ((-l.feb5) * 0.3333333333333333))))))) / (t360 * t360))), (-((1e-100 * (((-l.feb6) * t35e) + (t355 * (0.5 * (((-l.feb6) * t35b) + (t357 * ((-l.feb6) * 0.3333333333333333))))))) / (t360 * t360))), (-((1e-100 * (((-l.feb7) * t35e) + (t355 * (0.5 * (((-l.feb7) * t35b) + (t357 * ((-l.feb7) * 0.3333333333333333))))))) / (t360 * t360))), (-((1e-100 * (((-l.feb8) * t35e) + (t355 * (0.5 * (((-l.feb8) * t35b) + (t357 * ((-l.feb8) * 0.3333333333333333))))))) / (t360 * t360))), );}
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) && (l.f8f9 == 0.0)) {let t362: f64 = (2.0 * l.f15e7);let t363: f64 = (t362 - l.f3b9);(l.f3c5, l.f3cd, l.f3ce, l.f3cf, l.f3d0, ) = (t363, ((2.0 * l.f15f0) - l.f3c1), ((2.0 * l.f15f1) - l.f3c2), ((2.0 * l.f15f2) - l.f3c3), ((2.0 * l.f15f3) - l.f3c4), );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8f5 == 0.0)) {let t364: f64 = (1.772453850905516 * 0.5);let t365: f64 = (l.fd4 * l.f3c5);let t366: f64 = (t365 / l.fdfc);let t367: f64 = (t364 * t366);(l.f4fa, l.f502, l.f503, l.f504, l.f505, ) = (t367, (t364 * ((((l.fd4 * l.f3cd) * l.fdfc) - (t365 * l.fe05)) / (l.fdfc * l.fdfc))), (t364 * ((((l.fd4 * l.f3ce) * l.fdfc) - (t365 * l.fe06)) / (l.fdfc * l.fdfc))), (t364 * ((((l.fd4 * l.f3cf) * l.fdfc) - (t365 * l.fe07)) / (l.fdfc * l.fdfc))), (t364 * ((((l.fd4 * l.f3d0) * l.fdfc) - (t365 * l.fe08)) / (l.fdfc * l.fdfc))), );let t368: f64 = (l.fc8 * l.f4fa);let t369: f64 = (t368 * l.f1912);let t36a: f64 = (p.p838 * t369);(l.fdba, l.fdc2, l.fdc3, l.fdc4, l.fdc5, ) = (t36a, (p.p838 * ((((l.fd0 * l.f4fa) + (l.fc8 * l.f502)) * l.f1912) + (t368 * l.f191a))), (p.p838 * ((((l.fd1 * l.f4fa) + (l.fc8 * l.f503)) * l.f1912) + (t368 * l.f191b))), (p.p838 * ((((l.fd2 * l.f4fa) + (l.fc8 * l.f504)) * l.f1912) + (t368 * l.f191c))), (p.p838 * ((((l.fd3 * l.f4fa) + (l.fc8 * l.f505)) * l.f1912) + (t368 * l.f191d))), );}
        let t36b: f64 = if p.p844 == 0.0 { 1.0 } else { 0.0 };l.f8fb = t36b;
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8fb != 0.0)) {(l.fc72, l.fc7a, l.fc7b, l.fc7c, l.fc7d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t36c: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };l.f8fc = t36c;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8fb == 0.0)) && (l.f8fc != 0.0)) {let t36d: f64 = (p.p821 - l.f16e8);let t36e: f64 = (t36d * l.f1724);let t36f: f64 = (t36e).sqrt();(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t36f, 0.0, 0.0, 0.0, 0.0, );}
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8fb == 0.0)) && (l.f8fc == 0.0)) {let t370: f64 = (p.p821 - l.f16e8);let t371: f64 = (t370 * l.f1724);let t372: f64 = (t371).powf(p.p824);(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t372, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8fb == 0.0)) {let t373: f64 = (p.p821 - l.f16e8);let t374: f64 = (t373 * l.f18d8);let t375: f64 = (t374 / l.f15e7);let t376: f64 = (l.ff1f * t375);(l.f498, l.f4a1, l.f4a2, l.f4a3, l.f4a4, ) = (t376, (l.ff1f * (-((t374 * l.f15f0) / (l.f15e7 * l.f15e7)))), (l.ff1f * (-((t374 * l.f15f1) / (l.f15e7 * l.f15e7)))), (l.ff1f * (-((t374 * l.f15f2) / (l.f15e7 * l.f15e7)))), (l.ff1f * (-((t374 * l.f15f3) / (l.f15e7 * l.f15e7)))), );}
        let t377: f64 = (-l.f43a);let t378: f64 = (t377 / l.f498);let t379: f64 = (t378).abs();let t37a: f64 = if t379 < 230.25850929940458 { 1.0 } else { 0.0 };l.f8fd = t37a;
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8fb == 0.0)) && (l.f8fd != 0.0)) {let t37b: f64 = (-l.f43a);let t37c: f64 = (t37b / l.f498);let t37d: f64 = (t37c).exp();(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t37d, (t37d * (-((t37b * l.f4a1) / (l.f498 * l.f498)))), (t37d * (-((t37b * l.f4a2) / (l.f498 * l.f498)))), (t37d * (-((t37b * l.f4a3) / (l.f498 * l.f498)))), (t37d * (-((t37b * l.f4a4) / (l.f498 * l.f498)))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t37e: f64 = (-l.f43a);let t37f: f64 = (t37e / l.f498);let t380: f64 = if t37f < 0.0 { 1.0 } else { 0.0 };l.f8fe = t380;
        if ((((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8fb == 0.0)) && (l.f8fd == 0.0)) && (l.f8fe != 0.0)) {let t381: f64 = (-230.25850929940458);let t382: f64 = (-l.f43a);let t383: f64 = (t382 / l.f498);let t384: f64 = (t381 - t383);let t385: f64 = (-230.25850929940458);let t386: f64 = (-l.f43a);let t387: f64 = (t386 / l.f498);let t388: f64 = (t385 - t387);let t389: f64 = (-230.25850929940458);let t38a: f64 = (-l.f43a);let t38b: f64 = (t38a / l.f498);let t38c: f64 = (t389 - t38b);let t38d: f64 = (t38c * 0.3333333333333333);let t38e: f64 = (1.0 + t38d);let t38f: f64 = (t388 * t38e);let t390: f64 = (0.5 * t38f);let t391: f64 = (1.0 + t390);let t392: f64 = (t384 * t391);let t393: f64 = (1.0 + t392);let t394: f64 = (1e-100 / t393);(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t394, (-((1e-100 * (((-(-((t382 * l.f4a1) / (l.f498 * l.f498)))) * t391) + (t384 * (0.5 * (((-(-((t386 * l.f4a1) / (l.f498 * l.f498)))) * t38e) + (t388 * ((-(-((t38a * l.f4a1) / (l.f498 * l.f498)))) * 0.3333333333333333))))))) / (t393 * t393))), (-((1e-100 * (((-(-((t382 * l.f4a2) / (l.f498 * l.f498)))) * t391) + (t384 * (0.5 * (((-(-((t386 * l.f4a2) / (l.f498 * l.f498)))) * t38e) + (t388 * ((-(-((t38a * l.f4a2) / (l.f498 * l.f498)))) * 0.3333333333333333))))))) / (t393 * t393))), (-((1e-100 * (((-(-((t382 * l.f4a3) / (l.f498 * l.f498)))) * t391) + (t384 * (0.5 * (((-(-((t386 * l.f4a3) / (l.f498 * l.f498)))) * t38e) + (t388 * ((-(-((t38a * l.f4a3) / (l.f498 * l.f498)))) * 0.3333333333333333))))))) / (t393 * t393))), (-((1e-100 * (((-(-((t382 * l.f4a4) / (l.f498 * l.f498)))) * t391) + (t384 * (0.5 * (((-(-((t386 * l.f4a4) / (l.f498 * l.f498)))) * t38e) + (t388 * ((-(-((t38a * l.f4a4) / (l.f498 * l.f498)))) * 0.3333333333333333))))))) / (t393 * t393))), );}
        if ((((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8fb == 0.0)) && (l.f8fd == 0.0)) && (l.f8fe == 0.0)) {let t395: f64 = (-l.f43a);let t396: f64 = (t395 / l.f498);let t397: f64 = (t396 - 230.25850929940458);let t398: f64 = (-l.f43a);let t399: f64 = (t398 / l.f498);let t39a: f64 = (t399 - 230.25850929940458);let t39b: f64 = (-l.f43a);let t39c: f64 = (t39b / l.f498);let t39d: f64 = (t39c - 230.25850929940458);let t39e: f64 = (t39d * 0.3333333333333333);let t39f: f64 = (1.0 + t39e);let t3a0: f64 = (t39a * t39f);let t3a1: f64 = (0.5 * t3a0);let t3a2: f64 = (1.0 + t3a1);let t3a3: f64 = (t397 * t3a2);let t3a4: f64 = (1.0 + t3a3);let t3a5: f64 = (1e100 * t3a4);(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t3a5, (1e100 * (((-((t395 * l.f4a1) / (l.f498 * l.f498))) * t3a2) + (t397 * (0.5 * (((-((t398 * l.f4a1) / (l.f498 * l.f498))) * t39f) + (t39a * ((-((t39b * l.f4a1) / (l.f498 * l.f498))) * 0.3333333333333333))))))), (1e100 * (((-((t395 * l.f4a2) / (l.f498 * l.f498))) * t3a2) + (t397 * (0.5 * (((-((t398 * l.f4a2) / (l.f498 * l.f498))) * t39f) + (t39a * ((-((t39b * l.f4a2) / (l.f498 * l.f498))) * 0.3333333333333333))))))), (1e100 * (((-((t395 * l.f4a3) / (l.f498 * l.f498))) * t3a2) + (t397 * (0.5 * (((-((t398 * l.f4a3) / (l.f498 * l.f498))) * t39f) + (t39a * ((-((t39b * l.f4a3) / (l.f498 * l.f498))) * 0.3333333333333333))))))), (1e100 * (((-((t395 * l.f4a4) / (l.f498 * l.f498))) * t3a2) + (t397 * (0.5 * (((-((t398 * l.f4a4) / (l.f498 * l.f498))) * t39f) + (t39a * ((-((t39b * l.f4a4) / (l.f498 * l.f498))) * 0.3333333333333333))))))), );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8fb == 0.0)) {let t3a6: f64 = (l.f16ae * l.f498);let t3a7: f64 = (t3a6 * l.f498);let t3a8: f64 = (t3a7 * l.f15e7);let t3a9: f64 = (p.p844 * t3a8);(l.fc72, l.fc7a, l.fc7b, l.fc7c, l.fc7d, ) = (t3a9, (p.p844 * (((((l.f16ae * l.f4a1) * l.f498) + (t3a6 * l.f4a1)) * l.f15e7) + (t3a7 * l.f15f0))), (p.p844 * (((((l.f16ae * l.f4a2) * l.f498) + (t3a6 * l.f4a2)) * l.f15e7) + (t3a7 * l.f15f1))), (p.p844 * (((((l.f16ae * l.f4a3) * l.f498) + (t3a6 * l.f4a3)) * l.f15e7) + (t3a7 * l.f15f2))), (p.p844 * (((((l.f16ae * l.f4a4) * l.f498) + (t3a6 * l.f4a4)) * l.f15e7) + (t3a7 * l.f15f3))), );}
        let t3aa: f64 = if p.p853 > 1000.0 { 1.0 } else { 0.0 };l.f8ff = t3aa;
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8ff != 0.0)) {(l.f456, l.f45e, l.f45f, l.f460, l.f461, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
        let t3ab: f64 = (-l.f6d);let t3ac: f64 = (t3ab * p.p853);let t3ad: f64 = if l.f16e1 > t3ac { 1.0 } else { 0.0 };l.f900 = t3ad;let t3ae: f64 = if p.p856 == 4.0 { 1.0 } else { 0.0 };l.f901 = t3ae;
        if ((((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8ff == 0.0)) && (l.f900 != 0.0)) && (l.f901 != 0.0)) {let t3af: f64 = (l.f16e1 * l.f1750);let t3b0: f64 = (l.f16e1 * l.f1750);let t3b1: f64 = (t3af * t3b0);let t3b2: f64 = (l.f16e1 * l.f1750);let t3b3: f64 = (t3b1 * t3b2);let t3b4: f64 = (l.f16e1 * l.f1750);let t3b5: f64 = (t3b3 * t3b4);(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t3b5, 0.0, 0.0, 0.0, 0.0, );}
        if ((((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8ff == 0.0)) && (l.f900 != 0.0)) && (l.f901 == 0.0)) {let t3b6: f64 = (l.f16e1 * l.f1750);let t3b7: f64 = (t3b6).abs();let t3b8: f64 = (t3b7).powf(p.p856);(l.f15e7, l.f15f0, l.f15f1, l.f15f2, l.f15f3, ) = (t3b8, 0.0, 0.0, 0.0, 0.0, );}
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8ff == 0.0)) && (l.f900 != 0.0)) {let t3b9: f64 = (1.0 - l.f15e7);let t3ba: f64 = (1.0 / t3b9);(l.f456, l.f45e, l.f45f, l.f460, l.f461, ) = (t3ba, (-((-l.f15f0) / (t3b9 * t3b9))), (-((-l.f15f1) / (t3b9 * t3b9))), (-((-l.f15f2) / (t3b9 * t3b9))), (-((-l.f15f3) / (t3b9 * t3b9))), );}
        if (((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) && (l.f8ff == 0.0)) && (l.f900 == 0.0)) {let t3bb: f64 = (l.f6d * p.p853);let t3bc: f64 = (l.f16e1 + t3bb);let t3bd: f64 = (t3bc * l.f1325);let t3be: f64 = (l.f4d5 + t3bd);(l.f456, l.f45e, l.f45f, l.f460, l.f461, ) = (t3be, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f8f1 == 0.0)) {let t3bf: f64 = (l.fc83 + l.fdae);let t3c0: f64 = (t3bf + l.fdba);let t3c1: f64 = (t3c0 + l.fc72);let t3c2: f64 = (p.p29 * t3c1);let t3c3: f64 = (t3c2 * l.f456);(l.fce5, l.fced, l.fcee, l.fcef, l.fcf0, ) = (t3c3, (((p.p29 * ((l.fdb6 + l.fdc2) + l.fc7a)) * l.f456) + (t3c2 * l.f45e)), (((p.p29 * ((l.fdb7 + l.fdc3) + l.fc7b)) * l.f456) + (t3c2 * l.f45f)), (((p.p29 * ((l.fdb8 + l.fdc4) + l.fc7c)) * l.f456) + (t3c2 * l.f460)), (((p.p29 * ((l.fdb9 + l.fdc5) + l.fc7d)) * l.f456) + (t3c2 * l.f461)), );}
        let t3c4: f64 = if l.fe3b == 0.0 { 1.0 } else { 0.0 };l.f902 = t3c4;
        if (((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f902 != 0.0)) {(l.fd20, l.fd28, l.fd29, l.fd2a, l.fd2b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f902 == 0.0)) {let t3c5: f64 = (l.fc99 * l.fc84);l.fc83 = t3c5;}
        let t3c6: f64 = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };l.f903 = t3c6;
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f902 == 0.0)) && (l.f903 != 0.0)) {(l.fdae, l.fdb6, l.fdb7, l.fdb8, l.fdb9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f8bb != 0.0) && (l.f8e8 != 0.0)) && (l.f902 == 0.0)) && (l.f903 == 0.0)) {let t3c7: f64 = (l.f1738 - l.f1808);l.f16f3 = t3c7;}
    }
}
