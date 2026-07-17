#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let (t182,) = {
    if (p.p33 > 0.01) {
        (p.p33,)
    } else {
        (0.01,)
    }
};
        l.f5c5 = t182;l.f5c6 = 0.0;
        let (t183,) = {
    if (p.p34 > 0.0) {
        (p.p34,)
    } else {
        (0.0,)
    }
};
        l.f24 = t183;l.f25 = 0.0;
        let (t19c,) = {
    if (p.p35 > 0.0) {
        (p.p35,)
    } else {
        (0.0,)
    }
};
        l.f28 = t19c;l.f29 = 0.0;
        let (t1a1,) = {
    if (p.p36 > 0.0) {
        (p.p36,)
    } else {
        (0.0,)
    }
};
        l.f26 = t1a1;l.f27 = 0.0;l.fa5 = p.p37;l.fa6 = 0.0;l.fa9 = p.p38;l.faa = 0.0;l.fa7 = p.p39;l.fa8 = 0.0;l.f6c6 = p.p40;l.f6c7 = 0.0;l.f6ca = p.p41;l.f6cb = 0.0;l.f6c8 = p.p42;l.f6c9 = 0.0;
        let (t1ee,) = {
    if (p.p43 > 0.1) {
        (p.p43,)
    } else {
        (0.1,)
    }
};
        l.f783 = t1ee;l.f784 = 0.0;
        let (t207,) = {
    if (p.p44 > 0.1) {
        (p.p44,)
    } else {
        (0.1,)
    }
};
        l.f78d = t207;l.f78e = 0.0;
        let (t224,) = {
    if (p.p45 > 0.1) {
        (p.p45,)
    } else {
        (0.1,)
    }
};
        l.f785 = t224;l.f786 = 0.0;
        let (t28c,) = {
    if (p.p46 > 0.1) {
        (p.p46,)
    } else {
        (0.1,)
    }
};
        l.f625 = t28c;l.f626 = 0.0;
        let (t2a7,) = {
    if (p.p47 > 0.1) {
        (p.p47,)
    } else {
        (0.1,)
    }
};
        l.f629 = t2a7;l.f62a = 0.0;
        let (t2b3,) = {
    if (p.p48 > 0.1) {
        (p.p48,)
    } else {
        (0.1,)
    }
};
        l.f627 = t2b3;l.f628 = 0.0;l.fc1 = p.p7;l.fc2 = 0.0;
        let (t2b4,) = {
    if (p.p56 > 0.0) {
        (p.p56,)
    } else {
        (0.0,)
    }
};
        l.f711 = t2b4;l.f712 = 0.0;l.f6cd = p.p57;l.f6ce = 0.0;l.f6cf = p.p58;l.f6d0 = 0.0;l.f6d5 = p.p59;l.f6d6 = 0.0;l.f6d7 = p.p60;l.f6d8 = 0.0;l.f6d1 = p.p61;l.f6d2 = 0.0;l.f6d3 = p.p62;l.f6d4 = 0.0;
        let (t2b5,) = {
    if (p.p63 > 0.1) {
        (p.p63,)
    } else {
        (0.1,)
    }
};
        l.f5e5 = t2b5;l.f5e6 = 0.0;
        let (t2b6,) = {
    if (p.p64 > 0.1) {
        (p.p64,)
    } else {
        (0.1,)
    }
};
        l.f5e9 = t2b6;l.f5ea = 0.0;
        let (t2b7,) = {
    if (p.p65 > 0.1) {
        (p.p65,)
    } else {
        (0.1,)
    }
};
        l.f5e7 = t2b7;l.f5e8 = 0.0;
        let (t2b8,) = {
    if (p.p76 > 0.1) {
        (p.p76,)
    } else {
        (0.1,)
    }
};
        l.f80e = t2b8;l.f80f = 0.0;
        let (t2b9,) = {
    if (p.p77 > 0.0) {
        (p.p77,)
    } else {
        (0.0,)
    }
};
        l.f6b4 = t2b9;l.f6b5 = 0.0;
        let (t2ba,) = {
    if (p.p78 > 0.0) {
        (p.p78,)
    } else {
        (0.0,)
    }
};
        l.f6b6 = t2ba;l.f6b7 = 0.0;l.f6d9 = 0.0;l.f6da = 0.0;let t2bb: f64 = if p.p81 > 0.5 { 1.0 } else { 0.0 };l.f1ad = t2bb;l.f282 = 0.0;
        if (l.f1ad != 0.0) {l.f6d9 = 1.0;l.f6da = 0.0;}
        if (l.f1ad == 0.0) {l.f6d9 = 0.0;l.f6da = 0.0;}
        let (t2bc,) = {
    if (p.p82 > 0.5) {
        (p.p82,)
    } else {
        (0.5,)
    }
};
        l.f7ab = t2bc;l.f7ac = 0.0;
        let (t2bd,) = {
    if (p.p83 > 0.0) {
        (p.p83,)
    } else {
        (0.0,)
    }
};
        l.fb3 = t2bd;l.fb4 = 0.0;let t2be: f64 = (273.15 + l.f705);l.f6e9 = t2be;l.f6ea = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t2bf: f64 = ctx_temp;let t2c0: f64 = (t2bf + p.p102);let t2c1: f64 = (-250.0);let t2c2: f64 = (273.15 + t2c1);let t2c3: f64 = (t2c0).max(t2c2);l.f6e7 = t2c3;l.f6e8 = 0.0;let t2c4: f64 = (l.f6e7 / l.f6e9);l.f14 = t2c4;l.f15 = 0.0;let t2c5: f64 = (1.3806505e-23 / 1.6021918e-19);l.f5a5 = t2c5;l.f5a6 = 0.0;let t2c6: f64 = (l.f5a5 * l.f6e9);l.f647 = t2c6;l.f648 = 0.0;let t2c7: f64 = (1.0 / l.f647);l.f649 = t2c7;l.f64a = 0.0;let t2c8: f64 = (l.f5a5 * l.f6e7);l.f643 = t2c8;l.f644 = 0.0;let t2c9: f64 = (1.0 / l.f643);l.f645 = t2c9;l.f646 = 0.0;let t2ca: f64 = (0.000702 * l.f6e9);let t2cb: f64 = (t2ca * l.f6e9);let t2cc: f64 = (-t2cb);let t2cd: f64 = (1108.0 + l.f6e9);let t2ce: f64 = (t2cc / t2cd);l.f4f = t2ce;l.f50 = 0.0;let t2cf: f64 = (l.f631 + l.f4f);l.f63b = t2cf;l.f63c = 0.0;let t2d0: f64 = (l.f641 + l.f4f);l.f63f = t2d0;l.f640 = 0.0;let t2d1: f64 = (l.f639 + l.f4f);l.f63d = t2d1;l.f63e = 0.0;let t2d2: f64 = (0.000702 * l.f6e7);let t2d3: f64 = (t2d2 * l.f6e7);let t2d4: f64 = (-t2d3);let t2d5: f64 = (1108.0 + l.f6e7);let t2d6: f64 = (t2d4 / t2d5);l.f4d = t2d6;l.f4e = 0.0;let t2d7: f64 = (l.f631 + l.f4d);l.f633 = t2d7;l.f634 = 0.0;let t2d8: f64 = (l.f641 + l.f4d);l.f637 = t2d8;l.f638 = 0.0;let t2d9: f64 = (l.f639 + l.f4d);l.f635 = t2d9;l.f636 = 0.0;let t2da: f64 = (l.f80e / 2.0);let t2db: f64 = (l.f14).powf(t2da);let t2dc: f64 = (l.f63b * l.f649);let t2dd: f64 = (l.f633 * l.f645);let t2de: f64 = (t2dc - t2dd);let t2df: f64 = (0.5 * t2de);let t2e0: f64 = (t2df).exp();let t2e1: f64 = (t2db * t2e0);l.fc9 = t2e1;l.fcc = 0.0;let t0: f64 = (l.f80e / 2.0);let t1: f64 = (l.f14).powf(t0);let t2: f64 = (l.f63f * l.f649);let t3: f64 = (l.f637 * l.f645);let t4: f64 = (t2 - t3);let t5: f64 = (0.5 * t4);let t6: f64 = (t5).exp();let t7: f64 = (t1 * t6);l.fd1 = t7;l.fd4 = 0.0;let t8: f64 = (l.f80e / 2.0);let t9: f64 = (l.f14).powf(t8);let ta: f64 = (l.f63d * l.f649);let tb: f64 = (l.f635 * l.f645);let tc: f64 = (ta - tb);let td: f64 = (0.5 * tc);let te: f64 = (td).exp();let tf: f64 = (t9 * te);l.fcd = tf;l.fd0 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        l: &mut StampLocals,
    ) {
        let t10: f64 = (l.f80e / 2.0);let t11: f64 = (t10 / l.f5e5);let t12: f64 = (l.f14).powf(t11);let t13: f64 = (l.f63b * l.f649);let t14: f64 = (l.f633 * l.f645);let t15: f64 = (t13 - t14);let t16: f64 = (0.5 * t15);let t17: f64 = (t16 / l.f5e5);let t18: f64 = (t17).exp();let t19: f64 = (t12 * t18);l.fca = t19;l.fcb = 0.0;let t1a: f64 = (l.f80e / 2.0);let t1b: f64 = (t1a / l.f5e9);let t1c: f64 = (l.f14).powf(t1b);let t1d: f64 = (l.f63f * l.f649);let t1e: f64 = (l.f637 * l.f645);let t1f: f64 = (t1d - t1e);let t20: f64 = (0.5 * t1f);let t21: f64 = (t20 / l.f5e9);let t22: f64 = (t21).exp();let t23: f64 = (t1c * t22);l.fd2 = t23;l.fd3 = 0.0;let t24: f64 = (l.f80e / 2.0);let t25: f64 = (t24 / l.f5e7);let t26: f64 = (l.f14).powf(t25);let t27: f64 = (l.f63d * l.f649);let t28: f64 = (l.f635 * l.f645);let t29: f64 = (t27 - t28);let t2a: f64 = (0.5 * t29);let t2b: f64 = (t2a / l.f5e7);let t2c: f64 = (t2b).exp();let t2d: f64 = (t26 * t2c);l.fce = t2d;l.fcf = 0.0;let t2e: f64 = (l.f546 * l.fca);let t2f: f64 = (t2e * l.fca);l.f542 = t2f;l.f543 = 0.0;let t30: f64 = (l.f54a * l.fd2);let t31: f64 = (t30 * l.fd2);l.f54c = t31;l.f54d = 0.0;let t32: f64 = (l.f548 * l.fce);let t33: f64 = (t32 * l.fce);l.f544 = t33;l.f545 = 0.0;let t34: f64 = (l.f771 * l.f14);let t35: f64 = (2.0 * l.f643);let t36: f64 = (l.fc9).ln();let t37: f64 = (t35 * t36);let t38: f64 = (t34 - t37);l.f71f = t38;l.f720 = 0.0;let t39: f64 = (l.f779 * l.f14);let t3a: f64 = (2.0 * l.f643);let t3b: f64 = (l.fd1).ln();let t3c: f64 = (t3a * t3b);let t3d: f64 = (t39 - t3c);l.f723 = t3d;l.f724 = 0.0;let t3e: f64 = (l.f775 * l.f14);let t3f: f64 = (2.0 * l.f643);let t40: f64 = (l.fcd).ln();let t41: f64 = (t3f * t40);let t42: f64 = (t3e - t41);l.f721 = t42;l.f722 = 0.0;let t43: f64 = (0.05 - l.f71f);let t44: f64 = (t43 * l.f645);let t45: f64 = (t44).exp();let t46: f64 = (1.0 + t45);let t47: f64 = (t46).ln();let t48: f64 = (l.f643 * t47);let t49: f64 = (l.f71f + t48);l.f75d = t49;l.f762 = 0.0;let t4a: f64 = (0.05 - l.f723);let t4b: f64 = (t4a * l.f645);let t4c: f64 = (t4b).exp();let t4d: f64 = (1.0 + t4c);let t4e: f64 = (t4d).ln();let t4f: f64 = (l.f643 * t4e);let t50: f64 = (l.f723 + t4f);l.f77d = t50;l.f782 = 0.0;let t51: f64 = (0.05 - l.f721);let t52: f64 = (t51 * l.f645);let t53: f64 = (t52).exp();let t54: f64 = (1.0 + t53);let t55: f64 = (t54).ln();let t56: f64 = (l.f643 * t55);
        let t57: f64 = (l.f721 + t56);l.f763 = t57;l.f768 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        l: &mut StampLocals,
    ) {
        let t58: f64 = (1.0 / l.f75d);l.f769 = t58;l.f76a = 0.0;let t59: f64 = (1.0 / l.f77d);l.f76d = t59;l.f76e = 0.0;let t5a: f64 = (1.0 / l.f763);l.f76b = t5a;l.f76c = 0.0;let t5b: f64 = (1.0 - l.f623);l.f60b = t5b;l.f60c = 0.0;let t5c: f64 = (1.0 - l.f653);l.f60f = t5c;l.f610 = 0.0;let t5d: f64 = (1.0 - l.f62f);l.f60d = t5d;l.f60e = 0.0;let t5e: f64 = (1.0 / l.f60b);l.f611 = t5e;l.f612 = 0.0;let t5f: f64 = (1.0 / l.f60f);l.f615 = t5f;l.f616 = 0.0;let t60: f64 = (1.0 / l.f60d);l.f613 = t60;l.f614 = 0.0;let t61: f64 = (l.f771 * l.f769);let t62: f64 = (t61).powf(l.f623);let t63: f64 = (l.f30 * t62);l.f2c = t63;l.f2d = 0.0;let t64: f64 = (l.f779 * l.f76d);let t65: f64 = (t64).powf(l.f653);let t66: f64 = (l.f34 * t65);l.f36 = t66;l.f37 = 0.0;let t67: f64 = (l.f775 * l.f76b);let t68: f64 = (t67).powf(l.f62f);let t69: f64 = (l.f32 * t68);l.f2e = t69;l.f2f = 0.0;let t6a: f64 = (l.f2c * l.f75d);let t6b: f64 = (t6a * l.f611);l.f69e = t6b;l.f69f = 0.0;let t6c: f64 = (l.f36 * l.f77d);let t6d: f64 = (t6c * l.f615);l.f6a2 = t6d;l.f6a3 = 0.0;let t6e: f64 = (l.f2e * l.f763);let t6f: f64 = (t6e * l.f613);l.f6a0 = t6f;l.f6a1 = 0.0;let t70: f64 = (2.0 * l.f2c);l.f698 = t70;l.f699 = 0.0;let t71: f64 = (2.0 * l.f36);l.f69c = t71;l.f69d = 0.0;let t72: f64 = (2.0 * l.f2e);l.f69a = t72;l.f69b = 0.0;let t73: f64 = (l.f6b / l.f30);l.f7d6 = t73;l.f7d7 = 0.0;let t74: f64 = (l.f80c * l.f6b);let t75: f64 = (t74 / l.f34);l.f7e0 = t75;l.f7e1 = 0.0;let t76: f64 = (l.f80a * l.f6b);let t77: f64 = (t76 / l.f32);l.f7d8 = t77;l.f7d9 = 0.0;let t78: f64 = (1.0 / l.f7d6);l.f7da = t78;l.f7db = 0.0;let t79: f64 = (1.0 / l.f7e0);l.f7de = t79;l.f7df = 0.0;let t7a: f64 = (1.0 / l.f7d8);l.f7dc = t7a;l.f7dd = 0.0;let t7b: f64 = (1.0 / l.f771);l.f773 = t7b;l.f774 = 0.0;let t7c: f64 = (1.0 / l.f779);l.f77b = t7c;l.f77c = 0.0;let t7d: f64 = (1.0 / l.f775);l.f777 = t7d;l.f778 = 0.0;let t7e: f64 = (1.772453850905516 * 0.29214664);l.f62b = t7e;l.f62c = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        l: &mut StampLocals,
    ) {
        let t7f: f64 = (-5.0);let t80: f64 = (t7f * 0.29214664);let t81: f64 = (t80 + 6.0);let t82: f64 = (-2.0);let t83: f64 = (l.f62b).powf(t82);let t84: f64 = (t81 - t83);let t85: f64 = (t84 / 3.0);l.f16 = t85;l.f17 = 0.0;let t86: f64 = (1.0 - 0.29214664);let t87: f64 = (t86 - l.f16);l.f2a = t87;l.f2b = 0.0;let t88: f64 = (0.5 * l.f633);let t89: f64 = (t88).max(l.f643);l.f47 = t89;l.f48 = 0.0;let t8a: f64 = (0.5 * l.f637);let t8b: f64 = (t8a).max(l.f643);l.f4b = t8b;l.f4c = 0.0;let t8c: f64 = (0.5 * l.f635);let t8d: f64 = (t8c).max(l.f643);l.f49 = t8d;l.f4a = 0.0;let t8e: f64 = (l.f47 * l.f645);l.fe = t8e;l.ff = 0.0;let t8f: f64 = (l.f4b * l.f645);l.f12 = t8f;l.f13 = 0.0;let t90: f64 = (l.f49 * l.f645);l.f10 = t90;l.f11 = 0.0;let t91: f64 = (32.0 * l.f5c3);let t92: f64 = (t91 * 9.1093826e-31);let t93: f64 = (t92 * 1.6021918e-19);let t94: f64 = (l.f47 * l.f47);let t95: f64 = (t94 * l.f47);let t96: f64 = (t93 * t95);let t97: f64 = (t96).sqrt();let t98: f64 = (3.0 * 1.05457168e-34);let t99: f64 = (t97 / t98);l.f1e = t99;l.f1f = 0.0;let t9a: f64 = (32.0 * l.f5c7);let t9b: f64 = (t9a * 9.1093826e-31);let t9c: f64 = (t9b * 1.6021918e-19);let t9d: f64 = (l.f4b * l.f4b);let t9e: f64 = (t9d * l.f4b);let t9f: f64 = (t9c * t9e);let ta0: f64 = (t9f).sqrt();let ta1: f64 = (3.0 * 1.05457168e-34);let ta2: f64 = (ta0 / ta1);l.f22 = ta2;l.f23 = 0.0;let ta3: f64 = (32.0 * l.f5c5);let ta4: f64 = (ta3 * 9.1093826e-31);let ta5: f64 = (ta4 * 1.6021918e-19);let ta6: f64 = (l.f49 * l.f49);let ta7: f64 = (ta6 * l.f49);let ta8: f64 = (ta5 * ta7);let ta9: f64 = (ta8).sqrt();let taa: f64 = (3.0 * 1.05457168e-34);let tab: f64 = (ta9 / taa);l.f20 = tab;l.f21 = 0.0;let tac: f64 = (l.f6e7 - l.f6e9);let tad: f64 = (l.f6c6 * tac);let tae: f64 = (1.0 + tad);let taf: f64 = (l.fa5 * tae);l.fa1 = taf;l.fa2 = 0.0;let tb0: f64 = (l.f6e7 - l.f6e9);let tb1: f64 = (l.f6ca * tb0);let tb2: f64 = (1.0 + tb1);let tb3: f64 = (l.fa9 * tb2);l.fab = tb3;l.fac = 0.0;let tb4: f64 = (l.f6e7 - l.f6e9);let tb5: f64 = (l.f6c8 * tb4);let tb6: f64 = (1.0 + tb5);let tb7: f64 = (l.fa7 * tb6);l.fa3 = tb7;l.fa4 = 0.0;
        if (!(l.fa1 > 0.0)) {l.fa1 = 0.0;l.fa2 = 0.0;}
        if (!(l.fab > 0.0)) {l.fab = 0.0;l.fac = 0.0;}
        if (!(l.fa3 > 0.0)) {l.fa3 = 0.0;l.fa4 = 0.0;}
        let tb8: f64 = (l.fc1 - 1.0);let tb9: f64 = (tb8 / l.fc1);l.f2 = tb9;l.f3 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        l: &mut StampLocals,
    ) {
        let tba: f64 = (l.f2).powf(l.f625);let tbb: f64 = (1.0 - tba);let tbc: f64 = (1.0 / tbb);l.fc3 = tbc;l.fc4 = 0.0;let tbd: f64 = (l.f2).powf(l.f629);let tbe: f64 = (1.0 - tbd);let tbf: f64 = (1.0 / tbe);l.fc7 = tbf;l.fc8 = 0.0;let tc0: f64 = (l.f2).powf(l.f627);let tc1: f64 = (1.0 - tc0);let tc2: f64 = (1.0 / tc1);l.fc5 = tc2;l.fc6 = 0.0;let tc3: f64 = (l.f6e7 - l.f6e9);let tc4: f64 = (l.f6e7 - l.f6e9);let tc5: f64 = (tc4 * l.f6cf);let tc6: f64 = (l.f6cd + tc5);let tc7: f64 = (tc3 * tc6);let tc8: f64 = (1.0 + tc7);let tc9: f64 = (l.f783 * tc8);l.f783 = tc9;l.f784 = 0.0;let tca: f64 = (l.f6e7 - l.f6e9);let tcb: f64 = (l.f6e7 - l.f6e9);let tcc: f64 = (tcb * l.f6d7);let tcd: f64 = (l.f6d5 + tcc);let tce: f64 = (tca * tcd);let tcf: f64 = (1.0 + tce);let td0: f64 = (l.f78d * tcf);l.f78d = td0;l.f78e = 0.0;let td1: f64 = (l.f6e7 - l.f6e9);let td2: f64 = (l.f6e7 - l.f6e9);let td3: f64 = (td2 * l.f6d3);let td4: f64 = (l.f6d1 + td3);let td5: f64 = (td1 * td4);let td6: f64 = (1.0 + td5);let td7: f64 = (l.f785 * td6);l.f785 = td7;l.f786 = 0.0;let td8: f64 = if l.f783 <= 0.1 { 1.0 } else { 0.0 };l.f283 = td8;l.f350 = 0.0;
        if (l.f283 != 0.0) {l.f783 = 0.1;l.f784 = 0.0;l.f787 = 10.0;l.f788 = 0.0;}
        if (l.f283 == 0.0) {let td9: f64 = (1.0 / l.f783);l.f787 = td9;l.f788 = 0.0;}
        let tda: f64 = if l.f78d <= 0.1 { 1.0 } else { 0.0 };l.f351 = tda;l.f41a = 0.0;
        if (l.f351 != 0.0) {l.f78d = 0.1;l.f78e = 0.0;l.f78b = 10.0;l.f78c = 0.0;}
        if (l.f351 == 0.0) {let tdb: f64 = (1.0 / l.f78d);l.f78b = tdb;l.f78c = 0.0;}
        let tdc: f64 = if l.f785 <= 0.1 { 1.0 } else { 0.0 };l.f41b = tdc;l.f4a7 = 0.0;
        if (l.f41b != 0.0) {l.f785 = 0.1;l.f786 = 0.0;l.f789 = 10.0;l.f78a = 0.0;}
        if (l.f41b == 0.0) {let tdd: f64 = (1.0 / l.f785);l.f789 = tdd;l.f78a = 0.0;}
        let tde: f64 = (0.01 * l.f6b6);let tdf: f64 = (1.0 - tde);l.f6b8 = tdf;l.f6b9 = 0.0;let te0: f64 = (l.fc3 * l.fc3);let te1: f64 = (l.f625 - 1.0);let te2: f64 = (l.f2).powf(te1);let te3: f64 = (te0 * te2);let te4: f64 = (-te3);let te5: f64 = (te4 * l.f625);let te6: f64 = (te5 * l.f787);l.f6ba = te6;l.f6bb = 0.0;let te7: f64 = (l.fc7 * l.fc7);let te8: f64 = (l.f629 - 1.0);let te9: f64 = (l.f2).powf(te8);let tea: f64 = (te7 * te9);let teb: f64 = (-tea);let tec: f64 = (teb * l.f629);let ted: f64 = (tec * l.f78b);l.f6be = ted;l.f6bf = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tee: f64 = (l.fc5 * l.fc5);let tef: f64 = (l.f627 - 1.0);let tf0: f64 = (l.f2).powf(tef);let tf1: f64 = (tee * tf0);let tf2: f64 = (-tf1);let tf3: f64 = (tf2 * l.f627);let tf4: f64 = (tf3 * l.f789);l.f6bc = tf4;l.f6bd = 0.0;let tf5: f64 = (p.p87 * 1000000.0);l.f5df = tf5;l.f5e0 = 0.0;let tf6: f64 = (p.p89 * 1000000.0);l.f5e3 = tf6;l.f5e4 = 0.0;let tf7: f64 = (p.p88 * 1000000.0);l.f5e1 = tf7;l.f5e2 = 0.0;l.f5dd = l.f5df;l.f5de = 0.0;l.f609 = l.f5e5;l.f60a = 0.0;let tf8: f64 = (1450.0 * 0.0001);l.f5d9 = tf8;l.f5da = 0.0;let tf9: f64 = (500.0 * 0.0001);l.f5db = tf9;l.f5dc = 0.0;l.f61f = 0.6;l.f620 = 0.0;l.f5a3 = 0.001;l.f5a4 = 0.0;let tfa: f64 = (1.45e16 * l.fca);l.f5eb = tfa;l.f5ec = 0.0;let tfb: f64 = (l.f5eb * l.f5eb);let tfc: f64 = (tfb / l.f5dd);l.f64d = tfc;l.f64e = 0.0;let tfd: f64 = (-1.5);let tfe: f64 = (l.f14).powf(tfd);l.f6db = tfe;l.f6dc = 0.0;let tff: f64 = (l.f5d9 * l.f6db);let t100: f64 = (tff / l.f645);l.f59 = t100;l.f5a = 0.0;let t101: f64 = (l.f5db * l.f6db);let t102: f64 = (t101 / l.f645);l.f5f = t102;l.f60 = 0.0;let t103: f64 = (2.0 * l.f59);let t104: f64 = (t103 * l.f5f);let t105: f64 = (l.f59 + l.f5f);let t106: f64 = (t104 / t105);l.f45 = t106;l.f46 = 0.0;let t107: f64 = (l.f14).powf(p.p97);l.f6dd = t107;l.f6de = 0.0;let t108: f64 = (p.p93 * l.f6dd);l.f6df = t108;l.f6e0 = 0.0;let t109: f64 = (l.f6df * l.f45);let t10a: f64 = (t109).sqrt();l.f5ad = t10a;l.f5ae = 0.0;let t10b: f64 = (l.f609 / l.f645);let t10c: f64 = (l.f5dd / l.f64d);let t10d: f64 = (t10c).ln();let t10e: f64 = (t10b * t10d);l.f741 = t10e;l.f742 = 0.0;let t10f: f64 = (l.f609 / l.f645);let t110: f64 = (l.f5dd / l.f64d);let t111: f64 = (t110).ln();let t112: f64 = (p.p94 / l.f5ad);let t113: f64 = (t111 + t112);let t114: f64 = (t10f * t113);l.f743 = t114;l.f744 = 0.0;
        let (t115,) = {
    if (p.p99 > 0.0) {
        (p.p99,)
    } else {
        (0.0,)
    }
};
        let t116: f64 = (t115 * l.f6b4);let t117: f64 = (t116 * l.f6b4);let t118: f64 = (t117 * l.f6b8);let t119: f64 = (t118 * l.f6b8);l.f0 = t119;l.f1 = 0.0;
        let (t11a,) = {
    if (p.p100 > 0.0) {
        (p.p100,)
    } else {
        (0.0,)
    }
};
        let t11b: f64 = (t11a * l.f6b4);let t11c: f64 = (t11b * l.f6b8);l.f5b1 = t11c;l.f5b2 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let (t11d,) = {
    if (p.p101 > 0.0) {
        (p.p101,)
    } else {
        (0.0,)
    }
};
        let t11e: f64 = (t11d * l.f6b4);let t11f: f64 = (t11e * l.f6b8);l.f5af = t11f;l.f5b0 = 0.0;l.f89 = 0.0;l.f92 = 0.0;(l.f8a, l.f8b, l.f8c, ) = (0.0, 0.0, 0.0, );l.f8d = 0.0;(l.f93, l.f94, l.f95, ) = (0.0, 0.0, 0.0, );l.f96 = 0.0;(l.f8e, l.f8f, l.f90, ) = (0.0, 0.0, 0.0, );l.f91 = 0.0;let t120: f64 = (l.f542 * l.f0);let t121: f64 = if t120 > 0.0 { 1.0 } else { 0.0 };l.f4a8 = t121;l.f4a9 = 0.0;
        if (l.f4a8 != 0.0) {let t122: f64 = (l.f542 * l.f0);let t123: f64 = (l.f57a / t122);let t124: f64 = (t123 + 1.0);let t125: f64 = (t124).ln();let t126: f64 = (l.f643 * t125);let t127: f64 = (t126 * l.f5e5);l.f7b3 = t127;l.f7b4 = 0.0;}
        if (l.f4a8 == 0.0) {l.f7b3 = 100000000.0;l.f7b4 = 0.0;}
        let t128: f64 = (l.f54c * l.f5b1);let t129: f64 = if t128 > 0.0 { 1.0 } else { 0.0 };l.f4aa = t129;l.f4b3 = 0.0;
        if (l.f4aa != 0.0) {let t12a: f64 = (l.f54c * l.f5b1);let t12b: f64 = (l.f57a / t12a);let t12c: f64 = (t12b + 1.0);let t12d: f64 = (t12c).ln();let t12e: f64 = (l.f643 * t12d);let t12f: f64 = (t12e * l.f5e9);l.f7b7 = t12f;l.f7b8 = 0.0;}
        if (l.f4aa == 0.0) {l.f7b7 = 100000000.0;l.f7b8 = 0.0;}
        let t130: f64 = (l.f544 * l.f5af);let t131: f64 = if t130 > 0.0 { 1.0 } else { 0.0 };l.f4b4 = t131;l.f4c9 = 0.0;
        if (l.f4b4 != 0.0) {let t132: f64 = (l.f544 * l.f5af);let t133: f64 = (l.f57a / t132);let t134: f64 = (t133 + 1.0);let t135: f64 = (t134).ln();let t136: f64 = (l.f643 * t135);let t137: f64 = (t136 * l.f5e7);l.f7b5 = t137;l.f7b6 = 0.0;}
        if (l.f4b4 == 0.0) {l.f7b5 = 100000000.0;l.f7b6 = 0.0;}
        let t138: f64 = (l.f7b3).min(l.f7b7);let t139: f64 = (t138).min(l.f7b5);l.f7b1 = t139;l.f7b2 = 0.0;let t13a: f64 = (l.f7b1 * l.f645);let t13b: f64 = (t13a).abs();let t13c: f64 = if t13b < 230.25850929940458 { 1.0 } else { 0.0 };l.f4ca = t13c;l.f4df = 0.0;
        if (l.f4ca != 0.0) {let t13d: f64 = (l.f7b1 * l.f645);let t13e: f64 = (t13d).exp();l.f89 = t13e;l.f92 = 0.0;}
        let t13f: f64 = (l.f7b1 * l.f645);let t140: f64 = (-230.25850929940458);let t141: f64 = if t13f < t140 { 1.0 } else { 0.0 };l.fdc = t141;l.ff1 = 0.0;
        if ((l.f4ca == 0.0) && (l.fdc != 0.0)) {let t142: f64 = (-230.25850929940458);let t143: f64 = (l.f7b1 * l.f645);let t144: f64 = (t142 - t143);let t145: f64 = (-230.25850929940458);let t146: f64 = (l.f7b1 * l.f645);let t147: f64 = (t145 - t146);let t148: f64 = (-230.25850929940458);let t149: f64 = (l.f7b1 * l.f645);let t14a: f64 = (t148 - t149);let t14b: f64 = (t14a * 0.3333333333333333);let t14c: f64 = (1.0 + t14b);let t14d: f64 = (t147 * t14c);let t14e: f64 = (0.5 * t14d);let t14f: f64 = (1.0 + t14e);let t150: f64 = (t144 * t14f);let t151: f64 = (1.0 + t150);let t152: f64 = (1e-100 / t151);l.f89 = t152;l.f92 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        l: &mut StampLocals,
    ) {
        if ((l.f4ca == 0.0) && (l.fdc == 0.0)) {let t153: f64 = (l.f7b1 * l.f645);let t154: f64 = (t153 - 230.25850929940458);let t155: f64 = (l.f7b1 * l.f645);let t156: f64 = (t155 - 230.25850929940458);let t157: f64 = (l.f7b1 * l.f645);let t158: f64 = (t157 - 230.25850929940458);let t159: f64 = (t158 * 0.3333333333333333);let t15a: f64 = (1.0 + t159);let t15b: f64 = (t156 * t15a);let t15c: f64 = (0.5 * t15b);let t15d: f64 = (1.0 + t15c);let t15e: f64 = (t154 * t15d);let t15f: f64 = (1.0 + t15e);let t160: f64 = (1e100 * t15f);l.f89 = t160;l.f92 = 0.0;}
        l.f75e = l.f75d;l.f75f = 0.0;l.f77e = l.f77d;l.f77f = 0.0;l.f764 = l.f763;l.f765 = 0.0;l.f621 = l.f623;l.f622 = 0.0;l.f651 = l.f653;l.f652 = 0.0;l.f62d = l.f62f;l.f62e = 0.0;l.f760 = l.f771;l.f761 = 0.0;l.f780 = l.f779;l.f781 = 0.0;l.f766 = l.f775;l.f767 = 0.0;let t161: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.ff2 = t161;l.f107 = 0.0;
        if (l.ff2 != 0.0) {let t162: f64 = (l.f77d + l.f763);l.f75e = t162;l.f75f = 0.0;let t163: f64 = (l.f653).min(l.f62f);let t164: f64 = (0.9 * t163);l.f621 = t164;l.f622 = 0.0;let t165: f64 = (l.f779 + l.f775);l.f760 = t165;l.f761 = 0.0;}
        let t166: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f108 = t166;l.f11d = 0.0;
        if (l.f108 != 0.0) {let t167: f64 = (l.f75d + l.f763);l.f77e = t167;l.f77f = 0.0;let t168: f64 = (l.f623).min(l.f62f);let t169: f64 = (0.9 * t168);l.f651 = t169;l.f652 = 0.0;let t16a: f64 = (l.f771 + l.f775);l.f780 = t16a;l.f781 = 0.0;}
        let t16b: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f11e = t16b;l.f133 = 0.0;
        if (l.f11e != 0.0) {let t16c: f64 = (l.f75d + l.f77d);l.f764 = t16c;l.f765 = 0.0;let t16d: f64 = (l.f623).min(l.f653);let t16e: f64 = (0.9 * t16d);l.f62d = t16e;l.f62e = 0.0;let t16f: f64 = (l.f771 + l.f779);l.f766 = t16f;l.f767 = 0.0;}
        let t170: f64 = (l.f75e).min(l.f77e);let t171: f64 = (t170).min(l.f764);l.f76f = t171;l.f770 = 0.0;let t172: f64 = (l.f76f * 0.1);l.f78f = t172;l.f790 = 0.0;let t173: f64 = (l.f621).max(l.f651);let t174: f64 = (t173).max(l.f62d);l.f64b = t174;l.f64c = 0.0;let t175: f64 = (-1.0);let t176: f64 = (t175 / l.f64b);let t177: f64 = (2.0_f64).powf(t176);let t178: f64 = (1.0 - t177);let t179: f64 = (l.f76f * t178);l.f791 = t179;l.f792 = 0.0;let t17a: f64 = (l.f760).min(l.f780);let t17b: f64 = (t17a).min(l.f766);let t17c: f64 = (t17b - 0.05);l.f755 = t17c;l.f756 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        l: &mut StampLocals,
    ) {
        let t17d: f64 = (l.f0 * l.f542);let t17e: f64 = (l.f5b1 * l.f54c);let t17f: f64 = (t17d + t17e);let t180: f64 = (l.f5af * l.f544);let t181: f64 = (t17f + t180);l.f590 = t181;l.f591 = 0.0;l.f586 = 0.0;l.f587 = 0.0;l.f5c9 = 1.0;l.f5ca = 0.0;(l.f5cb, l.f5cc, l.f5cd, ) = (1.0, 0.0, 0.0, );l.f5ce = 0.0;(l.f588, l.f589, l.f58a, ) = (0.0, 0.0, 0.0, );l.f58b = 0.0;(l.f5cf, l.f5d0, l.f5d1, ) = (1.0, 0.0, 0.0, );l.f5d2 = 0.0;(l.f58c, l.f58d, l.f58e, ) = (0.0, 0.0, 0.0, );l.f58f = 0.0;l.f5bd = 0.0;l.f5be = 0.0;l.f800 = 0.0;l.f801 = 0.0;l.f97 = 0.0;l.f98 = 0.0;(l.f802, l.f803, l.f804, ) = (0.0, 0.0, 0.0, );l.f805 = 0.0;(l.f99, l.f9a, l.f9b, ) = (0.0, 0.0, 0.0, );l.f9c = 0.0;(l.f806, l.f807, l.f808, ) = (0.0, 0.0, 0.0, );l.f809 = 0.0;(l.f9d, l.f9e, l.f9f, ) = (0.0, 0.0, 0.0, );l.fa0 = 0.0;(l.f5b9, l.f5ba, l.f5bb, ) = (0.0, 0.0, 0.0, );l.f5bc = 0.0;(l.f5bf, l.f5c0, l.f5c1, ) = (0.0, 0.0, 0.0, );l.f5c2 = 0.0;(l.f501, l.f502, l.f503, ) = (0.0, 0.0, 0.0, );l.f504 = 0.0;(l.f509, l.f50a, l.f50b, ) = (0.0, 0.0, 0.0, );l.f50c = 0.0;(l.f511, l.f512, l.f513, ) = (0.0, 0.0, 0.0, );l.f514 = 0.0;(l.f519, l.f51a, l.f51b, ) = (0.0, 0.0, 0.0, );l.f51c = 0.0;(l.f521, l.f522, l.f523, ) = (0.0, 0.0, 0.0, );l.f524 = 0.0;l.f707 = 0.0;l.f708 = 0.0;(l.f709, l.f70a, l.f70b, ) = (0.0, 0.0, 0.0, );l.f70c = 0.0;(l.f70d, l.f70e, l.f70f, ) = (0.0, 0.0, 0.0, );l.f710 = 0.0;l.f822 = 0.0;l.f823 = 0.0;l.f81c = 1.0;l.f81d = 0.0;l.f820 = 1.0;l.f821 = 0.0;l.f81e = 1.0;l.f81f = 0.0;(l.f4, l.f5, l.f6, ) = (0.0, 0.0, 0.0, );l.f7 = 0.0;(l.f701, l.f702, l.f703, ) = (0.0, 0.0, 0.0, );l.f704 = 0.0;(l.f7ad, l.f7ae, l.f7af, ) = (0.0, 0.0, 0.0, );l.f7b0 = 0.0;(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );l.f539 = 0.0;(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f53d = 0.0;(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );l.f541 = 0.0;(l.f59e, l.f59f, l.f5a0, l.f5a1, ) = (0.0, 0.0, 0.0, 0.0, );l.f5a2 = 0.0;(l.f57c, l.f57d, l.f57e, l.f57f, ) = (0.0, 0.0, 0.0, 0.0, );l.f580 = 0.0;(l.f581, l.f582, l.f583, l.f584, ) = (0.0, 0.0, 0.0, 0.0, );l.f585 = 0.0;(l.f663, l.f664, ) = (0.0, 0.0, );l.f665 = 0.0;(l.f666, l.f667, ) = (0.0, 0.0, );l.f668 = 0.0;(l.f7c9, l.f7ca, ) = (0.0, 0.0, );l.f7cb = 0.0;let t184: f64 = (1.6021918e-19 * l.f0);l.f669 = t184;l.f66a = 0.0;let t185: f64 = (2.0 * l.f6b);let t186: f64 = (1.6021918e-19 * l.f5dd);let t187: f64 = (t185 / t186);let t188: f64 = (t187).sqrt();(l.f7bd, l.f7be, l.f7bf, ) = (t188, 0.0, 0.0, );l.f7c0 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t189: f64 = (p.p94 - l.f7bd);let t18a: f64 = (t189 - 1e-7);(l.f6f3, l.f6f4, l.f6f5, ) = (t18a, (-l.f7be), (-l.f7bf), );l.f6f6 = 0.0;let t18b: f64 = (4.0 * p.p94);let t18c: f64 = (t18b * 1e-7);(l.f6f7, l.f6f8, l.f6f9, ) = (t18c, 0.0, 0.0, );l.f6fa = 0.0;
        if (!(l.f6f7 > 0.0)) {let t18d: f64 = (-l.f6f7);(l.f6f7, l.f6f8, l.f6f9, ) = (t18d, (-l.f6f8), (-l.f6f9), );l.f6fa = 0.0;}
        let t18e: f64 = (l.f6f3 * l.f6f3);let t18f: f64 = (t18e + l.f6f7);let t190: f64 = (t18f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t190, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t190)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t190)), );l.f6fa = 0.0;let t191: f64 = (l.f6f3 + l.f6f7);let t192: f64 = (0.5 * t191);let t193: f64 = (p.p94 - t192);(l.f7bd, l.f7be, l.f7bf, ) = (t193, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f7c0 = 0.0;let t194: f64 = if l.f6d9 > 0.9 { 1.0 } else { 0.0 };l.f22a = t194;l.f23f = 0.0;let t195: f64 = (l.f5e5 - l.f5e7);let t196: f64 = (t195).abs();let t197: f64 = (l.f5e5 - l.f5e9);let t198: f64 = (t197).abs();let t199: f64 = (l.f5e7 - l.f5e9);let t19a: f64 = (t199).abs();let t19b: f64 = if (((((t196 > 1e-6) && (l.f0 > 0.0)) && (l.f5af > 0.0)) || (((t198 > 1e-6) && (l.f0 > 0.0)) && (l.f5b1 > 0.0))) || (((t19a > 1e-6) && (l.f5af > 0.0)) && (l.f5b1 > 0.0))) { 1.0 } else { 0.0 };l.f240 = t19b;l.f255 = 0.0;
        if ((l.f22a != 0.0) && (l.f240 != 0.0)) {l.f6d9 = 0.0;l.f6da = 0.0;}
        let t19d: f64 = if l.f0 > 0.0 { 1.0 } else { 0.0 };l.f256 = t19d;l.f26b = 0.0;
        if (((l.f22a != 0.0) && (l.f240 == 0.0)) && (l.f256 != 0.0)) {l.f5c9 = l.f5e5;l.f5ca = 0.0;}
        let t19e: f64 = if l.f5af > 0.0 { 1.0 } else { 0.0 };l.f26c = t19e;l.f281 = 0.0;
        if (((l.f22a != 0.0) && (l.f240 == 0.0)) && (l.f26c != 0.0)) {l.f5c9 = l.f5e7;l.f5ca = 0.0;}
        let t19f: f64 = if l.f5b1 > 0.0 { 1.0 } else { 0.0 };l.f284 = t19f;l.f299 = 0.0;
        if (((l.f22a != 0.0) && (l.f240 == 0.0)) && (l.f284 != 0.0)) {l.f5c9 = l.f5e9;l.f5ca = 0.0;}
        let t1a0: f64 = if l.f6d9 == 1.0 { 1.0 } else { 0.0 };l.f29a = t1a0;l.f2af = 0.0;
        if (l.f29a != 0.0) {l.f811 = 0.0;l.f812 = 0.0;l.f6e2 = 0.0;l.f6e3 = 0.0;l.f6e = 0.0;l.f6f = 0.0;l.f4e1 = 0.0;l.f4e2 = 0.0;l.f4e5 = 0.0;l.f4e6 = 0.0;l.f4e9 = 0.0;l.f4ea = 0.0;l.f4ef = 0.0;l.f4f0 = 0.0;l.f4f5 = 0.0;l.f4f6 = 0.0;l.f4fb = 0.0;l.f4fc = 0.0;(l.f745, l.f746, l.f747, ) = (0.0, 0.0, 0.0, );l.f748 = 0.0;l.f796 = 0.0;l.f797 = 0.0;l.f817 = 0.0;l.f818 = 0.0;l.f825 = 0.0;l.f826 = 0.0;l.f714 = 0.0;l.f715 = 0.0;l.f79c = 0.0;l.f79d = 0.0;l.f7a2 = 0.0;l.f7a3 = 0.0;l.f750 = 0.0;l.f751 = 0.0;l.f74a = 0.0;l.f74b = 0.0;l.f6fc = 0.0;l.f6fd = 0.0;(l.f52f, l.f530, l.f531, ) = (0.0, 0.0, 0.0, );l.f532 = 0.0;l.f593 = 0.0;l.f594 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        l: &mut StampLocals,
    ) {
        if (l.f29a != 0.0) {l.f758 = 0.0;l.f759 = 0.0;l.f7ef = 0.0;l.f7f0 = 0.0;l.f66 = 0.0;l.f67 = 0.0;l.f7e9 = 0.0;l.f7ea = 0.0;l.f7d1 = 0.0;l.f7d2 = 0.0;l.f9 = 0.0;l.fa = 0.0;l.f599 = 0.0;l.f59a = 0.0;l.f19 = 0.0;l.f1a = 0.0;l.f71a = 0.0;l.f71b = 0.0;l.f72c = 0.0;l.f72d = 0.0;l.f726 = 0.0;l.f727 = 0.0;l.f6c1 = 0.0;l.f6c2 = 0.0;l.f732 = 0.0;l.f733 = 0.0;l.f7e3 = 0.0;l.f7e4 = 0.0;l.f7f5 = 0.0;l.f7f6 = 0.0;l.f5a8 = 0.0;l.f5a9 = 0.0;l.f5b4 = 0.0;l.f5b5 = 0.0;l.f5d4 = 0.0;l.f5d5 = 0.0;l.f7fb = 0.0;l.f7fc = 0.0;l.f74 = 0.0;l.f75 = 0.0;l.fd6 = 0.0;l.fd7 = 0.0;l.f529 = 0.0;l.f52a = 0.0;l.fb6 = 0.0;l.fb7 = 0.0;l.fae = 0.0;l.faf = 0.0;l.fbd = 0.4;l.fbe = 0.0;l.fbf = 0.65;l.fc0 = 0.0;l.fbb = 0.8;l.fbc = 0.0;let t1a2: f64 = (-l.fbd);let t1a3: f64 = (t1a2 * l.f7ab);l.f737 = t1a3;l.f738 = 0.0;let t1a4: f64 = (-l.fbf);let t1a5: f64 = (t1a4 * l.f7ab);l.f739 = t1a5;l.f73a = 0.0;let t1a6: f64 = (-l.fbb);let t1a7: f64 = (t1a6 * l.f7ab);l.f73b = t1a7;l.f73c = 0.0;l.f73d = 0.1;l.f73e = 0.0;l.f73f = 0.2;l.f740 = 0.0;}
        let t1a8: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f4ab = t1a8;l.f4ac = 0.0;
        if ((l.f29a != 0.0) && (l.f4ab != 0.0)) {let t1a9: f64 = (4.0 * l.f78f);let t1aa: f64 = (t1a9 * l.f78f);l.f4e1 = t1aa;l.f4e2 = 0.0;let t1ab: f64 = (l.f78f / l.f791);l.f4e5 = t1ab;l.f4e6 = 0.0;let t1ac: f64 = (l.f78f * l.f4e5);let t1ad: f64 = (l.f737 + t1ac);l.f4e9 = t1ad;l.f4ea = 0.0;let t1ae: f64 = (l.f791 + l.f4e9);l.f4ef = t1ae;l.f4f0 = 0.0;let t1af: f64 = (l.f791 - l.f4e9);l.f4f5 = t1af;l.f4f6 = 0.0;let t1b0: f64 = (l.f4f5 * l.f4f5);let t1b1: f64 = (t1b0 + l.f4e1);let t1b2: f64 = (t1b1).sqrt();l.f4fb = t1b2;l.f4fc = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f29a != 0.0) && (l.f4ab != 0.0)) {let t1b3: f64 = (l.f737 * l.f791);let t1b4: f64 = (l.f4ef + l.f4fb);let t1b5: f64 = (t1b3 / t1b4);let t1b6: f64 = (2.0 * t1b5);l.f796 = t1b6;l.f797 = 0.0;}
        let t1b7: f64 = if l.f737 < l.f7b1 { 1.0 } else { 0.0 };l.f4ad = t1b7;l.f4ae = 0.0;let t1b8: f64 = (l.f737 * l.f645);let t1b9: f64 = (0.5 * t1b8);let t1ba: f64 = (t1b9).abs();let t1bb: f64 = if t1ba < 230.25850929940458 { 1.0 } else { 0.0 };l.f4af = t1bb;l.f4b0 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4af != 0.0)) {let t1bc: f64 = (l.f737 * l.f645);let t1bd: f64 = (0.5 * t1bc);let t1be: f64 = (t1bd).exp();l.f825 = t1be;l.f826 = 0.0;}
        let t1bf: f64 = (l.f737 * l.f645);let t1c0: f64 = (0.5 * t1bf);let t1c1: f64 = (-230.25850929940458);let t1c2: f64 = if t1c0 < t1c1 { 1.0 } else { 0.0 };l.f4b1 = t1c2;l.f4b2 = 0.0;
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4af == 0.0)) && (l.f4b1 != 0.0)) {let t1c3: f64 = (-230.25850929940458);let t1c4: f64 = (l.f737 * l.f645);let t1c5: f64 = (0.5 * t1c4);let t1c6: f64 = (t1c3 - t1c5);let t1c7: f64 = (-230.25850929940458);let t1c8: f64 = (l.f737 * l.f645);let t1c9: f64 = (0.5 * t1c8);let t1ca: f64 = (t1c7 - t1c9);let t1cb: f64 = (-230.25850929940458);let t1cc: f64 = (l.f737 * l.f645);let t1cd: f64 = (0.5 * t1cc);let t1ce: f64 = (t1cb - t1cd);let t1cf: f64 = (t1ce * 0.3333333333333333);let t1d0: f64 = (1.0 + t1cf);let t1d1: f64 = (t1ca * t1d0);let t1d2: f64 = (0.5 * t1d1);let t1d3: f64 = (1.0 + t1d2);let t1d4: f64 = (t1c6 * t1d3);let t1d5: f64 = (1.0 + t1d4);let t1d6: f64 = (1e-100 / t1d5);l.f825 = t1d6;l.f826 = 0.0;}
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4af == 0.0)) && (l.f4b1 == 0.0)) {let t1d7: f64 = (l.f737 * l.f645);let t1d8: f64 = (0.5 * t1d7);let t1d9: f64 = (t1d8 - 230.25850929940458);let t1da: f64 = (l.f737 * l.f645);let t1db: f64 = (0.5 * t1da);let t1dc: f64 = (t1db - 230.25850929940458);let t1dd: f64 = (l.f737 * l.f645);let t1de: f64 = (0.5 * t1dd);let t1df: f64 = (t1de - 230.25850929940458);let t1e0: f64 = (t1df * 0.3333333333333333);let t1e1: f64 = (1.0 + t1e0);let t1e2: f64 = (t1dc * t1e1);let t1e3: f64 = (0.5 * t1e2);let t1e4: f64 = (1.0 + t1e3);let t1e5: f64 = (t1d9 * t1e4);let t1e6: f64 = (1.0 + t1e5);let t1e7: f64 = (1e100 * t1e6);l.f825 = t1e7;l.f826 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) {let t1e8: f64 = (l.f5eb * l.f5eb);let t1e9: f64 = (t1e8 / l.f5df);l.f64f = t1e9;l.f650 = 0.0;let t1ea: f64 = (l.f5e5 / l.f645);let t1eb: f64 = (l.f5df / l.f64f);let t1ec: f64 = (t1eb).ln();let t1ed: f64 = (t1ea * t1ec);l.f793 = t1ed;l.f794 = 0.0;}
        let t1ef: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f4b5 = t1ef;l.f4b6 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t1f0: f64 = (l.f737 - l.f793);let t1f1: f64 = (p.p86 * t1f0);let t1f2: f64 = (t1f1 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t1f2, 0.0, 0.0, );l.f604 = 0.0;let t1f3: f64 = (p.p86 * l.f793);let t1f4: f64 = (l.f5e5 - t1f3);(l.f5ed, l.f5ee, l.f5ef, ) = (t1f4, 0.0, 0.0, );l.f5f0 = 0.0;let t1f5: f64 = (p.p85 - l.f601);let t1f6: f64 = (t1f5 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t1f6, (-l.f602), (-l.f603), );l.f6f6 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t1f7: f64 = (4.0 * p.p85);let t1f8: f64 = (t1f7 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t1f8, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {
            let (t1fa, t1fb, t1fc,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t1f9: f64 = (-l.f6f7);
        (t1f9, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t1fa, t1fb, t1fc, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t1fd: f64 = (l.f6f3 * l.f6f3);let t1fe: f64 = (t1fd + l.f6f7);let t1ff: f64 = (t1fe).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1ff, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1ff)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1ff)), );l.f6fa = 0.0;let t200: f64 = (l.f6f3 + l.f6f7);let t201: f64 = (0.5 * t200);let t202: f64 = (p.p85 - t201);(l.f605, l.f606, l.f607, ) = (t202, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t203: f64 = (l.f605 - l.f5e5);let t204: f64 = (t203 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t204, l.f606, l.f607, );l.f6f6 = 0.0;let t205: f64 = (4.0 * l.f5e5);let t206: f64 = (t205 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t206, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {
            let (t209, t20a, t20b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t208: f64 = (-l.f6f7);
        (t208, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t209, t20a, t20b, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t20c: f64 = (l.f6f3 * l.f6f3);let t20d: f64 = (t20c + l.f6f7);let t20e: f64 = (t20d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t20e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t20e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t20e)), );l.f6fa = 0.0;let t20f: f64 = (l.f6f3 + l.f6f7);let t210: f64 = (0.5 * t20f);let t211: f64 = (l.f5e5 + t210);(l.f5f1, l.f5f2, l.f5f3, ) = (t211, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t212: f64 = (p.p85 - l.f5ed);let t213: f64 = (t212 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t213, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t214: f64 = (4.0 * p.p85);let t215: f64 = (t214 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t215, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {
            let (t217, t218, t219,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t216: f64 = (-l.f6f7);
        (t216, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t217, t218, t219, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t21a: f64 = (l.f6f3 * l.f6f3);let t21b: f64 = (t21a + l.f6f7);let t21c: f64 = (t21b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t21c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t21c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t21c)), );l.f6fa = 0.0;let t21d: f64 = (l.f6f3 + l.f6f7);let t21e: f64 = (0.5 * t21d);let t21f: f64 = (p.p85 - t21e);(l.f5ed, l.f5ee, l.f5ef, ) = (t21f, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t220: f64 = (l.f5ed - l.f5e5);let t221: f64 = (t220 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t221, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t222: f64 = (4.0 * l.f5e5);let t223: f64 = (t222 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t223, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {
            let (t226, t227, t228,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t225: f64 = (-l.f6f7);
        (t225, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t226, t227, t228, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t229: f64 = (l.f6f3 * l.f6f3);let t22a: f64 = (t229 + l.f6f7);let t22b: f64 = (t22a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t22b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t22b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t22b)), );l.f6fa = 0.0;let t22c: f64 = (l.f6f3 + l.f6f7);let t22d: f64 = (0.5 * t22c);let t22e: f64 = (l.f5e5 + t22d);(l.f5ed, l.f5ee, l.f5ef, ) = (t22e, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );l.f5f4 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t22f: f64 = (l.f737 / l.f5f1);let t230: f64 = (l.f5f1 - l.f5ed);let t231: f64 = (l.f793 * t230);let t232: f64 = (l.f5ed * p.p85);let t233: f64 = (t231 / t232);let t234: f64 = (t22f + t233);let t235: f64 = (l.f645 * t234);let t236: f64 = (t235).abs();let t237: f64 = if t236 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4b7 = t237;l.f4b8 = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b7 != 0.0)) {let t238: f64 = (l.f737 / l.f5f1);let t239: f64 = (l.f5f1 - l.f5ed);let t23a: f64 = (l.f793 * t239);let t23b: f64 = (l.f5ed * p.p85);let t23c: f64 = (t23a / t23b);let t23d: f64 = (t238 + t23c);let t23e: f64 = (l.f645 * t23d);let t23f: f64 = (t23e).exp();(l.f536, l.f537, l.f538, ) = (t23f, (t23f * (l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t23b) - (t23a * (l.f5ee * p.p85))) / (t23b * t23b))))), (t23f * (l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t23b) - (t23a * (l.f5ef * p.p85))) / (t23b * t23b))))), );l.f539 = 0.0;}
        let t240: f64 = (l.f737 / l.f5f1);let t241: f64 = (l.f5f1 - l.f5ed);let t242: f64 = (l.f793 * t241);let t243: f64 = (l.f5ed * p.p85);let t244: f64 = (t242 / t243);let t245: f64 = (t240 + t244);let t246: f64 = (l.f645 * t245);let t247: f64 = (-230.25850929940458);let t248: f64 = if t246 < t247 { 1.0 } else { 0.0 };l.f4b9 = t248;l.f4ba = 0.0;
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b7 == 0.0)) && (l.f4b9 != 0.0)) {let t249: f64 = (-230.25850929940458);let t24a: f64 = (l.f737 / l.f5f1);let t24b: f64 = (l.f5f1 - l.f5ed);let t24c: f64 = (l.f793 * t24b);let t24d: f64 = (l.f5ed * p.p85);let t24e: f64 = (t24c / t24d);let t24f: f64 = (t24a + t24e);let t250: f64 = (l.f645 * t24f);let t251: f64 = (t249 - t250);let t252: f64 = (-230.25850929940458);let t253: f64 = (l.f737 / l.f5f1);let t254: f64 = (l.f5f1 - l.f5ed);let t255: f64 = (l.f793 * t254);let t256: f64 = (l.f5ed * p.p85);let t257: f64 = (t255 / t256);let t258: f64 = (t253 + t257);let t259: f64 = (l.f645 * t258);let t25a: f64 = (t252 - t259);let t25b: f64 = (-230.25850929940458);let t25c: f64 = (l.f737 / l.f5f1);let t25d: f64 = (l.f5f1 - l.f5ed);let t25e: f64 = (l.f793 * t25d);let t25f: f64 = (l.f5ed * p.p85);let t260: f64 = (t25e / t25f);let t261: f64 = (t25c + t260);let t262: f64 = (l.f645 * t261);let t263: f64 = (t25b - t262);let t264: f64 = (t263 * 0.3333333333333333);let t265: f64 = (1.0 + t264);let t266: f64 = (t25a * t265);let t267: f64 = (0.5 * t266);let t268: f64 = (1.0 + t267);let t269: f64 = (t251 * t268);let t26a: f64 = (1.0 + t269);let t26b: f64 = (1e-100 / t26a);(l.f536, l.f537, l.f538, ) = (t26b, (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t24d) - (t24c * (l.f5ee * p.p85))) / (t24d * t24d))))) * t268) + (t251 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t256) - (t255 * (l.f5ee * p.p85))) / (t256 * t256))))) * t265) + (t25a * ((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t25f) - (t25e * (l.f5ee * p.p85))) / (t25f * t25f))))) * 0.3333333333333333))))))) / (t26a * t26a))), (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t24d) - (t24c * (l.f5ef * p.p85))) / (t24d * t24d))))) * t268) + (t251 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t256) - (t255 * (l.f5ef * p.p85))) / (t256 * t256))))) * t265) + (t25a * ((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t25f) - (t25e * (l.f5ef * p.p85))) / (t25f * t25f))))) * 0.3333333333333333))))))) / (t26a * t26a))), );l.f539 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b7 == 0.0)) && (l.f4b9 == 0.0)) {let t26c: f64 = (l.f737 / l.f5f1);let t26d: f64 = (l.f5f1 - l.f5ed);let t26e: f64 = (l.f793 * t26d);let t26f: f64 = (l.f5ed * p.p85);let t270: f64 = (t26e / t26f);let t271: f64 = (t26c + t270);let t272: f64 = (l.f645 * t271);let t273: f64 = (t272 - 230.25850929940458);let t274: f64 = (l.f737 / l.f5f1);let t275: f64 = (l.f5f1 - l.f5ed);let t276: f64 = (l.f793 * t275);let t277: f64 = (l.f5ed * p.p85);let t278: f64 = (t276 / t277);let t279: f64 = (t274 + t278);let t27a: f64 = (l.f645 * t279);let t27b: f64 = (t27a - 230.25850929940458);let t27c: f64 = (l.f737 / l.f5f1);let t27d: f64 = (l.f5f1 - l.f5ed);let t27e: f64 = (l.f793 * t27d);let t27f: f64 = (l.f5ed * p.p85);let t280: f64 = (t27e / t27f);let t281: f64 = (t27c + t280);let t282: f64 = (l.f645 * t281);let t283: f64 = (t282 - 230.25850929940458);let t284: f64 = (t283 * 0.3333333333333333);let t285: f64 = (1.0 + t284);let t286: f64 = (t27b * t285);let t287: f64 = (0.5 * t286);let t288: f64 = (1.0 + t287);let t289: f64 = (t273 * t288);let t28a: f64 = (1.0 + t289);let t28b: f64 = (1e100 * t28a);(l.f536, l.f537, l.f538, ) = (t28b, (1e100 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t26f) - (t26e * (l.f5ee * p.p85))) / (t26f * t26f)))) * t288) + (t273 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t277) - (t276 * (l.f5ee * p.p85))) / (t277 * t277)))) * t285) + (t27b * ((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t27f) - (t27e * (l.f5ee * p.p85))) / (t27f * t27f)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t26f) - (t26e * (l.f5ef * p.p85))) / (t26f * t26f)))) * t288) + (t273 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t277) - (t276 * (l.f5ef * p.p85))) / (t277 * t277)))) * t285) + (t27b * ((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t27f) - (t27e * (l.f5ef * p.p85))) / (t27f * t27f)))) * 0.3333333333333333))))))), );l.f539 = 0.0;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) {let t28d: f64 = (l.f5eb * l.f5eb);let t28e: f64 = (t28d / l.f5e3);l.f64f = t28e;l.f650 = 0.0;let t28f: f64 = (l.f5e9 / l.f645);let t290: f64 = (l.f5e3 / l.f64f);let t291: f64 = (t290).ln();let t292: f64 = (t28f * t291);l.f793 = t292;l.f794 = 0.0;}
        let t293: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f4bb = t293;l.f4bc = 0.0;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t294: f64 = (l.f737 - l.f793);let t295: f64 = (p.p86 * t294);let t296: f64 = (t295 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t296, 0.0, 0.0, );l.f604 = 0.0;let t297: f64 = (p.p86 * l.f793);let t298: f64 = (l.f5e9 - t297);(l.f5ed, l.f5ee, l.f5ef, ) = (t298, 0.0, 0.0, );l.f5f0 = 0.0;let t299: f64 = (p.p85 - l.f601);let t29a: f64 = (t299 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t29a, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t29b: f64 = (4.0 * p.p85);let t29c: f64 = (t29b * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t29c, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {
            let (t29e, t29f, t2a0,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t29d: f64 = (-l.f6f7);
        (t29d, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t29e, t29f, t2a0, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t2a1: f64 = (l.f6f3 * l.f6f3);let t2a2: f64 = (t2a1 + l.f6f7);let t2a3: f64 = (t2a2).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2a3, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2a3)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2a3)), );l.f6fa = 0.0;let t2a4: f64 = (l.f6f3 + l.f6f7);let t2a5: f64 = (0.5 * t2a4);let t2a6: f64 = (p.p85 - t2a5);(l.f605, l.f606, l.f607, ) = (t2a6, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t2a8: f64 = (l.f605 - l.f5e9);let t2a9: f64 = (t2a8 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2a9, l.f606, l.f607, );l.f6f6 = 0.0;let t2aa: f64 = (4.0 * l.f5e9);let t2ab: f64 = (t2aa * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2ab, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {
            let (t2ad, t2ae, t2af,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2ac: f64 = (-l.f6f7);
        (t2ac, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2ad, t2ae, t2af, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t2b0: f64 = (l.f6f3 * l.f6f3);let t2b1: f64 = (t2b0 + l.f6f7);let t2b2: f64 = (t2b1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2b2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2b2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2b2)), );l.f6fa = 0.0;}
    }
}
