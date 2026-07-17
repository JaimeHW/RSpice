#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t2ae: f64 = (l.f739 / l.f5f1);let t2af: f64 = (l.f5f1 - l.f5ed);let t2b0: f64 = (l.f793 * t2af);let t2b1: f64 = (l.f5ed * p.p85);let t2b2: f64 = (t2b0 / t2b1);let t2b3: f64 = (t2ae + t2b2);let t2b4: f64 = (l.f645 * t2b3);let t2b5: f64 = (t2b4).abs();let t2b6: f64 = if t2b5 < 230.25850929940458 { 1.0 } else { 0.0 };l.f15e = t2b6;l.f15f = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15e != 0.0)) {let t2b7: f64 = (l.f739 / l.f5f1);let t2b8: f64 = (l.f5f1 - l.f5ed);let t2b9: f64 = (l.f793 * t2b8);let t2ba: f64 = (l.f5ed * p.p85);let t2bb: f64 = (t2b9 / t2ba);let t2bc: f64 = (t2b7 + t2bb);let t2bd: f64 = (l.f645 * t2bc);let t2be: f64 = (t2bd).exp();(l.f53e, l.f53f, l.f540, ) = (t2be, (t2be * (l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2ba) - (t2b9 * (l.f5ee * p.p85))) / (t2ba * t2ba))))), (t2be * (l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2ba) - (t2b9 * (l.f5ef * p.p85))) / (t2ba * t2ba))))), );l.f541 = 0.0;}
        let t2bf: f64 = (l.f739 / l.f5f1);let t2c0: f64 = (l.f5f1 - l.f5ed);let t2c1: f64 = (l.f793 * t2c0);let t2c2: f64 = (l.f5ed * p.p85);let t2c3: f64 = (t2c1 / t2c2);let t2c4: f64 = (t2bf + t2c3);let t2c5: f64 = (l.f645 * t2c4);let t2c6: f64 = (-230.25850929940458);let t2c7: f64 = if t2c5 < t2c6 { 1.0 } else { 0.0 };l.f160 = t2c7;l.f161 = 0.0;
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15e == 0.0)) && (l.f160 != 0.0)) {let t2c8: f64 = (-230.25850929940458);let t2c9: f64 = (l.f739 / l.f5f1);let t2ca: f64 = (l.f5f1 - l.f5ed);let t2cb: f64 = (l.f793 * t2ca);let t2cc: f64 = (l.f5ed * p.p85);let t2cd: f64 = (t2cb / t2cc);let t2ce: f64 = (t2c9 + t2cd);let t2cf: f64 = (l.f645 * t2ce);let t2d0: f64 = (t2c8 - t2cf);let t2d1: f64 = (-230.25850929940458);let t2d2: f64 = (l.f739 / l.f5f1);let t2d3: f64 = (l.f5f1 - l.f5ed);let t2d4: f64 = (l.f793 * t2d3);let t2d5: f64 = (l.f5ed * p.p85);let t2d6: f64 = (t2d4 / t2d5);let t2d7: f64 = (t2d2 + t2d6);let t2d8: f64 = (l.f645 * t2d7);let t2d9: f64 = (t2d1 - t2d8);let t2da: f64 = (-230.25850929940458);let t2db: f64 = (l.f739 / l.f5f1);let t2dc: f64 = (l.f5f1 - l.f5ed);let t2dd: f64 = (l.f793 * t2dc);let t2de: f64 = (l.f5ed * p.p85);let t2df: f64 = (t2dd / t2de);let t2e0: f64 = (t2db + t2df);let t2e1: f64 = (l.f645 * t2e0);let t2e2: f64 = (t2da - t2e1);let t2e3: f64 = (t2e2 * 0.3333333333333333);let t2e4: f64 = (1.0 + t2e3);let t2e5: f64 = (t2d9 * t2e4);let t2e6: f64 = (0.5 * t2e5);let t2e7: f64 = (1.0 + t2e6);let t2e8: f64 = (t2d0 * t2e7);let t2e9: f64 = (1.0 + t2e8);let t2ea: f64 = (1e-100 / t2e9);(l.f53e, l.f53f, l.f540, ) = (t2ea, (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2cc) - (t2cb * (l.f5ee * p.p85))) / (t2cc * t2cc))))) * t2e7) + (t2d0 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2d5) - (t2d4 * (l.f5ee * p.p85))) / (t2d5 * t2d5))))) * t2e4) + (t2d9 * ((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2de) - (t2dd * (l.f5ee * p.p85))) / (t2de * t2de))))) * 0.3333333333333333))))))) / (t2e9 * t2e9))), (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2cc) - (t2cb * (l.f5ef * p.p85))) / (t2cc * t2cc))))) * t2e7) + (t2d0 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2d5) - (t2d4 * (l.f5ef * p.p85))) / (t2d5 * t2d5))))) * t2e4) + (t2d9 * ((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2de) - (t2dd * (l.f5ef * p.p85))) / (t2de * t2de))))) * 0.3333333333333333))))))) / (t2e9 * t2e9))), );l.f541 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f15e == 0.0)) && (l.f160 == 0.0)) {let t2eb: f64 = (l.f739 / l.f5f1);let t2ec: f64 = (l.f5f1 - l.f5ed);let t2ed: f64 = (l.f793 * t2ec);let t2ee: f64 = (l.f5ed * p.p85);let t2ef: f64 = (t2ed / t2ee);let t2f0: f64 = (t2eb + t2ef);let t2f1: f64 = (l.f645 * t2f0);let t2f2: f64 = (t2f1 - 230.25850929940458);let t2f3: f64 = (l.f739 / l.f5f1);let t2f4: f64 = (l.f5f1 - l.f5ed);let t2f5: f64 = (l.f793 * t2f4);let t2f6: f64 = (l.f5ed * p.p85);let t2f7: f64 = (t2f5 / t2f6);let t2f8: f64 = (t2f3 + t2f7);let t2f9: f64 = (l.f645 * t2f8);let t2fa: f64 = (t2f9 - 230.25850929940458);let t2fb: f64 = (l.f739 / l.f5f1);let t2fc: f64 = (l.f5f1 - l.f5ed);let t2fd: f64 = (l.f793 * t2fc);let t2fe: f64 = (l.f5ed * p.p85);let t2ff: f64 = (t2fd / t2fe);let t300: f64 = (t2fb + t2ff);let t301: f64 = (l.f645 * t300);let t302: f64 = (t301 - 230.25850929940458);let t303: f64 = (t302 * 0.3333333333333333);let t304: f64 = (1.0 + t303);let t305: f64 = (t2fa * t304);let t306: f64 = (0.5 * t305);let t307: f64 = (1.0 + t306);let t308: f64 = (t2f2 * t307);let t309: f64 = (1.0 + t308);let t30a: f64 = (1e100 * t309);(l.f53e, l.f53f, l.f540, ) = (t30a, (1e100 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2ee) - (t2ed * (l.f5ee * p.p85))) / (t2ee * t2ee)))) * t307) + (t2f2 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2f6) - (t2f5 * (l.f5ee * p.p85))) / (t2f6 * t2f6)))) * t304) + (t2fa * ((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t2fe) - (t2fd * (l.f5ee * p.p85))) / (t2fe * t2fe)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2ee) - (t2ed * (l.f5ef * p.p85))) / (t2ee * t2ee)))) * t307) + (t2f2 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2f6) - (t2f5 * (l.f5ef * p.p85))) / (t2f6 * t2f6)))) * t304) + (t2fa * ((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t2fe) - (t2fd * (l.f5ef * p.p85))) / (t2fe * t2fe)))) * 0.3333333333333333))))))), );l.f541 = 0.0;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) {let t30b: f64 = (l.f5eb * l.f5eb);let t30c: f64 = (t30b / l.f5e1);l.f64f = t30c;l.f650 = 0.0;let t30d: f64 = (l.f5e7 / l.f645);let t30e: f64 = (l.f5e1 / l.f64f);let t30f: f64 = (t30e).ln();let t310: f64 = (t30d * t30f);l.f793 = t310;l.f794 = 0.0;}
        let t311: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f162 = t311;l.f163 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t312: f64 = (l.f739 - l.f793);let t313: f64 = (p.p86 * t312);let t314: f64 = (t313 + l.f5e7);(l.f601, l.f602, l.f603, ) = (t314, 0.0, 0.0, );l.f604 = 0.0;let t315: f64 = (p.p86 * l.f793);let t316: f64 = (l.f5e7 - t315);(l.f5ed, l.f5ee, l.f5ef, ) = (t316, 0.0, 0.0, );l.f5f0 = 0.0;let t317: f64 = (p.p85 - l.f601);let t318: f64 = (t317 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t318, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t319: f64 = (4.0 * p.p85);let t31a: f64 = (t319 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t31a, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {
            let (t31c, t31d, t31e,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t31b: f64 = (-l.f6f7);
        (t31b, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t31c, t31d, t31e, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t31f: f64 = (l.f6f3 * l.f6f3);let t320: f64 = (t31f + l.f6f7);let t321: f64 = (t320).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t321, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t321)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t321)), );l.f6fa = 0.0;let t322: f64 = (l.f6f3 + l.f6f7);let t323: f64 = (0.5 * t322);let t324: f64 = (p.p85 - t323);(l.f605, l.f606, l.f607, ) = (t324, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t325: f64 = (l.f605 - l.f5e7);let t326: f64 = (t325 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t326, l.f606, l.f607, );l.f6f6 = 0.0;let t327: f64 = (4.0 * l.f5e7);let t328: f64 = (t327 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t328, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {
            let (t32a, t32b, t32c,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t329: f64 = (-l.f6f7);
        (t329, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t32a, t32b, t32c, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t32d: f64 = (l.f6f3 * l.f6f3);let t32e: f64 = (t32d + l.f6f7);let t32f: f64 = (t32e).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t32f, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t32f)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t32f)), );l.f6fa = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t330: f64 = (l.f6f3 + l.f6f7);let t331: f64 = (0.5 * t330);let t332: f64 = (l.f5e7 + t331);(l.f5f1, l.f5f2, l.f5f3, ) = (t332, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t333: f64 = (p.p85 - l.f5ed);let t334: f64 = (t333 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t334, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t335: f64 = (4.0 * p.p85);let t336: f64 = (t335 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t336, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {
            let (t338, t339, t33a,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t337: f64 = (-l.f6f7);
        (t337, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t338, t339, t33a, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t33b: f64 = (l.f6f3 * l.f6f3);let t33c: f64 = (t33b + l.f6f7);let t33d: f64 = (t33c).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t33d, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t33d)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t33d)), );l.f6fa = 0.0;let t33e: f64 = (l.f6f3 + l.f6f7);let t33f: f64 = (0.5 * t33e);let t340: f64 = (p.p85 - t33f);(l.f5ed, l.f5ee, l.f5ef, ) = (t340, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t341: f64 = (l.f5ed - l.f5e7);let t342: f64 = (t341 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t342, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t343: f64 = (4.0 * l.f5e7);let t344: f64 = (t343 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t344, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {
            let (t346, t347, t348,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t345: f64 = (-l.f6f7);
        (t345, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t346, t347, t348, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 != 0.0)) {let t349: f64 = (l.f6f3 * l.f6f3);let t34a: f64 = (t349 + l.f6f7);let t34b: f64 = (t34a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t34b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t34b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t34b)), );l.f6fa = 0.0;let t34c: f64 = (l.f6f3 + l.f6f7);let t34d: f64 = (0.5 * t34c);let t34e: f64 = (l.f5e7 + t34d);(l.f5ed, l.f5ee, l.f5ef, ) = (t34e, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f162 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );l.f5f4 = 0.0;}
        let t34f: f64 = (l.f739 / l.f5f1);let t350: f64 = (l.f5f1 - l.f5ed);let t351: f64 = (l.f793 * t350);let t352: f64 = (l.f5ed * p.p85);let t353: f64 = (t351 / t352);let t354: f64 = (t34f + t353);let t355: f64 = (l.f645 * t354);let t356: f64 = (t355).abs();let t357: f64 = if t356 < 230.25850929940458 { 1.0 } else { 0.0 };l.f164 = t357;l.f165 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f164 != 0.0)) {let t358: f64 = (l.f739 / l.f5f1);let t359: f64 = (l.f5f1 - l.f5ed);let t35a: f64 = (l.f793 * t359);let t35b: f64 = (l.f5ed * p.p85);let t35c: f64 = (t35a / t35b);let t35d: f64 = (t358 + t35c);let t35e: f64 = (l.f645 * t35d);let t35f: f64 = (t35e).exp();(l.f53a, l.f53b, l.f53c, ) = (t35f, (t35f * (l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t35b) - (t35a * (l.f5ee * p.p85))) / (t35b * t35b))))), (t35f * (l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t35b) - (t35a * (l.f5ef * p.p85))) / (t35b * t35b))))), );l.f53d = 0.0;}
        let t360: f64 = (l.f739 / l.f5f1);let t361: f64 = (l.f5f1 - l.f5ed);let t362: f64 = (l.f793 * t361);let t363: f64 = (l.f5ed * p.p85);let t364: f64 = (t362 / t363);let t365: f64 = (t360 + t364);let t366: f64 = (l.f645 * t365);let t367: f64 = (-230.25850929940458);let t368: f64 = if t366 < t367 { 1.0 } else { 0.0 };l.f166 = t368;l.f167 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f164 == 0.0)) && (l.f166 != 0.0)) {let t369: f64 = (-230.25850929940458);let t36a: f64 = (l.f739 / l.f5f1);let t36b: f64 = (l.f5f1 - l.f5ed);let t36c: f64 = (l.f793 * t36b);let t36d: f64 = (l.f5ed * p.p85);let t36e: f64 = (t36c / t36d);let t36f: f64 = (t36a + t36e);let t370: f64 = (l.f645 * t36f);let t371: f64 = (t369 - t370);let t372: f64 = (-230.25850929940458);let t373: f64 = (l.f739 / l.f5f1);let t374: f64 = (l.f5f1 - l.f5ed);let t375: f64 = (l.f793 * t374);let t376: f64 = (l.f5ed * p.p85);let t377: f64 = (t375 / t376);let t378: f64 = (t373 + t377);let t379: f64 = (l.f645 * t378);let t37a: f64 = (t372 - t379);let t37b: f64 = (-230.25850929940458);let t37c: f64 = (l.f739 / l.f5f1);let t37d: f64 = (l.f5f1 - l.f5ed);let t37e: f64 = (l.f793 * t37d);let t37f: f64 = (l.f5ed * p.p85);let t380: f64 = (t37e / t37f);let t381: f64 = (t37c + t380);let t382: f64 = (l.f645 * t381);let t383: f64 = (t37b - t382);let t384: f64 = (t383 * 0.3333333333333333);let t385: f64 = (1.0 + t384);let t386: f64 = (t37a * t385);let t387: f64 = (0.5 * t386);let t388: f64 = (1.0 + t387);let t389: f64 = (t371 * t388);let t38a: f64 = (1.0 + t389);let t38b: f64 = (1e-100 / t38a);(l.f53a, l.f53b, l.f53c, ) = (t38b, (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t36d) - (t36c * (l.f5ee * p.p85))) / (t36d * t36d))))) * t388) + (t371 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t376) - (t375 * (l.f5ee * p.p85))) / (t376 * t376))))) * t385) + (t37a * ((-(l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t37f) - (t37e * (l.f5ee * p.p85))) / (t37f * t37f))))) * 0.3333333333333333))))))) / (t38a * t38a))), (-((1e-100 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t36d) - (t36c * (l.f5ef * p.p85))) / (t36d * t36d))))) * t388) + (t371 * (0.5 * (((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t376) - (t375 * (l.f5ef * p.p85))) / (t376 * t376))))) * t385) + (t37a * ((-(l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t37f) - (t37e * (l.f5ef * p.p85))) / (t37f * t37f))))) * 0.3333333333333333))))))) / (t38a * t38a))), );l.f53d = 0.0;}
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 != 0.0)) && (l.f164 == 0.0)) && (l.f166 == 0.0)) {let t38c: f64 = (l.f739 / l.f5f1);let t38d: f64 = (l.f5f1 - l.f5ed);let t38e: f64 = (l.f793 * t38d);let t38f: f64 = (l.f5ed * p.p85);let t390: f64 = (t38e / t38f);let t391: f64 = (t38c + t390);let t392: f64 = (l.f645 * t391);let t393: f64 = (t392 - 230.25850929940458);let t394: f64 = (l.f739 / l.f5f1);let t395: f64 = (l.f5f1 - l.f5ed);let t396: f64 = (l.f793 * t395);let t397: f64 = (l.f5ed * p.p85);let t398: f64 = (t396 / t397);let t399: f64 = (t394 + t398);let t39a: f64 = (l.f645 * t399);let t39b: f64 = (t39a - 230.25850929940458);let t39c: f64 = (l.f739 / l.f5f1);let t39d: f64 = (l.f5f1 - l.f5ed);let t39e: f64 = (l.f793 * t39d);let t39f: f64 = (l.f5ed * p.p85);let t3a0: f64 = (t39e / t39f);let t3a1: f64 = (t39c + t3a0);let t3a2: f64 = (l.f645 * t3a1);let t3a3: f64 = (t3a2 - 230.25850929940458);let t3a4: f64 = (t3a3 * 0.3333333333333333);let t3a5: f64 = (1.0 + t3a4);let t3a6: f64 = (t39b * t3a5);let t3a7: f64 = (0.5 * t3a6);let t3a8: f64 = (1.0 + t3a7);let t3a9: f64 = (t393 * t3a8);let t3aa: f64 = (1.0 + t3a9);let t3ab: f64 = (1e100 * t3aa);(l.f53a, l.f53b, l.f53c, ) = (t3ab, (1e100 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t38f) - (t38e * (l.f5ee * p.p85))) / (t38f * t38f)))) * t3a8) + (t393 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t397) - (t396 * (l.f5ee * p.p85))) / (t397 * t397)))) * t3a5) + (t39b * ((l.f645 * ((-((l.f739 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t39f) - (t39e * (l.f5ee * p.p85))) / (t39f * t39f)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t38f) - (t38e * (l.f5ef * p.p85))) / (t38f * t38f)))) * t3a8) + (t393 * (0.5 * (((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t397) - (t396 * (l.f5ef * p.p85))) / (t397 * t397)))) * t3a5) + (t39b * ((l.f645 * ((-((l.f739 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t39f) - (t39e * (l.f5ef * p.p85))) / (t39f * t39f)))) * 0.3333333333333333))))))), );l.f53d = 0.0;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) {let t3ac: f64 = (l.f739 - l.f7b1);let t3ad: f64 = (t3ac * l.f645);let t3ae: f64 = (1.0 + t3ad);let t3af: f64 = (t3ae * l.f89);let t3b0: f64 = (t3af).sqrt();l.f825 = t3b0;l.f826 = 0.0;let t3b1: f64 = (l.f5eb * l.f5eb);let t3b2: f64 = (t3b1 / l.f5df);l.f64f = t3b2;l.f650 = 0.0;let t3b3: f64 = (l.f5e5 / l.f645);let t3b4: f64 = (l.f5df / l.f64f);let t3b5: f64 = (t3b4).ln();let t3b6: f64 = (t3b3 * t3b5);l.f793 = t3b6;l.f794 = 0.0;}
        let t3b7: f64 = if l.f5e5 < p.p85 { 1.0 } else { 0.0 };l.f168 = t3b7;l.f169 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t3b8: f64 = (l.f7b1 - l.f793);let t3b9: f64 = (p.p86 * t3b8);let t3ba: f64 = (t3b9 + l.f5e5);(l.f601, l.f602, l.f603, ) = (t3ba, 0.0, 0.0, );l.f604 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t3bb: f64 = (p.p86 * l.f793);let t3bc: f64 = (l.f5e5 - t3bb);(l.f5ed, l.f5ee, l.f5ef, ) = (t3bc, 0.0, 0.0, );l.f5f0 = 0.0;let t3bd: f64 = (p.p85 - l.f601);let t3be: f64 = (t3bd - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3be, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t3bf: f64 = (4.0 * p.p85);let t3c0: f64 = (t3bf * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3c0, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {
            let (t3c2, t3c3, t3c4,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3c1: f64 = (-l.f6f7);
        (t3c1, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3c2, t3c3, t3c4, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t3c5: f64 = (l.f6f3 * l.f6f3);let t3c6: f64 = (t3c5 + l.f6f7);let t3c7: f64 = (t3c6).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3c7, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3c7)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3c7)), );l.f6fa = 0.0;let t3c8: f64 = (l.f6f3 / l.f6f7);let t3c9: f64 = (1.0 + t3c8);let t3ca: f64 = (0.5 * t3c9);(l.f55, l.f56, l.f57, ) = (t3ca, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;let t3cb: f64 = (l.f6f3 + l.f6f7);let t3cc: f64 = (0.5 * t3cb);let t3cd: f64 = (p.p85 - t3cc);(l.f605, l.f606, l.f607, ) = (t3cd, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t3ce: f64 = (l.f605 - l.f5e5);let t3cf: f64 = (t3ce - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t3cf, l.f606, l.f607, );l.f6f6 = 0.0;let t3d0: f64 = (4.0 * l.f5e5);let t3d1: f64 = (t3d0 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t3d1, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {
            let (t3d3, t3d4, t3d5,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3d2: f64 = (-l.f6f7);
        (t3d2, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3d3, t3d4, t3d5, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t3d6: f64 = (l.f6f3 * l.f6f3);let t3d7: f64 = (t3d6 + l.f6f7);let t3d8: f64 = (t3d7).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3d8, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3d8)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3d8)), );l.f6fa = 0.0;let t0: f64 = (l.f6f3 / l.f6f7);let t1: f64 = (1.0 + t0);let t2: f64 = (0.5 * t1);(l.f51, l.f52, l.f53, ) = (t2, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t3: f64 = (l.f6f3 + l.f6f7);let t4: f64 = (0.5 * t3);let t5: f64 = (l.f5e5 + t4);(l.f5f1, l.f5f2, l.f5f3, ) = (t5, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t6: f64 = (p.p85 - l.f5ed);let t7: f64 = (t6 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t7, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t8: f64 = (4.0 * p.p85);let t9: f64 = (t8 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t9, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {
            let (tb, tc, td,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let ta: f64 = (-l.f6f7);
        (ta, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tb, tc, td, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let te: f64 = (l.f6f3 * l.f6f3);let tf: f64 = (te + l.f6f7);let t10: f64 = (tf).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t10, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t10)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t10)), );l.f6fa = 0.0;let t11: f64 = (l.f6f3 + l.f6f7);let t12: f64 = (0.5 * t11);let t13: f64 = (p.p85 - t12);(l.f5ed, l.f5ee, l.f5ef, ) = (t13, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t14: f64 = (l.f5ed - l.f5e5);let t15: f64 = (t14 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t15, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t16: f64 = (4.0 * l.f5e5);let t17: f64 = (t16 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t17, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {
            let (t19, t1a, t1b,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t18: f64 = (-l.f6f7);
        (t18, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t19, t1a, t1b, );l.f6fa = 0.0;
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 != 0.0)) {let t1c: f64 = (l.f6f3 * l.f6f3);let t1d: f64 = (t1c + l.f6f7);let t1e: f64 = (t1d).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t1e, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t1e)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t1e)), );l.f6fa = 0.0;let t1f: f64 = (l.f6f3 + l.f6f7);let t20: f64 = (0.5 * t1f);let t21: f64 = (l.f5e5 + t20);(l.f5ed, l.f5ee, l.f5ef, ) = (t21, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t22: f64 = (p.p86 * l.f55);let t23: f64 = (t22 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t23, (((p.p86 * l.f56) * l.f51) + (t22 * l.f52)), (((p.p86 * l.f57) * l.f51) + (t22 * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f168 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e5, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e5, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
        let t24: f64 = (l.f7b1 / l.f5f1);let t25: f64 = (l.f5f1 - l.f5ed);let t26: f64 = (l.f793 * t25);let t27: f64 = (l.f5ed * p.p85);let t28: f64 = (t26 / t27);let t29: f64 = (t24 + t28);let t2a: f64 = (l.f645 * t29);let t2b: f64 = (t2a).abs();let t2c: f64 = if t2b < 230.25850929940458 { 1.0 } else { 0.0 };l.f16a = t2c;l.f16b = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16a != 0.0)) {let t2d: f64 = (l.f7b1 / l.f5f1);let t2e: f64 = (l.f5f1 - l.f5ed);let t2f: f64 = (l.f793 * t2e);let t30: f64 = (l.f5ed * p.p85);let t31: f64 = (t2f / t30);let t32: f64 = (t2d + t31);let t33: f64 = (l.f645 * t32);let t34: f64 = (t33).exp();(l.f8a, l.f8b, l.f8c, ) = (t34, (t34 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t30) - (t2f * (l.f5ee * p.p85))) / (t30 * t30))))), (t34 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t30) - (t2f * (l.f5ef * p.p85))) / (t30 * t30))))), );l.f8d = 0.0;}
        let t35: f64 = (l.f7b1 / l.f5f1);let t36: f64 = (l.f5f1 - l.f5ed);let t37: f64 = (l.f793 * t36);let t38: f64 = (l.f5ed * p.p85);let t39: f64 = (t37 / t38);let t3a: f64 = (t35 + t39);let t3b: f64 = (l.f645 * t3a);let t3c: f64 = (-230.25850929940458);let t3d: f64 = if t3b < t3c { 1.0 } else { 0.0 };l.f16c = t3d;l.f16d = 0.0;
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16a == 0.0)) && (l.f16c != 0.0)) {let t3e: f64 = (-230.25850929940458);let t3f: f64 = (l.f7b1 / l.f5f1);let t40: f64 = (l.f5f1 - l.f5ed);let t41: f64 = (l.f793 * t40);let t42: f64 = (l.f5ed * p.p85);let t43: f64 = (t41 / t42);let t44: f64 = (t3f + t43);let t45: f64 = (l.f645 * t44);let t46: f64 = (t3e - t45);let t47: f64 = (-230.25850929940458);let t48: f64 = (l.f7b1 / l.f5f1);let t49: f64 = (l.f5f1 - l.f5ed);let t4a: f64 = (l.f793 * t49);let t4b: f64 = (l.f5ed * p.p85);let t4c: f64 = (t4a / t4b);let t4d: f64 = (t48 + t4c);let t4e: f64 = (l.f645 * t4d);let t4f: f64 = (t47 - t4e);let t50: f64 = (-230.25850929940458);let t51: f64 = (l.f7b1 / l.f5f1);let t52: f64 = (l.f5f1 - l.f5ed);let t53: f64 = (l.f793 * t52);let t54: f64 = (l.f5ed * p.p85);let t55: f64 = (t53 / t54);let t56: f64 = (t51 + t55);let t57: f64 = (l.f645 * t56);let t58: f64 = (t50 - t57);let t59: f64 = (t58 * 0.3333333333333333);let t5a: f64 = (1.0 + t59);let t5b: f64 = (t4f * t5a);let t5c: f64 = (0.5 * t5b);let t5d: f64 = (1.0 + t5c);let t5e: f64 = (t46 * t5d);let t5f: f64 = (1.0 + t5e);let t60: f64 = (1e-100 / t5f);(l.f8a, l.f8b, l.f8c, ) = (t60, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t42) - (t41 * (l.f5ee * p.p85))) / (t42 * t42))))) * t5d) + (t46 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t4b) - (t4a * (l.f5ee * p.p85))) / (t4b * t4b))))) * t5a) + (t4f * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t54) - (t53 * (l.f5ee * p.p85))) / (t54 * t54))))) * 0.3333333333333333))))))) / (t5f * t5f))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t42) - (t41 * (l.f5ef * p.p85))) / (t42 * t42))))) * t5d) + (t46 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t4b) - (t4a * (l.f5ef * p.p85))) / (t4b * t4b))))) * t5a) + (t4f * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t54) - (t53 * (l.f5ef * p.p85))) / (t54 * t54))))) * 0.3333333333333333))))))) / (t5f * t5f))), );l.f8d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16a == 0.0)) && (l.f16c == 0.0)) {let t61: f64 = (l.f7b1 / l.f5f1);let t62: f64 = (l.f5f1 - l.f5ed);let t63: f64 = (l.f793 * t62);let t64: f64 = (l.f5ed * p.p85);let t65: f64 = (t63 / t64);let t66: f64 = (t61 + t65);let t67: f64 = (l.f645 * t66);let t68: f64 = (t67 - 230.25850929940458);let t69: f64 = (l.f7b1 / l.f5f1);let t6a: f64 = (l.f5f1 - l.f5ed);let t6b: f64 = (l.f793 * t6a);let t6c: f64 = (l.f5ed * p.p85);let t6d: f64 = (t6b / t6c);let t6e: f64 = (t69 + t6d);let t6f: f64 = (l.f645 * t6e);let t70: f64 = (t6f - 230.25850929940458);let t71: f64 = (l.f7b1 / l.f5f1);let t72: f64 = (l.f5f1 - l.f5ed);let t73: f64 = (l.f793 * t72);let t74: f64 = (l.f5ed * p.p85);let t75: f64 = (t73 / t74);let t76: f64 = (t71 + t75);let t77: f64 = (l.f645 * t76);let t78: f64 = (t77 - 230.25850929940458);let t79: f64 = (t78 * 0.3333333333333333);let t7a: f64 = (1.0 + t79);let t7b: f64 = (t70 * t7a);let t7c: f64 = (0.5 * t7b);let t7d: f64 = (1.0 + t7c);let t7e: f64 = (t68 * t7d);let t7f: f64 = (1.0 + t7e);let t80: f64 = (1e100 * t7f);(l.f8a, l.f8b, l.f8c, ) = (t80, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t64) - (t63 * (l.f5ee * p.p85))) / (t64 * t64)))) * t7d) + (t68 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t6c) - (t6b * (l.f5ee * p.p85))) / (t6c * t6c)))) * t7a) + (t70 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t74) - (t73 * (l.f5ee * p.p85))) / (t74 * t74)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t64) - (t63 * (l.f5ef * p.p85))) / (t64 * t64)))) * t7d) + (t68 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t6c) - (t6b * (l.f5ef * p.p85))) / (t6c * t6c)))) * t7a) + (t70 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t74) - (t73 * (l.f5ef * p.p85))) / (t74 * t74)))) * 0.3333333333333333))))))), );l.f8d = 0.0;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) {let t81: f64 = (l.f7b1 * l.f5b);let t82: f64 = (l.f5f1 - t81);let t83: f64 = (l.f5f1 * l.f5f1);let t84: f64 = (t82 / t83);let t85: f64 = (l.f793 * l.f5b);let t86: f64 = (l.f5ed * p.p85);let t87: f64 = (t85 / t86);let t88: f64 = (t84 + t87);let t89: f64 = (l.f645 * t88);(l.f61, l.f62, l.f63, ) = (t89, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t83) - (t82 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t83 * t83)) + ((((l.f793 * l.f5c) * t86) - (t85 * (l.f5ee * p.p85))) / (t86 * t86)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t83) - (t82 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t83 * t83)) + ((((l.f793 * l.f5d) * t86) - (t85 * (l.f5ef * p.p85))) / (t86 * t86)))), );l.f64 = 0.0;let t8a: f64 = (l.f739 - l.f7b1);let t8b: f64 = (t8a * l.f61);let t8c: f64 = (1.0 + t8b);let t8d: f64 = (t8c * l.f8a);(l.f536, l.f537, l.f538, ) = (t8d, (((t8a * l.f62) * l.f8a) + (t8c * l.f8b)), (((t8a * l.f63) * l.f8a) + (t8c * l.f8c)), );l.f539 = 0.0;let t8e: f64 = (l.f5eb * l.f5eb);let t8f: f64 = (t8e / l.f5e3);l.f64f = t8f;l.f650 = 0.0;let t90: f64 = (l.f5e9 / l.f645);let t91: f64 = (l.f5e3 / l.f64f);let t92: f64 = (t91).ln();let t93: f64 = (t90 * t92);l.f793 = t93;l.f794 = 0.0;}
        let t94: f64 = if l.f5e9 < p.p85 { 1.0 } else { 0.0 };l.f16e = t94;l.f16f = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let t95: f64 = (l.f7b1 - l.f793);let t96: f64 = (p.p86 * t95);let t97: f64 = (t96 + l.f5e9);(l.f601, l.f602, l.f603, ) = (t97, 0.0, 0.0, );l.f604 = 0.0;let t98: f64 = (p.p86 * l.f793);let t99: f64 = (l.f5e9 - t98);(l.f5ed, l.f5ee, l.f5ef, ) = (t99, 0.0, 0.0, );l.f5f0 = 0.0;let t9a: f64 = (p.p85 - l.f601);let t9b: f64 = (t9a - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t9b, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t9c: f64 = (4.0 * p.p85);let t9d: f64 = (t9c * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t9d, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {
            let (t9f, ta0, ta1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t9e: f64 = (-l.f6f7);
        (t9e, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t9f, ta0, ta1, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let ta2: f64 = (l.f6f3 * l.f6f3);let ta3: f64 = (ta2 + l.f6f7);let ta4: f64 = (ta3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (ta4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * ta4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * ta4)), );l.f6fa = 0.0;let ta5: f64 = (l.f6f3 / l.f6f7);let ta6: f64 = (1.0 + ta5);let ta7: f64 = (0.5 * ta6);(l.f55, l.f56, l.f57, ) = (ta7, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let ta8: f64 = (l.f6f3 + l.f6f7);let ta9: f64 = (0.5 * ta8);let taa: f64 = (p.p85 - ta9);(l.f605, l.f606, l.f607, ) = (taa, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let tab: f64 = (l.f605 - l.f5e9);let tac: f64 = (tab - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tac, l.f606, l.f607, );l.f6f6 = 0.0;let tad: f64 = (4.0 * l.f5e9);let tae: f64 = (tad * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tae, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {
            let (tb0, tb1, tb2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let taf: f64 = (-l.f6f7);
        (taf, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tb0, tb1, tb2, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let tb3: f64 = (l.f6f3 * l.f6f3);let tb4: f64 = (tb3 + l.f6f7);let tb5: f64 = (tb4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tb5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tb5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tb5)), );l.f6fa = 0.0;let tb6: f64 = (l.f6f3 / l.f6f7);let tb7: f64 = (1.0 + tb6);let tb8: f64 = (0.5 * tb7);(l.f51, l.f52, l.f53, ) = (tb8, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let tb9: f64 = (l.f6f3 + l.f6f7);let tba: f64 = (0.5 * tb9);let tbb: f64 = (l.f5e9 + tba);(l.f5f1, l.f5f2, l.f5f3, ) = (tbb, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let tbc: f64 = (p.p85 - l.f5ed);let tbd: f64 = (tbc - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tbd, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let tbe: f64 = (4.0 * p.p85);let tbf: f64 = (tbe * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tbf, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {
            let (tc1, tc2, tc3,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tc0: f64 = (-l.f6f7);
        (tc0, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tc1, tc2, tc3, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let tc4: f64 = (l.f6f3 * l.f6f3);let tc5: f64 = (tc4 + l.f6f7);let tc6: f64 = (tc5).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (tc6, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * tc6)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * tc6)), );l.f6fa = 0.0;let tc7: f64 = (l.f6f3 + l.f6f7);let tc8: f64 = (0.5 * tc7);let tc9: f64 = (p.p85 - tc8);(l.f5ed, l.f5ee, l.f5ef, ) = (tc9, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let tca: f64 = (l.f5ed - l.f5e9);let tcb: f64 = (tca - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (tcb, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let tcc: f64 = (4.0 * l.f5e9);let tcd: f64 = (tcc * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (tcd, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {
            let (tcf, td0, td1,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let tce: f64 = (-l.f6f7);
        (tce, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (tcf, td0, td1, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e != 0.0)) {let td2: f64 = (l.f6f3 * l.f6f3);let td3: f64 = (td2 + l.f6f7);let td4: f64 = (td3).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (td4, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * td4)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * td4)), );l.f6fa = 0.0;let td5: f64 = (l.f6f3 + l.f6f7);let td6: f64 = (0.5 * td5);let td7: f64 = (l.f5e9 + td6);(l.f5ed, l.f5ee, l.f5ef, ) = (td7, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let td8: f64 = (p.p86 * l.f55);let td9: f64 = (td8 * l.f51);(l.f5b, l.f5c, l.f5d, ) = (td9, (((p.p86 * l.f56) * l.f51) + (td8 * l.f52)), (((p.p86 * l.f57) * l.f51) + (td8 * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f16e == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e9, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e9, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tda: f64 = (l.f7b1 / l.f5f1);let tdb: f64 = (l.f5f1 - l.f5ed);let tdc: f64 = (l.f793 * tdb);let tdd: f64 = (l.f5ed * p.p85);let tde: f64 = (tdc / tdd);let tdf: f64 = (tda + tde);let te0: f64 = (l.f645 * tdf);let te1: f64 = (te0).abs();let te2: f64 = if te1 < 230.25850929940458 { 1.0 } else { 0.0 };l.f170 = te2;l.f171 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f170 != 0.0)) {let te3: f64 = (l.f7b1 / l.f5f1);let te4: f64 = (l.f5f1 - l.f5ed);let te5: f64 = (l.f793 * te4);let te6: f64 = (l.f5ed * p.p85);let te7: f64 = (te5 / te6);let te8: f64 = (te3 + te7);let te9: f64 = (l.f645 * te8);let tea: f64 = (te9).exp();(l.f93, l.f94, l.f95, ) = (tea, (tea * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * te6) - (te5 * (l.f5ee * p.p85))) / (te6 * te6))))), (tea * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * te6) - (te5 * (l.f5ef * p.p85))) / (te6 * te6))))), );l.f96 = 0.0;}
        let teb: f64 = (l.f7b1 / l.f5f1);let tec: f64 = (l.f5f1 - l.f5ed);let ted: f64 = (l.f793 * tec);let tee: f64 = (l.f5ed * p.p85);let tef: f64 = (ted / tee);let tf0: f64 = (teb + tef);let tf1: f64 = (l.f645 * tf0);let tf2: f64 = (-230.25850929940458);let tf3: f64 = if tf1 < tf2 { 1.0 } else { 0.0 };l.f172 = tf3;l.f173 = 0.0;
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f170 == 0.0)) && (l.f172 != 0.0)) {let tf4: f64 = (-230.25850929940458);let tf5: f64 = (l.f7b1 / l.f5f1);let tf6: f64 = (l.f5f1 - l.f5ed);let tf7: f64 = (l.f793 * tf6);let tf8: f64 = (l.f5ed * p.p85);let tf9: f64 = (tf7 / tf8);let tfa: f64 = (tf5 + tf9);let tfb: f64 = (l.f645 * tfa);let tfc: f64 = (tf4 - tfb);let tfd: f64 = (-230.25850929940458);let tfe: f64 = (l.f7b1 / l.f5f1);let tff: f64 = (l.f5f1 - l.f5ed);let t100: f64 = (l.f793 * tff);let t101: f64 = (l.f5ed * p.p85);let t102: f64 = (t100 / t101);let t103: f64 = (tfe + t102);let t104: f64 = (l.f645 * t103);let t105: f64 = (tfd - t104);let t106: f64 = (-230.25850929940458);let t107: f64 = (l.f7b1 / l.f5f1);let t108: f64 = (l.f5f1 - l.f5ed);let t109: f64 = (l.f793 * t108);let t10a: f64 = (l.f5ed * p.p85);let t10b: f64 = (t109 / t10a);let t10c: f64 = (t107 + t10b);let t10d: f64 = (l.f645 * t10c);let t10e: f64 = (t106 - t10d);let t10f: f64 = (t10e * 0.3333333333333333);let t110: f64 = (1.0 + t10f);let t111: f64 = (t105 * t110);let t112: f64 = (0.5 * t111);let t113: f64 = (1.0 + t112);let t114: f64 = (tfc * t113);let t115: f64 = (1.0 + t114);let t116: f64 = (1e-100 / t115);(l.f93, l.f94, l.f95, ) = (t116, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * tf8) - (tf7 * (l.f5ee * p.p85))) / (tf8 * tf8))))) * t113) + (tfc * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t101) - (t100 * (l.f5ee * p.p85))) / (t101 * t101))))) * t110) + (t105 * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t10a) - (t109 * (l.f5ee * p.p85))) / (t10a * t10a))))) * 0.3333333333333333))))))) / (t115 * t115))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * tf8) - (tf7 * (l.f5ef * p.p85))) / (tf8 * tf8))))) * t113) + (tfc * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t101) - (t100 * (l.f5ef * p.p85))) / (t101 * t101))))) * t110) + (t105 * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t10a) - (t109 * (l.f5ef * p.p85))) / (t10a * t10a))))) * 0.3333333333333333))))))) / (t115 * t115))), );l.f96 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f170 == 0.0)) && (l.f172 == 0.0)) {let t117: f64 = (l.f7b1 / l.f5f1);let t118: f64 = (l.f5f1 - l.f5ed);let t119: f64 = (l.f793 * t118);let t11a: f64 = (l.f5ed * p.p85);let t11b: f64 = (t119 / t11a);let t11c: f64 = (t117 + t11b);let t11d: f64 = (l.f645 * t11c);let t11e: f64 = (t11d - 230.25850929940458);let t11f: f64 = (l.f7b1 / l.f5f1);let t120: f64 = (l.f5f1 - l.f5ed);let t121: f64 = (l.f793 * t120);let t122: f64 = (l.f5ed * p.p85);let t123: f64 = (t121 / t122);let t124: f64 = (t11f + t123);let t125: f64 = (l.f645 * t124);let t126: f64 = (t125 - 230.25850929940458);let t127: f64 = (l.f7b1 / l.f5f1);let t128: f64 = (l.f5f1 - l.f5ed);let t129: f64 = (l.f793 * t128);let t12a: f64 = (l.f5ed * p.p85);let t12b: f64 = (t129 / t12a);let t12c: f64 = (t127 + t12b);let t12d: f64 = (l.f645 * t12c);let t12e: f64 = (t12d - 230.25850929940458);let t12f: f64 = (t12e * 0.3333333333333333);let t130: f64 = (1.0 + t12f);let t131: f64 = (t126 * t130);let t132: f64 = (0.5 * t131);let t133: f64 = (1.0 + t132);let t134: f64 = (t11e * t133);let t135: f64 = (1.0 + t134);let t136: f64 = (1e100 * t135);(l.f93, l.f94, l.f95, ) = (t136, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t11a) - (t119 * (l.f5ee * p.p85))) / (t11a * t11a)))) * t133) + (t11e * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t122) - (t121 * (l.f5ee * p.p85))) / (t122 * t122)))) * t130) + (t126 * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t12a) - (t129 * (l.f5ee * p.p85))) / (t12a * t12a)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t11a) - (t119 * (l.f5ef * p.p85))) / (t11a * t11a)))) * t133) + (t11e * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t122) - (t121 * (l.f5ef * p.p85))) / (t122 * t122)))) * t130) + (t126 * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t12a) - (t129 * (l.f5ef * p.p85))) / (t12a * t12a)))) * 0.3333333333333333))))))), );l.f96 = 0.0;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) {let t137: f64 = (l.f7b1 * l.f5b);let t138: f64 = (l.f5f1 - t137);let t139: f64 = (l.f5f1 * l.f5f1);let t13a: f64 = (t138 / t139);let t13b: f64 = (l.f793 * l.f5b);let t13c: f64 = (l.f5ed * p.p85);let t13d: f64 = (t13b / t13c);let t13e: f64 = (t13a + t13d);let t13f: f64 = (l.f645 * t13e);(l.f61, l.f62, l.f63, ) = (t13f, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t139) - (t138 * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t139 * t139)) + ((((l.f793 * l.f5c) * t13c) - (t13b * (l.f5ee * p.p85))) / (t13c * t13c)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t139) - (t138 * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t139 * t139)) + ((((l.f793 * l.f5d) * t13c) - (t13b * (l.f5ef * p.p85))) / (t13c * t13c)))), );l.f64 = 0.0;let t140: f64 = (l.f739 - l.f7b1);let t141: f64 = (t140 * l.f61);let t142: f64 = (1.0 + t141);let t143: f64 = (t142 * l.f93);(l.f53e, l.f53f, l.f540, ) = (t143, (((t140 * l.f62) * l.f93) + (t142 * l.f94)), (((t140 * l.f63) * l.f93) + (t142 * l.f95)), );l.f541 = 0.0;let t144: f64 = (l.f5eb * l.f5eb);let t145: f64 = (t144 / l.f5e1);l.f64f = t145;l.f650 = 0.0;let t146: f64 = (l.f5e7 / l.f645);let t147: f64 = (l.f5e1 / l.f64f);let t148: f64 = (t147).ln();let t149: f64 = (t146 * t148);l.f793 = t149;l.f794 = 0.0;}
        let t14a: f64 = if l.f5e7 < p.p85 { 1.0 } else { 0.0 };l.f174 = t14a;l.f175 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t14b: f64 = (l.f7b1 - l.f793);let t14c: f64 = (p.p86 * t14b);let t14d: f64 = (t14c + l.f5e7);(l.f601, l.f602, l.f603, ) = (t14d, 0.0, 0.0, );l.f604 = 0.0;let t14e: f64 = (p.p86 * l.f793);let t14f: f64 = (l.f5e7 - t14e);(l.f5ed, l.f5ee, l.f5ef, ) = (t14f, 0.0, 0.0, );l.f5f0 = 0.0;let t150: f64 = (p.p85 - l.f601);let t151: f64 = (t150 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t151, (-l.f602), (-l.f603), );l.f6f6 = 0.0;let t152: f64 = (4.0 * p.p85);let t153: f64 = (t152 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t153, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {
            let (t155, t156, t157,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t154: f64 = (-l.f6f7);
        (t154, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t155, t156, t157, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t158: f64 = (l.f6f3 * l.f6f3);let t159: f64 = (t158 + l.f6f7);let t15a: f64 = (t159).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t15a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t15a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t15a)), );l.f6fa = 0.0;let t15b: f64 = (l.f6f3 / l.f6f7);let t15c: f64 = (1.0 + t15b);let t15d: f64 = (0.5 * t15c);(l.f55, l.f56, l.f57, ) = (t15d, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f58 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t15e: f64 = (l.f6f3 + l.f6f7);let t15f: f64 = (0.5 * t15e);let t160: f64 = (p.p85 - t15f);(l.f605, l.f606, l.f607, ) = (t160, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f608 = 0.0;let t161: f64 = (l.f605 - l.f5e7);let t162: f64 = (t161 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t162, l.f606, l.f607, );l.f6f6 = 0.0;let t163: f64 = (4.0 * l.f5e7);let t164: f64 = (t163 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t164, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {
            let (t166, t167, t168,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t165: f64 = (-l.f6f7);
        (t165, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t166, t167, t168, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t169: f64 = (l.f6f3 * l.f6f3);let t16a: f64 = (t169 + l.f6f7);let t16b: f64 = (t16a).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t16b, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t16b)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t16b)), );l.f6fa = 0.0;let t16c: f64 = (l.f6f3 / l.f6f7);let t16d: f64 = (1.0 + t16c);let t16e: f64 = (0.5 * t16d);(l.f51, l.f52, l.f53, ) = (t16e, (0.5 * (((l.f6f4 * l.f6f7) - (l.f6f3 * l.f6f8)) / (l.f6f7 * l.f6f7))), (0.5 * (((l.f6f5 * l.f6f7) - (l.f6f3 * l.f6f9)) / (l.f6f7 * l.f6f7))), );l.f54 = 0.0;let t16f: f64 = (l.f6f3 + l.f6f7);let t170: f64 = (0.5 * t16f);let t171: f64 = (l.f5e7 + t170);(l.f5f1, l.f5f2, l.f5f3, ) = (t171, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f4 = 0.0;let t172: f64 = (p.p85 - l.f5ed);let t173: f64 = (t172 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t173, (-l.f5ee), (-l.f5ef), );l.f6f6 = 0.0;let t174: f64 = (4.0 * p.p85);let t175: f64 = (t174 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t175, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {
            let (t177, t178, t179,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t176: f64 = (-l.f6f7);
        (t176, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t177, t178, t179, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t17a: f64 = (l.f6f3 * l.f6f3);let t17b: f64 = (t17a + l.f6f7);let t17c: f64 = (t17b).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t17c, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t17c)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t17c)), );l.f6fa = 0.0;let t17d: f64 = (l.f6f3 + l.f6f7);let t17e: f64 = (0.5 * t17d);let t17f: f64 = (p.p85 - t17e);(l.f5ed, l.f5ee, l.f5ef, ) = (t17f, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );l.f5f0 = 0.0;let t180: f64 = (l.f5ed - l.f5e7);let t181: f64 = (t180 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t181, l.f5ee, l.f5ef, );l.f6f6 = 0.0;let t182: f64 = (4.0 * l.f5e7);let t183: f64 = (t182 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t183, 0.0, 0.0, );l.f6fa = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {
            let (t185, t186, t187,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t184: f64 = (-l.f6f7);
        (t184, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t185, t186, t187, );l.f6fa = 0.0;
        }
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 != 0.0)) {let t188: f64 = (l.f6f3 * l.f6f3);let t189: f64 = (t188 + l.f6f7);let t18a: f64 = (t189).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t18a, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t18a)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t18a)), );l.f6fa = 0.0;let t18b: f64 = (l.f6f3 + l.f6f7);let t18c: f64 = (0.5 * t18b);let t18d: f64 = (l.f5e7 + t18c);(l.f5ed, l.f5ee, l.f5ef, ) = (t18d, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );l.f5f0 = 0.0;let t18e: f64 = (p.p86 * l.f55);let t18f: f64 = (t18e * l.f51);(l.f5b, l.f5c, l.f5d, ) = (t18f, (((p.p86 * l.f56) * l.f51) + (t18e * l.f52)), (((p.p86 * l.f57) * l.f51) + (t18e * l.f53)), );l.f5e = 0.0;}
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f174 == 0.0)) {(l.f5ed, l.f5ee, l.f5ef, ) = (l.f5e7, 0.0, 0.0, );l.f5f0 = 0.0;(l.f5f1, l.f5f2, l.f5f3, ) = (l.f5e7, 0.0, 0.0, );l.f5f4 = 0.0;(l.f5b, l.f5c, l.f5d, ) = (0.0, 0.0, 0.0, );l.f5e = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t190: f64 = (l.f7b1 / l.f5f1);let t191: f64 = (l.f5f1 - l.f5ed);let t192: f64 = (l.f793 * t191);let t193: f64 = (l.f5ed * p.p85);let t194: f64 = (t192 / t193);let t195: f64 = (t190 + t194);let t196: f64 = (l.f645 * t195);let t197: f64 = (t196).abs();let t198: f64 = if t197 < 230.25850929940458 { 1.0 } else { 0.0 };l.f176 = t198;l.f177 = 0.0;
        if ((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f176 != 0.0)) {let t199: f64 = (l.f7b1 / l.f5f1);let t19a: f64 = (l.f5f1 - l.f5ed);let t19b: f64 = (l.f793 * t19a);let t19c: f64 = (l.f5ed * p.p85);let t19d: f64 = (t19b / t19c);let t19e: f64 = (t199 + t19d);let t19f: f64 = (l.f645 * t19e);let t1a0: f64 = (t19f).exp();(l.f8e, l.f8f, l.f90, ) = (t1a0, (t1a0 * (l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t19c) - (t19b * (l.f5ee * p.p85))) / (t19c * t19c))))), (t1a0 * (l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t19c) - (t19b * (l.f5ef * p.p85))) / (t19c * t19c))))), );l.f91 = 0.0;}
        let t1a1: f64 = (l.f7b1 / l.f5f1);let t1a2: f64 = (l.f5f1 - l.f5ed);let t1a3: f64 = (l.f793 * t1a2);let t1a4: f64 = (l.f5ed * p.p85);let t1a5: f64 = (t1a3 / t1a4);let t1a6: f64 = (t1a1 + t1a5);let t1a7: f64 = (l.f645 * t1a6);let t1a8: f64 = (-230.25850929940458);let t1a9: f64 = if t1a7 < t1a8 { 1.0 } else { 0.0 };l.f178 = t1a9;l.f179 = 0.0;
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f176 == 0.0)) && (l.f178 != 0.0)) {let t1aa: f64 = (-230.25850929940458);let t1ab: f64 = (l.f7b1 / l.f5f1);let t1ac: f64 = (l.f5f1 - l.f5ed);let t1ad: f64 = (l.f793 * t1ac);let t1ae: f64 = (l.f5ed * p.p85);let t1af: f64 = (t1ad / t1ae);let t1b0: f64 = (t1ab + t1af);let t1b1: f64 = (l.f645 * t1b0);let t1b2: f64 = (t1aa - t1b1);let t1b3: f64 = (-230.25850929940458);let t1b4: f64 = (l.f7b1 / l.f5f1);let t1b5: f64 = (l.f5f1 - l.f5ed);let t1b6: f64 = (l.f793 * t1b5);let t1b7: f64 = (l.f5ed * p.p85);let t1b8: f64 = (t1b6 / t1b7);let t1b9: f64 = (t1b4 + t1b8);let t1ba: f64 = (l.f645 * t1b9);let t1bb: f64 = (t1b3 - t1ba);let t1bc: f64 = (-230.25850929940458);let t1bd: f64 = (l.f7b1 / l.f5f1);let t1be: f64 = (l.f5f1 - l.f5ed);let t1bf: f64 = (l.f793 * t1be);let t1c0: f64 = (l.f5ed * p.p85);let t1c1: f64 = (t1bf / t1c0);let t1c2: f64 = (t1bd + t1c1);let t1c3: f64 = (l.f645 * t1c2);let t1c4: f64 = (t1bc - t1c3);let t1c5: f64 = (t1c4 * 0.3333333333333333);let t1c6: f64 = (1.0 + t1c5);let t1c7: f64 = (t1bb * t1c6);let t1c8: f64 = (0.5 * t1c7);let t1c9: f64 = (1.0 + t1c8);let t1ca: f64 = (t1b2 * t1c9);let t1cb: f64 = (1.0 + t1ca);let t1cc: f64 = (1e-100 / t1cb);(l.f8e, l.f8f, l.f90, ) = (t1cc, (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1ae) - (t1ad * (l.f5ee * p.p85))) / (t1ae * t1ae))))) * t1c9) + (t1b2 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1b7) - (t1b6 * (l.f5ee * p.p85))) / (t1b7 * t1b7))))) * t1c6) + (t1bb * ((-(l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1c0) - (t1bf * (l.f5ee * p.p85))) / (t1c0 * t1c0))))) * 0.3333333333333333))))))) / (t1cb * t1cb))), (-((1e-100 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1ae) - (t1ad * (l.f5ef * p.p85))) / (t1ae * t1ae))))) * t1c9) + (t1b2 * (0.5 * (((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1b7) - (t1b6 * (l.f5ef * p.p85))) / (t1b7 * t1b7))))) * t1c6) + (t1bb * ((-(l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1c0) - (t1bf * (l.f5ef * p.p85))) / (t1c0 * t1c0))))) * 0.3333333333333333))))))) / (t1cb * t1cb))), );l.f91 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) && (l.f176 == 0.0)) && (l.f178 == 0.0)) {let t1cd: f64 = (l.f7b1 / l.f5f1);let t1ce: f64 = (l.f5f1 - l.f5ed);let t1cf: f64 = (l.f793 * t1ce);let t1d0: f64 = (l.f5ed * p.p85);let t1d1: f64 = (t1cf / t1d0);let t1d2: f64 = (t1cd + t1d1);let t1d3: f64 = (l.f645 * t1d2);let t1d4: f64 = (t1d3 - 230.25850929940458);let t1d5: f64 = (l.f7b1 / l.f5f1);let t1d6: f64 = (l.f5f1 - l.f5ed);let t1d7: f64 = (l.f793 * t1d6);let t1d8: f64 = (l.f5ed * p.p85);let t1d9: f64 = (t1d7 / t1d8);let t1da: f64 = (t1d5 + t1d9);let t1db: f64 = (l.f645 * t1da);let t1dc: f64 = (t1db - 230.25850929940458);let t1dd: f64 = (l.f7b1 / l.f5f1);let t1de: f64 = (l.f5f1 - l.f5ed);let t1df: f64 = (l.f793 * t1de);let t1e0: f64 = (l.f5ed * p.p85);let t1e1: f64 = (t1df / t1e0);let t1e2: f64 = (t1dd + t1e1);let t1e3: f64 = (l.f645 * t1e2);let t1e4: f64 = (t1e3 - 230.25850929940458);let t1e5: f64 = (t1e4 * 0.3333333333333333);let t1e6: f64 = (1.0 + t1e5);let t1e7: f64 = (t1dc * t1e6);let t1e8: f64 = (0.5 * t1e7);let t1e9: f64 = (1.0 + t1e8);let t1ea: f64 = (t1d4 * t1e9);let t1eb: f64 = (1.0 + t1ea);let t1ec: f64 = (1e100 * t1eb);(l.f8e, l.f8f, l.f90, ) = (t1ec, (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1d0) - (t1cf * (l.f5ee * p.p85))) / (t1d0 * t1d0)))) * t1e9) + (t1d4 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1d8) - (t1d7 * (l.f5ee * p.p85))) / (t1d8 * t1d8)))) * t1e6) + (t1dc * ((l.f645 * ((-((l.f7b1 * l.f5f2) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f2 - l.f5ee)) * t1e0) - (t1df * (l.f5ee * p.p85))) / (t1e0 * t1e0)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1d0) - (t1cf * (l.f5ef * p.p85))) / (t1d0 * t1d0)))) * t1e9) + (t1d4 * (0.5 * (((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1d8) - (t1d7 * (l.f5ef * p.p85))) / (t1d8 * t1d8)))) * t1e6) + (t1dc * ((l.f645 * ((-((l.f7b1 * l.f5f3) / (l.f5f1 * l.f5f1))) + ((((l.f793 * (l.f5f3 - l.f5ef)) * t1e0) - (t1df * (l.f5ef * p.p85))) / (t1e0 * t1e0)))) * 0.3333333333333333))))))), );l.f91 = 0.0;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f150 == 0.0)) {let t1ed: f64 = (l.f7b1 * l.f5b);let t1ee: f64 = (l.f5f1 - t1ed);let t1ef: f64 = (l.f5f1 * l.f5f1);let t1f0: f64 = (t1ee / t1ef);let t1f1: f64 = (l.f793 * l.f5b);let t1f2: f64 = (l.f5ed * p.p85);let t1f3: f64 = (t1f1 / t1f2);let t1f4: f64 = (t1f0 + t1f3);let t1f5: f64 = (l.f645 * t1f4);(l.f61, l.f62, l.f63, ) = (t1f5, (l.f645 * (((((l.f5f2 - (l.f7b1 * l.f5c)) * t1ef) - (t1ee * ((l.f5f2 * l.f5f1) + (l.f5f1 * l.f5f2)))) / (t1ef * t1ef)) + ((((l.f793 * l.f5c) * t1f2) - (t1f1 * (l.f5ee * p.p85))) / (t1f2 * t1f2)))), (l.f645 * (((((l.f5f3 - (l.f7b1 * l.f5d)) * t1ef) - (t1ee * ((l.f5f3 * l.f5f1) + (l.f5f1 * l.f5f3)))) / (t1ef * t1ef)) + ((((l.f793 * l.f5d) * t1f2) - (t1f1 * (l.f5ef * p.p85))) / (t1f2 * t1f2)))), );l.f64 = 0.0;let t1f6: f64 = (l.f739 - l.f7b1);let t1f7: f64 = (t1f6 * l.f61);let t1f8: f64 = (1.0 + t1f7);let t1f9: f64 = (t1f8 * l.f8e);(l.f53a, l.f53b, l.f53c, ) = (t1f9, (((t1f6 * l.f62) * l.f8e) + (t1f8 * l.f8f)), (((t1f6 * l.f63) * l.f8e) + (t1f8 * l.f90)), );l.f53d = 0.0;}
        if ((l.f29a != 0.0) && (l.f14e != 0.0)) {let t1fa: f64 = (l.f536 - 1.0);(l.f536, l.f537, l.f538, ) = (t1fa, l.f537, l.f538, );l.f539 = 0.0;let t1fb: f64 = (l.f53e - 1.0);(l.f53e, l.f53f, l.f540, ) = (t1fb, l.f53f, l.f540, );l.f541 = 0.0;let t1fc: f64 = (l.f53a - 1.0);(l.f53a, l.f53b, l.f53c, ) = (t1fc, l.f53b, l.f53c, );l.f53d = 0.0;let t1fd: f64 = (1.0 / l.f825);l.f817 = t1fd;l.f818 = 0.0;}
        let t1fe: f64 = if l.f739 > 0.0 { 1.0 } else { 0.0 };l.f17a = t1fe;l.f17b = 0.0;
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f17a != 0.0)) {let t1ff: f64 = (2.0 + l.f817);let t200: f64 = (l.f817 + 1.0);let t201: f64 = (l.f817 + 3.0);let t202: f64 = (t200 * t201);let t203: f64 = (t202).sqrt();let t204: f64 = (t1ff + t203);let t205: f64 = (t204).ln();let t206: f64 = (l.f643 * t205);let t207: f64 = (2.0 * t206);l.f714 = t207;l.f715 = 0.0;}
        if (((l.f29a != 0.0) && (l.f14e != 0.0)) && (l.f17a == 0.0)) {let t208: f64 = (-l.f739);let t209: f64 = (2.0 * l.f825);let t20a: f64 = (t209 + 1.0);let t20b: f64 = (1.0 + l.f825);let t20c: f64 = (3.0 * l.f825);let t20d: f64 = (1.0 + t20c);let t20e: f64 = (t20b * t20d);let t20f: f64 = (t20e).sqrt();let t210: f64 = (t20a + t20f);let t211: f64 = (t210).ln();let t212: f64 = (l.f643 * t211);let t213: f64 = (2.0 * t212);let t214: f64 = (t208 + t213);l.f714 = t214;l.f715 = 0.0;}
        if ((l.f29a != 0.0) && (l.f14e != 0.0)) {let t215: f64 = (l.f76f - l.f714);l.f79c = t215;l.f79d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        l: &mut StampLocals,
    ) {
        if ((l.f29a != 0.0) && (l.f14e != 0.0)) {let t216: f64 = (l.f739 + l.f79c);let t217: f64 = (l.f739 - l.f79c);let t218: f64 = (l.f739 - l.f79c);let t219: f64 = (t217 * t218);let t21a: f64 = (4.0 * l.f643);let t21b: f64 = (t21a * l.f643);let t21c: f64 = (t219 + t21b);let t21d: f64 = (t21c).sqrt();let t21e: f64 = (t216 - t21d);let t21f: f64 = (0.5 * t21e);l.f7a2 = t21f;l.f7a3 = 0.0;let t220: f64 = (l.f739 + l.f755);let t221: f64 = (l.f739 - l.f755);let t222: f64 = (l.f739 - l.f755);let t223: f64 = (t221 * t222);let t224: f64 = (4.0 * l.f647);let t225: f64 = (t224 * l.f647);let t226: f64 = (t223 + t225);let t227: f64 = (t226).sqrt();let t228: f64 = (t220 - t227);let t229: f64 = (0.5 * t228);l.f750 = t229;l.f751 = 0.0;let t22a: f64 = l.f739;let t22b: f64 = l.f739;let t22c: f64 = l.f739;let t22d: f64 = (t22b * t22c);let t22e: f64 = (4.0 * 1e-6);let t22f: f64 = (t22e * 1e-6);let t230: f64 = (t22d + t22f);let t231: f64 = (t230).sqrt();let t232: f64 = (t22a - t231);let t233: f64 = (0.5 * t232);l.f74a = t233;l.f74b = 0.0;}
        if ((l.f29a != 0.0) && (l.f14e == 0.0)) {(l.f536, l.f537, l.f538, ) = (0.0, 0.0, 0.0, );l.f539 = 0.0;(l.f53e, l.f53f, l.f540, ) = (0.0, 0.0, 0.0, );l.f541 = 0.0;(l.f53a, l.f53b, l.f53c, ) = (0.0, 0.0, 0.0, );l.f53d = 0.0;l.f714 = 0.0;l.f715 = 0.0;l.f796 = 0.0;l.f797 = 0.0;l.f825 = 0.0;l.f826 = 0.0;l.f7a2 = 0.0;l.f7a3 = 0.0;l.f750 = 0.0;l.f751 = 0.0;l.f74a = 0.0;l.f74b = 0.0;}
        let t234: f64 = if l.f0 == 0.0 { 1.0 } else { 0.0 };l.f17c = t234;l.f17d = 0.0;
        if ((l.f29a != 0.0) && (l.f17c != 0.0)) {(l.f562, l.f563, l.f564, ) = (0.0, 0.0, 0.0, );l.f565 = 0.0;(l.f552, l.f553, l.f554, ) = (0.0, 0.0, 0.0, );l.f555 = 0.0;(l.f68c, l.f68d, l.f68e, ) = (0.0, 0.0, 0.0, );l.f68f = 0.0;}
        let t235: f64 = if l.f60b == 0.5 { 1.0 } else { 0.0 };l.f17e = t235;l.f17f = 0.0;
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f17e != 0.0)) {let t236: f64 = (l.f796 * l.f769);let t237: f64 = (1.0 - t236);let t238: f64 = (t237).sqrt();l.f6fc = t238;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f17e == 0.0)) {let t239: f64 = (l.f796 * l.f769);let t23a: f64 = (1.0 - t239);let t23b: f64 = (t23a).powf(l.f60b);l.f6fc = t23b;l.f6fd = 0.0;}
        if ((l.f29a != 0.0) && (l.f17c == 0.0)) {let t23c: f64 = (1.0 - l.f6fc);let t23d: f64 = (l.f69e * t23c);let t23e: f64 = (l.f739 - l.f796);let t23f: f64 = (l.f698 * t23e);let t240: f64 = (t23d + t23f);(l.f68c, l.f68d, l.f68e, ) = (t240, 0.0, 0.0, );l.f68f = 0.0;let t241: f64 = (l.f542 * l.f536);(l.f52f, l.f530, l.f531, ) = (t241, (l.f542 * l.f537), (l.f542 * l.f538), );l.f532 = 0.0;}
        let t242: f64 = if ((l.f39 == 0.0) && (l.f3f == 0.0)) { 1.0 } else { 0.0 };l.f180 = t242;l.f181 = 0.0;
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 != 0.0)) {l.f758 = 0.0;l.f759 = 0.0;l.f7e9 = 0.0;l.f7ea = 0.0;l.f7d1 = 0.0;l.f7d2 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        l: &mut StampLocals,
    ) {
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 != 0.0)) {l.f9 = 0.0;l.fa = 0.0;l.f593 = 0.0;l.f594 = 0.0;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) {let t243: f64 = (l.f75d - l.f7a2);l.f758 = t243;l.f759 = 0.0;let t244: f64 = (l.f714 / l.f758);let t245: f64 = (1.0 - t244);let t246: f64 = (t245).sqrt();let t247: f64 = (1.0 - t246);l.f7ef = t247;l.f7f0 = 0.0;}
        let t248: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f182 = t248;l.f183 = 0.0;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) && (l.f182 != 0.0)) {l.f66 = 0.0;l.f67 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) && (l.f182 == 0.0)) {let t249: f64 = (l.f7ef * l.f7ef);let t24a: f64 = (l.f7ef).ln();let t24b: f64 = (t249 * t24a);let t24c: f64 = (1.0 - l.f7ef);let t24d: f64 = (t24b / t24c);let t24e: f64 = (t24d + l.f7ef);let t24f: f64 = (2.0 * l.f623);let t250: f64 = (1.0 - t24f);let t251: f64 = (t24e * t250);l.f66 = t251;l.f67 = 0.0;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) {let t252: f64 = (l.f7ef + l.f66);l.f7e9 = t252;l.f7ea = 0.0;}
        let t253: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f184 = t253;l.f185 = 0.0;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) && (l.f184 != 0.0)) {let t254: f64 = (l.f758 * l.f773);let t255: f64 = (t254).sqrt();l.f6fc = t255;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) && (l.f184 == 0.0)) {let t256: f64 = (l.f758 * l.f773);let t257: f64 = (t256).powf(l.f623);l.f6fc = t257;l.f6fd = 0.0;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f180 == 0.0)) {let t258: f64 = (l.f7d6 * l.f6fc);l.f7d1 = t258;l.f7d2 = 0.0;let t259: f64 = (l.f825 - 1.0);let t25a: f64 = (t259 * l.f7d1);let t25b: f64 = (l.fc9 * t25a);l.f9 = t25b;l.fa = 0.0;let t25c: f64 = (l.f9 * l.f7e9);let t25d: f64 = (l.f39 * t25c);l.f593 = t25d;l.f594 = 0.0;}
        let t25e: f64 = if l.f3f == 0.0 { 1.0 } else { 0.0 };l.f186 = t25e;l.f187 = 0.0;
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 != 0.0)) {l.f599 = 0.0;l.f59a = 0.0;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) {let t25f: f64 = (l.f7d1 * l.f60b);let t260: f64 = (t25f / l.f758);let t261: f64 = (l.f1e * t260);l.f19 = t261;l.f1a = 0.0;let t262: f64 = (0.666666666666667 * l.fe);let t263: f64 = (t262 / l.f19);l.f71a = t263;l.f71b = 0.0;let t264: f64 = (l.f71a * l.f71a);l.f72c = t264;l.f72d = 0.0;let t265: f64 = (l.f72c * l.f72c);let t266: f64 = (l.f72c * l.f72c);let t267: f64 = (t266 + 1.0);let t268: f64 = (t265 / t267);let t269: f64 = (t268).sqrt();l.f726 = t269;l.f727 = 0.0;let t26a: f64 = (l.f726).abs();let t26b: f64 = (t26a).sqrt();l.f6c1 = t26b;l.f6c2 = 0.0;let t26c: f64 = (l.f726 * l.f6c1);l.f732 = t26c;l.f733 = 0.0;}
        let t26d: f64 = (-l.f623);let t26e: f64 = (t26d * l.f611);let t26f: f64 = (-1.0);let t270: f64 = if t26e == t26f { 1.0 } else { 0.0 };l.f188 = t270;l.f189 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        l: &mut StampLocals,
    ) {
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f188 != 0.0)) {let t271: f64 = (l.f19 * l.f732);let t272: f64 = (1.0 + t271);let t273: f64 = (1.0 / t272);l.f7e3 = t273;l.f7e4 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f188 == 0.0)) {let t274: f64 = (l.f19 * l.f732);let t275: f64 = (1.0 + t274);let t276: f64 = (-l.f623);let t277: f64 = (t276 * l.f611);let t278: f64 = (t275).powf(t277);l.f7e3 = t278;l.f7e4 = 0.0;}
        if (((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) {let t279: f64 = (l.f7e9 * l.f7e3);let t27a: f64 = (l.f7e9 + l.f7e3);let t27b: f64 = (t279 / t27a);l.f7f5 = t27b;l.f7f6 = 0.0;let t27c: f64 = (l.f19 / l.f6c1);let t27d: f64 = (0.375 * t27c);let t27e: f64 = (t27d).sqrt();l.f5a8 = t27e;l.f5a9 = 0.0;let t27f: f64 = (l.f71a * l.f6c1);let t280: f64 = (2.0 * t27f);let t281: f64 = (t280 - l.f726);l.f5b4 = t281;l.f5b5 = 0.0;let t282: f64 = (l.fe * l.f71a);let t283: f64 = (t282 * l.f6c1);let t284: f64 = (l.fe * l.f726);let t285: f64 = (t283 - t284);let t286: f64 = (l.f19 * l.f732);let t287: f64 = (0.5 * t286);let t288: f64 = (t285 + t287);l.f5d4 = t288;l.f5d5 = 0.0;let t289: f64 = (l.f5b4 - 1.0);let t28a: f64 = (t289 * l.f5a8);l.f7fb = t28a;l.f7fc = 0.0;let t28b: f64 = (l.f7fb * l.f7fb);l.f811 = t28b;l.f812 = 0.0;}
        let t28c: f64 = if l.f7fb > 0.0 { 1.0 } else { 0.0 };l.f18a = t28c;l.f18b = 0.0;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18a != 0.0)) {let t28d: f64 = (l.f62b * l.f7fb);let t28e: f64 = (1.0 + t28d);let t28f: f64 = (1.0 / t28e);l.f6e2 = t28f;l.f6e3 = 0.0;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18a == 0.0)) {let t290: f64 = (l.f62b * l.f7fb);let t291: f64 = (1.0 - t290);let t292: f64 = (1.0 / t291);l.f6e2 = t292;l.f6e3 = 0.0;}
        let t293: f64 = (-l.f811);let t294: f64 = (t293 + l.f5d4);let t295: f64 = (-230.25850929940458);let t296: f64 = if t294 > t295 { 1.0 } else { 0.0 };l.f18c = t296;l.f18d = 0.0;
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18c != 0.0)) {let t297: f64 = (-l.f811);let t298: f64 = (t297 + l.f5d4);let t299: f64 = (t298).exp();l.f6fc = t299;l.f6fd = 0.0;}
        if ((((l.f29a != 0.0) && (l.f17c == 0.0)) && (l.f186 == 0.0)) && (l.f18c == 0.0)) {let t29a: f64 = (-230.25850929940458);let t29b: f64 = (-l.f811);let t29c: f64 = (t29b + l.f5d4);let t29d: f64 = (t29a - t29c);let t29e: f64 = (-230.25850929940458);let t29f: f64 = (-l.f811);let t2a0: f64 = (t29f + l.f5d4);let t2a1: f64 = (t29e - t2a0);let t2a2: f64 = (-230.25850929940458);let t2a3: f64 = (-l.f811);let t2a4: f64 = (t2a3 + l.f5d4);let t2a5: f64 = (t2a2 - t2a4);let t2a6: f64 = (t2a5 * 0.3333333333333333);let t2a7: f64 = (1.0 + t2a6);let t2a8: f64 = (t2a1 * t2a7);let t2a9: f64 = (0.5 * t2a8);let t2aa: f64 = (1.0 + t2a9);let t2ab: f64 = (t29d * t2aa);let t2ac: f64 = (1.0 + t2ab);let t2ad: f64 = (1e-100 / t2ac);l.f6fc = t2ad;l.f6fd = 0.0;}
    }
}
