#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t0: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };l.f5e4 = t0;
        if (l.f5e4 != 0.0) {let t2: f64 = 1.0;l.f193 = t2;}
        if (l.f5e4 == 0.0) {let t62: f64 = (-1.0);l.f193 = t62;}
        let t196: f64 = (8.8541878176e-12 * 11.8);l.f3b7 = t196;let t24a: f64 = (273.15 + p.p38);l.f15cd = t24a;l.f1510 = 0.0;let t389: f64 = if p.p920 > 0.5 { 1.0 } else { 0.0 };l.f8eb = t389;
        if (l.f8eb != 0.0) {l.f1510 = 1.0;}
        if (l.f8eb == 0.0) {l.f1510 = 0.0;}
        let t38a: f64 = (273.15 + p.p816);l.f15ce = t38a;let t1: f64 = (1.3806505e-23 / 1.6021918e-19);l.fddc = t1;let t3: f64 = (l.fddc * l.f15ce);l.ffd2 = t3;let t4: f64 = (1.0 / l.ffd2);l.ffd4 = t4;let t5: f64 = (0.000702 * l.f15ce);let t6: f64 = (t5 * l.f15ce);let t7: f64 = (-t6);let t8: f64 = (1108.0 + l.f15ce);let t9: f64 = (t7 / t8);l.f28d = t9;let ta: f64 = (p.p827 + l.f28d);l.ff8c = ta;let tb: f64 = (p.p828 + l.f28d);l.ff98 = tb;let tc: f64 = (p.p829 + l.f28d);l.ff90 = tc;let t15: f64 = (1.0 - p.p824);l.ff0f = t15;let t22: f64 = (1.0 - p.p825);l.ff1b = t22;let t45: f64 = (1.0 - p.p826);l.ff13 = t45;let t58: f64 = (1.0 / l.ff0f);l.ff1f = t58;let t63: f64 = (1.0 / l.ff1b);l.ff2b = t63;let t7a: f64 = (1.0 / l.ff13);l.ff23 = t7a;let t9a: f64 = (l.f3b7 / p.p818);l.f18d0 = t9a;let tcc: f64 = (p.p836 * l.f3b7);let tcd: f64 = (tcc / p.p819);l.f18e4 = tcd;let te1: f64 = (p.p837 * l.f3b7);let te2: f64 = (te1 / p.p820);l.f18d4 = te2;let t100: f64 = (1.0 / l.f18d0);l.f18d8 = t100;let t11f: f64 = (1.0 / l.f18e4);l.f18e0 = t11f;let t13d: f64 = (1.0 / l.f18d4);l.f18dc = t13d;let t16f: f64 = (1.0 / p.p821);l.f1724 = t16f;let t189: f64 = (1.0 / p.p822);l.f1734 = t189;let t1a0: f64 = (1.0 / p.p823);l.f172e = t1a0;let t1c1: f64 = (1.772453850905516 * 0.29214664);l.ff5f = t1c1;let t1dc: f64 = (-5.0);let t1dd: f64 = (t1dc * 0.29214664);let t1de: f64 = (t1dd + 6.0);let t1df: f64 = (-2.0);let t1e0: f64 = (l.ff5f).powf(t1df);let t1e1: f64 = (t1de - t1e0);let t1e2: f64 = (t1e1 / 3.0);l.ffc = t1e2;let t1e3: f64 = (1.0 - 0.29214664);let t1e4: f64 = (t1e3 - l.ffc);l.f150 = t1e4;let t1e6: f64 = (1.0 / p.p817);let t1e7: f64 = (1.0 - t1e6);l.f6d = t1e7;let t1f2: f64 = (l.f6d).powf(p.p856);let t1f3: f64 = (1.0 - t1f2);let t1f4: f64 = (1.0 / t1f3);l.f4d5 = t1f4;let t215: f64 = (l.f6d).powf(p.p857);let t216: f64 = (1.0 - t215);let t217: f64 = (1.0 / t216);l.f4d9 = t217;let t240: f64 = (l.f6d).powf(p.p858);let t241: f64 = (1.0 - t240);let t242: f64 = (1.0 / t241);l.f4d7 = t242;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t248: f64 = (1.0 / p.p853);l.f1750 = t248;let t249: f64 = (1.0 / p.p854);l.f1760 = t249;let t24b: f64 = (1.0 / p.p855);(l.f1754, l.f175b, l.f175c, l.f175d, l.f175e, ) = (t24b, 0.0, 0.0, 0.0, 0.0, );let t24c: f64 = (l.f4d5 * l.f4d5);let t24d: f64 = (p.p856 - 1.0);let t24e: f64 = (l.f6d).powf(t24d);let t24f: f64 = (t24c * t24e);let t250: f64 = (-t24f);let t251: f64 = (t250 * p.p856);let t252: f64 = (t251 * l.f1750);l.f1325 = t252;let t253: f64 = (l.f4d9 * l.f4d9);let t254: f64 = (p.p857 - 1.0);let t255: f64 = (l.f6d).powf(t254);let t256: f64 = (t253 * t255);let t257: f64 = (-t256);let t258: f64 = (t257 * p.p857);let t259: f64 = (t258 * l.f1760);l.f1331 = t259;let t25a: f64 = (l.f4d7 * l.f4d7);let t25b: f64 = (p.p858 - 1.0);let t25c: f64 = (l.f6d).powf(t25b);let t25d: f64 = (t25a * t25c);let t25e: f64 = (-t25d);let t25f: f64 = (t25e * p.p858);let t260: f64 = (t25f * l.f1754);(l.f1327, l.f132d, l.f132e, l.f132f, l.f1330, ) = (t260, (t25f * l.f175b), (t25f * l.f175c), (t25f * l.f175d), (t25f * l.f175e), );let t263: f64 = if ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0)) { 1.0 } else { 0.0 };l.f92d = t263;
        if (l.f92d != 0.0) {l.f150c = 1.0;}
        if (l.f92d == 0.0) {l.f150c = 0.0;}
        let t26e: f64 = if l.f150c == 1.0 { 1.0 } else { 0.0 };l.f9a7 = t26e;
        if (l.f9a7 != 0.0) {
            let t26f: f64 = (p.p820 * p.p859);
            let (t271,) = {
    if (t26f > 1e-18) {
        let t270: f64 = (p.p820 * p.p859);
        (t270,)
    } else {
        (1e-18,)
    }
};
            l.f1ab = t271;
        }
        if (l.f9a7 != 0.0) {
            let t272: f64 = (p.p823 * p.p860);
            let (t274,) = {
    if (t272 > 0.05) {
        let t273: f64 = (p.p823 * p.p860);
        (t273,)
    } else {
        (0.05,)
    }
};
            l.f1728 = t274;
        }
        if (l.f9a7 != 0.0) {
            let t275: f64 = (p.p826 * p.p861);
            let (t277,) = {
    if (t275 > 0.05) {
        let t276: f64 = (p.p826 * p.p861);
        (t276,)
    } else {
        (0.05,)
    }
};
            let (t27b,) = {
    if (t277 < 0.95) {
        let t278: f64 = (p.p826 * p.p861);
        let (t27a,) = {
            if (t278 > 0.05) {
                let t279: f64 = (p.p826 * p.p861);
                (t279,)
            } else {
                (0.05,)
            }
        };
        (t27a,)
    } else {
        (0.95,)
    }
};
            l.ff62 = t27b;
        }
        if (l.f9a7 != 0.0) {let t27c: f64 = (p.p829 * p.p862);l.ff86 = t27c;let t299: f64 = (l.ff86 + l.f28d);l.ff91 = t299;let t2d1: f64 = (1.0 - l.ff62);l.ff14 = t2d1;let t2eb: f64 = (1.0 / l.ff14);l.ff24 = t2eb;}
        let t2ff: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.fa21 = t2ff;
        if (l.fa21 != 0.0) {l.f1a9 = p.p818;l.f1b1 = p.p819;l.f1af = p.p820;l.f1722 = p.p821;l.f1732 = p.p822;l.f172c = p.p823;l.ff3d = p.p824;l.f102e = p.p825;l.ff66 = p.p826;l.ff74 = p.p827;l.ff9c = p.p828;l.ff8a = p.p829;l.fc93 = p.p830;l.fc97 = p.p831;l.fc95 = p.p832;l.f1cf = p.p833;l.f1d3 = p.p834;l.f1d1 = p.p835;l.f1aea = p.p836;l.f1ae8 = p.p837;l.f1e7 = p.p838;l.f1eb = p.p839;l.f1e9 = p.p840;l.fe70 = p.p841;l.fe74 = p.p842;l.fe72 = p.p843;l.f14a = p.p844;l.f14e = p.p845;l.f14c = p.p846;l.f44a = p.p847;l.f44e = p.p848;l.f44c = p.p849;l.f14e2 = p.p850;l.f14e6 = p.p851;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fa21 != 0.0) {l.f14e4 = p.p852;l.f1740 = p.p853;l.f1764 = p.p854;l.f174e = p.p855;l.ff3f = p.p856;l.ff43 = p.p857;l.ff41 = p.p858;l.f181d = p.p921;l.f496 = p.p922;l.f20 = p.p865;l.ffa = p.p866;l.f1e = p.p867;l.ff8 = p.p868;l.f472 = p.p859;l.f4eb = p.p860;l.f4ab = p.p861;l.f4ad = p.p862;l.f18bc = p.p863;l.f98 = p.p864;}
        if (l.fa21 == 0.0) {l.f1a9 = p.p869;l.f1b1 = p.p870;l.f1af = p.p871;l.f1722 = p.p872;l.f1732 = p.p873;l.f172c = p.p874;l.ff3d = p.p875;l.f102e = p.p876;l.ff66 = p.p877;l.ff74 = p.p878;l.ff9c = p.p879;l.ff8a = p.p880;l.fc93 = p.p881;l.fc97 = p.p882;l.fc95 = p.p883;l.f1cf = p.p884;l.f1d3 = p.p885;l.f1d1 = p.p886;l.f1aea = p.p887;l.f1ae8 = p.p888;l.f1e7 = p.p889;l.f1eb = p.p890;l.f1e9 = p.p891;l.fe70 = p.p892;l.fe74 = p.p893;l.fe72 = p.p894;l.f14a = p.p895;l.f14e = p.p896;l.f14c = p.p897;l.f44a = p.p898;l.f44e = p.p899;l.f44c = p.p900;l.f14e2 = p.p901;l.f14e6 = p.p902;l.f14e4 = p.p903;l.f1740 = p.p904;l.f1764 = p.p905;l.f174e = p.p906;l.ff3f = p.p907;l.ff43 = p.p908;l.ff41 = p.p909;l.f181d = p.p923;l.f496 = p.p924;l.f20 = p.p916;l.ffa = p.p917;l.f1e = p.p918;l.ff8 = p.p919;l.f472 = p.p910;l.f4eb = p.p911;l.f4ab = p.p912;l.f4ad = p.p913;l.f18bc = p.p914;l.f98 = p.p915;}
        let td: f64 = (l.ff74 + l.f28d);l.ff8d = td;let te: f64 = (l.ff9c + l.f28d);l.ff99 = te;let tf: f64 = (l.ff8a + l.f28d);l.ff95 = tf;let t10: f64 = (1.0 - l.ff3d);l.ff10 = t10;let t11: f64 = (1.0 - l.f102e);l.ff1c = t11;let t12: f64 = (1.0 - l.ff66);l.ff18 = t12;let t13: f64 = (1.0 / l.ff10);l.ff20 = t13;let t14: f64 = (1.0 / l.ff1c);l.ff2c = t14;let t16: f64 = (1.0 / l.ff18);l.ff28 = t16;let t17: f64 = (l.f3b7 / l.f1a9);l.f18d1 = t17;let t18: f64 = (l.f1aea * l.f3b7);let t19: f64 = (t18 / l.f1b1);l.f18e5 = t19;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t1a: f64 = (l.f1ae8 * l.f3b7);let t1b: f64 = (t1a / l.f1af);l.f18d5 = t1b;let t1c: f64 = (1.0 / l.f18d1);l.f18d9 = t1c;let t1d: f64 = (1.0 / l.f18e5);l.f18e1 = t1d;let t1e: f64 = (1.0 / l.f18d5);l.f18dd = t1e;let t1f: f64 = (1.0 / l.f1722);l.f1725 = t1f;let t20: f64 = (1.0 / l.f1732);l.f1735 = t20;let t21: f64 = (1.0 / l.f172c);l.f172f = t21;let t23: f64 = (l.f6d).powf(l.ff3f);let t24: f64 = (1.0 - t23);let t25: f64 = (1.0 / t24);l.f4d6 = t25;let t26: f64 = (l.f6d).powf(l.ff43);let t27: f64 = (1.0 - t26);let t28: f64 = (1.0 / t27);l.f4da = t28;let t29: f64 = (l.f6d).powf(l.ff41);let t2a: f64 = (1.0 - t29);let t2b: f64 = (1.0 / t2a);l.f4d8 = t2b;let t2c: f64 = (1.0 / l.f1740);l.f1751 = t2c;let t2d: f64 = (1.0 / l.f1764);l.f1761 = t2d;let t2e: f64 = (1.0 / l.f174e);(l.f1755, l.f1756, l.f1757, l.f1758, l.f1759, ) = (t2e, 0.0, 0.0, 0.0, 0.0, );let t2f: f64 = (l.f4d6 * l.f4d6);let t30: f64 = (l.ff3f - 1.0);let t31: f64 = (l.f6d).powf(t30);let t32: f64 = (t2f * t31);let t33: f64 = (-t32);let t34: f64 = (t33 * l.ff3f);let t35: f64 = (t34 * l.f1751);l.f1326 = t35;let t36: f64 = (l.f4da * l.f4da);let t37: f64 = (l.ff43 - 1.0);let t38: f64 = (l.f6d).powf(t37);let t39: f64 = (t36 * t38);let t3a: f64 = (-t39);let t3b: f64 = (t3a * l.ff43);let t3c: f64 = (t3b * l.f1761);l.f1332 = t3c;let t3d: f64 = (l.f4d8 * l.f4d8);let t3e: f64 = (l.ff41 - 1.0);let t3f: f64 = (l.f6d).powf(t3e);let t40: f64 = (t3d * t3f);let t41: f64 = (-t40);let t42: f64 = (t41 * l.ff41);let t43: f64 = (t42 * l.f1755);(l.f1328, l.f1329, l.f132a, l.f132b, l.f132c, ) = (t43, (t42 * l.f1756), (t42 * l.f1757), (t42 * l.f1758), (t42 * l.f1759), );let t44: f64 = if ((((l.f472 != 1.0) || (l.f4eb != 1.0)) || (l.f4ab != 1.0)) || (l.f4ad != 1.0)) { 1.0 } else { 0.0 };l.fa9d = t44;
        if (l.fa9d != 0.0) {l.f150d = 1.0;}
        if (l.fa9d == 0.0) {l.f150d = 0.0;}
        let t46: f64 = if l.f150d == 1.0 { 1.0 } else { 0.0 };l.fb17 = t46;
        if (l.fb17 != 0.0) {
            let t47: f64 = (l.f1af * l.f472);
            let (t49,) = {
    if (t47 > 1e-18) {
        let t48: f64 = (l.f1af * l.f472);
        (t48,)
    } else {
        (1e-18,)
    }
};
            l.f1ac = t49;
        }
        if (l.fb17 != 0.0) {
            let t4a: f64 = (l.f172c * l.f4eb);
            let (t4c,) = {
    if (t4a > 0.05) {
        let t4b: f64 = (l.f172c * l.f4eb);
        (t4b,)
    } else {
        (0.05,)
    }
};
            l.f1729 = t4c;
        }
        if (l.fb17 != 0.0) {
            let t4d: f64 = (l.ff66 * l.f4ab);
            let (t4f,) = {
    if (t4d > 0.05) {
        let t4e: f64 = (l.ff66 * l.f4ab);
        (t4e,)
    } else {
        (0.05,)
    }
};
            let (t53,) = {
    if (t4f < 0.95) {
        let t50: f64 = (l.ff66 * l.f4ab);
        let (t52,) = {
            if (t50 > 0.05) {
                let t51: f64 = (l.ff66 * l.f4ab);
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
            l.ff63 = t53;
        }
        if (l.fb17 != 0.0) {let t54: f64 = (l.ff8a * l.f4ad);l.ff87 = t54;let t55: f64 = (l.ff87 + l.f28d);l.ff92 = t55;let t56: f64 = (1.0 - l.ff63);l.ff15 = t56;let t57: f64 = (1.0 / l.ff15);l.ff25 = t57;}
        let t59: f64 = 0.0;l.f570 = t59;let t5a: f64 = ctx_temp;let t5b: f64 = (t5a + p.p55);let t5c: f64 = (t5b + p.p35);l.f15c5 = t5c;let t5d: f64 = (l.f15c5 / l.f15cd);l.f12d7 = t5d;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t5e: f64 = (l.f15c5 - l.f15cd);l.f241 = t5e;let t5f: f64 = (l.f15c5 * 1.3806505e-23);let t60: f64 = (t5f / 1.6021918e-19);l.ffc0 = t60;let t61: f64 = (1.0 / l.ffc0);l.fd73 = t61;l.f15c7 = l.f15c5;let t64: f64 = (l.f15c7 * l.f15c7);l.f15cb = t64;let t65: f64 = (l.f15c7 - l.f15cd);l.f23f = t65;let t66: f64 = (l.f15cd / l.f15c7);l.f12d9 = t66;let t67: f64 = (l.f12d9).ln();l.fe2b = t67;let t68: f64 = (l.f15c7 * 1.3806505e-23);let t69: f64 = (t68 / 1.6021918e-19);l.ff9e = t69;let t6a: f64 = (1.0 / l.ff9e);l.fd59 = t6a;let t6b: f64 = (9.025e-5 * l.f15c7);let t6c: f64 = (1.179 - t6b);let t6d: f64 = (3.05e-7 * l.f15cb);let t6e: f64 = (t6c - t6d);l.f3a3 = t6e;let t6f: f64 = (0.00045 * l.f15c7);let t70: f64 = (1.045 + t6f);let t71: f64 = (0.0014 * l.f15c7);let t72: f64 = (0.523 + t71);let t73: f64 = (1.48e-6 * l.f15cb);let t74: f64 = (t72 - t73);let t75: f64 = (t70 * t74);let t76: f64 = (t75 * l.f15cb);let t77: f64 = (t76 / 90000.0);l.ff72 = t77;
        if (!(l.ff72 > 0.001)) {l.ff72 = 0.001;}
        let t78: f64 = (4.0 * 1.3806505e-23);let t79: f64 = (t78 * l.f15c7);l.ff01 = t79;let t7b: f64 = ctx_temp;let t7c: f64 = (t7b + p.p55);let t7d: f64 = (t7c + p.p35);let t7e: f64 = (-250.0);let t7f: f64 = (273.15 + t7e);let t80: f64 = (t7d).max(t7f);l.f15c8 = t80;let t81: f64 = (l.f15c8 / l.f15ce);l.fe0 = t81;let t82: f64 = (l.fddc * l.f15c8);l.ffce = t82;let t83: f64 = (1.0 / l.ffce);l.ffd0 = t83;let t84: f64 = (0.000702 * l.f15c8);let t85: f64 = (t84 * l.f15c8);let t86: f64 = (-t85);let t87: f64 = (1108.0 + l.f15c8);let t88: f64 = (t86 / t87);l.f28b = t88;let t89: f64 = (p.p827 + l.f28b);l.ff76 = t89;let t8a: f64 = (p.p828 + l.f28b);l.ff82 = t8a;let t8b: f64 = (p.p829 + l.f28b);l.ff7a = t8b;let t8c: f64 = (l.fe0).powf(1.5);let t8d: f64 = (l.ff8c * l.ffd4);let t8e: f64 = (l.ff76 * l.ffd0);let t8f: f64 = (t8d - t8e);let t90: f64 = (0.5 * t8f);let t91: f64 = (t90).exp();let t92: f64 = (t8c * t91);l.f4db = t92;let t93: f64 = (l.fe0).powf(1.5);let t94: f64 = (l.ff98 * l.ffd4);let t95: f64 = (l.ff82 * l.ffd0);let t96: f64 = (t94 - t95);let t97: f64 = (0.5 * t96);let t98: f64 = (t97).exp();let t99: f64 = (t93 * t98);l.f4e7 = t99;let t9b: f64 = (l.fe0).powf(1.5);let t9c: f64 = (l.ff90 * l.ffd4);let t9d: f64 = (l.ff7a * l.ffd0);let t9e: f64 = (t9c - t9d);let t9f: f64 = (0.5 * t9e);let ta0: f64 = (t9f).exp();let ta1: f64 = (t9b * ta0);l.f4df = ta1;let ta2: f64 = (p.p830 * l.f4db);let ta3: f64 = (ta2 * l.f4db);l.fc8b = ta3;let ta4: f64 = (p.p831 * l.f4e7);let ta5: f64 = (ta4 * l.f4e7);l.fc99 = ta5;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ta6: f64 = (p.p832 * l.f4df);let ta7: f64 = (ta6 * l.f4df);l.fc8f = ta7;let ta8: f64 = (p.p821 * l.fe0);let ta9: f64 = (2.0 * l.ffce);let taa: f64 = (l.f4db).ln();let tab: f64 = (ta9 * taa);let tac: f64 = (ta8 - tab);l.f1649 = tac;let tad: f64 = (p.p822 * l.fe0);let tae: f64 = (2.0 * l.ffce);let taf: f64 = (l.f4e7).ln();let tb0: f64 = (tae * taf);let tb1: f64 = (tad - tb0);l.f1655 = tb1;let tb2: f64 = (p.p823 * l.fe0);let tb3: f64 = (2.0 * l.ffce);let tb4: f64 = (l.f4df).ln();let tb5: f64 = (tb3 * tb4);let tb6: f64 = (tb2 - tb5);l.f164d = tb6;let tb7: f64 = (0.05 - l.f1649);let tb8: f64 = (tb7 * l.ffd0);let tb9: f64 = (tb8).exp();let tba: f64 = (1.0 + tb9);let tbb: f64 = (tba).ln();let tbc: f64 = (l.ffce * tbb);let tbd: f64 = (l.f1649 + tbc);l.f16fa = tbd;let tbe: f64 = (0.05 - l.f1655);let tbf: f64 = (tbe * l.ffd0);let tc0: f64 = (tbf).exp();let tc1: f64 = (1.0 + tc0);let tc2: f64 = (tc1).ln();let tc3: f64 = (l.ffce * tc2);let tc4: f64 = (l.f1655 + tc3);l.f1738 = tc4;let tc5: f64 = (0.05 - l.f164d);let tc6: f64 = (tc5 * l.ffd0);let tc7: f64 = (tc6).exp();let tc8: f64 = (1.0 + tc7);let tc9: f64 = (tc8).ln();let tca: f64 = (l.ffce * tc9);let tcb: f64 = (l.f164d + tca);l.f1702 = tcb;let tce: f64 = (1.0 / l.f16fa);l.f170e = tce;let tcf: f64 = (1.0 / l.f1738);l.f171a = tcf;let td0: f64 = (1.0 / l.f1702);l.f1712 = td0;let td1: f64 = (p.p821 * l.f170e);let td2: f64 = (td1).powf(p.p824);let td3: f64 = (p.p818 * td2);l.f19d = td3;let td4: f64 = (p.p822 * l.f171a);let td5: f64 = (td4).powf(p.p825);let td6: f64 = (p.p819 * td5);l.f1b3 = td6;let td7: f64 = (p.p823 * l.f1712);let td8: f64 = (td7).powf(p.p826);let td9: f64 = (p.p820 * td8);l.f1a1 = td9;let tda: f64 = (l.f19d * l.f16fa);let tdb: f64 = (tda * l.ff1f);l.f126a = tdb;let tdc: f64 = (l.f1b3 * l.f1738);let tdd: f64 = (tdc * l.ff2b);l.f1276 = tdd;let tde: f64 = (l.f1a1 * l.f1702);let tdf: f64 = (tde * l.ff23);l.f126e = tdf;let te0: f64 = (2.0 * l.f19d);l.f125a = te0;let te3: f64 = (2.0 * l.f1b3);l.f1266 = te3;let te4: f64 = (2.0 * l.f1a1);l.f125e = te4;let te5: f64 = (0.5 * l.ff76);let te6: f64 = (te5).max(l.ffce);l.f27f = te6;let te7: f64 = (0.5 * l.ff82);let te8: f64 = (te7).max(l.ffce);l.f287 = te8;let te9: f64 = (0.5 * l.ff7a);let tea: f64 = (te9).max(l.ffce);l.f283 = tea;let teb: f64 = (l.f27f * l.ffd0);l.fd4 = teb;let tec: f64 = (l.f287 * l.ffd0);
        l.fdc = tec;let ted: f64 = (l.f283 * l.ffd0);l.fd8 = ted;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tee: f64 = (32.0 * p.p841);let tef: f64 = (tee * 9.1093826e-31);let tf0: f64 = (tef * 1.6021918e-19);let tf1: f64 = (l.f27f * l.f27f);let tf2: f64 = (tf1 * l.f27f);let tf3: f64 = (tf0 * tf2);let tf4: f64 = (tf3).sqrt();let tf5: f64 = (3.0 * 1.05457168e-34);let tf6: f64 = (tf4 / tf5);l.f139 = tf6;let tf7: f64 = (32.0 * p.p842);let tf8: f64 = (tf7 * 9.1093826e-31);let tf9: f64 = (tf8 * 1.6021918e-19);let tfa: f64 = (l.f287 * l.f287);let tfb: f64 = (tfa * l.f287);let tfc: f64 = (tf9 * tfb);let tfd: f64 = (tfc).sqrt();let tfe: f64 = (3.0 * 1.05457168e-34);let tff: f64 = (tfd / tfe);l.f141 = tff;let t101: f64 = (32.0 * p.p843);let t102: f64 = (t101 * 9.1093826e-31);let t103: f64 = (t102 * 1.6021918e-19);let t104: f64 = (l.f283 * l.f283);let t105: f64 = (t104 * l.f283);let t106: f64 = (t103 * t105);let t107: f64 = (t106).sqrt();let t108: f64 = (3.0 * 1.05457168e-34);let t109: f64 = (t107 / t108);l.f13d = t109;let t10a: f64 = (l.f15c8 - l.f15ce);let t10b: f64 = (p.p850 * t10a);let t10c: f64 = (1.0 + t10b);let t10d: f64 = (p.p847 * t10c);l.f43a = t10d;let t10e: f64 = (l.f15c8 - l.f15ce);let t10f: f64 = (p.p851 * t10e);let t110: f64 = (1.0 + t10f);let t111: f64 = (p.p848 * t110);l.f450 = t111;let t112: f64 = (l.f15c8 - l.f15ce);let t113: f64 = (p.p852 * t112);let t114: f64 = (1.0 + t113);let t115: f64 = (p.p849 * t114);(l.f43e, l.f445, l.f446, l.f447, l.f448, ) = (t115, 0.0, 0.0, 0.0, 0.0, );
        if (!(l.f43a > 0.0)) {l.f43a = 0.0;}
        if (!(l.f450 > 0.0)) {l.f450 = 0.0;}
        if (!(l.f43e > 0.0)) {(l.f43e, l.f445, l.f446, l.f447, l.f448, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t116: f64 = if l.f150c == 1.0 { 1.0 } else { 0.0 };l.f908 = t116;
        if (l.f908 != 0.0) {let t117: f64 = (l.ff86 + l.f28b);l.ff7b = t117;let t118: f64 = (l.fe0).powf(1.5);let t119: f64 = (l.ff91 * l.ffd4);let t11a: f64 = (l.ff7b * l.ffd0);let t11b: f64 = (t119 - t11a);let t11c: f64 = (0.5 * t11b);let t11d: f64 = (t11c).exp();let t11e: f64 = (t118 * t11d);l.f4e0 = t11e;let t120: f64 = (l.f1728 * l.fe0);let t121: f64 = (2.0 * l.ffce);let t122: f64 = (l.f4e0).ln();let t123: f64 = (t121 * t122);let t124: f64 = (t120 - t123);l.f164e = t124;let t125: f64 = (0.05 - l.f164e);let t126: f64 = (t125 * l.ffd0);let t127: f64 = (t126).exp();let t128: f64 = (1.0 + t127);let t129: f64 = (t128).ln();let t12a: f64 = (l.ffce * t129);let t12b: f64 = (l.f164e + t12a);l.f1705 = t12b;let t12c: f64 = (1.0 / l.f1705);l.f1713 = t12c;let t12d: f64 = (l.f1728 * l.f1713);let t12e: f64 = (t12d).powf(l.ff62);let t12f: f64 = (l.f1ab * t12e);l.f1a2 = t12f;let t130: f64 = (l.f1a2 * l.f1705);let t131: f64 = (t130 * l.ff24);l.f126f = t131;let t132: f64 = (2.0 * l.f1a2);l.f125f = t132;}
        let t133: f64 = (l.ff74 + l.f28b);l.ff77 = t133;let t134: f64 = (l.ff9c + l.f28b);l.ff83 = t134;let t135: f64 = (l.ff8a + l.f28b);l.ff7f = t135;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        l: &mut StampLocals,
    ) {
        let t136: f64 = (l.fe0).powf(1.5);let t137: f64 = (l.ff8d * l.ffd4);let t138: f64 = (l.ff77 * l.ffd0);let t139: f64 = (t137 - t138);let t13a: f64 = (0.5 * t139);let t13b: f64 = (t13a).exp();let t13c: f64 = (t136 * t13b);l.f4dc = t13c;let t13e: f64 = (l.fe0).powf(1.5);let t13f: f64 = (l.ff99 * l.ffd4);let t140: f64 = (l.ff83 * l.ffd0);let t141: f64 = (t13f - t140);let t142: f64 = (0.5 * t141);let t143: f64 = (t142).exp();let t144: f64 = (t13e * t143);l.f4e8 = t144;let t145: f64 = (l.fe0).powf(1.5);let t146: f64 = (l.ff95 * l.ffd4);let t147: f64 = (l.ff7f * l.ffd0);let t148: f64 = (t146 - t147);let t149: f64 = (0.5 * t148);let t14a: f64 = (t149).exp();let t14b: f64 = (t145 * t14a);l.f4e4 = t14b;let t14c: f64 = (l.fc93 * l.f4dc);let t14d: f64 = (t14c * l.f4dc);l.fc8c = t14d;let t14e: f64 = (l.fc97 * l.f4e8);let t14f: f64 = (t14e * l.f4e8);l.fc9a = t14f;let t150: f64 = (l.fc95 * l.f4e4);let t151: f64 = (t150 * l.f4e4);l.fc90 = t151;let t152: f64 = (l.f1722 * l.fe0);let t153: f64 = (2.0 * l.ffce);let t154: f64 = (l.f4dc).ln();let t155: f64 = (t153 * t154);let t156: f64 = (t152 - t155);l.f164a = t156;let t157: f64 = (l.f1732 * l.fe0);let t158: f64 = (2.0 * l.ffce);let t159: f64 = (l.f4e8).ln();let t15a: f64 = (t158 * t159);let t15b: f64 = (t157 - t15a);l.f1656 = t15b;let t15c: f64 = (l.f172c * l.fe0);let t15d: f64 = (2.0 * l.ffce);let t15e: f64 = (l.f4e4).ln();let t15f: f64 = (t15d * t15e);let t160: f64 = (t15c - t15f);l.f1652 = t160;let t161: f64 = (0.05 - l.f164a);let t162: f64 = (t161 * l.ffd0);let t163: f64 = (t162).exp();let t164: f64 = (1.0 + t163);let t165: f64 = (t164).ln();let t166: f64 = (l.ffce * t165);let t167: f64 = (l.f164a + t166);l.f16ff = t167;let t168: f64 = (0.05 - l.f1656);let t169: f64 = (t168 * l.ffd0);let t16a: f64 = (t169).exp();let t16b: f64 = (1.0 + t16a);let t16c: f64 = (t16b).ln();let t16d: f64 = (l.ffce * t16c);let t16e: f64 = (l.f1656 + t16d);l.f173d = t16e;let t170: f64 = (0.05 - l.f1652);let t171: f64 = (t170 * l.ffd0);let t172: f64 = (t171).exp();let t173: f64 = (1.0 + t172);let t174: f64 = (t173).ln();let t175: f64 = (l.ffce * t174);let t176: f64 = (l.f1652 + t175);l.f170b = t176;let t177: f64 = (1.0 / l.f16ff);l.f170f = t177;let t178: f64 = (1.0 / l.f173d);l.f171b = t178;let t179: f64 = (1.0 / l.f170b);l.f1717 = t179;let t17a: f64 = (l.f1722 * l.f170f);let t17b: f64 = (t17a).powf(l.ff3d);let t17c: f64 = (l.f1a9 * t17b);l.f19e = t17c;
        let t17d: f64 = (l.f1732 * l.f171b);let t17e: f64 = (t17d).powf(l.f102e);let t17f: f64 = (l.f1b1 * t17e);l.f1b4 = t17f;let t180: f64 = (l.f172c * l.f1717);let t181: f64 = (t180).powf(l.ff66);let t182: f64 = (l.f1af * t181);l.f1a6 = t182;let t183: f64 = (l.f19e * l.f16ff);let t184: f64 = (t183 * l.ff20);l.f126b = t184;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        l: &mut StampLocals,
    ) {
        let t185: f64 = (l.f1b4 * l.f173d);let t186: f64 = (t185 * l.ff2c);l.f1277 = t186;let t187: f64 = (l.f1a6 * l.f170b);let t188: f64 = (t187 * l.ff28);l.f1273 = t188;let t18a: f64 = (2.0 * l.f19e);l.f125b = t18a;let t18b: f64 = (2.0 * l.f1b4);l.f1267 = t18b;let t18c: f64 = (2.0 * l.f1a6);l.f1263 = t18c;let t18d: f64 = (0.5 * l.ff77);let t18e: f64 = (t18d).max(l.ffce);l.f280 = t18e;let t18f: f64 = (0.5 * l.ff83);let t190: f64 = (t18f).max(l.ffce);l.f288 = t190;let t191: f64 = (0.5 * l.ff7f);let t192: f64 = (t191).max(l.ffce);l.f284 = t192;let t193: f64 = (l.f280 * l.ffd0);l.fd5 = t193;let t194: f64 = (l.f288 * l.ffd0);l.fdd = t194;let t195: f64 = (l.f284 * l.ffd0);l.fd9 = t195;let t197: f64 = (32.0 * l.fe70);let t198: f64 = (t197 * 9.1093826e-31);let t199: f64 = (t198 * 1.6021918e-19);let t19a: f64 = (l.f280 * l.f280);let t19b: f64 = (t19a * l.f280);let t19c: f64 = (t199 * t19b);let t19d: f64 = (t19c).sqrt();let t19e: f64 = (3.0 * 1.05457168e-34);let t19f: f64 = (t19d / t19e);l.f13a = t19f;let t1a1: f64 = (32.0 * l.fe74);let t1a2: f64 = (t1a1 * 9.1093826e-31);let t1a3: f64 = (t1a2 * 1.6021918e-19);let t1a4: f64 = (l.f288 * l.f288);let t1a5: f64 = (t1a4 * l.f288);let t1a6: f64 = (t1a3 * t1a5);let t1a7: f64 = (t1a6).sqrt();let t1a8: f64 = (3.0 * 1.05457168e-34);let t1a9: f64 = (t1a7 / t1a8);l.f142 = t1a9;let t1aa: f64 = (32.0 * l.fe72);let t1ab: f64 = (t1aa * 9.1093826e-31);let t1ac: f64 = (t1ab * 1.6021918e-19);let t1ad: f64 = (l.f284 * l.f284);let t1ae: f64 = (t1ad * l.f284);let t1af: f64 = (t1ac * t1ae);let t1b0: f64 = (t1af).sqrt();let t1b1: f64 = (3.0 * 1.05457168e-34);let t1b2: f64 = (t1b0 / t1b1);l.f13e = t1b2;let t1b3: f64 = (l.f15c8 - l.f15ce);let t1b4: f64 = (l.f14e2 * t1b3);let t1b5: f64 = (1.0 + t1b4);let t1b6: f64 = (l.f44a * t1b5);l.f43b = t1b6;let t1b7: f64 = (l.f15c8 - l.f15ce);let t1b8: f64 = (l.f14e6 * t1b7);let t1b9: f64 = (1.0 + t1b8);let t1ba: f64 = (l.f44e * t1b9);l.f451 = t1ba;let t1bb: f64 = (l.f15c8 - l.f15ce);let t1bc: f64 = (l.f14e4 * t1bb);let t1bd: f64 = (1.0 + t1bc);let t1be: f64 = (l.f44c * t1bd);(l.f43f, l.f440, l.f441, l.f442, l.f443, ) = (t1be, 0.0, 0.0, 0.0, 0.0, );
        if (!(l.f43b > 0.0)) {l.f43b = 0.0;}
        if (!(l.f451 > 0.0)) {l.f451 = 0.0;}
        if (!(l.f43f > 0.0)) {(l.f43f, l.f440, l.f441, l.f442, l.f443, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t1bf: f64 = if l.f150d == 1.0 { 1.0 } else { 0.0 };l.f914 = t1bf;
        if (l.f914 != 0.0) {let t1c0: f64 = (l.ff87 + l.f28b);l.ff7c = t1c0;let t1c2: f64 = (l.fe0).powf(1.5);let t1c3: f64 = (l.ff92 * l.ffd4);let t1c4: f64 = (l.ff7c * l.ffd0);let t1c5: f64 = (t1c3 - t1c4);let t1c6: f64 = (0.5 * t1c5);let t1c7: f64 = (t1c6).exp();let t1c8: f64 = (t1c2 * t1c7);l.f4e1 = t1c8;let t1c9: f64 = (l.f1729 * l.fe0);let t1ca: f64 = (2.0 * l.ffce);let t1cb: f64 = (l.f4e1).ln();let t1cc: f64 = (t1ca * t1cb);let t1cd: f64 = (t1c9 - t1cc);l.f164f = t1cd;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f914 != 0.0) {let t1ce: f64 = (0.05 - l.f164f);let t1cf: f64 = (t1ce * l.ffd0);let t1d0: f64 = (t1cf).exp();let t1d1: f64 = (1.0 + t1d0);let t1d2: f64 = (t1d1).ln();let t1d3: f64 = (l.ffce * t1d2);let t1d4: f64 = (l.f164f + t1d3);l.f1706 = t1d4;let t1d5: f64 = (1.0 / l.f1706);l.f1714 = t1d5;let t1d6: f64 = (l.f1729 * l.f1714);let t1d7: f64 = (t1d6).powf(l.ff63);let t1d8: f64 = (l.f1ac * t1d7);l.f1a3 = t1d8;let t1d9: f64 = (l.f1a3 * l.f1706);let t1da: f64 = (t1d9 * l.ff25);l.f1270 = t1da;let t1db: f64 = (2.0 * l.f1a3);l.f1260 = t1db;}
        l.fedd = 1.0;l.fd8e = 1.0;l.fe1f = 0.0;l.f18e8 = 0.0;l.fe10 = p.p0;l.f18bf = p.p1;l.f130b = p.p2;l.f130d = p.p3;l.f1317 = p.p4;l.f130f = p.p8;l.f1a64 = p.p11;l.f1a = p.p19;l.fe3b = p.p20;l.fe29 = p.p21;l.f16 = p.p22;l.fe37 = p.p23;l.fe25 = p.p24;l.fba = p.p25;l.f100b = p.p26;l.f1c = p.p27;l.ff5c = p.p28;l.fdca = p.p14;let t1e5: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };l.f920 = t1e5;
        if (l.f920 != 0.0) {
            let (t1e8,) = {
    if (p.p9 > 1.0) {
        (p.p9,)
    } else {
        (1.0,)
    }
};
            l.fedd = t1e8;
        }
        if (l.f920 != 0.0) {let t1e9: f64 = (l.fedd + 0.5);let t1ea: f64 = (t1e9).floor();l.fedd = t1ea;let t1eb: f64 = (1.0 / l.fedd);l.fd8e = t1eb;}
        let t1ec: f64 = (l.f18bf * l.fd8e);
        let (t1ee,) = {
    if (t1ec > 1e-9) {
        let t1ed: f64 = (l.f18bf * l.fd8e);
        (t1ed,)
    } else {
        (1e-9,)
    }
};
        l.f18bf = t1ee;l.f1311 = p.p5;l.f1313 = p.p6;l.f1315 = p.p7;
        let (t1ef,) = {
    if (p.p10 < 1.5) {
        (1.0,)
    } else {
        (2.0,)
    }
};
        l.fedf = t1ef;let t1f0: f64 = (1e-6 / l.fe10);l.fd33 = t1f0;let t1f1: f64 = (1e-6 / l.f18bf);l.fdc6 = t1f1;let t1f5: f64 = (p.p187 * l.fd33);let t1f6: f64 = (1.0 + t1f5);let t1f7: f64 = (p.p186 * t1f6);let t1f8: f64 = (p.p188 * l.fdc6);let t1f9: f64 = (1.0 + t1f8);let t1fa: f64 = (t1f7 * t1f9);l.f231 = t1fa;let t1fb: f64 = (p.p191 * l.fd33);let t1fc: f64 = (1.0 + t1fb);let t1fd: f64 = (p.p190 * t1fc);let t1fe: f64 = (p.p192 * l.fdc6);let t1ff: f64 = (1.0 + t1fe);let t200: f64 = (t1fd * t1ff);l.f2a3 = t200;let t201: f64 = (l.fe10 + l.f231);let t202: f64 = (2.0 * p.p189);let t203: f64 = (t201 - t202);
        let (t207,) = {
    if (t203 > 1e-9) {
        let t204: f64 = (l.fe10 + l.f231);let t205: f64 = (2.0 * p.p189);let t206: f64 = (t204 - t205);
        (t206,)
    } else {
        (1e-9,)
    }
};
        l.fe1f = t207;let t208: f64 = (l.f18bf + l.f2a3);let t209: f64 = (2.0 * p.p193);let t20a: f64 = (t208 - t209);
        let (t20e,) = {
    if (t20a > 1e-9) {
        let t20b: f64 = (l.f18bf + l.f2a3);let t20c: f64 = (2.0 * p.p193);let t20d: f64 = (t20b - t20c);
        (t20d,)
    } else {
        (1e-9,)
    }
};
        l.f18e8 = t20e;let t20f: f64 = (1e-6 / l.fe1f);l.fd35 = t20f;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t210: f64 = (l.fd35 * l.fd35);l.fd36 = t210;let t211: f64 = (1e-6 / l.f18e8);l.fdc8 = t211;let t212: f64 = (1.0 / l.fdc8);l.fcd3 = t212;let t213: f64 = (l.fd35 * l.fdc8);l.fc70 = t213;let t214: f64 = (1.0 / l.fc70);l.fcc7 = t214;let t218: f64 = (l.fe10 + l.f231);let t219: f64 = (2.0 * p.p189);let t21a: f64 = (t218 - t219);let t21b: f64 = (t21a + p.p194);
        let (t220,) = {
    if (t21b > 1e-9) {
        let t21c: f64 = (l.fe10 + l.f231);let t21d: f64 = (2.0 * p.p189);let t21e: f64 = (t21c - t21d);let t21f: f64 = (t21e + p.p194);
        (t21f,)
    } else {
        (1e-9,)
    }
};
        l.fe21 = t220;let t221: f64 = (l.f18bf + l.f2a3);let t222: f64 = (2.0 * p.p193);let t223: f64 = (t221 - t222);let t224: f64 = (t223 + p.p195);
        let (t229,) = {
    if (t224 > 1e-9) {
        let t225: f64 = (l.f18bf + l.f2a3);let t226: f64 = (2.0 * p.p193);let t227: f64 = (t225 - t226);let t228: f64 = (t227 + p.p195);
        (t228,)
    } else {
        (1e-9,)
    }
};
        l.f18ec = t229;let t22a: f64 = (l.f18ec / 1e-6);l.fcd5 = t22a;let t22b: f64 = (l.fe10 + l.f231);let t22c: f64 = (t22b + p.p194);
        let (t22f,) = {
    if (t22c > 1e-9) {
        let t22d: f64 = (l.fe10 + l.f231);let t22e: f64 = (t22d + p.p194);
        (t22e,)
    } else {
        (1e-9,)
    }
};
        l.fe1d = t22f;let t230: f64 = (l.f18bf + l.f2a3);let t231: f64 = (t230 + p.p195);
        let (t234,) = {
    if (t231 > 1e-9) {
        let t232: f64 = (l.f18bf + l.f2a3);let t233: f64 = (t232 + p.p195);
        (t233,)
    } else {
        (1e-9,)
    }
};
        l.f18c1 = t234;let t235: f64 = (l.fe1d / 1e-6);l.fcc9 = t235;let t236: f64 = (l.f18c1 / 1e-6);l.fcd1 = t236;let t237: f64 = (l.fe10 + l.f231);
        let (t239,) = {
    if (t237 > 1e-9) {
        let t238: f64 = (l.fe10 + l.f231);
        (t238,)
    } else {
        (1e-9,)
    }
};
        l.fe0f = t239;let t23a: f64 = (l.fe0f + p.p441);
        let (t23c,) = {
    if (t23a > 1e-9) {
        let t23b: f64 = (l.fe0f + p.p441);
        (t23b,)
    } else {
        (1e-9,)
    }
};
        l.fe12 = t23c;let t23d: f64 = (l.f18bf + l.f2a3);
        let (t23f,) = {
    if (t23d > 1e-9) {
        let t23e: f64 = (l.f18bf + l.f2a3);
        (t23e,)
    } else {
        (1e-9,)
    }
};
        l.f18be = t23f;let t243: f64 = (0.5 * l.f2a3);let t244: f64 = (l.f1a64 - t243);
        let (t247,) = {
    if (t244 > 1e-9) {
        let t245: f64 = (0.5 * l.f2a3);let t246: f64 = (l.f1a64 - t245);
        (t246,)
    } else {
        (1e-9,)
    }
};
        l.f1a65 = t247;l.f17ab = p.p56;l.f1502 = p.p57;l.f14c4 = p.p58;l.f1602 = p.p59;l.f3b5 = p.p60;l.fed5 = p.p61;l.f567 = p.p62;l.f1868 = p.p63;l.f369 = p.p64;l.f2dc = p.p65;l.feeb = p.p66;l.f1608 = p.p67;l.f160c = p.p68;l.fee2 = p.p69;l.fee6 = p.p70;l.f1e3 = p.p71;l.f1f7 = p.p73;l.f1ef = p.p72;l.f14e0 = p.p74;l.f1010 = p.p78;l.f101c = p.p80;l.f1014 = p.p79;l.f153 = p.p75;l.f15f = p.p77;l.f157 = p.p76;l.f103 = p.p81;l.f14cc = p.p82;l.febb = p.p83;l.f14ee = p.p84;l.f1564 = p.p85;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        l.f14fa = p.p86;l.f1cb = p.p87;l.f14dc = p.p88;l.f155e = p.p89;l.f14f6 = p.p90;l.f19bc = p.p91;l.f150a = p.p92;l.f476 = p.p93;l.f12c7 = p.p94;l.f14f2 = p.p95;l.f12cd = p.p96;l.f12d3 = p.p97;l.f1589 = p.p98;l.f14fe = p.p99;l.f159f = p.p100;l.f15bb = p.p101;l.f15c3 = p.p102;l.fe4 = p.p103;l.f41 = p.p104;l.f35 = p.p105;l.f3d = p.p106;l.f1864 = p.p107;l.f2 = p.p108;l.f6 = p.p109;l.f14c8 = p.p110;l.fc = p.p111;l.f10 = p.p112;l.fd3b = p.p113;l.f521 = p.p114;l.fcb8 = p.p115;l.fcbc = p.p116;l.fcc0 = p.p117;l.f14ea = p.p118;l.f509 = p.p119;l.f515 = p.p120;l.f50d = p.p119;let t261: f64 = if param_given[121] { 1.0 } else { 0.0 };let t262: f64 = if t261 == 1.0 { 1.0 } else { 0.0 };l.f92e = t262;
        if (l.f92e != 0.0) {l.f50d = p.p121;}
        l.f519 = p.p120;let t264: f64 = if param_given[122] { 1.0 } else { 0.0 };let t265: f64 = if t264 == 1.0 { 1.0 } else { 0.0 };l.f93a = t265;
        if (l.f93a != 0.0) {l.f519 = p.p122;}
        l.f511 = l.f50d;let t266: f64 = if param_given[123] { 1.0 } else { 0.0 };let t267: f64 = if t266 == 1.0 { 1.0 } else { 0.0 };l.f946 = t267;
        if (l.f946 != 0.0) {l.f511 = p.p123;}
        l.f51d = l.f519;let t268: f64 = if param_given[124] { 1.0 } else { 0.0 };let t269: f64 = if t268 == 1.0 { 1.0 } else { 0.0 };l.f952 = t269;
        if (l.f952 != 0.0) {l.f51d = p.p124;}
        l.f191 = p.p125;l.f29 = p.p126;l.f2d = p.p127;l.f114 = p.p128;l.f11a = p.p129;l.f14d4 = p.p130;l.f14d8 = p.p131;l.f17d = p.p132;l.f181 = p.p133;l.f1bb = p.p134;l.f29d = p.p135;l.f422 = p.p136;l.f158f = p.p98;let t26a: f64 = if param_given[137] { 1.0 } else { 0.0 };let t26b: f64 = if t26a == 1.0 { 1.0 } else { 0.0 };l.f95e = t26b;
        if (l.f95e != 0.0) {l.f158f = p.p137;}
        l.fe8 = p.p103;let t26c: f64 = if param_given[138] { 1.0 } else { 0.0 };let t26d: f64 = if t26c == 1.0 { 1.0 } else { 0.0 };l.f96a = t26d;
        if (l.f96a != 0.0) {l.fe8 = p.p138;}
        l.f45 = p.p139;l.f39 = p.p140;l.f185 = p.p141;l.f18d = p.p142;l.f464 = p.p143;l.f468 = p.p144;l.f189 = p.p145;l.f173 = p.p146;l.f197 = p.p147;l.f19b = p.p148;l.f353 = p.p149;l.f470 = p.p150;l.f46c = p.p151;l.ff0 = p.p152;l.f16b = p.p153;l.f16f = p.p154;l.f4a7 = p.p155;l.f4aa = p.p156;l.f17b1 = p.p161;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        l.f1506 = p.p162;l.f2e0 = p.p163;l.fedb = p.p164;l.f1f3 = p.p165;l.f109 = p.p166;l.f14d0 = p.p167;l.f1024 = p.p168;l.f1018 = p.p169;l.f1020 = p.p170;l.f167 = p.p171;l.f163 = p.p173;l.f15b = p.p172;l.f1298 = p.p179;l.f12d0 = p.p180;l.f1296 = p.p181;l.f12dc = p.p183;l.f1294 = p.p182;l.f12c4 = p.p184;l.f12c2 = p.p185;let t27d: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };l.f976 = t27d;
        if (l.f976 != 0.0) {let t27e: f64 = (l.fd35).powf(p.p198);let t27f: f64 = (p.p197 * t27e);let t280: f64 = (p.p196 + t27f);let t281: f64 = (p.p199 * l.fdc8);let t282: f64 = (t280 + t281);let t283: f64 = (p.p200 * l.fc70);let t284: f64 = (t282 + t283);l.f17ab = t284;let t285: f64 = (p.p202 * l.fd35);let t286: f64 = (p.p201 + t285);let t287: f64 = (p.p203 * l.fdc8);let t288: f64 = (t286 + t287);let t289: f64 = (p.p204 * l.fc70);let t28a: f64 = (t288 + t289);l.f1502 = t28a;l.f14c4 = p.p205;l.f1602 = p.p206;l.f3b5 = p.p207;}
        if (l.f976 != 0.0) {
            let t28b: f64 = (p.p209 * l.fdc8);let t28c: f64 = (l.f18e8 / p.p210);let t28d: f64 = (1.0 + t28c);let t28e: f64 = (t28d).ln();let t28f: f64 = (t28b * t28e);let t290: f64 = (1.0 + t28f);
            let (t297,) = {
    if (t290 > 0.001) {
        let t291: f64 = (p.p209 * l.fdc8);let t292: f64 = (l.f18e8 / p.p210);let t293: f64 = (1.0 + t292);let t294: f64 = (t293).ln();let t295: f64 = (t291 * t294);let t296: f64 = (1.0 + t295);
        (t296,)
    } else {
        (0.001,)
    }
};
            let t298: f64 = (p.p208 * t297);l.fefd = t298;
        }
        if (l.f976 != 0.0) {
            let t29a: f64 = (p.p212 * l.fdc8);let t29b: f64 = (l.f18e8 / p.p213);let t29c: f64 = (1.0 + t29b);let t29d: f64 = (t29c).ln();let t29e: f64 = (t29a * t29d);let t29f: f64 = (1.0 + t29e);
            let (t2a6,) = {
    if (t29f > 0.001) {
        let t2a0: f64 = (p.p212 * l.fdc8);let t2a1: f64 = (l.f18e8 / p.p213);let t2a2: f64 = (1.0 + t2a1);let t2a3: f64 = (t2a2).ln();let t2a4: f64 = (t2a0 * t2a3);let t2a5: f64 = (1.0 + t2a4);
        (t2a5,)
    } else {
        (0.001,)
    }
};
            let t2a7: f64 = (p.p211 * t2a6);l.feee = t2a7;
        }
        if (l.f976 != 0.0) {
            let t2a8: f64 = (p.p215 * l.fdc8);let t2a9: f64 = (l.f18e8 / p.p213);let t2aa: f64 = (1.0 + t2a9);let t2ab: f64 = (t2aa).ln();let t2ac: f64 = (t2a8 * t2ab);let t2ad: f64 = (1.0 + t2ac);
            let (t2b4,) = {
    if (t2ad > 0.001) {
        let t2ae: f64 = (p.p215 * l.fdc8);let t2af: f64 = (l.f18e8 / p.p213);let t2b0: f64 = (1.0 + t2af);let t2b1: f64 = (t2b0).ln();let t2b2: f64 = (t2ae * t2b1);let t2b3: f64 = (1.0 + t2b2);
        (t2b3,)
    } else {
        (0.001,)
    }
};
            let t2b5: f64 = (p.p214 * t2b4);l.fe33 = t2b5;
        }
        let t2b6: f64 = (2.0 * l.fe33);let t2b7: f64 = if l.fe1f > t2b6 { 1.0 } else { 0.0 };l.f982 = t2b7;
        if ((l.f976 != 0.0) && (l.f982 != 0.0)) {l.f12 = 75000000000.0;let t2b8: f64 = (0.5 * l.feee);let t2b9: f64 = (l.fefd + t2b8);let t2ba: f64 = (t2b9).sqrt();let t2bb: f64 = (l.fefd).sqrt();let t2bc: f64 = (t2ba - t2bb);l.ff4 = t2bc;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f982 != 0.0)) {let t2bd: f64 = (l.fefd).sqrt();let t2be: f64 = (2.0 * l.fe33);let t2bf: f64 = (t2be / l.fe1f);let t2c0: f64 = (l.ff4 / l.f12);let t2c1: f64 = (t2c0).exp();let t2c2: f64 = (t2c1 - 1.0);let t2c3: f64 = (t2bf * t2c2);let t2c4: f64 = (1.0 + t2c3);let t2c5: f64 = (t2c4).ln();let t2c6: f64 = (l.f12 * t2c5);let t2c7: f64 = (t2bd + t2c6);l.fefc = t2c7;let t2c8: f64 = (l.fefc * l.fefc);l.fefc = t2c8;}
        let t2c9: f64 = if l.fe1f >= l.fe33 { 1.0 } else { 0.0 };l.f98e = t2c9;
        if (((l.f976 != 0.0) && (l.f982 == 0.0)) && (l.f98e != 0.0)) {let t2ca: f64 = (l.feee * l.fe33);let t2cb: f64 = (t2ca / l.fe1f);let t2cc: f64 = (l.fefd + t2cb);l.fefc = t2cc;}
        if (((l.f976 != 0.0) && (l.f982 == 0.0)) && (l.f98e == 0.0)) {let t2cd: f64 = (l.fe1f / l.fe33);let t2ce: f64 = (2.0 - t2cd);let t2cf: f64 = (l.feee * t2ce);let t2d0: f64 = (l.fefd + t2cf);l.fefc = t2d0;}
        if (l.f976 != 0.0) {let t2d2: f64 = (p.p216 * l.fd35);let t2d3: f64 = (1.0 - t2d2);let t2d4: f64 = (p.p217 * l.fd36);let t2d5: f64 = (t2d3 - t2d4);let t2d6: f64 = (l.fefc * t2d5);l.fed5 = t2d6;let t2d7: f64 = (l.fd35).powf(p.p220);let t2d8: f64 = (p.p219 * t2d7);let t2d9: f64 = (p.p218 + t2d8);let t2da: f64 = (p.p221 * l.fdc8);let t2db: f64 = (t2d9 + t2da);let t2dc: f64 = (p.p222 * l.fc70);let t2dd: f64 = (t2db + t2dc);l.f567 = t2dd;l.f1868 = p.p223;l.f369 = p.p224;let t2de: f64 = (l.fd35).powf(p.p227);let t2df: f64 = (p.p226 * t2de);let t2e0: f64 = (p.p225 + t2df);let t2e1: f64 = (p.p228 * l.fdc8);let t2e2: f64 = (t2e0 + t2e1);let t2e3: f64 = (p.p229 * l.fc70);let t2e4: f64 = (t2e2 + t2e3);l.f2dc = t2e4;}
        if (l.f976 != 0.0) {
            let t2e5: f64 = (p.p231 * l.fd35);let t2e6: f64 = (1.0 + t2e5);
            let (t2e9,) = {
    if (1e-6 > t2e6) {
        (1e-6,)
    } else {
        let t2e7: f64 = (p.p231 * l.fd35);let t2e8: f64 = (1.0 + t2e7);
        (t2e8,)
    }
};
            let t2ea: f64 = (p.p230 * t2e9);l.feeb = t2ea;
        }
        if (l.f976 != 0.0) {l.f1608 = p.p232;l.f160c = p.p233;l.fee2 = p.p236;l.fee6 = p.p237;let t2ec: f64 = (l.fd35).powf(p.p240);let t2ed: f64 = (p.p239 * t2ec);let t2ee: f64 = (p.p238 + t2ed);let t2ef: f64 = (p.p241 * l.fdc8);let t2f0: f64 = (1.0 + t2ef);let t2f1: f64 = (t2ee * t2f0);let t2f2: f64 = (p.p242 * l.fc70);let t2f3: f64 = (1.0 + t2f2);let t2f4: f64 = (t2f1 * t2f3);l.f1e3 = t2f4;l.f1f7 = p.p244;l.f1ef = p.p243;l.f14e0 = p.p245;let t2f5: f64 = (l.fd35).powf(p.p247);let t2f6: f64 = (p.p246 * t2f5);let t2f7: f64 = (p.p248 * l.fdc8);let t2f8: f64 = (1.0 + t2f7);let t2f9: f64 = (t2f6 * t2f8);l.f153 = t2f9;l.f15f = p.p250;l.f157 = p.p249;let t2fa: f64 = (l.fd35).powf(p.p252);let t2fb: f64 = (p.p251 * t2fa);let t2fc: f64 = (p.p253 * l.fdc8);let t2fd: f64 = (1.0 + t2fc);let t2fe: f64 = (t2fb * t2fd);l.f1010 = t2fe;l.f101c = p.p255;l.f1014 = p.p254;let t300: f64 = (p.p258 * l.fdc8);let t301: f64 = (1.0 + t300);let t302: f64 = (p.p257 * t301);l.f454 = t302;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f976 != 0.0) {
            let t303: f64 = (p.p260 * l.fdc8);let t304: f64 = (1.0 + t303);
            let (t307,) = {
    if (t304 > 0.001) {
        let t305: f64 = (p.p260 * l.fdc8);let t306: f64 = (1.0 + t305);
        (t306,)
    } else {
        (0.001,)
    }
};
            let t308: f64 = (p.p259 * t307);l.fe31 = t308;
        }
        if (l.f976 != 0.0) {let t309: f64 = (l.f454 * l.fe31);let t30a: f64 = (t309 / l.fe1f);let t30b: f64 = (-l.fe1f);let t30c: f64 = (t30b / l.fe31);let t30d: f64 = (t30c).exp();let t30e: f64 = (1.0 - t30d);let t30f: f64 = (t30a * t30e);let t310: f64 = (1.0 + t30f);let t311: f64 = (p.p261 * p.p262);let t312: f64 = (t311 / l.fe1f);let t313: f64 = (-l.fe1f);let t314: f64 = (t313 / p.p262);let t315: f64 = (t314).exp();let t316: f64 = (1.0 - t315);let t317: f64 = (t312 * t316);let t318: f64 = (t310 + t317);l.f5c7 = t318;}
        if (l.f976 != 0.0) {
            let (t319,) = {
    if (l.f5c7 > 1e-15) {
        (l.f5c7,)
    } else {
        (1e-15,)
    }
};
            l.f5c7 = t319;
        }
        if (l.f976 != 0.0) {let t31a: f64 = (p.p263 * l.fdc8);let t31b: f64 = (1.0 + t31a);let t31c: f64 = (p.p264 * l.fdc8);let t31d: f64 = (l.f18e8 / p.p265);let t31e: f64 = (1.0 + t31d);let t31f: f64 = (t31e).ln();let t320: f64 = (t31c * t31f);let t321: f64 = (t31b + t320);l.fbe9 = t321;let t322: f64 = (p.p256 * l.f18e8);let t323: f64 = (l.f5c7 * l.fe1f);let t324: f64 = (t322 / t323);let t325: f64 = (t324 * l.fbe9);l.f103 = t325;let t326: f64 = (p.p267 * l.fd35);let t327: f64 = (p.p266 + t326);let t328: f64 = (p.p268 * l.fdc8);let t329: f64 = (t327 + t328);let t32a: f64 = (p.p269 * l.fc70);let t32b: f64 = (t329 + t32a);l.f14cc = t32b;let t32c: f64 = (p.p271 * l.fdc8);let t32d: f64 = (1.0 + t32c);let t32e: f64 = (p.p270 * t32d);l.febb = t32e;l.f14ee = p.p272;l.f1564 = p.p273;l.f14fa = p.p274;let t32f: f64 = (l.fd35).powf(p.p277);let t330: f64 = (p.p276 * t32f);let t331: f64 = (p.p275 + t330);let t332: f64 = (p.p278 * l.fdc8);let t333: f64 = (1.0 + t332);let t334: f64 = (t331 * t333);let t335: f64 = (p.p279 * l.fc70);let t336: f64 = (1.0 + t335);let t337: f64 = (t334 * t336);l.f1cb = t337;l.f14dc = p.p280;l.f155e = p.p281;l.f14f6 = p.p282;let t338: f64 = (p.p284 * l.fd35);let t339: f64 = (1.0 + t338);let t33a: f64 = (p.p283 * t339);let t33b: f64 = (p.p285 * l.fdc8);let t33c: f64 = (1.0 + t33b);let t33d: f64 = (t33a * t33c);let t33e: f64 = (p.p286 * l.fc70);let t33f: f64 = (1.0 + t33e);let t340: f64 = (t33d * t33f);l.f19bc = t340;l.f150a = p.p287;l.f476 = p.p288;let t341: f64 = (p.p289 * l.fdc8);let t342: f64 = (p.p290 * l.fdc8);let t343: f64 = (1.0 + t342);let t344: f64 = (t341 * t343);l.f12c7 = t344;l.f14f2 = p.p291;l.f12cd = p.p292;l.f12d3 = p.p293;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f976 != 0.0) {let t345: f64 = (p.p295 * l.fbe9);let t346: f64 = (t345 / l.f5c7);let t347: f64 = (l.fd35).powf(p.p296);let t348: f64 = (t346 * t347);let t349: f64 = (p.p294 + t348);let t34a: f64 = (p.p297 * l.fdc8);let t34b: f64 = (1.0 + t34a);let t34c: f64 = (t349 * t34b);let t34d: f64 = (p.p298 * l.fc70);let t34e: f64 = (1.0 + t34d);let t34f: f64 = (t34c * t34e);l.f1589 = t34f;let t350: f64 = (p.p300 * l.fd35);let t351: f64 = (p.p299 + t350);let t352: f64 = (p.p301 * l.fdc8);let t353: f64 = (t351 + t352);let t354: f64 = (p.p302 * l.fc70);let t355: f64 = (t353 + t354);l.f14fe = t355;l.f159f = p.p303;l.f15bb = p.p304;l.f15c3 = p.p305;let t356: f64 = (p.p307 * l.fd35);let t357: f64 = (1.0 + t356);let t358: f64 = (p.p306 / t357);l.fe4 = t358;let t359: f64 = (l.fd35).powf(p.p309);let t35a: f64 = (p.p308 * t359);let t35b: f64 = (p.p310 * l.fdc8);let t35c: f64 = (1.0 + t35b);let t35d: f64 = (t35a * t35c);l.f41 = t35d;let t35e: f64 = (l.fd35).powf(p.p312);l.f15fe = t35e;let t35f: f64 = (p.p311 * l.f15fe);let t360: f64 = (p.p314 * l.fdc8);let t361: f64 = (1.0 + t360);let t362: f64 = (t35f * t361);let t363: f64 = (p.p313 * l.fd35);let t364: f64 = (t363 * l.f15fe);let t365: f64 = (1.0 + t364);let t366: f64 = (t362 / t365);l.f35 = t366;let t367: f64 = (l.fd35).powf(p.p316);l.f15fe = t367;let t368: f64 = (p.p315 * l.f15fe);let t369: f64 = (p.p318 * l.fdc8);let t36a: f64 = (1.0 + t369);let t36b: f64 = (t368 * t36a);let t36c: f64 = (p.p317 * l.fd35);let t36d: f64 = (t36c * l.f15fe);let t36e: f64 = (1.0 + t36d);let t36f: f64 = (t36b / t36e);l.f3d = t36f;l.f1864 = p.p319;let t370: f64 = (p.p321 * l.fd35);let t371: f64 = (1.0 + t370);let t372: f64 = (p.p320 * t371);let t373: f64 = (p.p322 * l.fdc8);let t374: f64 = (1.0 + t373);let t375: f64 = (t372 * t374);l.f2 = t375;l.f6 = p.p323;l.f14c8 = p.p324;let t376: f64 = (p.p326 * l.fd35);let t377: f64 = (1.0 + t376);let t378: f64 = (p.p325 * t377);let t379: f64 = (p.p327 * l.fdc8);let t37a: f64 = (1.0 + t379);let t37b: f64 = (t378 * t37a);l.fc = t37b;let t37c: f64 = (p.p329 * l.fd35);let t37d: f64 = (1.0 + t37c);let t37e: f64 = (p.p328 * t37d);let t37f: f64 = (p.p330 * l.fdc8);let t380: f64 = (1.0 + t37f);let t381: f64 = (t37e * t380);l.f10 = t381;l.fd3b = p.p331;l.f521 = p.p332;let t382: f64 = (p.p333 / l.fc70);l.fcb8 = t382;let t383: f64 = (p.p334 * p.p234);let t384: f64 = (1e-6 * l.fdc8);let t385: f64 = (t383 / t384);l.fcbc = t385;}
        if (l.f976 != 0.0) {let t386: f64 = (p.p335 * p.p235);let t387: f64 = (1e-6 * l.fdc8);let t388: f64 = (t386 / t387);l.fcc0 = t388;l.f14ea = p.p336;l.f509 = p.p337;l.f515 = p.p338;l.f50d = p.p337;}
    }
}
