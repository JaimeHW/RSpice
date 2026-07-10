#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_124(
        l: &mut StampLocals,
    ) {
        if (((l.fb4b != 0.0) && (l.fb4d == 0.0)) && (l.fb55 != 0.0)) {let t0: f64 = (l.f1fee - l.f1c97);let t1: f64 = (l.f13f * t0);(l.f369, l.f36d, l.f36e, l.f36f, l.f370, l.f371, l.f372, l.f373, l.f36a, l.f36b, l.f36c, ) = (t1, ((l.f143 * t0) + (l.f13f * (l.f1ff2 - l.f1c9b))), ((l.f144 * t0) + (l.f13f * (l.f1ff3 - l.f1c9c))), ((l.f145 * t0) + (l.f13f * (l.f1ff4 - l.f1c9d))), ((l.f146 * t0) + (l.f13f * (l.f1ff5 - l.f1c9e))), ((l.f147 * t0) + (l.f13f * (l.f1ff6 - l.f1c9f))), ((l.f148 * t0) + (l.f13f * (l.f1ff7 - l.f1ca0))), ((l.f149 * t0) + (l.f13f * (l.f1ff8 - l.f1ca1))), ((l.f140 * t0) + (l.f13f * (l.f1fef - l.f1c98))), ((l.f141 * t0) + (l.f13f * (l.f1ff0 - l.f1c99))), ((l.f142 * t0) + (l.f13f * (l.f1ff1 - l.f1c9a))), );l.f374 = 0.0;}
        let t2: f64 = if l.f1fee < l.f1a3b { 1.0 } else { 0.0 };l.fb57 = t2;l.fb58 = 0.0;
        if ((((l.fb4b != 0.0) && (l.fb4d == 0.0)) && (l.fb55 == 0.0)) && (l.fb57 != 0.0)) {let t3: f64 = (l.f1fee - l.f1c97);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t3, (l.f1ff2 - l.f1c9b), (l.f1ff3 - l.f1c9c), (l.f1ff4 - l.f1c9d), (l.f1ff5 - l.f1c9e), (l.f1ff6 - l.f1c9f), (l.f1ff7 - l.f1ca0), (l.f1ff8 - l.f1ca1), (l.f1fef - l.f1c98), (l.f1ff0 - l.f1c99), (l.f1ff1 - l.f1c9a), );l.f1fd2 = 0.0;let t4: f64 = (l.f1fc7 * l.f1fc7);(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (t4, ((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)), ((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)), ((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)), ((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)), ((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)), ((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)), ((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)), ((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)), ((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)), ((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)), );l.f2040 = 0.0;let t5: f64 = (l.f1a8f / 3.0);let t6: f64 = (t5 * l.f2035);let t7: f64 = (l.f13f - t6);let t8: f64 = (l.f1fc7 * t7);(l.f369, l.f36d, l.f36e, l.f36f, l.f370, l.f371, l.f372, l.f373, l.f36a, l.f36b, l.f36c, ) = (t8, ((l.f1fcb * t7) + (l.f1fc7 * (l.f143 - (((l.f1a93 / 3.0) * l.f2035) + (t5 * l.f2039))))), ((l.f1fcc * t7) + (l.f1fc7 * (l.f144 - (((l.f1a94 / 3.0) * l.f2035) + (t5 * l.f203a))))), ((l.f1fcd * t7) + (l.f1fc7 * (l.f145 - (((l.f1a95 / 3.0) * l.f2035) + (t5 * l.f203b))))), ((l.f1fce * t7) + (l.f1fc7 * (l.f146 - (((l.f1a96 / 3.0) * l.f2035) + (t5 * l.f203c))))), ((l.f1fcf * t7) + (l.f1fc7 * (l.f147 - (((l.f1a97 / 3.0) * l.f2035) + (t5 * l.f203d))))), ((l.f1fd0 * t7) + (l.f1fc7 * (l.f148 - (((l.f1a98 / 3.0) * l.f2035) + (t5 * l.f203e))))), ((l.f1fd1 * t7) + (l.f1fc7 * (l.f149 - (((l.f1a99 / 3.0) * l.f2035) + (t5 * l.f203f))))), ((l.f1fc8 * t7) + (l.f1fc7 * (l.f140 - (((l.f1a90 / 3.0) * l.f2035) + (t5 * l.f2036))))), ((l.f1fc9 * t7) + (l.f1fc7 * (l.f141 - (((l.f1a91 / 3.0) * l.f2035) + (t5 * l.f2037))))), ((l.f1fca * t7) + (l.f1fc7 * (l.f142 - (((l.f1a92 / 3.0) * l.f2035) + (t5 * l.f2038))))), );l.f374 = 0.0;}
        let t9: f64 = if l.f1fee < l.f1c8b { 1.0 } else { 0.0 };l.fb59 = t9;l.fb5a = 0.0;
        if (((((l.fb4b != 0.0) && (l.fb4d == 0.0)) && (l.fb55 == 0.0)) && (l.fb57 == 0.0)) && (l.fb59 != 0.0)) {let ta: f64 = (l.f1fee - l.f1c8b);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (ta, (l.f1ff2 - l.f1c8f), (l.f1ff3 - l.f1c90), (l.f1ff4 - l.f1c91), (l.f1ff5 - l.f1c92), (l.f1ff6 - l.f1c93), (l.f1ff7 - l.f1c94), (l.f1ff8 - l.f1c95), (l.f1fef - l.f1c8c), (l.f1ff0 - l.f1c8d), (l.f1ff1 - l.f1c8e), );l.f1fd2 = 0.0;let tb: f64 = (l.f1fc7 * l.f1fc7);(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (tb, ((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)), ((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)), ((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)), ((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)), ((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)), ((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)), ((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)), ((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)), ((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)), ((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)), );l.f2040 = 0.0;let tc: f64 = (l.f121 * l.f1fee);let td: f64 = (tc + l.f3c1);let te: f64 = (l.f1a9b / 3.0);let tf: f64 = (te * l.f1fc7);let t10: f64 = (tf * l.f2035);let t11: f64 = (td + t10);(l.f369, l.f36d, l.f36e, l.f36f, l.f370, l.f371, l.f372, l.f373, l.f36a, l.f36b, l.f36c, ) = (t11, (((l.f121 * l.f1ff2) + l.f3c5) + (((((l.f1a9f / 3.0) * l.f1fc7) + (te * l.f1fcb)) * l.f2035) + (tf * l.f2039))), (((l.f121 * l.f1ff3) + l.f3c6) + (((((l.f1aa0 / 3.0) * l.f1fc7) + (te * l.f1fcc)) * l.f2035) + (tf * l.f203a))), (((l.f121 * l.f1ff4) + l.f3c7) + (((((l.f1aa1 / 3.0) * l.f1fc7) + (te * l.f1fcd)) * l.f2035) + (tf * l.f203b))), (((l.f121 * l.f1ff5) + l.f3c8) + (((((l.f1aa2 / 3.0) * l.f1fc7) + (te * l.f1fce)) * l.f2035) + (tf * l.f203c))), (((l.f121 * l.f1ff6) + l.f3c9) + (((((l.f1aa3 / 3.0) * l.f1fc7) + (te * l.f1fcf)) * l.f2035) + (tf * l.f203d))), (((l.f121 * l.f1ff7) + l.f3ca) + (((((l.f1aa4 / 3.0) * l.f1fc7) + (te * l.f1fd0)) * l.f2035) + (tf * l.f203e))), (((l.f121 * l.f1ff8) + l.f3cb) + (((((l.f1aa5 / 3.0) * l.f1fc7) + (te * l.f1fd1)) * l.f2035) + (tf * l.f203f))), (((l.f121 * l.f1fef) + l.f3c2) + (((((l.f1a9c / 3.0) * l.f1fc7) + (te * l.f1fc8)) * l.f2035) + (tf * l.f2036))), (((l.f121 * l.f1ff0) + l.f3c3) + (((((l.f1a9d / 3.0) * l.f1fc7) + (te * l.f1fc9)) * l.f2035) + (tf * l.f2037))), (((l.f121 * l.f1ff1) + l.f3c4) + (((((l.f1a9e / 3.0) * l.f1fc7) + (te * l.f1fca)) * l.f2035) + (tf * l.f2038))), );l.f374 = 0.0;}
        if (((((l.fb4b != 0.0) && (l.fb4d == 0.0)) && (l.fb55 == 0.0)) && (l.fb57 == 0.0)) && (l.fb59 == 0.0)) {let t12: f64 = (l.f121 * l.f1fee);let t13: f64 = (t12 + l.f3c1);(l.f369, l.f36d, l.f36e, l.f36f, l.f370, l.f371, l.f372, l.f373, l.f36a, l.f36b, l.f36c, ) = (t13, ((l.f121 * l.f1ff2) + l.f3c5), ((l.f121 * l.f1ff3) + l.f3c6), ((l.f121 * l.f1ff4) + l.f3c7), ((l.f121 * l.f1ff5) + l.f3c8), ((l.f121 * l.f1ff6) + l.f3c9), ((l.f121 * l.f1ff7) + l.f3ca), ((l.f121 * l.f1ff8) + l.f3cb), ((l.f121 * l.f1fef) + l.f3c2), ((l.f121 * l.f1ff0) + l.f3c3), ((l.f121 * l.f1ff1) + l.f3c4), );l.f374 = 0.0;}
        let t14: f64 = if (((l.f18b0 > 0.0) && (l.f411 > 0.0)) || ((l.f18b0 < 0.0) && (l.f411 < 0.0))) { 1.0 } else { 0.0 };l.fb5b = t14;l.fb5c = 0.0;let t15: f64 = if l.f2005 < l.f1c8b { 1.0 } else { 0.0 };l.fb5d = t15;l.fb5e = 0.0;
        if (((l.fb4b != 0.0) && (l.fb5b != 0.0)) && (l.fb5d != 0.0)) {let t16: f64 = (l.f2005 - l.f1c8b);let t17: f64 = (l.fdd * t16);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t17, (l.fdd * (l.f2009 - l.f1c8f)), (l.fdd * (l.f200a - l.f1c90)), (l.fdd * (l.f200b - l.f1c91)), (l.fdd * (l.f200c - l.f1c92)), (l.fdd * (l.f200d - l.f1c93)), (l.fdd * (l.f200e - l.f1c94)), (l.fdd * (l.f200f - l.f1c95)), (l.fdd * (l.f2006 - l.f1c8c)), (l.fdd * (l.f2007 - l.f1c8d)), (l.fdd * (l.f2008 - l.f1c8e)), );l.f35c = 0.0;}
        let t18: f64 = if l.f2005 < l.f1a3b { 1.0 } else { 0.0 };l.fb5f = t18;l.fb60 = 0.0;
        if ((((l.fb4b != 0.0) && (l.fb5b != 0.0)) && (l.fb5d == 0.0)) && (l.fb5f != 0.0)) {let t19: f64 = (l.f2005 - l.f1c8b);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t19, (l.f2009 - l.f1c8f), (l.f200a - l.f1c90), (l.f200b - l.f1c91), (l.f200c - l.f1c92), (l.f200d - l.f1c93), (l.f200e - l.f1c94), (l.f200f - l.f1c95), (l.f2006 - l.f1c8c), (l.f2007 - l.f1c8d), (l.f2008 - l.f1c8e), );l.f1fd2 = 0.0;let t1a: f64 = (l.f1fc7 * l.f1fc7);(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (t1a, ((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)), ((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)), ((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)), ((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)), ((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)), ((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)), ((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)), ((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)), ((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)), ((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)), );l.f2040 = 0.0;let t1b: f64 = (l.f13aa / 3.0);let t1c: f64 = (t1b * l.f2035);let t1d: f64 = (l.fdd - t1c);let t1e: f64 = (l.f1fc7 * t1d);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t1e, ((l.f1fcb * t1d) + (l.f1fc7 * (-(((l.f13ae / 3.0) * l.f2035) + (t1b * l.f2039))))), ((l.f1fcc * t1d) + (l.f1fc7 * (-(((l.f13af / 3.0) * l.f2035) + (t1b * l.f203a))))), ((l.f1fcd * t1d) + (l.f1fc7 * (-(((l.f13b0 / 3.0) * l.f2035) + (t1b * l.f203b))))), ((l.f1fce * t1d) + (l.f1fc7 * (-(((l.f13b1 / 3.0) * l.f2035) + (t1b * l.f203c))))), ((l.f1fcf * t1d) + (l.f1fc7 * (-(((l.f13b2 / 3.0) * l.f2035) + (t1b * l.f203d))))), ((l.f1fd0 * t1d) + (l.f1fc7 * (-(((l.f13b3 / 3.0) * l.f2035) + (t1b * l.f203e))))), ((l.f1fd1 * t1d) + (l.f1fc7 * (-(((l.f13b4 / 3.0) * l.f2035) + (t1b * l.f203f))))), ((l.f1fc8 * t1d) + (l.f1fc7 * (-(((l.f13ab / 3.0) * l.f2035) + (t1b * l.f2036))))), ((l.f1fc9 * t1d) + (l.f1fc7 * (-(((l.f13ac / 3.0) * l.f2035) + (t1b * l.f2037))))), ((l.f1fca * t1d) + (l.f1fc7 * (-(((l.f13ad / 3.0) * l.f2035) + (t1b * l.f2038))))), );l.f35c = 0.0;}
        let t1f: f64 = if l.f2005 < l.f1c97 { 1.0 } else { 0.0 };l.fb61 = t1f;l.fb62 = 0.0;
        if (((((l.fb4b != 0.0) && (l.fb5b != 0.0)) && (l.fb5d == 0.0)) && (l.fb5f == 0.0)) && (l.fb61 != 0.0)) {let t20: f64 = (l.f2005 - l.f1c97);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t20, (l.f2009 - l.f1c9b), (l.f200a - l.f1c9c), (l.f200b - l.f1c9d), (l.f200c - l.f1c9e), (l.f200d - l.f1c9f), (l.f200e - l.f1ca0), (l.f200f - l.f1ca1), (l.f2006 - l.f1c98), (l.f2007 - l.f1c99), (l.f2008 - l.f1c9a), );l.f1fd2 = 0.0;let t21: f64 = (l.f1fc7 * l.f1fc7);(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (t21, ((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)), ((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)), ((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)), ((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)), ((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)), ((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)), ((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)), ((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)), ((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)), ((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)), );l.f2040 = 0.0;let t22: f64 = (l.feb * l.f2005);let t23: f64 = (t22 + l.f16b);let t24: f64 = (l.f13b6 / 3.0);let t25: f64 = (t24 * l.f1fc7);let t26: f64 = (t25 * l.f2035);let t27: f64 = (t23 + t26);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t27, ((((l.fef * l.f2005) + (l.feb * l.f2009)) + l.f16f) + (((((l.f13ba / 3.0) * l.f1fc7) + (t24 * l.f1fcb)) * l.f2035) + (t25 * l.f2039))), ((((l.ff0 * l.f2005) + (l.feb * l.f200a)) + l.f170) + (((((l.f13bb / 3.0) * l.f1fc7) + (t24 * l.f1fcc)) * l.f2035) + (t25 * l.f203a))), ((((l.ff1 * l.f2005) + (l.feb * l.f200b)) + l.f171) + (((((l.f13bc / 3.0) * l.f1fc7) + (t24 * l.f1fcd)) * l.f2035) + (t25 * l.f203b))), ((((l.ff2 * l.f2005) + (l.feb * l.f200c)) + l.f172) + (((((l.f13bd / 3.0) * l.f1fc7) + (t24 * l.f1fce)) * l.f2035) + (t25 * l.f203c))), ((((l.ff3 * l.f2005) + (l.feb * l.f200d)) + l.f173) + (((((l.f13be / 3.0) * l.f1fc7) + (t24 * l.f1fcf)) * l.f2035) + (t25 * l.f203d))), ((((l.ff4 * l.f2005) + (l.feb * l.f200e)) + l.f174) + (((((l.f13bf / 3.0) * l.f1fc7) + (t24 * l.f1fd0)) * l.f2035) + (t25 * l.f203e))), ((((l.ff5 * l.f2005) + (l.feb * l.f200f)) + l.f175) + (((((l.f13c0 / 3.0) * l.f1fc7) + (t24 * l.f1fd1)) * l.f2035) + (t25 * l.f203f))), ((((l.fec * l.f2005) + (l.feb * l.f2006)) + l.f16c) + (((((l.f13b7 / 3.0) * l.f1fc7) + (t24 * l.f1fc8)) * l.f2035) + (t25 * l.f2036))), ((((l.fed * l.f2005) + (l.feb * l.f2007)) + l.f16d) + (((((l.f13b8 / 3.0) * l.f1fc7) + (t24 * l.f1fc9)) * l.f2035) + (t25 * l.f2037))), ((((l.fee * l.f2005) + (l.feb * l.f2008)) + l.f16e) + (((((l.f13b9 / 3.0) * l.f1fc7) + (t24 * l.f1fca)) * l.f2035) + (t25 * l.f2038))), );l.f35c = 0.0;}
        if (((((l.fb4b != 0.0) && (l.fb5b != 0.0)) && (l.fb5d == 0.0)) && (l.fb5f == 0.0)) && (l.fb61 == 0.0)) {let t28: f64 = (l.feb * l.f2005);let t29: f64 = (t28 + l.f16b);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t29, (((l.fef * l.f2005) + (l.feb * l.f2009)) + l.f16f), (((l.ff0 * l.f2005) + (l.feb * l.f200a)) + l.f170), (((l.ff1 * l.f2005) + (l.feb * l.f200b)) + l.f171), (((l.ff2 * l.f2005) + (l.feb * l.f200c)) + l.f172), (((l.ff3 * l.f2005) + (l.feb * l.f200d)) + l.f173), (((l.ff4 * l.f2005) + (l.feb * l.f200e)) + l.f174), (((l.ff5 * l.f2005) + (l.feb * l.f200f)) + l.f175), (((l.fec * l.f2005) + (l.feb * l.f2006)) + l.f16c), (((l.fed * l.f2005) + (l.feb * l.f2007)) + l.f16d), (((l.fee * l.f2005) + (l.feb * l.f2008)) + l.f16e), );l.f35c = 0.0;}
        let t2a: f64 = if l.f2005 < l.f1c97 { 1.0 } else { 0.0 };l.fb63 = t2a;l.fb64 = 0.0;
        if (((l.fb4b != 0.0) && (l.fb5b == 0.0)) && (l.fb63 != 0.0)) {let t2b: f64 = (l.f2005 - l.f1c97);let t2c: f64 = (l.feb * t2b);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t2c, ((l.fef * t2b) + (l.feb * (l.f2009 - l.f1c9b))), ((l.ff0 * t2b) + (l.feb * (l.f200a - l.f1c9c))), ((l.ff1 * t2b) + (l.feb * (l.f200b - l.f1c9d))), ((l.ff2 * t2b) + (l.feb * (l.f200c - l.f1c9e))), ((l.ff3 * t2b) + (l.feb * (l.f200d - l.f1c9f))), ((l.ff4 * t2b) + (l.feb * (l.f200e - l.f1ca0))), ((l.ff5 * t2b) + (l.feb * (l.f200f - l.f1ca1))), ((l.fec * t2b) + (l.feb * (l.f2006 - l.f1c98))), ((l.fed * t2b) + (l.feb * (l.f2007 - l.f1c99))), ((l.fee * t2b) + (l.feb * (l.f2008 - l.f1c9a))), );l.f35c = 0.0;}
        let t2d: f64 = if l.f2005 < l.f1a3b { 1.0 } else { 0.0 };l.fb65 = t2d;l.fb66 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_125(
        l: &mut StampLocals,
    ) {
        if ((((l.fb4b != 0.0) && (l.fb5b == 0.0)) && (l.fb63 == 0.0)) && (l.fb65 != 0.0)) {let t2e: f64 = (l.f2005 - l.f1c97);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t2e, (l.f2009 - l.f1c9b), (l.f200a - l.f1c9c), (l.f200b - l.f1c9d), (l.f200c - l.f1c9e), (l.f200d - l.f1c9f), (l.f200e - l.f1ca0), (l.f200f - l.f1ca1), (l.f2006 - l.f1c98), (l.f2007 - l.f1c99), (l.f2008 - l.f1c9a), );l.f1fd2 = 0.0;let t2f: f64 = (l.f1fc7 * l.f1fc7);(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (t2f, ((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)), ((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)), ((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)), ((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)), ((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)), ((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)), ((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)), ((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)), ((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)), ((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)), );l.f2040 = 0.0;let t30: f64 = (l.f13aa / 3.0);let t31: f64 = (t30 * l.f2035);let t32: f64 = (l.feb - t31);let t33: f64 = (l.f1fc7 * t32);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t33, ((l.f1fcb * t32) + (l.f1fc7 * (l.fef - (((l.f13ae / 3.0) * l.f2035) + (t30 * l.f2039))))), ((l.f1fcc * t32) + (l.f1fc7 * (l.ff0 - (((l.f13af / 3.0) * l.f2035) + (t30 * l.f203a))))), ((l.f1fcd * t32) + (l.f1fc7 * (l.ff1 - (((l.f13b0 / 3.0) * l.f2035) + (t30 * l.f203b))))), ((l.f1fce * t32) + (l.f1fc7 * (l.ff2 - (((l.f13b1 / 3.0) * l.f2035) + (t30 * l.f203c))))), ((l.f1fcf * t32) + (l.f1fc7 * (l.ff3 - (((l.f13b2 / 3.0) * l.f2035) + (t30 * l.f203d))))), ((l.f1fd0 * t32) + (l.f1fc7 * (l.ff4 - (((l.f13b3 / 3.0) * l.f2035) + (t30 * l.f203e))))), ((l.f1fd1 * t32) + (l.f1fc7 * (l.ff5 - (((l.f13b4 / 3.0) * l.f2035) + (t30 * l.f203f))))), ((l.f1fc8 * t32) + (l.f1fc7 * (l.fec - (((l.f13ab / 3.0) * l.f2035) + (t30 * l.f2036))))), ((l.f1fc9 * t32) + (l.f1fc7 * (l.fed - (((l.f13ac / 3.0) * l.f2035) + (t30 * l.f2037))))), ((l.f1fca * t32) + (l.f1fc7 * (l.fee - (((l.f13ad / 3.0) * l.f2035) + (t30 * l.f2038))))), );l.f35c = 0.0;}
        let t34: f64 = if l.f2005 < l.f1c8b { 1.0 } else { 0.0 };l.fb67 = t34;l.fb68 = 0.0;
        if (((((l.fb4b != 0.0) && (l.fb5b == 0.0)) && (l.fb63 == 0.0)) && (l.fb65 == 0.0)) && (l.fb67 != 0.0)) {let t35: f64 = (l.f2005 - l.f1c8b);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t35, (l.f2009 - l.f1c8f), (l.f200a - l.f1c90), (l.f200b - l.f1c91), (l.f200c - l.f1c92), (l.f200d - l.f1c93), (l.f200e - l.f1c94), (l.f200f - l.f1c95), (l.f2006 - l.f1c8c), (l.f2007 - l.f1c8d), (l.f2008 - l.f1c8e), );l.f1fd2 = 0.0;let t36: f64 = (l.f1fc7 * l.f1fc7);(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (t36, ((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)), ((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)), ((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)), ((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)), ((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)), ((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)), ((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)), ((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)), ((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)), ((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)), );l.f2040 = 0.0;let t37: f64 = (l.fdd * l.f2005);let t38: f64 = (t37 + l.f16b);let t39: f64 = (l.f13b6 / 3.0);let t3a: f64 = (t39 * l.f1fc7);let t3b: f64 = (t3a * l.f2035);let t3c: f64 = (t38 + t3b);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t3c, (((l.fdd * l.f2009) + l.f16f) + (((((l.f13ba / 3.0) * l.f1fc7) + (t39 * l.f1fcb)) * l.f2035) + (t3a * l.f2039))), (((l.fdd * l.f200a) + l.f170) + (((((l.f13bb / 3.0) * l.f1fc7) + (t39 * l.f1fcc)) * l.f2035) + (t3a * l.f203a))), (((l.fdd * l.f200b) + l.f171) + (((((l.f13bc / 3.0) * l.f1fc7) + (t39 * l.f1fcd)) * l.f2035) + (t3a * l.f203b))), (((l.fdd * l.f200c) + l.f172) + (((((l.f13bd / 3.0) * l.f1fc7) + (t39 * l.f1fce)) * l.f2035) + (t3a * l.f203c))), (((l.fdd * l.f200d) + l.f173) + (((((l.f13be / 3.0) * l.f1fc7) + (t39 * l.f1fcf)) * l.f2035) + (t3a * l.f203d))), (((l.fdd * l.f200e) + l.f174) + (((((l.f13bf / 3.0) * l.f1fc7) + (t39 * l.f1fd0)) * l.f2035) + (t3a * l.f203e))), (((l.fdd * l.f200f) + l.f175) + (((((l.f13c0 / 3.0) * l.f1fc7) + (t39 * l.f1fd1)) * l.f2035) + (t3a * l.f203f))), (((l.fdd * l.f2006) + l.f16c) + (((((l.f13b7 / 3.0) * l.f1fc7) + (t39 * l.f1fc8)) * l.f2035) + (t3a * l.f2036))), (((l.fdd * l.f2007) + l.f16d) + (((((l.f13b8 / 3.0) * l.f1fc7) + (t39 * l.f1fc9)) * l.f2035) + (t3a * l.f2037))), (((l.fdd * l.f2008) + l.f16e) + (((((l.f13b9 / 3.0) * l.f1fc7) + (t39 * l.f1fca)) * l.f2035) + (t3a * l.f2038))), );l.f35c = 0.0;}
        if (((((l.fb4b != 0.0) && (l.fb5b == 0.0)) && (l.fb63 == 0.0)) && (l.fb65 == 0.0)) && (l.fb67 == 0.0)) {let t3d: f64 = (l.fdd * l.f2005);let t3e: f64 = (t3d + l.f16b);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t3e, ((l.fdd * l.f2009) + l.f16f), ((l.fdd * l.f200a) + l.f170), ((l.fdd * l.f200b) + l.f171), ((l.fdd * l.f200c) + l.f172), ((l.fdd * l.f200d) + l.f173), ((l.fdd * l.f200e) + l.f174), ((l.fdd * l.f200f) + l.f175), ((l.fdd * l.f2006) + l.f16c), ((l.fdd * l.f2007) + l.f16d), ((l.fdd * l.f2008) + l.f16e), );l.f35c = 0.0;}
        if (l.fb4b == 0.0) {let t3f: f64 = (l.f121 * l.f1fee);(l.f369, l.f36d, l.f36e, l.f36f, l.f370, l.f371, l.f372, l.f373, l.f36a, l.f36b, l.f36c, ) = (t3f, (l.f121 * l.f1ff2), (l.f121 * l.f1ff3), (l.f121 * l.f1ff4), (l.f121 * l.f1ff5), (l.f121 * l.f1ff6), (l.f121 * l.f1ff7), (l.f121 * l.f1ff8), (l.f121 * l.f1fef), (l.f121 * l.f1ff0), (l.f121 * l.f1ff1), );l.f374 = 0.0;let t40: f64 = (l.fdd * l.f2005);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t40, (l.fdd * l.f2009), (l.fdd * l.f200a), (l.fdd * l.f200b), (l.fdd * l.f200c), (l.fdd * l.f200d), (l.fdd * l.f200e), (l.fdd * l.f200f), (l.fdd * l.f2006), (l.fdd * l.f2007), (l.fdd * l.f2008), );l.f35c = 0.0;}
        let t41: f64 = (l.f131 * l.f1fee);let t42: f64 = (l.f369 + t41);(l.f369, l.f36d, l.f36e, l.f36f, l.f370, l.f371, l.f372, l.f373, l.f36a, l.f36b, l.f36c, ) = (t42, (l.f36d + ((l.f135 * l.f1fee) + (l.f131 * l.f1ff2))), (l.f36e + ((l.f136 * l.f1fee) + (l.f131 * l.f1ff3))), (l.f36f + ((l.f137 * l.f1fee) + (l.f131 * l.f1ff4))), (l.f370 + ((l.f138 * l.f1fee) + (l.f131 * l.f1ff5))), (l.f371 + ((l.f139 * l.f1fee) + (l.f131 * l.f1ff6))), (l.f372 + ((l.f13a * l.f1fee) + (l.f131 * l.f1ff7))), (l.f373 + ((l.f13b * l.f1fee) + (l.f131 * l.f1ff8))), (l.f36a + ((l.f132 * l.f1fee) + (l.f131 * l.f1fef))), (l.f36b + ((l.f133 * l.f1fee) + (l.f131 * l.f1ff0))), (l.f36c + ((l.f134 * l.f1fee) + (l.f131 * l.f1ff1))), );l.f374 = 0.0;let t43: f64 = (l.fdf * l.f2005);let t44: f64 = (l.f351 + t43);(l.f351, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354, ) = (t44, (l.f355 + ((l.fe3 * l.f2005) + (l.fdf * l.f2009))), (l.f356 + ((l.fe4 * l.f2005) + (l.fdf * l.f200a))), (l.f357 + ((l.fe5 * l.f2005) + (l.fdf * l.f200b))), (l.f358 + ((l.fe6 * l.f2005) + (l.fdf * l.f200c))), (l.f359 + ((l.fe7 * l.f2005) + (l.fdf * l.f200d))), (l.f35a + ((l.fe8 * l.f2005) + (l.fdf * l.f200e))), (l.f35b + ((l.fe9 * l.f2005) + (l.fdf * l.f200f))), (l.f352 + ((l.fe0 * l.f2005) + (l.fdf * l.f2006))), (l.f353 + ((l.fe1 * l.f2005) + (l.fdf * l.f2007))), (l.f354 + ((l.fe2 * l.f2005) + (l.fdf * l.f2008))), );l.f35c = 0.0;let t45: f64 = if l.f392 == 3.0 { 1.0 } else { 0.0 };l.fb69 = t45;l.fb6a = 0.0;
        if (l.fb69 != 0.0) {let t46: f64 = (l.f2500 + 0.02);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t46, 0.0, 0.0, 0.0, 0.0, l.f2502, l.f2503, 0.0, l.f2501, 0.0, 0.0, );l.f1fd2 = 0.0;}
        if (l.fb69 == 0.0) {let t47: f64 = (l.f24de + 0.02);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t47, 0.0, 0.0, 0.0, 0.0, l.f24e4, l.f24e5, l.f24e6, 0.0, 0.0, 0.0, );l.f1fd2 = 0.0;}
        let t48: f64 = (l.f1fc7 * l.f1fc7);let t49: f64 = (4.0 * 0.02);let t4a: f64 = (t48 + t49);let t4b: f64 = (t4a).sqrt();(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (t4b, (((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)) / (2.0 * t4b)), (((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)) / (2.0 * t4b)), (((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)) / (2.0 * t4b)), (((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)) / (2.0 * t4b)), (((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)) / (2.0 * t4b)), (((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)) / (2.0 * t4b)), (((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)) / (2.0 * t4b)), (((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)) / (2.0 * t4b)), (((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)) / (2.0 * t4b)), (((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)) / (2.0 * t4b)), );l.f2040 = 0.0;let t4c: f64 = (l.f1fc7 - l.f2035);let t4d: f64 = (0.5 * t4c);(l.f2067, l.f206b, l.f206c, l.f206d, l.f206e, l.f206f, l.f2070, l.f2071, l.f2068, l.f2069, l.f206a, ) = (t4d, (0.5 * (l.f1fcb - l.f2039)), (0.5 * (l.f1fcc - l.f203a)), (0.5 * (l.f1fcd - l.f203b)), (0.5 * (l.f1fce - l.f203c)), (0.5 * (l.f1fcf - l.f203d)), (0.5 * (l.f1fd0 - l.f203e)), (0.5 * (l.f1fd1 - l.f203f)), (0.5 * (l.f1fc8 - l.f2036)), (0.5 * (l.f1fc9 - l.f2037)), (0.5 * (l.f1fca - l.f2038)), );l.f2072 = 0.0;let t4e: f64 = (l.f1cdf * l.f12aa);(l.f207f, l.f2083, l.f2084, l.f2085, l.f2086, l.f2087, l.f2088, l.f2089, l.f2080, l.f2081, l.f2082, ) = (t4e, ((l.f1ce3 * l.f12aa) + (l.f1cdf * l.f12ae)), ((l.f1ce4 * l.f12aa) + (l.f1cdf * l.f12af)), ((l.f1ce5 * l.f12aa) + (l.f1cdf * l.f12b0)), ((l.f1ce6 * l.f12aa) + (l.f1cdf * l.f12b1)), ((l.f1ce7 * l.f12aa) + (l.f1cdf * l.f12b2)), ((l.f1ce8 * l.f12aa) + (l.f1cdf * l.f12b3)), ((l.f1ce9 * l.f12aa) + (l.f1cdf * l.f12b4)), ((l.f1ce0 * l.f12aa) + (l.f1cdf * l.f12ab)), ((l.f1ce1 * l.f12aa) + (l.f1cdf * l.f12ac)), ((l.f1ce2 * l.f12aa) + (l.f1cdf * l.f12ad)), );l.f208a = 0.0;let t4f: f64 = (4.0 * l.f2067);let t50: f64 = (t4f / l.f132e);let t51: f64 = (1.0 - t50);let t52: f64 = (t51).sqrt();(l.f20a3, l.f20a7, l.f20a8, l.f20a9, l.f20aa, l.f20ab, l.f20ac, l.f20ad, l.f20a4, l.f20a5, l.f20a6, ) = (t52, ((-((((4.0 * l.f206b) * l.f132e) - (t4f * l.f1332)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f206c) * l.f132e) - (t4f * l.f1333)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f206d) * l.f132e) - (t4f * l.f1334)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f206e) * l.f132e) - (t4f * l.f1335)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f206f) * l.f132e) - (t4f * l.f1336)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f2070) * l.f132e) - (t4f * l.f1337)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f2071) * l.f132e) - (t4f * l.f1338)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f2068) * l.f132e) - (t4f * l.f132f)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f2069) * l.f132e) - (t4f * l.f1330)) / (l.f132e * l.f132e))) / (2.0 * t52)), ((-((((4.0 * l.f206a) * l.f132e) - (t4f * l.f1331)) / (l.f132e * l.f132e))) / (2.0 * t52)), );l.f20ae = 0.0;let t53: f64 = if l.f392 == 3.0 { 1.0 } else { 0.0 };l.fb6b = t53;l.fb6c = 0.0;
        if (l.fb6b != 0.0) {let t54: f64 = (l.f12b6 + l.f207f);let t55: f64 = (t54 * l.f2500);let t56: f64 = (0.5 * l.f132e);let t57: f64 = (l.f20a3 - 1.0);let t58: f64 = (t56 * t57);let t59: f64 = (l.f2067 + t58);let t5a: f64 = (l.f207f * t59);let t5b: f64 = (t55 - t5a);(l.f1e0d, l.f1e11, l.f1e12, l.f1e13, l.f1e14, l.f1e15, l.f1e16, l.f1e17, l.f1e0e, l.f1e0f, l.f1e10, ) = (t5b, (((l.f12ba + l.f2083) * l.f2500) - ((l.f2083 * t59) + (l.f207f * (l.f206b + (((0.5 * l.f1332) * t57) + (t56 * l.f20a7)))))), (((l.f12bb + l.f2084) * l.f2500) - ((l.f2084 * t59) + (l.f207f * (l.f206c + (((0.5 * l.f1333) * t57) + (t56 * l.f20a8)))))), (((l.f12bc + l.f2085) * l.f2500) - ((l.f2085 * t59) + (l.f207f * (l.f206d + (((0.5 * l.f1334) * t57) + (t56 * l.f20a9)))))), (((l.f12bd + l.f2086) * l.f2500) - ((l.f2086 * t59) + (l.f207f * (l.f206e + (((0.5 * l.f1335) * t57) + (t56 * l.f20aa)))))), ((((l.f12be + l.f2087) * l.f2500) + (t54 * l.f2502)) - ((l.f2087 * t59) + (l.f207f * (l.f206f + (((0.5 * l.f1336) * t57) + (t56 * l.f20ab)))))), ((((l.f12bf + l.f2088) * l.f2500) + (t54 * l.f2503)) - ((l.f2088 * t59) + (l.f207f * (l.f2070 + (((0.5 * l.f1337) * t57) + (t56 * l.f20ac)))))), (((l.f12c0 + l.f2089) * l.f2500) - ((l.f2089 * t59) + (l.f207f * (l.f2071 + (((0.5 * l.f1338) * t57) + (t56 * l.f20ad)))))), ((((l.f12b7 + l.f2080) * l.f2500) + (t54 * l.f2501)) - ((l.f2080 * t59) + (l.f207f * (l.f2068 + (((0.5 * l.f132f) * t57) + (t56 * l.f20a4)))))), (((l.f12b8 + l.f2081) * l.f2500) - ((l.f2081 * t59) + (l.f207f * (l.f2069 + (((0.5 * l.f1330) * t57) + (t56 * l.f20a5)))))), (((l.f12b9 + l.f2082) * l.f2500) - ((l.f2082 * t59) + (l.f207f * (l.f206a + (((0.5 * l.f1331) * t57) + (t56 * l.f20a6)))))), );l.f1e18 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_126(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fb6b == 0.0) {let t5c: f64 = (l.f12b6 + l.f207f);let t5d: f64 = (t5c * l.f24de);let t5e: f64 = (0.5 * l.f132e);let t5f: f64 = (l.f20a3 - 1.0);let t60: f64 = (t5e * t5f);let t61: f64 = (l.f2067 + t60);let t62: f64 = (l.f207f * t61);let t63: f64 = (t5d - t62);(l.f1e0d, l.f1e11, l.f1e12, l.f1e13, l.f1e14, l.f1e15, l.f1e16, l.f1e17, l.f1e0e, l.f1e0f, l.f1e10, ) = (t63, (((l.f12ba + l.f2083) * l.f24de) - ((l.f2083 * t61) + (l.f207f * (l.f206b + (((0.5 * l.f1332) * t5f) + (t5e * l.f20a7)))))), (((l.f12bb + l.f2084) * l.f24de) - ((l.f2084 * t61) + (l.f207f * (l.f206c + (((0.5 * l.f1333) * t5f) + (t5e * l.f20a8)))))), (((l.f12bc + l.f2085) * l.f24de) - ((l.f2085 * t61) + (l.f207f * (l.f206d + (((0.5 * l.f1334) * t5f) + (t5e * l.f20a9)))))), (((l.f12bd + l.f2086) * l.f24de) - ((l.f2086 * t61) + (l.f207f * (l.f206e + (((0.5 * l.f1335) * t5f) + (t5e * l.f20aa)))))), ((((l.f12be + l.f2087) * l.f24de) + (t5c * l.f24e4)) - ((l.f2087 * t61) + (l.f207f * (l.f206f + (((0.5 * l.f1336) * t5f) + (t5e * l.f20ab)))))), ((((l.f12bf + l.f2088) * l.f24de) + (t5c * l.f24e5)) - ((l.f2088 * t61) + (l.f207f * (l.f2070 + (((0.5 * l.f1337) * t5f) + (t5e * l.f20ac)))))), ((((l.f12c0 + l.f2089) * l.f24de) + (t5c * l.f24e6)) - ((l.f2089 * t61) + (l.f207f * (l.f2071 + (((0.5 * l.f1338) * t5f) + (t5e * l.f20ad)))))), (((l.f12b7 + l.f2080) * l.f24de) - ((l.f2080 * t61) + (l.f207f * (l.f2068 + (((0.5 * l.f132f) * t5f) + (t5e * l.f20a4)))))), (((l.f12b8 + l.f2081) * l.f24de) - ((l.f2081 * t61) + (l.f207f * (l.f2069 + (((0.5 * l.f1330) * t5f) + (t5e * l.f20a5)))))), (((l.f12b9 + l.f2082) * l.f24de) - ((l.f2082 * t61) + (l.f207f * (l.f206a + (((0.5 * l.f1331) * t5f) + (t5e * l.f20a6)))))), );l.f1e18 = 0.0;}
        let t64: f64 = if l.f392 == 3.0 { 1.0 } else { 0.0 };l.fb6d = t64;l.fb6e = 0.0;
        if (l.fb6d != 0.0) {let t65: f64 = (l.f2505 + 0.02);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t65, 0.0, 0.0, 0.0, 0.0, 0.0, l.f2507, 0.0, l.f2506, 0.0, 0.0, );l.f1fd2 = 0.0;}
        if (l.fb6d == 0.0) {let t66: f64 = (l.f2519 + 0.02);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t66, 0.0, 0.0, 0.0, 0.0, 0.0, l.f251f, l.f2520, 0.0, 0.0, 0.0, );l.f1fd2 = 0.0;}
        let t67: f64 = (l.f1fc7 * l.f1fc7);let t68: f64 = (4.0 * 0.02);let t69: f64 = (t67 + t68);let t6a: f64 = (t69).sqrt();(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (t6a, (((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)) / (2.0 * t6a)), (((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)) / (2.0 * t6a)), (((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)) / (2.0 * t6a)), (((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)) / (2.0 * t6a)), (((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)) / (2.0 * t6a)), (((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)) / (2.0 * t6a)), (((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)) / (2.0 * t6a)), (((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)) / (2.0 * t6a)), (((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)) / (2.0 * t6a)), (((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)) / (2.0 * t6a)), );l.f2040 = 0.0;let t6b: f64 = (l.f1fc7 - l.f2035);let t6c: f64 = (0.5 * t6b);(l.f2067, l.f206b, l.f206c, l.f206d, l.f206e, l.f206f, l.f2070, l.f2071, l.f2068, l.f2069, l.f206a, ) = (t6c, (0.5 * (l.f1fcb - l.f2039)), (0.5 * (l.f1fcc - l.f203a)), (0.5 * (l.f1fcd - l.f203b)), (0.5 * (l.f1fce - l.f203c)), (0.5 * (l.f1fcf - l.f203d)), (0.5 * (l.f1fd0 - l.f203e)), (0.5 * (l.f1fd1 - l.f203f)), (0.5 * (l.f1fc8 - l.f2036)), (0.5 * (l.f1fc9 - l.f2037)), (0.5 * (l.f1fca - l.f2038)), );l.f2072 = 0.0;let t6d: f64 = (l.f1cf7 * l.f12e6);(l.f207f, l.f2083, l.f2084, l.f2085, l.f2086, l.f2087, l.f2088, l.f2089, l.f2080, l.f2081, l.f2082, ) = (t6d, ((l.f1cfb * l.f12e6) + (l.f1cf7 * l.f12ea)), ((l.f1cfc * l.f12e6) + (l.f1cf7 * l.f12eb)), ((l.f1cfd * l.f12e6) + (l.f1cf7 * l.f12ec)), ((l.f1cfe * l.f12e6) + (l.f1cf7 * l.f12ed)), ((l.f1cff * l.f12e6) + (l.f1cf7 * l.f12ee)), ((l.f1d00 * l.f12e6) + (l.f1cf7 * l.f12ef)), ((l.f1d01 * l.f12e6) + (l.f1cf7 * l.f12f0)), ((l.f1cf8 * l.f12e6) + (l.f1cf7 * l.f12e7)), ((l.f1cf9 * l.f12e6) + (l.f1cf7 * l.f12e8)), ((l.f1cfa * l.f12e6) + (l.f1cf7 * l.f12e9)), );l.f208a = 0.0;let t6e: f64 = (4.0 * l.f2067);let t6f: f64 = (t6e / l.f132e);let t70: f64 = (1.0 - t6f);let t71: f64 = (t70).sqrt();(l.f20a3, l.f20a7, l.f20a8, l.f20a9, l.f20aa, l.f20ab, l.f20ac, l.f20ad, l.f20a4, l.f20a5, l.f20a6, ) = (t71, ((-((((4.0 * l.f206b) * l.f132e) - (t6e * l.f1332)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f206c) * l.f132e) - (t6e * l.f1333)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f206d) * l.f132e) - (t6e * l.f1334)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f206e) * l.f132e) - (t6e * l.f1335)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f206f) * l.f132e) - (t6e * l.f1336)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f2070) * l.f132e) - (t6e * l.f1337)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f2071) * l.f132e) - (t6e * l.f1338)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f2068) * l.f132e) - (t6e * l.f132f)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f2069) * l.f132e) - (t6e * l.f1330)) / (l.f132e * l.f132e))) / (2.0 * t71)), ((-((((4.0 * l.f206a) * l.f132e) - (t6e * l.f1331)) / (l.f132e * l.f132e))) / (2.0 * t71)), );l.f20ae = 0.0;let t72: f64 = if l.f392 == 3.0 { 1.0 } else { 0.0 };l.fb6f = t72;l.fb70 = 0.0;
        if (l.fb6f != 0.0) {let t73: f64 = (l.f12f2 + l.f207f);let t74: f64 = (t73 * l.f2505);let t75: f64 = (0.5 * l.f132e);let t76: f64 = (l.f20a3 - 1.0);let t77: f64 = (t75 * t76);let t78: f64 = (l.f2067 + t77);let t79: f64 = (l.f207f * t78);let t7a: f64 = (t74 - t79);(l.f1e25, l.f1e29, l.f1e2a, l.f1e2b, l.f1e2c, l.f1e2d, l.f1e2e, l.f1e2f, l.f1e26, l.f1e27, l.f1e28, ) = (t7a, (((l.f12f6 + l.f2083) * l.f2505) - ((l.f2083 * t78) + (l.f207f * (l.f206b + (((0.5 * l.f1332) * t76) + (t75 * l.f20a7)))))), (((l.f12f7 + l.f2084) * l.f2505) - ((l.f2084 * t78) + (l.f207f * (l.f206c + (((0.5 * l.f1333) * t76) + (t75 * l.f20a8)))))), (((l.f12f8 + l.f2085) * l.f2505) - ((l.f2085 * t78) + (l.f207f * (l.f206d + (((0.5 * l.f1334) * t76) + (t75 * l.f20a9)))))), (((l.f12f9 + l.f2086) * l.f2505) - ((l.f2086 * t78) + (l.f207f * (l.f206e + (((0.5 * l.f1335) * t76) + (t75 * l.f20aa)))))), (((l.f12fa + l.f2087) * l.f2505) - ((l.f2087 * t78) + (l.f207f * (l.f206f + (((0.5 * l.f1336) * t76) + (t75 * l.f20ab)))))), ((((l.f12fb + l.f2088) * l.f2505) + (t73 * l.f2507)) - ((l.f2088 * t78) + (l.f207f * (l.f2070 + (((0.5 * l.f1337) * t76) + (t75 * l.f20ac)))))), (((l.f12fc + l.f2089) * l.f2505) - ((l.f2089 * t78) + (l.f207f * (l.f2071 + (((0.5 * l.f1338) * t76) + (t75 * l.f20ad)))))), ((((l.f12f3 + l.f2080) * l.f2505) + (t73 * l.f2506)) - ((l.f2080 * t78) + (l.f207f * (l.f2068 + (((0.5 * l.f132f) * t76) + (t75 * l.f20a4)))))), (((l.f12f4 + l.f2081) * l.f2505) - ((l.f2081 * t78) + (l.f207f * (l.f2069 + (((0.5 * l.f1330) * t76) + (t75 * l.f20a5)))))), (((l.f12f5 + l.f2082) * l.f2505) - ((l.f2082 * t78) + (l.f207f * (l.f206a + (((0.5 * l.f1331) * t76) + (t75 * l.f20a6)))))), );l.f1e30 = 0.0;}
        if (l.fb6f == 0.0) {let t7b: f64 = (l.f12f2 + l.f207f);let t7c: f64 = (t7b * l.f2519);let t7d: f64 = (0.5 * l.f132e);let t7e: f64 = (l.f20a3 - 1.0);let t7f: f64 = (t7d * t7e);let t80: f64 = (l.f2067 + t7f);let t81: f64 = (l.f207f * t80);let t82: f64 = (t7c - t81);(l.f1e25, l.f1e29, l.f1e2a, l.f1e2b, l.f1e2c, l.f1e2d, l.f1e2e, l.f1e2f, l.f1e26, l.f1e27, l.f1e28, ) = (t82, (((l.f12f6 + l.f2083) * l.f2519) - ((l.f2083 * t80) + (l.f207f * (l.f206b + (((0.5 * l.f1332) * t7e) + (t7d * l.f20a7)))))), (((l.f12f7 + l.f2084) * l.f2519) - ((l.f2084 * t80) + (l.f207f * (l.f206c + (((0.5 * l.f1333) * t7e) + (t7d * l.f20a8)))))), (((l.f12f8 + l.f2085) * l.f2519) - ((l.f2085 * t80) + (l.f207f * (l.f206d + (((0.5 * l.f1334) * t7e) + (t7d * l.f20a9)))))), (((l.f12f9 + l.f2086) * l.f2519) - ((l.f2086 * t80) + (l.f207f * (l.f206e + (((0.5 * l.f1335) * t7e) + (t7d * l.f20aa)))))), (((l.f12fa + l.f2087) * l.f2519) - ((l.f2087 * t80) + (l.f207f * (l.f206f + (((0.5 * l.f1336) * t7e) + (t7d * l.f20ab)))))), ((((l.f12fb + l.f2088) * l.f2519) + (t7b * l.f251f)) - ((l.f2088 * t80) + (l.f207f * (l.f2070 + (((0.5 * l.f1337) * t7e) + (t7d * l.f20ac)))))), ((((l.f12fc + l.f2089) * l.f2519) + (t7b * l.f2520)) - ((l.f2089 * t80) + (l.f207f * (l.f2071 + (((0.5 * l.f1338) * t7e) + (t7d * l.f20ad)))))), (((l.f12f3 + l.f2080) * l.f2519) - ((l.f2080 * t80) + (l.f207f * (l.f2068 + (((0.5 * l.f132f) * t7e) + (t7d * l.f20a4)))))), (((l.f12f4 + l.f2081) * l.f2519) - ((l.f2081 * t80) + (l.f207f * (l.f2069 + (((0.5 * l.f1330) * t7e) + (t7d * l.f20a5)))))), (((l.f12f5 + l.f2082) * l.f2519) - ((l.f2082 * t80) + (l.f207f * (l.f206a + (((0.5 * l.f1331) * t7e) + (t7d * l.f20a6)))))), );l.f1e30 = 0.0;}
        let t83: f64 = if l.f305 != 1.0 { 1.0 } else { 0.0 };l.fb71 = t83;l.fb72 = 0.0;
        if (l.fb71 != 0.0) {let t84: f64 = (l.f1e0d * l.f305);(l.f1e0d, l.f1e11, l.f1e12, l.f1e13, l.f1e14, l.f1e15, l.f1e16, l.f1e17, l.f1e0e, l.f1e0f, l.f1e10, ) = (t84, (l.f1e11 * l.f305), (l.f1e12 * l.f305), (l.f1e13 * l.f305), (l.f1e14 * l.f305), (l.f1e15 * l.f305), (l.f1e16 * l.f305), (l.f1e17 * l.f305), (l.f1e0e * l.f305), (l.f1e0f * l.f305), (l.f1e10 * l.f305), );l.f1e18 = 0.0;let t85: f64 = (l.f1e25 * l.f305);(l.f1e25, l.f1e29, l.f1e2a, l.f1e2b, l.f1e2c, l.f1e2d, l.f1e2e, l.f1e2f, l.f1e26, l.f1e27, l.f1e28, ) = (t85, (l.f1e29 * l.f305), (l.f1e2a * l.f305), (l.f1e2b * l.f305), (l.f1e2c * l.f305), (l.f1e2d * l.f305), (l.f1e2e * l.f305), (l.f1e2f * l.f305), (l.f1e26 * l.f305), (l.f1e27 * l.f305), (l.f1e28 * l.f305), );l.f1e30 = 0.0;}
        (l.f1e19, l.f1e1d, l.f1e1e, l.f1e1f, l.f1e20, l.f1e21, l.f1e22, l.f1e23, l.f1e1a, l.f1e1b, l.f1e1c, ) = (l.f1e01, l.f1e05, l.f1e06, l.f1e07, l.f1e08, l.f1e09, l.f1e0a, l.f1e0b, l.f1e02, l.f1e03, l.f1e04, );l.f1e24 = 0.0;let t86: f64 = (l.f1e25 + l.f1e0d);(l.f1e55, l.f1e59, l.f1e5a, l.f1e5b, l.f1e5c, l.f1e5d, l.f1e5e, l.f1e5f, l.f1e56, l.f1e57, l.f1e58, ) = (t86, (l.f1e29 + l.f1e11), (l.f1e2a + l.f1e12), (l.f1e2b + l.f1e13), (l.f1e2c + l.f1e14), (l.f1e2d + l.f1e15), (l.f1e2e + l.f1e16), (l.f1e2f + l.f1e17), (l.f1e26 + l.f1e0e), (l.f1e27 + l.f1e0f), (l.f1e28 + l.f1e10), );l.f1e60 = 0.0;let t87: f64 = (l.f1e19 + l.f1e55);(l.f1e01, l.f1e05, l.f1e06, l.f1e07, l.f1e08, l.f1e09, l.f1e0a, l.f1e0b, l.f1e02, l.f1e03, l.f1e04, ) = (t87, (l.f1e1d + l.f1e59), (l.f1e1e + l.f1e5a), (l.f1e1f + l.f1e5b), (l.f1e20 + l.f1e5c), (l.f1e21 + l.f1e5d), (l.f1e22 + l.f1e5e), (l.f1e23 + l.f1e5f), (l.f1e1a + l.f1e56), (l.f1e1b + l.f1e57), (l.f1e1c + l.f1e58), );l.f1e0c = 0.0;let t88: f64 = if p.p213 == 0.0 { 1.0 } else { 0.0 };l.fb73 = t88;l.fb74 = 0.0;let t89: f64 = if p.p213 == 1.0 { 1.0 } else { 0.0 };l.fb75 = t89;l.fb76 = 0.0;
        if ((l.fb75 != 0.0) && (l.fb73 == 0.0)) {let t8a: f64 = (l.f1f7 + l.f1e9);let t8b: f64 = (t8a + l.f203);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t8b, ((l.f1fb + l.f1ed) + l.f207), ((l.f1fc + l.f1ee) + l.f208), ((l.f1fd + l.f1ef) + l.f209), ((l.f1fe + l.f1f0) + l.f20a), ((l.f1ff + l.f1f1) + l.f20b), ((l.f200 + l.f1f2) + l.f20c), ((l.f201 + l.f1f3) + l.f20d), ((l.f1f8 + l.f1ea) + l.f204), ((l.f1f9 + l.f1eb) + l.f205), ((l.f1fa + l.f1ec) + l.f206), );l.f1fd2 = 0.0;let t8c: f64 = (l.f1fc7 * l.f1fc7);(l.f1fc7, l.f1fcb, l.f1fcc, l.f1fcd, l.f1fce, l.f1fcf, l.f1fd0, l.f1fd1, l.f1fc8, l.f1fc9, l.f1fca, ) = (t8c, ((l.f1fcb * l.f1fc7) + (l.f1fc7 * l.f1fcb)), ((l.f1fcc * l.f1fc7) + (l.f1fc7 * l.f1fcc)), ((l.f1fcd * l.f1fc7) + (l.f1fc7 * l.f1fcd)), ((l.f1fce * l.f1fc7) + (l.f1fc7 * l.f1fce)), ((l.f1fcf * l.f1fc7) + (l.f1fc7 * l.f1fcf)), ((l.f1fd0 * l.f1fc7) + (l.f1fc7 * l.f1fd0)), ((l.f1fd1 * l.f1fc7) + (l.f1fc7 * l.f1fd1)), ((l.f1fc8 * l.f1fc7) + (l.f1fc7 * l.f1fc8)), ((l.f1fc9 * l.f1fc7) + (l.f1fc7 * l.f1fc9)), ((l.f1fca * l.f1fc7) + (l.f1fc7 * l.f1fca)), );l.f1fd2 = 0.0;let t8d: f64 = (2.0 * l.f262f);let t8e: f64 = (t8d / l.f429);(l.f7f6, l.f7fa, l.f7fb, l.f7fc, l.f7fd, l.f7fe, l.f7ff, l.f800, l.f7f7, l.f7f8, l.f7f9, ) = (t8e, ((((2.0 * l.f2633) * l.f429) - (t8d * l.f42d)) / (l.f429 * l.f429)), ((((2.0 * l.f2634) * l.f429) - (t8d * l.f42e)) / (l.f429 * l.f429)), ((((2.0 * l.f2635) * l.f429) - (t8d * l.f42f)) / (l.f429 * l.f429)), ((((2.0 * l.f2636) * l.f429) - (t8d * l.f430)) / (l.f429 * l.f429)), ((((2.0 * l.f2637) * l.f429) - (t8d * l.f431)) / (l.f429 * l.f429)), ((((2.0 * l.f2638) * l.f429) - (t8d * l.f432)) / (l.f429 * l.f429)), ((((2.0 * l.f2639) * l.f429) - (t8d * l.f433)) / (l.f429 * l.f429)), ((((2.0 * l.f2630) * l.f429) - (t8d * l.f42a)) / (l.f429 * l.f429)), ((((2.0 * l.f2631) * l.f429) - (t8d * l.f42b)) / (l.f429 * l.f429)), ((((2.0 * l.f2632) * l.f429) - (t8d * l.f42c)) / (l.f429 * l.f429)), );l.f801 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_127(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.fb75 != 0.0) && (l.fb73 == 0.0)) {let t8f: f64 = (l.f7f6 * l.f1724);let t90: f64 = (l.f465 / t8f);(l.f20bb, l.f20bf, l.f20c0, l.f20c1, l.f20c2, l.f20c3, l.f20c4, l.f20c5, l.f20bc, l.f20bd, l.f20be, ) = (t90, (((l.f469 * t8f) - (l.f465 * ((l.f7fa * l.f1724) + (l.f7f6 * l.f1728)))) / (t8f * t8f)), (((l.f46a * t8f) - (l.f465 * ((l.f7fb * l.f1724) + (l.f7f6 * l.f1729)))) / (t8f * t8f)), (((l.f46b * t8f) - (l.f465 * ((l.f7fc * l.f1724) + (l.f7f6 * l.f172a)))) / (t8f * t8f)), (((l.f46c * t8f) - (l.f465 * ((l.f7fd * l.f1724) + (l.f7f6 * l.f172b)))) / (t8f * t8f)), (((l.f46d * t8f) - (l.f465 * ((l.f7fe * l.f1724) + (l.f7f6 * l.f172c)))) / (t8f * t8f)), (((l.f46e * t8f) - (l.f465 * ((l.f7ff * l.f1724) + (l.f7f6 * l.f172d)))) / (t8f * t8f)), (((l.f46f * t8f) - (l.f465 * ((l.f800 * l.f1724) + (l.f7f6 * l.f172e)))) / (t8f * t8f)), (((l.f466 * t8f) - (l.f465 * ((l.f7f7 * l.f1724) + (l.f7f6 * l.f1725)))) / (t8f * t8f)), (((l.f467 * t8f) - (l.f465 * ((l.f7f8 * l.f1724) + (l.f7f6 * l.f1726)))) / (t8f * t8f)), (((l.f468 * t8f) - (l.f465 * ((l.f7f9 * l.f1724) + (l.f7f6 * l.f1727)))) / (t8f * t8f)), );l.f20c6 = 0.0;let t91: f64 = (l.f20bb * l.f20bb);(l.f20bb, l.f20bf, l.f20c0, l.f20c1, l.f20c2, l.f20c3, l.f20c4, l.f20c5, l.f20bc, l.f20bd, l.f20be, ) = (t91, ((l.f20bf * l.f20bb) + (l.f20bb * l.f20bf)), ((l.f20c0 * l.f20bb) + (l.f20bb * l.f20c0)), ((l.f20c1 * l.f20bb) + (l.f20bb * l.f20c1)), ((l.f20c2 * l.f20bb) + (l.f20bb * l.f20c2)), ((l.f20c3 * l.f20bb) + (l.f20bb * l.f20c3)), ((l.f20c4 * l.f20bb) + (l.f20bb * l.f20c4)), ((l.f20c5 * l.f20bb) + (l.f20bb * l.f20c5)), ((l.f20bc * l.f20bb) + (l.f20bb * l.f20bc)), ((l.f20bd * l.f20bb) + (l.f20bb * l.f20bd)), ((l.f20be * l.f20bb) + (l.f20bb * l.f20be)), );l.f20c6 = 0.0;let t92: f64 = (l.f20bb * l.f3ef);let t93: f64 = (t92 * l.f1724);let t94: f64 = (1.0 + t93);let t95: f64 = (l.f39a * t94);(l.ffdb, l.ffdf, l.ffe0, l.ffe1, l.ffe2, l.ffe3, l.ffe4, l.ffe5, l.ffdc, l.ffdd, l.ffde, ) = (t95, (l.f39a * (((l.f20bf * l.f3ef) * l.f1724) + (t92 * l.f1728))), (l.f39a * (((l.f20c0 * l.f3ef) * l.f1724) + (t92 * l.f1729))), (l.f39a * (((l.f20c1 * l.f3ef) * l.f1724) + (t92 * l.f172a))), (l.f39a * (((l.f20c2 * l.f3ef) * l.f1724) + (t92 * l.f172b))), (l.f39a * (((l.f20c3 * l.f3ef) * l.f1724) + (t92 * l.f172c))), (l.f39a * (((l.f20c4 * l.f3ef) * l.f1724) + (t92 * l.f172d))), (l.f39a * (((l.f20c5 * l.f3ef) * l.f1724) + (t92 * l.f172e))), (l.f39a * (((l.f20bc * l.f3ef) * l.f1724) + (t92 * l.f1725))), (l.f39a * (((l.f20bd * l.f3ef) * l.f1724) + (t92 * l.f1726))), (l.f39a * (((l.f20be * l.f3ef) * l.f1724) + (t92 * l.f1727))), );l.ffe6 = 0.0;}
        if ((l.fb75 != 0.0) && (l.fb73 == 0.0)) {let t96: f64 = (l.f1f7 + l.f203);let t97: f64 = (l.ffdb * t96);let t98: f64 = (t97 + l.f1e9);(l.f2035, l.f2039, l.f203a, l.f203b, l.f203c, l.f203d, l.f203e, l.f203f, l.f2036, l.f2037, l.f2038, ) = (t98, (((l.ffdf * t96) + (l.ffdb * (l.f1fb + l.f207))) + l.f1ed), (((l.ffe0 * t96) + (l.ffdb * (l.f1fc + l.f208))) + l.f1ee), (((l.ffe1 * t96) + (l.ffdb * (l.f1fd + l.f209))) + l.f1ef), (((l.ffe2 * t96) + (l.ffdb * (l.f1fe + l.f20a))) + l.f1f0), (((l.ffe3 * t96) + (l.ffdb * (l.f1ff + l.f20b))) + l.f1f1), (((l.ffe4 * t96) + (l.ffdb * (l.f200 + l.f20c))) + l.f1f2), (((l.ffe5 * t96) + (l.ffdb * (l.f201 + l.f20d))) + l.f1f3), (((l.ffdc * t96) + (l.ffdb * (l.f1f8 + l.f204))) + l.f1ea), (((l.ffdd * t96) + (l.ffdb * (l.f1f9 + l.f205))) + l.f1eb), (((l.ffde * t96) + (l.ffdb * (l.f1fa + l.f206))) + l.f1ec), );l.f2040 = 0.0;let t99: f64 = (l.f2035 * l.f2035);let t9a: f64 = (t99 / l.f221);(l.f2067, l.f206b, l.f206c, l.f206d, l.f206e, l.f206f, l.f2070, l.f2071, l.f2068, l.f2069, l.f206a, ) = (t9a, (((((l.f2039 * l.f2035) + (l.f2035 * l.f2039)) * l.f221) - (t99 * l.f225)) / (l.f221 * l.f221)), (((((l.f203a * l.f2035) + (l.f2035 * l.f203a)) * l.f221) - (t99 * l.f226)) / (l.f221 * l.f221)), (((((l.f203b * l.f2035) + (l.f2035 * l.f203b)) * l.f221) - (t99 * l.f227)) / (l.f221 * l.f221)), (((((l.f203c * l.f2035) + (l.f2035 * l.f203c)) * l.f221) - (t99 * l.f228)) / (l.f221 * l.f221)), (((((l.f203d * l.f2035) + (l.f2035 * l.f203d)) * l.f221) - (t99 * l.f229)) / (l.f221 * l.f221)), (((((l.f203e * l.f2035) + (l.f2035 * l.f203e)) * l.f221) - (t99 * l.f22a)) / (l.f221 * l.f221)), (((((l.f203f * l.f2035) + (l.f2035 * l.f203f)) * l.f221) - (t99 * l.f22b)) / (l.f221 * l.f221)), (((((l.f2036 * l.f2035) + (l.f2035 * l.f2036)) * l.f221) - (t99 * l.f222)) / (l.f221 * l.f221)), (((((l.f2037 * l.f2035) + (l.f2035 * l.f2037)) * l.f221) - (t99 * l.f223)) / (l.f221 * l.f221)), (((((l.f2038 * l.f2035) + (l.f2035 * l.f2038)) * l.f221) - (t99 * l.f224)) / (l.f221 * l.f221)), );l.f2072 = 0.0;}
        let t9b: f64 = if l.f2f3 > 0.0 { 1.0 } else { 0.0 };l.fb7a = t9b;l.fb7b = 0.0;
        if (l.fb7a != 0.0) {let t9c: f64 = (l.f411 * l.f1de9);(l.f35d, l.f361, l.f362, l.f363, l.f364, l.f365, l.f366, l.f367, l.f35e, l.f35f, l.f360, ) = (t9c, (l.f411 * l.f1ded), (l.f411 * l.f1dee), (l.f411 * l.f1def), (l.f411 * l.f1df0), (l.f411 * l.f1df1), (l.f411 * l.f1df2), (l.f411 * l.f1df3), (l.f411 * l.f1dea), (l.f411 * l.f1deb), (l.f411 * l.f1dec), );l.f368 = 0.0;let t9d: f64 = (l.f411 * l.f1e6d);(l.f375, l.f379, l.f37a, l.f37b, l.f37c, l.f37d, l.f37e, l.f37f, l.f376, l.f377, l.f378, ) = (t9d, (l.f411 * l.f1e7d), (l.f411 * l.f1e7e), (l.f411 * l.f1e7f), (l.f411 * l.f1e80), (l.f411 * l.f1e81), (l.f411 * l.f1e82), (l.f411 * l.f1e83), (l.f411 * l.f1e7a), (l.f411 * l.f1e7b), (l.f411 * l.f1e7c), );l.f380 = 0.0;}
        if (l.fb7a == 0.0) {let t9e: f64 = (l.f411 * l.f1de9);(l.f375, l.f379, l.f37a, l.f37b, l.f37c, l.f37d, l.f37e, l.f37f, l.f376, l.f377, l.f378, ) = (t9e, (l.f411 * l.f1ded), (l.f411 * l.f1dee), (l.f411 * l.f1def), (l.f411 * l.f1df0), (l.f411 * l.f1df1), (l.f411 * l.f1df2), (l.f411 * l.f1df3), (l.f411 * l.f1dea), (l.f411 * l.f1deb), (l.f411 * l.f1dec), );l.f380 = 0.0;let t9f: f64 = (l.f411 * l.f1e6d);(l.f35d, l.f361, l.f362, l.f363, l.f364, l.f365, l.f366, l.f367, l.f35e, l.f35f, l.f360, ) = (t9f, (l.f411 * l.f1e7d), (l.f411 * l.f1e7e), (l.f411 * l.f1e7f), (l.f411 * l.f1e80), (l.f411 * l.f1e81), (l.f411 * l.f1e82), (l.f411 * l.f1e83), (l.f411 * l.f1e7a), (l.f411 * l.f1e7b), (l.f411 * l.f1e7c), );l.f368 = 0.0;}
        let ta0: f64 = if p.p37 == 3.0 { 1.0 } else { 0.0 };l.fb7d = ta0;l.fb7e = 0.0;let ta1: f64 = if ((p.p33 == 1.0) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };l.fb83 = ta1;l.fb84 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq4_e1143, eq4_e1143_d_n0, eq4_e1143_d_n3, eq4_e1143_d_n4, eq4_e1143_d_n5, eq4_e1143_d_n6, eq4_e1143_d_n7, eq4_e1143_d_n8, eq4_e1143_d_n9, eq4_e1143_d_n10, eq4_e1143_d_n11, eq4_e1143_d_n12,) = {
    if (l.fb79 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / l.f1eb4;let eq4_e1141: f64 = ((nv0 - nv7) * __rspice_inv_cse_0);let eq4_e1141_d_n0: f64 = (1.0 * __rspice_inv_cse_0);let eq4_e1141_d_n3: f64 = (-(((nv0 - nv7) * l.f1ec4) / (l.f1eb4 * l.f1eb4)));let eq4_e1141_d_n4: f64 = (-(((nv0 - nv7) * l.f1ec5) / (l.f1eb4 * l.f1eb4)));let eq4_e1141_d_n5: f64 = (-(((nv0 - nv7) * l.f1ec6) / (l.f1eb4 * l.f1eb4)));let eq4_e1141_d_n6: f64 = (-(((nv0 - nv7) * l.f1ec7) / (l.f1eb4 * l.f1eb4)));let eq4_e1141_d_n7: f64 = (((-l.f1eb4) - ((nv0 - nv7) * l.f1ec8)) / (l.f1eb4 * l.f1eb4));let eq4_e1141_d_n8: f64 = (-(((nv0 - nv7) * l.f1ec9) / (l.f1eb4 * l.f1eb4)));let eq4_e1141_d_n9: f64 = (-(((nv0 - nv7) * l.f1eca) / (l.f1eb4 * l.f1eb4)));let eq4_e1141_d_n10: f64 = (-(((nv0 - nv7) * l.f1ec1) / (l.f1eb4 * l.f1eb4)));let eq4_e1141_d_n11: f64 = (-(((nv0 - nv7) * l.f1ec2) / (l.f1eb4 * l.f1eb4)));let eq4_e1141_d_n12: f64 = (-(((nv0 - nv7) * l.f1ec3) / (l.f1eb4 * l.f1eb4)));
        (eq4_e1141, eq4_e1141_d_n0, eq4_e1141_d_n3, eq4_e1141_d_n4, eq4_e1141_d_n5, eq4_e1141_d_n6, eq4_e1141_d_n7, eq4_e1141_d_n8, eq4_e1141_d_n9, eq4_e1141_d_n10, eq4_e1141_d_n11, eq4_e1141_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1143;let eq4_node_derivative_indices: [usize; 11] = [0, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];let eq4_node_derivatives: [f64; 11] = [eq4_e1143_d_n0, eq4_e1143_d_n3, eq4_e1143_d_n4, eq4_e1143_d_n5, eq4_e1143_d_n6, eq4_e1143_d_n7, eq4_e1143_d_n8, eq4_e1143_d_n9, eq4_e1143_d_n10, eq4_e1143_d_n11, eq4_e1143_d_n12];let eq4_branch_derivative_indices: [usize; 0] = [];let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1158, eq6_e1158_d_n2, eq6_e1158_d_n3, eq6_e1158_d_n4, eq6_e1158_d_n5, eq6_e1158_d_n6, eq6_e1158_d_n7, eq6_e1158_d_n8, eq6_e1158_d_n9, eq6_e1158_d_n10, eq6_e1158_d_n11, eq6_e1158_d_n12,) = {
    if (l.fb79 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / l.f1f2a;let eq6_e1156: f64 = ((nv2 - nv8) * __rspice_inv_cse_1);let eq6_e1156_d_n2: f64 = (1.0 * __rspice_inv_cse_1);let eq6_e1156_d_n3: f64 = (-(((nv2 - nv8) * l.f1f3a) / (l.f1f2a * l.f1f2a)));let eq6_e1156_d_n4: f64 = (-(((nv2 - nv8) * l.f1f3b) / (l.f1f2a * l.f1f2a)));let eq6_e1156_d_n5: f64 = (-(((nv2 - nv8) * l.f1f3c) / (l.f1f2a * l.f1f2a)));let eq6_e1156_d_n6: f64 = (-(((nv2 - nv8) * l.f1f3d) / (l.f1f2a * l.f1f2a)));let eq6_e1156_d_n7: f64 = (-(((nv2 - nv8) * l.f1f3e) / (l.f1f2a * l.f1f2a)));let eq6_e1156_d_n8: f64 = (((-l.f1f2a) - ((nv2 - nv8) * l.f1f3f)) / (l.f1f2a * l.f1f2a));let eq6_e1156_d_n9: f64 = (-(((nv2 - nv8) * l.f1f40) / (l.f1f2a * l.f1f2a)));let eq6_e1156_d_n10: f64 = (-(((nv2 - nv8) * l.f1f37) / (l.f1f2a * l.f1f2a)));let eq6_e1156_d_n11: f64 = (-(((nv2 - nv8) * l.f1f38) / (l.f1f2a * l.f1f2a)));let eq6_e1156_d_n12: f64 = (-(((nv2 - nv8) * l.f1f39) / (l.f1f2a * l.f1f2a)));
        (eq6_e1156, eq6_e1156_d_n2, eq6_e1156_d_n3, eq6_e1156_d_n4, eq6_e1156_d_n5, eq6_e1156_d_n6, eq6_e1156_d_n7, eq6_e1156_d_n8, eq6_e1156_d_n9, eq6_e1156_d_n10, eq6_e1156_d_n11, eq6_e1156_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1158;let eq6_node_derivative_indices: [usize; 11] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];let eq6_node_derivatives: [f64; 11] = [eq6_e1158_d_n2, eq6_e1158_d_n3, eq6_e1158_d_n4, eq6_e1158_d_n5, eq6_e1158_d_n6, eq6_e1158_d_n7, eq6_e1158_d_n8, eq6_e1158_d_n9, eq6_e1158_d_n10, eq6_e1158_d_n11, eq6_e1158_d_n12];let eq6_branch_derivative_indices: [usize; 0] = [];let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivative_indices,
            &eq6_node_derivatives,
            &eq6_branch_derivative_indices,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq8_e1172,) = {
    if (l.fb79 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e1172;
        stamper.stamp_potential_const_local(
            0,
            eq8_value,
        );
        let (eq9_e1177,) = {
    if (l.fb79 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e1177;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );
        let (eq10_e1185, eq10_e1185_d_n3, eq10_e1185_d_n4, eq10_e1185_d_n5, eq10_e1185_d_n6, eq10_e1185_d_n7, eq10_e1185_d_n8, eq10_e1185_d_n9, eq10_e1185_d_n10, eq10_e1185_d_n11, eq10_e1185_d_n12,) = {
    if (l.fb7a != 0.0) {
        let eq10_e1182: f64 = (l.fd70 + l.fd40);let eq10_e1182_d_n3: f64 = (l.fd74 + l.fd44);let eq10_e1182_d_n4: f64 = (l.fd75 + l.fd45);let eq10_e1182_d_n5: f64 = (l.fd76 + l.fd46);let eq10_e1182_d_n6: f64 = (l.fd77 + l.fd47);let eq10_e1182_d_n7: f64 = (l.fd78 + l.fd48);let eq10_e1182_d_n8: f64 = (l.fd79 + l.fd49);let eq10_e1182_d_n9: f64 = (l.fd7a + l.fd4a);let eq10_e1182_d_n10: f64 = (l.fd71 + l.fd41);let eq10_e1182_d_n11: f64 = (l.fd72 + l.fd42);let eq10_e1182_d_n12: f64 = (l.fd73 + l.fd43);let eq10_e1183: f64 = (l.f411 * eq10_e1182);let eq10_e1183_d_n3: f64 = (l.f411 * eq10_e1182_d_n3);let eq10_e1183_d_n4: f64 = (l.f411 * eq10_e1182_d_n4);let eq10_e1183_d_n5: f64 = (l.f411 * eq10_e1182_d_n5);let eq10_e1183_d_n6: f64 = (l.f411 * eq10_e1182_d_n6);let eq10_e1183_d_n7: f64 = (l.f411 * eq10_e1182_d_n7);let eq10_e1183_d_n8: f64 = (l.f411 * eq10_e1182_d_n8);let eq10_e1183_d_n9: f64 = (l.f411 * eq10_e1182_d_n9);let eq10_e1183_d_n10: f64 = (l.f411 * eq10_e1182_d_n10);let eq10_e1183_d_n11: f64 = (l.f411 * eq10_e1182_d_n11);let eq10_e1183_d_n12: f64 = (l.f411 * eq10_e1182_d_n12);
        (eq10_e1183, eq10_e1183_d_n3, eq10_e1183_d_n4, eq10_e1183_d_n5, eq10_e1183_d_n6, eq10_e1183_d_n7, eq10_e1183_d_n8, eq10_e1183_d_n9, eq10_e1183_d_n10, eq10_e1183_d_n11, eq10_e1183_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1185;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq10_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq10_e1185_d_n3), multiplicity * (eq10_e1185_d_n4), multiplicity * (eq10_e1185_d_n5), multiplicity * (eq10_e1185_d_n6), multiplicity * (eq10_e1185_d_n7), multiplicity * (eq10_e1185_d_n8), multiplicity * (eq10_e1185_d_n9), multiplicity * (eq10_e1185_d_n10), multiplicity * (eq10_e1185_d_n11), multiplicity * (eq10_e1185_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq11_e1191, eq11_e1191_d_n3, eq11_e1191_d_n4, eq11_e1191_d_n5, eq11_e1191_d_n6, eq11_e1191_d_n7, eq11_e1191_d_n8, eq11_e1191_d_n9, eq11_e1191_d_n10, eq11_e1191_d_n11, eq11_e1191_d_n12,) = {
    if (l.fb7a != 0.0) {
        let eq11_e1189: f64 = (l.f411 * l.fe18);let eq11_e1189_d_n3: f64 = (l.f411 * l.fe1c);let eq11_e1189_d_n4: f64 = (l.f411 * l.fe1d);let eq11_e1189_d_n5: f64 = (l.f411 * l.fe1e);let eq11_e1189_d_n6: f64 = (l.f411 * l.fe1f);let eq11_e1189_d_n7: f64 = (l.f411 * l.fe20);let eq11_e1189_d_n8: f64 = (l.f411 * l.fe21);let eq11_e1189_d_n9: f64 = (l.f411 * l.fe22);let eq11_e1189_d_n10: f64 = (l.f411 * l.fe19);let eq11_e1189_d_n11: f64 = (l.f411 * l.fe1a);let eq11_e1189_d_n12: f64 = (l.f411 * l.fe1b);
        (eq11_e1189, eq11_e1189_d_n3, eq11_e1189_d_n4, eq11_e1189_d_n5, eq11_e1189_d_n6, eq11_e1189_d_n7, eq11_e1189_d_n8, eq11_e1189_d_n9, eq11_e1189_d_n10, eq11_e1189_d_n11, eq11_e1189_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1191;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq11_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq11_e1191_d_n3), multiplicity * (eq11_e1191_d_n4), multiplicity * (eq11_e1191_d_n5), multiplicity * (eq11_e1191_d_n6), multiplicity * (eq11_e1191_d_n7), multiplicity * (eq11_e1191_d_n8), multiplicity * (eq11_e1191_d_n9), multiplicity * (eq11_e1191_d_n10), multiplicity * (eq11_e1191_d_n11), multiplicity * (eq11_e1191_d_n12)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let (eq12_e1200, eq12_e1200_d_n3, eq12_e1200_d_n4, eq12_e1200_d_n5, eq12_e1200_d_n6, eq12_e1200_d_n7, eq12_e1200_d_n8, eq12_e1200_d_n9, eq12_e1200_d_n10, eq12_e1200_d_n11, eq12_e1200_d_n12,) = {
    if (l.fb7a == 0.0) {
        let eq12_e1197: f64 = (l.fd70 - l.fd40);let eq12_e1197_d_n3: f64 = (l.fd74 - l.fd44);let eq12_e1197_d_n4: f64 = (l.fd75 - l.fd45);let eq12_e1197_d_n5: f64 = (l.fd76 - l.fd46);let eq12_e1197_d_n6: f64 = (l.fd77 - l.fd47);let eq12_e1197_d_n7: f64 = (l.fd78 - l.fd48);let eq12_e1197_d_n8: f64 = (l.fd79 - l.fd49);let eq12_e1197_d_n9: f64 = (l.fd7a - l.fd4a);let eq12_e1197_d_n10: f64 = (l.fd71 - l.fd41);let eq12_e1197_d_n11: f64 = (l.fd72 - l.fd42);let eq12_e1197_d_n12: f64 = (l.fd73 - l.fd43);let eq12_e1198: f64 = (l.f411 * eq12_e1197);let eq12_e1198_d_n3: f64 = (l.f411 * eq12_e1197_d_n3);let eq12_e1198_d_n4: f64 = (l.f411 * eq12_e1197_d_n4);let eq12_e1198_d_n5: f64 = (l.f411 * eq12_e1197_d_n5);let eq12_e1198_d_n6: f64 = (l.f411 * eq12_e1197_d_n6);let eq12_e1198_d_n7: f64 = (l.f411 * eq12_e1197_d_n7);let eq12_e1198_d_n8: f64 = (l.f411 * eq12_e1197_d_n8);let eq12_e1198_d_n9: f64 = (l.f411 * eq12_e1197_d_n9);let eq12_e1198_d_n10: f64 = (l.f411 * eq12_e1197_d_n10);let eq12_e1198_d_n11: f64 = (l.f411 * eq12_e1197_d_n11);let eq12_e1198_d_n12: f64 = (l.f411 * eq12_e1197_d_n12);
        (eq12_e1198, eq12_e1198_d_n3, eq12_e1198_d_n4, eq12_e1198_d_n5, eq12_e1198_d_n6, eq12_e1198_d_n7, eq12_e1198_d_n8, eq12_e1198_d_n9, eq12_e1198_d_n10, eq12_e1198_d_n11, eq12_e1198_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1200;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq12_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq12_e1200_d_n3), multiplicity * (eq12_e1200_d_n4), multiplicity * (eq12_e1200_d_n5), multiplicity * (eq12_e1200_d_n6), multiplicity * (eq12_e1200_d_n7), multiplicity * (eq12_e1200_d_n8), multiplicity * (eq12_e1200_d_n9), multiplicity * (eq12_e1200_d_n10), multiplicity * (eq12_e1200_d_n11), multiplicity * (eq12_e1200_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq13_e1207, eq13_e1207_d_n3, eq13_e1207_d_n4, eq13_e1207_d_n5, eq13_e1207_d_n6, eq13_e1207_d_n7, eq13_e1207_d_n8, eq13_e1207_d_n9, eq13_e1207_d_n10, eq13_e1207_d_n11, eq13_e1207_d_n12,) = {
    if (l.fb7a == 0.0) {
        let eq13_e1205: f64 = (l.f411 * l.fe18);let eq13_e1205_d_n3: f64 = (l.f411 * l.fe1c);let eq13_e1205_d_n4: f64 = (l.f411 * l.fe1d);let eq13_e1205_d_n5: f64 = (l.f411 * l.fe1e);let eq13_e1205_d_n6: f64 = (l.f411 * l.fe1f);let eq13_e1205_d_n7: f64 = (l.f411 * l.fe20);let eq13_e1205_d_n8: f64 = (l.f411 * l.fe21);let eq13_e1205_d_n9: f64 = (l.f411 * l.fe22);let eq13_e1205_d_n10: f64 = (l.f411 * l.fe19);let eq13_e1205_d_n11: f64 = (l.f411 * l.fe1a);let eq13_e1205_d_n12: f64 = (l.f411 * l.fe1b);
        (eq13_e1205, eq13_e1205_d_n3, eq13_e1205_d_n4, eq13_e1205_d_n5, eq13_e1205_d_n6, eq13_e1205_d_n7, eq13_e1205_d_n8, eq13_e1205_d_n9, eq13_e1205_d_n10, eq13_e1205_d_n11, eq13_e1205_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1207;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq13_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq13_e1207_d_n3), multiplicity * (eq13_e1207_d_n4), multiplicity * (eq13_e1207_d_n5), multiplicity * (eq13_e1207_d_n6), multiplicity * (eq13_e1207_d_n7), multiplicity * (eq13_e1207_d_n8), multiplicity * (eq13_e1207_d_n9), multiplicity * (eq13_e1207_d_n10), multiplicity * (eq13_e1207_d_n11), multiplicity * (eq13_e1207_d_n12)],
            [],
            [],
            1.0,
        );let eq14_value: f64 = l.f261;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq14_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (l.f265), multiplicity * (l.f266), multiplicity * (l.f267), multiplicity * (l.f268), multiplicity * (l.f269), multiplicity * (l.f26a), multiplicity * (l.f26b), multiplicity * (l.f262), multiplicity * (l.f263), multiplicity * (l.f264)],
            [],
            [],
            1.0,
        );let eq15_value: f64 = l.f26c;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq15_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (l.f270), multiplicity * (l.f271), multiplicity * (l.f272), multiplicity * (l.f273), multiplicity * (l.f274), multiplicity * (l.f275), multiplicity * (l.f276), multiplicity * (l.f26d), multiplicity * (l.f26e), multiplicity * (l.f26f)],
            [],
            [],
            1.0,
        );let eq16_e1212: f64 = (l.f411 * l.fcdb);let eq16_e1212_d_n3: f64 = (l.f411 * l.fcdf);let eq16_e1212_d_n4: f64 = (l.f411 * l.fce0);let eq16_e1212_d_n5: f64 = (l.f411 * l.fce1);let eq16_e1212_d_n6: f64 = (l.f411 * l.fce2);let eq16_e1212_d_n7: f64 = (l.f411 * l.fce3);let eq16_e1212_d_n8: f64 = (l.f411 * l.fce4);let eq16_e1212_d_n9: f64 = (l.f411 * l.fce5);let eq16_e1212_d_n10: f64 = (l.f411 * l.fcdc);let eq16_e1212_d_n11: f64 = (l.f411 * l.fcdd);let eq16_e1212_d_n12: f64 = (l.f411 * l.fcde);let eq16_value: f64 = eq16_e1212;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            Some(7),
            multiplicity * (eq16_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq16_e1212_d_n3), multiplicity * (eq16_e1212_d_n4), multiplicity * (eq16_e1212_d_n5), multiplicity * (eq16_e1212_d_n6), multiplicity * (eq16_e1212_d_n7), multiplicity * (eq16_e1212_d_n8), multiplicity * (eq16_e1212_d_n9), multiplicity * (eq16_e1212_d_n10), multiplicity * (eq16_e1212_d_n11), multiplicity * (eq16_e1212_d_n12)],
            [],
            [],
            1.0,
        );let eq17_e1215: f64 = (l.f411 * l.fd29);let eq17_e1215_d_n3: f64 = (l.f411 * l.fd2d);let eq17_e1215_d_n4: f64 = (l.f411 * l.fd2e);let eq17_e1215_d_n5: f64 = (l.f411 * l.fd2f);let eq17_e1215_d_n6: f64 = (l.f411 * l.fd30);let eq17_e1215_d_n7: f64 = (l.f411 * l.fd31);let eq17_e1215_d_n8: f64 = (l.f411 * l.fd32);let eq17_e1215_d_n9: f64 = (l.f411 * l.fd33);let eq17_e1215_d_n10: f64 = (l.f411 * l.fd2a);let eq17_e1215_d_n11: f64 = (l.f411 * l.fd2b);let eq17_e1215_d_n12: f64 = (l.f411 * l.fd2c);let eq17_value: f64 = eq17_e1215;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(8),
            multiplicity * (eq17_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq17_e1215_d_n3), multiplicity * (eq17_e1215_d_n4), multiplicity * (eq17_e1215_d_n5), multiplicity * (eq17_e1215_d_n6), multiplicity * (eq17_e1215_d_n7), multiplicity * (eq17_e1215_d_n8), multiplicity * (eq17_e1215_d_n9), multiplicity * (eq17_e1215_d_n10), multiplicity * (eq17_e1215_d_n11), multiplicity * (eq17_e1215_d_n12)],
            [],
            [],
            1.0,
        );let eq18_e1218: f64 = (l.f256 + l.f23e);let eq18_e1218_d_n3: f64 = (l.f25a + l.f242);let eq18_e1218_d_n4: f64 = (l.f25b + l.f243);let eq18_e1218_d_n5: f64 = (l.f25c + l.f244);let eq18_e1218_d_n6: f64 = (l.f25d + l.f245);let eq18_e1218_d_n7: f64 = (l.f25e + l.f246);let eq18_e1218_d_n8: f64 = (l.f25f + l.f247);let eq18_e1218_d_n9: f64 = (l.f260 + l.f248);let eq18_e1218_d_n10: f64 = (l.f257 + l.f23f);let eq18_e1218_d_n11: f64 = (l.f258 + l.f240);let eq18_e1218_d_n12: f64 = (l.f259 + l.f241);let eq18_value: f64 = eq18_e1218;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq18_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq18_e1218_d_n3), multiplicity * (eq18_e1218_d_n4), multiplicity * (eq18_e1218_d_n5), multiplicity * (eq18_e1218_d_n6), multiplicity * (eq18_e1218_d_n7), multiplicity * (eq18_e1218_d_n8), multiplicity * (eq18_e1218_d_n9), multiplicity * (eq18_e1218_d_n10), multiplicity * (eq18_e1218_d_n11), multiplicity * (eq18_e1218_d_n12)],
            [],
            [],
            1.0,
        );let eq19_e1221: f64 = (l.f282 + l.f24b);let eq19_e1221_d_n3: f64 = (l.f286 + l.f24f);let eq19_e1221_d_n4: f64 = (l.f287 + l.f250);let eq19_e1221_d_n5: f64 = (l.f288 + l.f251);let eq19_e1221_d_n6: f64 = (l.f289 + l.f252);let eq19_e1221_d_n7: f64 = (l.f28a + l.f253);let eq19_e1221_d_n8: f64 = (l.f28b + l.f254);let eq19_e1221_d_n9: f64 = (l.f28c + l.f255);let eq19_e1221_d_n10: f64 = (l.f283 + l.f24c);let eq19_e1221_d_n11: f64 = (l.f284 + l.f24d);let eq19_e1221_d_n12: f64 = (l.f285 + l.f24e);let eq19_value: f64 = eq19_e1221;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq19_e1221_d_n3), multiplicity * (eq19_e1221_d_n4), multiplicity * (eq19_e1221_d_n5), multiplicity * (eq19_e1221_d_n6), multiplicity * (eq19_e1221_d_n7), multiplicity * (eq19_e1221_d_n8), multiplicity * (eq19_e1221_d_n9), multiplicity * (eq19_e1221_d_n10), multiplicity * (eq19_e1221_d_n11), multiplicity * (eq19_e1221_d_n12)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        l: &mut StampLocals,
    ) {
        let eq20_value: f64 = l.f231;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq20_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (l.f235), multiplicity * (l.f236), multiplicity * (l.f237), multiplicity * (l.f238), multiplicity * (l.f239), multiplicity * (l.f23a), multiplicity * (l.f23b), multiplicity * (l.f232), multiplicity * (l.f233), multiplicity * (l.f234)],
            [],
            [],
            1.0,
        );let eq21_value: f64 = l.f277;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(4),
            multiplicity * (eq21_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (l.f27b), multiplicity * (l.f27c), multiplicity * (l.f27d), multiplicity * (l.f27e), multiplicity * (l.f27f), multiplicity * (l.f280), multiplicity * (l.f281), multiplicity * (l.f278), multiplicity * (l.f279), multiplicity * (l.f27a)],
            [],
            [],
            1.0,
        );
        let (eq22_e1227,) = {
    if (l.fb7c != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e1227;
        stamper.stamp_potential_const_local(
            2,
            eq22_value,
        );
        let (eq23_e1234, eq23_e1234_d_n3, eq23_e1234_d_n4, eq23_e1234_d_n5, eq23_e1234_d_n6, eq23_e1234_d_n7, eq23_e1234_d_n8, eq23_e1234_d_n9, eq23_e1234_d_n10, eq23_e1234_d_n11, eq23_e1234_d_n12,) = {
    if (l.fb7c == 0.0) {
        let eq23_e1232: f64 = (l.f411 * l.fcf2);let eq23_e1232_d_n3: f64 = (l.f411 * l.fcf6);let eq23_e1232_d_n4: f64 = (l.f411 * l.fcf7);let eq23_e1232_d_n5: f64 = (l.f411 * l.fcf8);let eq23_e1232_d_n6: f64 = (l.f411 * l.fcf9);let eq23_e1232_d_n7: f64 = (l.f411 * l.fcfa);let eq23_e1232_d_n8: f64 = (l.f411 * l.fcfb);let eq23_e1232_d_n9: f64 = (l.f411 * l.fcfc);let eq23_e1232_d_n10: f64 = (l.f411 * l.fcf3);let eq23_e1232_d_n11: f64 = (l.f411 * l.fcf4);let eq23_e1232_d_n12: f64 = (l.f411 * l.fcf5);
        (eq23_e1232, eq23_e1232_d_n3, eq23_e1232_d_n4, eq23_e1232_d_n5, eq23_e1232_d_n6, eq23_e1232_d_n7, eq23_e1232_d_n8, eq23_e1232_d_n9, eq23_e1232_d_n10, eq23_e1232_d_n11, eq23_e1232_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1234;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq23_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq23_e1234_d_n3), multiplicity * (eq23_e1234_d_n4), multiplicity * (eq23_e1234_d_n5), multiplicity * (eq23_e1234_d_n6), multiplicity * (eq23_e1234_d_n7), multiplicity * (eq23_e1234_d_n8), multiplicity * (eq23_e1234_d_n9), multiplicity * (eq23_e1234_d_n10), multiplicity * (eq23_e1234_d_n11), multiplicity * (eq23_e1234_d_n12)],
            [],
            [],
            1.0,
        );let eq30_e1299: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, l.f35d);let eq30_value: f64 = eq30_e1299;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq30_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((l.f361 * ddt_scale)), multiplicity * ((l.f362 * ddt_scale)), multiplicity * ((l.f363 * ddt_scale)), multiplicity * ((l.f364 * ddt_scale)), multiplicity * ((l.f365 * ddt_scale)), multiplicity * ((l.f366 * ddt_scale)), multiplicity * ((l.f367 * ddt_scale)), multiplicity * ((l.f35e * ddt_scale)), multiplicity * ((l.f35f * ddt_scale)), multiplicity * ((l.f360 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq31_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, l.f375);let eq31_value: f64 = eq31_e1301;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq31_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((l.f379 * ddt_scale)), multiplicity * ((l.f37a * ddt_scale)), multiplicity * ((l.f37b * ddt_scale)), multiplicity * ((l.f37c * ddt_scale)), multiplicity * ((l.f37d * ddt_scale)), multiplicity * ((l.f37e * ddt_scale)), multiplicity * ((l.f37f * ddt_scale)), multiplicity * ((l.f376 * ddt_scale)), multiplicity * ((l.f377 * ddt_scale)), multiplicity * ((l.f378 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq32_e1304: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, l.f1e01);let eq32_e1305: f64 = (l.f411 * eq32_e1304);let eq32_e1305_d_n3: f64 = (l.f411 * (l.f1e05 * ddt_scale));let eq32_e1305_d_n4: f64 = (l.f411 * (l.f1e06 * ddt_scale));let eq32_e1305_d_n5: f64 = (l.f411 * (l.f1e07 * ddt_scale));let eq32_e1305_d_n6: f64 = (l.f411 * (l.f1e08 * ddt_scale));let eq32_e1305_d_n7: f64 = (l.f411 * (l.f1e09 * ddt_scale));let eq32_e1305_d_n8: f64 = (l.f411 * (l.f1e0a * ddt_scale));let eq32_e1305_d_n9: f64 = (l.f411 * (l.f1e0b * ddt_scale));let eq32_e1305_d_n10: f64 = (l.f411 * (l.f1e02 * ddt_scale));let eq32_e1305_d_n11: f64 = (l.f411 * (l.f1e03 * ddt_scale));let eq32_e1305_d_n12: f64 = (l.f411 * (l.f1e04 * ddt_scale));let eq32_value: f64 = eq32_e1305;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq32_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq32_e1305_d_n3), multiplicity * (eq32_e1305_d_n4), multiplicity * (eq32_e1305_d_n5), multiplicity * (eq32_e1305_d_n6), multiplicity * (eq32_e1305_d_n7), multiplicity * (eq32_e1305_d_n8), multiplicity * (eq32_e1305_d_n9), multiplicity * (eq32_e1305_d_n10), multiplicity * (eq32_e1305_d_n11), multiplicity * (eq32_e1305_d_n12)],
            [],
            [],
            1.0,
        );let eq33_e1308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, l.f1e85);let eq33_e1309: f64 = (l.f411 * eq33_e1308);let eq33_e1309_d_n3: f64 = (l.f411 * (l.f1ea1 * ddt_scale));let eq33_e1309_d_n4: f64 = (l.f411 * (l.f1ea2 * ddt_scale));let eq33_e1309_d_n5: f64 = (l.f411 * (l.f1ea3 * ddt_scale));let eq33_e1309_d_n6: f64 = (l.f411 * (l.f1ea4 * ddt_scale));let eq33_e1309_d_n7: f64 = (l.f411 * (l.f1ea5 * ddt_scale));let eq33_e1309_d_n8: f64 = (l.f411 * (l.f1ea6 * ddt_scale));let eq33_e1309_d_n9: f64 = (l.f411 * (l.f1ea7 * ddt_scale));let eq33_e1309_d_n10: f64 = (l.f411 * (l.f1e9e * ddt_scale));let eq33_e1309_d_n11: f64 = (l.f411 * (l.f1e9f * ddt_scale));let eq33_e1309_d_n12: f64 = (l.f411 * (l.f1ea0 * ddt_scale));let eq33_value: f64 = eq33_e1309;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(3),
            Some(5),
            multiplicity * (eq33_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq33_e1309_d_n3), multiplicity * (eq33_e1309_d_n4), multiplicity * (eq33_e1309_d_n5), multiplicity * (eq33_e1309_d_n6), multiplicity * (eq33_e1309_d_n7), multiplicity * (eq33_e1309_d_n8), multiplicity * (eq33_e1309_d_n9), multiplicity * (eq33_e1309_d_n10), multiplicity * (eq33_e1309_d_n11), multiplicity * (eq33_e1309_d_n12)],
            [],
            [],
            1.0,
        );let eq34_e1312: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, l.f1e3d);let eq34_e1313: f64 = (l.f411 * eq34_e1312);let eq34_e1313_d_n3: f64 = (l.f411 * (l.f1e41 * ddt_scale));let eq34_e1313_d_n4: f64 = (l.f411 * (l.f1e42 * ddt_scale));let eq34_e1313_d_n5: f64 = (l.f411 * (l.f1e43 * ddt_scale));let eq34_e1313_d_n6: f64 = (l.f411 * (l.f1e44 * ddt_scale));let eq34_e1313_d_n7: f64 = (l.f411 * (l.f1e45 * ddt_scale));let eq34_e1313_d_n8: f64 = (l.f411 * (l.f1e46 * ddt_scale));let eq34_e1313_d_n9: f64 = (l.f411 * (l.f1e47 * ddt_scale));let eq34_e1313_d_n10: f64 = (l.f411 * (l.f1e3e * ddt_scale));let eq34_e1313_d_n11: f64 = (l.f411 * (l.f1e3f * ddt_scale));let eq34_e1313_d_n12: f64 = (l.f411 * (l.f1e40 * ddt_scale));let eq34_value: f64 = eq34_e1313;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            Some(7),
            multiplicity * (eq34_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq34_e1313_d_n3), multiplicity * (eq34_e1313_d_n4), multiplicity * (eq34_e1313_d_n5), multiplicity * (eq34_e1313_d_n6), multiplicity * (eq34_e1313_d_n7), multiplicity * (eq34_e1313_d_n8), multiplicity * (eq34_e1313_d_n9), multiplicity * (eq34_e1313_d_n10), multiplicity * (eq34_e1313_d_n11), multiplicity * (eq34_e1313_d_n12)],
            [],
            [],
            1.0,
        );let eq35_e1316: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, l.f1e49);let eq35_e1317: f64 = (l.f411 * eq35_e1316);let eq35_e1317_d_n3: f64 = (l.f411 * (l.f1e4d * ddt_scale));let eq35_e1317_d_n4: f64 = (l.f411 * (l.f1e4e * ddt_scale));let eq35_e1317_d_n5: f64 = (l.f411 * (l.f1e4f * ddt_scale));let eq35_e1317_d_n6: f64 = (l.f411 * (l.f1e50 * ddt_scale));let eq35_e1317_d_n7: f64 = (l.f411 * (l.f1e51 * ddt_scale));let eq35_e1317_d_n8: f64 = (l.f411 * (l.f1e52 * ddt_scale));let eq35_e1317_d_n9: f64 = (l.f411 * (l.f1e53 * ddt_scale));let eq35_e1317_d_n10: f64 = (l.f411 * (l.f1e4a * ddt_scale));let eq35_e1317_d_n11: f64 = (l.f411 * (l.f1e4b * ddt_scale));let eq35_e1317_d_n12: f64 = (l.f411 * (l.f1e4c * ddt_scale));let eq35_value: f64 = eq35_e1317;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(8),
            multiplicity * (eq35_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq35_e1317_d_n3), multiplicity * (eq35_e1317_d_n4), multiplicity * (eq35_e1317_d_n5), multiplicity * (eq35_e1317_d_n6), multiplicity * (eq35_e1317_d_n7), multiplicity * (eq35_e1317_d_n8), multiplicity * (eq35_e1317_d_n9), multiplicity * (eq35_e1317_d_n10), multiplicity * (eq35_e1317_d_n11), multiplicity * (eq35_e1317_d_n12)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq36_e1324, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12,) = {
    if (l.fb7d != 0.0) {
        let eq36_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, l.f1e0d);let eq36_e1322: f64 = (l.f411 * eq36_e1321);let eq36_e1322_d_n3: f64 = (l.f411 * (l.f1e11 * ddt_scale));let eq36_e1322_d_n4: f64 = (l.f411 * (l.f1e12 * ddt_scale));let eq36_e1322_d_n5: f64 = (l.f411 * (l.f1e13 * ddt_scale));let eq36_e1322_d_n6: f64 = (l.f411 * (l.f1e14 * ddt_scale));let eq36_e1322_d_n7: f64 = (l.f411 * (l.f1e15 * ddt_scale));let eq36_e1322_d_n8: f64 = (l.f411 * (l.f1e16 * ddt_scale));let eq36_e1322_d_n9: f64 = (l.f411 * (l.f1e17 * ddt_scale));let eq36_e1322_d_n10: f64 = (l.f411 * (l.f1e0e * ddt_scale));let eq36_e1322_d_n11: f64 = (l.f411 * (l.f1e0f * ddt_scale));let eq36_e1322_d_n12: f64 = (l.f411 * (l.f1e10 * ddt_scale));
        (eq36_e1322, eq36_e1322_d_n3, eq36_e1322_d_n4, eq36_e1322_d_n5, eq36_e1322_d_n6, eq36_e1322_d_n7, eq36_e1322_d_n8, eq36_e1322_d_n9, eq36_e1322_d_n10, eq36_e1322_d_n11, eq36_e1322_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1324;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(7),
            multiplicity * (eq36_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq36_e1324_d_n3), multiplicity * (eq36_e1324_d_n4), multiplicity * (eq36_e1324_d_n5), multiplicity * (eq36_e1324_d_n6), multiplicity * (eq36_e1324_d_n7), multiplicity * (eq36_e1324_d_n8), multiplicity * (eq36_e1324_d_n9), multiplicity * (eq36_e1324_d_n10), multiplicity * (eq36_e1324_d_n11), multiplicity * (eq36_e1324_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq37_e1331, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12,) = {
    if (l.fb7d != 0.0) {
        let eq37_e1328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, l.f1e25);let eq37_e1329: f64 = (l.f411 * eq37_e1328);let eq37_e1329_d_n3: f64 = (l.f411 * (l.f1e29 * ddt_scale));let eq37_e1329_d_n4: f64 = (l.f411 * (l.f1e2a * ddt_scale));let eq37_e1329_d_n5: f64 = (l.f411 * (l.f1e2b * ddt_scale));let eq37_e1329_d_n6: f64 = (l.f411 * (l.f1e2c * ddt_scale));let eq37_e1329_d_n7: f64 = (l.f411 * (l.f1e2d * ddt_scale));let eq37_e1329_d_n8: f64 = (l.f411 * (l.f1e2e * ddt_scale));let eq37_e1329_d_n9: f64 = (l.f411 * (l.f1e2f * ddt_scale));let eq37_e1329_d_n10: f64 = (l.f411 * (l.f1e26 * ddt_scale));let eq37_e1329_d_n11: f64 = (l.f411 * (l.f1e27 * ddt_scale));let eq37_e1329_d_n12: f64 = (l.f411 * (l.f1e28 * ddt_scale));
        (eq37_e1329, eq37_e1329_d_n3, eq37_e1329_d_n4, eq37_e1329_d_n5, eq37_e1329_d_n6, eq37_e1329_d_n7, eq37_e1329_d_n8, eq37_e1329_d_n9, eq37_e1329_d_n10, eq37_e1329_d_n11, eq37_e1329_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e1331;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(8),
            multiplicity * (eq37_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq37_e1331_d_n3), multiplicity * (eq37_e1331_d_n4), multiplicity * (eq37_e1331_d_n5), multiplicity * (eq37_e1331_d_n6), multiplicity * (eq37_e1331_d_n7), multiplicity * (eq37_e1331_d_n8), multiplicity * (eq37_e1331_d_n9), multiplicity * (eq37_e1331_d_n10), multiplicity * (eq37_e1331_d_n11), multiplicity * (eq37_e1331_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq38_e1338, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12,) = {
    if (l.fb7d != 0.0) {
        let eq38_e1335: f64 = ((nv10 - nv3) * l.f12c2);let eq38_e1335_d_n3: f64 = ((-l.f12c2) + ((nv10 - nv3) * l.f12c6));let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * l.f12c7);let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * l.f12c8);let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * l.f12c9);let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * l.f12ca);let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * l.f12cb);let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * l.f12cc);let eq38_e1335_d_n10: f64 = (l.f12c2 + ((nv10 - nv3) * l.f12c3));let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * l.f12c4);let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * l.f12c5);let eq38_e1336: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq38_e1335);
        (eq38_e1336, (eq38_e1335_d_n3 * ddt_scale), (eq38_e1335_d_n4 * ddt_scale), (eq38_e1335_d_n5 * ddt_scale), (eq38_e1335_d_n6 * ddt_scale), (eq38_e1335_d_n7 * ddt_scale), (eq38_e1335_d_n8 * ddt_scale), (eq38_e1335_d_n9 * ddt_scale), (eq38_e1335_d_n10 * ddt_scale), (eq38_e1335_d_n11 * ddt_scale), (eq38_e1335_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1338;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(3),
            multiplicity * (eq38_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq38_e1338_d_n3), multiplicity * (eq38_e1338_d_n4), multiplicity * (eq38_e1338_d_n5), multiplicity * (eq38_e1338_d_n6), multiplicity * (eq38_e1338_d_n7), multiplicity * (eq38_e1338_d_n8), multiplicity * (eq38_e1338_d_n9), multiplicity * (eq38_e1338_d_n10), multiplicity * (eq38_e1338_d_n11), multiplicity * (eq38_e1338_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq39_e1346, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12,) = {
    if (l.fb7d == 0.0) {
        let eq39_e1343: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, l.f1e0d);let eq39_e1344: f64 = (l.f411 * eq39_e1343);let eq39_e1344_d_n3: f64 = (l.f411 * (l.f1e11 * ddt_scale));let eq39_e1344_d_n4: f64 = (l.f411 * (l.f1e12 * ddt_scale));let eq39_e1344_d_n5: f64 = (l.f411 * (l.f1e13 * ddt_scale));let eq39_e1344_d_n6: f64 = (l.f411 * (l.f1e14 * ddt_scale));let eq39_e1344_d_n7: f64 = (l.f411 * (l.f1e15 * ddt_scale));let eq39_e1344_d_n8: f64 = (l.f411 * (l.f1e16 * ddt_scale));let eq39_e1344_d_n9: f64 = (l.f411 * (l.f1e17 * ddt_scale));let eq39_e1344_d_n10: f64 = (l.f411 * (l.f1e0e * ddt_scale));let eq39_e1344_d_n11: f64 = (l.f411 * (l.f1e0f * ddt_scale));let eq39_e1344_d_n12: f64 = (l.f411 * (l.f1e10 * ddt_scale));
        (eq39_e1344, eq39_e1344_d_n3, eq39_e1344_d_n4, eq39_e1344_d_n5, eq39_e1344_d_n6, eq39_e1344_d_n7, eq39_e1344_d_n8, eq39_e1344_d_n9, eq39_e1344_d_n10, eq39_e1344_d_n11, eq39_e1344_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1346;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq39_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq39_e1346_d_n3), multiplicity * (eq39_e1346_d_n4), multiplicity * (eq39_e1346_d_n5), multiplicity * (eq39_e1346_d_n6), multiplicity * (eq39_e1346_d_n7), multiplicity * (eq39_e1346_d_n8), multiplicity * (eq39_e1346_d_n9), multiplicity * (eq39_e1346_d_n10), multiplicity * (eq39_e1346_d_n11), multiplicity * (eq39_e1346_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq40_e1354, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12,) = {
    if (l.fb7d == 0.0) {
        let eq40_e1351: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, l.f1e25);let eq40_e1352: f64 = (l.f411 * eq40_e1351);let eq40_e1352_d_n3: f64 = (l.f411 * (l.f1e29 * ddt_scale));let eq40_e1352_d_n4: f64 = (l.f411 * (l.f1e2a * ddt_scale));let eq40_e1352_d_n5: f64 = (l.f411 * (l.f1e2b * ddt_scale));let eq40_e1352_d_n6: f64 = (l.f411 * (l.f1e2c * ddt_scale));let eq40_e1352_d_n7: f64 = (l.f411 * (l.f1e2d * ddt_scale));let eq40_e1352_d_n8: f64 = (l.f411 * (l.f1e2e * ddt_scale));let eq40_e1352_d_n9: f64 = (l.f411 * (l.f1e2f * ddt_scale));let eq40_e1352_d_n10: f64 = (l.f411 * (l.f1e26 * ddt_scale));let eq40_e1352_d_n11: f64 = (l.f411 * (l.f1e27 * ddt_scale));let eq40_e1352_d_n12: f64 = (l.f411 * (l.f1e28 * ddt_scale));
        (eq40_e1352, eq40_e1352_d_n3, eq40_e1352_d_n4, eq40_e1352_d_n5, eq40_e1352_d_n6, eq40_e1352_d_n7, eq40_e1352_d_n8, eq40_e1352_d_n9, eq40_e1352_d_n10, eq40_e1352_d_n11, eq40_e1352_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1354;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq40_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq40_e1354_d_n3), multiplicity * (eq40_e1354_d_n4), multiplicity * (eq40_e1354_d_n5), multiplicity * (eq40_e1354_d_n6), multiplicity * (eq40_e1354_d_n7), multiplicity * (eq40_e1354_d_n8), multiplicity * (eq40_e1354_d_n9), multiplicity * (eq40_e1354_d_n10), multiplicity * (eq40_e1354_d_n11), multiplicity * (eq40_e1354_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq41_e1362, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12,) = {
    if (l.fb7d == 0.0) {
        let eq41_e1359: f64 = ((nv9 - nv3) * l.f12c2);let eq41_e1359_d_n3: f64 = ((-l.f12c2) + ((nv9 - nv3) * l.f12c6));let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * l.f12c7);let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * l.f12c8);let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * l.f12c9);let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * l.f12ca);let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * l.f12cb);let eq41_e1359_d_n9: f64 = (l.f12c2 + ((nv9 - nv3) * l.f12cc));let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * l.f12c3);let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * l.f12c4);let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * l.f12c5);let eq41_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq41_e1359);
        (eq41_e1360, (eq41_e1359_d_n3 * ddt_scale), (eq41_e1359_d_n4 * ddt_scale), (eq41_e1359_d_n5 * ddt_scale), (eq41_e1359_d_n6 * ddt_scale), (eq41_e1359_d_n7 * ddt_scale), (eq41_e1359_d_n8 * ddt_scale), (eq41_e1359_d_n9 * ddt_scale), (eq41_e1359_d_n10 * ddt_scale), (eq41_e1359_d_n11 * ddt_scale), (eq41_e1359_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1362;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(3),
            multiplicity * (eq41_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq41_e1362_d_n3), multiplicity * (eq41_e1362_d_n4), multiplicity * (eq41_e1362_d_n5), multiplicity * (eq41_e1362_d_n6), multiplicity * (eq41_e1362_d_n7), multiplicity * (eq41_e1362_d_n8), multiplicity * (eq41_e1362_d_n9), multiplicity * (eq41_e1362_d_n10), multiplicity * (eq41_e1362_d_n11), multiplicity * (eq41_e1362_d_n12)],
            [],
            [],
            1.0,
        );let eq42_e1364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, l.f351);let eq42_value: f64 = eq42_e1364;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq42_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((l.f355 * ddt_scale)), multiplicity * ((l.f356 * ddt_scale)), multiplicity * ((l.f357 * ddt_scale)), multiplicity * ((l.f358 * ddt_scale)), multiplicity * ((l.f359 * ddt_scale)), multiplicity * ((l.f35a * ddt_scale)), multiplicity * ((l.f35b * ddt_scale)), multiplicity * ((l.f352 * ddt_scale)), multiplicity * ((l.f353 * ddt_scale)), multiplicity * ((l.f354 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq43_e1366: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, l.f369);let eq43_value: f64 = eq43_e1366;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(3),
            multiplicity * (eq43_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((l.f36d * ddt_scale)), multiplicity * ((l.f36e * ddt_scale)), multiplicity * ((l.f36f * ddt_scale)), multiplicity * ((l.f370 * ddt_scale)), multiplicity * ((l.f371 * ddt_scale)), multiplicity * ((l.f372 * ddt_scale)), multiplicity * ((l.f373 * ddt_scale)), multiplicity * ((l.f36a * ddt_scale)), multiplicity * ((l.f36b * ddt_scale)), multiplicity * ((l.f36c * ddt_scale))],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv5 = ctx.node_voltage(nodes[5]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq44_e1370,) = {
    if (l.fb7f != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq44_value: f64 = eq44_e1370;
        stamper.stamp_potential_const_local(
            3,
            eq44_value,
        );
        let (eq45_e1377, eq45_e1377_d_n1, eq45_e1377_d_n3, eq45_e1377_d_n4, eq45_e1377_d_n5, eq45_e1377_d_n6, eq45_e1377_d_n7, eq45_e1377_d_n8, eq45_e1377_d_n9, eq45_e1377_d_n10, eq45_e1377_d_n11, eq45_e1377_d_n12,) = {
    if (l.fb7f == 0.0) {
        let eq45_e1375: f64 = ((nv1 - nv10) * l.f211);let eq45_e1375_d_n3: f64 = ((nv1 - nv10) * l.f215);let eq45_e1375_d_n4: f64 = ((nv1 - nv10) * l.f216);let eq45_e1375_d_n5: f64 = ((nv1 - nv10) * l.f217);let eq45_e1375_d_n6: f64 = ((nv1 - nv10) * l.f218);let eq45_e1375_d_n7: f64 = ((nv1 - nv10) * l.f219);let eq45_e1375_d_n8: f64 = ((nv1 - nv10) * l.f21a);let eq45_e1375_d_n9: f64 = ((nv1 - nv10) * l.f21b);let eq45_e1375_d_n10: f64 = ((-l.f211) + ((nv1 - nv10) * l.f212));let eq45_e1375_d_n11: f64 = ((nv1 - nv10) * l.f213);let eq45_e1375_d_n12: f64 = ((nv1 - nv10) * l.f214);
        (eq45_e1375, l.f211, eq45_e1375_d_n3, eq45_e1375_d_n4, eq45_e1375_d_n5, eq45_e1375_d_n6, eq45_e1375_d_n7, eq45_e1375_d_n8, eq45_e1375_d_n9, eq45_e1375_d_n10, eq45_e1375_d_n11, eq45_e1375_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e1377;let eq45_node_derivative_indices: [usize; 11] = [1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];let eq45_node_derivatives: [f64; 11] = [eq45_e1377_d_n1, eq45_e1377_d_n3, eq45_e1377_d_n4, eq45_e1377_d_n5, eq45_e1377_d_n6, eq45_e1377_d_n7, eq45_e1377_d_n8, eq45_e1377_d_n9, eq45_e1377_d_n10, eq45_e1377_d_n11, eq45_e1377_d_n12];let eq45_branch_derivative_indices: [usize; 0] = [];let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq45_value),
            &eq45_node_derivative_indices,
            &eq45_node_derivatives,
            &eq45_branch_derivative_indices,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq47_e1391,) = {
    if (l.fb80 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e1391;
        stamper.stamp_potential_const_local(
            4,
            eq47_value,
        );
        let (eq48_e1398, eq48_e1398_d_n3, eq48_e1398_d_n4, eq48_e1398_d_n5, eq48_e1398_d_n6, eq48_e1398_d_n7, eq48_e1398_d_n8, eq48_e1398_d_n9, eq48_e1398_d_n10, eq48_e1398_d_n11, eq48_e1398_d_n12,) = {
    if (l.fb80 == 0.0) {
        let eq48_e1396: f64 = ((nv10 - nv9) * l.f1dd);let eq48_e1396_d_n3: f64 = ((nv10 - nv9) * l.f1e1);let eq48_e1396_d_n4: f64 = ((nv10 - nv9) * l.f1e2);let eq48_e1396_d_n5: f64 = ((nv10 - nv9) * l.f1e3);let eq48_e1396_d_n6: f64 = ((nv10 - nv9) * l.f1e4);let eq48_e1396_d_n7: f64 = ((nv10 - nv9) * l.f1e5);let eq48_e1396_d_n8: f64 = ((nv10 - nv9) * l.f1e6);let eq48_e1396_d_n9: f64 = ((-l.f1dd) + ((nv10 - nv9) * l.f1e7));let eq48_e1396_d_n10: f64 = (l.f1dd + ((nv10 - nv9) * l.f1de));let eq48_e1396_d_n11: f64 = ((nv10 - nv9) * l.f1df);let eq48_e1396_d_n12: f64 = ((nv10 - nv9) * l.f1e0);
        (eq48_e1396, eq48_e1396_d_n3, eq48_e1396_d_n4, eq48_e1396_d_n5, eq48_e1396_d_n6, eq48_e1396_d_n7, eq48_e1396_d_n8, eq48_e1396_d_n9, eq48_e1396_d_n10, eq48_e1396_d_n11, eq48_e1396_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e1398;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(9),
            multiplicity * (eq48_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq48_e1398_d_n3), multiplicity * (eq48_e1398_d_n4), multiplicity * (eq48_e1398_d_n5), multiplicity * (eq48_e1398_d_n6), multiplicity * (eq48_e1398_d_n7), multiplicity * (eq48_e1398_d_n8), multiplicity * (eq48_e1398_d_n9), multiplicity * (eq48_e1398_d_n10), multiplicity * (eq48_e1398_d_n11), multiplicity * (eq48_e1398_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq50_e1420, eq50_e1420_d_n5, eq50_e1420_d_n12,) = {
    if (l.f386 != 0.0) {
        let eq50_e1418: f64 = ((nv5 - nv12) * l.f20f);
        (eq50_e1418, l.f20f, (-l.f20f),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1420;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(12),
            multiplicity * (eq50_value),
            5,
            multiplicity * (eq50_e1420_d_n5),
            12,
            multiplicity * (eq50_e1420_d_n12),
        );
        let (eq51_e1426, eq51_e1426_d_n5, eq51_e1426_d_n11,) = {
    if (l.f386 != 0.0) {
        let eq51_e1424: f64 = ((nv5 - nv11) * l.f210);
        (eq51_e1424, l.f210, (-l.f210),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1426;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(11),
            multiplicity * (eq51_value),
            5,
            multiplicity * (eq51_e1426_d_n5),
            11,
            multiplicity * (eq51_e1426_d_n11),
        );
        let (eq54_e1449,) = {
    if (l.f386 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1449;
        stamper.stamp_potential_const_local(
            5,
            eq54_value,
        );
        let (eq55_e1454,) = {
    if (l.f386 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e1454;
        stamper.stamp_potential_const_local(
            6,
            eq55_value,
        );
        let (eq56_e1458,) = {
    if (l.fb82 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e1458;
        stamper.stamp_potential_const_local(
            7,
            eq56_value,
        );
        let (eq57_e1469, eq57_e1469_d_n3, eq57_e1469_d_n4, eq57_e1469_d_n5, eq57_e1469_d_n6, eq57_e1469_d_n7, eq57_e1469_d_n8, eq57_e1469_d_n9, eq57_e1469_d_n10, eq57_e1469_d_n11, eq57_e1469_d_n12,) = {
    if (l.fb83 != 0.0) {
        let eq57_e1461: f64 = (-l.fd70);let eq57_e1463: f64 = (eq57_e1461 * l.f23fd);let eq57_e1463_d_n3: f64 = ((-l.fd74) * l.f23fd);let eq57_e1463_d_n4: f64 = ((-l.fd75) * l.f23fd);let eq57_e1463_d_n5: f64 = ((-l.fd76) * l.f23fd);let eq57_e1463_d_n6: f64 = ((-l.fd77) * l.f23fd);let eq57_e1463_d_n7: f64 = (((-l.fd78) * l.f23fd) + (eq57_e1461 * l.f23fe));let eq57_e1463_d_n8: f64 = (((-l.fd79) * l.f23fd) + (eq57_e1461 * l.f23ff));let eq57_e1463_d_n9: f64 = ((-l.fd7a) * l.f23fd);let eq57_e1463_d_n10: f64 = ((-l.fd71) * l.f23fd);let eq57_e1463_d_n11: f64 = ((-l.fd72) * l.f23fd);let eq57_e1463_d_n12: f64 = ((-l.fd73) * l.f23fd);let eq57_e1466: f64 = (l.f672 / l.f1a30);let eq57_e1466_d_n3: f64 = (-((l.f672 * l.f1a34) / (l.f1a30 * l.f1a30)));let eq57_e1466_d_n4: f64 = (-((l.f672 * l.f1a35) / (l.f1a30 * l.f1a30)));let eq57_e1466_d_n5: f64 = (-((l.f672 * l.f1a36) / (l.f1a30 * l.f1a30)));let eq57_e1466_d_n6: f64 = (((l.f673 * l.f1a30) - (l.f672 * l.f1a37)) / (l.f1a30 * l.f1a30));let eq57_e1466_d_n7: f64 = (-((l.f672 * l.f1a38) / (l.f1a30 * l.f1a30)));let eq57_e1466_d_n8: f64 = (-((l.f672 * l.f1a39) / (l.f1a30 * l.f1a30)));let eq57_e1466_d_n9: f64 = (-((l.f672 * l.f1a3a) / (l.f1a30 * l.f1a30)));let eq57_e1466_d_n10: f64 = (-((l.f672 * l.f1a31) / (l.f1a30 * l.f1a30)));let eq57_e1466_d_n11: f64 = (-((l.f672 * l.f1a32) / (l.f1a30 * l.f1a30)));let eq57_e1466_d_n12: f64 = (-((l.f672 * l.f1a33) / (l.f1a30 * l.f1a30)));let eq57_e1467: f64 = (eq57_e1463 + eq57_e1466);let eq57_e1467_d_n3: f64 = (eq57_e1463_d_n3 + eq57_e1466_d_n3);let eq57_e1467_d_n4: f64 = (eq57_e1463_d_n4 + eq57_e1466_d_n4);let eq57_e1467_d_n5: f64 = (eq57_e1463_d_n5 + eq57_e1466_d_n5);let eq57_e1467_d_n6: f64 = (eq57_e1463_d_n6 + eq57_e1466_d_n6);let eq57_e1467_d_n7: f64 = (eq57_e1463_d_n7 + eq57_e1466_d_n7);let eq57_e1467_d_n8: f64 = (eq57_e1463_d_n8 + eq57_e1466_d_n8);let eq57_e1467_d_n9: f64 = (eq57_e1463_d_n9 + eq57_e1466_d_n9);let eq57_e1467_d_n10: f64 = (eq57_e1463_d_n10 + eq57_e1466_d_n10);let eq57_e1467_d_n11: f64 = (eq57_e1463_d_n11 + eq57_e1466_d_n11);let eq57_e1467_d_n12: f64 = (eq57_e1463_d_n12 + eq57_e1466_d_n12);
        (eq57_e1467, eq57_e1467_d_n3, eq57_e1467_d_n4, eq57_e1467_d_n5, eq57_e1467_d_n6, eq57_e1467_d_n7, eq57_e1467_d_n8, eq57_e1467_d_n9, eq57_e1467_d_n10, eq57_e1467_d_n11, eq57_e1467_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1469;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq57_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq57_e1469_d_n3), multiplicity * (eq57_e1469_d_n4), multiplicity * (eq57_e1469_d_n5), multiplicity * (eq57_e1469_d_n6), multiplicity * (eq57_e1469_d_n7), multiplicity * (eq57_e1469_d_n8), multiplicity * (eq57_e1469_d_n9), multiplicity * (eq57_e1469_d_n10), multiplicity * (eq57_e1469_d_n11), multiplicity * (eq57_e1469_d_n12)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        l: &mut StampLocals,
    ) {
        let (eq58_e1476, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12,) = {
    if (l.fb83 != 0.0) {
        let eq58_e1473: f64 = (l.f672 * l.f133e);let eq58_e1473_d_n3: f64 = (l.f672 * l.f1342);let eq58_e1473_d_n4: f64 = (l.f672 * l.f1343);let eq58_e1473_d_n5: f64 = (l.f672 * l.f1344);let eq58_e1473_d_n6: f64 = ((l.f673 * l.f133e) + (l.f672 * l.f1345));let eq58_e1473_d_n7: f64 = (l.f672 * l.f1346);let eq58_e1473_d_n8: f64 = (l.f672 * l.f1347);let eq58_e1473_d_n9: f64 = (l.f672 * l.f1348);let eq58_e1473_d_n10: f64 = (l.f672 * l.f133f);let eq58_e1473_d_n11: f64 = (l.f672 * l.f1340);let eq58_e1473_d_n12: f64 = (l.f672 * l.f1341);let eq58_e1474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq58_e1473);
        (eq58_e1474, (eq58_e1473_d_n3 * ddt_scale), (eq58_e1473_d_n4 * ddt_scale), (eq58_e1473_d_n5 * ddt_scale), (eq58_e1473_d_n6 * ddt_scale), (eq58_e1473_d_n7 * ddt_scale), (eq58_e1473_d_n8 * ddt_scale), (eq58_e1473_d_n9 * ddt_scale), (eq58_e1473_d_n10 * ddt_scale), (eq58_e1473_d_n11 * ddt_scale), (eq58_e1473_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1476;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq58_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq58_e1476_d_n3), multiplicity * (eq58_e1476_d_n4), multiplicity * (eq58_e1476_d_n5), multiplicity * (eq58_e1476_d_n6), multiplicity * (eq58_e1476_d_n7), multiplicity * (eq58_e1476_d_n8), multiplicity * (eq58_e1476_d_n9), multiplicity * (eq58_e1476_d_n10), multiplicity * (eq58_e1476_d_n11), multiplicity * (eq58_e1476_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq59_e1481,) = {
    if (l.fb83 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e1481;
        stamper.stamp_potential_const_local(
            8,
            eq59_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let eq30_e1299_q: f64 = l.f35d;let eq30_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, l.f361, l.f362, l.f363, l.f364, l.f365, l.f366, l.f367, l.f35e, l.f35f, l.f360];let eq30_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(5),
            &eq30_reactive_node_derivatives,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );let eq31_e1301_q: f64 = l.f375;let eq31_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, l.f379, l.f37a, l.f37b, l.f37c, l.f37d, l.f37e, l.f37f, l.f376, l.f377, l.f378];let eq31_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(5),
            &eq31_reactive_node_derivatives,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );let eq32_e1304_q: f64 = l.f1e01;let eq32_e1305: f64 = (l.f411 * l.f1e01);let eq32_e1305_d_n3: f64 = (l.f411 * l.f1e05);let eq32_e1305_d_n4: f64 = (l.f411 * l.f1e06);let eq32_e1305_d_n5: f64 = (l.f411 * l.f1e07);let eq32_e1305_d_n6: f64 = (l.f411 * l.f1e08);let eq32_e1305_d_n7: f64 = (l.f411 * l.f1e09);let eq32_e1305_d_n8: f64 = (l.f411 * l.f1e0a);let eq32_e1305_d_n9: f64 = (l.f411 * l.f1e0b);let eq32_e1305_d_n10: f64 = (l.f411 * l.f1e02);let eq32_e1305_d_n11: f64 = (l.f411 * l.f1e03);let eq32_e1305_d_n12: f64 = (l.f411 * l.f1e04);let eq32_e1305_q: f64 = (l.f411 * eq32_e1304_q);let eq32_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq32_e1305_d_n3, eq32_e1305_d_n4, eq32_e1305_d_n5, eq32_e1305_d_n6, eq32_e1305_d_n7, eq32_e1305_d_n8, eq32_e1305_d_n9, eq32_e1305_d_n10, eq32_e1305_d_n11, eq32_e1305_d_n12];let eq32_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(5),
            &eq32_reactive_node_derivatives,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );let eq33_e1308_q: f64 = l.f1e85;let eq33_e1309: f64 = (l.f411 * l.f1e85);let eq33_e1309_d_n3: f64 = (l.f411 * l.f1ea1);let eq33_e1309_d_n4: f64 = (l.f411 * l.f1ea2);let eq33_e1309_d_n5: f64 = (l.f411 * l.f1ea3);let eq33_e1309_d_n6: f64 = (l.f411 * l.f1ea4);let eq33_e1309_d_n7: f64 = (l.f411 * l.f1ea5);let eq33_e1309_d_n8: f64 = (l.f411 * l.f1ea6);let eq33_e1309_d_n9: f64 = (l.f411 * l.f1ea7);let eq33_e1309_d_n10: f64 = (l.f411 * l.f1e9e);let eq33_e1309_d_n11: f64 = (l.f411 * l.f1e9f);let eq33_e1309_d_n12: f64 = (l.f411 * l.f1ea0);let eq33_e1309_q: f64 = (l.f411 * eq33_e1308_q);let eq33_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq33_e1309_d_n3, eq33_e1309_d_n4, eq33_e1309_d_n5, eq33_e1309_d_n6, eq33_e1309_d_n7, eq33_e1309_d_n8, eq33_e1309_d_n9, eq33_e1309_d_n10, eq33_e1309_d_n11, eq33_e1309_d_n12];let eq33_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(5),
            &eq33_reactive_node_derivatives,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );let eq34_e1312_q: f64 = l.f1e3d;let eq34_e1313: f64 = (l.f411 * l.f1e3d);let eq34_e1313_d_n3: f64 = (l.f411 * l.f1e41);let eq34_e1313_d_n4: f64 = (l.f411 * l.f1e42);let eq34_e1313_d_n5: f64 = (l.f411 * l.f1e43);let eq34_e1313_d_n6: f64 = (l.f411 * l.f1e44);let eq34_e1313_d_n7: f64 = (l.f411 * l.f1e45);let eq34_e1313_d_n8: f64 = (l.f411 * l.f1e46);let eq34_e1313_d_n9: f64 = (l.f411 * l.f1e47);let eq34_e1313_d_n10: f64 = (l.f411 * l.f1e3e);let eq34_e1313_d_n11: f64 = (l.f411 * l.f1e3f);let eq34_e1313_d_n12: f64 = (l.f411 * l.f1e40);let eq34_e1313_q: f64 = (l.f411 * eq34_e1312_q);let eq34_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq34_e1313_d_n3, eq34_e1313_d_n4, eq34_e1313_d_n5, eq34_e1313_d_n6, eq34_e1313_d_n7, eq34_e1313_d_n8, eq34_e1313_d_n9, eq34_e1313_d_n10, eq34_e1313_d_n11, eq34_e1313_d_n12];let eq34_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            Some(7),
            &eq34_reactive_node_derivatives,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );let eq35_e1316_q: f64 = l.f1e49;let eq35_e1317: f64 = (l.f411 * l.f1e49);let eq35_e1317_d_n3: f64 = (l.f411 * l.f1e4d);let eq35_e1317_d_n4: f64 = (l.f411 * l.f1e4e);let eq35_e1317_d_n5: f64 = (l.f411 * l.f1e4f);let eq35_e1317_d_n6: f64 = (l.f411 * l.f1e50);let eq35_e1317_d_n7: f64 = (l.f411 * l.f1e51);let eq35_e1317_d_n8: f64 = (l.f411 * l.f1e52);let eq35_e1317_d_n9: f64 = (l.f411 * l.f1e53);let eq35_e1317_d_n10: f64 = (l.f411 * l.f1e4a);let eq35_e1317_d_n11: f64 = (l.f411 * l.f1e4b);let eq35_e1317_d_n12: f64 = (l.f411 * l.f1e4c);let eq35_e1317_q: f64 = (l.f411 * eq35_e1316_q);let eq35_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq35_e1317_d_n3, eq35_e1317_d_n4, eq35_e1317_d_n5, eq35_e1317_d_n6, eq35_e1317_d_n7, eq35_e1317_d_n8, eq35_e1317_d_n9, eq35_e1317_d_n10, eq35_e1317_d_n11, eq35_e1317_d_n12];let eq35_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(8),
            &eq35_reactive_node_derivatives,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq36_e1324, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12, eq36_e1324_q,) = {
    if (l.fb7d != 0.0) {
        let eq36_e1321_q: f64 = l.f1e0d;let eq36_e1322: f64 = (l.f411 * l.f1e0d);let eq36_e1322_d_n3: f64 = (l.f411 * l.f1e11);let eq36_e1322_d_n4: f64 = (l.f411 * l.f1e12);let eq36_e1322_d_n5: f64 = (l.f411 * l.f1e13);let eq36_e1322_d_n6: f64 = (l.f411 * l.f1e14);let eq36_e1322_d_n7: f64 = (l.f411 * l.f1e15);let eq36_e1322_d_n8: f64 = (l.f411 * l.f1e16);let eq36_e1322_d_n9: f64 = (l.f411 * l.f1e17);let eq36_e1322_d_n10: f64 = (l.f411 * l.f1e0e);let eq36_e1322_d_n11: f64 = (l.f411 * l.f1e0f);let eq36_e1322_d_n12: f64 = (l.f411 * l.f1e10);let eq36_e1322_q: f64 = (l.f411 * eq36_e1321_q);
        (eq36_e1322, eq36_e1322_d_n3, eq36_e1322_d_n4, eq36_e1322_d_n5, eq36_e1322_d_n6, eq36_e1322_d_n7, eq36_e1322_d_n8, eq36_e1322_d_n9, eq36_e1322_d_n10, eq36_e1322_d_n11, eq36_e1322_d_n12, eq36_e1322_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12];let eq36_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(7),
            &eq36_reactive_node_derivatives,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1331, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12, eq37_e1331_q,) = {
    if (l.fb7d != 0.0) {
        let eq37_e1328_q: f64 = l.f1e25;let eq37_e1329: f64 = (l.f411 * l.f1e25);let eq37_e1329_d_n3: f64 = (l.f411 * l.f1e29);let eq37_e1329_d_n4: f64 = (l.f411 * l.f1e2a);let eq37_e1329_d_n5: f64 = (l.f411 * l.f1e2b);let eq37_e1329_d_n6: f64 = (l.f411 * l.f1e2c);let eq37_e1329_d_n7: f64 = (l.f411 * l.f1e2d);let eq37_e1329_d_n8: f64 = (l.f411 * l.f1e2e);let eq37_e1329_d_n9: f64 = (l.f411 * l.f1e2f);let eq37_e1329_d_n10: f64 = (l.f411 * l.f1e26);let eq37_e1329_d_n11: f64 = (l.f411 * l.f1e27);let eq37_e1329_d_n12: f64 = (l.f411 * l.f1e28);let eq37_e1329_q: f64 = (l.f411 * eq37_e1328_q);
        (eq37_e1329, eq37_e1329_d_n3, eq37_e1329_d_n4, eq37_e1329_d_n5, eq37_e1329_d_n6, eq37_e1329_d_n7, eq37_e1329_d_n8, eq37_e1329_d_n9, eq37_e1329_d_n10, eq37_e1329_d_n11, eq37_e1329_d_n12, eq37_e1329_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12];let eq37_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(8),
            &eq37_reactive_node_derivatives,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e1338, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12, eq38_e1338_q,) = {
    if (l.fb7d != 0.0) {
        let eq38_e1335: f64 = ((nv10 - nv3) * l.f12c2);let eq38_e1335_d_n3: f64 = ((-l.f12c2) + ((nv10 - nv3) * l.f12c6));let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * l.f12c7);let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * l.f12c8);let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * l.f12c9);let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * l.f12ca);let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * l.f12cb);let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * l.f12cc);let eq38_e1335_d_n10: f64 = (l.f12c2 + ((nv10 - nv3) * l.f12c3));let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * l.f12c4);let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * l.f12c5);let eq38_e1336_q: f64 = eq38_e1335;
        (eq38_e1335, eq38_e1335_d_n3, eq38_e1335_d_n4, eq38_e1335_d_n5, eq38_e1335_d_n6, eq38_e1335_d_n7, eq38_e1335_d_n8, eq38_e1335_d_n9, eq38_e1335_d_n10, eq38_e1335_d_n11, eq38_e1335_d_n12, eq38_e1336_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12];let eq38_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(3),
            &eq38_reactive_node_derivatives,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1346, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12, eq39_e1346_q,) = {
    if (l.fb7d == 0.0) {
        let eq39_e1343_q: f64 = l.f1e0d;let eq39_e1344: f64 = (l.f411 * l.f1e0d);let eq39_e1344_d_n3: f64 = (l.f411 * l.f1e11);let eq39_e1344_d_n4: f64 = (l.f411 * l.f1e12);let eq39_e1344_d_n5: f64 = (l.f411 * l.f1e13);let eq39_e1344_d_n6: f64 = (l.f411 * l.f1e14);let eq39_e1344_d_n7: f64 = (l.f411 * l.f1e15);let eq39_e1344_d_n8: f64 = (l.f411 * l.f1e16);let eq39_e1344_d_n9: f64 = (l.f411 * l.f1e17);let eq39_e1344_d_n10: f64 = (l.f411 * l.f1e0e);let eq39_e1344_d_n11: f64 = (l.f411 * l.f1e0f);let eq39_e1344_d_n12: f64 = (l.f411 * l.f1e10);let eq39_e1344_q: f64 = (l.f411 * eq39_e1343_q);
        (eq39_e1344, eq39_e1344_d_n3, eq39_e1344_d_n4, eq39_e1344_d_n5, eq39_e1344_d_n6, eq39_e1344_d_n7, eq39_e1344_d_n8, eq39_e1344_d_n9, eq39_e1344_d_n10, eq39_e1344_d_n11, eq39_e1344_d_n12, eq39_e1344_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12];let eq39_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(7),
            &eq39_reactive_node_derivatives,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1354, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12, eq40_e1354_q,) = {
    if (l.fb7d == 0.0) {
        let eq40_e1351_q: f64 = l.f1e25;let eq40_e1352: f64 = (l.f411 * l.f1e25);let eq40_e1352_d_n3: f64 = (l.f411 * l.f1e29);let eq40_e1352_d_n4: f64 = (l.f411 * l.f1e2a);let eq40_e1352_d_n5: f64 = (l.f411 * l.f1e2b);let eq40_e1352_d_n6: f64 = (l.f411 * l.f1e2c);let eq40_e1352_d_n7: f64 = (l.f411 * l.f1e2d);let eq40_e1352_d_n8: f64 = (l.f411 * l.f1e2e);let eq40_e1352_d_n9: f64 = (l.f411 * l.f1e2f);let eq40_e1352_d_n10: f64 = (l.f411 * l.f1e26);let eq40_e1352_d_n11: f64 = (l.f411 * l.f1e27);let eq40_e1352_d_n12: f64 = (l.f411 * l.f1e28);let eq40_e1352_q: f64 = (l.f411 * eq40_e1351_q);
        (eq40_e1352, eq40_e1352_d_n3, eq40_e1352_d_n4, eq40_e1352_d_n5, eq40_e1352_d_n6, eq40_e1352_d_n7, eq40_e1352_d_n8, eq40_e1352_d_n9, eq40_e1352_d_n10, eq40_e1352_d_n11, eq40_e1352_d_n12, eq40_e1352_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12];let eq40_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(8),
            &eq40_reactive_node_derivatives,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq41_e1362, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12, eq41_e1362_q,) = {
    if (l.fb7d == 0.0) {
        let eq41_e1359: f64 = ((nv9 - nv3) * l.f12c2);let eq41_e1359_d_n3: f64 = ((-l.f12c2) + ((nv9 - nv3) * l.f12c6));let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * l.f12c7);let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * l.f12c8);let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * l.f12c9);let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * l.f12ca);let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * l.f12cb);let eq41_e1359_d_n9: f64 = (l.f12c2 + ((nv9 - nv3) * l.f12cc));let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * l.f12c3);let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * l.f12c4);let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * l.f12c5);let eq41_e1360_q: f64 = eq41_e1359;
        (eq41_e1359, eq41_e1359_d_n3, eq41_e1359_d_n4, eq41_e1359_d_n5, eq41_e1359_d_n6, eq41_e1359_d_n7, eq41_e1359_d_n8, eq41_e1359_d_n9, eq41_e1359_d_n10, eq41_e1359_d_n11, eq41_e1359_d_n12, eq41_e1360_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12];let eq41_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(3),
            &eq41_reactive_node_derivatives,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );let eq42_e1364_q: f64 = l.f351;let eq42_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, l.f355, l.f356, l.f357, l.f358, l.f359, l.f35a, l.f35b, l.f352, l.f353, l.f354];let eq42_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(3),
            &eq42_reactive_node_derivatives,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );let eq43_e1366_q: f64 = l.f369;let eq43_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, l.f36d, l.f36e, l.f36f, l.f370, l.f371, l.f372, l.f373, l.f36a, l.f36b, l.f36c];let eq43_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(3),
            &eq43_reactive_node_derivatives,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq58_e1476, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12, eq58_e1476_q,) = {
    if (l.fb83 != 0.0) {
        let eq58_e1473: f64 = (l.f672 * l.f133e);let eq58_e1473_d_n3: f64 = (l.f672 * l.f1342);let eq58_e1473_d_n4: f64 = (l.f672 * l.f1343);let eq58_e1473_d_n5: f64 = (l.f672 * l.f1344);let eq58_e1473_d_n6: f64 = ((l.f673 * l.f133e) + (l.f672 * l.f1345));let eq58_e1473_d_n7: f64 = (l.f672 * l.f1346);let eq58_e1473_d_n8: f64 = (l.f672 * l.f1347);let eq58_e1473_d_n9: f64 = (l.f672 * l.f1348);let eq58_e1473_d_n10: f64 = (l.f672 * l.f133f);let eq58_e1473_d_n11: f64 = (l.f672 * l.f1340);let eq58_e1473_d_n12: f64 = (l.f672 * l.f1341);let eq58_e1474_q: f64 = eq58_e1473;
        (eq58_e1473, eq58_e1473_d_n3, eq58_e1473_d_n4, eq58_e1473_d_n5, eq58_e1473_d_n6, eq58_e1473_d_n7, eq58_e1473_d_n8, eq58_e1473_d_n9, eq58_e1473_d_n10, eq58_e1473_d_n11, eq58_e1473_d_n12, eq58_e1474_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12];let eq58_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            None,
            &eq58_reactive_node_derivatives,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
