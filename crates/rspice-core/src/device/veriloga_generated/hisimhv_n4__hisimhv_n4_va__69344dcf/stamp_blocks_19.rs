#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_85(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(2093, 2113, 1.0, 2087, (-1.0), 781, 0.5, 782, 0.5);s.copy_ad(2089, 2084);s.copy_ad(2086, 2093);s.store_scalar(79, 0.0);s.store_mul(2137, 2125, 2126);s.store_scalar(98, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_86(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t7: usize = 0;
        while {
            let t6: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[98] <= 150.0)) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul_sub_mixed_iai(2091, 2125, A::add_scaled_product(s.ad_value(2113), 1.0, s.ad_value(2126), s.ad_value(2089), 1.0), 2087);s.store_sub(335, 2089, 2091);}
            s.b[2249] = ((s.v[335] < 0.001) && (0.001 >= 0.0));s.store_scalar(2249, if s.b[2249] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {s.store_sub_from_scalar(781, 0.001, 335);s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2250] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2250, if s.b[2250] { 1.0 } else { 0.0 });s.b[2251] = (2.0 == 1.0);s.store_scalar(2251, if s.b[2251] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && s.b[2251]) {s.store_scalar(720, 1.0);}
            s.b[2252] = (2.0 == 2.0);s.store_scalar(2252, if s.b[2252] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && s.b[2252]) {s.store_scalar(720, 2.0);}
            s.b[2253] = (2.0 == 4.0);s.store_scalar(2253, if s.b[2253] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && s.b[2253]) {s.store_scalar(720, 3.0);}
            s.b[2254] = (2.0 == 8.0);s.store_scalar(2254, if s.b[2254] { 1.0 } else { 0.0 });
            if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && (!s.b[2253])) && s.b[2254]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {s.store_scalar(719, 0.0);}
            let mut t3: usize = 0;
            while {
                let t2: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2 != 0.0
            } {
                t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {s.store_offset_sub(781, 2082, 2127, 1e-12);s.store_square(722, 781);s.store_scalar(723, (1e-12 * 1e-12));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2256] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2256, if s.b[2256] { 1.0 } else { 0.0 });s.b[2257] = (2.0 == 1.0);s.store_scalar(2257, if s.b[2257] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && s.b[2257]) {s.store_scalar(720, 1.0);}
            s.b[2258] = (2.0 == 2.0);s.store_scalar(2258, if s.b[2258] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {s.store_scalar(720, 2.0);}
            s.b[2259] = (2.0 == 4.0);s.store_scalar(2259, if s.b[2259] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && s.b[2259]) {s.store_scalar(720, 3.0);}
            s.b[2260] = (2.0 == 8.0);s.store_scalar(2260, if s.b[2260] { 1.0 } else { 0.0 });
            if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && (!s.b[2259])) && s.b[2260]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {s.store_scalar(719, 0.0);}
            let mut t5: usize = 0;
            while {
                let t4: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t4 != 0.0
            } {
                t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
            s.b[2261] = ((s.v[2133] > (s.v[2084] - p.p406)) && (p.p406 >= 0.0));s.store_scalar(2261, if s.b[2261] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {s.store_offset_sub(781, 2133, 2084, p.p406);s.store_square(722, 781);s.store_scalar(723, (p.p406 * p.p406));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2262] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2262, if s.b[2262] { 1.0 } else { 0.0 });s.b[2263] = (4.0 == 1.0);s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && s.b[2263]) {s.store_scalar(720, 1.0);}
            s.b[2264] = (4.0 == 2.0);s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && s.b[2264]) {s.store_scalar(720, 2.0);}
            s.b[2265] = (4.0 == 4.0);s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && s.b[2265]) {s.store_scalar(720, 3.0);}
            s.b[2266] = (4.0 == 8.0);s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });
            if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && (!s.b[2265])) && s.b[2266]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {s.store_scalar(719, 0.0);}
            let mut t1: usize = 0;
            while {
                let t0: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t0 != 0.0
            } {
                t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && (!s.b[2262])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p406);s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);s.store_add_offset_lhs(2133, 2084, (-p.p406), 780);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {s.store_scalar(334, 1.0);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul(2134, 2134, 334);s.store_mul(2135, 2135, 334);s.store_mul_sub_rhs(339, 154, 2086, 2089);s.store_exp(340, 339);s.store_sub_offset_lhs(344, 340, (-1.0), 339);}
            s.b[2267] = (s.v[339] >= 1e-7);s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2267]) {s.store_scalar(347, (-1.0));s.store_mul_scaled_sqrt_rhs(2095, 209, -1.0, 344);s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2095, 1.0);s.store_mul_scale_offset_indices(2122, 345, 340, 1.0, (-1.0));s.store_mul_scale_offset_indices(2124, 345, 340, -1.0, 1.0);}
            s.b[2268] = (s.v[339] < (-1e-7));s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && s.b[2268]) {s.store_scalar(347, 1.0);s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2086), 1.0, s.ad_value(2113), p.p398));s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2089), 1.0, s.ad_value(2113), p.p398));s.store_mul_sqrt_mixed_ia(2095, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2095, 1.0);s.store_mul_add_mixed_iaa(2122, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));s.store_mul_mixed_ia(2124, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));}
            s.b[2269] = (s.v[339] > 0.0);s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && s.b[2269]) {s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2160, 2159);s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);s.store_neg(2124, 2122);}
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && (!s.b[2269])) {s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2160, 2159);s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);s.store_neg(2124, 2122);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] != 0.0)) {s.store_scalar(98, (150.0 + 1.0));}
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
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2271]) {s.store_scalar(79, 1.0);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {s.store_add(2086, 2086, 2108);s.store_add(2089, 2089, 2109);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_primal_offset(98, 98, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_87(
        s: &mut ReactiveScratch,
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
        s.b[2273] = (1.0 == 1.0);s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });s.b[2274] = (((s.v[2086] - s.v[2084]) < p.p403) && (p.p403 >= 0.0));s.store_scalar(2274, if s.b[2274] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2086), s.ad_value(2084)));s.store_square(722, 781);s.store_scalar(723, (p.p403 * p.p403));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2275] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });s.b[2276] = (6.0 == 1.0);s.store_scalar(2276, if s.b[2276] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && s.b[2276]) {s.store_scalar(720, 1.0);}
        s.b[2277] = (6.0 == 2.0);s.store_scalar(2277, if s.b[2277] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && s.b[2277]) {s.store_scalar(720, 2.0);}
        s.b[2278] = (6.0 == 4.0);s.store_scalar(2278, if s.b[2278] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && (!s.b[2277])) && s.b[2278]) {s.store_scalar(720, 3.0);}
        s.b[2279] = (6.0 == 8.0);s.store_scalar(2279, if s.b[2279] { 1.0 } else { 0.0 });
        if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && (!s.b[2277])) && (!s.b[2278])) && s.b[2279]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {s.store_scalar(719, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && (!s.b[2275])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p403);s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);s.store_sub_from_scalar(336, p.p403, 780);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && (!s.b[2274])) {s.store_sub(336, 2086, 2084);s.store_scalar(334, 1.0);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(2115, 209, -1.0, 338);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2273])) {s.copy_ad(2115, 2119);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.copy_ad(87, 2085);s.copy_ad(91, 2086);s.store_sub(94, 2086, 2085);s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_88(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, (p.p263 * 0.1), 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[2280] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2280, if s.b[2280] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2281] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2281, if s.b[2281] { 1.0 } else { 0.0 });s.b[2282] = (2.0 == 1.0);s.store_scalar(2282, if s.b[2282] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && s.b[2282]) {s.store_scalar(720, 1.0);}
        s.b[2283] = (2.0 == 2.0);s.store_scalar(2283, if s.b[2283] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && s.b[2283]) {s.store_scalar(720, 2.0);}
        s.b[2284] = (2.0 == 4.0);s.store_scalar(2284, if s.b[2284] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && (!s.b[2283])) && s.b[2284]) {s.store_scalar(720, 3.0);}
        s.b[2285] = (2.0 == 8.0);s.store_scalar(2285, if s.b[2285] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && (!s.b[2283])) && (!s.b[2284])) && s.b[2285]) {s.store_scalar(720, 4.0);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {s.store_scalar(719, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        s.b[2286] = (((s.v[109] - s.v[2083]) < p.p403) && (p.p403 >= 0.0));s.store_scalar(2286, if s.b[2286] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(109), s.ad_value(2083)));s.store_square(722, 781);s.store_scalar(723, (p.p403 * p.p403));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_89(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2287] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2287, if s.b[2287] { 1.0 } else { 0.0 });s.b[2288] = (6.0 == 1.0);s.store_scalar(2288, if s.b[2288] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && s.b[2288]) {s.store_scalar(720, 1.0);}
        s.b[2289] = (6.0 == 2.0);s.store_scalar(2289, if s.b[2289] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && s.b[2289]) {s.store_scalar(720, 2.0);}
        s.b[2290] = (6.0 == 4.0);s.store_scalar(2290, if s.b[2290] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && (!s.b[2289])) && s.b[2290]) {s.store_scalar(720, 3.0);}
        s.b[2291] = (6.0 == 8.0);s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && (!s.b[2289])) && (!s.b[2290])) && s.b[2291]) {s.store_scalar(720, 4.0);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {s.store_scalar(719, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;assert!(td <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && (!s.b[2287])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p403);s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);s.store_sub_from_scalar(336, p.p403, 780);}
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
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2295] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2295, if s.b[2295] { 1.0 } else { 0.0 });s.b[2296] = (2.0 == 1.0);s.store_scalar(2296, if s.b[2296] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && s.b[2296]) {s.store_scalar(720, 1.0);}
        s.b[2297] = (2.0 == 2.0);s.store_scalar(2297, if s.b[2297] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && s.b[2297]) {s.store_scalar(720, 2.0);}
        s.b[2298] = (2.0 == 4.0);s.store_scalar(2298, if s.b[2298] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {s.store_scalar(720, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_90(
        s: &mut ReactiveScratch,
    ) {
        s.b[2299] = (2.0 == 8.0);s.store_scalar(2299, if s.b[2299] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) && s.b[2299]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;assert!(tf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {s.store_sub_offset_lhs_mixed_ai(781, A::sub_from_scalar(0.8, s.ad_value(2113)), 0.2, 342);s.store_square(722, 781);s.store_square_ad(723, A::sub_from_scalar(0.8, s.ad_value(2113)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2301] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2301, if s.b[2301] { 1.0 } else { 0.0 });s.b[2302] = (1.0 == 1.0);s.store_scalar(2302, if s.b[2302] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && s.b[2302]) {s.store_scalar(720, 1.0);}
        s.b[2303] = (1.0 == 2.0);s.store_scalar(2303, if s.b[2303] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && s.b[2303]) {s.store_scalar(720, 2.0);}
        s.b[2304] = (1.0 == 4.0);s.store_scalar(2304, if s.b[2304] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && (!s.b[2303])) && s.b[2304]) {s.store_scalar(720, 3.0);}
        s.b[2305] = (1.0 == 8.0);s.store_scalar(2305, if s.b[2305] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && (!s.b[2303])) && (!s.b[2304])) && s.b[2305]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;assert!(t11 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 109, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_91(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2307] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2307, if s.b[2307] { 1.0 } else { 0.0 });s.b[2308] = (2.0 == 1.0);s.store_scalar(2308, if s.b[2308] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && s.b[2308]) {s.store_scalar(720, 1.0);}
        s.b[2309] = (2.0 == 2.0);s.store_scalar(2309, if s.b[2309] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && s.b[2309]) {s.store_scalar(720, 2.0);}
        s.b[2310] = (2.0 == 4.0);s.store_scalar(2310, if s.b[2310] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && (!s.b[2309])) && s.b[2310]) {s.store_scalar(720, 3.0);}
        s.b[2311] = (2.0 == 8.0);s.store_scalar(2311, if s.b[2311] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && (!s.b[2309])) && (!s.b[2310])) && s.b[2311]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) {s.store_scalar(719, 0.0);}
        let mut t13: usize = 0;
        while {
            let t12: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;assert!(t13 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) {s.store_sub(340, 168, 91);s.store_mul(337, 154, 238);s.store_div_from_scalar(335, 1.0, 337);s.store_mul(339, 248, 335);s.store_scale(344, 2129, 9662367879.197212);s.store_scalar(335, 100000.0);s.store_div_from_scalar(336, 1.0, 162);s.store_mul_mixed_ai(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);s.store_mul(341, 345, 338);s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));s.store_mul3_lhs(342, 345, 338, 338);s.store_sqrt_square_add(343, 341, 342);s.store_scaled_sub(169, 343, 341, 0.5);s.copy_ad(335, 169);s.store_mul(169, 208, 335);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_scale(169, 169, s.v[619]);s.store_sub(170, 170, 169);s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));s.store_offset_mul(338, 980, 334, 1.0);s.store_mul(339, 336, 238);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 251, p.p160);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_indices(340, 251, 624);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 238, 343);s.store_scalar(336, s.v[474]);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(338), s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_92(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2312] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2312, if s.b[2312] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2312]) {s.copy_ad(336, 335);}
        s.b[2313] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2313, if s.b[2313] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2312])) && s.b[2313]) {s.store_square(336, 335);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2312])) && (!s.b[2313])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p178);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_offset(338, 336, 1.0);}
        s.b[2314] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2314, if s.b[2314] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2314]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2315] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2315, if s.b[2315] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2314])) && s.b[2315]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2314])) && (!s.b[2315])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 338, ((-1.0) / p.p178));
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_mul(253, 254, 339);s.copy_ad(984, 253);s.copy_ad(2112, 255);s.store_scalar(2320, 0.0);s.store_scalar(2151, 0.0);s.store_scalar(990, 0.0);s.store_scalar(2143, 0.0);s.store_scalar(2318, 0.0);s.store_add_scaled_inputs3_offset_indices(2140, 1436, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));}
        s.b[2322] = (0.0 == 0.0);s.store_scalar(2322, if s.b[2322] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2322]) {s.store_offset(2141, 2140, (-p.p393));}
        s.b[2323] = (0.0 == 1.0);s.store_scalar(2323, if s.b[2323] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2322])) && s.b[2323]) {s.store_offset(2141, 1436, (((-s.v[160])) + ((-p.p393))));}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2322])) && (!s.b[2323])) {s.store_offset(2141, 85, (-p.p393));}
        s.b[2324] = (((s.v[2144]) as f64).abs() <= 0.0);s.store_scalar(2324, if s.b[2324] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2324]) {s.store_scalar(2149, 0.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.copy_ad(983, 87);s.store_scale(2166, 2113, p.p399);s.store_scalar(2321, ((s.v[160] + p.p393) - 3.0));}
        s.b[2325] = (1.0 == 1.0);s.store_scalar(2325, if s.b[2325] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2325]) {s.store_scale(2166, 2113, p.p399);s.store_offset(983, 2166, (-1.0));s.copy_ad(2320, 2321);s.copy_ad(2142, 2321);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2325])) {s.store_offset_scaled(2166, 2113, p.p399, (-0.1));s.copy_ad(983, 87);s.copy_ad(2320, 2141);s.copy_ad(2142, 2141);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_scalar(79, 0.0);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_93(
        s: &mut ReactiveScratch,
    ) {
        let mut t15: usize = 0;
        while {
            let t14: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;assert!(t15 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2326] = (s.v[983] >= 0.0);s.store_scalar(2326, if s.b[2326] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2326]) {s.store_mul_scaled_sqrt_ad_rhs(2318, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2121, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2318, 1.0);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2326])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2166)));s.store_exp_mul(338, 154, 2166);s.store_mul_sqrt_mixed_ia(2318, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2318, 1.0);s.store_mul_add_mixed_iaa(2121, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2096, 2318, 1.0, 185, 2320, 983, 1.0);s.store_sub(2097, 2121, 185);s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);}
            s.b[2327] = (((s.v[2108]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2327, if s.b[2327] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && s.b[2327]) {s.store_scalar(79, 1.0);}
            s.b[2328] = (s.v[2108] > 0.1);s.store_scalar(2328, if s.b[2328] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2327])) && s.b[2328]) {s.store_scalar(2108, 0.1);}
            s.b[2329] = (s.v[2108] < (-0.1));s.store_scalar(2329, if s.b[2329] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2327])) && (!s.b[2328])) && s.b[2329]) {s.store_scalar(2108, (-0.1));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 2108);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_primal_offset(97, 97, 1.0);}
        }
        s.b[2331] = (1.0 == 1.0);s.store_scalar(2331, if s.b[2331] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2331]) {s.copy_ad(2167, 983);}
        s.b[2332] = ((s.v[983] < (s.v[2167] + 0.2)) && (0.2 >= 0.0));s.store_scalar(2332, if s.b[2332] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {s.store_sub_offset_lhs(781, 2167, 0.2, 983);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2333] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2333, if s.b[2333] { 1.0 } else { 0.0 });s.b[2334] = (2.0 == 1.0);s.store_scalar(2334, if s.b[2334] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && s.b[2334]) {s.store_scalar(720, 1.0);}
        s.b[2335] = (2.0 == 2.0);s.store_scalar(2335, if s.b[2335] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && s.b[2335]) {s.store_scalar(720, 2.0);}
        s.b[2336] = (2.0 == 4.0);s.store_scalar(2336, if s.b[2336] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && (!s.b[2335])) && s.b[2336]) {s.store_scalar(720, 3.0);}
        s.b[2337] = (2.0 == 8.0);s.store_scalar(2337, if s.b[2337] { 1.0 } else { 0.0 });
        if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && (!s.b[2335])) && (!s.b[2336])) && s.b[2337]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) {s.store_scalar(719, 0.0);}
        let mut t17: usize = 0;
        while {
            let t16: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t16 != 0.0
        } {
            t17 += 1;assert!(t17 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_94(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_scalar(2138, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));}
        s.b[2338] = ((s.v[2149] > (-s.v[2138])) && (s.v[2138] >= 0.0));s.store_scalar(2338, if s.b[2338] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {s.store_add(781, 2149, 2138);s.store_square(722, 781);s.store_square(723, 2138);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
        let mut t19: usize = 0;
        while {
            let t18: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && (s.v[719] < s.v[2139])) { 1.0 } else { 0.0 };
            t18 != 0.0
        } {
            t19 += 1;assert!(t19 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2339] = ((((s.v[2139] == 1.0) || (s.v[2139] == 2.0)) || (s.v[2139] == 4.0)) || (s.v[2139] == 8.0));s.store_scalar(2339, if s.b[2339] { 1.0 } else { 0.0 });s.b[2340] = (s.v[2139] == 1.0);s.store_scalar(2340, if s.b[2340] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && s.b[2340]) {s.store_scalar(720, 1.0);}
        s.b[2341] = (s.v[2139] == 2.0);s.store_scalar(2341, if s.b[2341] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && s.b[2341]) {s.store_scalar(720, 2.0);}
        s.b[2342] = (s.v[2139] == 4.0);s.store_scalar(2342, if s.b[2342] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && (!s.b[2341])) && s.b[2342]) {s.store_scalar(720, 3.0);}
        s.b[2343] = (s.v[2139] == 8.0);s.store_scalar(2343, if s.b[2343] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && (!s.b[2341])) && (!s.b[2342])) && s.b[2343]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) {s.store_scalar(719, 0.0);}
        let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;assert!(t1b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        s.b[2346] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2346, if s.b[2346] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_95(
        s: &mut ReactiveScratch,
    ) {
        s.b[2347] = (s.v[332] >= 500.0);s.store_scalar(2347, if s.b[2347] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && s.b[2347]) {s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(337, 1.403592217853e217);}
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) {s.copy_ad(781, 332);s.store_scalar(335, 1.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;assert!(t1d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
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
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {s.store_sub_from_scalar(781, 1e-16, 990);s.store_square(722, 781);s.store_scalar(723, (1e-16 * 1e-16));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2352] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2352, if s.b[2352] { 1.0 } else { 0.0 });s.b[2353] = (2.0 == 1.0);s.store_scalar(2353, if s.b[2353] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && s.b[2353]) {s.store_scalar(720, 1.0);}
        s.b[2354] = (2.0 == 2.0);s.store_scalar(2354, if s.b[2354] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && s.b[2354]) {s.store_scalar(720, 2.0);}
        s.b[2355] = (2.0 == 4.0);s.store_scalar(2355, if s.b[2355] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && (!s.b[2354])) && s.b[2355]) {s.store_scalar(720, 3.0);}
        s.b[2356] = (2.0 == 8.0);s.store_scalar(2356, if s.b[2356] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && (!s.b[2354])) && (!s.b[2355])) && s.b[2356]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_96(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t1f: usize = 0;
        while {
            let t1e: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;assert!(t1f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2358]) {s.store_scale(2166, 2113, p.p399);s.store_offset(983, 2166, (-1.0));s.copy_ad(2320, 2321);s.copy_ad(2142, 2321);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2358])) {s.store_offset_scaled(2166, 2113, p.p399, (-0.1));s.copy_ad(983, 87);s.copy_ad(2320, 2141);s.copy_ad(2142, 2141);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_scalar(79, 0.0);s.store_scalar(97, 1.0);}
        let mut t21: usize = 0;
        while {
            let t20: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;assert!(t21 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2359] = (s.v[983] >= 0.0);s.store_scalar(2359, if s.b[2359] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2359]) {s.store_mul_scaled_sqrt_ad_rhs(2318, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2121, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2318, 1.0);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2359])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2166)));s.store_exp_mul(338, 154, 2166);s.store_mul_sqrt_mixed_ia(2318, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2318, 1.0);s.store_mul_add_mixed_iaa(2121, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2096, 2318, 1.0, 185, 2320, 983, 1.0);s.store_sub(2097, 2121, 185);s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);}
            s.b[2360] = (((s.v[2108]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2360, if s.b[2360] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && s.b[2360]) {s.store_scalar(79, 1.0);}
            s.b[2361] = (s.v[2108] > 0.1);s.store_scalar(2361, if s.b[2361] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2360])) && s.b[2361]) {s.store_scalar(2108, 0.1);}
            s.b[2362] = (s.v[2108] < (-0.1));s.store_scalar(2362, if s.b[2362] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2360])) && (!s.b[2361])) && s.b[2362]) {s.store_scalar(2108, (-0.1));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 2108);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_primal_offset(97, 97, 1.0);}
        }
        s.b[2364] = (2.0 == 1.0);s.store_scalar(2364, if s.b[2364] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2364]) {s.copy_ad(2167, 983);}
        s.b[2365] = ((s.v[983] < (s.v[2167] + 0.2)) && (0.2 >= 0.0));s.store_scalar(2365, if s.b[2365] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {s.store_sub_offset_lhs(781, 2167, 0.2, 983);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_97(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2366] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2366, if s.b[2366] { 1.0 } else { 0.0 });s.b[2367] = (2.0 == 1.0);s.store_scalar(2367, if s.b[2367] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && s.b[2367]) {s.store_scalar(720, 1.0);}
        s.b[2368] = (2.0 == 2.0);s.store_scalar(2368, if s.b[2368] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && s.b[2368]) {s.store_scalar(720, 2.0);}
        s.b[2369] = (2.0 == 4.0);s.store_scalar(2369, if s.b[2369] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && (!s.b[2368])) && s.b[2369]) {s.store_scalar(720, 3.0);}
        s.b[2370] = (2.0 == 8.0);s.store_scalar(2370, if s.b[2370] { 1.0 } else { 0.0 });
        if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && (!s.b[2368])) && (!s.b[2369])) && s.b[2370]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) {s.store_scalar(719, 0.0);}
        let mut t23: usize = 0;
        while {
            let t22: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t22 != 0.0
        } {
            t23 += 1;assert!(t23 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_scalar(2138, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));}
        s.b[2371] = ((s.v[2149] > (-s.v[2138])) && (s.v[2138] >= 0.0));s.store_scalar(2371, if s.b[2371] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {s.store_add(781, 2149, 2138);s.store_square(722, 781);s.store_square(723, 2138);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
        let mut t25: usize = 0;
        while {
            let t24: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && (s.v[719] < s.v[2139])) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;assert!(t25 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2372] = ((((s.v[2139] == 1.0) || (s.v[2139] == 2.0)) || (s.v[2139] == 4.0)) || (s.v[2139] == 8.0));s.store_scalar(2372, if s.b[2372] { 1.0 } else { 0.0 });s.b[2373] = (s.v[2139] == 1.0);s.store_scalar(2373, if s.b[2373] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && s.b[2373]) {s.store_scalar(720, 1.0);}
        s.b[2374] = (s.v[2139] == 2.0);s.store_scalar(2374, if s.b[2374] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && s.b[2374]) {s.store_scalar(720, 2.0);}
        s.b[2375] = (s.v[2139] == 4.0);s.store_scalar(2375, if s.b[2375] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && (!s.b[2374])) && s.b[2375]) {s.store_scalar(720, 3.0);}
        s.b[2376] = (s.v[2139] == 8.0);s.store_scalar(2376, if s.b[2376] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && (!s.b[2374])) && (!s.b[2375])) && s.b[2376]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) {s.store_scalar(719, 0.0);}
        let mut t27: usize = 0;
        while {
            let t26: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;assert!(t27 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_98(
        s: &mut ReactiveScratch,
    ) {
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
        let mut t29: usize = 0;
        while {
            let t28: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            t28 != 0.0
        } {
            t29 += 1;assert!(t29 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
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
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2382])) && s.b[2383]) {s.store_mul(337, 154, 336);s.store_neg_ad(2143, A::sqrt(A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2382])) && (!s.b[2383])) {s.store_mul_scale_offset_indices(337, 336, 154, -1.0, 0.0);s.store_sqrt_ad(2143, A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {s.store_sub(990, 2144, 2143);}
        s.b[2384] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));s.store_scalar(2384, if s.b[2384] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_99(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {s.store_sub_from_scalar(781, 1e-16, 990);s.store_square(722, 781);s.store_scalar(723, (1e-16 * 1e-16));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2385] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2385, if s.b[2385] { 1.0 } else { 0.0 });s.b[2386] = (2.0 == 1.0);s.store_scalar(2386, if s.b[2386] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && s.b[2386]) {s.store_scalar(720, 1.0);}
        s.b[2387] = (2.0 == 2.0);s.store_scalar(2387, if s.b[2387] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && s.b[2387]) {s.store_scalar(720, 2.0);}
        s.b[2388] = (2.0 == 4.0);s.store_scalar(2388, if s.b[2388] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && (!s.b[2387])) && s.b[2388]) {s.store_scalar(720, 3.0);}
        s.b[2389] = (2.0 == 8.0);s.store_scalar(2389, if s.b[2389] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && (!s.b[2387])) && (!s.b[2388])) && s.b[2389]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) {s.store_scalar(719, 0.0);}
        let mut t2b: usize = 0;
        while {
            let t2a: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2a != 0.0
        } {
            t2b += 1;assert!(t2b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) {s.copy_ad(989, 349);s.store_scaled_add(344, 2113, 155, p.p396);s.store_offset_mul_ad(338, s.ad_value(2131), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 2131, 1.0);}
        s.b[2392] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2392, if s.b[2392] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2393] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2393, if s.b[2393] { 1.0 } else { 0.0 });s.b[2394] = (2.0 == 1.0);s.store_scalar(2394, if s.b[2394] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && s.b[2394]) {s.store_scalar(720, 1.0);}
        s.b[2395] = (2.0 == 2.0);s.store_scalar(2395, if s.b[2395] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && (!s.b[2394])) && s.b[2395]) {s.store_scalar(720, 2.0);}
        s.b[2396] = (2.0 == 4.0);s.store_scalar(2396, if s.b[2396] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && (!s.b[2394])) && (!s.b[2395])) && s.b[2396]) {s.store_scalar(720, 3.0);}
        s.b[2397] = (2.0 == 8.0);s.store_scalar(2397, if s.b[2397] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && (!s.b[2394])) && (!s.b[2395])) && (!s.b[2396])) && s.b[2397]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) {s.store_scalar(719, 0.0);}
        let mut t2d: usize = 0;
        while {
            let t2c: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2c != 0.0
        } {
            t2d += 1;assert!(t2d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_100(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && (!s.b[2393])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && (!s.b[2392])) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && (!s.b[2392])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2130, 1.0, 337);}
        s.b[2398] = ((s.v[344] < (s.v[972] + p.p405)) && (p.p405 >= 0.0));s.store_scalar(2398, if s.b[2398] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {s.store_sub_offset_lhs(781, 972, p.p405, 344);s.store_square(722, 781);s.store_scalar(723, (p.p405 * p.p405));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2399] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2399, if s.b[2399] { 1.0 } else { 0.0 });s.b[2400] = (2.0 == 1.0);s.store_scalar(2400, if s.b[2400] { 1.0 } else { 0.0 });
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && s.b[2400]) {s.store_scalar(720, 1.0);}
        s.b[2401] = (2.0 == 2.0);s.store_scalar(2401, if s.b[2401] { 1.0 } else { 0.0 });
        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && (!s.b[2400])) && s.b[2401]) {s.store_scalar(720, 2.0);}
        s.b[2402] = (2.0 == 4.0);s.store_scalar(2402, if s.b[2402] { 1.0 } else { 0.0 });
        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && (!s.b[2400])) && (!s.b[2401])) && s.b[2402]) {s.store_scalar(720, 3.0);}
        s.b[2403] = (2.0 == 8.0);s.store_scalar(2403, if s.b[2403] { 1.0 } else { 0.0 });
        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && (!s.b[2400])) && (!s.b[2401])) && (!s.b[2402])) && s.b[2403]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) {s.store_scalar(719, 0.0);}
        let mut t2f: usize = 0;
        while {
            let t2e: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2e != 0.0
        } {
            t2f += 1;assert!(t2f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && (!s.b[2399])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p405);s.store_div_scaled_product_indices(334, 725, 726, p.p405, 770, 1.0);s.store_sub_offset_lhs(992, 972, p.p405, 780);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && (!s.b[2398])) {s.copy_ad(992, 344);s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {s.copy_ad(2155, 2141);s.store_offset_mul(338, 2131, 2155, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {s.store_add_mul_sub_from_scalar_rhs_indices(2156, 2155, 2130, 1.0, 337);s.copy_ad(2152, 2156);s.store_scalar(79, 0.0);s.store_scalar(97, 1.0);}
    }
}
