#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t0: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };l.f684 = t0;
        if (l.f684 != 0.0) {let t2: f64 = 1.0;l.f1b5 = t2;}
        if (l.f684 == 0.0) {let t62: f64 = (-1.0);l.f1b5 = t62;}
        let t1a9: f64 = (8.8541878176e-12 * 11.8);l.f421 = t1a9;let t234: f64 = (273.15 + p.p38);l.f17ab = t234;l.f16ce = 0.0;let t379: f64 = if p.p944 > 0.5 { 1.0 } else { 0.0 };l.f992 = t379;
        if (l.f992 != 0.0) {l.f16ce = 1.0;}
        if (l.f992 == 0.0) {l.f16ce = 0.0;}
        let t37a: f64 = (273.15 + p.p840);l.f17ac = t37a;let t1: f64 = (1.3806505e-23 / 1.6021918e-19);l.fea7 = t1;let t3: f64 = (l.fea7 * l.f17ac);l.f10dc = t3;let t4: f64 = (1.0 / l.f10dc);l.f10de = t4;let t5: f64 = (0.000702 * l.f17ac);let t6: f64 = (t5 * l.f17ac);let t7: f64 = (-t6);let t8: f64 = (1108.0 + l.f17ac);let t9: f64 = (t7 / t8);l.f2cf = t9;let ta: f64 = (p.p851 + l.f2cf);l.f108d = ta;let tb: f64 = (p.p852 + l.f2cf);l.f1099 = tb;let tc: f64 = (p.p853 + l.f2cf);l.f1091 = tc;let t15: f64 = (1.0 - p.p848);l.fff6 = t15;let t22: f64 = (1.0 - p.p849);l.f1002 = t22;let t45: f64 = (1.0 - p.p850);l.fffa = t45;let t58: f64 = (1.0 / l.fff6);l.f1006 = t58;let t69: f64 = (1.0 / l.f1002);l.f1012 = t69;let t8a: f64 = (1.0 / l.fffa);l.f100a = t8a;let tb6: f64 = (l.f421 / p.p842);l.f1ae5 = tb6;let tca: f64 = (p.p860 * l.f421);let tcb: f64 = (tca / p.p843);l.f1af9 = tcb;let tf1: f64 = (p.p861 * l.f421);let tf2: f64 = (tf1 / p.p844);l.f1ae9 = tf2;let t10d: f64 = (1.0 / l.f1ae5);l.f1aed = t10d;let t12d: f64 = (1.0 / l.f1af9);l.f1af5 = t12d;let t15f: f64 = (1.0 / l.f1ae9);l.f1af1 = t15f;let t173: f64 = (1.0 / p.p845);l.f1916 = t173;let t191: f64 = (1.0 / p.p846);l.f1926 = t191;let t1b1: f64 = (1.0 / p.p847);l.f1920 = t1b1;let t1c5: f64 = (1.772453850905516 * 0.29214664);l.f105a = t1c5;let t1c6: f64 = (-5.0);let t1c7: f64 = (t1c6 * 0.29214664);let t1c8: f64 = (t1c7 + 6.0);let t1c9: f64 = (-2.0);let t1ca: f64 = (l.f105a).powf(t1c9);let t1cb: f64 = (t1c8 - t1ca);let t1cc: f64 = (t1cb / 3.0);l.f114 = t1cc;let t1cf: f64 = (1.0 - 0.29214664);let t1d0: f64 = (t1cf - l.f114);l.f171 = t1d0;let t1e0: f64 = (1.0 / p.p841);let t1e1: f64 = (1.0 - t1e0);l.f76 = t1e1;let t205: f64 = (l.f76).powf(p.p880);let t206: f64 = (1.0 - t205);let t207: f64 = (1.0 / t206);l.f553 = t207;let t22c: f64 = (l.f76).powf(p.p881);let t22d: f64 = (1.0 - t22c);let t22e: f64 = (1.0 / t22d);l.f557 = t22e;let t22f: f64 = (l.f76).powf(p.p882);let t230: f64 = (1.0 - t22f);let t231: f64 = (1.0 / t230);l.f555 = t231;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t232: f64 = (1.0 / p.p877);l.f1942 = t232;let t233: f64 = (1.0 / p.p878);l.f1952 = t233;let t235: f64 = (1.0 / p.p879);(l.f1946, l.f194d, l.f194e, l.f194f, l.f1950, ) = (t235, 0.0, 0.0, 0.0, 0.0, );let t236: f64 = (l.f553 * l.f553);let t237: f64 = (p.p880 - 1.0);let t238: f64 = (l.f76).powf(t237);let t239: f64 = (t236 * t238);let t23a: f64 = (-t239);let t23b: f64 = (t23a * p.p880);let t23c: f64 = (t23b * l.f1942);l.f14a3 = t23c;let t23d: f64 = (l.f557 * l.f557);let t23e: f64 = (p.p881 - 1.0);let t23f: f64 = (l.f76).powf(t23e);let t240: f64 = (t23d * t23f);let t241: f64 = (-t240);let t242: f64 = (t241 * p.p881);let t243: f64 = (t242 * l.f1952);l.f14af = t243;let t248: f64 = (l.f555 * l.f555);let t249: f64 = (p.p882 - 1.0);let t24a: f64 = (l.f76).powf(t249);let t24b: f64 = (t248 * t24a);let t24c: f64 = (-t24b);let t24d: f64 = (t24c * p.p882);let t24e: f64 = (t24d * l.f1946);(l.f14a5, l.f14ab, l.f14ac, l.f14ad, l.f14ae, ) = (t24e, (t24d * l.f194d), (t24d * l.f194e), (t24d * l.f194f), (t24d * l.f1950), );let t253: f64 = if ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0)) { 1.0 } else { 0.0 };l.f9d5 = t253;
        if (l.f9d5 != 0.0) {l.f16ca = 1.0;}
        if (l.f9d5 == 0.0) {l.f16ca = 0.0;}
        let t258: f64 = if l.f16ca == 1.0 { 1.0 } else { 0.0 };l.fa4f = t258;
        if (l.fa4f != 0.0) {
            let t259: f64 = (p.p844 * p.p883);
            let (t25b,) = {
    if (t259 > 1e-18) {
        let t25a: f64 = (p.p844 * p.p883);
        (t25a,)
    } else {
        (1e-18,)
    }
};
            l.f1cd = t25b;
        }
        if (l.fa4f != 0.0) {
            let t25c: f64 = (p.p847 * p.p884);
            let (t25e,) = {
    if (t25c > 0.05) {
        let t25d: f64 = (p.p847 * p.p884);
        (t25d,)
    } else {
        (0.05,)
    }
};
            l.f191a = t25e;
        }
        if (l.fa4f != 0.0) {
            let t25f: f64 = (p.p850 * p.p885);
            let (t261,) = {
    if (t25f > 0.05) {
        let t260: f64 = (p.p850 * p.p885);
        (t260,)
    } else {
        (0.05,)
    }
};
            let (t265,) = {
    if (t261 < 0.95) {
        let t262: f64 = (p.p850 * p.p885);
        let (t264,) = {
            if (t262 > 0.05) {
                let t263: f64 = (p.p850 * p.p885);
                (t263,)
            } else {
                (0.05,)
            }
        };
        (t264,)
    } else {
        (0.95,)
    }
};
            l.f105d = t265;
        }
        if (l.fa4f != 0.0) {let t274: f64 = (p.p853 * p.p886);l.f1087 = t274;let t2b3: f64 = (l.f1087 + l.f2cf);l.f1092 = t2b3;let t2d4: f64 = (1.0 - l.f105d);l.fffb = t2d4;let t2e8: f64 = (1.0 / l.fffb);l.f100b = t2e8;}
        let t318: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.fac9 = t318;
        if (l.fac9 != 0.0) {l.f1cb = p.p842;l.f1d3 = p.p843;l.f1d1 = p.p844;l.f1914 = p.p845;l.f1924 = p.p846;l.f191e = p.p847;l.f1026 = p.p848;l.f1146 = p.p849;l.f1061 = p.p850;l.f1075 = p.p851;l.f109d = p.p852;l.f108b = p.p853;l.fd48 = p.p854;l.fd4c = p.p855;l.fd4a = p.p856;l.f1f3 = p.p857;l.f1f7 = p.p858;l.f1f5 = p.p859;l.f1d47 = p.p860;l.f1d45 = p.p861;l.f20e = p.p862;l.f212 = p.p863;l.f210 = p.p864;l.ff48 = p.p865;l.ff4c = p.p866;l.ff4a = p.p867;l.f16b = p.p868;l.f16f = p.p869;l.f16d = p.p870;l.f4bf = p.p871;l.f4c3 = p.p872;l.f4c1 = p.p873;l.f169e = p.p874;l.f16a2 = p.p875;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fac9 != 0.0) {l.f16a0 = p.p876;l.f1932 = p.p877;l.f1956 = p.p878;l.f1940 = p.p879;l.f1028 = p.p880;l.f102c = p.p881;l.f102a = p.p882;l.f1a20 = p.p945;l.f510 = p.p946;l.f21 = p.p889;l.f112 = p.p890;l.f1f = p.p891;l.f110 = p.p892;l.f4e7 = p.p883;l.f569 = p.p884;l.f525 = p.p885;l.f527 = p.p886;l.f1ad1 = p.p887;l.fa7 = p.p888;}
        if (l.fac9 == 0.0) {l.f1cb = p.p893;l.f1d3 = p.p894;l.f1d1 = p.p895;l.f1914 = p.p896;l.f1924 = p.p897;l.f191e = p.p898;l.f1026 = p.p899;l.f1146 = p.p900;l.f1061 = p.p901;l.f1075 = p.p902;l.f109d = p.p903;l.f108b = p.p904;l.fd48 = p.p905;l.fd4c = p.p906;l.fd4a = p.p907;l.f1f3 = p.p908;l.f1f7 = p.p909;l.f1f5 = p.p910;l.f1d47 = p.p911;l.f1d45 = p.p912;l.f20e = p.p913;l.f212 = p.p914;l.f210 = p.p915;l.ff48 = p.p916;l.ff4c = p.p917;l.ff4a = p.p918;l.f16b = p.p919;l.f16f = p.p920;l.f16d = p.p921;l.f4bf = p.p922;l.f4c3 = p.p923;l.f4c1 = p.p924;l.f169e = p.p925;l.f16a2 = p.p926;l.f16a0 = p.p927;l.f1932 = p.p928;l.f1956 = p.p929;l.f1940 = p.p930;l.f1028 = p.p931;l.f102c = p.p932;l.f102a = p.p933;l.f1a20 = p.p947;l.f510 = p.p948;l.f21 = p.p940;l.f112 = p.p941;l.f1f = p.p942;l.f110 = p.p943;l.f4e7 = p.p934;l.f569 = p.p935;l.f525 = p.p936;l.f527 = p.p937;l.f1ad1 = p.p938;l.fa7 = p.p939;}
        let td: f64 = (l.f1075 + l.f2cf);l.f108e = td;let te: f64 = (l.f109d + l.f2cf);l.f109a = te;let tf: f64 = (l.f108b + l.f2cf);l.f1096 = tf;let t10: f64 = (1.0 - l.f1026);l.fff7 = t10;let t11: f64 = (1.0 - l.f1146);l.f1003 = t11;let t12: f64 = (1.0 - l.f1061);l.ffff = t12;let t13: f64 = (1.0 / l.fff7);l.f1007 = t13;let t14: f64 = (1.0 / l.f1003);l.f1013 = t14;let t16: f64 = (1.0 / l.ffff);l.f100f = t16;let t17: f64 = (l.f421 / l.f1cb);l.f1ae6 = t17;let t18: f64 = (l.f1d47 * l.f421);let t19: f64 = (t18 / l.f1d3);l.f1afa = t19;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t1a: f64 = (l.f1d45 * l.f421);let t1b: f64 = (t1a / l.f1d1);l.f1aea = t1b;let t1c: f64 = (1.0 / l.f1ae6);l.f1aee = t1c;let t1d: f64 = (1.0 / l.f1afa);l.f1af6 = t1d;let t1e: f64 = (1.0 / l.f1aea);l.f1af2 = t1e;let t1f: f64 = (1.0 / l.f1914);l.f1917 = t1f;let t20: f64 = (1.0 / l.f1924);l.f1927 = t20;let t21: f64 = (1.0 / l.f191e);l.f1921 = t21;let t23: f64 = (l.f76).powf(l.f1028);let t24: f64 = (1.0 - t23);let t25: f64 = (1.0 / t24);l.f554 = t25;let t26: f64 = (l.f76).powf(l.f102c);let t27: f64 = (1.0 - t26);let t28: f64 = (1.0 / t27);l.f558 = t28;let t29: f64 = (l.f76).powf(l.f102a);let t2a: f64 = (1.0 - t29);let t2b: f64 = (1.0 / t2a);l.f556 = t2b;let t2c: f64 = (1.0 / l.f1932);l.f1943 = t2c;let t2d: f64 = (1.0 / l.f1956);l.f1953 = t2d;let t2e: f64 = (1.0 / l.f1940);(l.f1947, l.f1948, l.f1949, l.f194a, l.f194b, ) = (t2e, 0.0, 0.0, 0.0, 0.0, );let t2f: f64 = (l.f554 * l.f554);let t30: f64 = (l.f1028 - 1.0);let t31: f64 = (l.f76).powf(t30);let t32: f64 = (t2f * t31);let t33: f64 = (-t32);let t34: f64 = (t33 * l.f1028);let t35: f64 = (t34 * l.f1943);l.f14a4 = t35;let t36: f64 = (l.f558 * l.f558);let t37: f64 = (l.f102c - 1.0);let t38: f64 = (l.f76).powf(t37);let t39: f64 = (t36 * t38);let t3a: f64 = (-t39);let t3b: f64 = (t3a * l.f102c);let t3c: f64 = (t3b * l.f1953);l.f14b0 = t3c;let t3d: f64 = (l.f556 * l.f556);let t3e: f64 = (l.f102a - 1.0);let t3f: f64 = (l.f76).powf(t3e);let t40: f64 = (t3d * t3f);let t41: f64 = (-t40);let t42: f64 = (t41 * l.f102a);let t43: f64 = (t42 * l.f1947);(l.f14a6, l.f14a7, l.f14a8, l.f14a9, l.f14aa, ) = (t43, (t42 * l.f1948), (t42 * l.f1949), (t42 * l.f194a), (t42 * l.f194b), );let t44: f64 = if ((((l.f4e7 != 1.0) || (l.f569 != 1.0)) || (l.f525 != 1.0)) || (l.f527 != 1.0)) { 1.0 } else { 0.0 };l.fb45 = t44;
        if (l.fb45 != 0.0) {l.f16cb = 1.0;}
        if (l.fb45 == 0.0) {l.f16cb = 0.0;}
        let t46: f64 = if l.f16cb == 1.0 { 1.0 } else { 0.0 };l.fbbf = t46;
        if (l.fbbf != 0.0) {
            let t47: f64 = (l.f1d1 * l.f4e7);
            let (t49,) = {
    if (t47 > 1e-18) {
        let t48: f64 = (l.f1d1 * l.f4e7);
        (t48,)
    } else {
        (1e-18,)
    }
};
            l.f1ce = t49;
        }
        if (l.fbbf != 0.0) {
            let t4a: f64 = (l.f191e * l.f569);
            let (t4c,) = {
    if (t4a > 0.05) {
        let t4b: f64 = (l.f191e * l.f569);
        (t4b,)
    } else {
        (0.05,)
    }
};
            l.f191b = t4c;
        }
        if (l.fbbf != 0.0) {
            let t4d: f64 = (l.f1061 * l.f525);
            let (t4f,) = {
    if (t4d > 0.05) {
        let t4e: f64 = (l.f1061 * l.f525);
        (t4e,)
    } else {
        (0.05,)
    }
};
            let (t53,) = {
    if (t4f < 0.95) {
        let t50: f64 = (l.f1061 * l.f525);
        let (t52,) = {
            if (t50 > 0.05) {
                let t51: f64 = (l.f1061 * l.f525);
                (t51,)
            } else {
                (0.05,)
            }
        };
        (t52,)
    } else {
        (0.95,)
    }
};
            l.f105e = t53;
        }
        if (l.fbbf != 0.0) {let t54: f64 = (l.f108b * l.f527);l.f1088 = t54;let t55: f64 = (l.f1088 + l.f2cf);l.f1093 = t55;let t56: f64 = (1.0 - l.f105e);l.fffc = t56;let t57: f64 = (1.0 / l.fffc);l.f100c = t57;}
        let t59: f64 = 0.0;l.f5ff = t59;let t5a: f64 = ctx_temp;let t5b: f64 = (t5a + p.p55);let t5c: f64 = (t5b + p.p35);l.f17a1 = t5c;let t5d: f64 = (l.f17a1 / l.f17ab);l.f1448 = t5d;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t5e: f64 = (l.f17a1 - l.f17ab);l.f279 = t5e;let t5f: f64 = (l.f17a1 * 1.3806505e-23);let t60: f64 = (t5f / 1.6021918e-19);l.f10c8 = t60;let t61: f64 = (1.0 / l.f10c8);l.fe38 = t61;let t63: f64 = ctx_temp;let t64: f64 = (t63 + p.p55);let t65: f64 = (t64 + p.p35);let t66: f64 = (-250.0);let t67: f64 = (273.15 + t66);let t68: f64 = (t65).max(t67);l.f17a4 = t68;let t6a: f64 = (l.f17a4 / l.f17ac);l.ff8 = t6a;let t6b: f64 = (l.fea7 * l.f17a4);l.f10d8 = t6b;let t6c: f64 = (1.0 / l.f10d8);l.f10da = t6c;let t6d: f64 = (0.000702 * l.f17a4);let t6e: f64 = (t6d * l.f17a4);let t6f: f64 = (-t6e);let t70: f64 = (1108.0 + l.f17a4);let t71: f64 = (t6f / t70);l.f2cd = t71;let t72: f64 = (p.p851 + l.f2cd);l.f1077 = t72;let t73: f64 = (p.p852 + l.f2cd);l.f1083 = t73;let t74: f64 = (p.p853 + l.f2cd);l.f107b = t74;let t75: f64 = (l.ff8).powf(1.5);let t76: f64 = (l.f108d * l.f10de);let t77: f64 = (l.f1077 * l.f10da);let t78: f64 = (t76 - t77);let t79: f64 = (0.5 * t78);let t7a: f64 = (t79).exp();let t7b: f64 = (t75 * t7a);l.f559 = t7b;let t7c: f64 = (l.ff8).powf(1.5);let t7d: f64 = (l.f1099 * l.f10de);let t7e: f64 = (l.f1083 * l.f10da);let t7f: f64 = (t7d - t7e);let t80: f64 = (0.5 * t7f);let t81: f64 = (t80).exp();let t82: f64 = (t7c * t81);l.f565 = t82;let t83: f64 = (l.ff8).powf(1.5);let t84: f64 = (l.f1091 * l.f10de);let t85: f64 = (l.f107b * l.f10da);let t86: f64 = (t84 - t85);let t87: f64 = (0.5 * t86);let t88: f64 = (t87).exp();let t89: f64 = (t83 * t88);l.f55d = t89;let t8b: f64 = (p.p854 * l.f559);let t8c: f64 = (t8b * l.f559);l.fd40 = t8c;let t8d: f64 = (p.p855 * l.f565);let t8e: f64 = (t8d * l.f565);l.fd4e = t8e;let t8f: f64 = (p.p856 * l.f55d);let t90: f64 = (t8f * l.f55d);l.fd44 = t90;let t91: f64 = (p.p845 * l.ff8);let t92: f64 = (2.0 * l.f10d8);let t93: f64 = (l.f559).ln();let t94: f64 = (t92 * t93);let t95: f64 = (t91 - t94);l.f182d = t95;let t96: f64 = (p.p846 * l.ff8);let t97: f64 = (2.0 * l.f10d8);let t98: f64 = (l.f565).ln();let t99: f64 = (t97 * t98);let t9a: f64 = (t96 - t99);l.f1839 = t9a;let t9b: f64 = (p.p847 * l.ff8);let t9c: f64 = (2.0 * l.f10d8);let t9d: f64 = (l.f55d).ln();let t9e: f64 = (t9c * t9d);let t9f: f64 = (t9b - t9e);l.f1831 = t9f;let ta0: f64 = (0.05 - l.f182d);let ta1: f64 = (ta0 * l.f10da);let ta2: f64 = (ta1).exp();let ta3: f64 = (1.0 + ta2);
        let ta4: f64 = (ta3).ln();let ta5: f64 = (l.f10d8 * ta4);let ta6: f64 = (l.f182d + ta5);l.f18ec = ta6;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ta7: f64 = (0.05 - l.f1839);let ta8: f64 = (ta7 * l.f10da);let ta9: f64 = (ta8).exp();let taa: f64 = (1.0 + ta9);let tab: f64 = (taa).ln();let tac: f64 = (l.f10d8 * tab);let tad: f64 = (l.f1839 + tac);l.f192a = tad;let tae: f64 = (0.05 - l.f1831);let taf: f64 = (tae * l.f10da);let tb0: f64 = (taf).exp();let tb1: f64 = (1.0 + tb0);let tb2: f64 = (tb1).ln();let tb3: f64 = (l.f10d8 * tb2);let tb4: f64 = (l.f1831 + tb3);l.f18f4 = tb4;let tb5: f64 = (1.0 / l.f18ec);l.f1900 = tb5;let tb7: f64 = (1.0 / l.f192a);l.f190c = tb7;let tb8: f64 = (1.0 / l.f18f4);l.f1904 = tb8;let tb9: f64 = (p.p845 * l.f1900);let tba: f64 = (tb9).powf(p.p848);let tbb: f64 = (p.p842 * tba);l.f1bf = tbb;let tbc: f64 = (p.p846 * l.f190c);let tbd: f64 = (tbc).powf(p.p849);let tbe: f64 = (p.p843 * tbd);l.f1d5 = tbe;let tbf: f64 = (p.p847 * l.f1904);let tc0: f64 = (tbf).powf(p.p850);let tc1: f64 = (p.p844 * tc0);l.f1c3 = tc1;let tc2: f64 = (l.f1bf * l.f18ec);let tc3: f64 = (tc2 * l.f1006);l.f13d0 = tc3;let tc4: f64 = (l.f1d5 * l.f192a);let tc5: f64 = (tc4 * l.f1012);l.f13dc = tc5;let tc6: f64 = (l.f1c3 * l.f18f4);let tc7: f64 = (tc6 * l.f100a);l.f13d4 = tc7;let tc8: f64 = (2.0 * l.f1bf);l.f13c0 = tc8;let tc9: f64 = (2.0 * l.f1d5);l.f13cc = tc9;let tcc: f64 = (2.0 * l.f1c3);l.f13c4 = tcc;let tcd: f64 = (0.5 * l.f1077);let tce: f64 = (tcd).max(l.f10d8);l.f2c1 = tce;let tcf: f64 = (0.5 * l.f1083);let td0: f64 = (tcf).max(l.f10d8);l.f2c9 = td0;let td1: f64 = (0.5 * l.f107b);let td2: f64 = (td1).max(l.f10d8);l.f2c5 = td2;let td3: f64 = (l.f2c1 * l.f10da);l.fec = td3;let td4: f64 = (l.f2c9 * l.f10da);l.ff4 = td4;let td5: f64 = (l.f2c5 * l.f10da);l.ff0 = td5;let td6: f64 = (32.0 * p.p865);let td7: f64 = (td6 * 9.1093826e-31);let td8: f64 = (td7 * 1.6021918e-19);let td9: f64 = (l.f2c1 * l.f2c1);let tda: f64 = (td9 * l.f2c1);let tdb: f64 = (td8 * tda);let tdc: f64 = (tdb).sqrt();let tdd: f64 = (3.0 * 1.05457168e-34);let tde: f64 = (tdc / tdd);l.f159 = tde;let tdf: f64 = (32.0 * p.p866);let te0: f64 = (tdf * 9.1093826e-31);let te1: f64 = (te0 * 1.6021918e-19);let te2: f64 = (l.f2c9 * l.f2c9);let te3: f64 = (te2 * l.f2c9);let te4: f64 = (te1 * te3);let te5: f64 = (te4).sqrt();let te6: f64 = (3.0 * 1.05457168e-34);let te7: f64 = (te5 / te6);l.f161 = te7;let te8: f64 = (32.0 * p.p867);let te9: f64 = (te8 * 9.1093826e-31);let tea: f64 = (te9 * 1.6021918e-19);let teb: f64 = (l.f2c5 * l.f2c5);
        let tec: f64 = (teb * l.f2c5);let ted: f64 = (tea * tec);let tee: f64 = (ted).sqrt();let tef: f64 = (3.0 * 1.05457168e-34);let tf0: f64 = (tee / tef);l.f15d = tf0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tf3: f64 = (l.f17a4 - l.f17ac);let tf4: f64 = (p.p874 * tf3);let tf5: f64 = (1.0 + tf4);let tf6: f64 = (p.p871 * tf5);l.f4af = tf6;let tf7: f64 = (l.f17a4 - l.f17ac);let tf8: f64 = (p.p875 * tf7);let tf9: f64 = (1.0 + tf8);let tfa: f64 = (p.p872 * tf9);l.f4c5 = tfa;let tfb: f64 = (l.f17a4 - l.f17ac);let tfc: f64 = (p.p876 * tfb);let tfd: f64 = (1.0 + tfc);let tfe: f64 = (p.p873 * tfd);(l.f4b3, l.f4ba, l.f4bb, l.f4bc, l.f4bd, ) = (tfe, 0.0, 0.0, 0.0, 0.0, );
        if (!(l.f4af > 0.0)) {l.f4af = 0.0;}
        if (!(l.f4c5 > 0.0)) {l.f4c5 = 0.0;}
        if (!(l.f4b3 > 0.0)) {(l.f4b3, l.f4ba, l.f4bb, l.f4bc, l.f4bd, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let tff: f64 = if l.f16ca == 1.0 { 1.0 } else { 0.0 };l.f9b0 = tff;
        if (l.f9b0 != 0.0) {let t100: f64 = (l.f1087 + l.f2cd);l.f107c = t100;let t101: f64 = (l.ff8).powf(1.5);let t102: f64 = (l.f1092 * l.f10de);let t103: f64 = (l.f107c * l.f10da);let t104: f64 = (t102 - t103);let t105: f64 = (0.5 * t104);let t106: f64 = (t105).exp();let t107: f64 = (t101 * t106);l.f55e = t107;let t108: f64 = (l.f191a * l.ff8);let t109: f64 = (2.0 * l.f10d8);let t10a: f64 = (l.f55e).ln();let t10b: f64 = (t109 * t10a);let t10c: f64 = (t108 - t10b);l.f1832 = t10c;let t10e: f64 = (0.05 - l.f1832);let t10f: f64 = (t10e * l.f10da);let t110: f64 = (t10f).exp();let t111: f64 = (1.0 + t110);let t112: f64 = (t111).ln();let t113: f64 = (l.f10d8 * t112);let t114: f64 = (l.f1832 + t113);l.f18f7 = t114;let t115: f64 = (1.0 / l.f18f7);l.f1905 = t115;let t116: f64 = (l.f191a * l.f1905);let t117: f64 = (t116).powf(l.f105d);let t118: f64 = (l.f1cd * t117);l.f1c4 = t118;let t119: f64 = (l.f1c4 * l.f18f7);let t11a: f64 = (t119 * l.f100b);l.f13d5 = t11a;let t11b: f64 = (2.0 * l.f1c4);l.f13c5 = t11b;}
        let t11c: f64 = (l.f1075 + l.f2cd);l.f1078 = t11c;let t11d: f64 = (l.f109d + l.f2cd);l.f1084 = t11d;let t11e: f64 = (l.f108b + l.f2cd);l.f1080 = t11e;let t11f: f64 = (l.ff8).powf(1.5);let t120: f64 = (l.f108e * l.f10de);let t121: f64 = (l.f1078 * l.f10da);let t122: f64 = (t120 - t121);let t123: f64 = (0.5 * t122);let t124: f64 = (t123).exp();let t125: f64 = (t11f * t124);l.f55a = t125;let t126: f64 = (l.ff8).powf(1.5);let t127: f64 = (l.f109a * l.f10de);let t128: f64 = (l.f1084 * l.f10da);let t129: f64 = (t127 - t128);let t12a: f64 = (0.5 * t129);let t12b: f64 = (t12a).exp();let t12c: f64 = (t126 * t12b);l.f566 = t12c;let t12e: f64 = (l.ff8).powf(1.5);let t12f: f64 = (l.f1096 * l.f10de);let t130: f64 = (l.f1080 * l.f10da);let t131: f64 = (t12f - t130);let t132: f64 = (0.5 * t131);let t133: f64 = (t132).exp();let t134: f64 = (t12e * t133);l.f562 = t134;let t135: f64 = (l.fd48 * l.f55a);let t136: f64 = (t135 * l.f55a);l.fd41 = t136;let t137: f64 = (l.fd4c * l.f566);let t138: f64 = (t137 * l.f566);l.fd4f = t138;let t139: f64 = (l.fd4a * l.f562);let t13a: f64 = (t139 * l.f562);l.fd45 = t13a;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        l: &mut StampLocals,
    ) {
        let t13b: f64 = (l.f1914 * l.ff8);let t13c: f64 = (2.0 * l.f10d8);let t13d: f64 = (l.f55a).ln();let t13e: f64 = (t13c * t13d);let t13f: f64 = (t13b - t13e);l.f182e = t13f;let t140: f64 = (l.f1924 * l.ff8);let t141: f64 = (2.0 * l.f10d8);let t142: f64 = (l.f566).ln();let t143: f64 = (t141 * t142);let t144: f64 = (t140 - t143);l.f183a = t144;let t145: f64 = (l.f191e * l.ff8);let t146: f64 = (2.0 * l.f10d8);let t147: f64 = (l.f562).ln();let t148: f64 = (t146 * t147);let t149: f64 = (t145 - t148);l.f1836 = t149;let t14a: f64 = (0.05 - l.f182e);let t14b: f64 = (t14a * l.f10da);let t14c: f64 = (t14b).exp();let t14d: f64 = (1.0 + t14c);let t14e: f64 = (t14d).ln();let t14f: f64 = (l.f10d8 * t14e);let t150: f64 = (l.f182e + t14f);l.f18f1 = t150;let t151: f64 = (0.05 - l.f183a);let t152: f64 = (t151 * l.f10da);let t153: f64 = (t152).exp();let t154: f64 = (1.0 + t153);let t155: f64 = (t154).ln();let t156: f64 = (l.f10d8 * t155);let t157: f64 = (l.f183a + t156);l.f192f = t157;let t158: f64 = (0.05 - l.f1836);let t159: f64 = (t158 * l.f10da);let t15a: f64 = (t159).exp();let t15b: f64 = (1.0 + t15a);let t15c: f64 = (t15b).ln();let t15d: f64 = (l.f10d8 * t15c);let t15e: f64 = (l.f1836 + t15d);l.f18fd = t15e;let t160: f64 = (1.0 / l.f18f1);l.f1901 = t160;let t161: f64 = (1.0 / l.f192f);l.f190d = t161;let t162: f64 = (1.0 / l.f18fd);l.f1909 = t162;let t163: f64 = (l.f1914 * l.f1901);let t164: f64 = (t163).powf(l.f1026);let t165: f64 = (l.f1cb * t164);l.f1c0 = t165;let t166: f64 = (l.f1924 * l.f190d);let t167: f64 = (t166).powf(l.f1146);let t168: f64 = (l.f1d3 * t167);l.f1d6 = t168;let t169: f64 = (l.f191e * l.f1909);let t16a: f64 = (t169).powf(l.f1061);let t16b: f64 = (l.f1d1 * t16a);l.f1c8 = t16b;let t16c: f64 = (l.f1c0 * l.f18f1);let t16d: f64 = (t16c * l.f1007);l.f13d1 = t16d;let t16e: f64 = (l.f1d6 * l.f192f);let t16f: f64 = (t16e * l.f1013);l.f13dd = t16f;let t170: f64 = (l.f1c8 * l.f18fd);let t171: f64 = (t170 * l.f100f);l.f13d9 = t171;let t172: f64 = (2.0 * l.f1c0);l.f13c1 = t172;let t174: f64 = (2.0 * l.f1d6);l.f13cd = t174;let t175: f64 = (2.0 * l.f1c8);l.f13c9 = t175;let t176: f64 = (0.5 * l.f1078);let t177: f64 = (t176).max(l.f10d8);l.f2c2 = t177;let t178: f64 = (0.5 * l.f1084);let t179: f64 = (t178).max(l.f10d8);l.f2ca = t179;let t17a: f64 = (0.5 * l.f1080);let t17b: f64 = (t17a).max(l.f10d8);l.f2c6 = t17b;let t17c: f64 = (l.f2c2 * l.f10da);l.fed = t17c;let t17d: f64 = (l.f2ca * l.f10da);l.ff5 = t17d;let t17e: f64 = (l.f2c6 * l.f10da);l.ff1 = t17e;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t17f: f64 = (32.0 * l.ff48);let t180: f64 = (t17f * 9.1093826e-31);let t181: f64 = (t180 * 1.6021918e-19);let t182: f64 = (l.f2c2 * l.f2c2);let t183: f64 = (t182 * l.f2c2);let t184: f64 = (t181 * t183);let t185: f64 = (t184).sqrt();let t186: f64 = (3.0 * 1.05457168e-34);let t187: f64 = (t185 / t186);l.f15a = t187;let t188: f64 = (32.0 * l.ff4c);let t189: f64 = (t188 * 9.1093826e-31);let t18a: f64 = (t189 * 1.6021918e-19);let t18b: f64 = (l.f2ca * l.f2ca);let t18c: f64 = (t18b * l.f2ca);let t18d: f64 = (t18a * t18c);let t18e: f64 = (t18d).sqrt();let t18f: f64 = (3.0 * 1.05457168e-34);let t190: f64 = (t18e / t18f);l.f162 = t190;let t192: f64 = (32.0 * l.ff4a);let t193: f64 = (t192 * 9.1093826e-31);let t194: f64 = (t193 * 1.6021918e-19);let t195: f64 = (l.f2c6 * l.f2c6);let t196: f64 = (t195 * l.f2c6);let t197: f64 = (t194 * t196);let t198: f64 = (t197).sqrt();let t199: f64 = (3.0 * 1.05457168e-34);let t19a: f64 = (t198 / t199);l.f15e = t19a;let t19b: f64 = (l.f17a4 - l.f17ac);let t19c: f64 = (l.f169e * t19b);let t19d: f64 = (1.0 + t19c);let t19e: f64 = (l.f4bf * t19d);l.f4b0 = t19e;let t19f: f64 = (l.f17a4 - l.f17ac);let t1a0: f64 = (l.f16a2 * t19f);let t1a1: f64 = (1.0 + t1a0);let t1a2: f64 = (l.f4c3 * t1a1);l.f4c6 = t1a2;let t1a3: f64 = (l.f17a4 - l.f17ac);let t1a4: f64 = (l.f16a0 * t1a3);let t1a5: f64 = (1.0 + t1a4);let t1a6: f64 = (l.f4c1 * t1a5);(l.f4b4, l.f4b5, l.f4b6, l.f4b7, l.f4b8, ) = (t1a6, 0.0, 0.0, 0.0, 0.0, );
        if (!(l.f4b0 > 0.0)) {l.f4b0 = 0.0;}
        if (!(l.f4c6 > 0.0)) {l.f4c6 = 0.0;}
        if (!(l.f4b4 > 0.0)) {(l.f4b4, l.f4b5, l.f4b6, l.f4b7, l.f4b8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t1a7: f64 = if l.f16cb == 1.0 { 1.0 } else { 0.0 };l.f9bc = t1a7;
        if (l.f9bc != 0.0) {let t1a8: f64 = (l.f1088 + l.f2cd);l.f107d = t1a8;let t1aa: f64 = (l.ff8).powf(1.5);let t1ab: f64 = (l.f1093 * l.f10de);let t1ac: f64 = (l.f107d * l.f10da);let t1ad: f64 = (t1ab - t1ac);let t1ae: f64 = (0.5 * t1ad);let t1af: f64 = (t1ae).exp();let t1b0: f64 = (t1aa * t1af);l.f55f = t1b0;let t1b2: f64 = (l.f191b * l.ff8);let t1b3: f64 = (2.0 * l.f10d8);let t1b4: f64 = (l.f55f).ln();let t1b5: f64 = (t1b3 * t1b4);let t1b6: f64 = (t1b2 - t1b5);l.f1833 = t1b6;let t1b7: f64 = (0.05 - l.f1833);let t1b8: f64 = (t1b7 * l.f10da);let t1b9: f64 = (t1b8).exp();let t1ba: f64 = (1.0 + t1b9);let t1bb: f64 = (t1ba).ln();let t1bc: f64 = (l.f10d8 * t1bb);let t1bd: f64 = (l.f1833 + t1bc);l.f18f8 = t1bd;let t1be: f64 = (1.0 / l.f18f8);l.f1906 = t1be;let t1bf: f64 = (l.f191b * l.f1906);let t1c0: f64 = (t1bf).powf(l.f105e);let t1c1: f64 = (l.f1ce * t1c0);l.f1c5 = t1c1;let t1c2: f64 = (l.f1c5 * l.f18f8);let t1c3: f64 = (t1c2 * l.f100c);l.f13d6 = t1c3;let t1c4: f64 = (2.0 * l.f1c5);l.f13c6 = t1c4;}
        l.ffbf = 1.0;l.fe57 = 1.0;l.fef1 = 0.0;l.f1afd = 0.0;l.fee0 = p.p0;l.f1ad4 = p.p1;l.f1487 = p.p2;l.f1489 = p.p3;l.f1493 = p.p4;l.f148b = p.p8;l.f1caf = p.p11;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        l.f1b = p.p19;l.ff0f = p.p20;l.fefb = p.p21;l.f17 = p.p22;l.ff0b = p.p23;l.fef7 = p.p24;l.fd0 = p.p25;l.f1122 = p.p26;l.f1d = p.p27;l.f1049 = p.p28;l.fe93 = p.p14;let t1cd: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };l.f9c8 = t1cd;
        if (l.f9c8 != 0.0) {
            let (t1ce,) = {
    if (p.p9 > 1.0) {
        (p.p9,)
    } else {
        (1.0,)
    }
};
            l.ffbf = t1ce;
        }
        if (l.f9c8 != 0.0) {let t1d1: f64 = (l.ffbf + 0.5);let t1d2: f64 = (t1d1).floor();l.ffbf = t1d2;let t1d3: f64 = (1.0 / l.ffbf);l.fe57 = t1d3;}
        let t1d4: f64 = (l.f1ad4 * l.fe57);
        let (t1d6,) = {
    if (t1d4 > 1e-9) {
        let t1d5: f64 = (l.f1ad4 * l.fe57);
        (t1d5,)
    } else {
        (1e-9,)
    }
};
        l.f1ad4 = t1d6;l.f148d = p.p5;l.f148f = p.p6;l.f1491 = p.p7;
        let (t1d7,) = {
    if (p.p10 < 1.5) {
        (1.0,)
    } else {
        (2.0,)
    }
};
        l.ffc1 = t1d7;let t1d8: f64 = (1e-6 / l.fee0);l.fdef = t1d8;let t1d9: f64 = (1e-6 / l.f1ad4);l.fe8f = t1d9;let t1da: f64 = (p.p190 * l.fdef);let t1db: f64 = (1.0 + t1da);let t1dc: f64 = (p.p189 * t1db);let t1dd: f64 = (p.p191 * l.fe8f);let t1de: f64 = (1.0 + t1dd);let t1df: f64 = (t1dc * t1de);l.f266 = t1df;let t1e2: f64 = (p.p194 * l.fdef);let t1e3: f64 = (1.0 + t1e2);let t1e4: f64 = (p.p193 * t1e3);let t1e5: f64 = (p.p195 * l.fe8f);let t1e6: f64 = (1.0 + t1e5);let t1e7: f64 = (t1e4 * t1e6);l.f2e8 = t1e7;let t1e8: f64 = (l.fee0 + l.f266);let t1e9: f64 = (2.0 * p.p192);let t1ea: f64 = (t1e8 - t1e9);
        let (t1ee,) = {
    if (t1ea > 1e-9) {
        let t1eb: f64 = (l.fee0 + l.f266);let t1ec: f64 = (2.0 * p.p192);let t1ed: f64 = (t1eb - t1ec);
        (t1ed,)
    } else {
        (1e-9,)
    }
};
        l.fef1 = t1ee;let t1ef: f64 = (l.f1ad4 + l.f2e8);let t1f0: f64 = (2.0 * p.p196);let t1f1: f64 = (t1ef - t1f0);
        let (t1f5,) = {
    if (t1f1 > 1e-9) {
        let t1f2: f64 = (l.f1ad4 + l.f2e8);let t1f3: f64 = (2.0 * p.p196);let t1f4: f64 = (t1f2 - t1f3);
        (t1f4,)
    } else {
        (1e-9,)
    }
};
        l.f1afd = t1f5;let t1f6: f64 = (1e-6 / l.fef1);l.fdf1 = t1f6;let t1f7: f64 = (l.fdf1 * l.fdf1);l.fdf2 = t1f7;let t1f8: f64 = (1e-6 / l.f1afd);l.fe91 = t1f8;let t1f9: f64 = (1.0 / l.fe91);l.fd8f = t1f9;let t1fa: f64 = (l.fdf1 * l.fe91);l.fd25 = t1fa;let t1fb: f64 = (1.0 / l.fd25);l.fd82 = t1fb;let t1fc: f64 = (l.fee0 + l.f266);let t1fd: f64 = (2.0 * p.p192);let t1fe: f64 = (t1fc - t1fd);let t1ff: f64 = (t1fe + p.p197);
        let (t204,) = {
    if (t1ff > 1e-9) {
        let t200: f64 = (l.fee0 + l.f266);let t201: f64 = (2.0 * p.p192);let t202: f64 = (t200 - t201);let t203: f64 = (t202 + p.p197);
        (t203,)
    } else {
        (1e-9,)
    }
};
        l.fef3 = t204;let t208: f64 = (l.f1ad4 + l.f2e8);let t209: f64 = (2.0 * p.p196);let t20a: f64 = (t208 - t209);let t20b: f64 = (t20a + p.p198);
        let (t210,) = {
    if (t20b > 1e-9) {
        let t20c: f64 = (l.f1ad4 + l.f2e8);let t20d: f64 = (2.0 * p.p196);let t20e: f64 = (t20c - t20d);let t20f: f64 = (t20e + p.p198);
        (t20f,)
    } else {
        (1e-9,)
    }
};
        l.f1b01 = t210;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t211: f64 = (l.f1b01 / 1e-6);l.fd91 = t211;let t212: f64 = (l.fee0 + l.f266);let t213: f64 = (t212 + p.p197);
        let (t216,) = {
    if (t213 > 1e-9) {
        let t214: f64 = (l.fee0 + l.f266);let t215: f64 = (t214 + p.p197);
        (t215,)
    } else {
        (1e-9,)
    }
};
        l.feef = t216;let t217: f64 = (l.f1ad4 + l.f2e8);let t218: f64 = (t217 + p.p198);
        let (t21b,) = {
    if (t218 > 1e-9) {
        let t219: f64 = (l.f1ad4 + l.f2e8);let t21a: f64 = (t219 + p.p198);
        (t21a,)
    } else {
        (1e-9,)
    }
};
        l.f1ad6 = t21b;let t21c: f64 = (l.feef / 1e-6);l.fd84 = t21c;let t21d: f64 = (l.f1ad6 / 1e-6);l.fd8d = t21d;let t21e: f64 = (l.fee0 + l.f266);
        let (t220,) = {
    if (t21e > 1e-9) {
        let t21f: f64 = (l.fee0 + l.f266);
        (t21f,)
    } else {
        (1e-9,)
    }
};
        l.fedf = t220;let t221: f64 = (l.fedf + p.p444);
        let (t223,) = {
    if (t221 > 1e-9) {
        let t222: f64 = (l.fedf + p.p444);
        (t222,)
    } else {
        (1e-9,)
    }
};
        l.fee2 = t223;let t224: f64 = (l.f1ad4 + l.f2e8);
        let (t226,) = {
    if (t224 > 1e-9) {
        let t225: f64 = (l.f1ad4 + l.f2e8);
        (t225,)
    } else {
        (1e-9,)
    }
};
        l.f1ad3 = t226;let t227: f64 = (0.5 * l.f2e8);let t228: f64 = (l.f1caf - t227);
        let (t22b,) = {
    if (t228 > 1e-9) {
        let t229: f64 = (0.5 * l.f2e8);let t22a: f64 = (l.f1caf - t229);
        (t22a,)
    } else {
        (1e-9,)
    }
};
        l.f1cb0 = t22b;l.f19a4 = p.p56;l.f16c0 = p.p57;l.f1680 = p.p58;l.f17e1 = p.p59;l.f41f = p.p60;l.ffb7 = p.p61;l.f5f4 = p.p62;l.f1a72 = p.p63;l.f3c9 = p.p64;l.f328 = p.p65;l.ffcd = p.p66;l.f17e7 = p.p67;l.f17eb = p.p68;l.ffc4 = p.p69;l.ffc8 = p.p70;l.f209 = p.p71;l.f21e = p.p73;l.f216 = p.p72;l.f169c = p.p74;l.f1127 = p.p78;l.f1133 = p.p80;l.f112b = p.p79;l.f174 = p.p75;l.f180 = p.p77;l.f178 = p.p76;l.f11d = p.p81;l.f1688 = p.p82;l.ff99 = p.p83;l.f16aa = p.p84;l.f1731 = p.p85;l.f16b8 = p.p86;l.f1ee = p.p87;l.f1698 = p.p88;l.f172a = p.p89;l.f16b4 = p.p90;l.f1bee = p.p91;l.f16c8 = p.p92;l.f4eb = p.p93;l.f1437 = p.p94;l.f16ae = p.p95;l.f143e = p.p96;l.f1444 = p.p97;l.f175d = p.p98;l.f16bc = p.p99;l.f1775 = p.p100;l.f1795 = p.p101;l.f179f = p.p102;l.ffc = p.p103;l.f43 = p.p104;l.f37 = p.p105;l.f3f = p.p106;l.f1a6e = p.p107;l.f2 = p.p108;l.f6 = p.p109;l.f1684 = p.p110;l.fd = p.p111;l.f11 = p.p112;l.fdf7 = p.p113;l.f5a4 = p.p114;l.fd72 = p.p115;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        l.fd76 = p.p116;l.fd7a = p.p117;l.f16a6 = p.p118;l.f58c = p.p119;l.f598 = p.p120;l.f590 = p.p119;let t244: f64 = if param_given[121] { 1.0 } else { 0.0 };let t245: f64 = if t244 == 1.0 { 1.0 } else { 0.0 };l.f9d6 = t245;
        if (l.f9d6 != 0.0) {l.f590 = p.p121;}
        l.f59c = p.p120;let t246: f64 = if param_given[122] { 1.0 } else { 0.0 };let t247: f64 = if t246 == 1.0 { 1.0 } else { 0.0 };l.f9e2 = t247;
        if (l.f9e2 != 0.0) {l.f59c = p.p122;}
        l.f594 = l.f590;let t24f: f64 = if param_given[123] { 1.0 } else { 0.0 };let t250: f64 = if t24f == 1.0 { 1.0 } else { 0.0 };l.f9ee = t250;
        if (l.f9ee != 0.0) {l.f594 = p.p123;}
        l.f5a0 = l.f59c;let t251: f64 = if param_given[124] { 1.0 } else { 0.0 };let t252: f64 = if t251 == 1.0 { 1.0 } else { 0.0 };l.f9fa = t252;
        if (l.f9fa != 0.0) {l.f5a0 = p.p124;}
        l.f1b3 = p.p125;l.f2b = p.p126;l.f2f = p.p127;l.f131 = p.p128;l.f137 = p.p129;l.f1690 = p.p130;l.f1694 = p.p131;l.f19f = p.p132;l.f1a3 = p.p133;l.f1dd = p.p134;l.f2e2 = p.p135;l.f494 = p.p136;l.f1764 = p.p98;let t254: f64 = if param_given[137] { 1.0 } else { 0.0 };let t255: f64 = if t254 == 1.0 { 1.0 } else { 0.0 };l.fa06 = t255;
        if (l.fa06 != 0.0) {l.f1764 = p.p137;}
        l.f100 = p.p103;let t256: f64 = if param_given[138] { 1.0 } else { 0.0 };let t257: f64 = if t256 == 1.0 { 1.0 } else { 0.0 };l.fa12 = t257;
        if (l.fa12 != 0.0) {l.f100 = p.p138;}
        l.f47 = p.p139;l.f3b = p.p140;l.f1a7 = p.p141;l.f1af = p.p142;l.f4d9 = p.p143;l.f4dd = p.p144;l.f1ab = p.p145;l.f194 = p.p146;l.f1b9 = p.p147;l.f1bd = p.p148;l.f3b0 = p.p149;l.f4e5 = p.p150;l.f4e1 = p.p151;l.f108 = p.p152;l.f18c = p.p153;l.f190 = p.p154;l.f521 = p.p155;l.f524 = p.p156;l.f19ab = p.p161;l.f16c4 = p.p162;l.f32c = p.p163;l.ffbd = p.p164;l.f21a = p.p165;l.f124 = p.p166;l.f168c = p.p167;l.f113b = p.p168;l.f112f = p.p169;l.f1137 = p.p170;l.f188 = p.p171;l.f184 = p.p173;l.f17c = p.p172;l.f1402 = p.p179;l.f1441 = p.p180;l.f1400 = p.p181;l.f1451 = p.p183;l.f13fe = p.p182;l.f1434 = p.p184;l.f1432 = p.p185;l.f144b = p.p186;l.f225 = p.p187;l.f16b1 = p.p188;let t266: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };l.fa1e = t266;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fa1e != 0.0) {let t267: f64 = (l.fdf1).powf(p.p201);let t268: f64 = (p.p200 * t267);let t269: f64 = (p.p199 + t268);let t26a: f64 = (p.p202 * l.fe91);let t26b: f64 = (t269 + t26a);let t26c: f64 = (p.p203 * l.fd25);let t26d: f64 = (t26b + t26c);l.f19a4 = t26d;let t26e: f64 = (p.p205 * l.fdf1);let t26f: f64 = (p.p204 + t26e);let t270: f64 = (p.p206 * l.fe91);let t271: f64 = (t26f + t270);let t272: f64 = (p.p207 * l.fd25);let t273: f64 = (t271 + t272);l.f16c0 = t273;l.f1680 = p.p208;l.f17e1 = p.p209;l.f41f = p.p210;}
        if (l.fa1e != 0.0) {
            let t275: f64 = (p.p212 * l.fe91);let t276: f64 = (l.f1afd / p.p213);let t277: f64 = (1.0 + t276);let t278: f64 = (t277).ln();let t279: f64 = (t275 * t278);let t27a: f64 = (1.0 + t279);
            let (t281,) = {
    if (t27a > 0.001) {
        let t27b: f64 = (p.p212 * l.fe91);let t27c: f64 = (l.f1afd / p.p213);let t27d: f64 = (1.0 + t27c);let t27e: f64 = (t27d).ln();let t27f: f64 = (t27b * t27e);let t280: f64 = (1.0 + t27f);
        (t280,)
    } else {
        (0.001,)
    }
};
            let t282: f64 = (p.p211 * t281);l.ffe1 = t282;
        }
        if (l.fa1e != 0.0) {
            let t283: f64 = (p.p215 * l.fe91);let t284: f64 = (l.f1afd / p.p216);let t285: f64 = (1.0 + t284);let t286: f64 = (t285).ln();let t287: f64 = (t283 * t286);let t288: f64 = (1.0 + t287);
            let (t28f,) = {
    if (t288 > 0.001) {
        let t289: f64 = (p.p215 * l.fe91);let t28a: f64 = (l.f1afd / p.p216);let t28b: f64 = (1.0 + t28a);let t28c: f64 = (t28b).ln();let t28d: f64 = (t289 * t28c);let t28e: f64 = (1.0 + t28d);
        (t28e,)
    } else {
        (0.001,)
    }
};
            let t290: f64 = (p.p214 * t28f);l.ffd0 = t290;
        }
        if (l.fa1e != 0.0) {
            let t291: f64 = (p.p218 * l.fe91);let t292: f64 = (l.f1afd / p.p216);let t293: f64 = (1.0 + t292);let t294: f64 = (t293).ln();let t295: f64 = (t291 * t294);let t296: f64 = (1.0 + t295);
            let (t29d,) = {
    if (t296 > 0.001) {
        let t297: f64 = (p.p218 * l.fe91);let t298: f64 = (l.f1afd / p.p216);let t299: f64 = (1.0 + t298);let t29a: f64 = (t299).ln();let t29b: f64 = (t297 * t29a);let t29c: f64 = (1.0 + t29b);
        (t29c,)
    } else {
        (0.001,)
    }
};
            let t29e: f64 = (p.p217 * t29d);l.ff07 = t29e;
        }
        let t29f: f64 = (2.0 * l.ff07);let t2a0: f64 = if l.fef1 > t29f { 1.0 } else { 0.0 };l.fa2a = t2a0;
        if ((l.fa1e != 0.0) && (l.fa2a != 0.0)) {l.f13 = 75000000000.0;let t2a1: f64 = (0.5 * l.ffd0);let t2a2: f64 = (l.ffe1 + t2a1);let t2a3: f64 = (t2a2).sqrt();let t2a4: f64 = (l.ffe1).sqrt();let t2a5: f64 = (t2a3 - t2a4);l.f10c = t2a5;let t2a6: f64 = (l.ffe1).sqrt();let t2a7: f64 = (2.0 * l.ff07);let t2a8: f64 = (t2a7 / l.fef1);let t2a9: f64 = (l.f10c / l.f13);let t2aa: f64 = (t2a9).exp();let t2ab: f64 = (t2aa - 1.0);let t2ac: f64 = (t2a8 * t2ab);let t2ad: f64 = (1.0 + t2ac);let t2ae: f64 = (t2ad).ln();let t2af: f64 = (l.f13 * t2ae);let t2b0: f64 = (t2a6 + t2af);l.ffe0 = t2b0;let t2b1: f64 = (l.ffe0 * l.ffe0);l.ffe0 = t2b1;}
        let t2b2: f64 = if l.fef1 >= l.ff07 { 1.0 } else { 0.0 };l.fa36 = t2b2;
        if (((l.fa1e != 0.0) && (l.fa2a == 0.0)) && (l.fa36 != 0.0)) {let t2b4: f64 = (l.ffd0 * l.ff07);let t2b5: f64 = (t2b4 / l.fef1);let t2b6: f64 = (l.ffe1 + t2b5);l.ffe0 = t2b6;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.fa1e != 0.0) && (l.fa2a == 0.0)) && (l.fa36 == 0.0)) {let t2b7: f64 = (l.fef1 / l.ff07);let t2b8: f64 = (2.0 - t2b7);let t2b9: f64 = (l.ffd0 * t2b8);let t2ba: f64 = (l.ffe1 + t2b9);l.ffe0 = t2ba;}
        if (l.fa1e != 0.0) {let t2bb: f64 = (p.p219 * l.fdf1);let t2bc: f64 = (1.0 - t2bb);let t2bd: f64 = (p.p220 * l.fdf2);let t2be: f64 = (t2bc - t2bd);let t2bf: f64 = (l.ffe0 * t2be);l.ffb7 = t2bf;let t2c0: f64 = (l.fdf1).powf(p.p223);let t2c1: f64 = (p.p222 * t2c0);let t2c2: f64 = (p.p221 + t2c1);let t2c3: f64 = (p.p224 * l.fe91);let t2c4: f64 = (t2c2 + t2c3);let t2c5: f64 = (p.p225 * l.fd25);let t2c6: f64 = (t2c4 + t2c5);l.f5f4 = t2c6;l.f1a72 = p.p226;l.f3c9 = p.p227;let t2c7: f64 = (l.fdf1).powf(p.p230);let t2c8: f64 = (p.p229 * t2c7);let t2c9: f64 = (p.p228 + t2c8);let t2ca: f64 = (p.p231 * l.fe91);let t2cb: f64 = (t2c9 + t2ca);let t2cc: f64 = (p.p232 * l.fd25);let t2cd: f64 = (t2cb + t2cc);l.f328 = t2cd;}
        if (l.fa1e != 0.0) {
            let t2ce: f64 = (p.p234 * l.fdf1);let t2cf: f64 = (1.0 + t2ce);
            let (t2d2,) = {
    if (1e-6 > t2cf) {
        (1e-6,)
    } else {
        let t2d0: f64 = (p.p234 * l.fdf1);let t2d1: f64 = (1.0 + t2d0);
        (t2d1,)
    }
};
            let t2d3: f64 = (p.p233 * t2d2);l.ffcd = t2d3;
        }
        if (l.fa1e != 0.0) {l.f17e7 = p.p235;l.f17eb = p.p236;l.ffc4 = p.p239;l.ffc8 = p.p240;let t2d5: f64 = (l.fdf1).powf(p.p243);let t2d6: f64 = (p.p242 * t2d5);let t2d7: f64 = (p.p241 + t2d6);let t2d8: f64 = (p.p244 * l.fe91);let t2d9: f64 = (1.0 + t2d8);let t2da: f64 = (t2d7 * t2d9);let t2db: f64 = (p.p245 * l.fd25);let t2dc: f64 = (1.0 + t2db);let t2dd: f64 = (t2da * t2dc);l.f209 = t2dd;l.f21e = p.p247;l.f216 = p.p246;l.f169c = p.p248;let t2de: f64 = (l.fdf1).powf(p.p250);let t2df: f64 = (p.p249 * t2de);let t2e0: f64 = (p.p251 * l.fe91);let t2e1: f64 = (1.0 + t2e0);let t2e2: f64 = (t2df * t2e1);l.f174 = t2e2;l.f180 = p.p253;l.f178 = p.p252;let t2e3: f64 = (l.fdf1).powf(p.p255);let t2e4: f64 = (p.p254 * t2e3);let t2e5: f64 = (p.p256 * l.fe91);let t2e6: f64 = (1.0 + t2e5);let t2e7: f64 = (t2e4 * t2e6);l.f1127 = t2e7;l.f1133 = p.p258;l.f112b = p.p257;let t2e9: f64 = (p.p261 * l.fe91);let t2ea: f64 = (1.0 + t2e9);let t2eb: f64 = (p.p260 * t2ea);l.f4c9 = t2eb;}
        if (l.fa1e != 0.0) {
            let t2ec: f64 = (p.p263 * l.fe91);let t2ed: f64 = (1.0 + t2ec);
            let (t2f0,) = {
    if (t2ed > 0.001) {
        let t2ee: f64 = (p.p263 * l.fe91);let t2ef: f64 = (1.0 + t2ee);
        (t2ef,)
    } else {
        (0.001,)
    }
};
            let t2f1: f64 = (p.p262 * t2f0);l.ff05 = t2f1;
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fa1e != 0.0) {let t2f2: f64 = (l.f4c9 * l.ff05);let t2f3: f64 = (t2f2 / l.fef1);let t2f4: f64 = (-l.fef1);let t2f5: f64 = (t2f4 / l.ff05);let t2f6: f64 = (t2f5).exp();let t2f7: f64 = (1.0 - t2f6);let t2f8: f64 = (t2f3 * t2f7);let t2f9: f64 = (1.0 + t2f8);let t2fa: f64 = (p.p264 * p.p265);let t2fb: f64 = (t2fa / l.fef1);let t2fc: f64 = (-l.fef1);let t2fd: f64 = (t2fc / p.p265);let t2fe: f64 = (t2fd).exp();let t2ff: f64 = (1.0 - t2fe);let t300: f64 = (t2fb * t2ff);let t301: f64 = (t2f9 + t300);l.f663 = t301;}
        if (l.fa1e != 0.0) {
            let (t302,) = {
    if (l.f663 > 1e-15) {
        (l.f663,)
    } else {
        (1e-15,)
    }
};
            l.f663 = t302;
        }
        if (l.fa1e != 0.0) {let t303: f64 = (p.p266 * l.fe91);let t304: f64 = (1.0 + t303);let t305: f64 = (p.p267 * l.fe91);let t306: f64 = (l.f1afd / p.p268);let t307: f64 = (1.0 + t306);let t308: f64 = (t307).ln();let t309: f64 = (t305 * t308);let t30a: f64 = (t304 + t309);l.fc94 = t30a;let t30b: f64 = (p.p259 * l.f1afd);let t30c: f64 = (l.f663 * l.fef1);let t30d: f64 = (t30b / t30c);let t30e: f64 = (t30d * l.fc94);l.f11d = t30e;let t30f: f64 = (p.p270 * l.fdf1);let t310: f64 = (p.p269 + t30f);let t311: f64 = (p.p271 * l.fe91);let t312: f64 = (t310 + t311);let t313: f64 = (p.p272 * l.fd25);let t314: f64 = (t312 + t313);l.f1688 = t314;let t315: f64 = (p.p274 * l.fe91);let t316: f64 = (1.0 + t315);let t317: f64 = (p.p273 * t316);l.ff99 = t317;l.f16aa = p.p275;l.f1731 = p.p276;l.f16b8 = p.p277;let t319: f64 = (l.fdf1).powf(p.p280);let t31a: f64 = (p.p279 * t319);let t31b: f64 = (p.p278 + t31a);let t31c: f64 = (p.p281 * l.fe91);let t31d: f64 = (1.0 + t31c);let t31e: f64 = (t31b * t31d);let t31f: f64 = (p.p282 * l.fd25);let t320: f64 = (1.0 + t31f);let t321: f64 = (t31e * t320);l.f1ee = t321;l.f1698 = p.p283;l.f172a = p.p284;l.f16b4 = p.p285;let t322: f64 = (p.p287 * l.fdf1);let t323: f64 = (1.0 + t322);let t324: f64 = (p.p286 * t323);let t325: f64 = (p.p288 * l.fe91);let t326: f64 = (1.0 + t325);let t327: f64 = (t324 * t326);let t328: f64 = (p.p289 * l.fd25);let t329: f64 = (1.0 + t328);let t32a: f64 = (t327 * t329);l.f1bee = t32a;l.f16c8 = p.p290;l.f4eb = p.p291;let t32b: f64 = (p.p292 * l.fe91);let t32c: f64 = (p.p293 * l.fe91);let t32d: f64 = (1.0 + t32c);let t32e: f64 = (t32b * t32d);l.f1437 = t32e;l.f16ae = p.p294;l.f143e = p.p295;l.f1444 = p.p296;let t32f: f64 = (p.p298 * l.fc94);let t330: f64 = (t32f / l.f663);let t331: f64 = (l.fdf1).powf(p.p299);let t332: f64 = (t330 * t331);let t333: f64 = (p.p297 + t332);let t334: f64 = (p.p300 * l.fe91);let t335: f64 = (1.0 + t334);let t336: f64 = (t333 * t335);let t337: f64 = (p.p301 * l.fd25);let t338: f64 = (1.0 + t337);let t339: f64 = (t336 * t338);l.f175d = t339;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if (l.fa1e != 0.0) {let t33a: f64 = (p.p303 * l.fdf1);let t33b: f64 = (p.p302 + t33a);let t33c: f64 = (p.p304 * l.fe91);let t33d: f64 = (t33b + t33c);let t33e: f64 = (p.p305 * l.fd25);let t33f: f64 = (t33d + t33e);l.f16bc = t33f;l.f1775 = p.p306;l.f1795 = p.p307;l.f179f = p.p308;let t340: f64 = (p.p310 * l.fdf1);let t341: f64 = (1.0 + t340);let t342: f64 = (p.p309 / t341);l.ffc = t342;let t343: f64 = (l.fdf1).powf(p.p312);let t344: f64 = (p.p311 * t343);let t345: f64 = (p.p313 * l.fe91);let t346: f64 = (1.0 + t345);let t347: f64 = (t344 * t346);l.f43 = t347;let t348: f64 = (l.fdf1).powf(p.p315);l.f17dd = t348;let t349: f64 = (p.p314 * l.f17dd);let t34a: f64 = (p.p317 * l.fe91);let t34b: f64 = (1.0 + t34a);let t34c: f64 = (t349 * t34b);let t34d: f64 = (p.p316 * l.fdf1);let t34e: f64 = (t34d * l.f17dd);let t34f: f64 = (1.0 + t34e);let t350: f64 = (t34c / t34f);l.f37 = t350;let t351: f64 = (l.fdf1).powf(p.p319);l.f17dd = t351;let t352: f64 = (p.p318 * l.f17dd);let t353: f64 = (p.p321 * l.fe91);let t354: f64 = (1.0 + t353);let t355: f64 = (t352 * t354);let t356: f64 = (p.p320 * l.fdf1);let t357: f64 = (t356 * l.f17dd);let t358: f64 = (1.0 + t357);let t359: f64 = (t355 / t358);l.f3f = t359;l.f1a6e = p.p322;let t35a: f64 = (p.p324 * l.fdf1);let t35b: f64 = (1.0 + t35a);let t35c: f64 = (p.p323 * t35b);let t35d: f64 = (p.p325 * l.fe91);let t35e: f64 = (1.0 + t35d);let t35f: f64 = (t35c * t35e);l.f2 = t35f;l.f6 = p.p326;l.f1684 = p.p327;let t360: f64 = (p.p329 * l.fdf1);let t361: f64 = (1.0 + t360);let t362: f64 = (p.p328 * t361);let t363: f64 = (p.p330 * l.fe91);let t364: f64 = (1.0 + t363);let t365: f64 = (t362 * t364);l.fd = t365;let t366: f64 = (p.p332 * l.fdf1);let t367: f64 = (1.0 + t366);let t368: f64 = (p.p331 * t367);let t369: f64 = (p.p333 * l.fe91);let t36a: f64 = (1.0 + t369);let t36b: f64 = (t368 * t36a);l.f11 = t36b;l.fdf7 = p.p334;l.f5a4 = p.p335;let t36c: f64 = (p.p336 / l.fd25);l.fd72 = t36c;let t36d: f64 = (p.p337 * p.p237);let t36e: f64 = (1e-6 * l.fe91);let t36f: f64 = (t36d / t36e);l.fd76 = t36f;let t370: f64 = (p.p338 * p.p238);let t371: f64 = (1e-6 * l.fe91);let t372: f64 = (t370 / t371);l.fd7a = t372;l.f16a6 = p.p339;l.f58c = p.p340;l.f598 = p.p341;l.f590 = p.p340;}
        let t373: f64 = if param_given[342] { 1.0 } else { 0.0 };let t374: f64 = if t373 == 1.0 { 1.0 } else { 0.0 };l.fa42 = t374;
        if ((l.fa1e != 0.0) && (l.fa42 != 0.0)) {l.f590 = p.p342;}
        if (l.fa1e != 0.0) {l.f59c = p.p341;}
        let t375: f64 = if param_given[343] { 1.0 } else { 0.0 };let t376: f64 = if t375 == 1.0 { 1.0 } else { 0.0 };l.fa50 = t376;
        if ((l.fa1e != 0.0) && (l.fa50 != 0.0)) {l.f59c = p.p343;}
        if (l.fa1e != 0.0) {l.f594 = l.f590;}
        let t377: f64 = if param_given[344] { 1.0 } else { 0.0 };let t378: f64 = if t377 == 1.0 { 1.0 } else { 0.0 };l.fa5c = t378;
        if ((l.fa1e != 0.0) && (l.fa5c != 0.0)) {l.f594 = p.p344;}
    }
}
