#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_128(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t0,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t0);
        let (t1,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1);
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2395] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2395, if s.b[2395] { 1.0 } else { 0.0 });s.b[2396] = (2.0 == 1.0);s.store_scalar(2396, if s.b[2396] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && s.b[2396]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2);s.b[2397] = (2.0 == 2.0);s.store_scalar(2397, if s.b[2397] { 1.0 } else { 0.0 });
        let (t3,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && s.b[2397]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3);s.b[2398] = (2.0 == 4.0);s.store_scalar(2398, if s.b[2398] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && (!s.b[2397])) && s.b[2398]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);s.b[2399] = (2.0 == 8.0);s.store_scalar(2399, if s.b[2399] { 1.0 } else { 0.0 });
        let (t5,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && (!s.b[2397])) && (!s.b[2398])) && s.b[2399]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5);
        let (t6,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6);let mut ta: usize = 0;
        while {
            let t9: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;
            if ta > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) {s.store_sqrt(726, 726);}
            let (t8,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) {
        let t7: f64 = (s.v[719] + 1.0);
        (t7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8);
        }
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && (!s.b[2395])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2394])) {
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2394])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2132, 1.0, 337);}
        s.b[2400] = ((s.v[344] < (s.v[972] + p[405])) && (p[405] >= 0.0));s.store_scalar(2400, if s.b[2400] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {s.store_sub_offset_lhs(781, 972, p[405], 344);s.store_square(722, 781);s.store_scalar(723, (p[405] * p[405]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb);
        let (tc,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc);
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2401] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2401, if s.b[2401] { 1.0 } else { 0.0 });s.b[2402] = (2.0 == 1.0);s.store_scalar(2402, if s.b[2402] { 1.0 } else { 0.0 });
        let (td,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && s.b[2402]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td);s.b[2403] = (2.0 == 2.0);s.store_scalar(2403, if s.b[2403] { 1.0 } else { 0.0 });
        let (te,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && s.b[2403]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te);s.b[2404] = (2.0 == 4.0);s.store_scalar(2404, if s.b[2404] { 1.0 } else { 0.0 });
        let (tf,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && (!s.b[2403])) && s.b[2404]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf);s.b[2405] = (2.0 == 8.0);s.store_scalar(2405, if s.b[2405] { 1.0 } else { 0.0 });
        let (t10,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && (!s.b[2403])) && (!s.b[2404])) && s.b[2405]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t10);
        let (t11,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t11);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_129(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t15: usize = 0;
        while {
            let t14: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) {s.store_sqrt(726, 726);}
            let (t13,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) {
        let t12: f64 = (s.v[719] + 1.0);
        (t12,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t13);
        }
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && (!s.b[2401])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[405]);s.store_div_scaled_product_indices(334, 725, 726, p[405], 770, 1.0);s.store_sub_offset_lhs(992, 972, p[405], 780);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2400])) {s.copy_ad(992, 344);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {s.copy_ad(2157, 2143);s.store_offset_mul(338, 2133, 2157, 1.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {s.store_add_mul_sub_from_scalar_rhs_indices(2158, 2157, 2132, 1.0, 337);s.copy_ad(2154, 2158);}
        let (t16,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t16);
        let (t17,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t17);let mut t1e: usize = 0;
        while {
            let t1d: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t1d != 0.0
        } {
            t1e += 1;
            if t1e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {s.store_mul_scale_offset_indices(335, 2154, 154, -1.0, 0.0);s.store_exp(336, 335);s.store_sqrt_div_scaled_inputs(338, 2112, 2.0, 154, 1.0);s.store_offset_sub(344, 336, 335, (-1.0));s.store_mul_sqrt_mixed_ia(2155, 338, A::offset(s.ad_value(344), 1e-15));}
            s.b[2406] = (s.v[335] > 0.0);s.store_scalar(2406, if s.b[2406] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && s.b[2406]) {s.store_neg(2155, 2155);}
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2155, 1.0);s.store_mul_scale_offset_indices(2156, 345, 336, -1.0, 1.0);}
            let (t1c,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] != 0.0)) {
        let t1b: f64 = (150.0 + 1.0);
        (t1b,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t1c);
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2098, 2155, 1.0, 185, 2157, 2154, -1.0);s.store_add(2099, 185, 2156);s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);}
            s.b[2407] = (((s.v[2110]) as f64).abs() < 1e-10);s.store_scalar(2407, if s.b[2407] { 1.0 } else { 0.0 });
            let (t18,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && s.b[2407]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t18);s.b[2408] = (s.v[2110] > 0.1);s.store_scalar(2408, if s.b[2408] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && (!s.b[2407])) && s.b[2408]) {s.store_scalar(2110, 0.1);}
            s.b[2409] = (s.v[2110] < (-0.1));s.store_scalar(2409, if s.b[2409] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && (!s.b[2407])) && (!s.b[2408])) && s.b[2409]) {s.store_scalar(2110, (-0.1));}
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) {s.store_add(2154, 2154, 2110);}
            let (t1a,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
        let t19: f64 = (s.v[97] + 1.0);
        (t19,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t1a);
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {s.copy_ad(2151, 2154);s.copy_ad(989, 349);s.store_sqrt_square_offset(782, 2151, ((4.0 * p[405]) * p[405]));s.store_offset_scaled_div(334, 2151, 782, 0.5, 0.5);s.store_scaled_add(992, 2151, 782, 0.5);}
        s.b[2410] = (s.v[992] < 0.0);s.store_scalar(2410, if s.b[2410] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && s.b[2410]) {s.store_scalar(992, 0.0);s.store_scalar(334, 0.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_div(335, 989, 992);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p[383] - 1.0));
            }
        }
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_offset_mul(337, 336, 335, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p[383]) - 1.0));
            }
        }
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_mul(340, 338, 337);}
        s.b[2411] = ((s.v[349] > (s.v[972] - (s.v[972] * 0.5))) && ((s.v[972] * 0.5) >= 0.0));s.store_scalar(2411, if s.b[2411] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {s.store_add_scaled_inputs3_indices(781, 349, 1.0, 972, (-1.0), 972, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_130(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {s.store_square(722, 781);s.store_scaled_mul(723, 972, 972, (0.5 * 0.5));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t1f,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t1f);
        let (t20,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t20);
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2412] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2412, if s.b[2412] { 1.0 } else { 0.0 });s.b[2413] = (2.0 == 1.0);s.store_scalar(2413, if s.b[2413] { 1.0 } else { 0.0 });
        let (t21,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && s.b[2413]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t21);s.b[2414] = (2.0 == 2.0);s.store_scalar(2414, if s.b[2414] { 1.0 } else { 0.0 });
        let (t22,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && s.b[2414]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t22);s.b[2415] = (2.0 == 4.0);s.store_scalar(2415, if s.b[2415] { 1.0 } else { 0.0 });
        let (t23,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && (!s.b[2414])) && s.b[2415]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t23);s.b[2416] = (2.0 == 8.0);s.store_scalar(2416, if s.b[2416] { 1.0 } else { 0.0 });
        let (t24,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && (!s.b[2414])) && (!s.b[2415])) && s.b[2416]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t24);
        let (t25,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t25);let mut t29: usize = 0;
        while {
            let t28: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t28 != 0.0
        } {
            t29 += 1;
            if t29 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t29, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {s.store_sqrt(726, 726);}
            let (t27,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {
        let t26: f64 = (s.v[719] + 1.0);
        (t26,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t27);
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && (!s.b[2412])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 972, 0.5, 0.0, 726);s.store_div_scaled_product3_indices(334, 972, 725, 726, 0.5, 770, 1.0);s.store_add_scaled_inputs3_indices(2164, 972, 1.0, 972, (-0.5), 780, 1.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2411])) {s.copy_ad(2164, 349);s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_add_div_lhs_indices(989, 989, 340, 2164);s.store_mul_square_lhs(338, 2164, 2164);s.store_offset(334, 338, 0.0001);s.store_div(2165, 338, 334);}
        s.b[2417] = (p[43] == (-1.0));s.store_scalar(2417, if s.b[2417] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2417]) {s.store_scalar(2165, 0.0);s.copy_ad(989, 349);}
        s.b[2418] = (p[43] == 2.0);s.store_scalar(2418, if s.b[2418] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) && s.b[2418]) {s.copy_ad(989, 349);s.store_scalar(2164, 0.0);s.store_scalar(2165, 0.0);s.store_sub(335, 2144, 972);s.store_add_scaled_inputs3_offset_mixed_iai(992, 335, 0.5, A::ln(A::cosh(s.ad_value(335))), 0.5, 972, 1.0, (((2.0) as f64).ln() * 0.5));}
        s.b[2419] = (p[43] == 3.0);s.store_scalar(2419, if s.b[2419] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) && (!s.b[2418])) && s.b[2419]) {s.store_add_mixed_ai(992, A::ln_one_plus_exp(A::sub(s.ad_value(2144), s.ad_value(972))), 972);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {s.store_div(335, 989, 992);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p[383] - 1.0));
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p[383]) - 1.0));
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {s.store_mul(340, 338, 337);s.store_add_div_lhs_indices(989, 989, 340, 2164);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_mul(2122, 990, 2131);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 2122, 343);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_131(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_offset_sqrt_ad(2166, A::offset(A::square(s.ad_value(989)), p[262]), (-((p[262]) as f64).sqrt()));s.store_offset_mul(338, 2166, 688, 1.0);s.store_offset_mul(339, 2166, 689, 1.0);}
        s.b[2420] = param_given[408];s.store_scalar(2420, if s.b[2420] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2420]) {s.store_div_scaled_value_by_product_mixed_aii(2152, A::sub_from_scalar(p[408], s.ad_value(2090)), 1.0, 965, 339, 100.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2420])) {s.store_div_scaled_inputs_indices(2152, 2122, 9662367879.197212, 339, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[2152] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 2152, p[376]);
            }
        }
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add(s.ad_value(966), A::mul3_scaled_output(s.ad_value(968), s.ad_value(338), s.ad_value(252), 1e-10)), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div_scaled_value_offset_denominator(2113, s.ad_value(989), 1.0, s.ad_value(162), p[401], 1.0);s.store_square(781, 989);s.store_scalar(782, {let pb=0.01;pb*pb});s.store_sub_ad(334, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));s.store_div_scaled_value_offset_denominator(2167, s.ad_value(334), 1.0, s.ad_value(162), (-p[402]), 1.0);s.store_div_scaled_product_indices(335, 254, 2167, 1.0, 973, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[378]);
            }
        }
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_offset(337, 336, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p[378]));
            }
        }
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_div(985, 254, 338);s.store_mul_scale_offset_mixed_ia(2130, 964, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2113), 1.0, A::div_scalar_offset_denominator(1.0, A::div_scaled_product(s.ad_value(254), s.ad_value(2113), 1.0, s.ad_value(973), 1.0), 1.0, 1.0), p[400]), 1.0, 1.0);s.store_scaled_mul(335, 990, 2130, 1.6021918e-19);s.store_scale_ad(336, A::pow(A::div_from_scalar(s.v[163], s.ad_value(162)), s.ad_value(976)), p[7]);s.store_mul3_affine_lhs(987, 335, 985, s.v[632], 0.0, 2113);s.store_mul3_affine_lhs(988, 336, 2153, p[363], 0.0, 2165);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_mul3_lhs(986, 115, 248, 984);s.store_add_scaled_inputs3_indices(135, 986, 1.0, 987, 1.0, 988, 1.0);s.copy_ad(790, 349);}
        s.b[2421] = (p[283] != 0.0);s.store_scalar(2421, if s.b[2421] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_132(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) {s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(2087), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[2422] = (s.v[336] < 0.0);s.store_scalar(2422, if s.b[2422] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) && s.b[2422]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p[284]);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1437, p[285], 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 2087, 1.0, 340, 1.0, 1436, -1.0);s.store_add_product3_rhs_indices(338, 338, 1437, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2421])) {s.store_scalar(343, 0.0);}
        s.b[2423] = (p[287] != 0.0);s.store_scalar(2423, if s.b[2423] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2423]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1437);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2423])) {s.store_scalar(342, 0.0);}
        s.b[2424] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(2424, if s.b[2424] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2424]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.copy_ad(134, 135);s.store_add_scaled_inputs4_indices(131, 2096, (-0.5), 2120, ((-1.0) * (-0.5)), 2097, (-0.5), 2121, (-(-0.5)));s.store_scaled_add(133, 2120, 2121, (-0.5));s.store_scalar(247, 0.5);s.store_scaled_add(978, 2120, 2121, (-0.5));s.store_neg(238, 2120);s.copy_ad(255, 2114);}
        s.b[2425] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(2425, if s.b[2425] { 1.0 } else { 0.0 });
        let (t2a,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2425]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t2a);s.b[2426] = (s.v[791] < s.v[86]);s.store_scalar(2426, if s.b[2426] { 1.0 } else { 0.0 });
        let (t2c,) = {
    if ((!s.b[1441]) && s.b[2426]) {
        let t2b: f64 = (-1.0);
        (t2b,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, t2c);
        if ((!s.b[1441]) && s.b[2426]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_sub_rhs(332, 154, 85, 1433);s.store_div_scalar_by_product_indices(335, 1.0, 154, 209, 1.0);s.store_mul(333, 335, 185);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_offset(338, 332, (-2.0));s.store_scaled_mul(339, 333, 338, 9.0);s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);s.store_square(276, 278);}
        s.b[2427] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2427, if s.b[2427] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2426]) && s.b[2427]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((!s.b[1441]) && s.b[2426]) && (!s.b[2427])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((!s.b[1441]) && s.b[2426]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }
        if ((!s.b[1441]) && s.b[2426]) {s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div_from_scalar(335, 1.0, 273);s.store_mul(116, 272, 335);s.store_add_scaled_product_indices(167, 1433, 1.0, 116, 155, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_133(
        s: &mut Scratch,
    ) {
        if ((!s.b[1441]) && s.b[2426]) {s.store_sub(335, 167, 1433);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_add_div_lhs_indices(87, 335, 337, 1433);s.copy_ad(91, 87);s.store_scalar(94, 0.0);s.store_sub(336, 85, 87);s.store_mul(131, 185, 336);s.store_scalar(133, 0.0);s.store_scalar(247, 0.0);s.store_scalar(169, 0.0);s.store_scalar(134, 0.0);s.store_scalar(127, 0.0);}
        let (t2d,) = {
    if ((!s.b[1441]) && s.b[2426]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t2d);
        let (t2e,) = {
    if ((!s.b[1441]) && s.b[2426]) {
        (1.0,)
    } else {
        (s.v[946],)
    }
};
        s.store_scalar(946, t2e);s.b[2428] = (s.v[946] == 0.0);s.store_scalar(2428, if s.b[2428] { 1.0 } else { 0.0 });
        if ((!s.b[1441]) && s.b[2428]) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1433))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[1441]) && s.b[2428]) {s.store_add_product3_rhs_mixed_iia(89, 85, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);}
        s.b[2429] = (s.v[77] == 0.0);s.store_scalar(2429, if s.b[2429] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2429]) {s.store_mul_sub_rhs(116, 154, 89, 1433);}
        s.b[2430] = (s.v[116] < 3.0);s.store_scalar(2430, if s.b[2430] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {s.store_mul_sub_rhs(333, 154, 85, 1433);s.store_div_scalar_by_product_indices(335, 1.0, 154, 212, (1.414213562373095 / 108.0));s.store_offset_scaled(336, 335, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);s.store_square(338, 338);}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {s.store_add_scaled_inputs_mixed_ai(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 1.0, 339, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(89, 1433, 1.0, 332, 155, 1.0);s.copy_ad(88, 89);}
        s.b[2431] = (s.v[791] <= s.v[118]);s.store_scalar(2431, if s.b[2431] { 1.0 } else { 0.0 });
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && s.b[2431]) {s.copy_ad(88, 89);}
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {s.store_div_scalar_by_product_indices(335, 1.0, 210, 211, 1.0);s.store_mul3_lhs(336, 335, 85, 85);s.store_add_div_from_scalar_rhs(337, 154, 2.0, 85);s.store_div_ln_lhs(90, 336, 337);s.store_offset_sub(781, 90, 89, (-0.0008));s.store_scale(782, 90, (4.0 * 0.0008));}
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((!s.b[1441]) && s.b[2428]) {s.store_offset(332, 1433, (1e-12 / 2.0));}
        s.b[2432] = (s.v[88] < s.v[332]);s.store_scalar(2432, if s.b[2432] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2432]) {s.copy_ad(88, 332);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_134(
        s: &mut Scratch,
    ) {
        if ((!s.b[1441]) && s.b[2428]) {s.copy_ad(87, 88);s.copy_ad(92, 89);s.store_exp_mul(229, 154, 1433);s.store_mul(222, 210, 229);}
        let (t2f,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t2f);
        let (t30,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t30);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_135(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t3a: usize = 0;
        while {
            let t38: f64 = (s.v[421] + 1.0);let t39: f64 = if (((!s.b[1441]) && s.b[2428]) && (s.v[97] <= t38)) { 1.0 } else { 0.0 };
            t39 != 0.0
        } {
            t3a += 1;
            if t3a > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3a, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((!s.b[1441]) && s.b[2428]) {s.store_mul_sub_rhs(116, 154, 87, 1433);}
            s.b[2433] = (s.v[116] < 5.0);s.store_scalar(2433, if s.b[2433] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2433]) {s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(214, 222, 225, 225);s.store_mul_product3_indices(215, 226, 222, 154, 225, 2.0);s.store_mul_scale_offset_mixed_ia(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_inputs2_mixed_aii(217, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 215, 1.0, 216, 2.0);}
            s.b[2434] = (s.v[116] < 60.0);s.store_scalar(2434, if s.b[2434] { 1.0 } else { 0.0 });
            if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) && s.b[2434]) {s.store_exp(227, 116);s.store_mul_scale_offset_indices(214, 222, 227, 1.0, (-1.0));s.store_mul3_lhs(215, 222, 154, 227);}
            if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) && (!s.b[2434])) {s.store_exp_mul(231, 154, 87);s.store_mul_sub_rhs(214, 210, 231, 229);s.store_mul3_lhs(215, 210, 154, 231);}
            if (((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) {s.store_sqrt_add_ad(216, A::offset(s.ad_value(116), (-1.0)), s.ad_value(214));s.store_div_scaled_inputs2_indices(217, 154, 1.0, 215, 1.0, 216, 2.0);}
            if ((!s.b[1441]) && s.b[2428]) {s.store_add_scaled_inputs_product_indices(232, 85, 1.0, 87, (-1.0), 212, 216, (-1.0));s.store_sub_from_scalar_scaled_mul(233, (-1.0), 212, 217, 1.0);}
            s.b[2435] = (s.v[79] == 1.0);s.store_scalar(2435, if s.b[2435] { 1.0 } else { 0.0 });
            let (t31,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2435]) {
        (1.0,)
    } else {
        (s.v[944],)
    }
};
            s.store_scalar(944, t31);s.b[2436] = (s.v[944] == 0.0);s.store_scalar(2436, if s.b[2436] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[87]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(87))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2437] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2437, if s.b[2437] { 1.0 } else { 0.0 });
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2436]) && s.b[2437]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {s.store_add(87, 87, 236);}
            s.b[2438] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2438, if s.b[2438] { 1.0 } else { 0.0 });
            let (t32,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2436]) && s.b[2438]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t32);
            let (t34,) = {
    if (((!s.b[1441]) && s.b[2428]) && (s.v[944] != 0.0)) {
        let t33: f64 = (s.v[421] + 1.0);
        (t33,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t34);
            let (t35,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (0.0,)
    } else {
        (s.v[944],)
    }
};
            s.store_scalar(944, t35);
            let (t37,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        let t36: f64 = (s.v[97] + 1.0);
        (t36,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t37);
        }
        let (t3c,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        let t3b: f64 = (s.v[97] - 1.0);
        (t3b,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t3c);s.b[2440] = (s.v[116] < 5.0);s.store_scalar(2440, if s.b[2440] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2440]) {s.store_offset_square(99, 223, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_136(
        s: &mut Scratch,
    ) {
        if (((!s.b[1441]) && s.b[2428]) && s.b[2440]) {s.store_offset(100, 223, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(101, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));}
        let (t3d,) = {
    if (((!s.b[1441]) && s.b[2428]) && (!s.b[2440])) {
        (3.0,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, t3d);
        let (t3e,) = {
    if (((!s.b[1441]) && s.b[2428]) && (!s.b[2440])) {
        (0.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t3e);
        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2440])) {s.store_offset(99, 116, (-1.0));s.store_sqrt(100, 99);s.store_mul(101, 99, 100);}
        if ((!s.b[1441]) && s.b[2428]) {s.store_mul(239, 209, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_offset_product3(238, s.ad_value(209), s.ad_value(214), s.ad_value(335), 1.0, 1e-25);}
        s.b[2441] = (s.v[116] < 5.0);s.store_scalar(2441, if s.b[2441] { 1.0 } else { 0.0 });s.b[2442] = (s.v[116] < 3.0);s.store_scalar(2442, if s.b[2442] { 1.0 } else { 0.0 });
        let (t3f,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && s.b[2442]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, t3f);
        let (t40,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && s.b[2442]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t40);
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && s.b[2442]) {s.copy_ad(133, 238);s.copy_ad(131, 239);s.store_scalar(247, 0.5);s.store_scalar(169, 0.0);}
        let (t41,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && (!s.b[2442])) {
        (2.0,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, t41);
        let (t42,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && (!s.b[2442])) {
        (0.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t42);
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && (!s.b[2442])) {s.store_scalar(335, (1.0 / (5.0 - 3.0)));s.store_mul_scale_offset_indices(332, 335, 116, 1.0, (-3.0));s.store_mul3_ad_middle(207, A::square(s.ad_value(332)), 332, A::offset(A::mul(s.ad_value(332), A::scale_offset(s.ad_value(332), 6.0, (-15.0))), 10.0));}
        if ((!s.b[1441]) && s.b[2428]) {s.store_mul(127, 238, 186);s.copy_ad(349, 790);s.store_div_square_rhs(336, 636, 185);s.store_add_scaled_inputs3_indices(334, 85, 1.0, 155, (-1.0), 1436, -1.0);s.store_offset_mul_ad(335, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(332, 335, 782, 0.5, 0.5);s.store_scaled_add(343, 335, 782, 0.5);}
        s.b[2443] = (s.v[343] < 0.0);s.store_scalar(2443, if s.b[2443] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2443]) {s.store_scalar(343, 0.0);s.store_scalar(332, 0.0);}
        if ((!s.b[1441]) && s.b[2428]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 336, 1.0, 337);s.store_sqrt_square_offset(782, 344, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 344, 782, 0.5, 0.5);s.store_scaled_add(344, 344, 782, 0.5);}
        s.b[2444] = (s.v[344] < 0.0);s.store_scalar(2444, if s.b[2444] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2444]) {s.store_scalar(344, 0.0);s.store_scalar(334, 0.0);}
        if ((!s.b[1441]) && s.b[2428]) {s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));s.store_div(335, 790, 344);}
        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((!s.b[1441]) && s.b[2428]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((!s.b[1441]) && s.b[2428]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);s.copy_ad(790, 348);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_137(
        s: &mut Scratch,
    ) {
        if ((!s.b[1441]) && s.b[2428]) {s.store_exp_ad(230, A::mul(s.ad_value(154), A::sub(s.ad_value(1433), s.ad_value(790))));}
        s.b[2445] = (s.v[790] < 0.0);s.store_scalar(2445, if s.b[2445] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2445]) {s.store_scalar(94, 0.0);s.copy_ad(91, 87);}
        let (t43,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2445]) {
        (1.0,)
    } else {
        (s.v[947],)
    }
};
        s.store_scalar(947, t43);s.b[2446] = (s.v[947] == 0.0);s.store_scalar(2446, if s.b[2446] { 1.0 } else { 0.0 });s.b[2447] = (s.v[77] == 0.0);s.store_scalar(2447, if s.b[2447] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) {
            if ((s.v[92] - s.v[87]) >= 0.0) {
                s.store_sub(96, 92, 87);
            } else {
                s.store_scalar(96, 0.0);
            }
        }
        s.b[2448] = (((1.0 + 0.3) * s.v[96]) > 0.03);s.store_scalar(2448, if s.b[2448] { 1.0 } else { 0.0 });
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {s.store_offset_sub_scaled_inputs_indices(781, 96, (1.0 + 0.3), 790, 1.0, (-0.03));s.store_scale(782, 96, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(95, 96, (1.0 + 0.3), 781, (-0.5), 782, (-0.5));}
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && (!s.b[2448])) {s.store_scale(95, 96, (1.0 + 0.3));}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) {
            if (s.v[95] <= s.v[96]) {
            } else {
                s.copy_ad(95, 96);
            }
        }
        s.b[2449] = (s.v[95] < 0.0);s.store_scalar(2449, if s.b[2449] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2449]) {s.store_scalar(95, 0.0);}
        s.b[2450] = (s.v[95] > s.v[790]);s.store_scalar(2450, if s.b[2450] { 1.0 } else { 0.0 });
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && (!s.b[2449])) && s.b[2450]) {s.copy_ad(95, 790);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2446]) {s.copy_ad(94, 95);s.store_add(91, 87, 94);}
        let (t44,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2446]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t44);
        let (t45,) = {
    if (((!s.b[1441]) && s.b[2428]) && (s.v[947] != 0.0)) {
        (0.0,)
    } else {
        (s.v[947],)
    }
};
        s.store_scalar(947, t45);
        let (t46,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.store_scalar(98, t46);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_138(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t50: usize = 0;
        while {
            let t4e: f64 = (40.0 + 1.0);let t4f: f64 = if (((!s.b[1441]) && s.b[2428]) && (s.v[98] <= t4e)) { 1.0 } else { 0.0 };
            t4f != 0.0
        } {
            t50 += 1;
            if t50 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t50, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((!s.b[1441]) && s.b[2428]) {s.store_mul_sub_rhs(116, 154, 91, 1433);}
            s.b[2451] = (s.v[116] < 5.0);s.store_scalar(2451, if s.b[2451] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2451]) {s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul(222, 210, 230);s.store_mul3_lhs(218, 222, 225, 225);s.store_mul_product3_indices(219, 226, 222, 154, 225, 2.0);s.store_mul_scale_offset_mixed_ia(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_square_add(220, 223, 218);s.store_div_scaled_inputs2_mixed_aii(221, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 219, 1.0, 220, 2.0);}
            if (((!s.b[1441]) && s.b[2428]) && (!s.b[2451])) {s.store_mul_sub_rhs(117, 154, 91, 790);s.store_exp(228, 117);s.store_mul_sub_rhs(218, 210, 228, 230);s.store_mul3_lhs(219, 210, 154, 228);s.store_offset(102, 116, (-1.0));s.store_sqrt_add(220, 102, 218);s.store_div_scaled_inputs2_indices(221, 154, 1.0, 219, 1.0, 220, 2.0);}
            if ((!s.b[1441]) && s.b[2428]) {s.store_add_scaled_inputs_product_indices(234, 85, 1.0, 91, (-1.0), 212, 220, (-1.0));s.store_sub_from_scalar_scaled_mul(235, (-1.0), 212, 221, 1.0);}
            s.b[2452] = (s.v[79] == 1.0);s.store_scalar(2452, if s.b[2452] { 1.0 } else { 0.0 });
            let (t47,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2452]) {
        (1.0,)
    } else {
        (s.v[945],)
    }
};
            s.store_scalar(945, t47);s.b[2453] = (s.v[945] == 0.0);s.store_scalar(2453, if s.b[2453] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {s.store_div_scaled_inputs_indices(237, 234, -1.0, 235, 1.0);}
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[91]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(91))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2454] = (((s.v[237]) as f64).abs() > s.v[93]);s.store_scalar(2454, if s.b[2454] { 1.0 } else { 0.0 });
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2453]) && s.b[2454]) {s.store_scale(237, 93, (if (s.v[237] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {s.store_add(91, 91, 237);}
            s.b[2455] = ((((s.v[237]) as f64).abs() <= 1e-12) && (((s.v[234]) as f64).abs() <= 1e-8));s.store_scalar(2455, if s.b[2455] { 1.0 } else { 0.0 });
            let (t48,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2453]) && s.b[2455]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t48);
            let (t4a,) = {
    if (((!s.b[1441]) && s.b[2428]) && (s.v[945] != 0.0)) {
        let t49: f64 = (40.0 + 1.0);
        (t49,)
    } else {
        (s.v[98],)
    }
};
            s.store_scalar(98, t4a);
            let (t4b,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (0.0,)
    } else {
        (s.v[945],)
    }
};
            s.store_scalar(945, t4b);
            let (t4d,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        let t4c: f64 = (s.v[98] + 1.0);
        (t4c,)
    } else {
        (s.v[98],)
    }
};
            s.store_scalar(98, t4d);
        }
        let (t52,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        let t51: f64 = (s.v[98] - 1.0);
        (t51,)
    } else {
        (s.v[98],)
    }
};
        s.store_scalar(98, t52);s.b[2457] = (s.v[116] < 5.0);s.store_scalar(2457, if s.b[2457] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2457]) {s.store_offset_square(102, 223, (10.0 * 2.220446049250313e-16));s.store_offset(103, 223, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(104, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_139(
        s: &mut Scratch,
    ) {
        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2457])) {s.store_offset(102, 116, (-1.0));s.store_sqrt(103, 102);s.store_mul(104, 102, 103);}
        if ((!s.b[1441]) && s.b[2428]) {s.store_sub(94, 91, 87);s.copy_ad(790, 349);s.store_div(335, 154, 99);s.store_mul(258, 335, 94);s.store_offset(259, 258, 1.0);s.store_sqrt(260, 259);s.store_mul(261, 260, 259);s.store_mul(262, 261, 259);s.store_div_from_scalar_offset_input(263, 1.0, 260, 1.0);s.store_div_from_scalar_offset_input(264, 1.0, 261, 1.0);s.store_div_from_scalar_offset_input(265, 1.0, 262, 1.0);s.store_div(266, 263, 100);s.store_offset_mul_offset_rhs(335, 258, 258, 3.0, 3.0);s.store_mul3_affine_lhs(267, 100, 264, 0.6666666666666667, 0.0, 335);s.store_offset_mul_offset_rhs_mixed_ia(335, 258, A::mul_offset_rhs(s.ad_value(258), A::mul_offset_rhs(s.ad_value(258), s.ad_value(258), 5.0), 10.0), 10.0, 5.0);s.store_mul_product3_mixed_iaii(268, 335, A::div_from_scalar(4.0, A::scale(s.ad_value(154), 15.0)), 101, 265, 1.0);s.store_sub_mixed_ai(269, A::add_scaled_products(s.ad_value(87), s.ad_value(267), 1.0, s.ad_value(155), s.ad_value(104), 0.6666666666666667), 268);s.store_add_scaled_inputs4_indices(335, 85, 1.0, 155, 1.0, 87, (-(2.0 * 0.5)), 94, (-0.5));s.store_sub(336, 266, 267);s.store_mul(337, 154, 185);s.store_mul(338, 154, 209);s.store_add_scaled_products_indices(250, 337, 335, 1.0, 338, 336, 1.0);s.store_mul(248, 94, 250);}
        s.b[2458] = (s.v[347] == 1.0);s.store_scalar(2458, if s.b[2458] { 1.0 } else { 0.0 });
        let (t53,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2458]) {
        (1.0,)
    } else {
        (s.v[948],)
    }
};
        s.store_scalar(948, t53);s.b[2459] = (s.v[948] == 0.0);s.store_scalar(2459, if s.b[2459] { 1.0 } else { 0.0 });s.b[2460] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2460, if s.b[2460] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) {s.store_scalar(169, 0.0);s.copy_ad(168, 91);}
        s.b[2461] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2461, if s.b[2461] { 1.0 } else { 0.0 });
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t54,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t54);
        let (t55,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t55);
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2462] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2462, if s.b[2462] { 1.0 } else { 0.0 });s.b[2463] = (2.0 == 1.0);s.store_scalar(2463, if s.b[2463] { 1.0 } else { 0.0 });
        let (t56,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t56);s.b[2464] = (2.0 == 2.0);s.store_scalar(2464, if s.b[2464] { 1.0 } else { 0.0 });
        let (t57,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) && s.b[2464]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t57);s.b[2465] = (2.0 == 4.0);s.store_scalar(2465, if s.b[2465] { 1.0 } else { 0.0 });
        let (t58,) = {
    if (((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) && (!s.b[2464])) && s.b[2465]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t58);s.b[2466] = (2.0 == 8.0);s.store_scalar(2466, if s.b[2466] { 1.0 } else { 0.0 });
        let (t59,) = {
    if ((((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) && (!s.b[2464])) && (!s.b[2465])) && s.b[2466]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t59);
        let (t5a,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t5a);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_140(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t5e: usize = 0;
        while {
            let t5d: f64 = if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t5d != 0.0
        } {
            t5e += 1;
            if t5e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t5e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) {s.store_sqrt(726, 726);}
            let (t5c,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) {
        let t5b: f64 = (s.v[719] + 1.0);
        (t5b,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t5c);
        }
        if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && (!s.b[2462])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) {
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) {s.store_scalar(334, 1.0);}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) {s.copy_ad(335, 684);s.store_sqrt_sub(342, 91, 1433);s.store_mul(171, 335, 342);s.store_div_scaled_inputs_indices(343, 335, 0.5, 342, 1.0);s.store_div_from_scalar(334, 1.0, 171);s.store_mul(335, 238, 334);s.store_scale(336, 335, s.v[509]);s.store_scale(337, 334, s.v[509]);s.store_add_scaled_product_indices(339, 336, 1.0, 508, 166, 1.0);s.store_div_from_scalar(335, 1.0, 339);s.store_scale(338, 335, 1.034943e-10);s.store_scalar(335, (1.0 - s.v[507]));s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 87, s.v[507], 335, 91, 1.0);}
        s.b[2467] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2467, if s.b[2467] { 1.0 } else { 0.0 });
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t5f,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t5f);
        let (t60,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t60);
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2468] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2468, if s.b[2468] { 1.0 } else { 0.0 });s.b[2469] = (2.0 == 1.0);s.store_scalar(2469, if s.b[2469] { 1.0 } else { 0.0 });
        let (t61,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && s.b[2469]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t61);s.b[2470] = (2.0 == 2.0);s.store_scalar(2470, if s.b[2470] { 1.0 } else { 0.0 });
        let (t62,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && (!s.b[2469])) && s.b[2470]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t62);s.b[2471] = (2.0 == 4.0);s.store_scalar(2471, if s.b[2471] { 1.0 } else { 0.0 });
        let (t63,) = {
    if (((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && (!s.b[2469])) && (!s.b[2470])) && s.b[2471]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t63);s.b[2472] = (2.0 == 8.0);s.store_scalar(2472, if s.b[2472] { 1.0 } else { 0.0 });
        let (t64,) = {
    if ((((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && (!s.b[2469])) && (!s.b[2470])) && (!s.b[2471])) && s.b[2472]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t64);
        let (t65,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t65);let mut t69: usize = 0;
        while {
            let t68: f64 = if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t68 != 0.0
        } {
            t69 += 1;
            if t69 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t69, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) {s.store_sqrt(726, 726);}
            let (t67,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) {
        let t66: f64 = (s.v[719] + 1.0);
        (t66,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t67);
        }
        if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && (!s.b[2468])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && (!s.b[2467])) {
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && (!s.b[2467])) {s.store_scalar(334, 1.0);}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) {s.store_sub(340, 168, 91);s.store_mul(337, 154, 238);s.store_div_from_scalar(335, 1.0, 337);s.store_mul_ad_product_lhs_mixed_ai(339, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), 250, 335);s.store_mul(336, 339, 154);s.store_scale(344, 166, 9662367879.197212);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_141(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) {s.store_scalar(335, 100000.0);s.store_div_from_scalar(336, 1.0, 162);s.store_mul_mixed_ai(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);s.store_mul(337, 336, 338);s.store_mul(341, 345, 338);s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));s.store_mul3_affine_lhs(335, 344, 338, 8.0, 0.0, 338);s.store_scaled_mul(336, 345, 338, 2.0);s.store_mul3_lhs(342, 345, 338, 338);s.store_sqrt_square_add(343, 341, 342);s.store_scaled_sub(169, 343, 341, 0.5);s.copy_ad(335, 169);s.store_mul(169, 208, 335);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {s.store_scale(169, 169, s.v[619]);s.store_add(335, 85, 155);s.store_add_scaled_product_indices(336, 269, (-1.0), 335, 267, 1.0);s.store_mul_mixed_ia(240, 209, A::add_scaled_products(s.ad_value(209), A::add_scaled_sub_value_product(1.5, A::offset(s.ad_value(99), 1.0), 1.0, s.ad_value(154), s.ad_value(94), (-0.5)), 1.0, s.ad_value(185), s.ad_value(336), 1.0));s.copy_ad(335, 154);s.store_div_scaled_product_indices(131, 335, 240, 1.0, 250, 1.0);s.store_scale(335, 212, 2.0);s.store_mul_sub_rhs(241, 335, 267, 100);s.store_scaled_sub(336, 267, 100, 2.0);s.store_add(126, 94, 241);s.store_div_from_scalar(335, 1.0, 127);s.store_mul(336, 126, 335);s.store_sub_from_scalar(337, 1.0, 336);s.store_sub_from_scalar(332, 1.0, 337);s.store_square(722, 332);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t6a,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6a);
        let (t6b,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6b);
        if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2473] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2473, if s.b[2473] { 1.0 } else { 0.0 });s.b[2474] = (4.0 == 1.0);s.store_scalar(2474, if s.b[2474] { 1.0 } else { 0.0 });
        let (t6c,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && s.b[2474]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6c);s.b[2475] = (4.0 == 2.0);s.store_scalar(2475, if s.b[2475] { 1.0 } else { 0.0 });
        let (t6d,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && (!s.b[2474])) && s.b[2475]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6d);s.b[2476] = (4.0 == 4.0);s.store_scalar(2476, if s.b[2476] { 1.0 } else { 0.0 });
        let (t6e,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && (!s.b[2474])) && (!s.b[2475])) && s.b[2476]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6e);s.b[2477] = (4.0 == 8.0);s.store_scalar(2477, if s.b[2477] { 1.0 } else { 0.0 });
        let (t6f,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && (!s.b[2474])) && (!s.b[2475])) && (!s.b[2476])) && s.b[2477]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6f);
        let (t70,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t70);let mut t74: usize = 0;
        while {
            let t73: f64 = if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t73 != 0.0
        } {
            t74 += 1;
            if t74 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t74, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) {s.store_sqrt(726, 726);}
            let (t72,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) {
        let t71: f64 = (s.v[719] + 1.0);
        (t71,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t72);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_142(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2473])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(333, 332, 726, 1.0);s.store_div_scaled_product_indices(338, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(125, 1.0, 333);s.store_offset_mul_offset_rhs(242, 125, 125, 1.0, 1.0);}
        s.b[2478] = (((1.0 + s.v[125]) < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2478, if s.b[2478] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {s.store_sub_from_scalar_ad(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), A::offset(s.ad_value(125), 1.0));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t75,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t75);
        let (t76,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t76);
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2479] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2479, if s.b[2479] { 1.0 } else { 0.0 });s.b[2480] = (2.0 == 1.0);s.store_scalar(2480, if s.b[2480] { 1.0 } else { 0.0 });
        let (t77,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && s.b[2480]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t77);s.b[2481] = (2.0 == 2.0);s.store_scalar(2481, if s.b[2481] { 1.0 } else { 0.0 });
        let (t78,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && (!s.b[2480])) && s.b[2481]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t78);s.b[2482] = (2.0 == 4.0);s.store_scalar(2482, if s.b[2482] { 1.0 } else { 0.0 });
        let (t79,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && (!s.b[2480])) && (!s.b[2481])) && s.b[2482]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t79);s.b[2483] = (2.0 == 8.0);s.store_scalar(2483, if s.b[2483] { 1.0 } else { 0.0 });
        let (t7a,) = {
    if (((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && (!s.b[2480])) && (!s.b[2481])) && (!s.b[2482])) && s.b[2483]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7a);
        let (t7b,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t7b);let mut t7f: usize = 0;
        while {
            let t7e: f64 = if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t7e != 0.0
        } {
            t7f += 1;
            if t7f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t7f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) {s.store_sqrt(726, 726);}
            let (t7d,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) {
        let t7c: f64 = (s.v[719] + 1.0);
        (t7c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t7d);
        }
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && (!s.b[2479])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(243, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
        }
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2478])) {s.store_offset(243, 125, 1.0);s.store_scalar(334, 1.0);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {s.store_div_scaled_product_indices(335, 127, 242, 0.6666666666666667, 243, 1.0);s.store_mul(133, 335, 185);s.store_offset(244, 125, 0.5);s.store_mul(245, 243, 242);s.store_div_scaled_inputs_indices(246, 244, 0.4, 245, 1.0);s.store_sub_from_scalar(247, 0.6, 246);}
        s.b[2484] = (s.v[247] > 0.5);s.store_scalar(2484, if s.b[2484] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2484]) {s.store_scalar(247, 0.5);}
        s.b[2485] = (s.v[347] == 2.0);s.store_scalar(2485, if s.b[2485] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) {s.copy_ad(335, 131);s.store_add_scaled_product_mixed_aii(131, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(239)), 1.0, 207, 131, 1.0);}
        s.b[2486] = (s.v[131] < 0.0);s.store_scalar(2486, if s.b[2486] { 1.0 } else { 0.0 });
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) && s.b[2486]) {s.store_scalar(131, 0.0);}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) {s.copy_ad(335, 133);s.store_add_scaled_product_mixed_aii(133, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(238)), 1.0, 207, 133, 1.0);}
        s.b[2487] = (s.v[133] < 0.0);s.store_scalar(2487, if s.b[2487] { 1.0 } else { 0.0 });
        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) && s.b[2487]) {s.store_scalar(133, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_143(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) {s.copy_ad(335, 247);s.store_add_scaled_product_mixed_aii(247, A::scale_offset(s.ad_value(207), (-0.5), 0.5), 1.0, 207, 247, 1.0);s.copy_ad(335, 169);s.store_mul(169, 207, 169);}
        let (t80,) = {
    if (((!s.b[1441]) && s.b[2428]) && (s.v[948] != 0.0)) {
        (0.0,)
    } else {
        (s.v[948],)
    }
};
        s.store_scalar(948, t80);
        if ((!s.b[1441]) && s.b[2428]) {s.store_sub(170, 162, 169);}
        s.b[2488] = (s.v[170] < 1e-9);s.store_scalar(2488, if s.b[2488] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2488]) {s.store_scalar(170, 1e-9);}
        if ((!s.b[1441]) && s.b[2428]) {s.store_scalar(335, (s.v[625] / 100.0));s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_mul_ad(338, A::sub(s.ad_value(91), s.ad_value(87)), s.ad_value(334), 1.0);s.store_add_scaled_products_indices(339, 335, 131, 1.0, 336, 133, 1.0);s.store_div(337, 339, 338);s.store_mul_scale_offset_rhs(251, 337, 1436, p[166], 1.0);}
        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[160] - 1.0));
            }
        }
        if ((!s.b[1441]) && s.b[2428]) {s.store_mul(342, 339, 251);}
        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if ((!s.b[1441]) && s.b[2428]) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), s.v[474])), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_square(337, 335);s.store_mul_scale_offset_indices(338, 337, 154, -1.0, 0.0);s.store_mul(339, 338, 170);s.store_mul_scale_offset_indices(340, 338, 238, 1.0, 1e-25);s.store_mul_ad_product_lhs_mixed_ai(333, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), 250, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_div_scaled_inputs_indices(337, 336, -1.0, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2489] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2489, if s.b[2489] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2489]) {s.store_scalar(337, 1.0);}
        s.b[2490] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2490, if s.b[2490] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2489])) && s.b[2490]) {s.copy_ad(337, 335);}
        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2489])) && (!s.b[2490])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p[178] - 1.0));
            }
        }
        if ((!s.b[1441]) && s.b[2428]) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[2491] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2491, if s.b[2491] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2491]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2492] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2492, if s.b[2492] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2491])) && s.b[2492]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2491])) && (!s.b[2492])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p[178]) - 1.0));
            }
        }
        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2491])) && (!s.b[2492])) {s.store_mul(339, 338, 340);}
        if ((!s.b[1441]) && s.b[2428]) {s.store_mul(253, 254, 339);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_div_scaled_inputs_indices(335, 115, -1.0, 170, 1.0);s.store_mul3_lhs(135, 115, 248, 253);}
        s.b[2493] = (p[283] != 0.0);s.store_scalar(2493, if s.b[2493] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2493]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));}
    }
}
