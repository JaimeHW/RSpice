#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_99(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2356] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2356, if s.b[2356] { 1.0 } else { 0.0 });s.b[2357] = (2.0 == 1.0);s.store_scalar(2357, if s.b[2357] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && s.b[2357]) {s.store_scalar(720, 1.0);}
        s.b[2358] = (2.0 == 2.0);s.store_scalar(2358, if s.b[2358] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && s.b[2358]) {s.store_scalar(720, 2.0);}
        s.b[2359] = (2.0 == 4.0);s.store_scalar(2359, if s.b[2359] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && (!s.b[2358])) && s.b[2359]) {s.store_scalar(720, 3.0);}
        s.b[2360] = (2.0 == 8.0);s.store_scalar(2360, if s.b[2360] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && (!s.b[2358])) && (!s.b[2359])) && s.b[2360]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2362]) {s.store_scale(2170, 2117, p[399]);s.store_offset(983, 2170, (-1.0));s.copy_ad(2324, 2325);s.copy_ad(2146, 2325);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2362])) {s.store_offset_scaled(2170, 2117, p[399], (-0.1));s.copy_ad(983, 87);s.copy_ad(2324, 2145);s.copy_ad(2146, 2145);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_scalar(79, 0.0);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_100(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t3: usize = 0;
        while {
            let t2: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2363] = (s.v[983] >= 0.0);s.store_scalar(2363, if s.b[2363] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2363]) {s.store_mul_scaled_sqrt_ad_rhs(2322, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2125, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2322, 1.0);}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2363])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2170)));s.store_exp_mul(338, 154, 2170);s.store_mul_sqrt_mixed_ia(2322, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2322, 1.0);s.store_mul_add_mixed_iaa(2125, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2100, 2322, 1.0, 185, 2324, 983, 1.0);s.store_sub(2101, 2125, 185);s.store_div_scaled_inputs_indices(2112, 2100, -1.0, 2101, 1.0);}
            s.b[2364] = (((s.v[2112]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2364, if s.b[2364] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && s.b[2364]) {s.store_scalar(79, 1.0);}
            s.b[2365] = (s.v[2112] > 0.1);s.store_scalar(2365, if s.b[2365] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2364])) && s.b[2365]) {s.store_scalar(2112, 0.1);}
            s.b[2366] = (s.v[2112] < (-0.1));s.store_scalar(2366, if s.b[2366] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2364])) && (!s.b[2365])) && s.b[2366]) {s.store_scalar(2112, (-0.1));}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 2112);}
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_primal_offset(97, 97, 1.0);}
        }
        s.b[2368] = (2.0 == 1.0);s.store_scalar(2368, if s.b[2368] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2368]) {s.copy_ad(2171, 983);}
        s.b[2369] = ((s.v[983] < (s.v[2171] + 0.2)) && (0.2 >= 0.0));s.store_scalar(2369, if s.b[2369] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {s.store_sub_offset_lhs(781, 2171, 0.2, 983);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2370] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2370, if s.b[2370] { 1.0 } else { 0.0 });s.b[2371] = (2.0 == 1.0);s.store_scalar(2371, if s.b[2371] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && s.b[2371]) {s.store_scalar(720, 1.0);}
        s.b[2372] = (2.0 == 2.0);s.store_scalar(2372, if s.b[2372] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && s.b[2372]) {s.store_scalar(720, 2.0);}
        s.b[2373] = (2.0 == 4.0);s.store_scalar(2373, if s.b[2373] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && (!s.b[2372])) && s.b[2373]) {s.store_scalar(720, 3.0);}
        s.b[2374] = (2.0 == 8.0);s.store_scalar(2374, if s.b[2374] { 1.0 } else { 0.0 });
        if (((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && (!s.b[2372])) && (!s.b[2373])) && s.b[2374]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_101(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t5: usize = 0;
        while {
            let t4: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {s.store_add(781, 2153, 2142);s.store_square(722, 781);s.store_square(723, 2142);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && (s.v[719] < s.v[2143])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2376] = ((((s.v[2143] == 1.0) || (s.v[2143] == 2.0)) || (s.v[2143] == 4.0)) || (s.v[2143] == 8.0));s.store_scalar(2376, if s.b[2376] { 1.0 } else { 0.0 });s.b[2377] = (s.v[2143] == 1.0);s.store_scalar(2377, if s.b[2377] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && s.b[2377]) {s.store_scalar(720, 1.0);}
        s.b[2378] = (s.v[2143] == 2.0);s.store_scalar(2378, if s.b[2378] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && s.b[2378]) {s.store_scalar(720, 2.0);}
        s.b[2379] = (s.v[2143] == 4.0);s.store_scalar(2379, if s.b[2379] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && (!s.b[2378])) && s.b[2379]) {s.store_scalar(720, 3.0);}
        s.b[2380] = (s.v[2143] == 8.0);s.store_scalar(2380, if s.b[2380] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && (!s.b[2378])) && (!s.b[2379])) && s.b[2380]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) {s.store_scalar(719, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_102(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2381])) {s.store_mul3_ad_middle(2321, A::square(s.ad_value(2152)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_mul(332, 2321, 983);}
        s.b[2382] = (s.v[332] > 500.0);s.store_scalar(2382, if s.b[2382] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2382]) {s.store_sub(2165, 983, 2320);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) {s.store_exp_mul_scaled_lhs_indices(334, 2321, -1.0, 2320);}
        s.b[2383] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2383, if s.b[2383] { 1.0 } else { 0.0 });s.b[2384] = (s.v[332] >= 500.0);s.store_scalar(2384, if s.b[2384] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && s.b[2384]) {s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(337, 1.403592217853e217);}
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) {s.copy_ad(781, 332);s.store_scalar(335, 1.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
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
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2386])) && s.b[2387]) {s.store_mul(337, 154, 336);s.store_neg_ad(2147, A::sqrt(A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2386])) && (!s.b[2387])) {s.store_mul_scale_offset_indices(337, 336, 154, -1.0, 0.0);s.store_sqrt_ad(2147, A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {s.store_sub(990, 2148, 2147);}
        s.b[2388] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));s.store_scalar(2388, if s.b[2388] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {s.store_sub_from_scalar(781, 1e-16, 990);s.store_square(722, 781);s.store_scalar(723, (1e-16 * 1e-16));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_103(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2389] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2389, if s.b[2389] { 1.0 } else { 0.0 });s.b[2390] = (2.0 == 1.0);s.store_scalar(2390, if s.b[2390] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && s.b[2390]) {s.store_scalar(720, 1.0);}
        s.b[2391] = (2.0 == 2.0);s.store_scalar(2391, if s.b[2391] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && s.b[2391]) {s.store_scalar(720, 2.0);}
        s.b[2392] = (2.0 == 4.0);s.store_scalar(2392, if s.b[2392] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && (!s.b[2391])) && s.b[2392]) {s.store_scalar(720, 3.0);}
        s.b[2393] = (2.0 == 8.0);s.store_scalar(2393, if s.b[2393] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && (!s.b[2391])) && (!s.b[2392])) && s.b[2393]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) {s.store_scalar(719, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2397] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2397, if s.b[2397] { 1.0 } else { 0.0 });s.b[2398] = (2.0 == 1.0);s.store_scalar(2398, if s.b[2398] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && s.b[2398]) {s.store_scalar(720, 1.0);}
        s.b[2399] = (2.0 == 2.0);s.store_scalar(2399, if s.b[2399] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && s.b[2399]) {s.store_scalar(720, 2.0);}
        s.b[2400] = (2.0 == 4.0);s.store_scalar(2400, if s.b[2400] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && (!s.b[2399])) && s.b[2400]) {s.store_scalar(720, 3.0);}
        s.b[2401] = (2.0 == 8.0);s.store_scalar(2401, if s.b[2401] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && (!s.b[2399])) && (!s.b[2400])) && s.b[2401]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;
            if tf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) {s.store_sqrt(337, 338);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_104(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) {s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2134, 1.0, 337);}
        s.b[2402] = ((s.v[344] < (s.v[972] + p[405])) && (p[405] >= 0.0));s.store_scalar(2402, if s.b[2402] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {s.store_sub_offset_lhs(781, 972, p[405], 344);s.store_square(722, 781);s.store_scalar(723, (p[405] * p[405]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2403] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2403, if s.b[2403] { 1.0 } else { 0.0 });s.b[2404] = (2.0 == 1.0);s.store_scalar(2404, if s.b[2404] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && s.b[2404]) {s.store_scalar(720, 1.0);}
        s.b[2405] = (2.0 == 2.0);s.store_scalar(2405, if s.b[2405] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && s.b[2405]) {s.store_scalar(720, 2.0);}
        s.b[2406] = (2.0 == 4.0);s.store_scalar(2406, if s.b[2406] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && (!s.b[2405])) && s.b[2406]) {s.store_scalar(720, 3.0);}
        s.b[2407] = (2.0 == 8.0);s.store_scalar(2407, if s.b[2407] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && (!s.b[2405])) && (!s.b[2406])) && s.b[2407]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;
            if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.store_add_mul_sub_from_scalar_rhs_indices(2160, 2159, 2134, 1.0, 337);s.copy_ad(2156, 2160);s.store_scalar(79, 0.0);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_105(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t13: usize = 0;
        while {
            let t12: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;
            if t13 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t13, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.store_mul_scale_offset_indices(335, 2156, 154, -1.0, 0.0);s.store_exp(336, 335);s.store_sqrt_div_scaled_inputs(338, 2114, 2.0, 154, 1.0);s.store_offset_sub(344, 336, 335, (-1.0));s.store_mul_sqrt_mixed_ia(2157, 338, A::offset(s.ad_value(344), 1e-15));}
            s.b[2408] = (s.v[335] > 0.0);s.store_scalar(2408, if s.b[2408] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && s.b[2408]) {s.store_neg(2157, 2157);}
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2157, 1.0);s.store_mul_scale_offset_indices(2158, 345, 336, -1.0, 1.0);}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2100, 2157, 1.0, 185, 2159, 2156, -1.0);s.store_add(2101, 185, 2158);s.store_div_scaled_inputs_indices(2112, 2100, -1.0, 2101, 1.0);}
            s.b[2409] = (((s.v[2112]) as f64).abs() < 1e-10);s.store_scalar(2409, if s.b[2409] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && s.b[2409]) {s.store_scalar(79, 1.0);}
            s.b[2410] = (s.v[2112] > 0.1);s.store_scalar(2410, if s.b[2410] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && (!s.b[2409])) && s.b[2410]) {s.store_scalar(2112, 0.1);}
            s.b[2411] = (s.v[2112] < (-0.1));s.store_scalar(2411, if s.b[2411] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && (!s.b[2409])) && (!s.b[2410])) && s.b[2411]) {s.store_scalar(2112, (-0.1));}
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) {s.store_add(2156, 2156, 2112);}
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {s.store_primal_offset(97, 97, 1.0);}
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
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {s.store_add_scaled_inputs3_indices(781, 349, 1.0, 972, (-1.0), 972, 0.5);s.store_square(722, 781);s.store_scaled_mul(723, 972, 972, (0.5 * 0.5));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2414] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2414, if s.b[2414] { 1.0 } else { 0.0 });s.b[2415] = (2.0 == 1.0);s.store_scalar(2415, if s.b[2415] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && s.b[2415]) {s.store_scalar(720, 1.0);}
        s.b[2416] = (2.0 == 2.0);s.store_scalar(2416, if s.b[2416] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && s.b[2416]) {s.store_scalar(720, 2.0);}
        s.b[2417] = (2.0 == 4.0);s.store_scalar(2417, if s.b[2417] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && (!s.b[2416])) && s.b[2417]) {s.store_scalar(720, 3.0);}
        s.b[2418] = (2.0 == 8.0);s.store_scalar(2418, if s.b[2418] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && (!s.b[2416])) && (!s.b[2417])) && s.b[2418]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) {s.store_scalar(719, 0.0);}
        let mut t15: usize = 0;
        while {
            let t14: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && (!s.b[2414])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {s.store_div_from_scalar(726, 1.0, 726);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_106(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {s.store_mul3_affine_lhs(780, 781, 972, 0.5, 0.0, 726);s.store_div_scaled_product3_indices(334, 972, 725, 726, 0.5, 770, 1.0);s.store_add_scaled_inputs3_indices(2166, 972, 1.0, 972, (-0.5), 780, 1.0);}
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
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_mul(2124, 990, 2133);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 2124, 343);s.store_offset_sqrt_ad(2168, A::offset(A::square(s.ad_value(989)), p[262]), (-((p[262]) as f64).sqrt()));s.store_offset_mul(338, 2168, 688, 1.0);s.store_offset_mul(339, 2168, 689, 1.0);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_107(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p[378]));
            }
        }
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_div(985, 254, 338);s.store_mul_scale_offset_mixed_ia(2132, 964, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2115), 1.0, A::div_scalar_offset_denominator(1.0, A::div_scaled_product(s.ad_value(254), s.ad_value(2115), 1.0, s.ad_value(973), 1.0), 1.0, 1.0), p[400]), 1.0, 1.0);s.store_scaled_mul(335, 990, 2132, 1.6021918e-19);s.store_scale_ad(336, A::pow(A::div_from_scalar(s.v[163], s.ad_value(162)), s.ad_value(976)), p[7]);s.store_mul3_affine_lhs(987, 335, 985, s.v[632], 0.0, 2115);s.store_mul3_affine_lhs(988, 336, 2155, p[363], 0.0, 2167);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_mul3_lhs(986, 115, 248, 984);s.store_add_scaled_inputs3_indices(135, 986, 1.0, 987, 1.0, 988, 1.0);s.copy_ad(790, 349);}
        s.b[2423] = (p[283] != 0.0);s.store_scalar(2423, if s.b[2423] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2423]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(2089), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[2424] = (s.v[336] < 0.0);s.store_scalar(2424, if s.b[2424] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2423]) && s.b[2424]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2423]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p[284]);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1439, p[285], 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 2089, 1.0, 340, 1.0, 1438, -1.0);s.store_add_product3_rhs_indices(338, 338, 1439, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2423])) {s.store_scalar(343, 0.0);}
        s.b[2425] = (p[287] != 0.0);s.store_scalar(2425, if s.b[2425] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2425]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1439);}
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2425])) {s.store_scalar(342, 0.0);}
        s.b[2426] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(2426, if s.b[2426] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2426]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);}
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.copy_ad(134, 135);s.store_add_scaled_inputs4_indices(131, 2098, (-0.5), 2122, ((-1.0) * (-0.5)), 2099, (-0.5), 2123, (-(-0.5)));s.store_scaled_add(133, 2122, 2123, (-0.5));s.store_scalar(247, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_108(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {s.store_scaled_add(978, 2122, 2123, (-0.5));s.store_neg(238, 2122);s.copy_ad(255, 2116);}
        s.b[2427] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(2427, if s.b[2427] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2427]) {s.store_scalar(78, 1.0);}
        s.b[2428] = (s.v[791] < s.v[86]);s.store_scalar(2428, if s.b[2428] { 1.0 } else { 0.0 });
        if ((!s.b[1443]) && s.b[2428]) {s.store_scalar(347, (-1.0));s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_sub_rhs(332, 154, 85, 1435);s.store_div_scalar_by_product_indices(335, 1.0, 154, 209, 1.0);s.store_mul(333, 335, 185);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_offset(338, 332, (-2.0));s.store_scaled_mul(339, 333, 338, 9.0);s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);s.store_square(276, 278);}
        s.b[2429] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2429, if s.b[2429] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2428]) && s.b[2429]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((!s.b[1443]) && s.b[2428]) && (!s.b[2429])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((!s.b[1443]) && s.b[2428]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }
        if ((!s.b[1443]) && s.b[2428]) {s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div_from_scalar(335, 1.0, 273);s.store_mul(116, 272, 335);s.store_add_scaled_product_indices(167, 1435, 1.0, 116, 155, 1.0);s.store_sub(335, 167, 1435);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_add_div_lhs_indices(87, 335, 337, 1435);s.copy_ad(91, 87);s.store_scalar(94, 0.0);s.store_sub(336, 85, 87);s.store_mul(131, 185, 336);s.store_scalar(133, 0.0);s.store_scalar(247, 0.0);s.store_scalar(169, 0.0);s.store_scalar(134, 0.0);s.store_scalar(127, 0.0);s.store_scalar(78, 1.0);s.store_scalar(946, 1.0);}
        s.b[2430] = (s.v[946] == 0.0);s.store_scalar(2430, if s.b[2430] { 1.0 } else { 0.0 });
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1435))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_add_product3_rhs_mixed_iia(89, 85, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);}
        s.b[2431] = (s.v[77] == 0.0);s.store_scalar(2431, if s.b[2431] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2431]) {s.store_mul_sub_rhs(116, 154, 89, 1435);}
        s.b[2432] = (s.v[116] < 3.0);s.store_scalar(2432, if s.b[2432] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {s.store_mul_sub_rhs(333, 154, 85, 1435);s.store_div_scalar_by_product_indices(335, 1.0, 154, 212, (1.414213562373095 / 108.0));s.store_offset_scaled(336, 335, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);s.store_square(338, 338);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_109(
        s: &mut ReactiveScratch,
    ) {
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {s.store_add_scaled_inputs_mixed_ai(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 1.0, 339, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(89, 1435, 1.0, 332, 155, 1.0);s.copy_ad(88, 89);}
        s.b[2433] = (s.v[791] <= s.v[118]);s.store_scalar(2433, if s.b[2433] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && s.b[2433]) {s.copy_ad(88, 89);}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {s.store_div_scalar_by_product_indices(335, 1.0, 210, 211, 1.0);s.store_mul3_lhs(336, 335, 85, 85);s.store_add_div_from_scalar_rhs(337, 154, 2.0, 85);s.store_div_ln_lhs(90, 336, 337);s.store_offset_sub(781, 90, 89, (-0.0008));s.store_scale(782, 90, (4.0 * 0.0008));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset(332, 1435, (1e-12 / 2.0));}
        s.b[2434] = (s.v[88] < s.v[332]);s.store_scalar(2434, if s.b[2434] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2434]) {s.copy_ad(88, 332);}
        if ((!s.b[1443]) && s.b[2430]) {s.copy_ad(87, 88);s.copy_ad(92, 89);s.store_exp_mul(229, 154, 1435);s.store_mul(222, 210, 229);s.store_scalar(79, 0.0);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_110(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t18: usize = 0;
        while {
            let t16: f64 = (s.v[421] + 1.0);let t17: f64 = if (((!s.b[1443]) && s.b[2430]) && (s.v[97] <= t16)) { 1.0 } else { 0.0 };
            t17 != 0.0
        } {
            t18 += 1;
            if t18 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t18, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((!s.b[1443]) && s.b[2430]) {s.store_mul_sub_rhs(116, 154, 87, 1435);}
            s.b[2435] = (s.v[116] < 5.0);s.store_scalar(2435, if s.b[2435] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2435]) {s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(214, 222, 225, 225);s.store_mul_product3_indices(215, 226, 222, 154, 225, 2.0);s.store_mul_scale_offset_mixed_ia(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_inputs2_mixed_aii(217, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 215, 1.0, 216, 2.0);}
            s.b[2436] = (s.v[116] < 60.0);s.store_scalar(2436, if s.b[2436] { 1.0 } else { 0.0 });
            if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) && s.b[2436]) {s.store_exp(227, 116);s.store_mul_scale_offset_indices(214, 222, 227, 1.0, (-1.0));s.store_mul3_lhs(215, 222, 154, 227);}
            if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) && (!s.b[2436])) {s.store_exp_mul(231, 154, 87);s.store_mul_sub_rhs(214, 210, 231, 229);s.store_mul3_lhs(215, 210, 154, 231);}
            if (((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) {s.store_sqrt_add_ad(216, A::offset(s.ad_value(116), (-1.0)), s.ad_value(214));s.store_div_scaled_inputs2_indices(217, 154, 1.0, 215, 1.0, 216, 2.0);}
            if ((!s.b[1443]) && s.b[2430]) {s.store_add_scaled_inputs_product_indices(232, 85, 1.0, 87, (-1.0), 212, 216, (-1.0));s.store_sub_from_scalar_scaled_mul(233, (-1.0), 212, 217, 1.0);}
            s.b[2437] = (s.v[79] == 1.0);s.store_scalar(2437, if s.b[2437] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2437]) {s.store_scalar(944, 1.0);}
            s.b[2438] = (s.v[944] == 0.0);s.store_scalar(2438, if s.b[2438] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[87]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(87))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2439] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2439, if s.b[2439] { 1.0 } else { 0.0 });
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2438]) && s.b[2439]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {s.store_add(87, 87, 236);}
            s.b[2440] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2440, if s.b[2440] { 1.0 } else { 0.0 });
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2438]) && s.b[2440]) {s.store_scalar(79, 1.0);}
            if (((!s.b[1443]) && s.b[2430]) && (s.v[944] != 0.0)) {s.store_scalar(97, (s.v[421] + 1.0));}
            if ((!s.b[1443]) && s.b[2430]) {s.store_scalar(944, 0.0);s.store_primal_offset(97, 97, 1.0);}
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_primal_offset(97, 97, (-1.0));}
        s.b[2442] = (s.v[116] < 5.0);s.store_scalar(2442, if s.b[2442] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2442]) {s.store_offset_square(99, 223, (10.0 * 2.220446049250313e-16));s.store_offset(100, 223, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(101, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2442])) {s.store_scalar(347, 3.0);s.store_scalar(78, 0.0);s.store_offset(99, 116, (-1.0));s.store_sqrt(100, 99);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_111(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2442])) {s.store_mul(101, 99, 100);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(239, 209, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_offset_product3(238, s.ad_value(209), s.ad_value(214), s.ad_value(335), 1.0, 1e-25);}
        s.b[2443] = (s.v[116] < 5.0);s.store_scalar(2443, if s.b[2443] { 1.0 } else { 0.0 });s.b[2444] = (s.v[116] < 3.0);s.store_scalar(2444, if s.b[2444] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && s.b[2444]) {s.store_scalar(347, 1.0);s.store_scalar(78, 1.0);s.copy_ad(133, 238);s.copy_ad(131, 239);s.store_scalar(247, 0.5);s.store_scalar(169, 0.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && (!s.b[2444])) {s.store_scalar(347, 2.0);s.store_scalar(78, 0.0);s.store_scalar(335, (1.0 / (5.0 - 3.0)));s.store_mul_scale_offset_indices(332, 335, 116, 1.0, (-3.0));s.store_mul3_ad_middle(207, A::square(s.ad_value(332)), 332, A::offset(A::mul(s.ad_value(332), A::scale_offset(s.ad_value(332), 6.0, (-15.0))), 10.0));}
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(127, 238, 186);s.copy_ad(349, 790);s.store_div_square_rhs(336, 636, 185);s.store_add_scaled_inputs3_indices(334, 85, 1.0, 155, (-1.0), 1438, -1.0);s.store_offset_mul_ad(335, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(332, 335, 782, 0.5, 0.5);s.store_scaled_add(343, 335, 782, 0.5);}
        s.b[2445] = (s.v[343] < 0.0);s.store_scalar(2445, if s.b[2445] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2445]) {s.store_scalar(343, 0.0);s.store_scalar(332, 0.0);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 336, 1.0, 337);s.store_sqrt_square_offset(782, 344, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 344, 782, 0.5, 0.5);s.store_scaled_add(344, 344, 782, 0.5);}
        s.b[2446] = (s.v[344] < 0.0);s.store_scalar(2446, if s.b[2446] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2446]) {s.store_scalar(344, 0.0);s.store_scalar(334, 0.0);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));s.store_div(335, 790, 344);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);s.copy_ad(790, 348);s.store_exp_ad(230, A::mul(s.ad_value(154), A::sub(s.ad_value(1435), s.ad_value(790))));}
        s.b[2447] = (s.v[790] < 0.0);s.store_scalar(2447, if s.b[2447] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2447]) {s.store_scalar(94, 0.0);s.copy_ad(91, 87);s.store_scalar(947, 1.0);}
        s.b[2448] = (s.v[947] == 0.0);s.store_scalar(2448, if s.b[2448] { 1.0 } else { 0.0 });s.b[2449] = (s.v[77] == 0.0);s.store_scalar(2449, if s.b[2449] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_112(
        s: &mut ReactiveScratch,
    ) {
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) {
            if ((s.v[92] - s.v[87]) >= 0.0) {
                s.store_sub(96, 92, 87);
            } else {
                s.store_scalar(96, 0.0);
            }
        }
        s.b[2450] = (((1.0 + 0.3) * s.v[96]) > 0.03);s.store_scalar(2450, if s.b[2450] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {s.store_offset_sub_scaled_inputs_indices(781, 96, (1.0 + 0.3), 790, 1.0, (-0.03));s.store_scale(782, 96, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(95, 96, (1.0 + 0.3), 781, (-0.5), 782, (-0.5));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && (!s.b[2450])) {s.store_scale(95, 96, (1.0 + 0.3));}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) {
            if (s.v[95] <= s.v[96]) {
            } else {
                s.copy_ad(95, 96);
            }
        }
        s.b[2451] = (s.v[95] < 0.0);s.store_scalar(2451, if s.b[2451] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2451]) {s.store_scalar(95, 0.0);}
        s.b[2452] = (s.v[95] > s.v[790]);s.store_scalar(2452, if s.b[2452] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && (!s.b[2451])) && s.b[2452]) {s.copy_ad(95, 790);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2448]) {s.copy_ad(94, 95);s.store_add(91, 87, 94);s.store_scalar(79, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && (s.v[947] != 0.0)) {s.store_scalar(947, 0.0);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_scalar(98, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_113(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t1b: usize = 0;
        while {
            let t19: f64 = (40.0 + 1.0);let t1a: f64 = if (((!s.b[1443]) && s.b[2430]) && (s.v[98] <= t19)) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;
            if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((!s.b[1443]) && s.b[2430]) {s.store_mul_sub_rhs(116, 154, 91, 1435);}
            s.b[2453] = (s.v[116] < 5.0);s.store_scalar(2453, if s.b[2453] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2453]) {s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul(222, 210, 230);s.store_mul3_lhs(218, 222, 225, 225);s.store_mul_product3_indices(219, 226, 222, 154, 225, 2.0);s.store_mul_scale_offset_mixed_ia(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_square_add(220, 223, 218);s.store_div_scaled_inputs2_mixed_aii(221, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 219, 1.0, 220, 2.0);}
            if (((!s.b[1443]) && s.b[2430]) && (!s.b[2453])) {s.store_mul_sub_rhs(117, 154, 91, 790);s.store_exp(228, 117);s.store_mul_sub_rhs(218, 210, 228, 230);s.store_mul3_lhs(219, 210, 154, 228);s.store_offset(102, 116, (-1.0));s.store_sqrt_add(220, 102, 218);s.store_div_scaled_inputs2_indices(221, 154, 1.0, 219, 1.0, 220, 2.0);}
            if ((!s.b[1443]) && s.b[2430]) {s.store_add_scaled_inputs_product_indices(234, 85, 1.0, 91, (-1.0), 212, 220, (-1.0));s.store_sub_from_scalar_scaled_mul(235, (-1.0), 212, 221, 1.0);}
            s.b[2454] = (s.v[79] == 1.0);s.store_scalar(2454, if s.b[2454] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2454]) {s.store_scalar(945, 1.0);}
            s.b[2455] = (s.v[945] == 0.0);s.store_scalar(2455, if s.b[2455] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {s.store_div_scaled_inputs_indices(237, 234, -1.0, 235, 1.0);}
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[91]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(91))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2456] = (((s.v[237]) as f64).abs() > s.v[93]);s.store_scalar(2456, if s.b[2456] { 1.0 } else { 0.0 });
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2455]) && s.b[2456]) {s.store_scale(237, 93, (if (s.v[237] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {s.store_add(91, 91, 237);}
            s.b[2457] = ((((s.v[237]) as f64).abs() <= 1e-12) && (((s.v[234]) as f64).abs() <= 1e-8));s.store_scalar(2457, if s.b[2457] { 1.0 } else { 0.0 });
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2455]) && s.b[2457]) {s.store_scalar(79, 1.0);}
            if (((!s.b[1443]) && s.b[2430]) && (s.v[945] != 0.0)) {s.store_scalar(98, (40.0 + 1.0));}
            if ((!s.b[1443]) && s.b[2430]) {s.store_scalar(945, 0.0);s.store_primal_offset(98, 98, 1.0);}
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_primal_offset(98, 98, (-1.0));}
        s.b[2459] = (s.v[116] < 5.0);s.store_scalar(2459, if s.b[2459] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2459]) {s.store_offset_square(102, 223, (10.0 * 2.220446049250313e-16));s.store_offset(103, 223, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(104, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2459])) {s.store_offset(102, 116, (-1.0));s.store_sqrt(103, 102);s.store_mul(104, 102, 103);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_sub(94, 91, 87);s.copy_ad(790, 349);s.store_div(335, 154, 99);s.store_mul(258, 335, 94);s.store_offset(259, 258, 1.0);s.store_sqrt(260, 259);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_114(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(261, 260, 259);s.store_mul(262, 261, 259);s.store_div_from_scalar_offset_input(263, 1.0, 260, 1.0);s.store_div_from_scalar_offset_input(264, 1.0, 261, 1.0);s.store_div_from_scalar_offset_input(265, 1.0, 262, 1.0);s.store_div(266, 263, 100);s.store_offset_mul_offset_rhs(335, 258, 258, 3.0, 3.0);s.store_mul3_affine_lhs(267, 100, 264, 0.6666666666666667, 0.0, 335);s.store_offset_mul_offset_rhs_mixed_ia(335, 258, A::mul_offset_rhs(s.ad_value(258), A::mul_offset_rhs(s.ad_value(258), s.ad_value(258), 5.0), 10.0), 10.0, 5.0);s.store_mul_product3_mixed_iaii(268, 335, A::div_from_scalar(4.0, A::scale(s.ad_value(154), 15.0)), 101, 265, 1.0);s.store_sub_mixed_ai(269, A::add_scaled_products(s.ad_value(87), s.ad_value(267), 1.0, s.ad_value(155), s.ad_value(104), 0.6666666666666667), 268);s.store_add_scaled_inputs4_indices(335, 85, 1.0, 155, 1.0, 87, (-(2.0 * 0.5)), 94, (-0.5));s.store_sub(336, 266, 267);s.store_mul(337, 154, 185);s.store_mul(338, 154, 209);s.store_add_scaled_products_indices(250, 337, 335, 1.0, 338, 336, 1.0);s.store_mul(248, 94, 250);}
        s.b[2460] = (s.v[347] == 1.0);s.store_scalar(2460, if s.b[2460] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2460]) {s.store_scalar(948, 1.0);}
        s.b[2461] = (s.v[948] == 0.0);s.store_scalar(2461, if s.b[2461] { 1.0 } else { 0.0 });s.b[2462] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2462, if s.b[2462] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) {s.store_scalar(169, 0.0);s.copy_ad(168, 91);}
        s.b[2463] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2463, if s.b[2463] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2464] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2464, if s.b[2464] { 1.0 } else { 0.0 });s.b[2465] = (2.0 == 1.0);s.store_scalar(2465, if s.b[2465] { 1.0 } else { 0.0 });
        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && s.b[2465]) {s.store_scalar(720, 1.0);}
        s.b[2466] = (2.0 == 2.0);s.store_scalar(2466, if s.b[2466] { 1.0 } else { 0.0 });
        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && s.b[2466]) {s.store_scalar(720, 2.0);}
        s.b[2467] = (2.0 == 4.0);s.store_scalar(2467, if s.b[2467] { 1.0 } else { 0.0 });
        if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2467]) {s.store_scalar(720, 3.0);}
        s.b[2468] = (2.0 == 8.0);s.store_scalar(2468, if s.b[2468] { 1.0 } else { 0.0 });
        if ((((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2467])) && s.b[2468]) {s.store_scalar(720, 4.0);}
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) {s.store_scalar(719, 0.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;
            if t1d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && (!s.b[2464])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) {
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) {s.store_scalar(334, 1.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) {s.copy_ad(335, 684);s.store_sqrt_sub(342, 91, 1435);s.store_mul(171, 335, 342);s.store_div_scaled_inputs_indices(343, 335, 0.5, 342, 1.0);s.store_div_from_scalar(334, 1.0, 171);s.store_mul(335, 238, 334);s.store_scale(336, 335, s.v[509]);s.store_scale(337, 334, s.v[509]);s.store_add_scaled_product_indices(339, 336, 1.0, 508, 166, 1.0);}
    }
}
