#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let nv4 = ctx.node_voltage(nodes[4]);let t0: f64 = 0.0;l.f1d87 = t0;let t38: f64 = (p.p5 + 273.15);l.f22f4 = t38;let t8f: f64 = ctx_temp;l.f22ed = t8f;(l.f22f6, l.f22f7, ) = ((nv4 - 0.0), 1.0, );let t372: f64 = (l.f22ed + p.p3);let t373: f64 = (t372 + l.f22f6);(l.f22ef, l.f22f0, ) = (t373, l.f22f7, );let t3ce: f64 = (-270.0);let t3cf: f64 = (t3ce + 273.15);let t3d0: f64 = if l.f22ef < t3cf { 1.0 } else { 0.0 };l.f1e5c = t3d0;
        if (l.f1e5c != 0.0) {let t42b: f64 = (-270.0);let t42c: f64 = (t42b + 273.15);(l.f22ef, l.f22f0, ) = (t42c, 0.0, );}
        let t460: f64 = (1500.0 + 273.15);let t461: f64 = if l.f22ef > t460 { 1.0 } else { 0.0 };l.f1f32 = t461;
        if ((l.f1e5c == 0.0) && (l.f1f32 != 0.0)) {let t462: f64 = (1500.0 + 273.15);(l.f22ef, l.f22f0, ) = (t462, 0.0, );}
        (l.f22eb, l.f22ec, ) = (0.0, 0.0, );(l.f22e6, l.f22e7, ) = (0.0, 0.0, );let t44: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };l.f1fae = t44;
        if (l.f1fae != 0.0) {let t4f: f64 = (p.p30 / p.p0);let t50: f64 = (t4f / p.p2);l.f22e5 = t50;let t5b: f64 = (p.p31 / p.p0);let t5c: f64 = (t5b / p.p2);l.f22e4 = t5c;}
        if (l.f1fae == 0.0) {let t67: f64 = (p.p30 / p.p0);let t68: f64 = (p.p29 * p.p54);let t69: f64 = (t68 / p.p0);let t6a: f64 = (t67 + t69);let t6b: f64 = (t6a / p.p2);l.f22e5 = t6b;let t76: f64 = (p.p31 / p.p0);let t77: f64 = (p.p29 * p.p66);let t78: f64 = (t77 / p.p0);let t79: f64 = (t76 + t78);let t7a: f64 = (t79 / p.p2);l.f22e4 = t7a;}
        let t81: f64 = if ((l.f22e5 >= p.p353) && (l.f22e5 > 0.0)) { 1.0 } else { 0.0 };l.f2036 = t81;
        if (l.f2036 != 0.0) {let t83: f64 = (l.f22ef - l.f22f4);let t84: f64 = (p.p48 * t83);let t85: f64 = (1.0 + t84);let t86: f64 = (l.f22ef - l.f22f4);let t87: f64 = (p.p49 * t86);let t88: f64 = (l.f22ef - l.f22f4);let t89: f64 = (t87 * t88);let t8a: f64 = (t85 + t89);let t8b: f64 = (l.f22e5 * t8a);(l.f22eb, l.f22ec, ) = (t8b, (l.f22e5 * ((p.p48 * l.f22f0) + (((p.p49 * l.f22f0) * t88) + (t87 * l.f22f0)))), );}
        let t8c: f64 = (0.1 * l.f22e5);let t8d: f64 = if l.f22eb < t8c { 1.0 } else { 0.0 };l.f206c = t8d;
        if ((l.f2036 != 0.0) && (l.f206c != 0.0)) {let t8e: f64 = (0.1 * l.f22e5);(l.f22eb, l.f22ec, ) = (t8e, 0.0, );}
        if (l.f2036 == 0.0) {(l.f22eb, l.f22ec, ) = (0.0, 0.0, );}
        let t90: f64 = if ((l.f22e4 >= p.p353) && (l.f22e4 > 0.0)) { 1.0 } else { 0.0 };l.f207f = t90;
        if (l.f207f != 0.0) {let t91: f64 = (l.f22ef - l.f22f4);let t92: f64 = (p.p48 * t91);let t93: f64 = (1.0 + t92);let t94: f64 = (l.f22ef - l.f22f4);let t95: f64 = (p.p49 * t94);let t96: f64 = (l.f22ef - l.f22f4);let t97: f64 = (t95 * t96);let t98: f64 = (t93 + t97);let t99: f64 = (l.f22e4 * t98);(l.f22e6, l.f22e7, ) = (t99, (l.f22e4 * ((p.p48 * l.f22f0) + (((p.p49 * l.f22f0) * t96) + (t95 * l.f22f0)))), );}
        let t9a: f64 = (0.1 * l.f22e4);let t9b: f64 = if l.f22e6 < t9a { 1.0 } else { 0.0 };l.f2094 = t9b;
        if ((l.f207f != 0.0) && (l.f2094 != 0.0)) {let t9c: f64 = (0.1 * l.f22e4);(l.f22e6, l.f22e7, ) = (t9c, 0.0, );}
        if (l.f207f == 0.0) {(l.f22e6, l.f22e7, ) = (0.0, 0.0, );}
        let t9d: f64 = (p.p324 / p.p2);let t9e: f64 = (t9d / p.p325);let t9f: f64 = (p.p327 * p.p0);let ta0: f64 = (t9f / p.p325);let ta1: f64 = (p.p326 + ta0);let ta2: f64 = (t9e * ta1);l.f22e8 = ta2;let ta3: f64 = (p.p324 / p.p2);let ta4: f64 = (ta3 / p.p325);let ta5: f64 = (1.0 - p.p327);let ta6: f64 = (ta5 * p.p0);let ta7: f64 = (ta6 / p.p325);let ta8: f64 = (ta4 * ta7);l.f22e9 = ta8;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tb8: f64 = (1.38062e-23 * l.f22ef);let tb9: f64 = (tb8 / 1.60219e-19);(l.f215b, l.f215c, ) = (tb9, ((1.38062e-23 * l.f22f0) / 1.60219e-19), );let td1: f64 = (l.f22ef - l.f22f4);let td2: f64 = (p.p336 * td1);let td3: f64 = (1.0 + td2);(l.f22f9, l.f22fa, ) = (td3, (p.p336 * l.f22f0), );let t11d: f64 = if l.f22f9 < 0.1 { 1.0 } else { 0.0 };l.f20a9 = t11d;
        if (l.f20a9 != 0.0) {(l.f22f9, l.f22fa, ) = (0.1, 0.0, );}
        let t1b6: f64 = (l.f22ef / l.f22f4);let t1b7: f64 = {let pb=t1b6;pb*pb*pb};(l.f22f2, l.f22f3, ) = (t1b7, ((l.f22f0 / l.f22f4) * (3.0 * {let pb=t1b6;pb*pb})), );let t1ce: f64 = (l.f22ef - l.f22f4);let t1cf: f64 = (p.p21 * t1ce);let t1d0: f64 = (1.0 + t1cf);
        let (t1d4, t1d5,) = {
    if (t1d0 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t1d1: f64 = (l.f22ef - l.f22f4);let t1d2: f64 = (p.p21 * t1d1);let t1d3: f64 = (1.0 + t1d2);
        (t1d3, (p.p21 * l.f22f0),)
    }
};
        let t1d6: f64 = (p.p9 * t1d4);(l.f71, l.f75, ) = (t1d6, (p.p9 * t1d5), );let t1ee: f64 = (l.f22ef - l.f22f4);let t1ef: f64 = (p.p22 * t1ee);let t1f0: f64 = (1.0 + t1ef);
        let (t1f4, t1f5,) = {
    if (t1f0 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t1f1: f64 = (l.f22ef - l.f22f4);let t1f2: f64 = (p.p22 * t1f1);let t1f3: f64 = (1.0 + t1f2);
        (t1f3, (p.p22 * l.f22f0),)
    }
};
        let t1f6: f64 = (p.p10 * t1f4);(l.f59, l.f5d, ) = (t1f6, (p.p10 * t1f5), );let t23f: f64 = (l.f22ef - l.f22f4);let t240: f64 = (p.p23 * t23f);let t241: f64 = (1.0 + t240);
        let (t245, t246,) = {
    if (t241 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t242: f64 = (l.f22ef - l.f22f4);let t243: f64 = (p.p23 * t242);let t244: f64 = (1.0 + t243);
        (t244, (p.p23 * l.f22f0),)
    }
};
        let t247: f64 = (p.p11 * t245);(l.f5f, l.f63, ) = (t247, (p.p11 * t246), );let t279: f64 = (l.f22ef - l.f22f4);let t27a: f64 = (p.p24 * t279);let t27b: f64 = (1.0 + t27a);
        let (t27f, t280,) = {
    if (t27b < 0.01) {
        (0.01, 0.0,)
    } else {
        let t27c: f64 = (l.f22ef - l.f22f4);let t27d: f64 = (p.p24 * t27c);let t27e: f64 = (1.0 + t27d);
        (t27e, (p.p24 * l.f22f0),)
    }
};
        let t281: f64 = (p.p13 * t27f);(l.f77, l.f7b, ) = (t281, (p.p13 * t280), );let t2f1: f64 = (l.f22ef - l.f22f4);let t2f2: f64 = (p.p25 * t2f1);let t2f3: f64 = (1.0 + t2f2);
        let (t2f7, t2f8,) = {
    if (t2f3 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t2f4: f64 = (l.f22ef - l.f22f4);let t2f5: f64 = (p.p25 * t2f4);let t2f6: f64 = (1.0 + t2f5);
        (t2f6, (p.p25 * l.f22f0),)
    }
};
        let t2f9: f64 = (p.p12 * t2f7);(l.f65, l.f69, ) = (t2f9, (p.p12 * t2f8), );let t306: f64 = (l.f22ef - l.f22f4);let t307: f64 = (p.p26 * t306);let t308: f64 = (1.0 + t307);
        let (t30c, t30d,) = {
    if (t308 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t309: f64 = (l.f22ef - l.f22f4);let t30a: f64 = (p.p26 * t309);let t30b: f64 = (1.0 + t30a);
        (t30b, (p.p26 * l.f22f0),)
    }
};
        let t30e: f64 = (p.p14 * t30c);(l.f6b, l.f6f, ) = (t30e, (p.p14 * t30d), );let t30f: f64 = (l.f22ef - l.f22f4);let t310: f64 = (p.p21 * t30f);let t311: f64 = (1.0 + t310);
        let (t315, t316,) = {
    if (t311 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t312: f64 = (l.f22ef - l.f22f4);let t313: f64 = (p.p21 * t312);let t314: f64 = (1.0 + t313);
        (t314, (p.p21 * l.f22f0),)
    }
};
        let t317: f64 = (p.p15 * t315);(l.f72, l.f73, ) = (t317, (p.p15 * t316), );let t318: f64 = (l.f22ef - l.f22f4);let t319: f64 = (p.p22 * t318);let t31a: f64 = (1.0 + t319);
        let (t31e, t31f,) = {
    if (t31a < 0.01) {
        (0.01, 0.0,)
    } else {
        let t31b: f64 = (l.f22ef - l.f22f4);let t31c: f64 = (p.p22 * t31b);let t31d: f64 = (1.0 + t31c);
        (t31d, (p.p22 * l.f22f0),)
    }
};
        let t320: f64 = (p.p16 * t31e);(l.f5a, l.f5b, ) = (t320, (p.p16 * t31f), );let t321: f64 = (l.f22ef - l.f22f4);let t322: f64 = (p.p23 * t321);let t323: f64 = (1.0 + t322);
        let (t327, t328,) = {
    if (t323 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t324: f64 = (l.f22ef - l.f22f4);let t325: f64 = (p.p23 * t324);let t326: f64 = (1.0 + t325);
        (t326, (p.p23 * l.f22f0),)
    }
};
        let t329: f64 = (p.p17 * t327);(l.f60, l.f61, ) = (t329, (p.p17 * t328), );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t32a: f64 = (l.f22ef - l.f22f4);let t32b: f64 = (p.p24 * t32a);let t32c: f64 = (1.0 + t32b);
        let (t330, t331,) = {
    if (t32c < 0.01) {
        (0.01, 0.0,)
    } else {
        let t32d: f64 = (l.f22ef - l.f22f4);let t32e: f64 = (p.p24 * t32d);let t32f: f64 = (1.0 + t32e);
        (t32f, (p.p24 * l.f22f0),)
    }
};
        let t332: f64 = (p.p19 * t330);(l.f78, l.f79, ) = (t332, (p.p19 * t331), );let t333: f64 = (l.f22ef - l.f22f4);let t334: f64 = (p.p25 * t333);let t335: f64 = (1.0 + t334);
        let (t339, t33a,) = {
    if (t335 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t336: f64 = (l.f22ef - l.f22f4);let t337: f64 = (p.p25 * t336);let t338: f64 = (1.0 + t337);
        (t338, (p.p25 * l.f22f0),)
    }
};
        let t33b: f64 = (p.p18 * t339);(l.f66, l.f67, ) = (t33b, (p.p18 * t33a), );let t33c: f64 = (l.f22ef - l.f22f4);let t33d: f64 = (p.p26 * t33c);let t33e: f64 = (1.0 + t33d);
        let (t342, t343,) = {
    if (t33e < 0.01) {
        (0.01, 0.0,)
    } else {
        let t33f: f64 = (l.f22ef - l.f22f4);let t340: f64 = (p.p26 * t33f);let t341: f64 = (1.0 + t340);
        (t341, (p.p26 * l.f22f0),)
    }
};
        let t344: f64 = (p.p20 * t342);(l.f6c, l.f6d, ) = (t344, (p.p20 * t343), );let t345: f64 = (l.f22ef - l.f22f4);let t346: f64 = (p.p8 * t345);let t347: f64 = (1.0 + t346);
        let (t34b, t34c,) = {
    if (t347 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t348: f64 = (l.f22ef - l.f22f4);let t349: f64 = (p.p8 * t348);let t34a: f64 = (1.0 + t349);
        (t34a, (p.p8 * l.f22f0),)
    }
};
        let t34d: f64 = (p.p7 * t34b);(l.f48, l.f49, ) = (t34d, (p.p7 * t34c), );let t34e: f64 = (l.f22ef - l.f22f4);let t34f: f64 = (p.p82 * t34e);let t350: f64 = (1.0 + t34f);
        let (t354, t355,) = {
    if (t350 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t351: f64 = (l.f22ef - l.f22f4);let t352: f64 = (p.p82 * t351);let t353: f64 = (1.0 + t352);
        (t353, (p.p82 * l.f22f0),)
    }
};
        let t356: f64 = (p.p81 * t354);(l.f3c, l.f3d, ) = (t356, (p.p81 * t355), );let t357: f64 = (l.f22ef - l.f22f4);let t358: f64 = (p.p104 * t357);let t359: f64 = (1.0 + t358);
        let (t35d, t35e,) = {
    if (t359 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t35a: f64 = (l.f22ef - l.f22f4);let t35b: f64 = (p.p104 * t35a);let t35c: f64 = (1.0 + t35b);
        (t35c, (p.p104 * l.f22f0),)
    }
};
        let t35f: f64 = (p.p103 * t35d);(l.f3f, l.f40, ) = (t35f, (p.p103 * t35e), );let t360: f64 = (l.f22ef - l.f22f4);let t361: f64 = (p.p126 * t360);let t362: f64 = (1.0 + t361);
        let (t366, t367,) = {
    if (t362 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t363: f64 = (l.f22ef - l.f22f4);let t364: f64 = (p.p126 * t363);let t365: f64 = (1.0 + t364);
        (t365, (p.p126 * l.f22f0),)
    }
};
        let t368: f64 = (p.p125 * t366);(l.f42, l.f43, ) = (t368, (p.p125 * t367), );let t369: f64 = (l.f22ef - l.f22f4);let t36a: f64 = (p.p148 * t369);let t36b: f64 = (1.0 + t36a);
        let (t36f, t370,) = {
    if (t36b < 0.01) {
        (0.01, 0.0,)
    } else {
        let t36c: f64 = (l.f22ef - l.f22f4);let t36d: f64 = (p.p148 * t36c);let t36e: f64 = (1.0 + t36d);
        (t36e, (p.p148 * l.f22f0),)
    }
};
        let t371: f64 = (p.p147 * t36f);(l.f45, l.f46, ) = (t371, (p.p147 * t370), );let t374: f64 = (l.f22ef - l.f22f4);let t375: f64 = (p.p87 * t374);let t376: f64 = (1.0 + t375);
        let (t37a, t37b,) = {
    if (t376 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t377: f64 = (l.f22ef - l.f22f4);let t378: f64 = (p.p87 * t377);let t379: f64 = (1.0 + t378);
        (t379, (p.p87 * l.f22f0),)
    }
};
        let t37c: f64 = (p.p86 * t37a);(l.f24, l.f25, ) = (t37c, (p.p86 * t37b), );let t37d: f64 = (l.f22ef - l.f22f4);let t37e: f64 = (p.p109 * t37d);let t37f: f64 = (1.0 + t37e);
        let (t383, t384,) = {
    if (t37f < 0.01) {
        (0.01, 0.0,)
    } else {
        let t380: f64 = (l.f22ef - l.f22f4);let t381: f64 = (p.p109 * t380);let t382: f64 = (1.0 + t381);
        (t382, (p.p109 * l.f22f0),)
    }
};
        let t385: f64 = (p.p108 * t383);(l.f27, l.f28, ) = (t385, (p.p108 * t384), );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t386: f64 = (l.f22ef - l.f22f4);let t387: f64 = (p.p131 * t386);let t388: f64 = (1.0 + t387);
        let (t38c, t38d,) = {
    if (t388 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t389: f64 = (l.f22ef - l.f22f4);let t38a: f64 = (p.p131 * t389);let t38b: f64 = (1.0 + t38a);
        (t38b, (p.p131 * l.f22f0),)
    }
};
        let t38e: f64 = (p.p130 * t38c);(l.f2a, l.f2b, ) = (t38e, (p.p130 * t38d), );let t38f: f64 = (l.f22ef - l.f22f4);let t390: f64 = (p.p153 * t38f);let t391: f64 = (1.0 + t390);
        let (t395, t396,) = {
    if (t391 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t392: f64 = (l.f22ef - l.f22f4);let t393: f64 = (p.p153 * t392);let t394: f64 = (1.0 + t393);
        (t394, (p.p153 * l.f22f0),)
    }
};
        let t397: f64 = (p.p152 * t395);(l.f2d, l.f2e, ) = (t397, (p.p152 * t396), );let t398: f64 = (l.f22ef - l.f22f4);let t399: f64 = (p.p89 * t398);let t39a: f64 = (1.0 + t399);
        let (t39e, t39f,) = {
    if (t39a < 0.01) {
        (0.01, 0.0,)
    } else {
        let t39b: f64 = (l.f22ef - l.f22f4);let t39c: f64 = (p.p89 * t39b);let t39d: f64 = (1.0 + t39c);
        (t39d, (p.p89 * l.f22f0),)
    }
};
        let t3a0: f64 = (p.p88 * t39e);(l.fc, l.fd, ) = (t3a0, (p.p88 * t39f), );let t3a1: f64 = (l.f22ef - l.f22f4);let t3a2: f64 = (p.p111 * t3a1);let t3a3: f64 = (1.0 + t3a2);
        let (t3a7, t3a8,) = {
    if (t3a3 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3a4: f64 = (l.f22ef - l.f22f4);let t3a5: f64 = (p.p111 * t3a4);let t3a6: f64 = (1.0 + t3a5);
        (t3a6, (p.p111 * l.f22f0),)
    }
};
        let t3a9: f64 = (p.p110 * t3a7);(l.ff, l.f10, ) = (t3a9, (p.p110 * t3a8), );let t3aa: f64 = (l.f22ef - l.f22f4);let t3ab: f64 = (p.p133 * t3aa);let t3ac: f64 = (1.0 + t3ab);
        let (t3b0, t3b1,) = {
    if (t3ac < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3ad: f64 = (l.f22ef - l.f22f4);let t3ae: f64 = (p.p133 * t3ad);let t3af: f64 = (1.0 + t3ae);
        (t3af, (p.p133 * l.f22f0),)
    }
};
        let t3b2: f64 = (p.p132 * t3b0);(l.f12, l.f13, ) = (t3b2, (p.p132 * t3b1), );let t3b3: f64 = (l.f22ef - l.f22f4);let t3b4: f64 = (p.p155 * t3b3);let t3b5: f64 = (1.0 + t3b4);
        let (t3b9, t3ba,) = {
    if (t3b5 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3b6: f64 = (l.f22ef - l.f22f4);let t3b7: f64 = (p.p155 * t3b6);let t3b8: f64 = (1.0 + t3b7);
        (t3b8, (p.p155 * l.f22f0),)
    }
};
        let t3bb: f64 = (p.p154 * t3b9);(l.f15, l.f16, ) = (t3bb, (p.p154 * t3ba), );let t3bc: f64 = (l.f22ef - l.f22f4);let t3bd: f64 = (p.p170 * t3bc);let t3be: f64 = (1.0 + t3bd);
        let (t3c2, t3c3,) = {
    if (t3be < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3bf: f64 = (l.f22ef - l.f22f4);let t3c0: f64 = (p.p170 * t3bf);let t3c1: f64 = (1.0 + t3c0);
        (t3c1, (p.p170 * l.f22f0),)
    }
};
        let t3c4: f64 = (p.p169 * t3c2);(l.f30, l.f31, ) = (t3c4, (p.p169 * t3c3), );let t3c5: f64 = (l.f22ef - l.f22f4);let t3c6: f64 = (p.p192 * t3c5);let t3c7: f64 = (1.0 + t3c6);
        let (t3cb, t3cc,) = {
    if (t3c7 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3c8: f64 = (l.f22ef - l.f22f4);let t3c9: f64 = (p.p192 * t3c8);let t3ca: f64 = (1.0 + t3c9);
        (t3ca, (p.p192 * l.f22f0),)
    }
};
        let t3cd: f64 = (p.p191 * t3cb);(l.f33, l.f34, ) = (t3cd, (p.p191 * t3cc), );let t3d1: f64 = (l.f22ef - l.f22f4);let t3d2: f64 = (p.p214 * t3d1);let t3d3: f64 = (1.0 + t3d2);
        let (t3d7, t3d8,) = {
    if (t3d3 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3d4: f64 = (l.f22ef - l.f22f4);let t3d5: f64 = (p.p214 * t3d4);let t3d6: f64 = (1.0 + t3d5);
        (t3d6, (p.p214 * l.f22f0),)
    }
};
        let t3d9: f64 = (p.p213 * t3d7);(l.f36, l.f37, ) = (t3d9, (p.p213 * t3d8), );let t3da: f64 = (l.f22ef - l.f22f4);let t3db: f64 = (p.p236 * t3da);let t3dc: f64 = (1.0 + t3db);
        let (t3e0, t3e1,) = {
    if (t3dc < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3dd: f64 = (l.f22ef - l.f22f4);let t3de: f64 = (p.p236 * t3dd);let t3df: f64 = (1.0 + t3de);
        (t3df, (p.p236 * l.f22f0),)
    }
};
        let t3e2: f64 = (p.p235 * t3e0);(l.f39, l.f3a, ) = (t3e2, (p.p235 * t3e1), );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv19 = ctx.node_voltage(nodes[19]);let t3e3: f64 = (l.f22ef - l.f22f4);let t3e4: f64 = (p.p175 * t3e3);let t3e5: f64 = (1.0 + t3e4);
        let (t3e9, t3ea,) = {
    if (t3e5 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3e6: f64 = (l.f22ef - l.f22f4);let t3e7: f64 = (p.p175 * t3e6);let t3e8: f64 = (1.0 + t3e7);
        (t3e8, (p.p175 * l.f22f0),)
    }
};
        let t3eb: f64 = (p.p174 * t3e9);(l.f18, l.f19, ) = (t3eb, (p.p174 * t3ea), );let t3ec: f64 = (l.f22ef - l.f22f4);let t3ed: f64 = (p.p197 * t3ec);let t3ee: f64 = (1.0 + t3ed);
        let (t3f2, t3f3,) = {
    if (t3ee < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3ef: f64 = (l.f22ef - l.f22f4);let t3f0: f64 = (p.p197 * t3ef);let t3f1: f64 = (1.0 + t3f0);
        (t3f1, (p.p197 * l.f22f0),)
    }
};
        let t3f4: f64 = (p.p196 * t3f2);(l.f1b, l.f1c, ) = (t3f4, (p.p196 * t3f3), );let t3f5: f64 = (l.f22ef - l.f22f4);let t3f6: f64 = (p.p219 * t3f5);let t3f7: f64 = (1.0 + t3f6);
        let (t3fb, t3fc,) = {
    if (t3f7 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3f8: f64 = (l.f22ef - l.f22f4);let t3f9: f64 = (p.p219 * t3f8);let t3fa: f64 = (1.0 + t3f9);
        (t3fa, (p.p219 * l.f22f0),)
    }
};
        let t3fd: f64 = (p.p218 * t3fb);(l.f1e, l.f1f, ) = (t3fd, (p.p218 * t3fc), );let t3fe: f64 = (l.f22ef - l.f22f4);let t3ff: f64 = (p.p241 * t3fe);let t400: f64 = (1.0 + t3ff);
        let (t404, t405,) = {
    if (t400 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t401: f64 = (l.f22ef - l.f22f4);let t402: f64 = (p.p241 * t401);let t403: f64 = (1.0 + t402);
        (t403, (p.p241 * l.f22f0),)
    }
};
        let t406: f64 = (p.p240 * t404);(l.f21, l.f22, ) = (t406, (p.p240 * t405), );let t407: f64 = (l.f22ef - l.f22f4);let t408: f64 = (p.p177 * t407);let t409: f64 = (1.0 + t408);
        let (t40d, t40e,) = {
    if (t409 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t40a: f64 = (l.f22ef - l.f22f4);let t40b: f64 = (p.p177 * t40a);let t40c: f64 = (1.0 + t40b);
        (t40c, (p.p177 * l.f22f0),)
    }
};
        let t40f: f64 = (p.p176 * t40d);(l.f0, l.f1, ) = (t40f, (p.p176 * t40e), );let t410: f64 = (l.f22ef - l.f22f4);let t411: f64 = (p.p199 * t410);let t412: f64 = (1.0 + t411);
        let (t416, t417,) = {
    if (t412 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t413: f64 = (l.f22ef - l.f22f4);let t414: f64 = (p.p199 * t413);let t415: f64 = (1.0 + t414);
        (t415, (p.p199 * l.f22f0),)
    }
};
        let t418: f64 = (p.p198 * t416);(l.f3, l.f4, ) = (t418, (p.p198 * t417), );let t419: f64 = (l.f22ef - l.f22f4);let t41a: f64 = (p.p221 * t419);let t41b: f64 = (1.0 + t41a);
        let (t41f, t420,) = {
    if (t41b < 0.01) {
        (0.01, 0.0,)
    } else {
        let t41c: f64 = (l.f22ef - l.f22f4);let t41d: f64 = (p.p221 * t41c);let t41e: f64 = (1.0 + t41d);
        (t41e, (p.p221 * l.f22f0),)
    }
};
        let t421: f64 = (p.p220 * t41f);(l.f6, l.f7, ) = (t421, (p.p220 * t420), );let t422: f64 = (l.f22ef - l.f22f4);let t423: f64 = (p.p243 * t422);let t424: f64 = (1.0 + t423);
        let (t428, t429,) = {
    if (t424 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t425: f64 = (l.f22ef - l.f22f4);let t426: f64 = (p.p243 * t425);let t427: f64 = (1.0 + t426);
        (t427, (p.p243 * l.f22f0),)
    }
};
        let t42a: f64 = (p.p242 * t428);(l.f9, l.fa, ) = (t42a, (p.p242 * t429), );let t42d: f64 = (p.p6 * (nv5 - nv9));(l.f236d, l.f236e, l.f236f, ) = (t42d, p.p6, (-p.p6), );let t42e: f64 = (p.p6 * (nv8 - nv9));(l.f23a5, l.f23a6, l.f23a7, ) = (t42e, p.p6, (-p.p6), );let t42f: f64 = if p.p52 == 0.0 { 1.0 } else { 0.0 };l.f1d88 = t42f;let t430: f64 = (p.p6 * (nv19 - nv0));let t431: f64 = (p.p6 * (nv19 - nv2));let t432: f64 = if t430 <= t431 { 1.0 } else { 0.0 };l.f1d9d = t432;
        if ((l.f1d88 != 0.0) && (l.f1d9d != 0.0)) {let t433: f64 = (p.p6 * (nv19 - nv2));(l.f23c3, l.f23c4, l.f23c6, l.f23c5, ) = (t433, 0.0, (-p.p6), p.p6, );}
        if ((l.f1d88 != 0.0) && (l.f1d9d == 0.0)) {let t434: f64 = (p.p6 * (nv19 - nv0));(l.f23c3, l.f23c4, l.f23c6, l.f23c5, ) = (t434, (-p.p6), 0.0, p.p6, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv13 = ctx.node_voltage(nodes[13]);let nv19 = ctx.node_voltage(nodes[19]);
        if (l.f1d88 == 0.0) {
            let (t456, t457, t459, t458,) = {
    if (p.p52 != 0.0) {
        let t435: f64 = (p.p6 * (nv19 - nv0));let t436: f64 = (p.p6 * (nv19 - nv2));let t437: f64 = (t435 + t436);let t438: f64 = (p.p6 * (nv19 - nv0));let t439: f64 = (p.p6 * (nv19 - nv2));let t43a: f64 = (t438 - t439);let t43b: f64 = (0.001 / p.p53);let t43c: f64 = (p.p6 * (nv19 - nv0));let t43d: f64 = (p.p6 * (nv19 - nv2));let t43e: f64 = (t43c - t43d);let t43f: f64 = (t43b * t43e);let t440: f64 = (t43f).tanh();let t441: f64 = (t43a * t440);let t442: f64 = (t437 + t441);let t443: f64 = (0.5 * t442);
        (t443, (0.5 * ((-p.p6) + (((-p.p6) * t440) + (t43a * ((t43b * (-p.p6)) / ((t43f).cosh() * (t43f).cosh())))))), (0.5 * ((-p.p6) + (((-(-p.p6)) * t440) + (t43a * ((t43b * (-(-p.p6))) / ((t43f).cosh() * (t43f).cosh())))))), (0.5 * ((p.p6 + p.p6) + (((p.p6 - p.p6) * t440) + (t43a * ((t43b * (p.p6 - p.p6)) / ((t43f).cosh() * (t43f).cosh())))))),)
    } else {
        let (t452, t453, t455, t454,) = {
            if (p.p52 == 0.0) {
                let t444: f64 = (p.p6 * (nv19 - nv0));let t445: f64 = (p.p6 * (nv19 - nv2));let t446: f64 = (t444 + t445);let t447: f64 = (p.p6 * (nv19 - nv0));let t448: f64 = (p.p6 * (nv19 - nv2));let t449: f64 = (t447 - t448);let t44a: f64 = (p.p6 * (nv19 - nv0));let t44b: f64 = (p.p6 * (nv19 - nv2));let t44c: f64 = (t44a - t44b);let t44d: f64 = (t449 * t44c);let t44e: f64 = (t44d + p.p53);let t44f: f64 = (t44e).sqrt();let t450: f64 = (t446 + t44f);let t451: f64 = (0.5 * t450);
                (t451, (0.5 * ((-p.p6) + ((((-p.p6) * t44c) + (t449 * (-p.p6))) / (2.0 * t44f)))), (0.5 * ((-p.p6) + ((((-(-p.p6)) * t44c) + (t449 * (-(-p.p6)))) / (2.0 * t44f)))), (0.5 * ((p.p6 + p.p6) + ((((p.p6 - p.p6) * t44c) + (t449 * (p.p6 - p.p6))) / (2.0 * t44f)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t452, t453, t455, t454,)
    }
};
            (l.f23c3, l.f23c4, l.f23c6, l.f23c5, ) = (t456, t457, t459, t458, );
        }
        let t45a: f64 = (p.p29 * p.p56);let t45b: f64 = (t45a * p.p33);let t45c: f64 = (1.0 / t45b);let t45d: f64 = (p.p55 + t45c);l.f23b6 = t45d;let t45e: f64 = (p.p6 * (nv13 - nv19));(l.f2374, l.f2375, l.f2376, ) = (t45e, p.p6, (-p.p6), );let t45f: f64 = (l.f23b6 - l.f23c3);(l.f23af, l.f23b0, l.f23b2, l.f23b1, ) = (t45f, (-l.f23c4), (-l.f23c6), (-l.f23c5), );(l.f23e0, l.f23e1, l.f23e2, l.f23e3, ) = (0.0, 0.0, 0.0, 0.0, );(l.f23df, l.f23e4, ) = (0.0, 0.0, );(l.f7d, l.f7f, l.f7e, ) = (1.0, 0.0, 0.0, );(l.f2347, l.f2348, ) = (0.0, 0.0, );(l.f2377, l.f2378, ) = (0.0, 0.0, );(l.f234a, l.f234b, ) = (0.0, 0.0, );(l.f237a, l.f237b, ) = (0.0, 0.0, );(l.f51, l.f52, l.f53, ) = (0.0, 0.0, 0.0, );(l.f55, l.f56, l.f57, ) = (0.0, 0.0, 0.0, );(l.f4b, l.f4c, l.f4d, l.f4e, l.f4f, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );let t463: f64 = if p.p328 == 1.0 { 1.0 } else { 0.0 };l.f1db2 = t463;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv17 = ctx.node_voltage(nodes[17]);let nv20 = ctx.node_voltage(nodes[20]);let nv21 = ctx.node_voltage(nodes[21]);let nv22 = ctx.node_voltage(nodes[22]);let nv23 = ctx.node_voltage(nodes[23]);let nv25 = ctx.node_voltage(nodes[25]);let nv26 = ctx.node_voltage(nodes[26]);
        if (l.f1db2 != 0.0) {
            let t464: f64 = ((nv0 - nv1)).abs();let t465: f64 = (p.p333 * t464);let t466: f64 = ((nv0 - nv1) - p.p331);let t467: f64 = ((nv21 - 0.0) * p.p335);let t468: f64 = (t466 - t467);let t469: f64 = (t468 / p.p334);let t46a: f64 = ((nv0 - nv1) - p.p331);let t46b: f64 = ((nv21 - 0.0) * p.p335);let t46c: f64 = (t46a - t46b);let t46d: f64 = (t46c / p.p334);let t46e: f64 = (-50.0);
            let (t493, t494, t495, t496,) = {
    if ((!(t469 > 50.0)) && (!(t46d < t46e))) {
        let t46f: f64 = ((nv0 - nv1) - p.p331);let t470: f64 = ((nv21 - 0.0) * p.p335);let t471: f64 = (t46f - t470);let t472: f64 = (t471 / p.p334);let t473: f64 = (t472).exp();
        (t473, (t473 * (1.0 / p.p334)), (t473 * (-1.0 / p.p334)), (t473 * ((-p.p335) / p.p334)),)
    } else {
        let t474: f64 = ((nv0 - nv1) - p.p331);let t475: f64 = ((nv21 - 0.0) * p.p335);let t476: f64 = (t474 - t475);let t477: f64 = (t476 / p.p334);let t478: f64 = ((nv0 - nv1) - p.p331);let t479: f64 = ((nv21 - 0.0) * p.p335);let t47a: f64 = (t478 - t479);let t47b: f64 = (t47a / p.p334);let t47c: f64 = (-50.0);
        let (t48f, t490, t491, t492,) = {
            if ((!(t477 > 50.0)) && (t47b < t47c)) {
                let t47d: f64 = (-50.0);let t47e: f64 = (t47d).exp();
                (t47e, 0.0, 0.0, 0.0,)
            } else {
                let t47f: f64 = ((nv0 - nv1) - p.p331);let t480: f64 = ((nv21 - 0.0) * p.p335);let t481: f64 = (t47f - t480);let t482: f64 = (t481 / p.p334);
                let (t48b, t48c, t48d, t48e,) = {
                    if (t482 > 50.0) {
                        let t483: f64 = (50.0_f64).exp();let t484: f64 = ((nv0 - nv1) - p.p331);let t485: f64 = ((nv21 - 0.0) * p.p335);let t486: f64 = (t484 - t485);let t487: f64 = (t486 / p.p334);let t488: f64 = (t487 - 50.0);let t489: f64 = (1.0 + t488);let t48a: f64 = (t483 * t489);
                        (t48a, (t483 * (1.0 / p.p334)), (t483 * (-1.0 / p.p334)), (t483 * ((-p.p335) / p.p334)),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t48b, t48c, t48d, t48e,)
            }
        };
        (t48f, t490, t491, t492,)
    }
};
            let t497: f64 = (t465 + t493);(l.f23e0, l.f23e1, l.f23e2, l.f23e3, ) = (t497, ((p.p333 * if (nv0 - nv1) >= 0.0 { 1.0 } else { (-1.0) }) + t494), ((p.p333 * if (nv0 - nv1) >= 0.0 { -1.0 } else { 1.0 }) + t495), t496, );
        }
        if (l.f1db2 != 0.0) {(l.f23df, l.f23e4, ) = ((nv20 - 0.0), 1.0, );let t498: f64 = (l.f23df * l.f22f9);let t499: f64 = (1.0 + t498);(l.f7d, l.f7f, l.f7e, ) = (t499, (l.f23df * l.f22fa), (l.f23e4 * l.f22f9), );}
        let t49a: f64 = if p.p328 == 2.0 { 1.0 } else { 0.0 };l.f1dc8 = t49a;
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {(l.f2347, l.f2348, ) = ((nv22 - 0.0), 1.0, );(l.f234a, l.f234b, ) = ((nv23 - 0.0), 1.0, );}
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {let t49b: f64 = (l.f234a - l.f2347);let t49c: f64 = (t49b).abs();let t49d: f64 = (t49c / p.p338);(l.f51, l.f52, l.f53, ) = (t49d, (if t49b >= 0.0 { (-l.f2348) } else { (-(-l.f2348)) } / p.p338), (if t49b >= 0.0 { l.f234b } else { (-l.f234b) } / p.p338), );}
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {(l.f2377, l.f2378, ) = ((nv25 - 0.0), 1.0, );(l.f237a, l.f237b, ) = ((nv26 - 0.0), 1.0, );}
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {let t1: f64 = (l.f237a - l.f2377);let t2: f64 = (t1).abs();let t3: f64 = (t2 / p.p337);(l.f55, l.f56, l.f57, ) = (t3, (if t1 >= 0.0 { (-l.f2378) } else { (-(-l.f2378)) } / p.p337), (if t1 >= 0.0 { l.f237b } else { (-l.f237b) } / p.p337), );}
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {let t4: f64 = (1.0 + l.f51);let t5: f64 = (t4 + l.f55);let t6: f64 = (1.0 / t5);(l.f4b, l.f4c, l.f4d, l.f4e, l.f4f, ) = (t6, (-(l.f52 / (t5 * t5))), (-(l.f53 / (t5 * t5))), (-(l.f56 / (t5 * t5))), (-(l.f57 / (t5 * t5))), );}
        let t7: f64 = if p.p52 == 0.0 { 1.0 } else { 0.0 };l.f1ddc = t7;let t8: f64 = (p.p6 * (nv17 - nv0));let t9: f64 = (p.p6 * (nv17 - nv2));let ta: f64 = if t8 <= t9 { 1.0 } else { 0.0 };l.f1df1 = ta;
        if ((l.f1ddc != 0.0) && (l.f1df1 != 0.0)) {let tb: f64 = (p.p6 * (nv17 - nv2));(l.f2343, l.f2344, l.f2346, l.f2345, ) = (tb, 0.0, (-p.p6), p.p6, );}
        if ((l.f1ddc != 0.0) && (l.f1df1 == 0.0)) {let tc: f64 = (p.p6 * (nv17 - nv0));(l.f2343, l.f2344, l.f2346, l.f2345, ) = (tc, (-p.p6), 0.0, p.p6, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let nv17 = ctx.node_voltage(nodes[17]);let nv18 = ctx.node_voltage(nodes[18]);
        if (l.f1ddc == 0.0) {
            let (t2e, t2f, t31, t30,) = {
    if (p.p52 != 0.0) {
        let td: f64 = (p.p6 * (nv17 - nv0));let te: f64 = (p.p6 * (nv17 - nv2));let tf: f64 = (td + te);let t10: f64 = (p.p6 * (nv17 - nv0));let t11: f64 = (p.p6 * (nv17 - nv2));let t12: f64 = (t10 - t11);let t13: f64 = (0.001 / p.p53);let t14: f64 = (p.p6 * (nv17 - nv0));let t15: f64 = (p.p6 * (nv17 - nv2));let t16: f64 = (t14 - t15);let t17: f64 = (t13 * t16);let t18: f64 = (t17).tanh();let t19: f64 = (t12 * t18);let t1a: f64 = (tf + t19);let t1b: f64 = (0.5 * t1a);
        (t1b, (0.5 * ((-p.p6) + (((-p.p6) * t18) + (t12 * ((t13 * (-p.p6)) / ((t17).cosh() * (t17).cosh())))))), (0.5 * ((-p.p6) + (((-(-p.p6)) * t18) + (t12 * ((t13 * (-(-p.p6))) / ((t17).cosh() * (t17).cosh())))))), (0.5 * ((p.p6 + p.p6) + (((p.p6 - p.p6) * t18) + (t12 * ((t13 * (p.p6 - p.p6)) / ((t17).cosh() * (t17).cosh())))))),)
    } else {
        let (t2a, t2b, t2d, t2c,) = {
            if (p.p52 == 0.0) {
                let t1c: f64 = (p.p6 * (nv17 - nv0));let t1d: f64 = (p.p6 * (nv17 - nv2));let t1e: f64 = (t1c + t1d);let t1f: f64 = (p.p6 * (nv17 - nv0));let t20: f64 = (p.p6 * (nv17 - nv2));let t21: f64 = (t1f - t20);let t22: f64 = (p.p6 * (nv17 - nv0));let t23: f64 = (p.p6 * (nv17 - nv2));let t24: f64 = (t22 - t23);let t25: f64 = (t21 * t24);let t26: f64 = (t25 + p.p53);let t27: f64 = (t26).sqrt();let t28: f64 = (t1e + t27);let t29: f64 = (0.5 * t28);
                (t29, (0.5 * ((-p.p6) + ((((-p.p6) * t24) + (t21 * (-p.p6))) / (2.0 * t27)))), (0.5 * ((-p.p6) + ((((-(-p.p6)) * t24) + (t21 * (-(-p.p6)))) / (2.0 * t27)))), (0.5 * ((p.p6 + p.p6) + ((((p.p6 - p.p6) * t24) + (t21 * (p.p6 - p.p6))) / (2.0 * t27)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2a, t2b, t2d, t2c,)
    }
};
            (l.f2343, l.f2344, l.f2346, l.f2345, ) = (t2e, t2f, t31, t30, );
        }
        let t32: f64 = (l.f7d * p.p29);let t33: f64 = (t32 * p.p68);let t34: f64 = (t33 * p.p33);let t35: f64 = (1.0 / t34);let t36: f64 = (p.p67 + t35);(l.f23b3, l.f23b5, l.f23b4, ) = (t36, (-((((l.f7f * p.p29) * p.p68) * p.p33) / (t34 * t34))), (-((((l.f7e * p.p29) * p.p68) * p.p33) / (t34 * t34))), );let t37: f64 = (p.p6 * (nv18 - nv17));(l.f2371, l.f2372, l.f2373, ) = (t37, (-p.p6), p.p6, );let t39: f64 = (l.f23b3 - l.f2343);(l.f23a9, l.f23aa, l.f23ac, l.f23ae, l.f23ab, l.f23ad, ) = (t39, (-l.f2344), (-l.f2346), l.f23b5, (-l.f2345), l.f23b4, );let t3a: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };l.f1e06 = t3a;
        if (l.f1e06 != 0.0) {let t3b: f64 = (p.p6 * (nv7 - nv10));(l.f2391, l.f2393, l.f2394, l.f2392, ) = (t3b, 0.0, p.p6, (-p.p6), );let t3c: f64 = (p.p6 * (nv2 - nv10));(l.f232f, l.f2331, l.f2332, l.f2330, ) = (t3c, p.p6, 0.0, (-p.p6), );}
        if (l.f1e06 == 0.0) {let t3d: f64 = (p.p6 * (nv2 - nv10));(l.f2391, l.f2393, l.f2394, l.f2392, ) = (t3d, p.p6, 0.0, (-p.p6), );let t3e: f64 = (p.p6 * (nv7 - nv10));(l.f232f, l.f2331, l.f2332, l.f2330, ) = (t3e, 0.0, p.p6, (-p.p6), );}
        let t3f: f64 = (p.p6 * (nv9 - nv10));(l.f235d, l.f235f, l.f235e, ) = (t3f, p.p6, (-p.p6), );let t40: f64 = (p.p6 * (nv3 - nv10));(l.f230b, l.f230d, l.f230c, ) = (t40, p.p6, (-p.p6), );let t41: f64 = if p.p100 == 1.0 { 1.0 } else { 0.0 };l.f1e1a = t41;
        if (l.f1e1a != 0.0) {let t42: f64 = (p.p6 * (nv7 - nv11));(l.f2396, l.f2398, l.f2399, l.f2397, ) = (t42, 0.0, p.p6, (-p.p6), );let t43: f64 = (p.p6 * (nv2 - nv11));(l.f2334, l.f2336, l.f2337, l.f2335, ) = (t43, p.p6, 0.0, (-p.p6), );}
        if (l.f1e1a == 0.0) {let t45: f64 = (p.p6 * (nv2 - nv11));(l.f2396, l.f2398, l.f2399, l.f2397, ) = (t45, p.p6, 0.0, (-p.p6), );let t46: f64 = (p.p6 * (nv7 - nv11));(l.f2334, l.f2336, l.f2337, l.f2335, ) = (t46, 0.0, p.p6, (-p.p6), );}
        let t47: f64 = (p.p6 * (nv10 - nv11));(l.f2361, l.f2362, l.f2363, ) = (t47, p.p6, (-p.p6), );let t48: f64 = (p.p6 * (nv3 - nv11));(l.f230f, l.f2311, l.f2310, ) = (t48, p.p6, (-p.p6), );let t49: f64 = if p.p122 == 1.0 { 1.0 } else { 0.0 };l.f1e30 = t49;
        if (l.f1e30 != 0.0) {let t4a: f64 = (p.p6 * (nv7 - nv12));(l.f239b, l.f239d, l.f239e, l.f239c, ) = (t4a, 0.0, p.p6, (-p.p6), );let t4b: f64 = (p.p6 * (nv2 - nv12));(l.f2339, l.f233b, l.f233c, l.f233a, ) = (t4b, p.p6, 0.0, (-p.p6), );}
        if (l.f1e30 == 0.0) {let t4c: f64 = (p.p6 * (nv2 - nv12));(l.f239b, l.f239d, l.f239e, l.f239c, ) = (t4c, p.p6, 0.0, (-p.p6), );let t4d: f64 = (p.p6 * (nv7 - nv12));(l.f2339, l.f233b, l.f233c, l.f233a, ) = (t4d, 0.0, p.p6, (-p.p6), );}
        let t4e: f64 = (p.p6 * (nv11 - nv12));(l.f2365, l.f2366, l.f2367, ) = (t4e, p.p6, (-p.p6), );let t51: f64 = (p.p6 * (nv3 - nv12));(l.f2313, l.f2315, l.f2314, ) = (t51, p.p6, (-p.p6), );let t52: f64 = if p.p144 == 1.0 { 1.0 } else { 0.0 };l.f1e46 = t52;
        if (l.f1e46 != 0.0) {let t53: f64 = (p.p6 * (nv7 - nv13));(l.f23a0, l.f23a2, l.f23a3, l.f23a1, ) = (t53, 0.0, p.p6, (-p.p6), );let t54: f64 = (p.p6 * (nv2 - nv13));(l.f233e, l.f2340, l.f2341, l.f233f, ) = (t54, p.p6, 0.0, (-p.p6), );}
        if (l.f1e46 == 0.0) {let t55: f64 = (p.p6 * (nv2 - nv13));(l.f23a0, l.f23a2, l.f23a3, l.f23a1, ) = (t55, p.p6, 0.0, (-p.p6), );let t56: f64 = (p.p6 * (nv7 - nv13));(l.f233e, l.f2340, l.f2341, l.f233f, ) = (t56, 0.0, p.p6, (-p.p6), );}
        let t57: f64 = (p.p6 * (nv12 - nv13));(l.f2369, l.f236a, l.f236b, ) = (t57, p.p6, (-p.p6), );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv13 = ctx.node_voltage(nodes[13]);let nv14 = ctx.node_voltage(nodes[14]);let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);let nv17 = ctx.node_voltage(nodes[17]);let t58: f64 = (p.p6 * (nv3 - nv13));(l.f2317, l.f2319, l.f2318, ) = (t58, p.p6, (-p.p6), );let t59: f64 = if p.p166 == 1.0 { 1.0 } else { 0.0 };l.f1e5d = t59;
        if (l.f1e5d != 0.0) {let t5a: f64 = (p.p6 * (nv7 - nv5));(l.f237d, l.f237e, l.f237f, l.f2380, ) = (t5a, 0.0, (-p.p6), p.p6, );let t5d: f64 = (p.p6 * (nv2 - nv5));(l.f231b, l.f231c, l.f231d, l.f231e, ) = (t5d, p.p6, (-p.p6), 0.0, );}
        if (l.f1e5d == 0.0) {let t5e: f64 = (p.p6 * (nv2 - nv5));(l.f237d, l.f237e, l.f237f, l.f2380, ) = (t5e, p.p6, (-p.p6), 0.0, );let t5f: f64 = (p.p6 * (nv7 - nv5));(l.f231b, l.f231c, l.f231d, l.f231e, ) = (t5f, 0.0, (-p.p6), p.p6, );}
        let t60: f64 = (p.p6 * (nv14 - nv5));(l.f234d, l.f234f, l.f234e, ) = (t60, (-p.p6), p.p6, );let t61: f64 = (p.p6 * (nv3 - nv5));(l.f22fb, l.f22fc, l.f22fd, ) = (t61, p.p6, (-p.p6), );let t62: f64 = if p.p188 == 1.0 { 1.0 } else { 0.0 };l.f1e71 = t62;
        if (l.f1e71 != 0.0) {let t63: f64 = (p.p6 * (nv7 - nv14));(l.f2382, l.f2384, l.f2385, l.f2383, ) = (t63, 0.0, p.p6, (-p.p6), );let t64: f64 = (p.p6 * (nv2 - nv14));(l.f2320, l.f2322, l.f2323, l.f2321, ) = (t64, p.p6, 0.0, (-p.p6), );}
        if (l.f1e71 == 0.0) {let t65: f64 = (p.p6 * (nv2 - nv14));(l.f2382, l.f2384, l.f2385, l.f2383, ) = (t65, p.p6, 0.0, (-p.p6), );let t66: f64 = (p.p6 * (nv7 - nv14));(l.f2320, l.f2322, l.f2323, l.f2321, ) = (t66, 0.0, p.p6, (-p.p6), );}
        let t6c: f64 = (p.p6 * (nv15 - nv14));(l.f2351, l.f2352, l.f2353, ) = (t6c, (-p.p6), p.p6, );let t6d: f64 = (p.p6 * (nv3 - nv14));(l.f22ff, l.f2301, l.f2300, ) = (t6d, p.p6, (-p.p6), );let t6e: f64 = if p.p210 == 1.0 { 1.0 } else { 0.0 };l.f1e87 = t6e;
        if (l.f1e87 != 0.0) {let t6f: f64 = (p.p6 * (nv7 - nv15));(l.f2387, l.f2389, l.f238a, l.f2388, ) = (t6f, 0.0, p.p6, (-p.p6), );let t70: f64 = (p.p6 * (nv2 - nv15));(l.f2325, l.f2327, l.f2328, l.f2326, ) = (t70, p.p6, 0.0, (-p.p6), );}
        if (l.f1e87 == 0.0) {let t71: f64 = (p.p6 * (nv2 - nv15));(l.f2387, l.f2389, l.f238a, l.f2388, ) = (t71, p.p6, 0.0, (-p.p6), );let t72: f64 = (p.p6 * (nv7 - nv15));(l.f2325, l.f2327, l.f2328, l.f2326, ) = (t72, 0.0, p.p6, (-p.p6), );}
        let t73: f64 = (p.p6 * (nv16 - nv15));(l.f2355, l.f2356, l.f2357, ) = (t73, (-p.p6), p.p6, );let t74: f64 = (p.p6 * (nv3 - nv15));(l.f2303, l.f2305, l.f2304, ) = (t74, p.p6, (-p.p6), );let t75: f64 = if p.p232 == 1.0 { 1.0 } else { 0.0 };l.f1e9d = t75;
        if (l.f1e9d != 0.0) {let t7b: f64 = (p.p6 * (nv7 - nv16));(l.f238c, l.f238e, l.f238f, l.f238d, ) = (t7b, 0.0, p.p6, (-p.p6), );let t7c: f64 = (p.p6 * (nv2 - nv16));(l.f232a, l.f232c, l.f232d, l.f232b, ) = (t7c, p.p6, 0.0, (-p.p6), );}
        if (l.f1e9d == 0.0) {let t7d: f64 = (p.p6 * (nv2 - nv16));(l.f238c, l.f238e, l.f238f, l.f238d, ) = (t7d, p.p6, 0.0, (-p.p6), );let t7e: f64 = (p.p6 * (nv7 - nv16));(l.f232a, l.f232c, l.f232d, l.f232b, ) = (t7e, 0.0, p.p6, (-p.p6), );}
        let t7f: f64 = (p.p6 * (nv17 - nv16));(l.f2359, l.f235a, l.f235b, ) = (t7f, (-p.p6), p.p6, );let t80: f64 = (p.p6 * (nv3 - nv16));(l.f2307, l.f2309, l.f2308, ) = (t80, p.p6, (-p.p6), );(l.f20e3, l.f20e6, l.f20e7, l.f20e8, l.f20e9, l.f20e4, l.f20e5, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f223f, l.f2242, l.f2243, l.f2244, l.f2240, l.f2241, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f21fd, l.f2200, l.f2201, l.f2202, l.f21fe, l.f21ff, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f21b6, l.f21b9, l.f21ba, l.f21bb, l.f21bc, l.f21b7, l.f21b8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f2176, l.f2179, l.f217a, l.f217b, l.f217c, l.f2177, l.f2178, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f22bc, l.f22bf, l.f22c0, l.f22c1, l.f22c2, l.f22bd, l.f22be, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );let t82: f64 = if p.p233 > p.p354 { 1.0 } else { 0.0 };l.f1eb3 = t82;
        if (l.f1eb3 != 0.0) {(l.fb53, l.fb56, l.fb57, l.fb58, l.fb59, l.fb54, l.fb55, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa81, l.fa84, l.fa85, l.fa86, l.fa87, l.fa82, l.fa83, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.faec, l.faef, l.faf0, l.faf1, l.faed, l.faee, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fae3, l.fae6, l.fae7, l.fae8, l.fae4, l.fae5, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fabf, l.fac2, l.fac3, l.fac4, l.fac5, l.fac0, l.fac1, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fab5, l.fab8, l.fab9, l.faba, l.fabb, l.fab6, l.fab7, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb44, l.fb47, l.fb48, l.fb49, l.fb4a, l.fb45, l.fb46, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbec, l.fbef, l.fbed, l.fbee, ) = (0.0, 0.0, 0.0, 0.0, );(l.fb7a, l.fb84, l.fb85, l.fb86, l.fb87, l.fb82, l.fb83, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbd4, l.fbd6, l.fbd7, l.fbd5, ) = (l.f238c, l.f238e, l.f238f, l.f238d, );(l.fbb2, l.fbb3, l.fbb4, ) = (l.f2359, l.f235a, l.f235b, );l.fabd = p.p239;(l.fb74, l.fb76, l.fb77, l.fb75, ) = (l.f232a, l.f232c, l.f232d, l.f232b, );(l.fb70, l.fb72, l.fb71, ) = (l.f2307, l.f2309, l.f2308, );l.faea = p.p237;(l.fb5c, l.fb5d, ) = (l.f22ef, l.f22f0, );l.fb62 = l.f22f4;(l.fab2, l.fab3, ) = (l.f215b, l.f215c, );l.fc0e = p.p0;l.fa8a = p.p233;(l.f9d1, l.f9d2, ) = (l.f39, l.f3a, );l.f9d4 = p.p238;(l.f9ce, l.f9cf, ) = (l.f21, l.f22, );(l.f9cb, l.f9cc, ) = (l.f9, l.fa, );l.fbf3 = p.p234;l.fb5a = p.p248;l.f9d7 = p.p247;l.f9d9 = 0.0;l.faae = p.p249;l.f9c4 = p.p253;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1eb3 != 0.0) {
            l.fbc6 = p.p244;l.fa8e = p.p245;l.f9c9 = p.p246;l.fa8c = p.p252;l.fbf1 = p.p251;l.fbf8 = p.p250;l.f9de = p.p39;l.f9e0 = p.p47;l.fc0c = p.p45;l.fa88 = p.p42;l.fab0 = p.p2;l.fb6e = p.p6;l.fb64 = 1.0;(l.f9c5, l.f9c6, ) = (0.0, 0.0, );(l.f9d6, l.f9db, l.f9dc, ) = (0.0, 0.0, 0.0, );(l.faa6, l.faac, l.faaa, l.faab, ) = (0.0, 0.0, 0.0, 0.0, );(l.fbf5, l.fbf6, ) = (0.0, 0.0, );(l.fbd9, l.fbda, l.fbdb, ) = (0.0, 0.0, 0.0, );(l.fa5c, l.fa66, l.fa67, l.fa68, l.fa69, l.fa64, l.fa65, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb66, l.fb6c, l.fb6a, l.fb6b, ) = (0.0, 0.0, 0.0, 0.0, );(l.fb27, l.fb2d, l.fb2b, l.fb2c, ) = (0.0, 0.0, 0.0, 0.0, );(l.fa11, l.fa1b, l.fa1c, l.fa1d, l.fa1e, l.fa19, l.fa1a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb09, l.fb13, l.fb14, l.fb15, l.fb16, l.fb11, l.fb12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa90, l.fa96, l.fa97, l.fa98, l.fa99, l.fa94, l.fa95, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbfa, l.fc00, l.fc01, l.fc02, l.fc03, l.fbfe, l.fbff, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fc05, l.fc08, l.fc09, l.fc0a, l.fc0b, l.fc06, l.fc07, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.faa7, l.faa8, ) = (0.0, 0.0, );(l.fa5d, l.fa60, l.fa61, l.fa62, l.fa5e, l.fa5f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb67, l.fb68, ) = (0.0, 0.0, );(l.fb28, l.fb29, ) = (0.0, 0.0, );(l.fa12, l.fa15, l.fa16, l.fa17, l.fa13, l.fa14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb0a, l.fb0d, l.fb0e, l.fb0f, l.fb0b, l.fb0c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa91, l.fa92, ) = (0.0, 0.0, );(l.fbfb, l.fbfc, ) = (0.0, 0.0, );(l.fb5f, l.fb60, ) = (0.0, 0.0, );(l.fa3e, l.fa48, l.fa49, l.fa4a, l.fa4b, l.fa46, l.fa47, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f9e2, l.f9ec, l.f9ed, l.f9ee, l.f9ef, l.f9ea, l.f9eb, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb18, l.fb22, l.fb23, l.fb24, l.fb25, l.fb20, l.fb21, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa3f, l.fa42, l.fa43, l.fa44, l.fa40, l.fa41, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f9e3, l.f9e6, l.f9e7, l.f9e8, l.f9e4, l.f9e5, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb19, l.fb1c, l.fb1d, l.fb1e, l.fb1a, l.fb1b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb90, l.fba5, l.fba6, l.fba7, l.fba8, l.fba3, l.fba4, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb94, l.fb9e, l.fb9f, l.fba0, l.fba1, l.fb9c, l.fb9d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb79, l.fb8b, l.fb8c, l.fb8d, l.fb8e, l.fb89, l.fb8a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa72, l.fa7c, l.fa7d, l.fa7e, l.fa7f, l.fa7a, l.fa7b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbb6, l.fbc0, l.fbc1, l.fbc2, l.fbc3, l.fbbe, l.fbbf, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa2f, l.fa39, l.fa3a, l.fa3b, l.fa3c, l.fa37, l.fa38, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbdd, l.fbe7, l.fbe8, l.fbe9, l.fbea, l.fbe5, l.fbe6, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa4d, l.fa57, l.fa58, l.fa59, l.fa5a, l.fa55, l.fa56, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f9fc, l.fa06, l.fa07, l.fa08, l.fa09, l.fa04, l.fa05, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.faf3, l.fafd, l.fafe, l.faff, l.fb00, l.fafb, l.fafc, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbaa, l.fbad, l.fbae, l.fbaf, l.fbb0, l.fbab, l.fbac, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa6b, l.fa6e, l.fa6f, l.fa70, l.fa71, l.fa6c, l.fa6d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbc5, l.fbca, l.fbcb, l.fbcc, l.fbcd, l.fbc8, l.fbc9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb91, l.fb92, ) = (0.0, 0.0, );(l.fb95, l.fb98, l.fb99, l.fb9a, l.fb96, l.fb97, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb7b, l.fb7e, l.fb7f, l.fb80, l.fb7c, l.fb7d, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa73, l.fa76, l.fa77, l.fa78, l.fa74, l.fa75, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbb7, l.fbba, l.fbbb, l.fbbc, l.fbb8, l.fbb9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa30, l.fa33, l.fa34, l.fa35, l.fa31, l.fa32, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fbde, l.fbe1, l.fbe2, l.fbe3, l.fbdf, l.fbe0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa4e, l.fa51, l.fa52, l.fa53, l.fa4f, l.fa50, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (l.f9fd, l.fa00, l.fa01, l.fa02, l.f9fe, l.f9ff, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.faf4, l.faf7, l.faf8, l.faf9, l.faf5, l.faf6, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb30, l.fb33, l.fb34, l.fb35, l.fb31, l.fb32, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb37, l.fb3a, l.fb3b, l.fb3c, l.fb38, l.fb39, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.facf, l.fad2, l.fad3, l.fad4, l.fad0, l.fad1, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fad6, l.fad9, l.fada, l.fadb, l.fad7, l.fad8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb4c, l.fb4f, l.fb50, l.fb51, l.fb4d, l.fb4e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb02, l.fb05, l.fb06, l.fb07, l.fb03, l.fb04, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fac8, l.facb, l.facc, l.facd, l.fac9, l.faca, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fb2f, l.fb40, l.fb41, l.fb42, l.fb3e, l.fb3f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fac7, l.fadf, l.fae0, l.fae1, l.fadd, l.fade, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f9f6, l.f9f8, l.f9f9, l.f9fa, l.f9f7, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f9f1, l.f9f3, l.f9f4, l.f9f2, ) = (0.0, 0.0, 0.0, 0.0, );(l.fa0b, l.fa0d, l.fa0e, l.fa0f, l.fa0c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa20, l.fa2a, l.fa2b, l.fa2c, l.fa2d, l.fa28, l.fa29, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa9b, l.faa1, l.faa2, l.faa3, l.faa4, l.fa9f, l.faa0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f9c0, l.f9c1, l.f9c2, ) = (0.0, 0.0, 0.0, );(l.fbce, l.fbd1, l.fbd2, l.fbcf, l.fbd0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa21, l.fa24, l.fa25, l.fa26, l.fa22, l.fa23, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.fa9c, l.fa9d, ) = (0.0, 0.0, );
        }
        if (l.f1eb3 != 0.0) {
            let (tb3, tb4, tb5,) = {
    if (p.p52 != 0.0) {
        let ta9: f64 = (0.001 / p.p53);let taa: f64 = (ta9 * l.fbb2);let tab: f64 = (taa).tanh();let tac: f64 = (l.fbb2 * tab);
        (tac, ((l.fbb3 * tab) + (l.fbb2 * ((ta9 * l.fbb3) / ((taa).cosh() * (taa).cosh())))), ((l.fbb4 * tab) + (l.fbb2 * ((ta9 * l.fbb4) / ((taa).cosh() * (taa).cosh())))),)
    } else {
        let (tb0, tb1, tb2,) = {
            if (p.p52 == 0.0) {
                let tad: f64 = (l.fbb2 * l.fbb2);let tae: f64 = (tad + p.p53);let taf: f64 = (tae).sqrt();
                (taf, (((l.fbb3 * l.fbb2) + (l.fbb2 * l.fbb3)) / (2.0 * taf)), (((l.fbb4 * l.fbb2) + (l.fbb2 * l.fbb4)) / (2.0 * taf)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (tb0, tb1, tb2,)
    }
};
            (l.f9c0, l.f9c1, l.f9c2, ) = (tb3, tb4, tb5, );
        }
        if (l.f1eb3 != 0.0) {let tb6: f64 = (l.fbd4 - l.fbb2);(l.fbce, l.fbd1, l.fbd2, l.fbcf, l.fbd0, ) = (tb6, l.fbd6, l.fbd7, (l.fbd5 - l.fbb3), (-l.fbb4), );let tb7: f64 = (l.f9c4 * l.fab2);(l.f9c5, l.f9c6, ) = (tb7, (l.f9c4 * l.fab3), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1eb3 != 0.0) {let tba: f64 = (2.302585092994046 * l.fab2);let tbb: f64 = (l.fb5a / tba);let tbc: f64 = (l.faae * l.f9c0);let tbd: f64 = (tbb + tbc);(l.faa6, l.faac, l.faaa, l.faab, ) = (tbd, (-((l.fb5a * (2.302585092994046 * l.fab3)) / (tba * tba))), (l.faae * l.f9c1), (l.faae * l.f9c2), );let tbe: f64 = (l.fb5c - l.fb62);let tbf: f64 = (l.fbf8 * tbe);let tc0: f64 = (l.fbf3 + tbf);(l.fbf5, l.fbf6, ) = (tc0, (l.fbf8 * l.fb5d), );}
        if (l.f1eb3 != 0.0) {let tc1: f64 = (l.fb5c / l.fb62);let tc2: f64 = (tc1).powf(l.f9e0);(l.fb5f, l.fb60, ) = (tc2, if 0.0 == 0.0 && ((l.f9e0) as f64).is_finite() && ((l.f9e0) as f64).fract() == 0.0 { if l.f9e0 == 0.0 { 0.0 } else { (l.f9e0 * ((tc1).powf(l.f9e0 - 1.0) * (l.fb5d / l.fb62))) } } else { (tc2 * (l.f9e0 * ((l.fb5d / l.fb62) / tc1))) }, );}
        let tc3: f64 = if l.f9de != 0.0 { 1.0 } else { 0.0 };l.f1edb = tc3;
        if ((l.f1eb3 != 0.0) && (l.f1edb != 0.0)) {let tc4: f64 = (l.f9c0 / l.f9de);let tc5: f64 = (tc4).powf(l.f9c9);let tc6: f64 = (1.0 + tc5);let tc7: f64 = (1.0 / l.f9c9);let tc8: f64 = (tc6).powf(tc7);let tc9: f64 = (l.f9c0 / tc8);(l.fbd9, l.fbda, l.fbdb, ) = (tc9, (((l.f9c1 * tc8) - (l.f9c0 * if 0.0 == 0.0 && ((tc7) as f64).is_finite() && ((tc7) as f64).fract() == 0.0 { if tc7 == 0.0 { 0.0 } else { (tc7 * ((tc6).powf(tc7 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((tc4).powf(l.f9c9 - 1.0) * (l.f9c1 / l.f9de))) } } else { (tc5 * (l.f9c9 * ((l.f9c1 / l.f9de) / tc4))) })) } } else { (tc8 * (tc7 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((tc4).powf(l.f9c9 - 1.0) * (l.f9c1 / l.f9de))) } } else { (tc5 * (l.f9c9 * ((l.f9c1 / l.f9de) / tc4))) } / tc6))) })) / (tc8 * tc8)), (((l.f9c2 * tc8) - (l.f9c0 * if 0.0 == 0.0 && ((tc7) as f64).is_finite() && ((tc7) as f64).fract() == 0.0 { if tc7 == 0.0 { 0.0 } else { (tc7 * ((tc6).powf(tc7 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((tc4).powf(l.f9c9 - 1.0) * (l.f9c2 / l.f9de))) } } else { (tc5 * (l.f9c9 * ((l.f9c2 / l.f9de) / tc4))) })) } } else { (tc8 * (tc7 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((tc4).powf(l.f9c9 - 1.0) * (l.f9c2 / l.f9de))) } } else { (tc5 * (l.f9c9 * ((l.f9c2 / l.f9de) / tc4))) } / tc6))) })) / (tc8 * tc8)), );}
        if ((l.f1eb3 != 0.0) && (l.f1edb == 0.0)) {(l.fbd9, l.fbda, l.fbdb, ) = (0.0, 0.0, 0.0, );}
        if (l.f1eb3 != 0.0) {let tca: f64 = (l.fbd9 * l.f9d9);let tcb: f64 = (l.f9d7 - tca);let tcc: f64 = (tcb * l.f9c0);(l.f9d6, l.f9db, l.f9dc, ) = (tcc, (((-(l.fbda * l.f9d9)) * l.f9c0) + (tcb * l.f9c1)), (((-(l.fbdb * l.f9d9)) * l.f9c0) + (tcb * l.f9c2)), );let tcd: f64 = (l.fbf5 - l.f9d6);(l.fbec, l.fbef, l.fbed, l.fbee, ) = (tcd, l.fbf6, (-l.f9db), (-l.f9dc), );let tce: f64 = (2.0 * l.faa6);let tcf: f64 = (tce * l.fab2);(l.fb66, l.fb6c, l.fb6a, l.fb6b, ) = (tcf, (((2.0 * l.faac) * l.fab2) + (tce * l.fab3)), ((2.0 * l.faaa) * l.fab2), ((2.0 * l.faab) * l.fab2), );let td0: f64 = (l.f9d1 * l.fb66);(l.fb27, l.fb2d, l.fb2b, l.fb2c, ) = (td0, ((l.f9d2 * l.fb66) + (l.f9d1 * l.fb6c)), (l.f9d1 * l.fb6a), (l.f9d1 * l.fb6b), );let td4: f64 = (p.p51 * l.f9c5);let td5: f64 = (td4 / 2.0);let td6: f64 = (l.fbec - td5);(l.fa9b, l.faa1, l.faa2, l.faa3, l.faa4, l.fa9f, l.faa0, ) = (td6, 0.0, 0.0, (l.fbef - ((p.p51 * l.f9c6) / 2.0)), 0.0, l.fbed, l.fbee, );}
        if (l.f1eb3 != 0.0) {
            let (ted, tf0, tf1, tee, tef,) = {
    if (p.p52 != 0.0) {
        let td7: f64 = (l.fbd4 + l.fbce);let td8: f64 = (l.fbd4 - l.fbce);let td9: f64 = (0.001 / p.p53);let tda: f64 = (l.fbd4 - l.fbce);let tdb: f64 = (td9 * tda);let tdc: f64 = (tdb).tanh();let tdd: f64 = (td8 * tdc);let tde: f64 = (td7 + tdd);let tdf: f64 = (0.5 * tde);
        (tdf, (0.5 * ((l.fbd6 + l.fbd1) + (((l.fbd6 - l.fbd1) * tdc) + (td8 * ((td9 * (l.fbd6 - l.fbd1)) / ((tdb).cosh() * (tdb).cosh())))))), (0.5 * ((l.fbd7 + l.fbd2) + (((l.fbd7 - l.fbd2) * tdc) + (td8 * ((td9 * (l.fbd7 - l.fbd2)) / ((tdb).cosh() * (tdb).cosh())))))), (0.5 * ((l.fbd5 + l.fbcf) + (((l.fbd5 - l.fbcf) * tdc) + (td8 * ((td9 * (l.fbd5 - l.fbcf)) / ((tdb).cosh() * (tdb).cosh())))))), (0.5 * (l.fbd0 + (((-l.fbd0) * tdc) + (td8 * ((td9 * (-l.fbd0)) / ((tdb).cosh() * (tdb).cosh())))))),)
    } else {
        let (te8, teb, tec, te9, tea,) = {
            if (p.p52 == 0.0) {
                let te0: f64 = (l.fbd4 + l.fbce);let te1: f64 = (l.fbd4 - l.fbce);let te2: f64 = (l.fbd4 - l.fbce);let te3: f64 = (te1 * te2);let te4: f64 = (te3 + p.p53);let te5: f64 = (te4).sqrt();let te6: f64 = (te0 + te5);let te7: f64 = (0.5 * te6);
                (te7, (0.5 * ((l.fbd6 + l.fbd1) + ((((l.fbd6 - l.fbd1) * te2) + (te1 * (l.fbd6 - l.fbd1))) / (2.0 * te5)))), (0.5 * ((l.fbd7 + l.fbd2) + ((((l.fbd7 - l.fbd2) * te2) + (te1 * (l.fbd7 - l.fbd2))) / (2.0 * te5)))), (0.5 * ((l.fbd5 + l.fbcf) + ((((l.fbd5 - l.fbcf) * te2) + (te1 * (l.fbd5 - l.fbcf))) / (2.0 * te5)))), (0.5 * (l.fbd0 + ((((-l.fbd0) * te2) + (te1 * (-l.fbd0))) / (2.0 * te5)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (te8, teb, tec, te9, tea,)
    }
};
            let tf2: f64 = (ted - l.fa9b);let tf3: f64 = (tf2 / l.f9c5);(l.fa20, l.fa2a, l.fa2b, l.fa2c, l.fa2d, l.fa28, l.fa29, ) = (tf3, ((tf0 - l.faa1) / l.f9c5), ((-l.faa2) / l.f9c5), ((((-l.faa3) * l.f9c5) - (tf2 * l.f9c6)) / (l.f9c5 * l.f9c5)), ((tf1 - l.faa4) / l.f9c5), ((tee - l.fa9f) / l.f9c5), ((tef - l.faa0) / l.f9c5), );
        }
        let tf4: f64 = if l.fa20 > 50.0 { 1.0 } else { 0.0 };l.f1ef1 = tf4;
        if ((l.f1eb3 != 0.0) && (l.f1ef1 != 0.0)) {(l.fa3e, l.fa48, l.fa49, l.fa4a, l.fa4b, l.fa46, l.fa47, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let tf5: f64 = (-50.0);let tf6: f64 = if l.fa20 < tf5 { 1.0 } else { 0.0 };l.f1f05 = tf6;
        if (((l.f1eb3 != 0.0) && (l.f1ef1 == 0.0)) && (l.f1f05 != 0.0)) {(l.fa3e, l.fa48, l.fa49, l.fa4a, l.fa4b, l.fa46, l.fa47, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f1eb3 != 0.0) && (l.f1ef1 == 0.0)) && (l.f1f05 == 0.0)) {let tf7: f64 = (l.fa20).exp();let tf8: f64 = (1.0 + tf7);let tf9: f64 = (1.0 / tf8);(l.fa3e, l.fa48, l.fa49, l.fa4a, l.fa4b, l.fa46, l.fa47, ) = (tf9, (-((tf7 * l.fa2a) / (tf8 * tf8))), (-((tf7 * l.fa2b) / (tf8 * tf8))), (-((tf7 * l.fa2c) / (tf8 * tf8))), (-((tf7 * l.fa2d) / (tf8 * tf8))), (-((tf7 * l.fa28) / (tf8 * tf8))), (-((tf7 * l.fa29) / (tf8 * tf8))), );}
        if (l.f1eb3 != 0.0) {
            let (t110, t113, t114, t111, t112,) = {
    if (p.p52 != 0.0) {
        let tfa: f64 = (l.fbd4 + l.fbce);let tfb: f64 = (l.fbd4 - l.fbce);let tfc: f64 = (0.001 / p.p53);let tfd: f64 = (l.fbd4 - l.fbce);let tfe: f64 = (tfc * tfd);let tff: f64 = (tfe).tanh();let t100: f64 = (tfb * tff);let t101: f64 = (tfa + t100);let t102: f64 = (0.5 * t101);
        (t102, (0.5 * ((l.fbd6 + l.fbd1) + (((l.fbd6 - l.fbd1) * tff) + (tfb * ((tfc * (l.fbd6 - l.fbd1)) / ((tfe).cosh() * (tfe).cosh())))))), (0.5 * ((l.fbd7 + l.fbd2) + (((l.fbd7 - l.fbd2) * tff) + (tfb * ((tfc * (l.fbd7 - l.fbd2)) / ((tfe).cosh() * (tfe).cosh())))))), (0.5 * ((l.fbd5 + l.fbcf) + (((l.fbd5 - l.fbcf) * tff) + (tfb * ((tfc * (l.fbd5 - l.fbcf)) / ((tfe).cosh() * (tfe).cosh())))))), (0.5 * (l.fbd0 + (((-l.fbd0) * tff) + (tfb * ((tfc * (-l.fbd0)) / ((tfe).cosh() * (tfe).cosh())))))),)
    } else {
        let (t10b, t10e, t10f, t10c, t10d,) = {
            if (p.p52 == 0.0) {
                let t103: f64 = (l.fbd4 + l.fbce);let t104: f64 = (l.fbd4 - l.fbce);let t105: f64 = (l.fbd4 - l.fbce);let t106: f64 = (t104 * t105);let t107: f64 = (t106 + p.p53);let t108: f64 = (t107).sqrt();let t109: f64 = (t103 + t108);let t10a: f64 = (0.5 * t109);
                (t10a, (0.5 * ((l.fbd6 + l.fbd1) + ((((l.fbd6 - l.fbd1) * t105) + (t104 * (l.fbd6 - l.fbd1))) / (2.0 * t108)))), (0.5 * ((l.fbd7 + l.fbd2) + ((((l.fbd7 - l.fbd2) * t105) + (t104 * (l.fbd7 - l.fbd2))) / (2.0 * t108)))), (0.5 * ((l.fbd5 + l.fbcf) + ((((l.fbd5 - l.fbcf) * t105) + (t104 * (l.fbd5 - l.fbcf))) / (2.0 * t108)))), (0.5 * (l.fbd0 + ((((-l.fbd0) * t105) + (t104 * (-l.fbd0))) / (2.0 * t108)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t10b, t10e, t10f, t10c, t10d,)
    }
};
            let t115: f64 = (p.p51 * 0.1);let t116: f64 = (t115 * l.f9c5);let t117: f64 = (t116 * l.fa3e);let t118: f64 = (l.fbec - t117);let t119: f64 = (t110 - t118);let t11a: f64 = (t119 / l.fb66);(l.f9e2, l.f9ec, l.f9ed, l.f9ee, l.f9ef, l.f9ea, l.f9eb, ) = (t11a, ((t113 - (-(t116 * l.fa48))) / l.fb66), ((-(-(t116 * l.fa49))) / l.fb66), ((((-(l.fbef - (((t115 * l.f9c6) * l.fa3e) + (t116 * l.fa4a)))) * l.fb66) - (t119 * l.fb6c)) / (l.fb66 * l.fb66)), ((t114 - (-(t116 * l.fa4b))) / l.fb66), ((((t111 - (l.fbed - (t116 * l.fa46))) * l.fb66) - (t119 * l.fb6a)) / (l.fb66 * l.fb66)), ((((t112 - (l.fbee - (t116 * l.fa47))) * l.fb66) - (t119 * l.fb6b)) / (l.fb66 * l.fb66)), );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t11b: f64 = if l.f9e2 > 50.0 { 1.0 } else { 0.0 };l.f1f1b = t11b;
        if ((l.f1eb3 != 0.0) && (l.f1f1b != 0.0)) {let t11c: f64 = (l.fb27 * l.f9e2);(l.fb18, l.fb22, l.fb23, l.fb24, l.fb25, l.fb20, l.fb21, ) = (t11c, (l.fb27 * l.f9ec), (l.fb27 * l.f9ed), ((l.fb2d * l.f9e2) + (l.fb27 * l.f9ee)), (l.fb27 * l.f9ef), ((l.fb2b * l.f9e2) + (l.fb27 * l.f9ea)), ((l.fb2c * l.f9e2) + (l.fb27 * l.f9eb)), );}
        let t11e: f64 = (-50.0);let t11f: f64 = if l.f9e2 < t11e { 1.0 } else { 0.0 };l.f1f33 = t11f;
        if (((l.f1eb3 != 0.0) && (l.f1f1b == 0.0)) && (l.f1f33 != 0.0)) {let t120: f64 = (l.f9e2).exp();let t121: f64 = (l.fb27 * t120);(l.fb18, l.fb22, l.fb23, l.fb24, l.fb25, l.fb20, l.fb21, ) = (t121, (l.fb27 * (t120 * l.f9ec)), (l.fb27 * (t120 * l.f9ed)), ((l.fb2d * t120) + (l.fb27 * (t120 * l.f9ee))), (l.fb27 * (t120 * l.f9ef)), ((l.fb2b * t120) + (l.fb27 * (t120 * l.f9ea))), ((l.fb2c * t120) + (l.fb27 * (t120 * l.f9eb))), );}
        if (((l.f1eb3 != 0.0) && (l.f1f1b == 0.0)) && (l.f1f33 == 0.0)) {let t122: f64 = (l.f9e2).exp();let t123: f64 = (1.0 + t122);let t124: f64 = (t123).ln();let t125: f64 = (l.fb27 * t124);(l.fb18, l.fb22, l.fb23, l.fb24, l.fb25, l.fb20, l.fb21, ) = (t125, (l.fb27 * ((t122 * l.f9ec) / t123)), (l.fb27 * ((t122 * l.f9ed) / t123)), ((l.fb2d * t124) + (l.fb27 * ((t122 * l.f9ee) / t123))), (l.fb27 * ((t122 * l.f9ef) / t123)), ((l.fb2b * t124) + (l.fb27 * ((t122 * l.f9ea) / t123))), ((l.fb2c * t124) + (l.fb27 * ((t122 * l.f9eb) / t123))), );}
        if (l.f1eb3 != 0.0) {let t126: f64 = (l.fa8c * l.fb18);let t127: f64 = (t126 / l.f9d1);let t128: f64 = (1.0 + t127);let t129: f64 = (l.fb5f * t128);let t12a: f64 = (l.fa8e / t129);(l.fa90, l.fa96, l.fa97, l.fa98, l.fa99, l.fa94, l.fa95, ) = (t12a, (-((l.fa8e * (l.fb5f * ((l.fa8c * l.fb22) / l.f9d1))) / (t129 * t129))), (-((l.fa8e * (l.fb5f * ((l.fa8c * l.fb23) / l.f9d1))) / (t129 * t129))), (-((l.fa8e * ((l.fb60 * t128) + (l.fb5f * ((((l.fa8c * l.fb24) * l.f9d1) - (t126 * l.f9d2)) / (l.f9d1 * l.f9d1))))) / (t129 * t129))), (-((l.fa8e * (l.fb5f * ((l.fa8c * l.fb25) / l.f9d1))) / (t129 * t129))), (-((l.fa8e * (l.fb5f * ((l.fa8c * l.fb20) / l.f9d1))) / (t129 * t129))), (-((l.fa8e * (l.fb5f * ((l.fa8c * l.fb21) / l.f9d1))) / (t129 * t129))), );let t12b: f64 = (l.fc0c * l.fb62);let t12c: f64 = (1.0 + t12b);let t12d: f64 = (l.fc0c * l.fb5c);let t12e: f64 = (1.0 + t12d);let t12f: f64 = (t12c / t12e);let t130: f64 = (l.fbc6 * t12f);let t131: f64 = (l.fa88 * l.f9c0);let t132: f64 = (t131 / l.fa8a);let t133: f64 = (1.0 + t132);let t134: f64 = (t130 * t133);let t135: f64 = (l.fbf1 * l.fb18);let t136: f64 = (t135 / l.f9d1);let t137: f64 = (1.0 + t136);let t138: f64 = (t134 / t137);(l.fbfa, l.fc00, l.fc01, l.fc02, l.fc03, l.fbfe, l.fbff, ) = (t138, (-((t134 * ((l.fbf1 * l.fb22) / l.f9d1)) / (t137 * t137))), (-((t134 * ((l.fbf1 * l.fb23) / l.f9d1)) / (t137 * t137))), (((((l.fbc6 * (-((t12c * (l.fc0c * l.fb5d)) / (t12e * t12e)))) * t133) * t137) - (t134 * ((((l.fbf1 * l.fb24) * l.f9d1) - (t135 * l.f9d2)) / (l.f9d1 * l.f9d1)))) / (t137 * t137)), (-((t134 * ((l.fbf1 * l.fb25) / l.f9d1)) / (t137 * t137))), ((((t130 * ((l.fa88 * l.f9c1) / l.fa8a)) * t137) - (t134 * ((l.fbf1 * l.fb20) / l.f9d1))) / (t137 * t137)), ((((t130 * ((l.fa88 * l.f9c2) / l.fa8a)) * t137) - (t134 * ((l.fbf1 * l.fb21) / l.f9d1))) / (t137 * t137)), );}
        if (l.f1eb3 != 0.0) {let t139: f64 = (2.0 * l.fa3e);let t13a: f64 = (t139 * l.fab2);let t13b: f64 = (t13a * l.fa90);let t13c: f64 = (t13b / l.fa8a);let t13d: f64 = (1.0 - l.fa3e);let t13e: f64 = (t13d * l.fbfa);let t13f: f64 = (t13c + t13e);(l.fc05, l.fc08, l.fc09, l.fc0a, l.fc0b, l.fc06, l.fc07, ) = (t13f, ((((((2.0 * l.fa48) * l.fab2) * l.fa90) + (t13a * l.fa96)) / l.fa8a) + (((-l.fa48) * l.fbfa) + (t13d * l.fc00))), ((((((2.0 * l.fa49) * l.fab2) * l.fa90) + (t13a * l.fa97)) / l.fa8a) + (((-l.fa49) * l.fbfa) + (t13d * l.fc01))), (((((((2.0 * l.fa4a) * l.fab2) + (t139 * l.fab3)) * l.fa90) + (t13a * l.fa98)) / l.fa8a) + (((-l.fa4a) * l.fbfa) + (t13d * l.fc02))), ((((((2.0 * l.fa4b) * l.fab2) * l.fa90) + (t13a * l.fa99)) / l.fa8a) + (((-l.fa4b) * l.fbfa) + (t13d * l.fc03))), ((((((2.0 * l.fa46) * l.fab2) * l.fa90) + (t13a * l.fa94)) / l.fa8a) + (((-l.fa46) * l.fbfa) + (t13d * l.fbfe))), ((((((2.0 * l.fa47) * l.fab2) * l.fa90) + (t13a * l.fa95)) / l.fa8a) + (((-l.fa47) * l.fbfa) + (t13d * l.fbff))), );let t140: f64 = (l.fbfa * l.fa8a);let t141: f64 = (t140 / l.fa90);(l.fb90, l.fba5, l.fba6, l.fba7, l.fba8, l.fba3, l.fba4, ) = (t141, ((((l.fc00 * l.fa8a) * l.fa90) - (t140 * l.fa96)) / (l.fa90 * l.fa90)), ((((l.fc01 * l.fa8a) * l.fa90) - (t140 * l.fa97)) / (l.fa90 * l.fa90)), ((((l.fc02 * l.fa8a) * l.fa90) - (t140 * l.fa98)) / (l.fa90 * l.fa90)), ((((l.fc03 * l.fa8a) * l.fa90) - (t140 * l.fa99)) / (l.fa90 * l.fa90)), ((((l.fbfe * l.fa8a) * l.fa90) - (t140 * l.fa94)) / (l.fa90 * l.fa90)), ((((l.fbff * l.fa8a) * l.fa90) - (t140 * l.fa95)) / (l.fa90 * l.fa90)), );let t142: f64 = (2.0 * l.fb18);let t143: f64 = (t142 / l.f9d1);let t144: f64 = (t143 / l.fb90);let t145: f64 = (1.0 + t144);let t146: f64 = (t145).sqrt();let t147: f64 = (l.fb90 * t146);let t148: f64 = (t147 - l.fb90);(l.fb94, l.fb9e, l.fb9f, l.fba0, l.fba1, l.fb9c, l.fb9d, ) = (t148, (((l.fba5 * t146) + (l.fb90 * ((((((2.0 * l.fb22) / l.f9d1) * l.fb90) - (t143 * l.fba5)) / (l.fb90 * l.fb90)) / (2.0 * t146)))) - l.fba5), (((l.fba6 * t146) + (l.fb90 * ((((((2.0 * l.fb23) / l.f9d1) * l.fb90) - (t143 * l.fba6)) / (l.fb90 * l.fb90)) / (2.0 * t146)))) - l.fba6), (((l.fba7 * t146) + (l.fb90 * ((((((((2.0 * l.fb24) * l.f9d1) - (t142 * l.f9d2)) / (l.f9d1 * l.f9d1)) * l.fb90) - (t143 * l.fba7)) / (l.fb90 * l.fb90)) / (2.0 * t146)))) - l.fba7), (((l.fba8 * t146) + (l.fb90 * ((((((2.0 * l.fb25) / l.f9d1) * l.fb90) - (t143 * l.fba8)) / (l.fb90 * l.fb90)) / (2.0 * t146)))) - l.fba8), (((l.fba3 * t146) + (l.fb90 * ((((((2.0 * l.fb20) / l.f9d1) * l.fb90) - (t143 * l.fba3)) / (l.fb90 * l.fb90)) / (2.0 * t146)))) - l.fba3), (((l.fba4 * t146) + (l.fb90 * ((((((2.0 * l.fb21) / l.f9d1) * l.fb90) - (t143 * l.fba4)) / (l.fb90 * l.fb90)) / (2.0 * t146)))) - l.fba4), );}
        if (l.f1eb3 != 0.0) {let t149: f64 = (1.0 - l.fa3e);let t14a: f64 = (l.fb90 * t149);let t14b: f64 = (l.fb66 * l.fa3e);let t14c: f64 = (t14a + t14b);(l.fb79, l.fb8b, l.fb8c, l.fb8d, l.fb8e, l.fb89, l.fb8a, ) = (t14c, (((l.fba5 * t149) + (l.fb90 * (-l.fa48))) + (l.fb66 * l.fa48)), (((l.fba6 * t149) + (l.fb90 * (-l.fa49))) + (l.fb66 * l.fa49)), (((l.fba7 * t149) + (l.fb90 * (-l.fa4a))) + ((l.fb6c * l.fa3e) + (l.fb66 * l.fa4a))), (((l.fba8 * t149) + (l.fb90 * (-l.fa4b))) + (l.fb66 * l.fa4b)), (((l.fba3 * t149) + (l.fb90 * (-l.fa46))) + ((l.fb6a * l.fa3e) + (l.fb66 * l.fa46))), (((l.fba4 * t149) + (l.fb90 * (-l.fa47))) + ((l.fb6b * l.fa3e) + (l.fb66 * l.fa47))), );let t14d: f64 = (1.0 - l.fa3e);let t14e: f64 = (l.fb94 * t14d);let t14f: f64 = (l.fb66 * l.fa3e);let t150: f64 = (t14e + t14f);(l.fb7a, l.fb84, l.fb85, l.fb86, l.fb87, l.fb82, l.fb83, ) = (t150, (((l.fb9e * t14d) + (l.fb94 * (-l.fa48))) + (l.fb66 * l.fa48)), (((l.fb9f * t14d) + (l.fb94 * (-l.fa49))) + (l.fb66 * l.fa49)), (((l.fba0 * t14d) + (l.fb94 * (-l.fa4a))) + ((l.fb6c * l.fa3e) + (l.fb66 * l.fa4a))), (((l.fba1 * t14d) + (l.fb94 * (-l.fa4b))) + (l.fb66 * l.fa4b)), (((l.fb9c * t14d) + (l.fb94 * (-l.fa46))) + ((l.fb6a * l.fa3e) + (l.fb66 * l.fa46))), (((l.fb9d * t14d) + (l.fb94 * (-l.fa47))) + ((l.fb6b * l.fa3e) + (l.fb66 * l.fa47))), );}
        if (l.f1eb3 != 0.0) {
            let (t16f, t172, t173, t174, t175, t170, t171,) = {
    if (p.p52 != 0.0) {
        let t151: f64 = (l.fbb2 / l.fb7a);let t152: f64 = t151;let t153: f64 = (l.fbb2 / l.fb7a);let t154: f64 = (-t153);let t155: f64 = (0.001 / p.p53);let t156: f64 = (l.fbb2 / l.fb7a);let t157: f64 = (-t156);let t158: f64 = (t155 * t157);let t159: f64 = (t158).tanh();let t15a: f64 = (t154 * t159);let t15b: f64 = (t152 + t15a);let t15c: f64 = (0.5 * t15b);
        (t15c, (0.5 * ((-((l.fbb2 * l.fb84) / (l.fb7a * l.fb7a))) + (((-(-((l.fbb2 * l.fb84) / (l.fb7a * l.fb7a)))) * t159) + (t154 * ((t155 * (-(-((l.fbb2 * l.fb84) / (l.fb7a * l.fb7a))))) / ((t158).cosh() * (t158).cosh())))))), (0.5 * ((-((l.fbb2 * l.fb85) / (l.fb7a * l.fb7a))) + (((-(-((l.fbb2 * l.fb85) / (l.fb7a * l.fb7a)))) * t159) + (t154 * ((t155 * (-(-((l.fbb2 * l.fb85) / (l.fb7a * l.fb7a))))) / ((t158).cosh() * (t158).cosh())))))), (0.5 * ((-((l.fbb2 * l.fb86) / (l.fb7a * l.fb7a))) + (((-(-((l.fbb2 * l.fb86) / (l.fb7a * l.fb7a)))) * t159) + (t154 * ((t155 * (-(-((l.fbb2 * l.fb86) / (l.fb7a * l.fb7a))))) / ((t158).cosh() * (t158).cosh())))))), (0.5 * ((-((l.fbb2 * l.fb87) / (l.fb7a * l.fb7a))) + (((-(-((l.fbb2 * l.fb87) / (l.fb7a * l.fb7a)))) * t159) + (t154 * ((t155 * (-(-((l.fbb2 * l.fb87) / (l.fb7a * l.fb7a))))) / ((t158).cosh() * (t158).cosh())))))), (0.5 * ((((l.fbb3 * l.fb7a) - (l.fbb2 * l.fb82)) / (l.fb7a * l.fb7a)) + (((-(((l.fbb3 * l.fb7a) - (l.fbb2 * l.fb82)) / (l.fb7a * l.fb7a))) * t159) + (t154 * ((t155 * (-(((l.fbb3 * l.fb7a) - (l.fbb2 * l.fb82)) / (l.fb7a * l.fb7a)))) / ((t158).cosh() * (t158).cosh())))))), (0.5 * ((((l.fbb4 * l.fb7a) - (l.fbb2 * l.fb83)) / (l.fb7a * l.fb7a)) + (((-(((l.fbb4 * l.fb7a) - (l.fbb2 * l.fb83)) / (l.fb7a * l.fb7a))) * t159) + (t154 * ((t155 * (-(((l.fbb4 * l.fb7a) - (l.fbb2 * l.fb83)) / (l.fb7a * l.fb7a)))) / ((t158).cosh() * (t158).cosh())))))),)
    } else {
        let (t168, t16b, t16c, t16d, t16e, t169, t16a,) = {
            if (p.p52 == 0.0) {
                let t15d: f64 = (l.fbb2 / l.fb7a);let t15e: f64 = t15d;let t15f: f64 = (l.fbb2 / l.fb7a);let t160: f64 = (-t15f);let t161: f64 = (l.fbb2 / l.fb7a);let t162: f64 = (-t161);let t163: f64 = (t160 * t162);let t164: f64 = (t163 + p.p53);let t165: f64 = (t164).sqrt();let t166: f64 = (t15e + t165);let t167: f64 = (0.5 * t166);
                (t167, (0.5 * ((-((l.fbb2 * l.fb84) / (l.fb7a * l.fb7a))) + ((((-(-((l.fbb2 * l.fb84) / (l.fb7a * l.fb7a)))) * t162) + (t160 * (-(-((l.fbb2 * l.fb84) / (l.fb7a * l.fb7a)))))) / (2.0 * t165)))), (0.5 * ((-((l.fbb2 * l.fb85) / (l.fb7a * l.fb7a))) + ((((-(-((l.fbb2 * l.fb85) / (l.fb7a * l.fb7a)))) * t162) + (t160 * (-(-((l.fbb2 * l.fb85) / (l.fb7a * l.fb7a)))))) / (2.0 * t165)))), (0.5 * ((-((l.fbb2 * l.fb86) / (l.fb7a * l.fb7a))) + ((((-(-((l.fbb2 * l.fb86) / (l.fb7a * l.fb7a)))) * t162) + (t160 * (-(-((l.fbb2 * l.fb86) / (l.fb7a * l.fb7a)))))) / (2.0 * t165)))), (0.5 * ((-((l.fbb2 * l.fb87) / (l.fb7a * l.fb7a))) + ((((-(-((l.fbb2 * l.fb87) / (l.fb7a * l.fb7a)))) * t162) + (t160 * (-(-((l.fbb2 * l.fb87) / (l.fb7a * l.fb7a)))))) / (2.0 * t165)))), (0.5 * ((((l.fbb3 * l.fb7a) - (l.fbb2 * l.fb82)) / (l.fb7a * l.fb7a)) + ((((-(((l.fbb3 * l.fb7a) - (l.fbb2 * l.fb82)) / (l.fb7a * l.fb7a))) * t162) + (t160 * (-(((l.fbb3 * l.fb7a) - (l.fbb2 * l.fb82)) / (l.fb7a * l.fb7a))))) / (2.0 * t165)))), (0.5 * ((((l.fbb4 * l.fb7a) - (l.fbb2 * l.fb83)) / (l.fb7a * l.fb7a)) + ((((-(((l.fbb4 * l.fb7a) - (l.fbb2 * l.fb83)) / (l.fb7a * l.fb7a))) * t162) + (t160 * (-(((l.fbb4 * l.fb7a) - (l.fbb2 * l.fb83)) / (l.fb7a * l.fb7a))))) / (2.0 * t165)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t168, t16b, t16c, t16d, t16e, t169, t16a,)
    }
};
            let t176: f64 = (t16f).powf(l.f9c9);let t177: f64 = (1.0 + t176);let t178: f64 = (1.0 / l.f9c9);let t179: f64 = (t177).powf(t178);let t17a: f64 = (1.0 / t179);
            (l.fa72, l.fa7c, l.fa7d, l.fa7e, l.fa7f, l.fa7a, l.fa7b, ) = (t17a, (-(if 0.0 == 0.0 && ((t178) as f64).is_finite() && ((t178) as f64).fract() == 0.0 { if t178 == 0.0 { 0.0 } else { (t178 * ((t177).powf(t178 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t172)) } } else { (t176 * (l.f9c9 * (t172 / t16f))) })) } } else { (t179 * (t178 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t172)) } } else { (t176 * (l.f9c9 * (t172 / t16f))) } / t177))) } / (t179 * t179))), (-(if 0.0 == 0.0 && ((t178) as f64).is_finite() && ((t178) as f64).fract() == 0.0 { if t178 == 0.0 { 0.0 } else { (t178 * ((t177).powf(t178 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t173)) } } else { (t176 * (l.f9c9 * (t173 / t16f))) })) } } else { (t179 * (t178 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t173)) } } else { (t176 * (l.f9c9 * (t173 / t16f))) } / t177))) } / (t179 * t179))), (-(if 0.0 == 0.0 && ((t178) as f64).is_finite() && ((t178) as f64).fract() == 0.0 { if t178 == 0.0 { 0.0 } else { (t178 * ((t177).powf(t178 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t174)) } } else { (t176 * (l.f9c9 * (t174 / t16f))) })) } } else { (t179 * (t178 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t174)) } } else { (t176 * (l.f9c9 * (t174 / t16f))) } / t177))) } / (t179 * t179))), (-(if 0.0 == 0.0 && ((t178) as f64).is_finite() && ((t178) as f64).fract() == 0.0 { if t178 == 0.0 { 0.0 } else { (t178 * ((t177).powf(t178 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t175)) } } else { (t176 * (l.f9c9 * (t175 / t16f))) })) } } else { (t179 * (t178 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t175)) } } else { (t176 * (l.f9c9 * (t175 / t16f))) } / t177))) } / (t179 * t179))), (-(if 0.0 == 0.0 && ((t178) as f64).is_finite() && ((t178) as f64).fract() == 0.0 { if t178 == 0.0 { 0.0 } else { (t178 * ((t177).powf(t178 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t170)) } } else { (t176 * (l.f9c9 * (t170 / t16f))) })) } } else { (t179 * (t178 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t170)) } } else { (t176 * (l.f9c9 * (t170 / t16f))) } / t177))) } / (t179 * t179))), (-(if 0.0 == 0.0 && ((t178) as f64).is_finite() && ((t178) as f64).fract() == 0.0 { if t178 == 0.0 { 0.0 } else { (t178 * ((t177).powf(t178 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t171)) } } else { (t176 * (l.f9c9 * (t171 / t16f))) })) } } else { (t179 * (t178 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t16f).powf(l.f9c9 - 1.0) * t171)) } } else { (t176 * (l.f9c9 * (t171 / t16f))) } / t177))) } / (t179 * t179))), );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1eb3 != 0.0) {let t17b: f64 = (l.fbb2 * l.fa72);(l.fbb6, l.fbc0, l.fbc1, l.fbc2, l.fbc3, l.fbbe, l.fbbf, ) = (t17b, (l.fbb2 * l.fa7c), (l.fbb2 * l.fa7d), (l.fbb2 * l.fa7e), (l.fbb2 * l.fa7f), ((l.fbb3 * l.fa72) + (l.fbb2 * l.fa7a)), ((l.fbb4 * l.fa72) + (l.fbb2 * l.fa7b)), );}
        if (l.f1eb3 != 0.0) {
            let (t1a0, t1a3, t1a4, t1a5, t1a6, t1a1, t1a2,) = {
    if (p.p52 != 0.0) {
        let t17c: f64 = (-l.fbb2);let t17d: f64 = (t17c / l.fb7a);let t17e: f64 = t17d;let t17f: f64 = (-l.fbb2);let t180: f64 = (t17f / l.fb7a);let t181: f64 = (-t180);let t182: f64 = (0.001 / p.p53);let t183: f64 = (-l.fbb2);let t184: f64 = (t183 / l.fb7a);let t185: f64 = (-t184);let t186: f64 = (t182 * t185);let t187: f64 = (t186).tanh();let t188: f64 = (t181 * t187);let t189: f64 = (t17e + t188);let t18a: f64 = (0.5 * t189);
        (t18a, (0.5 * ((-((t17c * l.fb84) / (l.fb7a * l.fb7a))) + (((-(-((t17f * l.fb84) / (l.fb7a * l.fb7a)))) * t187) + (t181 * ((t182 * (-(-((t183 * l.fb84) / (l.fb7a * l.fb7a))))) / ((t186).cosh() * (t186).cosh())))))), (0.5 * ((-((t17c * l.fb85) / (l.fb7a * l.fb7a))) + (((-(-((t17f * l.fb85) / (l.fb7a * l.fb7a)))) * t187) + (t181 * ((t182 * (-(-((t183 * l.fb85) / (l.fb7a * l.fb7a))))) / ((t186).cosh() * (t186).cosh())))))), (0.5 * ((-((t17c * l.fb86) / (l.fb7a * l.fb7a))) + (((-(-((t17f * l.fb86) / (l.fb7a * l.fb7a)))) * t187) + (t181 * ((t182 * (-(-((t183 * l.fb86) / (l.fb7a * l.fb7a))))) / ((t186).cosh() * (t186).cosh())))))), (0.5 * ((-((t17c * l.fb87) / (l.fb7a * l.fb7a))) + (((-(-((t17f * l.fb87) / (l.fb7a * l.fb7a)))) * t187) + (t181 * ((t182 * (-(-((t183 * l.fb87) / (l.fb7a * l.fb7a))))) / ((t186).cosh() * (t186).cosh())))))), (0.5 * (((((-l.fbb3) * l.fb7a) - (t17c * l.fb82)) / (l.fb7a * l.fb7a)) + (((-((((-l.fbb3) * l.fb7a) - (t17f * l.fb82)) / (l.fb7a * l.fb7a))) * t187) + (t181 * ((t182 * (-((((-l.fbb3) * l.fb7a) - (t183 * l.fb82)) / (l.fb7a * l.fb7a)))) / ((t186).cosh() * (t186).cosh())))))), (0.5 * (((((-l.fbb4) * l.fb7a) - (t17c * l.fb83)) / (l.fb7a * l.fb7a)) + (((-((((-l.fbb4) * l.fb7a) - (t17f * l.fb83)) / (l.fb7a * l.fb7a))) * t187) + (t181 * ((t182 * (-((((-l.fbb4) * l.fb7a) - (t183 * l.fb83)) / (l.fb7a * l.fb7a)))) / ((t186).cosh() * (t186).cosh())))))),)
    } else {
        let (t199, t19c, t19d, t19e, t19f, t19a, t19b,) = {
            if (p.p52 == 0.0) {
                let t18b: f64 = (-l.fbb2);let t18c: f64 = (t18b / l.fb7a);let t18d: f64 = t18c;let t18e: f64 = (-l.fbb2);let t18f: f64 = (t18e / l.fb7a);let t190: f64 = (-t18f);let t191: f64 = (-l.fbb2);let t192: f64 = (t191 / l.fb7a);let t193: f64 = (-t192);let t194: f64 = (t190 * t193);let t195: f64 = (t194 + p.p53);let t196: f64 = (t195).sqrt();let t197: f64 = (t18d + t196);let t198: f64 = (0.5 * t197);
                (t198, (0.5 * ((-((t18b * l.fb84) / (l.fb7a * l.fb7a))) + ((((-(-((t18e * l.fb84) / (l.fb7a * l.fb7a)))) * t193) + (t190 * (-(-((t191 * l.fb84) / (l.fb7a * l.fb7a)))))) / (2.0 * t196)))), (0.5 * ((-((t18b * l.fb85) / (l.fb7a * l.fb7a))) + ((((-(-((t18e * l.fb85) / (l.fb7a * l.fb7a)))) * t193) + (t190 * (-(-((t191 * l.fb85) / (l.fb7a * l.fb7a)))))) / (2.0 * t196)))), (0.5 * ((-((t18b * l.fb86) / (l.fb7a * l.fb7a))) + ((((-(-((t18e * l.fb86) / (l.fb7a * l.fb7a)))) * t193) + (t190 * (-(-((t191 * l.fb86) / (l.fb7a * l.fb7a)))))) / (2.0 * t196)))), (0.5 * ((-((t18b * l.fb87) / (l.fb7a * l.fb7a))) + ((((-(-((t18e * l.fb87) / (l.fb7a * l.fb7a)))) * t193) + (t190 * (-(-((t191 * l.fb87) / (l.fb7a * l.fb7a)))))) / (2.0 * t196)))), (0.5 * (((((-l.fbb3) * l.fb7a) - (t18b * l.fb82)) / (l.fb7a * l.fb7a)) + ((((-((((-l.fbb3) * l.fb7a) - (t18e * l.fb82)) / (l.fb7a * l.fb7a))) * t193) + (t190 * (-((((-l.fbb3) * l.fb7a) - (t191 * l.fb82)) / (l.fb7a * l.fb7a))))) / (2.0 * t196)))), (0.5 * (((((-l.fbb4) * l.fb7a) - (t18b * l.fb83)) / (l.fb7a * l.fb7a)) + ((((-((((-l.fbb4) * l.fb7a) - (t18e * l.fb83)) / (l.fb7a * l.fb7a))) * t193) + (t190 * (-((((-l.fbb4) * l.fb7a) - (t191 * l.fb83)) / (l.fb7a * l.fb7a))))) / (2.0 * t196)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t199, t19c, t19d, t19e, t19f, t19a, t19b,)
    }
};
            let t1a7: f64 = (t1a0).powf(l.f9c9);let t1a8: f64 = (1.0 + t1a7);let t1a9: f64 = (1.0 / l.f9c9);let t1aa: f64 = (t1a8).powf(t1a9);let t1ab: f64 = (1.0 / t1aa);
            (l.fa2f, l.fa39, l.fa3a, l.fa3b, l.fa3c, l.fa37, l.fa38, ) = (t1ab, (-(if 0.0 == 0.0 && ((t1a9) as f64).is_finite() && ((t1a9) as f64).fract() == 0.0 { if t1a9 == 0.0 { 0.0 } else { (t1a9 * ((t1a8).powf(t1a9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a3)) } } else { (t1a7 * (l.f9c9 * (t1a3 / t1a0))) })) } } else { (t1aa * (t1a9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a3)) } } else { (t1a7 * (l.f9c9 * (t1a3 / t1a0))) } / t1a8))) } / (t1aa * t1aa))), (-(if 0.0 == 0.0 && ((t1a9) as f64).is_finite() && ((t1a9) as f64).fract() == 0.0 { if t1a9 == 0.0 { 0.0 } else { (t1a9 * ((t1a8).powf(t1a9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a4)) } } else { (t1a7 * (l.f9c9 * (t1a4 / t1a0))) })) } } else { (t1aa * (t1a9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a4)) } } else { (t1a7 * (l.f9c9 * (t1a4 / t1a0))) } / t1a8))) } / (t1aa * t1aa))), (-(if 0.0 == 0.0 && ((t1a9) as f64).is_finite() && ((t1a9) as f64).fract() == 0.0 { if t1a9 == 0.0 { 0.0 } else { (t1a9 * ((t1a8).powf(t1a9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a5)) } } else { (t1a7 * (l.f9c9 * (t1a5 / t1a0))) })) } } else { (t1aa * (t1a9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a5)) } } else { (t1a7 * (l.f9c9 * (t1a5 / t1a0))) } / t1a8))) } / (t1aa * t1aa))), (-(if 0.0 == 0.0 && ((t1a9) as f64).is_finite() && ((t1a9) as f64).fract() == 0.0 { if t1a9 == 0.0 { 0.0 } else { (t1a9 * ((t1a8).powf(t1a9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a6)) } } else { (t1a7 * (l.f9c9 * (t1a6 / t1a0))) })) } } else { (t1aa * (t1a9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a6)) } } else { (t1a7 * (l.f9c9 * (t1a6 / t1a0))) } / t1a8))) } / (t1aa * t1aa))), (-(if 0.0 == 0.0 && ((t1a9) as f64).is_finite() && ((t1a9) as f64).fract() == 0.0 { if t1a9 == 0.0 { 0.0 } else { (t1a9 * ((t1a8).powf(t1a9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a1)) } } else { (t1a7 * (l.f9c9 * (t1a1 / t1a0))) })) } } else { (t1aa * (t1a9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a1)) } } else { (t1a7 * (l.f9c9 * (t1a1 / t1a0))) } / t1a8))) } / (t1aa * t1aa))), (-(if 0.0 == 0.0 && ((t1a9) as f64).is_finite() && ((t1a9) as f64).fract() == 0.0 { if t1a9 == 0.0 { 0.0 } else { (t1a9 * ((t1a8).powf(t1a9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a2)) } } else { (t1a7 * (l.f9c9 * (t1a2 / t1a0))) })) } } else { (t1aa * (t1a9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t1a0).powf(l.f9c9 - 1.0) * t1a2)) } } else { (t1a7 * (l.f9c9 * (t1a2 / t1a0))) } / t1a8))) } / (t1aa * t1aa))), );
        }
        if (l.f1eb3 != 0.0) {let t1ac: f64 = (-l.fbb2);let t1ad: f64 = (t1ac * l.fa2f);(l.fbdd, l.fbe7, l.fbe8, l.fbe9, l.fbea, l.fbe5, l.fbe6, ) = (t1ad, (t1ac * l.fa39), (t1ac * l.fa3a), (t1ac * l.fa3b), (t1ac * l.fa3c), (((-l.fbb3) * l.fa2f) + (t1ac * l.fa37)), (((-l.fbb4) * l.fa2f) + (t1ac * l.fa38)), );let t1ae: f64 = (l.fbd4 - l.fa9b);let t1af: f64 = (t1ae / l.f9c5);(l.fa20, l.fa2a, l.fa2b, l.fa2c, l.fa2d, l.fa28, l.fa29, ) = (t1af, ((l.fbd6 - l.faa1) / l.f9c5), ((-l.faa2) / l.f9c5), ((((-l.faa3) * l.f9c5) - (t1ae * l.f9c6)) / (l.f9c5 * l.f9c5)), ((l.fbd7 - l.faa4) / l.f9c5), ((l.fbd5 - l.fa9f) / l.f9c5), ((-l.faa0) / l.f9c5), );}
        let t1b0: f64 = if l.fa20 > 50.0 { 1.0 } else { 0.0 };l.f1f49 = t1b0;
        if ((l.f1eb3 != 0.0) && (l.f1f49 != 0.0)) {(l.fa5c, l.fa66, l.fa67, l.fa68, l.fa69, l.fa64, l.fa65, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t1b1: f64 = (-50.0);let t1b2: f64 = if l.fa20 < t1b1 { 1.0 } else { 0.0 };l.f1f56 = t1b2;
        if (((l.f1eb3 != 0.0) && (l.f1f49 == 0.0)) && (l.f1f56 != 0.0)) {(l.fa5c, l.fa66, l.fa67, l.fa68, l.fa69, l.fa64, l.fa65, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f1eb3 != 0.0) && (l.f1f49 == 0.0)) && (l.f1f56 == 0.0)) {let t1b3: f64 = (l.fa20).exp();let t1b4: f64 = (1.0 + t1b3);let t1b5: f64 = (1.0 / t1b4);(l.fa5c, l.fa66, l.fa67, l.fa68, l.fa69, l.fa64, l.fa65, ) = (t1b5, (-((t1b3 * l.fa2a) / (t1b4 * t1b4))), (-((t1b3 * l.fa2b) / (t1b4 * t1b4))), (-((t1b3 * l.fa2c) / (t1b4 * t1b4))), (-((t1b3 * l.fa2d) / (t1b4 * t1b4))), (-((t1b3 * l.fa28) / (t1b4 * t1b4))), (-((t1b3 * l.fa29) / (t1b4 * t1b4))), );}
        if (l.f1eb3 != 0.0) {let t1b8: f64 = (l.fbce - l.fbdd);let t1b9: f64 = (p.p51 * 0.1);let t1ba: f64 = (t1b9 * l.f9c5);let t1bb: f64 = (t1ba * l.fa5c);let t1bc: f64 = (l.fbec - t1bb);let t1bd: f64 = (t1b8 - t1bc);let t1be: f64 = (t1bd / l.fb66);(l.fa11, l.fa1b, l.fa1c, l.fa1d, l.fa1e, l.fa19, l.fa1a, ) = (t1be, (((l.fbd1 - l.fbe7) - (-(t1ba * l.fa66))) / l.fb66), (((-l.fbe8) - (-(t1ba * l.fa67))) / l.fb66), (((((-l.fbe9) - (l.fbef - (((t1b9 * l.f9c6) * l.fa5c) + (t1ba * l.fa68)))) * l.fb66) - (t1bd * l.fb6c)) / (l.fb66 * l.fb66)), (((l.fbd2 - l.fbea) - (-(t1ba * l.fa69))) / l.fb66), (((((l.fbcf - l.fbe5) - (l.fbed - (t1ba * l.fa64))) * l.fb66) - (t1bd * l.fb6a)) / (l.fb66 * l.fb66)), (((((l.fbd0 - l.fbe6) - (l.fbee - (t1ba * l.fa65))) * l.fb66) - (t1bd * l.fb6b)) / (l.fb66 * l.fb66)), );}
        let t1bf: f64 = if l.fa11 > 50.0 { 1.0 } else { 0.0 };l.f1f5f = t1bf;
        if ((l.f1eb3 != 0.0) && (l.f1f5f != 0.0)) {let t1c0: f64 = (l.fb27 * l.fa11);(l.fb09, l.fb13, l.fb14, l.fb15, l.fb16, l.fb11, l.fb12, ) = (t1c0, (l.fb27 * l.fa1b), (l.fb27 * l.fa1c), ((l.fb2d * l.fa11) + (l.fb27 * l.fa1d)), (l.fb27 * l.fa1e), ((l.fb2b * l.fa11) + (l.fb27 * l.fa19)), ((l.fb2c * l.fa11) + (l.fb27 * l.fa1a)), );}
        let t1c1: f64 = (-50.0);let t1c2: f64 = if l.fa11 < t1c1 { 1.0 } else { 0.0 };l.f1f62 = t1c2;
        if (((l.f1eb3 != 0.0) && (l.f1f5f == 0.0)) && (l.f1f62 != 0.0)) {let t1c3: f64 = (l.fa11).exp();let t1c4: f64 = (l.fb27 * t1c3);(l.fb09, l.fb13, l.fb14, l.fb15, l.fb16, l.fb11, l.fb12, ) = (t1c4, (l.fb27 * (t1c3 * l.fa1b)), (l.fb27 * (t1c3 * l.fa1c)), ((l.fb2d * t1c3) + (l.fb27 * (t1c3 * l.fa1d))), (l.fb27 * (t1c3 * l.fa1e)), ((l.fb2b * t1c3) + (l.fb27 * (t1c3 * l.fa19))), ((l.fb2c * t1c3) + (l.fb27 * (t1c3 * l.fa1a))), );}
        if (((l.f1eb3 != 0.0) && (l.f1f5f == 0.0)) && (l.f1f62 == 0.0)) {let t1c5: f64 = (l.fa11).exp();let t1c6: f64 = (1.0 + t1c5);let t1c7: f64 = (t1c6).ln();let t1c8: f64 = (l.fb27 * t1c7);(l.fb09, l.fb13, l.fb14, l.fb15, l.fb16, l.fb11, l.fb12, ) = (t1c8, (l.fb27 * ((t1c5 * l.fa1b) / t1c6)), (l.fb27 * ((t1c5 * l.fa1c) / t1c6)), ((l.fb2d * t1c7) + (l.fb27 * ((t1c5 * l.fa1d) / t1c6))), (l.fb27 * ((t1c5 * l.fa1e) / t1c6)), ((l.fb2b * t1c7) + (l.fb27 * ((t1c5 * l.fa19) / t1c6))), ((l.fb2c * t1c7) + (l.fb27 * ((t1c5 * l.fa1a) / t1c6))), );}
        if (l.f1eb3 != 0.0) {let t1c9: f64 = (l.fbce - l.fa9b);let t1ca: f64 = (t1c9 / l.f9c5);(l.fa20, l.fa2a, l.fa2b, l.fa2c, l.fa2d, l.fa28, l.fa29, ) = (t1ca, ((l.fbd1 - l.faa1) / l.f9c5), ((-l.faa2) / l.f9c5), ((((-l.faa3) * l.f9c5) - (t1c9 * l.f9c6)) / (l.f9c5 * l.f9c5)), ((l.fbd2 - l.faa4) / l.f9c5), ((l.fbcf - l.fa9f) / l.f9c5), ((l.fbd0 - l.faa0) / l.f9c5), );}
        let t1cb: f64 = if l.fa20 > 50.0 { 1.0 } else { 0.0 };l.f1f6d = t1cb;
        if ((l.f1eb3 != 0.0) && (l.f1f6d != 0.0)) {(l.fa4d, l.fa57, l.fa58, l.fa59, l.fa5a, l.fa55, l.fa56, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t1cc: f64 = (-50.0);let t1cd: f64 = if l.fa20 < t1cc { 1.0 } else { 0.0 };l.f1f79 = t1cd;
        if (((l.f1eb3 != 0.0) && (l.f1f6d == 0.0)) && (l.f1f79 != 0.0)) {(l.fa4d, l.fa57, l.fa58, l.fa59, l.fa5a, l.fa55, l.fa56, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f1eb3 != 0.0) && (l.f1f6d == 0.0)) && (l.f1f79 == 0.0)) {let t1d7: f64 = (l.fa20).exp();let t1d8: f64 = (1.0 + t1d7);let t1d9: f64 = (1.0 / t1d8);(l.fa4d, l.fa57, l.fa58, l.fa59, l.fa5a, l.fa55, l.fa56, ) = (t1d9, (-((t1d7 * l.fa2a) / (t1d8 * t1d8))), (-((t1d7 * l.fa2b) / (t1d8 * t1d8))), (-((t1d7 * l.fa2c) / (t1d8 * t1d8))), (-((t1d7 * l.fa2d) / (t1d8 * t1d8))), (-((t1d7 * l.fa28) / (t1d8 * t1d8))), (-((t1d7 * l.fa29) / (t1d8 * t1d8))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1eb3 != 0.0) {let t1da: f64 = (l.fbd4 - l.fbb6);let t1db: f64 = (p.p51 * 0.1);let t1dc: f64 = (t1db * l.f9c5);let t1dd: f64 = (t1dc * l.fa4d);let t1de: f64 = (l.fbec - t1dd);let t1df: f64 = (t1da - t1de);let t1e0: f64 = (t1df / l.fb66);(l.f9fc, l.fa06, l.fa07, l.fa08, l.fa09, l.fa04, l.fa05, ) = (t1e0, (((l.fbd6 - l.fbc0) - (-(t1dc * l.fa57))) / l.fb66), (((-l.fbc1) - (-(t1dc * l.fa58))) / l.fb66), (((((-l.fbc2) - (l.fbef - (((t1db * l.f9c6) * l.fa4d) + (t1dc * l.fa59)))) * l.fb66) - (t1df * l.fb6c)) / (l.fb66 * l.fb66)), (((l.fbd7 - l.fbc3) - (-(t1dc * l.fa5a))) / l.fb66), (((((l.fbd5 - l.fbbe) - (l.fbed - (t1dc * l.fa55))) * l.fb66) - (t1df * l.fb6a)) / (l.fb66 * l.fb66)), (((((-l.fbbf) - (l.fbee - (t1dc * l.fa56))) * l.fb66) - (t1df * l.fb6b)) / (l.fb66 * l.fb66)), );}
        let t1e1: f64 = if l.f9fc > 50.0 { 1.0 } else { 0.0 };l.f1f7d = t1e1;
        if ((l.f1eb3 != 0.0) && (l.f1f7d != 0.0)) {let t1e2: f64 = (l.fb27 * l.f9fc);(l.faf3, l.fafd, l.fafe, l.faff, l.fb00, l.fafb, l.fafc, ) = (t1e2, (l.fb27 * l.fa06), (l.fb27 * l.fa07), ((l.fb2d * l.f9fc) + (l.fb27 * l.fa08)), (l.fb27 * l.fa09), ((l.fb2b * l.f9fc) + (l.fb27 * l.fa04)), ((l.fb2c * l.f9fc) + (l.fb27 * l.fa05)), );}
        let t1e3: f64 = (-50.0);let t1e4: f64 = if l.f9fc < t1e3 { 1.0 } else { 0.0 };l.f1f85 = t1e4;
        if (((l.f1eb3 != 0.0) && (l.f1f7d == 0.0)) && (l.f1f85 != 0.0)) {let t1e5: f64 = (l.f9fc).exp();let t1e6: f64 = (l.fb27 * t1e5);(l.faf3, l.fafd, l.fafe, l.faff, l.fb00, l.fafb, l.fafc, ) = (t1e6, (l.fb27 * (t1e5 * l.fa06)), (l.fb27 * (t1e5 * l.fa07)), ((l.fb2d * t1e5) + (l.fb27 * (t1e5 * l.fa08))), (l.fb27 * (t1e5 * l.fa09)), ((l.fb2b * t1e5) + (l.fb27 * (t1e5 * l.fa04))), ((l.fb2c * t1e5) + (l.fb27 * (t1e5 * l.fa05))), );}
        if (((l.f1eb3 != 0.0) && (l.f1f7d == 0.0)) && (l.f1f85 == 0.0)) {let t1e7: f64 = (l.f9fc).exp();let t1e8: f64 = (1.0 + t1e7);let t1e9: f64 = (t1e8).ln();let t1ea: f64 = (l.fb27 * t1e9);(l.faf3, l.fafd, l.fafe, l.faff, l.fb00, l.fafb, l.fafc, ) = (t1ea, (l.fb27 * ((t1e7 * l.fa06) / t1e8)), (l.fb27 * ((t1e7 * l.fa07) / t1e8)), ((l.fb2d * t1e9) + (l.fb27 * ((t1e7 * l.fa08) / t1e8))), (l.fb27 * ((t1e7 * l.fa09) / t1e8)), ((l.fb2b * t1e9) + (l.fb27 * ((t1e7 * l.fa04) / t1e8))), ((l.fb2c * t1e9) + (l.fb27 * ((t1e7 * l.fa05) / t1e8))), );}
        if (l.f1eb3 != 0.0) {let t1eb: f64 = (l.fb09 - l.faf3);let t1ec: f64 = (t1eb / l.f9d1);(l.fbaa, l.fbad, l.fbae, l.fbaf, l.fbb0, l.fbab, l.fbac, ) = (t1ec, ((l.fb13 - l.fafd) / l.f9d1), ((l.fb14 - l.fafe) / l.f9d1), ((((l.fb15 - l.faff) * l.f9d1) - (t1eb * l.f9d2)) / (l.f9d1 * l.f9d1)), ((l.fb16 - l.fb00) / l.f9d1), ((l.fb11 - l.fafb) / l.f9d1), ((l.fb12 - l.fafc) / l.f9d1), );let t1ed: f64 = (l.fbaa / l.fb79);(l.fa9b, l.faa1, l.faa2, l.faa3, l.faa4, l.fa9f, l.faa0, ) = (t1ed, (((l.fbad * l.fb79) - (l.fbaa * l.fb8b)) / (l.fb79 * l.fb79)), (((l.fbae * l.fb79) - (l.fbaa * l.fb8c)) / (l.fb79 * l.fb79)), (((l.fbaf * l.fb79) - (l.fbaa * l.fb8d)) / (l.fb79 * l.fb79)), (((l.fbb0 * l.fb79) - (l.fbaa * l.fb8e)) / (l.fb79 * l.fb79)), (((l.fbab * l.fb79) - (l.fbaa * l.fb89)) / (l.fb79 * l.fb79)), (((l.fbac * l.fb79) - (l.fbaa * l.fb8a)) / (l.fb79 * l.fb79)), );}
        if (l.f1eb3 != 0.0) {
            let (t205, t208, t209, t20a, t20b, t206, t207,) = {
    if (p.p52 != 0.0) {
        let t1f7: f64 = (0.001 / p.p53);let t1f8: f64 = (t1f7 * l.fa9b);let t1f9: f64 = (t1f8).tanh();let t1fa: f64 = (l.fa9b * t1f9);
        (t1fa, ((l.faa1 * t1f9) + (l.fa9b * ((t1f7 * l.faa1) / ((t1f8).cosh() * (t1f8).cosh())))), ((l.faa2 * t1f9) + (l.fa9b * ((t1f7 * l.faa2) / ((t1f8).cosh() * (t1f8).cosh())))), ((l.faa3 * t1f9) + (l.fa9b * ((t1f7 * l.faa3) / ((t1f8).cosh() * (t1f8).cosh())))), ((l.faa4 * t1f9) + (l.fa9b * ((t1f7 * l.faa4) / ((t1f8).cosh() * (t1f8).cosh())))), ((l.fa9f * t1f9) + (l.fa9b * ((t1f7 * l.fa9f) / ((t1f8).cosh() * (t1f8).cosh())))), ((l.faa0 * t1f9) + (l.fa9b * ((t1f7 * l.faa0) / ((t1f8).cosh() * (t1f8).cosh())))),)
    } else {
        let (t1fe, t201, t202, t203, t204, t1ff, t200,) = {
            if (p.p52 == 0.0) {
                let t1fb: f64 = (l.fa9b * l.fa9b);let t1fc: f64 = (t1fb + p.p53);let t1fd: f64 = (t1fc).sqrt();
                (t1fd, (((l.faa1 * l.fa9b) + (l.fa9b * l.faa1)) / (2.0 * t1fd)), (((l.faa2 * l.fa9b) + (l.fa9b * l.faa2)) / (2.0 * t1fd)), (((l.faa3 * l.fa9b) + (l.fa9b * l.faa3)) / (2.0 * t1fd)), (((l.faa4 * l.fa9b) + (l.fa9b * l.faa4)) / (2.0 * t1fd)), (((l.fa9f * l.fa9b) + (l.fa9b * l.fa9f)) / (2.0 * t1fd)), (((l.faa0 * l.fa9b) + (l.fa9b * l.faa0)) / (2.0 * t1fd)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t1fe, t201, t202, t203, t204, t1ff, t200,)
    }
};
            let t20c: f64 = (t205).powf(l.f9c9);let t20d: f64 = (1.0 + t20c);let t20e: f64 = (1.0 / l.f9c9);let t20f: f64 = (t20d).powf(t20e);let t210: f64 = (l.fa9b / t20f);
            (l.fa6b, l.fa6e, l.fa6f, l.fa70, l.fa71, l.fa6c, l.fa6d, ) = (t210, (((l.faa1 * t20f) - (l.fa9b * if 0.0 == 0.0 && ((t20e) as f64).is_finite() && ((t20e) as f64).fract() == 0.0 { if t20e == 0.0 { 0.0 } else { (t20e * ((t20d).powf(t20e - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t208)) } } else { (t20c * (l.f9c9 * (t208 / t205))) })) } } else { (t20f * (t20e * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t208)) } } else { (t20c * (l.f9c9 * (t208 / t205))) } / t20d))) })) / (t20f * t20f)), (((l.faa2 * t20f) - (l.fa9b * if 0.0 == 0.0 && ((t20e) as f64).is_finite() && ((t20e) as f64).fract() == 0.0 { if t20e == 0.0 { 0.0 } else { (t20e * ((t20d).powf(t20e - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t209)) } } else { (t20c * (l.f9c9 * (t209 / t205))) })) } } else { (t20f * (t20e * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t209)) } } else { (t20c * (l.f9c9 * (t209 / t205))) } / t20d))) })) / (t20f * t20f)), (((l.faa3 * t20f) - (l.fa9b * if 0.0 == 0.0 && ((t20e) as f64).is_finite() && ((t20e) as f64).fract() == 0.0 { if t20e == 0.0 { 0.0 } else { (t20e * ((t20d).powf(t20e - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t20a)) } } else { (t20c * (l.f9c9 * (t20a / t205))) })) } } else { (t20f * (t20e * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t20a)) } } else { (t20c * (l.f9c9 * (t20a / t205))) } / t20d))) })) / (t20f * t20f)), (((l.faa4 * t20f) - (l.fa9b * if 0.0 == 0.0 && ((t20e) as f64).is_finite() && ((t20e) as f64).fract() == 0.0 { if t20e == 0.0 { 0.0 } else { (t20e * ((t20d).powf(t20e - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t20b)) } } else { (t20c * (l.f9c9 * (t20b / t205))) })) } } else { (t20f * (t20e * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t20b)) } } else { (t20c * (l.f9c9 * (t20b / t205))) } / t20d))) })) / (t20f * t20f)), (((l.fa9f * t20f) - (l.fa9b * if 0.0 == 0.0 && ((t20e) as f64).is_finite() && ((t20e) as f64).fract() == 0.0 { if t20e == 0.0 { 0.0 } else { (t20e * ((t20d).powf(t20e - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t206)) } } else { (t20c * (l.f9c9 * (t206 / t205))) })) } } else { (t20f * (t20e * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t206)) } } else { (t20c * (l.f9c9 * (t206 / t205))) } / t20d))) })) / (t20f * t20f)), (((l.faa0 * t20f) - (l.fa9b * if 0.0 == 0.0 && ((t20e) as f64).is_finite() && ((t20e) as f64).fract() == 0.0 { if t20e == 0.0 { 0.0 } else { (t20e * ((t20d).powf(t20e - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t207)) } } else { (t20c * (l.f9c9 * (t207 / t205))) })) } } else { (t20f * (t20e * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t205).powf(l.f9c9 - 1.0) * t207)) } } else { (t20c * (l.f9c9 * (t207 / t205))) } / t20d))) })) / (t20f * t20f)), );
        }
        if (l.f1eb3 != 0.0) {let t211: f64 = (l.fc05 * l.fa6b);(l.fbc5, l.fbca, l.fbcb, l.fbcc, l.fbcd, l.fbc8, l.fbc9, ) = (t211, ((l.fc08 * l.fa6b) + (l.fc05 * l.fa6e)), ((l.fc09 * l.fa6b) + (l.fc05 * l.fa6f)), ((l.fc0a * l.fa6b) + (l.fc05 * l.fa70)), ((l.fc0b * l.fa6b) + (l.fc05 * l.fa71)), ((l.fc06 * l.fa6b) + (l.fc05 * l.fa6c)), ((l.fc07 * l.fa6b) + (l.fc05 * l.fa6d)), );let t212: f64 = (l.fb6e * l.fc0e);let t213: f64 = (t212 * l.fab0);let t214: f64 = (t213 * 0.5);let t215: f64 = (l.fb09 + l.faf3);let t216: f64 = (t214 * t215);let t217: f64 = (t216 * l.fbc5);let t218: f64 = (t217 * l.fb64);(l.fa81, l.fa84, l.fa85, l.fa86, l.fa87, l.fa82, l.fa83, ) = (t218, ((((t214 * (l.fb13 + l.fafd)) * l.fbc5) + (t216 * l.fbca)) * l.fb64), ((((t214 * (l.fb14 + l.fafe)) * l.fbc5) + (t216 * l.fbcb)) * l.fb64), ((((t214 * (l.fb15 + l.faff)) * l.fbc5) + (t216 * l.fbcc)) * l.fb64), ((((t214 * (l.fb16 + l.fb00)) * l.fbc5) + (t216 * l.fbcd)) * l.fb64), ((((t214 * (l.fb11 + l.fafb)) * l.fbc5) + (t216 * l.fbc8)) * l.fb64), ((((t214 * (l.fb12 + l.fafc)) * l.fbc5) + (t216 * l.fbc9)) * l.fb64), );let t219: f64 = (2.302585092994046 * l.fab2);let t21a: f64 = (l.fb5a / t219);(l.faa7, l.faa8, ) = (t21a, (-((l.fb5a * (2.302585092994046 * l.fab3)) / (t219 * t219))), );let t21b: f64 = (2.0 * l.faa7);let t21c: f64 = (t21b * l.fab2);(l.fb67, l.fb68, ) = (t21c, (((2.0 * l.faa8) * l.fab2) + (t21b * l.fab3)), );let t21d: f64 = (l.f9d1 * l.fb67);(l.fb28, l.fb29, ) = (t21d, ((l.f9d2 * l.fb67) + (l.f9d1 * l.fb68)), );let t21e: f64 = (p.p51 * l.f9c5);let t21f: f64 = (t21e / 2.0);let t220: f64 = (l.fbf5 - t21f);(l.fa9c, l.fa9d, ) = (t220, (l.fbf6 - ((p.p51 * l.f9c6) / 2.0)), );}
        if (l.f1eb3 != 0.0) {
            let (t237, t23a, t23b, t238, t239,) = {
    if (p.p52 != 0.0) {
        let t221: f64 = (l.fbd4 + l.fbce);let t222: f64 = (l.fbd4 - l.fbce);let t223: f64 = (0.001 / p.p53);let t224: f64 = (l.fbd4 - l.fbce);let t225: f64 = (t223 * t224);let t226: f64 = (t225).tanh();let t227: f64 = (t222 * t226);let t228: f64 = (t221 + t227);let t229: f64 = (0.5 * t228);
        (t229, (0.5 * ((l.fbd6 + l.fbd1) + (((l.fbd6 - l.fbd1) * t226) + (t222 * ((t223 * (l.fbd6 - l.fbd1)) / ((t225).cosh() * (t225).cosh())))))), (0.5 * ((l.fbd7 + l.fbd2) + (((l.fbd7 - l.fbd2) * t226) + (t222 * ((t223 * (l.fbd7 - l.fbd2)) / ((t225).cosh() * (t225).cosh())))))), (0.5 * ((l.fbd5 + l.fbcf) + (((l.fbd5 - l.fbcf) * t226) + (t222 * ((t223 * (l.fbd5 - l.fbcf)) / ((t225).cosh() * (t225).cosh())))))), (0.5 * (l.fbd0 + (((-l.fbd0) * t226) + (t222 * ((t223 * (-l.fbd0)) / ((t225).cosh() * (t225).cosh())))))),)
    } else {
        let (t232, t235, t236, t233, t234,) = {
            if (p.p52 == 0.0) {
                let t22a: f64 = (l.fbd4 + l.fbce);let t22b: f64 = (l.fbd4 - l.fbce);let t22c: f64 = (l.fbd4 - l.fbce);let t22d: f64 = (t22b * t22c);let t22e: f64 = (t22d + p.p53);let t22f: f64 = (t22e).sqrt();let t230: f64 = (t22a + t22f);let t231: f64 = (0.5 * t230);
                (t231, (0.5 * ((l.fbd6 + l.fbd1) + ((((l.fbd6 - l.fbd1) * t22c) + (t22b * (l.fbd6 - l.fbd1))) / (2.0 * t22f)))), (0.5 * ((l.fbd7 + l.fbd2) + ((((l.fbd7 - l.fbd2) * t22c) + (t22b * (l.fbd7 - l.fbd2))) / (2.0 * t22f)))), (0.5 * ((l.fbd5 + l.fbcf) + ((((l.fbd5 - l.fbcf) * t22c) + (t22b * (l.fbd5 - l.fbcf))) / (2.0 * t22f)))), (0.5 * (l.fbd0 + ((((-l.fbd0) * t22c) + (t22b * (-l.fbd0))) / (2.0 * t22f)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t232, t235, t236, t233, t234,)
    }
};
            let t23c: f64 = (t237 - l.fa9c);let t23d: f64 = (t23c / l.f9c5);(l.fa21, l.fa24, l.fa25, l.fa26, l.fa22, l.fa23, ) = (t23d, (t23a / l.f9c5), ((((-l.fa9d) * l.f9c5) - (t23c * l.f9c6)) / (l.f9c5 * l.f9c5)), (t23b / l.f9c5), (t238 / l.f9c5), (t239 / l.f9c5), );
        }
        let t23e: f64 = if l.fa21 > 50.0 { 1.0 } else { 0.0 };l.f1f97 = t23e;
        if ((l.f1eb3 != 0.0) && (l.f1f97 != 0.0)) {(l.fa3f, l.fa42, l.fa43, l.fa44, l.fa40, l.fa41, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t248: f64 = (-50.0);let t249: f64 = if l.fa21 < t248 { 1.0 } else { 0.0 };l.f1faf = t249;
        if (((l.f1eb3 != 0.0) && (l.f1f97 == 0.0)) && (l.f1faf != 0.0)) {(l.fa3f, l.fa42, l.fa43, l.fa44, l.fa40, l.fa41, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f1eb3 != 0.0) && (l.f1f97 == 0.0)) && (l.f1faf == 0.0)) {let t24a: f64 = (l.fa21).exp();let t24b: f64 = (1.0 + t24a);let t24c: f64 = (1.0 / t24b);(l.fa3f, l.fa42, l.fa43, l.fa44, l.fa40, l.fa41, ) = (t24c, (-((t24a * l.fa24) / (t24b * t24b))), (-((t24a * l.fa25) / (t24b * t24b))), (-((t24a * l.fa26) / (t24b * t24b))), (-((t24a * l.fa22) / (t24b * t24b))), (-((t24a * l.fa23) / (t24b * t24b))), );}
        if (l.f1eb3 != 0.0) {
            let (t263, t266, t267, t264, t265,) = {
    if (p.p52 != 0.0) {
        let t24d: f64 = (l.fbd4 + l.fbce);let t24e: f64 = (l.fbd4 - l.fbce);let t24f: f64 = (0.001 / p.p53);let t250: f64 = (l.fbd4 - l.fbce);let t251: f64 = (t24f * t250);let t252: f64 = (t251).tanh();let t253: f64 = (t24e * t252);let t254: f64 = (t24d + t253);let t255: f64 = (0.5 * t254);
        (t255, (0.5 * ((l.fbd6 + l.fbd1) + (((l.fbd6 - l.fbd1) * t252) + (t24e * ((t24f * (l.fbd6 - l.fbd1)) / ((t251).cosh() * (t251).cosh())))))), (0.5 * ((l.fbd7 + l.fbd2) + (((l.fbd7 - l.fbd2) * t252) + (t24e * ((t24f * (l.fbd7 - l.fbd2)) / ((t251).cosh() * (t251).cosh())))))), (0.5 * ((l.fbd5 + l.fbcf) + (((l.fbd5 - l.fbcf) * t252) + (t24e * ((t24f * (l.fbd5 - l.fbcf)) / ((t251).cosh() * (t251).cosh())))))), (0.5 * (l.fbd0 + (((-l.fbd0) * t252) + (t24e * ((t24f * (-l.fbd0)) / ((t251).cosh() * (t251).cosh())))))),)
    } else {
        let (t25e, t261, t262, t25f, t260,) = {
            if (p.p52 == 0.0) {
                let t256: f64 = (l.fbd4 + l.fbce);let t257: f64 = (l.fbd4 - l.fbce);let t258: f64 = (l.fbd4 - l.fbce);let t259: f64 = (t257 * t258);let t25a: f64 = (t259 + p.p53);let t25b: f64 = (t25a).sqrt();let t25c: f64 = (t256 + t25b);let t25d: f64 = (0.5 * t25c);
                (t25d, (0.5 * ((l.fbd6 + l.fbd1) + ((((l.fbd6 - l.fbd1) * t258) + (t257 * (l.fbd6 - l.fbd1))) / (2.0 * t25b)))), (0.5 * ((l.fbd7 + l.fbd2) + ((((l.fbd7 - l.fbd2) * t258) + (t257 * (l.fbd7 - l.fbd2))) / (2.0 * t25b)))), (0.5 * ((l.fbd5 + l.fbcf) + ((((l.fbd5 - l.fbcf) * t258) + (t257 * (l.fbd5 - l.fbcf))) / (2.0 * t25b)))), (0.5 * (l.fbd0 + ((((-l.fbd0) * t258) + (t257 * (-l.fbd0))) / (2.0 * t25b)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t25e, t261, t262, t25f, t260,)
    }
};
            let t268: f64 = (p.p51 * 0.1);let t269: f64 = (t268 * l.f9c5);let t26a: f64 = (t269 * l.fa3f);let t26b: f64 = (l.fbf5 - t26a);let t26c: f64 = (t263 - t26b);let t26d: f64 = (t26c / l.fb67);(l.f9e3, l.f9e6, l.f9e7, l.f9e8, l.f9e4, l.f9e5, ) = (t26d, ((t266 - (-(t269 * l.fa42))) / l.fb67), ((((-(l.fbf6 - (((t268 * l.f9c6) * l.fa3f) + (t269 * l.fa43)))) * l.fb67) - (t26c * l.fb68)) / (l.fb67 * l.fb67)), ((t267 - (-(t269 * l.fa44))) / l.fb67), ((t264 - (-(t269 * l.fa40))) / l.fb67), ((t265 - (-(t269 * l.fa41))) / l.fb67), );
        }
        let t26e: f64 = if l.f9e3 > 50.0 { 1.0 } else { 0.0 };l.f1fc5 = t26e;
        if ((l.f1eb3 != 0.0) && (l.f1fc5 != 0.0)) {let t26f: f64 = (l.fb28 * l.f9e3);(l.fb19, l.fb1c, l.fb1d, l.fb1e, l.fb1a, l.fb1b, ) = (t26f, (l.fb28 * l.f9e6), ((l.fb29 * l.f9e3) + (l.fb28 * l.f9e7)), (l.fb28 * l.f9e8), (l.fb28 * l.f9e4), (l.fb28 * l.f9e5), );}
        let t270: f64 = (-50.0);let t271: f64 = if l.f9e3 < t270 { 1.0 } else { 0.0 };l.f1fd7 = t271;
        if (((l.f1eb3 != 0.0) && (l.f1fc5 == 0.0)) && (l.f1fd7 != 0.0)) {let t272: f64 = (l.f9e3).exp();let t273: f64 = (l.fb28 * t272);(l.fb19, l.fb1c, l.fb1d, l.fb1e, l.fb1a, l.fb1b, ) = (t273, (l.fb28 * (t272 * l.f9e6)), ((l.fb29 * t272) + (l.fb28 * (t272 * l.f9e7))), (l.fb28 * (t272 * l.f9e8)), (l.fb28 * (t272 * l.f9e4)), (l.fb28 * (t272 * l.f9e5)), );}
        if (((l.f1eb3 != 0.0) && (l.f1fc5 == 0.0)) && (l.f1fd7 == 0.0)) {let t274: f64 = (l.f9e3).exp();let t275: f64 = (1.0 + t274);let t276: f64 = (t275).ln();let t277: f64 = (l.fb28 * t276);(l.fb19, l.fb1c, l.fb1d, l.fb1e, l.fb1a, l.fb1b, ) = (t277, (l.fb28 * ((t274 * l.f9e6) / t275)), ((l.fb29 * t276) + (l.fb28 * ((t274 * l.f9e7) / t275))), (l.fb28 * ((t274 * l.f9e8) / t275)), (l.fb28 * ((t274 * l.f9e4) / t275)), (l.fb28 * ((t274 * l.f9e5) / t275)), );}
        if (l.f1eb3 != 0.0) {let t278: f64 = (l.fa8e / l.fb5f);(l.fa91, l.fa92, ) = (t278, (-((l.fa8e * l.fb60) / (l.fb5f * l.fb5f))), );let t282: f64 = (l.fc0c * l.fb62);let t283: f64 = (1.0 + t282);let t284: f64 = (l.fc0c * l.fb5c);let t285: f64 = (1.0 + t284);let t286: f64 = (t283 / t285);let t287: f64 = (l.fbc6 * t286);(l.fbfb, l.fbfc, ) = (t287, (l.fbc6 * (-((t283 * (l.fc0c * l.fb5d)) / (t285 * t285)))), );let t288: f64 = (l.fbfb * l.fa8a);let t289: f64 = (t288 / l.fa91);(l.fb91, l.fb92, ) = (t289, ((((l.fbfc * l.fa8a) * l.fa91) - (t288 * l.fa92)) / (l.fa91 * l.fa91)), );let t28a: f64 = (2.0 * l.fb19);let t28b: f64 = (t28a / l.f9d1);let t28c: f64 = (t28b / l.fb91);let t28d: f64 = (1.0 + t28c);let t28e: f64 = (t28d).sqrt();let t28f: f64 = (l.fb91 * t28e);let t290: f64 = (t28f - l.fb91);(l.fb95, l.fb98, l.fb99, l.fb9a, l.fb96, l.fb97, ) = (t290, (l.fb91 * ((((2.0 * l.fb1c) / l.f9d1) / l.fb91) / (2.0 * t28e))), (((l.fb92 * t28e) + (l.fb91 * ((((((((2.0 * l.fb1d) * l.f9d1) - (t28a * l.f9d2)) / (l.f9d1 * l.f9d1)) * l.fb91) - (t28b * l.fb92)) / (l.fb91 * l.fb91)) / (2.0 * t28e)))) - l.fb92), (l.fb91 * ((((2.0 * l.fb1e) / l.f9d1) / l.fb91) / (2.0 * t28e))), (l.fb91 * ((((2.0 * l.fb1a) / l.f9d1) / l.fb91) / (2.0 * t28e))), (l.fb91 * ((((2.0 * l.fb1b) / l.f9d1) / l.fb91) / (2.0 * t28e))), );let t291: f64 = (1.0 - l.fa3f);let t292: f64 = (l.fb95 * t291);let t293: f64 = (l.fb67 * l.fa3f);let t294: f64 = (t292 + t293);(l.fb7b, l.fb7e, l.fb7f, l.fb80, l.fb7c, l.fb7d, ) = (t294, (((l.fb98 * t291) + (l.fb95 * (-l.fa42))) + (l.fb67 * l.fa42)), (((l.fb99 * t291) + (l.fb95 * (-l.fa43))) + ((l.fb68 * l.fa3f) + (l.fb67 * l.fa43))), (((l.fb9a * t291) + (l.fb95 * (-l.fa44))) + (l.fb67 * l.fa44)), (((l.fb96 * t291) + (l.fb95 * (-l.fa40))) + (l.fb67 * l.fa40)), (((l.fb97 * t291) + (l.fb95 * (-l.fa41))) + (l.fb67 * l.fa41)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f1eb3 != 0.0) {
            let (t2b2, t2b5, t2b6, t2b7, t2b3, t2b4,) = {
    if (p.p52 != 0.0) {
        let t295: f64 = (l.fbb2 / l.fb7b);let t296: f64 = t295;let t297: f64 = (l.fbb2 / l.fb7b);let t298: f64 = (-t297);let t299: f64 = (0.001 / p.p53);let t29a: f64 = (l.fbb2 / l.fb7b);let t29b: f64 = (-t29a);let t29c: f64 = (t299 * t29b);let t29d: f64 = (t29c).tanh();let t29e: f64 = (t298 * t29d);let t29f: f64 = (t296 + t29e);let t2a0: f64 = (0.5 * t29f);
        (t2a0, (0.5 * ((-((l.fbb2 * l.fb7e) / (l.fb7b * l.fb7b))) + (((-(-((l.fbb2 * l.fb7e) / (l.fb7b * l.fb7b)))) * t29d) + (t298 * ((t299 * (-(-((l.fbb2 * l.fb7e) / (l.fb7b * l.fb7b))))) / ((t29c).cosh() * (t29c).cosh())))))), (0.5 * ((-((l.fbb2 * l.fb7f) / (l.fb7b * l.fb7b))) + (((-(-((l.fbb2 * l.fb7f) / (l.fb7b * l.fb7b)))) * t29d) + (t298 * ((t299 * (-(-((l.fbb2 * l.fb7f) / (l.fb7b * l.fb7b))))) / ((t29c).cosh() * (t29c).cosh())))))), (0.5 * ((-((l.fbb2 * l.fb80) / (l.fb7b * l.fb7b))) + (((-(-((l.fbb2 * l.fb80) / (l.fb7b * l.fb7b)))) * t29d) + (t298 * ((t299 * (-(-((l.fbb2 * l.fb80) / (l.fb7b * l.fb7b))))) / ((t29c).cosh() * (t29c).cosh())))))), (0.5 * ((((l.fbb3 * l.fb7b) - (l.fbb2 * l.fb7c)) / (l.fb7b * l.fb7b)) + (((-(((l.fbb3 * l.fb7b) - (l.fbb2 * l.fb7c)) / (l.fb7b * l.fb7b))) * t29d) + (t298 * ((t299 * (-(((l.fbb3 * l.fb7b) - (l.fbb2 * l.fb7c)) / (l.fb7b * l.fb7b)))) / ((t29c).cosh() * (t29c).cosh())))))), (0.5 * ((((l.fbb4 * l.fb7b) - (l.fbb2 * l.fb7d)) / (l.fb7b * l.fb7b)) + (((-(((l.fbb4 * l.fb7b) - (l.fbb2 * l.fb7d)) / (l.fb7b * l.fb7b))) * t29d) + (t298 * ((t299 * (-(((l.fbb4 * l.fb7b) - (l.fbb2 * l.fb7d)) / (l.fb7b * l.fb7b)))) / ((t29c).cosh() * (t29c).cosh())))))),)
    } else {
        let (t2ac, t2af, t2b0, t2b1, t2ad, t2ae,) = {
            if (p.p52 == 0.0) {
                let t2a1: f64 = (l.fbb2 / l.fb7b);let t2a2: f64 = t2a1;let t2a3: f64 = (l.fbb2 / l.fb7b);let t2a4: f64 = (-t2a3);let t2a5: f64 = (l.fbb2 / l.fb7b);let t2a6: f64 = (-t2a5);let t2a7: f64 = (t2a4 * t2a6);let t2a8: f64 = (t2a7 + p.p53);let t2a9: f64 = (t2a8).sqrt();let t2aa: f64 = (t2a2 + t2a9);let t2ab: f64 = (0.5 * t2aa);
                (t2ab, (0.5 * ((-((l.fbb2 * l.fb7e) / (l.fb7b * l.fb7b))) + ((((-(-((l.fbb2 * l.fb7e) / (l.fb7b * l.fb7b)))) * t2a6) + (t2a4 * (-(-((l.fbb2 * l.fb7e) / (l.fb7b * l.fb7b)))))) / (2.0 * t2a9)))), (0.5 * ((-((l.fbb2 * l.fb7f) / (l.fb7b * l.fb7b))) + ((((-(-((l.fbb2 * l.fb7f) / (l.fb7b * l.fb7b)))) * t2a6) + (t2a4 * (-(-((l.fbb2 * l.fb7f) / (l.fb7b * l.fb7b)))))) / (2.0 * t2a9)))), (0.5 * ((-((l.fbb2 * l.fb80) / (l.fb7b * l.fb7b))) + ((((-(-((l.fbb2 * l.fb80) / (l.fb7b * l.fb7b)))) * t2a6) + (t2a4 * (-(-((l.fbb2 * l.fb80) / (l.fb7b * l.fb7b)))))) / (2.0 * t2a9)))), (0.5 * ((((l.fbb3 * l.fb7b) - (l.fbb2 * l.fb7c)) / (l.fb7b * l.fb7b)) + ((((-(((l.fbb3 * l.fb7b) - (l.fbb2 * l.fb7c)) / (l.fb7b * l.fb7b))) * t2a6) + (t2a4 * (-(((l.fbb3 * l.fb7b) - (l.fbb2 * l.fb7c)) / (l.fb7b * l.fb7b))))) / (2.0 * t2a9)))), (0.5 * ((((l.fbb4 * l.fb7b) - (l.fbb2 * l.fb7d)) / (l.fb7b * l.fb7b)) + ((((-(((l.fbb4 * l.fb7b) - (l.fbb2 * l.fb7d)) / (l.fb7b * l.fb7b))) * t2a6) + (t2a4 * (-(((l.fbb4 * l.fb7b) - (l.fbb2 * l.fb7d)) / (l.fb7b * l.fb7b))))) / (2.0 * t2a9)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2ac, t2af, t2b0, t2b1, t2ad, t2ae,)
    }
};
            let t2b8: f64 = (t2b2).powf(l.f9c9);let t2b9: f64 = (1.0 + t2b8);let t2ba: f64 = (1.0 / l.f9c9);let t2bb: f64 = (t2b9).powf(t2ba);let t2bc: f64 = (1.0 / t2bb);
            (l.fa73, l.fa76, l.fa77, l.fa78, l.fa74, l.fa75, ) = (t2bc, (-(if 0.0 == 0.0 && ((t2ba) as f64).is_finite() && ((t2ba) as f64).fract() == 0.0 { if t2ba == 0.0 { 0.0 } else { (t2ba * ((t2b9).powf(t2ba - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b5)) } } else { (t2b8 * (l.f9c9 * (t2b5 / t2b2))) })) } } else { (t2bb * (t2ba * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b5)) } } else { (t2b8 * (l.f9c9 * (t2b5 / t2b2))) } / t2b9))) } / (t2bb * t2bb))), (-(if 0.0 == 0.0 && ((t2ba) as f64).is_finite() && ((t2ba) as f64).fract() == 0.0 { if t2ba == 0.0 { 0.0 } else { (t2ba * ((t2b9).powf(t2ba - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b6)) } } else { (t2b8 * (l.f9c9 * (t2b6 / t2b2))) })) } } else { (t2bb * (t2ba * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b6)) } } else { (t2b8 * (l.f9c9 * (t2b6 / t2b2))) } / t2b9))) } / (t2bb * t2bb))), (-(if 0.0 == 0.0 && ((t2ba) as f64).is_finite() && ((t2ba) as f64).fract() == 0.0 { if t2ba == 0.0 { 0.0 } else { (t2ba * ((t2b9).powf(t2ba - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b7)) } } else { (t2b8 * (l.f9c9 * (t2b7 / t2b2))) })) } } else { (t2bb * (t2ba * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b7)) } } else { (t2b8 * (l.f9c9 * (t2b7 / t2b2))) } / t2b9))) } / (t2bb * t2bb))), (-(if 0.0 == 0.0 && ((t2ba) as f64).is_finite() && ((t2ba) as f64).fract() == 0.0 { if t2ba == 0.0 { 0.0 } else { (t2ba * ((t2b9).powf(t2ba - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b3)) } } else { (t2b8 * (l.f9c9 * (t2b3 / t2b2))) })) } } else { (t2bb * (t2ba * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b3)) } } else { (t2b8 * (l.f9c9 * (t2b3 / t2b2))) } / t2b9))) } / (t2bb * t2bb))), (-(if 0.0 == 0.0 && ((t2ba) as f64).is_finite() && ((t2ba) as f64).fract() == 0.0 { if t2ba == 0.0 { 0.0 } else { (t2ba * ((t2b9).powf(t2ba - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b4)) } } else { (t2b8 * (l.f9c9 * (t2b4 / t2b2))) })) } } else { (t2bb * (t2ba * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2b2).powf(l.f9c9 - 1.0) * t2b4)) } } else { (t2b8 * (l.f9c9 * (t2b4 / t2b2))) } / t2b9))) } / (t2bb * t2bb))), );
        }
        if (l.f1eb3 != 0.0) {let t2bd: f64 = (l.fbb2 * l.fa73);(l.fbb7, l.fbba, l.fbbb, l.fbbc, l.fbb8, l.fbb9, ) = (t2bd, (l.fbb2 * l.fa76), (l.fbb2 * l.fa77), (l.fbb2 * l.fa78), ((l.fbb3 * l.fa73) + (l.fbb2 * l.fa74)), ((l.fbb4 * l.fa73) + (l.fbb2 * l.fa75)), );}
        if (l.f1eb3 != 0.0) {
            let (t2e1, t2e4, t2e5, t2e6, t2e2, t2e3,) = {
    if (p.p52 != 0.0) {
        let t2be: f64 = (-l.fbb2);let t2bf: f64 = (t2be / l.fb7b);let t2c0: f64 = t2bf;let t2c1: f64 = (-l.fbb2);let t2c2: f64 = (t2c1 / l.fb7b);let t2c3: f64 = (-t2c2);let t2c4: f64 = (0.001 / p.p53);let t2c5: f64 = (-l.fbb2);let t2c6: f64 = (t2c5 / l.fb7b);let t2c7: f64 = (-t2c6);let t2c8: f64 = (t2c4 * t2c7);let t2c9: f64 = (t2c8).tanh();let t2ca: f64 = (t2c3 * t2c9);let t2cb: f64 = (t2c0 + t2ca);let t2cc: f64 = (0.5 * t2cb);
        (t2cc, (0.5 * ((-((t2be * l.fb7e) / (l.fb7b * l.fb7b))) + (((-(-((t2c1 * l.fb7e) / (l.fb7b * l.fb7b)))) * t2c9) + (t2c3 * ((t2c4 * (-(-((t2c5 * l.fb7e) / (l.fb7b * l.fb7b))))) / ((t2c8).cosh() * (t2c8).cosh())))))), (0.5 * ((-((t2be * l.fb7f) / (l.fb7b * l.fb7b))) + (((-(-((t2c1 * l.fb7f) / (l.fb7b * l.fb7b)))) * t2c9) + (t2c3 * ((t2c4 * (-(-((t2c5 * l.fb7f) / (l.fb7b * l.fb7b))))) / ((t2c8).cosh() * (t2c8).cosh())))))), (0.5 * ((-((t2be * l.fb80) / (l.fb7b * l.fb7b))) + (((-(-((t2c1 * l.fb80) / (l.fb7b * l.fb7b)))) * t2c9) + (t2c3 * ((t2c4 * (-(-((t2c5 * l.fb80) / (l.fb7b * l.fb7b))))) / ((t2c8).cosh() * (t2c8).cosh())))))), (0.5 * (((((-l.fbb3) * l.fb7b) - (t2be * l.fb7c)) / (l.fb7b * l.fb7b)) + (((-((((-l.fbb3) * l.fb7b) - (t2c1 * l.fb7c)) / (l.fb7b * l.fb7b))) * t2c9) + (t2c3 * ((t2c4 * (-((((-l.fbb3) * l.fb7b) - (t2c5 * l.fb7c)) / (l.fb7b * l.fb7b)))) / ((t2c8).cosh() * (t2c8).cosh())))))), (0.5 * (((((-l.fbb4) * l.fb7b) - (t2be * l.fb7d)) / (l.fb7b * l.fb7b)) + (((-((((-l.fbb4) * l.fb7b) - (t2c1 * l.fb7d)) / (l.fb7b * l.fb7b))) * t2c9) + (t2c3 * ((t2c4 * (-((((-l.fbb4) * l.fb7b) - (t2c5 * l.fb7d)) / (l.fb7b * l.fb7b)))) / ((t2c8).cosh() * (t2c8).cosh())))))),)
    } else {
        let (t2db, t2de, t2df, t2e0, t2dc, t2dd,) = {
            if (p.p52 == 0.0) {
                let t2cd: f64 = (-l.fbb2);let t2ce: f64 = (t2cd / l.fb7b);let t2cf: f64 = t2ce;let t2d0: f64 = (-l.fbb2);let t2d1: f64 = (t2d0 / l.fb7b);let t2d2: f64 = (-t2d1);let t2d3: f64 = (-l.fbb2);let t2d4: f64 = (t2d3 / l.fb7b);let t2d5: f64 = (-t2d4);let t2d6: f64 = (t2d2 * t2d5);let t2d7: f64 = (t2d6 + p.p53);let t2d8: f64 = (t2d7).sqrt();let t2d9: f64 = (t2cf + t2d8);let t2da: f64 = (0.5 * t2d9);
                (t2da, (0.5 * ((-((t2cd * l.fb7e) / (l.fb7b * l.fb7b))) + ((((-(-((t2d0 * l.fb7e) / (l.fb7b * l.fb7b)))) * t2d5) + (t2d2 * (-(-((t2d3 * l.fb7e) / (l.fb7b * l.fb7b)))))) / (2.0 * t2d8)))), (0.5 * ((-((t2cd * l.fb7f) / (l.fb7b * l.fb7b))) + ((((-(-((t2d0 * l.fb7f) / (l.fb7b * l.fb7b)))) * t2d5) + (t2d2 * (-(-((t2d3 * l.fb7f) / (l.fb7b * l.fb7b)))))) / (2.0 * t2d8)))), (0.5 * ((-((t2cd * l.fb80) / (l.fb7b * l.fb7b))) + ((((-(-((t2d0 * l.fb80) / (l.fb7b * l.fb7b)))) * t2d5) + (t2d2 * (-(-((t2d3 * l.fb80) / (l.fb7b * l.fb7b)))))) / (2.0 * t2d8)))), (0.5 * (((((-l.fbb3) * l.fb7b) - (t2cd * l.fb7c)) / (l.fb7b * l.fb7b)) + ((((-((((-l.fbb3) * l.fb7b) - (t2d0 * l.fb7c)) / (l.fb7b * l.fb7b))) * t2d5) + (t2d2 * (-((((-l.fbb3) * l.fb7b) - (t2d3 * l.fb7c)) / (l.fb7b * l.fb7b))))) / (2.0 * t2d8)))), (0.5 * (((((-l.fbb4) * l.fb7b) - (t2cd * l.fb7d)) / (l.fb7b * l.fb7b)) + ((((-((((-l.fbb4) * l.fb7b) - (t2d0 * l.fb7d)) / (l.fb7b * l.fb7b))) * t2d5) + (t2d2 * (-((((-l.fbb4) * l.fb7b) - (t2d3 * l.fb7d)) / (l.fb7b * l.fb7b))))) / (2.0 * t2d8)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2db, t2de, t2df, t2e0, t2dc, t2dd,)
    }
};
            let t2e7: f64 = (t2e1).powf(l.f9c9);let t2e8: f64 = (1.0 + t2e7);let t2e9: f64 = (1.0 / l.f9c9);let t2ea: f64 = (t2e8).powf(t2e9);let t2eb: f64 = (1.0 / t2ea);
            (l.fa30, l.fa33, l.fa34, l.fa35, l.fa31, l.fa32, ) = (t2eb, (-(if 0.0 == 0.0 && ((t2e9) as f64).is_finite() && ((t2e9) as f64).fract() == 0.0 { if t2e9 == 0.0 { 0.0 } else { (t2e9 * ((t2e8).powf(t2e9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e4)) } } else { (t2e7 * (l.f9c9 * (t2e4 / t2e1))) })) } } else { (t2ea * (t2e9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e4)) } } else { (t2e7 * (l.f9c9 * (t2e4 / t2e1))) } / t2e8))) } / (t2ea * t2ea))), (-(if 0.0 == 0.0 && ((t2e9) as f64).is_finite() && ((t2e9) as f64).fract() == 0.0 { if t2e9 == 0.0 { 0.0 } else { (t2e9 * ((t2e8).powf(t2e9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e5)) } } else { (t2e7 * (l.f9c9 * (t2e5 / t2e1))) })) } } else { (t2ea * (t2e9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e5)) } } else { (t2e7 * (l.f9c9 * (t2e5 / t2e1))) } / t2e8))) } / (t2ea * t2ea))), (-(if 0.0 == 0.0 && ((t2e9) as f64).is_finite() && ((t2e9) as f64).fract() == 0.0 { if t2e9 == 0.0 { 0.0 } else { (t2e9 * ((t2e8).powf(t2e9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e6)) } } else { (t2e7 * (l.f9c9 * (t2e6 / t2e1))) })) } } else { (t2ea * (t2e9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e6)) } } else { (t2e7 * (l.f9c9 * (t2e6 / t2e1))) } / t2e8))) } / (t2ea * t2ea))), (-(if 0.0 == 0.0 && ((t2e9) as f64).is_finite() && ((t2e9) as f64).fract() == 0.0 { if t2e9 == 0.0 { 0.0 } else { (t2e9 * ((t2e8).powf(t2e9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e2)) } } else { (t2e7 * (l.f9c9 * (t2e2 / t2e1))) })) } } else { (t2ea * (t2e9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e2)) } } else { (t2e7 * (l.f9c9 * (t2e2 / t2e1))) } / t2e8))) } / (t2ea * t2ea))), (-(if 0.0 == 0.0 && ((t2e9) as f64).is_finite() && ((t2e9) as f64).fract() == 0.0 { if t2e9 == 0.0 { 0.0 } else { (t2e9 * ((t2e8).powf(t2e9 - 1.0) * if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e3)) } } else { (t2e7 * (l.f9c9 * (t2e3 / t2e1))) })) } } else { (t2ea * (t2e9 * (if 0.0 == 0.0 && ((l.f9c9) as f64).is_finite() && ((l.f9c9) as f64).fract() == 0.0 { if l.f9c9 == 0.0 { 0.0 } else { (l.f9c9 * ((t2e1).powf(l.f9c9 - 1.0) * t2e3)) } } else { (t2e7 * (l.f9c9 * (t2e3 / t2e1))) } / t2e8))) } / (t2ea * t2ea))), );
        }
        if (l.f1eb3 != 0.0) {let t2ec: f64 = (-l.fbb2);let t2ed: f64 = (t2ec * l.fa30);(l.fbde, l.fbe1, l.fbe2, l.fbe3, l.fbdf, l.fbe0, ) = (t2ed, (t2ec * l.fa33), (t2ec * l.fa34), (t2ec * l.fa35), (((-l.fbb3) * l.fa30) + (t2ec * l.fa31)), (((-l.fbb4) * l.fa30) + (t2ec * l.fa32)), );let t2ee: f64 = (l.fbd4 - l.fa9c);let t2ef: f64 = (t2ee / l.f9c5);(l.fa21, l.fa24, l.fa25, l.fa26, l.fa22, l.fa23, ) = (t2ef, (l.fbd6 / l.f9c5), ((((-l.fa9d) * l.f9c5) - (t2ee * l.f9c6)) / (l.f9c5 * l.f9c5)), (l.fbd7 / l.f9c5), (l.fbd5 / l.f9c5), 0.0, );}
        let t2f0: f64 = if l.fa21 > 50.0 { 1.0 } else { 0.0 };l.f1fe1 = t2f0;
        if ((l.f1eb3 != 0.0) && (l.f1fe1 != 0.0)) {(l.fa5d, l.fa60, l.fa61, l.fa62, l.fa5e, l.fa5f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t2fa: f64 = (-50.0);let t2fb: f64 = if l.fa21 < t2fa { 1.0 } else { 0.0 };l.f1fec = t2fb;
        if (((l.f1eb3 != 0.0) && (l.f1fe1 == 0.0)) && (l.f1fec != 0.0)) {(l.fa5d, l.fa60, l.fa61, l.fa62, l.fa5e, l.fa5f, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f1eb3 != 0.0) && (l.f1fe1 == 0.0)) && (l.f1fec == 0.0)) {let t2fc: f64 = (l.fa21).exp();let t2fd: f64 = (1.0 + t2fc);let t2fe: f64 = (1.0 / t2fd);(l.fa5d, l.fa60, l.fa61, l.fa62, l.fa5e, l.fa5f, ) = (t2fe, (-((t2fc * l.fa24) / (t2fd * t2fd))), (-((t2fc * l.fa25) / (t2fd * t2fd))), (-((t2fc * l.fa26) / (t2fd * t2fd))), (-((t2fc * l.fa22) / (t2fd * t2fd))), (-((t2fc * l.fa23) / (t2fd * t2fd))), );}
        if (l.f1eb3 != 0.0) {let t2ff: f64 = (l.fbce - l.fbde);let t300: f64 = (p.p51 * 0.1);let t301: f64 = (t300 * l.f9c5);let t302: f64 = (t301 * l.fa5d);let t303: f64 = (l.fbf5 - t302);let t304: f64 = (t2ff - t303);let t305: f64 = (t304 / l.fb67);(l.fa12, l.fa15, l.fa16, l.fa17, l.fa13, l.fa14, ) = (t305, (((l.fbd1 - l.fbe1) - (-(t301 * l.fa60))) / l.fb67), (((((-l.fbe2) - (l.fbf6 - (((t300 * l.f9c6) * l.fa5d) + (t301 * l.fa61)))) * l.fb67) - (t304 * l.fb68)) / (l.fb67 * l.fb67)), (((l.fbd2 - l.fbe3) - (-(t301 * l.fa62))) / l.fb67), (((l.fbcf - l.fbdf) - (-(t301 * l.fa5e))) / l.fb67), (((l.fbd0 - l.fbe0) - (-(t301 * l.fa5f))) / l.fb67), );}
    }
}
