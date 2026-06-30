#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2493] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(2493, if s.b[2493] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2430]) && s.b[2493]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2494] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(2494, if s.b[2494] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && s.b[2494]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && (!s.b[2494])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && (!s.b[2494])) {
            s.store_mul(339, 338, 340);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_mul(253, 254, 339);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_div_scaled_inputs_indices(335, 115, -1.0, 170, 1.0);
            s.store_mul3_lhs(135, 115, 248, 253);
        }

        s.b[2495] = (p.p283 != 0.0);
        s.store_scalar(2495, if s.b[2495] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
            s.store_scale(336, 336, 0.5);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(87), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2496] = (s.v[336] < 0.0);
        s.store_scalar(2496, if s.b[2496] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2495]) && s.b[2496]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1439, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 87, 1.0, 340, 1.0, 1438, -1.0);
            s.store_add_product3_rhs_indices(338, 338, 1439, 334, 339, 1.0);
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2495])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2497] = (p.p287 != 0.0);
        s.store_scalar(2497, if s.b[2497] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2430]) && s.b[2497]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1439);
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2497])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2498] = ((s.v[343] + s.v[342]) > 0.0);
        s.store_scalar(2498, if s.b[2498] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2430]) && s.b[2498]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_mul3_lhs(45, 115, 249, 253);
            s.store_add(135, 135, 45);
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2498])) {
            s.store_scalar(45, 0.0);
        }

        s.b[2499] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.store_scalar(2499, if s.b[2499] { 1.0 } else { 0.0 });

        s.b[2500] = (p.p296 > 0.0);
        s.store_scalar(2500, if s.b[2500] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2500])) {
            s.copy_ad(341, 647);
        }

        s.b[2501] = (s.v[793] >= 0.0);
        s.store_scalar(2501, if s.b[2501] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2501]) {
            s.copy_ad(369, 793);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2501])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2502] = (s.v[369] < (20.0 * 1e-12));
        s.store_scalar(2502, if s.b[2502] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2502]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2502])) {
            s.store_powf_offset_input(335, 369, 1e-12, p.p297);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2499]) {
            s.store_powf_offset_input(343, 369, 1e-12, p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2499])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        s.b[2503] = (p.p27 != 0.0);
        s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_scale(335, 186, 1.034943e-10);
            s.copy_ad(336, 684);
            s.store_scalar(337, (s.v[628] - p.p139));
            s.store_div_from_scalar_square_ad(338, 1.0, s.ad_value(337));
            s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, 338);
            s.store_mul(121, 339, 181);
            s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);
            s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));
            s.store_scalar(338, s.v[496]);
            s.store_scalar(340, s.v[497]);
            s.store_add_scaled_product_indices(335, 338, 1.0, 340, 1439, 1.0);
            s.store_mul(137, 121, 335);
            s.store_sub_from_scalar_scaled_input(335, s.v[498], 790, p.p213);
            s.store_add_scaled_inputs3_offset_indices(138, 1440, 1.0, 335, 1.0, 137, 1.0, (-s.v[160]));
            s.store_mul3_lhs(141, 694, 186, 186);
            s.store_scaled_mul(142, 141, 154, 0.5);
            s.store_scaled_mul(143, 142, 154, 2.0);
            s.store_scale(345, 154, 0.25);
            s.store_offset_sub_ad(344, A::offset(A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(141), s.ad_value(345), (-1.0)), ((s.v[160]) + ((-s.v[498])))), s.ad_value(137), 1e-25);
            s.store_offset_sub(335, 1440, 344, (-0.005));
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_scalar(334, (if (s.v[344] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_sqrt_add_scaled_square_product(336, 335, 1.0, 334, 344, (4.0 * 0.005));
            s.store_sub_ad_lhs(337, A::add_scaled_inputs4_offset(s.ad_value(344), 1.0, s.ad_value(335), 0.5, s.ad_value(336), 0.5, s.ad_value(137), 1.0, (((-s.v[160])) + (s.v[498]))), 1438);
            s.store_offset_mul(338, 154, 337, (-1.0));
            s.store_div_from_scalar(339, 4.0, 143);
            s.store_offset_mul(335, 338, 339, 1.0);
            s.store_mul(340, 154, 339);
            s.store_mul(341, 338, 339);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2504] = (s.v[335] < 0.0);
        s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2504]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(144, 335);
            s.store_mul_sub_from_scalar_rhs(334, 142, 1.0, 144);
            s.store_add(146, 138, 334);
            s.store_div_from_scalar_add_ad(334, 1.0, s.ad_value(154), A::div_scalar_offset_denominator(2.0, s.ad_value(138), 1e-25, 1.0));
            s.store_mul_ln_ad_lhs(147, A::mul(A::div_scalar_by_product(1.0, s.ad_value(140), s.ad_value(141), 1.0), A::square(s.ad_value(138))), 334);
            s.store_offset_sub(148, 147, 146, (-0.002));
        }

    }

    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_sqrt_add_scaled_square_input(334, 148, 1.0, 147, (4.0 * 0.002));
            s.store_add_scaled_inputs3_indices(149, 147, 1.0, 148, (-0.5), 334, (-0.5));
            s.store_mul_exp_ad_rhs(334, 140, A::mul(s.ad_value(154), s.ad_value(149)));
            s.store_add_offset_ad_lhs(335, A::mul(s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1438))), (-1.0), 334);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2505] = (s.v[335] < 0.0);
        s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2505]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(150, 335);
            s.store_offset_mul_ad(335, s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1438)), (-1.0));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2506] = (s.v[335] < 0.0);
        s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2506]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(151, 335);
            s.store_div_from_scalar(336, 0.5, 151);
            s.store_mul_sub_rhs(152, 139, 150, 151);
            s.store_sub(335, 146, 149);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2507] = (s.v[335] < 0.0);
        s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2507]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_offset(335, 335, 1e-25);
            s.store_div(332, 790, 335);
            s.store_div_from_scalar_square_ad(336, 1.0, s.ad_value(335));
            s.store_square(722, 332);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign63880_e98768,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign63880_e98768);

        let (assign63890_e98777,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign63890_e98777);

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2508] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(2508, if s.b[2508] { 1.0 } else { 0.0 });

        s.b[2509] = (4.0 == 1.0);
        s.store_scalar(2509, if s.b[2509] { 1.0 } else { 0.0 });

        let (assign64040_e98934,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && s.b[2509]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign64040_e98934);

        s.b[2510] = (4.0 == 2.0);
        s.store_scalar(2510, if s.b[2510] { 1.0 } else { 0.0 });

        let (assign64060_e98953,) = {
    if ((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && s.b[2510]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign64060_e98953);

        s.b[2511] = (4.0 == 4.0);
        s.store_scalar(2511, if s.b[2511] { 1.0 } else { 0.0 });

        let (assign64080_e98975,) = {
    if (((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && (!s.b[2510])) && s.b[2511]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign64080_e98975);

        s.b[2512] = (4.0 == 8.0);
        s.store_scalar(2512, if s.b[2512] { 1.0 } else { 0.0 });

        let (assign64100_e99000,) = {
    if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && (!s.b[2510])) && (!s.b[2511])) && s.b[2512]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign64100_e99000);

        let (assign64110_e99011,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign64110_e99011);

        let mut assign64120_loop_guard: usize = 0;
        while {
            let assign64120_cond_e99023: f64 = if (((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign64120_cond_e99023 != 0.0
        } {
            assign64120_loop_guard += 1;
            assert!(assign64120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {
                s.store_sqrt(726, 726);
            }
            let (assign64120_body1_e99048,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {
        let assign64120_body1_e99046: f64 = (s.v[719] + 1.0);
        (assign64120_body1_e99046,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign64120_body1_e99048);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && (!s.b[2508])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);
            s.store_scale(145, 155, ((2.0 * s.v[495]) * p.p7));
            s.copy_ad(335, 170);
            s.store_div_scaled_product_left_ad(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);
            s.store_add(134, 134, 153);
        }

        s.b[2513] = (((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[963] == 0.0));
        s.store_scalar(2513, if s.b[2513] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {
            s.store_square(317, 127);
            s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);
            s.store_sub(319, 317, 318);
            s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);
            s.store_scaled_add(317, 317, 782, 0.5);
        }

        s.b[2514] = (s.v[317] < 0.0);
        s.store_scalar(2514, if s.b[2514] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2514]) {
            s.store_scalar(317, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {
            s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);
            s.store_scaled_add(319, 319, 782, 0.5);
        }

        s.b[2515] = (s.v[319] < 0.0);
        s.store_scalar(2515, if s.b[2515] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2515]) {
            s.store_scalar(319, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {
            s.store_sub(320, 317, 319);
        }

        s.b[2516] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));
        s.store_scalar(2516, if s.b[2516] { 1.0 } else { 0.0 });

        let (assign64390_e99385,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2516]) {
        (0.0,)
    } else {
        (s.v[321],)
    }
};
        s.store_scalar(321, assign64390_e99385);

        let (assign64400_e99397,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && (!s.b[2516])) {
        (1.0,)
    } else {
        (s.v[321],)
    }
};
        s.store_scalar(321, assign64400_e99397);

        let (assign64410_e99404,) = {
    if ((!s.b[1443]) && (s.v[946] != 0.0)) {
        (0.0,)
    } else {
        (s.v[946],)
    }
};
        s.store_scalar(946, assign64410_e99404);

        s.b[2517] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));
        s.store_scalar(2517, if s.b[2517] { 1.0 } else { 0.0 });

        if ((!s.b[1443]) && s.b[2517]) {
            s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);
            s.store_add_ad_lhs(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);
        }

        if ((!s.b[1443]) && (!s.b[2517])) {
            s.store_scalar(128, 0.0);
        }

        if (!s.b[1443]) {
            s.copy_ad(136, 134);
            s.store_scalar(46, 0.0);
        }

        s.b[2519] = ((p.p450 > 0.0) && (p.p454 > 0.0));
        s.store_scalar(2519, if s.b[2519] { 1.0 } else { 0.0 });

        if ((!s.b[1443]) && s.b[2519]) {
            s.store_scalar(2524, 1e-5);
            s.store_offset_add_scaled_inputs3_offset_indices(2525, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]), (-p.p455));
        }

        let (assign64540_e99510,) = {
    if ((!s.b[1443]) && s.b[2519]) {
        let assign64540_e99508: f64 = (s.v[118] + p.p455);
        (assign64540_e99508,)
    } else {
        (s.v[2526],)
    }
};
        s.store_scalar(2526, assign64540_e99510);

        if ((!s.b[1443]) && s.b[2519]) {
            s.store_sqrt_offset_ad(781, A::square(A::sub(s.ad_value(960), s.ad_value(1435))), ((4.0 * 0.01) * 0.01));
            s.store_add_scaled_inputs3_indices(2536, 960, 0.5, 1435, ((-1.0) * 0.5), 781, 0.5);
            s.store_sqrt_ad(2520, A::div_scaled_product_offset_denominator(s.ad_value(2536), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));
            s.store_mul(2522, 2520, 162);
            s.store_div_scaled_product_add_scaled_denominator_indices(993, 2522, 2522, (-0.25), 790, 1.0, 2522, 1.0, 1.0);
        }

        s.b[2538] = (p.p457 > 0.0);
        s.store_scalar(2538, if s.b[2538] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2519]) && s.b[2538]) {
            s.store_scalar(2523, p.p457);
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.copy_ad(2539, 993);
        }

        let (assign64630_e99622,) = {
    if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
        (s.v[2526],)
    } else {
        (s.v[2540],)
    }
};
        s.store_scalar(2540, assign64630_e99622);

    }

    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(2525), s.ad_value(2539))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.store_add_product3_rhs_mixed_iia(89, 2525, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);
            s.store_mul_sub_rhs(116, 154, 89, 2539);
        }

        s.b[2541] = (s.v[116] < 3.0);
        s.store_scalar(2541, if s.b[2541] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {
            s.store_mul_sub_rhs(333, 154, 2525, 2539);
            s.store_div_from_scalar_scaled_mul(335, 1.0, 154, 212, (1.414213562373095 / 108.0));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);
            s.store_square(338, 338);
        }

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 2539, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2542] = (s.v[791] <= s.v[2540]);
        s.store_scalar(2542, if s.b[2542] { 1.0 } else { 0.0 });

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && s.b[2542]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 2525, 2525);
            s.store_add_div_from_scalar_rhs(337, 154, 2.0, 2525);
            s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p.p456);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.store_offset(332, 2539, (1e-12 / 2.0));
        }

        s.b[2543] = (s.v[88] < s.v[332]);
        s.store_scalar(2543, if s.b[2543] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2543]) {
            s.copy_ad(88, 332);
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.copy_ad(2523, 88);
        }

        s.b[2544] = (p.p451 == 1.0);
        s.store_scalar(2544, if s.b[2544] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) {
            s.copy_ad(88, 2523);
            s.copy_ad(2545, 993);
        }

        let (assign64970_e100203,) = {
    if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) {
        let assign64970_e100195: f64 = (s.v[160] - s.v[120]);
        let assign64970_e100197: f64 = (assign64970_e100195 + s.v[182]);
        let assign64970_e100199: f64 = (assign64970_e100197 + s.v[2545]);
        let assign64970_e100201: f64 = (assign64970_e100199 + p.p455);
        (assign64970_e100201,)
    } else {
        (s.v[86],)
    }
};
        s.store_scalar(86, assign64970_e100203);

        s.b[2554] = (s.v[791] < s.v[86]);
        s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });

        let (assign64990_e100221,) = {
    if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
        let assign64990_e100219: f64 = (-1.0);
        (assign64990_e100219,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, assign64990_e100221);

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_sub_rhs(332, 154, 2525, 2545);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2555] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });

        if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) && s.b[2555]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(274, 278, 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, 339, 1.0, ((-7.0) * 1.414213562373095));
        }

        if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) && (!s.b[2555])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_add_offset_lhs(274, 275, ((-7.0) * 1.414213562373095), 339);
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 2545, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 2545);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_div_lhs_indices(2523, 335, 337, 2545);
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
            s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2545), (-p.p456)));
        }

        let (assign65240_e100687,) = {
    if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign65240_e100687);

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
            s.copy_ad(2546, 88);
            s.store_mul3_affine_lhs(2547, 166, 2524, (0.5 * 9662367879.197212), 0.0, 2524);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 2547);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(2548, 335, 2547);
        }

        let (assign65300_e100800,) = {
    if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign65300_e100800);

        let mut assign65310_loop_guard: usize = 0;
        while {
            let assign65310_cond_e100816: f64 = (s.v[421] + 1.0);
            let assign65310_cond_e100818: f64 = if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (s.v[97] <= assign65310_cond_e100816)) { 1.0 } else { 0.0 };
            assign65310_cond_e100818 != 0.0
        } {
            assign65310_loop_guard += 1;
            assert!(assign65310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
                s.store_sub(2549, 2546, 2545);
                s.store_mul(116, 154, 2549);
                s.store_mul_sub_rhs(333, 2548, 2549, 2547);
            }
            s.b[2556] = (s.v[333] < 60.0);
            s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2556]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 2548, -1.0, 2547);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(2551, 336, 1.0, 2548);
                s.store_div_scaled_value_offset_denominator(2552, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2556])) {
                s.store_sub(2551, 2549, 2547);
                s.store_scalar(2552, 1.0);
            }
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
                s.store_mul(2550, 154, 2551);
            }
            s.b[2557] = (((s.v[116]) as f64).abs() < 1e-16);
            s.store_scalar(2557, if s.b[2557] { 1.0 } else { 0.0 });
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2557]) {
                s.store_sqrt_scaled_input_ad(334, A::sub_from_scalar(1.0, A::square(s.ad_value(2552))), 1.0 / (2.0));
                s.store_mul(223, 116, 334);
                s.store_mul(2553, 154, 334);
            }
            s.b[2558] = (s.v[116] < 0.0);
            s.store_scalar(2558, if s.b[2558] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2557]) && s.b[2558]) {
                s.store_neg(223, 223);
                s.store_neg(2553, 2553);
            }
            s.b[2559] = (((s.v[116]) as f64).abs() < 0.005);
            s.store_scalar(2559, if s.b[2559] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2557])) && s.b[2559]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 2550, 1.0, 2550, 1.0, 2550, 1.0, 2550, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 2550, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2550), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2550), 1.0, A::scale(s.ad_value(2550), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(223, 334, 336);
                s.store_div_scaled_product_right_ad(2553, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(2552), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2557])) && (!s.b[2559])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 2550);
                s.store_sqrt_ad(223, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(2550), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
                s.store_div_scaled_product_right_ad(2553, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(2552), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            s.b[2560] = ((s.v[79] == 1.0) && (s.v[116] < 0.0));
            s.store_scalar(2560, if s.b[2560] { 1.0 } else { 0.0 });
            let (assign65310_body31_e101490,) = {
    if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2560]) {
        let assign65310_body31_e101488: f64 = (-1.0);
        (assign65310_body31_e101488,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, assign65310_body31_e101490);
            s.b[2561] = (s.v[116] < 0.0);
            s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2561]) {
                s.store_neg(216, 223);
                s.store_neg(217, 2553);
            }
            s.b[2562] = (s.v[116] < 1e-7);
            s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2561])) && s.b[2562]) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 2553);
            }
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2561])) && (!s.b[2562])) {
                s.store_mul_offset_rhs(117, 154, 2546, (-p.p456));
                s.store_exp(228, 117);
                s.store_mul_ad_rhs(214, 210, A::add_scaled_offset_product_rhs(s.ad_value(228), 1.0, s.ad_value(230), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 210, 154, A::sub(s.ad_value(228), s.ad_value(230)));
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(2553), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
                s.store_add_scaled_inputs_product_indices(232, 2546, 1.0, 2525, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2563] = (s.v[79] == 1.0);
            s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });
            let (assign65310_body47_e101795,) = {
    if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2563]) {
        let assign65310_body47_e101793: f64 = (s.v[421] + 1.0);
        (assign65310_body47_e101793,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign65310_body47_e101795);
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[2546]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(2546))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2564] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) && s.b[2564]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {
                s.store_add(2546, 2546, 236);
            }
            s.b[2565] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });
            let (assign65310_body54_e101928,) = {
    if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) && s.b[2565]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign65310_body54_e101928);
            let (assign65310_body55_e101945,) = {
    if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
        let assign65310_body55_e101943: f64 = (s.v[97] + 1.0);
        (assign65310_body55_e101943,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign65310_body55_e101945);
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
            s.copy_ad(2523, 2546);
        }

    }

    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
        var_weffcv_nf: f64,
    ) {
        if ((!s.b[1443]) && s.b[2519]) {
            s.store_mul_sub_scaled_inputs_rhs(339, 154, s.ad_value(2523), -1.0, s.ad_value(993), -1.0);
            s.store_abs(2535, 339);
            s.store_exp(340, 339);
            s.store_sub_offset_lhs(341, 340, (-1.0), 339);
        }

        s.b[2566] = (s.v[339] > 1e-7);
        s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2519]) && s.b[2566]) {
            s.store_mul_scaled_sqrt_rhs(2537, 209, -1.0, 341);
        }

        s.b[2567] = (s.v[2535] > 1e-7);
        s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2566])) && s.b[2567]) {
            s.store_mul_sqrt_rhs(2537, 209, 341);
        }

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2566])) && (!s.b[2567])) {
            s.store_mul_scaled_sqrt_ad_rhs(2537, 339, (-0.7071067811865475), A::offset(A::mul_scaled_lhs(s.ad_value(2535), 0.3333333333333333, A::scale_offset(s.ad_value(2535), 0.25, 1.0)), 1.0));
        }

        if ((!s.b[1443]) && s.b[2519]) {
            s.store_sqrt_square_offset(781, 2537, ((4.0 * 1e-6) * 1e-6));
            s.store_scaled_add(2532, 2537, 781, 0.5);
            s.store_div_scaled_inputs_indices(2533, 2532, 1.0, 586, 1.6021918e-19);
            s.store_offset(335, 2533, (-p.p452));
            s.store_scale(2534, 2533, 0.01);
            s.store_sqrt_add_scaled_square_product(781, 335, 1.0, 2534, 2534, 4.0);
            s.store_scaled_add(336, 335, 781, 0.5);
            s.store_div_scaled_product_by_product(2531, s.ad_value(336), s.ad_value(336), 1.0, s.ad_value(2533), s.ad_value(2533), 1.0);
            s.store_add_scaled_product_left_ad(994, 993, 1.0, A::sub(s.ad_value(2523), s.ad_value(993)), 2531, 1.0);
            s.store_mul_sub_from_scalar_rhs_ad(333, A::exp(A::mul(s.ad_value(154), A::add_scaled_inputs3(s.ad_value(994), 1.0, s.ad_value(960), -1.0, s.ad_value(1435), 1.0))), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, s.ad_value(790))));
            s.store_scalar(2527, (((((2.0 * 1.6021918e-19) * s.v[489]) * 1.034943e-10)) as f64).sqrt());
            s.store_mul_sqrt_rhs(2528, 2527, 155);
            s.store_mul_sub_rhs(2521, 154, 994, 993);
        }

        s.b[2568] = ((s.v[2521] < (0.2 * s.v[154])) && ((0.2 * s.v[154]) >= 0.0));
        s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });

        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
            s.store_sub_scaled_inputs(781, 154, 0.2, 2521, 1.0);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 154, 154, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign65610_e102309,) = {
    if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign65610_e102309);

        let (assign65620_e102318,) = {
    if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65620_e102318);

        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2569] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });

        s.b[2570] = (1.0 == 1.0);
        s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });

        let (assign65710_e102409,) = {
    if (((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && s.b[2570]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65710_e102409);

        s.b[2571] = (1.0 == 2.0);
        s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });

        let (assign65730_e102428,) = {
    if ((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && s.b[2571]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65730_e102428);

        s.b[2572] = (1.0 == 4.0);
        s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });

        let (assign65750_e102450,) = {
    if (((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && (!s.b[2571])) && s.b[2572]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65750_e102450);

        s.b[2573] = (1.0 == 8.0);
        s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });

        let (assign65770_e102475,) = {
    if ((((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && (!s.b[2571])) && (!s.b[2572])) && s.b[2573]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65770_e102475);

        let (assign65780_e102486,) = {
    if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign65780_e102486);

        let mut assign65790_loop_guard: usize = 0;
        while {
            let assign65790_cond_e102498: f64 = if (((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign65790_cond_e102498 != 0.0
        } {
            assign65790_loop_guard += 1;
            assert!(assign65790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) {
                s.store_sqrt(726, 726);
            }
            let (assign65790_body1_e102523,) = {
    if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) {
        let assign65790_body1_e102521: f64 = (s.v[719] + 1.0);
        (assign65790_body1_e102521,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign65790_body1_e102523);
        }

        if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && (!s.b[2569])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 154, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 154, 725, 726, 0.2, 770, 1.0);
            s.store_sub_scaled_inputs(335, 154, 0.2, 780, 1.0);
        }

        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2568])) {
            s.copy_ad(335, 2521);
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[1443]) && s.b[2519]) {
            s.store_sqrt_offset_input(2529, 335, (10.0 * 2.220446049250313e-16));
            s.store_mul(2530, 2528, 2529);
            s.store_mul_scaled_ad_lhs(995, A::div_scaled_inputs(s.ad_value(155), 2.0, s.ad_value(162), 1.0), 2530, p.p454);
            s.store_scaled_mul(46, 995, 333, s.v[632]);
            s.store_add(134, 136, 46);
        }

        if (!s.b[1443]) {
            s.store_add(134, 136, 46);
            s.copy_ad(978, 133);
        }

        s.store_scale(335, 162, (-var_weffcv_nf));

        s.store_mul(20, 335, 131);

        s.store_mul(132, 335, 133);

        s.store_mul(19, 132, 247);

        s.store_mul(979, 335, 978);

        s.store_scaled_sub(335, 790, 94, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p263)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(110, p.p263, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[2574] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });

        if s.b[2574] {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign66130_e102870,) = {
    if s.b[2574] {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign66130_e102870);

        let (assign66140_e102874,) = {
    if s.b[2574] {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66140_e102874);

        if s.b[2574] {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2575] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });

        s.b[2576] = (2.0 == 1.0);
        s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });

        let (assign66250_e102942,) = {
    if ((s.b[2574] && s.b[2575]) && s.b[2576]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66250_e102942);

        s.b[2577] = (2.0 == 2.0);
        s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });

        let (assign66270_e102956,) = {
    if (((s.b[2574] && s.b[2575]) && (!s.b[2576])) && s.b[2577]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66270_e102956);

        s.b[2578] = (2.0 == 4.0);
        s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });

        let (assign66290_e102973,) = {
    if ((((s.b[2574] && s.b[2575]) && (!s.b[2576])) && (!s.b[2577])) && s.b[2578]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66290_e102973);

        s.b[2579] = (2.0 == 8.0);
        s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });

        let (assign66310_e102993,) = {
    if (((((s.b[2574] && s.b[2575]) && (!s.b[2576])) && (!s.b[2577])) && (!s.b[2578])) && s.b[2579]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66310_e102993);

        let (assign66320_e102999,) = {
    if (s.b[2574] && s.b[2575]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign66320_e102999);

        let mut assign66330_loop_guard: usize = 0;
        while {
            let assign66330_cond_e103006: f64 = if ((s.b[2574] && s.b[2575]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66330_cond_e103006 != 0.0
        } {
            assign66330_loop_guard += 1;
            assert!(assign66330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[2574] && s.b[2575]) {
                s.store_sqrt(726, 726);
            }
            let (assign66330_body1_e103021,) = {
    if (s.b[2574] && s.b[2575]) {
        let assign66330_body1_e103019: f64 = (s.v[719] + 1.0);
        (assign66330_body1_e103019,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign66330_body1_e103021);
        }

        if (s.b[2574] && (!s.b[2575])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if s.b[2574] {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if s.b[2574] {
        }

        if (!s.b[2574]) {
        }

        if (!s.b[2574]) {
            s.store_scalar(334, 1.0);
        }

        s.store_add(109, 87, 110);

    }

    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
        p: &Parameters,
        var_cox0: f64,
        var_lgate: f64,
    ) {
        s.store_add_scaled_product_left_ad(134, 134, 1.0, A::div_from_scalar(s.v[163], s.ad_value(162)), 790, p.p435);

        s.b[2580] = (p.p23 == 0.0);
        s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });

        if s.b[2580] {
            s.store_scalar(280, 0.0);
            s.store_scalar(288, 0.0);
        }

        s.b[2581] = ((s.v[481] > 0.0) && (s.v[454] > 0.0));
        s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });

        if ((!s.b[2580]) && s.b[2581]) {
            s.store_mul(335, 659, 85);
            s.store_scale(337, 636, 1.0 / ((var_cox0 * var_cox0)));
            s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (var_cox0 * var_cox0));
            s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 660, 1438, (-1.0));
            s.store_offset_mul(340, 338, 339, 1.0);
            s.store_scaled_offset(341, 338, 1.0, 2.0);
        }

        s.b[2582] = ((s.v[340] < (1e-6 + s.v[341])) && (s.v[341] >= 0.0));
        s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });

        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
            s.store_sub_offset_lhs(781, 341, 1e-6, 340);
            s.store_square(722, 781);
            s.store_square(723, 341);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign66600_e103262,) = {
    if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign66600_e103262);

        let (assign66610_e103271,) = {
    if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66610_e103271);

        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2583] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });

        s.b[2584] = (4.0 == 1.0);
        s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });

        let (assign66760_e103428,) = {
    if (((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && s.b[2584]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66760_e103428);

        s.b[2585] = (4.0 == 2.0);
        s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });

        let (assign66780_e103447,) = {
    if ((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && s.b[2585]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66780_e103447);

        s.b[2586] = (4.0 == 4.0);
        s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });

        let (assign66800_e103469,) = {
    if (((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && (!s.b[2585])) && s.b[2586]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66800_e103469);

        s.b[2587] = (4.0 == 8.0);
        s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });

        let (assign66820_e103494,) = {
    if ((((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && (!s.b[2585])) && (!s.b[2586])) && s.b[2587]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66820_e103494);

        let (assign66830_e103505,) = {
    if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign66830_e103505);

        let mut assign66840_loop_guard: usize = 0;
        while {
            let assign66840_cond_e103517: f64 = if (((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66840_cond_e103517 != 0.0
        } {
            assign66840_loop_guard += 1;
            assert!(assign66840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) {
                s.store_sqrt(726, 726);
            }
            let (assign66840_body1_e103542,) = {
    if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) {
        let assign66840_body1_e103540: f64 = (s.v[719] + 1.0);
        (assign66840_body1_e103540,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign66840_body1_e103542);
        }

        if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && (!s.b[2583])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 341, 726);
            s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);
            s.store_sub_offset_lhs(340, 341, 1e-6, 780);
        }

        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
        }

        if (((!s.b[2580]) && s.b[2581]) && (!s.b[2582])) {
        }

        if (((!s.b[2580]) && s.b[2581]) && (!s.b[2582])) {
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[2580]) && s.b[2581]) {
            s.store_sqrt(340, 340);
            s.store_add_mul_sub_from_scalar_rhs_indices(282, 335, 337, 1.0, 340);
            s.store_div_from_scalar_offset_input(336, var_lgate, 661, var_lgate);
            s.store_add_scaled_inputs_product_indices(283, 1439, s.v[483], 109, 1.0, 336, 282, (-1.0));
            s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);
            s.store_scaled_add(283, 283, 782, 0.5);
        }

        s.b[2588] = (s.v[283] < 0.0);
        s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });

        if (((!s.b[2580]) && s.b[2581]) && s.b[2588]) {
            s.store_scalar(283, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2580]) && s.b[2581]) {
            s.store_offset(283, 283, 1e-25);
            s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);
        }

        if ((!s.b[2580]) && s.b[2581]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }

        if ((!s.b[2580]) && s.b[2581]) {
            s.store_div(339, 662, 958);
            s.store_mul(340, 663, 958);
            s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));
            s.store_mul_product3_indices(280, 336, 339, 283, 134, 1.0);
            s.store_mul3_lhs(288, 339, 283, 336);
        }

        if ((!s.b[2580]) && (!s.b[2581])) {
            s.store_scalar(280, 0.0);
        }

        s.b[2589] = (s.v[664] != 0.0);
        s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });

        if ((!s.b[2580]) && s.b[2589]) {
            s.copy_ad(334, 799);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(335, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[2590] = (s.v[334] < 0.0);
        s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });

        if (((!s.b[2580]) && s.b[2589]) && s.b[2590]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(335, 0.0);
        }

        if ((!s.b[2580]) && s.b[2589]) {
            s.store_sqrt_offset_input(335, 127, 1e-25);
            s.store_div_from_scalar_scaled_input(337, 1.0, 335, 2.0);
            s.store_sub_ad_rhs(338, 334, A::scale_offset(s.ad_value(791), ((p.p106) * (p.p105)), p.p105));
            s.store_sqrt_square_offset(782, 338, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 338, 782, 0.5, 0.5);
            s.store_scaled_add(338, 338, 782, 0.5);
        }

        s.b[2591] = (s.v[338] < 0.0);
        s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });

        if (((!s.b[2580]) && s.b[2589]) && s.b[2591]) {
            s.store_scalar(338, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2580]) && s.b[2589]) {
            s.store_offset(338, 338, 1e-25);
            s.store_mul_ad_product_rhs_mixed_ia(344, 450, 451, A::exp(A::div_from_scalar((-1.0), s.ad_value(338))));
            s.store_mul_offset_ad_rhs(345, 344, A::div_from_scalar(1.0, s.ad_value(338)), 1.0);
            s.store_mul(337, 338, 344);
            s.store_sub(334, 334, 337);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[2592] = (s.v[334] < 0.0);
        s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });

        if (((!s.b[2580]) && s.b[2589]) && s.b[2592]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2580]) && s.b[2589]) {
            s.store_offset(334, 334, 1e-25);
            s.store_div_from_scalar_mul_ad(338, 1.0, s.ad_value(334), s.ad_value(335));
            s.store_scalar(341, (s.v[165] * s.v[554]));
            s.store_exp_mul_scaled_lhs_indices(336, 341, -1.0, 338);
            s.store_mul_product3_indices(340, 338, 341, 336, 338, 1.0);
            s.store_mul_product3_indices(281, 336, 664, 134, 334, 1.0);
        }

        s.b[2593] = (p.p45 == 0.0);
        s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });

        if s.b[2593] {
            s.store_scalar(423, 0.0);
        }

        s.b[2594] = ((p.p45 * (s.v[796] - p.p446)) < 0.0);
        s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });

        if ((!s.b[2593]) && s.b[2594]) {
            s.copy_ad(426, 427);
        }

        if ((!s.b[2593]) && (!s.b[2594])) {
            s.store_add_scaled_inputs_ad_lhs(426, A::square(A::offset(s.ad_value(796), (-p.p446))), p.p445, 427, 1.0);
        }

        if (!s.b[2593]) {
            s.store_scaled_limited_exp_ad(423, A::mul(s.ad_value(154), A::sub(s.ad_value(793), s.ad_value(426))), p.p449);
        }

        s.b[2595] = (s.v[423] > 0.0);
        s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });

        s.b[2596] = ((s.v[423] > (100000.0 - 50000.0)) && (50000.0 >= 0.0));
        s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });

        if (s.b[2595] && s.b[2596]) {
            s.store_offset(781, 423, (((-100000.0)) + (50000.0)));
            s.store_square(722, 781);
            s.store_scalar(723, (50000.0 * 50000.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
        var_cox0: f64,
        var_mfactor: f64,
        var_tox0: f64,
    ) {
        let (assign67590_e104304,) = {
    if (s.b[2595] && s.b[2596]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign67590_e104304);

        let (assign67600_e104310,) = {
    if (s.b[2595] && s.b[2596]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67600_e104310);

        if (s.b[2595] && s.b[2596]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2597] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });

        s.b[2598] = (1.0 == 1.0);
        s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });

        let (assign67690_e104380,) = {
    if (((s.b[2595] && s.b[2596]) && s.b[2597]) && s.b[2598]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67690_e104380);

        s.b[2599] = (1.0 == 2.0);
        s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });

        let (assign67710_e104396,) = {
    if ((((s.b[2595] && s.b[2596]) && s.b[2597]) && (!s.b[2598])) && s.b[2599]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67710_e104396);

        s.b[2600] = (1.0 == 4.0);
        s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });

        let (assign67730_e104415,) = {
    if (((((s.b[2595] && s.b[2596]) && s.b[2597]) && (!s.b[2598])) && (!s.b[2599])) && s.b[2600]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67730_e104415);

        s.b[2601] = (1.0 == 8.0);
        s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });

        let (assign67750_e104437,) = {
    if ((((((s.b[2595] && s.b[2596]) && s.b[2597]) && (!s.b[2598])) && (!s.b[2599])) && (!s.b[2600])) && s.b[2601]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67750_e104437);

        let (assign67760_e104445,) = {
    if ((s.b[2595] && s.b[2596]) && s.b[2597]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign67760_e104445);

        let mut assign67770_loop_guard: usize = 0;
        while {
            let assign67770_cond_e104454: f64 = if (((s.b[2595] && s.b[2596]) && s.b[2597]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign67770_cond_e104454 != 0.0
        } {
            assign67770_loop_guard += 1;
            assert!(assign67770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[2595] && s.b[2596]) && s.b[2597]) {
                s.store_sqrt(726, 726);
            }
            let (assign67770_body1_e104473,) = {
    if ((s.b[2595] && s.b[2596]) && s.b[2597]) {
        let assign67770_body1_e104471: f64 = (s.v[719] + 1.0);
        (assign67770_body1_e104471,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign67770_body1_e104473);
        }

        if ((s.b[2595] && s.b[2596]) && (!s.b[2597])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (s.b[2595] && s.b[2596]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 50000.0);
            s.store_div_scaled_product_indices(334, 725, 726, 50000.0, 770, 1.0);
            s.store_offset(336, 780, (100000.0 - 50000.0));
        }

        if (s.b[2595] && s.b[2596]) {
        }

        if (s.b[2595] && (!s.b[2596])) {
            s.copy_ad(336, 423);
            s.store_scalar(334, 1.0);
        }

        if s.b[2595] {
            s.store_scale(422, 336, (var_mfactor * s.v[632]));
        }

        if (!s.b[2595]) {
            s.store_scalar(422, 0.0);
        }

        s.b[2602] = ((((s.v[280] + s.v[281]) > 0.0) && (s.v[523] != 0.0)) && (s.v[963] == 0.0));
        s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });

        if s.b[2602] {
            s.store_offset_scaled(334, 120, s.v[524], 1.0);
            s.store_add(335, 280, 281);
            s.store_scaled_mul(111, 334, 335, s.v[523]);
            s.store_div_from_scalar(344, 1.0, 99);
            s.store_mul3_lhs(335, 154, 111, 344);
            s.store_square(345, 344);
            s.store_div_from_scalar(344, 1.0, 102);
            s.store_mul3_lhs(336, 154, 111, 344);
            s.store_square(345, 344);
            s.store_mul_ad_rhs(112, 209, A::add_scaled_products(s.ad_value(104), s.ad_value(336), 1.0, s.ad_value(101), s.ad_value(335), (-1.0)));
            s.store_mul_add_scaled_products_indices_rhs(113, 209, 103, 336, ((-1.0) * (0.5)), 100, 335, 0.5);
            s.store_add(114, 112, 113);
            s.store_mul3_lhs(400, 115, 114, 253);
            s.store_mul(287, 288, 400);
        }

        s.b[2603] = (p.p24 != 0.0);
        s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });

        s.b[2604] = (s.v[78] == 0.0);
        s.store_scalar(2604, if s.b[2604] { 1.0 } else { 0.0 });

        if (s.b[2603] && s.b[2604]) {
            s.store_offset_add(191, 109, 1439, (-(10.0 * 2.220446049250313e-16)));
            s.store_sub_scaled_ad_lhs(335, A::add_scaled_product(A::offset(s.ad_value(1440), (-s.v[160])), 1.0, A::sub(s.ad_value(120), s.ad_value(182)), s.ad_value(162), s.v[560]), 191, s.v[515]);
            s.store_square(335, 335);
            s.store_scalar(337, (1.0 / var_tox0));
            s.store_mul(336, 335, 337);
            s.store_scalar(337, (1.0 / s.v[561]));
            s.store_offset_mul(341, 255, 337, 1.0);
            s.store_mul(195, 336, 341);
            s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);
            s.store_scaled_add(195, 195, 782, 0.5);
        }

        s.b[2605] = (s.v[195] < 0.0);
        s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });

        if ((s.b[2603] && s.b[2604]) && s.b[2605]) {
            s.store_scalar(195, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (s.b[2603] && s.b[2604]) {
            s.store_sqrt_square_offset(782, 1440, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(338, 1440, 782, 0.5, 0.5);
            s.store_scaled_add(337, 1440, 782, 0.5);
        }

        s.b[2606] = (s.v[337] < 0.0);
        s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });

        if ((s.b[2603] && s.b[2604]) && s.b[2606]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if (s.b[2603] && s.b[2604]) {
            s.store_offset(337, 337, (-p.p262));
            s.store_scale(332, 337, 10.0);
            s.store_offset_square(336, 332, 1.0);
            s.store_sub_from_scalar_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(336)));
            s.store_mul(195, 195, 335);
            s.store_scale(334, 162, s.v[632]);
            s.store_div_from_scalar_offset_input(341, s.v[562], 334, s.v[562]);
            s.store_scalar(340, s.v[516]);
            s.store_div_add_scaled_inputs_rhs_indices(343, 340, 340, 1.0, 1439, 1.0);
            s.store_div_from_scalar_offset_input(338, 1.0, 195, 1e-25);
            s.store_scaled_mul(335, 193, 338, (-s.v[514]));
            s.store_scaled_mul(337, 338, 338, s.v[514]);
        }

        s.b[2607] = (s.v[335] < (-34.0));
        s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });

        if ((s.b[2603] && s.b[2604]) && s.b[2607]) {
            s.store_scalar(199, 0.0);
        }

        if ((s.b[2603] && s.b[2604]) && (!s.b[2607])) {
            s.store_exp(336, 335);
            s.store_mul_scale_ad_lhs(337, A::div_from_scalar(s.v[513], s.ad_value(192)), 1.6021918e-19, 334);
            s.store_div_from_scalar(339, 1.0, 209);
            s.store_sqrt_ad(340, A::mul_offset_lhs(s.ad_value(978), (var_cox0 * 1e-12), s.ad_value(339)));
            s.store_mul3_lhs(338, 336, 337, 340);
            s.store_mul(339, 338, 195);
            s.store_mul(344, 339, 195);
            s.store_mul3_lhs(199, 341, 343, 344);
        }

        if s.b[2603] {
            s.store_offset_scaled(334, 791, (-s.v[518]), s.v[559]);
            s.store_exp_scaled_input(336, 334, var_tox0);
            s.store_scale(334, 791, (1.0 / (var_tox0) * 1.0 / (var_tox0)));
            s.store_mul(337, 791, 334);
            s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));
            s.store_mul3_lhs(200, 338, 336, 337);
        }

        s.b[2608] = (s.v[791] >= 0.0);
        s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });

        if (s.b[2603] && s.b[2608]) {
            s.store_scale(200, 200, (-1.0));
        }

        if s.b[2603] {
            s.store_sub(335, 791, 790);
            s.store_offset_scaled(334, 335, (-s.v[518]), s.v[559]);
            s.store_exp_scaled_input(336, 334, var_tox0);
            s.store_scale(334, 335, (1.0 / (var_tox0) * 1.0 / (var_tox0)));
            s.store_mul(337, 335, 334);
            s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));
            s.store_mul3_lhs(201, 338, 336, 337);
        }

        s.b[2609] = (s.v[335] >= 0.0);
        s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });

        if (s.b[2603] && s.b[2609]) {
            s.store_scale(201, 201, (-1.0));
        }

        if s.b[2603] {
            s.store_scaled_offset_ad(195, A::neg(A::sub(s.ad_value(791), s.ad_value(792))), ((s.v[160]) + (p.p258)), 1.0 / (var_tox0));
            s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);
            s.store_scaled_add(195, 195, 782, 0.5);
        }

        s.b[2610] = (s.v[195] < 0.0);
        s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });

        if (s.b[2603] && s.b[2610]) {
            s.store_scalar(195, 0.0);
            s.store_scalar(339, 0.0);
        }

        if s.b[2603] {
            s.store_offset(195, 195, 1e-25);
            s.store_div_from_scalar(335, (-s.v[520]), 195);
        }

        s.b[2611] = (s.v[335] < (-34.0));
        s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });

        if (s.b[2603] && s.b[2611]) {
            s.store_scalar(202, 0.0);
        }

        if (s.b[2603] && (!s.b[2611])) {
            s.store_exp(336, 335);
        }

    }

    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
        var_cox0: f64,
        var_coxb0: f64,
        var_lgate: f64,
        var_tox0: f64,
        var_uc_nover: f64,
        var_uc_novers: f64,
        var_flg_coovlp_slot: &mut f64,
        var_flg_coovlps_slot: &mut f64,
        var_guard1631_slot: &mut f64,
        var_guard1632_slot: &mut f64,
        var_guard1633_slot: &mut f64,
        var_guard1635_slot: &mut f64,
        var_guard1637_slot: &mut f64,
    ) {
        let mut var_flg_coovlp: f64 = *var_flg_coovlp_slot;
        let mut var_flg_coovlps: f64 = *var_flg_coovlps_slot;
        let mut var_guard1631: f64 = *var_guard1631_slot;
        let mut var_guard1632: f64 = *var_guard1632_slot;
        let mut var_guard1633: f64 = *var_guard1633_slot;
        let mut var_guard1635: f64 = *var_guard1635_slot;
        let mut var_guard1637: f64 = *var_guard1637_slot;

        if (s.b[2603] && (!s.b[2611])) {
            s.store_mul_div_from_scalar_ad_lhs(337, s.v[520], A::square(s.ad_value(195)), 336);
            s.store_scale(337, 162, (s.v[519] * s.v[632]));
            s.store_mul_product3_indices(202, 336, 337, 195, 195, 1.0);
        }

        if s.b[2603] {
            s.copy_ad(285, 677);
            s.store_mul(286, 393, 285);
            s.store_scaled_offset_ad(336, A::add_scaled_inputs4(s.ad_value(1438), s.v[493], s.ad_value(1440), (-1.0), s.ad_value(122), 1.0, s.ad_value(174), 1.0), (-s.v[492]), (-1.0 / (var_tox0)));
            s.store_square(334, 336);
            s.store_scale(335, 286, s.v[491]);
            s.store_div_scaled_inputs_indices(337, 335, -1.0, 336, 1.0);
        }

        s.b[2612] = (s.v[337] < (-34.0));
        s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });

        if (s.b[2603] && s.b[2612]) {
            s.store_scalar(339, 0.0);
        }

        if (s.b[2603] && (!s.b[2612])) {
            s.store_exp(339, 337);
        }

        if s.b[2603] {
            s.store_div_from_scalar(338, (((1.6021918e-19 * s.v[490]) * s.v[632]) * var_lgate), 285);
        }

        s.b[2613] = (((2.0 * s.v[336]) + s.v[335]) < 0.0);
        s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });

        if (s.b[2603] && s.b[2613]) {
            s.store_mul3_affine_lhs(284, 338, 335, (0.25 * 7.38905609893065), 0.0, 335);
        }

        if (s.b[2603] && (!s.b[2613])) {
            s.store_mul3_lhs(284, 338, 334, 339);
        }

        if s.b[2603] {
            s.store_sub(202, 202, 284);
        }

        s.b[2614] = (p.p25 != 0.0);
        s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });

        if s.b[2614] {
            s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(790), 1.0, A::scale(s.ad_value(790), 100.0)), (-1e-5));
            s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 790, (4.0 * 1e-5));
            s.store_add_scaled_inputs3_indices(196, 790, 1.0, 335, (-0.5), 336, (-0.5));
        }

        s.b[2615] = (p.p25 == 0.0);
        s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });

        if s.b[2615] {
            s.store_scalar(203, 0.0);
        }

        if (!s.b[2615]) {
            s.store_add_scaled_inputs4_offset_indices(335, 196, p.p242, 791, (-1.0), 122, p.p244, 174, p.p244, (p.p243 * p.p242));
            s.store_scalar(336, (1.0 / var_tox0));
            s.store_mul(194, 335, 336);
            s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);
            s.store_scaled_add(197, 194, 782, 0.5);
        }

        s.b[2616] = (s.v[197] < 0.0);
        s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });

        if ((!s.b[2615]) && s.b[2616]) {
            s.store_scalar(197, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (!s.b[2615]) {
            s.store_div_from_scalar_offset_input(337, 1.0, 197, 1e-25);
            s.store_scaled_mul(334, 193, 337, (-s.v[512]));
        }

        s.b[2617] = (s.v[334] < (-34.0));
        s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });

        if ((!s.b[2615]) && s.b[2617]) {
            s.store_scalar(203, 0.0);
        }

        if ((!s.b[2615]) && (!s.b[2617])) {
            s.store_exp(335, 334);
            s.store_scale_ad(336, A::div_from_scalar(s.v[511], s.ad_value(192)), (1.6021918e-19 * s.v[632]));
            s.store_mul_product3_indices(203, 335, 336, 197, 197, 1.0);
        }

        if (!s.b[2615]) {
            s.store_sub(205, 790, 792);
        }

        s.b[2618] = (s.v[205] > 0.0);
        s.store_scalar(2618, if s.b[2618] { 1.0 } else { 0.0 });

        if ((!s.b[2615]) && s.b[2618]) {
            s.store_square(336, 205);
            s.store_mul(338, 336, 205);
            s.store_offset(334, 338, 0.5);
            s.store_div(339, 338, 334);
            s.store_div_ad(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), A::square(s.ad_value(334)));
            s.store_mul(203, 203, 339);
        }

        if ((!s.b[2615]) && (!s.b[2618])) {
            s.store_scalar(203, 0.0);
        }

        s.b[2619] = (p.p25 == 0.0);
        s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });

        if s.b[2619] {
            s.store_scalar(204, 0.0);
        }

        if (!s.b[2619]) {
            s.store_add_scaled_inputs3_mixed_aii(335, A::add_scaled_inputs3_offset(s.ad_value(196), (-p.p242), s.ad_value(791), -1.0, s.ad_value(196), 1.0, ((p.p243) * (p.p242))), 1.0, 122, p.p244, 174, p.p244);
            s.store_scalar(336, (1.0 / var_tox0));
            s.store_mul(194, 335, 336);
            s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);
            s.store_scaled_add(198, 194, 782, 0.5);
        }

        s.b[2620] = (s.v[198] < 0.0);
        s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });

        if ((!s.b[2619]) && s.b[2620]) {
            s.store_scalar(198, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (!s.b[2619]) {
            s.store_div_from_scalar_offset_input(337, 1.0, 198, 1e-25);
            s.store_scaled_mul(334, 193, 337, (-s.v[512]));
        }

        s.b[2621] = (s.v[334] < (-34.0));
        s.store_scalar(2621, if s.b[2621] { 1.0 } else { 0.0 });

        if ((!s.b[2619]) && s.b[2621]) {
            s.store_scalar(204, 0.0);
        }

        if ((!s.b[2619]) && (!s.b[2621])) {
            s.store_exp(335, 334);
            s.store_div_from_scalar(337, 1.0, 192);
            s.store_scale(336, 337, (s.v[511] * (1.6021918e-19 * s.v[632])));
            s.store_mul_product3_indices(204, 335, 336, 198, 198, 1.0);
        }

        if (!s.b[2619]) {
            s.store_neg(206, 792);
        }

        s.b[2622] = (s.v[206] > 0.0);
        s.store_scalar(2622, if s.b[2622] { 1.0 } else { 0.0 });

        if ((!s.b[2619]) && s.b[2622]) {
            s.store_square(336, 206);
            s.store_mul(338, 336, 206);
            s.store_offset(334, 338, 0.5);
            s.store_div(339, 338, 334);
            s.store_div_ad(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), A::square(s.ad_value(334)));
            s.store_mul(204, 204, 339);
        }

        if ((!s.b[2619]) && (!s.b[2622])) {
            s.store_scalar(204, 0.0);
        }

        var_flg_coovlps = 0.0;

        var_flg_coovlp = 0.0;

        s.store_scalar(2625, 0.0);

        s.store_scalar(406, 0.0);

        s.store_scalar(2625, 0.0);

        let assign69580_e106027: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1631 = assign69580_e106027;

        let assign69590_e106030: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1632 = assign69590_e106030;

        let assign69600_e106033: f64 = if 1.0 == 3.0 { 1.0 } else { 0.0 };
        var_guard1633 = assign69600_e106033;

        s.b[2630] = (1.0 == 4.0);
        s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });

        let assign69620_e106047: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        var_guard1635 = assign69620_e106047;

        let (assign69630_e106053,) = {
    if ((var_guard1631 != 0.0) && (var_guard1635 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign69630_e106053);

        let (assign69640_e106059,) = {
    if ((var_guard1631 != 0.0) && (var_guard1635 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlps,)
    }
};
        var_flg_coovlps = assign69640_e106059;

        if ((s.v[2627] != 0.0) && (s.v[2631] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, var_uc_novers);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, var_cox0);
        }

        s.b[2632] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2632, if s.b[2632] { 1.0 } else { 0.0 });

        let (assign69730_e106132,) = {
    if (((var_guard1632 != 0.0) && (var_guard1631 == 0.0)) && s.b[2632]) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign69730_e106132);

        if (((s.v[2628] != 0.0) && (s.v[2627] == 0.0)) && s.b[2632]) {
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        let assign69760_e106164: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        var_guard1637 = assign69760_e106164;

        let (assign69770_e106175,) = {
    if (((var_guard1633 != 0.0) && (!((var_guard1631 != 0.0) || (var_guard1632 != 0.0)))) && (var_guard1637 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign69770_e106175);

        let (assign69780_e106186,) = {
    if (((var_guard1633 != 0.0) && (!((var_guard1631 != 0.0) || (var_guard1632 != 0.0)))) && (var_guard1637 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlp,)
    }
};
        var_flg_coovlp = assign69780_e106186;

        if (((s.v[2629] != 0.0) && (!((s.v[2627] != 0.0) || (s.v[2628] != 0.0)))) && (s.v[2633] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, var_uc_nover);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.store_scalar(413, var_coxb0);
            s.store_neg(407, 407);
        }

        s.b[2634] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.store_scalar(2634, if s.b[2634] { 1.0 } else { 0.0 });

        if ((((s.v[2629] != 0.0) && (!((s.v[2627] != 0.0) || (s.v[2628] != 0.0)))) && (s.v[2633] != 0.0)) && s.b[2634]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2635] = (p.p113 > 0.0);
        s.store_scalar(2635, if s.b[2635] { 1.0 } else { 0.0 });

        s.b[2636] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.store_scalar(2636, if s.b[2636] { 1.0 } else { 0.0 });

        if ((((((s.v[2629] != 0.0) && (!((s.v[2627] != 0.0) || (s.v[2628] != 0.0)))) && (s.v[2633] != 0.0)) && s.b[2634]) && s.b[2635]) && s.b[2636]) {
        }

        if ((((((s.v[2629] != 0.0) && (!((s.v[2627] != 0.0) || (s.v[2628] != 0.0)))) && (s.v[2633] != 0.0)) && s.b[2634]) && s.b[2635]) && (!s.b[2636])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if ((((((s.v[2629] != 0.0) && (!((s.v[2627] != 0.0) || (s.v[2628] != 0.0)))) && (s.v[2633] != 0.0)) && s.b[2634]) && s.b[2635]) && (!s.b[2636])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if (((((s.v[2629] != 0.0) && (!((s.v[2627] != 0.0) || (s.v[2628] != 0.0)))) && (s.v[2633] != 0.0)) && s.b[2634]) && s.b[2635]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2637] = (s.v[336] < 0.0);
        s.store_scalar(2637, if s.b[2637] { 1.0 } else { 0.0 });

        if ((((((s.v[2629] != 0.0) && (!((s.v[2627] != 0.0) || (s.v[2628] != 0.0)))) && (s.v[2633] != 0.0)) && s.b[2634]) && s.b[2635]) && s.b[2637]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((((s.v[2629] != 0.0) && (!((s.v[2627] != 0.0) || (s.v[2628] != 0.0)))) && (s.v[2633] != 0.0)) && s.b[2634]) && s.b[2635]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub(407, 407, 600);
        }


        *var_flg_coovlp_slot = var_flg_coovlp;
        *var_flg_coovlps_slot = var_flg_coovlps;
        *var_guard1631_slot = var_guard1631;
        *var_guard1632_slot = var_guard1632;
        *var_guard1633_slot = var_guard1633;
        *var_guard1635_slot = var_guard1635;
        *var_guard1637_slot = var_guard1637;
    }

    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard1631: f64,
        var_guard1632: f64,
        var_guard1633: f64,
    ) {
        s.b[2638] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2638, if s.b[2638] { 1.0 } else { 0.0 });

        let (assign70080_e106657,) = {
    if ((s.b[2630] && (!(((var_guard1631 != 0.0) || (var_guard1632 != 0.0)) || (var_guard1633 != 0.0)))) && s.b[2638]) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign70080_e106657);

        if ((s.b[2630] && (!(((s.v[2627] != 0.0) || (s.v[2628] != 0.0)) || (s.v[2629] != 0.0)))) && s.b[2638]) {
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2625] != 0.0) {
            s.store_scalar(2646, 0.4);
        }

        let (assign70130_e106699,) = {
    if (s.v[2625] != 0.0) {
        (0.0,)
    } else {
        (s.v[2647],)
    }
};
        s.store_scalar(2647, assign70130_e106699);

        if (s.v[2625] != 0.0) {
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
        }

        let (assign70260_e106752,) = {
    if (s.v[2625] != 0.0) {
        let assign70260_e106750: f64 = (-1.0);
        (assign70260_e106750,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign70260_e106752);

        if (s.v[2625] != 0.0) {
            s.store_scalar(2648, 0.0);
            s.store_scalar(2649, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2644, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2644), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2625] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2625] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2645, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2651] = (s.v[2646] > (s.v[2645] * 0.5));
        s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2651]) {
            s.store_scale(2646, 2645, 0.5);
        }

        s.b[2652] = param_given[338];
        s.store_scalar(2652, if s.b[2652] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2652]) {
            s.store_scalar(2645, p.p338);
        }

        s.b[2653] = param_given[339];
        s.store_scalar(2653, if s.b[2653] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2653]) {
            s.store_scalar(2646, p.p339);
        }

        s.b[2654] = param_given[338];
        s.store_scalar(2654, if s.b[2654] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2653])) && s.b[2654]) {
            s.store_scale(2646, 2645, 0.5);
        }

        s.b[2655] = (s.v[2646] > (s.v[2645] * 0.5));
        s.store_scalar(2655, if s.b[2655] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2655]) {
            s.store_scale(2646, 2645, 0.5);
        }

        s.b[2656] = (p.p38 == 1.0);
        s.store_scalar(2656, if s.b[2656] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2656]) {
            s.store_neg(334, 396);
        }

        s.b[2657] = (s.v[334] > s.v[2646]);
        s.store_scalar(2657, if s.b[2657] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2656]) && s.b[2657]) {
            s.store_sub(335, 334, 2646);
            s.store_sub(336, 2645, 2646);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2646, 333);
        }

        if (((s.v[2625] != 0.0) && s.b[2656]) && (!s.b[2657])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2625] != 0.0) && s.b[2656]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2656])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2625] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign70670_e107093,) = {
    if (s.v[2625] != 0.0) {
        let assign70670_e107087: f64 = (-s.v[397]);
        let assign70670_e107090: f64 = (10.0 * 2.220446049250313e-16);
        let assign70670_e107091: f64 = (assign70670_e107087 + assign70670_e107090);
        (assign70670_e107091,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign70670_e107093);

        if (s.v[2625] != 0.0) {
            s.store_scalar(2640, 0.0);
            s.store_scale(2641, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2658] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2658]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2658])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign70770_loop_guard: usize = 0;
        while {
            let assign70770_cond_e107167: f64 = if (((s.v[2625] != 0.0) && (!s.b[2658])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign70770_cond_e107167 != 0.0
        } {
            assign70770_loop_guard += 1;
            assert!(assign70770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && (!s.b[2658])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[2658])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2659] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign70920_e107341,) = {
    if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign70920_e107341);

        let (assign70930_e107349,) = {
    if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign70930_e107349);

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2660] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2660, if s.b[2660] { 1.0 } else { 0.0 });

        s.b[2661] = (1.0 == 1.0);
        s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });

        let (assign71020_e107433,) = {
    if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && s.b[2660]) && s.b[2661]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign71020_e107433);

        s.b[2662] = (1.0 == 2.0);
        s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });

        let (assign71040_e107451,) = {
    if ((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && s.b[2660]) && (!s.b[2661])) && s.b[2662]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign71040_e107451);

        s.b[2663] = (1.0 == 4.0);
        s.store_scalar(2663, if s.b[2663] { 1.0 } else { 0.0 });

        let (assign71060_e107472,) = {
    if (((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && s.b[2660]) && (!s.b[2661])) && (!s.b[2662])) && s.b[2663]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign71060_e107472);

        s.b[2664] = (1.0 == 8.0);
        s.store_scalar(2664, if s.b[2664] { 1.0 } else { 0.0 });

        let (assign71080_e107496,) = {
    if ((((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && s.b[2660]) && (!s.b[2661])) && (!s.b[2662])) && (!s.b[2663])) && s.b[2664]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign71080_e107496);

        let (assign71090_e107506,) = {
    if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && s.b[2660]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign71090_e107506);

        let mut assign71100_loop_guard: usize = 0;
        while {
            let assign71100_cond_e107517: f64 = if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && s.b[2660]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign71100_cond_e107517 != 0.0
        } {
            assign71100_loop_guard += 1;
            assert!(assign71100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && s.b[2660]) {
                s.store_sqrt(726, 726);
            }
            let (assign71100_body1_e107540,) = {
    if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && s.b[2660]) {
        let assign71100_body1_e107538: f64 = (s.v[719] + 1.0);
        (assign71100_body1_e107538,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign71100_body1_e107540);
        }

        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) && (!s.b[2660])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

    }

    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) {
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2659]) {
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2659])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign71200_e107657,) = {
    if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
        let assign71200_e107651: f64 = (-s.v[397]);
        let assign71200_e107654: f64 = (10.0 * 2.220446049250313e-16);
        let assign71200_e107655: f64 = (assign71200_e107651 + assign71200_e107654);
        (assign71200_e107655,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign71200_e107657);

        s.b[2665] = (s.v[402] < s.v[403]);
        s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2665]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2666] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2665]) && s.b[2666]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2665]) && (!s.b[2666])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2625] != 0.0) && s.b[2665]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_div_lhs_indices(404, 335, 337, 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2648, 404);
        }

        s.b[2667] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2667]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && (!s.b[2667])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.v[2625] != 0.0) && (!s.b[2665])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2668] = (s.v[116] >= 3.0);
        s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2668]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && (!s.b[2668])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2669] = (p.p33 > 0.0);
        s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2670] = (p.p33 == 2.0);
        s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2670]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2670]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2670]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && (!s.b[2670])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) {
            s.copy_ad(445, 116);
        }

        s.b[2671] = (p.p33 == 2.0);
        s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });

        s.b[2672] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign72030_e108803,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign72030_e108803);

        let (assign72040_e108816,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72040_e108816);

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2673] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });

        s.b[2674] = (2.0 == 1.0);
        s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });

        let (assign72150_e108965,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && s.b[2673]) && s.b[2674]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72150_e108965);

        s.b[2675] = (2.0 == 2.0);
        s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });

        let (assign72170_e108988,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && s.b[2673]) && (!s.b[2674])) && s.b[2675]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72170_e108988);

        s.b[2676] = (2.0 == 4.0);
        s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });

        let (assign72190_e109014,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && s.b[2673]) && (!s.b[2674])) && (!s.b[2675])) && s.b[2676]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72190_e109014);

        s.b[2677] = (2.0 == 8.0);
        s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });

        let (assign72210_e109043,) = {
    if ((((((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && s.b[2673]) && (!s.b[2674])) && (!s.b[2675])) && (!s.b[2676])) && s.b[2677]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72210_e109043);

        let (assign72220_e109058,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && s.b[2673]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign72220_e109058);

        let mut assign72230_loop_guard: usize = 0;
        while {
            let assign72230_cond_e109074: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && s.b[2673]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign72230_cond_e109074 != 0.0
        } {
            assign72230_loop_guard += 1;
            assert!(assign72230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && s.b[2673]) {
                s.store_sqrt(726, 726);
            }
            let (assign72230_body1_e109107,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && s.b[2673]) {
        let assign72230_body1_e109105: f64 = (s.v[719] + 1.0);
        (assign72230_body1_e109105,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign72230_body1_e109107);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) && (!s.b[2673])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

    }

    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) {
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && s.b[2672]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && s.b[2671]) && (!s.b[2672])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2669]) && (!s.b[2671])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2678] = (p.p33 == 1.0);
        s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2679] = (s.v[411] > 0.0);
        s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && s.b[2679]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && (!s.b[2679])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2680] = (s.v[336] < 0.0);
        s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && (!s.b[2679])) && s.b[2680]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && (!s.b[2679])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2681] = (s.v[336] < 0.0);
        s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && s.b[2681]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2641, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2682] = (s.v[333] < 60.0);
        s.store_scalar(2682, if s.b[2682] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && s.b[2682]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && (!s.b[2682])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2683] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });

        let (assign72660_e109696,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && s.b[2683]) {
        let assign72660_e109694: f64 = (s.v[2647] + 1.0);
        (assign72660_e109694,)
    } else {
        (s.v[2647],)
    }
};
        s.store_scalar(2647, assign72660_e109696);

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2678]) && s.b[2683]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2665])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2684] = (((s.v[116]) as f64).abs() > 1e-6);
        s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2684]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && (!s.b[2684])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2625] != 0.0) && (!s.b[2665])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2685, 354, 2641);
        }

        s.b[2687] = (p.p33 == 2.0);
        s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });

        s.b[2688] = ((s.v[2685] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) {
            s.store_add_scaled_inputs3_indices(781, 2685, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign72840_e109903,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign72840_e109903);

        let (assign72850_e109914,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72850_e109914);

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2689] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });

        s.b[2690] = (2.0 == 1.0);
        s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });

        let (assign72960_e110045,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && s.b[2689]) && s.b[2690]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72960_e110045);

        s.b[2691] = (2.0 == 2.0);
        s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });

        let (assign72980_e110066,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && s.b[2689]) && (!s.b[2690])) && s.b[2691]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72980_e110066);

        s.b[2692] = (2.0 == 4.0);
        s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });

        let (assign73000_e110090,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && s.b[2689]) && (!s.b[2690])) && (!s.b[2691])) && s.b[2692]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73000_e110090);

        s.b[2693] = (2.0 == 8.0);
        s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });

        let (assign73020_e110117,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && s.b[2689]) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2692])) && s.b[2693]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73020_e110117);

        let (assign73030_e110130,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && s.b[2689]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign73030_e110130);

        let mut assign73040_loop_guard: usize = 0;
        while {
            let assign73040_cond_e110144: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && s.b[2689]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73040_cond_e110144 != 0.0
        } {
            assign73040_loop_guard += 1;
            assert!(assign73040_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && s.b[2689]) {
                s.store_sqrt(726, 726);
            }
            let (assign73040_body1_e110173,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && s.b[2689]) {
        let assign73040_body1_e110171: f64 = (s.v[719] + 1.0);
        (assign73040_body1_e110171,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign73040_body1_e110173);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) && (!s.b[2689])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2688]) {
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && (!s.b[2688])) {
            s.copy_ad(335, 2685);
            s.store_scalar(334, 1.0);
        }

        s.b[2694] = (s.v[334] < 1.0);
        s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });

        let (assign73140_e110315,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2687]) && s.b[2694]) {
        let assign73140_e110313: f64 = (s.v[2647] + 2.0);
        (assign73140_e110313,)
    } else {
        (s.v[2647],)
    }
};
        s.store_scalar(2647, assign73140_e110315);

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && (!s.b[2687])) {
            if (s.v[2685] <= s.v[386]) {
                s.copy_ad(335, 2685);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2695] = (s.v[2685] >= s.v[386]);
        s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });

        let (assign73170_e110347,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2665])) && (!s.b[2687])) && s.b[2695]) {
        let assign73170_e110345: f64 = (s.v[2647] + 2.0);
        (assign73170_e110345,)
    } else {
        (s.v[2647],)
    }
};
        s.store_scalar(2647, assign73170_e110347);

        s.b[2696] = (s.v[2647] >= 2.0);
        s.store_scalar(2696, if s.b[2696] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) {
            s.copy_ad(2686, 404);
            s.store_mul(354, 335, 2641);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2697] = (p.p33 == 2.0);
        s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });

        s.b[2698] = ((s.v[404] > (s.v[2686] - 0.1)) && (0.1 >= 0.0));
        s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
            s.store_offset_sub(781, 404, 2686, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign73290_e110481,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign73290_e110481);

        let (assign73300_e110494,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73300_e110494);

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
        }

    }

    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2699] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });

        s.b[2700] = (2.0 == 1.0);
        s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });

        let (assign73410_e110643,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && s.b[2699]) && s.b[2700]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73410_e110643);

        s.b[2701] = (2.0 == 2.0);
        s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });

        let (assign73430_e110666,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && s.b[2699]) && (!s.b[2700])) && s.b[2701]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73430_e110666);

        s.b[2702] = (2.0 == 4.0);
        s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });

        let (assign73450_e110692,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && s.b[2699]) && (!s.b[2700])) && (!s.b[2701])) && s.b[2702]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73450_e110692);

        s.b[2703] = (2.0 == 8.0);
        s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });

        let (assign73470_e110721,) = {
    if ((((((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && s.b[2699]) && (!s.b[2700])) && (!s.b[2701])) && (!s.b[2702])) && s.b[2703]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73470_e110721);

        let (assign73480_e110736,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && s.b[2699]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign73480_e110736);

        let mut assign73490_loop_guard: usize = 0;
        while {
            let assign73490_cond_e110752: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && s.b[2699]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73490_cond_e110752 != 0.0
        } {
            assign73490_loop_guard += 1;
            assert!(assign73490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && s.b[2699]) {
                s.store_sqrt(726, 726);
            }
            let (assign73490_body1_e110785,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && s.b[2699]) {
        let assign73490_body1_e110783: f64 = (s.v[719] + 1.0);
        (assign73490_body1_e110783,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign73490_body1_e110785);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) && (!s.b[2699])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2686, (-0.1), 780);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2696]) && (!s.b[2697])) {
            if (s.v[404] <= s.v[2686]) {
            } else {
                s.copy_ad(404, 2686);
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[2665])) {
            s.copy_ad(2648, 404);
        }

        s.b[2704] = (p.p33 == 1.0);
        s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });

        let (assign73610_e110957,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign73610_e110957);

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2641)), s.ad_value(155)), 2.0);
        }

        s.b[2705] = (s.v[411] > 0.0);
        s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && s.b[2705]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2705])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2706] = (s.v[336] < 0.0);
        s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2705])) && s.b[2706]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2705])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2707] = (s.v[336] < 0.0);
        s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && s.b[2707]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2641, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign73840_e111266,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign73840_e111266);

        let mut assign73850_loop_guard: usize = 0;
        while {
            let assign73850_cond_e111276: f64 = (s.v[421] + 1.0);
            let assign73850_cond_e111278: f64 = if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (s.v[97] <= assign73850_cond_e111276)) { 1.0 } else { 0.0 };
            assign73850_cond_e111278 != 0.0
        } {
            assign73850_loop_guard += 1;
            assert!(assign73850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2709] = (s.v[333] < 60.0);
            s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && s.b[2709]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2709])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2710] = (s.v[116] < 0.0);
            s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && s.b[2710]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2711] = (s.v[116] < 1e-6);
            s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2710])) && s.b[2711]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2712] = (s.v[338] > 0.0);
            s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2710])) && s.b[2711]) && s.b[2712]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2710])) && s.b[2711]) && (!s.b[2712])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2710])) && (!s.b[2711])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2713] = (s.v[338] > 0.0);
            s.store_scalar(2713, if s.b[2713] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2710])) && (!s.b[2711])) && s.b[2713]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2710])) && (!s.b[2711])) && (!s.b[2713])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2714] = (s.v[116] < 0.0);
            s.store_scalar(2714, if s.b[2714] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && s.b[2714]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2715] = (s.v[116] < 60.0);
            s.store_scalar(2715, if s.b[2715] { 1.0 } else { 0.0 });
            s.b[2716] = (s.v[116] < 5e-5);
            s.store_scalar(2716, if s.b[2716] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2714])) && s.b[2715]) && s.b[2716]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2714])) && s.b[2715]) && (!s.b[2716])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2714])) && (!s.b[2715])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2717] = (s.v[214] > 0.0);
            s.store_scalar(2717, if s.b[2717] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2714])) && s.b[2717]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2714])) && (!s.b[2717])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2718] = (s.v[79] == 1.0);
            s.store_scalar(2718, if s.b[2718] { 1.0 } else { 0.0 });
            let (assign73850_body72_e112424,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && s.b[2718]) {
        let assign73850_body72_e112422: f64 = (s.v[421] + 1.0);
        (assign73850_body72_e112422,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign73850_body72_e112424);
            if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2718])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2718])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2719] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2719, if s.b[2719] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2718])) && s.b[2719]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2718])) {
                s.store_add(404, 404, 236);
            }
            s.b[2720] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2720, if s.b[2720] { 1.0 } else { 0.0 });
            let (assign73850_body79_e112527,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) && (!s.b[2718])) && s.b[2720]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign73850_body79_e112527);
            let (assign73850_body80_e112538,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
        let assign73850_body80_e112536: f64 = (s.v[97] + 1.0);
        (assign73850_body80_e112536,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign73850_body80_e112538);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2665])) && s.b[2704]) {
            s.store_mul(2639, 982, 223);
            s.store_mul(2640, 2641, 2639);
            s.store_offset_div(100, 2640, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

    }

    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
        var_weffcv_nf: f64,
    ) {
        s.b[2722] = (p.p33 == 4.0);
        s.store_scalar(2722, if s.b[2722] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2722]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2648);
        }

        let (assign74000_e112675,) = {
    if ((s.v[2625] != 0.0) && s.b[2722]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign74000_e112675);

        if ((s.v[2625] != 0.0) && s.b[2722]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2641)), s.ad_value(155)), 2.0);
        }

        s.b[2723] = (s.v[411] > 0.0);
        s.store_scalar(2723, if s.b[2723] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2722]) && s.b[2723]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2723])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2724] = (s.v[336] < 0.0);
        s.store_scalar(2724, if s.b[2724] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2723])) && s.b[2724]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2723])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2625] != 0.0) && s.b[2722]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2725] = (s.v[336] < 0.0);
        s.store_scalar(2725, if s.b[2725] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2722]) && s.b[2725]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2625] != 0.0) && s.b[2722]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2641, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign74230_e112924,) = {
    if ((s.v[2625] != 0.0) && s.b[2722]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign74230_e112924);

        let mut assign74240_loop_guard: usize = 0;
        while {
            let assign74240_cond_e112931: f64 = (s.v[421] + 1.0);
            let assign74240_cond_e112933: f64 = if (((s.v[2625] != 0.0) && s.b[2722]) && (s.v[97] <= assign74240_cond_e112931)) { 1.0 } else { 0.0 };
            assign74240_cond_e112933 != 0.0
        } {
            assign74240_loop_guard += 1;
            assert!(assign74240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && s.b[2722]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2727] = (s.v[333] < 60.0);
            s.store_scalar(2727, if s.b[2727] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2722]) && s.b[2727]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2727])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2722]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2728] = (((s.v[116]) as f64).abs() < 1e-6);
            s.store_scalar(2728, if s.b[2728] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2722]) && s.b[2728]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2649, 334, 336);
                s.store_mul_add_scaled_product_rhs(2650, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2728])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2649, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2650, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2729] = (((s.v[116]) as f64).abs() < 5e-5);
            s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2722]) && s.b[2729]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2730] = (((s.v[116]) as f64).abs() < 60.0);
            s.store_scalar(2730, if s.b[2730] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2729])) && s.b[2730]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2729])) && (!s.b[2730])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2731] = (s.v[214] > 0.0);
            s.store_scalar(2731, if s.b[2731] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2722]) && s.b[2731]) {
                s.store_sqrt_add(216, 2649, 214);
                s.store_div_scaled_inputs2_indices(217, 2650, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2732] = (s.v[2649] > 0.0);
            s.store_scalar(2732, if s.b[2732] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2731])) && s.b[2732]) {
                s.store_sqrt(216, 2649);
                s.store_div_scaled_inputs_indices(217, 2650, 0.5, 216, 1.0);
            }
            if ((((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2731])) && (!s.b[2732])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2722]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[2722]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[2722]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2733] = (s.v[79] > 0.0);
            s.store_scalar(2733, if s.b[2733] { 1.0 } else { 0.0 });
            let (assign74240_body56_e113673,) = {
    if (((s.v[2625] != 0.0) && s.b[2722]) && s.b[2733]) {
        let assign74240_body56_e113671: f64 = (s.v[421] + 1.0);
        (assign74240_body56_e113671,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign74240_body56_e113673);
            if (((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2733])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2733])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2734] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2734, if s.b[2734] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2733])) && s.b[2734]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2733])) {
                s.store_add(404, 404, 236);
            }
            s.b[2735] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2735, if s.b[2735] { 1.0 } else { 0.0 });
            let (assign74240_body63_e113763,) = {
    if ((((s.v[2625] != 0.0) && s.b[2722]) && (!s.b[2733])) && s.b[2735]) {
        let assign74240_body63_e113761: f64 = (s.v[79] + 2.0);
        (assign74240_body63_e113761,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign74240_body63_e113763);
            let (assign74240_body64_e113771,) = {
    if ((s.v[2625] != 0.0) && s.b[2722]) {
        let assign74240_body64_e113769: f64 = (s.v[97] + 1.0);
        (assign74240_body64_e113769,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign74240_body64_e113771);
        }

        if ((s.v[2625] != 0.0) && s.b[2722]) {
            if (s.v[2649] >= 0.0) {
                s.store_scaled_sqrt(223, 2649, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2625] != 0.0) && s.b[2722]) {
            s.store_mul(2639, 982, 223);
            s.store_mul(2640, 2641, 2639);
            s.store_offset_div(100, 2640, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2625] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2737] = (s.v[407] < 0.0);
        s.store_scalar(2737, if s.b[2737] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2737]) {
            s.store_neg(407, 407);
        }

        s.b[2738] = (p.p55 == 0.0);
        s.store_scalar(2738, if s.b[2738] { 1.0 } else { 0.0 });

        s.b[2739] = (p.p50 == 0.0);
        s.store_scalar(2739, if s.b[2739] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2737]) && s.b[2738]) && s.b[2739]) {
            s.store_neg(2642, 404);
        }

        if ((((s.v[2625] != 0.0) && s.b[2737]) && s.b[2738]) && (!s.b[2739])) {
            s.copy_ad(2642, 396);
        }

        if (((s.v[2625] != 0.0) && s.b[2737]) && s.b[2738]) {
            s.store_sqrt_offset_square_offset(782, 2642, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2642), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2642), p.p137), 782, 0.5);
        }

        s.b[2740] = (s.v[336] < 0.0);
        s.store_scalar(2740, if s.b[2740] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2737]) && s.b[2738]) && s.b[2740]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2737]) && s.b[2738]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2625] != 0.0) && s.b[2737]) && s.b[2738]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2625] != 0.0) && s.b[2737]) && s.b[2738]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2741] = (1.0 == 1.0);
        s.store_scalar(2741, if s.b[2741] { 1.0 } else { 0.0 });

        s.b[2742] = (1.0 == 2.0);
        s.store_scalar(2742, if s.b[2742] { 1.0 } else { 0.0 });

        s.b[2743] = (1.0 == 3.0);
        s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });

        s.b[2744] = (1.0 == 4.0);
        s.store_scalar(2744, if s.b[2744] { 1.0 } else { 0.0 });

        s.b[2745] = (p.p55 == 1.0);
        s.store_scalar(2745, if s.b[2745] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2741]) && s.b[2745]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2625] != 0.0) && s.b[2741]) && (!s.b[2745])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && s.b[2741]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[2742] && (!s.b[2741]))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2746] = (p.p55 == 1.0);
        s.store_scalar(2746, if s.b[2746] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.b[2743] && (!(s.b[2741] || s.b[2742])))) && s.b[2746]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2625] != 0.0) && (s.b[2743] && (!(s.b[2741] || s.b[2742])))) && (!s.b[2746])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && (s.b[2743] && (!(s.b[2741] || s.b[2742])))) {
            s.copy_ad(697, 404);
        }

        s.b[2747] = (p.p430 == 0.0);
        s.store_scalar(2747, if s.b[2747] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_cox0: f64,
        var_coxb0: f64,
        var_uc_nover: f64,
        var_uc_novers: f64,
        var_weffcv_nf: f64,
        var_flg_coovlp_slot: &mut f64,
        var_flg_coovlps_slot: &mut f64,
        var_guard1752_slot: &mut f64,
        var_guard1753_slot: &mut f64,
        var_guard1754_slot: &mut f64,
        var_guard1756_slot: &mut f64,
        var_guard1758_slot: &mut f64,
    ) {
        let mut var_flg_coovlp: f64 = *var_flg_coovlp_slot;
        let mut var_flg_coovlps: f64 = *var_flg_coovlps_slot;
        let mut var_guard1752: f64 = *var_guard1752_slot;
        let mut var_guard1753: f64 = *var_guard1753_slot;
        let mut var_guard1754: f64 = *var_guard1754_slot;
        let mut var_guard1756: f64 = *var_guard1756_slot;
        let mut var_guard1758: f64 = *var_guard1758_slot;

        if (((s.v[2625] != 0.0) && (s.b[2743] && (!(s.b[2741] || s.b[2742])))) && s.b[2747]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[2743] && (!(s.b[2741] || s.b[2742])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2625] != 0.0) && (s.b[2744] && (!((s.b[2741] || s.b[2742]) || s.b[2743])))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.store_scalar(2625, 0.0);

        let assign74820_e114338: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1752 = assign74820_e114338;

        let assign74830_e114341: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1753 = assign74830_e114341;

        let assign74840_e114344: f64 = if 2.0 == 3.0 { 1.0 } else { 0.0 };
        var_guard1754 = assign74840_e114344;

        s.b[2751] = (2.0 == 4.0);
        s.store_scalar(2751, if s.b[2751] { 1.0 } else { 0.0 });

        let assign74860_e114358: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        var_guard1756 = assign74860_e114358;

        let (assign74870_e114364,) = {
    if ((var_guard1752 != 0.0) && (var_guard1756 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign74870_e114364);

        let (assign74880_e114370,) = {
    if ((var_guard1752 != 0.0) && (var_guard1756 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlps,)
    }
};
        var_flg_coovlps = assign74880_e114370;

        if ((s.v[2748] != 0.0) && (s.v[2752] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, var_uc_novers);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, var_cox0);
        }

        s.b[2753] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2753, if s.b[2753] { 1.0 } else { 0.0 });

        let (assign74970_e114443,) = {
    if (((var_guard1753 != 0.0) && (var_guard1752 == 0.0)) && s.b[2753]) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign74970_e114443);

        if (((s.v[2749] != 0.0) && (s.v[2748] == 0.0)) && s.b[2753]) {
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        let assign75000_e114475: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        var_guard1758 = assign75000_e114475;

        let (assign75010_e114486,) = {
    if (((var_guard1754 != 0.0) && (!((var_guard1752 != 0.0) || (var_guard1753 != 0.0)))) && (var_guard1758 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign75010_e114486);

        let (assign75020_e114497,) = {
    if (((var_guard1754 != 0.0) && (!((var_guard1752 != 0.0) || (var_guard1753 != 0.0)))) && (var_guard1758 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlp,)
    }
};
        var_flg_coovlp = assign75020_e114497;

        if (((s.v[2750] != 0.0) && (!((s.v[2748] != 0.0) || (s.v[2749] != 0.0)))) && (s.v[2754] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, var_uc_nover);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.store_scalar(413, var_coxb0);
            s.store_neg(407, 407);
        }

        s.b[2755] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.store_scalar(2755, if s.b[2755] { 1.0 } else { 0.0 });

        if ((((s.v[2750] != 0.0) && (!((s.v[2748] != 0.0) || (s.v[2749] != 0.0)))) && (s.v[2754] != 0.0)) && s.b[2755]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2756] = (p.p113 > 0.0);
        s.store_scalar(2756, if s.b[2756] { 1.0 } else { 0.0 });

        s.b[2757] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.store_scalar(2757, if s.b[2757] { 1.0 } else { 0.0 });

        if ((((((s.v[2750] != 0.0) && (!((s.v[2748] != 0.0) || (s.v[2749] != 0.0)))) && (s.v[2754] != 0.0)) && s.b[2755]) && s.b[2756]) && s.b[2757]) {
        }

        if ((((((s.v[2750] != 0.0) && (!((s.v[2748] != 0.0) || (s.v[2749] != 0.0)))) && (s.v[2754] != 0.0)) && s.b[2755]) && s.b[2756]) && (!s.b[2757])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if ((((((s.v[2750] != 0.0) && (!((s.v[2748] != 0.0) || (s.v[2749] != 0.0)))) && (s.v[2754] != 0.0)) && s.b[2755]) && s.b[2756]) && (!s.b[2757])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if (((((s.v[2750] != 0.0) && (!((s.v[2748] != 0.0) || (s.v[2749] != 0.0)))) && (s.v[2754] != 0.0)) && s.b[2755]) && s.b[2756]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2758] = (s.v[336] < 0.0);
        s.store_scalar(2758, if s.b[2758] { 1.0 } else { 0.0 });

        if ((((((s.v[2750] != 0.0) && (!((s.v[2748] != 0.0) || (s.v[2749] != 0.0)))) && (s.v[2754] != 0.0)) && s.b[2755]) && s.b[2756]) && s.b[2758]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((((s.v[2750] != 0.0) && (!((s.v[2748] != 0.0) || (s.v[2749] != 0.0)))) && (s.v[2754] != 0.0)) && s.b[2755]) && s.b[2756]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2759] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2759, if s.b[2759] { 1.0 } else { 0.0 });

        let (assign75320_e114968,) = {
    if ((s.b[2751] && (!(((var_guard1752 != 0.0) || (var_guard1753 != 0.0)) || (var_guard1754 != 0.0)))) && s.b[2759]) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign75320_e114968);

        if ((s.b[2751] && (!(((s.v[2748] != 0.0) || (s.v[2749] != 0.0)) || (s.v[2750] != 0.0)))) && s.b[2759]) {
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2625] != 0.0) {
            s.store_scalar(2767, 0.4);
        }

        let (assign75370_e115010,) = {
    if (s.v[2625] != 0.0) {
        (0.0,)
    } else {
        (s.v[2768],)
    }
};
        s.store_scalar(2768, assign75370_e115010);

        if (s.v[2625] != 0.0) {
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
        }

        let (assign75500_e115063,) = {
    if (s.v[2625] != 0.0) {
        let assign75500_e115061: f64 = (-1.0);
        (assign75500_e115061,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign75500_e115063);

        if (s.v[2625] != 0.0) {
            s.store_scalar(2769, 0.0);
            s.store_scalar(2770, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2765, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2765), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2625] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2625] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2766, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2772] = (s.v[2767] > (s.v[2766] * 0.5));
        s.store_scalar(2772, if s.b[2772] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2772]) {
            s.store_scale(2767, 2766, 0.5);
        }

        s.b[2773] = param_given[338];
        s.store_scalar(2773, if s.b[2773] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2773]) {
            s.store_scalar(2766, p.p338);
        }

        s.b[2774] = param_given[339];
        s.store_scalar(2774, if s.b[2774] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2774]) {
            s.store_scalar(2767, p.p339);
        }

        s.b[2775] = param_given[338];
        s.store_scalar(2775, if s.b[2775] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2774])) && s.b[2775]) {
            s.store_scale(2767, 2766, 0.5);
        }

        s.b[2776] = (s.v[2767] > (s.v[2766] * 0.5));
        s.store_scalar(2776, if s.b[2776] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2776]) {
            s.store_scale(2767, 2766, 0.5);
        }

        s.b[2777] = (p.p38 == 1.0);
        s.store_scalar(2777, if s.b[2777] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2777]) {
            s.store_neg(334, 396);
        }

        s.b[2778] = (s.v[334] > s.v[2767]);
        s.store_scalar(2778, if s.b[2778] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2777]) && s.b[2778]) {
            s.store_sub(335, 334, 2767);
            s.store_sub(336, 2766, 2767);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2767, 333);
        }

        if (((s.v[2625] != 0.0) && s.b[2777]) && (!s.b[2778])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2625] != 0.0) && s.b[2777]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2777])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2625] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign75910_e115404,) = {
    if (s.v[2625] != 0.0) {
        let assign75910_e115398: f64 = (-s.v[397]);
        let assign75910_e115401: f64 = (10.0 * 2.220446049250313e-16);
        let assign75910_e115402: f64 = (assign75910_e115398 + assign75910_e115401);
        (assign75910_e115402,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign75910_e115404);

        if (s.v[2625] != 0.0) {
            s.store_scalar(2761, 0.0);
            s.store_scale(2762, 409, 1.6021918e-19);
        }


        *var_flg_coovlp_slot = var_flg_coovlp;
        *var_flg_coovlps_slot = var_flg_coovlps;
        *var_guard1752_slot = var_guard1752;
        *var_guard1753_slot = var_guard1753;
        *var_guard1754_slot = var_guard1754;
        *var_guard1756_slot = var_guard1756;
        *var_guard1758_slot = var_guard1758;
    }

    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[2625] != 0.0) {
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2779] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(2779, if s.b[2779] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2779]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2779])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign76010_loop_guard: usize = 0;
        while {
            let assign76010_cond_e115478: f64 = if (((s.v[2625] != 0.0) && (!s.b[2779])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign76010_cond_e115478 != 0.0
        } {
            assign76010_loop_guard += 1;
            assert!(assign76010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && (!s.b[2779])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[2779])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2780] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(2780, if s.b[2780] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign76160_e115652,) = {
    if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign76160_e115652);

        let (assign76170_e115660,) = {
    if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76170_e115660);

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2781] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2781, if s.b[2781] { 1.0 } else { 0.0 });

        s.b[2782] = (1.0 == 1.0);
        s.store_scalar(2782, if s.b[2782] { 1.0 } else { 0.0 });

        let (assign76260_e115744,) = {
    if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && s.b[2781]) && s.b[2782]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76260_e115744);

        s.b[2783] = (1.0 == 2.0);
        s.store_scalar(2783, if s.b[2783] { 1.0 } else { 0.0 });

        let (assign76280_e115762,) = {
    if ((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && s.b[2781]) && (!s.b[2782])) && s.b[2783]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76280_e115762);

        s.b[2784] = (1.0 == 4.0);
        s.store_scalar(2784, if s.b[2784] { 1.0 } else { 0.0 });

        let (assign76300_e115783,) = {
    if (((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && s.b[2781]) && (!s.b[2782])) && (!s.b[2783])) && s.b[2784]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76300_e115783);

        s.b[2785] = (1.0 == 8.0);
        s.store_scalar(2785, if s.b[2785] { 1.0 } else { 0.0 });

        let (assign76320_e115807,) = {
    if ((((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && s.b[2781]) && (!s.b[2782])) && (!s.b[2783])) && (!s.b[2784])) && s.b[2785]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76320_e115807);

        let (assign76330_e115817,) = {
    if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && s.b[2781]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign76330_e115817);

        let mut assign76340_loop_guard: usize = 0;
        while {
            let assign76340_cond_e115828: f64 = if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && s.b[2781]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign76340_cond_e115828 != 0.0
        } {
            assign76340_loop_guard += 1;
            assert!(assign76340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && s.b[2781]) {
                s.store_sqrt(726, 726);
            }
            let (assign76340_body1_e115851,) = {
    if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && s.b[2781]) {
        let assign76340_body1_e115849: f64 = (s.v[719] + 1.0);
        (assign76340_body1_e115849,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign76340_body1_e115851);
        }

        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) && (!s.b[2781])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2780]) {
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2780])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign76440_e115968,) = {
    if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
        let assign76440_e115962: f64 = (-s.v[397]);
        let assign76440_e115965: f64 = (10.0 * 2.220446049250313e-16);
        let assign76440_e115966: f64 = (assign76440_e115962 + assign76440_e115965);
        (assign76440_e115966,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign76440_e115968);

        s.b[2786] = (s.v[402] < s.v[403]);
        s.store_scalar(2786, if s.b[2786] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2786]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2787] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2787, if s.b[2787] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2786]) && s.b[2787]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2786]) && (!s.b[2787])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2625] != 0.0) && s.b[2786]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_div_lhs_indices(404, 335, 337, 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2769, 404);
        }

        s.b[2788] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(2788, if s.b[2788] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2788]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && (!s.b[2788])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.v[2625] != 0.0) && (!s.b[2786])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2789] = (s.v[116] >= 3.0);
        s.store_scalar(2789, if s.b[2789] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2789]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && (!s.b[2789])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2790] = (p.p33 > 0.0);
        s.store_scalar(2790, if s.b[2790] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2791] = (p.p33 == 2.0);
        s.store_scalar(2791, if s.b[2791] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2791]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

    }

    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2791]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2791]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && (!s.b[2791])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) {
            s.copy_ad(445, 116);
        }

        s.b[2792] = (p.p33 == 2.0);
        s.store_scalar(2792, if s.b[2792] { 1.0 } else { 0.0 });

        s.b[2793] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(2793, if s.b[2793] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign77270_e117114,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign77270_e117114);

        let (assign77280_e117127,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77280_e117127);

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2794] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2794, if s.b[2794] { 1.0 } else { 0.0 });

        s.b[2795] = (2.0 == 1.0);
        s.store_scalar(2795, if s.b[2795] { 1.0 } else { 0.0 });

        let (assign77390_e117276,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && s.b[2795]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77390_e117276);

        s.b[2796] = (2.0 == 2.0);
        s.store_scalar(2796, if s.b[2796] { 1.0 } else { 0.0 });

        let (assign77410_e117299,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && (!s.b[2795])) && s.b[2796]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77410_e117299);

        s.b[2797] = (2.0 == 4.0);
        s.store_scalar(2797, if s.b[2797] { 1.0 } else { 0.0 });

        let (assign77430_e117325,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && (!s.b[2795])) && (!s.b[2796])) && s.b[2797]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77430_e117325);

        s.b[2798] = (2.0 == 8.0);
        s.store_scalar(2798, if s.b[2798] { 1.0 } else { 0.0 });

        let (assign77450_e117354,) = {
    if ((((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && (!s.b[2795])) && (!s.b[2796])) && (!s.b[2797])) && s.b[2798]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77450_e117354);

        let (assign77460_e117369,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign77460_e117369);

        let mut assign77470_loop_guard: usize = 0;
        while {
            let assign77470_cond_e117385: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign77470_cond_e117385 != 0.0
        } {
            assign77470_loop_guard += 1;
            assert!(assign77470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) {
                s.store_sqrt(726, 726);
            }
            let (assign77470_body1_e117418,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) {
        let assign77470_body1_e117416: f64 = (s.v[719] + 1.0);
        (assign77470_body1_e117416,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign77470_body1_e117418);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && (!s.b[2794])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && (!s.b[2793])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && (!s.b[2792])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2799] = (p.p33 == 1.0);
        s.store_scalar(2799, if s.b[2799] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2800] = (s.v[411] > 0.0);
        s.store_scalar(2800, if s.b[2800] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2800]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && (!s.b[2800])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2801] = (s.v[336] < 0.0);
        s.store_scalar(2801, if s.b[2801] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && (!s.b[2800])) && s.b[2801]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && (!s.b[2800])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2802] = (s.v[336] < 0.0);
        s.store_scalar(2802, if s.b[2802] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2802]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2762, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2803] = (s.v[333] < 60.0);
        s.store_scalar(2803, if s.b[2803] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2803]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && (!s.b[2803])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2804] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.store_scalar(2804, if s.b[2804] { 1.0 } else { 0.0 });

        let (assign77900_e118007,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2804]) {
        let assign77900_e118005: f64 = (s.v[2768] + 1.0);
        (assign77900_e118005,)
    } else {
        (s.v[2768],)
    }
};
        s.store_scalar(2768, assign77900_e118007);

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2804]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2786])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2805] = (((s.v[116]) as f64).abs() > 1e-6);
        s.store_scalar(2805, if s.b[2805] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2805]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && (!s.b[2805])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2625] != 0.0) && (!s.b[2786])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2806, 354, 2762);
        }

        s.b[2808] = (p.p33 == 2.0);
        s.store_scalar(2808, if s.b[2808] { 1.0 } else { 0.0 });

        s.b[2809] = ((s.v[2806] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.store_scalar(2809, if s.b[2809] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {
            s.store_add_scaled_inputs3_indices(781, 2806, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign78080_e118214,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign78080_e118214);

        let (assign78090_e118225,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78090_e118225);

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2810] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2810, if s.b[2810] { 1.0 } else { 0.0 });

        s.b[2811] = (2.0 == 1.0);
        s.store_scalar(2811, if s.b[2811] { 1.0 } else { 0.0 });

        let (assign78200_e118356,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && s.b[2811]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78200_e118356);

        s.b[2812] = (2.0 == 2.0);
        s.store_scalar(2812, if s.b[2812] { 1.0 } else { 0.0 });

        let (assign78220_e118377,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && (!s.b[2811])) && s.b[2812]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78220_e118377);

        s.b[2813] = (2.0 == 4.0);
        s.store_scalar(2813, if s.b[2813] { 1.0 } else { 0.0 });

        let (assign78240_e118401,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && (!s.b[2811])) && (!s.b[2812])) && s.b[2813]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78240_e118401);

        s.b[2814] = (2.0 == 8.0);
        s.store_scalar(2814, if s.b[2814] { 1.0 } else { 0.0 });

        let (assign78260_e118428,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && (!s.b[2811])) && (!s.b[2812])) && (!s.b[2813])) && s.b[2814]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78260_e118428);

        let (assign78270_e118441,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign78270_e118441);

        let mut assign78280_loop_guard: usize = 0;
        while {
            let assign78280_cond_e118455: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78280_cond_e118455 != 0.0
        } {
            assign78280_loop_guard += 1;
            assert!(assign78280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) {
                s.store_sqrt(726, 726);
            }
            let (assign78280_body1_e118484,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) {
        let assign78280_body1_e118482: f64 = (s.v[719] + 1.0);
        (assign78280_body1_e118482,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign78280_body1_e118484);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && (!s.b[2810])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && (!s.b[2809])) {
            s.copy_ad(335, 2806);
            s.store_scalar(334, 1.0);
        }

        s.b[2815] = (s.v[334] < 1.0);
        s.store_scalar(2815, if s.b[2815] { 1.0 } else { 0.0 });

        let (assign78380_e118626,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2815]) {
        let assign78380_e118624: f64 = (s.v[2768] + 2.0);
        (assign78380_e118624,)
    } else {
        (s.v[2768],)
    }
};
        s.store_scalar(2768, assign78380_e118626);

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && (!s.b[2808])) {
            if (s.v[2806] <= s.v[386]) {
                s.copy_ad(335, 2806);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2816] = (s.v[2806] >= s.v[386]);
        s.store_scalar(2816, if s.b[2816] { 1.0 } else { 0.0 });

        let (assign78410_e118658,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2786])) && (!s.b[2808])) && s.b[2816]) {
        let assign78410_e118656: f64 = (s.v[2768] + 2.0);
        (assign78410_e118656,)
    } else {
        (s.v[2768],)
    }
};
        s.store_scalar(2768, assign78410_e118658);

        s.b[2817] = (s.v[2768] >= 2.0);
        s.store_scalar(2817, if s.b[2817] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) {
            s.copy_ad(2807, 404);
            s.store_mul(354, 335, 2762);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2818] = (p.p33 == 2.0);
        s.store_scalar(2818, if s.b[2818] { 1.0 } else { 0.0 });

        s.b[2819] = ((s.v[404] > (s.v[2807] - 0.1)) && (0.1 >= 0.0));
        s.store_scalar(2819, if s.b[2819] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
            s.store_offset_sub(781, 404, 2807, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign78530_e118792,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign78530_e118792);

        let (assign78540_e118805,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78540_e118805);

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2820] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2820, if s.b[2820] { 1.0 } else { 0.0 });

        s.b[2821] = (2.0 == 1.0);
        s.store_scalar(2821, if s.b[2821] { 1.0 } else { 0.0 });

        let (assign78650_e118954,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && s.b[2821]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78650_e118954);

        s.b[2822] = (2.0 == 2.0);
        s.store_scalar(2822, if s.b[2822] { 1.0 } else { 0.0 });

        let (assign78670_e118977,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && (!s.b[2821])) && s.b[2822]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78670_e118977);

        s.b[2823] = (2.0 == 4.0);
        s.store_scalar(2823, if s.b[2823] { 1.0 } else { 0.0 });

        let (assign78690_e119003,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && (!s.b[2821])) && (!s.b[2822])) && s.b[2823]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78690_e119003);

        s.b[2824] = (2.0 == 8.0);
        s.store_scalar(2824, if s.b[2824] { 1.0 } else { 0.0 });

        let (assign78710_e119032,) = {
    if ((((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && (!s.b[2821])) && (!s.b[2822])) && (!s.b[2823])) && s.b[2824]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78710_e119032);

        let (assign78720_e119047,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign78720_e119047);

        let mut assign78730_loop_guard: usize = 0;
        while {
            let assign78730_cond_e119063: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78730_cond_e119063 != 0.0
        } {
            assign78730_loop_guard += 1;
            assert!(assign78730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) {
                s.store_sqrt(726, 726);
            }
            let (assign78730_body1_e119096,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) {
        let assign78730_body1_e119094: f64 = (s.v[719] + 1.0);
        (assign78730_body1_e119094,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign78730_body1_e119096);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && (!s.b[2820])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2807, (-0.1), 780);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && (!s.b[2818])) {
            if (s.v[404] <= s.v[2807]) {
            } else {
                s.copy_ad(404, 2807);
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[2786])) {
            s.copy_ad(2769, 404);
        }

        s.b[2825] = (p.p33 == 1.0);
        s.store_scalar(2825, if s.b[2825] { 1.0 } else { 0.0 });

        let (assign78850_e119268,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign78850_e119268);

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2762)), s.ad_value(155)), 2.0);
        }

        s.b[2826] = (s.v[411] > 0.0);
        s.store_scalar(2826, if s.b[2826] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2826]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2826])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2827] = (s.v[336] < 0.0);
        s.store_scalar(2827, if s.b[2827] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2826])) && s.b[2827]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2826])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2828] = (s.v[336] < 0.0);
        s.store_scalar(2828, if s.b[2828] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2828]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2762, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign79080_e119577,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign79080_e119577);

    }
}
