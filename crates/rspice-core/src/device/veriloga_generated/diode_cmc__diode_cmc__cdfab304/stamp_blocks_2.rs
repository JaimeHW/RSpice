#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        l: &mut StampLocals,
    ) {
        let t169: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f12b = t169;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12b != 0.0)) {let t16a: f64 = (l.f796 * l.f76b);let t16b: f64 = (1.0 - t16a);let t16c: f64 = (t16b).sqrt();l.f6fc = t16c;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12b == 0.0)) {let t16d: f64 = (l.f796 * l.f76b);let t16e: f64 = (1.0 - t16d);let t16f: f64 = (t16e).powf(l.f60d);l.f6fc = t16f;}
        if ((l.f29a != 0.0) && (l.f129 == 0.0)) {let t170: f64 = (1.0 - l.f6fc);let t171: f64 = (l.f6a0 * t170);let t172: f64 = (l.f737 - l.f796);let t173: f64 = (l.f69a * t172);let t174: f64 = (t171 + t173);(l.f690, l.f691, l.f692, ) = (t174, 0.0, 0.0, );let t175: f64 = (l.f544 * l.f53a);(l.f52f, l.f530, l.f531, ) = (t175, (l.f544 * l.f53b), (l.f544 * l.f53c), );}
        let t176: f64 = if ((l.f3b == 0.0) && (l.f41 == 0.0)) { 1.0 } else { 0.0 };l.f12d = t176;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d != 0.0)) {l.f758 = 0.0;l.f7e9 = 0.0;l.f7d1 = 0.0;l.f9 = 0.0;l.f593 = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) {let t177: f64 = (l.f763 - l.f7a2);l.f758 = t177;let t178: f64 = (l.f714 / l.f758);let t179: f64 = (1.0 - t178);let t17a: f64 = (t179).sqrt();let t17b: f64 = (1.0 - t17a);l.f7ef = t17b;}
        let t17c: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f12f = t17c;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) && (l.f12f != 0.0)) {l.f66 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) && (l.f12f == 0.0)) {let t17d: f64 = (l.f7ef * l.f7ef);let t17e: f64 = (l.f7ef).ln();let t17f: f64 = (t17d * t17e);let t180: f64 = (1.0 - l.f7ef);let t181: f64 = (t17f / t180);let t182: f64 = (t181 + l.f7ef);let t183: f64 = (2.0 * l.f62f);let t184: f64 = (1.0 - t183);let t185: f64 = (t182 * t184);l.f66 = t185;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) {let t186: f64 = (l.f7ef + l.f66);l.f7e9 = t186;}
        let t187: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f131 = t187;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) && (l.f131 != 0.0)) {let t188: f64 = (l.f758 * l.f777);let t189: f64 = (t188).sqrt();l.f6fc = t189;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) && (l.f131 == 0.0)) {let t18a: f64 = (l.f758 * l.f777);let t18b: f64 = (t18a).powf(l.f62f);l.f6fc = t18b;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f12d == 0.0)) {let t18c: f64 = (l.f7d8 * l.f6fc);l.f7d1 = t18c;let t18d: f64 = (l.f825 - 1.0);let t18e: f64 = (t18d * l.f7d1);let t18f: f64 = (l.fcd * t18e);l.f9 = t18f;let t190: f64 = (l.f9 * l.f7e9);let t191: f64 = (l.f3b * t190);l.f593 = t191;}
        let t192: f64 = if l.f41 == 0.0 { 1.0 } else { 0.0 };l.f134 = t192;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 != 0.0)) {l.f599 = 0.0;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t193: f64 = (l.f7d1 * l.f60d);let t194: f64 = (t193 / l.f758);let t195: f64 = (l.f20 * t194);l.f19 = t195;let t196: f64 = (0.666666666666667 * l.f10);let t197: f64 = (t196 / l.f19);l.f71a = t197;let t198: f64 = (l.f71a * l.f71a);l.f72c = t198;let t199: f64 = (l.f72c * l.f72c);let t19a: f64 = (l.f72c * l.f72c);let t19b: f64 = (t19a + 1.0);let t19c: f64 = (t199 / t19b);let t19d: f64 = (t19c).sqrt();l.f726 = t19d;let t19e: f64 = (l.f726).abs();let t19f: f64 = (t19e).sqrt();l.f6c1 = t19f;let t1a0: f64 = (l.f726 * l.f6c1);l.f732 = t1a0;}
        let t1a1: f64 = (-l.f62f);let t1a2: f64 = (t1a1 * l.f613);let t1a3: f64 = (-1.0);let t1a4: f64 = if t1a2 == t1a3 { 1.0 } else { 0.0 };l.f136 = t1a4;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f136 != 0.0)) {let t1a5: f64 = (l.f19 * l.f732);let t1a6: f64 = (1.0 + t1a5);let t1a7: f64 = (1.0 / t1a6);l.f7e3 = t1a7;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f136 == 0.0)) {let t1a8: f64 = (l.f19 * l.f732);let t1a9: f64 = (1.0 + t1a8);let t1aa: f64 = (-l.f62f);let t1ab: f64 = (t1aa * l.f613);let t1ac: f64 = (t1a9).powf(t1ab);l.f7e3 = t1ac;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t1ad: f64 = (l.f7e9 * l.f7e3);let t1ae: f64 = (l.f7e9 + l.f7e3);let t1af: f64 = (t1ad / t1ae);l.f7f5 = t1af;let t1b0: f64 = (l.f19 / l.f6c1);let t1b1: f64 = (0.375 * t1b0);let t1b2: f64 = (t1b1).sqrt();l.f5a8 = t1b2;let t1b3: f64 = (l.f71a * l.f6c1);let t1b4: f64 = (2.0 * t1b3);let t1b5: f64 = (t1b4 - l.f726);l.f5b4 = t1b5;let t1b6: f64 = (l.f10 * l.f71a);let t1b7: f64 = (t1b6 * l.f6c1);let t1b8: f64 = (l.f10 * l.f726);let t1b9: f64 = (t1b7 - t1b8);let t1ba: f64 = (l.f19 * l.f732);let t1bb: f64 = (0.5 * t1ba);let t1bc: f64 = (t1b9 + t1bb);l.f5d4 = t1bc;let t1bd: f64 = (l.f5b4 - 1.0);let t1be: f64 = (t1bd * l.f5a8);l.f7fb = t1be;let t1bf: f64 = (l.f7fb * l.f7fb);l.f811 = t1bf;}
        let t1c0: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f138 = t1c0;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f138 != 0.0)) {let t1c1: f64 = (l.f62b * l.f7fb);let t1c2: f64 = (1.0 + t1c1);let t1c3: f64 = (1.0 / t1c2);l.f6e2 = t1c3;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f138 == 0.0)) {let t1c4: f64 = (l.f62b * l.f7fb);let t1c5: f64 = (1.0 - t1c4);let t1c6: f64 = (1.0 / t1c5);l.f6e2 = t1c6;}
        let t1c7: f64 = (-l.f811);let t1c8: f64 = (t1c7 + l.f5d4);let t1c9: f64 = (-230.25850929940458);let t1ca: f64 = if t1c8 > t1c9 { 1.0 } else { 0.0 };l.f13a = t1ca;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13a != 0.0)) {let t1cb: f64 = (-l.f811);let t1cc: f64 = (t1cb + l.f5d4);let t1cd: f64 = (t1cc).exp();l.f6fc = t1cd;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13a == 0.0)) {let t1ce: f64 = (-230.25850929940458);let t1cf: f64 = (-l.f811);let t1d0: f64 = (t1cf + l.f5d4);let t1d1: f64 = (t1ce - t1d0);let t1d2: f64 = (-230.25850929940458);let t1d3: f64 = (-l.f811);let t1d4: f64 = (t1d3 + l.f5d4);let t1d5: f64 = (t1d2 - t1d4);let t1d6: f64 = (-230.25850929940458);let t1d7: f64 = (-l.f811);let t1d8: f64 = (t1d7 + l.f5d4);let t1d9: f64 = (t1d6 - t1d8);let t1da: f64 = (t1d9 * 0.3333333333333333);let t1db: f64 = (1.0 + t1da);let t1dc: f64 = (t1d5 * t1db);let t1dd: f64 = (0.5 * t1dc);let t1de: f64 = (1.0 + t1dd);let t1df: f64 = (t1d1 * t1de);let t1e0: f64 = (1.0 + t1df);let t1e1: f64 = (1e-100 / t1e0);l.f6fc = t1e1;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t1e2: f64 = (0.29214664 * l.f6e2);let t1e3: f64 = (l.f6e2 * l.f6e2);let t1e4: f64 = (l.f16 * t1e3);let t1e5: f64 = (t1e2 + t1e4);let t1e6: f64 = (l.f6e2 * l.f6e2);let t1e7: f64 = (t1e6 * l.f6e2);let t1e8: f64 = (l.f2a * t1e7);let t1e9: f64 = (t1e5 + t1e8);let t1ea: f64 = (t1e9 * l.f6fc);l.f6e = t1ea;}
        let t1eb: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f13c = t1eb;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13c != 0.0)) {l.f74 = l.f6e;}
        let t1ec: f64 = (-230.25850929940458);let t1ed: f64 = if l.f5d4 > t1ec { 1.0 } else { 0.0 };l.f13e = t1ed;
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13c == 0.0)) && (l.f13e != 0.0)) {let t1ee: f64 = (l.f5d4).exp();l.f6fc = t1ee;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13c == 0.0)) && (l.f13e == 0.0)) {let t1ef: f64 = (-230.25850929940458);let t1f0: f64 = (t1ef - l.f5d4);let t1f1: f64 = (-230.25850929940458);let t1f2: f64 = (t1f1 - l.f5d4);let t1f3: f64 = (-230.25850929940458);let t1f4: f64 = (t1f3 - l.f5d4);let t1f5: f64 = (t1f4 * 0.3333333333333333);let t1f6: f64 = (1.0 + t1f5);let t1f7: f64 = (t1f2 * t1f6);let t1f8: f64 = (0.5 * t1f7);let t1f9: f64 = (1.0 + t1f8);let t1fa: f64 = (t1f0 * t1f9);let t1fb: f64 = (1.0 + t1fa);let t1fc: f64 = (1e-100 / t1fb);l.f6fc = t1fc;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) && (l.f13c == 0.0)) {let t1fd: f64 = (2.0 * l.f6fc);let t1fe: f64 = (t1fd - l.f6e);l.f74 = t1fe;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f134 == 0.0)) {let t1ff: f64 = (1.772453850905516 * 0.5);let t200: f64 = (l.f10 * l.f74);let t201: f64 = (t200 / l.f5a8);let t202: f64 = (t1ff * t201);l.fd6 = t202;let t203: f64 = (l.f9 * l.fd6);let t204: f64 = (t203 * l.f7f5);let t205: f64 = (l.f41 * t204);l.f599 = t205;}
        let t206: f64 = if l.f26 == 0.0 { 1.0 } else { 0.0 };l.f140 = t206;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 != 0.0)) {l.f529 = 0.0;}
        let t207: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f142 = t207;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f142 != 0.0)) {let t208: f64 = (l.f775 - l.f750);let t209: f64 = (t208 * l.f777);let t20a: f64 = (t209).sqrt();l.f6fc = t20a;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f142 == 0.0)) {let t20b: f64 = (l.f775 - l.f750);let t20c: f64 = (t20b * l.f777);let t20d: f64 = (t20c).powf(l.f62f);l.f6fc = t20d;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) {let t20e: f64 = (l.f775 - l.f750);let t20f: f64 = (t20e * l.f7dc);let t210: f64 = (t20f / l.f6fc);let t211: f64 = (l.f613 * t210);l.fb6 = t211;}
        let t212: f64 = (-l.fa3);let t213: f64 = (t212 / l.fb6);let t214: f64 = (t213).abs();let t215: f64 = if t214 < 230.25850929940458 { 1.0 } else { 0.0 };l.f144 = t215;
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f144 != 0.0)) {let t216: f64 = (-l.fa3);let t217: f64 = (t216 / l.fb6);let t218: f64 = (t217).exp();l.f6fc = t218;}
        let t219: f64 = (-l.fa3);let t21a: f64 = (t219 / l.fb6);let t21b: f64 = (-230.25850929940458);let t21c: f64 = if t21a < t21b { 1.0 } else { 0.0 };l.f146 = t21c;
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f144 == 0.0)) && (l.f146 != 0.0)) {let t21d: f64 = (-230.25850929940458);let t21e: f64 = (-l.fa3);let t21f: f64 = (t21e / l.fb6);let t220: f64 = (t21d - t21f);let t221: f64 = (-230.25850929940458);let t222: f64 = (-l.fa3);let t223: f64 = (t222 / l.fb6);let t224: f64 = (t221 - t223);let t225: f64 = (-230.25850929940458);let t226: f64 = (-l.fa3);let t227: f64 = (t226 / l.fb6);let t228: f64 = (t225 - t227);let t229: f64 = (t228 * 0.3333333333333333);let t22a: f64 = (1.0 + t229);let t22b: f64 = (t224 * t22a);let t22c: f64 = (0.5 * t22b);let t22d: f64 = (1.0 + t22c);let t22e: f64 = (t220 * t22d);let t22f: f64 = (1.0 + t22e);let t230: f64 = (1e-100 / t22f);l.f6fc = t230;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) && (l.f144 == 0.0)) && (l.f146 == 0.0)) {let t236: f64 = (-l.fa3);let t237: f64 = (t236 / l.fb6);let t238: f64 = (t237 - 230.25850929940458);let t239: f64 = (-l.fa3);let t23a: f64 = (t239 / l.fb6);let t23b: f64 = (t23a - 230.25850929940458);let t23c: f64 = (-l.fa3);let t23d: f64 = (t23c / l.fb6);let t23e: f64 = (t23d - 230.25850929940458);let t23f: f64 = (t23e * 0.3333333333333333);let t240: f64 = (1.0 + t23f);let t241: f64 = (t23b * t240);let t231: f64 = (0.5 * t241);let t232: f64 = (1.0 + t231);let t233: f64 = (t238 * t232);let t234: f64 = (1.0 + t233);let t235: f64 = (1e100 * t234);l.f6fc = t235;}
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f140 == 0.0)) {let t242: f64 = (l.f737 * l.fb6);let t243: f64 = (t242 * l.fb6);let t244: f64 = (t243 * l.f6fc);let t245: f64 = (l.f26 * t244);l.f529 = t245;}
        let t246: f64 = if ((l.f785 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f148 = t246;
        if (((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 != 0.0)) {l.fae = 1.0;}
        let t247: f64 = (-l.f2);let t248: f64 = (t247 * l.f785);let t249: f64 = if l.f74a > t248 { 1.0 } else { 0.0 };l.f14a = t249;let t24a: f64 = if l.f627 == 4.0 { 1.0 } else { 0.0 };l.f14c = t24a;
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 == 0.0)) && (l.f14a != 0.0)) && (l.f14c != 0.0)) {let t24b: f64 = (l.f74a * l.f789);let t24c: f64 = (t24b).abs();let t24d: f64 = (l.f74a * l.f789);let t24e: f64 = (t24d).abs();let t24f: f64 = (t24c * t24e);let t250: f64 = (l.f74a * l.f789);let t251: f64 = (t250).abs();let t252: f64 = (t24f * t251);let t253: f64 = (l.f74a * l.f789);let t254: f64 = (t253).abs();let t255: f64 = (t252 * t254);l.f6fc = t255;}
        if (((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 == 0.0)) && (l.f14a != 0.0)) && (l.f14c == 0.0)) {let t256: f64 = (l.f74a * l.f789);let t257: f64 = (t256).abs();let t258: f64 = (t257).powf(l.f627);l.f6fc = t258;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 == 0.0)) && (l.f14a != 0.0)) {let t259: f64 = (1.0 - l.f6fc);let t25a: f64 = (1.0 / t259);l.fae = t25a;}
        if ((((l.f29a != 0.0) && (l.f129 == 0.0)) && (l.f148 == 0.0)) && (l.f14a == 0.0)) {let t25b: f64 = (l.f2 * l.f785);let t25c: f64 = (l.f74a + t25b);let t25d: f64 = (t25c * l.f6bc);let t25e: f64 = (l.fc5 + t25d);l.fae = t25e;}
        if ((l.f29a != 0.0) && (l.f129 == 0.0)) {let t25f: f64 = (l.f52f + l.f593);let t260: f64 = (t25f + l.f599);let t261: f64 = (t260 + l.f529);let t262: f64 = (t261 * l.fae);(l.f56e, l.f56f, l.f570, ) = (t262, (l.f530 * l.fae), (l.f531 * l.fae), );let t263: f64 = (l.f593 + l.f599);let t264: f64 = (t263 + l.f529);let t265: f64 = (t264 * l.fae);(l.f556, l.f557, l.f558, ) = (t265, 0.0, 0.0, );}
        if (l.f29a != 0.0) {let t266: f64 = (l.f0 * l.f562);let t267: f64 = (l.f5b1 * l.f576);let t268: f64 = (t266 + t267);let t269: f64 = (l.f5af * l.f56e);let t26a: f64 = (t268 + t269);(l.f500, l.f505, l.f506, ) = (t26a, (((l.f0 * l.f563) + (l.f5b1 * l.f577)) + (l.f5af * l.f56f)), (((l.f0 * l.f564) + (l.f5b1 * l.f578)) + (l.f5af * l.f570)), );}
        let t26b: f64 = if (!(((l.f0 == 0.0) && (l.f5b1 == 0.0)) && (l.f5af == 0.0))) { 1.0 } else { 0.0 };l.f14e = t26b;
        if ((l.f29a != 0.0) && (l.f14e != 0.0)) {let t26c: f64 = (4.0 * l.f78f);let t26d: f64 = (t26c * l.f78f);l.f4e1 = t26d;let t26e: f64 = (l.f78f / l.f791);l.f4e5 = t26e;let t26f: f64 = (l.f78f * l.f4e5);let t270: f64 = (l.f739 + t26f);l.f4e9 = t270;let t271: f64 = (l.f791 + l.f4e9);l.f4ef = t271;let t272: f64 = (l.f791 - l.f4e9);l.f4f5 = t272;let t273: f64 = (l.f4f5 * l.f4f5);let t274: f64 = (t273 + l.f4e1);let t275: f64 = (t274).sqrt();l.f4fb = t275;let t276: f64 = (l.f739 * l.f791);let t277: f64 = (l.f4ef + l.f4fb);let t278: f64 = (t276 / t277);let t279: f64 = (2.0 * t278);l.f796 = t279;}
        let t27a: f64 = if l.f739 < l.f7b1 { 1.0 } else { 0.0 };l.f150 = t27a;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t27b: f64 = (l.f739 * l.f645);let t27c: f64 = (0.5 * t27b);let t27d: f64 = (t27c).abs();let t27e: f64 = if t27d < 230.25850929940458 { 1.0 } else { 0.0 };l.f152 = t27e;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f152 != 0.0)) {let t27f: f64 = (l.f739 * l.f645);let t280: f64 = (0.5 * t27f);let t281: f64 = (t280).exp();l.f825 = t281;}
        let t282: f64 = (l.f739 * l.f645);let t283: f64 = (0.5 * t282);let t284: f64 = (-230.25850929940458);let t285: f64 = if t283 < t284 { 1.0 } else { 0.0 };l.f154 = t285;
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f152 == 0.0)) && (l.f154 != 0.0)) {let t286: f64 = (-230.25850929940458);let t287: f64 = (l.f739 * l.f645);let t288: f64 = (0.5 * t287);let t289: f64 = (t286 - t288);let t28a: f64 = (-230.25850929940458);let t28b: f64 = (l.f739 * l.f645);let t28c: f64 = (0.5 * t28b);let t28d: f64 = (t28a - t28c);let t28e: f64 = (-230.25850929940458);let t28f: f64 = (l.f739 * l.f645);let t290: f64 = (0.5 * t28f);let t291: f64 = (t28e - t290);let t292: f64 = (t291 * 0.3333333333333333);let t293: f64 = (1.0 + t292);let t294: f64 = (t28d * t293);let t295: f64 = (0.5 * t294);let t296: f64 = (1.0 + t295);let t297: f64 = (t289 * t296);let t298: f64 = (1.0 + t297);let t299: f64 = (1e-100 / t298);l.f825 = t299;}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f152 == 0.0)) && (l.f154 == 0.0)) {let t29a: f64 = (l.f739 * l.f645);let t29b: f64 = (0.5 * t29a);let t29c: f64 = (t29b - 230.25850929940458);let t29d: f64 = (l.f739 * l.f645);let t29e: f64 = (0.5 * t29d);let t29f: f64 = (t29e - 230.25850929940458);let t2a0: f64 = (l.f739 * l.f645);let t2a1: f64 = (0.5 * t2a0);let t2a2: f64 = (t2a1 - 230.25850929940458);let t2a3: f64 = (t2a2 * 0.3333333333333333);let t2a4: f64 = (1.0 + t2a3);let t2a5: f64 = (t29f * t2a4);let t2a6: f64 = (0.5 * t2a5);let t2a7: f64 = (1.0 + t2a6);let t2a8: f64 = (t29c * t2a7);let t2a9: f64 = (1.0 + t2a8);let t2aa: f64 = (1e100 * t2a9);l.f825 = t2aa;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) {let t2ab: f64 = (l.f5eb * l.f5eb);let t2ac: f64 = (t2ab / l.f5df);l.f64f = t2ac;let t2ad: f64 = (l.f5e5 / l.f645);let t2ae: f64 = (l.f5df / l.f64f);let t2af: f64 = (t2ae).ln();let t2b0: f64 = (t2ad * t2af);l.f793 = t2b0;}
        let t2b1: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f156 = t2b1;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t2b2: f64 = (l.f739 - l.f793);let t2b3: f64 = (p.p86 * t2b2);let t2b4: f64 = (t2b3 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t2b4, 0.0, 0.0, );let t2b5: f64 = (p.p86 * l.f793);let t2b6: f64 = (l.f5e5 - t2b5);(l.f5ed, l.f5ee, l.f5ef, ) = (t2b6, 0.0, 0.0, );let t2b7: f64 = (p.p85 - l.f601);let t2b8: f64 = (t2b7 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2b8, (-l.f602), (-l.f603), );let t2b9: f64 = (4.0 * p.p85);let t2ba: f64 = (t2b9 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2ba, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {
            let (t2bc, t2bd, t2be,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2bb: f64 = (-l.f6f7);
        (t2bb, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2bc, t2bd, t2be, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t2bf: f64 = (l.f6f3 * l.f6f3);let t2c0: f64 = (t2bf + l.f6f7);let t2c1: f64 = (t2c0).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2c1, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2c1)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2c1)), );let t2c2: f64 = (l.f6f3 + l.f6f7);let t2c3: f64 = (0.5 * t2c2);let t2c4: f64 = (p.p85 - t2c3);(l.f605, l.f606, l.f607, ) = (t2c4, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2c5: f64 = (l.f605 - l.f5e5);let t2c6: f64 = (t2c5 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2c6, l.f606, l.f607, );let t2c7: f64 = (4.0 * l.f5e5);let t2c8: f64 = (t2c7 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2c8, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {
            let (t2ca, t2cb, t2cc,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2c9: f64 = (-l.f6f7);
        (t2c9, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2ca, t2cb, t2cc, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t2cd: f64 = (l.f6f3 * l.f6f3);let t2ce: f64 = (t2cd + l.f6f7);let t2cf: f64 = (t2ce).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2cf, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2cf)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2cf)), );let t2d0: f64 = (l.f6f3 + l.f6f7);let t2d1: f64 = (0.5 * t2d0);let t2d2: f64 = (l.f5e5 + t2d1);(l.f5f1, l.f5f2, l.f5f3, ) = (t2d2, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t2d3: f64 = (p.p85 - l.f5ed);let t2d4: f64 = (t2d3 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2d4, (-l.f5ee), (-l.f5ef), );let t2d5: f64 = (4.0 * p.p85);let t2d6: f64 = (t2d5 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2d6, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {
            let (t2d8, t2d9, t2da,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2d7: f64 = (-l.f6f7);
        (t2d7, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2d8, t2d9, t2da, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t2db: f64 = (l.f6f3 * l.f6f3);let t2dc: f64 = (t2db + l.f6f7);let t2dd: f64 = (t2dc).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2dd, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2dd)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2dd)), );let t2de: f64 = (l.f6f3 + l.f6f7);let t2df: f64 = (0.5 * t2de);let t2e0: f64 = (p.p85 - t2df);(l.f5ed, l.f5ee, l.f5ef, ) = (t2e0, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2e1: f64 = (l.f5ed - l.f5e5);let t2e2: f64 = (t2e1 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2e2, l.f5ee, l.f5ef, );let t2e3: f64 = (4.0 * l.f5e5);let t2e4: f64 = (t2e3 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2e4, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {
            let (t2e6, t2e7, t2e8,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2e5: f64 = (-l.f6f7);
        (t2e5, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2e6, t2e7, t2e8, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 != 0.0)) {let t2e9: f64 = (l.f6f3 * l.f6f3);let t2ea: f64 = (t2e9 + l.f6f7);let t2eb: f64 = (t2ea).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2eb, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2eb)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2eb)), );let t2ec: f64 = (l.f6f3 + l.f6f7);let t2ed: f64 = (0.5 * t2ec);let t2ee: f64 = (l.f5e5 + t2ed);(l.f5ed, l.f5ee, l.f5ef, ) = (t2ee, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f156 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );}
        let t2ef: f64 = (l.f739 / l.f5f1);let t2f0: f64 = (l.f5f1 - l.f5ed);let t2f1: f64 = (l.f793 * t2f0);let t2f2: f64 = (l.f5ed * p.p85);let t2f3: f64 = (t2f1 / t2f2);let t2f4: f64 = (t2ef + t2f3);let t2f5: f64 = (l.f645 * t2f4);let t2f6: f64 = (t2f5).abs();let t2f7: f64 = if t2f6 < 230.25850929940458 { 1.0 } else { 0.0 };l.f158 = t2f7;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f158 != 0.0)) {let t2f8: f64 = (l.f739 / l.f5f1);let t2f9: f64 = (l.f5f1 - l.f5ed);let t2fa: f64 = (l.f793 * t2f9);let t2fb: f64 = (l.f5ed * p.p85);let t2fc: f64 = (t2fa / t2fb);let t2fd: f64 = (t2f8 + t2fc);let t2fe: f64 = (l.f645 * t2fd);let t2ff: f64 = (t2fe).exp();(l.f536, l.f537, l.f538, ) = (t2ff, (t2ff * (l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2fb) - (t2fa * (l.f5ee * p.p85))) / (t2fb * t2fb))))), (t2ff * (l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2fb) - (t2fa * (l.f5ef * p.p85))) / (t2fb * t2fb))))), );}
        let t300: f64 = (l.f739 / l.f5f1);let t301: f64 = (l.f5f1 - l.f5ed);let t302: f64 = (l.f793 * t301);let t303: f64 = (l.f5ed * p.p85);let t304: f64 = (t302 / t303);let t305: f64 = (t300 + t304);let t306: f64 = (l.f645 * t305);let t307: f64 = (-230.25850929940458);let t308: f64 = if t306 < t307 { 1.0 } else { 0.0 };l.f15a = t308;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f158 == 0.0)) && (l.f15a != 0.0)) {let t309: f64 = (-230.25850929940458);let t30a: f64 = (l.f739 / l.f5f1);let t30b: f64 = (l.f5f1 - l.f5ed);let t30c: f64 = (l.f793 * t30b);let t30d: f64 = (l.f5ed * p.p85);let t30e: f64 = (t30c / t30d);let t30f: f64 = (t30a + t30e);let t310: f64 = (l.f645 * t30f);let t311: f64 = (t309 - t310);let t312: f64 = (-230.25850929940458);let t313: f64 = (l.f739 / l.f5f1);let t314: f64 = (l.f5f1 - l.f5ed);let t315: f64 = (l.f793 * t314);let t316: f64 = (l.f5ed * p.p85);let t317: f64 = (t315 / t316);let t318: f64 = (t313 + t317);let t319: f64 = (l.f645 * t318);let t31a: f64 = (t312 - t319);let t31b: f64 = (-230.25850929940458);let t31c: f64 = (l.f739 / l.f5f1);let t31d: f64 = (l.f5f1 - l.f5ed);let t31e: f64 = (l.f793 * t31d);let t31f: f64 = (l.f5ed * p.p85);let t320: f64 = (t31e / t31f);let t321: f64 = (t31c + t320);let t322: f64 = (l.f645 * t321);let t323: f64 = (t31b - t322);let t324: f64 = (t323 * 0.3333333333333333);let t325: f64 = (1.0 + t324);let t326: f64 = (t31a * t325);let t327: f64 = (0.5 * t326);let t328: f64 = (1.0 + t327);let t329: f64 = (t311 * t328);let t32a: f64 = (1.0 + t329);let t32b: f64 = (1e-100 / t32a);(l.f536, l.f537, l.f538, ) = (t32b, (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t30d) - (t30c * (l.f5ee * p.p85))) / (t30d * t30d))))) * t328) + (t311 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t316) - (t315 * (l.f5ee * p.p85))) / (t316 * t316))))) * t325) + (t31a * ((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t31f) - (t31e * (l.f5ee * p.p85))) / (t31f * t31f))))) * 0.3333333333333333))))))) / (t32a * t32a))), (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t30d) - (t30c * (l.f5ef * p.p85))) / (t30d * t30d))))) * t328) + (t311 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t316) - (t315 * (l.f5ef * p.p85))) / (t316 * t316))))) * t325) + (t31a * ((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t31f) - (t31e * (l.f5ef * p.p85))) / (t31f * t31f))))) * 0.3333333333333333))))))) / (t32a * t32a))), );}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f158 == 0.0)) && (l.f15a == 0.0)) {let t32c: f64 = (l.f739 / l.f5f1);let t32d: f64 = (l.f5f1 - l.f5ed);let t32e: f64 = (l.f793 * t32d);let t32f: f64 = (l.f5ed * p.p85);let t330: f64 = (t32e / t32f);let t331: f64 = (t32c + t330);let t332: f64 = (l.f645 * t331);let t333: f64 = (t332 - 230.25850929940458);let t334: f64 = (l.f739 / l.f5f1);let t335: f64 = (l.f5f1 - l.f5ed);let t336: f64 = (l.f793 * t335);let t337: f64 = (l.f5ed * p.p85);let t338: f64 = (t336 / t337);let t339: f64 = (t334 + t338);let t33a: f64 = (l.f645 * t339);let t33b: f64 = (t33a - 230.25850929940458);let t33c: f64 = (l.f739 / l.f5f1);let t33d: f64 = (l.f5f1 - l.f5ed);let t33e: f64 = (l.f793 * t33d);let t33f: f64 = (l.f5ed * p.p85);let t340: f64 = (t33e / t33f);let t341: f64 = (t33c + t340);let t342: f64 = (l.f645 * t341);let t343: f64 = (t342 - 230.25850929940458);let t344: f64 = (t343 * 0.3333333333333333);let t345: f64 = (1.0 + t344);let t346: f64 = (t33b * t345);let t347: f64 = (0.5 * t346);let t348: f64 = (1.0 + t347);let t349: f64 = (t333 * t348);let t34a: f64 = (1.0 + t349);let t34b: f64 = (1e100 * t34a);(l.f536, l.f537, l.f538, ) = (t34b, (1e100 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t32f) - (t32e * (l.f5ee * p.p85))) / (t32f * t32f)))) * t348) + (t333 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t337) - (t336 * (l.f5ee * p.p85))) / (t337 * t337)))) * t345) + (t33b * ((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t33f) - (t33e * (l.f5ee * p.p85))) / (t33f * t33f)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t32f) - (t32e * (l.f5ef * p.p85))) / (t32f * t32f)))) * t348) + (t333 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t337) - (t336 * (l.f5ef * p.p85))) / (t337 * t337)))) * t345) + (t33b * ((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t33f) - (t33e * (l.f5ef * p.p85))) / (t33f * t33f)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) {let t34c: f64 = (l.f5eb * l.f5eb);let t34d: f64 = (t34c / l.f5e3);l.f64f = t34d;let t34e: f64 = (l.f5e9 / l.f645);let t34f: f64 = (l.f5e3 / l.f64f);let t350: f64 = (t34f).ln();let t351: f64 = (t34e * t350);l.f793 = t351;}
        let t352: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f15c = t352;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t353: f64 = (l.f739 - l.f793);let t354: f64 = (p.p86 * t353);let t355: f64 = (t354 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t355, 0.0, 0.0, );let t356: f64 = (p.p86 * l.f793);let t357: f64 = (l.f5e9 - t356);(l.f5ed, l.f5ee, l.f5ef, ) = (t357, 0.0, 0.0, );let t358: f64 = (p.p85 - l.f601);let t359: f64 = (t358 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t359, (-l.f602), (-l.f603), );let t35a: f64 = (4.0 * p.p85);let t35b: f64 = (t35a * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t35b, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {
            let (t35d, t35e, t35f,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t35c: f64 = (-l.f6f7);
        (t35c, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t35d, t35e, t35f, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t360: f64 = (l.f6f3 * l.f6f3);let t361: f64 = (t360 + l.f6f7);let t362: f64 = (t361).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t362, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t362)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t362)), );let t363: f64 = (l.f6f3 + l.f6f7);let t364: f64 = (0.5 * t363);let t365: f64 = (p.p85 - t364);(l.f605, l.f606, l.f607, ) = (t365, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t366: f64 = (l.f605 - l.f5e9);let t367: f64 = (t366 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t367, l.f606, l.f607, );let t368: f64 = (4.0 * l.f5e9);let t369: f64 = (t368 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t369, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {
            let (t36b, t36c, t36d,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t36a: f64 = (-l.f6f7);
        (t36a, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t36b, t36c, t36d, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t36e: f64 = (l.f6f3 * l.f6f3);let t36f: f64 = (t36e + l.f6f7);let t370: f64 = (t36f).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t370, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t370)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t370)), );let t371: f64 = (l.f6f3 + l.f6f7);let t372: f64 = (0.5 * t371);let t373: f64 = (l.f5e9 + t372);(l.f5f1, l.f5f2, l.f5f3, ) = (t373, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t374: f64 = (p.p85 - l.f5ed);let t375: f64 = (t374 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t375, (-l.f5ee), (-l.f5ef), );let t376: f64 = (4.0 * p.p85);let t377: f64 = (t376 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t377, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {
            let (t379, t37a, t37b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t378: f64 = (-l.f6f7);
        (t378, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t379, t37a, t37b, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t37c: f64 = (l.f6f3 * l.f6f3);let t37d: f64 = (t37c + l.f6f7);let t37e: f64 = (t37d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t37e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t37e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t37e)), );let t37f: f64 = (l.f6f3 + l.f6f7);let t380: f64 = (0.5 * t37f);let t381: f64 = (p.p85 - t380);(l.f5ed, l.f5ee, l.f5ef, ) = (t381, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t382: f64 = (l.f5ed - l.f5e9);let t383: f64 = (t382 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t383, l.f5ee, l.f5ef, );let t384: f64 = (4.0 * l.f5e9);let t385: f64 = (t384 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t385, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {
            let (t387, t388, t389,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t386: f64 = (-l.f6f7);
        (t386, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t387, t388, t389, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c != 0.0)) {let t38a: f64 = (l.f6f3 * l.f6f3);let t38b: f64 = (t38a + l.f6f7);let t38c: f64 = (t38b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t38c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t38c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t38c)), );let t38d: f64 = (l.f6f3 + l.f6f7);let t38e: f64 = (0.5 * t38d);let t38f: f64 = (l.f5e9 + t38e);(l.f5ed, l.f5ee, l.f5ef, ) = (t38f, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15c == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );}
        let t390: f64 = (l.f739 / l.f5f1);let t391: f64 = (l.f5f1 - l.f5ed);let t392: f64 = (l.f793 * t391);let t393: f64 = (l.f5ed * p.p85);let t394: f64 = (t392 / t393);let t395: f64 = (t390 + t394);let t396: f64 = (l.f645 * t395);let t397: f64 = (t396).abs();let t398: f64 = if t397 < 230.25850929940458 { 1.0 } else { 0.0 };l.f15e = t398;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15e != 0.0)) {let t399: f64 = (l.f739 / l.f5f1);let t39a: f64 = (l.f5f1 - l.f5ed);let t39b: f64 = (l.f793 * t39a);let t39c: f64 = (l.f5ed * p.p85);let t39d: f64 = (t39b / t39c);let t39e: f64 = (t399 + t39d);let t39f: f64 = (l.f645 * t39e);let t3a0: f64 = (t39f).exp();(l.f53e, l.f53f, l.f540, ) = (t3a0, (t3a0 * (l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t39c) - (t39b * (l.f5ee * p.p85))) / (t39c * t39c))))), (t3a0 * (l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t39c) - (t39b * (l.f5ef * p.p85))) / (t39c * t39c))))), );}
        let t3a1: f64 = (l.f739 / l.f5f1);let t3a2: f64 = (l.f5f1 - l.f5ed);let t3a3: f64 = (l.f793 * t3a2);let t3a4: f64 = (l.f5ed * p.p85);let t3a5: f64 = (t3a3 / t3a4);let t3a6: f64 = (t3a1 + t3a5);let t3a7: f64 = (l.f645 * t3a6);let t3a8: f64 = (-230.25850929940458);let t3a9: f64 = if t3a7 < t3a8 { 1.0 } else { 0.0 };l.f160 = t3a9;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15e == 0.0)) && (l.f160 != 0.0)) {let t3aa: f64 = (-230.25850929940458);let t3ab: f64 = (l.f739 / l.f5f1);let t3ac: f64 = (l.f5f1 - l.f5ed);let t3ad: f64 = (l.f793 * t3ac);let t3ae: f64 = (l.f5ed * p.p85);let t3af: f64 = (t3ad / t3ae);let t3b0: f64 = (t3ab + t3af);let t3b1: f64 = (l.f645 * t3b0);let t3b2: f64 = (t3aa - t3b1);let t3b3: f64 = (-230.25850929940458);let t3b4: f64 = (l.f739 / l.f5f1);let t3b5: f64 = (l.f5f1 - l.f5ed);let t3b6: f64 = (l.f793 * t3b5);let t3b7: f64 = (l.f5ed * p.p85);let t3b8: f64 = (t3b6 / t3b7);let t3b9: f64 = (t3b4 + t3b8);let t3ba: f64 = (l.f645 * t3b9);let t3bb: f64 = (t3b3 - t3ba);let t3bc: f64 = (-230.25850929940458);let t3bd: f64 = (l.f739 / l.f5f1);let t3be: f64 = (l.f5f1 - l.f5ed);let t3bf: f64 = (l.f793 * t3be);let t3c0: f64 = (l.f5ed * p.p85);let t3c1: f64 = (t3bf / t3c0);let t3c2: f64 = (t3bd + t3c1);let t3c3: f64 = (l.f645 * t3c2);let t3c4: f64 = (t3bc - t3c3);let t3c5: f64 = (t3c4 * 0.3333333333333333);let t3c6: f64 = (1.0 + t3c5);let t3c7: f64 = (t3bb * t3c6);let t3c8: f64 = (0.5 * t3c7);let t3c9: f64 = (1.0 + t3c8);let t3ca: f64 = (t3b2 * t3c9);let t3cb: f64 = (1.0 + t3ca);let t3cc: f64 = (1e-100 / t3cb);(l.f53e, l.f53f, l.f540, ) = (t3cc, (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3ae) - (t3ad * (l.f5ee * p.p85))) / (t3ae * t3ae))))) * t3c9) + (t3b2 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3b7) - (t3b6 * (l.f5ee * p.p85))) / (t3b7 * t3b7))))) * t3c6) + (t3bb * ((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3c0) - (t3bf * (l.f5ee * p.p85))) / (t3c0 * t3c0))))) * 0.3333333333333333))))))) / (t3cb * t3cb))), (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3ae) - (t3ad * (l.f5ef * p.p85))) / (t3ae * t3ae))))) * t3c9) + (t3b2 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3b7) - (t3b6 * (l.f5ef * p.p85))) / (t3b7 * t3b7))))) * t3c6) + (t3bb * ((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3c0) - (t3bf * (l.f5ef * p.p85))) / (t3c0 * t3c0))))) * 0.3333333333333333))))))) / (t3cb * t3cb))), );}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15e == 0.0)) && (l.f160 == 0.0)) {let t3cd: f64 = (l.f739 / l.f5f1);let t3ce: f64 = (l.f5f1 - l.f5ed);let t3cf: f64 = (l.f793 * t3ce);let t3d0: f64 = (l.f5ed * p.p85);let t3d1: f64 = (t3cf / t3d0);let t3d2: f64 = (t3cd + t3d1);let t3d3: f64 = (l.f645 * t3d2);let t3d4: f64 = (t3d3 - 230.25850929940458);let t3d5: f64 = (l.f739 / l.f5f1);let t3d6: f64 = (l.f5f1 - l.f5ed);let t3d7: f64 = (l.f793 * t3d6);let t3d8: f64 = (l.f5ed * p.p85);let t3d9: f64 = (t3d7 / t3d8);let t3da: f64 = (t3d5 + t3d9);let t3db: f64 = (l.f645 * t3da);let t3dc: f64 = (t3db - 230.25850929940458);let t3dd: f64 = (l.f739 / l.f5f1);let t3de: f64 = (l.f5f1 - l.f5ed);let t3df: f64 = (l.f793 * t3de);let t3e0: f64 = (l.f5ed * p.p85);let t3e1: f64 = (t3df / t3e0);let t3e2: f64 = (t3dd + t3e1);let t3e3: f64 = (l.f645 * t3e2);let t3e4: f64 = (t3e3 - 230.25850929940458);let t3e5: f64 = (t3e4 * 0.3333333333333333);let t3e6: f64 = (1.0 + t3e5);let t3e7: f64 = (t3dc * t3e6);let t3e8: f64 = (0.5 * t3e7);let t3e9: f64 = (1.0 + t3e8);let t3ea: f64 = (t3d4 * t3e9);let t3eb: f64 = (1.0 + t3ea);let t3ec: f64 = (1e100 * t3eb);(l.f53e, l.f53f, l.f540, ) = (t3ec, (1e100 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3d0) - (t3cf * (l.f5ee * p.p85))) / (t3d0 * t3d0)))) * t3e9) + (t3d4 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3d8) - (t3d7 * (l.f5ee * p.p85))) / (t3d8 * t3d8)))) * t3e6) + (t3dc * ((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t3e0) - (t3df * (l.f5ee * p.p85))) / (t3e0 * t3e0)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3d0) - (t3cf * (l.f5ef * p.p85))) / (t3d0 * t3d0)))) * t3e9) + (t3d4 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3d8) - (t3d7 * (l.f5ef * p.p85))) / (t3d8 * t3d8)))) * t3e6) + (t3dc * ((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t3e0) - (t3df * (l.f5ef * p.p85))) / (t3e0 * t3e0)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) {let t3ed: f64 = (l.f5eb * l.f5eb);let t3ee: f64 = (t3ed / l.f5e1);l.f64f = t3ee;let t3ef: f64 = (l.f5e7 / l.f645);let t3f0: f64 = (l.f5e1 / l.f64f);let t3f1: f64 = (t3f0).ln();let t3f2: f64 = (t3ef * t3f1);l.f793 = t3f2;}
        let t3f3: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f162 = t3f3;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t3f4: f64 = (l.f739 - l.f793);let t3f5: f64 = (p.p86 * t3f4);let t3f6: f64 = (t3f5 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t3f6, 0.0, 0.0, );let t3f7: f64 = (p.p86 * l.f793);let t3f8: f64 = (l.f5e7 - t3f7);(l.f5ed, l.f5ee, l.f5ef, ) = (t3f8, 0.0, 0.0, );let t3f9: f64 = (p.p85 - l.f601);let t3fa: f64 = (t3f9 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3fa, (-l.f602), (-l.f603), );let t3fb: f64 = (4.0 * p.p85);let t3fc: f64 = (t3fb * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3fc, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {
            let (t3fe, t3ff, t400,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3fd: f64 = (-l.f6f7);
        (t3fd, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3fe, t3ff, t400, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t401: f64 = (l.f6f3 * l.f6f3);let t402: f64 = (t401 + l.f6f7);let t403: f64 = (t402).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t403, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t403)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t403)), );let t404: f64 = (l.f6f3 + l.f6f7);let t405: f64 = (0.5 * t404);let t406: f64 = (p.p85 - t405);(l.f605, l.f606, l.f607, ) = (t406, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t407: f64 = (l.f605 - l.f5e7);let t408: f64 = (t407 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t408, l.f606, l.f607, );let t409: f64 = (4.0 * l.f5e7);let t40a: f64 = (t409 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t40a, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {
            let (t40c, t40d, t40e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t40b: f64 = (-l.f6f7);
        (t40b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t40c, t40d, t40e, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t40f: f64 = (l.f6f3 * l.f6f3);let t410: f64 = (t40f + l.f6f7);let t411: f64 = (t410).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t411, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t411)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t411)), );let t412: f64 = (l.f6f3 + l.f6f7);let t413: f64 = (0.5 * t412);let t414: f64 = (l.f5e7 + t413);(l.f5f1, l.f5f2, l.f5f3, ) = (t414, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t415: f64 = (p.p85 - l.f5ed);let t416: f64 = (t415 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t416, (-l.f5ee), (-l.f5ef), );let t417: f64 = (4.0 * p.p85);let t418: f64 = (t417 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t418, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {
            let (t41a, t41b, t41c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t419: f64 = (-l.f6f7);
        (t419, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t41a, t41b, t41c, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t41d: f64 = (l.f6f3 * l.f6f3);let t41e: f64 = (t41d + l.f6f7);let t41f: f64 = (t41e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t41f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t41f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t41f)), );let t420: f64 = (l.f6f3 + l.f6f7);let t421: f64 = (0.5 * t420);let t422: f64 = (p.p85 - t421);(l.f5ed, l.f5ee, l.f5ef, ) = (t422, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t423: f64 = (l.f5ed - l.f5e7);let t424: f64 = (t423 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t424, l.f5ee, l.f5ef, );let t425: f64 = (4.0 * l.f5e7);let t426: f64 = (t425 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t426, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {
            let (t428, t429, t42a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t427: f64 = (-l.f6f7);
        (t427, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t428, t429, t42a, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t42b: f64 = (l.f6f3 * l.f6f3);let t42c: f64 = (t42b + l.f6f7);let t42d: f64 = (t42c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t42d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t42d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t42d)), );let t42e: f64 = (l.f6f3 + l.f6f7);let t42f: f64 = (0.5 * t42e);let t430: f64 = (l.f5e7 + t42f);(l.f5ed, l.f5ee, l.f5ef, ) = (t430, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );}
        let t431: f64 = (l.f739 / l.f5f1);let t432: f64 = (l.f5f1 - l.f5ed);let t433: f64 = (l.f793 * t432);let t434: f64 = (l.f5ed * p.p85);let t435: f64 = (t433 / t434);let t436: f64 = (t431 + t435);let t437: f64 = (l.f645 * t436);let t438: f64 = (t437).abs();let t439: f64 = if t438 < 230.25850929940458 { 1.0 } else { 0.0 };l.f164 = t439;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f164 != 0.0)) {let t43a: f64 = (l.f739 / l.f5f1);let t43b: f64 = (l.f5f1 - l.f5ed);let t43c: f64 = (l.f793 * t43b);let t43d: f64 = (l.f5ed * p.p85);let t43e: f64 = (t43c / t43d);let t43f: f64 = (t43a + t43e);let t440: f64 = (l.f645 * t43f);let t441: f64 = (t440).exp();(l.f53a, l.f53b, l.f53c, ) = (t441, (t441 * (l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t43d) - (t43c * (l.f5ee * p.p85))) / (t43d * t43d))))), (t441 * (l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t43d) - (t43c * (l.f5ef * p.p85))) / (t43d * t43d))))), );}
        let t442: f64 = (l.f739 / l.f5f1);let t443: f64 = (l.f5f1 - l.f5ed);let t444: f64 = (l.f793 * t443);let t445: f64 = (l.f5ed * p.p85);let t446: f64 = (t444 / t445);let t447: f64 = (t442 + t446);let t448: f64 = (l.f645 * t447);let t449: f64 = (-230.25850929940458);let t44a: f64 = if t448 < t449 { 1.0 } else { 0.0 };l.f166 = t44a;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f164 == 0.0)) && (l.f166 != 0.0)) {let t44b: f64 = (-230.25850929940458);let t44c: f64 = (l.f739 / l.f5f1);let t44d: f64 = (l.f5f1 - l.f5ed);let t44e: f64 = (l.f793 * t44d);let t44f: f64 = (l.f5ed * p.p85);let t450: f64 = (t44e / t44f);let t451: f64 = (t44c + t450);let t452: f64 = (l.f645 * t451);let t453: f64 = (t44b - t452);let t454: f64 = (-230.25850929940458);let t455: f64 = (l.f739 / l.f5f1);let t456: f64 = (l.f5f1 - l.f5ed);let t457: f64 = (l.f793 * t456);let t458: f64 = (l.f5ed * p.p85);let t459: f64 = (t457 / t458);let t45a: f64 = (t455 + t459);let t45b: f64 = (l.f645 * t45a);let t45c: f64 = (t454 - t45b);let t45d: f64 = (-230.25850929940458);let t45e: f64 = (l.f739 / l.f5f1);let t45f: f64 = (l.f5f1 - l.f5ed);let t460: f64 = (l.f793 * t45f);let t461: f64 = (l.f5ed * p.p85);let t462: f64 = (t460 / t461);let t463: f64 = (t45e + t462);let t464: f64 = (l.f645 * t463);let t465: f64 = (t45d - t464);let t466: f64 = (t465 * 0.3333333333333333);let t467: f64 = (1.0 + t466);let t468: f64 = (t45c * t467);let t469: f64 = (0.5 * t468);let t46a: f64 = (1.0 + t469);let t46b: f64 = (t453 * t46a);let t46c: f64 = (1.0 + t46b);let t46d: f64 = (1e-100 / t46c);(l.f53a, l.f53b, l.f53c, ) = (t46d, (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t44f) - (t44e * (l.f5ee * p.p85))) / (t44f * t44f))))) * t46a) + (t453 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t458) - (t457 * (l.f5ee * p.p85))) / (t458 * t458))))) * t467) + (t45c * ((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t461) - (t460 * (l.f5ee * p.p85))) / (t461 * t461))))) * 0.3333333333333333))))))) / (t46c * t46c))), (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t44f) - (t44e * (l.f5ef * p.p85))) / (t44f * t44f))))) * t46a) + (t453 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t458) - (t457 * (l.f5ef * p.p85))) / (t458 * t458))))) * t467) + (t45c * ((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t461) - (t460 * (l.f5ef * p.p85))) / (t461 * t461))))) * 0.3333333333333333))))))) / (t46c * t46c))), );}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f164 == 0.0)) && (l.f166 == 0.0)) {let t46e: f64 = (l.f739 / l.f5f1);let t46f: f64 = (l.f5f1 - l.f5ed);let t470: f64 = (l.f793 * t46f);let t471: f64 = (l.f5ed * p.p85);let t472: f64 = (t470 / t471);let t473: f64 = (t46e + t472);let t474: f64 = (l.f645 * t473);let t475: f64 = (t474 - 230.25850929940458);let t476: f64 = (l.f739 / l.f5f1);let t477: f64 = (l.f5f1 - l.f5ed);let t478: f64 = (l.f793 * t477);let t479: f64 = (l.f5ed * p.p85);let t47a: f64 = (t478 / t479);let t47b: f64 = (t476 + t47a);let t47c: f64 = (l.f645 * t47b);let t47d: f64 = (t47c - 230.25850929940458);let t47e: f64 = (l.f739 / l.f5f1);let t47f: f64 = (l.f5f1 - l.f5ed);let t480: f64 = (l.f793 * t47f);let t481: f64 = (l.f5ed * p.p85);let t482: f64 = (t480 / t481);let t483: f64 = (t47e + t482);let t484: f64 = (l.f645 * t483);let t485: f64 = (t484 - 230.25850929940458);let t486: f64 = (t485 * 0.3333333333333333);let t487: f64 = (1.0 + t486);let t488: f64 = (t47d * t487);let t489: f64 = (0.5 * t488);let t48a: f64 = (1.0 + t489);let t48b: f64 = (t475 * t48a);let t48c: f64 = (1.0 + t48b);let t48d: f64 = (1e100 * t48c);(l.f53a, l.f53b, l.f53c, ) = (t48d, (1e100 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t471) - (t470 * (l.f5ee * p.p85))) / (t471 * t471)))) * t48a) + (t475 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t479) - (t478 * (l.f5ee * p.p85))) / (t479 * t479)))) * t487) + (t47d * ((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t481) - (t480 * (l.f5ee * p.p85))) / (t481 * t481)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t471) - (t470 * (l.f5ef * p.p85))) / (t471 * t471)))) * t48a) + (t475 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t479) - (t478 * (l.f5ef * p.p85))) / (t479 * t479)))) * t487) + (t47d * ((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t481) - (t480 * (l.f5ef * p.p85))) / (t481 * t481)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) {let t48e: f64 = (l.f739 - l.f7b1);let t48f: f64 = (t48e * l.f645);let t490: f64 = (1.0 + t48f);let t491: f64 = (t490 * l.f89);let t492: f64 = (t491).sqrt();l.f825 = t492;let t493: f64 = (l.f5eb * l.f5eb);let t494: f64 = (t493 / l.f5df);l.f64f = t494;let t495: f64 = (l.f5e5 / l.f645);let t496: f64 = (l.f5df / l.f64f);let t497: f64 = (t496).ln();let t498: f64 = (t495 * t497);l.f793 = t498;}
        let t499: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f168 = t499;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t49a: f64 = (l.f7b1 - l.f793);let t49b: f64 = (p.p86 * t49a);let t49c: f64 = (t49b + l.f5e5);(l.f601, l.f602, l.f603, ) = (t49c, 0.0, 0.0, );let t49d: f64 = (p.p86 * l.f793);let t49e: f64 = (l.f5e5 - t49d);(l.f5ed, l.f5ee, l.f5ef, ) = (t49e, 0.0, 0.0, );let t49f: f64 = (p.p85 - l.f601);let t4a0: f64 = (t49f - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t4a0, (-l.f602), (-l.f603), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t4a1: f64 = (4.0 * p.p85);let t4a2: f64 = (t4a1 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t4a2, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {
            let (t4a4, t4a5, t4a6,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t4a3: f64 = (-l.f6f7);
        (t4a3, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t4a4, t4a5, t4a6, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t4a7: f64 = (l.f6f3 * l.f6f3);let t4a8: f64 = (t4a7 + l.f6f7);let t4a9: f64 = (t4a8).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t4a9, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t4a9)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t4a9)), );let t4aa: f64 = (l.f6f3 / l.f6f7);let t4ab: f64 = (1.0 + t4aa);let t4ac: f64 = (0.5 * t4ab);(l.f55, l.f56, l.f57, ) = (t4ac, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t4ad: f64 = (l.f6f3 + l.f6f7);let t4ae: f64 = (0.5 * t4ad);let t4af: f64 = (p.p85 - t4ae);(l.f605, l.f606, l.f607, ) = (t4af, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t4b0: f64 = (l.f605 - l.f5e5);let t4b1: f64 = (t4b0 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t4b1, l.f606, l.f607, );let t4b2: f64 = (4.0 * l.f5e5);let t4b3: f64 = (t4b2 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t4b3, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {
            let (t4b5, t4b6, t4b7,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t4b4: f64 = (-l.f6f7);
        (t4b4, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t4b5, t4b6, t4b7, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t4b8: f64 = (l.f6f3 * l.f6f3);let t4b9: f64 = (t4b8 + l.f6f7);let t4ba: f64 = (t4b9).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t4ba, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t4ba)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t4ba)), );let t0: f64 = (l.f6f3 / l.f6f7);let t1: f64 = (1.0 + t0);let t2: f64 = (0.5 * t1);(l.f51, l.f52, l.f53, ) = (t2, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t3: f64 = (l.f6f3 + l.f6f7);let t4: f64 = (0.5 * t3);let t5: f64 = (l.f5e5 + t4);(l.f5f1, l.f5f2, l.f5f3, ) = (t5, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t6: f64 = (p.p85 - l.f5ed);let t7: f64 = (t6 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t7, (-l.f5ee), (-l.f5ef), );let t8: f64 = (4.0 * p.p85);let t9: f64 = (t8 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t9, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {
            let (tb, tc, td,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let ta: f64 = (-l.f6f7);
        (ta, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tb, tc, td, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let te: f64 = (l.f6f3 * l.f6f3);let tf: f64 = (te + l.f6f7);let t10: f64 = (tf).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t10, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t10)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t10)), );let t11: f64 = (l.f6f3 + l.f6f7);let t12: f64 = (0.5 * t11);let t13: f64 = (p.p85 - t12);(l.f5ed, l.f5ee, l.f5ef, ) = (t13, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t14: f64 = (l.f5ed - l.f5e5);let t15: f64 = (t14 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t15, l.f5ee, l.f5ef, );let t16: f64 = (4.0 * l.f5e5);let t17: f64 = (t16 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t17, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {
            let (t19, t1a, t1b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t18: f64 = (-l.f6f7);
        (t18, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t19, t1a, t1b, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t1c: f64 = (l.f6f3 * l.f6f3);let t1d: f64 = (t1c + l.f6f7);let t1e: f64 = (t1d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1e)), );let t1f: f64 = (l.f6f3 + l.f6f7);let t20: f64 = (0.5 * t1f);let t21: f64 = (l.f5e5 + t20);(l.f5ed, l.f5ee, l.f5ef, ) = (t21, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t22: f64 = (p.p86 * l.f55);let t23: f64 = (t22 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t23, (((p.p86 * l.f56) * l.f51) + (t22 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t22 * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let t24: f64 = (l.f7b1 / l.f5f1);let t25: f64 = (l.f5f1 - l.f5ed);let t26: f64 = (l.f793 * t25);let t27: f64 = (l.f5ed * p.p85);let t28: f64 = (t26 / t27);let t29: f64 = (t24 + t28);let t2a: f64 = (l.f645 * t29);let t2b: f64 = (t2a).abs();let t2c: f64 = if t2b < 230.25850929940458 { 1.0 } else { 0.0 };l.f16a = t2c;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16a != 0.0)) {let t2d: f64 = (l.f7b1 / l.f5f1);let t2e: f64 = (l.f5f1 - l.f5ed);let t2f: f64 = (l.f793 * t2e);let t30: f64 = (l.f5ed * p.p85);let t31: f64 = (t2f / t30);let t32: f64 = (t2d + t31);let t33: f64 = (l.f645 * t32);let t34: f64 = (t33).exp();(l.f8a, l.f8b, l.f8c, ) = (t34, (t34 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t30) - (t2f * (l.f5ee * p.p85))) / (t30 * t30))))), (t34 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t30) - (t2f * (l.f5ef * p.p85))) / (t30 * t30))))), );}
        let t35: f64 = (l.f7b1 / l.f5f1);let t36: f64 = (l.f5f1 - l.f5ed);let t37: f64 = (l.f793 * t36);let t38: f64 = (l.f5ed * p.p85);let t39: f64 = (t37 / t38);let t3a: f64 = (t35 + t39);let t3b: f64 = (l.f645 * t3a);let t3c: f64 = (-230.25850929940458);let t3d: f64 = if t3b < t3c { 1.0 } else { 0.0 };l.f16c = t3d;
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16a == 0.0)) && (l.f16c != 0.0)) {let t3e: f64 = (-230.25850929940458);let t3f: f64 = (l.f7b1 / l.f5f1);let t40: f64 = (l.f5f1 - l.f5ed);let t41: f64 = (l.f793 * t40);let t42: f64 = (l.f5ed * p.p85);let t43: f64 = (t41 / t42);let t44: f64 = (t3f + t43);let t45: f64 = (l.f645 * t44);let t46: f64 = (t3e - t45);let t47: f64 = (-230.25850929940458);let t48: f64 = (l.f7b1 / l.f5f1);let t49: f64 = (l.f5f1 - l.f5ed);let t4a: f64 = (l.f793 * t49);let t4b: f64 = (l.f5ed * p.p85);let t4c: f64 = (t4a / t4b);let t4d: f64 = (t48 + t4c);let t4e: f64 = (l.f645 * t4d);let t4f: f64 = (t47 - t4e);let t50: f64 = (-230.25850929940458);let t51: f64 = (l.f7b1 / l.f5f1);let t52: f64 = (l.f5f1 - l.f5ed);let t53: f64 = (l.f793 * t52);let t54: f64 = (l.f5ed * p.p85);let t55: f64 = (t53 / t54);let t56: f64 = (t51 + t55);let t57: f64 = (l.f645 * t56);let t58: f64 = (t50 - t57);let t59: f64 = (t58 * 0.3333333333333333);let t5a: f64 = (1.0 + t59);let t5b: f64 = (t4f * t5a);let t5c: f64 = (0.5 * t5b);let t5d: f64 = (1.0 + t5c);let t5e: f64 = (t46 * t5d);let t5f: f64 = (1.0 + t5e);let t60: f64 = (1e-100 / t5f);(l.f8a, l.f8b, l.f8c, ) = (t60, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t42) - (t41 * (l.f5ee * p.p85))) / (t42 * t42))))) * t5d) + (t46 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t4b) - (t4a * (l.f5ee * p.p85))) / (t4b * t4b))))) * t5a) + (t4f * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t54) - (t53 * (l.f5ee * p.p85))) / (t54 * t54))))) * 0.3333333333333333))))))) / (t5f * t5f))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t42) - (t41 * (l.f5ef * p.p85))) / (t42 * t42))))) * t5d) + (t46 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t4b) - (t4a * (l.f5ef * p.p85))) / (t4b * t4b))))) * t5a) + (t4f * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t54) - (t53 * (l.f5ef * p.p85))) / (t54 * t54))))) * 0.3333333333333333))))))) / (t5f * t5f))), );}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16a == 0.0)) && (l.f16c == 0.0)) {let t61: f64 = (l.f7b1 / l.f5f1);let t62: f64 = (l.f5f1 - l.f5ed);let t63: f64 = (l.f793 * t62);let t64: f64 = (l.f5ed * p.p85);let t65: f64 = (t63 / t64);let t66: f64 = (t61 + t65);let t67: f64 = (l.f645 * t66);let t68: f64 = (t67 - 230.25850929940458);let t69: f64 = (l.f7b1 / l.f5f1);let t6a: f64 = (l.f5f1 - l.f5ed);let t6b: f64 = (l.f793 * t6a);let t6c: f64 = (l.f5ed * p.p85);let t6d: f64 = (t6b / t6c);let t6e: f64 = (t69 + t6d);let t6f: f64 = (l.f645 * t6e);let t70: f64 = (t6f - 230.25850929940458);let t71: f64 = (l.f7b1 / l.f5f1);let t72: f64 = (l.f5f1 - l.f5ed);let t73: f64 = (l.f793 * t72);let t74: f64 = (l.f5ed * p.p85);let t75: f64 = (t73 / t74);let t76: f64 = (t71 + t75);let t77: f64 = (l.f645 * t76);let t78: f64 = (t77 - 230.25850929940458);let t79: f64 = (t78 * 0.3333333333333333);let t7a: f64 = (1.0 + t79);let t7b: f64 = (t70 * t7a);let t7c: f64 = (0.5 * t7b);let t7d: f64 = (1.0 + t7c);let t7e: f64 = (t68 * t7d);let t7f: f64 = (1.0 + t7e);let t80: f64 = (1e100 * t7f);(l.f8a, l.f8b, l.f8c, ) = (t80, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t64) - (t63 * (l.f5ee * p.p85))) / (t64 * t64)))) * t7d) + (t68 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t6c) - (t6b * (l.f5ee * p.p85))) / (t6c * t6c)))) * t7a) + (t70 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t74) - (t73 * (l.f5ee * p.p85))) / (t74 * t74)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t64) - (t63 * (l.f5ef * p.p85))) / (t64 * t64)))) * t7d) + (t68 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t6c) - (t6b * (l.f5ef * p.p85))) / (t6c * t6c)))) * t7a) + (t70 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t74) - (t73 * (l.f5ef * p.p85))) / (t74 * t74)))) * 0.3333333333333333))))))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) {let t81: f64 = (l.f7b1 * l.f5b);let t82: f64 = (l.f5f1 - t81);let t83: f64 = (l.f5f1 * l.f5f1);let t84: f64 = (t82 / t83);let t85: f64 = (l.f793 * l.f5b);let t86: f64 = (l.f5ed * p.p85);let t87: f64 = (t85 / t86);let t88: f64 = (t84 + t87);let t89: f64 = (l.f645 * t88);(l.f61, l.f62, l.f63, ) = (t89, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t83) - (t82 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t83 * t83)) + ((((l.f793 * l.f5c) * t86) - (t85 * (l.f5ee * p.p85))) / (t86 * t86)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t83) - (t82 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t83 * t83)) + ((((l.f793 * l.f5d) * t86) - (t85 * (l.f5ef * p.p85))) / (t86 * t86)))), );let t8a: f64 = (l.f739 - l.f7b1);let t8b: f64 = (t8a * l.f61);let t8c: f64 = (1.0 + t8b);let t8d: f64 = (t8c * l.f8a);(l.f536, l.f537, l.f538, ) = (t8d, (((t8a * l.f62) * l.f8a) + (t8c * l.f8b)), (((t8a * l.f63) * l.f8a) + (t8c * l.f8c)), );let t8e: f64 = (l.f5eb * l.f5eb);let t8f: f64 = (t8e / l.f5e3);l.f64f = t8f;let t90: f64 = (l.f5e9 / l.f645);let t91: f64 = (l.f5e3 / l.f64f);let t92: f64 = (t91).ln();let t93: f64 = (t90 * t92);l.f793 = t93;}
        let t94: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f16e = t94;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let t95: f64 = (l.f7b1 - l.f793);let t96: f64 = (p.p86 * t95);let t97: f64 = (t96 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t97, 0.0, 0.0, );let t98: f64 = (p.p86 * l.f793);let t99: f64 = (l.f5e9 - t98);(l.f5ed, l.f5ee, l.f5ef, ) = (t99, 0.0, 0.0, );let t9a: f64 = (p.p85 - l.f601);let t9b: f64 = (t9a - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t9b, (-l.f602), (-l.f603), );let t9c: f64 = (4.0 * p.p85);let t9d: f64 = (t9c * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t9d, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {
            let (t9f, ta0, ta1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t9e: f64 = (-l.f6f7);
        (t9e, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t9f, ta0, ta1, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let ta2: f64 = (l.f6f3 * l.f6f3);let ta3: f64 = (ta2 + l.f6f7);let ta4: f64 = (ta3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (ta4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * ta4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * ta4)), );let ta5: f64 = (l.f6f3 / l.f6f7);let ta6: f64 = (1.0 + ta5);let ta7: f64 = (0.5 * ta6);(l.f55, l.f56, l.f57, ) = (ta7, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let ta8: f64 = (l.f6f3 + l.f6f7);let ta9: f64 = (0.5 * ta8);let taa: f64 = (p.p85 - ta9);(l.f605, l.f606, l.f607, ) = (taa, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let tab: f64 = (l.f605 - l.f5e9);let tac: f64 = (tab - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tac, l.f606, l.f607, );let tad: f64 = (4.0 * l.f5e9);let tae: f64 = (tad * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tae, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {
            let (tb0, tb1, tb2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let taf: f64 = (-l.f6f7);
        (taf, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tb0, tb1, tb2, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let tb3: f64 = (l.f6f3 * l.f6f3);let tb4: f64 = (tb3 + l.f6f7);let tb5: f64 = (tb4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tb5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tb5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tb5)), );let tb6: f64 = (l.f6f3 / l.f6f7);let tb7: f64 = (1.0 + tb6);let tb8: f64 = (0.5 * tb7);(l.f51, l.f52, l.f53, ) = (tb8, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let tb9: f64 = (l.f6f3 + l.f6f7);let tba: f64 = (0.5 * tb9);let tbb: f64 = (l.f5e9 + tba);(l.f5f1, l.f5f2, l.f5f3, ) = (tbb, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let tbc: f64 = (p.p85 - l.f5ed);let tbd: f64 = (tbc - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tbd, (-l.f5ee), (-l.f5ef), );let tbe: f64 = (4.0 * p.p85);let tbf: f64 = (tbe * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tbf, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {
            let (tc1, tc2, tc3,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tc0: f64 = (-l.f6f7);
        (tc0, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tc1, tc2, tc3, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let tc4: f64 = (l.f6f3 * l.f6f3);let tc5: f64 = (tc4 + l.f6f7);let tc6: f64 = (tc5).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tc6, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tc6)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tc6)), );let tc7: f64 = (l.f6f3 + l.f6f7);let tc8: f64 = (0.5 * tc7);let tc9: f64 = (p.p85 - tc8);(l.f5ed, l.f5ee, l.f5ef, ) = (tc9, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let tca: f64 = (l.f5ed - l.f5e9);let tcb: f64 = (tca - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tcb, l.f5ee, l.f5ef, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let tcc: f64 = (4.0 * l.f5e9);let tcd: f64 = (tcc * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tcd, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {
            let (tcf, td0, td1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tce: f64 = (-l.f6f7);
        (tce, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tcf, td0, td1, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let td2: f64 = (l.f6f3 * l.f6f3);let td3: f64 = (td2 + l.f6f7);let td4: f64 = (td3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (td4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * td4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * td4)), );let td5: f64 = (l.f6f3 + l.f6f7);let td6: f64 = (0.5 * td5);let td7: f64 = (l.f5e9 + td6);(l.f5ed, l.f5ee, l.f5ef, ) = (td7, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let td8: f64 = (p.p86 * l.f55);let td9: f64 = (td8 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (td9, (((p.p86 * l.f56) * l.f51) + (td8 * l.f52)), (((p.p86 * l.f57) * l.f51) + (td8 * l.f53)), );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );}
        let tda: f64 = (l.f7b1 / l.f5f1);let tdb: f64 = (l.f5f1 - l.f5ed);let tdc: f64 = (l.f793 * tdb);let tdd: f64 = (l.f5ed * p.p85);let tde: f64 = (tdc / tdd);let tdf: f64 = (tda + tde);let te0: f64 = (l.f645 * tdf);let te1: f64 = (te0).abs();let te2: f64 = if te1 < 230.25850929940458 { 1.0 } else { 0.0 };l.f170 = te2;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f170 != 0.0)) {let te3: f64 = (l.f7b1 / l.f5f1);let te4: f64 = (l.f5f1 - l.f5ed);let te5: f64 = (l.f793 * te4);let te6: f64 = (l.f5ed * p.p85);let te7: f64 = (te5 / te6);let te8: f64 = (te3 + te7);let te9: f64 = (l.f645 * te8);let tea: f64 = (te9).exp();(l.f93, l.f94, l.f95, ) = (tea, (tea * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * te6) - (te5 * (l.f5ee * p.p85))) / (te6 * te6))))), (tea * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * te6) - (te5 * (l.f5ef * p.p85))) / (te6 * te6))))), );}
        let teb: f64 = (l.f7b1 / l.f5f1);let tec: f64 = (l.f5f1 - l.f5ed);let ted: f64 = (l.f793 * tec);let tee: f64 = (l.f5ed * p.p85);let tef: f64 = (ted / tee);let tf0: f64 = (teb + tef);let tf1: f64 = (l.f645 * tf0);let tf2: f64 = (-230.25850929940458);let tf3: f64 = if tf1 < tf2 { 1.0 } else { 0.0 };l.f172 = tf3;
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f170 == 0.0)) && (l.f172 != 0.0)) {let tf4: f64 = (-230.25850929940458);let tf5: f64 = (l.f7b1 / l.f5f1);let tf6: f64 = (l.f5f1 - l.f5ed);let tf7: f64 = (l.f793 * tf6);let tf8: f64 = (l.f5ed * p.p85);let tf9: f64 = (tf7 / tf8);let tfa: f64 = (tf5 + tf9);let tfb: f64 = (l.f645 * tfa);let tfc: f64 = (tf4 - tfb);let tfd: f64 = (-230.25850929940458);let tfe: f64 = (l.f7b1 / l.f5f1);let tff: f64 = (l.f5f1 - l.f5ed);let t100: f64 = (l.f793 * tff);let t101: f64 = (l.f5ed * p.p85);let t102: f64 = (t100 / t101);let t103: f64 = (tfe + t102);let t104: f64 = (l.f645 * t103);let t105: f64 = (tfd - t104);let t106: f64 = (-230.25850929940458);let t107: f64 = (l.f7b1 / l.f5f1);let t108: f64 = (l.f5f1 - l.f5ed);let t109: f64 = (l.f793 * t108);let t10a: f64 = (l.f5ed * p.p85);let t10b: f64 = (t109 / t10a);let t10c: f64 = (t107 + t10b);let t10d: f64 = (l.f645 * t10c);let t10e: f64 = (t106 - t10d);let t10f: f64 = (t10e * 0.3333333333333333);let t110: f64 = (1.0 + t10f);let t111: f64 = (t105 * t110);let t112: f64 = (0.5 * t111);let t113: f64 = (1.0 + t112);let t114: f64 = (tfc * t113);let t115: f64 = (1.0 + t114);let t116: f64 = (1e-100 / t115);(l.f93, l.f94, l.f95, ) = (t116, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tf8) - (tf7 * (l.f5ee * p.p85))) / (tf8 * tf8))))) * t113) + (tfc * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t101) - (t100 * (l.f5ee * p.p85))) / (t101 * t101))))) * t110) + (t105 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t10a) - (t109 * (l.f5ee * p.p85))) / (t10a * t10a))))) * 0.3333333333333333))))))) / (t115 * t115))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tf8) - (tf7 * (l.f5ef * p.p85))) / (tf8 * tf8))))) * t113) + (tfc * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t101) - (t100 * (l.f5ef * p.p85))) / (t101 * t101))))) * t110) + (t105 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t10a) - (t109 * (l.f5ef * p.p85))) / (t10a * t10a))))) * 0.3333333333333333))))))) / (t115 * t115))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f170 == 0.0)) && (l.f172 == 0.0)) {let t117: f64 = (l.f7b1 / l.f5f1);let t118: f64 = (l.f5f1 - l.f5ed);let t119: f64 = (l.f793 * t118);let t11a: f64 = (l.f5ed * p.p85);let t11b: f64 = (t119 / t11a);let t11c: f64 = (t117 + t11b);let t11d: f64 = (l.f645 * t11c);let t11e: f64 = (t11d - 230.25850929940458);let t11f: f64 = (l.f7b1 / l.f5f1);let t120: f64 = (l.f5f1 - l.f5ed);let t121: f64 = (l.f793 * t120);let t122: f64 = (l.f5ed * p.p85);let t123: f64 = (t121 / t122);let t124: f64 = (t11f + t123);let t125: f64 = (l.f645 * t124);let t126: f64 = (t125 - 230.25850929940458);let t127: f64 = (l.f7b1 / l.f5f1);let t128: f64 = (l.f5f1 - l.f5ed);let t129: f64 = (l.f793 * t128);let t12a: f64 = (l.f5ed * p.p85);let t12b: f64 = (t129 / t12a);let t12c: f64 = (t127 + t12b);let t12d: f64 = (l.f645 * t12c);let t12e: f64 = (t12d - 230.25850929940458);let t12f: f64 = (t12e * 0.3333333333333333);let t130: f64 = (1.0 + t12f);let t131: f64 = (t126 * t130);let t132: f64 = (0.5 * t131);let t133: f64 = (1.0 + t132);let t134: f64 = (t11e * t133);let t135: f64 = (1.0 + t134);let t136: f64 = (1e100 * t135);(l.f93, l.f94, l.f95, ) = (t136, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t11a) - (t119 * (l.f5ee * p.p85))) / (t11a * t11a)))) * t133) + (t11e * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t122) - (t121 * (l.f5ee * p.p85))) / (t122 * t122)))) * t130) + (t126 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t12a) - (t129 * (l.f5ee * p.p85))) / (t12a * t12a)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t11a) - (t119 * (l.f5ef * p.p85))) / (t11a * t11a)))) * t133) + (t11e * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t122) - (t121 * (l.f5ef * p.p85))) / (t122 * t122)))) * t130) + (t126 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t12a) - (t129 * (l.f5ef * p.p85))) / (t12a * t12a)))) * 0.3333333333333333))))))), );}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) {let t137: f64 = (l.f7b1 * l.f5b);let t138: f64 = (l.f5f1 - t137);let t139: f64 = (l.f5f1 * l.f5f1);let t13a: f64 = (t138 / t139);let t13b: f64 = (l.f793 * l.f5b);let t13c: f64 = (l.f5ed * p.p85);let t13d: f64 = (t13b / t13c);let t13e: f64 = (t13a + t13d);let t13f: f64 = (l.f645 * t13e);(l.f61, l.f62, l.f63, ) = (t13f, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t139) - (t138 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t139 * t139)) + ((((l.f793 * l.f5c) * t13c) - (t13b * (l.f5ee * p.p85))) / (t13c * t13c)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t139) - (t138 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t139 * t139)) + ((((l.f793 * l.f5d) * t13c) - (t13b * (l.f5ef * p.p85))) / (t13c * t13c)))), );let t140: f64 = (l.f739 - l.f7b1);let t141: f64 = (t140 * l.f61);let t142: f64 = (1.0 + t141);let t143: f64 = (t142 * l.f93);(l.f53e, l.f53f, l.f540, ) = (t143, (((t140 * l.f62) * l.f93) + (t142 * l.f94)), (((t140 * l.f63) * l.f93) + (t142 * l.f95)), );let t144: f64 = (l.f5eb * l.f5eb);let t145: f64 = (t144 / l.f5e1);l.f64f = t145;let t146: f64 = (l.f5e7 / l.f645);let t147: f64 = (l.f5e1 / l.f64f);let t148: f64 = (t147).ln();let t149: f64 = (t146 * t148);l.f793 = t149;}
        let t14a: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f174 = t14a;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t14b: f64 = (l.f7b1 - l.f793);let t14c: f64 = (p.p86 * t14b);let t14d: f64 = (t14c + l.f5e7);(l.f601, l.f602, l.f603, ) = (t14d, 0.0, 0.0, );let t14e: f64 = (p.p86 * l.f793);let t14f: f64 = (l.f5e7 - t14e);(l.f5ed, l.f5ee, l.f5ef, ) = (t14f, 0.0, 0.0, );let t150: f64 = (p.p85 - l.f601);let t151: f64 = (t150 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t151, (-l.f602), (-l.f603), );let t152: f64 = (4.0 * p.p85);let t153: f64 = (t152 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t153, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {
            let (t155, t156, t157,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t154: f64 = (-l.f6f7);
        (t154, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t155, t156, t157, );
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t158: f64 = (l.f6f3 * l.f6f3);let t159: f64 = (t158 + l.f6f7);let t15a: f64 = (t159).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t15a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t15a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t15a)), );let t15b: f64 = (l.f6f3 / l.f6f7);let t15c: f64 = (1.0 + t15b);let t15d: f64 = (0.5 * t15c);(l.f55, l.f56, l.f57, ) = (t15d, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );let t15e: f64 = (l.f6f3 + l.f6f7);let t15f: f64 = (0.5 * t15e);let t160: f64 = (p.p85 - t15f);(l.f605, l.f606, l.f607, ) = (t160, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t161: f64 = (l.f605 - l.f5e7);let t162: f64 = (t161 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t162, l.f606, l.f607, );let t163: f64 = (4.0 * l.f5e7);let t164: f64 = (t163 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t164, 0.0, 0.0, );}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {
            let (t166, t167, t168,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t165: f64 = (-l.f6f7);
        (t165, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t166, t167, t168, );
        }
    }
}
