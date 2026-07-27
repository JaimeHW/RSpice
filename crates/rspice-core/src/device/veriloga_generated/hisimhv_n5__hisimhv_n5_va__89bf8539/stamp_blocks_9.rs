#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_144(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1441]) && s.b[2428]) && s.b[2493]) {s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_scale(336, 336, 0.5);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(87), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[2494] = (s.v[336] < 0.0);s.store_scalar(2494, if s.b[2494] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2493]) && s.b[2494]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2493]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p[284]);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1437, p[285], 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 87, 1.0, 340, 1.0, 1436, -1.0);s.store_add_product3_rhs_indices(338, 338, 1437, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2493])) {s.store_scalar(343, 0.0);}
        s.b[2495] = (p[287] != 0.0);s.store_scalar(2495, if s.b[2495] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2495]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1437);}
        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2495])) {s.store_scalar(342, 0.0);}
        s.b[2496] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(2496, if s.b[2496] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2496]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_mul3_lhs(45, 115, 249, 253);s.store_add(135, 135, 45);}
        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2496])) {s.store_scalar(45, 0.0);}
        s.b[2497] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(2497, if s.b[2497] { 1.0 } else { 0.0 });s.b[2498] = (p[296] > 0.0);s.store_scalar(2498, if s.b[2498] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p[300]), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p[296] + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && (!s.b[2498])) {s.copy_ad(341, 647);}
        s.b[2499] = (s.v[793] >= 0.0);s.store_scalar(2499, if s.b[2499] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2499]) {s.copy_ad(369, 793);}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && (!s.b[2499])) {s.store_scalar(369, 0.0);}
        s.b[2500] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(2500, if s.b[2500] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2500]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p[297] - 1.0)) * ((20.0 + 1.0) - ((0.5 * p[297]) * 20.0))) * ((1e-12) as f64).powf(p[297])));s.store_scalar(379, ((((0.5 * p[297]) * (((20.0 + 1.0)) as f64).powf((p[297] - 1.0))) / 20.0) * ((1e-12) as f64).powf((p[297] - 2.0))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_145(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2500]) {s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && (!s.b[2500])) {s.store_powf_offset_input(335, 369, 1e-12, p[297]);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2497]) {s.store_powf_offset_input(343, 369, 1e-12, p[299]);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2497])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        s.b[2501] = (p[27] != 0.0);s.store_scalar(2501, if s.b[2501] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 684);s.store_scalar(337, (s.v[628] - p[139]));s.store_div_from_scalar_square_ad(338, 1.0, s.ad_value(337));s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(335), 2.0), 336, 338);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));s.store_scalar(338, s.v[496]);s.store_scalar(340, s.v[497]);s.store_add_scaled_product_indices(335, 338, 1.0, 340, 1437, 1.0);s.store_mul(137, 121, 335);s.store_sub_from_scalar_scaled_input(335, s.v[498], 790, p[213]);s.store_add_scaled_inputs3_offset_indices(138, 1438, 1.0, 335, 1.0, 137, 1.0, (-s.v[160]));s.store_mul3_lhs(141, 694, 186, 186);s.store_scaled_mul(142, 141, 154, 0.5);s.store_scaled_mul(143, 142, 154, 2.0);s.store_scale(345, 154, 0.25);s.store_offset_sub_ad(344, A::offset(A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(141), s.ad_value(345), (-1.0)), ((s.v[160]) + ((-s.v[498])))), s.ad_value(137), 1e-25);s.store_offset_sub(335, 1438, 344, (-0.005));}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_scalar(334, (if (s.v[344] >= 0.0) { 1.0 } else { (-1.0) }));}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_sqrt_add_scaled_square_product(336, 335, 1.0, 334, 344, (4.0 * 0.005));s.store_sub_mixed_ai(337, A::add_scaled_inputs4_offset(s.ad_value(344), 1.0, s.ad_value(335), 0.5, s.ad_value(336), 0.5, s.ad_value(137), 1.0, (((-s.v[160])) + (s.v[498]))), 1436);s.store_offset_mul(338, 154, 337, (-1.0));s.store_div_from_scalar(339, 4.0, 143);s.store_offset_mul(335, 338, 339, 1.0);s.store_mul(340, 154, 339);s.store_mul(341, 338, 339);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2502] = (s.v[335] < 0.0);s.store_scalar(2502, if s.b[2502] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2502]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(144, 335);s.store_mul_scale_offset_indices(334, 142, 144, -1.0, 1.0);s.store_add(146, 138, 334);s.store_div_from_scalar_add_ad(334, 1.0, s.ad_value(154), A::div_scalar_offset_denominator(2.0, s.ad_value(138), 1e-25, 1.0));s.store_mul_ln_mixed_ia(147, 334, A::mul(A::div_scalar_by_product(1.0, s.ad_value(140), s.ad_value(141), 1.0), A::square(s.ad_value(138))));s.store_offset_sub(148, 147, 146, (-0.002));s.store_sqrt_add_scaled_square_input(334, 148, 1.0, 147, (4.0 * 0.002));s.store_add_scaled_inputs3_indices(149, 147, 1.0, 148, (-0.5), 334, (-0.5));s.store_mul_exp_mixed_ia(334, 140, A::mul(s.ad_value(154), s.ad_value(149)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_146(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_add_offset_lhs_mixed_ai(335, A::mul(s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1436))), (-1.0), 334);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2503] = (s.v[335] < 0.0);s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2503]) {s.store_scalar(335, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(150, 335);s.store_offset_mul_ad(335, s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1436)), (-1.0));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2504] = (s.v[335] < 0.0);s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2504]) {s.store_scalar(335, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(151, 335);s.store_div_from_scalar(336, 0.5, 151);s.store_mul_sub_rhs(152, 139, 150, 151);s.store_sub(335, 146, 149);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2505] = (s.v[335] < 0.0);s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2505]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_offset(335, 335, 1e-25);s.store_div(332, 790, 335);s.store_div_from_scalar_square_ad(336, 1.0, s.ad_value(335));s.store_square(722, 332);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t0,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t0);
        let (t1,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1);
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2506] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });s.b[2507] = (4.0 == 1.0);s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && s.b[2507]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2);s.b[2508] = (4.0 == 2.0);s.store_scalar(2508, if s.b[2508] { 1.0 } else { 0.0 });
        let (t3,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && (!s.b[2507])) && s.b[2508]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3);s.b[2509] = (4.0 == 4.0);s.store_scalar(2509, if s.b[2509] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && (!s.b[2507])) && (!s.b[2508])) && s.b[2509]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);s.b[2510] = (4.0 == 8.0);s.store_scalar(2510, if s.b[2510] { 1.0 } else { 0.0 });
        let (t5,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && (!s.b[2507])) && (!s.b[2508])) && (!s.b[2509])) && s.b[2510]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5);
        let (t6,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6);let mut ta: usize = 0;
        while {
            let t9: f64 = if (((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;
            if ta > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) {s.store_sqrt(726, 726);}
            let (t8,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) {
        let t7: f64 = (s.v[719] + 1.0);
        (t7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8);
        }
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && (!s.b[2506])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_147(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(333, 332, 726, 1.0);s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);s.store_scale(145, 155, ((2.0 * s.v[495]) * p[7]));s.copy_ad(335, 170);s.store_div_scaled_product_mixed_aii(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);s.store_add(134, 134, 153);}
        s.b[2511] = (((p[31] != 0.0) && (p[30] != 0.0)) && (s.v[963] == 0.0));s.store_scalar(2511, if s.b[2511] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2428]) && s.b[2511]) {s.store_square(317, 127);s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);s.store_sub(319, 317, 318);s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);s.store_scaled_add(317, 317, 782, 0.5);}
        s.b[2512] = (s.v[317] < 0.0);s.store_scalar(2512, if s.b[2512] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2511]) && s.b[2512]) {s.store_scalar(317, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2511]) {s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);s.store_scaled_add(319, 319, 782, 0.5);}
        s.b[2513] = (s.v[319] < 0.0);s.store_scalar(2513, if s.b[2513] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2511]) && s.b[2513]) {s.store_scalar(319, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1441]) && s.b[2428]) && s.b[2511]) {s.store_sub(320, 317, 319);}
        s.b[2514] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2514, if s.b[2514] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2511]) && s.b[2514]) {
        (0.0,)
    } else {
        (s.v[321],)
    }
};
        s.store_scalar(321, tb);
        let (tc,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2511]) && (!s.b[2514])) {
        (1.0,)
    } else {
        (s.v[321],)
    }
};
        s.store_scalar(321, tc);
        let (td,) = {
    if ((!s.b[1441]) && (s.v[946] != 0.0)) {
        (0.0,)
    } else {
        (s.v[946],)
    }
};
        s.store_scalar(946, td);s.b[2515] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));s.store_scalar(2515, if s.b[2515] { 1.0 } else { 0.0 });
        if ((!s.b[1441]) && s.b[2515]) {s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);s.store_add_mixed_ai(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);}
        if ((!s.b[1441]) && (!s.b[2515])) {s.store_scalar(128, 0.0);}
        if (!s.b[1441]) {s.copy_ad(136, 134);s.store_scalar(46, 0.0);}
        s.b[2517] = ((p[450] > 0.0) && (p[454] > 0.0));s.store_scalar(2517, if s.b[2517] { 1.0 } else { 0.0 });
        if ((!s.b[1441]) && s.b[2517]) {s.store_scalar(2522, 1e-5);s.store_offset_add_scaled_inputs3_offset_indices(2523, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]), (-p[455]));}
        let (tf,) = {
    if ((!s.b[1441]) && s.b[2517]) {
        let te: f64 = (s.v[118] + p[455]);
        (te,)
    } else {
        (s.v[2524],)
    }
};
        s.store_scalar(2524, tf);
        if ((!s.b[1441]) && s.b[2517]) {s.store_sqrt_offset_ad(781, A::square(A::sub(s.ad_value(960), s.ad_value(1433))), ((4.0 * 0.01) * 0.01));s.store_add_scaled_inputs3_indices(2534, 960, 0.5, 1433, ((-1.0) * 0.5), 781, 0.5);s.store_sqrt_ad(2518, A::div_scaled_product_offset_denominator(s.ad_value(2534), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));s.store_mul(2520, 2518, 162);s.store_div_scaled_product_add_scaled_denominator_indices(993, 2520, 2520, (-0.25), 790, 1.0, 2520, 1.0, 1.0);}
        s.b[2536] = (p[457] > 0.0);s.store_scalar(2536, if s.b[2536] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2517]) && s.b[2536]) {s.store_scalar(2521, p[457]);}
        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {s.copy_ad(2537, 993);}
        let (t10,) = {
    if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
        (s.v[2524],)
    } else {
        (s.v[2538],)
    }
};
        s.store_scalar(2538, t10);
        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(2523), s.ad_value(2537))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {s.store_add_product3_rhs_mixed_iia(89, 2523, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);s.store_mul_sub_rhs(116, 154, 89, 2537);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_148(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2539] = (s.v[116] < 3.0);s.store_scalar(2539, if s.b[2539] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2539]) {s.store_mul_sub_rhs(333, 154, 2523, 2537);s.store_div_scalar_by_product_indices(335, 1.0, 154, 212, (1.414213562373095 / 108.0));s.store_offset_scaled(336, 335, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);s.store_square(338, 338);}
        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2539]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }
        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2539]) {s.store_add_scaled_inputs_mixed_ai(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 1.0, 339, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(89, 2537, 1.0, 332, 155, 1.0);s.copy_ad(88, 89);}
        s.b[2540] = (s.v[791] <= s.v[2538]);s.store_scalar(2540, if s.b[2540] { 1.0 } else { 0.0 });
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && (!s.b[2539])) && s.b[2540]) {s.copy_ad(88, 89);}
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && (!s.b[2539])) && (!s.b[2540])) {s.store_div_scalar_by_product_indices(335, 1.0, 210, 211, 1.0);s.store_mul3_lhs(336, 335, 2523, 2523);s.store_add_div_from_scalar_rhs(337, 154, 2.0, 2523);s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p[456]);s.store_offset_sub(781, 90, 89, (-0.0008));s.store_scale(782, 90, (4.0 * 0.0008));}
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && (!s.b[2539])) && (!s.b[2540])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && (!s.b[2539])) && (!s.b[2540])) {s.store_sqrt_square_add(782, 781, 782);s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {s.store_offset(332, 2537, (1e-12 / 2.0));}
        s.b[2541] = (s.v[88] < s.v[332]);s.store_scalar(2541, if s.b[2541] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2541]) {s.copy_ad(88, 332);}
        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {s.copy_ad(2521, 88);}
        s.b[2542] = (p[451] == 1.0);s.store_scalar(2542, if s.b[2542] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) {s.copy_ad(88, 2521);s.copy_ad(2543, 993);}
        let (t15,) = {
    if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) {
        let t11: f64 = (s.v[160] - s.v[120]);let t12: f64 = (t11 + s.v[182]);let t13: f64 = (t12 + s.v[2543]);let t14: f64 = (t13 + p[455]);
        (t14,)
    } else {
        (s.v[86],)
    }
};
        s.store_scalar(86, t15);s.b[2552] = (s.v[791] < s.v[86]);s.store_scalar(2552, if s.b[2552] { 1.0 } else { 0.0 });
        let (t17,) = {
    if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) {
        let t16: f64 = (-1.0);
        (t16,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, t17);
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_sub_rhs(332, 154, 2523, 2543);s.store_div_scalar_by_product_indices(335, 1.0, 154, 209, 1.0);s.store_mul(333, 335, 185);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_offset(338, 332, (-2.0));s.store_scaled_mul(339, 333, 338, 9.0);s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);s.store_square(276, 278);}
        s.b[2553] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2553, if s.b[2553] { 1.0 } else { 0.0 });
        if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) && s.b[2553]) {s.store_add_scaled_inputs3_offset_mixed_iai(274, 278, 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, 339, 1.0, ((-7.0) * 1.414213562373095));}
        if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) && (!s.b[2553])) {s.store_sqrt_add(275, 277, 276);s.store_add_offset_lhs(274, 275, ((-7.0) * 1.414213562373095), 339);}
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_149(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) {s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div_from_scalar(335, 1.0, 273);s.store_mul(116, 272, 335);s.store_add_scaled_product_indices(167, 2543, 1.0, 116, 155, 1.0);s.store_sub(335, 167, 2543);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_add_div_lhs_indices(2521, 335, 337, 2543);}
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2543), (-p[456])));}
        let (t18,) = {
    if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t18);
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {s.copy_ad(2544, 88);s.store_mul3_affine_lhs(2545, 166, 2522, (0.5 * 9662367879.197212), 0.0, 2522);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 2545);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(2546, 335, 2545);}
        let (t19,) = {
    if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t19);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_150(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t23: usize = 0;
        while {
            let t21: f64 = (s.v[421] + 1.0);let t22: f64 = if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (s.v[97] <= t21)) { 1.0 } else { 0.0 };
            t22 != 0.0
        } {
            t23 += 1;
            if t23 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t23, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {s.store_sub(2547, 2544, 2543);s.store_mul(116, 154, 2547);s.store_mul_sub_rhs(333, 2546, 2547, 2545);}
            s.b[2554] = (s.v[333] < 60.0);s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2554]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 2546, -1.0, 2545);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(2549, 336, 1.0, 2546);s.store_div_scaled_value_offset_denominator(2550, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2554])) {s.store_sub(2549, 2547, 2545);s.store_scalar(2550, 1.0);}
            if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {s.store_mul(2548, 154, 2549);}
            s.b[2555] = (((s.v[116]) as f64).abs() < 1e-16);s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2555]) {s.store_sqrt_scaled_input_ad(334, A::sub_from_scalar(1.0, A::square(s.ad_value(2550))), 1.0 / (2.0));s.store_mul(223, 116, 334);s.store_mul(2551, 154, 334);}
            s.b[2556] = (s.v[116] < 0.0);s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2555]) && s.b[2556]) {s.store_neg(223, 223);s.store_neg(2551, 2551);}
            s.b[2557] = (((s.v[116]) as f64).abs() < 0.005);s.store_scalar(2557, if s.b[2557] { 1.0 } else { 0.0 });
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2555])) && s.b[2557]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 2548, 1.0, 2548, 1.0, 2548, 1.0, 2548, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 2548, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2548), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2548), 1.0, A::scale(s.ad_value(2548), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(223, 334, 336);s.store_div_scaled_product_mixed_iai(2551, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(2550), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2555])) && (!s.b[2557])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 2548);s.store_sqrt_ad(223, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(2548), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));s.store_div_scaled_product_mixed_iai(2551, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(2550), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            s.b[2558] = ((s.v[79] == 1.0) && (s.v[116] < 0.0));s.store_scalar(2558, if s.b[2558] { 1.0 } else { 0.0 });
            let (t1b,) = {
    if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2558]) {
        let t1a: f64 = (-1.0);
        (t1a,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, t1b);s.b[2559] = (s.v[116] < 0.0);s.store_scalar(2559, if s.b[2559] { 1.0 } else { 0.0 });
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2559]) {s.store_neg(216, 223);s.store_neg(217, 2551);}
            s.b[2560] = (s.v[116] < 1e-7);s.store_scalar(2560, if s.b[2560] { 1.0 } else { 0.0 });
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2559])) && s.b[2560]) {s.copy_ad(216, 223);s.copy_ad(217, 2551);}
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2559])) && (!s.b[2560])) {s.store_mul_scale_offset_indices(117, 154, 2544, 1.0, (-p[456]));s.store_exp(228, 117);s.store_mul_mixed_ia(214, 210, A::add_scaled_offset_product_rhs(s.ad_value(228), 1.0, s.ad_value(230), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 210, 154, A::sub(s.ad_value(228), s.ad_value(230)));s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 2551, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {s.store_add_scaled_inputs_product_indices(232, 2544, 1.0, 2523, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2561] = (s.v[79] == 1.0);s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });
            let (t1d,) = {
    if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2561]) {
        let t1c: f64 = (s.v[421] + 1.0);
        (t1c,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t1d);
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[2544]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(2544))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2562] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) && s.b[2562]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) {s.store_add(2544, 2544, 236);}
            s.b[2563] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });
            let (t1e,) = {
    if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) && s.b[2563]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t1e);
            let (t20,) = {
    if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
        let t1f: f64 = (s.v[97] + 1.0);
        (t1f,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t20);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_151(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {s.copy_ad(2521, 2544);}
        if ((!s.b[1441]) && s.b[2517]) {s.store_mul_sub_scaled_inputs_rhs_indices(339, 154, 2521, -1.0, 993, -1.0);s.store_abs(2533, 339);s.store_exp(340, 339);s.store_sub_offset_lhs(341, 340, (-1.0), 339);}
        s.b[2564] = (s.v[339] > 1e-7);s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2517]) && s.b[2564]) {s.store_mul_scaled_sqrt_rhs(2535, 209, -1.0, 341);}
        s.b[2565] = (s.v[2533] > 1e-7);s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });
        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2564])) && s.b[2565]) {s.store_mul_sqrt_rhs(2535, 209, 341);}
        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2564])) && (!s.b[2565])) {s.store_mul_scaled_sqrt_ad_rhs(2535, 339, (-0.7071067811865475), A::offset(A::mul_scaled_lhs(s.ad_value(2533), 0.3333333333333333, A::scale_offset(s.ad_value(2533), 0.25, 1.0)), 1.0));}
        if ((!s.b[1441]) && s.b[2517]) {s.store_sqrt_square_offset(781, 2535, ((4.0 * 1e-6) * 1e-6));s.store_scaled_add(2530, 2535, 781, 0.5);s.store_div_scaled_inputs_indices(2531, 2530, 1.0, 586, 1.6021918e-19);s.store_offset(335, 2531, (-p[452]));s.store_scale(2532, 2531, 0.01);s.store_sqrt_add_scaled_square_product(781, 335, 1.0, 2532, 2532, 4.0);s.store_scaled_add(336, 335, 781, 0.5);s.store_div_scaled_product_by_product_indices(2529, 336, 336, 1.0, 2531, 2531, 1.0);s.store_add_scaled_product_mixed_iai(994, 993, 1.0, A::sub(s.ad_value(2521), s.ad_value(993)), 2529, 1.0);s.store_mul_scale_offset(333, A::exp(A::mul(s.ad_value(154), A::add_scaled_inputs3(s.ad_value(994), 1.0, s.ad_value(960), -1.0, s.ad_value(1433), 1.0))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, s.ad_value(790))), -1.0, 1.0);s.store_scalar(2525, (((((2.0 * 1.6021918e-19) * s.v[489]) * 1.034943e-10)) as f64).sqrt());s.store_mul_sqrt_rhs(2526, 2525, 155);s.store_mul_sub_rhs(2519, 154, 994, 993);}
        s.b[2566] = ((s.v[2519] < (0.2 * s.v[154])) && ((0.2 * s.v[154]) >= 0.0));s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });
        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {s.store_sub_scaled_inputs(781, 154, 0.2, 2519, 1.0);s.store_square(722, 781);s.store_scaled_mul(723, 154, 154, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t24,) = {
    if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t24);
        let (t25,) = {
    if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t25);
        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2567] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });s.b[2568] = (1.0 == 1.0);s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });
        let (t26,) = {
    if (((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && s.b[2568]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t26);s.b[2569] = (1.0 == 2.0);s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });
        let (t27,) = {
    if ((((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && (!s.b[2568])) && s.b[2569]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t27);s.b[2570] = (1.0 == 4.0);s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });
        let (t28,) = {
    if (((((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && (!s.b[2568])) && (!s.b[2569])) && s.b[2570]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t28);s.b[2571] = (1.0 == 8.0);s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });
        let (t29,) = {
    if ((((((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && (!s.b[2568])) && (!s.b[2569])) && (!s.b[2570])) && s.b[2571]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t29);
        let (t2a,) = {
    if ((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t2a);let mut t2e: usize = 0;
        while {
            let t2d: f64 = if (((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2d != 0.0
        } {
            t2e += 1;
            if t2e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) {s.store_sqrt(726, 726);}
            let (t2c,) = {
    if ((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) {
        let t2b: f64 = (s.v[719] + 1.0);
        (t2b,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2c);
        }
        if ((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && (!s.b[2567])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 154, 0.2, 0.0, 726);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_152(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {s.store_div_scaled_product3_indices(334, 154, 725, 726, 0.2, 770, 1.0);s.store_sub_scaled_inputs(335, 154, 0.2, 780, 1.0);}
        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
        }
        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2566])) {s.copy_ad(335, 2519);s.store_scalar(334, 1.0);}
        if ((!s.b[1441]) && s.b[2517]) {s.store_sqrt_offset_input(2527, 335, (10.0 * 2.220446049250313e-16));s.store_mul(2528, 2526, 2527);s.store_mul_scale_offset_mixed_ai(995, A::div_scaled_inputs(s.ad_value(155), 2.0, s.ad_value(162), 1.0), 2528, p[454], 0.0);s.store_scaled_mul(46, 995, 333, s.v[632]);s.store_add(134, 136, 46);}
        if (!s.b[1441]) {s.store_add(134, 136, 46);s.copy_ad(978, 133);}
        s.store_scale(335, 162, (-s.v[635]));s.store_mul(20, 335, 131);s.store_mul(132, 335, 133);s.store_mul(19, 132, 247);s.store_mul(979, 335, 978);s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p[263])));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, p[263], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[2572] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });
        if s.b[2572] {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t2f,) = {
    if s.b[2572] {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t2f);
        let (t30,) = {
    if s.b[2572] {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t30);
        if s.b[2572] {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2573] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });s.b[2574] = (2.0 == 1.0);s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });
        let (t31,) = {
    if ((s.b[2572] && s.b[2573]) && s.b[2574]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t31);s.b[2575] = (2.0 == 2.0);s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });
        let (t32,) = {
    if (((s.b[2572] && s.b[2573]) && (!s.b[2574])) && s.b[2575]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t32);s.b[2576] = (2.0 == 4.0);s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });
        let (t33,) = {
    if ((((s.b[2572] && s.b[2573]) && (!s.b[2574])) && (!s.b[2575])) && s.b[2576]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t33);s.b[2577] = (2.0 == 8.0);s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });
        let (t34,) = {
    if (((((s.b[2572] && s.b[2573]) && (!s.b[2574])) && (!s.b[2575])) && (!s.b[2576])) && s.b[2577]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t34);
        let (t35,) = {
    if (s.b[2572] && s.b[2573]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t35);let mut t39: usize = 0;
        while {
            let t38: f64 = if ((s.b[2572] && s.b[2573]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t38 != 0.0
        } {
            t39 += 1;
            if t39 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t39, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[2572] && s.b[2573]) {s.store_sqrt(726, 726);}
            let (t37,) = {
    if (s.b[2572] && s.b[2573]) {
        let t36: f64 = (s.v[719] + 1.0);
        (t36,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t37);
        }
        if (s.b[2572] && (!s.b[2573])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if s.b[2572] {s.store_div_from_scalar(726, 1.0, 726);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_153(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2572] {s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if s.b[2572] {
        }
        if (!s.b[2572]) {
        }
        if (!s.b[2572]) {s.store_scalar(334, 1.0);}
        s.store_add(109, 87, 110);s.store_add_scaled_product_mixed_iai(134, 134, 1.0, A::div_from_scalar(s.v[163], s.ad_value(162)), 790, p[435]);s.b[2578] = (p[23] == 0.0);s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });
        if s.b[2578] {s.store_scalar(280, 0.0);s.store_scalar(288, 0.0);}
        s.b[2579] = ((s.v[481] > 0.0) && (s.v[454] > 0.0));s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });
        if ((!s.b[2578]) && s.b[2579]) {s.store_mul(335, 659, 85);s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 660, 1436, (-1.0));s.store_offset_mul(340, 338, 339, 1.0);s.store_scaled_offset(341, 338, 1.0, 2.0);}
        s.b[2580] = ((s.v[340] < (1e-6 + s.v[341])) && (s.v[341] >= 0.0));s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });
        if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {s.store_sub_offset_lhs(781, 341, 1e-6, 340);s.store_square(722, 781);s.store_square(723, 341);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t3a,) = {
    if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t3a);
        let (t3b,) = {
    if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3b);
        if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2581] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });s.b[2582] = (4.0 == 1.0);s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });
        let (t3c,) = {
    if (((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && s.b[2582]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3c);s.b[2583] = (4.0 == 2.0);s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });
        let (t3d,) = {
    if ((((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && (!s.b[2582])) && s.b[2583]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3d);s.b[2584] = (4.0 == 4.0);s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });
        let (t3e,) = {
    if (((((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && (!s.b[2582])) && (!s.b[2583])) && s.b[2584]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3e);s.b[2585] = (4.0 == 8.0);s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });
        let (t3f,) = {
    if ((((((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && (!s.b[2582])) && (!s.b[2583])) && (!s.b[2584])) && s.b[2585]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3f);
        let (t40,) = {
    if ((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t40);let mut t44: usize = 0;
        while {
            let t43: f64 = if (((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t43 != 0.0
        } {
            t44 += 1;
            if t44 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t44, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) {s.store_sqrt(726, 726);}
            let (t42,) = {
    if ((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) {
        let t41: f64 = (s.v[719] + 1.0);
        (t41,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t42);
        }
        if ((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && (!s.b[2581])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 341, 726);s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);s.store_sub_offset_lhs(340, 341, 1e-6, 780);}
        if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
        }
        if (((!s.b[2578]) && s.b[2579]) && (!s.b[2580])) {
        }
        if (((!s.b[2578]) && s.b[2579]) && (!s.b[2580])) {s.store_scalar(334, 1.0);}
        if ((!s.b[2578]) && s.b[2579]) {s.store_sqrt(340, 340);s.store_add_mul_sub_from_scalar_rhs_indices(282, 335, 337, 1.0, 340);s.store_div_from_scalar_offset_input(336, s.v[582], 661, s.v[582]);s.store_add_scaled_inputs_product_indices(283, 1437, s.v[483], 109, 1.0, 336, 282, (-1.0));s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);s.store_scaled_add(283, 283, 782, 0.5);}
        s.b[2586] = (s.v[283] < 0.0);s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });
        if (((!s.b[2578]) && s.b[2579]) && s.b[2586]) {s.store_scalar(283, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_154(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[2578]) && s.b[2579]) && s.b[2586]) {s.store_scalar(343, 0.0);}
        if ((!s.b[2578]) && s.b[2579]) {s.store_offset(283, 283, 1e-25);s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);}
        if ((!s.b[2578]) && s.b[2579]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }
        if ((!s.b[2578]) && s.b[2579]) {s.store_div(339, 662, 958);s.store_mul(340, 663, 958);s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));s.store_mul_product3_indices(280, 336, 339, 283, 134, 1.0);s.store_mul3_lhs(288, 339, 283, 336);}
        if ((!s.b[2578]) && (!s.b[2579])) {s.store_scalar(280, 0.0);}
        s.b[2587] = (s.v[664] != 0.0);s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });
        if ((!s.b[2578]) && s.b[2587]) {s.copy_ad(334, 799);s.store_sqrt_square_offset(782, 334, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(335, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[2588] = (s.v[334] < 0.0);s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });
        if (((!s.b[2578]) && s.b[2587]) && s.b[2588]) {s.store_scalar(334, 0.0);s.store_scalar(335, 0.0);}
        if ((!s.b[2578]) && s.b[2587]) {s.store_sqrt_offset_input(335, 127, 1e-25);s.store_div_from_scalar_scaled_input(337, 1.0, 335, 2.0);s.store_sub_mixed_ia(338, 334, A::scale_offset(s.ad_value(791), ((p[106]) * (p[105])), p[105]));s.store_sqrt_square_offset(782, 338, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 338, 782, 0.5, 0.5);s.store_scaled_add(338, 338, 782, 0.5);}
        s.b[2589] = (s.v[338] < 0.0);s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });
        if (((!s.b[2578]) && s.b[2587]) && s.b[2589]) {s.store_scalar(338, 0.0);s.store_scalar(343, 0.0);}
        if ((!s.b[2578]) && s.b[2587]) {s.store_offset(338, 338, 1e-25);s.store_mul_ad_product_rhs_mixed_ia(344, 450, 451, A::exp(A::div_from_scalar((-1.0), s.ad_value(338))));s.store_mul_scale_offset_mixed_ia(345, 344, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 1.0);s.store_mul(337, 338, 344);s.store_sub(334, 334, 337);s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[2590] = (s.v[334] < 0.0);s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });
        if (((!s.b[2578]) && s.b[2587]) && s.b[2590]) {s.store_scalar(334, 0.0);s.store_scalar(343, 0.0);}
        if ((!s.b[2578]) && s.b[2587]) {s.store_offset(334, 334, 1e-25);s.store_div_scalar_by_product_indices(338, 1.0, 334, 335, 1.0);s.store_scalar(341, (s.v[165] * s.v[554]));s.store_exp_mul_scaled_lhs_indices(336, 341, -1.0, 338);s.store_mul_product3_indices(340, 338, 341, 336, 338, 1.0);s.store_mul_product3_indices(281, 336, 664, 134, 334, 1.0);}
        s.b[2591] = (p[45] == 0.0);s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });
        if s.b[2591] {s.store_scalar(423, 0.0);}
        s.b[2592] = ((p[45] * (s.v[796] - p[446])) < 0.0);s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });
        if ((!s.b[2591]) && s.b[2592]) {s.copy_ad(426, 427);}
        if ((!s.b[2591]) && (!s.b[2592])) {s.store_add_scaled_inputs_mixed_ai(426, A::square(A::offset(s.ad_value(796), (-p[446]))), p[445], 427, 1.0);}
        if (!s.b[2591]) {s.store_scaled_limited_exp_ad(423, A::mul(s.ad_value(154), A::sub(s.ad_value(793), s.ad_value(426))), p[449]);}
        s.b[2593] = (s.v[423] > 0.0);s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });s.b[2594] = ((s.v[423] > (100000.0 - 50000.0)) && (50000.0 >= 0.0));s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });
        if (s.b[2593] && s.b[2594]) {s.store_offset(781, 423, (((-100000.0)) + (50000.0)));s.store_square(722, 781);s.store_scalar(723, (50000.0 * 50000.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t45,) = {
    if (s.b[2593] && s.b[2594]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t45);
        let (t46,) = {
    if (s.b[2593] && s.b[2594]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t46);
        if (s.b[2593] && s.b[2594]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_155(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2595] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });s.b[2596] = (1.0 == 1.0);s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });
        let (t47,) = {
    if (((s.b[2593] && s.b[2594]) && s.b[2595]) && s.b[2596]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t47);s.b[2597] = (1.0 == 2.0);s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });
        let (t48,) = {
    if ((((s.b[2593] && s.b[2594]) && s.b[2595]) && (!s.b[2596])) && s.b[2597]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t48);s.b[2598] = (1.0 == 4.0);s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });
        let (t49,) = {
    if (((((s.b[2593] && s.b[2594]) && s.b[2595]) && (!s.b[2596])) && (!s.b[2597])) && s.b[2598]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t49);s.b[2599] = (1.0 == 8.0);s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });
        let (t4a,) = {
    if ((((((s.b[2593] && s.b[2594]) && s.b[2595]) && (!s.b[2596])) && (!s.b[2597])) && (!s.b[2598])) && s.b[2599]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4a);
        let (t4b,) = {
    if ((s.b[2593] && s.b[2594]) && s.b[2595]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t4b);let mut t4f: usize = 0;
        while {
            let t4e: f64 = if (((s.b[2593] && s.b[2594]) && s.b[2595]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4e != 0.0
        } {
            t4f += 1;
            if t4f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t4f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[2593] && s.b[2594]) && s.b[2595]) {s.store_sqrt(726, 726);}
            let (t4d,) = {
    if ((s.b[2593] && s.b[2594]) && s.b[2595]) {
        let t4c: f64 = (s.v[719] + 1.0);
        (t4c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t4d);
        }
        if ((s.b[2593] && s.b[2594]) && (!s.b[2595])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (s.b[2593] && s.b[2594]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 50000.0);s.store_div_scaled_product_indices(334, 725, 726, 50000.0, 770, 1.0);s.store_offset(336, 780, (100000.0 - 50000.0));}
        if (s.b[2593] && s.b[2594]) {
        }
        if (s.b[2593] && (!s.b[2594])) {s.copy_ad(336, 423);s.store_scalar(334, 1.0);}
        if s.b[2593] {s.store_scale(422, 336, (s.v[365] * s.v[632]));}
        if (!s.b[2593]) {s.store_scalar(422, 0.0);}
        s.b[2600] = ((((s.v[280] + s.v[281]) > 0.0) && (s.v[523] != 0.0)) && (s.v[963] == 0.0));s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });
        if s.b[2600] {s.store_offset_scaled(334, 120, s.v[524], 1.0);s.store_add(335, 280, 281);s.store_scaled_mul(111, 334, 335, s.v[523]);s.store_div_from_scalar(344, 1.0, 99);s.store_mul3_lhs(335, 154, 111, 344);s.store_square(345, 344);s.store_div_from_scalar(344, 1.0, 102);s.store_mul3_lhs(336, 154, 111, 344);s.store_square(345, 344);s.store_mul_mixed_ia(112, 209, A::add_scaled_products(s.ad_value(104), s.ad_value(336), 1.0, s.ad_value(101), s.ad_value(335), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(113, 209, 103, 336, ((-1.0) * (0.5)), 100, 335, 0.5);s.store_add(114, 112, 113);s.store_mul3_lhs(400, 115, 114, 253);s.store_mul(287, 288, 400);}
        s.b[2601] = (p[24] != 0.0);s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });s.b[2602] = (s.v[78] == 0.0);s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2602]) {s.store_offset_add(191, 109, 1437, (-(10.0 * 2.220446049250313e-16)));s.store_sub_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::offset(s.ad_value(1438), (-s.v[160])), 1.0, A::sub(s.ad_value(120), s.ad_value(182)), s.ad_value(162), s.v[560]), 1.0, 191, s.v[515]);s.store_square(335, 335);s.store_scalar(337, (1.0 / s.v[187]));s.store_mul(336, 335, 337);s.store_scalar(337, (1.0 / s.v[561]));s.store_offset_mul(341, 255, 337, 1.0);s.store_mul(195, 336, 341);s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);s.store_scaled_add(195, 195, 782, 0.5);}
        s.b[2603] = (s.v[195] < 0.0);s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });
        if ((s.b[2601] && s.b[2602]) && s.b[2603]) {s.store_scalar(195, 0.0);s.store_scalar(339, 0.0);}
        if (s.b[2601] && s.b[2602]) {s.store_sqrt_square_offset(782, 1438, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(338, 1438, 782, 0.5, 0.5);s.store_scaled_add(337, 1438, 782, 0.5);}
        s.b[2604] = (s.v[337] < 0.0);s.store_scalar(2604, if s.b[2604] { 1.0 } else { 0.0 });
        if ((s.b[2601] && s.b[2602]) && s.b[2604]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if (s.b[2601] && s.b[2602]) {s.store_offset(337, 337, (-p[262]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_156(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2601] && s.b[2602]) {s.store_scale(332, 337, 10.0);s.store_offset_square(336, 332, 1.0);s.store_sub_from_scalar_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(336)));s.store_mul(195, 195, 335);s.store_scale(334, 162, s.v[632]);s.store_div_from_scalar_offset_input(341, s.v[562], 334, s.v[562]);s.store_scalar(340, s.v[516]);s.store_div_add_scaled_inputs_rhs_indices(343, 340, 340, 1.0, 1437, 1.0);s.store_div_from_scalar_offset_input(338, 1.0, 195, 1e-25);s.store_scaled_mul(335, 193, 338, (-s.v[514]));s.store_scaled_mul(337, 338, 338, s.v[514]);}
        s.b[2605] = (s.v[335] < (-34.0));s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });
        if ((s.b[2601] && s.b[2602]) && s.b[2605]) {s.store_scalar(199, 0.0);}
        if ((s.b[2601] && s.b[2602]) && (!s.b[2605])) {s.store_exp(336, 335);s.store_mul_scale_offset_mixed_ia(337, 334, A::div_from_scalar(s.v[513], s.ad_value(192)), 1.6021918e-19, 0.0);s.store_div_from_scalar(339, 1.0, 209);s.store_sqrt_ad(340, A::mul_offset_lhs(s.ad_value(978), (s.v[188] * 1e-12), s.ad_value(339)));s.store_mul3_lhs(338, 336, 337, 340);s.store_mul(339, 338, 195);s.store_mul(344, 339, 195);s.store_mul3_lhs(199, 341, 343, 344);}
        if s.b[2601] {s.store_offset_scaled(334, 791, (-s.v[518]), s.v[559]);s.store_exp_scaled_input(336, 334, s.v[187]);s.store_scale(334, 791, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));s.store_mul(337, 791, 334);s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));s.store_mul3_lhs(200, 338, 336, 337);}
        s.b[2606] = (s.v[791] >= 0.0);s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2606]) {s.store_scale(200, 200, (-1.0));}
        if s.b[2601] {s.store_sub(335, 791, 790);s.store_offset_scaled(334, 335, (-s.v[518]), s.v[559]);s.store_exp_scaled_input(336, 334, s.v[187]);s.store_scale(334, 335, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));s.store_mul(337, 335, 334);s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));s.store_mul3_lhs(201, 338, 336, 337);}
        s.b[2607] = (s.v[335] >= 0.0);s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2607]) {s.store_scale(201, 201, (-1.0));}
        if s.b[2601] {s.store_scaled_offset_ad(195, A::neg(A::sub(s.ad_value(791), s.ad_value(792))), ((s.v[160]) + (p[258])), 1.0 / (s.v[187]));s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);s.store_scaled_add(195, 195, 782, 0.5);}
        s.b[2608] = (s.v[195] < 0.0);s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2608]) {s.store_scalar(195, 0.0);s.store_scalar(339, 0.0);}
        if s.b[2601] {s.store_offset(195, 195, 1e-25);s.store_div_from_scalar(335, (-s.v[520]), 195);}
        s.b[2609] = (s.v[335] < (-34.0));s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2609]) {s.store_scalar(202, 0.0);}
        if (s.b[2601] && (!s.b[2609])) {s.store_exp(336, 335);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(337, s.v[520], A::square(s.ad_value(195)), 336);s.store_scale(337, 162, (s.v[519] * s.v[632]));s.store_mul_product3_indices(202, 336, 337, 195, 195, 1.0);}
        if s.b[2601] {s.copy_ad(285, 677);s.store_mul(286, 393, 285);s.store_scaled_offset_ad(336, A::add_scaled_inputs4(s.ad_value(1436), s.v[493], s.ad_value(1438), (-1.0), s.ad_value(122), 1.0, s.ad_value(174), 1.0), (-s.v[492]), (-1.0 / (s.v[187])));s.store_square(334, 336);s.store_scale(335, 286, s.v[491]);s.store_div_scaled_inputs_indices(337, 335, -1.0, 336, 1.0);}
        s.b[2610] = (s.v[337] < (-34.0));s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2610]) {s.store_scalar(339, 0.0);}
        if (s.b[2601] && (!s.b[2610])) {s.store_exp(339, 337);}
        if s.b[2601] {s.store_div_from_scalar(338, (((1.6021918e-19 * s.v[490]) * s.v[632]) * s.v[582]), 285);}
        s.b[2611] = (((2.0 * s.v[336]) + s.v[335]) < 0.0);s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2611]) {s.store_mul3_affine_lhs(284, 338, 335, (0.25 * 7.38905609893065), 0.0, 335);}
        if (s.b[2601] && (!s.b[2611])) {s.store_mul3_lhs(284, 338, 334, 339);}
        if s.b[2601] {s.store_sub(202, 202, 284);}
        s.b[2612] = (p[25] != 0.0);s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_157(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2612] {s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(790), 1.0, A::scale(s.ad_value(790), 100.0)), (-1e-5));s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 790, (4.0 * 1e-5));s.store_add_scaled_inputs3_indices(196, 790, 1.0, 335, (-0.5), 336, (-0.5));}
        s.b[2613] = (p[25] == 0.0);s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });
        if s.b[2613] {s.store_scalar(203, 0.0);}
        if (!s.b[2613]) {s.store_add_scaled_inputs4_offset_indices(335, 196, p[242], 791, (-1.0), 122, p[244], 174, p[244], (p[243] * p[242]));s.store_scalar(336, (1.0 / s.v[187]));s.store_mul(194, 335, 336);s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);s.store_scaled_add(197, 194, 782, 0.5);}
        s.b[2614] = (s.v[197] < 0.0);s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });
        if ((!s.b[2613]) && s.b[2614]) {s.store_scalar(197, 0.0);s.store_scalar(339, 0.0);}
        if (!s.b[2613]) {s.store_div_from_scalar_offset_input(337, 1.0, 197, 1e-25);s.store_scaled_mul(334, 193, 337, (-s.v[512]));}
        s.b[2615] = (s.v[334] < (-34.0));s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });
        if ((!s.b[2613]) && s.b[2615]) {s.store_scalar(203, 0.0);}
        if ((!s.b[2613]) && (!s.b[2615])) {s.store_exp(335, 334);s.store_scale_ad(336, A::div_from_scalar(s.v[511], s.ad_value(192)), (1.6021918e-19 * s.v[632]));s.store_mul_product3_indices(203, 335, 336, 197, 197, 1.0);}
        if (!s.b[2613]) {s.store_sub(205, 790, 792);}
        s.b[2616] = (s.v[205] > 0.0);s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });
        if ((!s.b[2613]) && s.b[2616]) {s.store_square(336, 205);s.store_mul(338, 336, 205);s.store_offset(334, 338, 0.5);s.store_div(339, 338, 334);s.store_div_square_rhs_mixed_ai(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), 334);s.store_mul(203, 203, 339);}
        if ((!s.b[2613]) && (!s.b[2616])) {s.store_scalar(203, 0.0);}
        s.b[2617] = (p[25] == 0.0);s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });
        if s.b[2617] {s.store_scalar(204, 0.0);}
        if (!s.b[2617]) {s.store_add_scaled_inputs3_mixed_aii(335, A::add_scaled_inputs3_offset(s.ad_value(196), (-p[242]), s.ad_value(791), -1.0, s.ad_value(196), 1.0, ((p[243]) * (p[242]))), 1.0, 122, p[244], 174, p[244]);s.store_scalar(336, (1.0 / s.v[187]));s.store_mul(194, 335, 336);s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);s.store_scaled_add(198, 194, 782, 0.5);}
        s.b[2618] = (s.v[198] < 0.0);s.store_scalar(2618, if s.b[2618] { 1.0 } else { 0.0 });
        if ((!s.b[2617]) && s.b[2618]) {s.store_scalar(198, 0.0);s.store_scalar(339, 0.0);}
        if (!s.b[2617]) {s.store_div_from_scalar_offset_input(337, 1.0, 198, 1e-25);s.store_scaled_mul(334, 193, 337, (-s.v[512]));}
        s.b[2619] = (s.v[334] < (-34.0));s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });
        if ((!s.b[2617]) && s.b[2619]) {s.store_scalar(204, 0.0);}
        if ((!s.b[2617]) && (!s.b[2619])) {s.store_exp(335, 334);s.store_div_from_scalar(337, 1.0, 192);s.store_scale(336, 337, (s.v[511] * (1.6021918e-19 * s.v[632])));s.store_mul_product3_indices(204, 335, 336, 198, 198, 1.0);}
        if (!s.b[2617]) {s.store_neg(206, 792);}
        s.b[2620] = (s.v[206] > 0.0);s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });
        if ((!s.b[2617]) && s.b[2620]) {s.store_square(336, 206);s.store_mul(338, 336, 206);s.store_offset(334, 338, 0.5);s.store_div(339, 338, 334);s.store_div_square_rhs_mixed_ai(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), 334);s.store_mul(204, 204, 339);}
        if ((!s.b[2617]) && (!s.b[2620])) {s.store_scalar(204, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_158(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(2621, 0.0);s.store_scalar(2624, 0.0);s.store_scalar(2623, 0.0);s.store_scalar(406, 0.0);s.store_scalar(2623, 0.0);s.b[2625] = (1.0 == 1.0);s.store_scalar(2625, if s.b[2625] { 1.0 } else { 0.0 });s.b[2626] = (1.0 == 2.0);s.store_scalar(2626, if s.b[2626] { 1.0 } else { 0.0 });s.b[2627] = (1.0 == 3.0);s.store_scalar(2627, if s.b[2627] { 1.0 } else { 0.0 });s.b[2628] = (1.0 == 4.0);s.store_scalar(2628, if s.b[2628] { 1.0 } else { 0.0 });s.b[2629] = (((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] > 0.0));s.store_scalar(2629, if s.b[2629] { 1.0 } else { 0.0 });
        let (t50,) = {
    if (s.b[2625] && s.b[2629]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t50);
        let (t51,) = {
    if (s.b[2625] && s.b[2629]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, t51);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_159(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2625] && s.b[2629]) {s.store_sub(395, 731, 728);s.store_neg(396, 728);s.store_scalar(409, s.v[460]);s.store_scalar(407, p[66]);s.store_scalar(411, 0.0);s.copy_ad(410, 687);s.store_scalar(413, s.v[188]);}
        s.b[2630] = (((((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p[55] != 1.0));s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });
        let (t52,) = {
    if ((s.b[2626] && (!s.b[2625])) && s.b[2630]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t52);
        if ((s.b[2626] && (!s.b[2625])) && s.b[2630]) {s.store_sub(395, 734, 735);s.store_neg(396, 735);}
        s.b[2631] = (((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] > 0.0));s.store_scalar(2631, if s.b[2631] { 1.0 } else { 0.0 });
        let (t53,) = {
    if ((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t53);
        let (t54,) = {
    if ((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) {
        (1.0,)
    } else {
        (s.v[2624],)
    }
};
        s.store_scalar(2624, t54);
        if ((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) {s.store_sub(395, 731, 728);s.store_sub(396, 729, 728);s.store_scalar(409, s.v[459]);s.store_scalar(407, (p[63] + (p[64] * p[55])));s.copy_ad(411, 384);s.copy_ad(410, 686);s.copy_ad(413, 412);s.store_neg(407, 407);}
        s.b[2632] = (((s.v[407] < 0.0) && (p[432] > 0.0)) && (p[55] == 1.0));s.store_scalar(2632, if s.b[2632] { 1.0 } else { 0.0 });
        if (((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) {s.store_neg(407, 407);s.store_scalar(335, p[63]);s.store_offset_div_scaled_product_indices(996, 335, 335, 1.0, 651, 1.0, (-p[137]));}
        s.b[2633] = (p[113] > 0.0);s.store_scalar(2633, if s.b[2633] { 1.0 } else { 0.0 });s.b[2634] = ((s.v[396] == 0.0) || (p[113] <= 0.0));s.store_scalar(2634, if s.b[2634] { 1.0 } else { 0.0 });
        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && s.b[2634]) {
        }
        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && (!s.b[2634])) {s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));}
        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && (!s.b[2634])) {s.store_mul(784, 783, 396);s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p[113], 1.0);s.store_powf(782, 781, (1.0 / p[113]));s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);}
        if ((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) {s.store_sqrt_offset_square_offset(782, 396, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(396), p[137]), 782, 0.5);}
        s.b[2635] = (s.v[336] < 0.0);s.store_scalar(2635, if s.b[2635] { 1.0 } else { 0.0 });
        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && s.b[2635]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub(407, 407, 600);}
        s.b[2636] = (((((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p[55] != 1.0));s.store_scalar(2636, if s.b[2636] { 1.0 } else { 0.0 });
        let (t55,) = {
    if ((s.b[2628] && (!((s.b[2625] || s.b[2626]) || s.b[2627]))) && s.b[2636]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, t55);
        if ((s.b[2628] && (!((s.b[2625] || s.b[2626]) || s.b[2627]))) && s.b[2636]) {s.store_sub(395, 734, 735);s.store_sub(396, 733, 735);}
        if (s.v[2623] != 0.0) {s.store_scalar(2644, 0.4);}
        let (t56,) = {
    if (s.v[2623] != 0.0) {
        (0.0,)
    } else {
        (s.v[2645],)
    }
};
        s.store_scalar(2645, t56);
        if (s.v[2623] != 0.0) {s.store_scalar(223, 0.0);s.store_scalar(214, 0.0);s.store_scalar(216, 0.0);s.store_scalar(232, 0.0);s.store_scalar(236, 0.0);s.store_scalar(233, 0.0);s.store_scalar(217, 0.0);s.store_scalar(420, 0.0);s.store_scalar(215, 0.0);s.store_scalar(447, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);}
        let (t58,) = {
    if (s.v[2623] != 0.0) {
        let t57: f64 = (-1.0);
        (t57,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t58);
        if (s.v[2623] != 0.0) {s.store_scalar(2646, 0.0);s.store_scalar(2647, 0.0);s.store_mul_scaled_ln_ad_rhs(2642, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2642), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));}
        if (s.v[2623] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.v[2623] != 0.0) {s.store_sqrt_square_add(782, 781, 782);}
    }
}
