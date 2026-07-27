#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t0,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t0);
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2291] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });s.b[2292] = (6.0 == 1.0);s.store_scalar(2292, if s.b[2292] { 1.0 } else { 0.0 });
        let (t1,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && s.b[2292]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1);s.b[2293] = (6.0 == 2.0);s.store_scalar(2293, if s.b[2293] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && (!s.b[2292])) && s.b[2293]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2);s.b[2294] = (6.0 == 4.0);s.store_scalar(2294, if s.b[2294] { 1.0 } else { 0.0 });
        let (t3,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && (!s.b[2292])) && (!s.b[2293])) && s.b[2294]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3);s.b[2295] = (6.0 == 8.0);s.store_scalar(2295, if s.b[2295] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && (!s.b[2292])) && (!s.b[2293])) && (!s.b[2294])) && s.b[2295]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);
        let (t5,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t5);let mut t9: usize = 0;
        while {
            let t8: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) {s.store_sqrt(726, 726);}
            let (t7,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) {
        let t6: f64 = (s.v[719] + 1.0);
        (t6,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t7);
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && (!s.b[2291])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[403]);s.store_div_scaled_product_indices(334, 725, 726, p[403], 770, 1.0);s.store_sub_from_scalar(336, p[403], 780);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) {
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2290])) {s.store_sub(336, 109, 2087);s.store_scalar(334, 1.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(2120, 209, -1.0, 338);s.store_sqrt_offset_ad(782, A::mul_scaled_lhs(A::add(s.ad_value(2119), s.ad_value(2118)), 1.0, A::add(s.ad_value(2119), s.ad_value(2118))), ((4.0 * (1e-12 * 1e-6)) * (1e-12 * 1e-6)));s.store_scaled_offset_ad(335, A::div_scaled_inputs2(s.ad_value(2119), -1.0, s.ad_value(2118), -1.0, s.ad_value(782), 1.0), 1.0, 0.5);s.store_add_scaled_inputs3_indices(2121, 2119, (-0.5), 2118, (-0.5), 782, 0.5);}
        s.b[2296] = (s.v[2121] < 0.0);s.store_scalar(2296, if s.b[2296] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2296]) {s.store_scalar(2121, 0.0);s.store_scalar(335, 0.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_neg(2121, 2121);s.store_mul3_affine_lhs(248, 154, 2121, (-1.0 / (2.0)), 0.0, 94);s.store_neg(238, 2120);s.copy_ad(170, 162);s.copy_ad(790, 349);}
        s.b[2297] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2297, if s.b[2297] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) {s.store_scalar(169, 0.0);s.copy_ad(168, 91);}
        s.b[2298] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2298, if s.b[2298] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (ta,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta);
        let (tb,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2299] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2299, if s.b[2299] { 1.0 } else { 0.0 });s.b[2300] = (2.0 == 1.0);s.store_scalar(2300, if s.b[2300] { 1.0 } else { 0.0 });
        let (tc,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && s.b[2300]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc);s.b[2301] = (2.0 == 2.0);s.store_scalar(2301, if s.b[2301] { 1.0 } else { 0.0 });
        let (td,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && (!s.b[2300])) && s.b[2301]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td);s.b[2302] = (2.0 == 4.0);s.store_scalar(2302, if s.b[2302] { 1.0 } else { 0.0 });
        let (te,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && (!s.b[2300])) && (!s.b[2301])) && s.b[2302]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te);s.b[2303] = (2.0 == 8.0);s.store_scalar(2303, if s.b[2303] { 1.0 } else { 0.0 });
        let (tf,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && (!s.b[2300])) && (!s.b[2301])) && (!s.b[2302])) && s.b[2303]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf);
        let (t10,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t10);let mut t14: usize = 0;
        while {
            let t13: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t13 != 0.0
        } {
            t14 += 1;
            if t14 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t14, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) {s.store_sqrt(726, 726);}
            let (t12,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) {
        let t11: f64 = (s.v[719] + 1.0);
        (t11,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t12);
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && (!s.b[2299])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && (!s.b[2298])) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && (!s.b[2298])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) {s.store_sub(342, 91, 2117);}
        s.b[2304] = ((s.v[342] < (0.2 + ((-s.v[2117]) + 0.8))) && (((-s.v[2117]) + 0.8) >= 0.0));s.store_scalar(2304, if s.b[2304] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {s.store_sub_offset_lhs_mixed_ai(781, A::sub_from_scalar(0.8, s.ad_value(2117)), 0.2, 342);s.store_square(722, 781);s.store_square_ad(723, A::sub_from_scalar(0.8, s.ad_value(2117)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t15,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t15);
        let (t16,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t16);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2305] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2305, if s.b[2305] { 1.0 } else { 0.0 });s.b[2306] = (1.0 == 1.0);s.store_scalar(2306, if s.b[2306] { 1.0 } else { 0.0 });
        let (t17,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && s.b[2306]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t17);s.b[2307] = (1.0 == 2.0);s.store_scalar(2307, if s.b[2307] { 1.0 } else { 0.0 });
        let (t18,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && (!s.b[2306])) && s.b[2307]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t18);s.b[2308] = (1.0 == 4.0);s.store_scalar(2308, if s.b[2308] { 1.0 } else { 0.0 });
        let (t19,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && (!s.b[2306])) && (!s.b[2307])) && s.b[2308]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t19);s.b[2309] = (1.0 == 8.0);s.store_scalar(2309, if s.b[2309] { 1.0 } else { 0.0 });
        let (t1a,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && (!s.b[2306])) && (!s.b[2307])) && (!s.b[2308])) && s.b[2309]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1a);
        let (t1b,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t1b);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t1f: usize = 0;
        while {
            let t1e: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;
            if t1f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) {s.store_sqrt(726, 726);}
            let (t1d,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) {
        let t1c: f64 = (s.v[719] + 1.0);
        (t1c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1d);
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && (!s.b[2305])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul_mixed_ai(780, A::mul_sub_from_scalar_rhs(s.ad_value(781), 0.8, s.ad_value(2117)), 726);s.store_div_scaled_product_mixed_aii(334, A::mul_sub_from_scalar_lhs(0.8, s.ad_value(2117), s.ad_value(725)), 726, 1.0, 770, 1.0);s.store_sub_offset_lhs_mixed_ai(342, A::sub_from_scalar(0.8, s.ad_value(2117)), 0.2, 780);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && (!s.b[2304])) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && (!s.b[2304])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) {s.store_mul(343, 2136, 342);s.store_sqrt(171, 343);s.store_div_from_scalar(334, 1.0, 171);s.store_mul(335, 238, 334);s.store_scale(336, 335, s.v[509]);s.store_scale(337, 334, s.v[509]);s.store_add_scaled_product_indices(339, 336, 1.0, 508, 2133, 1.0);s.store_div_from_scalar(335, 1.0, 339);s.store_scale(338, 335, 1.034943e-10);s.store_scalar(335, (1.0 - s.v[507]));s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 109, s.v[507], 335, 91, 1.0);}
        s.b[2310] = ((s.v[168] > (((s.v[109] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2310, if s.b[2310] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 109, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t20,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t20);
        let (t21,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t21);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2311] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2311, if s.b[2311] { 1.0 } else { 0.0 });s.b[2312] = (2.0 == 1.0);s.store_scalar(2312, if s.b[2312] { 1.0 } else { 0.0 });
        let (t22,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && s.b[2312]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t22);s.b[2313] = (2.0 == 2.0);s.store_scalar(2313, if s.b[2313] { 1.0 } else { 0.0 });
        let (t23,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && (!s.b[2312])) && s.b[2313]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t23);s.b[2314] = (2.0 == 4.0);s.store_scalar(2314, if s.b[2314] { 1.0 } else { 0.0 });
        let (t24,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && (!s.b[2312])) && (!s.b[2313])) && s.b[2314]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t24);s.b[2315] = (2.0 == 8.0);s.store_scalar(2315, if s.b[2315] { 1.0 } else { 0.0 });
        let (t25,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && (!s.b[2312])) && (!s.b[2313])) && (!s.b[2314])) && s.b[2315]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t25);
        let (t26,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t26);let mut t2a: usize = 0;
        while {
            let t29: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t29 != 0.0
        } {
            t2a += 1;
            if t2a > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2a, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) {s.store_sqrt(726, 726);}
            let (t28,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) {
        let t27: f64 = (s.v[719] + 1.0);
        (t27,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t28);
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && (!s.b[2311])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 109, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && (!s.b[2310])) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && (!s.b[2310])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) {s.store_sub(340, 168, 91);s.store_mul(337, 154, 238);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) {s.store_div_from_scalar(335, 1.0, 337);s.store_mul(339, 248, 335);s.store_scale(344, 2133, 9662367879.197212);s.store_scalar(335, 100000.0);s.store_div_from_scalar(336, 1.0, 162);s.store_mul_mixed_ai(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);s.store_mul(341, 345, 338);s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));s.store_mul3_lhs(342, 345, 338, 338);s.store_sqrt_square_add(343, 341, 342);s.store_scaled_sub(169, 343, 341, 0.5);s.copy_ad(335, 169);s.store_mul(169, 208, 335);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_scale(169, 169, s.v[619]);s.store_sub(170, 170, 169);s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p[262]), (-((p[262]) as f64).sqrt()));s.store_offset_mul(338, 980, 334, 1.0);s.store_mul(339, 336, 238);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 251, p[160]);
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_indices(340, 251, 624);
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 238, 343);s.store_scalar(336, s.v[474]);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(338), s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2316] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2316, if s.b[2316] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2316]) {s.copy_ad(336, 335);}
        s.b[2317] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2317, if s.b[2317] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2316])) && s.b[2317]) {s.store_square(336, 335);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2316])) && (!s.b[2317])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[178]);
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_offset(338, 336, 1.0);}
        s.b[2318] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2318, if s.b[2318] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2318]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2319] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2319, if s.b[2319] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2318])) && s.b[2319]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2318])) && (!s.b[2319])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 338, ((-1.0) / p[178]));
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_mul(253, 254, 339);s.copy_ad(984, 253);s.copy_ad(2116, 255);s.store_scalar(2324, 0.0);s.store_scalar(2155, 0.0);s.store_scalar(990, 0.0);s.store_scalar(2147, 0.0);s.store_scalar(2322, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_add_scaled_inputs3_offset_indices(2144, 1440, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));}
        s.b[2326] = (0.0 == 0.0);s.store_scalar(2326, if s.b[2326] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2326]) {s.store_offset(2145, 2144, (-p[393]));}
        s.b[2327] = (0.0 == 1.0);s.store_scalar(2327, if s.b[2327] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2326])) && s.b[2327]) {s.store_offset(2145, 1440, (((-s.v[160])) + ((-p[393]))));}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2326])) && (!s.b[2327])) {s.store_offset(2145, 85, (-p[393]));}
        s.b[2328] = (((s.v[2148]) as f64).abs() <= 0.0);s.store_scalar(2328, if s.b[2328] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2328]) {s.store_scalar(2153, 0.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.copy_ad(983, 87);s.store_scale(2170, 2117, p[399]);s.store_scalar(2325, ((s.v[160] + p[393]) - 3.0));}
        s.b[2329] = (1.0 == 1.0);s.store_scalar(2329, if s.b[2329] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2329]) {s.store_scale(2170, 2117, p[399]);s.store_offset(983, 2170, (-1.0));s.copy_ad(2324, 2325);s.copy_ad(2146, 2325);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2329])) {s.store_offset_scaled(2170, 2117, p[399], (-0.1));s.copy_ad(983, 87);s.copy_ad(2324, 2145);s.copy_ad(2146, 2145);}
        let (t2b,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t2b);
        let (t2c,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t2c);let mut t33: usize = 0;
        while {
            let t32: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t32 != 0.0
        } {
            t33 += 1;
            if t33 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t33, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2330] = (s.v[983] >= 0.0);s.store_scalar(2330, if s.b[2330] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2330]) {s.store_mul_scaled_sqrt_ad_rhs(2322, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2125, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2322, 1.0);}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2330])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2170)));s.store_exp_mul(338, 154, 2170);s.store_mul_sqrt_mixed_ia(2322, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2322, 1.0);s.store_mul_add_mixed_iaa(2125, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            let (t2e,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] != 0.0)) {
        let t2d: f64 = (150.0 + 1.0);
        (t2d,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t2e);
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2100, 2322, 1.0, 185, 2324, 983, 1.0);s.store_sub(2101, 2125, 185);s.store_div_scaled_inputs_indices(2112, 2100, -1.0, 2101, 1.0);}
            s.b[2331] = (((s.v[2112]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2331, if s.b[2331] { 1.0 } else { 0.0 });
            let (t2f,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && s.b[2331]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t2f);s.b[2332] = (s.v[2112] > 0.1);s.store_scalar(2332, if s.b[2332] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2331])) && s.b[2332]) {s.store_scalar(2112, 0.1);}
            s.b[2333] = (s.v[2112] < (-0.1));s.store_scalar(2333, if s.b[2333] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2331])) && (!s.b[2332])) && s.b[2333]) {s.store_scalar(2112, (-0.1));}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 2112);}
            let (t31,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
        let t30: f64 = (s.v[97] + 1.0);
        (t30,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t31);
        }
        s.b[2335] = (1.0 == 1.0);s.store_scalar(2335, if s.b[2335] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2335]) {s.copy_ad(2171, 983);}
        s.b[2336] = ((s.v[983] < (s.v[2171] + 0.2)) && (0.2 >= 0.0));s.store_scalar(2336, if s.b[2336] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {s.store_sub_offset_lhs(781, 2171, 0.2, 983);s.store_square(722, 781);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t34,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t34);
        let (t35,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t35);
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2337] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2337, if s.b[2337] { 1.0 } else { 0.0 });s.b[2338] = (2.0 == 1.0);s.store_scalar(2338, if s.b[2338] { 1.0 } else { 0.0 });
        let (t36,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && s.b[2338]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t36);s.b[2339] = (2.0 == 2.0);s.store_scalar(2339, if s.b[2339] { 1.0 } else { 0.0 });
        let (t37,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && (!s.b[2338])) && s.b[2339]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t37);s.b[2340] = (2.0 == 4.0);s.store_scalar(2340, if s.b[2340] { 1.0 } else { 0.0 });
        let (t38,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && (!s.b[2338])) && (!s.b[2339])) && s.b[2340]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t38);s.b[2341] = (2.0 == 8.0);s.store_scalar(2341, if s.b[2341] { 1.0 } else { 0.0 });
        let (t39,) = {
    if (((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && (!s.b[2338])) && (!s.b[2339])) && (!s.b[2340])) && s.b[2341]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t39);
        let (t3a,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t3a);let mut t3e: usize = 0;
        while {
            let t3d: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t3d != 0.0
        } {
            t3e += 1;
            if t3e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) {s.store_sqrt(726, 726);}
            let (t3c,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) {
        let t3b: f64 = (s.v[719] + 1.0);
        (t3b,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t3c);
        }
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && (!s.b[2337])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.2);s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);s.store_sub_offset_lhs(983, 2171, 0.2, 780);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && (!s.b[2336])) {
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && (!s.b[2336])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.copy_ad(2153, 983);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_scalar(2142, (if (1e-6 >= p[407]) { 1e-6 } else { p[407] }));}
        s.b[2342] = ((s.v[2153] > (-s.v[2142])) && (s.v[2142] >= 0.0));s.store_scalar(2342, if s.b[2342] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {s.store_add(781, 2153, 2142);s.store_square(722, 781);s.store_square(723, 2142);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t3f,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t3f);
        let (t40,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t40);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (t41,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t41);let mut t45: usize = 0;
        while {
            let t44: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && (s.v[719] < s.v[2143])) { 1.0 } else { 0.0 };
            t44 != 0.0
        } {
            t45 += 1;
            if t45 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t45, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (t43,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
        let t42: f64 = (s.v[719] + 1.0);
        (t42,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t43);
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2343] = ((((s.v[2143] == 1.0) || (s.v[2143] == 2.0)) || (s.v[2143] == 4.0)) || (s.v[2143] == 8.0));s.store_scalar(2343, if s.b[2343] { 1.0 } else { 0.0 });s.b[2344] = (s.v[2143] == 1.0);s.store_scalar(2344, if s.b[2344] { 1.0 } else { 0.0 });
        let (t46,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && s.b[2344]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t46);s.b[2345] = (s.v[2143] == 2.0);s.store_scalar(2345, if s.b[2345] { 1.0 } else { 0.0 });
        let (t47,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && (!s.b[2344])) && s.b[2345]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t47);s.b[2346] = (s.v[2143] == 4.0);s.store_scalar(2346, if s.b[2346] { 1.0 } else { 0.0 });
        let (t48,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && (!s.b[2344])) && (!s.b[2345])) && s.b[2346]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t48);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        s.b[2347] = (s.v[2143] == 8.0);s.store_scalar(2347, if s.b[2347] { 1.0 } else { 0.0 });
        let (t49,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && (!s.b[2344])) && (!s.b[2345])) && (!s.b[2346])) && s.b[2347]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t49);
        let (t4a,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t4a);let mut t4e: usize = 0;
        while {
            let t4d: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4d != 0.0
        } {
            t4e += 1;
            if t4e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t4e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) {s.store_sqrt(726, 726);}
            let (t4c,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) {
        let t4b: f64 = (s.v[719] + 1.0);
        (t4b,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t4c);
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && (!s.b[2343])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2143), 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 2142, 726);s.store_div_scaled_product3_indices(334, 2142, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(983, 2142, -1.0, 780, 1.0);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2342])) {s.copy_ad(983, 2153);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_neg(983, 983);s.store_mul3_affine_lhs(2320, 2133, 2148, (0.5 * 9662367879.197212), 0.0, 2148);s.store_mul_sqrt_mixed_ia(334, 2152, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2320)));s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);}
        s.b[2348] = (((s.v[334]) as f64).abs() > 0.0001);s.store_scalar(2348, if s.b[2348] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2348]) {s.store_div_ln_lhs(2321, 335, 2320);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2348])) {s.store_mul3_ad_middle(2321, A::square(s.ad_value(2152)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_mul(332, 2321, 983);}
        s.b[2349] = (s.v[332] > 500.0);s.store_scalar(2349, if s.b[2349] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2349]) {s.store_sub(2165, 983, 2320);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) {s.store_exp_mul_scaled_lhs_indices(334, 2321, -1.0, 2320);}
        s.b[2350] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2350, if s.b[2350] { 1.0 } else { 0.0 });s.b[2351] = (s.v[332] >= 500.0);s.store_scalar(2351, if s.b[2351] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && s.b[2351]) {s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(337, 1.403592217853e217);}
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && (!s.b[2351])) {s.copy_ad(781, 332);s.store_scalar(335, 1.0);}
        let mut t50: usize = 0;
        while {
            let t4f: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && (!s.b[2351])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            t4f != 0.0
        } {
            t50 += 1;
            if t50 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t50, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && (!s.b[2351])) {s.store_scale(335, 335, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && (!s.b[2351])) {s.store_mul_exp_rhs(335, 335, 781);s.copy_ad(337, 335);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) {s.store_mul(335, 335, 334);s.store_sub(336, 335, 334);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && (!s.b[2350])) {s.store_mul_scale_offset_indices(335, 334, 332, 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);}
        s.b[2352] = (((s.v[336]) as f64).abs() > 1e-8);s.store_scalar(2352, if s.b[2352] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2352]) {s.store_div_ln_offset_lhs(2165, 336, 1.0, 2321);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && (!s.b[2352])) {s.store_div(2165, 336, 2321);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_sub(336, 983, 2165);}
        s.b[2353] = (0.0 == 0.0);s.store_scalar(2353, if s.b[2353] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2353]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2147, A::sqrt(A::mul_scaled_lhs(s.ad_value(2136), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2147, 2136, 336);
            }
        }
        s.b[2354] = (s.v[336] < 0.0);s.store_scalar(2354, if s.b[2354] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2353])) && s.b[2354]) {s.store_mul(337, 154, 336);s.store_neg_ad(2147, A::sqrt(A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2353])) && (!s.b[2354])) {s.store_mul_scale_offset_indices(337, 336, 154, -1.0, 0.0);s.store_sqrt_ad(2147, A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_sub(990, 2148, 2147);}
        s.b[2355] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));s.store_scalar(2355, if s.b[2355] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {s.store_sub_from_scalar(781, 1e-16, 990);s.store_square(722, 781);s.store_scalar(723, (1e-16 * 1e-16));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t51,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t51);
        let (t52,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t52);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2356] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2356, if s.b[2356] { 1.0 } else { 0.0 });s.b[2357] = (2.0 == 1.0);s.store_scalar(2357, if s.b[2357] { 1.0 } else { 0.0 });
        let (t53,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && s.b[2357]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t53);s.b[2358] = (2.0 == 2.0);s.store_scalar(2358, if s.b[2358] { 1.0 } else { 0.0 });
        let (t54,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && s.b[2358]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t54);s.b[2359] = (2.0 == 4.0);s.store_scalar(2359, if s.b[2359] { 1.0 } else { 0.0 });
        let (t55,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && (!s.b[2358])) && s.b[2359]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t55);s.b[2360] = (2.0 == 8.0);s.store_scalar(2360, if s.b[2360] { 1.0 } else { 0.0 });
        let (t56,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && (!s.b[2358])) && (!s.b[2359])) && s.b[2360]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t56);
        let (t57,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t57);let mut t5b: usize = 0;
        while {
            let t5a: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t5a != 0.0
        } {
            t5b += 1;
            if t5b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t5b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) {s.store_sqrt(726, 726);}
            let (t59,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) {
        let t58: f64 = (s.v[719] + 1.0);
        (t58,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t59);
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && (!s.b[2356])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-16);s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);s.store_sub_from_scalar(990, 1e-16, 780);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2355])) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2355])) {s.store_scalar(334, 1.0);}
        s.b[2361] = (1.0 == 1.0);s.store_scalar(2361, if s.b[2361] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2361]) {s.copy_ad(2155, 990);}
        s.b[2362] = (2.0 == 1.0);s.store_scalar(2362, if s.b[2362] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2362]) {s.store_scale(2170, 2117, p[399]);s.store_offset(983, 2170, (-1.0));s.copy_ad(2324, 2325);s.copy_ad(2146, 2325);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2362])) {s.store_offset_scaled(2170, 2117, p[399], (-0.1));s.copy_ad(983, 87);s.copy_ad(2324, 2145);s.copy_ad(2146, 2145);}
        let (t5c,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t5c);
        let (t5d,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t5d);let mut t64: usize = 0;
        while {
            let t63: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t63 != 0.0
        } {
            t64 += 1;
            if t64 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t64, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2363] = (s.v[983] >= 0.0);s.store_scalar(2363, if s.b[2363] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2363]) {s.store_mul_scaled_sqrt_ad_rhs(2322, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2125, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2322, 1.0);}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2363])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2170)));s.store_exp_mul(338, 154, 2170);s.store_mul_sqrt_mixed_ia(2322, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2322, 1.0);s.store_mul_add_mixed_iaa(2125, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            let (t5f,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] != 0.0)) {
        let t5e: f64 = (150.0 + 1.0);
        (t5e,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t5f);
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2100, 2322, 1.0, 185, 2324, 983, 1.0);s.store_sub(2101, 2125, 185);s.store_div_scaled_inputs_indices(2112, 2100, -1.0, 2101, 1.0);}
            s.b[2364] = (((s.v[2112]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2364, if s.b[2364] { 1.0 } else { 0.0 });
            let (t60,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && s.b[2364]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t60);s.b[2365] = (s.v[2112] > 0.1);s.store_scalar(2365, if s.b[2365] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2364])) && s.b[2365]) {s.store_scalar(2112, 0.1);}
            s.b[2366] = (s.v[2112] < (-0.1));s.store_scalar(2366, if s.b[2366] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2364])) && (!s.b[2365])) && s.b[2366]) {s.store_scalar(2112, (-0.1));}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 2112);}
            let (t62,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
        let t61: f64 = (s.v[97] + 1.0);
        (t61,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t62);
        }
        s.b[2368] = (2.0 == 1.0);s.store_scalar(2368, if s.b[2368] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2368]) {s.copy_ad(2171, 983);}
        s.b[2369] = ((s.v[983] < (s.v[2171] + 0.2)) && (0.2 >= 0.0));s.store_scalar(2369, if s.b[2369] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {s.store_sub_offset_lhs(781, 2171, 0.2, 983);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t65,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t65);
        let (t66,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t66);
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2370] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2370, if s.b[2370] { 1.0 } else { 0.0 });s.b[2371] = (2.0 == 1.0);s.store_scalar(2371, if s.b[2371] { 1.0 } else { 0.0 });
        let (t67,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && s.b[2371]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t67);s.b[2372] = (2.0 == 2.0);s.store_scalar(2372, if s.b[2372] { 1.0 } else { 0.0 });
        let (t68,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && s.b[2372]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t68);s.b[2373] = (2.0 == 4.0);s.store_scalar(2373, if s.b[2373] { 1.0 } else { 0.0 });
        let (t69,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && (!s.b[2372])) && s.b[2373]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t69);s.b[2374] = (2.0 == 8.0);s.store_scalar(2374, if s.b[2374] { 1.0 } else { 0.0 });
        let (t6a,) = {
    if (((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && (!s.b[2372])) && (!s.b[2373])) && s.b[2374]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6a);
        let (t6b,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6b);let mut t6f: usize = 0;
        while {
            let t6e: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6e != 0.0
        } {
            t6f += 1;
            if t6f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t6f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) {s.store_sqrt(726, 726);}
            let (t6d,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) {
        let t6c: f64 = (s.v[719] + 1.0);
        (t6c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t6d);
        }
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && (!s.b[2370])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.2);s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);s.store_sub_offset_lhs(983, 2171, 0.2, 780);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && (!s.b[2369])) {
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && (!s.b[2369])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.copy_ad(2153, 983);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_scalar(2142, (if (1e-6 >= p[407]) { 1e-6 } else { p[407] }));}
        s.b[2375] = ((s.v[2153] > (-s.v[2142])) && (s.v[2142] >= 0.0));s.store_scalar(2375, if s.b[2375] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {s.store_add(781, 2153, 2142);s.store_square(722, 781);s.store_square(723, 2142);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t70,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t70);
        let (t71,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t71);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (t72,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t72);let mut t76: usize = 0;
        while {
            let t75: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && (s.v[719] < s.v[2143])) { 1.0 } else { 0.0 };
            t75 != 0.0
        } {
            t76 += 1;
            if t76 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t76, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (t74,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
        let t73: f64 = (s.v[719] + 1.0);
        (t73,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t74);
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2376] = ((((s.v[2143] == 1.0) || (s.v[2143] == 2.0)) || (s.v[2143] == 4.0)) || (s.v[2143] == 8.0));s.store_scalar(2376, if s.b[2376] { 1.0 } else { 0.0 });s.b[2377] = (s.v[2143] == 1.0);s.store_scalar(2377, if s.b[2377] { 1.0 } else { 0.0 });
        let (t77,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && s.b[2377]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t77);s.b[2378] = (s.v[2143] == 2.0);s.store_scalar(2378, if s.b[2378] { 1.0 } else { 0.0 });
        let (t78,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && s.b[2378]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t78);s.b[2379] = (s.v[2143] == 4.0);s.store_scalar(2379, if s.b[2379] { 1.0 } else { 0.0 });
        let (t79,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && (!s.b[2378])) && s.b[2379]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t79);s.b[2380] = (s.v[2143] == 8.0);s.store_scalar(2380, if s.b[2380] { 1.0 } else { 0.0 });
        let (t7a,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && (!s.b[2378])) && (!s.b[2379])) && s.b[2380]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7a);
        let (t7b,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t7b);let mut t7f: usize = 0;
        while {
            let t7e: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t7e != 0.0
        } {
            t7f += 1;
            if t7f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t7f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) {s.store_sqrt(726, 726);}
            let (t7d,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) {
        let t7c: f64 = (s.v[719] + 1.0);
        (t7c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t7d);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && (!s.b[2376])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2143), 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 2142, 726);s.store_div_scaled_product3_indices(334, 2142, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(983, 2142, -1.0, 780, 1.0);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2375])) {s.copy_ad(983, 2153);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_neg(983, 983);s.store_mul3_affine_lhs(2320, 2133, 2148, (0.5 * 9662367879.197212), 0.0, 2148);s.store_mul_sqrt_mixed_ia(334, 2152, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2320)));s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);}
        s.b[2381] = (((s.v[334]) as f64).abs() > 0.0001);s.store_scalar(2381, if s.b[2381] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2381]) {s.store_div_ln_lhs(2321, 335, 2320);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2381])) {s.store_mul3_ad_middle(2321, A::square(s.ad_value(2152)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_mul(332, 2321, 983);}
        s.b[2382] = (s.v[332] > 500.0);s.store_scalar(2382, if s.b[2382] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2382]) {s.store_sub(2165, 983, 2320);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) {s.store_exp_mul_scaled_lhs_indices(334, 2321, -1.0, 2320);}
        s.b[2383] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2383, if s.b[2383] { 1.0 } else { 0.0 });s.b[2384] = (s.v[332] >= 500.0);s.store_scalar(2384, if s.b[2384] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && s.b[2384]) {s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(337, 1.403592217853e217);}
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) {s.copy_ad(781, 332);s.store_scalar(335, 1.0);}
        let mut t81: usize = 0;
        while {
            let t80: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            t80 != 0.0
        } {
            t81 += 1;
            if t81 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t81, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) {s.store_scale(335, 335, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) {s.store_mul_exp_rhs(335, 335, 781);s.copy_ad(337, 335);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) {s.store_mul(335, 335, 334);s.store_sub(336, 335, 334);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && (!s.b[2383])) {s.store_mul_scale_offset_indices(335, 334, 332, 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);}
        s.b[2385] = (((s.v[336]) as f64).abs() > 1e-8);s.store_scalar(2385, if s.b[2385] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2385]) {s.store_div_ln_offset_lhs(2165, 336, 1.0, 2321);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && (!s.b[2385])) {s.store_div(2165, 336, 2321);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_sub(336, 983, 2165);}
        s.b[2386] = (0.0 == 0.0);s.store_scalar(2386, if s.b[2386] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2386]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2147, A::sqrt(A::mul_scaled_lhs(s.ad_value(2136), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2147, 2136, 336);
            }
        }
        s.b[2387] = (s.v[336] < 0.0);s.store_scalar(2387, if s.b[2387] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2386])) && s.b[2387]) {s.store_mul(337, 154, 336);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_107(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2386])) && s.b[2387]) {s.store_neg_ad(2147, A::sqrt(A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2386])) && (!s.b[2387])) {s.store_mul_scale_offset_indices(337, 336, 154, -1.0, 0.0);s.store_sqrt_ad(2147, A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_sub(990, 2148, 2147);}
        s.b[2388] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));s.store_scalar(2388, if s.b[2388] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {s.store_sub_from_scalar(781, 1e-16, 990);s.store_square(722, 781);s.store_scalar(723, (1e-16 * 1e-16));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t82,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t82);
        let (t83,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t83);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2389] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2389, if s.b[2389] { 1.0 } else { 0.0 });s.b[2390] = (2.0 == 1.0);s.store_scalar(2390, if s.b[2390] { 1.0 } else { 0.0 });
        let (t84,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && s.b[2390]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t84);s.b[2391] = (2.0 == 2.0);s.store_scalar(2391, if s.b[2391] { 1.0 } else { 0.0 });
        let (t85,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && s.b[2391]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t85);s.b[2392] = (2.0 == 4.0);s.store_scalar(2392, if s.b[2392] { 1.0 } else { 0.0 });
        let (t86,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && (!s.b[2391])) && s.b[2392]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t86);s.b[2393] = (2.0 == 8.0);s.store_scalar(2393, if s.b[2393] { 1.0 } else { 0.0 });
        let (t87,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && (!s.b[2391])) && (!s.b[2392])) && s.b[2393]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t87);
        let (t88,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t88);let mut t8c: usize = 0;
        while {
            let t8b: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8b != 0.0
        } {
            t8c += 1;
            if t8c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t8c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) {s.store_sqrt(726, 726);}
            let (t8a,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) {
        let t89: f64 = (s.v[719] + 1.0);
        (t89,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8a);
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && (!s.b[2389])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-16);s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);s.store_sub_from_scalar(990, 1e-16, 780);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2388])) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2388])) {s.store_scalar(334, 1.0);}
        s.b[2394] = (2.0 == 1.0);s.store_scalar(2394, if s.b[2394] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2394]) {s.copy_ad(2155, 990);}
        s.b[2395] = (0.0 == 0.0);s.store_scalar(2395, if s.b[2395] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) {s.copy_ad(989, 349);s.store_scaled_add(344, 2117, 155, p[396]);s.store_offset_mul_ad(338, s.ad_value(2135), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 2135, 1.0);}
        s.b[2396] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2396, if s.b[2396] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_108(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t8d,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t8d);
        let (t8e,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8e);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2397] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2397, if s.b[2397] { 1.0 } else { 0.0 });s.b[2398] = (2.0 == 1.0);s.store_scalar(2398, if s.b[2398] { 1.0 } else { 0.0 });
        let (t8f,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && s.b[2398]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8f);s.b[2399] = (2.0 == 2.0);s.store_scalar(2399, if s.b[2399] { 1.0 } else { 0.0 });
        let (t90,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && s.b[2399]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t90);s.b[2400] = (2.0 == 4.0);s.store_scalar(2400, if s.b[2400] { 1.0 } else { 0.0 });
        let (t91,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && (!s.b[2399])) && s.b[2400]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t91);s.b[2401] = (2.0 == 8.0);s.store_scalar(2401, if s.b[2401] { 1.0 } else { 0.0 });
        let (t92,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && (!s.b[2399])) && (!s.b[2400])) && s.b[2401]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t92);
        let (t93,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t93);let mut t97: usize = 0;
        while {
            let t96: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t96 != 0.0
        } {
            t97 += 1;
            if t97 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t97, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) {s.store_sqrt(726, 726);}
            let (t95,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) {
        let t94: f64 = (s.v[719] + 1.0);
        (t94,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t95);
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && (!s.b[2397])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && (!s.b[2396])) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && (!s.b[2396])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2134, 1.0, 337);}
        s.b[2402] = ((s.v[344] < (s.v[972] + p[405])) && (p[405] >= 0.0));s.store_scalar(2402, if s.b[2402] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {s.store_sub_offset_lhs(781, 972, p[405], 344);s.store_square(722, 781);s.store_scalar(723, (p[405] * p[405]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t98,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t98);
        let (t99,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t99);
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2403] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2403, if s.b[2403] { 1.0 } else { 0.0 });s.b[2404] = (2.0 == 1.0);s.store_scalar(2404, if s.b[2404] { 1.0 } else { 0.0 });
        let (t9a,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && s.b[2404]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9a);s.b[2405] = (2.0 == 2.0);s.store_scalar(2405, if s.b[2405] { 1.0 } else { 0.0 });
        let (t9b,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && s.b[2405]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9b);s.b[2406] = (2.0 == 4.0);s.store_scalar(2406, if s.b[2406] { 1.0 } else { 0.0 });
        let (t9c,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && (!s.b[2405])) && s.b[2406]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9c);s.b[2407] = (2.0 == 8.0);s.store_scalar(2407, if s.b[2407] { 1.0 } else { 0.0 });
        let (t9d,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && (!s.b[2405])) && (!s.b[2406])) && s.b[2407]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9d);
        let (t9e,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t9e);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_109(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut ta2: usize = 0;
        while {
            let ta1: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta1 != 0.0
        } {
            ta2 += 1;
            if ta2 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta2, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) {s.store_sqrt(726, 726);}
            let (ta0,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) {
        let t9f: f64 = (s.v[719] + 1.0);
        (t9f,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta0);
        }
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && (!s.b[2403])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[405]);s.store_div_scaled_product_indices(334, 725, 726, p[405], 770, 1.0);s.store_sub_offset_lhs(992, 972, p[405], 780);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && (!s.b[2402])) {s.copy_ad(992, 344);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.copy_ad(2159, 2145);s.store_offset_mul(338, 2135, 2159, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.store_add_mul_sub_from_scalar_rhs_indices(2160, 2159, 2134, 1.0, 337);s.copy_ad(2156, 2160);}
        let (ta3,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, ta3);
        let (ta4,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, ta4);let mut tab: usize = 0;
        while {
            let taa: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            taa != 0.0
        } {
            tab += 1;
            if tab > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tab, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.store_mul_scale_offset_indices(335, 2156, 154, -1.0, 0.0);s.store_exp(336, 335);s.store_sqrt_div_scaled_inputs(338, 2114, 2.0, 154, 1.0);s.store_offset_sub(344, 336, 335, (-1.0));s.store_mul_sqrt_mixed_ia(2157, 338, A::offset(s.ad_value(344), 1e-15));}
            s.b[2408] = (s.v[335] > 0.0);s.store_scalar(2408, if s.b[2408] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && s.b[2408]) {s.store_neg(2157, 2157);}
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2157, 1.0);s.store_mul_scale_offset_indices(2158, 345, 336, -1.0, 1.0);}
            let (ta9,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] != 0.0)) {
        let ta8: f64 = (150.0 + 1.0);
        (ta8,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, ta9);
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2100, 2157, 1.0, 185, 2159, 2156, -1.0);s.store_add(2101, 185, 2158);s.store_div_scaled_inputs_indices(2112, 2100, -1.0, 2101, 1.0);}
            s.b[2409] = (((s.v[2112]) as f64).abs() < 1e-10);s.store_scalar(2409, if s.b[2409] { 1.0 } else { 0.0 });
            let (ta5,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && s.b[2409]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, ta5);s.b[2410] = (s.v[2112] > 0.1);s.store_scalar(2410, if s.b[2410] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && (!s.b[2409])) && s.b[2410]) {s.store_scalar(2112, 0.1);}
            s.b[2411] = (s.v[2112] < (-0.1));s.store_scalar(2411, if s.b[2411] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && (!s.b[2409])) && (!s.b[2410])) && s.b[2411]) {s.store_scalar(2112, (-0.1));}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) {s.store_add(2156, 2156, 2112);}
            let (ta7,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
        let ta6: f64 = (s.v[97] + 1.0);
        (ta6,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, ta7);
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.copy_ad(2153, 2156);s.copy_ad(989, 349);s.store_sqrt_square_offset(782, 2153, ((4.0 * p[405]) * p[405]));s.store_offset_scaled_div(334, 2153, 782, 0.5, 0.5);s.store_scaled_add(992, 2153, 782, 0.5);}
        s.b[2412] = (s.v[992] < 0.0);s.store_scalar(2412, if s.b[2412] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && s.b[2412]) {s.store_scalar(992, 0.0);s.store_scalar(334, 0.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_div(335, 989, 992);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p[383] - 1.0));
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_offset_mul(337, 336, 335, 1.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p[383]) - 1.0));
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_mul(340, 338, 337);}
        s.b[2413] = ((s.v[349] > (s.v[972] - (s.v[972] * 0.5))) && ((s.v[972] * 0.5) >= 0.0));s.store_scalar(2413, if s.b[2413] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {s.store_add_scaled_inputs3_indices(781, 349, 1.0, 972, (-1.0), 972, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_110(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {s.store_square(722, 781);s.store_scaled_mul(723, 972, 972, (0.5 * 0.5));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tac,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tac);
        let (tad,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tad);
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2414] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2414, if s.b[2414] { 1.0 } else { 0.0 });s.b[2415] = (2.0 == 1.0);s.store_scalar(2415, if s.b[2415] { 1.0 } else { 0.0 });
        let (tae,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && s.b[2415]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tae);s.b[2416] = (2.0 == 2.0);s.store_scalar(2416, if s.b[2416] { 1.0 } else { 0.0 });
        let (taf,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && s.b[2416]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, taf);s.b[2417] = (2.0 == 4.0);s.store_scalar(2417, if s.b[2417] { 1.0 } else { 0.0 });
        let (tb0,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && (!s.b[2416])) && s.b[2417]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb0);s.b[2418] = (2.0 == 8.0);s.store_scalar(2418, if s.b[2418] { 1.0 } else { 0.0 });
        let (tb1,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && (!s.b[2416])) && (!s.b[2417])) && s.b[2418]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb1);
        let (tb2,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb2);let mut tb6: usize = 0;
        while {
            let tb5: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb5 != 0.0
        } {
            tb6 += 1;
            if tb6 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tb6, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) {s.store_sqrt(726, 726);}
            let (tb4,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) {
        let tb3: f64 = (s.v[719] + 1.0);
        (tb3,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tb4);
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && (!s.b[2414])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 972, 0.5, 0.0, 726);s.store_div_scaled_product3_indices(334, 972, 725, 726, 0.5, 770, 1.0);s.store_add_scaled_inputs3_indices(2166, 972, 1.0, 972, (-0.5), 780, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2413])) {s.copy_ad(2166, 349);s.store_scalar(334, 1.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_add_div_lhs_indices(989, 989, 340, 2166);s.store_mul_square_lhs(338, 2166, 2166);s.store_offset(334, 338, 0.0001);s.store_div(2167, 338, 334);}
        s.b[2419] = (p[43] == (-1.0));s.store_scalar(2419, if s.b[2419] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2419]) {s.store_scalar(2167, 0.0);s.copy_ad(989, 349);}
        s.b[2420] = (p[43] == 2.0);s.store_scalar(2420, if s.b[2420] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) && s.b[2420]) {s.copy_ad(989, 349);s.store_scalar(2166, 0.0);s.store_scalar(2167, 0.0);s.store_sub(335, 2146, 972);s.store_add_scaled_inputs3_offset_mixed_iai(992, 335, 0.5, A::ln(A::cosh(s.ad_value(335))), 0.5, 972, 1.0, (((2.0) as f64).ln() * 0.5));}
        s.b[2421] = (p[43] == 3.0);s.store_scalar(2421, if s.b[2421] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) && (!s.b[2420])) && s.b[2421]) {s.store_add_mixed_ai(992, A::ln_one_plus_exp(A::sub(s.ad_value(2146), s.ad_value(972))), 972);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {s.store_div(335, 989, 992);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p[383] - 1.0));
            }
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p[383]) - 1.0));
            }
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {s.store_mul(340, 338, 337);s.store_add_div_lhs_indices(989, 989, 340, 2166);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_mul(2124, 990, 2133);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 2124, 343);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_111(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_offset_sqrt_ad(2168, A::offset(A::square(s.ad_value(989)), p[262]), (-((p[262]) as f64).sqrt()));s.store_offset_mul(338, 2168, 688, 1.0);s.store_offset_mul(339, 2168, 689, 1.0);}
        s.b[2422] = param_given[408];s.store_scalar(2422, if s.b[2422] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2422]) {s.store_div_scaled_value_by_product_mixed_aii(2154, A::sub_from_scalar(p[408], s.ad_value(2092)), 1.0, 965, 339, 100.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2422])) {s.store_div_scaled_inputs_indices(2154, 2124, 9662367879.197212, 339, 1.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[2154] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 2154, p[376]);
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add(s.ad_value(966), A::mul3_scaled_output(s.ad_value(968), s.ad_value(338), s.ad_value(252), 1e-10)), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div_scaled_value_offset_denominator(2115, s.ad_value(989), 1.0, s.ad_value(162), p[401], 1.0);s.store_square(781, 989);s.store_scalar(782, {let pb=0.01;pb*pb});s.store_sub_ad(334, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));s.store_div_scaled_value_offset_denominator(2169, s.ad_value(334), 1.0, s.ad_value(162), (-p[402]), 1.0);s.store_div_scaled_product_indices(335, 254, 2169, 1.0, 973, 1.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[378]);
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_offset(337, 336, 1.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p[378]));
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_div(985, 254, 338);s.store_mul_scale_offset_mixed_ia(2132, 964, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2115), 1.0, A::div_scalar_offset_denominator(1.0, A::div_scaled_product(s.ad_value(254), s.ad_value(2115), 1.0, s.ad_value(973), 1.0), 1.0, 1.0), p[400]), 1.0, 1.0);s.store_scaled_mul(335, 990, 2132, 1.6021918e-19);s.store_scale_ad(336, A::pow(A::div_from_scalar(s.v[163], s.ad_value(162)), s.ad_value(976)), p[7]);s.store_mul3_affine_lhs(987, 335, 985, s.v[632], 0.0, 2115);s.store_mul3_affine_lhs(988, 336, 2155, p[363], 0.0, 2167);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_mul3_lhs(986, 115, 248, 984);s.store_add_scaled_inputs3_indices(135, 986, 1.0, 987, 1.0, 988, 1.0);s.copy_ad(790, 349);}
        s.b[2423] = (p[283] != 0.0);s.store_scalar(2423, if s.b[2423] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2423]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);}
    }
}
