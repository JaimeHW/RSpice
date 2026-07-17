#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        let t0: f64 = (8.8541878176e-12 * 11.8);l.f6b = t0;let t40: f64 = (-250.0);
        let (t42,) = {
    if (p.p6 > t40) {
        (p.p6,)
    } else {
        let t41: f64 = (-250.0);
        (t41,)
    }
};
        l.f705 = t42;let t137: f64 = if ((!param_given[6]) && param_given[96]) { 1.0 } else { 0.0 };l.fdb = t137;
        if (l.fdb != 0.0) {
            let t1b2: f64 = (-250.0);
            let (t1b4,) = {
    if (p.p96 > t1b2) {
        (p.p96,)
    } else {
        let t1b3: f64 = (-250.0);
        (t1b3,)
    }
};
            l.f705 = t1b4;
        }
        let (t237,) = {
    if (p.p5 > 1e-12) {
        (p.p5,)
    } else {
        (1e-12,)
    }
};
        l.f57a = t237;
        let (t3e8,) = {
    if (p.p8 > 1e-12) {
        (p.p8,)
    } else {
        (1e-12,)
    }
};
        l.f30 = t3e8;
        let (t3eb,) = {
    if (p.p9 > 1e-18) {
        (p.p9,)
    } else {
        (1e-18,)
    }
};
        l.f34 = t3eb;
        let (t3ef,) = {
    if (p.p10 > 1e-18) {
        (p.p10,)
    } else {
        (1e-18,)
    }
};
        l.f32 = t3ef;
        let (t3f4,) = {
    if (p.p11 > 0.05) {
        (p.p11,)
    } else {
        (0.05,)
    }
};
        l.f771 = t3f4;
        let (t403,) = {
    if (p.p12 > 0.05) {
        (p.p12,)
    } else {
        (0.05,)
    }
};
        l.f779 = t403;
        let (t9,) = {
    if (p.p13 > 0.05) {
        (p.p13,)
    } else {
        (0.05,)
    }
};
        l.f775 = t9;
        let (t49,) = {
    if (p.p14 > 0.05) {
        let (t48,) = {
            if (p.p14 < 0.95) {
                (p.p14,)
            } else {
                (0.95,)
            }
        };
        (t48,)
    } else {
        (0.05,)
    }
};
        l.f623 = t49;
        let (t67,) = {
    if (p.p15 > 0.05) {
        let (t66,) = {
            if (p.p15 < 0.95) {
                (p.p15,)
            } else {
                (0.95,)
            }
        };
        (t66,)
    } else {
        (0.05,)
    }
};
        l.f653 = t67;
        let (t7c,) = {
    if (p.p16 > 0.05) {
        let (t7b,) = {
            if (p.p16 < 0.95) {
                (p.p16,)
            } else {
                (0.95,)
            }
        };
        (t7b,)
    } else {
        (0.05,)
    }
};
        l.f62f = t7c;l.f631 = p.p17;l.f641 = p.p18;l.f639 = p.p19;
        let (te6,) = {
    if (p.p20 > 0.0) {
        (p.p20,)
    } else {
        (0.0,)
    }
};
        l.f546 = te6;
        let (tfa,) = {
    if (p.p21 > 0.0) {
        (p.p21,)
    } else {
        (0.0,)
    }
};
        l.f54a = tfa;
        let (t10d,) = {
    if (p.p22 > 0.0) {
        (p.p22,)
    } else {
        (0.0,)
    }
};
        l.f548 = t10d;
        let (t11d,) = {
    if (p.p23 > 0.0) {
        (p.p23,)
    } else {
        (0.0,)
    }
};
        l.f39 = t11d;
        let (t138,) = {
    if (p.p24 > 0.0) {
        (p.p24,)
    } else {
        (0.0,)
    }
};
        l.f3d = t138;
        let (t151,) = {
    if (p.p25 > 0.0) {
        (p.p25,)
    } else {
        (0.0,)
    }
};
        l.f3b = t151;
        let (t17b,) = {
    if (p.p26 > 1e-9) {
        (p.p26,)
    } else {
        (1e-9,)
    }
};
        l.f80c = t17b;
        let (t181,) = {
    if (p.p27 > 1e-9) {
        (p.p27,)
    } else {
        (1e-9,)
    }
};
        l.f80a = t181;
        let (t18f,) = {
    if (p.p28 > 0.0) {
        (p.p28,)
    } else {
        (0.0,)
    }
};
        l.f3f = t18f;
        let (t19a,) = {
    if (p.p29 > 0.0) {
        (p.p29,)
    } else {
        (0.0,)
    }
};
        l.f43 = t19a;
        let (t1a0,) = {
    if (p.p30 > 0.0) {
        (p.p30,)
    } else {
        (0.0,)
    }
};
        l.f41 = t1a0;
        let (t1a1,) = {
    if (p.p31 > 0.01) {
        (p.p31,)
    } else {
        (0.01,)
    }
};
        l.f5c3 = t1a1;
        let (t1a2,) = {
    if (p.p32 > 0.01) {
        (p.p32,)
    } else {
        (0.01,)
    }
};
        l.f5c7 = t1a2;
        let (t1af,) = {
    if (p.p33 > 0.01) {
        (p.p33,)
    } else {
        (0.01,)
    }
};
        l.f5c5 = t1af;
        let (t1b5,) = {
    if (p.p34 > 0.0) {
        (p.p34,)
    } else {
        (0.0,)
    }
};
        l.f24 = t1b5;
        let (t1ce,) = {
    if (p.p35 > 0.0) {
        (p.p35,)
    } else {
        (0.0,)
    }
};
        l.f28 = t1ce;
        let (t1d3,) = {
    if (p.p36 > 0.0) {
        (p.p36,)
    } else {
        (0.0,)
    }
};
        l.f26 = t1d3;l.fa5 = p.p37;l.fa9 = p.p38;l.fa7 = p.p39;l.f6c6 = p.p40;l.f6ca = p.p41;l.f6c8 = p.p42;
        let (t220,) = {
    if (p.p43 > 0.1) {
        (p.p43,)
    } else {
        (0.1,)
    }
};
        l.f783 = t220;
        let (t23a,) = {
    if (p.p44 > 0.1) {
        (p.p44,)
    } else {
        (0.1,)
    }
};
        l.f78d = t23a;
        let (t257,) = {
    if (p.p45 > 0.1) {
        (p.p45,)
    } else {
        (0.1,)
    }
};
        l.f785 = t257;
        let (t2bf,) = {
    if (p.p46 > 0.1) {
        (p.p46,)
    } else {
        (0.1,)
    }
};
        l.f625 = t2bf;
        let (t2da,) = {
    if (p.p47 > 0.1) {
        (p.p47,)
    } else {
        (0.1,)
    }
};
        l.f629 = t2da;
        let (t2f7,) = {
    if (p.p48 > 0.1) {
        (p.p48,)
    } else {
        (0.1,)
    }
};
        l.f627 = t2f7;l.fc1 = p.p7;
        let (t377,) = {
    if (p.p49 > 0.0) {
        (p.p49,)
    } else {
        (0.0,)
    }
};
        l.f6b0 = t377;
        let (t394,) = {
    if (p.p50 > 0.0) {
        (p.p50,)
    } else {
        (0.0,)
    }
};
        l.f6b3 = t394;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (t3b2,) = {
    if (p.p51 > 0.0) {
        (p.p51,)
    } else {
        (0.0,)
    }
};
        l.f6b2 = t3b2;
        let (t3e7,) = {
    if (p.p52 > 0.0) {
        (p.p52,)
    } else {
        (0.0,)
    }
};
        l.f6b1 = t3e7;
        let (t3e9,) = {
    if (p.p53 > 0.0) {
        (p.p53,)
    } else {
        (0.0,)
    }
};
        l.f6cc = t3e9;
        let (t3ea,) = {
    if (p.p56 > 0.0) {
        (p.p56,)
    } else {
        (0.0,)
    }
};
        l.f711 = t3ea;l.f6cd = p.p57;l.f6cf = p.p58;l.f6d5 = p.p59;l.f6d7 = p.p60;l.f6d1 = p.p61;l.f6d3 = p.p62;
        let (t3ec,) = {
    if (p.p63 > 0.1) {
        (p.p63,)
    } else {
        (0.1,)
    }
};
        l.f5e5 = t3ec;
        let (t3ed,) = {
    if (p.p64 > 0.1) {
        (p.p64,)
    } else {
        (0.1,)
    }
};
        l.f5e9 = t3ed;
        let (t3ee,) = {
    if (p.p65 > 0.1) {
        (p.p65,)
    } else {
        (0.1,)
    }
};
        l.f5e7 = t3ee;
        let (t3f0,) = {
    if (p.p76 > 0.1) {
        (p.p76,)
    } else {
        (0.1,)
    }
};
        l.f80e = t3f0;
        let (t3f1,) = {
    if (p.p77 > 0.0) {
        (p.p77,)
    } else {
        (0.0,)
    }
};
        l.f6b4 = t3f1;
        let (t3f2,) = {
    if (p.p78 > 0.0) {
        (p.p78,)
    } else {
        (0.0,)
    }
};
        l.f6b6 = t3f2;l.f6d9 = 0.0;let t3f3: f64 = if p.p81 > 0.5 { 1.0 } else { 0.0 };l.f1ad = t3f3;
        if (l.f1ad != 0.0) {l.f6d9 = 1.0;}
        if (l.f1ad == 0.0) {l.f6d9 = 0.0;}
        let (t3f5,) = {
    if (p.p82 > 0.5) {
        (p.p82,)
    } else {
        (0.5,)
    }
};
        l.f7ab = t3f5;
        let (t3f6,) = {
    if (p.p83 > 0.0) {
        (p.p83,)
    } else {
        (0.0,)
    }
};
        l.fb3 = t3f6;let t3f7: f64 = (273.15 + l.f705);l.f6e9 = t3f7;let t3f8: f64 = ctx_temp;let t3f9: f64 = (t3f8 + p.p102);let t3fa: f64 = (-250.0);let t3fb: f64 = (273.15 + t3fa);let t3fc: f64 = (t3f9).max(t3fb);l.f6e7 = t3fc;let t3fd: f64 = (l.f6e7 / l.f6e9);l.f14 = t3fd;let t3fe: f64 = (1.3806505e-23 / 1.6021918e-19);l.f5a5 = t3fe;let t3ff: f64 = (l.f5a5 * l.f6e9);l.f647 = t3ff;let t400: f64 = (1.0 / l.f647);l.f649 = t400;let t401: f64 = (l.f5a5 * l.f6e7);l.f643 = t401;let t402: f64 = (1.0 / l.f643);l.f645 = t402;let t404: f64 = (0.000702 * l.f6e9);let t405: f64 = (t404 * l.f6e9);let t406: f64 = (-t405);let t407: f64 = (1108.0 + l.f6e9);let t408: f64 = (t406 / t407);l.f4f = t408;let t409: f64 = (l.f631 + l.f4f);l.f63b = t409;let t40a: f64 = (l.f641 + l.f4f);l.f63f = t40a;let t40b: f64 = (l.f639 + l.f4f);l.f63d = t40b;let t40c: f64 = (0.000702 * l.f6e7);let t40d: f64 = (t40c * l.f6e7);let t40e: f64 = (-t40d);let t40f: f64 = (1108.0 + l.f6e7);let t410: f64 = (t40e / t40f);l.f4d = t410;let t411: f64 = (l.f631 + l.f4d);l.f633 = t411;let t412: f64 = (l.f641 + l.f4d);l.f637 = t412;let t413: f64 = (l.f639 + l.f4d);l.f635 = t413;let t414: f64 = (l.f80e / 2.0);let t415: f64 = (l.f14).powf(t414);let t416: f64 = (l.f63b * l.f649);let t417: f64 = (l.f633 * l.f645);let t418: f64 = (t416 - t417);let t419: f64 = (0.5 * t418);let t41a: f64 = (t419).exp();let t41b: f64 = (t415 * t41a);l.fc9 = t41b;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        l: &mut StampLocals,
    ) {
        let t1: f64 = (l.f80e / 2.0);let t2: f64 = (l.f14).powf(t1);let t3: f64 = (l.f63f * l.f649);let t4: f64 = (l.f637 * l.f645);let t5: f64 = (t3 - t4);let t6: f64 = (0.5 * t5);let t7: f64 = (t6).exp();let t8: f64 = (t2 * t7);l.fd1 = t8;let ta: f64 = (l.f80e / 2.0);let tb: f64 = (l.f14).powf(ta);let tc: f64 = (l.f63d * l.f649);let td: f64 = (l.f635 * l.f645);let te: f64 = (tc - td);let tf: f64 = (0.5 * te);let t10: f64 = (tf).exp();let t11: f64 = (tb * t10);l.fcd = t11;let t12: f64 = (l.f80e / 2.0);let t13: f64 = (t12 / l.f5e5);let t14: f64 = (l.f14).powf(t13);let t15: f64 = (l.f63b * l.f649);let t16: f64 = (l.f633 * l.f645);let t17: f64 = (t15 - t16);let t18: f64 = (0.5 * t17);let t19: f64 = (t18 / l.f5e5);let t1a: f64 = (t19).exp();let t1b: f64 = (t14 * t1a);l.fca = t1b;let t1c: f64 = (l.f80e / 2.0);let t1d: f64 = (t1c / l.f5e9);let t1e: f64 = (l.f14).powf(t1d);let t1f: f64 = (l.f63f * l.f649);let t20: f64 = (l.f637 * l.f645);let t21: f64 = (t1f - t20);let t22: f64 = (0.5 * t21);let t23: f64 = (t22 / l.f5e9);let t24: f64 = (t23).exp();let t25: f64 = (t1e * t24);l.fd2 = t25;let t26: f64 = (l.f80e / 2.0);let t27: f64 = (t26 / l.f5e7);let t28: f64 = (l.f14).powf(t27);let t29: f64 = (l.f63d * l.f649);let t2a: f64 = (l.f635 * l.f645);let t2b: f64 = (t29 - t2a);let t2c: f64 = (0.5 * t2b);let t2d: f64 = (t2c / l.f5e7);let t2e: f64 = (t2d).exp();let t2f: f64 = (t28 * t2e);l.fce = t2f;let t30: f64 = (l.f546 * l.fca);let t31: f64 = (t30 * l.fca);l.f542 = t31;let t32: f64 = (l.f54a * l.fd2);let t33: f64 = (t32 * l.fd2);l.f54c = t33;let t34: f64 = (l.f548 * l.fce);let t35: f64 = (t34 * l.fce);l.f544 = t35;let t36: f64 = (l.f771 * l.f14);let t37: f64 = (2.0 * l.f643);let t38: f64 = (l.fc9).ln();let t39: f64 = (t37 * t38);let t3a: f64 = (t36 - t39);l.f71f = t3a;let t3b: f64 = (l.f779 * l.f14);let t3c: f64 = (2.0 * l.f643);let t3d: f64 = (l.fd1).ln();let t3e: f64 = (t3c * t3d);let t3f: f64 = (t3b - t3e);l.f723 = t3f;let t43: f64 = (l.f775 * l.f14);let t44: f64 = (2.0 * l.f643);let t45: f64 = (l.fcd).ln();let t46: f64 = (t44 * t45);let t47: f64 = (t43 - t46);l.f721 = t47;let t4a: f64 = (0.05 - l.f71f);let t4b: f64 = (t4a * l.f645);let t4c: f64 = (t4b).exp();let t4d: f64 = (1.0 + t4c);let t4e: f64 = (t4d).ln();let t4f: f64 = (l.f643 * t4e);let t50: f64 = (l.f71f + t4f);
        l.f75d = t50;let t51: f64 = (0.05 - l.f723);let t52: f64 = (t51 * l.f645);let t53: f64 = (t52).exp();let t54: f64 = (1.0 + t53);let t55: f64 = (t54).ln();let t56: f64 = (l.f643 * t55);let t57: f64 = (l.f723 + t56);l.f77d = t57;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        l: &mut StampLocals,
    ) {
        let t58: f64 = (0.05 - l.f721);let t59: f64 = (t58 * l.f645);let t5a: f64 = (t59).exp();let t5b: f64 = (1.0 + t5a);let t5c: f64 = (t5b).ln();let t5d: f64 = (l.f643 * t5c);let t5e: f64 = (l.f721 + t5d);l.f763 = t5e;let t5f: f64 = (1.0 / l.f75d);l.f769 = t5f;let t60: f64 = (1.0 / l.f77d);l.f76d = t60;let t61: f64 = (1.0 / l.f763);l.f76b = t61;let t62: f64 = (1.0 - l.f623);l.f60b = t62;let t63: f64 = (1.0 - l.f653);l.f60f = t63;let t64: f64 = (1.0 - l.f62f);l.f60d = t64;let t65: f64 = (1.0 / l.f60b);l.f611 = t65;let t68: f64 = (1.0 / l.f60f);l.f615 = t68;let t69: f64 = (1.0 / l.f60d);l.f613 = t69;let t6a: f64 = (l.f771 * l.f769);let t6b: f64 = (t6a).powf(l.f623);let t6c: f64 = (l.f30 * t6b);l.f2c = t6c;let t6d: f64 = (l.f779 * l.f76d);let t6e: f64 = (t6d).powf(l.f653);let t6f: f64 = (l.f34 * t6e);l.f36 = t6f;let t70: f64 = (l.f775 * l.f76b);let t71: f64 = (t70).powf(l.f62f);let t72: f64 = (l.f32 * t71);l.f2e = t72;let t73: f64 = (l.f2c * l.f75d);let t74: f64 = (t73 * l.f611);l.f69e = t74;let t75: f64 = (l.f36 * l.f77d);let t76: f64 = (t75 * l.f615);l.f6a2 = t76;let t77: f64 = (l.f2e * l.f763);let t78: f64 = (t77 * l.f613);l.f6a0 = t78;let t79: f64 = (2.0 * l.f2c);l.f698 = t79;let t7a: f64 = (2.0 * l.f36);l.f69c = t7a;let t7d: f64 = (2.0 * l.f2e);l.f69a = t7d;let t7e: f64 = (l.f6b / l.f30);l.f7d6 = t7e;let t7f: f64 = (l.f80c * l.f6b);let t80: f64 = (t7f / l.f34);l.f7e0 = t80;let t81: f64 = (l.f80a * l.f6b);let t82: f64 = (t81 / l.f32);l.f7d8 = t82;let t83: f64 = (1.0 / l.f7d6);l.f7da = t83;let t84: f64 = (1.0 / l.f7e0);l.f7de = t84;let t85: f64 = (1.0 / l.f7d8);l.f7dc = t85;let t86: f64 = (1.0 / l.f771);l.f773 = t86;let t87: f64 = (1.0 / l.f779);l.f77b = t87;let t88: f64 = (1.0 / l.f775);l.f777 = t88;let t89: f64 = (1.772453850905516 * 0.29214664);l.f62b = t89;let t8a: f64 = (-5.0);let t8b: f64 = (t8a * 0.29214664);let t8c: f64 = (t8b + 6.0);let t8d: f64 = (-2.0);let t8e: f64 = (l.f62b).powf(t8d);let t8f: f64 = (t8c - t8e);let t90: f64 = (t8f / 3.0);l.f16 = t90;let t91: f64 = (1.0 - 0.29214664);let t92: f64 = (t91 - l.f16);l.f2a = t92;let t93: f64 = (0.5 * l.f633);let t94: f64 = (t93).max(l.f643);l.f47 = t94;let t95: f64 = (0.5 * l.f637);let t96: f64 = (t95).max(l.f643);l.f4b = t96;let t97: f64 = (0.5 * l.f635);let t98: f64 = (t97).max(l.f643);l.f49 = t98;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        l: &mut StampLocals,
    ) {
        let t99: f64 = (l.f47 * l.f645);l.fe = t99;let t9a: f64 = (l.f4b * l.f645);l.f12 = t9a;let t9b: f64 = (l.f49 * l.f645);l.f10 = t9b;let t9c: f64 = (32.0 * l.f5c3);let t9d: f64 = (t9c * 9.1093826e-31);let t9e: f64 = (t9d * 1.6021918e-19);let t9f: f64 = (l.f47 * l.f47);let ta0: f64 = (t9f * l.f47);let ta1: f64 = (t9e * ta0);let ta2: f64 = (ta1).sqrt();let ta3: f64 = (3.0 * 1.05457168e-34);let ta4: f64 = (ta2 / ta3);l.f1e = ta4;let ta5: f64 = (32.0 * l.f5c7);let ta6: f64 = (ta5 * 9.1093826e-31);let ta7: f64 = (ta6 * 1.6021918e-19);let ta8: f64 = (l.f4b * l.f4b);let ta9: f64 = (ta8 * l.f4b);let taa: f64 = (ta7 * ta9);let tab: f64 = (taa).sqrt();let tac: f64 = (3.0 * 1.05457168e-34);let tad: f64 = (tab / tac);l.f22 = tad;let tae: f64 = (32.0 * l.f5c5);let taf: f64 = (tae * 9.1093826e-31);let tb0: f64 = (taf * 1.6021918e-19);let tb1: f64 = (l.f49 * l.f49);let tb2: f64 = (tb1 * l.f49);let tb3: f64 = (tb0 * tb2);let tb4: f64 = (tb3).sqrt();let tb5: f64 = (3.0 * 1.05457168e-34);let tb6: f64 = (tb4 / tb5);l.f20 = tb6;let tb7: f64 = (l.f6e7 - l.f6e9);let tb8: f64 = (l.f6c6 * tb7);let tb9: f64 = (1.0 + tb8);let tba: f64 = (l.fa5 * tb9);l.fa1 = tba;let tbb: f64 = (l.f6e7 - l.f6e9);let tbc: f64 = (l.f6ca * tbb);let tbd: f64 = (1.0 + tbc);let tbe: f64 = (l.fa9 * tbd);l.fab = tbe;let tbf: f64 = (l.f6e7 - l.f6e9);let tc0: f64 = (l.f6c8 * tbf);let tc1: f64 = (1.0 + tc0);let tc2: f64 = (l.fa7 * tc1);l.fa3 = tc2;
        if (!(l.fa1 > 0.0)) {l.fa1 = 0.0;}
        if (!(l.fab > 0.0)) {l.fab = 0.0;}
        if (!(l.fa3 > 0.0)) {l.fa3 = 0.0;}
        let tc3: f64 = (l.fc1 - 1.0);let tc4: f64 = (tc3 / l.fc1);l.f2 = tc4;let tc5: f64 = (l.f2).powf(l.f625);let tc6: f64 = (1.0 - tc5);let tc7: f64 = (1.0 / tc6);l.fc3 = tc7;let tc8: f64 = (l.f2).powf(l.f629);let tc9: f64 = (1.0 - tc8);let tca: f64 = (1.0 / tc9);l.fc7 = tca;let tcb: f64 = (l.f2).powf(l.f627);let tcc: f64 = (1.0 - tcb);let tcd: f64 = (1.0 / tcc);l.fc5 = tcd;let tce: f64 = (l.f6e7 - l.f6e9);let tcf: f64 = (l.f6e7 - l.f6e9);let td0: f64 = (tcf * l.f6cf);let td1: f64 = (l.f6cd + td0);let td2: f64 = (tce * td1);let td3: f64 = (1.0 + td2);let td4: f64 = (l.f783 * td3);l.f783 = td4;let td5: f64 = (l.f6e7 - l.f6e9);let td6: f64 = (l.f6e7 - l.f6e9);let td7: f64 = (td6 * l.f6d7);let td8: f64 = (l.f6d5 + td7);let td9: f64 = (td5 * td8);let tda: f64 = (1.0 + td9);let tdb: f64 = (l.f78d * tda);l.f78d = tdb;let tdc: f64 = (l.f6e7 - l.f6e9);let tdd: f64 = (l.f6e7 - l.f6e9);let tde: f64 = (tdd * l.f6d3);let tdf: f64 = (l.f6d1 + tde);let te0: f64 = (tdc * tdf);let te1: f64 = (1.0 + te0);let te2: f64 = (l.f785 * te1);l.f785 = te2;let te3: f64 = if l.f783 <= 0.1 { 1.0 } else { 0.0 };l.f283 = te3;
        if (l.f283 != 0.0) {l.f783 = 0.1;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f283 != 0.0) {l.f787 = 10.0;}
        if (l.f283 == 0.0) {let te4: f64 = (1.0 / l.f783);l.f787 = te4;}
        let te5: f64 = if l.f78d <= 0.1 { 1.0 } else { 0.0 };l.f351 = te5;
        if (l.f351 != 0.0) {l.f78d = 0.1;l.f78b = 10.0;}
        if (l.f351 == 0.0) {let te7: f64 = (1.0 / l.f78d);l.f78b = te7;}
        let te8: f64 = if l.f785 <= 0.1 { 1.0 } else { 0.0 };l.f41b = te8;
        if (l.f41b != 0.0) {l.f785 = 0.1;l.f789 = 10.0;}
        if (l.f41b == 0.0) {let te9: f64 = (1.0 / l.f785);l.f789 = te9;}
        let tea: f64 = (0.01 * l.f6b6);let teb: f64 = (1.0 - tea);l.f6b8 = teb;let tec: f64 = (l.fc3 * l.fc3);let ted: f64 = (l.f625 - 1.0);let tee: f64 = (l.f2).powf(ted);let tef: f64 = (tec * tee);let tf0: f64 = (-tef);let tf1: f64 = (tf0 * l.f625);let tf2: f64 = (tf1 * l.f787);l.f6ba = tf2;let tf3: f64 = (l.fc7 * l.fc7);let tf4: f64 = (l.f629 - 1.0);let tf5: f64 = (l.f2).powf(tf4);let tf6: f64 = (tf3 * tf5);let tf7: f64 = (-tf6);let tf8: f64 = (tf7 * l.f629);let tf9: f64 = (tf8 * l.f78b);l.f6be = tf9;let tfb: f64 = (l.fc5 * l.fc5);let tfc: f64 = (l.f627 - 1.0);let tfd: f64 = (l.f2).powf(tfc);let tfe: f64 = (tfb * tfd);let tff: f64 = (-tfe);let t100: f64 = (tff * l.f627);let t101: f64 = (t100 * l.f789);l.f6bc = t101;let t102: f64 = (l.f14).powf(l.f6cc);let t103: f64 = (l.f6b0 * t102);l.f6ab = t103;let t104: f64 = (l.f14).powf(l.f6cc);let t105: f64 = (l.f6b2 * t104);l.f6ad = t105;let t106: f64 = (l.f14).powf(l.f6cc);let t107: f64 = (l.f6b3 * t106);l.f6ae = t107;let t108: f64 = (l.f14).powf(l.f6cc);let t109: f64 = (l.f6b1 * t108);l.f6ac = t109;let t10a: f64 = (p.p87 * 1000000.0);l.f5df = t10a;let t10b: f64 = (p.p89 * 1000000.0);l.f5e3 = t10b;let t10c: f64 = (p.p88 * 1000000.0);l.f5e1 = t10c;l.f5dd = l.f5df;l.f609 = l.f5e5;let t10e: f64 = (1450.0 * 0.0001);l.f5d9 = t10e;let t10f: f64 = (500.0 * 0.0001);l.f5db = t10f;l.f61f = 0.6;l.f5a3 = 0.001;let t110: f64 = (1.45e16 * l.fca);l.f5eb = t110;let t111: f64 = (l.f5eb * l.f5eb);let t112: f64 = (t111 / l.f5dd);l.f64d = t112;let t113: f64 = (-1.5);let t114: f64 = (l.f14).powf(t113);l.f6db = t114;let t115: f64 = (l.f5d9 * l.f6db);let t116: f64 = (t115 / l.f645);l.f59 = t116;let t117: f64 = (l.f5db * l.f6db);let t118: f64 = (t117 / l.f645);l.f5f = t118;let t119: f64 = (2.0 * l.f59);let t11a: f64 = (t119 * l.f5f);let t11b: f64 = (l.f59 + l.f5f);let t11c: f64 = (t11a / t11b);l.f45 = t11c;let t11e: f64 = (l.f14).powf(p.p97);l.f6dd = t11e;let t11f: f64 = (p.p93 * l.f6dd);l.f6df = t11f;let t120: f64 = (l.f6df * l.f45);let t121: f64 = (t120).sqrt();l.f5ad = t121;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t122: f64 = (l.f609 / l.f645);let t123: f64 = (l.f5dd / l.f64d);let t124: f64 = (t123).ln();let t125: f64 = (t122 * t124);l.f741 = t125;let t126: f64 = (l.f609 / l.f645);let t127: f64 = (l.f5dd / l.f64d);let t128: f64 = (t127).ln();let t129: f64 = (p.p94 / l.f5ad);let t12a: f64 = (t128 + t129);let t12b: f64 = (t126 * t12a);l.f743 = t12b;
        let (t12c,) = {
    if (p.p99 > 0.0) {
        (p.p99,)
    } else {
        (0.0,)
    }
};
        let t12d: f64 = (t12c * l.f6b4);let t12e: f64 = (t12d * l.f6b4);let t12f: f64 = (t12e * l.f6b8);let t130: f64 = (t12f * l.f6b8);l.f0 = t130;
        let (t131,) = {
    if (p.p100 > 0.0) {
        (p.p100,)
    } else {
        (0.0,)
    }
};
        let t132: f64 = (t131 * l.f6b4);let t133: f64 = (t132 * l.f6b8);l.f5b1 = t133;
        let (t134,) = {
    if (p.p101 > 0.0) {
        (p.p101,)
    } else {
        (0.0,)
    }
};
        let t135: f64 = (t134 * l.f6b4);let t136: f64 = (t135 * l.f6b8);l.f5af = t136;l.f89 = 0.0;(l.f8a, l.f8b, l.f8c, ) = (0.0, 0.0, 0.0, );(l.f93, l.f94, l.f95, ) = (0.0, 0.0, 0.0, );(l.f8e, l.f8f, l.f90, ) = (0.0, 0.0, 0.0, );let t139: f64 = (l.f542 * l.f0);let t13a: f64 = if t139 > 0.0 { 1.0 } else { 0.0 };l.f4a8 = t13a;
        if (l.f4a8 != 0.0) {let t13b: f64 = (l.f542 * l.f0);let t13c: f64 = (l.f57a / t13b);let t13d: f64 = (t13c + 1.0);let t13e: f64 = (t13d).ln();let t13f: f64 = (l.f643 * t13e);let t140: f64 = (t13f * l.f5e5);l.f7b3 = t140;}
        if (l.f4a8 == 0.0) {l.f7b3 = 100000000.0;}
        let t141: f64 = (l.f54c * l.f5b1);let t142: f64 = if t141 > 0.0 { 1.0 } else { 0.0 };l.f4aa = t142;
        if (l.f4aa != 0.0) {let t143: f64 = (l.f54c * l.f5b1);let t144: f64 = (l.f57a / t143);let t145: f64 = (t144 + 1.0);let t146: f64 = (t145).ln();let t147: f64 = (l.f643 * t146);let t148: f64 = (t147 * l.f5e9);l.f7b7 = t148;}
        if (l.f4aa == 0.0) {l.f7b7 = 100000000.0;}
        let t149: f64 = (l.f544 * l.f5af);let t14a: f64 = if t149 > 0.0 { 1.0 } else { 0.0 };l.f4b4 = t14a;
        if (l.f4b4 != 0.0) {let t14b: f64 = (l.f544 * l.f5af);let t14c: f64 = (l.f57a / t14b);let t14d: f64 = (t14c + 1.0);let t14e: f64 = (t14d).ln();let t14f: f64 = (l.f643 * t14e);let t150: f64 = (t14f * l.f5e7);l.f7b5 = t150;}
        if (l.f4b4 == 0.0) {l.f7b5 = 100000000.0;}
        let t152: f64 = (l.f7b3).min(l.f7b7);let t153: f64 = (t152).min(l.f7b5);l.f7b1 = t153;let t154: f64 = (l.f7b1 * l.f645);let t155: f64 = (t154).abs();let t156: f64 = if t155 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4ca = t156;
        if (l.f4ca != 0.0) {let t157: f64 = (l.f7b1 * l.f645);let t158: f64 = (t157).exp();l.f89 = t158;}
        let t159: f64 = (l.f7b1 * l.f645);let t15a: f64 = (-230.25850929940458);let t15b: f64 = if t159 < t15a { 1.0 } else { 0.0 };l.fdc = t15b;
        if ((l.f4ca == 0.0) && (l.fdc != 0.0)) {let t15c: f64 = (-230.25850929940458);let t15d: f64 = (l.f7b1 * l.f645);let t15e: f64 = (t15c - t15d);let t15f: f64 = (-230.25850929940458);let t160: f64 = (l.f7b1 * l.f645);let t161: f64 = (t15f - t160);let t162: f64 = (-230.25850929940458);let t163: f64 = (l.f7b1 * l.f645);let t164: f64 = (t162 - t163);let t165: f64 = (t164 * 0.3333333333333333);let t166: f64 = (1.0 + t165);let t167: f64 = (t161 * t166);let t168: f64 = (0.5 * t167);let t169: f64 = (1.0 + t168);let t16a: f64 = (t15e * t169);let t16b: f64 = (1.0 + t16a);let t16c: f64 = (1e-100 / t16b);l.f89 = t16c;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        l: &mut StampLocals,
    ) {
        if ((l.f4ca == 0.0) && (l.fdc == 0.0)) {let t16d: f64 = (l.f7b1 * l.f645);let t16e: f64 = (t16d - 230.25850929940458);let t16f: f64 = (l.f7b1 * l.f645);let t170: f64 = (t16f - 230.25850929940458);let t171: f64 = (l.f7b1 * l.f645);let t172: f64 = (t171 - 230.25850929940458);let t173: f64 = (t172 * 0.3333333333333333);let t174: f64 = (1.0 + t173);let t175: f64 = (t170 * t174);let t176: f64 = (0.5 * t175);let t177: f64 = (1.0 + t176);let t178: f64 = (t16e * t177);let t179: f64 = (1.0 + t178);let t17a: f64 = (1e100 * t179);l.f89 = t17a;}
        l.f75e = l.f75d;l.f77e = l.f77d;l.f764 = l.f763;l.f621 = l.f623;l.f651 = l.f653;l.f62d = l.f62f;l.f760 = l.f771;l.f780 = l.f779;l.f766 = l.f775;let t17c: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.ff2 = t17c;
        if (l.ff2 != 0.0) {let t17d: f64 = (l.f77d + l.f763);l.f75e = t17d;let t17e: f64 = (l.f653).min(l.f62f);let t17f: f64 = (0.9 * t17e);l.f621 = t17f;let t180: f64 = (l.f779 + l.f775);l.f760 = t180;}
        let t182: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f108 = t182;
        if (l.f108 != 0.0) {let t183: f64 = (l.f75d + l.f763);l.f77e = t183;let t184: f64 = (l.f623).min(l.f62f);let t185: f64 = (0.9 * t184);l.f651 = t185;let t186: f64 = (l.f771 + l.f775);l.f780 = t186;}
        let t187: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f11e = t187;
        if (l.f11e != 0.0) {let t188: f64 = (l.f75d + l.f77d);l.f764 = t188;let t189: f64 = (l.f623).min(l.f653);let t18a: f64 = (0.9 * t189);l.f62d = t18a;let t18b: f64 = (l.f771 + l.f779);l.f766 = t18b;}
        let t18c: f64 = (l.f75e).min(l.f77e);let t18d: f64 = (t18c).min(l.f764);l.f76f = t18d;let t18e: f64 = (l.f76f * 0.1);l.f78f = t18e;let t190: f64 = (l.f621).max(l.f651);let t191: f64 = (t190).max(l.f62d);l.f64b = t191;let t192: f64 = (-1.0);let t193: f64 = (t192 / l.f64b);let t194: f64 = (2.0_f64).powf(t193);let t195: f64 = (1.0 - t194);let t196: f64 = (l.f76f * t195);l.f791 = t196;let t197: f64 = (l.f760).min(l.f780);let t198: f64 = (t197).min(l.f766);let t199: f64 = (t198 - 0.05);l.f755 = t199;let t19b: f64 = (l.f0 * l.f542);let t19c: f64 = (l.f5b1 * l.f54c);let t19d: f64 = (t19b + t19c);let t19e: f64 = (l.f5af * l.f544);let t19f: f64 = (t19d + t19e);l.f590 = t19f;l.f586 = 0.0;l.f5c9 = 1.0;(l.f5cb, l.f5cc, l.f5cd, ) = (1.0, 0.0, 0.0, );(l.f588, l.f589, l.f58a, ) = (0.0, 0.0, 0.0, );(l.f5cf, l.f5d0, l.f5d1, ) = (1.0, 0.0, 0.0, );(l.f58c, l.f58d, l.f58e, ) = (0.0, 0.0, 0.0, );l.f5bd = 0.0;l.f800 = 0.0;l.f97 = 0.0;(l.f802, l.f803, l.f804, ) = (0.0, 0.0, 0.0, );(l.f99, l.f9a, l.f9b, ) = (0.0, 0.0, 0.0, );(l.f806, l.f807, l.f808, ) = (0.0, 0.0, 0.0, );(l.f9d, l.f9e, l.f9f, ) = (0.0, 0.0, 0.0, );(l.f5b9, l.f5ba, l.f5bb, ) = (0.0, 0.0, 0.0, );(l.f5bf, l.f5c0, l.f5c1, ) = (0.0, 0.0, 0.0, );(l.f501, l.f502, l.f503, ) = (0.0, 0.0, 0.0, );(l.f509, l.f50a, l.f50b, ) = (0.0, 0.0, 0.0, );(l.f511, l.f512, l.f513, ) = (0.0, 0.0, 0.0, );(l.f519, l.f51a, l.f51b, ) = (0.0, 0.0, 0.0, );(l.f521, l.f522, l.f523, ) = (0.0, 0.0, 0.0, );l.f707 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        (l.f709, l.f70a, l.f70b, ) = (0.0, 0.0, 0.0, );(l.f70d, l.f70e, l.f70f, ) = (0.0, 0.0, 0.0, );l.f822 = 0.0;l.f81c = 1.0;l.f820 = 1.0;l.f81e = 1.0;(l.f4, l.f5, l.f6, ) = (0.0, 0.0, 0.0, );(l.f701, l.f702, l.f703, ) = (0.0, 0.0, 0.0, );(l.f7ad, l.f7ae, l.f7af, ) = (0.0, 0.0, 0.0, );l.f38 = 0.0;let t1a3: f64 = (l.f0 * l.f6ab);let t1a4: f64 = if t1a3 > 0.0 { 1.0 } else { 0.0 };l.f1d6 = t1a4;
        if (l.f1d6 != 0.0) {let t1a5: f64 = (l.f0 / l.f6ab);l.f38 = t1a5;}
        let t1a6: f64 = (l.f5b1 * l.f6ae);let t1a7: f64 = if t1a6 > 0.0 { 1.0 } else { 0.0 };l.f1eb = t1a7;
        if (l.f1eb != 0.0) {let t1a8: f64 = (l.f5b1 / l.f6ae);let t1a9: f64 = (t1a8 + l.f38);l.f38 = t1a9;}
        let t1aa: f64 = (l.f5af * l.f6ad);let t1ab: f64 = if t1aa > 0.0 { 1.0 } else { 0.0 };l.f200 = t1ab;
        if (l.f200 != 0.0) {let t1ac: f64 = (l.f5af / l.f6ad);let t1ad: f64 = (t1ac + l.f38);l.f38 = t1ad;}
        let t1ae: f64 = if l.f38 > 0.0 { 1.0 } else { 0.0 };l.f215 = t1ae;
        if (l.f215 != 0.0) {let t1b0: f64 = (1.0 / l.f38);let t1b1: f64 = (t1b0 + l.f6ac);l.f6af = t1b1;}
        if (l.f215 == 0.0) {l.f6af = l.f6ac;}
        (l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );(l.f59e, l.f59f, l.f5a0, l.f5a1, ) = (0.0, 0.0, 0.0, 0.0, );(l.f57c, l.f57d, l.f57e, l.f57f, ) = (0.0, 0.0, 0.0, 0.0, );(l.f581, l.f582, l.f583, l.f584, ) = (0.0, 0.0, 0.0, 0.0, );(l.f663, l.f664, ) = (0.0, 0.0, );(l.f666, l.f667, ) = (0.0, 0.0, );(l.f7c9, l.f7ca, ) = (0.0, 0.0, );let t1b6: f64 = (1.6021918e-19 * l.f0);l.f669 = t1b6;let t1b7: f64 = (2.0 * l.f6b);let t1b8: f64 = (1.6021918e-19 * l.f5dd);let t1b9: f64 = (t1b7 / t1b8);let t1ba: f64 = (t1b9).sqrt();(l.f7bd, l.f7be, l.f7bf, ) = (t1ba, 0.0, 0.0, );let t1bb: f64 = (p.p94 - l.f7bd);let t1bc: f64 = (t1bb - 1e-7);(l.f6f3, l.f6f4, l.f6f5, ) = (t1bc, (-l.f7be), (-l.f7bf), );let t1bd: f64 = (4.0 * p.p94);let t1be: f64 = (t1bd * 1e-7);(l.f6f7, l.f6f8, l.f6f9, ) = (t1be, 0.0, 0.0, );
        if (!(l.f6f7 > 0.0)) {let t1bf: f64 = (-l.f6f7);(l.f6f7, l.f6f8, l.f6f9, ) = (t1bf, (-l.f6f8), (-l.f6f9), );}
        let t1c0: f64 = (l.f6f3 * l.f6f3);let t1c1: f64 = (t1c0 + l.f6f7);let t1c2: f64 = (t1c1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1c2, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1c2)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1c2)), );let t1c3: f64 = (l.f6f3 + l.f6f7);let t1c4: f64 = (0.5 * t1c3);let t1c5: f64 = (p.p94 - t1c4);(l.f7bd, l.f7be, l.f7bf, ) = (t1c5, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t1c6: f64 = if l.f6d9 > 0.9 { 1.0 } else { 0.0 };l.f22a = t1c6;let t1c7: f64 = (l.f5e5 - l.f5e7);let t1c8: f64 = (t1c7).abs();let t1c9: f64 = (l.f5e5 - l.f5e9);let t1ca: f64 = (t1c9).abs();let t1cb: f64 = (l.f5e7 - l.f5e9);let t1cc: f64 = (t1cb).abs();let t1cd: f64 = if (((((t1c8 > 1e-6) && (l.f0 > 0.0)) && (l.f5af > 0.0)) || (((t1ca > 1e-6) && (l.f0 > 0.0)) && (l.f5b1 > 0.0))) || (((t1cc > 1e-6) && (l.f5af > 0.0)) && (l.f5b1 > 0.0))) { 1.0 } else { 0.0 };l.f240 = t1cd;
        if ((l.f22a != 0.0) && (l.f240 != 0.0)) {l.f6d9 = 0.0;}
        let t1cf: f64 = if l.f0 > 0.0 { 1.0 } else { 0.0 };l.f256 = t1cf;
        if (((l.f22a != 0.0) && (l.f240 == 0.0)) && (l.f256 != 0.0)) {l.f5c9 = l.f5e5;}
        let t1d0: f64 = if l.f5af > 0.0 { 1.0 } else { 0.0 };l.f26c = t1d0;
        if (((l.f22a != 0.0) && (l.f240 == 0.0)) && (l.f26c != 0.0)) {l.f5c9 = l.f5e7;}
        let t1d1: f64 = if l.f5b1 > 0.0 { 1.0 } else { 0.0 };l.f284 = t1d1;
        if (((l.f22a != 0.0) && (l.f240 == 0.0)) && (l.f284 != 0.0)) {l.f5c9 = l.f5e9;}
        let t1d2: f64 = if l.f6d9 == 1.0 { 1.0 } else { 0.0 };l.f29a = t1d2;
        if (l.f29a != 0.0) {l.f811 = 0.0;l.f6e2 = 0.0;l.f6e = 0.0;l.f4e1 = 0.0;l.f4e5 = 0.0;l.f4e9 = 0.0;l.f4ef = 0.0;l.f4f5 = 0.0;l.f4fb = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        l: &mut StampLocals,
    ) {
        if (l.f29a != 0.0) {(l.f745, l.f746, l.f747, ) = (0.0, 0.0, 0.0, );l.f796 = 0.0;l.f817 = 0.0;l.f825 = 0.0;l.f714 = 0.0;l.f79c = 0.0;l.f7a2 = 0.0;l.f750 = 0.0;l.f74a = 0.0;l.f6fc = 0.0;(l.f52f, l.f530, l.f531, ) = (0.0, 0.0, 0.0, );l.f593 = 0.0;l.f758 = 0.0;l.f7ef = 0.0;l.f66 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f599 = 0.0;l.f19 = 0.0;l.f71a = 0.0;l.f72c = 0.0;l.f726 = 0.0;l.f6c1 = 0.0;l.f732 = 0.0;l.f7e3 = 0.0;l.f7f5 = 0.0;l.f5a8 = 0.0;l.f5b4 = 0.0;l.f5d4 = 0.0;l.f7fb = 0.0;l.f74 = 0.0;l.fd6 = 0.0;l.f529 = 0.0;l.fb6 = 0.0;l.fae = 0.0;l.fbd = 0.4;l.fbf = 0.65;l.fbb = 0.8;let t1d4: f64 = (-l.fbd);let t1d5: f64 = (t1d4 * l.f7ab);l.f737 = t1d5;let t1d6: f64 = (-l.fbf);let t1d7: f64 = (t1d6 * l.f7ab);l.f739 = t1d7;let t1d8: f64 = (-l.fbb);let t1d9: f64 = (t1d8 * l.f7ab);l.f73b = t1d9;l.f73d = 0.1;l.f73f = 0.2;}
        let t1da: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f4ab = t1da;
        if ((l.f29a != 0.0) && (l.f4ab != 0.0)) {let t1db: f64 = (4.0 * l.f78f);let t1dc: f64 = (t1db * l.f78f);l.f4e1 = t1dc;let t1dd: f64 = (l.f78f / l.f791);l.f4e5 = t1dd;let t1de: f64 = (l.f78f * l.f4e5);let t1df: f64 = (l.f737 + t1de);l.f4e9 = t1df;let t1e0: f64 = (l.f791 + l.f4e9);l.f4ef = t1e0;let t1e1: f64 = (l.f791 - l.f4e9);l.f4f5 = t1e1;let t1e2: f64 = (l.f4f5 * l.f4f5);let t1e3: f64 = (t1e2 + l.f4e1);let t1e4: f64 = (t1e3).sqrt();l.f4fb = t1e4;let t1e5: f64 = (l.f737 * l.f791);let t1e6: f64 = (l.f4ef + l.f4fb);let t1e7: f64 = (t1e5 / t1e6);let t1e8: f64 = (2.0 * t1e7);l.f796 = t1e8;}
        let t1e9: f64 = if l.f737 < l.f7b1 { 1.0 } else { 0.0 };l.f4ad = t1e9;let t1ea: f64 = (l.f737 * l.f645);let t1eb: f64 = (0.5 * t1ea);let t1ec: f64 = (t1eb).abs();let t1ed: f64 = if t1ec < 230.25850929940458 { 1.0 } else { 0.0 };l.f4af = t1ed;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4af != 0.0)) {let t1ee: f64 = (l.f737 * l.f645);let t1ef: f64 = (0.5 * t1ee);let t1f0: f64 = (t1ef).exp();l.f825 = t1f0;}
        let t1f1: f64 = (l.f737 * l.f645);let t1f2: f64 = (0.5 * t1f1);let t1f3: f64 = (-230.25850929940458);let t1f4: f64 = if t1f2 < t1f3 { 1.0 } else { 0.0 };l.f4b1 = t1f4;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4af == 0.0)) && (l.f4b1 != 0.0)) {let t1f5: f64 = (-230.25850929940458);let t1f6: f64 = (l.f737 * l.f645);let t1f7: f64 = (0.5 * t1f6);let t1f8: f64 = (t1f5 - t1f7);let t1f9: f64 = (-230.25850929940458);let t1fa: f64 = (l.f737 * l.f645);let t1fb: f64 = (0.5 * t1fa);let t1fc: f64 = (t1f9 - t1fb);let t1fd: f64 = (-230.25850929940458);let t1fe: f64 = (l.f737 * l.f645);let t1ff: f64 = (0.5 * t1fe);let t200: f64 = (t1fd - t1ff);let t201: f64 = (t200 * 0.3333333333333333);let t202: f64 = (1.0 + t201);let t203: f64 = (t1fc * t202);let t204: f64 = (0.5 * t203);let t205: f64 = (1.0 + t204);let t206: f64 = (t1f8 * t205);let t207: f64 = (1.0 + t206);let t208: f64 = (1e-100 / t207);l.f825 = t208;}
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4af == 0.0)) && (l.f4b1 == 0.0)) {let t209: f64 = (l.f737 * l.f645);let t20a: f64 = (0.5 * t209);let t20b: f64 = (t20a - 230.25850929940458);let t20c: f64 = (l.f737 * l.f645);let t20d: f64 = (0.5 * t20c);let t20e: f64 = (t20d - 230.25850929940458);let t20f: f64 = (l.f737 * l.f645);let t210: f64 = (0.5 * t20f);let t211: f64 = (t210 - 230.25850929940458);let t212: f64 = (t211 * 0.3333333333333333);let t213: f64 = (1.0 + t212);let t214: f64 = (t20e * t213);let t215: f64 = (0.5 * t214);let t216: f64 = (1.0 + t215);let t217: f64 = (t20b * t216);let t218: f64 = (1.0 + t217);let t219: f64 = (1e100 * t218);l.f825 = t219;}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) {let t21a: f64 = (l.f5eb * l.f5eb);let t21b: f64 = (t21a / l.f5df);l.f64f = t21b;let t21c: f64 = (l.f5e5 / l.f645);let t21d: f64 = (l.f5df / l.f64f);let t21e: f64 = (t21d).ln();let t21f: f64 = (t21c * t21e);l.f793 = t21f;}
        let t221: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f4b5 = t221;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t222: f64 = (l.f737 - l.f793);let t223: f64 = (p.p86 * t222);let t224: f64 = (t223 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t224, 0.0, 0.0, );let t225: f64 = (p.p86 * l.f793);let t226: f64 = (l.f5e5 - t225);(l.f5ed, l.f5ee, l.f5ef, ) = (t226, 0.0, 0.0, );let t227: f64 = (p.p85 - l.f601);let t228: f64 = (t227 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t228, (-l.f602), (-l.f603), );let t229: f64 = (4.0 * p.p85);let t22a: f64 = (t229 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t22a, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {
            let (t22c, t22d, t22e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t22b: f64 = (-l.f6f7);
        (t22b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t22c, t22d, t22e, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t22f: f64 = (l.f6f3 * l.f6f3);let t230: f64 = (t22f + l.f6f7);let t231: f64 = (t230).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t231, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t231)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t231)), );let t232: f64 = (l.f6f3 + l.f6f7);let t233: f64 = (0.5 * t232);let t234: f64 = (p.p85 - t233);(l.f605, l.f606, l.f607, ) = (t234, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t235: f64 = (l.f605 - l.f5e5);let t236: f64 = (t235 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t236, l.f606, l.f607, );let t238: f64 = (4.0 * l.f5e5);let t239: f64 = (t238 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t239, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {
            let (t23c, t23d, t23e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t23b: f64 = (-l.f6f7);
        (t23b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t23c, t23d, t23e, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t23f: f64 = (l.f6f3 * l.f6f3);let t240: f64 = (t23f + l.f6f7);let t241: f64 = (t240).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t241, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t241)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t241)), );let t242: f64 = (l.f6f3 + l.f6f7);let t243: f64 = (0.5 * t242);let t244: f64 = (l.f5e5 + t243);(l.f5f1, l.f5f2, l.f5f3, ) = (t244, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t245: f64 = (p.p85 - l.f5ed);let t246: f64 = (t245 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t246, (-l.f5ee), (-l.f5ef), );let t247: f64 = (4.0 * p.p85);let t248: f64 = (t247 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t248, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {
            let (t24a, t24b, t24c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t249: f64 = (-l.f6f7);
        (t249, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t24a, t24b, t24c, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t24d: f64 = (l.f6f3 * l.f6f3);let t24e: f64 = (t24d + l.f6f7);let t24f: f64 = (t24e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t24f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t24f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t24f)), );let t250: f64 = (l.f6f3 + l.f6f7);let t251: f64 = (0.5 * t250);let t252: f64 = (p.p85 - t251);(l.f5ed, l.f5ee, l.f5ef, ) = (t252, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t253: f64 = (l.f5ed - l.f5e5);let t254: f64 = (t253 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t254, l.f5ee, l.f5ef, );let t255: f64 = (4.0 * l.f5e5);let t256: f64 = (t255 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t256, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {
            let (t259, t25a, t25b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t258: f64 = (-l.f6f7);
        (t258, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t259, t25a, t25b, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 != 0.0)) {let t25c: f64 = (l.f6f3 * l.f6f3);let t25d: f64 = (t25c + l.f6f7);let t25e: f64 = (t25d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t25e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t25e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t25e)), );let t25f: f64 = (l.f6f3 + l.f6f7);let t260: f64 = (0.5 * t25f);let t261: f64 = (l.f5e5 + t260);(l.f5ed, l.f5ee, l.f5ef, ) = (t261, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b5 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );}
        let t262: f64 = (l.f737 / l.f5f1);let t263: f64 = (l.f5f1 - l.f5ed);let t264: f64 = (l.f793 * t263);let t265: f64 = (l.f5ed * p.p85);let t266: f64 = (t264 / t265);let t267: f64 = (t262 + t266);let t268: f64 = (l.f645 * t267);let t269: f64 = (t268).abs();let t26a: f64 = if t269 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4b7 = t26a;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b7 != 0.0)) {let t26b: f64 = (l.f737 / l.f5f1);let t26c: f64 = (l.f5f1 - l.f5ed);let t26d: f64 = (l.f793 * t26c);let t26e: f64 = (l.f5ed * p.p85);let t26f: f64 = (t26d / t26e);let t270: f64 = (t26b + t26f);let t271: f64 = (l.f645 * t270);let t272: f64 = (t271).exp();(l.f536, l.f537, l.f538, ) = (t272, (t272 * (l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t26e) - (t26d * (l.f5ee * p.p85))) / (t26e * t26e))))), (t272 * (l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t26e) - (t26d * (l.f5ef * p.p85))) / (t26e * t26e))))), );}
        let t273: f64 = (l.f737 / l.f5f1);let t274: f64 = (l.f5f1 - l.f5ed);let t275: f64 = (l.f793 * t274);let t276: f64 = (l.f5ed * p.p85);let t277: f64 = (t275 / t276);let t278: f64 = (t273 + t277);let t279: f64 = (l.f645 * t278);let t27a: f64 = (-230.25850929940458);let t27b: f64 = if t279 < t27a { 1.0 } else { 0.0 };l.f4b9 = t27b;
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b7 == 0.0)) && (l.f4b9 != 0.0)) {let t27c: f64 = (-230.25850929940458);let t27d: f64 = (l.f737 / l.f5f1);let t27e: f64 = (l.f5f1 - l.f5ed);let t27f: f64 = (l.f793 * t27e);let t280: f64 = (l.f5ed * p.p85);let t281: f64 = (t27f / t280);let t282: f64 = (t27d + t281);let t283: f64 = (l.f645 * t282);let t284: f64 = (t27c - t283);let t285: f64 = (-230.25850929940458);let t286: f64 = (l.f737 / l.f5f1);let t287: f64 = (l.f5f1 - l.f5ed);let t288: f64 = (l.f793 * t287);let t289: f64 = (l.f5ed * p.p85);let t28a: f64 = (t288 / t289);let t28b: f64 = (t286 + t28a);let t28c: f64 = (l.f645 * t28b);let t28d: f64 = (t285 - t28c);let t28e: f64 = (-230.25850929940458);let t28f: f64 = (l.f737 / l.f5f1);let t290: f64 = (l.f5f1 - l.f5ed);let t291: f64 = (l.f793 * t290);let t292: f64 = (l.f5ed * p.p85);let t293: f64 = (t291 / t292);let t294: f64 = (t28f + t293);let t295: f64 = (l.f645 * t294);let t296: f64 = (t28e - t295);let t297: f64 = (t296 * 0.3333333333333333);let t298: f64 = (1.0 + t297);let t299: f64 = (t28d * t298);let t29a: f64 = (0.5 * t299);let t29b: f64 = (1.0 + t29a);let t29c: f64 = (t284 * t29b);let t29d: f64 = (1.0 + t29c);let t29e: f64 = (1e-100 / t29d);(l.f536, l.f537, l.f538, ) = (t29e, (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t280) - (t27f * (l.f5ee * p.p85))) / (t280 * t280))))) * t29b) + (t284 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t289) - (t288 * (l.f5ee * p.p85))) / (t289 * t289))))) * t298) + (t28d * ((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t292) - (t291 * (l.f5ee * p.p85))) / (t292 * t292))))) * 0.3333333333333333))))))) / (t29d * t29d))), (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t280) - (t27f * (l.f5ef * p.p85))) / (t280 * t280))))) * t29b) + (t284 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t289) - (t288 * (l.f5ef * p.p85))) / (t289 * t289))))) * t298) + (t28d * ((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t292) - (t291 * (l.f5ef * p.p85))) / (t292 * t292))))) * 0.3333333333333333))))))) / (t29d * t29d))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4b7 == 0.0)) && (l.f4b9 == 0.0)) {let t29f: f64 = (l.f737 / l.f5f1);let t2a0: f64 = (l.f5f1 - l.f5ed);let t2a1: f64 = (l.f793 * t2a0);let t2a2: f64 = (l.f5ed * p.p85);let t2a3: f64 = (t2a1 / t2a2);let t2a4: f64 = (t29f + t2a3);let t2a5: f64 = (l.f645 * t2a4);let t2a6: f64 = (t2a5 - 230.25850929940458);let t2a7: f64 = (l.f737 / l.f5f1);let t2a8: f64 = (l.f5f1 - l.f5ed);let t2a9: f64 = (l.f793 * t2a8);let t2aa: f64 = (l.f5ed * p.p85);let t2ab: f64 = (t2a9 / t2aa);let t2ac: f64 = (t2a7 + t2ab);let t2ad: f64 = (l.f645 * t2ac);let t2ae: f64 = (t2ad - 230.25850929940458);let t2af: f64 = (l.f737 / l.f5f1);let t2b0: f64 = (l.f5f1 - l.f5ed);let t2b1: f64 = (l.f793 * t2b0);let t2b2: f64 = (l.f5ed * p.p85);let t2b3: f64 = (t2b1 / t2b2);let t2b4: f64 = (t2af + t2b3);let t2b5: f64 = (l.f645 * t2b4);let t2b6: f64 = (t2b5 - 230.25850929940458);let t2b7: f64 = (t2b6 * 0.3333333333333333);let t2b8: f64 = (1.0 + t2b7);let t2b9: f64 = (t2ae * t2b8);let t2ba: f64 = (0.5 * t2b9);let t2bb: f64 = (1.0 + t2ba);let t2bc: f64 = (t2a6 * t2bb);let t2bd: f64 = (1.0 + t2bc);let t2be: f64 = (1e100 * t2bd);(l.f536, l.f537, l.f538, ) = (t2be, (1e100 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2a2) - (t2a1 * (l.f5ee * p.p85))) / (t2a2 * t2a2)))) * t2bb) + (t2a6 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2aa) - (t2a9 * (l.f5ee * p.p85))) / (t2aa * t2aa)))) * t2b8) + (t2ae * ((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2b2) - (t2b1 * (l.f5ee * p.p85))) / (t2b2 * t2b2)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2a2) - (t2a1 * (l.f5ef * p.p85))) / (t2a2 * t2a2)))) * t2bb) + (t2a6 * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2aa) - (t2a9 * (l.f5ef * p.p85))) / (t2aa * t2aa)))) * t2b8) + (t2ae * ((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2b2) - (t2b1 * (l.f5ef * p.p85))) / (t2b2 * t2b2)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) {let t2c0: f64 = (l.f5eb * l.f5eb);let t2c1: f64 = (t2c0 / l.f5e3);l.f64f = t2c1;let t2c2: f64 = (l.f5e9 / l.f645);let t2c3: f64 = (l.f5e3 / l.f64f);let t2c4: f64 = (t2c3).ln();let t2c5: f64 = (t2c2 * t2c4);l.f793 = t2c5;}
        let t2c6: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f4bb = t2c6;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t2c7: f64 = (l.f737 - l.f793);let t2c8: f64 = (p.p86 * t2c7);let t2c9: f64 = (t2c8 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t2c9, 0.0, 0.0, );let t2ca: f64 = (p.p86 * l.f793);let t2cb: f64 = (l.f5e9 - t2ca);(l.f5ed, l.f5ee, l.f5ef, ) = (t2cb, 0.0, 0.0, );let t2cc: f64 = (p.p85 - l.f601);let t2cd: f64 = (t2cc - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2cd, (-l.f602), (-l.f603), );let t2ce: f64 = (4.0 * p.p85);let t2cf: f64 = (t2ce * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2cf, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {
            let (t2d1, t2d2, t2d3,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2d0: f64 = (-l.f6f7);
        (t2d0, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2d1, t2d2, t2d3, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t2d4: f64 = (l.f6f3 * l.f6f3);let t2d5: f64 = (t2d4 + l.f6f7);let t2d6: f64 = (t2d5).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2d6, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2d6)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2d6)), );let t2d7: f64 = (l.f6f3 + l.f6f7);let t2d8: f64 = (0.5 * t2d7);let t2d9: f64 = (p.p85 - t2d8);(l.f605, l.f606, l.f607, ) = (t2d9, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2db: f64 = (l.f605 - l.f5e9);let t2dc: f64 = (t2db - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2dc, l.f606, l.f607, );let t2dd: f64 = (4.0 * l.f5e9);let t2de: f64 = (t2dd * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2de, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {
            let (t2e0, t2e1, t2e2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2df: f64 = (-l.f6f7);
        (t2df, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2e0, t2e1, t2e2, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t2e3: f64 = (l.f6f3 * l.f6f3);let t2e4: f64 = (t2e3 + l.f6f7);let t2e5: f64 = (t2e4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2e5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2e5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2e5)), );let t2e6: f64 = (l.f6f3 + l.f6f7);let t2e7: f64 = (0.5 * t2e6);let t2e8: f64 = (l.f5e9 + t2e7);(l.f5f1, l.f5f2, l.f5f3, ) = (t2e8, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t2e9: f64 = (p.p85 - l.f5ed);let t2ea: f64 = (t2e9 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2ea, (-l.f5ee), (-l.f5ef), );let t2eb: f64 = (4.0 * p.p85);let t2ec: f64 = (t2eb * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2ec, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {
            let (t2ee, t2ef, t2f0,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2ed: f64 = (-l.f6f7);
        (t2ed, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2ee, t2ef, t2f0, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t2f1: f64 = (l.f6f3 * l.f6f3);let t2f2: f64 = (t2f1 + l.f6f7);let t2f3: f64 = (t2f2).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2f3, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2f3)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2f3)), );let t2f4: f64 = (l.f6f3 + l.f6f7);let t2f5: f64 = (0.5 * t2f4);let t2f6: f64 = (p.p85 - t2f5);(l.f5ed, l.f5ee, l.f5ef, ) = (t2f6, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2f8: f64 = (l.f5ed - l.f5e9);let t2f9: f64 = (t2f8 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2f9, l.f5ee, l.f5ef, );let t2fa: f64 = (4.0 * l.f5e9);let t2fb: f64 = (t2fa * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2fb, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {
            let (t2fd, t2fe, t2ff,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2fc: f64 = (-l.f6f7);
        (t2fc, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2fd, t2fe, t2ff, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb != 0.0)) {let t300: f64 = (l.f6f3 * l.f6f3);let t301: f64 = (t300 + l.f6f7);let t302: f64 = (t301).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t302, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t302)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t302)), );let t303: f64 = (l.f6f3 + l.f6f7);let t304: f64 = (0.5 * t303);let t305: f64 = (l.f5e9 + t304);(l.f5ed, l.f5ee, l.f5ef, ) = (t305, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bb == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );}
        let t306: f64 = (l.f737 / l.f5f1);let t307: f64 = (l.f5f1 - l.f5ed);let t308: f64 = (l.f793 * t307);let t309: f64 = (l.f5ed * p.p85);let t30a: f64 = (t308 / t309);let t30b: f64 = (t306 + t30a);let t30c: f64 = (l.f645 * t30b);let t30d: f64 = (t30c).abs();let t30e: f64 = if t30d < 230.25850929940458 { 1.0 } else { 0.0 };l.f4bd = t30e;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bd != 0.0)) {let t30f: f64 = (l.f737 / l.f5f1);let t310: f64 = (l.f5f1 - l.f5ed);let t311: f64 = (l.f793 * t310);let t312: f64 = (l.f5ed * p.p85);let t313: f64 = (t311 / t312);let t314: f64 = (t30f + t313);let t315: f64 = (l.f645 * t314);let t316: f64 = (t315).exp();(l.f53e, l.f53f, l.f540, ) = (t316, (t316 * (l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t312) - (t311 * (l.f5ee * p.p85))) / (t312 * t312))))), (t316 * (l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t312) - (t311 * (l.f5ef * p.p85))) / (t312 * t312))))), );}
        let t317: f64 = (l.f737 / l.f5f1);let t318: f64 = (l.f5f1 - l.f5ed);let t319: f64 = (l.f793 * t318);let t31a: f64 = (l.f5ed * p.p85);let t31b: f64 = (t319 / t31a);let t31c: f64 = (t317 + t31b);let t31d: f64 = (l.f645 * t31c);let t31e: f64 = (-230.25850929940458);let t31f: f64 = if t31d < t31e { 1.0 } else { 0.0 };l.f4bf = t31f;
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bd == 0.0)) && (l.f4bf != 0.0)) {let t320: f64 = (-230.25850929940458);let t321: f64 = (l.f737 / l.f5f1);let t322: f64 = (l.f5f1 - l.f5ed);let t323: f64 = (l.f793 * t322);let t324: f64 = (l.f5ed * p.p85);let t325: f64 = (t323 / t324);let t326: f64 = (t321 + t325);let t327: f64 = (l.f645 * t326);let t328: f64 = (t320 - t327);let t329: f64 = (-230.25850929940458);let t32a: f64 = (l.f737 / l.f5f1);let t32b: f64 = (l.f5f1 - l.f5ed);let t32c: f64 = (l.f793 * t32b);let t32d: f64 = (l.f5ed * p.p85);let t32e: f64 = (t32c / t32d);let t32f: f64 = (t32a + t32e);let t330: f64 = (l.f645 * t32f);let t331: f64 = (t329 - t330);let t332: f64 = (-230.25850929940458);let t333: f64 = (l.f737 / l.f5f1);let t334: f64 = (l.f5f1 - l.f5ed);let t335: f64 = (l.f793 * t334);let t336: f64 = (l.f5ed * p.p85);let t337: f64 = (t335 / t336);let t338: f64 = (t333 + t337);let t339: f64 = (l.f645 * t338);let t33a: f64 = (t332 - t339);let t33b: f64 = (t33a * 0.3333333333333333);let t33c: f64 = (1.0 + t33b);let t33d: f64 = (t331 * t33c);let t33e: f64 = (0.5 * t33d);let t33f: f64 = (1.0 + t33e);let t340: f64 = (t328 * t33f);let t341: f64 = (1.0 + t340);let t342: f64 = (1e-100 / t341);(l.f53e, l.f53f, l.f540, ) = (t342, (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t324) - (t323 * (l.f5ee * p.p85))) / (t324 * t324))))) * t33f) + (t328 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t32d) - (t32c * (l.f5ee * p.p85))) / (t32d * t32d))))) * t33c) + (t331 * ((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t336) - (t335 * (l.f5ee * p.p85))) / (t336 * t336))))) * 0.3333333333333333))))))) / (t341 * t341))), (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t324) - (t323 * (l.f5ef * p.p85))) / (t324 * t324))))) * t33f) + (t328 * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t32d) - (t32c * (l.f5ef * p.p85))) / (t32d * t32d))))) * t33c) + (t331 * ((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t336) - (t335 * (l.f5ef * p.p85))) / (t336 * t336))))) * 0.3333333333333333))))))) / (t341 * t341))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4bd == 0.0)) && (l.f4bf == 0.0)) {let t343: f64 = (l.f737 / l.f5f1);let t344: f64 = (l.f5f1 - l.f5ed);let t345: f64 = (l.f793 * t344);let t346: f64 = (l.f5ed * p.p85);let t347: f64 = (t345 / t346);let t348: f64 = (t343 + t347);let t349: f64 = (l.f645 * t348);let t34a: f64 = (t349 - 230.25850929940458);let t34b: f64 = (l.f737 / l.f5f1);let t34c: f64 = (l.f5f1 - l.f5ed);let t34d: f64 = (l.f793 * t34c);let t34e: f64 = (l.f5ed * p.p85);let t34f: f64 = (t34d / t34e);let t350: f64 = (t34b + t34f);let t351: f64 = (l.f645 * t350);let t352: f64 = (t351 - 230.25850929940458);let t353: f64 = (l.f737 / l.f5f1);let t354: f64 = (l.f5f1 - l.f5ed);let t355: f64 = (l.f793 * t354);let t356: f64 = (l.f5ed * p.p85);let t357: f64 = (t355 / t356);let t358: f64 = (t353 + t357);let t359: f64 = (l.f645 * t358);let t35a: f64 = (t359 - 230.25850929940458);let t35b: f64 = (t35a * 0.3333333333333333);let t35c: f64 = (1.0 + t35b);let t35d: f64 = (t352 * t35c);let t35e: f64 = (0.5 * t35d);let t35f: f64 = (1.0 + t35e);let t360: f64 = (t34a * t35f);let t361: f64 = (1.0 + t360);let t362: f64 = (1e100 * t361);(l.f53e, l.f53f, l.f540, ) = (t362, (1e100 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t346) - (t345 * (l.f5ee * p.p85))) / (t346 * t346)))) * t35f) + (t34a * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t34e) - (t34d * (l.f5ee * p.p85))) / (t34e * t34e)))) * t35c) + (t352 * ((l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t356) - (t355 * (l.f5ee * p.p85))) / (t356 * t356)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t346) - (t345 * (l.f5ef * p.p85))) / (t346 * t346)))) * t35f) + (t34a * (0.5 * (((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t34e) - (t34d * (l.f5ef * p.p85))) / (t34e * t34e)))) * t35c) + (t352 * ((l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t356) - (t355 * (l.f5ef * p.p85))) / (t356 * t356)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) {let t363: f64 = (l.f5eb * l.f5eb);let t364: f64 = (t363 / l.f5e1);l.f64f = t364;let t365: f64 = (l.f5e7 / l.f645);let t366: f64 = (l.f5e1 / l.f64f);let t367: f64 = (t366).ln();let t368: f64 = (t365 * t367);l.f793 = t368;}
        let t369: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f4c1 = t369;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t36a: f64 = (l.f737 - l.f793);let t36b: f64 = (p.p86 * t36a);let t36c: f64 = (t36b + l.f5e7);(l.f601, l.f602, l.f603, ) = (t36c, 0.0, 0.0, );let t36d: f64 = (p.p86 * l.f793);let t36e: f64 = (l.f5e7 - t36d);(l.f5ed, l.f5ee, l.f5ef, ) = (t36e, 0.0, 0.0, );let t36f: f64 = (p.p85 - l.f601);let t370: f64 = (t36f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t370, (-l.f602), (-l.f603), );let t371: f64 = (4.0 * p.p85);let t372: f64 = (t371 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t372, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {
            let (t374, t375, t376,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t373: f64 = (-l.f6f7);
        (t373, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t374, t375, t376, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t378: f64 = (l.f6f3 * l.f6f3);let t379: f64 = (t378 + l.f6f7);let t37a: f64 = (t379).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t37a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t37a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t37a)), );let t37b: f64 = (l.f6f3 + l.f6f7);let t37c: f64 = (0.5 * t37b);let t37d: f64 = (p.p85 - t37c);(l.f605, l.f606, l.f607, ) = (t37d, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t37e: f64 = (l.f605 - l.f5e7);let t37f: f64 = (t37e - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t37f, l.f606, l.f607, );let t380: f64 = (4.0 * l.f5e7);let t381: f64 = (t380 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t381, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {
            let (t383, t384, t385,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t382: f64 = (-l.f6f7);
        (t382, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t383, t384, t385, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t386: f64 = (l.f6f3 * l.f6f3);let t387: f64 = (t386 + l.f6f7);let t388: f64 = (t387).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t388, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t388)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t388)), );let t389: f64 = (l.f6f3 + l.f6f7);let t38a: f64 = (0.5 * t389);let t38b: f64 = (l.f5e7 + t38a);(l.f5f1, l.f5f2, l.f5f3, ) = (t38b, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t38c: f64 = (p.p85 - l.f5ed);let t38d: f64 = (t38c - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t38d, (-l.f5ee), (-l.f5ef), );let t38e: f64 = (4.0 * p.p85);let t38f: f64 = (t38e * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t38f, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {
            let (t391, t392, t393,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t390: f64 = (-l.f6f7);
        (t390, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t391, t392, t393, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t395: f64 = (l.f6f3 * l.f6f3);let t396: f64 = (t395 + l.f6f7);let t397: f64 = (t396).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t397, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t397)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t397)), );let t398: f64 = (l.f6f3 + l.f6f7);let t399: f64 = (0.5 * t398);let t39a: f64 = (p.p85 - t399);(l.f5ed, l.f5ee, l.f5ef, ) = (t39a, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t39b: f64 = (l.f5ed - l.f5e7);let t39c: f64 = (t39b - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t39c, l.f5ee, l.f5ef, );let t39d: f64 = (4.0 * l.f5e7);let t39e: f64 = (t39d * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t39e, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {
            let (t3a0, t3a1, t3a2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t39f: f64 = (-l.f6f7);
        (t39f, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3a0, t3a1, t3a2, );
        }
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 != 0.0)) {let t3a3: f64 = (l.f6f3 * l.f6f3);let t3a4: f64 = (t3a3 + l.f6f7);let t3a5: f64 = (t3a4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3a5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3a5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3a5)), );let t3a6: f64 = (l.f6f3 + l.f6f7);let t3a7: f64 = (0.5 * t3a6);let t3a8: f64 = (l.f5e7 + t3a7);(l.f5ed, l.f5ee, l.f5ef, ) = (t3a8, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c1 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );}
        let t3a9: f64 = (l.f737 / l.f5f1);let t3aa: f64 = (l.f5f1 - l.f5ed);let t3ab: f64 = (l.f793 * t3aa);let t3ac: f64 = (l.f5ed * p.p85);let t3ad: f64 = (t3ab / t3ac);let t3ae: f64 = (t3a9 + t3ad);let t3af: f64 = (l.f645 * t3ae);let t3b0: f64 = (t3af).abs();let t3b1: f64 = if t3b0 < 230.25850929940458 { 1.0 } else { 0.0 };l.f4c3 = t3b1;
        if ((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c3 != 0.0)) {let t3b3: f64 = (l.f737 / l.f5f1);let t3b4: f64 = (l.f5f1 - l.f5ed);let t3b5: f64 = (l.f793 * t3b4);let t3b6: f64 = (l.f5ed * p.p85);let t3b7: f64 = (t3b5 / t3b6);let t3b8: f64 = (t3b3 + t3b7);let t3b9: f64 = (l.f645 * t3b8);let t3ba: f64 = (t3b9).exp();(l.f53a, l.f53b, l.f53c, ) = (t3ba, (t3ba * (l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3b6) - (t3b5 * (l.f5ee * p.p85))) / (t3b6 * t3b6))))), (t3ba * (l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3b6) - (t3b5 * (l.f5ef * p.p85))) / (t3b6 * t3b6))))), );}
        let t3bb: f64 = (l.f737 / l.f5f1);let t3bc: f64 = (l.f5f1 - l.f5ed);let t3bd: f64 = (l.f793 * t3bc);let t3be: f64 = (l.f5ed * p.p85);let t3bf: f64 = (t3bd / t3be);let t3c0: f64 = (t3bb + t3bf);let t3c1: f64 = (l.f645 * t3c0);let t3c2: f64 = (-230.25850929940458);let t3c3: f64 = if t3c1 < t3c2 { 1.0 } else { 0.0 };l.f4c5 = t3c3;
        if (((((l.f29a != 0.0) && (l.f4ab != 0.0)) && (l.f4ad != 0.0)) && (l.f4c3 == 0.0)) && (l.f4c5 != 0.0)) {let t3c4: f64 = (-230.25850929940458);let t3c5: f64 = (l.f737 / l.f5f1);let t3c6: f64 = (l.f5f1 - l.f5ed);let t3c7: f64 = (l.f793 * t3c6);let t3c8: f64 = (l.f5ed * p.p85);let t3c9: f64 = (t3c7 / t3c8);let t3ca: f64 = (t3c5 + t3c9);let t3cb: f64 = (l.f645 * t3ca);let t3cc: f64 = (t3c4 - t3cb);let t3cd: f64 = (-230.25850929940458);let t3ce: f64 = (l.f737 / l.f5f1);let t3cf: f64 = (l.f5f1 - l.f5ed);let t3d0: f64 = (l.f793 * t3cf);let t3d1: f64 = (l.f5ed * p.p85);let t3d2: f64 = (t3d0 / t3d1);let t3d3: f64 = (t3ce + t3d2);let t3d4: f64 = (l.f645 * t3d3);let t3d5: f64 = (t3cd - t3d4);let t3d6: f64 = (-230.25850929940458);let t3d7: f64 = (l.f737 / l.f5f1);let t3d8: f64 = (l.f5f1 - l.f5ed);let t3d9: f64 = (l.f793 * t3d8);let t3da: f64 = (l.f5ed * p.p85);let t3db: f64 = (t3d9 / t3da);let t3dc: f64 = (t3d7 + t3db);let t3dd: f64 = (l.f645 * t3dc);let t3de: f64 = (t3d6 - t3dd);let t3df: f64 = (t3de * 0.3333333333333333);let t3e0: f64 = (1.0 + t3df);let t3e1: f64 = (t3d5 * t3e0);let t3e2: f64 = (0.5 * t3e1);let t3e3: f64 = (1.0 + t3e2);let t3e4: f64 = (t3cc * t3e3);let t3e5: f64 = (1.0 + t3e4);let t3e6: f64 = (1e-100 / t3e5);(l.f53a, l.f53b, l.f53c, ) = (t3e6, (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3c8) - (t3c7 * (l.f5ee * p.p85))) / (t3c8 * t3c8))))) * t3e3) + (t3cc * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3d1) - (t3d0 * (l.f5ee * p.p85))) / (t3d1 * t3d1))))) * t3e0) + (t3d5 * ((-(l.f645 * ((-((l.f737 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3da) - (t3d9 * (l.f5ee * p.p85))) / (t3da * t3da))))) * 0.3333333333333333))))))) / (t3e5 * t3e5))), (-((1e-100 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3c8) - (t3c7 * (l.f5ef * p.p85))) / (t3c8 * t3c8))))) * t3e3) + (t3cc * (0.5 * (((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3d1) - (t3d0 * (l.f5ef * p.p85))) / (t3d1 * t3d1))))) * t3e0) + (t3d5 * ((-(l.f645 * ((-((l.f737 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3da) - (t3d9 * (l.f5ef * p.p85))) / (t3da * t3da))))) * 0.3333333333333333))))))) / (t3e5 * t3e5))), );}
    }
}
