#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        s: &mut Scratch,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_indices(336, 335, 658);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_offset(337, 336, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::div_from_scalar(1.0, s.ad_value(658)));
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_div(348, 790, 338);s.copy_ad(790, 348);}
        s.b[2248] = (s.v[790] < 0.0);s.store_scalar(2248, if s.b[2248] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2248]) {s.copy_ad(2086, 2085);s.copy_ad(2091, 2090);s.copy_ad(2089, 2088);s.copy_ad(2119, 2118);s.copy_ad(2115, 2114);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.copy_ad(2084, 790);s.store_add_scaled_inputs3_offset_indices(781, 2085, 1.0, 2084, 1.0, 85, -1.0, (-0.01));s.store_scaled_add(782, 2085, 2084, (4.0 * 0.01));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(2093, 2085, 1.0, 2084, 1.0, 781, (-0.5), 782, (-0.5));s.store_add_scaled_inputs3_offset_indices(781, 2093, 1.0, 2113, -1.0, 2087, 1.0, (-0.01));s.store_scaled_sub(782, 2113, 2087, (4.0 * 0.01));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(2093, 2113, 1.0, 2087, (-1.0), 781, 0.5, 782, 0.5);s.copy_ad(2089, 2084);s.copy_ad(2086, 2093);}
        let (t0,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t0);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul(2137, 2125, 2126);}
        let (t1,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.store_scalar(98, t1);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t2c: usize = 0;
        while {
            let t2b: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[98] <= 150.0)) { 1.0 } else { 0.0 };
            t2b != 0.0
        } {
            t2c += 1;
            if t2c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul_sub_mixed_iai(2091, 2125, A::add_scaled_product(s.ad_value(2113), 1.0, s.ad_value(2126), s.ad_value(2089), 1.0), 2087);s.store_sub(335, 2089, 2091);}
            s.b[2249] = ((s.v[335] < 0.001) && (0.001 >= 0.0));s.store_scalar(2249, if s.b[2249] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {s.store_sub_from_scalar(781, 0.001, 335);s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t28,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t28);
            let (t2a,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t2a);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2250] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2250, if s.b[2250] { 1.0 } else { 0.0 });s.b[2251] = (2.0 == 1.0);s.store_scalar(2251, if s.b[2251] { 1.0 } else { 0.0 });
            let (t12,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && s.b[2251]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t12);s.b[2252] = (2.0 == 2.0);s.store_scalar(2252, if s.b[2252] { 1.0 } else { 0.0 });
            let (t13,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && s.b[2252]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t13);s.b[2253] = (2.0 == 4.0);s.store_scalar(2253, if s.b[2253] { 1.0 } else { 0.0 });
            let (t14,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && s.b[2253]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t14);s.b[2254] = (2.0 == 8.0);s.store_scalar(2254, if s.b[2254] { 1.0 } else { 0.0 });
            let (t15,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && (!s.b[2253])) && s.b[2254]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t15);
            let (t16,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t16);let mut t1a: usize = 0;
            while {
                let t19: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t19 != 0.0
            } {
                t1a += 1;
                if t1a > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1a, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {s.store_sqrt(726, 726);}
                let (t18,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
        let t17: f64 = (s.v[719] + 1.0);
        (t17,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t18);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && (!s.b[2250])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.001);s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);s.store_sub_from_scalar(335, 0.001, 780);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2249])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2249])) {s.store_scalar(336, 1.0);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_sqrt_mul(2082, 2132, 335);}
            s.b[2255] = ((s.v[2082] > (s.v[2127] - 1e-12)) && (1e-12 >= 0.0));s.store_scalar(2255, if s.b[2255] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {s.store_offset_sub(781, 2082, 2127, 1e-12);s.store_square(722, 781);s.store_scalar(723, (1e-12 * 1e-12));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t1b,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1b);
            let (t1c,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1c);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2256] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2256, if s.b[2256] { 1.0 } else { 0.0 });s.b[2257] = (2.0 == 1.0);s.store_scalar(2257, if s.b[2257] { 1.0 } else { 0.0 });
            let (t1d,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && s.b[2257]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1d);s.b[2258] = (2.0 == 2.0);s.store_scalar(2258, if s.b[2258] { 1.0 } else { 0.0 });
            let (t1e,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1e);s.b[2259] = (2.0 == 4.0);s.store_scalar(2259, if s.b[2259] { 1.0 } else { 0.0 });
            let (t1f,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && s.b[2259]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1f);s.b[2260] = (2.0 == 8.0);s.store_scalar(2260, if s.b[2260] { 1.0 } else { 0.0 });
            let (t20,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && (!s.b[2259])) && s.b[2260]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t20);
            let (t21,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t21);let mut t25: usize = 0;
            while {
                let t24: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t24 != 0.0
            } {
                t25 += 1;
                if t25 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t25, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {s.store_sqrt(726, 726);}
                let (t23,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
        let t22: f64 = (s.v[719] + 1.0);
        (t22,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t23);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && (!s.b[2256])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-12);s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);s.store_add_offset_lhs(2082, 2127, (-1e-12), 780);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2255])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2255])) {s.store_scalar(337, 1.0);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul(337, 336, 337);s.store_add_div_rhs_mixed_ai(2133, 2086, A::add_scaled_square_product(s.ad_value(2127), 1.0, s.ad_value(2082), A::sub_scaled_inputs(s.ad_value(2082), 1.0, s.ad_value(2127), 2.0), 1.0), 2132);s.store_scalar(2134, 1.0);s.store_mul_scale_offset_mixed_ai(2135, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2127), s.ad_value(2082)), s.ad_value(337), (-1.0)), 2137, -1.0, 1.0);}
            s.b[2261] = ((s.v[2133] > (s.v[2084] - p[406])) && (p[406] >= 0.0));s.store_scalar(2261, if s.b[2261] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {s.store_offset_sub(781, 2133, 2084, p[406]);s.store_square(722, 781);s.store_scalar(723, (p[406] * p[406]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t26,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t26);
            let (t27,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t27);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2262] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2262, if s.b[2262] { 1.0 } else { 0.0 });s.b[2263] = (4.0 == 1.0);s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });
            let (t29,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && s.b[2263]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t29);s.b[2264] = (4.0 == 2.0);s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });
            let (t2,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && s.b[2264]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t2);s.b[2265] = (4.0 == 4.0);s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });
            let (t3,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && s.b[2265]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t3);s.b[2266] = (4.0 == 8.0);s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });
            let (t4,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && (!s.b[2265])) && s.b[2266]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t4);
            let (t5,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t5);let mut t9: usize = 0;
            while {
                let t8: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t8 != 0.0
            } {
                t9 += 1;
                if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {s.store_sqrt(726, 726);}
                let (t7,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
        let t6: f64 = (s.v[719] + 1.0);
        (t6,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t7);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && (!s.b[2262])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[406]);s.store_div_scaled_product_indices(334, 725, 726, p[406], 770, 1.0);s.store_add_offset_lhs(2133, 2084, (-p[406]), 780);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {s.store_scalar(334, 1.0);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul(2134, 2134, 334);s.store_mul(2135, 2135, 334);s.store_mul_sub_rhs(339, 154, 2086, 2089);s.store_exp(340, 339);s.store_sub_offset_lhs(344, 340, (-1.0), 339);}
            s.b[2267] = (s.v[339] >= 1e-7);s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });
            let (tb,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2267]) {
        let ta: f64 = (-1.0);
        (ta,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, tb);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2267]) {s.store_mul_scaled_sqrt_rhs(2095, 209, -1.0, 344);s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2095, 1.0);s.store_mul_scale_offset_indices(2122, 345, 340, 1.0, (-1.0));s.store_mul_scale_offset_indices(2124, 345, 340, -1.0, 1.0);}
            s.b[2268] = (s.v[339] < (-1e-7));s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });
            let (tc,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && s.b[2268]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, tc);
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && s.b[2268]) {s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2086), 1.0, s.ad_value(2113), p[398]));s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2089), 1.0, s.ad_value(2113), p[398]));s.store_mul_sqrt_mixed_ia(2095, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2095, 1.0);s.store_mul_add_mixed_iaa(2122, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));s.store_mul_mixed_ia(2124, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));}
            s.b[2269] = (s.v[339] > 0.0);s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && s.b[2269]) {s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2160, 2159);s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);s.store_neg(2124, 2122);}
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && (!s.b[2269])) {s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2160, 2159);s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);s.store_neg(2124, 2122);}
            let (te,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] != 0.0)) {
        let td: f64 = (150.0 + 1.0);
        (td,)
    } else {
        (s.v[98],)
    }
};
            s.store_scalar(98, te);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2096, 2095, 1.0, 185, 85, 2086, 1.0);s.store_sub(2097, 2122, 185);s.copy_ad(2098, 2124);s.store_sub(2099, 2089, 2133);s.store_neg(2100, 2134);s.store_sub_from_scalar(2101, 1.0, 2135);s.store_add_scaled_products_indices(2102, 2097, 2101, 1.0, 2098, 2100, (-1.0));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                if (s.v[2102] > 0.0) {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, (-1e-25));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {s.copy_ad(2104, 2101);s.store_neg(2105, 2098);s.store_neg(2106, 2100);s.copy_ad(2107, 2097);s.store_mul_add_scaled_products_indices_rhs(2108, 2103, 2104, 2096, -1.0, 2105, 2099, -1.0);s.store_mul_add_scaled_products_indices_rhs(2109, 2103, 2106, 2096, -1.0, 2107, 2099, -1.0);s.store_abs(335, 2108);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2109]) as f64).abs()) {
                    s.store_abs(335, 2109);
                } else {
                }
            }
            s.b[2270] = (s.v[335] > 0.1);s.store_scalar(2270, if s.b[2270] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2270]) {s.store_mul_div_from_scalar_lhs_ad_indices(2108, 0.1, 335, 2108);s.store_mul_div_from_scalar_lhs_ad_indices(2109, 0.1, 335, 2109);}
            s.b[2271] = (s.v[335] < 1e-10);s.store_scalar(2271, if s.b[2271] { 1.0 } else { 0.0 });
            let (tf,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2271]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, tf);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {s.store_add(2086, 2086, 2108);s.store_add(2089, 2089, 2109);}
            let (t11,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        let t10: f64 = (s.v[98] + 1.0);
        (t10,)
    } else {
        (s.v[98],)
    }
};
            s.store_scalar(98, t11);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul_sub_rhs(339, 154, 2086, 2089);s.store_exp(340, 339);s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[2086] > s.v[2089]) {
                s.store_mul_scaled_sqrt_rhs(2119, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2119, 209, 344);
            }
        }
        s.b[2273] = (1.0 == 1.0);s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });s.b[2274] = (((s.v[2086] - s.v[2084]) < p[403]) && (p[403] >= 0.0));s.store_scalar(2274, if s.b[2274] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {s.store_sub_from_scalar_ad(781, p[403], A::sub(s.ad_value(2086), s.ad_value(2084)));s.store_square(722, 781);s.store_scalar(723, (p[403] * p[403]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t2d,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t2d);
        let (t2e,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2e);
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2275] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });s.b[2276] = (6.0 == 1.0);s.store_scalar(2276, if s.b[2276] { 1.0 } else { 0.0 });
        let (t2f,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && s.b[2276]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2f);s.b[2277] = (6.0 == 2.0);s.store_scalar(2277, if s.b[2277] { 1.0 } else { 0.0 });
        let (t30,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && s.b[2277]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t30);s.b[2278] = (6.0 == 4.0);s.store_scalar(2278, if s.b[2278] { 1.0 } else { 0.0 });
        let (t31,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && (!s.b[2277])) && s.b[2278]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t31);s.b[2279] = (6.0 == 8.0);s.store_scalar(2279, if s.b[2279] { 1.0 } else { 0.0 });
        let (t32,) = {
    if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && (!s.b[2277])) && (!s.b[2278])) && s.b[2279]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t32);
        let (t33,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t33);let mut t37: usize = 0;
        while {
            let t36: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t36 != 0.0
        } {
            t37 += 1;
            if t37 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t37, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {s.store_sqrt(726, 726);}
            let (t35,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {
        let t34: f64 = (s.v[719] + 1.0);
        (t34,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t35);
        }
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && (!s.b[2275])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[403]);s.store_div_scaled_product_indices(334, 725, 726, p[403], 770, 1.0);s.store_sub_from_scalar(336, p[403], 780);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && (!s.b[2274])) {s.store_sub(336, 2086, 2084);s.store_scalar(334, 1.0);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(2115, 209, -1.0, 338);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2273])) {s.copy_ad(2115, 2119);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.copy_ad(87, 2085);s.copy_ad(91, 2086);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_sub(94, 2086, 2085);s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / ((p[263] * 0.1))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, (p[263] * 0.1), 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[2280] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2280, if s.b[2280] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t38,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t38);
        let (t39,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t39);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2281] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2281, if s.b[2281] { 1.0 } else { 0.0 });s.b[2282] = (2.0 == 1.0);s.store_scalar(2282, if s.b[2282] { 1.0 } else { 0.0 });
        let (t3a,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && s.b[2282]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3a);s.b[2283] = (2.0 == 2.0);s.store_scalar(2283, if s.b[2283] { 1.0 } else { 0.0 });
        let (t3b,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && s.b[2283]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3b);s.b[2284] = (2.0 == 4.0);s.store_scalar(2284, if s.b[2284] { 1.0 } else { 0.0 });
        let (t3c,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && (!s.b[2283])) && s.b[2284]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3c);s.b[2285] = (2.0 == 8.0);s.store_scalar(2285, if s.b[2285] { 1.0 } else { 0.0 });
        let (t3d,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && (!s.b[2283])) && (!s.b[2284])) && s.b[2285]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3d);
        let (t3e,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t3e);let mut t42: usize = 0;
        while {
            let t41: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t41 != 0.0
        } {
            t42 += 1;
            if t42 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t42, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {s.store_sqrt(726, 726);}
            let (t40,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {
        let t3f: f64 = (s.v[719] + 1.0);
        (t3f,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t40);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && (!s.b[2281])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2280])) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2280])) {s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_add(109, 87, 110);}
        s.b[2286] = (((s.v[109] - s.v[2083]) < p[403]) && (p[403] >= 0.0));s.store_scalar(2286, if s.b[2286] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_sub_from_scalar_ad(781, p[403], A::sub(s.ad_value(109), s.ad_value(2083)));s.store_square(722, 781);s.store_scalar(723, (p[403] * p[403]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t43,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t43);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t44,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t44);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2287] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2287, if s.b[2287] { 1.0 } else { 0.0 });s.b[2288] = (6.0 == 1.0);s.store_scalar(2288, if s.b[2288] { 1.0 } else { 0.0 });
        let (t45,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && s.b[2288]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t45);s.b[2289] = (6.0 == 2.0);s.store_scalar(2289, if s.b[2289] { 1.0 } else { 0.0 });
        let (t46,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && s.b[2289]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t46);s.b[2290] = (6.0 == 4.0);s.store_scalar(2290, if s.b[2290] { 1.0 } else { 0.0 });
        let (t47,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && (!s.b[2289])) && s.b[2290]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t47);s.b[2291] = (6.0 == 8.0);s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });
        let (t48,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && (!s.b[2289])) && (!s.b[2290])) && s.b[2291]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t48);
        let (t49,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t49);let mut t4d: usize = 0;
        while {
            let t4c: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4c != 0.0
        } {
            t4d += 1;
            if t4d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t4d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {s.store_sqrt(726, 726);}
            let (t4b,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {
        let t4a: f64 = (s.v[719] + 1.0);
        (t4a,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t4b);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && (!s.b[2287])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[403]);s.store_div_scaled_product_indices(334, 725, 726, p[403], 770, 1.0);s.store_sub_from_scalar(336, p[403], 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2286])) {s.store_sub(336, 109, 2083);s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(2116, 209, -1.0, 338);s.store_sqrt_offset_ad(782, A::mul_scaled_lhs(A::add(s.ad_value(2115), s.ad_value(2114)), 1.0, A::add(s.ad_value(2115), s.ad_value(2114))), ((4.0 * (1e-12 * 1e-6)) * (1e-12 * 1e-6)));s.store_scaled_offset_ad(335, A::div_scaled_inputs2(s.ad_value(2115), -1.0, s.ad_value(2114), -1.0, s.ad_value(782), 1.0), 1.0, 0.5);s.store_add_scaled_inputs3_indices(2117, 2115, (-0.5), 2114, (-0.5), 782, 0.5);}
        s.b[2292] = (s.v[2117] < 0.0);s.store_scalar(2292, if s.b[2292] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2292]) {s.store_scalar(2117, 0.0);s.store_scalar(335, 0.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_neg(2117, 2117);s.store_mul3_affine_lhs(248, 154, 2117, (-1.0 / (2.0)), 0.0, 94);s.store_neg(238, 2116);s.copy_ad(170, 162);s.copy_ad(790, 349);}
        s.b[2293] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2293, if s.b[2293] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) {s.store_scalar(169, 0.0);s.copy_ad(168, 91);}
        s.b[2294] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2294, if s.b[2294] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t4e,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t4e);
        let (t4f,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4f);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2295] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2295, if s.b[2295] { 1.0 } else { 0.0 });s.b[2296] = (2.0 == 1.0);s.store_scalar(2296, if s.b[2296] { 1.0 } else { 0.0 });
        let (t50,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && s.b[2296]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t50);s.b[2297] = (2.0 == 2.0);s.store_scalar(2297, if s.b[2297] { 1.0 } else { 0.0 });
        let (t51,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && s.b[2297]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t51);s.b[2298] = (2.0 == 4.0);s.store_scalar(2298, if s.b[2298] { 1.0 } else { 0.0 });
        let (t52,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t52);s.b[2299] = (2.0 == 8.0);s.store_scalar(2299, if s.b[2299] { 1.0 } else { 0.0 });
        let (t53,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) && s.b[2299]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t53);
        let (t54,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t54);let mut t58: usize = 0;
        while {
            let t57: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t57 != 0.0
        } {
            t58 += 1;
            if t58 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t58, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) {s.store_sqrt(726, 726);}
            let (t56,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) {
        let t55: f64 = (s.v[719] + 1.0);
        (t55,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t56);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && (!s.b[2295])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && (!s.b[2294])) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && (!s.b[2294])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) {s.store_sub(342, 91, 2113);}
        s.b[2300] = ((s.v[342] < (0.2 + ((-s.v[2113]) + 0.8))) && (((-s.v[2113]) + 0.8) >= 0.0));s.store_scalar(2300, if s.b[2300] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {s.store_sub_offset_lhs_mixed_ai(781, A::sub_from_scalar(0.8, s.ad_value(2113)), 0.2, 342);s.store_square(722, 781);s.store_square_ad(723, A::sub_from_scalar(0.8, s.ad_value(2113)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t59,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t59);
        let (t5a,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5a);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2301] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2301, if s.b[2301] { 1.0 } else { 0.0 });s.b[2302] = (1.0 == 1.0);s.store_scalar(2302, if s.b[2302] { 1.0 } else { 0.0 });
        let (t5b,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && s.b[2302]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5b);s.b[2303] = (1.0 == 2.0);s.store_scalar(2303, if s.b[2303] { 1.0 } else { 0.0 });
        let (t5c,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && s.b[2303]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5c);s.b[2304] = (1.0 == 4.0);s.store_scalar(2304, if s.b[2304] { 1.0 } else { 0.0 });
        let (t5d,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && (!s.b[2303])) && s.b[2304]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5d);s.b[2305] = (1.0 == 8.0);s.store_scalar(2305, if s.b[2305] { 1.0 } else { 0.0 });
        let (t5e,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && (!s.b[2303])) && (!s.b[2304])) && s.b[2305]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5e);
        let (t5f,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t5f);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t63: usize = 0;
        while {
            let t62: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t62 != 0.0
        } {
            t63 += 1;
            if t63 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t63, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) {s.store_sqrt(726, 726);}
            let (t61,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) {
        let t60: f64 = (s.v[719] + 1.0);
        (t60,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t61);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && (!s.b[2301])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul_mixed_ai(780, A::mul_sub_from_scalar_rhs(s.ad_value(781), 0.8, s.ad_value(2113)), 726);s.store_div_scaled_product_mixed_aii(334, A::mul_sub_from_scalar_lhs(0.8, s.ad_value(2113), s.ad_value(725)), 726, 1.0, 770, 1.0);s.store_sub_offset_lhs_mixed_ai(342, A::sub_from_scalar(0.8, s.ad_value(2113)), 0.2, 780);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && (!s.b[2300])) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && (!s.b[2300])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) {s.store_mul(343, 2132, 342);s.store_sqrt(171, 343);s.store_div_from_scalar(334, 1.0, 171);s.store_mul(335, 238, 334);s.store_scale(336, 335, s.v[509]);s.store_scale(337, 334, s.v[509]);s.store_add_scaled_product_indices(339, 336, 1.0, 508, 2129, 1.0);s.store_div_from_scalar(335, 1.0, 339);s.store_scale(338, 335, 1.034943e-10);s.store_scalar(335, (1.0 - s.v[507]));s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 109, s.v[507], 335, 91, 1.0);}
        s.b[2306] = ((s.v[168] > (((s.v[109] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2306, if s.b[2306] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 109, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t64,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t64);
        let (t65,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t65);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2307] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2307, if s.b[2307] { 1.0 } else { 0.0 });s.b[2308] = (2.0 == 1.0);s.store_scalar(2308, if s.b[2308] { 1.0 } else { 0.0 });
        let (t66,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && s.b[2308]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t66);s.b[2309] = (2.0 == 2.0);s.store_scalar(2309, if s.b[2309] { 1.0 } else { 0.0 });
        let (t67,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && s.b[2309]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t67);s.b[2310] = (2.0 == 4.0);s.store_scalar(2310, if s.b[2310] { 1.0 } else { 0.0 });
        let (t68,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && (!s.b[2309])) && s.b[2310]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t68);s.b[2311] = (2.0 == 8.0);s.store_scalar(2311, if s.b[2311] { 1.0 } else { 0.0 });
        let (t69,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && (!s.b[2309])) && (!s.b[2310])) && s.b[2311]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t69);
        let (t6a,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6a);let mut t6e: usize = 0;
        while {
            let t6d: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6d != 0.0
        } {
            t6e += 1;
            if t6e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t6e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) {s.store_sqrt(726, 726);}
            let (t6c,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) {
        let t6b: f64 = (s.v[719] + 1.0);
        (t6b,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t6c);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && (!s.b[2307])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 109, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && (!s.b[2306])) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && (!s.b[2306])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) {s.store_sub(340, 168, 91);s.store_mul(337, 154, 238);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) {s.store_div_from_scalar(335, 1.0, 337);s.store_mul(339, 248, 335);s.store_scale(344, 2129, 9662367879.197212);s.store_scalar(335, 100000.0);s.store_div_from_scalar(336, 1.0, 162);s.store_mul_mixed_ai(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);s.store_mul(341, 345, 338);s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));s.store_mul3_lhs(342, 345, 338, 338);s.store_sqrt_square_add(343, 341, 342);s.store_scaled_sub(169, 343, 341, 0.5);s.copy_ad(335, 169);s.store_mul(169, 208, 335);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_scale(169, 169, s.v[619]);s.store_sub(170, 170, 169);s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p[262]), (-((p[262]) as f64).sqrt()));s.store_offset_mul(338, 980, 334, 1.0);s.store_mul(339, 336, 238);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 251, p[160]);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_indices(340, 251, 624);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 238, 343);s.store_scalar(336, s.v[474]);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(338), s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2312] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2312, if s.b[2312] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2312]) {s.copy_ad(336, 335);}
        s.b[2313] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2313, if s.b[2313] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2312])) && s.b[2313]) {s.store_square(336, 335);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2312])) && (!s.b[2313])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[178]);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_offset(338, 336, 1.0);}
        s.b[2314] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2314, if s.b[2314] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2314]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2315] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2315, if s.b[2315] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2314])) && s.b[2315]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2314])) && (!s.b[2315])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 338, ((-1.0) / p[178]));
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_mul(253, 254, 339);s.copy_ad(984, 253);s.copy_ad(2112, 255);s.store_scalar(2320, 0.0);s.store_scalar(2151, 0.0);s.store_scalar(990, 0.0);s.store_scalar(2143, 0.0);s.store_scalar(2318, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_add_scaled_inputs3_offset_indices(2140, 1436, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));}
        s.b[2322] = (0.0 == 0.0);s.store_scalar(2322, if s.b[2322] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2322]) {s.store_offset(2141, 2140, (-p[393]));}
        s.b[2323] = (0.0 == 1.0);s.store_scalar(2323, if s.b[2323] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2322])) && s.b[2323]) {s.store_offset(2141, 1436, (((-s.v[160])) + ((-p[393]))));}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2322])) && (!s.b[2323])) {s.store_offset(2141, 85, (-p[393]));}
        s.b[2324] = (((s.v[2144]) as f64).abs() <= 0.0);s.store_scalar(2324, if s.b[2324] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2324]) {s.store_scalar(2149, 0.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.copy_ad(983, 87);s.store_scale(2166, 2113, p[399]);s.store_scalar(2321, ((s.v[160] + p[393]) - 3.0));}
        s.b[2325] = (1.0 == 1.0);s.store_scalar(2325, if s.b[2325] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2325]) {s.store_scale(2166, 2113, p[399]);s.store_offset(983, 2166, (-1.0));s.copy_ad(2320, 2321);s.copy_ad(2142, 2321);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2325])) {s.store_offset_scaled(2166, 2113, p[399], (-0.1));s.copy_ad(983, 87);s.copy_ad(2320, 2141);s.copy_ad(2142, 2141);}
        let (t6f,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t6f);
        let (t70,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t70);let mut t77: usize = 0;
        while {
            let t76: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t76 != 0.0
        } {
            t77 += 1;
            if t77 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t77, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2326] = (s.v[983] >= 0.0);s.store_scalar(2326, if s.b[2326] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2326]) {s.store_mul_scaled_sqrt_ad_rhs(2318, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2121, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2318, 1.0);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2326])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2166)));s.store_exp_mul(338, 154, 2166);s.store_mul_sqrt_mixed_ia(2318, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2318, 1.0);s.store_mul_add_mixed_iaa(2121, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            let (t72,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] != 0.0)) {
        let t71: f64 = (150.0 + 1.0);
        (t71,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t72);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2096, 2318, 1.0, 185, 2320, 983, 1.0);s.store_sub(2097, 2121, 185);s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);}
            s.b[2327] = (((s.v[2108]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2327, if s.b[2327] { 1.0 } else { 0.0 });
            let (t73,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && s.b[2327]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t73);s.b[2328] = (s.v[2108] > 0.1);s.store_scalar(2328, if s.b[2328] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2327])) && s.b[2328]) {s.store_scalar(2108, 0.1);}
            s.b[2329] = (s.v[2108] < (-0.1));s.store_scalar(2329, if s.b[2329] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2327])) && (!s.b[2328])) && s.b[2329]) {s.store_scalar(2108, (-0.1));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 2108);}
            let (t75,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        let t74: f64 = (s.v[97] + 1.0);
        (t74,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t75);
        }
        s.b[2331] = (1.0 == 1.0);s.store_scalar(2331, if s.b[2331] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2331]) {s.copy_ad(2167, 983);}
        s.b[2332] = ((s.v[983] < (s.v[2167] + 0.2)) && (0.2 >= 0.0));s.store_scalar(2332, if s.b[2332] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {s.store_sub_offset_lhs(781, 2167, 0.2, 983);s.store_square(722, 781);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t78,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t78);
        let (t79,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t79);
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2333] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2333, if s.b[2333] { 1.0 } else { 0.0 });s.b[2334] = (2.0 == 1.0);s.store_scalar(2334, if s.b[2334] { 1.0 } else { 0.0 });
        let (t7a,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && s.b[2334]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7a);s.b[2335] = (2.0 == 2.0);s.store_scalar(2335, if s.b[2335] { 1.0 } else { 0.0 });
        let (t7b,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && s.b[2335]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7b);s.b[2336] = (2.0 == 4.0);s.store_scalar(2336, if s.b[2336] { 1.0 } else { 0.0 });
        let (t7c,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && (!s.b[2335])) && s.b[2336]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7c);s.b[2337] = (2.0 == 8.0);s.store_scalar(2337, if s.b[2337] { 1.0 } else { 0.0 });
        let (t7d,) = {
    if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && (!s.b[2335])) && (!s.b[2336])) && s.b[2337]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7d);
        let (t7e,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t7e);let mut t82: usize = 0;
        while {
            let t81: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t81 != 0.0
        } {
            t82 += 1;
            if t82 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t82, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) {s.store_sqrt(726, 726);}
            let (t80,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) {
        let t7f: f64 = (s.v[719] + 1.0);
        (t7f,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t80);
        }
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && (!s.b[2333])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.2);s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);s.store_sub_offset_lhs(983, 2167, 0.2, 780);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && (!s.b[2332])) {
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && (!s.b[2332])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.copy_ad(2149, 983);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_scalar(2138, (if (1e-6 >= p[407]) { 1e-6 } else { p[407] }));}
        s.b[2338] = ((s.v[2149] > (-s.v[2138])) && (s.v[2138] >= 0.0));s.store_scalar(2338, if s.b[2338] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {s.store_add(781, 2149, 2138);s.store_square(722, 781);s.store_square(723, 2138);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t83,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t83);
        let (t84,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t84);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (t85,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t85);let mut t89: usize = 0;
        while {
            let t88: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && (s.v[719] < s.v[2139])) { 1.0 } else { 0.0 };
            t88 != 0.0
        } {
            t89 += 1;
            if t89 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t89, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (t87,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        let t86: f64 = (s.v[719] + 1.0);
        (t86,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t87);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2339] = ((((s.v[2139] == 1.0) || (s.v[2139] == 2.0)) || (s.v[2139] == 4.0)) || (s.v[2139] == 8.0));s.store_scalar(2339, if s.b[2339] { 1.0 } else { 0.0 });s.b[2340] = (s.v[2139] == 1.0);s.store_scalar(2340, if s.b[2340] { 1.0 } else { 0.0 });
        let (t8a,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && s.b[2340]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8a);s.b[2341] = (s.v[2139] == 2.0);s.store_scalar(2341, if s.b[2341] { 1.0 } else { 0.0 });
        let (t8b,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && s.b[2341]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8b);s.b[2342] = (s.v[2139] == 4.0);s.store_scalar(2342, if s.b[2342] { 1.0 } else { 0.0 });
        let (t8c,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && (!s.b[2341])) && s.b[2342]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8c);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_122(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        s.b[2343] = (s.v[2139] == 8.0);s.store_scalar(2343, if s.b[2343] { 1.0 } else { 0.0 });
        let (t8d,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && (!s.b[2341])) && (!s.b[2342])) && s.b[2343]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8d);
        let (t8e,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t8e);let mut t92: usize = 0;
        while {
            let t91: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t91 != 0.0
        } {
            t92 += 1;
            if t92 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t92, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) {s.store_sqrt(726, 726);}
            let (t90,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) {
        let t8f: f64 = (s.v[719] + 1.0);
        (t8f,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t90);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && (!s.b[2339])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2139), 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 2138, 726);s.store_div_scaled_product3_indices(334, 2138, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(983, 2138, -1.0, 780, 1.0);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2338])) {s.copy_ad(983, 2149);s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_neg(983, 983);s.store_mul3_affine_lhs(2316, 2129, 2144, (0.5 * 9662367879.197212), 0.0, 2144);s.store_mul_sqrt_mixed_ia(334, 2148, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2316)));s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);}
        s.b[2344] = (((s.v[334]) as f64).abs() > 0.0001);s.store_scalar(2344, if s.b[2344] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2344]) {s.store_div_ln_lhs(2317, 335, 2316);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2344])) {s.store_mul3_ad_middle(2317, A::square(s.ad_value(2148)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_mul(332, 2317, 983);}
        s.b[2345] = (s.v[332] > 500.0);s.store_scalar(2345, if s.b[2345] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2345]) {s.store_sub(2161, 983, 2316);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) {s.store_exp_mul_scaled_lhs_indices(334, 2317, -1.0, 2316);}
        s.b[2346] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2346, if s.b[2346] { 1.0 } else { 0.0 });s.b[2347] = (s.v[332] >= 500.0);s.store_scalar(2347, if s.b[2347] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && s.b[2347]) {s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(337, 1.403592217853e217);}
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) {s.copy_ad(781, 332);s.store_scalar(335, 1.0);}
        let mut t94: usize = 0;
        while {
            let t93: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            t93 != 0.0
        } {
            t94 += 1;
            if t94 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t94, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) {s.store_scale(335, 335, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) {s.store_mul_exp_rhs(335, 335, 781);s.copy_ad(337, 335);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) {s.store_mul(335, 335, 334);s.store_sub(336, 335, 334);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && (!s.b[2346])) {s.store_mul_scale_offset_indices(335, 334, 332, 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);}
        s.b[2348] = (((s.v[336]) as f64).abs() > 1e-8);s.store_scalar(2348, if s.b[2348] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2348]) {s.store_div_ln_offset_lhs(2161, 336, 1.0, 2317);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && (!s.b[2348])) {s.store_div(2161, 336, 2317);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_sub(336, 983, 2161);}
        s.b[2349] = (0.0 == 0.0);s.store_scalar(2349, if s.b[2349] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_123(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2349]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2143, A::sqrt(A::mul_scaled_lhs(s.ad_value(2132), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2143, 2132, 336);
            }
        }
        s.b[2350] = (s.v[336] < 0.0);s.store_scalar(2350, if s.b[2350] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2349])) && s.b[2350]) {s.store_mul(337, 154, 336);s.store_neg_ad(2143, A::sqrt(A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2349])) && (!s.b[2350])) {s.store_mul_scale_offset_indices(337, 336, 154, -1.0, 0.0);s.store_sqrt_ad(2143, A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_sub(990, 2144, 2143);}
        s.b[2351] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));s.store_scalar(2351, if s.b[2351] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {s.store_sub_from_scalar(781, 1e-16, 990);s.store_square(722, 781);s.store_scalar(723, (1e-16 * 1e-16));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t95,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t95);
        let (t96,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t96);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2352] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2352, if s.b[2352] { 1.0 } else { 0.0 });s.b[2353] = (2.0 == 1.0);s.store_scalar(2353, if s.b[2353] { 1.0 } else { 0.0 });
        let (t97,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && s.b[2353]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t97);s.b[2354] = (2.0 == 2.0);s.store_scalar(2354, if s.b[2354] { 1.0 } else { 0.0 });
        let (t98,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && s.b[2354]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t98);s.b[2355] = (2.0 == 4.0);s.store_scalar(2355, if s.b[2355] { 1.0 } else { 0.0 });
        let (t99,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && (!s.b[2354])) && s.b[2355]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t99);s.b[2356] = (2.0 == 8.0);s.store_scalar(2356, if s.b[2356] { 1.0 } else { 0.0 });
        let (t9a,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && (!s.b[2354])) && (!s.b[2355])) && s.b[2356]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9a);
        let (t9b,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t9b);let mut t9f: usize = 0;
        while {
            let t9e: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9e != 0.0
        } {
            t9f += 1;
            if t9f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t9f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) {s.store_sqrt(726, 726);}
            let (t9d,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) {
        let t9c: f64 = (s.v[719] + 1.0);
        (t9c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t9d);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && (!s.b[2352])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-16);s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);s.store_sub_from_scalar(990, 1e-16, 780);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2351])) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2351])) {s.store_scalar(334, 1.0);}
        s.b[2357] = (1.0 == 1.0);s.store_scalar(2357, if s.b[2357] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2357]) {s.copy_ad(2151, 990);}
        s.b[2358] = (2.0 == 1.0);s.store_scalar(2358, if s.b[2358] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_124(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2358]) {s.store_scale(2166, 2113, p[399]);s.store_offset(983, 2166, (-1.0));s.copy_ad(2320, 2321);s.copy_ad(2142, 2321);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2358])) {s.store_offset_scaled(2166, 2113, p[399], (-0.1));s.copy_ad(983, 87);s.copy_ad(2320, 2141);s.copy_ad(2142, 2141);}
        let (ta0,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, ta0);
        let (ta1,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, ta1);let mut ta8: usize = 0;
        while {
            let ta7: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            ta7 != 0.0
        } {
            ta8 += 1;
            if ta8 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta8, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2359] = (s.v[983] >= 0.0);s.store_scalar(2359, if s.b[2359] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2359]) {s.store_mul_scaled_sqrt_ad_rhs(2318, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2121, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2318, 1.0);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2359])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2166)));s.store_exp_mul(338, 154, 2166);s.store_mul_sqrt_mixed_ia(2318, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2318, 1.0);s.store_mul_add_mixed_iaa(2121, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            let (ta3,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] != 0.0)) {
        let ta2: f64 = (150.0 + 1.0);
        (ta2,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, ta3);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2096, 2318, 1.0, 185, 2320, 983, 1.0);s.store_sub(2097, 2121, 185);s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);}
            s.b[2360] = (((s.v[2108]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2360, if s.b[2360] { 1.0 } else { 0.0 });
            let (ta4,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && s.b[2360]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, ta4);s.b[2361] = (s.v[2108] > 0.1);s.store_scalar(2361, if s.b[2361] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2360])) && s.b[2361]) {s.store_scalar(2108, 0.1);}
            s.b[2362] = (s.v[2108] < (-0.1));s.store_scalar(2362, if s.b[2362] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2360])) && (!s.b[2361])) && s.b[2362]) {s.store_scalar(2108, (-0.1));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 2108);}
            let (ta6,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        let ta5: f64 = (s.v[97] + 1.0);
        (ta5,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, ta6);
        }
        s.b[2364] = (2.0 == 1.0);s.store_scalar(2364, if s.b[2364] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2364]) {s.copy_ad(2167, 983);}
        s.b[2365] = ((s.v[983] < (s.v[2167] + 0.2)) && (0.2 >= 0.0));s.store_scalar(2365, if s.b[2365] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {s.store_sub_offset_lhs(781, 2167, 0.2, 983);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (ta9,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta9);
        let (taa,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, taa);
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_125(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2366] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2366, if s.b[2366] { 1.0 } else { 0.0 });s.b[2367] = (2.0 == 1.0);s.store_scalar(2367, if s.b[2367] { 1.0 } else { 0.0 });
        let (tab,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && s.b[2367]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tab);s.b[2368] = (2.0 == 2.0);s.store_scalar(2368, if s.b[2368] { 1.0 } else { 0.0 });
        let (tac,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && s.b[2368]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tac);s.b[2369] = (2.0 == 4.0);s.store_scalar(2369, if s.b[2369] { 1.0 } else { 0.0 });
        let (tad,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && (!s.b[2368])) && s.b[2369]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tad);s.b[2370] = (2.0 == 8.0);s.store_scalar(2370, if s.b[2370] { 1.0 } else { 0.0 });
        let (tae,) = {
    if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && (!s.b[2368])) && (!s.b[2369])) && s.b[2370]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tae);
        let (taf,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, taf);let mut tb3: usize = 0;
        while {
            let tb2: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb2 != 0.0
        } {
            tb3 += 1;
            if tb3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tb3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) {s.store_sqrt(726, 726);}
            let (tb1,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) {
        let tb0: f64 = (s.v[719] + 1.0);
        (tb0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tb1);
        }
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && (!s.b[2366])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.2);s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);s.store_sub_offset_lhs(983, 2167, 0.2, 780);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && (!s.b[2365])) {
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && (!s.b[2365])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.copy_ad(2149, 983);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_scalar(2138, (if (1e-6 >= p[407]) { 1e-6 } else { p[407] }));}
        s.b[2371] = ((s.v[2149] > (-s.v[2138])) && (s.v[2138] >= 0.0));s.store_scalar(2371, if s.b[2371] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {s.store_add(781, 2149, 2138);s.store_square(722, 781);s.store_square(723, 2138);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb4,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb4);
        let (tb5,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb5);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (tb6,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb6);let mut tba: usize = 0;
        while {
            let tb9: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && (s.v[719] < s.v[2139])) { 1.0 } else { 0.0 };
            tb9 != 0.0
        } {
            tba += 1;
            if tba > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tba, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (tb8,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        let tb7: f64 = (s.v[719] + 1.0);
        (tb7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tb8);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2372] = ((((s.v[2139] == 1.0) || (s.v[2139] == 2.0)) || (s.v[2139] == 4.0)) || (s.v[2139] == 8.0));s.store_scalar(2372, if s.b[2372] { 1.0 } else { 0.0 });s.b[2373] = (s.v[2139] == 1.0);s.store_scalar(2373, if s.b[2373] { 1.0 } else { 0.0 });
        let (tbb,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && s.b[2373]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbb);s.b[2374] = (s.v[2139] == 2.0);s.store_scalar(2374, if s.b[2374] { 1.0 } else { 0.0 });
        let (tbc,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && s.b[2374]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbc);s.b[2375] = (s.v[2139] == 4.0);s.store_scalar(2375, if s.b[2375] { 1.0 } else { 0.0 });
        let (tbd,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && (!s.b[2374])) && s.b[2375]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbd);s.b[2376] = (s.v[2139] == 8.0);s.store_scalar(2376, if s.b[2376] { 1.0 } else { 0.0 });
        let (tbe,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && (!s.b[2374])) && (!s.b[2375])) && s.b[2376]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbe);
        let (tbf,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tbf);let mut tc3: usize = 0;
        while {
            let tc2: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc2 != 0.0
        } {
            tc3 += 1;
            if tc3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tc3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) {s.store_sqrt(726, 726);}
            let (tc1,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) {
        let tc0: f64 = (s.v[719] + 1.0);
        (tc0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc1);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_126(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && (!s.b[2372])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2139), 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 2138, 726);s.store_div_scaled_product3_indices(334, 2138, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(983, 2138, -1.0, 780, 1.0);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2371])) {s.copy_ad(983, 2149);s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_neg(983, 983);s.store_mul3_affine_lhs(2316, 2129, 2144, (0.5 * 9662367879.197212), 0.0, 2144);s.store_mul_sqrt_mixed_ia(334, 2148, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2316)));s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);}
        s.b[2377] = (((s.v[334]) as f64).abs() > 0.0001);s.store_scalar(2377, if s.b[2377] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2377]) {s.store_div_ln_lhs(2317, 335, 2316);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2377])) {s.store_mul3_ad_middle(2317, A::square(s.ad_value(2148)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_mul(332, 2317, 983);}
        s.b[2378] = (s.v[332] > 500.0);s.store_scalar(2378, if s.b[2378] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2378]) {s.store_sub(2161, 983, 2316);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) {s.store_exp_mul_scaled_lhs_indices(334, 2317, -1.0, 2316);}
        s.b[2379] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2379, if s.b[2379] { 1.0 } else { 0.0 });s.b[2380] = (s.v[332] >= 500.0);s.store_scalar(2380, if s.b[2380] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && s.b[2380]) {s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(337, 1.403592217853e217);}
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) {s.copy_ad(781, 332);s.store_scalar(335, 1.0);}
        let mut tc5: usize = 0;
        while {
            let tc4: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            tc4 != 0.0
        } {
            tc5 += 1;
            if tc5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tc5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) {s.store_scale(335, 335, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) {s.store_mul_exp_rhs(335, 335, 781);s.copy_ad(337, 335);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) {s.store_mul(335, 335, 334);s.store_sub(336, 335, 334);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && (!s.b[2379])) {s.store_mul_scale_offset_indices(335, 334, 332, 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);}
        s.b[2381] = (((s.v[336]) as f64).abs() > 1e-8);s.store_scalar(2381, if s.b[2381] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2381]) {s.store_div_ln_offset_lhs(2161, 336, 1.0, 2317);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && (!s.b[2381])) {s.store_div(2161, 336, 2317);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_sub(336, 983, 2161);}
        s.b[2382] = (0.0 == 0.0);s.store_scalar(2382, if s.b[2382] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2382]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2143, A::sqrt(A::mul_scaled_lhs(s.ad_value(2132), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2143, 2132, 336);
            }
        }
        s.b[2383] = (s.v[336] < 0.0);s.store_scalar(2383, if s.b[2383] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2382])) && s.b[2383]) {s.store_mul(337, 154, 336);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_127(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2382])) && s.b[2383]) {s.store_neg_ad(2143, A::sqrt(A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2382])) && (!s.b[2383])) {s.store_mul_scale_offset_indices(337, 336, 154, -1.0, 0.0);s.store_sqrt_ad(2143, A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_sub(990, 2144, 2143);}
        s.b[2384] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));s.store_scalar(2384, if s.b[2384] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {s.store_sub_from_scalar(781, 1e-16, 990);s.store_square(722, 781);s.store_scalar(723, (1e-16 * 1e-16));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tc6,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc6);
        let (tc7,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc7);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2385] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2385, if s.b[2385] { 1.0 } else { 0.0 });s.b[2386] = (2.0 == 1.0);s.store_scalar(2386, if s.b[2386] { 1.0 } else { 0.0 });
        let (tc8,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && s.b[2386]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc8);s.b[2387] = (2.0 == 2.0);s.store_scalar(2387, if s.b[2387] { 1.0 } else { 0.0 });
        let (tc9,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && s.b[2387]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc9);s.b[2388] = (2.0 == 4.0);s.store_scalar(2388, if s.b[2388] { 1.0 } else { 0.0 });
        let (tca,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && (!s.b[2387])) && s.b[2388]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tca);s.b[2389] = (2.0 == 8.0);s.store_scalar(2389, if s.b[2389] { 1.0 } else { 0.0 });
        let (tcb,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && (!s.b[2387])) && (!s.b[2388])) && s.b[2389]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcb);
        let (tcc,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tcc);let mut td0: usize = 0;
        while {
            let tcf: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tcf != 0.0
        } {
            td0 += 1;
            if td0 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", td0, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) {s.store_sqrt(726, 726);}
            let (tce,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) {
        let tcd: f64 = (s.v[719] + 1.0);
        (tcd,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tce);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && (!s.b[2385])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-16);s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);s.store_sub_from_scalar(990, 1e-16, 780);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2384])) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2384])) {s.store_scalar(334, 1.0);}
        s.b[2390] = (2.0 == 1.0);s.store_scalar(2390, if s.b[2390] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2390]) {s.copy_ad(2151, 990);}
        s.b[2391] = (0.0 == 0.0);s.store_scalar(2391, if s.b[2391] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) {s.copy_ad(989, 349);s.store_scaled_add(344, 2113, 155, p[396]);s.store_offset_mul_ad(338, s.ad_value(2131), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 2131, 1.0);}
        s.b[2392] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2392, if s.b[2392] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);}
    }
}
