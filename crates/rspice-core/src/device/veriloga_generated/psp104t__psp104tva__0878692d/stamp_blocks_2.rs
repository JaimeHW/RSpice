#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        l.f168a = l.f168c;
        let (t0,) = {
    if (l.f113b > 0.0) {
        (l.f113b,)
    } else {
        (0.0,)
    }
};
        l.f1139 = t0;
        let (t2,) = {
    if (l.f112f > 0.0) {
        let (t1,) = {
            if (l.f112f < 1.0) {
                (l.f112f,)
            } else {
                (1.0,)
            }
        };
        (t1,)
    } else {
        (0.0,)
    }
};
        l.f112d = t2;
        let (t3,) = {
    if (l.f1137 > 0.0) {
        (l.f1137,)
    } else {
        (0.0,)
    }
};
        l.f1135 = t3;
        let (t4,) = {
    if (l.f188 > 0.0) {
        (l.f188,)
    } else {
        (0.0,)
    }
};
        l.f186 = t4;
        let (t6,) = {
    if (l.f17c > 0.0) {
        let (t5,) = {
            if (l.f17c < 1.0) {
                (l.f17c,)
            } else {
                (1.0,)
            }
        };
        (t5,)
    } else {
        (0.0,)
    }
};
        l.f17a = t6;
        let (t7,) = {
    if (l.f184 > 0.0) {
        (l.f184,)
    } else {
        (0.0,)
    }
};
        l.f182 = t7;
        let (t8,) = {
    if (l.f1402 > 0.0) {
        (l.f1402,)
    } else {
        (0.0,)
    }
};
        l.f1401 = t8;l.f1440 = l.f1441;l.f13ff = l.f1400;l.f13fd = l.f13fe;l.f1433 = l.f1434;l.f1431 = l.f1432;l.f1450 = l.f1451;
        let (t9,) = {
    if (l.f144b > 0.0001) {
        (l.f144b,)
    } else {
        (0.0001,)
    }
};
        l.f144a = t9;
        let (ta,) = {
    if (l.f225 > 0.0) {
        (l.f225,)
    } else {
        (0.0,)
    }
};
        l.f223 = ta;l.f16b0 = l.f16b1;let tb: f64 = (p.p31 * l.ffbf);
        let (td,) = {
    if (tb > 0.0) {
        let tc: f64 = (p.p31 * l.ffbf);
        (tc,)
    } else {
        (0.0,)
    }
};
        l.ff9e = td;l.f4ab = p.p16;l.f2e4 = p.p15;l.f4ad = p.p18;l.f2e6 = p.p17;let te: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.f7ca = te;
        if (l.f7ca != 0.0) {l.f17e9 = l.f17e5;l.ffc6 = l.ffc2;l.f2d = l.f29;l.f135 = l.f12f;l.f1692 = l.f168e;l.f1a1 = l.f19d;l.fd78 = l.fd74;l.f592 = l.f58e;l.f59e = l.f59a;l.f1ad = l.f1a5;l.f4db = l.f4d7;l.f1bb = l.f1b7;l.f18e = l.f18a;}
        let tf: f64 = (8.8541878176e-12 * l.f41d);l.f41b = tf;let t10: f64 = (l.f41b / l.f17df);l.f1ea = t10;let t11: f64 = (l.f17df * l.f17df);l.f17e3 = t11;let t12: f64 = (l.f1ea / 1.6021918e-19);l.f1db = t12;let t13: f64 = (l.f492 * l.ffb5);l.ffb9 = t13;
        let (t15,) = {
    if (l.ffb9 > 1e20) {
        let (t14,) = {
            if (l.ffb9 < 1e26) {
                (l.ffb9,)
            } else {
                (1e26,)
            }
        };
        (t14,)
    } else {
        (1e20,)
    }
};
        l.ffb9 = t15;l.f13e0 = 0.0;let t16: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };l.f7e0 = t16;
        if (l.f7e0 != 0.0) {let t17: f64 = (0.4 * 5.951993);let t18: f64 = (t17 * p.p51);let t19: f64 = (l.f1ea).powf(0.6666666666666666);let t1a: f64 = (t18 * t19);l.f13e0 = t1a;}
        let t1b: f64 = (-1.0);let t1c: f64 = if l.f1b5 == t1b { 1.0 } else { 0.0 };l.f7f6 = t1c;
        if ((l.f7e0 != 0.0) && (l.f7f6 != 0.0)) {let t1d: f64 = (7.448711 / 5.951993);let t1e: f64 = (t1d * l.f13e0);l.f13e0 = t1e;}
        let t1f: f64 = (1e-8 * l.f1ea);let t20: f64 = (t1f / l.f421);l.f3de = t20;let t21: f64 = (0.5 * l.f4e9);l.f450 = t21;l.f451 = 0.5;let t22: f64 = (-1.0);let t23: f64 = if l.f1b5 == t22 { 1.0 } else { 0.0 };l.f80c = t23;
        if (l.f80c != 0.0) {let t24: f64 = (0.3333333333333333 * l.f4e9);l.f450 = t24;l.f451 = 0.3333333333333333;}
        let t25: f64 = (-2.0);let t26: f64 = (t25 / l.ffa);let t27: f64 = (t26 + 1.0);let t28: f64 = (2.0_f64).powf(t27);let t29: f64 = (t28 - 1.0);l.f16dc = t29;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        l: &mut StampLocals,
    ) {
        let t2a: f64 = (l.f16dc - 1.0);let t2b: f64 = (l.f16dc - 1.0);let t2c: f64 = (t2a * t2b);let t2d: f64 = (4.0 * l.f16dc);
        let (t2f,) = {
    if (t2d > 0.0001) {
        let t2e: f64 = (4.0 * l.f16dc);
        (t2e,)
    } else {
        (0.0001,)
    }
};
        let t30: f64 = (t2c / t2f);l.fb8 = t30;let t31: f64 = (-2.0);let t32: f64 = (t31 / l.ffe);let t33: f64 = (t32 + 1.0);let t34: f64 = (2.0_f64).powf(t33);let t35: f64 = (t34 - 1.0);l.f16dc = t35;let t36: f64 = (l.f16dc - 1.0);let t37: f64 = (l.f16dc - 1.0);let t38: f64 = (t36 * t37);let t39: f64 = (4.0 * l.f16dc);
        let (t3b,) = {
    if (t39 > 0.0001) {
        let t3a: f64 = (4.0 * l.f16dc);
        (t3a,)
    } else {
        (0.0001,)
    }
};
        let t3c: f64 = (t38 / t3b);l.fba = t3c;let t3d: f64 = (1.0 / l.f1a6c);l.fe3a = t3d;let t3e: f64 = (l.f41b / l.f17e5);l.f1e6 = t3e;let t3f: f64 = (l.f41b / l.f17e9);l.f1e7 = t3f;let t40: f64 = (2.0 * 1.6021918e-19);let t41: f64 = (t40 * l.ffc2);let t42: f64 = (t41 * l.f421);let t43: f64 = (t42 * l.fe38);let t44: f64 = (t43).sqrt();let t45: f64 = (t44 / l.f1e6);l.f661 = t45;let t46: f64 = (2.0 * 1.6021918e-19);let t47: f64 = (t46 * l.ffc6);let t48: f64 = (t47 * l.f421);let t49: f64 = (t48 * l.fe38);let t4a: f64 = (t49).sqrt();let t4b: f64 = (t4a / l.f1e7);l.f65f = t4b;let t4c: f64 = (l.f661 * l.f661);l.f65d = t4c;let t4d: f64 = (l.f65f * l.f65f);l.f65b = t4d;let t4e: f64 = (l.f1a9 * 0.005);let t4f: f64 = (t4e * l.fe38);let t50: f64 = (t4f).exp();let t51: f64 = (t50 - 1.0);let t52: f64 = (t51).ln();let t53: f64 = (t52 / l.f1a9);let t54: f64 = (0.005 * l.fe38);let t55: f64 = (t54).exp();let t56: f64 = (t55 - 1.0);let t57: f64 = (t56).ln();let t58: f64 = (t53 - t57);l.f3d5 = t58;let t59: f64 = (0.5 * l.f661);let t5a: f64 = (t59).ln();let t5b: f64 = (t5a + l.f3d5);l.f3d3 = t5b;let t5c: f64 = (0.5 * l.f65f);let t5d: f64 = (t5c).ln();let t5e: f64 = (t5d + l.f3d5);l.f3d1 = t5e;let t5f: f64 = (1.0 / l.f661);l.fe17 = t5f;let t60: f64 = (3.1 * l.f661);let t61: f64 = (t60 + 8.5);l.f14bb = t61;let t62: f64 = (l.f14bb * l.f14bb);l.f14be = t62;let t63: f64 = (0.5 * l.f14bb);l.f14b5 = t63;let t64: f64 = if l.fe17 < 0.06 { 1.0 } else { 0.0 };l.f822 = t64;
        if (l.f822 != 0.0) {let t65: f64 = (64.0 * l.fe17);l.f14b3 = t65;}
        let t66: f64 = if l.fe17 <= 0.45 { 1.0 } else { 0.0 };l.f828 = t66;
        if ((l.f822 == 0.0) && (l.f828 != 0.0)) {let t67: f64 = (22.0 * l.fe17);let t68: f64 = (t67 + 3.0);l.f14b3 = t68;}
        let t69: f64 = if l.fe17 <= 1.6 { 1.0 } else { 0.0 };l.f82a = t69;
        if (((l.f822 == 0.0) && (l.f828 == 0.0)) && (l.f82a != 0.0)) {let t6a: f64 = (-7.2);let t6b: f64 = (t6a * l.fe17);let t6c: f64 = (t6b + 15.5);l.f14b3 = t6c;}
        if (((l.f822 == 0.0) && (l.f828 == 0.0)) && (l.f82a == 0.0)) {l.f14b3 = l.f661;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        l: &mut StampLocals,
    ) {
        let t6d: f64 = (l.f65d * 0.5);let t6e: f64 = (l.f14b5 + t6d);let t6f: f64 = (l.f65d * 0.25);let t70: f64 = (l.f14b5 + t6f);let t71: f64 = (t70 + l.f14b3);let t72: f64 = (t71).sqrt();let t73: f64 = (l.f661 * t72);let t74: f64 = (t6e - t73);l.f14b8 = t74;let t75: f64 = (1.0 / l.f65f);l.fe17 = t75;let t76: f64 = (3.1 * l.f65f);let t77: f64 = (t76 + 8.5);l.f14bb = t77;let t78: f64 = (l.f14bb * l.f14bb);l.f14bc = t78;let t79: f64 = (0.5 * l.f14bb);l.f14b5 = t79;let t7a: f64 = if l.fe17 < 0.06 { 1.0 } else { 0.0 };l.f82c = t7a;
        if (l.f82c != 0.0) {let t7b: f64 = (64.0 * l.fe17);l.f14b1 = t7b;}
        let t7c: f64 = if l.fe17 <= 0.45 { 1.0 } else { 0.0 };l.f82e = t7c;
        if ((l.f82c == 0.0) && (l.f82e != 0.0)) {let t7d: f64 = (22.0 * l.fe17);let t7e: f64 = (t7d + 3.0);l.f14b1 = t7e;}
        let t7f: f64 = if l.fe17 <= 1.6 { 1.0 } else { 0.0 };l.f832 = t7f;
        if (((l.f82c == 0.0) && (l.f82e == 0.0)) && (l.f832 != 0.0)) {let t80: f64 = (-7.2);let t81: f64 = (t80 * l.fe17);let t82: f64 = (t81 + 15.5);l.f14b1 = t82;}
        if (((l.f82c == 0.0) && (l.f82e == 0.0)) && (l.f832 == 0.0)) {l.f14b1 = l.f65f;}
        let t83: f64 = (l.f65b * 0.5);let t84: f64 = (l.f14b5 + t83);let t85: f64 = (l.f65b * 0.25);let t86: f64 = (l.f14b5 + t85);let t87: f64 = (t86 + l.f14b1);let t88: f64 = (t87).sqrt();let t89: f64 = (l.f65f * t88);let t8a: f64 = (t84 - t89);l.f14b6 = t8a;let t8b: f64 = (1.0 / l.f1b1);l.fdf9 = t8b;let t8c: f64 = (4.0 * 0.3333333333333333);let t8d: f64 = (2.0 * 1.6021918e-19);let t8e: f64 = (t8d * 9.1093826e-31);let t8f: f64 = (t8e * l.f1b1);let t90: f64 = (t8f).sqrt();let t91: f64 = (t8c * t90);let t92: f64 = (t91 / 1.05457168e-34);l.f10a = t92;let t93: f64 = (l.f10a * l.f17df);l.f10e = t93;let t94: f64 = (l.f10a * l.f17e5);l.f13f = t94;let t95: f64 = (l.f10a * l.f17e9);l.f140 = t95;l.f5a6 = 0.0;let t96: f64 = if l.f596 < 0.0 { 1.0 } else { 0.0 };l.f842 = t96;
        if (l.f842 != 0.0) {let t97: f64 = (-0.495);let t98: f64 = (t97 * l.f58a);let t99: f64 = (t98 / l.f596);l.f5a6 = t99;}
        l.f5a8 = 0.0;let t9a: f64 = if l.f59a < 0.0 { 1.0 } else { 0.0 };l.f858 = t9a;
        if (l.f858 != 0.0) {let t9b: f64 = (-0.495);let t9c: f64 = (t9b * l.f58e);let t9d: f64 = (t9c / l.f59a);l.f5a8 = t9d;}
        let t9e: f64 = if l.f59e < 0.0 { 1.0 } else { 0.0 };l.f86c = t9e;
        if (l.f86c != 0.0) {let t9f: f64 = (-0.495);let ta0: f64 = (t9f * l.f592);let ta1: f64 = (ta0 / l.f59e);l.f5aa = ta1;}
        let ta2: f64 = (l.f1448).powf(l.f16a4);l.f171a = ta2;let ta3: f64 = (l.fd70 * l.f171a);l.fd70 = ta3;let ta4: f64 = (l.fd74 * l.f171a);l.fd74 = ta4;let ta5: f64 = (l.fd78 * l.f171a);l.fd78 = ta5;let ta6: f64 = (l.f29 * 4e-18);let ta7: f64 = (l.f17e5 * l.f17e5);let ta8: f64 = (ta6 / ta7);l.f32 = ta8;let ta9: f64 = (l.f2d * 4e-18);let taa: f64 = (l.f17e9 * l.f17e9);let tab: f64 = (ta9 / taa);l.f31 = tab;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tac: f64 = (l.f168e * l.f279);let tad: f64 = (1.0 + tac);
        let (tb0,) = {
    if (tad > 0.0) {
        let tae: f64 = (l.f168e * l.f279);let taf: f64 = (1.0 + tae);
        (taf,)
    } else {
        (0.0,)
    }
};
        l.f10a = tb0;let tb1: f64 = (l.f12f * l.f10a);l.f133 = tb1;let tb2: f64 = (l.f133 * l.f17e5);let tb3: f64 = (tb2 * 500000000.0);l.f13d = tb3;let tb4: f64 = (l.f1692 * l.f279);let tb5: f64 = (1.0 + tb4);
        let (tb8,) = {
    if (tb5 > 0.0) {
        let tb6: f64 = (l.f1692 * l.f279);let tb7: f64 = (1.0 + tb6);
        (tb7,)
    } else {
        (0.0,)
    }
};
        l.f10a = tb8;let tb9: f64 = (l.f135 * l.f10a);l.f139 = tb9;let tba: f64 = (l.f139 * l.f17e9);let tbb: f64 = (tba * 500000000.0);l.f13b = tbb;l.f19fc = 0.0;let tbc: f64 = if l.f4df > 1e-10 { 1.0 } else { 0.0 };l.f881 = tbc;
        if (l.f881 != 0.0) {let tbd: f64 = (0.75 / l.f4df);l.f19fc = tbd;}
        let tbe: f64 = (l.f106 * l.f106);l.f33 = tbe;let tbf: f64 = (l.f1448).powf(l.f16b0);let tc0: f64 = (l.f144a * tbf);l.f144c = tc0;let tc1: f64 = (9.1093826e-31 * 1000000000.0);let tc2: f64 = (tc1 * l.f523);l.f491 = tc2;let tc3: f64 = if l.f1401 > 0.0 { 1.0 } else { 0.0 };l.f894 = tc3;
        if (l.f894 != 0.0) {let tc4: f64 = (1.0 / l.f1401);l.f5fc = tc4;}
        if (l.f894 == 0.0) {l.f5fc = 0.0;}
        let tc5: f64 = if l.f1440 > 0.0 { 1.0 } else { 0.0 };l.f8a9 = tc5;
        if (l.f8a9 != 0.0) {let tc6: f64 = (1.0 / l.f1440);l.f683 = tc6;}
        if (l.f8a9 == 0.0) {l.f683 = 0.0;}
        let tc7: f64 = if l.f13ff > 0.0 { 1.0 } else { 0.0 };l.f8bb = tc7;
        if (l.f8bb != 0.0) {let tc8: f64 = (1.0 / l.f13ff);l.f5ba = tc8;}
        if (l.f8bb == 0.0) {l.f5ba = 0.0;}
        let tc9: f64 = if l.f13fd > 0.0 { 1.0 } else { 0.0 };l.f8d0 = tc9;
        if (l.f8d0 != 0.0) {let tca: f64 = (1.0 / l.f13fd);l.f589 = tca;}
        if (l.f8d0 == 0.0) {l.f589 = 0.0;}
        let tcb: f64 = if l.f1433 > 0.0 { 1.0 } else { 0.0 };l.f8e5 = tcb;
        if (l.f8e5 != 0.0) {let tcc: f64 = (1.0 / l.f1433);l.f5fe = tcc;}
        if (l.f8e5 == 0.0) {l.f5fe = 0.0;}
        let tcd: f64 = if l.f1431 > 0.0 { 1.0 } else { 0.0 };l.f8f7 = tcd;
        if (l.f8f7 != 0.0) {let tce: f64 = (1.0 / l.f1431);l.f5fd = tce;}
        if (l.f8f7 == 0.0) {l.f5fd = 0.0;}
        let tcf: f64 = if l.f1450 > 0.0 { 1.0 } else { 0.0 };l.f90b = tcf;
        if (l.f90b != 0.0) {let td0: f64 = (1.0 / l.f1450);l.fc96 = td0;}
        if (l.f90b == 0.0) {l.fc96 = 0.0;}
        let td1: f64 = (l.f1b * l.fe57);l.f19 = td1;let td2: f64 = (l.ff0f * l.fe57);l.ff0d = td2;let td3: f64 = (l.fefb * l.fe57);l.fef9 = td3;let td4: f64 = (l.f17 * l.fe57);l.f15 = td4;let td5: f64 = (l.ff0b * l.fe57);l.ff09 = td5;let td6: f64 = (l.fef7 * l.fe57);l.fef5 = td6;l.fe95 = 0.0;let td7: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };l.f91e = td7;
        if (l.f91e != 0.0) {l.fe95 = 1.0;}
        l.fe97 = l.f1afd;let td8: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };l.f932 = td8;
        if (l.f932 != 0.0) {
            let (td9,) = {
    if (l.fe93 > 0.0) {
        (l.fe93,)
    } else {
        (0.0,)
    }
};
            l.fe97 = td9;
        }
        let tda: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };l.f947 = tda;
        if (l.f947 != 0.0) {let tdb: f64 = (l.fd0 * l.fe57);l.f19 = tdb;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f947 != 0.0) {let tdc: f64 = (l.f1122 * l.fe57);let tdd: f64 = (l.fe95 * l.fe97);let tde: f64 = (tdc - tdd);l.ff0d = tde;l.fef9 = l.fe97;let tdf: f64 = (l.f1d * l.fe57);l.f15 = tdf;let te0: f64 = (l.f1049 * l.fe57);let te1: f64 = (l.fe95 * l.fe97);let te2: f64 = (te0 - te1);l.ff09 = te2;l.fef5 = l.fe97;}
        let te3: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };l.f958 = te3;
        if (l.f958 != 0.0) {
            let (te4,) = {
    if (l.f19 > 0.0) {
        (l.f19,)
    } else {
        (0.0,)
    }
};
            l.f1b = te4;
        }
        if (l.f958 != 0.0) {
            let (te5,) = {
    if (l.ff0d > 0.0) {
        (l.ff0d,)
    } else {
        (0.0,)
    }
};
            l.ff0f = te5;
        }
        if (l.f958 != 0.0) {
            let (te6,) = {
    if (l.fef9 > 0.0) {
        (l.fef9,)
    } else {
        (0.0,)
    }
};
            l.fefb = te6;
        }
        if (l.f958 != 0.0) {
            let (te7,) = {
    if (l.f15 > 0.0) {
        (l.f15,)
    } else {
        (0.0,)
    }
};
            l.f17 = te7;
        }
        if (l.f958 != 0.0) {
            let (te8,) = {
    if (l.ff09 > 0.0) {
        (l.ff09,)
    } else {
        (0.0,)
    }
};
            l.ff0b = te8;
        }
        if (l.f958 != 0.0) {
            let (te9,) = {
    if (l.fef5 > 0.0) {
        (l.fef5,)
    } else {
        (0.0,)
    }
};
            l.fef7 = te9;
        }
        if (l.f958 == 0.0) {l.f1b = 0.0;l.ff0f = 0.0;l.fefb = 0.0;l.f17 = 0.0;l.ff0b = 0.0;l.fef7 = 0.0;}
        l.f1912 = 0.0;l.f1910 = 0.0;l.f19b2 = 0.0;l.f19b0 = 0.0;l.f195a = 0.0;l.f1958 = 0.0;l.f18e3 = 0.0;l.f18e1 = 0.0;l.f1a30 = 0.0;l.f1a2e = 0.0;l.f479 = 0.0;l.f477 = 0.0;l.fe62 = 0.0;l.fe61 = 0.0;l.ff4f = 1.0;l.ff4e = 1.0;(l.fe68, l.fe69, l.fe6a, l.fe6b, l.fe6c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe63, l.fe64, l.fe65, l.fe66, l.fe67, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.ff55, l.ff56, l.ff57, l.ff58, l.ff59, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );(l.ff50, l.ff51, l.ff52, l.ff53, l.ff54, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );(l.fe72, l.fe73, l.fe74, l.fe75, l.fe76, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe6d, l.fe6e, l.fe6f, l.fe70, l.fe71, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.ff85, l.ff86, l.ff87, l.ff88, l.ff89, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );(l.ff80, l.ff81, l.ff82, l.ff83, l.ff84, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );l.ff26 = 0.0;l.ff25 = 0.0;l.f1cb2 = 0.0;l.f1cb1 = 0.0;l.f47c = 0.0;l.f47b = 0.0;(l.f1cb8, l.f1cb9, l.f1cba, l.f1cbb, l.f1cbc, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1cb3, l.f1cb4, l.f1cb5, l.f1cb6, l.f1cb7, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f482, l.f483, l.f484, l.f485, l.f486, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f47d, l.f47e, l.f47f, l.f480, l.f481, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1cc2, l.f1cc3, l.f1cc4, l.f1cc5, l.f1cc6, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1cbd, l.f1cbe, l.f1cbf, l.f1cc0, l.f1cc1, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f48c, l.f48d, l.f48e, l.f48f, l.f490, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f487, l.f488, l.f489, l.f48a, l.f48b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f1e35 = 1.0;l.f1e33 = 1.0;l.f1e3d = 1.0;l.f1e3b = 1.0;l.f1e39 = 1.0;l.f1e37 = 1.0;(l.ff20, l.ff21, l.ff22, l.ff23, l.ff24, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.ff43, l.ff44, l.ff45, l.ff46, l.ff47, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fcc8, l.fcc9, l.fcca, l.fccb, l.fccc, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fcd2, l.fcd3, l.fcd4, l.fcd5, l.fcd6, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fcdc, l.fcdd, l.fcde, l.fcdf, l.fce0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fce6, l.fce7, l.fce8, l.fce9, l.fcea, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fcf0, l.fcf1, l.fcf2, l.fcf3, l.fcf4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f17f3 = 0.0;(l.f17f4, l.f17f5, l.f17f6, l.f17f7, l.f17f8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17f9, l.f17fa, l.f17fb, l.f17fc, l.f17fd, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f1e3f = 0.0;(l.f7f, l.f80, l.f81, l.f82, l.f83, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );let tea: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };l.f962 = tea;let teb: f64 = (l.fd40 * l.f1b);let tec: f64 = if teb > 0.0 { 1.0 } else { 0.0 };l.f964 = tec;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f962 != 0.0) && (l.f964 != 0.0)) {let ted: f64 = (l.fd40 * l.f1b);let tee: f64 = (p.p839 / ted);let tef: f64 = (tee + 1.0);let tf0: f64 = (tef).ln();let tf1: f64 = (l.f10d8 * tf0);l.f1a32 = tf1;}
        if ((l.f962 != 0.0) && (l.f964 == 0.0)) {l.f1a32 = 100000000.0;}
        let tf2: f64 = (l.fd4e * l.ff0f);let tf3: f64 = if tf2 > 0.0 { 1.0 } else { 0.0 };l.f966 = tf3;
        if ((l.f962 != 0.0) && (l.f966 != 0.0)) {let tf4: f64 = (l.fd4e * l.ff0f);let tf5: f64 = (p.p839 / tf4);let tf6: f64 = (tf5 + 1.0);let tf7: f64 = (tf6).ln();let tf8: f64 = (l.f10d8 * tf7);l.f1a36 = tf8;}
        if ((l.f962 != 0.0) && (l.f966 == 0.0)) {l.f1a36 = 100000000.0;}
        let tf9: f64 = (l.fd44 * l.fefb);let tfa: f64 = if tf9 > 0.0 { 1.0 } else { 0.0 };l.f968 = tfa;
        if ((l.f962 != 0.0) && (l.f968 != 0.0)) {let tfb: f64 = (l.fd44 * l.fefb);let tfc: f64 = (p.p839 / tfb);let tfd: f64 = (tfc + 1.0);let tfe: f64 = (tfd).ln();let tff: f64 = (l.f10d8 * tfe);l.f1a34 = tff;}
        if ((l.f962 != 0.0) && (l.f968 == 0.0)) {l.f1a34 = 100000000.0;}
        if (l.f962 != 0.0) {let t100: f64 = (l.f1a32).min(l.f1a36);let t101: f64 = (t100).min(l.f1a34);l.f1a30 = t101;}
        let t102: f64 = (l.f1a30 * l.f10da);let t103: f64 = (t102).abs();let t104: f64 = if t103 < 230.25850929940458 { 1.0 } else { 0.0 };l.f973 = t104;
        if ((l.f962 != 0.0) && (l.f973 != 0.0)) {let t105: f64 = (l.f1a30 * l.f10da);let t106: f64 = (t105).exp();l.f479 = t106;}
        let t107: f64 = (l.f1a30 * l.f10da);let t108: f64 = if t107 < 0.0 { 1.0 } else { 0.0 };l.f977 = t108;
        if (((l.f962 != 0.0) && (l.f973 == 0.0)) && (l.f977 != 0.0)) {let t109: f64 = (-230.25850929940458);let t10a: f64 = (l.f1a30 * l.f10da);let t10b: f64 = (t109 - t10a);let t10c: f64 = (-230.25850929940458);let t10d: f64 = (l.f1a30 * l.f10da);let t10e: f64 = (t10c - t10d);let t10f: f64 = (-230.25850929940458);let t110: f64 = (l.f1a30 * l.f10da);let t111: f64 = (t10f - t110);let t112: f64 = (t111 * 0.3333333333333333);let t113: f64 = (1.0 + t112);let t114: f64 = (t10e * t113);let t115: f64 = (0.5 * t114);let t116: f64 = (1.0 + t115);let t117: f64 = (t10b * t116);let t118: f64 = (1.0 + t117);let t119: f64 = (1e-100 / t118);l.f479 = t119;}
        if (((l.f962 != 0.0) && (l.f973 == 0.0)) && (l.f977 == 0.0)) {let t11a: f64 = (l.f1a30 * l.f10da);let t11b: f64 = (t11a - 230.25850929940458);let t11c: f64 = (l.f1a30 * l.f10da);let t11d: f64 = (t11c - 230.25850929940458);let t11e: f64 = (l.f1a30 * l.f10da);let t11f: f64 = (t11e - 230.25850929940458);let t120: f64 = (t11f * 0.3333333333333333);let t121: f64 = (1.0 + t120);let t122: f64 = (t11d * t121);let t123: f64 = (0.5 * t122);let t124: f64 = (1.0 + t123);let t125: f64 = (t11b * t124);let t126: f64 = (1.0 + t125);let t127: f64 = (1e100 * t126);l.f479 = t127;}
        if (l.f962 != 0.0) {l.f18ed = l.f18ec;l.f192b = l.f192a;l.f18f5 = l.f18f4;l.f1024 = p.p848;l.f1144 = p.p849;l.f105b = p.p850;l.f18ef = p.p845;l.f192d = p.p846;l.f18fb = p.p847;}
        let t128: f64 = if l.f1b == 0.0 { 1.0 } else { 0.0 };l.f979 = t128;
        if ((l.f962 != 0.0) && (l.f979 != 0.0)) {let t129: f64 = (l.f192a + l.f18f4);l.f18ed = t129;let t12a: f64 = (p.p849).min(p.p850);let t12b: f64 = (0.9 * t12a);l.f1024 = t12b;let t12c: f64 = (p.p846 + p.p847);l.f18ef = t12c;}
        let t12d: f64 = if l.ff0f == 0.0 { 1.0 } else { 0.0 };l.f97b = t12d;
        if ((l.f962 != 0.0) && (l.f97b != 0.0)) {let t12e: f64 = (l.f18ec + l.f18f4);l.f192b = t12e;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f962 != 0.0) && (l.f97b != 0.0)) {let t12f: f64 = (p.p848).min(p.p850);let t130: f64 = (0.9 * t12f);l.f1144 = t130;let t131: f64 = (p.p845 + p.p847);l.f192d = t131;}
        let t132: f64 = if l.fefb == 0.0 { 1.0 } else { 0.0 };l.f97d = t132;
        if ((l.f962 != 0.0) && (l.f97d != 0.0)) {let t133: f64 = (l.f18ec + l.f192a);l.f18f5 = t133;let t134: f64 = (p.p848).min(p.p849);let t135: f64 = (0.9 * t134);l.f105b = t135;let t136: f64 = (p.p845 + p.p846);l.f18fb = t136;}
        if (l.f962 != 0.0) {let t137: f64 = (l.f18ed).min(l.f192b);let t138: f64 = (t137).min(l.f18f5);l.f1912 = t138;let t139: f64 = (l.f1912 * 0.1);l.f195a = t139;let t13a: f64 = (l.f1024).max(l.f1144);let t13b: f64 = (t13a).max(l.f105b);l.f110a = t13b;let t13c: f64 = (-1.0);let t13d: f64 = (t13c / l.f110a);let t13e: f64 = (2.0_f64).powf(t13d);let t13f: f64 = (1.0 - t13e);let t140: f64 = (l.f1912 * t13f);l.f19b2 = t140;let t141: f64 = (l.f18ef).min(l.f192d);let t142: f64 = (t141).min(l.f18fb);let t143: f64 = (t142 - 0.05);l.f18e3 = t143;}
        let t144: f64 = (l.fd41 * l.f17);let t145: f64 = if t144 > 0.0 { 1.0 } else { 0.0 };l.f97f = t145;
        if ((l.f962 != 0.0) && (l.f97f != 0.0)) {let t146: f64 = (l.fd41 * l.f17);let t147: f64 = (p.p839 / t146);let t148: f64 = (t147 + 1.0);let t149: f64 = (t148).ln();let t14a: f64 = (l.f10d8 * t149);l.f1a32 = t14a;}
        if ((l.f962 != 0.0) && (l.f97f == 0.0)) {l.f1a32 = 100000000.0;}
        let t14b: f64 = (l.fd4f * l.ff0b);let t14c: f64 = if t14b > 0.0 { 1.0 } else { 0.0 };l.f981 = t14c;
        if ((l.f962 != 0.0) && (l.f981 != 0.0)) {let t14d: f64 = (l.fd4f * l.ff0b);let t14e: f64 = (p.p839 / t14d);let t14f: f64 = (t14e + 1.0);let t150: f64 = (t14f).ln();let t151: f64 = (l.f10d8 * t150);l.f1a36 = t151;}
        if ((l.f962 != 0.0) && (l.f981 == 0.0)) {l.f1a36 = 100000000.0;}
        let t152: f64 = (l.fd45 * l.fef7);let t153: f64 = if t152 > 0.0 { 1.0 } else { 0.0 };l.f983 = t153;
        if ((l.f962 != 0.0) && (l.f983 != 0.0)) {let t154: f64 = (l.fd45 * l.fef7);let t155: f64 = (p.p839 / t154);let t156: f64 = (t155 + 1.0);let t157: f64 = (t156).ln();let t158: f64 = (l.f10d8 * t157);l.f1a34 = t158;}
        if ((l.f962 != 0.0) && (l.f983 == 0.0)) {l.f1a34 = 100000000.0;}
        if (l.f962 != 0.0) {let t159: f64 = (l.f1a32).min(l.f1a36);let t15a: f64 = (t159).min(l.f1a34);l.f1a2e = t15a;}
        let t15b: f64 = (l.f1a2e * l.f10da);let t15c: f64 = (t15b).abs();let t15d: f64 = if t15c < 230.25850929940458 { 1.0 } else { 0.0 };l.f985 = t15d;
        if ((l.f962 != 0.0) && (l.f985 != 0.0)) {let t15e: f64 = (l.f1a2e * l.f10da);let t15f: f64 = (t15e).exp();l.f477 = t15f;}
        let t160: f64 = (l.f1a2e * l.f10da);let t161: f64 = if t160 < 0.0 { 1.0 } else { 0.0 };l.f987 = t161;
        if (((l.f962 != 0.0) && (l.f985 == 0.0)) && (l.f987 != 0.0)) {let t162: f64 = (-230.25850929940458);let t163: f64 = (l.f1a2e * l.f10da);let t164: f64 = (t162 - t163);let t165: f64 = (-230.25850929940458);let t166: f64 = (l.f1a2e * l.f10da);let t167: f64 = (t165 - t166);let t168: f64 = (-230.25850929940458);let t169: f64 = (l.f1a2e * l.f10da);let t16a: f64 = (t168 - t169);let t16b: f64 = (t16a * 0.3333333333333333);let t16c: f64 = (1.0 + t16b);let t16d: f64 = (t167 * t16c);let t16e: f64 = (0.5 * t16d);let t16f: f64 = (1.0 + t16e);let t170: f64 = (t164 * t16f);let t171: f64 = (1.0 + t170);let t172: f64 = (1e-100 / t171);l.f477 = t172;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        l: &mut StampLocals,
    ) {
        if (((l.f962 != 0.0) && (l.f985 == 0.0)) && (l.f987 == 0.0)) {let t173: f64 = (l.f1a2e * l.f10da);let t174: f64 = (t173 - 230.25850929940458);let t175: f64 = (l.f1a2e * l.f10da);let t176: f64 = (t175 - 230.25850929940458);let t177: f64 = (l.f1a2e * l.f10da);let t178: f64 = (t177 - 230.25850929940458);let t179: f64 = (t178 * 0.3333333333333333);let t17a: f64 = (1.0 + t179);let t17b: f64 = (t176 * t17a);let t17c: f64 = (0.5 * t17b);let t17d: f64 = (1.0 + t17c);let t17e: f64 = (t174 * t17d);let t17f: f64 = (1.0 + t17e);let t180: f64 = (1e100 * t17f);l.f477 = t180;}
        if (l.f962 != 0.0) {l.f18ed = l.f18f1;l.f192b = l.f192f;l.f18f5 = l.f18fd;l.f1024 = l.f1026;l.f1144 = l.f1146;l.f105b = l.f1061;l.f18ef = l.f1914;l.f192d = l.f1924;l.f18fb = l.f191e;}
        let t181: f64 = if l.f17 == 0.0 { 1.0 } else { 0.0 };l.f989 = t181;
        if ((l.f962 != 0.0) && (l.f989 != 0.0)) {let t182: f64 = (l.f192f + l.f18fd);l.f18ed = t182;let t183: f64 = (l.f1146).min(l.f1061);let t184: f64 = (0.9 * t183);l.f1024 = t184;let t185: f64 = (l.f1924 + l.f191e);l.f18ef = t185;}
        let t186: f64 = if l.ff0b == 0.0 { 1.0 } else { 0.0 };l.f98b = t186;
        if ((l.f962 != 0.0) && (l.f98b != 0.0)) {let t187: f64 = (l.f18f1 + l.f18fd);l.f192b = t187;let t188: f64 = (l.f1026).min(l.f1061);let t189: f64 = (0.9 * t188);l.f1144 = t189;let t18a: f64 = (l.f1914 + l.f191e);l.f192d = t18a;}
        let t18b: f64 = if l.fef7 == 0.0 { 1.0 } else { 0.0 };l.f98d = t18b;
        if ((l.f962 != 0.0) && (l.f98d != 0.0)) {let t18c: f64 = (l.f18f1 + l.f192f);l.f18f5 = t18c;let t18d: f64 = (l.f1026).min(l.f1146);let t18e: f64 = (0.9 * t18d);l.f105b = t18e;let t18f: f64 = (l.f1914 + l.f1924);l.f18fb = t18f;}
        if (l.f962 != 0.0) {let t190: f64 = (l.f18ed).min(l.f192b);let t191: f64 = (t190).min(l.f18f5);l.f1910 = t191;let t192: f64 = (l.f1910 * 0.1);l.f1958 = t192;let t193: f64 = (l.f1024).max(l.f1144);let t194: f64 = (t193).max(l.f105b);l.f110a = t194;let t195: f64 = (-1.0);let t196: f64 = (t195 / l.f110a);let t197: f64 = (2.0_f64).powf(t196);let t198: f64 = (1.0 - t197);let t199: f64 = (l.f1910 * t198);l.f19b0 = t199;let t19a: f64 = (l.f18ef).min(l.f192d);let t19b: f64 = (t19a).min(l.f18fb);let t19c: f64 = (t19b - 0.05);l.f18e1 = t19c;}
        let t19d: f64 = if l.f16ce == 1.0 { 1.0 } else { 0.0 };l.f98f = t19d;
        if ((l.f962 != 0.0) && (l.f98f != 0.0)) {(l.f1e11, l.f1e1a, l.f1e1b, l.f1e1c, l.f1e1d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1702, l.f170a, l.f170b, l.f170c, l.f170d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f423, l.f42b, l.f42c, l.f42d, l.f42e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.fd39 = 0.0;l.f1e1e = 0.0;l.f1e48 = 0.0;l.f17fe = 0.0;l.f1a04 = 0.0;l.f1a0b = 0.0;l.f18da = 0.0;l.f18d3 = 0.0;(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.fd38 = 0.0;(l.fe77, l.fe7f, l.fe80, l.fe81, l.fe82, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f18e5 = 0.0;l.f1b23 = 0.0;l.f3cb = 0.0;l.f1b1d = 0.0;(l.f1ad8, l.f1ae1, l.f1ae2, l.f1ae3, l.f1ae4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe0, l.fe8, l.fe9, l.fea, l.feb, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe83, l.fe8b, l.fe8c, l.fe8d, l.fe8e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f14c, l.f155, l.f156, l.f157, l.f158, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1805, l.f180e, l.f180f, l.f1810, l.f1811, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f185f, l.f1868, l.f1869, l.f186a, l.f186b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1852, l.f185b, l.f185c, l.f185d, l.f185e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f962 != 0.0) && (l.f98f != 0.0)) {(l.f1656, l.f165f, l.f1660, l.f1661, l.f1662, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f186c, l.f1875, l.f1876, l.f1877, l.f1878, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1b03, l.f1b0b, l.f1b0c, l.f1b0d, l.f1b0e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1b29, l.f1b31, l.f1b32, l.f1b33, l.f1b34, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fecc, l.fed5, l.fed6, l.fed7, l.fed8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.ff11, l.ff1a, l.ff1b, l.ff1c, l.ff1d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.ff8a, l.ff93, l.ff94, l.ff95, l.ff96, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1c0c, l.f1c15, l.f1c16, l.f1c17, l.f1c18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f42f, l.f437, l.f438, l.f439, l.f43a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f57d, l.f585, l.f586, l.f587, l.f588, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fd27, l.fd2f, l.fd30, l.fd31, l.fd32, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f512, l.f51b, l.f51c, l.f51d, l.f51e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f4cb, l.f4d3, l.f4d4, l.f4d5, l.f4d6, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f531 = 0.4;l.f532 = 0.65;l.f530 = 0.8;let t19e: f64 = (-l.f531);let t19f: f64 = (t19e * p.p945);l.f189b = t19f;let t1a0: f64 = (-l.f532);let t1a1: f64 = (t1a0 * p.p945);l.f189c = t1a1;let t1a2: f64 = (-l.f530);let t1a3: f64 = (t1a2 * p.p945);l.f189d = t1a3;l.f189e = 0.1;l.f189f = 0.2;l.f18da = 0.0;l.f17fe = 0.0;}
        let t1a4: f64 = if (!(((l.f1b == 0.0) && (l.ff0f == 0.0)) && (l.fefb == 0.0))) { 1.0 } else { 0.0 };l.f993 = t1a4;let t1a5: f64 = if l.f189b < l.f1a30 { 1.0 } else { 0.0 };l.f994 = t1a5;let t1a6: f64 = (-0.5);let t1a7: f64 = (l.f189b * l.f10da);let t1a8: f64 = (t1a6 * t1a7);let t1a9: f64 = (t1a8).abs();let t1aa: f64 = if t1a9 < 230.25850929940458 { 1.0 } else { 0.0 };l.f995 = t1aa;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) && (l.f994 != 0.0)) && (l.f995 != 0.0)) {let t1ab: f64 = (-0.5);let t1ac: f64 = (l.f189b * l.f10da);let t1ad: f64 = (t1ab * t1ac);let t1ae: f64 = (t1ad).exp();l.f1e1e = t1ae;}
        let t1af: f64 = (-0.5);let t1b0: f64 = (l.f189b * l.f10da);let t1b1: f64 = (t1af * t1b0);let t1b2: f64 = if t1b1 < 0.0 { 1.0 } else { 0.0 };l.f996 = t1b2;
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) && (l.f994 != 0.0)) && (l.f995 == 0.0)) && (l.f996 != 0.0)) {let t1b3: f64 = (-230.25850929940458);let t1b4: f64 = (-0.5);let t1b5: f64 = (l.f189b * l.f10da);let t1b6: f64 = (t1b4 * t1b5);let t1b7: f64 = (t1b3 - t1b6);let t1b8: f64 = (-230.25850929940458);let t1b9: f64 = (-0.5);let t1ba: f64 = (l.f189b * l.f10da);let t1bb: f64 = (t1b9 * t1ba);let t1bc: f64 = (t1b8 - t1bb);let t1bd: f64 = (-230.25850929940458);let t1be: f64 = (-0.5);let t1bf: f64 = (l.f189b * l.f10da);let t1c0: f64 = (t1be * t1bf);let t1c1: f64 = (t1bd - t1c0);let t1c2: f64 = (t1c1 * 0.3333333333333333);let t1c3: f64 = (1.0 + t1c2);let t1c4: f64 = (t1bc * t1c3);let t1c5: f64 = (0.5 * t1c4);let t1c6: f64 = (1.0 + t1c5);let t1c7: f64 = (t1b7 * t1c6);let t1c8: f64 = (1.0 + t1c7);let t1c9: f64 = (1e-100 / t1c8);l.f1e1e = t1c9;}
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) && (l.f994 != 0.0)) && (l.f995 == 0.0)) && (l.f996 == 0.0)) {let t1ca: f64 = (-0.5);let t1cb: f64 = (l.f189b * l.f10da);let t1cc: f64 = (t1ca * t1cb);let t1cd: f64 = (t1cc - 230.25850929940458);let t1ce: f64 = (-0.5);let t1cf: f64 = (l.f189b * l.f10da);let t1d0: f64 = (t1ce * t1cf);let t1d1: f64 = (t1d0 - 230.25850929940458);let t1d2: f64 = (-0.5);let t1d3: f64 = (l.f189b * l.f10da);let t1d4: f64 = (t1d2 * t1d3);let t1d5: f64 = (t1d4 - 230.25850929940458);let t1d6: f64 = (t1d5 * 0.3333333333333333);let t1d7: f64 = (1.0 + t1d6);let t1d8: f64 = (t1d1 * t1d7);let t1d9: f64 = (0.5 * t1d8);let t1da: f64 = (1.0 + t1d9);let t1db: f64 = (t1cd * t1da);let t1dc: f64 = (1.0 + t1db);let t1dd: f64 = (1e100 * t1dc);l.f1e1e = t1dd;}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) && (l.f994 != 0.0)) {let t1de: f64 = (1.0 / l.f1e1e);l.f1e48 = t1de;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) && (l.f994 != 0.0)) {let t1df: f64 = (l.f1e48 * l.f1e48);l.fd39 = t1df;}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) && (l.f994 == 0.0)) {let t1e0: f64 = (l.f189b - l.f1a30);let t1e1: f64 = (t1e0 * l.f10da);let t1e2: f64 = (1.0 + t1e1);let t1e3: f64 = (t1e2 * l.f479);l.fd39 = t1e3;let t1e4: f64 = (l.fd39).sqrt();l.f1e48 = t1e4;let t1e5: f64 = (1.0 / l.f1e48);l.f1e1e = t1e5;}
        if (((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) {let t1e6: f64 = (l.fd39 - 1.0);l.fd39 = t1e6;}
        let t1e7: f64 = if l.f189b > 0.0 { 1.0 } else { 0.0 };l.f997 = t1e7;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) && (l.f997 != 0.0)) {let t1e8: f64 = (2.0 + l.f1e1e);let t1e9: f64 = (l.f1e1e + 1.0);let t1ea: f64 = (l.f1e1e + 3.0);let t1eb: f64 = (t1e9 * t1ea);let t1ec: f64 = (t1eb).sqrt();let t1ed: f64 = (t1e8 + t1ec);let t1ee: f64 = (t1ed).ln();let t1ef: f64 = (l.f10d8 * t1ee);let t1f0: f64 = (2.0 * t1ef);l.f17fe = t1f0;}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) && (l.f997 == 0.0)) {let t1f1: f64 = (-l.f189b);let t1f2: f64 = (2.0 * l.f1e48);let t1f3: f64 = (t1f2 + 1.0);let t1f4: f64 = (1.0 + l.f1e48);let t1f5: f64 = (3.0 * l.f1e48);let t1f6: f64 = (1.0 + t1f5);let t1f7: f64 = (t1f4 * t1f6);let t1f8: f64 = (t1f7).sqrt();let t1f9: f64 = (t1f3 + t1f8);let t1fa: f64 = (t1f9).ln();let t1fb: f64 = (l.f10d8 * t1fa);let t1fc: f64 = (2.0 * t1fb);let t1fd: f64 = (t1f1 + t1fc);l.f17fe = t1fd;}
        if (((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f993 != 0.0)) {let t1fe: f64 = (l.f1912 - l.f17fe);l.f1a04 = t1fe;let t1ff: f64 = (l.f189b + l.f1a04);let t200: f64 = (l.f189b - l.f1a04);let t201: f64 = (l.f189b - l.f1a04);let t202: f64 = (t200 * t201);let t203: f64 = (4.0 * l.f10d8);let t204: f64 = (t203 * l.f10d8);let t205: f64 = (t202 + t204);let t206: f64 = (t205).sqrt();let t207: f64 = (t1ff - t206);let t208: f64 = (0.5 * t207);l.f1a0b = t208;let t209: f64 = (l.f189b + l.f18e3);let t20a: f64 = (l.f189b - l.f18e3);let t20b: f64 = (l.f189b - l.f18e3);let t20c: f64 = (t20a * t20b);let t20d: f64 = (4.0 * l.f10dc);let t20e: f64 = (t20d * l.f10dc);let t20f: f64 = (t20c + t20e);let t210: f64 = (t20f).sqrt();let t211: f64 = (t209 - t210);let t212: f64 = (0.5 * t211);l.f18da = t212;let t213: f64 = l.f189b;let t214: f64 = l.f189b;let t215: f64 = l.f189b;let t216: f64 = (t214 * t215);let t217: f64 = (4.0 * 1e-6);let t218: f64 = (t217 * 1e-6);let t219: f64 = (t216 + t218);let t21a: f64 = (t219).sqrt();let t21b: f64 = (t213 - t21a);let t21c: f64 = (0.5 * t21b);l.f18d3 = t21c;}
        let t21d: f64 = if l.f1b == 0.0 { 1.0 } else { 0.0 };l.f998 = t21d;
        if (((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 != 0.0)) {(l.fda1, l.fda9, l.fdaa, l.fdab, l.fdac, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) {let t21e: f64 = (l.fd40 * l.fd39);l.fd38 = t21e;}
        let t21f: f64 = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };l.f999 = t21f;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f999 != 0.0)) {(l.fe77, l.fe7f, l.fe80, l.fe81, l.fe82, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f999 == 0.0)) {let t220: f64 = (l.f18ec - l.f1a0b);l.f18e5 = t220;let t221: f64 = (l.f17fe / l.f18e5);let t222: f64 = (1.0 - t221);let t223: f64 = (t222).sqrt();let t224: f64 = (1.0 - t223);l.f1b23 = t224;}
        let t225: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };l.f99a = t225;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f999 == 0.0)) && (l.f99a != 0.0)) {l.f3cb = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f999 == 0.0)) && (l.f99a == 0.0)) {let t226: f64 = (l.f1b23 * l.f1b23);let t227: f64 = (l.f1b23).ln();let t228: f64 = (t226 * t227);let t229: f64 = (1.0 - l.f1b23);let t22a: f64 = (t228 / t229);let t22b: f64 = (t22a + l.f1b23);let t22c: f64 = (2.0 * p.p848);let t22d: f64 = (1.0 - t22c);let t22e: f64 = (t22b * t22d);l.f3cb = t22e;}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f999 == 0.0)) {let t22f: f64 = (l.f1b23 + l.f3cb);l.f1b1d = t22f;}
        let t230: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };l.f99b = t230;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f999 == 0.0)) && (l.f99b != 0.0)) {let t231: f64 = (l.f18e5 * l.f1916);let t232: f64 = (t231).sqrt();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t232, 0.0, 0.0, 0.0, 0.0, );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f999 == 0.0)) && (l.f99b == 0.0)) {let t233: f64 = (l.f18e5 * l.f1916);let t234: f64 = (t233).powf(p.p848);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t234, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f999 == 0.0)) {let t235: f64 = (l.f1ae5 * l.f17c6);(l.f1ad8, l.f1ae1, l.f1ae2, l.f1ae3, l.f1ae4, ) = (t235, (l.f1ae5 * l.f17cf), (l.f1ae5 * l.f17d0), (l.f1ae5 * l.f17d1), (l.f1ae5 * l.f17d2), );let t236: f64 = (l.f1e48 - 1.0);let t237: f64 = (t236 * l.f1ad8);let t238: f64 = (l.f559 * t237);(l.fe0, l.fe8, l.fe9, l.fea, l.feb, ) = (t238, (l.f559 * (t236 * l.f1ae1)), (l.f559 * (t236 * l.f1ae2)), (l.f559 * (t236 * l.f1ae3)), (l.f559 * (t236 * l.f1ae4)), );let t239: f64 = (l.fe0 * l.f1b1d);let t23a: f64 = (p.p857 * t239);(l.fe77, l.fe7f, l.fe80, l.fe81, l.fe82, ) = (t23a, (p.p857 * (l.fe8 * l.f1b1d)), (p.p857 * (l.fe9 * l.f1b1d)), (p.p857 * (l.fea * l.f1b1d)), (p.p857 * (l.feb * l.f1b1d)), );}
        let t23b: f64 = if p.p862 == 0.0 { 1.0 } else { 0.0 };l.f99c = t23b;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c != 0.0)) {(l.fe83, l.fe8b, l.fe8c, l.fe8d, l.fe8e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) {let t23c: f64 = (l.f1ad8 * l.fff6);let t23d: f64 = (t23c / l.f18e5);let t23e: f64 = (l.f159 * t23d);(l.f14c, l.f155, l.f156, l.f157, l.f158, ) = (t23e, (l.f159 * ((l.f1ae1 * l.fff6) / l.f18e5)), (l.f159 * ((l.f1ae2 * l.fff6) / l.f18e5)), (l.f159 * ((l.f1ae3 * l.fff6) / l.f18e5)), (l.f159 * ((l.f1ae4 * l.fff6) / l.f18e5)), );let t23f: f64 = (0.666666666666667 * l.fec);let t240: f64 = (t23f / l.f14c);(l.f1805, l.f180e, l.f180f, l.f1810, l.f1811, ) = (t240, (-((t23f * l.f155) / (l.f14c * l.f14c))), (-((t23f * l.f156) / (l.f14c * l.f14c))), (-((t23f * l.f157) / (l.f14c * l.f14c))), (-((t23f * l.f158) / (l.f14c * l.f14c))), );let t241: f64 = (l.f1805 * l.f1805);(l.f185f, l.f1868, l.f1869, l.f186a, l.f186b, ) = (t241, ((l.f180e * l.f1805) + (l.f1805 * l.f180e)), ((l.f180f * l.f1805) + (l.f1805 * l.f180f)), ((l.f1810 * l.f1805) + (l.f1805 * l.f1810)), ((l.f1811 * l.f1805) + (l.f1805 * l.f1811)), );let t242: f64 = (l.f185f * l.f185f);let t243: f64 = (l.f185f * l.f185f);let t244: f64 = (t243 + 1.0);let t245: f64 = (t242 / t244);let t246: f64 = (t245).sqrt();(l.f1852, l.f185b, l.f185c, l.f185d, l.f185e, ) = (t246, ((((((l.f1868 * l.f185f) + (l.f185f * l.f1868)) * t244) - (t242 * ((l.f1868 * l.f185f) + (l.f185f * l.f1868)))) / (t244 * t244)) / (2.0 * t246)), ((((((l.f1869 * l.f185f) + (l.f185f * l.f1869)) * t244) - (t242 * ((l.f1869 * l.f185f) + (l.f185f * l.f1869)))) / (t244 * t244)) / (2.0 * t246)), ((((((l.f186a * l.f185f) + (l.f185f * l.f186a)) * t244) - (t242 * ((l.f186a * l.f185f) + (l.f185f * l.f186a)))) / (t244 * t244)) / (2.0 * t246)), ((((((l.f186b * l.f185f) + (l.f185f * l.f186b)) * t244) - (t242 * ((l.f186b * l.f185f) + (l.f185f * l.f186b)))) / (t244 * t244)) / (2.0 * t246)), );let t247: f64 = (l.f1852).sqrt();(l.f1656, l.f165f, l.f1660, l.f1661, l.f1662, ) = (t247, (l.f185b / (2.0 * t247)), (l.f185c / (2.0 * t247)), (l.f185d / (2.0 * t247)), (l.f185e / (2.0 * t247)), );let t248: f64 = (l.f1852 * l.f1656);(l.f186c, l.f1875, l.f1876, l.f1877, l.f1878, ) = (t248, ((l.f185b * l.f1656) + (l.f1852 * l.f165f)), ((l.f185c * l.f1656) + (l.f1852 * l.f1660)), ((l.f185d * l.f1656) + (l.f1852 * l.f1661)), ((l.f185e * l.f1656) + (l.f1852 * l.f1662)), );}
        let t249: f64 = (-p.p848);let t24a: f64 = (t249 * l.f1006);let t24b: f64 = (-1.0);let t24c: f64 = if t24a == t24b { 1.0 } else { 0.0 };l.f99d = t24c;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f99d != 0.0)) {let t24d: f64 = (l.f14c * l.f186c);let t24e: f64 = (1.0 + t24d);let t24f: f64 = (1.0 / t24e);(l.f1b03, l.f1b0b, l.f1b0c, l.f1b0d, l.f1b0e, ) = (t24f, (-(((l.f155 * l.f186c) + (l.f14c * l.f1875)) / (t24e * t24e))), (-(((l.f156 * l.f186c) + (l.f14c * l.f1876)) / (t24e * t24e))), (-(((l.f157 * l.f186c) + (l.f14c * l.f1877)) / (t24e * t24e))), (-(((l.f158 * l.f186c) + (l.f14c * l.f1878)) / (t24e * t24e))), );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f99d == 0.0)) {let t250: f64 = (l.f14c * l.f186c);let t251: f64 = (1.0 + t250);let t252: f64 = (-p.p848);let t253: f64 = (t252 * l.f1006);let t254: f64 = (t251).powf(t253);(l.f1b03, l.f1b0b, l.f1b0c, l.f1b0d, l.f1b0e, ) = (t254, if 0.0 == 0.0 && ((t253) as f64).is_finite() && ((t253) as f64).fract() == 0.0 { if t253 == 0.0 { 0.0 } else { (t253 * ((t251).powf(t253 - 1.0) * ((l.f155 * l.f186c) + (l.f14c * l.f1875)))) } } else { (t254 * (t253 * (((l.f155 * l.f186c) + (l.f14c * l.f1875)) / t251))) }, if 0.0 == 0.0 && ((t253) as f64).is_finite() && ((t253) as f64).fract() == 0.0 { if t253 == 0.0 { 0.0 } else { (t253 * ((t251).powf(t253 - 1.0) * ((l.f156 * l.f186c) + (l.f14c * l.f1876)))) } } else { (t254 * (t253 * (((l.f156 * l.f186c) + (l.f14c * l.f1876)) / t251))) }, if 0.0 == 0.0 && ((t253) as f64).is_finite() && ((t253) as f64).fract() == 0.0 { if t253 == 0.0 { 0.0 } else { (t253 * ((t251).powf(t253 - 1.0) * ((l.f157 * l.f186c) + (l.f14c * l.f1877)))) } } else { (t254 * (t253 * (((l.f157 * l.f186c) + (l.f14c * l.f1877)) / t251))) }, if 0.0 == 0.0 && ((t253) as f64).is_finite() && ((t253) as f64).fract() == 0.0 { if t253 == 0.0 { 0.0 } else { (t253 * ((t251).powf(t253 - 1.0) * ((l.f158 * l.f186c) + (l.f14c * l.f1878)))) } } else { (t254 * (t253 * (((l.f158 * l.f186c) + (l.f14c * l.f1878)) / t251))) }, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) {let t255: f64 = (l.f1b1d * l.f1b03);let t256: f64 = (l.f1b1d + l.f1b03);let t257: f64 = (t255 / t256);(l.f1b29, l.f1b31, l.f1b32, l.f1b33, l.f1b34, ) = (t257, ((((l.f1b1d * l.f1b0b) * t256) - (t255 * l.f1b0b)) / (t256 * t256)), ((((l.f1b1d * l.f1b0c) * t256) - (t255 * l.f1b0c)) / (t256 * t256)), ((((l.f1b1d * l.f1b0d) * t256) - (t255 * l.f1b0d)) / (t256 * t256)), ((((l.f1b1d * l.f1b0e) * t256) - (t255 * l.f1b0e)) / (t256 * t256)), );let t258: f64 = (l.f14c / l.f1656);let t259: f64 = (0.375 * t258);let t25a: f64 = (t259).sqrt();(l.fecc, l.fed5, l.fed6, l.fed7, l.fed8, ) = (t25a, ((0.375 * (((l.f155 * l.f1656) - (l.f14c * l.f165f)) / (l.f1656 * l.f1656))) / (2.0 * t25a)), ((0.375 * (((l.f156 * l.f1656) - (l.f14c * l.f1660)) / (l.f1656 * l.f1656))) / (2.0 * t25a)), ((0.375 * (((l.f157 * l.f1656) - (l.f14c * l.f1661)) / (l.f1656 * l.f1656))) / (2.0 * t25a)), ((0.375 * (((l.f158 * l.f1656) - (l.f14c * l.f1662)) / (l.f1656 * l.f1656))) / (2.0 * t25a)), );let t25b: f64 = (l.f1805 * l.f1656);let t25c: f64 = (2.0 * t25b);let t25d: f64 = (t25c - l.f1852);(l.ff11, l.ff1a, l.ff1b, l.ff1c, l.ff1d, ) = (t25d, ((2.0 * ((l.f180e * l.f1656) + (l.f1805 * l.f165f))) - l.f185b), ((2.0 * ((l.f180f * l.f1656) + (l.f1805 * l.f1660))) - l.f185c), ((2.0 * ((l.f1810 * l.f1656) + (l.f1805 * l.f1661))) - l.f185d), ((2.0 * ((l.f1811 * l.f1656) + (l.f1805 * l.f1662))) - l.f185e), );let t25e: f64 = (l.fec * l.f1805);let t25f: f64 = (t25e * l.f1656);let t260: f64 = (l.fec * l.f1852);let t261: f64 = (t25f - t260);let t262: f64 = (l.f14c * l.f186c);let t263: f64 = (0.5 * t262);let t264: f64 = (t261 + t263);(l.ff8a, l.ff93, l.ff94, l.ff95, l.ff96, ) = (t264, (((((l.fec * l.f180e) * l.f1656) + (t25e * l.f165f)) - (l.fec * l.f185b)) + (0.5 * ((l.f155 * l.f186c) + (l.f14c * l.f1875)))), (((((l.fec * l.f180f) * l.f1656) + (t25e * l.f1660)) - (l.fec * l.f185c)) + (0.5 * ((l.f156 * l.f186c) + (l.f14c * l.f1876)))), (((((l.fec * l.f1810) * l.f1656) + (t25e * l.f1661)) - (l.fec * l.f185d)) + (0.5 * ((l.f157 * l.f186c) + (l.f14c * l.f1877)))), (((((l.fec * l.f1811) * l.f1656) + (t25e * l.f1662)) - (l.fec * l.f185e)) + (0.5 * ((l.f158 * l.f186c) + (l.f14c * l.f1878)))), );let t265: f64 = (l.ff11 - 1.0);let t266: f64 = (t265 * l.fecc);(l.f1c0c, l.f1c15, l.f1c16, l.f1c17, l.f1c18, ) = (t266, ((l.ff1a * l.fecc) + (t265 * l.fed5)), ((l.ff1b * l.fecc) + (t265 * l.fed6)), ((l.ff1c * l.fecc) + (t265 * l.fed7)), ((l.ff1d * l.fecc) + (t265 * l.fed8)), );let t267: f64 = (l.f1c0c * l.f1c0c);(l.f1e11, l.f1e1a, l.f1e1b, l.f1e1c, l.f1e1d, ) = (t267, ((l.f1c15 * l.f1c0c) + (l.f1c0c * l.f1c15)), ((l.f1c16 * l.f1c0c) + (l.f1c0c * l.f1c16)), ((l.f1c17 * l.f1c0c) + (l.f1c0c * l.f1c17)), ((l.f1c18 * l.f1c0c) + (l.f1c0c * l.f1c18)), );}
        let t268: f64 = if l.f1c0c > 0.0 { 1.0 } else { 0.0 };l.f99e = t268;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f99e != 0.0)) {let t269: f64 = (l.f105a * l.f1c0c);let t26a: f64 = (1.0 + t269);let t26b: f64 = (1.0 / t26a);(l.f1702, l.f170a, l.f170b, l.f170c, l.f170d, ) = (t26b, (-((l.f105a * l.f1c15) / (t26a * t26a))), (-((l.f105a * l.f1c16) / (t26a * t26a))), (-((l.f105a * l.f1c17) / (t26a * t26a))), (-((l.f105a * l.f1c18) / (t26a * t26a))), );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f99e == 0.0)) {let t26c: f64 = (l.f105a * l.f1c0c);let t26d: f64 = (1.0 - t26c);let t26e: f64 = (1.0 / t26d);(l.f1702, l.f170a, l.f170b, l.f170c, l.f170d, ) = (t26e, (-((-(l.f105a * l.f1c15)) / (t26d * t26d))), (-((-(l.f105a * l.f1c16)) / (t26d * t26d))), (-((-(l.f105a * l.f1c17)) / (t26d * t26d))), (-((-(l.f105a * l.f1c18)) / (t26d * t26d))), );}
        let t26f: f64 = (-l.f1e11);let t270: f64 = (t26f + l.ff8a);let t271: f64 = (-230.25850929940458);let t272: f64 = if t270 > t271 { 1.0 } else { 0.0 };l.f99f = t272;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f99f != 0.0)) {let t273: f64 = (-l.f1e11);let t274: f64 = (t273 + l.ff8a);let t275: f64 = (t274).exp();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t275, (t275 * ((-l.f1e1a) + l.ff93)), (t275 * ((-l.f1e1b) + l.ff94)), (t275 * ((-l.f1e1c) + l.ff95)), (t275 * ((-l.f1e1d) + l.ff96)), );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f99f == 0.0)) {let t276: f64 = (-230.25850929940458);let t277: f64 = (-l.f1e11);let t278: f64 = (t277 + l.ff8a);let t279: f64 = (t276 - t278);let t27a: f64 = (-230.25850929940458);let t27b: f64 = (-l.f1e11);let t27c: f64 = (t27b + l.ff8a);let t27d: f64 = (t27a - t27c);let t27e: f64 = (-230.25850929940458);let t27f: f64 = (-l.f1e11);let t280: f64 = (t27f + l.ff8a);let t281: f64 = (t27e - t280);let t282: f64 = (t281 * 0.3333333333333333);let t283: f64 = (1.0 + t282);let t284: f64 = (t27d * t283);let t285: f64 = (0.5 * t284);let t286: f64 = (1.0 + t285);let t287: f64 = (t279 * t286);let t288: f64 = (1.0 + t287);let t289: f64 = (1e-100 / t288);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t289, (-((1e-100 * (((-((-l.f1e1a) + l.ff93)) * t286) + (t279 * (0.5 * (((-((-l.f1e1a) + l.ff93)) * t283) + (t27d * ((-((-l.f1e1a) + l.ff93)) * 0.3333333333333333))))))) / (t288 * t288))), (-((1e-100 * (((-((-l.f1e1b) + l.ff94)) * t286) + (t279 * (0.5 * (((-((-l.f1e1b) + l.ff94)) * t283) + (t27d * ((-((-l.f1e1b) + l.ff94)) * 0.3333333333333333))))))) / (t288 * t288))), (-((1e-100 * (((-((-l.f1e1c) + l.ff95)) * t286) + (t279 * (0.5 * (((-((-l.f1e1c) + l.ff95)) * t283) + (t27d * ((-((-l.f1e1c) + l.ff95)) * 0.3333333333333333))))))) / (t288 * t288))), (-((1e-100 * (((-((-l.f1e1d) + l.ff96)) * t286) + (t279 * (0.5 * (((-((-l.f1e1d) + l.ff96)) * t283) + (t27d * ((-((-l.f1e1d) + l.ff96)) * 0.3333333333333333))))))) / (t288 * t288))), );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) {let t28a: f64 = (0.29214664 * l.f1702);let t28b: f64 = (l.f1702 * l.f1702);let t28c: f64 = (l.f114 * t28b);let t28d: f64 = (t28a + t28c);let t28e: f64 = (l.f1702 * l.f1702);let t28f: f64 = (t28e * l.f1702);let t290: f64 = (l.f171 * t28f);let t291: f64 = (t28d + t290);let t292: f64 = (t291 * l.f17c6);(l.f423, l.f42b, l.f42c, l.f42d, l.f42e, ) = (t292, (((((0.29214664 * l.f170a) + (l.f114 * ((l.f170a * l.f1702) + (l.f1702 * l.f170a)))) + (l.f171 * ((((l.f170a * l.f1702) + (l.f1702 * l.f170a)) * l.f1702) + (t28e * l.f170a)))) * l.f17c6) + (t291 * l.f17cf)), (((((0.29214664 * l.f170b) + (l.f114 * ((l.f170b * l.f1702) + (l.f1702 * l.f170b)))) + (l.f171 * ((((l.f170b * l.f1702) + (l.f1702 * l.f170b)) * l.f1702) + (t28e * l.f170b)))) * l.f17c6) + (t291 * l.f17d0)), (((((0.29214664 * l.f170c) + (l.f114 * ((l.f170c * l.f1702) + (l.f1702 * l.f170c)))) + (l.f171 * ((((l.f170c * l.f1702) + (l.f1702 * l.f170c)) * l.f1702) + (t28e * l.f170c)))) * l.f17c6) + (t291 * l.f17d1)), (((((0.29214664 * l.f170d) + (l.f114 * ((l.f170d * l.f1702) + (l.f1702 * l.f170d)))) + (l.f171 * ((((l.f170d * l.f1702) + (l.f1702 * l.f170d)) * l.f1702) + (t28e * l.f170d)))) * l.f17c6) + (t291 * l.f17d2)), );}
        let t293: f64 = if l.f1c0c > 0.0 { 1.0 } else { 0.0 };l.f9a0 = t293;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f9a0 != 0.0)) {(l.f42f, l.f437, l.f438, l.f439, l.f43a, ) = (l.f423, l.f42b, l.f42c, l.f42d, l.f42e, );}
        let t294: f64 = (-230.25850929940458);let t295: f64 = if l.ff8a > t294 { 1.0 } else { 0.0 };l.f9a1 = t295;
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f9a0 == 0.0)) && (l.f9a1 != 0.0)) {let t296: f64 = (l.ff8a).exp();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t296, (t296 * l.ff93), (t296 * l.ff94), (t296 * l.ff95), (t296 * l.ff96), );}
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f9a0 == 0.0)) && (l.f9a1 == 0.0)) {let t297: f64 = (-230.25850929940458);let t298: f64 = (t297 - l.ff8a);let t299: f64 = (-230.25850929940458);let t29a: f64 = (t299 - l.ff8a);let t29b: f64 = (-230.25850929940458);let t29c: f64 = (t29b - l.ff8a);let t29d: f64 = (t29c * 0.3333333333333333);let t29e: f64 = (1.0 + t29d);let t29f: f64 = (t29a * t29e);let t2a0: f64 = (0.5 * t29f);let t2a1: f64 = (1.0 + t2a0);let t2a2: f64 = (t298 * t2a1);let t2a3: f64 = (1.0 + t2a2);let t2a4: f64 = (1e-100 / t2a3);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t2a4, (-((1e-100 * (((-l.ff93) * t2a1) + (t298 * (0.5 * (((-l.ff93) * t29e) + (t29a * ((-l.ff93) * 0.3333333333333333))))))) / (t2a3 * t2a3))), (-((1e-100 * (((-l.ff94) * t2a1) + (t298 * (0.5 * (((-l.ff94) * t29e) + (t29a * ((-l.ff94) * 0.3333333333333333))))))) / (t2a3 * t2a3))), (-((1e-100 * (((-l.ff95) * t2a1) + (t298 * (0.5 * (((-l.ff95) * t29e) + (t29a * ((-l.ff95) * 0.3333333333333333))))))) / (t2a3 * t2a3))), (-((1e-100 * (((-l.ff96) * t2a1) + (t298 * (0.5 * (((-l.ff96) * t29e) + (t29a * ((-l.ff96) * 0.3333333333333333))))))) / (t2a3 * t2a3))), );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) && (l.f9a0 == 0.0)) {let t2a5: f64 = (2.0 * l.f17c6);let t2a6: f64 = (t2a5 - l.f423);(l.f42f, l.f437, l.f438, l.f439, l.f43a, ) = (t2a6, ((2.0 * l.f17cf) - l.f42b), ((2.0 * l.f17d0) - l.f42c), ((2.0 * l.f17d1) - l.f42d), ((2.0 * l.f17d2) - l.f42e), );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f99c == 0.0)) {let t2a7: f64 = (1.772453850905516 * 0.5);let t2a8: f64 = (l.fec * l.f42f);let t2a9: f64 = (t2a8 / l.fecc);let t2aa: f64 = (t2a7 * t2a9);(l.f57d, l.f585, l.f586, l.f587, l.f588, ) = (t2aa, (t2a7 * ((((l.fec * l.f437) * l.fecc) - (t2a8 * l.fed5)) / (l.fecc * l.fecc))), (t2a7 * ((((l.fec * l.f438) * l.fecc) - (t2a8 * l.fed6)) / (l.fecc * l.fecc))), (t2a7 * ((((l.fec * l.f439) * l.fecc) - (t2a8 * l.fed7)) / (l.fecc * l.fecc))), (t2a7 * ((((l.fec * l.f43a) * l.fecc) - (t2a8 * l.fed8)) / (l.fecc * l.fecc))), );let t2ab: f64 = (l.fe0 * l.f57d);let t2ac: f64 = (t2ab * l.f1b29);let t2ad: f64 = (p.p862 * t2ac);(l.fe83, l.fe8b, l.fe8c, l.fe8d, l.fe8e, ) = (t2ad, (p.p862 * ((((l.fe8 * l.f57d) + (l.fe0 * l.f585)) * l.f1b29) + (t2ab * l.f1b31))), (p.p862 * ((((l.fe9 * l.f57d) + (l.fe0 * l.f586)) * l.f1b29) + (t2ab * l.f1b32))), (p.p862 * ((((l.fea * l.f57d) + (l.fe0 * l.f587)) * l.f1b29) + (t2ab * l.f1b33))), (p.p862 * ((((l.feb * l.f57d) + (l.fe0 * l.f588)) * l.f1b29) + (t2ab * l.f1b34))), );}
        let t2ae: f64 = if p.p868 == 0.0 { 1.0 } else { 0.0 };l.f9a2 = t2ae;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a2 != 0.0)) {(l.fd27, l.fd2f, l.fd30, l.fd31, l.fd32, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t2af: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };l.f9a3 = t2af;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a2 == 0.0)) && (l.f9a3 != 0.0)) {let t2b0: f64 = (p.p845 - l.f18da);let t2b1: f64 = (t2b0 * l.f1916);let t2b2: f64 = (t2b1).sqrt();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t2b2, 0.0, 0.0, 0.0, 0.0, );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a2 == 0.0)) && (l.f9a3 == 0.0)) {let t2b3: f64 = (p.p845 - l.f18da);let t2b4: f64 = (t2b3 * l.f1916);let t2b5: f64 = (t2b4).powf(p.p848);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t2b5, 0.0, 0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a2 == 0.0)) {let t2b6: f64 = (p.p845 - l.f18da);let t2b7: f64 = (t2b6 * l.f1aed);let t2b8: f64 = (t2b7 / l.f17c6);let t2b9: f64 = (l.f1006 * t2b8);(l.f512, l.f51b, l.f51c, l.f51d, l.f51e, ) = (t2b9, (l.f1006 * (-((t2b7 * l.f17cf) / (l.f17c6 * l.f17c6)))), (l.f1006 * (-((t2b7 * l.f17d0) / (l.f17c6 * l.f17c6)))), (l.f1006 * (-((t2b7 * l.f17d1) / (l.f17c6 * l.f17c6)))), (l.f1006 * (-((t2b7 * l.f17d2) / (l.f17c6 * l.f17c6)))), );}
        let t2ba: f64 = (-l.f4af);let t2bb: f64 = (t2ba / l.f512);let t2bc: f64 = (t2bb).abs();let t2bd: f64 = if t2bc < 230.25850929940458 { 1.0 } else { 0.0 };l.f9a4 = t2bd;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a2 == 0.0)) && (l.f9a4 != 0.0)) {let t2be: f64 = (-l.f4af);let t2bf: f64 = (t2be / l.f512);let t2c0: f64 = (t2bf).exp();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t2c0, (t2c0 * (-((t2be * l.f51b) / (l.f512 * l.f512)))), (t2c0 * (-((t2be * l.f51c) / (l.f512 * l.f512)))), (t2c0 * (-((t2be * l.f51d) / (l.f512 * l.f512)))), (t2c0 * (-((t2be * l.f51e) / (l.f512 * l.f512)))), );}
        let t2c1: f64 = (-l.f4af);let t2c2: f64 = (t2c1 / l.f512);let t2c3: f64 = if t2c2 < 0.0 { 1.0 } else { 0.0 };l.f9a5 = t2c3;
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a2 == 0.0)) && (l.f9a4 == 0.0)) && (l.f9a5 != 0.0)) {let t2c4: f64 = (-230.25850929940458);let t2c5: f64 = (-l.f4af);let t2c6: f64 = (t2c5 / l.f512);let t2c7: f64 = (t2c4 - t2c6);let t2c8: f64 = (-230.25850929940458);let t2c9: f64 = (-l.f4af);let t2ca: f64 = (t2c9 / l.f512);let t2cb: f64 = (t2c8 - t2ca);let t2cc: f64 = (-230.25850929940458);let t2cd: f64 = (-l.f4af);let t2ce: f64 = (t2cd / l.f512);let t2cf: f64 = (t2cc - t2ce);let t2d0: f64 = (t2cf * 0.3333333333333333);let t2d1: f64 = (1.0 + t2d0);let t2d2: f64 = (t2cb * t2d1);let t2d3: f64 = (0.5 * t2d2);let t2d4: f64 = (1.0 + t2d3);let t2d5: f64 = (t2c7 * t2d4);let t2d6: f64 = (1.0 + t2d5);let t2d7: f64 = (1e-100 / t2d6);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t2d7, (-((1e-100 * (((-(-((t2c5 * l.f51b) / (l.f512 * l.f512)))) * t2d4) + (t2c7 * (0.5 * (((-(-((t2c9 * l.f51b) / (l.f512 * l.f512)))) * t2d1) + (t2cb * ((-(-((t2cd * l.f51b) / (l.f512 * l.f512)))) * 0.3333333333333333))))))) / (t2d6 * t2d6))), (-((1e-100 * (((-(-((t2c5 * l.f51c) / (l.f512 * l.f512)))) * t2d4) + (t2c7 * (0.5 * (((-(-((t2c9 * l.f51c) / (l.f512 * l.f512)))) * t2d1) + (t2cb * ((-(-((t2cd * l.f51c) / (l.f512 * l.f512)))) * 0.3333333333333333))))))) / (t2d6 * t2d6))), (-((1e-100 * (((-(-((t2c5 * l.f51d) / (l.f512 * l.f512)))) * t2d4) + (t2c7 * (0.5 * (((-(-((t2c9 * l.f51d) / (l.f512 * l.f512)))) * t2d1) + (t2cb * ((-(-((t2cd * l.f51d) / (l.f512 * l.f512)))) * 0.3333333333333333))))))) / (t2d6 * t2d6))), (-((1e-100 * (((-(-((t2c5 * l.f51e) / (l.f512 * l.f512)))) * t2d4) + (t2c7 * (0.5 * (((-(-((t2c9 * l.f51e) / (l.f512 * l.f512)))) * t2d1) + (t2cb * ((-(-((t2cd * l.f51e) / (l.f512 * l.f512)))) * 0.3333333333333333))))))) / (t2d6 * t2d6))), );}
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a2 == 0.0)) && (l.f9a4 == 0.0)) && (l.f9a5 == 0.0)) {let t2d8: f64 = (-l.f4af);let t2d9: f64 = (t2d8 / l.f512);let t2da: f64 = (t2d9 - 230.25850929940458);let t2db: f64 = (-l.f4af);let t2dc: f64 = (t2db / l.f512);let t2dd: f64 = (t2dc - 230.25850929940458);let t2de: f64 = (-l.f4af);let t2df: f64 = (t2de / l.f512);let t2e0: f64 = (t2df - 230.25850929940458);let t2e1: f64 = (t2e0 * 0.3333333333333333);let t2e2: f64 = (1.0 + t2e1);let t2e3: f64 = (t2dd * t2e2);let t2e4: f64 = (0.5 * t2e3);let t2e5: f64 = (1.0 + t2e4);let t2e6: f64 = (t2da * t2e5);let t2e7: f64 = (1.0 + t2e6);let t2e8: f64 = (1e100 * t2e7);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t2e8, (1e100 * (((-((t2d8 * l.f51b) / (l.f512 * l.f512))) * t2e5) + (t2da * (0.5 * (((-((t2db * l.f51b) / (l.f512 * l.f512))) * t2e2) + (t2dd * ((-((t2de * l.f51b) / (l.f512 * l.f512))) * 0.3333333333333333))))))), (1e100 * (((-((t2d8 * l.f51c) / (l.f512 * l.f512))) * t2e5) + (t2da * (0.5 * (((-((t2db * l.f51c) / (l.f512 * l.f512))) * t2e2) + (t2dd * ((-((t2de * l.f51c) / (l.f512 * l.f512))) * 0.3333333333333333))))))), (1e100 * (((-((t2d8 * l.f51d) / (l.f512 * l.f512))) * t2e5) + (t2da * (0.5 * (((-((t2db * l.f51d) / (l.f512 * l.f512))) * t2e2) + (t2dd * ((-((t2de * l.f51d) / (l.f512 * l.f512))) * 0.3333333333333333))))))), (1e100 * (((-((t2d8 * l.f51e) / (l.f512 * l.f512))) * t2e5) + (t2da * (0.5 * (((-((t2db * l.f51e) / (l.f512 * l.f512))) * t2e2) + (t2dd * ((-((t2de * l.f51e) / (l.f512 * l.f512))) * 0.3333333333333333))))))), );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a2 == 0.0)) {let t2e9: f64 = (l.f189b * l.f512);let t2ea: f64 = (t2e9 * l.f512);let t2eb: f64 = (t2ea * l.f17c6);let t2ec: f64 = (p.p868 * t2eb);(l.fd27, l.fd2f, l.fd30, l.fd31, l.fd32, ) = (t2ec, (p.p868 * (((((l.f189b * l.f51b) * l.f512) + (t2e9 * l.f51b)) * l.f17c6) + (t2ea * l.f17cf))), (p.p868 * (((((l.f189b * l.f51c) * l.f512) + (t2e9 * l.f51c)) * l.f17c6) + (t2ea * l.f17d0))), (p.p868 * (((((l.f189b * l.f51d) * l.f512) + (t2e9 * l.f51d)) * l.f17c6) + (t2ea * l.f17d1))), (p.p868 * (((((l.f189b * l.f51e) * l.f512) + (t2e9 * l.f51e)) * l.f17c6) + (t2ea * l.f17d2))), );}
        let t2ed: f64 = if p.p877 > 1000.0 { 1.0 } else { 0.0 };l.f9a6 = t2ed;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a6 != 0.0)) {(l.f4cb, l.f4d3, l.f4d4, l.f4d5, l.f4d6, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
        let t2ee: f64 = (-l.f76);let t2ef: f64 = (t2ee * p.p877);let t2f0: f64 = if l.f18d3 > t2ef { 1.0 } else { 0.0 };l.f9a7 = t2f0;let t2f1: f64 = if p.p880 == 4.0 { 1.0 } else { 0.0 };l.f9a8 = t2f1;
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a6 == 0.0)) && (l.f9a7 != 0.0)) && (l.f9a8 != 0.0)) {let t2f2: f64 = (l.f18d3 * l.f1942);let t2f3: f64 = (l.f18d3 * l.f1942);let t2f4: f64 = (t2f2 * t2f3);let t2f5: f64 = (l.f18d3 * l.f1942);let t2f6: f64 = (t2f4 * t2f5);let t2f7: f64 = (l.f18d3 * l.f1942);let t2f8: f64 = (t2f6 * t2f7);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t2f8, 0.0, 0.0, 0.0, 0.0, );}
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a6 == 0.0)) && (l.f9a7 != 0.0)) && (l.f9a8 == 0.0)) {let t2f9: f64 = (l.f18d3 * l.f1942);let t2fa: f64 = (t2f9).abs();let t2fb: f64 = (t2fa).powf(p.p880);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t2fb, 0.0, 0.0, 0.0, 0.0, );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a6 == 0.0)) && (l.f9a7 != 0.0)) {let t2fc: f64 = (1.0 - l.f17c6);let t2fd: f64 = (1.0 / t2fc);(l.f4cb, l.f4d3, l.f4d4, l.f4d5, l.f4d6, ) = (t2fd, (-((-l.f17cf) / (t2fc * t2fc))), (-((-l.f17d0) / (t2fc * t2fc))), (-((-l.f17d1) / (t2fc * t2fc))), (-((-l.f17d2) / (t2fc * t2fc))), );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) && (l.f9a6 == 0.0)) && (l.f9a7 == 0.0)) {let t2fe: f64 = (l.f76 * p.p877);let t2ff: f64 = (l.f18d3 + t2fe);let t300: f64 = (t2ff * l.f14a3);let t301: f64 = (l.f553 + t300);(l.f4cb, l.f4d3, l.f4d4, l.f4d5, l.f4d6, ) = (t301, 0.0, 0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f998 == 0.0)) {let t302: f64 = (l.fd38 + l.fe77);let t303: f64 = (t302 + l.fe83);let t304: f64 = (t303 + l.fd27);let t305: f64 = (p.p29 * t304);let t306: f64 = (t305 * l.f4cb);(l.fda1, l.fda9, l.fdaa, l.fdab, l.fdac, ) = (t306, (((p.p29 * ((l.fe7f + l.fe8b) + l.fd2f)) * l.f4cb) + (t305 * l.f4d3)), (((p.p29 * ((l.fe80 + l.fe8c) + l.fd30)) * l.f4cb) + (t305 * l.f4d4)), (((p.p29 * ((l.fe81 + l.fe8d) + l.fd31)) * l.f4cb) + (t305 * l.f4d5)), (((p.p29 * ((l.fe82 + l.fe8e) + l.fd32)) * l.f4cb) + (t305 * l.f4d6)), );}
        let t307: f64 = if l.ff0f == 0.0 { 1.0 } else { 0.0 };l.f9a9 = t307;
        if (((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 != 0.0)) {(l.fddc, l.fde4, l.fde5, l.fde6, l.fde7, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) {let t308: f64 = (l.fd4e * l.fd39);l.fd38 = t308;}
        let t309: f64 = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };l.f9aa = t309;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9aa != 0.0)) {(l.fe77, l.fe7f, l.fe80, l.fe81, l.fe82, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9aa == 0.0)) {let t30a: f64 = (l.f192a - l.f1a0b);l.f18e5 = t30a;let t30b: f64 = (l.f17fe / l.f18e5);let t30c: f64 = (1.0 - t30b);let t30d: f64 = (t30c).sqrt();let t30e: f64 = (1.0 - t30d);l.f1b23 = t30e;}
        let t30f: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };l.f9ab = t30f;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9aa == 0.0)) && (l.f9ab != 0.0)) {l.f3cb = 0.0;}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9aa == 0.0)) && (l.f9ab == 0.0)) {let t310: f64 = (l.f1b23 * l.f1b23);let t311: f64 = (l.f1b23).ln();let t312: f64 = (t310 * t311);let t313: f64 = (1.0 - l.f1b23);let t314: f64 = (t312 / t313);let t315: f64 = (t314 + l.f1b23);let t316: f64 = (2.0 * p.p849);let t317: f64 = (1.0 - t316);let t318: f64 = (t315 * t317);l.f3cb = t318;}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9aa == 0.0)) {let t319: f64 = (l.f1b23 + l.f3cb);l.f1b1d = t319;}
        let t31a: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };l.f9ac = t31a;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9aa == 0.0)) && (l.f9ac != 0.0)) {let t31b: f64 = (l.f18e5 * l.f1926);let t31c: f64 = (t31b).sqrt();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t31c, 0.0, 0.0, 0.0, 0.0, );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9aa == 0.0)) && (l.f9ac == 0.0)) {let t31d: f64 = (l.f18e5 * l.f1926);let t31e: f64 = (t31d).powf(p.p849);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t31e, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9aa == 0.0)) {let t31f: f64 = (l.f1af9 * l.f17c6);(l.f1ad8, l.f1ae1, l.f1ae2, l.f1ae3, l.f1ae4, ) = (t31f, (l.f1af9 * l.f17cf), (l.f1af9 * l.f17d0), (l.f1af9 * l.f17d1), (l.f1af9 * l.f17d2), );let t320: f64 = (l.f1e48 - 1.0);let t321: f64 = (t320 * l.f1ad8);let t322: f64 = (l.f565 * t321);(l.fe0, l.fe8, l.fe9, l.fea, l.feb, ) = (t322, (l.f565 * (t320 * l.f1ae1)), (l.f565 * (t320 * l.f1ae2)), (l.f565 * (t320 * l.f1ae3)), (l.f565 * (t320 * l.f1ae4)), );let t323: f64 = (l.fe0 * l.f1b1d);let t324: f64 = (p.p858 * t323);(l.fe77, l.fe7f, l.fe80, l.fe81, l.fe82, ) = (t324, (p.p858 * (l.fe8 * l.f1b1d)), (p.p858 * (l.fe9 * l.f1b1d)), (p.p858 * (l.fea * l.f1b1d)), (p.p858 * (l.feb * l.f1b1d)), );}
        let t325: f64 = if p.p863 == 0.0 { 1.0 } else { 0.0 };l.f9ad = t325;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad != 0.0)) {(l.fe83, l.fe8b, l.fe8c, l.fe8d, l.fe8e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) {let t326: f64 = (l.f1ad8 * l.f1002);let t327: f64 = (t326 / l.f18e5);let t328: f64 = (l.f161 * t327);(l.f14c, l.f155, l.f156, l.f157, l.f158, ) = (t328, (l.f161 * ((l.f1ae1 * l.f1002) / l.f18e5)), (l.f161 * ((l.f1ae2 * l.f1002) / l.f18e5)), (l.f161 * ((l.f1ae3 * l.f1002) / l.f18e5)), (l.f161 * ((l.f1ae4 * l.f1002) / l.f18e5)), );let t329: f64 = (0.666666666666667 * l.ff4);let t32a: f64 = (t329 / l.f14c);(l.f1805, l.f180e, l.f180f, l.f1810, l.f1811, ) = (t32a, (-((t329 * l.f155) / (l.f14c * l.f14c))), (-((t329 * l.f156) / (l.f14c * l.f14c))), (-((t329 * l.f157) / (l.f14c * l.f14c))), (-((t329 * l.f158) / (l.f14c * l.f14c))), );let t32b: f64 = (l.f1805 * l.f1805);(l.f185f, l.f1868, l.f1869, l.f186a, l.f186b, ) = (t32b, ((l.f180e * l.f1805) + (l.f1805 * l.f180e)), ((l.f180f * l.f1805) + (l.f1805 * l.f180f)), ((l.f1810 * l.f1805) + (l.f1805 * l.f1810)), ((l.f1811 * l.f1805) + (l.f1805 * l.f1811)), );let t32c: f64 = (l.f185f * l.f185f);let t32d: f64 = (l.f185f * l.f185f);let t32e: f64 = (t32d + 1.0);let t32f: f64 = (t32c / t32e);let t330: f64 = (t32f).sqrt();(l.f1852, l.f185b, l.f185c, l.f185d, l.f185e, ) = (t330, ((((((l.f1868 * l.f185f) + (l.f185f * l.f1868)) * t32e) - (t32c * ((l.f1868 * l.f185f) + (l.f185f * l.f1868)))) / (t32e * t32e)) / (2.0 * t330)), ((((((l.f1869 * l.f185f) + (l.f185f * l.f1869)) * t32e) - (t32c * ((l.f1869 * l.f185f) + (l.f185f * l.f1869)))) / (t32e * t32e)) / (2.0 * t330)), ((((((l.f186a * l.f185f) + (l.f185f * l.f186a)) * t32e) - (t32c * ((l.f186a * l.f185f) + (l.f185f * l.f186a)))) / (t32e * t32e)) / (2.0 * t330)), ((((((l.f186b * l.f185f) + (l.f185f * l.f186b)) * t32e) - (t32c * ((l.f186b * l.f185f) + (l.f185f * l.f186b)))) / (t32e * t32e)) / (2.0 * t330)), );let t331: f64 = (l.f1852).sqrt();(l.f1656, l.f165f, l.f1660, l.f1661, l.f1662, ) = (t331, (l.f185b / (2.0 * t331)), (l.f185c / (2.0 * t331)), (l.f185d / (2.0 * t331)), (l.f185e / (2.0 * t331)), );let t332: f64 = (l.f1852 * l.f1656);(l.f186c, l.f1875, l.f1876, l.f1877, l.f1878, ) = (t332, ((l.f185b * l.f1656) + (l.f1852 * l.f165f)), ((l.f185c * l.f1656) + (l.f1852 * l.f1660)), ((l.f185d * l.f1656) + (l.f1852 * l.f1661)), ((l.f185e * l.f1656) + (l.f1852 * l.f1662)), );}
        let t333: f64 = (-p.p849);let t334: f64 = (t333 * l.f1012);let t335: f64 = (-1.0);let t336: f64 = if t334 == t335 { 1.0 } else { 0.0 };l.f9ae = t336;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9ae != 0.0)) {let t337: f64 = (l.f14c * l.f186c);let t338: f64 = (1.0 + t337);let t339: f64 = (1.0 / t338);(l.f1b03, l.f1b0b, l.f1b0c, l.f1b0d, l.f1b0e, ) = (t339, (-(((l.f155 * l.f186c) + (l.f14c * l.f1875)) / (t338 * t338))), (-(((l.f156 * l.f186c) + (l.f14c * l.f1876)) / (t338 * t338))), (-(((l.f157 * l.f186c) + (l.f14c * l.f1877)) / (t338 * t338))), (-(((l.f158 * l.f186c) + (l.f14c * l.f1878)) / (t338 * t338))), );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9ae == 0.0)) {let t33a: f64 = (l.f14c * l.f186c);let t33b: f64 = (1.0 + t33a);let t33c: f64 = (-p.p849);let t33d: f64 = (t33c * l.f1012);let t33e: f64 = (t33b).powf(t33d);(l.f1b03, l.f1b0b, l.f1b0c, l.f1b0d, l.f1b0e, ) = (t33e, if 0.0 == 0.0 && ((t33d) as f64).is_finite() && ((t33d) as f64).fract() == 0.0 { if t33d == 0.0 { 0.0 } else { (t33d * ((t33b).powf(t33d - 1.0) * ((l.f155 * l.f186c) + (l.f14c * l.f1875)))) } } else { (t33e * (t33d * (((l.f155 * l.f186c) + (l.f14c * l.f1875)) / t33b))) }, if 0.0 == 0.0 && ((t33d) as f64).is_finite() && ((t33d) as f64).fract() == 0.0 { if t33d == 0.0 { 0.0 } else { (t33d * ((t33b).powf(t33d - 1.0) * ((l.f156 * l.f186c) + (l.f14c * l.f1876)))) } } else { (t33e * (t33d * (((l.f156 * l.f186c) + (l.f14c * l.f1876)) / t33b))) }, if 0.0 == 0.0 && ((t33d) as f64).is_finite() && ((t33d) as f64).fract() == 0.0 { if t33d == 0.0 { 0.0 } else { (t33d * ((t33b).powf(t33d - 1.0) * ((l.f157 * l.f186c) + (l.f14c * l.f1877)))) } } else { (t33e * (t33d * (((l.f157 * l.f186c) + (l.f14c * l.f1877)) / t33b))) }, if 0.0 == 0.0 && ((t33d) as f64).is_finite() && ((t33d) as f64).fract() == 0.0 { if t33d == 0.0 { 0.0 } else { (t33d * ((t33b).powf(t33d - 1.0) * ((l.f158 * l.f186c) + (l.f14c * l.f1878)))) } } else { (t33e * (t33d * (((l.f158 * l.f186c) + (l.f14c * l.f1878)) / t33b))) }, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) {let t33f: f64 = (l.f1b1d * l.f1b03);let t340: f64 = (l.f1b1d + l.f1b03);let t341: f64 = (t33f / t340);(l.f1b29, l.f1b31, l.f1b32, l.f1b33, l.f1b34, ) = (t341, ((((l.f1b1d * l.f1b0b) * t340) - (t33f * l.f1b0b)) / (t340 * t340)), ((((l.f1b1d * l.f1b0c) * t340) - (t33f * l.f1b0c)) / (t340 * t340)), ((((l.f1b1d * l.f1b0d) * t340) - (t33f * l.f1b0d)) / (t340 * t340)), ((((l.f1b1d * l.f1b0e) * t340) - (t33f * l.f1b0e)) / (t340 * t340)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        l: &mut StampLocals,
    ) {
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) {let t342: f64 = (l.f14c / l.f1656);let t343: f64 = (0.375 * t342);let t344: f64 = (t343).sqrt();(l.fecc, l.fed5, l.fed6, l.fed7, l.fed8, ) = (t344, ((0.375 * (((l.f155 * l.f1656) - (l.f14c * l.f165f)) / (l.f1656 * l.f1656))) / (2.0 * t344)), ((0.375 * (((l.f156 * l.f1656) - (l.f14c * l.f1660)) / (l.f1656 * l.f1656))) / (2.0 * t344)), ((0.375 * (((l.f157 * l.f1656) - (l.f14c * l.f1661)) / (l.f1656 * l.f1656))) / (2.0 * t344)), ((0.375 * (((l.f158 * l.f1656) - (l.f14c * l.f1662)) / (l.f1656 * l.f1656))) / (2.0 * t344)), );let t345: f64 = (l.f1805 * l.f1656);let t346: f64 = (2.0 * t345);let t347: f64 = (t346 - l.f1852);(l.ff11, l.ff1a, l.ff1b, l.ff1c, l.ff1d, ) = (t347, ((2.0 * ((l.f180e * l.f1656) + (l.f1805 * l.f165f))) - l.f185b), ((2.0 * ((l.f180f * l.f1656) + (l.f1805 * l.f1660))) - l.f185c), ((2.0 * ((l.f1810 * l.f1656) + (l.f1805 * l.f1661))) - l.f185d), ((2.0 * ((l.f1811 * l.f1656) + (l.f1805 * l.f1662))) - l.f185e), );let t348: f64 = (l.ff4 * l.f1805);let t349: f64 = (t348 * l.f1656);let t34a: f64 = (l.ff4 * l.f1852);let t34b: f64 = (t349 - t34a);let t34c: f64 = (l.f14c * l.f186c);let t34d: f64 = (0.5 * t34c);let t34e: f64 = (t34b + t34d);(l.ff8a, l.ff93, l.ff94, l.ff95, l.ff96, ) = (t34e, (((((l.ff4 * l.f180e) * l.f1656) + (t348 * l.f165f)) - (l.ff4 * l.f185b)) + (0.5 * ((l.f155 * l.f186c) + (l.f14c * l.f1875)))), (((((l.ff4 * l.f180f) * l.f1656) + (t348 * l.f1660)) - (l.ff4 * l.f185c)) + (0.5 * ((l.f156 * l.f186c) + (l.f14c * l.f1876)))), (((((l.ff4 * l.f1810) * l.f1656) + (t348 * l.f1661)) - (l.ff4 * l.f185d)) + (0.5 * ((l.f157 * l.f186c) + (l.f14c * l.f1877)))), (((((l.ff4 * l.f1811) * l.f1656) + (t348 * l.f1662)) - (l.ff4 * l.f185e)) + (0.5 * ((l.f158 * l.f186c) + (l.f14c * l.f1878)))), );let t34f: f64 = (l.ff11 - 1.0);let t350: f64 = (t34f * l.fecc);(l.f1c0c, l.f1c15, l.f1c16, l.f1c17, l.f1c18, ) = (t350, ((l.ff1a * l.fecc) + (t34f * l.fed5)), ((l.ff1b * l.fecc) + (t34f * l.fed6)), ((l.ff1c * l.fecc) + (t34f * l.fed7)), ((l.ff1d * l.fecc) + (t34f * l.fed8)), );let t351: f64 = (l.f1c0c * l.f1c0c);(l.f1e11, l.f1e1a, l.f1e1b, l.f1e1c, l.f1e1d, ) = (t351, ((l.f1c15 * l.f1c0c) + (l.f1c0c * l.f1c15)), ((l.f1c16 * l.f1c0c) + (l.f1c0c * l.f1c16)), ((l.f1c17 * l.f1c0c) + (l.f1c0c * l.f1c17)), ((l.f1c18 * l.f1c0c) + (l.f1c0c * l.f1c18)), );}
        let t352: f64 = if l.f1c0c > 0.0 { 1.0 } else { 0.0 };l.f9af = t352;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9af != 0.0)) {let t353: f64 = (l.f105a * l.f1c0c);let t354: f64 = (1.0 + t353);let t355: f64 = (1.0 / t354);(l.f1702, l.f170a, l.f170b, l.f170c, l.f170d, ) = (t355, (-((l.f105a * l.f1c15) / (t354 * t354))), (-((l.f105a * l.f1c16) / (t354 * t354))), (-((l.f105a * l.f1c17) / (t354 * t354))), (-((l.f105a * l.f1c18) / (t354 * t354))), );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9af == 0.0)) {let t356: f64 = (l.f105a * l.f1c0c);let t357: f64 = (1.0 - t356);let t358: f64 = (1.0 / t357);(l.f1702, l.f170a, l.f170b, l.f170c, l.f170d, ) = (t358, (-((-(l.f105a * l.f1c15)) / (t357 * t357))), (-((-(l.f105a * l.f1c16)) / (t357 * t357))), (-((-(l.f105a * l.f1c17)) / (t357 * t357))), (-((-(l.f105a * l.f1c18)) / (t357 * t357))), );}
        let t359: f64 = (-l.f1e11);let t35a: f64 = (t359 + l.ff8a);let t35b: f64 = (-230.25850929940458);let t35c: f64 = if t35a > t35b { 1.0 } else { 0.0 };l.f9b1 = t35c;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9b1 != 0.0)) {let t35d: f64 = (-l.f1e11);let t35e: f64 = (t35d + l.ff8a);let t35f: f64 = (t35e).exp();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t35f, (t35f * ((-l.f1e1a) + l.ff93)), (t35f * ((-l.f1e1b) + l.ff94)), (t35f * ((-l.f1e1c) + l.ff95)), (t35f * ((-l.f1e1d) + l.ff96)), );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9b1 == 0.0)) {let t360: f64 = (-230.25850929940458);let t361: f64 = (-l.f1e11);let t362: f64 = (t361 + l.ff8a);let t363: f64 = (t360 - t362);let t364: f64 = (-230.25850929940458);let t365: f64 = (-l.f1e11);let t366: f64 = (t365 + l.ff8a);let t367: f64 = (t364 - t366);let t368: f64 = (-230.25850929940458);let t369: f64 = (-l.f1e11);let t36a: f64 = (t369 + l.ff8a);let t36b: f64 = (t368 - t36a);let t36c: f64 = (t36b * 0.3333333333333333);let t36d: f64 = (1.0 + t36c);let t36e: f64 = (t367 * t36d);let t36f: f64 = (0.5 * t36e);let t370: f64 = (1.0 + t36f);let t371: f64 = (t363 * t370);let t372: f64 = (1.0 + t371);let t373: f64 = (1e-100 / t372);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t373, (-((1e-100 * (((-((-l.f1e1a) + l.ff93)) * t370) + (t363 * (0.5 * (((-((-l.f1e1a) + l.ff93)) * t36d) + (t367 * ((-((-l.f1e1a) + l.ff93)) * 0.3333333333333333))))))) / (t372 * t372))), (-((1e-100 * (((-((-l.f1e1b) + l.ff94)) * t370) + (t363 * (0.5 * (((-((-l.f1e1b) + l.ff94)) * t36d) + (t367 * ((-((-l.f1e1b) + l.ff94)) * 0.3333333333333333))))))) / (t372 * t372))), (-((1e-100 * (((-((-l.f1e1c) + l.ff95)) * t370) + (t363 * (0.5 * (((-((-l.f1e1c) + l.ff95)) * t36d) + (t367 * ((-((-l.f1e1c) + l.ff95)) * 0.3333333333333333))))))) / (t372 * t372))), (-((1e-100 * (((-((-l.f1e1d) + l.ff96)) * t370) + (t363 * (0.5 * (((-((-l.f1e1d) + l.ff96)) * t36d) + (t367 * ((-((-l.f1e1d) + l.ff96)) * 0.3333333333333333))))))) / (t372 * t372))), );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) {let t374: f64 = (0.29214664 * l.f1702);let t375: f64 = (l.f1702 * l.f1702);let t376: f64 = (l.f114 * t375);let t377: f64 = (t374 + t376);let t378: f64 = (l.f1702 * l.f1702);let t379: f64 = (t378 * l.f1702);let t37a: f64 = (l.f171 * t379);let t37b: f64 = (t377 + t37a);let t37c: f64 = (t37b * l.f17c6);(l.f423, l.f42b, l.f42c, l.f42d, l.f42e, ) = (t37c, (((((0.29214664 * l.f170a) + (l.f114 * ((l.f170a * l.f1702) + (l.f1702 * l.f170a)))) + (l.f171 * ((((l.f170a * l.f1702) + (l.f1702 * l.f170a)) * l.f1702) + (t378 * l.f170a)))) * l.f17c6) + (t37b * l.f17cf)), (((((0.29214664 * l.f170b) + (l.f114 * ((l.f170b * l.f1702) + (l.f1702 * l.f170b)))) + (l.f171 * ((((l.f170b * l.f1702) + (l.f1702 * l.f170b)) * l.f1702) + (t378 * l.f170b)))) * l.f17c6) + (t37b * l.f17d0)), (((((0.29214664 * l.f170c) + (l.f114 * ((l.f170c * l.f1702) + (l.f1702 * l.f170c)))) + (l.f171 * ((((l.f170c * l.f1702) + (l.f1702 * l.f170c)) * l.f1702) + (t378 * l.f170c)))) * l.f17c6) + (t37b * l.f17d1)), (((((0.29214664 * l.f170d) + (l.f114 * ((l.f170d * l.f1702) + (l.f1702 * l.f170d)))) + (l.f171 * ((((l.f170d * l.f1702) + (l.f1702 * l.f170d)) * l.f1702) + (t378 * l.f170d)))) * l.f17c6) + (t37b * l.f17d2)), );}
        let t37d: f64 = if l.f1c0c > 0.0 { 1.0 } else { 0.0 };l.f9b2 = t37d;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9b2 != 0.0)) {(l.f42f, l.f437, l.f438, l.f439, l.f43a, ) = (l.f423, l.f42b, l.f42c, l.f42d, l.f42e, );}
        let t37e: f64 = (-230.25850929940458);let t37f: f64 = if l.ff8a > t37e { 1.0 } else { 0.0 };l.f9b3 = t37f;
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9b2 == 0.0)) && (l.f9b3 != 0.0)) {let t380: f64 = (l.ff8a).exp();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t380, (t380 * l.ff93), (t380 * l.ff94), (t380 * l.ff95), (t380 * l.ff96), );}
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9b2 == 0.0)) && (l.f9b3 == 0.0)) {let t381: f64 = (-230.25850929940458);let t382: f64 = (t381 - l.ff8a);let t383: f64 = (-230.25850929940458);let t384: f64 = (t383 - l.ff8a);let t385: f64 = (-230.25850929940458);let t386: f64 = (t385 - l.ff8a);let t387: f64 = (t386 * 0.3333333333333333);let t388: f64 = (1.0 + t387);let t389: f64 = (t384 * t388);let t38a: f64 = (0.5 * t389);let t38b: f64 = (1.0 + t38a);let t38c: f64 = (t382 * t38b);let t38d: f64 = (1.0 + t38c);let t38e: f64 = (1e-100 / t38d);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t38e, (-((1e-100 * (((-l.ff93) * t38b) + (t382 * (0.5 * (((-l.ff93) * t388) + (t384 * ((-l.ff93) * 0.3333333333333333))))))) / (t38d * t38d))), (-((1e-100 * (((-l.ff94) * t38b) + (t382 * (0.5 * (((-l.ff94) * t388) + (t384 * ((-l.ff94) * 0.3333333333333333))))))) / (t38d * t38d))), (-((1e-100 * (((-l.ff95) * t38b) + (t382 * (0.5 * (((-l.ff95) * t388) + (t384 * ((-l.ff95) * 0.3333333333333333))))))) / (t38d * t38d))), (-((1e-100 * (((-l.ff96) * t38b) + (t382 * (0.5 * (((-l.ff96) * t388) + (t384 * ((-l.ff96) * 0.3333333333333333))))))) / (t38d * t38d))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) && (l.f9b2 == 0.0)) {let t38f: f64 = (2.0 * l.f17c6);let t390: f64 = (t38f - l.f423);(l.f42f, l.f437, l.f438, l.f439, l.f43a, ) = (t390, ((2.0 * l.f17cf) - l.f42b), ((2.0 * l.f17d0) - l.f42c), ((2.0 * l.f17d1) - l.f42d), ((2.0 * l.f17d2) - l.f42e), );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9ad == 0.0)) {let t391: f64 = (1.772453850905516 * 0.5);let t392: f64 = (l.ff4 * l.f42f);let t393: f64 = (t392 / l.fecc);let t394: f64 = (t391 * t393);(l.f57d, l.f585, l.f586, l.f587, l.f588, ) = (t394, (t391 * ((((l.ff4 * l.f437) * l.fecc) - (t392 * l.fed5)) / (l.fecc * l.fecc))), (t391 * ((((l.ff4 * l.f438) * l.fecc) - (t392 * l.fed6)) / (l.fecc * l.fecc))), (t391 * ((((l.ff4 * l.f439) * l.fecc) - (t392 * l.fed7)) / (l.fecc * l.fecc))), (t391 * ((((l.ff4 * l.f43a) * l.fecc) - (t392 * l.fed8)) / (l.fecc * l.fecc))), );let t395: f64 = (l.fe0 * l.f57d);let t396: f64 = (t395 * l.f1b29);let t397: f64 = (p.p863 * t396);(l.fe83, l.fe8b, l.fe8c, l.fe8d, l.fe8e, ) = (t397, (p.p863 * ((((l.fe8 * l.f57d) + (l.fe0 * l.f585)) * l.f1b29) + (t395 * l.f1b31))), (p.p863 * ((((l.fe9 * l.f57d) + (l.fe0 * l.f586)) * l.f1b29) + (t395 * l.f1b32))), (p.p863 * ((((l.fea * l.f57d) + (l.fe0 * l.f587)) * l.f1b29) + (t395 * l.f1b33))), (p.p863 * ((((l.feb * l.f57d) + (l.fe0 * l.f588)) * l.f1b29) + (t395 * l.f1b34))), );}
        let t398: f64 = if p.p869 == 0.0 { 1.0 } else { 0.0 };l.f9b4 = t398;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b4 != 0.0)) {(l.fd27, l.fd2f, l.fd30, l.fd31, l.fd32, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t399: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };l.f9b5 = t399;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b4 == 0.0)) && (l.f9b5 != 0.0)) {let t39a: f64 = (p.p846 - l.f18da);let t39b: f64 = (t39a * l.f1926);let t39c: f64 = (t39b).sqrt();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t39c, 0.0, 0.0, 0.0, 0.0, );}
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b4 == 0.0)) && (l.f9b5 == 0.0)) {let t39d: f64 = (p.p846 - l.f18da);let t39e: f64 = (t39d * l.f1926);let t39f: f64 = (t39e).powf(p.p849);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t39f, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b4 == 0.0)) {let t3a0: f64 = (p.p846 - l.f18da);let t3a1: f64 = (t3a0 * l.f1af5);let t3a2: f64 = (t3a1 / l.f17c6);let t3a3: f64 = (l.f1012 * t3a2);(l.f512, l.f51b, l.f51c, l.f51d, l.f51e, ) = (t3a3, (l.f1012 * (-((t3a1 * l.f17cf) / (l.f17c6 * l.f17c6)))), (l.f1012 * (-((t3a1 * l.f17d0) / (l.f17c6 * l.f17c6)))), (l.f1012 * (-((t3a1 * l.f17d1) / (l.f17c6 * l.f17c6)))), (l.f1012 * (-((t3a1 * l.f17d2) / (l.f17c6 * l.f17c6)))), );}
        let t3a4: f64 = (-l.f4c5);let t3a5: f64 = (t3a4 / l.f512);let t3a6: f64 = (t3a5).abs();let t3a7: f64 = if t3a6 < 230.25850929940458 { 1.0 } else { 0.0 };l.f9b6 = t3a7;
        if (((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b4 == 0.0)) && (l.f9b6 != 0.0)) {let t3a8: f64 = (-l.f4c5);let t3a9: f64 = (t3a8 / l.f512);let t3aa: f64 = (t3a9).exp();(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t3aa, (t3aa * (-((t3a8 * l.f51b) / (l.f512 * l.f512)))), (t3aa * (-((t3a8 * l.f51c) / (l.f512 * l.f512)))), (t3aa * (-((t3a8 * l.f51d) / (l.f512 * l.f512)))), (t3aa * (-((t3a8 * l.f51e) / (l.f512 * l.f512)))), );}
        let t3ab: f64 = (-l.f4c5);let t3ac: f64 = (t3ab / l.f512);let t3ad: f64 = if t3ac < 0.0 { 1.0 } else { 0.0 };l.f9b7 = t3ad;
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b4 == 0.0)) && (l.f9b6 == 0.0)) && (l.f9b7 != 0.0)) {let t3ae: f64 = (-230.25850929940458);let t3af: f64 = (-l.f4c5);let t3b0: f64 = (t3af / l.f512);let t3b1: f64 = (t3ae - t3b0);let t3b2: f64 = (-230.25850929940458);let t3b3: f64 = (-l.f4c5);let t3b4: f64 = (t3b3 / l.f512);let t3b5: f64 = (t3b2 - t3b4);let t3b6: f64 = (-230.25850929940458);let t3b7: f64 = (-l.f4c5);let t3b8: f64 = (t3b7 / l.f512);let t3b9: f64 = (t3b6 - t3b8);let t3ba: f64 = (t3b9 * 0.3333333333333333);let t3bb: f64 = (1.0 + t3ba);let t3bc: f64 = (t3b5 * t3bb);let t3bd: f64 = (0.5 * t3bc);let t3be: f64 = (1.0 + t3bd);let t3bf: f64 = (t3b1 * t3be);let t3c0: f64 = (1.0 + t3bf);let t3c1: f64 = (1e-100 / t3c0);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t3c1, (-((1e-100 * (((-(-((t3af * l.f51b) / (l.f512 * l.f512)))) * t3be) + (t3b1 * (0.5 * (((-(-((t3b3 * l.f51b) / (l.f512 * l.f512)))) * t3bb) + (t3b5 * ((-(-((t3b7 * l.f51b) / (l.f512 * l.f512)))) * 0.3333333333333333))))))) / (t3c0 * t3c0))), (-((1e-100 * (((-(-((t3af * l.f51c) / (l.f512 * l.f512)))) * t3be) + (t3b1 * (0.5 * (((-(-((t3b3 * l.f51c) / (l.f512 * l.f512)))) * t3bb) + (t3b5 * ((-(-((t3b7 * l.f51c) / (l.f512 * l.f512)))) * 0.3333333333333333))))))) / (t3c0 * t3c0))), (-((1e-100 * (((-(-((t3af * l.f51d) / (l.f512 * l.f512)))) * t3be) + (t3b1 * (0.5 * (((-(-((t3b3 * l.f51d) / (l.f512 * l.f512)))) * t3bb) + (t3b5 * ((-(-((t3b7 * l.f51d) / (l.f512 * l.f512)))) * 0.3333333333333333))))))) / (t3c0 * t3c0))), (-((1e-100 * (((-(-((t3af * l.f51e) / (l.f512 * l.f512)))) * t3be) + (t3b1 * (0.5 * (((-(-((t3b3 * l.f51e) / (l.f512 * l.f512)))) * t3bb) + (t3b5 * ((-(-((t3b7 * l.f51e) / (l.f512 * l.f512)))) * 0.3333333333333333))))))) / (t3c0 * t3c0))), );}
        if ((((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b4 == 0.0)) && (l.f9b6 == 0.0)) && (l.f9b7 == 0.0)) {let t3c2: f64 = (-l.f4c5);let t3c3: f64 = (t3c2 / l.f512);let t3c4: f64 = (t3c3 - 230.25850929940458);let t3c5: f64 = (-l.f4c5);let t3c6: f64 = (t3c5 / l.f512);let t3c7: f64 = (t3c6 - 230.25850929940458);let t3c8: f64 = (-l.f4c5);let t3c9: f64 = (t3c8 / l.f512);let t3ca: f64 = (t3c9 - 230.25850929940458);let t3cb: f64 = (t3ca * 0.3333333333333333);let t3cc: f64 = (1.0 + t3cb);let t3cd: f64 = (t3c7 * t3cc);let t3ce: f64 = (0.5 * t3cd);let t3cf: f64 = (1.0 + t3ce);let t3d0: f64 = (t3c4 * t3cf);let t3d1: f64 = (1.0 + t3d0);let t3d2: f64 = (1e100 * t3d1);(l.f17c6, l.f17cf, l.f17d0, l.f17d1, l.f17d2, ) = (t3d2, (1e100 * (((-((t3c2 * l.f51b) / (l.f512 * l.f512))) * t3cf) + (t3c4 * (0.5 * (((-((t3c5 * l.f51b) / (l.f512 * l.f512))) * t3cc) + (t3c7 * ((-((t3c8 * l.f51b) / (l.f512 * l.f512))) * 0.3333333333333333))))))), (1e100 * (((-((t3c2 * l.f51c) / (l.f512 * l.f512))) * t3cf) + (t3c4 * (0.5 * (((-((t3c5 * l.f51c) / (l.f512 * l.f512))) * t3cc) + (t3c7 * ((-((t3c8 * l.f51c) / (l.f512 * l.f512))) * 0.3333333333333333))))))), (1e100 * (((-((t3c2 * l.f51d) / (l.f512 * l.f512))) * t3cf) + (t3c4 * (0.5 * (((-((t3c5 * l.f51d) / (l.f512 * l.f512))) * t3cc) + (t3c7 * ((-((t3c8 * l.f51d) / (l.f512 * l.f512))) * 0.3333333333333333))))))), (1e100 * (((-((t3c2 * l.f51e) / (l.f512 * l.f512))) * t3cf) + (t3c4 * (0.5 * (((-((t3c5 * l.f51e) / (l.f512 * l.f512))) * t3cc) + (t3c7 * ((-((t3c8 * l.f51e) / (l.f512 * l.f512))) * 0.3333333333333333))))))), );}
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b4 == 0.0)) {let t3d3: f64 = (l.f189b * l.f512);let t3d4: f64 = (t3d3 * l.f512);let t3d5: f64 = (t3d4 * l.f17c6);let t3d6: f64 = (p.p869 * t3d5);(l.fd27, l.fd2f, l.fd30, l.fd31, l.fd32, ) = (t3d6, (p.p869 * (((((l.f189b * l.f51b) * l.f512) + (t3d3 * l.f51b)) * l.f17c6) + (t3d4 * l.f17cf))), (p.p869 * (((((l.f189b * l.f51c) * l.f512) + (t3d3 * l.f51c)) * l.f17c6) + (t3d4 * l.f17d0))), (p.p869 * (((((l.f189b * l.f51d) * l.f512) + (t3d3 * l.f51d)) * l.f17c6) + (t3d4 * l.f17d1))), (p.p869 * (((((l.f189b * l.f51e) * l.f512) + (t3d3 * l.f51e)) * l.f17c6) + (t3d4 * l.f17d2))), );}
        let t3d7: f64 = if p.p878 > 1000.0 { 1.0 } else { 0.0 };l.f9b8 = t3d7;
        if ((((l.f962 != 0.0) && (l.f98f != 0.0)) && (l.f9a9 == 0.0)) && (l.f9b8 != 0.0)) {(l.f4cb, l.f4d3, l.f4d4, l.f4d5, l.f4d6, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );}
        let t3d8: f64 = (-l.f76);let t3d9: f64 = (t3d8 * p.p878);let t3da: f64 = if l.f18d3 > t3d9 { 1.0 } else { 0.0 };l.f9b9 = t3da;let t3db: f64 = if p.p881 == 4.0 { 1.0 } else { 0.0 };l.f9ba = t3db;
    }
}
