#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1562] && s.b[1580]) {
            s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1562] && (!s.b[1580])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_offset_ad_lhs(91, A::square(s.ad_value(14)), 1.0, 15);
        }

        if s.b[1562] {
            s.store_mul_pow_mixed_aii(15, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(62), 1.0), 156, 407);
            s.store_offset(16, 15, 1.0);
        }

        s.b[1581] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });

        if (s.b[1562] && s.b[1581]) {
            s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (s.b[1562] && (!s.b[1581])) {
            s.store_scaled_add_offset_sqrt_square_offset(159, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);
        }

        if s.b[1562] {
            s.store_div_scaled_product_by_product(138, s.ad_value(499), s.ad_value(108), 2.0, s.ad_value(159), s.ad_value(411), s.v[34]);
            s.store_sub(87, 200, 144);
            s.store_mul_ad_affine_product_rhs(13, 138, s.ad_value(87), A::mul(s.ad_value(138), s.ad_value(87)), 2.0, 0.0);
            s.store_sqrt_offset_input(161, 13, 1.0);
            s.store_scaled_offset(162, 161, 1.0, 0.5);
            s.store_div_scaled_inputs_mixed_ia(134, 411, 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0);
            s.store_scale(135, 134, s.v[34]);
            s.store_add(170, 141, 135);
            s.store_sub(167, 75, 139);
        }

        s.b[1582] = (s.v[542] != 0.0);
        s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });

        if s.b[1582] {
            s.store_offset_mul_ad(176, s.ad_value(542), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(542), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);
        }

        if (!s.b[1582]) {
            s.store_scalar(176, 1.0);
        }

        s.store_square(207, 176);

        s.store_div_from_scalar(208, 1.0, 176);

        s.store_div_from_scalar(209, 1.0, 207);

        s.store_offset(210, 176, (-1.0));

        s.store_sub(213, 60, 91);

        s.store_sub(216, 200, 144);

        s.store_square_ad(217, A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_add_scaled_inputs(211, 213, 1.0, 200, 2.0);

        s.store_add_scaled_inputs(212, 213, 1.0, 144, 2.0);

        s.b[1583] = ((0.0 == 0.0) && (s.v[211] < ((-2500.0) * 0.5)));
        s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });

        if s.b[1583] {
            s.store_div_from_scalar_scaled_input(13, ((-0.5) * 0.5), 211, 16.0);
        }

        if (!s.b[1583]) {
            s.store_scaled_add_sqrt_square_offset_rhs(13, 211, 211, ((0.25 * 0.5) * 0.5), 0.5);
        }

        s.b[1584] = ((0.0 == 0.0) && (s.v[212] < ((-2500.0) * 0.5)));
        s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });

        if s.b[1584] {
            s.store_div_from_scalar_scaled_input(14, ((-0.5) * 0.5), 212, 16.0);
        }

        if (!s.b[1584]) {
            s.store_scaled_add_sqrt_square_offset_rhs(14, 212, 212, ((0.25 * 0.5) * 0.5), 0.5);
        }

        s.store_sqrt_offset_ad(214, A::mul(s.ad_value(13), s.ad_value(218)), 0.25);

        s.store_sqrt_offset_ad(215, A::mul(s.ad_value(14), s.ad_value(218)), 0.25);

        s.store_div_ad_rhs(13, 211, A::scale_offset(s.ad_value(214), 2.0, 1.0));

        s.store_div_ad_rhs(14, 212, A::scale_offset(s.ad_value(215), 2.0, 1.0));

        s.store_add(15, 214, 215);

        s.store_scaled_div_ad_rhs(16, 217, A::mul(A::square(s.ad_value(15)), s.ad_value(15)), 0.3333333333333333);

        s.store_div_scaled_product3_mixed_iiia(17, 783, 162, 208, 1.0, A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)), 1.0);

        s.store_mul_scale_ad_lhs(18, A::add_scaled_square_product(s.ad_value(15), 1.0, s.ad_value(214), s.ad_value(215), 1.0), 0.8, 17);

        s.store_add_scaled_inputs(19, 18, 1.0, 218, 2.0);

        s.store_scaled_mul(20, 217, 17, 0.3333333333333333);

        s.store_div_scaled_product_mixed_iaa(202, 212, A::scale_offset(s.ad_value(215), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(215), 2.0, 1.0), 1.0);

        s.store_add_ad_lhs(201, A::add_scaled_offset_product_lhs(s.ad_value(213), 1.0, s.ad_value(90), (-1.0), s.ad_value(144), (-2.0)), 202);

        s.store_add_scaled_products_left_right_ad(189, 208, A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, A::add_scaled_products(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(90), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(20), 1.0), (-1.0)), 1.0), 1.0, 210, 201, 1.0);

        s.store_add(21, 200, 144);

        s.store_mul3_lhs(22, 217, 17, 17);

        s.store_add_ad(194, A::mul3(s.ad_value(90), s.ad_value(208), A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(217), s.ad_value(17), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(90), s.ad_value(210), s.ad_value(144), 2.0));

        s.store_mul_ad_product_rhs_mixed_ia(191, 90, 209, A::add_scaled_product(s.ad_value(21), 0.5, s.ad_value(216), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(17))), 1.0, s.ad_value(22), 0.2), (-1.0 / (6.0))));

        s.store_mul_ad_product_lhs_mixed_ia(192, 90, A::sub(s.ad_value(176), s.ad_value(208)), 144);

        s.store_add(193, 191, 192);

        s.store_sub(190, 194, 193);

        s.b[1585] = ((0.0 == 0.0) && ((s.v[108] * s.v[189]) < ((-2500.0) * p.p694)));
        s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });

        if s.b[1585] {
            s.store_div_from_scalar_scaled_mul(83, ((-p.p694) * p.p694), 108, 189, 16.0);
        }

        if (!s.b[1585]) {
            s.store_add_scaled_product_value_ad(83, A::sqrt_square_offset(A::mul(s.ad_value(108), s.ad_value(189)), ((0.25 * p.p694) * p.p694)), 0.5, 108, 189, 0.5);
        }

        s.store_mul_add_rhs(82, 108, 190, 193);

        s.store_add_scaled_inputs(12, 82, 1.0 / (p.p207), 83, (p.p208 * 1.0 / (p.p207)));

        s.store_offset_powf_ad(13, s.ad_value(12), (0.7 * p.p206), 1.0);

        s.store_div_from_scalar(227, (p.p205 * 1.9e-9), 13);

        s.store_div_from_scalar_ad(228, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(227), 1.0 / (s.v[47])));

        s.store_mul_ad_affine_product_lhs(195, A::div_from_scalar((8.85418e-12 * p.p111), s.ad_value(229)), s.ad_value(108), (((-p.p2) * s.v[33]) * s.v[34]), 0.0, 189);

        s.store_scaled_mul(199, 228, 108, ((p.p2 * s.v[33]) * s.v[34]));

        s.store_mul_neg_lhs(196, 199, 190);

        s.store_mul_neg_lhs(197, 199, 193);

        s.store_add_scaled_inputs3_indices(198, 195, (-1.0), 196, (-1.0), 197, (-1.0));

        s.b[1586] = (!param_given[666]);
        s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });

        if s.b[1586] {
            s.store_scalar(544, ((((2.0 * p.p111) * 8.85418e-12) / 3.141592653589793) * ((((p.p670 * (1.0 + (4e-7 / p.p77)))).max(1e-38)) as f64).ln()));
        }

        s.store_offset(225, 544, p.p671);

        s.store_offset(226, 544, p.p672);

        s.b[1587] = (p.p41 == 0.0);
        s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });

        if s.b[1587] {
            s.store_scaled_mul(223, 225, 231, ((-s.v[33]) * p.p2));
            s.store_scaled_mul(224, 226, 232, ((-s.v[33]) * p.p2));
        }

        if (!s.b[1587]) {
            s.store_sqrt_offset_ad(12, A::square(A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset_indices(219, 231, 0.5, 63, ((-1.0) * 0.5), 12, (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(18, 219, A::powf(A::offset(A::powf(A::scale(s.ad_value(219), (-1.0 / (p.p692))), p.p693), 1.0), (1.0 / p.p693)));
            s.store_sqrt_sub_from_scalar_ad(13, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(547), 1.0));
            s.store_add_scaled_products_right_right_ad(223, 225, 231, ((-s.v[33]) * p.p2), 545, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(63), (-1.0), s.ad_value(219), -1.0), 1.0, s.ad_value(547), s.ad_value(13), (-1.0), (-0.5)), ((-s.v[33]) * p.p2));
            s.store_sqrt_offset_ad(12, A::square(A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset_indices(220, 232, 0.5, 63, ((-1.0) * 0.5), 12, (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(18, 220, A::powf(A::offset(A::powf(A::scale(s.ad_value(220), (-1.0 / (p.p690))), p.p691), 1.0), (1.0 / p.p691)));
            s.store_sqrt_sub_from_scalar_ad(14, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(548), 1.0));
            s.store_add_scaled_products_right_right_ad(224, 226, 232, ((-s.v[33]) * p.p2), 546, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(232), 1.0, s.ad_value(63), (-1.0), s.ad_value(220), -1.0), 1.0, s.ad_value(548), s.ad_value(14), (-1.0), (-0.5)), ((-s.v[33]) * p.p2));
        }

        s.store_mul_scaled_voltage(221, 187, (((-p.p2) * s.v[34]) * p.p673), ctx, nodes, Some(10), Some(11));

        s.b[1588] = (p.p37 == 1.0);
        s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });

        if s.b[1588] {
            s.store_ln_ad(684, A::max_with_scalar(A::div(s.ad_value(686), s.ad_value(28)), 1e-38));
            s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(684)), 0.4), s.ad_value(489)), 0.4);
            s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(686), 1.60219e-19));
        }

        if s.b[1588] {
            s.store_mul_ad_rhs(674, 612, {
                if (!((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0, 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1588] {
            s.store_mul_offset_ad_rhs(673, 616, A::mul_offset_rhs(s.ad_value(623), s.ad_value(395), (-1.0)), 1.0);
        }

        s.b[1589] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));
        s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1589]) {
            s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));
        }

        if (s.b[1588] && (!s.b[1589])) {
            s.store_add_scaled_inputs3_offset_mixed_iia(110, 127, 0.5, 61, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05)), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));
        }

        if s.b[1588] {
            s.store_sqrt(111, 110);
            s.store_mul(112, 114, 111);
            s.store_div_from_scalar(97, s.v[26], 112);
            s.store_add_scaled_inputs_products_indices(113, 613, 1.0, 674, 1.0, 614, 76, 1.0, 615, 61, (-1.0));
            s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);
        }

        s.b[1590] = ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05)));
        s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1590]) {
            s.store_div_from_scalar_scaled_input(104, ((-0.05) * 0.05), 13, 16.0);
        }

        if (s.b[1588] && (!s.b[1590])) {
            s.store_scaled_add_offset_sqrt_square_offset(104, 13, 1.0, (-1.0), ((0.25 * 0.05) * 0.05), 0.5);
        }

        if s.b[1588] {
            s.store_mul(106, 104, 108);
            s.store_div_from_scalar(107, 1.0, 106);
            s.store_mul(65, 64, 107);
            s.store_mul(73, 70, 107);
            s.store_mul(58, 482, 107);
            s.store_mul_neg_ad_lhs(677, A::add_scaled_product(s.ad_value(673), 1.0, s.ad_value(617), s.ad_value(61), 1.0), 76);
            s.store_mul_offset_rhs_ad(124, A::add_scaled_inputs_product(s.ad_value(618), 1.0, s.ad_value(619), 1.0 / (s.v[30]), s.ad_value(620), s.ad_value(61), 1.0), A::pow(s.ad_value(395), s.ad_value(621)), (-1.0));
            s.store_mul_scale_offset_rhs(679, 129, 61, p.p1016, 1.0);
        }

        s.b[1591] = (s.v[679] > 0.0);
        s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1591]) {
            s.store_div_from_scalar(12, (p.p1015 * s.v[30]), 679);
        }

        s.b[1592] = (s.v[12] < 40.0);
        s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });

        if ((s.b[1588] && s.b[1591]) && s.b[1592]) {
            s.store_div_from_scalar_offset_ad(676, (0.5 * p.p1014), A::cosh(s.ad_value(12)), (-1.0));
        }

        if ((s.b[1588] && s.b[1591]) && (!s.b[1592])) {
            s.store_scaled_limited_exp_scaled_input(676, 12, -1.0, p.p1014);
        }

        if (s.b[1588] && (!s.b[1591])) {
            s.store_scalar(676, 0.0);
        }

        if s.b[1588] {
            s.store_mul_sub_rhs(678, 676, 675, 127);
            s.store_add_ad_lhs(79, A::add_scaled_product(A::add_scaled_inputs4_offset(s.ad_value(677), 1.0, s.ad_value(124), (-1.0), s.ad_value(678), 1.0, s.ad_value(688), 1.0, p.p961), 1.0, A::add(s.ad_value(624), s.ad_value(666)), s.ad_value(61), (-1.0)), 665);
            s.store_add_scaled_inputs_product_indices(59, 65, 1.0, 58, (-1.0), 79, 107, (-1.0));
            s.store_scalar(680, (p.p958 * (1.0 + (p.p959 * ((s.v[30]) as f64).powf((-p.p960))))));
            s.store_scaled_sqrt_mul_scaled_lhs(687, 686, ((2.0 * 1.60219e-19) * s.v[26]), 107, 1.0 / (s.v[46]));
            s.store_mul_offset_rhs(687, 687, 680, 1.0);
            s.store_div(685, 684, 104);
            s.store_scalar(13, 1.0);
            s.store_div(204, 59, 13);
            s.store_div(205, 687, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1593] = (s.v[204] < 0.0);
        s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1593]) {
            s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1588] && (!s.b[1593])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_offset_ad_lhs(91, A::square(s.ad_value(14)), 1.0, 15);
        }

        if s.b[1588] {
            s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(687), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(687), 1.0);
            s.store_add_scaled_inputs3_indices(13, 91, 1.0, 685, (-2.0), 73, -1.0);
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1594] = (s.v[20] <= (-68.0));
        s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1594]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1595] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });

        if ((s.b[1588] && s.b[1594]) && s.b[1595]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1596] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });

        if (((s.b[1588] && s.b[1594]) && (!s.b[1595])) && s.b[1596]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1588] && s.b[1594]) && (!s.b[1595])) && (!s.b[1596])) {
            s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1588] && s.b[1594]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(693, 15, s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (s.b[1588] && (!s.b[1594])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_div_rhs_indices(15, 15, 16, 17);
            s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(693, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        if s.b[1588] {
            s.store_add_scaled_product_indices(681, 106, 2.0, 106, 693, 2.0);
            s.copy_ad(682, 681);
            s.store_add(682, 682, 70);
        }

        s.b[1597] = ((0.0 == 0.0) && ((s.v[682] - s.v[70]) < ((-2500.0) * 0.001)));
        s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1597]) {
            s.store_div_from_scalar_ad(683, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(682), 16.0, s.ad_value(70), 16.0));
        }

        if (s.b[1588] && (!s.b[1597])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(683, 682, 0.5, 70, ((-1.0) * 0.5), 682, 70, ((0.25 * 0.001) * 0.001), 0.5);
        }

        if s.b[1588] {
            s.store_pow_ad(19, A::div(s.ad_value(74), s.ad_value(683)), A::div_from_scalar(1.0, s.ad_value(412)));
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
            s.store_mul(139, 74, 20);
            s.store_mul_add_lhs(142, 139, 70, 107);
            s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_scaled_offset_numerator(12, A::div_scaled_inputs(s.ad_value(687), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(687), 1.0);
            s.store_add_scaled_inputs3_indices(13, 91, 1.0, 685, (-2.0), 142, -1.0);
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1598] = (s.v[20] <= (-68.0));
        s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1598]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1599] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });

        if ((s.b[1588] && s.b[1598]) && s.b[1599]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1600] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });

        if (((s.b[1588] && s.b[1598]) && (!s.b[1599])) && s.b[1600]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1588] && s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1588] && s.b[1598]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(692, 15, s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if (s.b[1588] && (!s.b[1598])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_div_rhs_indices(15, 15, 16, 17);
            s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(692, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));
        }

        s.b[1601] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));
        s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1601]) {
            s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);
        }

        if (s.b[1588] && (!s.b[1601])) {
            s.store_scaled_add_offset_sqrt_square_offset(93, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
        }

        if s.b[1588] {
            s.store_sqrt(96, 93);
            s.store_add_scaled_inputs3_offset_indices(92, 91, 1.0, 693, (-1.0), 692, -1.0, (-1.0));
        }

        s.b[1602] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1602]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (s.b[1588] && (!s.b[1602])) {
            s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
        }

        if s.b[1588] {
            s.store_sqrt(14, 12);
            s.store_offset_div_ad(691, s.ad_value(687), A::add(s.ad_value(96), s.ad_value(14)), 1.0);
            s.store_mul_product3_mixed_iaia(672, 175, A::mul3_scaled_output(s.ad_value(691), s.ad_value(157), s.ad_value(106), ((2.0 * p.p2) * ((p.p957 * 1.0 / (s.v[30])) * s.v[46]))), 106, A::mul(A::sub(s.ad_value(693), s.ad_value(692)), A::add(A::offset(s.ad_value(693), 1.0), s.ad_value(692))), 1.0);
            s.store_add(188, 672, 188);
            s.store_scalar(696, (p.p785 * p.p1062));
            s.store_scalar(697, (p.p799 * p.p1062));
            s.store_scalar(698, (p.p800 * p.p1062));
            s.store_sub_from_scalar_scaled_input(694, s.v[30], 359, 2.0);
            s.store_square(695, 694);
            s.store_mul_add_scaled_inputs_rhs(367, 108, A::offset(s.ad_value(97), s.v[46]), 1.0 / (1.60219e-19), s.ad_value(613), 1.0 / (1.60219e-19));
            s.store_mul3_affine_lhs(366, 691, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 692);
            s.store_mul_ad_affine_product_lhs(736, s.ad_value(108), A::abs(s.ad_value(672)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 157);
            s.store_mul3_affine_lhs(737, 108, 672, 1.60219e-19, 0.0, 672);
            s.store_add_ad(738, A::add_scaled_product(s.ad_value(696), 1.0, s.ad_value(697), s.ad_value(366), 1.0), A::mul3(s.ad_value(698), s.ad_value(366), s.ad_value(366)));
            s.store_square_ad(739, A::add(s.ad_value(366), s.ad_value(367)));
            s.store_scaled_mul(740, 696, 108, 1.60219e-19);
            s.store_mul3_affine_lhs(365, 691, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 693);
            s.store_mul_ln_ad_rhs(13, 696, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(365), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38));
            s.store_mul_sub_rhs(14, 697, 365, 366);
            s.store_mul_sub_scaled_inputs_rhs(15, 698, A::square(s.ad_value(365)), 0.5, A::square(s.ad_value(366)), 0.5);
            s.store_scale(16, 695, (10000000000.0 * (p.p957 * p.p2)));
            s.store_add_scaled_product(368, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(16), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(12)), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, s.ad_value(15), 1.0), 1.0);
            s.store_mul3_affine_lhs(17, 694, 367, ((p.p957 * p.p2) * 10000000000.0), 0.0, 367);
            s.store_mul_ad_product_lhs_mixed_ai(369, A::div(s.ad_value(740), s.ad_value(17)), 672, 672);
            s.store_add(18, 369, 368);
        }

        s.b[1603] = (s.v[18] > 0.0);
        s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });

        if (s.b[1588] && s.b[1603]) {
            s.store_div_scaled_product_indices(19, 368, 369, 1.0, 18, 1.0);
            s.store_offset_scaled_ad(20, A::powf(A::sub(s.ad_value(693), s.ad_value(692)), p.p1064), p.p1063, 1.0);
        }

        s.b[1604] = (s.v[57] > 0.0);
        s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });

        if s.b[1604] {
            s.store_scaled_mul(785, 187, 196, p.p29);
            s.store_scaled_mul(786, 187, 197, p.p29);
        }

        if (!s.b[1604]) {
            s.store_scaled_mul(785, 187, 197, p.p29);
            s.store_scaled_mul(786, 187, 196, p.p29);
        }

        s.b[1605] = ((p.p1094 == 1.0) && (p.p1095 == 1.0));
        s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });

        if s.b[1605] {
            s.store_add(221, 221, 774);
            s.store_add(224, 224, 775);
        }

        s.b[1606] = (p.p1096 == 1.0);
        s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });

        if (s.b[1605] && s.b[1606]) {
            s.store_add(221, 221, 776);
            s.store_add(223, 223, 777);
        }

        s.store_scaled_mul(787, 187, 198, p.p29);

        s.b[1612] = ((p.p42 != 2.0) && (s.v[240] > 0.0));
        s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });

        if s.b[1612] {
            s.store_div_from_scalar(372, 1.0, 242);
        }

        s.b[1613] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0));
        s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });

        if (s.b[1612] && s.b[1613]) {
            s.store_div_from_scalar(374, 1.0, 759);
        }

        s.b[1614] = ((p.p42 != 2.0) && (s.v[239] > 0.0));
        s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });

        if s.b[1614] {
            s.store_div_from_scalar(371, 1.0, 241);
        }

        s.b[1615] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0));
        s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });

        if (s.b[1614] && s.b[1615]) {
            s.store_div_from_scalar(373, 1.0, 761);
        }

        s.b[1621] = ((p.p49 != 0.0) && (p.p909 > 0.0));
        s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });

        if s.b[1621] {
            s.store_mul_voltage_ad(749, A::mul3(s.ad_value(187), s.ad_value(57), s.ad_value(188)), ctx, nodes, Some(5), Some(7));
        }

        s.b[1622] = ((p.p42 != 2.0) && (s.v[240] > 0.0));
        s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });

        s.b[1623] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0));
        s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });

        if ((s.b[1621] && s.b[1622]) && s.b[1623]) {
            s.store_add_scaled_value_products(749, s.ad_value(749), 1.0, A::square(A::voltage(ctx, nodes, Some(0), Some(6))), s.ad_value(372), 1.0, A::square(A::voltage(ctx, nodes, Some(6), Some(5))), s.ad_value(374), 1.0);
        }

        if ((s.b[1621] && s.b[1622]) && (!s.b[1623])) {
            s.store_add_scaled_product_left_ad(749, 749, 1.0, A::square(A::voltage(ctx, nodes, Some(0), Some(6))), 372, 1.0);
        }

        s.b[1624] = ((p.p42 != 2.0) && (s.v[239] > 0.0));
        s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });

        s.b[1625] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0));
        s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });

        if ((s.b[1621] && s.b[1624]) && s.b[1625]) {
            s.store_add_scaled_value_products(749, s.ad_value(749), 1.0, A::square(A::voltage(ctx, nodes, Some(2), Some(8))), s.ad_value(371), 1.0, A::square(A::voltage(ctx, nodes, Some(8), Some(7))), s.ad_value(373), 1.0);
        }

        if ((s.b[1621] && s.b[1624]) && (!s.b[1625])) {
            s.store_add_scaled_product_left_ad(749, 749, 1.0, A::square(A::voltage(ctx, nodes, Some(2), Some(8))), 371, 1.0);
        }

        s.b[1627] = (p.p8 != 0.0);
        s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });

        s.b[1628] = (p.p1097 == 0.0);
        s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });

        s.b[1630] = ((p.p8 != 0.0) && (p.p1097 == 1.0));
        s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_cox: f64,
        var_devsign: f64,
        var_guard680: f64,
        var_guard697: f64,
        var_guard698: f64,
        var_issl: f64,
        var_issl_dn0: f64,
        var_issl_dn10: f64,
        var_issl_dn11: f64,
        var_issl_dn12: f64,
        var_issl_dn13: f64,
        var_issl_dn14: f64,
        var_issl_dn2: f64,
        var_issl_dn3: f64,
        var_issl_dn4: f64,
        var_issl_dn5: f64,
        var_issl_dn6: f64,
        var_issl_dn7: f64,
        var_issl_dn8: f64,
        var_issl_dn9: f64,
        var_leff: f64,
        var_mig: f64,
        var_mig_dn0: f64,
        var_mig_dn10: f64,
        var_mig_dn11: f64,
        var_mig_dn12: f64,
        var_mig_dn13: f64,
        var_mig_dn14: f64,
        var_mig_dn2: f64,
        var_mig_dn3: f64,
        var_mig_dn4: f64,
        var_mig_dn5: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mig_dn9: f64,
        var_sigvds: f64,
        var_sqid: f64,
        var_sqid_dn0: f64,
        var_sqid_dn10: f64,
        var_sqid_dn11: f64,
        var_sqid_dn12: f64,
        var_sqid_dn13: f64,
        var_sqid_dn14: f64,
        var_sqid_dn2: f64,
        var_sqid_dn3: f64,
        var_sqid_dn4: f64,
        var_sqid_dn5: f64,
        var_sqid_dn6: f64,
        var_sqid_dn7: f64,
        var_sqid_dn8: f64,
        var_sqid_dn9: f64,
        var_sqig: f64,
        var_sqig_dn0: f64,
        var_sqig_dn10: f64,
        var_sqig_dn11: f64,
        var_sqig_dn12: f64,
        var_sqig_dn13: f64,
        var_sqig_dn14: f64,
        var_sqig_dn2: f64,
        var_sqig_dn3: f64,
        var_sqig_dn4: f64,
        var_sqig_dn5: f64,
        var_sqig_dn6: f64,
        var_sqig_dn7: f64,
        var_sqig_dn8: f64,
        var_sqig_dn9: f64,
        var_weff: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq1_e1207, eq1_e1207_d_n0, eq1_e1207_d_n2, eq1_e1207_d_n3, eq1_e1207_d_n4, eq1_e1207_d_n5, eq1_e1207_d_n6, eq1_e1207_d_n7, eq1_e1207_d_n8, eq1_e1207_d_n9, eq1_e1207_d_n10, eq1_e1207_d_n11, eq1_e1207_d_n12, eq1_e1207_d_n13, eq1_e1207_d_n14,) = {
    if (var_guard680 != 0.0) {
        let eq1_e1203: f64 = (var_devsign * p.p28);
        let eq1_e1205: f64 = (eq1_e1203 * var_issl);
        let eq1_e1205_d_n0: f64 = (eq1_e1203 * var_issl_dn0);
        let eq1_e1205_d_n2: f64 = (eq1_e1203 * var_issl_dn2);
        let eq1_e1205_d_n3: f64 = (eq1_e1203 * var_issl_dn3);
        let eq1_e1205_d_n4: f64 = (eq1_e1203 * var_issl_dn4);
        let eq1_e1205_d_n5: f64 = (eq1_e1203 * var_issl_dn5);
        let eq1_e1205_d_n6: f64 = (eq1_e1203 * var_issl_dn6);
        let eq1_e1205_d_n7: f64 = (eq1_e1203 * var_issl_dn7);
        let eq1_e1205_d_n8: f64 = (eq1_e1203 * var_issl_dn8);
        let eq1_e1205_d_n9: f64 = (eq1_e1203 * var_issl_dn9);
        let eq1_e1205_d_n10: f64 = (eq1_e1203 * var_issl_dn10);
        let eq1_e1205_d_n11: f64 = (eq1_e1203 * var_issl_dn11);
        let eq1_e1205_d_n12: f64 = (eq1_e1203 * var_issl_dn12);
        let eq1_e1205_d_n13: f64 = (eq1_e1203 * var_issl_dn13);
        let eq1_e1205_d_n14: f64 = (eq1_e1203 * var_issl_dn14);
        (eq1_e1205, eq1_e1205_d_n0, eq1_e1205_d_n2, eq1_e1205_d_n3, eq1_e1205_d_n4, eq1_e1205_d_n5, eq1_e1205_d_n6, eq1_e1205_d_n7, eq1_e1205_d_n8, eq1_e1205_d_n9, eq1_e1205_d_n10, eq1_e1205_d_n11, eq1_e1205_d_n12, eq1_e1205_d_n13, eq1_e1205_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1207;
        let eq1_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq1_node_derivatives: [f64; 14] = [eq1_e1207_d_n0, eq1_e1207_d_n2, eq1_e1207_d_n3, eq1_e1207_d_n4, eq1_e1207_d_n5, eq1_e1207_d_n6, eq1_e1207_d_n7, eq1_e1207_d_n8, eq1_e1207_d_n9, eq1_e1207_d_n10, eq1_e1207_d_n11, eq1_e1207_d_n12, eq1_e1207_d_n13, eq1_e1207_d_n14];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1272, eq7_e1272_d_n0, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n16,) = {
    if ((var_guard698 != 0.0) && (var_guard697 == 0.0)) {
        let eq7_e1268: f64 = (-var_sqig);
        let eq7_e1270: f64 = (eq7_e1268 * (nv16 - 0.0));
        let eq7_e1270_d_n0: f64 = ((-var_sqig_dn0) * (nv16 - 0.0));
        let eq7_e1270_d_n2: f64 = ((-var_sqig_dn2) * (nv16 - 0.0));
        let eq7_e1270_d_n3: f64 = ((-var_sqig_dn3) * (nv16 - 0.0));
        let eq7_e1270_d_n4: f64 = ((-var_sqig_dn4) * (nv16 - 0.0));
        let eq7_e1270_d_n5: f64 = ((-var_sqig_dn5) * (nv16 - 0.0));
        let eq7_e1270_d_n6: f64 = ((-var_sqig_dn6) * (nv16 - 0.0));
        let eq7_e1270_d_n7: f64 = ((-var_sqig_dn7) * (nv16 - 0.0));
        let eq7_e1270_d_n8: f64 = ((-var_sqig_dn8) * (nv16 - 0.0));
        let eq7_e1270_d_n9: f64 = ((-var_sqig_dn9) * (nv16 - 0.0));
        let eq7_e1270_d_n10: f64 = ((-var_sqig_dn10) * (nv16 - 0.0));
        let eq7_e1270_d_n11: f64 = ((-var_sqig_dn11) * (nv16 - 0.0));
        let eq7_e1270_d_n12: f64 = ((-var_sqig_dn12) * (nv16 - 0.0));
        let eq7_e1270_d_n13: f64 = ((-var_sqig_dn13) * (nv16 - 0.0));
        let eq7_e1270_d_n14: f64 = ((-var_sqig_dn14) * (nv16 - 0.0));
        (eq7_e1270, eq7_e1270_d_n0, eq7_e1270_d_n2, eq7_e1270_d_n3, eq7_e1270_d_n4, eq7_e1270_d_n5, eq7_e1270_d_n6, eq7_e1270_d_n7, eq7_e1270_d_n8, eq7_e1270_d_n9, eq7_e1270_d_n10, eq7_e1270_d_n11, eq7_e1270_d_n12, eq7_e1270_d_n13, eq7_e1270_d_n14, eq7_e1268,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1272;
        let eq7_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16];
        let eq7_node_derivatives: [f64; 15] = [eq7_e1272_d_n0, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n16];
        let eq7_branch_derivative_indices: [usize; 0] = [];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivative_indices,
            &eq7_node_derivatives,
            &eq7_branch_derivative_indices,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15,) = {
    if ((var_guard698 != 0.0) && (var_guard697 == 0.0)) {
        let eq8_e1279: f64 = (var_mig * var_cox);
        let eq8_e1279_d_n0: f64 = (var_mig_dn0 * var_cox);
        let eq8_e1279_d_n2: f64 = (var_mig_dn2 * var_cox);
        let eq8_e1279_d_n3: f64 = (var_mig_dn3 * var_cox);
        let eq8_e1279_d_n4: f64 = (var_mig_dn4 * var_cox);
        let eq8_e1279_d_n5: f64 = (var_mig_dn5 * var_cox);
        let eq8_e1279_d_n6: f64 = (var_mig_dn6 * var_cox);
        let eq8_e1279_d_n7: f64 = (var_mig_dn7 * var_cox);
        let eq8_e1279_d_n8: f64 = (var_mig_dn8 * var_cox);
        let eq8_e1279_d_n9: f64 = (var_mig_dn9 * var_cox);
        let eq8_e1279_d_n10: f64 = (var_mig_dn10 * var_cox);
        let eq8_e1279_d_n11: f64 = (var_mig_dn11 * var_cox);
        let eq8_e1279_d_n12: f64 = (var_mig_dn12 * var_cox);
        let eq8_e1279_d_n13: f64 = (var_mig_dn13 * var_cox);
        let eq8_e1279_d_n14: f64 = (var_mig_dn14 * var_cox);
        let eq8_e1281: f64 = (eq8_e1279 * var_weff);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * var_weff);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * var_weff);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * var_weff);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * var_weff);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * var_weff);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * var_weff);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * var_weff);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * var_weff);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * var_weff);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * var_weff);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * var_weff);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * var_weff);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * var_weff);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * var_weff);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * var_leff);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * var_leff);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * var_leff);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * var_leff);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * var_leff);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * var_leff);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * var_leff);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * var_leff);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * var_leff);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * var_leff);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * var_leff);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * var_leff);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * var_leff);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * var_leff);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * var_leff);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1288: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq8_e1287);
        (eq8_e1288, (eq8_e1287_d_n0 * ddt_scale), (eq8_e1287_d_n2 * ddt_scale), (eq8_e1287_d_n3 * ddt_scale), (eq8_e1287_d_n4 * ddt_scale), (eq8_e1287_d_n5 * ddt_scale), (eq8_e1287_d_n6 * ddt_scale), (eq8_e1287_d_n7 * ddt_scale), (eq8_e1287_d_n8 * ddt_scale), (eq8_e1287_d_n9 * ddt_scale), (eq8_e1287_d_n10 * ddt_scale), (eq8_e1287_d_n11 * ddt_scale), (eq8_e1287_d_n12 * ddt_scale), (eq8_e1287_d_n13 * ddt_scale), (eq8_e1287_d_n14 * ddt_scale), (eq8_e1285 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e1290;
        let eq8_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq8_node_derivatives: [f64; 15] = [eq8_e1290_d_n0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1318, eq10_e1318_d_n0, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n16,) = {
    if ((var_guard698 != 0.0) && (var_guard697 == 0.0)) {
        let eq10_e1314: f64 = (var_sqid * p.p28);
        let eq10_e1314_d_n0: f64 = (var_sqid_dn0 * p.p28);
        let eq10_e1314_d_n2: f64 = (var_sqid_dn2 * p.p28);
        let eq10_e1314_d_n3: f64 = (var_sqid_dn3 * p.p28);
        let eq10_e1314_d_n4: f64 = (var_sqid_dn4 * p.p28);
        let eq10_e1314_d_n5: f64 = (var_sqid_dn5 * p.p28);
        let eq10_e1314_d_n6: f64 = (var_sqid_dn6 * p.p28);
        let eq10_e1314_d_n7: f64 = (var_sqid_dn7 * p.p28);
        let eq10_e1314_d_n8: f64 = (var_sqid_dn8 * p.p28);
        let eq10_e1314_d_n9: f64 = (var_sqid_dn9 * p.p28);
        let eq10_e1314_d_n10: f64 = (var_sqid_dn10 * p.p28);
        let eq10_e1314_d_n11: f64 = (var_sqid_dn11 * p.p28);
        let eq10_e1314_d_n12: f64 = (var_sqid_dn12 * p.p28);
        let eq10_e1314_d_n13: f64 = (var_sqid_dn13 * p.p28);
        let eq10_e1314_d_n14: f64 = (var_sqid_dn14 * p.p28);
        let eq10_e1316: f64 = (eq10_e1314 * (nv16 - 0.0));
        let eq10_e1316_d_n0: f64 = (eq10_e1314_d_n0 * (nv16 - 0.0));
        let eq10_e1316_d_n2: f64 = (eq10_e1314_d_n2 * (nv16 - 0.0));
        let eq10_e1316_d_n3: f64 = (eq10_e1314_d_n3 * (nv16 - 0.0));
        let eq10_e1316_d_n4: f64 = (eq10_e1314_d_n4 * (nv16 - 0.0));
        let eq10_e1316_d_n5: f64 = (eq10_e1314_d_n5 * (nv16 - 0.0));
        let eq10_e1316_d_n6: f64 = (eq10_e1314_d_n6 * (nv16 - 0.0));
        let eq10_e1316_d_n7: f64 = (eq10_e1314_d_n7 * (nv16 - 0.0));
        let eq10_e1316_d_n8: f64 = (eq10_e1314_d_n8 * (nv16 - 0.0));
        let eq10_e1316_d_n9: f64 = (eq10_e1314_d_n9 * (nv16 - 0.0));
        let eq10_e1316_d_n10: f64 = (eq10_e1314_d_n10 * (nv16 - 0.0));
        let eq10_e1316_d_n11: f64 = (eq10_e1314_d_n11 * (nv16 - 0.0));
        let eq10_e1316_d_n12: f64 = (eq10_e1314_d_n12 * (nv16 - 0.0));
        let eq10_e1316_d_n13: f64 = (eq10_e1314_d_n13 * (nv16 - 0.0));
        let eq10_e1316_d_n14: f64 = (eq10_e1314_d_n14 * (nv16 - 0.0));
        (eq10_e1316, eq10_e1316_d_n0, eq10_e1316_d_n2, eq10_e1316_d_n3, eq10_e1316_d_n4, eq10_e1316_d_n5, eq10_e1316_d_n6, eq10_e1316_d_n7, eq10_e1316_d_n8, eq10_e1316_d_n9, eq10_e1316_d_n10, eq10_e1316_d_n11, eq10_e1316_d_n12, eq10_e1316_d_n13, eq10_e1316_d_n14, eq10_e1314,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1318;
        let eq10_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16];
        let eq10_node_derivatives: [f64; 15] = [eq10_e1318_d_n0, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n16];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15,) = {
    if ((var_guard698 != 0.0) && (var_guard697 == 0.0)) {
        let eq11_e1327: f64 = (1.0 + var_sigvds);
        let eq11_e1329: f64 = (eq11_e1327 * var_mig);
        let eq11_e1329_d_n0: f64 = (eq11_e1327 * var_mig_dn0);
        let eq11_e1329_d_n2: f64 = (eq11_e1327 * var_mig_dn2);
        let eq11_e1329_d_n3: f64 = (eq11_e1327 * var_mig_dn3);
        let eq11_e1329_d_n4: f64 = (eq11_e1327 * var_mig_dn4);
        let eq11_e1329_d_n5: f64 = (eq11_e1327 * var_mig_dn5);
        let eq11_e1329_d_n6: f64 = (eq11_e1327 * var_mig_dn6);
        let eq11_e1329_d_n7: f64 = (eq11_e1327 * var_mig_dn7);
        let eq11_e1329_d_n8: f64 = (eq11_e1327 * var_mig_dn8);
        let eq11_e1329_d_n9: f64 = (eq11_e1327 * var_mig_dn9);
        let eq11_e1329_d_n10: f64 = (eq11_e1327 * var_mig_dn10);
        let eq11_e1329_d_n11: f64 = (eq11_e1327 * var_mig_dn11);
        let eq11_e1329_d_n12: f64 = (eq11_e1327 * var_mig_dn12);
        let eq11_e1329_d_n13: f64 = (eq11_e1327 * var_mig_dn13);
        let eq11_e1329_d_n14: f64 = (eq11_e1327 * var_mig_dn14);
        let eq11_e1331: f64 = (eq11_e1329 * var_cox);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * var_cox);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * var_cox);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * var_cox);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * var_cox);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * var_cox);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * var_cox);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * var_cox);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * var_cox);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * var_cox);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * var_cox);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * var_cox);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * var_cox);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * var_cox);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * var_cox);
        let eq11_e1333: f64 = (eq11_e1331 * var_weff);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * var_weff);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * var_weff);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * var_weff);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * var_weff);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * var_weff);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * var_weff);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * var_weff);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * var_weff);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * var_weff);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * var_weff);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * var_weff);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * var_weff);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * var_weff);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * var_weff);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * var_leff);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * var_leff);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * var_leff);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * var_leff);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * var_leff);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * var_leff);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * var_leff);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * var_leff);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * var_leff);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * var_leff);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * var_leff);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * var_leff);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * var_leff);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * var_leff);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * var_leff);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1337);
        let eq11_e1341: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e1340);
        let eq11_e1342: f64 = (p.p29 * eq11_e1341);
        let eq11_e1342_d_n0: f64 = (p.p29 * (eq11_e1340_d_n0 * ddt_scale));
        let eq11_e1342_d_n2: f64 = (p.p29 * (eq11_e1340_d_n2 * ddt_scale));
        let eq11_e1342_d_n3: f64 = (p.p29 * (eq11_e1340_d_n3 * ddt_scale));
        let eq11_e1342_d_n4: f64 = (p.p29 * (eq11_e1340_d_n4 * ddt_scale));
        let eq11_e1342_d_n5: f64 = (p.p29 * (eq11_e1340_d_n5 * ddt_scale));
        let eq11_e1342_d_n6: f64 = (p.p29 * (eq11_e1340_d_n6 * ddt_scale));
        let eq11_e1342_d_n7: f64 = (p.p29 * (eq11_e1340_d_n7 * ddt_scale));
        let eq11_e1342_d_n8: f64 = (p.p29 * (eq11_e1340_d_n8 * ddt_scale));
        let eq11_e1342_d_n9: f64 = (p.p29 * (eq11_e1340_d_n9 * ddt_scale));
        let eq11_e1342_d_n10: f64 = (p.p29 * (eq11_e1340_d_n10 * ddt_scale));
        let eq11_e1342_d_n11: f64 = (p.p29 * (eq11_e1340_d_n11 * ddt_scale));
        let eq11_e1342_d_n12: f64 = (p.p29 * (eq11_e1340_d_n12 * ddt_scale));
        let eq11_e1342_d_n13: f64 = (p.p29 * (eq11_e1340_d_n13 * ddt_scale));
        let eq11_e1342_d_n14: f64 = (p.p29 * (eq11_e1340_d_n14 * ddt_scale));
        let eq11_e1342_d_n15: f64 = (p.p29 * (eq11_e1340_d_n15 * ddt_scale));
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1344;
        let eq11_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq11_node_derivatives: [f64; 15] = [eq11_e1344_d_n0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_cox: f64,
        var_devsign: f64,
        var_guard697: f64,
        var_guard698: f64,
        var_guard751: f64,
        var_guard752: f64,
        var_guard753: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn13: f64,
        var_ids_dn14: f64,
        var_ids_dn2: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_igb_1: f64,
        var_igb_1_dn0: f64,
        var_igb_1_dn10: f64,
        var_igb_1_dn11: f64,
        var_igb_1_dn12: f64,
        var_igb_1_dn13: f64,
        var_igb_1_dn14: f64,
        var_igb_1_dn2: f64,
        var_igb_1_dn3: f64,
        var_igb_1_dn4: f64,
        var_igb_1_dn5: f64,
        var_igb_1_dn6: f64,
        var_igb_1_dn7: f64,
        var_igb_1_dn8: f64,
        var_igb_1_dn9: f64,
        var_igcd_1: f64,
        var_igcd_1_dn0: f64,
        var_igcd_1_dn10: f64,
        var_igcd_1_dn11: f64,
        var_igcd_1_dn12: f64,
        var_igcd_1_dn13: f64,
        var_igcd_1_dn14: f64,
        var_igcd_1_dn2: f64,
        var_igcd_1_dn3: f64,
        var_igcd_1_dn4: f64,
        var_igcd_1_dn5: f64,
        var_igcd_1_dn6: f64,
        var_igcd_1_dn7: f64,
        var_igcd_1_dn8: f64,
        var_igcd_1_dn9: f64,
        var_igcs_1: f64,
        var_igcs_1_dn0: f64,
        var_igcs_1_dn10: f64,
        var_igcs_1_dn11: f64,
        var_igcs_1_dn12: f64,
        var_igcs_1_dn13: f64,
        var_igcs_1_dn14: f64,
        var_igcs_1_dn2: f64,
        var_igcs_1_dn3: f64,
        var_igcs_1_dn4: f64,
        var_igcs_1_dn5: f64,
        var_igcs_1_dn6: f64,
        var_igcs_1_dn7: f64,
        var_igcs_1_dn8: f64,
        var_igcs_1_dn9: f64,
        var_igd_1: f64,
        var_igd_1_dn0: f64,
        var_igd_1_dn10: f64,
        var_igd_1_dn11: f64,
        var_igd_1_dn12: f64,
        var_igd_1_dn13: f64,
        var_igd_1_dn14: f64,
        var_igd_1_dn2: f64,
        var_igd_1_dn3: f64,
        var_igd_1_dn4: f64,
        var_igd_1_dn5: f64,
        var_igd_1_dn6: f64,
        var_igd_1_dn7: f64,
        var_igd_1_dn8: f64,
        var_igd_1_dn9: f64,
        var_igidl_1: f64,
        var_igidl_1_dn0: f64,
        var_igidl_1_dn10: f64,
        var_igidl_1_dn11: f64,
        var_igidl_1_dn12: f64,
        var_igidl_1_dn13: f64,
        var_igidl_1_dn14: f64,
        var_igidl_1_dn2: f64,
        var_igidl_1_dn3: f64,
        var_igidl_1_dn4: f64,
        var_igidl_1_dn5: f64,
        var_igidl_1_dn6: f64,
        var_igidl_1_dn7: f64,
        var_igidl_1_dn8: f64,
        var_igidl_1_dn9: f64,
        var_igs_1: f64,
        var_igs_1_dn0: f64,
        var_igs_1_dn10: f64,
        var_igs_1_dn11: f64,
        var_igs_1_dn12: f64,
        var_igs_1_dn13: f64,
        var_igs_1_dn14: f64,
        var_igs_1_dn2: f64,
        var_igs_1_dn3: f64,
        var_igs_1_dn4: f64,
        var_igs_1_dn5: f64,
        var_igs_1_dn6: f64,
        var_igs_1_dn7: f64,
        var_igs_1_dn8: f64,
        var_igs_1_dn9: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn13: f64,
        var_isub_dn14: f64,
        var_isub_dn2: f64,
        var_isub_dn3: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_leff: f64,
        var_mig: f64,
        var_mig_dn0: f64,
        var_mig_dn10: f64,
        var_mig_dn11: f64,
        var_mig_dn12: f64,
        var_mig_dn13: f64,
        var_mig_dn14: f64,
        var_mig_dn2: f64,
        var_mig_dn3: f64,
        var_mig_dn4: f64,
        var_mig_dn5: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mig_dn9: f64,
        var_qdi_1: f64,
        var_qdi_1_dn0: f64,
        var_qdi_1_dn10: f64,
        var_qdi_1_dn11: f64,
        var_qdi_1_dn12: f64,
        var_qdi_1_dn13: f64,
        var_qdi_1_dn14: f64,
        var_qdi_1_dn2: f64,
        var_qdi_1_dn3: f64,
        var_qdi_1_dn4: f64,
        var_qdi_1_dn5: f64,
        var_qdi_1_dn6: f64,
        var_qdi_1_dn7: f64,
        var_qdi_1_dn8: f64,
        var_qdi_1_dn9: f64,
        var_qgi_1: f64,
        var_qgi_1_dn0: f64,
        var_qgi_1_dn10: f64,
        var_qgi_1_dn11: f64,
        var_qgi_1_dn12: f64,
        var_qgi_1_dn13: f64,
        var_qgi_1_dn14: f64,
        var_qgi_1_dn2: f64,
        var_qgi_1_dn3: f64,
        var_qgi_1_dn4: f64,
        var_qgi_1_dn5: f64,
        var_qgi_1_dn6: f64,
        var_qgi_1_dn7: f64,
        var_qgi_1_dn8: f64,
        var_qgi_1_dn9: f64,
        var_qovb: f64,
        var_qovb_dn0: f64,
        var_qovb_dn10: f64,
        var_qovb_dn11: f64,
        var_qovb_dn12: f64,
        var_qovb_dn13: f64,
        var_qovb_dn14: f64,
        var_qovb_dn2: f64,
        var_qovb_dn3: f64,
        var_qovb_dn4: f64,
        var_qovb_dn5: f64,
        var_qovb_dn6: f64,
        var_qovb_dn7: f64,
        var_qovb_dn8: f64,
        var_qovb_dn9: f64,
        var_qovd: f64,
        var_qovd_dn0: f64,
        var_qovd_dn10: f64,
        var_qovd_dn11: f64,
        var_qovd_dn12: f64,
        var_qovd_dn13: f64,
        var_qovd_dn14: f64,
        var_qovd_dn2: f64,
        var_qovd_dn3: f64,
        var_qovd_dn4: f64,
        var_qovd_dn5: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovd_dn8: f64,
        var_qovd_dn9: f64,
        var_qovs: f64,
        var_qovs_dn0: f64,
        var_qovs_dn10: f64,
        var_qovs_dn11: f64,
        var_qovs_dn12: f64,
        var_qovs_dn13: f64,
        var_qovs_dn14: f64,
        var_qovs_dn2: f64,
        var_qovs_dn3: f64,
        var_qovs_dn4: f64,
        var_qovs_dn5: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qovs_dn8: f64,
        var_qovs_dn9: f64,
        var_qsi_1: f64,
        var_qsi_1_dn0: f64,
        var_qsi_1_dn10: f64,
        var_qsi_1_dn11: f64,
        var_qsi_1_dn12: f64,
        var_qsi_1_dn13: f64,
        var_qsi_1_dn14: f64,
        var_qsi_1_dn2: f64,
        var_qsi_1_dn3: f64,
        var_qsi_1_dn4: f64,
        var_qsi_1_dn5: f64,
        var_qsi_1_dn6: f64,
        var_qsi_1_dn7: f64,
        var_qsi_1_dn8: f64,
        var_qsi_1_dn9: f64,
        var_sigvds: f64,
        var_weff: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15,) = {
    if ((var_guard698 != 0.0) && (var_guard697 == 0.0)) {
        let eq12_e1353: f64 = (1.0 - var_sigvds);
        let eq12_e1355: f64 = (eq12_e1353 * var_mig);
        let eq12_e1355_d_n0: f64 = (eq12_e1353 * var_mig_dn0);
        let eq12_e1355_d_n2: f64 = (eq12_e1353 * var_mig_dn2);
        let eq12_e1355_d_n3: f64 = (eq12_e1353 * var_mig_dn3);
        let eq12_e1355_d_n4: f64 = (eq12_e1353 * var_mig_dn4);
        let eq12_e1355_d_n5: f64 = (eq12_e1353 * var_mig_dn5);
        let eq12_e1355_d_n6: f64 = (eq12_e1353 * var_mig_dn6);
        let eq12_e1355_d_n7: f64 = (eq12_e1353 * var_mig_dn7);
        let eq12_e1355_d_n8: f64 = (eq12_e1353 * var_mig_dn8);
        let eq12_e1355_d_n9: f64 = (eq12_e1353 * var_mig_dn9);
        let eq12_e1355_d_n10: f64 = (eq12_e1353 * var_mig_dn10);
        let eq12_e1355_d_n11: f64 = (eq12_e1353 * var_mig_dn11);
        let eq12_e1355_d_n12: f64 = (eq12_e1353 * var_mig_dn12);
        let eq12_e1355_d_n13: f64 = (eq12_e1353 * var_mig_dn13);
        let eq12_e1355_d_n14: f64 = (eq12_e1353 * var_mig_dn14);
        let eq12_e1357: f64 = (eq12_e1355 * var_cox);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * var_cox);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * var_cox);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * var_cox);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * var_cox);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * var_cox);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * var_cox);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * var_cox);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * var_cox);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * var_cox);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * var_cox);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * var_cox);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * var_cox);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * var_cox);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * var_cox);
        let eq12_e1359: f64 = (eq12_e1357 * var_weff);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * var_weff);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * var_weff);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * var_weff);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * var_weff);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * var_weff);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * var_weff);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * var_weff);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * var_weff);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * var_weff);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * var_weff);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * var_weff);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * var_weff);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * var_weff);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * var_weff);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * var_leff);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * var_leff);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * var_leff);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * var_leff);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * var_leff);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * var_leff);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * var_leff);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * var_leff);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * var_leff);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * var_leff);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * var_leff);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * var_leff);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * var_leff);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * var_leff);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * var_leff);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1363);
        let eq12_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e1366);
        let eq12_e1368: f64 = (p.p29 * eq12_e1367);
        let eq12_e1368_d_n0: f64 = (p.p29 * (eq12_e1366_d_n0 * ddt_scale));
        let eq12_e1368_d_n2: f64 = (p.p29 * (eq12_e1366_d_n2 * ddt_scale));
        let eq12_e1368_d_n3: f64 = (p.p29 * (eq12_e1366_d_n3 * ddt_scale));
        let eq12_e1368_d_n4: f64 = (p.p29 * (eq12_e1366_d_n4 * ddt_scale));
        let eq12_e1368_d_n5: f64 = (p.p29 * (eq12_e1366_d_n5 * ddt_scale));
        let eq12_e1368_d_n6: f64 = (p.p29 * (eq12_e1366_d_n6 * ddt_scale));
        let eq12_e1368_d_n7: f64 = (p.p29 * (eq12_e1366_d_n7 * ddt_scale));
        let eq12_e1368_d_n8: f64 = (p.p29 * (eq12_e1366_d_n8 * ddt_scale));
        let eq12_e1368_d_n9: f64 = (p.p29 * (eq12_e1366_d_n9 * ddt_scale));
        let eq12_e1368_d_n10: f64 = (p.p29 * (eq12_e1366_d_n10 * ddt_scale));
        let eq12_e1368_d_n11: f64 = (p.p29 * (eq12_e1366_d_n11 * ddt_scale));
        let eq12_e1368_d_n12: f64 = (p.p29 * (eq12_e1366_d_n12 * ddt_scale));
        let eq12_e1368_d_n13: f64 = (p.p29 * (eq12_e1366_d_n13 * ddt_scale));
        let eq12_e1368_d_n14: f64 = (p.p29 * (eq12_e1366_d_n14 * ddt_scale));
        let eq12_e1368_d_n15: f64 = (p.p29 * (eq12_e1366_d_n15 * ddt_scale));
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1370;
        let eq12_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq12_node_derivatives: [f64; 15] = [eq12_e1370_d_n0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq19_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qgi_1);
        let eq19_value: f64 = eq19_e1428;
        let eq19_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq19_node_derivatives: [f64; 14] = [(var_qgi_1_dn0 * ddt_scale), (var_qgi_1_dn2 * ddt_scale), (var_qgi_1_dn3 * ddt_scale), (var_qgi_1_dn4 * ddt_scale), (var_qgi_1_dn5 * ddt_scale), (var_qgi_1_dn6 * ddt_scale), (var_qgi_1_dn7 * ddt_scale), (var_qgi_1_dn8 * ddt_scale), (var_qgi_1_dn9 * ddt_scale), (var_qgi_1_dn10 * ddt_scale), (var_qgi_1_dn11 * ddt_scale), (var_qgi_1_dn12 * ddt_scale), (var_qgi_1_dn13 * ddt_scale), (var_qgi_1_dn14 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(11),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e1430: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qsi_1);
        let eq20_value: f64 = eq20_e1430;
        let eq20_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq20_node_derivatives: [f64; 14] = [(var_qsi_1_dn0 * ddt_scale), (var_qsi_1_dn2 * ddt_scale), (var_qsi_1_dn3 * ddt_scale), (var_qsi_1_dn4 * ddt_scale), (var_qsi_1_dn5 * ddt_scale), (var_qsi_1_dn6 * ddt_scale), (var_qsi_1_dn7 * ddt_scale), (var_qsi_1_dn8 * ddt_scale), (var_qsi_1_dn9 * ddt_scale), (var_qsi_1_dn10 * ddt_scale), (var_qsi_1_dn11 * ddt_scale), (var_qsi_1_dn12 * ddt_scale), (var_qsi_1_dn13 * ddt_scale), (var_qsi_1_dn14 * ddt_scale)];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e1432: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, var_qdi_1);
        let eq21_value: f64 = eq21_e1432;
        let eq21_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq21_node_derivatives: [f64; 14] = [(var_qdi_1_dn0 * ddt_scale), (var_qdi_1_dn2 * ddt_scale), (var_qdi_1_dn3 * ddt_scale), (var_qdi_1_dn4 * ddt_scale), (var_qdi_1_dn5 * ddt_scale), (var_qdi_1_dn6 * ddt_scale), (var_qdi_1_dn7 * ddt_scale), (var_qdi_1_dn8 * ddt_scale), (var_qdi_1_dn9 * ddt_scale), (var_qdi_1_dn10 * ddt_scale), (var_qdi_1_dn11 * ddt_scale), (var_qdi_1_dn12 * ddt_scale), (var_qdi_1_dn13 * ddt_scale), (var_qdi_1_dn14 * ddt_scale)];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e1435: f64 = (-var_devsign);
        let eq22_e1437: f64 = (eq22_e1435 * var_qovs);
        let eq22_e1437_d_n0: f64 = (eq22_e1435 * var_qovs_dn0);
        let eq22_e1437_d_n2: f64 = (eq22_e1435 * var_qovs_dn2);
        let eq22_e1437_d_n3: f64 = (eq22_e1435 * var_qovs_dn3);
        let eq22_e1437_d_n4: f64 = (eq22_e1435 * var_qovs_dn4);
        let eq22_e1437_d_n5: f64 = (eq22_e1435 * var_qovs_dn5);
        let eq22_e1437_d_n6: f64 = (eq22_e1435 * var_qovs_dn6);
        let eq22_e1437_d_n7: f64 = (eq22_e1435 * var_qovs_dn7);
        let eq22_e1437_d_n8: f64 = (eq22_e1435 * var_qovs_dn8);
        let eq22_e1437_d_n9: f64 = (eq22_e1435 * var_qovs_dn9);
        let eq22_e1437_d_n10: f64 = (eq22_e1435 * var_qovs_dn10);
        let eq22_e1437_d_n11: f64 = (eq22_e1435 * var_qovs_dn11);
        let eq22_e1437_d_n12: f64 = (eq22_e1435 * var_qovs_dn12);
        let eq22_e1437_d_n13: f64 = (eq22_e1435 * var_qovs_dn13);
        let eq22_e1437_d_n14: f64 = (eq22_e1435 * var_qovs_dn14);
        let eq22_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq22_e1437);
        let eq22_e1439: f64 = (p.p29 * eq22_e1438);
        let eq22_e1439_d_n0: f64 = (p.p29 * (eq22_e1437_d_n0 * ddt_scale));
        let eq22_e1439_d_n2: f64 = (p.p29 * (eq22_e1437_d_n2 * ddt_scale));
        let eq22_e1439_d_n3: f64 = (p.p29 * (eq22_e1437_d_n3 * ddt_scale));
        let eq22_e1439_d_n4: f64 = (p.p29 * (eq22_e1437_d_n4 * ddt_scale));
        let eq22_e1439_d_n5: f64 = (p.p29 * (eq22_e1437_d_n5 * ddt_scale));
        let eq22_e1439_d_n6: f64 = (p.p29 * (eq22_e1437_d_n6 * ddt_scale));
        let eq22_e1439_d_n7: f64 = (p.p29 * (eq22_e1437_d_n7 * ddt_scale));
        let eq22_e1439_d_n8: f64 = (p.p29 * (eq22_e1437_d_n8 * ddt_scale));
        let eq22_e1439_d_n9: f64 = (p.p29 * (eq22_e1437_d_n9 * ddt_scale));
        let eq22_e1439_d_n10: f64 = (p.p29 * (eq22_e1437_d_n10 * ddt_scale));
        let eq22_e1439_d_n11: f64 = (p.p29 * (eq22_e1437_d_n11 * ddt_scale));
        let eq22_e1439_d_n12: f64 = (p.p29 * (eq22_e1437_d_n12 * ddt_scale));
        let eq22_e1439_d_n13: f64 = (p.p29 * (eq22_e1437_d_n13 * ddt_scale));
        let eq22_e1439_d_n14: f64 = (p.p29 * (eq22_e1437_d_n14 * ddt_scale));
        let eq22_value: f64 = eq22_e1439;
        let eq22_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq22_node_derivatives: [f64; 14] = [eq22_e1439_d_n0, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14];
        let eq22_branch_derivative_indices: [usize; 0] = [];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq22_value),
            &eq22_node_derivative_indices,
            &eq22_node_derivatives,
            &eq22_branch_derivative_indices,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-var_devsign);
        let eq23_e1444: f64 = (eq23_e1442 * var_qovd);
        let eq23_e1444_d_n0: f64 = (eq23_e1442 * var_qovd_dn0);
        let eq23_e1444_d_n2: f64 = (eq23_e1442 * var_qovd_dn2);
        let eq23_e1444_d_n3: f64 = (eq23_e1442 * var_qovd_dn3);
        let eq23_e1444_d_n4: f64 = (eq23_e1442 * var_qovd_dn4);
        let eq23_e1444_d_n5: f64 = (eq23_e1442 * var_qovd_dn5);
        let eq23_e1444_d_n6: f64 = (eq23_e1442 * var_qovd_dn6);
        let eq23_e1444_d_n7: f64 = (eq23_e1442 * var_qovd_dn7);
        let eq23_e1444_d_n8: f64 = (eq23_e1442 * var_qovd_dn8);
        let eq23_e1444_d_n9: f64 = (eq23_e1442 * var_qovd_dn9);
        let eq23_e1444_d_n10: f64 = (eq23_e1442 * var_qovd_dn10);
        let eq23_e1444_d_n11: f64 = (eq23_e1442 * var_qovd_dn11);
        let eq23_e1444_d_n12: f64 = (eq23_e1442 * var_qovd_dn12);
        let eq23_e1444_d_n13: f64 = (eq23_e1442 * var_qovd_dn13);
        let eq23_e1444_d_n14: f64 = (eq23_e1442 * var_qovd_dn14);
        let eq23_e1445: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq23_e1444);
        let eq23_e1446: f64 = (p.p29 * eq23_e1445);
        let eq23_e1446_d_n0: f64 = (p.p29 * (eq23_e1444_d_n0 * ddt_scale));
        let eq23_e1446_d_n2: f64 = (p.p29 * (eq23_e1444_d_n2 * ddt_scale));
        let eq23_e1446_d_n3: f64 = (p.p29 * (eq23_e1444_d_n3 * ddt_scale));
        let eq23_e1446_d_n4: f64 = (p.p29 * (eq23_e1444_d_n4 * ddt_scale));
        let eq23_e1446_d_n5: f64 = (p.p29 * (eq23_e1444_d_n5 * ddt_scale));
        let eq23_e1446_d_n6: f64 = (p.p29 * (eq23_e1444_d_n6 * ddt_scale));
        let eq23_e1446_d_n7: f64 = (p.p29 * (eq23_e1444_d_n7 * ddt_scale));
        let eq23_e1446_d_n8: f64 = (p.p29 * (eq23_e1444_d_n8 * ddt_scale));
        let eq23_e1446_d_n9: f64 = (p.p29 * (eq23_e1444_d_n9 * ddt_scale));
        let eq23_e1446_d_n10: f64 = (p.p29 * (eq23_e1444_d_n10 * ddt_scale));
        let eq23_e1446_d_n11: f64 = (p.p29 * (eq23_e1444_d_n11 * ddt_scale));
        let eq23_e1446_d_n12: f64 = (p.p29 * (eq23_e1444_d_n12 * ddt_scale));
        let eq23_e1446_d_n13: f64 = (p.p29 * (eq23_e1444_d_n13 * ddt_scale));
        let eq23_e1446_d_n14: f64 = (p.p29 * (eq23_e1444_d_n14 * ddt_scale));
        let eq23_value: f64 = eq23_e1446;
        let eq23_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq23_node_derivatives: [f64; 14] = [eq23_e1446_d_n0, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14];
        let eq23_branch_derivative_indices: [usize; 0] = [];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivative_indices,
            &eq23_node_derivatives,
            &eq23_branch_derivative_indices,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq24_e1449: f64 = (-var_devsign);
        let eq24_e1451: f64 = (eq24_e1449 * var_qovb);
        let eq24_e1451_d_n0: f64 = (eq24_e1449 * var_qovb_dn0);
        let eq24_e1451_d_n2: f64 = (eq24_e1449 * var_qovb_dn2);
        let eq24_e1451_d_n3: f64 = (eq24_e1449 * var_qovb_dn3);
        let eq24_e1451_d_n4: f64 = (eq24_e1449 * var_qovb_dn4);
        let eq24_e1451_d_n5: f64 = (eq24_e1449 * var_qovb_dn5);
        let eq24_e1451_d_n6: f64 = (eq24_e1449 * var_qovb_dn6);
        let eq24_e1451_d_n7: f64 = (eq24_e1449 * var_qovb_dn7);
        let eq24_e1451_d_n8: f64 = (eq24_e1449 * var_qovb_dn8);
        let eq24_e1451_d_n9: f64 = (eq24_e1449 * var_qovb_dn9);
        let eq24_e1451_d_n10: f64 = (eq24_e1449 * var_qovb_dn10);
        let eq24_e1451_d_n11: f64 = (eq24_e1449 * var_qovb_dn11);
        let eq24_e1451_d_n12: f64 = (eq24_e1449 * var_qovb_dn12);
        let eq24_e1451_d_n13: f64 = (eq24_e1449 * var_qovb_dn13);
        let eq24_e1451_d_n14: f64 = (eq24_e1449 * var_qovb_dn14);
        let eq24_e1452: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq24_e1451);
        let eq24_e1453: f64 = (p.p29 * eq24_e1452);
        let eq24_e1453_d_n0: f64 = (p.p29 * (eq24_e1451_d_n0 * ddt_scale));
        let eq24_e1453_d_n2: f64 = (p.p29 * (eq24_e1451_d_n2 * ddt_scale));
        let eq24_e1453_d_n3: f64 = (p.p29 * (eq24_e1451_d_n3 * ddt_scale));
        let eq24_e1453_d_n4: f64 = (p.p29 * (eq24_e1451_d_n4 * ddt_scale));
        let eq24_e1453_d_n5: f64 = (p.p29 * (eq24_e1451_d_n5 * ddt_scale));
        let eq24_e1453_d_n6: f64 = (p.p29 * (eq24_e1451_d_n6 * ddt_scale));
        let eq24_e1453_d_n7: f64 = (p.p29 * (eq24_e1451_d_n7 * ddt_scale));
        let eq24_e1453_d_n8: f64 = (p.p29 * (eq24_e1451_d_n8 * ddt_scale));
        let eq24_e1453_d_n9: f64 = (p.p29 * (eq24_e1451_d_n9 * ddt_scale));
        let eq24_e1453_d_n10: f64 = (p.p29 * (eq24_e1451_d_n10 * ddt_scale));
        let eq24_e1453_d_n11: f64 = (p.p29 * (eq24_e1451_d_n11 * ddt_scale));
        let eq24_e1453_d_n12: f64 = (p.p29 * (eq24_e1451_d_n12 * ddt_scale));
        let eq24_e1453_d_n13: f64 = (p.p29 * (eq24_e1451_d_n13 * ddt_scale));
        let eq24_e1453_d_n14: f64 = (p.p29 * (eq24_e1451_d_n14 * ddt_scale));
        let eq24_value: f64 = eq24_e1453;
        let eq24_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq24_node_derivatives: [f64; 14] = [eq24_e1453_d_n0, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(11),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq25_e1456: f64 = (var_devsign * p.p28);
        let eq25_e1458: f64 = (eq25_e1456 * var_sigvds);
        let eq25_e1460: f64 = (eq25_e1458 * var_ids);
        let eq25_e1460_d_n0: f64 = (eq25_e1458 * var_ids_dn0);
        let eq25_e1460_d_n2: f64 = (eq25_e1458 * var_ids_dn2);
        let eq25_e1460_d_n3: f64 = (eq25_e1458 * var_ids_dn3);
        let eq25_e1460_d_n4: f64 = (eq25_e1458 * var_ids_dn4);
        let eq25_e1460_d_n5: f64 = (eq25_e1458 * var_ids_dn5);
        let eq25_e1460_d_n6: f64 = (eq25_e1458 * var_ids_dn6);
        let eq25_e1460_d_n7: f64 = (eq25_e1458 * var_ids_dn7);
        let eq25_e1460_d_n8: f64 = (eq25_e1458 * var_ids_dn8);
        let eq25_e1460_d_n9: f64 = (eq25_e1458 * var_ids_dn9);
        let eq25_e1460_d_n10: f64 = (eq25_e1458 * var_ids_dn10);
        let eq25_e1460_d_n11: f64 = (eq25_e1458 * var_ids_dn11);
        let eq25_e1460_d_n12: f64 = (eq25_e1458 * var_ids_dn12);
        let eq25_e1460_d_n13: f64 = (eq25_e1458 * var_ids_dn13);
        let eq25_e1460_d_n14: f64 = (eq25_e1458 * var_ids_dn14);
        let eq25_value: f64 = eq25_e1460;
        let eq25_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq25_node_derivatives: [f64; 14] = [eq25_e1460_d_n0, eq25_e1460_d_n2, eq25_e1460_d_n3, eq25_e1460_d_n4, eq25_e1460_d_n5, eq25_e1460_d_n6, eq25_e1460_d_n7, eq25_e1460_d_n8, eq25_e1460_d_n9, eq25_e1460_d_n10, eq25_e1460_d_n11, eq25_e1460_d_n12, eq25_e1460_d_n13, eq25_e1460_d_n14];
        let eq25_branch_derivative_indices: [usize; 0] = [];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq25_value),
            &eq25_node_derivative_indices,
            &eq25_node_derivatives,
            &eq25_branch_derivative_indices,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1464, eq26_e1464_d_n0, eq26_e1464_d_n2, eq26_e1464_d_n3, eq26_e1464_d_n4, eq26_e1464_d_n5, eq26_e1464_d_n6, eq26_e1464_d_n7, eq26_e1464_d_n8, eq26_e1464_d_n9, eq26_e1464_d_n10, eq26_e1464_d_n11, eq26_e1464_d_n12, eq26_e1464_d_n13, eq26_e1464_d_n14,) = {
    if (var_guard751 != 0.0) {
        (var_igb_1, var_igb_1_dn0, var_igb_1_dn2, var_igb_1_dn3, var_igb_1_dn4, var_igb_1_dn5, var_igb_1_dn6, var_igb_1_dn7, var_igb_1_dn8, var_igb_1_dn9, var_igb_1_dn10, var_igb_1_dn11, var_igb_1_dn12, var_igb_1_dn13, var_igb_1_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1464;
        let eq26_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq26_node_derivatives: [f64; 14] = [eq26_e1464_d_n0, eq26_e1464_d_n2, eq26_e1464_d_n3, eq26_e1464_d_n4, eq26_e1464_d_n5, eq26_e1464_d_n6, eq26_e1464_d_n7, eq26_e1464_d_n8, eq26_e1464_d_n9, eq26_e1464_d_n10, eq26_e1464_d_n11, eq26_e1464_d_n12, eq26_e1464_d_n13, eq26_e1464_d_n14];
        let eq26_branch_derivative_indices: [usize; 0] = [];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(11),
            multiplicity * (eq26_value),
            &eq26_node_derivative_indices,
            &eq26_node_derivatives,
            &eq26_branch_derivative_indices,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1470, eq27_e1470_d_n0, eq27_e1470_d_n2, eq27_e1470_d_n3, eq27_e1470_d_n4, eq27_e1470_d_n5, eq27_e1470_d_n6, eq27_e1470_d_n7, eq27_e1470_d_n8, eq27_e1470_d_n9, eq27_e1470_d_n10, eq27_e1470_d_n11, eq27_e1470_d_n12, eq27_e1470_d_n13, eq27_e1470_d_n14,) = {
    if (var_guard752 != 0.0) {
        let eq27_e1468: f64 = (var_igs_1 + var_igcs_1);
        let eq27_e1468_d_n0: f64 = (var_igs_1_dn0 + var_igcs_1_dn0);
        let eq27_e1468_d_n2: f64 = (var_igs_1_dn2 + var_igcs_1_dn2);
        let eq27_e1468_d_n3: f64 = (var_igs_1_dn3 + var_igcs_1_dn3);
        let eq27_e1468_d_n4: f64 = (var_igs_1_dn4 + var_igcs_1_dn4);
        let eq27_e1468_d_n5: f64 = (var_igs_1_dn5 + var_igcs_1_dn5);
        let eq27_e1468_d_n6: f64 = (var_igs_1_dn6 + var_igcs_1_dn6);
        let eq27_e1468_d_n7: f64 = (var_igs_1_dn7 + var_igcs_1_dn7);
        let eq27_e1468_d_n8: f64 = (var_igs_1_dn8 + var_igcs_1_dn8);
        let eq27_e1468_d_n9: f64 = (var_igs_1_dn9 + var_igcs_1_dn9);
        let eq27_e1468_d_n10: f64 = (var_igs_1_dn10 + var_igcs_1_dn10);
        let eq27_e1468_d_n11: f64 = (var_igs_1_dn11 + var_igcs_1_dn11);
        let eq27_e1468_d_n12: f64 = (var_igs_1_dn12 + var_igcs_1_dn12);
        let eq27_e1468_d_n13: f64 = (var_igs_1_dn13 + var_igcs_1_dn13);
        let eq27_e1468_d_n14: f64 = (var_igs_1_dn14 + var_igcs_1_dn14);
        (eq27_e1468, eq27_e1468_d_n0, eq27_e1468_d_n2, eq27_e1468_d_n3, eq27_e1468_d_n4, eq27_e1468_d_n5, eq27_e1468_d_n6, eq27_e1468_d_n7, eq27_e1468_d_n8, eq27_e1468_d_n9, eq27_e1468_d_n10, eq27_e1468_d_n11, eq27_e1468_d_n12, eq27_e1468_d_n13, eq27_e1468_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1470;
        let eq27_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq27_node_derivatives: [f64; 14] = [eq27_e1470_d_n0, eq27_e1470_d_n2, eq27_e1470_d_n3, eq27_e1470_d_n4, eq27_e1470_d_n5, eq27_e1470_d_n6, eq27_e1470_d_n7, eq27_e1470_d_n8, eq27_e1470_d_n9, eq27_e1470_d_n10, eq27_e1470_d_n11, eq27_e1470_d_n12, eq27_e1470_d_n13, eq27_e1470_d_n14];
        let eq27_branch_derivative_indices: [usize; 0] = [];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivative_indices,
            &eq27_node_derivatives,
            &eq27_branch_derivative_indices,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1476, eq28_e1476_d_n0, eq28_e1476_d_n2, eq28_e1476_d_n3, eq28_e1476_d_n4, eq28_e1476_d_n5, eq28_e1476_d_n6, eq28_e1476_d_n7, eq28_e1476_d_n8, eq28_e1476_d_n9, eq28_e1476_d_n10, eq28_e1476_d_n11, eq28_e1476_d_n12, eq28_e1476_d_n13, eq28_e1476_d_n14,) = {
    if (var_guard752 != 0.0) {
        let eq28_e1474: f64 = (var_igd_1 + var_igcd_1);
        let eq28_e1474_d_n0: f64 = (var_igd_1_dn0 + var_igcd_1_dn0);
        let eq28_e1474_d_n2: f64 = (var_igd_1_dn2 + var_igcd_1_dn2);
        let eq28_e1474_d_n3: f64 = (var_igd_1_dn3 + var_igcd_1_dn3);
        let eq28_e1474_d_n4: f64 = (var_igd_1_dn4 + var_igcd_1_dn4);
        let eq28_e1474_d_n5: f64 = (var_igd_1_dn5 + var_igcd_1_dn5);
        let eq28_e1474_d_n6: f64 = (var_igd_1_dn6 + var_igcd_1_dn6);
        let eq28_e1474_d_n7: f64 = (var_igd_1_dn7 + var_igcd_1_dn7);
        let eq28_e1474_d_n8: f64 = (var_igd_1_dn8 + var_igcd_1_dn8);
        let eq28_e1474_d_n9: f64 = (var_igd_1_dn9 + var_igcd_1_dn9);
        let eq28_e1474_d_n10: f64 = (var_igd_1_dn10 + var_igcd_1_dn10);
        let eq28_e1474_d_n11: f64 = (var_igd_1_dn11 + var_igcd_1_dn11);
        let eq28_e1474_d_n12: f64 = (var_igd_1_dn12 + var_igcd_1_dn12);
        let eq28_e1474_d_n13: f64 = (var_igd_1_dn13 + var_igcd_1_dn13);
        let eq28_e1474_d_n14: f64 = (var_igd_1_dn14 + var_igcd_1_dn14);
        (eq28_e1474, eq28_e1474_d_n0, eq28_e1474_d_n2, eq28_e1474_d_n3, eq28_e1474_d_n4, eq28_e1474_d_n5, eq28_e1474_d_n6, eq28_e1474_d_n7, eq28_e1474_d_n8, eq28_e1474_d_n9, eq28_e1474_d_n10, eq28_e1474_d_n11, eq28_e1474_d_n12, eq28_e1474_d_n13, eq28_e1474_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1476;
        let eq28_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq28_node_derivatives: [f64; 14] = [eq28_e1476_d_n0, eq28_e1476_d_n2, eq28_e1476_d_n3, eq28_e1476_d_n4, eq28_e1476_d_n5, eq28_e1476_d_n6, eq28_e1476_d_n7, eq28_e1476_d_n8, eq28_e1476_d_n9, eq28_e1476_d_n10, eq28_e1476_d_n11, eq28_e1476_d_n12, eq28_e1476_d_n13, eq28_e1476_d_n14];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e1482, eq29_e1482_d_n0, eq29_e1482_d_n2, eq29_e1482_d_n3, eq29_e1482_d_n4, eq29_e1482_d_n5, eq29_e1482_d_n6, eq29_e1482_d_n7, eq29_e1482_d_n8, eq29_e1482_d_n9, eq29_e1482_d_n10, eq29_e1482_d_n11, eq29_e1482_d_n12, eq29_e1482_d_n13, eq29_e1482_d_n14,) = {
    if (var_guard753 != 0.0) {
        let eq29_e1480: f64 = (var_isub + var_igidl_1);
        let eq29_e1480_d_n0: f64 = (var_isub_dn0 + var_igidl_1_dn0);
        let eq29_e1480_d_n2: f64 = (var_isub_dn2 + var_igidl_1_dn2);
        let eq29_e1480_d_n3: f64 = (var_isub_dn3 + var_igidl_1_dn3);
        let eq29_e1480_d_n4: f64 = (var_isub_dn4 + var_igidl_1_dn4);
        let eq29_e1480_d_n5: f64 = (var_isub_dn5 + var_igidl_1_dn5);
        let eq29_e1480_d_n6: f64 = (var_isub_dn6 + var_igidl_1_dn6);
        let eq29_e1480_d_n7: f64 = (var_isub_dn7 + var_igidl_1_dn7);
        let eq29_e1480_d_n8: f64 = (var_isub_dn8 + var_igidl_1_dn8);
        let eq29_e1480_d_n9: f64 = (var_isub_dn9 + var_igidl_1_dn9);
        let eq29_e1480_d_n10: f64 = (var_isub_dn10 + var_igidl_1_dn10);
        let eq29_e1480_d_n11: f64 = (var_isub_dn11 + var_igidl_1_dn11);
        let eq29_e1480_d_n12: f64 = (var_isub_dn12 + var_igidl_1_dn12);
        let eq29_e1480_d_n13: f64 = (var_isub_dn13 + var_igidl_1_dn13);
        let eq29_e1480_d_n14: f64 = (var_isub_dn14 + var_igidl_1_dn14);
        (eq29_e1480, eq29_e1480_d_n0, eq29_e1480_d_n2, eq29_e1480_d_n3, eq29_e1480_d_n4, eq29_e1480_d_n5, eq29_e1480_d_n6, eq29_e1480_d_n7, eq29_e1480_d_n8, eq29_e1480_d_n9, eq29_e1480_d_n10, eq29_e1480_d_n11, eq29_e1480_d_n12, eq29_e1480_d_n13, eq29_e1480_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e1482;
        let eq29_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq29_node_derivatives: [f64; 14] = [eq29_e1482_d_n0, eq29_e1482_d_n2, eq29_e1482_d_n3, eq29_e1482_d_n4, eq29_e1482_d_n5, eq29_e1482_d_n6, eq29_e1482_d_n7, eq29_e1482_d_n8, eq29_e1482_d_n9, eq29_e1482_d_n10, eq29_e1482_d_n11, eq29_e1482_d_n12, eq29_e1482_d_n13, eq29_e1482_d_n14];
        let eq29_branch_derivative_indices: [usize; 0] = [];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_cth: f64,
        var_deltemp1: f64,
        var_deltemp1_dn4: f64,
        var_devsign: f64,
        var_gcrg: f64,
        var_gcrg_dn0: f64,
        var_gcrg_dn10: f64,
        var_gcrg_dn11: f64,
        var_gcrg_dn12: f64,
        var_gcrg_dn13: f64,
        var_gcrg_dn14: f64,
        var_gcrg_dn2: f64,
        var_gcrg_dn3: f64,
        var_gcrg_dn4: f64,
        var_gcrg_dn5: f64,
        var_gcrg_dn6: f64,
        var_gcrg_dn7: f64,
        var_gcrg_dn8: f64,
        var_gcrg_dn9: f64,
        var_gdpr: f64,
        var_gdpr_dn0: f64,
        var_gdpr_dn10: f64,
        var_gdpr_dn11: f64,
        var_gdpr_dn12: f64,
        var_gdpr_dn13: f64,
        var_gdpr_dn14: f64,
        var_gdpr_dn2: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gdpr_dn9: f64,
        var_gdrift_d: f64,
        var_gdrift_d_dn0: f64,
        var_gdrift_d_dn10: f64,
        var_gdrift_d_dn11: f64,
        var_gdrift_d_dn12: f64,
        var_gdrift_d_dn13: f64,
        var_gdrift_d_dn14: f64,
        var_gdrift_d_dn2: f64,
        var_gdrift_d_dn3: f64,
        var_gdrift_d_dn4: f64,
        var_gdrift_d_dn5: f64,
        var_gdrift_d_dn6: f64,
        var_gdrift_d_dn7: f64,
        var_gdrift_d_dn8: f64,
        var_gdrift_d_dn9: f64,
        var_gdrift_s: f64,
        var_gdrift_s_dn0: f64,
        var_gdrift_s_dn10: f64,
        var_gdrift_s_dn11: f64,
        var_gdrift_s_dn12: f64,
        var_gdrift_s_dn13: f64,
        var_gdrift_s_dn14: f64,
        var_gdrift_s_dn2: f64,
        var_gdrift_s_dn3: f64,
        var_gdrift_s_dn4: f64,
        var_gdrift_s_dn5: f64,
        var_gdrift_s_dn6: f64,
        var_gdrift_s_dn7: f64,
        var_gdrift_s_dn8: f64,
        var_gdrift_s_dn9: f64,
        var_ggate: f64,
        var_ggate_dn0: f64,
        var_ggate_dn10: f64,
        var_ggate_dn11: f64,
        var_ggate_dn12: f64,
        var_ggate_dn13: f64,
        var_ggate_dn14: f64,
        var_ggate_dn2: f64,
        var_ggate_dn3: f64,
        var_ggate_dn4: f64,
        var_ggate_dn5: f64,
        var_ggate_dn6: f64,
        var_ggate_dn7: f64,
        var_ggate_dn8: f64,
        var_ggate_dn9: f64,
        var_gmin: f64,
        var_gspr: f64,
        var_gspr_dn0: f64,
        var_gspr_dn10: f64,
        var_gspr_dn11: f64,
        var_gspr_dn12: f64,
        var_gspr_dn13: f64,
        var_gspr_dn14: f64,
        var_gspr_dn2: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gspr_dn9: f64,
        var_gth: f64,
        var_guard753: f64,
        var_guard754: f64,
        var_guard755: f64,
        var_guard756: f64,
        var_guard757: f64,
        var_guard758: f64,
        var_guard762: f64,
        var_guard763: f64,
        var_guard769: f64,
        var_ibs: f64,
        var_ibs_dn0: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn12: f64,
        var_ibs_dn13: f64,
        var_ibs_dn14: f64,
        var_ibs_dn2: f64,
        var_ibs_dn3: f64,
        var_ibs_dn4: f64,
        var_ibs_dn5: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_ibs_dn8: f64,
        var_ibs_dn9: f64,
        var_igidl_1: f64,
        var_igidl_1_dn0: f64,
        var_igidl_1_dn10: f64,
        var_igidl_1_dn11: f64,
        var_igidl_1_dn12: f64,
        var_igidl_1_dn13: f64,
        var_igidl_1_dn14: f64,
        var_igidl_1_dn2: f64,
        var_igidl_1_dn3: f64,
        var_igidl_1_dn4: f64,
        var_igidl_1_dn5: f64,
        var_igidl_1_dn6: f64,
        var_igidl_1_dn7: f64,
        var_igidl_1_dn8: f64,
        var_igidl_1_dn9: f64,
        var_igisl_1: f64,
        var_igisl_1_dn0: f64,
        var_igisl_1_dn10: f64,
        var_igisl_1_dn11: f64,
        var_igisl_1_dn12: f64,
        var_igisl_1_dn13: f64,
        var_igisl_1_dn14: f64,
        var_igisl_1_dn2: f64,
        var_igisl_1_dn3: f64,
        var_igisl_1_dn4: f64,
        var_igisl_1_dn5: f64,
        var_igisl_1_dn6: f64,
        var_igisl_1_dn7: f64,
        var_igisl_1_dn8: f64,
        var_igisl_1_dn9: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn13: f64,
        var_isub_dn14: f64,
        var_isub_dn2: f64,
        var_isub_dn3: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_isubdr: f64,
        var_isubdr_dn0: f64,
        var_isubdr_dn10: f64,
        var_isubdr_dn11: f64,
        var_isubdr_dn12: f64,
        var_isubdr_dn13: f64,
        var_isubdr_dn14: f64,
        var_isubdr_dn2: f64,
        var_isubdr_dn3: f64,
        var_isubdr_dn4: f64,
        var_isubdr_dn5: f64,
        var_isubdr_dn6: f64,
        var_isubdr_dn7: f64,
        var_isubdr_dn8: f64,
        var_isubdr_dn9: f64,
        var_pdiss: f64,
        var_pdiss_dn0: f64,
        var_pdiss_dn10: f64,
        var_pdiss_dn11: f64,
        var_pdiss_dn12: f64,
        var_pdiss_dn13: f64,
        var_pdiss_dn14: f64,
        var_pdiss_dn2: f64,
        var_pdiss_dn3: f64,
        var_pdiss_dn4: f64,
        var_pdiss_dn5: f64,
        var_pdiss_dn6: f64,
        var_pdiss_dn7: f64,
        var_pdiss_dn8: f64,
        var_pdiss_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq30_e1490, eq30_e1490_d_n0, eq30_e1490_d_n2, eq30_e1490_d_n3, eq30_e1490_d_n4, eq30_e1490_d_n5, eq30_e1490_d_n6, eq30_e1490_d_n7, eq30_e1490_d_n8, eq30_e1490_d_n9, eq30_e1490_d_n10, eq30_e1490_d_n11, eq30_e1490_d_n12, eq30_e1490_d_n13, eq30_e1490_d_n14,) = {
    if (var_guard753 != 0.0) {
        let eq30_e1486: f64 = (p.p28 * var_devsign);
        let eq30_e1488: f64 = (eq30_e1486 * var_isubdr);
        let eq30_e1488_d_n0: f64 = (eq30_e1486 * var_isubdr_dn0);
        let eq30_e1488_d_n2: f64 = (eq30_e1486 * var_isubdr_dn2);
        let eq30_e1488_d_n3: f64 = (eq30_e1486 * var_isubdr_dn3);
        let eq30_e1488_d_n4: f64 = (eq30_e1486 * var_isubdr_dn4);
        let eq30_e1488_d_n5: f64 = (eq30_e1486 * var_isubdr_dn5);
        let eq30_e1488_d_n6: f64 = (eq30_e1486 * var_isubdr_dn6);
        let eq30_e1488_d_n7: f64 = (eq30_e1486 * var_isubdr_dn7);
        let eq30_e1488_d_n8: f64 = (eq30_e1486 * var_isubdr_dn8);
        let eq30_e1488_d_n9: f64 = (eq30_e1486 * var_isubdr_dn9);
        let eq30_e1488_d_n10: f64 = (eq30_e1486 * var_isubdr_dn10);
        let eq30_e1488_d_n11: f64 = (eq30_e1486 * var_isubdr_dn11);
        let eq30_e1488_d_n12: f64 = (eq30_e1486 * var_isubdr_dn12);
        let eq30_e1488_d_n13: f64 = (eq30_e1486 * var_isubdr_dn13);
        let eq30_e1488_d_n14: f64 = (eq30_e1486 * var_isubdr_dn14);
        (eq30_e1488, eq30_e1488_d_n0, eq30_e1488_d_n2, eq30_e1488_d_n3, eq30_e1488_d_n4, eq30_e1488_d_n5, eq30_e1488_d_n6, eq30_e1488_d_n7, eq30_e1488_d_n8, eq30_e1488_d_n9, eq30_e1488_d_n10, eq30_e1488_d_n11, eq30_e1488_d_n12, eq30_e1488_d_n13, eq30_e1488_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1490;
        let eq30_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq30_node_derivatives: [f64; 14] = [eq30_e1490_d_n0, eq30_e1490_d_n2, eq30_e1490_d_n3, eq30_e1490_d_n4, eq30_e1490_d_n5, eq30_e1490_d_n6, eq30_e1490_d_n7, eq30_e1490_d_n8, eq30_e1490_d_n9, eq30_e1490_d_n10, eq30_e1490_d_n11, eq30_e1490_d_n12, eq30_e1490_d_n13, eq30_e1490_d_n14];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e1494, eq31_e1494_d_n0, eq31_e1494_d_n2, eq31_e1494_d_n3, eq31_e1494_d_n4, eq31_e1494_d_n5, eq31_e1494_d_n6, eq31_e1494_d_n7, eq31_e1494_d_n8, eq31_e1494_d_n9, eq31_e1494_d_n10, eq31_e1494_d_n11, eq31_e1494_d_n12, eq31_e1494_d_n13, eq31_e1494_d_n14,) = {
    if (var_guard753 != 0.0) {
        (var_igisl_1, var_igisl_1_dn0, var_igisl_1_dn2, var_igisl_1_dn3, var_igisl_1_dn4, var_igisl_1_dn5, var_igisl_1_dn6, var_igisl_1_dn7, var_igisl_1_dn8, var_igisl_1_dn9, var_igisl_1_dn10, var_igisl_1_dn11, var_igisl_1_dn12, var_igisl_1_dn13, var_igisl_1_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e1494;
        let eq31_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq31_node_derivatives: [f64; 14] = [eq31_e1494_d_n0, eq31_e1494_d_n2, eq31_e1494_d_n3, eq31_e1494_d_n4, eq31_e1494_d_n5, eq31_e1494_d_n6, eq31_e1494_d_n7, eq31_e1494_d_n8, eq31_e1494_d_n9, eq31_e1494_d_n10, eq31_e1494_d_n11, eq31_e1494_d_n12, eq31_e1494_d_n13, eq31_e1494_d_n14];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e1499, eq32_e1499_d_n0, eq32_e1499_d_n2, eq32_e1499_d_n3, eq32_e1499_d_n4, eq32_e1499_d_n5, eq32_e1499_d_n6, eq32_e1499_d_n7, eq32_e1499_d_n8, eq32_e1499_d_n9, eq32_e1499_d_n10, eq32_e1499_d_n11, eq32_e1499_d_n12, eq32_e1499_d_n13, eq32_e1499_d_n14,) = {
    if (var_guard753 == 0.0) {
        (var_igidl_1, var_igidl_1_dn0, var_igidl_1_dn2, var_igidl_1_dn3, var_igidl_1_dn4, var_igidl_1_dn5, var_igidl_1_dn6, var_igidl_1_dn7, var_igidl_1_dn8, var_igidl_1_dn9, var_igidl_1_dn10, var_igidl_1_dn11, var_igidl_1_dn12, var_igidl_1_dn13, var_igidl_1_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e1499;
        let eq32_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq32_node_derivatives: [f64; 14] = [eq32_e1499_d_n0, eq32_e1499_d_n2, eq32_e1499_d_n3, eq32_e1499_d_n4, eq32_e1499_d_n5, eq32_e1499_d_n6, eq32_e1499_d_n7, eq32_e1499_d_n8, eq32_e1499_d_n9, eq32_e1499_d_n10, eq32_e1499_d_n11, eq32_e1499_d_n12, eq32_e1499_d_n13, eq32_e1499_d_n14];
        let eq32_branch_derivative_indices: [usize; 0] = [];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e1506, eq33_e1506_d_n0, eq33_e1506_d_n2, eq33_e1506_d_n3, eq33_e1506_d_n4, eq33_e1506_d_n5, eq33_e1506_d_n6, eq33_e1506_d_n7, eq33_e1506_d_n8, eq33_e1506_d_n9, eq33_e1506_d_n10, eq33_e1506_d_n11, eq33_e1506_d_n12, eq33_e1506_d_n13, eq33_e1506_d_n14,) = {
    if (var_guard753 == 0.0) {
        let eq33_e1504: f64 = (var_isub + var_igisl_1);
        let eq33_e1504_d_n0: f64 = (var_isub_dn0 + var_igisl_1_dn0);
        let eq33_e1504_d_n2: f64 = (var_isub_dn2 + var_igisl_1_dn2);
        let eq33_e1504_d_n3: f64 = (var_isub_dn3 + var_igisl_1_dn3);
        let eq33_e1504_d_n4: f64 = (var_isub_dn4 + var_igisl_1_dn4);
        let eq33_e1504_d_n5: f64 = (var_isub_dn5 + var_igisl_1_dn5);
        let eq33_e1504_d_n6: f64 = (var_isub_dn6 + var_igisl_1_dn6);
        let eq33_e1504_d_n7: f64 = (var_isub_dn7 + var_igisl_1_dn7);
        let eq33_e1504_d_n8: f64 = (var_isub_dn8 + var_igisl_1_dn8);
        let eq33_e1504_d_n9: f64 = (var_isub_dn9 + var_igisl_1_dn9);
        let eq33_e1504_d_n10: f64 = (var_isub_dn10 + var_igisl_1_dn10);
        let eq33_e1504_d_n11: f64 = (var_isub_dn11 + var_igisl_1_dn11);
        let eq33_e1504_d_n12: f64 = (var_isub_dn12 + var_igisl_1_dn12);
        let eq33_e1504_d_n13: f64 = (var_isub_dn13 + var_igisl_1_dn13);
        let eq33_e1504_d_n14: f64 = (var_isub_dn14 + var_igisl_1_dn14);
        (eq33_e1504, eq33_e1504_d_n0, eq33_e1504_d_n2, eq33_e1504_d_n3, eq33_e1504_d_n4, eq33_e1504_d_n5, eq33_e1504_d_n6, eq33_e1504_d_n7, eq33_e1504_d_n8, eq33_e1504_d_n9, eq33_e1504_d_n10, eq33_e1504_d_n11, eq33_e1504_d_n12, eq33_e1504_d_n13, eq33_e1504_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1506;
        let eq33_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq33_node_derivatives: [f64; 14] = [eq33_e1506_d_n0, eq33_e1506_d_n2, eq33_e1506_d_n3, eq33_e1506_d_n4, eq33_e1506_d_n5, eq33_e1506_d_n6, eq33_e1506_d_n7, eq33_e1506_d_n8, eq33_e1506_d_n9, eq33_e1506_d_n10, eq33_e1506_d_n11, eq33_e1506_d_n12, eq33_e1506_d_n13, eq33_e1506_d_n14];
        let eq33_branch_derivative_indices: [usize; 0] = [];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq33_value),
            &eq33_node_derivative_indices,
            &eq33_node_derivatives,
            &eq33_branch_derivative_indices,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e1514, eq34_e1514_d_n0, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14,) = {
    if (var_guard754 != 0.0) {
        let eq34_e1510: f64 = (p.p28 * (nv0 - nv6));
        let eq34_e1512: f64 = (eq34_e1510 * var_gdpr);
        let eq34_e1512_d_n0: f64 = ((p.p28 * var_gdpr) + (eq34_e1510 * var_gdpr_dn0));
        let eq34_e1512_d_n2: f64 = (eq34_e1510 * var_gdpr_dn2);
        let eq34_e1512_d_n3: f64 = (eq34_e1510 * var_gdpr_dn3);
        let eq34_e1512_d_n4: f64 = (eq34_e1510 * var_gdpr_dn4);
        let eq34_e1512_d_n5: f64 = (eq34_e1510 * var_gdpr_dn5);
        let eq34_e1512_d_n6: f64 = (((-p.p28) * var_gdpr) + (eq34_e1510 * var_gdpr_dn6));
        let eq34_e1512_d_n7: f64 = (eq34_e1510 * var_gdpr_dn7);
        let eq34_e1512_d_n8: f64 = (eq34_e1510 * var_gdpr_dn8);
        let eq34_e1512_d_n9: f64 = (eq34_e1510 * var_gdpr_dn9);
        let eq34_e1512_d_n10: f64 = (eq34_e1510 * var_gdpr_dn10);
        let eq34_e1512_d_n11: f64 = (eq34_e1510 * var_gdpr_dn11);
        let eq34_e1512_d_n12: f64 = (eq34_e1510 * var_gdpr_dn12);
        let eq34_e1512_d_n13: f64 = (eq34_e1510 * var_gdpr_dn13);
        let eq34_e1512_d_n14: f64 = (eq34_e1510 * var_gdpr_dn14);
        (eq34_e1512, eq34_e1512_d_n0, eq34_e1512_d_n2, eq34_e1512_d_n3, eq34_e1512_d_n4, eq34_e1512_d_n5, eq34_e1512_d_n6, eq34_e1512_d_n7, eq34_e1512_d_n8, eq34_e1512_d_n9, eq34_e1512_d_n10, eq34_e1512_d_n11, eq34_e1512_d_n12, eq34_e1512_d_n13, eq34_e1512_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e1514;
        let eq34_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq34_node_derivatives: [f64; 14] = [eq34_e1514_d_n0, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14];
        let eq34_branch_derivative_indices: [usize; 0] = [];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq34_value),
            &eq34_node_derivative_indices,
            &eq34_node_derivatives,
            &eq34_branch_derivative_indices,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq36_e1534, eq36_e1534_d_n0, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14,) = {
    if ((var_guard754 != 0.0) && (var_guard755 != 0.0)) {
        let eq36_e1530: f64 = (p.p28 * (nv6 - nv5));
        let eq36_e1532: f64 = (eq36_e1530 * var_gdrift_d);
        let eq36_e1532_d_n0: f64 = (eq36_e1530 * var_gdrift_d_dn0);
        let eq36_e1532_d_n2: f64 = (eq36_e1530 * var_gdrift_d_dn2);
        let eq36_e1532_d_n3: f64 = (eq36_e1530 * var_gdrift_d_dn3);
        let eq36_e1532_d_n4: f64 = (eq36_e1530 * var_gdrift_d_dn4);
        let eq36_e1532_d_n5: f64 = (((-p.p28) * var_gdrift_d) + (eq36_e1530 * var_gdrift_d_dn5));
        let eq36_e1532_d_n6: f64 = ((p.p28 * var_gdrift_d) + (eq36_e1530 * var_gdrift_d_dn6));
        let eq36_e1532_d_n7: f64 = (eq36_e1530 * var_gdrift_d_dn7);
        let eq36_e1532_d_n8: f64 = (eq36_e1530 * var_gdrift_d_dn8);
        let eq36_e1532_d_n9: f64 = (eq36_e1530 * var_gdrift_d_dn9);
        let eq36_e1532_d_n10: f64 = (eq36_e1530 * var_gdrift_d_dn10);
        let eq36_e1532_d_n11: f64 = (eq36_e1530 * var_gdrift_d_dn11);
        let eq36_e1532_d_n12: f64 = (eq36_e1530 * var_gdrift_d_dn12);
        let eq36_e1532_d_n13: f64 = (eq36_e1530 * var_gdrift_d_dn13);
        let eq36_e1532_d_n14: f64 = (eq36_e1530 * var_gdrift_d_dn14);
        (eq36_e1532, eq36_e1532_d_n0, eq36_e1532_d_n2, eq36_e1532_d_n3, eq36_e1532_d_n4, eq36_e1532_d_n5, eq36_e1532_d_n6, eq36_e1532_d_n7, eq36_e1532_d_n8, eq36_e1532_d_n9, eq36_e1532_d_n10, eq36_e1532_d_n11, eq36_e1532_d_n12, eq36_e1532_d_n13, eq36_e1532_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1534;
        let eq36_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq36_node_derivatives: [f64; 14] = [eq36_e1534_d_n0, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14];
        let eq36_branch_derivative_indices: [usize; 0] = [];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq36_value),
            &eq36_node_derivative_indices,
            &eq36_node_derivatives,
            &eq36_branch_derivative_indices,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1572,) = {
    if ((var_guard754 != 0.0) && (var_guard755 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq39_value: f64 = eq39_e1572;
        stamper.stamp_potential_const_local(
            1,
            eq39_value,
        );
        let (eq40_e1577,) = {
    if (var_guard754 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e1577;
        stamper.stamp_potential_const_local(
            2,
            eq40_value,
        );
        let (eq41_e1582,) = {
    if (var_guard754 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e1582;
        stamper.stamp_potential_const_local(
            3,
            eq41_value,
        );
        let (eq42_e1590, eq42_e1590_d_n0, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14,) = {
    if (var_guard756 != 0.0) {
        let eq42_e1586: f64 = (p.p28 * (nv2 - nv8));
        let eq42_e1588: f64 = (eq42_e1586 * var_gspr);
        let eq42_e1588_d_n0: f64 = (eq42_e1586 * var_gspr_dn0);
        let eq42_e1588_d_n2: f64 = ((p.p28 * var_gspr) + (eq42_e1586 * var_gspr_dn2));
        let eq42_e1588_d_n3: f64 = (eq42_e1586 * var_gspr_dn3);
        let eq42_e1588_d_n4: f64 = (eq42_e1586 * var_gspr_dn4);
        let eq42_e1588_d_n5: f64 = (eq42_e1586 * var_gspr_dn5);
        let eq42_e1588_d_n6: f64 = (eq42_e1586 * var_gspr_dn6);
        let eq42_e1588_d_n7: f64 = (eq42_e1586 * var_gspr_dn7);
        let eq42_e1588_d_n8: f64 = (((-p.p28) * var_gspr) + (eq42_e1586 * var_gspr_dn8));
        let eq42_e1588_d_n9: f64 = (eq42_e1586 * var_gspr_dn9);
        let eq42_e1588_d_n10: f64 = (eq42_e1586 * var_gspr_dn10);
        let eq42_e1588_d_n11: f64 = (eq42_e1586 * var_gspr_dn11);
        let eq42_e1588_d_n12: f64 = (eq42_e1586 * var_gspr_dn12);
        let eq42_e1588_d_n13: f64 = (eq42_e1586 * var_gspr_dn13);
        let eq42_e1588_d_n14: f64 = (eq42_e1586 * var_gspr_dn14);
        (eq42_e1588, eq42_e1588_d_n0, eq42_e1588_d_n2, eq42_e1588_d_n3, eq42_e1588_d_n4, eq42_e1588_d_n5, eq42_e1588_d_n6, eq42_e1588_d_n7, eq42_e1588_d_n8, eq42_e1588_d_n9, eq42_e1588_d_n10, eq42_e1588_d_n11, eq42_e1588_d_n12, eq42_e1588_d_n13, eq42_e1588_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e1590;
        let eq42_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq42_node_derivatives: [f64; 14] = [eq42_e1590_d_n0, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq44_e1610, eq44_e1610_d_n0, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14,) = {
    if ((var_guard756 != 0.0) && (var_guard757 != 0.0)) {
        let eq44_e1606: f64 = (p.p28 * (nv8 - nv7));
        let eq44_e1608: f64 = (eq44_e1606 * var_gdrift_s);
        let eq44_e1608_d_n0: f64 = (eq44_e1606 * var_gdrift_s_dn0);
        let eq44_e1608_d_n2: f64 = (eq44_e1606 * var_gdrift_s_dn2);
        let eq44_e1608_d_n3: f64 = (eq44_e1606 * var_gdrift_s_dn3);
        let eq44_e1608_d_n4: f64 = (eq44_e1606 * var_gdrift_s_dn4);
        let eq44_e1608_d_n5: f64 = (eq44_e1606 * var_gdrift_s_dn5);
        let eq44_e1608_d_n6: f64 = (eq44_e1606 * var_gdrift_s_dn6);
        let eq44_e1608_d_n7: f64 = (((-p.p28) * var_gdrift_s) + (eq44_e1606 * var_gdrift_s_dn7));
        let eq44_e1608_d_n8: f64 = ((p.p28 * var_gdrift_s) + (eq44_e1606 * var_gdrift_s_dn8));
        let eq44_e1608_d_n9: f64 = (eq44_e1606 * var_gdrift_s_dn9);
        let eq44_e1608_d_n10: f64 = (eq44_e1606 * var_gdrift_s_dn10);
        let eq44_e1608_d_n11: f64 = (eq44_e1606 * var_gdrift_s_dn11);
        let eq44_e1608_d_n12: f64 = (eq44_e1606 * var_gdrift_s_dn12);
        let eq44_e1608_d_n13: f64 = (eq44_e1606 * var_gdrift_s_dn13);
        let eq44_e1608_d_n14: f64 = (eq44_e1606 * var_gdrift_s_dn14);
        (eq44_e1608, eq44_e1608_d_n0, eq44_e1608_d_n2, eq44_e1608_d_n3, eq44_e1608_d_n4, eq44_e1608_d_n5, eq44_e1608_d_n6, eq44_e1608_d_n7, eq44_e1608_d_n8, eq44_e1608_d_n9, eq44_e1608_d_n10, eq44_e1608_d_n11, eq44_e1608_d_n12, eq44_e1608_d_n13, eq44_e1608_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e1610;
        let eq44_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq44_node_derivatives: [f64; 14] = [eq44_e1610_d_n0, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14];
        let eq44_branch_derivative_indices: [usize; 0] = [];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq44_value),
            &eq44_node_derivative_indices,
            &eq44_node_derivatives,
            &eq44_branch_derivative_indices,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq47_e1648,) = {
    if ((var_guard756 != 0.0) && (var_guard757 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e1648;
        stamper.stamp_potential_const_local(
            4,
            eq47_value,
        );
        let (eq48_e1653,) = {
    if (var_guard756 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1653;
        stamper.stamp_potential_const_local(
            5,
            eq48_value,
        );
        let (eq49_e1658,) = {
    if (var_guard756 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e1658;
        stamper.stamp_potential_const_local(
            6,
            eq49_value,
        );
        let (eq51_e1671, eq51_e1671_d_n0, eq51_e1671_d_n1, eq51_e1671_d_n2, eq51_e1671_d_n3, eq51_e1671_d_n4, eq51_e1671_d_n5, eq51_e1671_d_n6, eq51_e1671_d_n7, eq51_e1671_d_n8, eq51_e1671_d_n9, eq51_e1671_d_n10, eq51_e1671_d_n11, eq51_e1671_d_n12, eq51_e1671_d_n13, eq51_e1671_d_n14,) = {
    if (var_guard758 == 0.0) {
        let eq51_e1667: f64 = (p.p28 * (nv1 - nv10));
        let eq51_e1669: f64 = (eq51_e1667 * var_ggate);
        let eq51_e1669_d_n0: f64 = (eq51_e1667 * var_ggate_dn0);
        let eq51_e1669_d_n1: f64 = (p.p28 * var_ggate);
        let eq51_e1669_d_n2: f64 = (eq51_e1667 * var_ggate_dn2);
        let eq51_e1669_d_n3: f64 = (eq51_e1667 * var_ggate_dn3);
        let eq51_e1669_d_n4: f64 = (eq51_e1667 * var_ggate_dn4);
        let eq51_e1669_d_n5: f64 = (eq51_e1667 * var_ggate_dn5);
        let eq51_e1669_d_n6: f64 = (eq51_e1667 * var_ggate_dn6);
        let eq51_e1669_d_n7: f64 = (eq51_e1667 * var_ggate_dn7);
        let eq51_e1669_d_n8: f64 = (eq51_e1667 * var_ggate_dn8);
        let eq51_e1669_d_n9: f64 = (eq51_e1667 * var_ggate_dn9);
        let eq51_e1669_d_n10: f64 = (((-p.p28) * var_ggate) + (eq51_e1667 * var_ggate_dn10));
        let eq51_e1669_d_n11: f64 = (eq51_e1667 * var_ggate_dn11);
        let eq51_e1669_d_n12: f64 = (eq51_e1667 * var_ggate_dn12);
        let eq51_e1669_d_n13: f64 = (eq51_e1667 * var_ggate_dn13);
        let eq51_e1669_d_n14: f64 = (eq51_e1667 * var_ggate_dn14);
        (eq51_e1669, eq51_e1669_d_n0, eq51_e1669_d_n1, eq51_e1669_d_n2, eq51_e1669_d_n3, eq51_e1669_d_n4, eq51_e1669_d_n5, eq51_e1669_d_n6, eq51_e1669_d_n7, eq51_e1669_d_n8, eq51_e1669_d_n9, eq51_e1669_d_n10, eq51_e1669_d_n11, eq51_e1669_d_n12, eq51_e1669_d_n13, eq51_e1669_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1671;
        let eq51_node_derivative_indices: [usize; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq51_node_derivatives: [f64; 15] = [eq51_e1671_d_n0, eq51_e1671_d_n1, eq51_e1671_d_n2, eq51_e1671_d_n3, eq51_e1671_d_n4, eq51_e1671_d_n5, eq51_e1671_d_n6, eq51_e1671_d_n7, eq51_e1671_d_n8, eq51_e1671_d_n9, eq51_e1671_d_n10, eq51_e1671_d_n11, eq51_e1671_d_n12, eq51_e1671_d_n13, eq51_e1671_d_n14];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq51_value),
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1690, eq53_e1690_d_n0, eq53_e1690_d_n2, eq53_e1690_d_n3, eq53_e1690_d_n4, eq53_e1690_d_n5, eq53_e1690_d_n6, eq53_e1690_d_n7, eq53_e1690_d_n8, eq53_e1690_d_n9, eq53_e1690_d_n10, eq53_e1690_d_n11, eq53_e1690_d_n12, eq53_e1690_d_n13, eq53_e1690_d_n14,) = {
    if (var_guard762 != 0.0) {
        let eq53_e1686: f64 = ((nv10 - nv9) * p.p28);
        let eq53_e1688: f64 = (eq53_e1686 * var_gcrg);
        let eq53_e1688_d_n0: f64 = (eq53_e1686 * var_gcrg_dn0);
        let eq53_e1688_d_n2: f64 = (eq53_e1686 * var_gcrg_dn2);
        let eq53_e1688_d_n3: f64 = (eq53_e1686 * var_gcrg_dn3);
        let eq53_e1688_d_n4: f64 = (eq53_e1686 * var_gcrg_dn4);
        let eq53_e1688_d_n5: f64 = (eq53_e1686 * var_gcrg_dn5);
        let eq53_e1688_d_n6: f64 = (eq53_e1686 * var_gcrg_dn6);
        let eq53_e1688_d_n7: f64 = (eq53_e1686 * var_gcrg_dn7);
        let eq53_e1688_d_n8: f64 = (eq53_e1686 * var_gcrg_dn8);
        let eq53_e1688_d_n9: f64 = (((-p.p28) * var_gcrg) + (eq53_e1686 * var_gcrg_dn9));
        let eq53_e1688_d_n10: f64 = ((p.p28 * var_gcrg) + (eq53_e1686 * var_gcrg_dn10));
        let eq53_e1688_d_n11: f64 = (eq53_e1686 * var_gcrg_dn11);
        let eq53_e1688_d_n12: f64 = (eq53_e1686 * var_gcrg_dn12);
        let eq53_e1688_d_n13: f64 = (eq53_e1686 * var_gcrg_dn13);
        let eq53_e1688_d_n14: f64 = (eq53_e1686 * var_gcrg_dn14);
        (eq53_e1688, eq53_e1688_d_n0, eq53_e1688_d_n2, eq53_e1688_d_n3, eq53_e1688_d_n4, eq53_e1688_d_n5, eq53_e1688_d_n6, eq53_e1688_d_n7, eq53_e1688_d_n8, eq53_e1688_d_n9, eq53_e1688_d_n10, eq53_e1688_d_n11, eq53_e1688_d_n12, eq53_e1688_d_n13, eq53_e1688_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1690;
        let eq53_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq53_node_derivatives: [f64; 14] = [eq53_e1690_d_n0, eq53_e1690_d_n2, eq53_e1690_d_n3, eq53_e1690_d_n4, eq53_e1690_d_n5, eq53_e1690_d_n6, eq53_e1690_d_n7, eq53_e1690_d_n8, eq53_e1690_d_n9, eq53_e1690_d_n10, eq53_e1690_d_n11, eq53_e1690_d_n12, eq53_e1690_d_n13, eq53_e1690_d_n14];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq53_value),
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14,) = {
    if (var_guard763 != 0.0) {
        let eq55_e1699: f64 = (var_deltemp1 * var_gth);
        let eq55_e1699_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq55_e1702: f64 = (var_deltemp1 * var_cth);
        let eq55_e1702_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq55_e1703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq55_e1702);
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1703);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + (eq55_e1702_d_n4 * ddt_scale));
        let eq55_e1706: f64 = (eq55_e1704 - var_pdiss);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - var_pdiss_dn4);
        (eq55_e1706, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq55_e1706_d_n4, (-var_pdiss_dn5), (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11), (-var_pdiss_dn12), (-var_pdiss_dn13), (-var_pdiss_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1708;
        let eq55_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq55_node_derivatives: [f64; 14] = [eq55_e1708_d_n0, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq55_value),
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq70_e1832, eq70_e1832_d_n0, eq70_e1832_d_n2, eq70_e1832_d_n3, eq70_e1832_d_n4, eq70_e1832_d_n5, eq70_e1832_d_n6, eq70_e1832_d_n7, eq70_e1832_d_n8, eq70_e1832_d_n9, eq70_e1832_d_n10, eq70_e1832_d_n11, eq70_e1832_d_n12, eq70_e1832_d_n13, eq70_e1832_d_n14,) = {
    if (var_guard769 != 0.0) {
        let eq70_e1822: f64 = (var_devsign * p.p28);
        let eq70_e1824: f64 = (eq70_e1822 * var_ibs);
        let eq70_e1824_d_n0: f64 = (eq70_e1822 * var_ibs_dn0);
        let eq70_e1824_d_n2: f64 = (eq70_e1822 * var_ibs_dn2);
        let eq70_e1824_d_n3: f64 = (eq70_e1822 * var_ibs_dn3);
        let eq70_e1824_d_n4: f64 = (eq70_e1822 * var_ibs_dn4);
        let eq70_e1824_d_n5: f64 = (eq70_e1822 * var_ibs_dn5);
        let eq70_e1824_d_n6: f64 = (eq70_e1822 * var_ibs_dn6);
        let eq70_e1824_d_n7: f64 = (eq70_e1822 * var_ibs_dn7);
        let eq70_e1824_d_n8: f64 = (eq70_e1822 * var_ibs_dn8);
        let eq70_e1824_d_n9: f64 = (eq70_e1822 * var_ibs_dn9);
        let eq70_e1824_d_n10: f64 = (eq70_e1822 * var_ibs_dn10);
        let eq70_e1824_d_n11: f64 = (eq70_e1822 * var_ibs_dn11);
        let eq70_e1824_d_n12: f64 = (eq70_e1822 * var_ibs_dn12);
        let eq70_e1824_d_n13: f64 = (eq70_e1822 * var_ibs_dn13);
        let eq70_e1824_d_n14: f64 = (eq70_e1822 * var_ibs_dn14);
        let eq70_e1827: f64 = ((nv12 - nv7) * p.p28);
        let eq70_e1829: f64 = (eq70_e1827 * var_gmin);
        let eq70_e1829_d_n7: f64 = ((-p.p28) * var_gmin);
        let eq70_e1829_d_n12: f64 = (p.p28 * var_gmin);
        let eq70_e1830: f64 = (eq70_e1824 + eq70_e1829);
        let eq70_e1830_d_n7: f64 = (eq70_e1824_d_n7 + eq70_e1829_d_n7);
        let eq70_e1830_d_n12: f64 = (eq70_e1824_d_n12 + eq70_e1829_d_n12);
        (eq70_e1830, eq70_e1824_d_n0, eq70_e1824_d_n2, eq70_e1824_d_n3, eq70_e1824_d_n4, eq70_e1824_d_n5, eq70_e1824_d_n6, eq70_e1830_d_n7, eq70_e1824_d_n8, eq70_e1824_d_n9, eq70_e1824_d_n10, eq70_e1824_d_n11, eq70_e1830_d_n12, eq70_e1824_d_n13, eq70_e1824_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1832;
        let eq70_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq70_node_derivatives: [f64; 14] = [eq70_e1832_d_n0, eq70_e1832_d_n2, eq70_e1832_d_n3, eq70_e1832_d_n4, eq70_e1832_d_n5, eq70_e1832_d_n6, eq70_e1832_d_n7, eq70_e1832_d_n8, eq70_e1832_d_n9, eq70_e1832_d_n10, eq70_e1832_d_n11, eq70_e1832_d_n12, eq70_e1832_d_n13, eq70_e1832_d_n14];
        let eq70_branch_derivative_indices: [usize; 0] = [];
        let eq70_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq70_value),
            &eq70_node_derivative_indices,
            &eq70_node_derivatives,
            &eq70_branch_derivative_indices,
            &eq70_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_devsign: f64,
        var_gmin: f64,
        var_guard769: f64,
        var_guard770: f64,
        var_guard772: f64,
        var_ibd: f64,
        var_ibd_dn0: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn12: f64,
        var_ibd_dn13: f64,
        var_ibd_dn14: f64,
        var_ibd_dn2: f64,
        var_ibd_dn3: f64,
        var_ibd_dn4: f64,
        var_ibd_dn5: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_ibd_dn8: f64,
        var_ibd_dn9: f64,
        var_ibd_ext: f64,
        var_ibd_ext_dn0: f64,
        var_ibd_ext_dn10: f64,
        var_ibd_ext_dn11: f64,
        var_ibd_ext_dn12: f64,
        var_ibd_ext_dn13: f64,
        var_ibd_ext_dn14: f64,
        var_ibd_ext_dn2: f64,
        var_ibd_ext_dn3: f64,
        var_ibd_ext_dn4: f64,
        var_ibd_ext_dn5: f64,
        var_ibd_ext_dn6: f64,
        var_ibd_ext_dn7: f64,
        var_ibd_ext_dn8: f64,
        var_ibd_ext_dn9: f64,
        var_ibs: f64,
        var_ibs_dn0: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn12: f64,
        var_ibs_dn13: f64,
        var_ibs_dn14: f64,
        var_ibs_dn2: f64,
        var_ibs_dn3: f64,
        var_ibs_dn4: f64,
        var_ibs_dn5: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_ibs_dn8: f64,
        var_ibs_dn9: f64,
        var_qbdj: f64,
        var_qbdj_dn0: f64,
        var_qbdj_dn10: f64,
        var_qbdj_dn11: f64,
        var_qbdj_dn12: f64,
        var_qbdj_dn13: f64,
        var_qbdj_dn14: f64,
        var_qbdj_dn2: f64,
        var_qbdj_dn3: f64,
        var_qbdj_dn4: f64,
        var_qbdj_dn5: f64,
        var_qbdj_dn6: f64,
        var_qbdj_dn7: f64,
        var_qbdj_dn8: f64,
        var_qbdj_dn9: f64,
        var_qbsj: f64,
        var_qbsj_dn0: f64,
        var_qbsj_dn10: f64,
        var_qbsj_dn11: f64,
        var_qbsj_dn12: f64,
        var_qbsj_dn13: f64,
        var_qbsj_dn14: f64,
        var_qbsj_dn2: f64,
        var_qbsj_dn3: f64,
        var_qbsj_dn4: f64,
        var_qbsj_dn5: f64,
        var_qbsj_dn6: f64,
        var_qbsj_dn7: f64,
        var_qbsj_dn8: f64,
        var_qbsj_dn9: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14,) = {
    if (var_guard769 != 0.0) {
        let eq71_e1837: f64 = (p.p29 * var_qbsj);
        let eq71_e1837_d_n0: f64 = (p.p29 * var_qbsj_dn0);
        let eq71_e1837_d_n2: f64 = (p.p29 * var_qbsj_dn2);
        let eq71_e1837_d_n3: f64 = (p.p29 * var_qbsj_dn3);
        let eq71_e1837_d_n4: f64 = (p.p29 * var_qbsj_dn4);
        let eq71_e1837_d_n5: f64 = (p.p29 * var_qbsj_dn5);
        let eq71_e1837_d_n6: f64 = (p.p29 * var_qbsj_dn6);
        let eq71_e1837_d_n7: f64 = (p.p29 * var_qbsj_dn7);
        let eq71_e1837_d_n8: f64 = (p.p29 * var_qbsj_dn8);
        let eq71_e1837_d_n9: f64 = (p.p29 * var_qbsj_dn9);
        let eq71_e1837_d_n10: f64 = (p.p29 * var_qbsj_dn10);
        let eq71_e1837_d_n11: f64 = (p.p29 * var_qbsj_dn11);
        let eq71_e1837_d_n12: f64 = (p.p29 * var_qbsj_dn12);
        let eq71_e1837_d_n13: f64 = (p.p29 * var_qbsj_dn13);
        let eq71_e1837_d_n14: f64 = (p.p29 * var_qbsj_dn14);
        let eq71_e1838: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq71_e1837);
        let eq71_e1839: f64 = (var_devsign * eq71_e1838);
        let eq71_e1839_d_n0: f64 = (var_devsign * (eq71_e1837_d_n0 * ddt_scale));
        let eq71_e1839_d_n2: f64 = (var_devsign * (eq71_e1837_d_n2 * ddt_scale));
        let eq71_e1839_d_n3: f64 = (var_devsign * (eq71_e1837_d_n3 * ddt_scale));
        let eq71_e1839_d_n4: f64 = (var_devsign * (eq71_e1837_d_n4 * ddt_scale));
        let eq71_e1839_d_n5: f64 = (var_devsign * (eq71_e1837_d_n5 * ddt_scale));
        let eq71_e1839_d_n6: f64 = (var_devsign * (eq71_e1837_d_n6 * ddt_scale));
        let eq71_e1839_d_n7: f64 = (var_devsign * (eq71_e1837_d_n7 * ddt_scale));
        let eq71_e1839_d_n8: f64 = (var_devsign * (eq71_e1837_d_n8 * ddt_scale));
        let eq71_e1839_d_n9: f64 = (var_devsign * (eq71_e1837_d_n9 * ddt_scale));
        let eq71_e1839_d_n10: f64 = (var_devsign * (eq71_e1837_d_n10 * ddt_scale));
        let eq71_e1839_d_n11: f64 = (var_devsign * (eq71_e1837_d_n11 * ddt_scale));
        let eq71_e1839_d_n12: f64 = (var_devsign * (eq71_e1837_d_n12 * ddt_scale));
        let eq71_e1839_d_n13: f64 = (var_devsign * (eq71_e1837_d_n13 * ddt_scale));
        let eq71_e1839_d_n14: f64 = (var_devsign * (eq71_e1837_d_n14 * ddt_scale));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1841;
        let eq71_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq71_node_derivatives: [f64; 14] = [eq71_e1841_d_n0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14];
        let eq71_branch_derivative_indices: [usize; 0] = [];
        let eq71_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq71_value),
            &eq71_node_derivative_indices,
            &eq71_node_derivatives,
            &eq71_branch_derivative_indices,
            &eq71_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1857, eq72_e1857_d_n0, eq72_e1857_d_n2, eq72_e1857_d_n3, eq72_e1857_d_n4, eq72_e1857_d_n5, eq72_e1857_d_n6, eq72_e1857_d_n7, eq72_e1857_d_n8, eq72_e1857_d_n9, eq72_e1857_d_n10, eq72_e1857_d_n11, eq72_e1857_d_n12, eq72_e1857_d_n13, eq72_e1857_d_n14,) = {
    if ((var_guard769 != 0.0) && (var_guard770 != 0.0)) {
        let eq72_e1847: f64 = (var_devsign * p.p28);
        let eq72_e1849: f64 = (eq72_e1847 * var_ibd);
        let eq72_e1849_d_n0: f64 = (eq72_e1847 * var_ibd_dn0);
        let eq72_e1849_d_n2: f64 = (eq72_e1847 * var_ibd_dn2);
        let eq72_e1849_d_n3: f64 = (eq72_e1847 * var_ibd_dn3);
        let eq72_e1849_d_n4: f64 = (eq72_e1847 * var_ibd_dn4);
        let eq72_e1849_d_n5: f64 = (eq72_e1847 * var_ibd_dn5);
        let eq72_e1849_d_n6: f64 = (eq72_e1847 * var_ibd_dn6);
        let eq72_e1849_d_n7: f64 = (eq72_e1847 * var_ibd_dn7);
        let eq72_e1849_d_n8: f64 = (eq72_e1847 * var_ibd_dn8);
        let eq72_e1849_d_n9: f64 = (eq72_e1847 * var_ibd_dn9);
        let eq72_e1849_d_n10: f64 = (eq72_e1847 * var_ibd_dn10);
        let eq72_e1849_d_n11: f64 = (eq72_e1847 * var_ibd_dn11);
        let eq72_e1849_d_n12: f64 = (eq72_e1847 * var_ibd_dn12);
        let eq72_e1849_d_n13: f64 = (eq72_e1847 * var_ibd_dn13);
        let eq72_e1849_d_n14: f64 = (eq72_e1847 * var_ibd_dn14);
        let eq72_e1852: f64 = ((nv13 - nv5) * p.p28);
        let eq72_e1854: f64 = (eq72_e1852 * var_gmin);
        let eq72_e1854_d_n5: f64 = ((-p.p28) * var_gmin);
        let eq72_e1854_d_n13: f64 = (p.p28 * var_gmin);
        let eq72_e1855: f64 = (eq72_e1849 + eq72_e1854);
        let eq72_e1855_d_n5: f64 = (eq72_e1849_d_n5 + eq72_e1854_d_n5);
        let eq72_e1855_d_n13: f64 = (eq72_e1849_d_n13 + eq72_e1854_d_n13);
        (eq72_e1855, eq72_e1849_d_n0, eq72_e1849_d_n2, eq72_e1849_d_n3, eq72_e1849_d_n4, eq72_e1855_d_n5, eq72_e1849_d_n6, eq72_e1849_d_n7, eq72_e1849_d_n8, eq72_e1849_d_n9, eq72_e1849_d_n10, eq72_e1849_d_n11, eq72_e1849_d_n12, eq72_e1855_d_n13, eq72_e1849_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1857;
        let eq72_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq72_node_derivatives: [f64; 14] = [eq72_e1857_d_n0, eq72_e1857_d_n2, eq72_e1857_d_n3, eq72_e1857_d_n4, eq72_e1857_d_n5, eq72_e1857_d_n6, eq72_e1857_d_n7, eq72_e1857_d_n8, eq72_e1857_d_n9, eq72_e1857_d_n10, eq72_e1857_d_n11, eq72_e1857_d_n12, eq72_e1857_d_n13, eq72_e1857_d_n14];
        let eq72_branch_derivative_indices: [usize; 0] = [];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq72_value),
            &eq72_node_derivative_indices,
            &eq72_node_derivatives,
            &eq72_branch_derivative_indices,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14,) = {
    if ((var_guard769 != 0.0) && (var_guard770 != 0.0)) {
        let eq73_e1864: f64 = (p.p29 * var_qbdj);
        let eq73_e1864_d_n0: f64 = (p.p29 * var_qbdj_dn0);
        let eq73_e1864_d_n2: f64 = (p.p29 * var_qbdj_dn2);
        let eq73_e1864_d_n3: f64 = (p.p29 * var_qbdj_dn3);
        let eq73_e1864_d_n4: f64 = (p.p29 * var_qbdj_dn4);
        let eq73_e1864_d_n5: f64 = (p.p29 * var_qbdj_dn5);
        let eq73_e1864_d_n6: f64 = (p.p29 * var_qbdj_dn6);
        let eq73_e1864_d_n7: f64 = (p.p29 * var_qbdj_dn7);
        let eq73_e1864_d_n8: f64 = (p.p29 * var_qbdj_dn8);
        let eq73_e1864_d_n9: f64 = (p.p29 * var_qbdj_dn9);
        let eq73_e1864_d_n10: f64 = (p.p29 * var_qbdj_dn10);
        let eq73_e1864_d_n11: f64 = (p.p29 * var_qbdj_dn11);
        let eq73_e1864_d_n12: f64 = (p.p29 * var_qbdj_dn12);
        let eq73_e1864_d_n13: f64 = (p.p29 * var_qbdj_dn13);
        let eq73_e1864_d_n14: f64 = (p.p29 * var_qbdj_dn14);
        let eq73_e1865: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq73_e1864);
        let eq73_e1866: f64 = (var_devsign * eq73_e1865);
        let eq73_e1866_d_n0: f64 = (var_devsign * (eq73_e1864_d_n0 * ddt_scale));
        let eq73_e1866_d_n2: f64 = (var_devsign * (eq73_e1864_d_n2 * ddt_scale));
        let eq73_e1866_d_n3: f64 = (var_devsign * (eq73_e1864_d_n3 * ddt_scale));
        let eq73_e1866_d_n4: f64 = (var_devsign * (eq73_e1864_d_n4 * ddt_scale));
        let eq73_e1866_d_n5: f64 = (var_devsign * (eq73_e1864_d_n5 * ddt_scale));
        let eq73_e1866_d_n6: f64 = (var_devsign * (eq73_e1864_d_n6 * ddt_scale));
        let eq73_e1866_d_n7: f64 = (var_devsign * (eq73_e1864_d_n7 * ddt_scale));
        let eq73_e1866_d_n8: f64 = (var_devsign * (eq73_e1864_d_n8 * ddt_scale));
        let eq73_e1866_d_n9: f64 = (var_devsign * (eq73_e1864_d_n9 * ddt_scale));
        let eq73_e1866_d_n10: f64 = (var_devsign * (eq73_e1864_d_n10 * ddt_scale));
        let eq73_e1866_d_n11: f64 = (var_devsign * (eq73_e1864_d_n11 * ddt_scale));
        let eq73_e1866_d_n12: f64 = (var_devsign * (eq73_e1864_d_n12 * ddt_scale));
        let eq73_e1866_d_n13: f64 = (var_devsign * (eq73_e1864_d_n13 * ddt_scale));
        let eq73_e1866_d_n14: f64 = (var_devsign * (eq73_e1864_d_n14 * ddt_scale));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1868;
        let eq73_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq73_node_derivatives: [f64; 14] = [eq73_e1868_d_n0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14];
        let eq73_branch_derivative_indices: [usize; 0] = [];
        let eq73_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq73_value),
            &eq73_node_derivative_indices,
            &eq73_node_derivatives,
            &eq73_branch_derivative_indices,
            &eq73_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1883, eq74_e1883_d_n0, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14,) = {
    if (var_guard769 == 0.0) {
        let eq74_e1873: f64 = (var_devsign * p.p28);
        let eq74_e1875: f64 = (eq74_e1873 * var_ibs);
        let eq74_e1875_d_n0: f64 = (eq74_e1873 * var_ibs_dn0);
        let eq74_e1875_d_n2: f64 = (eq74_e1873 * var_ibs_dn2);
        let eq74_e1875_d_n3: f64 = (eq74_e1873 * var_ibs_dn3);
        let eq74_e1875_d_n4: f64 = (eq74_e1873 * var_ibs_dn4);
        let eq74_e1875_d_n5: f64 = (eq74_e1873 * var_ibs_dn5);
        let eq74_e1875_d_n6: f64 = (eq74_e1873 * var_ibs_dn6);
        let eq74_e1875_d_n7: f64 = (eq74_e1873 * var_ibs_dn7);
        let eq74_e1875_d_n8: f64 = (eq74_e1873 * var_ibs_dn8);
        let eq74_e1875_d_n9: f64 = (eq74_e1873 * var_ibs_dn9);
        let eq74_e1875_d_n10: f64 = (eq74_e1873 * var_ibs_dn10);
        let eq74_e1875_d_n11: f64 = (eq74_e1873 * var_ibs_dn11);
        let eq74_e1875_d_n12: f64 = (eq74_e1873 * var_ibs_dn12);
        let eq74_e1875_d_n13: f64 = (eq74_e1873 * var_ibs_dn13);
        let eq74_e1875_d_n14: f64 = (eq74_e1873 * var_ibs_dn14);
        let eq74_e1878: f64 = ((nv11 - nv7) * p.p28);
        let eq74_e1880: f64 = (eq74_e1878 * var_gmin);
        let eq74_e1880_d_n7: f64 = ((-p.p28) * var_gmin);
        let eq74_e1880_d_n11: f64 = (p.p28 * var_gmin);
        let eq74_e1881: f64 = (eq74_e1875 + eq74_e1880);
        let eq74_e1881_d_n7: f64 = (eq74_e1875_d_n7 + eq74_e1880_d_n7);
        let eq74_e1881_d_n11: f64 = (eq74_e1875_d_n11 + eq74_e1880_d_n11);
        (eq74_e1881, eq74_e1875_d_n0, eq74_e1875_d_n2, eq74_e1875_d_n3, eq74_e1875_d_n4, eq74_e1875_d_n5, eq74_e1875_d_n6, eq74_e1881_d_n7, eq74_e1875_d_n8, eq74_e1875_d_n9, eq74_e1875_d_n10, eq74_e1881_d_n11, eq74_e1875_d_n12, eq74_e1875_d_n13, eq74_e1875_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1883;
        let eq74_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq74_node_derivatives: [f64; 14] = [eq74_e1883_d_n0, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14];
        let eq74_branch_derivative_indices: [usize; 0] = [];
        let eq74_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq74_value),
            &eq74_node_derivative_indices,
            &eq74_node_derivatives,
            &eq74_branch_derivative_indices,
            &eq74_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1898, eq75_e1898_d_n0, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14,) = {
    if (var_guard769 == 0.0) {
        let eq75_e1888: f64 = (var_devsign * p.p28);
        let eq75_e1890: f64 = (eq75_e1888 * var_ibd);
        let eq75_e1890_d_n0: f64 = (eq75_e1888 * var_ibd_dn0);
        let eq75_e1890_d_n2: f64 = (eq75_e1888 * var_ibd_dn2);
        let eq75_e1890_d_n3: f64 = (eq75_e1888 * var_ibd_dn3);
        let eq75_e1890_d_n4: f64 = (eq75_e1888 * var_ibd_dn4);
        let eq75_e1890_d_n5: f64 = (eq75_e1888 * var_ibd_dn5);
        let eq75_e1890_d_n6: f64 = (eq75_e1888 * var_ibd_dn6);
        let eq75_e1890_d_n7: f64 = (eq75_e1888 * var_ibd_dn7);
        let eq75_e1890_d_n8: f64 = (eq75_e1888 * var_ibd_dn8);
        let eq75_e1890_d_n9: f64 = (eq75_e1888 * var_ibd_dn9);
        let eq75_e1890_d_n10: f64 = (eq75_e1888 * var_ibd_dn10);
        let eq75_e1890_d_n11: f64 = (eq75_e1888 * var_ibd_dn11);
        let eq75_e1890_d_n12: f64 = (eq75_e1888 * var_ibd_dn12);
        let eq75_e1890_d_n13: f64 = (eq75_e1888 * var_ibd_dn13);
        let eq75_e1890_d_n14: f64 = (eq75_e1888 * var_ibd_dn14);
        let eq75_e1893: f64 = ((nv11 - nv5) * p.p28);
        let eq75_e1895: f64 = (eq75_e1893 * var_gmin);
        let eq75_e1895_d_n5: f64 = ((-p.p28) * var_gmin);
        let eq75_e1895_d_n11: f64 = (p.p28 * var_gmin);
        let eq75_e1896: f64 = (eq75_e1890 + eq75_e1895);
        let eq75_e1896_d_n5: f64 = (eq75_e1890_d_n5 + eq75_e1895_d_n5);
        let eq75_e1896_d_n11: f64 = (eq75_e1890_d_n11 + eq75_e1895_d_n11);
        (eq75_e1896, eq75_e1890_d_n0, eq75_e1890_d_n2, eq75_e1890_d_n3, eq75_e1890_d_n4, eq75_e1896_d_n5, eq75_e1890_d_n6, eq75_e1890_d_n7, eq75_e1890_d_n8, eq75_e1890_d_n9, eq75_e1890_d_n10, eq75_e1896_d_n11, eq75_e1890_d_n12, eq75_e1890_d_n13, eq75_e1890_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1898;
        let eq75_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq75_node_derivatives: [f64; 14] = [eq75_e1898_d_n0, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14];
        let eq75_branch_derivative_indices: [usize; 0] = [];
        let eq75_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq75_value),
            &eq75_node_derivative_indices,
            &eq75_node_derivatives,
            &eq75_branch_derivative_indices,
            &eq75_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14,) = {
    if (var_guard769 == 0.0) {
        let eq76_e1904: f64 = (p.p29 * var_qbsj);
        let eq76_e1904_d_n0: f64 = (p.p29 * var_qbsj_dn0);
        let eq76_e1904_d_n2: f64 = (p.p29 * var_qbsj_dn2);
        let eq76_e1904_d_n3: f64 = (p.p29 * var_qbsj_dn3);
        let eq76_e1904_d_n4: f64 = (p.p29 * var_qbsj_dn4);
        let eq76_e1904_d_n5: f64 = (p.p29 * var_qbsj_dn5);
        let eq76_e1904_d_n6: f64 = (p.p29 * var_qbsj_dn6);
        let eq76_e1904_d_n7: f64 = (p.p29 * var_qbsj_dn7);
        let eq76_e1904_d_n8: f64 = (p.p29 * var_qbsj_dn8);
        let eq76_e1904_d_n9: f64 = (p.p29 * var_qbsj_dn9);
        let eq76_e1904_d_n10: f64 = (p.p29 * var_qbsj_dn10);
        let eq76_e1904_d_n11: f64 = (p.p29 * var_qbsj_dn11);
        let eq76_e1904_d_n12: f64 = (p.p29 * var_qbsj_dn12);
        let eq76_e1904_d_n13: f64 = (p.p29 * var_qbsj_dn13);
        let eq76_e1904_d_n14: f64 = (p.p29 * var_qbsj_dn14);
        let eq76_e1905: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq76_e1904);
        let eq76_e1906: f64 = (var_devsign * eq76_e1905);
        let eq76_e1906_d_n0: f64 = (var_devsign * (eq76_e1904_d_n0 * ddt_scale));
        let eq76_e1906_d_n2: f64 = (var_devsign * (eq76_e1904_d_n2 * ddt_scale));
        let eq76_e1906_d_n3: f64 = (var_devsign * (eq76_e1904_d_n3 * ddt_scale));
        let eq76_e1906_d_n4: f64 = (var_devsign * (eq76_e1904_d_n4 * ddt_scale));
        let eq76_e1906_d_n5: f64 = (var_devsign * (eq76_e1904_d_n5 * ddt_scale));
        let eq76_e1906_d_n6: f64 = (var_devsign * (eq76_e1904_d_n6 * ddt_scale));
        let eq76_e1906_d_n7: f64 = (var_devsign * (eq76_e1904_d_n7 * ddt_scale));
        let eq76_e1906_d_n8: f64 = (var_devsign * (eq76_e1904_d_n8 * ddt_scale));
        let eq76_e1906_d_n9: f64 = (var_devsign * (eq76_e1904_d_n9 * ddt_scale));
        let eq76_e1906_d_n10: f64 = (var_devsign * (eq76_e1904_d_n10 * ddt_scale));
        let eq76_e1906_d_n11: f64 = (var_devsign * (eq76_e1904_d_n11 * ddt_scale));
        let eq76_e1906_d_n12: f64 = (var_devsign * (eq76_e1904_d_n12 * ddt_scale));
        let eq76_e1906_d_n13: f64 = (var_devsign * (eq76_e1904_d_n13 * ddt_scale));
        let eq76_e1906_d_n14: f64 = (var_devsign * (eq76_e1904_d_n14 * ddt_scale));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1908;
        let eq76_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq76_node_derivatives: [f64; 14] = [eq76_e1908_d_n0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14];
        let eq76_branch_derivative_indices: [usize; 0] = [];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq76_value),
            &eq76_node_derivative_indices,
            &eq76_node_derivatives,
            &eq76_branch_derivative_indices,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14,) = {
    if (var_guard769 == 0.0) {
        let eq77_e1914: f64 = (p.p29 * var_qbdj);
        let eq77_e1914_d_n0: f64 = (p.p29 * var_qbdj_dn0);
        let eq77_e1914_d_n2: f64 = (p.p29 * var_qbdj_dn2);
        let eq77_e1914_d_n3: f64 = (p.p29 * var_qbdj_dn3);
        let eq77_e1914_d_n4: f64 = (p.p29 * var_qbdj_dn4);
        let eq77_e1914_d_n5: f64 = (p.p29 * var_qbdj_dn5);
        let eq77_e1914_d_n6: f64 = (p.p29 * var_qbdj_dn6);
        let eq77_e1914_d_n7: f64 = (p.p29 * var_qbdj_dn7);
        let eq77_e1914_d_n8: f64 = (p.p29 * var_qbdj_dn8);
        let eq77_e1914_d_n9: f64 = (p.p29 * var_qbdj_dn9);
        let eq77_e1914_d_n10: f64 = (p.p29 * var_qbdj_dn10);
        let eq77_e1914_d_n11: f64 = (p.p29 * var_qbdj_dn11);
        let eq77_e1914_d_n12: f64 = (p.p29 * var_qbdj_dn12);
        let eq77_e1914_d_n13: f64 = (p.p29 * var_qbdj_dn13);
        let eq77_e1914_d_n14: f64 = (p.p29 * var_qbdj_dn14);
        let eq77_e1915: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq77_e1914);
        let eq77_e1916: f64 = (var_devsign * eq77_e1915);
        let eq77_e1916_d_n0: f64 = (var_devsign * (eq77_e1914_d_n0 * ddt_scale));
        let eq77_e1916_d_n2: f64 = (var_devsign * (eq77_e1914_d_n2 * ddt_scale));
        let eq77_e1916_d_n3: f64 = (var_devsign * (eq77_e1914_d_n3 * ddt_scale));
        let eq77_e1916_d_n4: f64 = (var_devsign * (eq77_e1914_d_n4 * ddt_scale));
        let eq77_e1916_d_n5: f64 = (var_devsign * (eq77_e1914_d_n5 * ddt_scale));
        let eq77_e1916_d_n6: f64 = (var_devsign * (eq77_e1914_d_n6 * ddt_scale));
        let eq77_e1916_d_n7: f64 = (var_devsign * (eq77_e1914_d_n7 * ddt_scale));
        let eq77_e1916_d_n8: f64 = (var_devsign * (eq77_e1914_d_n8 * ddt_scale));
        let eq77_e1916_d_n9: f64 = (var_devsign * (eq77_e1914_d_n9 * ddt_scale));
        let eq77_e1916_d_n10: f64 = (var_devsign * (eq77_e1914_d_n10 * ddt_scale));
        let eq77_e1916_d_n11: f64 = (var_devsign * (eq77_e1914_d_n11 * ddt_scale));
        let eq77_e1916_d_n12: f64 = (var_devsign * (eq77_e1914_d_n12 * ddt_scale));
        let eq77_e1916_d_n13: f64 = (var_devsign * (eq77_e1914_d_n13 * ddt_scale));
        let eq77_e1916_d_n14: f64 = (var_devsign * (eq77_e1914_d_n14 * ddt_scale));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1918;
        let eq77_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq77_node_derivatives: [f64; 14] = [eq77_e1918_d_n0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14];
        let eq77_branch_derivative_indices: [usize; 0] = [];
        let eq77_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq77_value),
            &eq77_node_derivative_indices,
            &eq77_node_derivatives,
            &eq77_branch_derivative_indices,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let (eq81_e1959, eq81_e1959_d_n0, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14,) = {
    if (var_guard772 != 0.0) {
        let eq81_e1945: f64 = (var_devsign * p.p28);
        let eq81_e1947: f64 = (eq81_e1945 * var_ibd);
        let eq81_e1947_d_n0: f64 = (eq81_e1945 * var_ibd_dn0);
        let eq81_e1947_d_n2: f64 = (eq81_e1945 * var_ibd_dn2);
        let eq81_e1947_d_n3: f64 = (eq81_e1945 * var_ibd_dn3);
        let eq81_e1947_d_n4: f64 = (eq81_e1945 * var_ibd_dn4);
        let eq81_e1947_d_n5: f64 = (eq81_e1945 * var_ibd_dn5);
        let eq81_e1947_d_n6: f64 = (eq81_e1945 * var_ibd_dn6);
        let eq81_e1947_d_n7: f64 = (eq81_e1945 * var_ibd_dn7);
        let eq81_e1947_d_n8: f64 = (eq81_e1945 * var_ibd_dn8);
        let eq81_e1947_d_n9: f64 = (eq81_e1945 * var_ibd_dn9);
        let eq81_e1947_d_n10: f64 = (eq81_e1945 * var_ibd_dn10);
        let eq81_e1947_d_n11: f64 = (eq81_e1945 * var_ibd_dn11);
        let eq81_e1947_d_n12: f64 = (eq81_e1945 * var_ibd_dn12);
        let eq81_e1947_d_n13: f64 = (eq81_e1945 * var_ibd_dn13);
        let eq81_e1947_d_n14: f64 = (eq81_e1945 * var_ibd_dn14);
        let eq81_e1950: f64 = (1.0 - p.p1128);
        let eq81_e1952: f64 = (eq81_e1950 * p.p28);
        let eq81_e1954: f64 = (eq81_e1952 * (nv13 - nv5));
        let eq81_e1956: f64 = (eq81_e1954 * var_gmin);
        let eq81_e1956_d_n5: f64 = ((-eq81_e1952) * var_gmin);
        let eq81_e1956_d_n13: f64 = (eq81_e1952 * var_gmin);
        let eq81_e1957: f64 = (eq81_e1947 + eq81_e1956);
        let eq81_e1957_d_n5: f64 = (eq81_e1947_d_n5 + eq81_e1956_d_n5);
        let eq81_e1957_d_n13: f64 = (eq81_e1947_d_n13 + eq81_e1956_d_n13);
        (eq81_e1957, eq81_e1947_d_n0, eq81_e1947_d_n2, eq81_e1947_d_n3, eq81_e1947_d_n4, eq81_e1957_d_n5, eq81_e1947_d_n6, eq81_e1947_d_n7, eq81_e1947_d_n8, eq81_e1947_d_n9, eq81_e1947_d_n10, eq81_e1947_d_n11, eq81_e1947_d_n12, eq81_e1957_d_n13, eq81_e1947_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq81_value: f64 = eq81_e1959;
        let eq81_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq81_node_derivatives: [f64; 14] = [eq81_e1959_d_n0, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14];
        let eq81_branch_derivative_indices: [usize; 0] = [];
        let eq81_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq81_value),
            &eq81_node_derivative_indices,
            &eq81_node_derivatives,
            &eq81_branch_derivative_indices,
            &eq81_branch_derivatives,
            multiplicity,
        );
        let (eq82_e1975, eq82_e1975_d_n0, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14,) = {
    if (var_guard772 != 0.0) {
        let eq82_e1963: f64 = (var_devsign * p.p28);
        let eq82_e1965: f64 = (eq82_e1963 * var_ibd_ext);
        let eq82_e1965_d_n0: f64 = (eq82_e1963 * var_ibd_ext_dn0);
        let eq82_e1965_d_n2: f64 = (eq82_e1963 * var_ibd_ext_dn2);
        let eq82_e1965_d_n3: f64 = (eq82_e1963 * var_ibd_ext_dn3);
        let eq82_e1965_d_n4: f64 = (eq82_e1963 * var_ibd_ext_dn4);
        let eq82_e1965_d_n5: f64 = (eq82_e1963 * var_ibd_ext_dn5);
        let eq82_e1965_d_n6: f64 = (eq82_e1963 * var_ibd_ext_dn6);
        let eq82_e1965_d_n7: f64 = (eq82_e1963 * var_ibd_ext_dn7);
        let eq82_e1965_d_n8: f64 = (eq82_e1963 * var_ibd_ext_dn8);
        let eq82_e1965_d_n9: f64 = (eq82_e1963 * var_ibd_ext_dn9);
        let eq82_e1965_d_n10: f64 = (eq82_e1963 * var_ibd_ext_dn10);
        let eq82_e1965_d_n11: f64 = (eq82_e1963 * var_ibd_ext_dn11);
        let eq82_e1965_d_n12: f64 = (eq82_e1963 * var_ibd_ext_dn12);
        let eq82_e1965_d_n13: f64 = (eq82_e1963 * var_ibd_ext_dn13);
        let eq82_e1965_d_n14: f64 = (eq82_e1963 * var_ibd_ext_dn14);
        let eq82_e1968: f64 = (p.p1128 * p.p28);
        let eq82_e1970: f64 = (eq82_e1968 * (nv13 - nv14));
        let eq82_e1972: f64 = (eq82_e1970 * var_gmin);
        let eq82_e1972_d_n13: f64 = (eq82_e1968 * var_gmin);
        let eq82_e1972_d_n14: f64 = ((-eq82_e1968) * var_gmin);
        let eq82_e1973: f64 = (eq82_e1965 + eq82_e1972);
        let eq82_e1973_d_n13: f64 = (eq82_e1965_d_n13 + eq82_e1972_d_n13);
        let eq82_e1973_d_n14: f64 = (eq82_e1965_d_n14 + eq82_e1972_d_n14);
        (eq82_e1973, eq82_e1965_d_n0, eq82_e1965_d_n2, eq82_e1965_d_n3, eq82_e1965_d_n4, eq82_e1965_d_n5, eq82_e1965_d_n6, eq82_e1965_d_n7, eq82_e1965_d_n8, eq82_e1965_d_n9, eq82_e1965_d_n10, eq82_e1965_d_n11, eq82_e1965_d_n12, eq82_e1973_d_n13, eq82_e1973_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1975;
        let eq82_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq82_node_derivatives: [f64; 14] = [eq82_e1975_d_n0, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14];
        let eq82_branch_derivative_indices: [usize; 0] = [];
        let eq82_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(14),
            multiplicity * (eq82_value),
            &eq82_node_derivative_indices,
            &eq82_node_derivatives,
            &eq82_branch_derivative_indices,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14,) = {
    if (var_guard772 != 0.0) {
        let eq83_e1980: f64 = (p.p29 * var_qbdj);
        let eq83_e1980_d_n0: f64 = (p.p29 * var_qbdj_dn0);
        let eq83_e1980_d_n2: f64 = (p.p29 * var_qbdj_dn2);
        let eq83_e1980_d_n3: f64 = (p.p29 * var_qbdj_dn3);
        let eq83_e1980_d_n4: f64 = (p.p29 * var_qbdj_dn4);
        let eq83_e1980_d_n5: f64 = (p.p29 * var_qbdj_dn5);
        let eq83_e1980_d_n6: f64 = (p.p29 * var_qbdj_dn6);
        let eq83_e1980_d_n7: f64 = (p.p29 * var_qbdj_dn7);
        let eq83_e1980_d_n8: f64 = (p.p29 * var_qbdj_dn8);
        let eq83_e1980_d_n9: f64 = (p.p29 * var_qbdj_dn9);
        let eq83_e1980_d_n10: f64 = (p.p29 * var_qbdj_dn10);
        let eq83_e1980_d_n11: f64 = (p.p29 * var_qbdj_dn11);
        let eq83_e1980_d_n12: f64 = (p.p29 * var_qbdj_dn12);
        let eq83_e1980_d_n13: f64 = (p.p29 * var_qbdj_dn13);
        let eq83_e1980_d_n14: f64 = (p.p29 * var_qbdj_dn14);
        let eq83_e1981: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq83_e1980);
        let eq83_e1982: f64 = (var_devsign * eq83_e1981);
        let eq83_e1982_d_n0: f64 = (var_devsign * (eq83_e1980_d_n0 * ddt_scale));
        let eq83_e1982_d_n2: f64 = (var_devsign * (eq83_e1980_d_n2 * ddt_scale));
        let eq83_e1982_d_n3: f64 = (var_devsign * (eq83_e1980_d_n3 * ddt_scale));
        let eq83_e1982_d_n4: f64 = (var_devsign * (eq83_e1980_d_n4 * ddt_scale));
        let eq83_e1982_d_n5: f64 = (var_devsign * (eq83_e1980_d_n5 * ddt_scale));
        let eq83_e1982_d_n6: f64 = (var_devsign * (eq83_e1980_d_n6 * ddt_scale));
        let eq83_e1982_d_n7: f64 = (var_devsign * (eq83_e1980_d_n7 * ddt_scale));
        let eq83_e1982_d_n8: f64 = (var_devsign * (eq83_e1980_d_n8 * ddt_scale));
        let eq83_e1982_d_n9: f64 = (var_devsign * (eq83_e1980_d_n9 * ddt_scale));
        let eq83_e1982_d_n10: f64 = (var_devsign * (eq83_e1980_d_n10 * ddt_scale));
        let eq83_e1982_d_n11: f64 = (var_devsign * (eq83_e1980_d_n11 * ddt_scale));
        let eq83_e1982_d_n12: f64 = (var_devsign * (eq83_e1980_d_n12 * ddt_scale));
        let eq83_e1982_d_n13: f64 = (var_devsign * (eq83_e1980_d_n13 * ddt_scale));
        let eq83_e1982_d_n14: f64 = (var_devsign * (eq83_e1980_d_n14 * ddt_scale));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1984;
        let eq83_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq83_node_derivatives: [f64; 14] = [eq83_e1984_d_n0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14];
        let eq83_branch_derivative_indices: [usize; 0] = [];
        let eq83_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq83_value),
            &eq83_node_derivative_indices,
            &eq83_node_derivatives,
            &eq83_branch_derivative_indices,
            &eq83_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_devsign: f64,
        var_guard772: f64,
        var_qbdj_ext: f64,
        var_qbdj_ext_dn0: f64,
        var_qbdj_ext_dn10: f64,
        var_qbdj_ext_dn11: f64,
        var_qbdj_ext_dn12: f64,
        var_qbdj_ext_dn13: f64,
        var_qbdj_ext_dn14: f64,
        var_qbdj_ext_dn2: f64,
        var_qbdj_ext_dn3: f64,
        var_qbdj_ext_dn4: f64,
        var_qbdj_ext_dn5: f64,
        var_qbdj_ext_dn6: f64,
        var_qbdj_ext_dn7: f64,
        var_qbdj_ext_dn8: f64,
        var_qbdj_ext_dn9: f64,
    ) {
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14,) = {
    if (var_guard772 != 0.0) {
        let eq84_e1989: f64 = (p.p29 * var_qbdj_ext);
        let eq84_e1989_d_n0: f64 = (p.p29 * var_qbdj_ext_dn0);
        let eq84_e1989_d_n2: f64 = (p.p29 * var_qbdj_ext_dn2);
        let eq84_e1989_d_n3: f64 = (p.p29 * var_qbdj_ext_dn3);
        let eq84_e1989_d_n4: f64 = (p.p29 * var_qbdj_ext_dn4);
        let eq84_e1989_d_n5: f64 = (p.p29 * var_qbdj_ext_dn5);
        let eq84_e1989_d_n6: f64 = (p.p29 * var_qbdj_ext_dn6);
        let eq84_e1989_d_n7: f64 = (p.p29 * var_qbdj_ext_dn7);
        let eq84_e1989_d_n8: f64 = (p.p29 * var_qbdj_ext_dn8);
        let eq84_e1989_d_n9: f64 = (p.p29 * var_qbdj_ext_dn9);
        let eq84_e1989_d_n10: f64 = (p.p29 * var_qbdj_ext_dn10);
        let eq84_e1989_d_n11: f64 = (p.p29 * var_qbdj_ext_dn11);
        let eq84_e1989_d_n12: f64 = (p.p29 * var_qbdj_ext_dn12);
        let eq84_e1989_d_n13: f64 = (p.p29 * var_qbdj_ext_dn13);
        let eq84_e1989_d_n14: f64 = (p.p29 * var_qbdj_ext_dn14);
        let eq84_e1990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq84_e1989);
        let eq84_e1991: f64 = (var_devsign * eq84_e1990);
        let eq84_e1991_d_n0: f64 = (var_devsign * (eq84_e1989_d_n0 * ddt_scale));
        let eq84_e1991_d_n2: f64 = (var_devsign * (eq84_e1989_d_n2 * ddt_scale));
        let eq84_e1991_d_n3: f64 = (var_devsign * (eq84_e1989_d_n3 * ddt_scale));
        let eq84_e1991_d_n4: f64 = (var_devsign * (eq84_e1989_d_n4 * ddt_scale));
        let eq84_e1991_d_n5: f64 = (var_devsign * (eq84_e1989_d_n5 * ddt_scale));
        let eq84_e1991_d_n6: f64 = (var_devsign * (eq84_e1989_d_n6 * ddt_scale));
        let eq84_e1991_d_n7: f64 = (var_devsign * (eq84_e1989_d_n7 * ddt_scale));
        let eq84_e1991_d_n8: f64 = (var_devsign * (eq84_e1989_d_n8 * ddt_scale));
        let eq84_e1991_d_n9: f64 = (var_devsign * (eq84_e1989_d_n9 * ddt_scale));
        let eq84_e1991_d_n10: f64 = (var_devsign * (eq84_e1989_d_n10 * ddt_scale));
        let eq84_e1991_d_n11: f64 = (var_devsign * (eq84_e1989_d_n11 * ddt_scale));
        let eq84_e1991_d_n12: f64 = (var_devsign * (eq84_e1989_d_n12 * ddt_scale));
        let eq84_e1991_d_n13: f64 = (var_devsign * (eq84_e1989_d_n13 * ddt_scale));
        let eq84_e1991_d_n14: f64 = (var_devsign * (eq84_e1989_d_n14 * ddt_scale));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_value: f64 = eq84_e1993;
        let eq84_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq84_node_derivatives: [f64; 14] = [eq84_e1993_d_n0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14];
        let eq84_branch_derivative_indices: [usize; 0] = [];
        let eq84_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(14),
            multiplicity * (eq84_value),
            &eq84_node_derivative_indices,
            &eq84_node_derivatives,
            &eq84_branch_derivative_indices,
            &eq84_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16, eq8_e1290_d_b0, eq8_e1290_d_b1, eq8_e1290_d_b2, eq8_e1290_d_b3, eq8_e1290_d_b4, eq8_e1290_d_b5, eq8_e1290_d_b6, eq8_e1290_d_b7, eq8_e1290_d_b8, eq8_e1290_d_b9, eq8_e1290_d_b10, eq8_e1290_d_b11, eq8_e1290_d_b12, eq8_e1290_d_b13, eq8_e1290_q,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq8_e1279: f64 = (s.v[378] * s.v[46]);
        let eq8_e1281: f64 = (eq8_e1279 * s.v[29]);
        let eq8_e1281_d_n0: f64 = ((s.dn[378][0] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n1: f64 = ((s.dn[378][1] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n2: f64 = ((s.dn[378][2] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n3: f64 = ((s.dn[378][3] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n4: f64 = ((s.dn[378][4] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n5: f64 = ((s.dn[378][5] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n6: f64 = ((s.dn[378][6] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n7: f64 = ((s.dn[378][7] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n8: f64 = ((s.dn[378][8] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n9: f64 = ((s.dn[378][9] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n10: f64 = ((s.dn[378][10] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n11: f64 = ((s.dn[378][11] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n12: f64 = ((s.dn[378][12] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n13: f64 = ((s.dn[378][13] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n14: f64 = ((s.dn[378][14] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n15: f64 = ((s.dn[378][15] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n16: f64 = ((s.dn[378][16] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b0: f64 = ((s.db[378][0] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b1: f64 = ((s.db[378][1] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b2: f64 = ((s.db[378][2] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b3: f64 = ((s.db[378][3] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b4: f64 = ((s.db[378][4] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b5: f64 = ((s.db[378][5] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b6: f64 = ((s.db[378][6] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b7: f64 = ((s.db[378][7] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b8: f64 = ((s.db[378][8] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b9: f64 = ((s.db[378][9] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b10: f64 = ((s.db[378][10] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b11: f64 = ((s.db[378][11] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b12: f64 = ((s.db[378][12] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b13: f64 = ((s.db[378][13] * s.v[46]) * s.v[29]);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n1: f64 = (eq8_e1281_d_n1 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1283_d_n15: f64 = (eq8_e1281_d_n15 * p.p2);
        let eq8_e1283_d_n16: f64 = (eq8_e1281_d_n16 * p.p2);
        let eq8_e1283_d_b0: f64 = (eq8_e1281_d_b0 * p.p2);
        let eq8_e1283_d_b1: f64 = (eq8_e1281_d_b1 * p.p2);
        let eq8_e1283_d_b2: f64 = (eq8_e1281_d_b2 * p.p2);
        let eq8_e1283_d_b3: f64 = (eq8_e1281_d_b3 * p.p2);
        let eq8_e1283_d_b4: f64 = (eq8_e1281_d_b4 * p.p2);
        let eq8_e1283_d_b5: f64 = (eq8_e1281_d_b5 * p.p2);
        let eq8_e1283_d_b6: f64 = (eq8_e1281_d_b6 * p.p2);
        let eq8_e1283_d_b7: f64 = (eq8_e1281_d_b7 * p.p2);
        let eq8_e1283_d_b8: f64 = (eq8_e1281_d_b8 * p.p2);
        let eq8_e1283_d_b9: f64 = (eq8_e1281_d_b9 * p.p2);
        let eq8_e1283_d_b10: f64 = (eq8_e1281_d_b10 * p.p2);
        let eq8_e1283_d_b11: f64 = (eq8_e1281_d_b11 * p.p2);
        let eq8_e1283_d_b12: f64 = (eq8_e1281_d_b12 * p.p2);
        let eq8_e1283_d_b13: f64 = (eq8_e1281_d_b13 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * s.v[30]);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * s.v[30]);
        let eq8_e1285_d_n1: f64 = (eq8_e1283_d_n1 * s.v[30]);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * s.v[30]);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * s.v[30]);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * s.v[30]);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * s.v[30]);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * s.v[30]);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * s.v[30]);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * s.v[30]);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * s.v[30]);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * s.v[30]);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * s.v[30]);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * s.v[30]);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * s.v[30]);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * s.v[30]);
        let eq8_e1285_d_n15: f64 = (eq8_e1283_d_n15 * s.v[30]);
        let eq8_e1285_d_n16: f64 = (eq8_e1283_d_n16 * s.v[30]);
        let eq8_e1285_d_b0: f64 = (eq8_e1283_d_b0 * s.v[30]);
        let eq8_e1285_d_b1: f64 = (eq8_e1283_d_b1 * s.v[30]);
        let eq8_e1285_d_b2: f64 = (eq8_e1283_d_b2 * s.v[30]);
        let eq8_e1285_d_b3: f64 = (eq8_e1283_d_b3 * s.v[30]);
        let eq8_e1285_d_b4: f64 = (eq8_e1283_d_b4 * s.v[30]);
        let eq8_e1285_d_b5: f64 = (eq8_e1283_d_b5 * s.v[30]);
        let eq8_e1285_d_b6: f64 = (eq8_e1283_d_b6 * s.v[30]);
        let eq8_e1285_d_b7: f64 = (eq8_e1283_d_b7 * s.v[30]);
        let eq8_e1285_d_b8: f64 = (eq8_e1283_d_b8 * s.v[30]);
        let eq8_e1285_d_b9: f64 = (eq8_e1283_d_b9 * s.v[30]);
        let eq8_e1285_d_b10: f64 = (eq8_e1283_d_b10 * s.v[30]);
        let eq8_e1285_d_b11: f64 = (eq8_e1283_d_b11 * s.v[30]);
        let eq8_e1285_d_b12: f64 = (eq8_e1283_d_b12 * s.v[30]);
        let eq8_e1285_d_b13: f64 = (eq8_e1283_d_b13 * s.v[30]);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n1: f64 = (eq8_e1285_d_n1 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1287_d_n15: f64 = ((eq8_e1285_d_n15 * (nv15 - 0.0)) + eq8_e1285);
        let eq8_e1287_d_n16: f64 = (eq8_e1285_d_n16 * (nv15 - 0.0));
        let eq8_e1287_d_b0: f64 = (eq8_e1285_d_b0 * (nv15 - 0.0));
        let eq8_e1287_d_b1: f64 = (eq8_e1285_d_b1 * (nv15 - 0.0));
        let eq8_e1287_d_b2: f64 = (eq8_e1285_d_b2 * (nv15 - 0.0));
        let eq8_e1287_d_b3: f64 = (eq8_e1285_d_b3 * (nv15 - 0.0));
        let eq8_e1287_d_b4: f64 = (eq8_e1285_d_b4 * (nv15 - 0.0));
        let eq8_e1287_d_b5: f64 = (eq8_e1285_d_b5 * (nv15 - 0.0));
        let eq8_e1287_d_b6: f64 = (eq8_e1285_d_b6 * (nv15 - 0.0));
        let eq8_e1287_d_b7: f64 = (eq8_e1285_d_b7 * (nv15 - 0.0));
        let eq8_e1287_d_b8: f64 = (eq8_e1285_d_b8 * (nv15 - 0.0));
        let eq8_e1287_d_b9: f64 = (eq8_e1285_d_b9 * (nv15 - 0.0));
        let eq8_e1287_d_b10: f64 = (eq8_e1285_d_b10 * (nv15 - 0.0));
        let eq8_e1287_d_b11: f64 = (eq8_e1285_d_b11 * (nv15 - 0.0));
        let eq8_e1287_d_b12: f64 = (eq8_e1285_d_b12 * (nv15 - 0.0));
        let eq8_e1287_d_b13: f64 = (eq8_e1285_d_b13 * (nv15 - 0.0));
        let eq8_e1288_q: f64 = eq8_e1287;
        (eq8_e1287, eq8_e1287_d_n0, eq8_e1287_d_n1, eq8_e1287_d_n2, eq8_e1287_d_n3, eq8_e1287_d_n4, eq8_e1287_d_n5, eq8_e1287_d_n6, eq8_e1287_d_n7, eq8_e1287_d_n8, eq8_e1287_d_n9, eq8_e1287_d_n10, eq8_e1287_d_n11, eq8_e1287_d_n12, eq8_e1287_d_n13, eq8_e1287_d_n14, eq8_e1287_d_n15, eq8_e1287_d_n16, eq8_e1287_d_b0, eq8_e1287_d_b1, eq8_e1287_d_b2, eq8_e1287_d_b3, eq8_e1287_d_b4, eq8_e1287_d_b5, eq8_e1287_d_b6, eq8_e1287_d_b7, eq8_e1287_d_b8, eq8_e1287_d_b9, eq8_e1287_d_b10, eq8_e1287_d_b11, eq8_e1287_d_b12, eq8_e1287_d_b13, eq8_e1288_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 17] = [eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16];
        let eq8_reactive_branch_derivatives: [f64; 14] = [eq8_e1290_d_b0, eq8_e1290_d_b1, eq8_e1290_d_b2, eq8_e1290_d_b3, eq8_e1290_d_b4, eq8_e1290_d_b5, eq8_e1290_d_b6, eq8_e1290_d_b7, eq8_e1290_d_b8, eq8_e1290_d_b9, eq8_e1290_d_b10, eq8_e1290_d_b11, eq8_e1290_d_b12, eq8_e1290_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16, eq11_e1344_d_b0, eq11_e1344_d_b1, eq11_e1344_d_b2, eq11_e1344_d_b3, eq11_e1344_d_b4, eq11_e1344_d_b5, eq11_e1344_d_b6, eq11_e1344_d_b7, eq11_e1344_d_b8, eq11_e1344_d_b9, eq11_e1344_d_b10, eq11_e1344_d_b11, eq11_e1344_d_b12, eq11_e1344_d_b13, eq11_e1344_q,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq11_e1327: f64 = (1.0 + s.v[57]);
        let eq11_e1329: f64 = (eq11_e1327 * s.v[378]);
        let eq11_e1329_d_n0: f64 = ((s.dn[57][0] * s.v[378]) + (eq11_e1327 * s.dn[378][0]));
        let eq11_e1329_d_n1: f64 = ((s.dn[57][1] * s.v[378]) + (eq11_e1327 * s.dn[378][1]));
        let eq11_e1329_d_n2: f64 = ((s.dn[57][2] * s.v[378]) + (eq11_e1327 * s.dn[378][2]));
        let eq11_e1329_d_n3: f64 = ((s.dn[57][3] * s.v[378]) + (eq11_e1327 * s.dn[378][3]));
        let eq11_e1329_d_n4: f64 = ((s.dn[57][4] * s.v[378]) + (eq11_e1327 * s.dn[378][4]));
        let eq11_e1329_d_n5: f64 = ((s.dn[57][5] * s.v[378]) + (eq11_e1327 * s.dn[378][5]));
        let eq11_e1329_d_n6: f64 = ((s.dn[57][6] * s.v[378]) + (eq11_e1327 * s.dn[378][6]));
        let eq11_e1329_d_n7: f64 = ((s.dn[57][7] * s.v[378]) + (eq11_e1327 * s.dn[378][7]));
        let eq11_e1329_d_n8: f64 = ((s.dn[57][8] * s.v[378]) + (eq11_e1327 * s.dn[378][8]));
        let eq11_e1329_d_n9: f64 = ((s.dn[57][9] * s.v[378]) + (eq11_e1327 * s.dn[378][9]));
        let eq11_e1329_d_n10: f64 = ((s.dn[57][10] * s.v[378]) + (eq11_e1327 * s.dn[378][10]));
        let eq11_e1329_d_n11: f64 = ((s.dn[57][11] * s.v[378]) + (eq11_e1327 * s.dn[378][11]));
        let eq11_e1329_d_n12: f64 = ((s.dn[57][12] * s.v[378]) + (eq11_e1327 * s.dn[378][12]));
        let eq11_e1329_d_n13: f64 = ((s.dn[57][13] * s.v[378]) + (eq11_e1327 * s.dn[378][13]));
        let eq11_e1329_d_n14: f64 = ((s.dn[57][14] * s.v[378]) + (eq11_e1327 * s.dn[378][14]));
        let eq11_e1329_d_n15: f64 = ((s.dn[57][15] * s.v[378]) + (eq11_e1327 * s.dn[378][15]));
        let eq11_e1329_d_n16: f64 = ((s.dn[57][16] * s.v[378]) + (eq11_e1327 * s.dn[378][16]));
        let eq11_e1329_d_b0: f64 = ((s.db[57][0] * s.v[378]) + (eq11_e1327 * s.db[378][0]));
        let eq11_e1329_d_b1: f64 = ((s.db[57][1] * s.v[378]) + (eq11_e1327 * s.db[378][1]));
        let eq11_e1329_d_b2: f64 = ((s.db[57][2] * s.v[378]) + (eq11_e1327 * s.db[378][2]));
        let eq11_e1329_d_b3: f64 = ((s.db[57][3] * s.v[378]) + (eq11_e1327 * s.db[378][3]));
        let eq11_e1329_d_b4: f64 = ((s.db[57][4] * s.v[378]) + (eq11_e1327 * s.db[378][4]));
        let eq11_e1329_d_b5: f64 = ((s.db[57][5] * s.v[378]) + (eq11_e1327 * s.db[378][5]));
        let eq11_e1329_d_b6: f64 = ((s.db[57][6] * s.v[378]) + (eq11_e1327 * s.db[378][6]));
        let eq11_e1329_d_b7: f64 = ((s.db[57][7] * s.v[378]) + (eq11_e1327 * s.db[378][7]));
        let eq11_e1329_d_b8: f64 = ((s.db[57][8] * s.v[378]) + (eq11_e1327 * s.db[378][8]));
        let eq11_e1329_d_b9: f64 = ((s.db[57][9] * s.v[378]) + (eq11_e1327 * s.db[378][9]));
        let eq11_e1329_d_b10: f64 = ((s.db[57][10] * s.v[378]) + (eq11_e1327 * s.db[378][10]));
        let eq11_e1329_d_b11: f64 = ((s.db[57][11] * s.v[378]) + (eq11_e1327 * s.db[378][11]));
        let eq11_e1329_d_b12: f64 = ((s.db[57][12] * s.v[378]) + (eq11_e1327 * s.db[378][12]));
        let eq11_e1329_d_b13: f64 = ((s.db[57][13] * s.v[378]) + (eq11_e1327 * s.db[378][13]));
        let eq11_e1331: f64 = (eq11_e1329 * s.v[46]);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * s.v[46]);
        let eq11_e1331_d_n1: f64 = (eq11_e1329_d_n1 * s.v[46]);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * s.v[46]);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * s.v[46]);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * s.v[46]);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * s.v[46]);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * s.v[46]);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * s.v[46]);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * s.v[46]);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * s.v[46]);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * s.v[46]);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * s.v[46]);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * s.v[46]);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * s.v[46]);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * s.v[46]);
        let eq11_e1331_d_n15: f64 = (eq11_e1329_d_n15 * s.v[46]);
        let eq11_e1331_d_n16: f64 = (eq11_e1329_d_n16 * s.v[46]);
        let eq11_e1331_d_b0: f64 = (eq11_e1329_d_b0 * s.v[46]);
        let eq11_e1331_d_b1: f64 = (eq11_e1329_d_b1 * s.v[46]);
        let eq11_e1331_d_b2: f64 = (eq11_e1329_d_b2 * s.v[46]);
        let eq11_e1331_d_b3: f64 = (eq11_e1329_d_b3 * s.v[46]);
        let eq11_e1331_d_b4: f64 = (eq11_e1329_d_b4 * s.v[46]);
        let eq11_e1331_d_b5: f64 = (eq11_e1329_d_b5 * s.v[46]);
        let eq11_e1331_d_b6: f64 = (eq11_e1329_d_b6 * s.v[46]);
        let eq11_e1331_d_b7: f64 = (eq11_e1329_d_b7 * s.v[46]);
        let eq11_e1331_d_b8: f64 = (eq11_e1329_d_b8 * s.v[46]);
        let eq11_e1331_d_b9: f64 = (eq11_e1329_d_b9 * s.v[46]);
        let eq11_e1331_d_b10: f64 = (eq11_e1329_d_b10 * s.v[46]);
        let eq11_e1331_d_b11: f64 = (eq11_e1329_d_b11 * s.v[46]);
        let eq11_e1331_d_b12: f64 = (eq11_e1329_d_b12 * s.v[46]);
        let eq11_e1331_d_b13: f64 = (eq11_e1329_d_b13 * s.v[46]);
        let eq11_e1333: f64 = (eq11_e1331 * s.v[29]);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * s.v[29]);
        let eq11_e1333_d_n1: f64 = (eq11_e1331_d_n1 * s.v[29]);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * s.v[29]);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * s.v[29]);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * s.v[29]);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * s.v[29]);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * s.v[29]);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * s.v[29]);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * s.v[29]);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * s.v[29]);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * s.v[29]);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * s.v[29]);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * s.v[29]);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * s.v[29]);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * s.v[29]);
        let eq11_e1333_d_n15: f64 = (eq11_e1331_d_n15 * s.v[29]);
        let eq11_e1333_d_n16: f64 = (eq11_e1331_d_n16 * s.v[29]);
        let eq11_e1333_d_b0: f64 = (eq11_e1331_d_b0 * s.v[29]);
        let eq11_e1333_d_b1: f64 = (eq11_e1331_d_b1 * s.v[29]);
        let eq11_e1333_d_b2: f64 = (eq11_e1331_d_b2 * s.v[29]);
        let eq11_e1333_d_b3: f64 = (eq11_e1331_d_b3 * s.v[29]);
        let eq11_e1333_d_b4: f64 = (eq11_e1331_d_b4 * s.v[29]);
        let eq11_e1333_d_b5: f64 = (eq11_e1331_d_b5 * s.v[29]);
        let eq11_e1333_d_b6: f64 = (eq11_e1331_d_b6 * s.v[29]);
        let eq11_e1333_d_b7: f64 = (eq11_e1331_d_b7 * s.v[29]);
        let eq11_e1333_d_b8: f64 = (eq11_e1331_d_b8 * s.v[29]);
        let eq11_e1333_d_b9: f64 = (eq11_e1331_d_b9 * s.v[29]);
        let eq11_e1333_d_b10: f64 = (eq11_e1331_d_b10 * s.v[29]);
        let eq11_e1333_d_b11: f64 = (eq11_e1331_d_b11 * s.v[29]);
        let eq11_e1333_d_b12: f64 = (eq11_e1331_d_b12 * s.v[29]);
        let eq11_e1333_d_b13: f64 = (eq11_e1331_d_b13 * s.v[29]);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n1: f64 = (eq11_e1333_d_n1 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1335_d_n15: f64 = (eq11_e1333_d_n15 * p.p2);
        let eq11_e1335_d_n16: f64 = (eq11_e1333_d_n16 * p.p2);
        let eq11_e1335_d_b0: f64 = (eq11_e1333_d_b0 * p.p2);
        let eq11_e1335_d_b1: f64 = (eq11_e1333_d_b1 * p.p2);
        let eq11_e1335_d_b2: f64 = (eq11_e1333_d_b2 * p.p2);
        let eq11_e1335_d_b3: f64 = (eq11_e1333_d_b3 * p.p2);
        let eq11_e1335_d_b4: f64 = (eq11_e1333_d_b4 * p.p2);
        let eq11_e1335_d_b5: f64 = (eq11_e1333_d_b5 * p.p2);
        let eq11_e1335_d_b6: f64 = (eq11_e1333_d_b6 * p.p2);
        let eq11_e1335_d_b7: f64 = (eq11_e1333_d_b7 * p.p2);
        let eq11_e1335_d_b8: f64 = (eq11_e1333_d_b8 * p.p2);
        let eq11_e1335_d_b9: f64 = (eq11_e1333_d_b9 * p.p2);
        let eq11_e1335_d_b10: f64 = (eq11_e1333_d_b10 * p.p2);
        let eq11_e1335_d_b11: f64 = (eq11_e1333_d_b11 * p.p2);
        let eq11_e1335_d_b12: f64 = (eq11_e1333_d_b12 * p.p2);
        let eq11_e1335_d_b13: f64 = (eq11_e1333_d_b13 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * s.v[30]);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * s.v[30]);
        let eq11_e1337_d_n1: f64 = (eq11_e1335_d_n1 * s.v[30]);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * s.v[30]);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * s.v[30]);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * s.v[30]);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * s.v[30]);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * s.v[30]);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * s.v[30]);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * s.v[30]);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * s.v[30]);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * s.v[30]);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * s.v[30]);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * s.v[30]);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * s.v[30]);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * s.v[30]);
        let eq11_e1337_d_n15: f64 = (eq11_e1335_d_n15 * s.v[30]);
        let eq11_e1337_d_n16: f64 = (eq11_e1335_d_n16 * s.v[30]);
        let eq11_e1337_d_b0: f64 = (eq11_e1335_d_b0 * s.v[30]);
        let eq11_e1337_d_b1: f64 = (eq11_e1335_d_b1 * s.v[30]);
        let eq11_e1337_d_b2: f64 = (eq11_e1335_d_b2 * s.v[30]);
        let eq11_e1337_d_b3: f64 = (eq11_e1335_d_b3 * s.v[30]);
        let eq11_e1337_d_b4: f64 = (eq11_e1335_d_b4 * s.v[30]);
        let eq11_e1337_d_b5: f64 = (eq11_e1335_d_b5 * s.v[30]);
        let eq11_e1337_d_b6: f64 = (eq11_e1335_d_b6 * s.v[30]);
        let eq11_e1337_d_b7: f64 = (eq11_e1335_d_b7 * s.v[30]);
        let eq11_e1337_d_b8: f64 = (eq11_e1335_d_b8 * s.v[30]);
        let eq11_e1337_d_b9: f64 = (eq11_e1335_d_b9 * s.v[30]);
        let eq11_e1337_d_b10: f64 = (eq11_e1335_d_b10 * s.v[30]);
        let eq11_e1337_d_b11: f64 = (eq11_e1335_d_b11 * s.v[30]);
        let eq11_e1337_d_b12: f64 = (eq11_e1335_d_b12 * s.v[30]);
        let eq11_e1337_d_b13: f64 = (eq11_e1335_d_b13 * s.v[30]);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n1: f64 = (eq11_e1337_d_n1 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1339_d_n15: f64 = ((eq11_e1337_d_n15 * (nv15 - 0.0)) + eq11_e1337);
        let eq11_e1339_d_n16: f64 = (eq11_e1337_d_n16 * (nv15 - 0.0));
        let eq11_e1339_d_b0: f64 = (eq11_e1337_d_b0 * (nv15 - 0.0));
        let eq11_e1339_d_b1: f64 = (eq11_e1337_d_b1 * (nv15 - 0.0));
        let eq11_e1339_d_b2: f64 = (eq11_e1337_d_b2 * (nv15 - 0.0));
        let eq11_e1339_d_b3: f64 = (eq11_e1337_d_b3 * (nv15 - 0.0));
        let eq11_e1339_d_b4: f64 = (eq11_e1337_d_b4 * (nv15 - 0.0));
        let eq11_e1339_d_b5: f64 = (eq11_e1337_d_b5 * (nv15 - 0.0));
        let eq11_e1339_d_b6: f64 = (eq11_e1337_d_b6 * (nv15 - 0.0));
        let eq11_e1339_d_b7: f64 = (eq11_e1337_d_b7 * (nv15 - 0.0));
        let eq11_e1339_d_b8: f64 = (eq11_e1337_d_b8 * (nv15 - 0.0));
        let eq11_e1339_d_b9: f64 = (eq11_e1337_d_b9 * (nv15 - 0.0));
        let eq11_e1339_d_b10: f64 = (eq11_e1337_d_b10 * (nv15 - 0.0));
        let eq11_e1339_d_b11: f64 = (eq11_e1337_d_b11 * (nv15 - 0.0));
        let eq11_e1339_d_b12: f64 = (eq11_e1337_d_b12 * (nv15 - 0.0));
        let eq11_e1339_d_b13: f64 = (eq11_e1337_d_b13 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n1: f64 = (0.5 * eq11_e1339_d_n1);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1339_d_n15);
        let eq11_e1340_d_n16: f64 = (0.5 * eq11_e1339_d_n16);
        let eq11_e1340_d_b0: f64 = (0.5 * eq11_e1339_d_b0);
        let eq11_e1340_d_b1: f64 = (0.5 * eq11_e1339_d_b1);
        let eq11_e1340_d_b2: f64 = (0.5 * eq11_e1339_d_b2);
        let eq11_e1340_d_b3: f64 = (0.5 * eq11_e1339_d_b3);
        let eq11_e1340_d_b4: f64 = (0.5 * eq11_e1339_d_b4);
        let eq11_e1340_d_b5: f64 = (0.5 * eq11_e1339_d_b5);
        let eq11_e1340_d_b6: f64 = (0.5 * eq11_e1339_d_b6);
        let eq11_e1340_d_b7: f64 = (0.5 * eq11_e1339_d_b7);
        let eq11_e1340_d_b8: f64 = (0.5 * eq11_e1339_d_b8);
        let eq11_e1340_d_b9: f64 = (0.5 * eq11_e1339_d_b9);
        let eq11_e1340_d_b10: f64 = (0.5 * eq11_e1339_d_b10);
        let eq11_e1340_d_b11: f64 = (0.5 * eq11_e1339_d_b11);
        let eq11_e1340_d_b12: f64 = (0.5 * eq11_e1339_d_b12);
        let eq11_e1340_d_b13: f64 = (0.5 * eq11_e1339_d_b13);
        let eq11_e1341_q: f64 = eq11_e1340;
        let eq11_e1342: f64 = (p.p29 * eq11_e1340);
        let eq11_e1342_d_n0: f64 = (p.p29 * eq11_e1340_d_n0);
        let eq11_e1342_d_n1: f64 = (p.p29 * eq11_e1340_d_n1);
        let eq11_e1342_d_n2: f64 = (p.p29 * eq11_e1340_d_n2);
        let eq11_e1342_d_n3: f64 = (p.p29 * eq11_e1340_d_n3);
        let eq11_e1342_d_n4: f64 = (p.p29 * eq11_e1340_d_n4);
        let eq11_e1342_d_n5: f64 = (p.p29 * eq11_e1340_d_n5);
        let eq11_e1342_d_n6: f64 = (p.p29 * eq11_e1340_d_n6);
        let eq11_e1342_d_n7: f64 = (p.p29 * eq11_e1340_d_n7);
        let eq11_e1342_d_n8: f64 = (p.p29 * eq11_e1340_d_n8);
        let eq11_e1342_d_n9: f64 = (p.p29 * eq11_e1340_d_n9);
        let eq11_e1342_d_n10: f64 = (p.p29 * eq11_e1340_d_n10);
        let eq11_e1342_d_n11: f64 = (p.p29 * eq11_e1340_d_n11);
        let eq11_e1342_d_n12: f64 = (p.p29 * eq11_e1340_d_n12);
        let eq11_e1342_d_n13: f64 = (p.p29 * eq11_e1340_d_n13);
        let eq11_e1342_d_n14: f64 = (p.p29 * eq11_e1340_d_n14);
        let eq11_e1342_d_n15: f64 = (p.p29 * eq11_e1340_d_n15);
        let eq11_e1342_d_n16: f64 = (p.p29 * eq11_e1340_d_n16);
        let eq11_e1342_d_b0: f64 = (p.p29 * eq11_e1340_d_b0);
        let eq11_e1342_d_b1: f64 = (p.p29 * eq11_e1340_d_b1);
        let eq11_e1342_d_b2: f64 = (p.p29 * eq11_e1340_d_b2);
        let eq11_e1342_d_b3: f64 = (p.p29 * eq11_e1340_d_b3);
        let eq11_e1342_d_b4: f64 = (p.p29 * eq11_e1340_d_b4);
        let eq11_e1342_d_b5: f64 = (p.p29 * eq11_e1340_d_b5);
        let eq11_e1342_d_b6: f64 = (p.p29 * eq11_e1340_d_b6);
        let eq11_e1342_d_b7: f64 = (p.p29 * eq11_e1340_d_b7);
        let eq11_e1342_d_b8: f64 = (p.p29 * eq11_e1340_d_b8);
        let eq11_e1342_d_b9: f64 = (p.p29 * eq11_e1340_d_b9);
        let eq11_e1342_d_b10: f64 = (p.p29 * eq11_e1340_d_b10);
        let eq11_e1342_d_b11: f64 = (p.p29 * eq11_e1340_d_b11);
        let eq11_e1342_d_b12: f64 = (p.p29 * eq11_e1340_d_b12);
        let eq11_e1342_d_b13: f64 = (p.p29 * eq11_e1340_d_b13);
        let eq11_e1342_q: f64 = (p.p29 * eq11_e1341_q);
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n1, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15, eq11_e1342_d_n16, eq11_e1342_d_b0, eq11_e1342_d_b1, eq11_e1342_d_b2, eq11_e1342_d_b3, eq11_e1342_d_b4, eq11_e1342_d_b5, eq11_e1342_d_b6, eq11_e1342_d_b7, eq11_e1342_d_b8, eq11_e1342_d_b9, eq11_e1342_d_b10, eq11_e1342_d_b11, eq11_e1342_d_b12, eq11_e1342_d_b13, eq11_e1342_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 17] = [eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16];
        let eq11_reactive_branch_derivatives: [f64; 14] = [eq11_e1344_d_b0, eq11_e1344_d_b1, eq11_e1344_d_b2, eq11_e1344_d_b3, eq11_e1344_d_b4, eq11_e1344_d_b5, eq11_e1344_d_b6, eq11_e1344_d_b7, eq11_e1344_d_b8, eq11_e1344_d_b9, eq11_e1344_d_b10, eq11_e1344_d_b11, eq11_e1344_d_b12, eq11_e1344_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16, eq12_e1370_d_b0, eq12_e1370_d_b1, eq12_e1370_d_b2, eq12_e1370_d_b3, eq12_e1370_d_b4, eq12_e1370_d_b5, eq12_e1370_d_b6, eq12_e1370_d_b7, eq12_e1370_d_b8, eq12_e1370_d_b9, eq12_e1370_d_b10, eq12_e1370_d_b11, eq12_e1370_d_b12, eq12_e1370_d_b13, eq12_e1370_q,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq12_e1353: f64 = (1.0 - s.v[57]);
        let eq12_e1355: f64 = (eq12_e1353 * s.v[378]);
        let eq12_e1355_d_n0: f64 = (((-s.dn[57][0]) * s.v[378]) + (eq12_e1353 * s.dn[378][0]));
        let eq12_e1355_d_n1: f64 = (((-s.dn[57][1]) * s.v[378]) + (eq12_e1353 * s.dn[378][1]));
        let eq12_e1355_d_n2: f64 = (((-s.dn[57][2]) * s.v[378]) + (eq12_e1353 * s.dn[378][2]));
        let eq12_e1355_d_n3: f64 = (((-s.dn[57][3]) * s.v[378]) + (eq12_e1353 * s.dn[378][3]));
        let eq12_e1355_d_n4: f64 = (((-s.dn[57][4]) * s.v[378]) + (eq12_e1353 * s.dn[378][4]));
        let eq12_e1355_d_n5: f64 = (((-s.dn[57][5]) * s.v[378]) + (eq12_e1353 * s.dn[378][5]));
        let eq12_e1355_d_n6: f64 = (((-s.dn[57][6]) * s.v[378]) + (eq12_e1353 * s.dn[378][6]));
        let eq12_e1355_d_n7: f64 = (((-s.dn[57][7]) * s.v[378]) + (eq12_e1353 * s.dn[378][7]));
        let eq12_e1355_d_n8: f64 = (((-s.dn[57][8]) * s.v[378]) + (eq12_e1353 * s.dn[378][8]));
        let eq12_e1355_d_n9: f64 = (((-s.dn[57][9]) * s.v[378]) + (eq12_e1353 * s.dn[378][9]));
        let eq12_e1355_d_n10: f64 = (((-s.dn[57][10]) * s.v[378]) + (eq12_e1353 * s.dn[378][10]));
        let eq12_e1355_d_n11: f64 = (((-s.dn[57][11]) * s.v[378]) + (eq12_e1353 * s.dn[378][11]));
        let eq12_e1355_d_n12: f64 = (((-s.dn[57][12]) * s.v[378]) + (eq12_e1353 * s.dn[378][12]));
        let eq12_e1355_d_n13: f64 = (((-s.dn[57][13]) * s.v[378]) + (eq12_e1353 * s.dn[378][13]));
        let eq12_e1355_d_n14: f64 = (((-s.dn[57][14]) * s.v[378]) + (eq12_e1353 * s.dn[378][14]));
        let eq12_e1355_d_n15: f64 = (((-s.dn[57][15]) * s.v[378]) + (eq12_e1353 * s.dn[378][15]));
        let eq12_e1355_d_n16: f64 = (((-s.dn[57][16]) * s.v[378]) + (eq12_e1353 * s.dn[378][16]));
        let eq12_e1355_d_b0: f64 = (((-s.db[57][0]) * s.v[378]) + (eq12_e1353 * s.db[378][0]));
        let eq12_e1355_d_b1: f64 = (((-s.db[57][1]) * s.v[378]) + (eq12_e1353 * s.db[378][1]));
        let eq12_e1355_d_b2: f64 = (((-s.db[57][2]) * s.v[378]) + (eq12_e1353 * s.db[378][2]));
        let eq12_e1355_d_b3: f64 = (((-s.db[57][3]) * s.v[378]) + (eq12_e1353 * s.db[378][3]));
        let eq12_e1355_d_b4: f64 = (((-s.db[57][4]) * s.v[378]) + (eq12_e1353 * s.db[378][4]));
        let eq12_e1355_d_b5: f64 = (((-s.db[57][5]) * s.v[378]) + (eq12_e1353 * s.db[378][5]));
        let eq12_e1355_d_b6: f64 = (((-s.db[57][6]) * s.v[378]) + (eq12_e1353 * s.db[378][6]));
        let eq12_e1355_d_b7: f64 = (((-s.db[57][7]) * s.v[378]) + (eq12_e1353 * s.db[378][7]));
        let eq12_e1355_d_b8: f64 = (((-s.db[57][8]) * s.v[378]) + (eq12_e1353 * s.db[378][8]));
        let eq12_e1355_d_b9: f64 = (((-s.db[57][9]) * s.v[378]) + (eq12_e1353 * s.db[378][9]));
        let eq12_e1355_d_b10: f64 = (((-s.db[57][10]) * s.v[378]) + (eq12_e1353 * s.db[378][10]));
        let eq12_e1355_d_b11: f64 = (((-s.db[57][11]) * s.v[378]) + (eq12_e1353 * s.db[378][11]));
        let eq12_e1355_d_b12: f64 = (((-s.db[57][12]) * s.v[378]) + (eq12_e1353 * s.db[378][12]));
        let eq12_e1355_d_b13: f64 = (((-s.db[57][13]) * s.v[378]) + (eq12_e1353 * s.db[378][13]));
        let eq12_e1357: f64 = (eq12_e1355 * s.v[46]);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * s.v[46]);
        let eq12_e1357_d_n1: f64 = (eq12_e1355_d_n1 * s.v[46]);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * s.v[46]);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * s.v[46]);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * s.v[46]);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * s.v[46]);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * s.v[46]);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * s.v[46]);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * s.v[46]);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * s.v[46]);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * s.v[46]);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * s.v[46]);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * s.v[46]);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * s.v[46]);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * s.v[46]);
        let eq12_e1357_d_n15: f64 = (eq12_e1355_d_n15 * s.v[46]);
        let eq12_e1357_d_n16: f64 = (eq12_e1355_d_n16 * s.v[46]);
        let eq12_e1357_d_b0: f64 = (eq12_e1355_d_b0 * s.v[46]);
        let eq12_e1357_d_b1: f64 = (eq12_e1355_d_b1 * s.v[46]);
        let eq12_e1357_d_b2: f64 = (eq12_e1355_d_b2 * s.v[46]);
        let eq12_e1357_d_b3: f64 = (eq12_e1355_d_b3 * s.v[46]);
        let eq12_e1357_d_b4: f64 = (eq12_e1355_d_b4 * s.v[46]);
        let eq12_e1357_d_b5: f64 = (eq12_e1355_d_b5 * s.v[46]);
        let eq12_e1357_d_b6: f64 = (eq12_e1355_d_b6 * s.v[46]);
        let eq12_e1357_d_b7: f64 = (eq12_e1355_d_b7 * s.v[46]);
        let eq12_e1357_d_b8: f64 = (eq12_e1355_d_b8 * s.v[46]);
        let eq12_e1357_d_b9: f64 = (eq12_e1355_d_b9 * s.v[46]);
        let eq12_e1357_d_b10: f64 = (eq12_e1355_d_b10 * s.v[46]);
        let eq12_e1357_d_b11: f64 = (eq12_e1355_d_b11 * s.v[46]);
        let eq12_e1357_d_b12: f64 = (eq12_e1355_d_b12 * s.v[46]);
        let eq12_e1357_d_b13: f64 = (eq12_e1355_d_b13 * s.v[46]);
        let eq12_e1359: f64 = (eq12_e1357 * s.v[29]);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * s.v[29]);
        let eq12_e1359_d_n1: f64 = (eq12_e1357_d_n1 * s.v[29]);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * s.v[29]);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * s.v[29]);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * s.v[29]);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * s.v[29]);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * s.v[29]);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * s.v[29]);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * s.v[29]);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * s.v[29]);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * s.v[29]);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * s.v[29]);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * s.v[29]);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * s.v[29]);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * s.v[29]);
        let eq12_e1359_d_n15: f64 = (eq12_e1357_d_n15 * s.v[29]);
        let eq12_e1359_d_n16: f64 = (eq12_e1357_d_n16 * s.v[29]);
        let eq12_e1359_d_b0: f64 = (eq12_e1357_d_b0 * s.v[29]);
        let eq12_e1359_d_b1: f64 = (eq12_e1357_d_b1 * s.v[29]);
        let eq12_e1359_d_b2: f64 = (eq12_e1357_d_b2 * s.v[29]);
        let eq12_e1359_d_b3: f64 = (eq12_e1357_d_b3 * s.v[29]);
        let eq12_e1359_d_b4: f64 = (eq12_e1357_d_b4 * s.v[29]);
        let eq12_e1359_d_b5: f64 = (eq12_e1357_d_b5 * s.v[29]);
        let eq12_e1359_d_b6: f64 = (eq12_e1357_d_b6 * s.v[29]);
        let eq12_e1359_d_b7: f64 = (eq12_e1357_d_b7 * s.v[29]);
        let eq12_e1359_d_b8: f64 = (eq12_e1357_d_b8 * s.v[29]);
        let eq12_e1359_d_b9: f64 = (eq12_e1357_d_b9 * s.v[29]);
        let eq12_e1359_d_b10: f64 = (eq12_e1357_d_b10 * s.v[29]);
        let eq12_e1359_d_b11: f64 = (eq12_e1357_d_b11 * s.v[29]);
        let eq12_e1359_d_b12: f64 = (eq12_e1357_d_b12 * s.v[29]);
        let eq12_e1359_d_b13: f64 = (eq12_e1357_d_b13 * s.v[29]);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n1: f64 = (eq12_e1359_d_n1 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1361_d_n15: f64 = (eq12_e1359_d_n15 * p.p2);
        let eq12_e1361_d_n16: f64 = (eq12_e1359_d_n16 * p.p2);
        let eq12_e1361_d_b0: f64 = (eq12_e1359_d_b0 * p.p2);
        let eq12_e1361_d_b1: f64 = (eq12_e1359_d_b1 * p.p2);
        let eq12_e1361_d_b2: f64 = (eq12_e1359_d_b2 * p.p2);
        let eq12_e1361_d_b3: f64 = (eq12_e1359_d_b3 * p.p2);
        let eq12_e1361_d_b4: f64 = (eq12_e1359_d_b4 * p.p2);
        let eq12_e1361_d_b5: f64 = (eq12_e1359_d_b5 * p.p2);
        let eq12_e1361_d_b6: f64 = (eq12_e1359_d_b6 * p.p2);
        let eq12_e1361_d_b7: f64 = (eq12_e1359_d_b7 * p.p2);
        let eq12_e1361_d_b8: f64 = (eq12_e1359_d_b8 * p.p2);
        let eq12_e1361_d_b9: f64 = (eq12_e1359_d_b9 * p.p2);
        let eq12_e1361_d_b10: f64 = (eq12_e1359_d_b10 * p.p2);
        let eq12_e1361_d_b11: f64 = (eq12_e1359_d_b11 * p.p2);
        let eq12_e1361_d_b12: f64 = (eq12_e1359_d_b12 * p.p2);
        let eq12_e1361_d_b13: f64 = (eq12_e1359_d_b13 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * s.v[30]);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * s.v[30]);
        let eq12_e1363_d_n1: f64 = (eq12_e1361_d_n1 * s.v[30]);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * s.v[30]);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * s.v[30]);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * s.v[30]);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * s.v[30]);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * s.v[30]);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * s.v[30]);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * s.v[30]);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * s.v[30]);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * s.v[30]);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * s.v[30]);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * s.v[30]);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * s.v[30]);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * s.v[30]);
        let eq12_e1363_d_n15: f64 = (eq12_e1361_d_n15 * s.v[30]);
        let eq12_e1363_d_n16: f64 = (eq12_e1361_d_n16 * s.v[30]);
        let eq12_e1363_d_b0: f64 = (eq12_e1361_d_b0 * s.v[30]);
        let eq12_e1363_d_b1: f64 = (eq12_e1361_d_b1 * s.v[30]);
        let eq12_e1363_d_b2: f64 = (eq12_e1361_d_b2 * s.v[30]);
        let eq12_e1363_d_b3: f64 = (eq12_e1361_d_b3 * s.v[30]);
        let eq12_e1363_d_b4: f64 = (eq12_e1361_d_b4 * s.v[30]);
        let eq12_e1363_d_b5: f64 = (eq12_e1361_d_b5 * s.v[30]);
        let eq12_e1363_d_b6: f64 = (eq12_e1361_d_b6 * s.v[30]);
        let eq12_e1363_d_b7: f64 = (eq12_e1361_d_b7 * s.v[30]);
        let eq12_e1363_d_b8: f64 = (eq12_e1361_d_b8 * s.v[30]);
        let eq12_e1363_d_b9: f64 = (eq12_e1361_d_b9 * s.v[30]);
        let eq12_e1363_d_b10: f64 = (eq12_e1361_d_b10 * s.v[30]);
        let eq12_e1363_d_b11: f64 = (eq12_e1361_d_b11 * s.v[30]);
        let eq12_e1363_d_b12: f64 = (eq12_e1361_d_b12 * s.v[30]);
        let eq12_e1363_d_b13: f64 = (eq12_e1361_d_b13 * s.v[30]);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n1: f64 = (eq12_e1363_d_n1 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1365_d_n15: f64 = ((eq12_e1363_d_n15 * (nv15 - 0.0)) + eq12_e1363);
        let eq12_e1365_d_n16: f64 = (eq12_e1363_d_n16 * (nv15 - 0.0));
        let eq12_e1365_d_b0: f64 = (eq12_e1363_d_b0 * (nv15 - 0.0));
        let eq12_e1365_d_b1: f64 = (eq12_e1363_d_b1 * (nv15 - 0.0));
        let eq12_e1365_d_b2: f64 = (eq12_e1363_d_b2 * (nv15 - 0.0));
        let eq12_e1365_d_b3: f64 = (eq12_e1363_d_b3 * (nv15 - 0.0));
        let eq12_e1365_d_b4: f64 = (eq12_e1363_d_b4 * (nv15 - 0.0));
        let eq12_e1365_d_b5: f64 = (eq12_e1363_d_b5 * (nv15 - 0.0));
        let eq12_e1365_d_b6: f64 = (eq12_e1363_d_b6 * (nv15 - 0.0));
        let eq12_e1365_d_b7: f64 = (eq12_e1363_d_b7 * (nv15 - 0.0));
        let eq12_e1365_d_b8: f64 = (eq12_e1363_d_b8 * (nv15 - 0.0));
        let eq12_e1365_d_b9: f64 = (eq12_e1363_d_b9 * (nv15 - 0.0));
        let eq12_e1365_d_b10: f64 = (eq12_e1363_d_b10 * (nv15 - 0.0));
        let eq12_e1365_d_b11: f64 = (eq12_e1363_d_b11 * (nv15 - 0.0));
        let eq12_e1365_d_b12: f64 = (eq12_e1363_d_b12 * (nv15 - 0.0));
        let eq12_e1365_d_b13: f64 = (eq12_e1363_d_b13 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n1: f64 = (0.5 * eq12_e1365_d_n1);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1365_d_n15);
        let eq12_e1366_d_n16: f64 = (0.5 * eq12_e1365_d_n16);
        let eq12_e1366_d_b0: f64 = (0.5 * eq12_e1365_d_b0);
        let eq12_e1366_d_b1: f64 = (0.5 * eq12_e1365_d_b1);
        let eq12_e1366_d_b2: f64 = (0.5 * eq12_e1365_d_b2);
        let eq12_e1366_d_b3: f64 = (0.5 * eq12_e1365_d_b3);
        let eq12_e1366_d_b4: f64 = (0.5 * eq12_e1365_d_b4);
        let eq12_e1366_d_b5: f64 = (0.5 * eq12_e1365_d_b5);
        let eq12_e1366_d_b6: f64 = (0.5 * eq12_e1365_d_b6);
        let eq12_e1366_d_b7: f64 = (0.5 * eq12_e1365_d_b7);
        let eq12_e1366_d_b8: f64 = (0.5 * eq12_e1365_d_b8);
        let eq12_e1366_d_b9: f64 = (0.5 * eq12_e1365_d_b9);
        let eq12_e1366_d_b10: f64 = (0.5 * eq12_e1365_d_b10);
        let eq12_e1366_d_b11: f64 = (0.5 * eq12_e1365_d_b11);
        let eq12_e1366_d_b12: f64 = (0.5 * eq12_e1365_d_b12);
        let eq12_e1366_d_b13: f64 = (0.5 * eq12_e1365_d_b13);
        let eq12_e1367_q: f64 = eq12_e1366;
        let eq12_e1368: f64 = (p.p29 * eq12_e1366);
        let eq12_e1368_d_n0: f64 = (p.p29 * eq12_e1366_d_n0);
        let eq12_e1368_d_n1: f64 = (p.p29 * eq12_e1366_d_n1);
        let eq12_e1368_d_n2: f64 = (p.p29 * eq12_e1366_d_n2);
        let eq12_e1368_d_n3: f64 = (p.p29 * eq12_e1366_d_n3);
        let eq12_e1368_d_n4: f64 = (p.p29 * eq12_e1366_d_n4);
        let eq12_e1368_d_n5: f64 = (p.p29 * eq12_e1366_d_n5);
        let eq12_e1368_d_n6: f64 = (p.p29 * eq12_e1366_d_n6);
        let eq12_e1368_d_n7: f64 = (p.p29 * eq12_e1366_d_n7);
        let eq12_e1368_d_n8: f64 = (p.p29 * eq12_e1366_d_n8);
        let eq12_e1368_d_n9: f64 = (p.p29 * eq12_e1366_d_n9);
        let eq12_e1368_d_n10: f64 = (p.p29 * eq12_e1366_d_n10);
        let eq12_e1368_d_n11: f64 = (p.p29 * eq12_e1366_d_n11);
        let eq12_e1368_d_n12: f64 = (p.p29 * eq12_e1366_d_n12);
        let eq12_e1368_d_n13: f64 = (p.p29 * eq12_e1366_d_n13);
        let eq12_e1368_d_n14: f64 = (p.p29 * eq12_e1366_d_n14);
        let eq12_e1368_d_n15: f64 = (p.p29 * eq12_e1366_d_n15);
        let eq12_e1368_d_n16: f64 = (p.p29 * eq12_e1366_d_n16);
        let eq12_e1368_d_b0: f64 = (p.p29 * eq12_e1366_d_b0);
        let eq12_e1368_d_b1: f64 = (p.p29 * eq12_e1366_d_b1);
        let eq12_e1368_d_b2: f64 = (p.p29 * eq12_e1366_d_b2);
        let eq12_e1368_d_b3: f64 = (p.p29 * eq12_e1366_d_b3);
        let eq12_e1368_d_b4: f64 = (p.p29 * eq12_e1366_d_b4);
        let eq12_e1368_d_b5: f64 = (p.p29 * eq12_e1366_d_b5);
        let eq12_e1368_d_b6: f64 = (p.p29 * eq12_e1366_d_b6);
        let eq12_e1368_d_b7: f64 = (p.p29 * eq12_e1366_d_b7);
        let eq12_e1368_d_b8: f64 = (p.p29 * eq12_e1366_d_b8);
        let eq12_e1368_d_b9: f64 = (p.p29 * eq12_e1366_d_b9);
        let eq12_e1368_d_b10: f64 = (p.p29 * eq12_e1366_d_b10);
        let eq12_e1368_d_b11: f64 = (p.p29 * eq12_e1366_d_b11);
        let eq12_e1368_d_b12: f64 = (p.p29 * eq12_e1366_d_b12);
        let eq12_e1368_d_b13: f64 = (p.p29 * eq12_e1366_d_b13);
        let eq12_e1368_q: f64 = (p.p29 * eq12_e1367_q);
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n1, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15, eq12_e1368_d_n16, eq12_e1368_d_b0, eq12_e1368_d_b1, eq12_e1368_d_b2, eq12_e1368_d_b3, eq12_e1368_d_b4, eq12_e1368_d_b5, eq12_e1368_d_b6, eq12_e1368_d_b7, eq12_e1368_d_b8, eq12_e1368_d_b9, eq12_e1368_d_b10, eq12_e1368_d_b11, eq12_e1368_d_b12, eq12_e1368_d_b13, eq12_e1368_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_reactive_node_derivatives: [f64; 17] = [eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16];
        let eq12_reactive_branch_derivatives: [f64; 14] = [eq12_e1370_d_b0, eq12_e1370_d_b1, eq12_e1370_d_b2, eq12_e1370_d_b3, eq12_e1370_d_b4, eq12_e1370_d_b5, eq12_e1370_d_b6, eq12_e1370_d_b7, eq12_e1370_d_b8, eq12_e1370_d_b9, eq12_e1370_d_b10, eq12_e1370_d_b11, eq12_e1370_d_b12, eq12_e1370_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e1428_q: f64 = s.v[787];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[11]),
            nodes,
            &s.dn[787],
            branches,
            &s.db[787],
            multiplicity,
        );
        let eq20_e1430_q: f64 = s.v[785];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &s.dn[785],
            branches,
            &s.db[785],
            multiplicity,
        );
        let eq21_e1432_q: f64 = s.v[786];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &s.dn[786],
            branches,
            &s.db[786],
            multiplicity,
        );
        let eq22_e1435: f64 = (-s.v[187]);
        let eq22_e1437: f64 = (eq22_e1435 * s.v[223]);
        let eq22_e1437_d_n0: f64 = (((-s.dn[187][0]) * s.v[223]) + (eq22_e1435 * s.dn[223][0]));
        let eq22_e1437_d_n1: f64 = (((-s.dn[187][1]) * s.v[223]) + (eq22_e1435 * s.dn[223][1]));
        let eq22_e1437_d_n2: f64 = (((-s.dn[187][2]) * s.v[223]) + (eq22_e1435 * s.dn[223][2]));
        let eq22_e1437_d_n3: f64 = (((-s.dn[187][3]) * s.v[223]) + (eq22_e1435 * s.dn[223][3]));
        let eq22_e1437_d_n4: f64 = (((-s.dn[187][4]) * s.v[223]) + (eq22_e1435 * s.dn[223][4]));
        let eq22_e1437_d_n5: f64 = (((-s.dn[187][5]) * s.v[223]) + (eq22_e1435 * s.dn[223][5]));
        let eq22_e1437_d_n6: f64 = (((-s.dn[187][6]) * s.v[223]) + (eq22_e1435 * s.dn[223][6]));
        let eq22_e1437_d_n7: f64 = (((-s.dn[187][7]) * s.v[223]) + (eq22_e1435 * s.dn[223][7]));
        let eq22_e1437_d_n8: f64 = (((-s.dn[187][8]) * s.v[223]) + (eq22_e1435 * s.dn[223][8]));
        let eq22_e1437_d_n9: f64 = (((-s.dn[187][9]) * s.v[223]) + (eq22_e1435 * s.dn[223][9]));
        let eq22_e1437_d_n10: f64 = (((-s.dn[187][10]) * s.v[223]) + (eq22_e1435 * s.dn[223][10]));
        let eq22_e1437_d_n11: f64 = (((-s.dn[187][11]) * s.v[223]) + (eq22_e1435 * s.dn[223][11]));
        let eq22_e1437_d_n12: f64 = (((-s.dn[187][12]) * s.v[223]) + (eq22_e1435 * s.dn[223][12]));
        let eq22_e1437_d_n13: f64 = (((-s.dn[187][13]) * s.v[223]) + (eq22_e1435 * s.dn[223][13]));
        let eq22_e1437_d_n14: f64 = (((-s.dn[187][14]) * s.v[223]) + (eq22_e1435 * s.dn[223][14]));
        let eq22_e1437_d_n15: f64 = (((-s.dn[187][15]) * s.v[223]) + (eq22_e1435 * s.dn[223][15]));
        let eq22_e1437_d_n16: f64 = (((-s.dn[187][16]) * s.v[223]) + (eq22_e1435 * s.dn[223][16]));
        let eq22_e1437_d_b0: f64 = (((-s.db[187][0]) * s.v[223]) + (eq22_e1435 * s.db[223][0]));
        let eq22_e1437_d_b1: f64 = (((-s.db[187][1]) * s.v[223]) + (eq22_e1435 * s.db[223][1]));
        let eq22_e1437_d_b2: f64 = (((-s.db[187][2]) * s.v[223]) + (eq22_e1435 * s.db[223][2]));
        let eq22_e1437_d_b3: f64 = (((-s.db[187][3]) * s.v[223]) + (eq22_e1435 * s.db[223][3]));
        let eq22_e1437_d_b4: f64 = (((-s.db[187][4]) * s.v[223]) + (eq22_e1435 * s.db[223][4]));
        let eq22_e1437_d_b5: f64 = (((-s.db[187][5]) * s.v[223]) + (eq22_e1435 * s.db[223][5]));
        let eq22_e1437_d_b6: f64 = (((-s.db[187][6]) * s.v[223]) + (eq22_e1435 * s.db[223][6]));
        let eq22_e1437_d_b7: f64 = (((-s.db[187][7]) * s.v[223]) + (eq22_e1435 * s.db[223][7]));
        let eq22_e1437_d_b8: f64 = (((-s.db[187][8]) * s.v[223]) + (eq22_e1435 * s.db[223][8]));
        let eq22_e1437_d_b9: f64 = (((-s.db[187][9]) * s.v[223]) + (eq22_e1435 * s.db[223][9]));
        let eq22_e1437_d_b10: f64 = (((-s.db[187][10]) * s.v[223]) + (eq22_e1435 * s.db[223][10]));
        let eq22_e1437_d_b11: f64 = (((-s.db[187][11]) * s.v[223]) + (eq22_e1435 * s.db[223][11]));
        let eq22_e1437_d_b12: f64 = (((-s.db[187][12]) * s.v[223]) + (eq22_e1435 * s.db[223][12]));
        let eq22_e1437_d_b13: f64 = (((-s.db[187][13]) * s.v[223]) + (eq22_e1435 * s.db[223][13]));
        let eq22_e1438_q: f64 = eq22_e1437;
        let eq22_e1439: f64 = (p.p29 * eq22_e1437);
        let eq22_e1439_d_n0: f64 = (p.p29 * eq22_e1437_d_n0);
        let eq22_e1439_d_n1: f64 = (p.p29 * eq22_e1437_d_n1);
        let eq22_e1439_d_n2: f64 = (p.p29 * eq22_e1437_d_n2);
        let eq22_e1439_d_n3: f64 = (p.p29 * eq22_e1437_d_n3);
        let eq22_e1439_d_n4: f64 = (p.p29 * eq22_e1437_d_n4);
        let eq22_e1439_d_n5: f64 = (p.p29 * eq22_e1437_d_n5);
        let eq22_e1439_d_n6: f64 = (p.p29 * eq22_e1437_d_n6);
        let eq22_e1439_d_n7: f64 = (p.p29 * eq22_e1437_d_n7);
        let eq22_e1439_d_n8: f64 = (p.p29 * eq22_e1437_d_n8);
        let eq22_e1439_d_n9: f64 = (p.p29 * eq22_e1437_d_n9);
        let eq22_e1439_d_n10: f64 = (p.p29 * eq22_e1437_d_n10);
        let eq22_e1439_d_n11: f64 = (p.p29 * eq22_e1437_d_n11);
        let eq22_e1439_d_n12: f64 = (p.p29 * eq22_e1437_d_n12);
        let eq22_e1439_d_n13: f64 = (p.p29 * eq22_e1437_d_n13);
        let eq22_e1439_d_n14: f64 = (p.p29 * eq22_e1437_d_n14);
        let eq22_e1439_d_n15: f64 = (p.p29 * eq22_e1437_d_n15);
        let eq22_e1439_d_n16: f64 = (p.p29 * eq22_e1437_d_n16);
        let eq22_e1439_d_b0: f64 = (p.p29 * eq22_e1437_d_b0);
        let eq22_e1439_d_b1: f64 = (p.p29 * eq22_e1437_d_b1);
        let eq22_e1439_d_b2: f64 = (p.p29 * eq22_e1437_d_b2);
        let eq22_e1439_d_b3: f64 = (p.p29 * eq22_e1437_d_b3);
        let eq22_e1439_d_b4: f64 = (p.p29 * eq22_e1437_d_b4);
        let eq22_e1439_d_b5: f64 = (p.p29 * eq22_e1437_d_b5);
        let eq22_e1439_d_b6: f64 = (p.p29 * eq22_e1437_d_b6);
        let eq22_e1439_d_b7: f64 = (p.p29 * eq22_e1437_d_b7);
        let eq22_e1439_d_b8: f64 = (p.p29 * eq22_e1437_d_b8);
        let eq22_e1439_d_b9: f64 = (p.p29 * eq22_e1437_d_b9);
        let eq22_e1439_d_b10: f64 = (p.p29 * eq22_e1437_d_b10);
        let eq22_e1439_d_b11: f64 = (p.p29 * eq22_e1437_d_b11);
        let eq22_e1439_d_b12: f64 = (p.p29 * eq22_e1437_d_b12);
        let eq22_e1439_d_b13: f64 = (p.p29 * eq22_e1437_d_b13);
        let eq22_e1439_q: f64 = (p.p29 * eq22_e1438_q);
        let eq22_reactive_node_derivatives: [f64; 17] = [eq22_e1439_d_n0, eq22_e1439_d_n1, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14, eq22_e1439_d_n15, eq22_e1439_d_n16];
        let eq22_reactive_branch_derivatives: [f64; 14] = [eq22_e1439_d_b0, eq22_e1439_d_b1, eq22_e1439_d_b2, eq22_e1439_d_b3, eq22_e1439_d_b4, eq22_e1439_d_b5, eq22_e1439_d_b6, eq22_e1439_d_b7, eq22_e1439_d_b8, eq22_e1439_d_b9, eq22_e1439_d_b10, eq22_e1439_d_b11, eq22_e1439_d_b12, eq22_e1439_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-s.v[187]);
        let eq23_e1444: f64 = (eq23_e1442 * s.v[224]);
        let eq23_e1444_d_n0: f64 = (((-s.dn[187][0]) * s.v[224]) + (eq23_e1442 * s.dn[224][0]));
        let eq23_e1444_d_n1: f64 = (((-s.dn[187][1]) * s.v[224]) + (eq23_e1442 * s.dn[224][1]));
        let eq23_e1444_d_n2: f64 = (((-s.dn[187][2]) * s.v[224]) + (eq23_e1442 * s.dn[224][2]));
        let eq23_e1444_d_n3: f64 = (((-s.dn[187][3]) * s.v[224]) + (eq23_e1442 * s.dn[224][3]));
        let eq23_e1444_d_n4: f64 = (((-s.dn[187][4]) * s.v[224]) + (eq23_e1442 * s.dn[224][4]));
        let eq23_e1444_d_n5: f64 = (((-s.dn[187][5]) * s.v[224]) + (eq23_e1442 * s.dn[224][5]));
        let eq23_e1444_d_n6: f64 = (((-s.dn[187][6]) * s.v[224]) + (eq23_e1442 * s.dn[224][6]));
        let eq23_e1444_d_n7: f64 = (((-s.dn[187][7]) * s.v[224]) + (eq23_e1442 * s.dn[224][7]));
        let eq23_e1444_d_n8: f64 = (((-s.dn[187][8]) * s.v[224]) + (eq23_e1442 * s.dn[224][8]));
        let eq23_e1444_d_n9: f64 = (((-s.dn[187][9]) * s.v[224]) + (eq23_e1442 * s.dn[224][9]));
        let eq23_e1444_d_n10: f64 = (((-s.dn[187][10]) * s.v[224]) + (eq23_e1442 * s.dn[224][10]));
        let eq23_e1444_d_n11: f64 = (((-s.dn[187][11]) * s.v[224]) + (eq23_e1442 * s.dn[224][11]));
        let eq23_e1444_d_n12: f64 = (((-s.dn[187][12]) * s.v[224]) + (eq23_e1442 * s.dn[224][12]));
        let eq23_e1444_d_n13: f64 = (((-s.dn[187][13]) * s.v[224]) + (eq23_e1442 * s.dn[224][13]));
        let eq23_e1444_d_n14: f64 = (((-s.dn[187][14]) * s.v[224]) + (eq23_e1442 * s.dn[224][14]));
        let eq23_e1444_d_n15: f64 = (((-s.dn[187][15]) * s.v[224]) + (eq23_e1442 * s.dn[224][15]));
        let eq23_e1444_d_n16: f64 = (((-s.dn[187][16]) * s.v[224]) + (eq23_e1442 * s.dn[224][16]));
        let eq23_e1444_d_b0: f64 = (((-s.db[187][0]) * s.v[224]) + (eq23_e1442 * s.db[224][0]));
        let eq23_e1444_d_b1: f64 = (((-s.db[187][1]) * s.v[224]) + (eq23_e1442 * s.db[224][1]));
        let eq23_e1444_d_b2: f64 = (((-s.db[187][2]) * s.v[224]) + (eq23_e1442 * s.db[224][2]));
        let eq23_e1444_d_b3: f64 = (((-s.db[187][3]) * s.v[224]) + (eq23_e1442 * s.db[224][3]));
        let eq23_e1444_d_b4: f64 = (((-s.db[187][4]) * s.v[224]) + (eq23_e1442 * s.db[224][4]));
        let eq23_e1444_d_b5: f64 = (((-s.db[187][5]) * s.v[224]) + (eq23_e1442 * s.db[224][5]));
        let eq23_e1444_d_b6: f64 = (((-s.db[187][6]) * s.v[224]) + (eq23_e1442 * s.db[224][6]));
        let eq23_e1444_d_b7: f64 = (((-s.db[187][7]) * s.v[224]) + (eq23_e1442 * s.db[224][7]));
        let eq23_e1444_d_b8: f64 = (((-s.db[187][8]) * s.v[224]) + (eq23_e1442 * s.db[224][8]));
        let eq23_e1444_d_b9: f64 = (((-s.db[187][9]) * s.v[224]) + (eq23_e1442 * s.db[224][9]));
        let eq23_e1444_d_b10: f64 = (((-s.db[187][10]) * s.v[224]) + (eq23_e1442 * s.db[224][10]));
        let eq23_e1444_d_b11: f64 = (((-s.db[187][11]) * s.v[224]) + (eq23_e1442 * s.db[224][11]));
        let eq23_e1444_d_b12: f64 = (((-s.db[187][12]) * s.v[224]) + (eq23_e1442 * s.db[224][12]));
        let eq23_e1444_d_b13: f64 = (((-s.db[187][13]) * s.v[224]) + (eq23_e1442 * s.db[224][13]));
        let eq23_e1445_q: f64 = eq23_e1444;
        let eq23_e1446: f64 = (p.p29 * eq23_e1444);
        let eq23_e1446_d_n0: f64 = (p.p29 * eq23_e1444_d_n0);
        let eq23_e1446_d_n1: f64 = (p.p29 * eq23_e1444_d_n1);
        let eq23_e1446_d_n2: f64 = (p.p29 * eq23_e1444_d_n2);
        let eq23_e1446_d_n3: f64 = (p.p29 * eq23_e1444_d_n3);
        let eq23_e1446_d_n4: f64 = (p.p29 * eq23_e1444_d_n4);
        let eq23_e1446_d_n5: f64 = (p.p29 * eq23_e1444_d_n5);
        let eq23_e1446_d_n6: f64 = (p.p29 * eq23_e1444_d_n6);
        let eq23_e1446_d_n7: f64 = (p.p29 * eq23_e1444_d_n7);
        let eq23_e1446_d_n8: f64 = (p.p29 * eq23_e1444_d_n8);
        let eq23_e1446_d_n9: f64 = (p.p29 * eq23_e1444_d_n9);
        let eq23_e1446_d_n10: f64 = (p.p29 * eq23_e1444_d_n10);
        let eq23_e1446_d_n11: f64 = (p.p29 * eq23_e1444_d_n11);
        let eq23_e1446_d_n12: f64 = (p.p29 * eq23_e1444_d_n12);
        let eq23_e1446_d_n13: f64 = (p.p29 * eq23_e1444_d_n13);
        let eq23_e1446_d_n14: f64 = (p.p29 * eq23_e1444_d_n14);
        let eq23_e1446_d_n15: f64 = (p.p29 * eq23_e1444_d_n15);
        let eq23_e1446_d_n16: f64 = (p.p29 * eq23_e1444_d_n16);
        let eq23_e1446_d_b0: f64 = (p.p29 * eq23_e1444_d_b0);
        let eq23_e1446_d_b1: f64 = (p.p29 * eq23_e1444_d_b1);
        let eq23_e1446_d_b2: f64 = (p.p29 * eq23_e1444_d_b2);
        let eq23_e1446_d_b3: f64 = (p.p29 * eq23_e1444_d_b3);
        let eq23_e1446_d_b4: f64 = (p.p29 * eq23_e1444_d_b4);
        let eq23_e1446_d_b5: f64 = (p.p29 * eq23_e1444_d_b5);
        let eq23_e1446_d_b6: f64 = (p.p29 * eq23_e1444_d_b6);
        let eq23_e1446_d_b7: f64 = (p.p29 * eq23_e1444_d_b7);
        let eq23_e1446_d_b8: f64 = (p.p29 * eq23_e1444_d_b8);
        let eq23_e1446_d_b9: f64 = (p.p29 * eq23_e1444_d_b9);
        let eq23_e1446_d_b10: f64 = (p.p29 * eq23_e1444_d_b10);
        let eq23_e1446_d_b11: f64 = (p.p29 * eq23_e1444_d_b11);
        let eq23_e1446_d_b12: f64 = (p.p29 * eq23_e1444_d_b12);
        let eq23_e1446_d_b13: f64 = (p.p29 * eq23_e1444_d_b13);
        let eq23_e1446_q: f64 = (p.p29 * eq23_e1445_q);
        let eq23_reactive_node_derivatives: [f64; 17] = [eq23_e1446_d_n0, eq23_e1446_d_n1, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14, eq23_e1446_d_n15, eq23_e1446_d_n16];
        let eq23_reactive_branch_derivatives: [f64; 14] = [eq23_e1446_d_b0, eq23_e1446_d_b1, eq23_e1446_d_b2, eq23_e1446_d_b3, eq23_e1446_d_b4, eq23_e1446_d_b5, eq23_e1446_d_b6, eq23_e1446_d_b7, eq23_e1446_d_b8, eq23_e1446_d_b9, eq23_e1446_d_b10, eq23_e1446_d_b11, eq23_e1446_d_b12, eq23_e1446_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq24_e1449: f64 = (-s.v[187]);
        let eq24_e1451: f64 = (eq24_e1449 * s.v[221]);
        let eq24_e1451_d_n0: f64 = (((-s.dn[187][0]) * s.v[221]) + (eq24_e1449 * s.dn[221][0]));
        let eq24_e1451_d_n1: f64 = (((-s.dn[187][1]) * s.v[221]) + (eq24_e1449 * s.dn[221][1]));
        let eq24_e1451_d_n2: f64 = (((-s.dn[187][2]) * s.v[221]) + (eq24_e1449 * s.dn[221][2]));
        let eq24_e1451_d_n3: f64 = (((-s.dn[187][3]) * s.v[221]) + (eq24_e1449 * s.dn[221][3]));
        let eq24_e1451_d_n4: f64 = (((-s.dn[187][4]) * s.v[221]) + (eq24_e1449 * s.dn[221][4]));
        let eq24_e1451_d_n5: f64 = (((-s.dn[187][5]) * s.v[221]) + (eq24_e1449 * s.dn[221][5]));
        let eq24_e1451_d_n6: f64 = (((-s.dn[187][6]) * s.v[221]) + (eq24_e1449 * s.dn[221][6]));
        let eq24_e1451_d_n7: f64 = (((-s.dn[187][7]) * s.v[221]) + (eq24_e1449 * s.dn[221][7]));
        let eq24_e1451_d_n8: f64 = (((-s.dn[187][8]) * s.v[221]) + (eq24_e1449 * s.dn[221][8]));
        let eq24_e1451_d_n9: f64 = (((-s.dn[187][9]) * s.v[221]) + (eq24_e1449 * s.dn[221][9]));
        let eq24_e1451_d_n10: f64 = (((-s.dn[187][10]) * s.v[221]) + (eq24_e1449 * s.dn[221][10]));
        let eq24_e1451_d_n11: f64 = (((-s.dn[187][11]) * s.v[221]) + (eq24_e1449 * s.dn[221][11]));
        let eq24_e1451_d_n12: f64 = (((-s.dn[187][12]) * s.v[221]) + (eq24_e1449 * s.dn[221][12]));
        let eq24_e1451_d_n13: f64 = (((-s.dn[187][13]) * s.v[221]) + (eq24_e1449 * s.dn[221][13]));
        let eq24_e1451_d_n14: f64 = (((-s.dn[187][14]) * s.v[221]) + (eq24_e1449 * s.dn[221][14]));
        let eq24_e1451_d_n15: f64 = (((-s.dn[187][15]) * s.v[221]) + (eq24_e1449 * s.dn[221][15]));
        let eq24_e1451_d_n16: f64 = (((-s.dn[187][16]) * s.v[221]) + (eq24_e1449 * s.dn[221][16]));
        let eq24_e1451_d_b0: f64 = (((-s.db[187][0]) * s.v[221]) + (eq24_e1449 * s.db[221][0]));
        let eq24_e1451_d_b1: f64 = (((-s.db[187][1]) * s.v[221]) + (eq24_e1449 * s.db[221][1]));
        let eq24_e1451_d_b2: f64 = (((-s.db[187][2]) * s.v[221]) + (eq24_e1449 * s.db[221][2]));
        let eq24_e1451_d_b3: f64 = (((-s.db[187][3]) * s.v[221]) + (eq24_e1449 * s.db[221][3]));
        let eq24_e1451_d_b4: f64 = (((-s.db[187][4]) * s.v[221]) + (eq24_e1449 * s.db[221][4]));
        let eq24_e1451_d_b5: f64 = (((-s.db[187][5]) * s.v[221]) + (eq24_e1449 * s.db[221][5]));
        let eq24_e1451_d_b6: f64 = (((-s.db[187][6]) * s.v[221]) + (eq24_e1449 * s.db[221][6]));
        let eq24_e1451_d_b7: f64 = (((-s.db[187][7]) * s.v[221]) + (eq24_e1449 * s.db[221][7]));
        let eq24_e1451_d_b8: f64 = (((-s.db[187][8]) * s.v[221]) + (eq24_e1449 * s.db[221][8]));
        let eq24_e1451_d_b9: f64 = (((-s.db[187][9]) * s.v[221]) + (eq24_e1449 * s.db[221][9]));
        let eq24_e1451_d_b10: f64 = (((-s.db[187][10]) * s.v[221]) + (eq24_e1449 * s.db[221][10]));
        let eq24_e1451_d_b11: f64 = (((-s.db[187][11]) * s.v[221]) + (eq24_e1449 * s.db[221][11]));
        let eq24_e1451_d_b12: f64 = (((-s.db[187][12]) * s.v[221]) + (eq24_e1449 * s.db[221][12]));
        let eq24_e1451_d_b13: f64 = (((-s.db[187][13]) * s.v[221]) + (eq24_e1449 * s.db[221][13]));
        let eq24_e1452_q: f64 = eq24_e1451;
        let eq24_e1453: f64 = (p.p29 * eq24_e1451);
        let eq24_e1453_d_n0: f64 = (p.p29 * eq24_e1451_d_n0);
        let eq24_e1453_d_n1: f64 = (p.p29 * eq24_e1451_d_n1);
        let eq24_e1453_d_n2: f64 = (p.p29 * eq24_e1451_d_n2);
        let eq24_e1453_d_n3: f64 = (p.p29 * eq24_e1451_d_n3);
        let eq24_e1453_d_n4: f64 = (p.p29 * eq24_e1451_d_n4);
        let eq24_e1453_d_n5: f64 = (p.p29 * eq24_e1451_d_n5);
        let eq24_e1453_d_n6: f64 = (p.p29 * eq24_e1451_d_n6);
        let eq24_e1453_d_n7: f64 = (p.p29 * eq24_e1451_d_n7);
        let eq24_e1453_d_n8: f64 = (p.p29 * eq24_e1451_d_n8);
        let eq24_e1453_d_n9: f64 = (p.p29 * eq24_e1451_d_n9);
        let eq24_e1453_d_n10: f64 = (p.p29 * eq24_e1451_d_n10);
        let eq24_e1453_d_n11: f64 = (p.p29 * eq24_e1451_d_n11);
        let eq24_e1453_d_n12: f64 = (p.p29 * eq24_e1451_d_n12);
        let eq24_e1453_d_n13: f64 = (p.p29 * eq24_e1451_d_n13);
        let eq24_e1453_d_n14: f64 = (p.p29 * eq24_e1451_d_n14);
        let eq24_e1453_d_n15: f64 = (p.p29 * eq24_e1451_d_n15);
        let eq24_e1453_d_n16: f64 = (p.p29 * eq24_e1451_d_n16);
        let eq24_e1453_d_b0: f64 = (p.p29 * eq24_e1451_d_b0);
        let eq24_e1453_d_b1: f64 = (p.p29 * eq24_e1451_d_b1);
        let eq24_e1453_d_b2: f64 = (p.p29 * eq24_e1451_d_b2);
        let eq24_e1453_d_b3: f64 = (p.p29 * eq24_e1451_d_b3);
        let eq24_e1453_d_b4: f64 = (p.p29 * eq24_e1451_d_b4);
        let eq24_e1453_d_b5: f64 = (p.p29 * eq24_e1451_d_b5);
        let eq24_e1453_d_b6: f64 = (p.p29 * eq24_e1451_d_b6);
        let eq24_e1453_d_b7: f64 = (p.p29 * eq24_e1451_d_b7);
        let eq24_e1453_d_b8: f64 = (p.p29 * eq24_e1451_d_b8);
        let eq24_e1453_d_b9: f64 = (p.p29 * eq24_e1451_d_b9);
        let eq24_e1453_d_b10: f64 = (p.p29 * eq24_e1451_d_b10);
        let eq24_e1453_d_b11: f64 = (p.p29 * eq24_e1451_d_b11);
        let eq24_e1453_d_b12: f64 = (p.p29 * eq24_e1451_d_b12);
        let eq24_e1453_d_b13: f64 = (p.p29 * eq24_e1451_d_b13);
        let eq24_e1453_q: f64 = (p.p29 * eq24_e1452_q);
        let eq24_reactive_node_derivatives: [f64; 17] = [eq24_e1453_d_n0, eq24_e1453_d_n1, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14, eq24_e1453_d_n15, eq24_e1453_d_n16];
        let eq24_reactive_branch_derivatives: [f64; 14] = [eq24_e1453_d_b0, eq24_e1453_d_b1, eq24_e1453_d_b2, eq24_e1453_d_b3, eq24_e1453_d_b4, eq24_e1453_d_b5, eq24_e1453_d_b6, eq24_e1453_d_b7, eq24_e1453_d_b8, eq24_e1453_d_b9, eq24_e1453_d_b10, eq24_e1453_d_b11, eq24_e1453_d_b12, eq24_e1453_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n1, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_d_n15, eq55_e1708_d_n16, eq55_e1708_d_b0, eq55_e1708_d_b1, eq55_e1708_d_b2, eq55_e1708_d_b3, eq55_e1708_d_b4, eq55_e1708_d_b5, eq55_e1708_d_b6, eq55_e1708_d_b7, eq55_e1708_d_b8, eq55_e1708_d_b9, eq55_e1708_d_b10, eq55_e1708_d_b11, eq55_e1708_d_b12, eq55_e1708_d_b13, eq55_e1708_q, eq55_e1708_q_d_n0, eq55_e1708_q_d_n1, eq55_e1708_q_d_n2, eq55_e1708_q_d_n3, eq55_e1708_q_d_n4, eq55_e1708_q_d_n5, eq55_e1708_q_d_n6, eq55_e1708_q_d_n7, eq55_e1708_q_d_n8, eq55_e1708_q_d_n9, eq55_e1708_q_d_n10, eq55_e1708_q_d_n11, eq55_e1708_q_d_n12, eq55_e1708_q_d_n13, eq55_e1708_q_d_n14, eq55_e1708_q_d_n15, eq55_e1708_q_d_n16, eq55_e1708_q_d_b0, eq55_e1708_q_d_b1, eq55_e1708_q_d_b2, eq55_e1708_q_d_b3, eq55_e1708_q_d_b4, eq55_e1708_q_d_b5, eq55_e1708_q_d_b6, eq55_e1708_q_d_b7, eq55_e1708_q_d_b8, eq55_e1708_q_d_b9, eq55_e1708_q_d_b10, eq55_e1708_q_d_b11, eq55_e1708_q_d_b12, eq55_e1708_q_d_b13,) = {
    if s.b[1621] {
        let eq55_e1699: f64 = (s.v[390] * s.v[747]);
        let eq55_e1699_d_n0: f64 = ((s.dn[390][0] * s.v[747]) + (s.v[390] * s.dn[747][0]));
        let eq55_e1699_d_n1: f64 = ((s.dn[390][1] * s.v[747]) + (s.v[390] * s.dn[747][1]));
        let eq55_e1699_d_n2: f64 = ((s.dn[390][2] * s.v[747]) + (s.v[390] * s.dn[747][2]));
        let eq55_e1699_d_n3: f64 = ((s.dn[390][3] * s.v[747]) + (s.v[390] * s.dn[747][3]));
        let eq55_e1699_d_n4: f64 = ((s.dn[390][4] * s.v[747]) + (s.v[390] * s.dn[747][4]));
        let eq55_e1699_d_n5: f64 = ((s.dn[390][5] * s.v[747]) + (s.v[390] * s.dn[747][5]));
        let eq55_e1699_d_n6: f64 = ((s.dn[390][6] * s.v[747]) + (s.v[390] * s.dn[747][6]));
        let eq55_e1699_d_n7: f64 = ((s.dn[390][7] * s.v[747]) + (s.v[390] * s.dn[747][7]));
        let eq55_e1699_d_n8: f64 = ((s.dn[390][8] * s.v[747]) + (s.v[390] * s.dn[747][8]));
        let eq55_e1699_d_n9: f64 = ((s.dn[390][9] * s.v[747]) + (s.v[390] * s.dn[747][9]));
        let eq55_e1699_d_n10: f64 = ((s.dn[390][10] * s.v[747]) + (s.v[390] * s.dn[747][10]));
        let eq55_e1699_d_n11: f64 = ((s.dn[390][11] * s.v[747]) + (s.v[390] * s.dn[747][11]));
        let eq55_e1699_d_n12: f64 = ((s.dn[390][12] * s.v[747]) + (s.v[390] * s.dn[747][12]));
        let eq55_e1699_d_n13: f64 = ((s.dn[390][13] * s.v[747]) + (s.v[390] * s.dn[747][13]));
        let eq55_e1699_d_n14: f64 = ((s.dn[390][14] * s.v[747]) + (s.v[390] * s.dn[747][14]));
        let eq55_e1699_d_n15: f64 = ((s.dn[390][15] * s.v[747]) + (s.v[390] * s.dn[747][15]));
        let eq55_e1699_d_n16: f64 = ((s.dn[390][16] * s.v[747]) + (s.v[390] * s.dn[747][16]));
        let eq55_e1699_d_b0: f64 = ((s.db[390][0] * s.v[747]) + (s.v[390] * s.db[747][0]));
        let eq55_e1699_d_b1: f64 = ((s.db[390][1] * s.v[747]) + (s.v[390] * s.db[747][1]));
        let eq55_e1699_d_b2: f64 = ((s.db[390][2] * s.v[747]) + (s.v[390] * s.db[747][2]));
        let eq55_e1699_d_b3: f64 = ((s.db[390][3] * s.v[747]) + (s.v[390] * s.db[747][3]));
        let eq55_e1699_d_b4: f64 = ((s.db[390][4] * s.v[747]) + (s.v[390] * s.db[747][4]));
        let eq55_e1699_d_b5: f64 = ((s.db[390][5] * s.v[747]) + (s.v[390] * s.db[747][5]));
        let eq55_e1699_d_b6: f64 = ((s.db[390][6] * s.v[747]) + (s.v[390] * s.db[747][6]));
        let eq55_e1699_d_b7: f64 = ((s.db[390][7] * s.v[747]) + (s.v[390] * s.db[747][7]));
        let eq55_e1699_d_b8: f64 = ((s.db[390][8] * s.v[747]) + (s.v[390] * s.db[747][8]));
        let eq55_e1699_d_b9: f64 = ((s.db[390][9] * s.v[747]) + (s.v[390] * s.db[747][9]));
        let eq55_e1699_d_b10: f64 = ((s.db[390][10] * s.v[747]) + (s.v[390] * s.db[747][10]));
        let eq55_e1699_d_b11: f64 = ((s.db[390][11] * s.v[747]) + (s.v[390] * s.db[747][11]));
        let eq55_e1699_d_b12: f64 = ((s.db[390][12] * s.v[747]) + (s.v[390] * s.db[747][12]));
        let eq55_e1699_d_b13: f64 = ((s.db[390][13] * s.v[747]) + (s.v[390] * s.db[747][13]));
        let eq55_e1702: f64 = (s.v[390] * s.v[748]);
        let eq55_e1702_d_n0: f64 = ((s.dn[390][0] * s.v[748]) + (s.v[390] * s.dn[748][0]));
        let eq55_e1702_d_n1: f64 = ((s.dn[390][1] * s.v[748]) + (s.v[390] * s.dn[748][1]));
        let eq55_e1702_d_n2: f64 = ((s.dn[390][2] * s.v[748]) + (s.v[390] * s.dn[748][2]));
        let eq55_e1702_d_n3: f64 = ((s.dn[390][3] * s.v[748]) + (s.v[390] * s.dn[748][3]));
        let eq55_e1702_d_n4: f64 = ((s.dn[390][4] * s.v[748]) + (s.v[390] * s.dn[748][4]));
        let eq55_e1702_d_n5: f64 = ((s.dn[390][5] * s.v[748]) + (s.v[390] * s.dn[748][5]));
        let eq55_e1702_d_n6: f64 = ((s.dn[390][6] * s.v[748]) + (s.v[390] * s.dn[748][6]));
        let eq55_e1702_d_n7: f64 = ((s.dn[390][7] * s.v[748]) + (s.v[390] * s.dn[748][7]));
        let eq55_e1702_d_n8: f64 = ((s.dn[390][8] * s.v[748]) + (s.v[390] * s.dn[748][8]));
        let eq55_e1702_d_n9: f64 = ((s.dn[390][9] * s.v[748]) + (s.v[390] * s.dn[748][9]));
        let eq55_e1702_d_n10: f64 = ((s.dn[390][10] * s.v[748]) + (s.v[390] * s.dn[748][10]));
        let eq55_e1702_d_n11: f64 = ((s.dn[390][11] * s.v[748]) + (s.v[390] * s.dn[748][11]));
        let eq55_e1702_d_n12: f64 = ((s.dn[390][12] * s.v[748]) + (s.v[390] * s.dn[748][12]));
        let eq55_e1702_d_n13: f64 = ((s.dn[390][13] * s.v[748]) + (s.v[390] * s.dn[748][13]));
        let eq55_e1702_d_n14: f64 = ((s.dn[390][14] * s.v[748]) + (s.v[390] * s.dn[748][14]));
        let eq55_e1702_d_n15: f64 = ((s.dn[390][15] * s.v[748]) + (s.v[390] * s.dn[748][15]));
        let eq55_e1702_d_n16: f64 = ((s.dn[390][16] * s.v[748]) + (s.v[390] * s.dn[748][16]));
        let eq55_e1702_d_b0: f64 = ((s.db[390][0] * s.v[748]) + (s.v[390] * s.db[748][0]));
        let eq55_e1702_d_b1: f64 = ((s.db[390][1] * s.v[748]) + (s.v[390] * s.db[748][1]));
        let eq55_e1702_d_b2: f64 = ((s.db[390][2] * s.v[748]) + (s.v[390] * s.db[748][2]));
        let eq55_e1702_d_b3: f64 = ((s.db[390][3] * s.v[748]) + (s.v[390] * s.db[748][3]));
        let eq55_e1702_d_b4: f64 = ((s.db[390][4] * s.v[748]) + (s.v[390] * s.db[748][4]));
        let eq55_e1702_d_b5: f64 = ((s.db[390][5] * s.v[748]) + (s.v[390] * s.db[748][5]));
        let eq55_e1702_d_b6: f64 = ((s.db[390][6] * s.v[748]) + (s.v[390] * s.db[748][6]));
        let eq55_e1702_d_b7: f64 = ((s.db[390][7] * s.v[748]) + (s.v[390] * s.db[748][7]));
        let eq55_e1702_d_b8: f64 = ((s.db[390][8] * s.v[748]) + (s.v[390] * s.db[748][8]));
        let eq55_e1702_d_b9: f64 = ((s.db[390][9] * s.v[748]) + (s.v[390] * s.db[748][9]));
        let eq55_e1702_d_b10: f64 = ((s.db[390][10] * s.v[748]) + (s.v[390] * s.db[748][10]));
        let eq55_e1702_d_b11: f64 = ((s.db[390][11] * s.v[748]) + (s.v[390] * s.db[748][11]));
        let eq55_e1702_d_b12: f64 = ((s.db[390][12] * s.v[748]) + (s.v[390] * s.db[748][12]));
        let eq55_e1702_d_b13: f64 = ((s.db[390][13] * s.v[748]) + (s.v[390] * s.db[748][13]));
        let eq55_e1703_q: f64 = eq55_e1702;
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1702);
        let eq55_e1704_d_n0: f64 = (eq55_e1699_d_n0 + eq55_e1702_d_n0);
        let eq55_e1704_d_n1: f64 = (eq55_e1699_d_n1 + eq55_e1702_d_n1);
        let eq55_e1704_d_n2: f64 = (eq55_e1699_d_n2 + eq55_e1702_d_n2);
        let eq55_e1704_d_n3: f64 = (eq55_e1699_d_n3 + eq55_e1702_d_n3);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + eq55_e1702_d_n4);
        let eq55_e1704_d_n5: f64 = (eq55_e1699_d_n5 + eq55_e1702_d_n5);
        let eq55_e1704_d_n6: f64 = (eq55_e1699_d_n6 + eq55_e1702_d_n6);
        let eq55_e1704_d_n7: f64 = (eq55_e1699_d_n7 + eq55_e1702_d_n7);
        let eq55_e1704_d_n8: f64 = (eq55_e1699_d_n8 + eq55_e1702_d_n8);
        let eq55_e1704_d_n9: f64 = (eq55_e1699_d_n9 + eq55_e1702_d_n9);
        let eq55_e1704_d_n10: f64 = (eq55_e1699_d_n10 + eq55_e1702_d_n10);
        let eq55_e1704_d_n11: f64 = (eq55_e1699_d_n11 + eq55_e1702_d_n11);
        let eq55_e1704_d_n12: f64 = (eq55_e1699_d_n12 + eq55_e1702_d_n12);
        let eq55_e1704_d_n13: f64 = (eq55_e1699_d_n13 + eq55_e1702_d_n13);
        let eq55_e1704_d_n14: f64 = (eq55_e1699_d_n14 + eq55_e1702_d_n14);
        let eq55_e1704_d_n15: f64 = (eq55_e1699_d_n15 + eq55_e1702_d_n15);
        let eq55_e1704_d_n16: f64 = (eq55_e1699_d_n16 + eq55_e1702_d_n16);
        let eq55_e1704_d_b0: f64 = (eq55_e1699_d_b0 + eq55_e1702_d_b0);
        let eq55_e1704_d_b1: f64 = (eq55_e1699_d_b1 + eq55_e1702_d_b1);
        let eq55_e1704_d_b2: f64 = (eq55_e1699_d_b2 + eq55_e1702_d_b2);
        let eq55_e1704_d_b3: f64 = (eq55_e1699_d_b3 + eq55_e1702_d_b3);
        let eq55_e1704_d_b4: f64 = (eq55_e1699_d_b4 + eq55_e1702_d_b4);
        let eq55_e1704_d_b5: f64 = (eq55_e1699_d_b5 + eq55_e1702_d_b5);
        let eq55_e1704_d_b6: f64 = (eq55_e1699_d_b6 + eq55_e1702_d_b6);
        let eq55_e1704_d_b7: f64 = (eq55_e1699_d_b7 + eq55_e1702_d_b7);
        let eq55_e1704_d_b8: f64 = (eq55_e1699_d_b8 + eq55_e1702_d_b8);
        let eq55_e1704_d_b9: f64 = (eq55_e1699_d_b9 + eq55_e1702_d_b9);
        let eq55_e1704_d_b10: f64 = (eq55_e1699_d_b10 + eq55_e1702_d_b10);
        let eq55_e1704_d_b11: f64 = (eq55_e1699_d_b11 + eq55_e1702_d_b11);
        let eq55_e1704_d_b12: f64 = (eq55_e1699_d_b12 + eq55_e1702_d_b12);
        let eq55_e1704_d_b13: f64 = (eq55_e1699_d_b13 + eq55_e1702_d_b13);
        let eq55_e1704_q: f64 = eq55_e1703_q;
        let eq55_e1706: f64 = (eq55_e1704 - s.v[749]);
        let eq55_e1706_d_n0: f64 = (eq55_e1704_d_n0 - s.dn[749][0]);
        let eq55_e1706_d_n1: f64 = (eq55_e1704_d_n1 - s.dn[749][1]);
        let eq55_e1706_d_n2: f64 = (eq55_e1704_d_n2 - s.dn[749][2]);
        let eq55_e1706_d_n3: f64 = (eq55_e1704_d_n3 - s.dn[749][3]);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - s.dn[749][4]);
        let eq55_e1706_d_n5: f64 = (eq55_e1704_d_n5 - s.dn[749][5]);
        let eq55_e1706_d_n6: f64 = (eq55_e1704_d_n6 - s.dn[749][6]);
        let eq55_e1706_d_n7: f64 = (eq55_e1704_d_n7 - s.dn[749][7]);
        let eq55_e1706_d_n8: f64 = (eq55_e1704_d_n8 - s.dn[749][8]);
        let eq55_e1706_d_n9: f64 = (eq55_e1704_d_n9 - s.dn[749][9]);
        let eq55_e1706_d_n10: f64 = (eq55_e1704_d_n10 - s.dn[749][10]);
        let eq55_e1706_d_n11: f64 = (eq55_e1704_d_n11 - s.dn[749][11]);
        let eq55_e1706_d_n12: f64 = (eq55_e1704_d_n12 - s.dn[749][12]);
        let eq55_e1706_d_n13: f64 = (eq55_e1704_d_n13 - s.dn[749][13]);
        let eq55_e1706_d_n14: f64 = (eq55_e1704_d_n14 - s.dn[749][14]);
        let eq55_e1706_d_n15: f64 = (eq55_e1704_d_n15 - s.dn[749][15]);
        let eq55_e1706_d_n16: f64 = (eq55_e1704_d_n16 - s.dn[749][16]);
        let eq55_e1706_d_b0: f64 = (eq55_e1704_d_b0 - s.db[749][0]);
        let eq55_e1706_d_b1: f64 = (eq55_e1704_d_b1 - s.db[749][1]);
        let eq55_e1706_d_b2: f64 = (eq55_e1704_d_b2 - s.db[749][2]);
        let eq55_e1706_d_b3: f64 = (eq55_e1704_d_b3 - s.db[749][3]);
        let eq55_e1706_d_b4: f64 = (eq55_e1704_d_b4 - s.db[749][4]);
        let eq55_e1706_d_b5: f64 = (eq55_e1704_d_b5 - s.db[749][5]);
        let eq55_e1706_d_b6: f64 = (eq55_e1704_d_b6 - s.db[749][6]);
        let eq55_e1706_d_b7: f64 = (eq55_e1704_d_b7 - s.db[749][7]);
        let eq55_e1706_d_b8: f64 = (eq55_e1704_d_b8 - s.db[749][8]);
        let eq55_e1706_d_b9: f64 = (eq55_e1704_d_b9 - s.db[749][9]);
        let eq55_e1706_d_b10: f64 = (eq55_e1704_d_b10 - s.db[749][10]);
        let eq55_e1706_d_b11: f64 = (eq55_e1704_d_b11 - s.db[749][11]);
        let eq55_e1706_d_b12: f64 = (eq55_e1704_d_b12 - s.db[749][12]);
        let eq55_e1706_d_b13: f64 = (eq55_e1704_d_b13 - s.db[749][13]);
        let eq55_e1706_q: f64 = eq55_e1704_q;
        (eq55_e1706, eq55_e1706_d_n0, eq55_e1706_d_n1, eq55_e1706_d_n2, eq55_e1706_d_n3, eq55_e1706_d_n4, eq55_e1706_d_n5, eq55_e1706_d_n6, eq55_e1706_d_n7, eq55_e1706_d_n8, eq55_e1706_d_n9, eq55_e1706_d_n10, eq55_e1706_d_n11, eq55_e1706_d_n12, eq55_e1706_d_n13, eq55_e1706_d_n14, eq55_e1706_d_n15, eq55_e1706_d_n16, eq55_e1706_d_b0, eq55_e1706_d_b1, eq55_e1706_d_b2, eq55_e1706_d_b3, eq55_e1706_d_b4, eq55_e1706_d_b5, eq55_e1706_d_b6, eq55_e1706_d_b7, eq55_e1706_d_b8, eq55_e1706_d_b9, eq55_e1706_d_b10, eq55_e1706_d_b11, eq55_e1706_d_b12, eq55_e1706_d_b13, eq55_e1706_q, eq55_e1702_d_n0, eq55_e1702_d_n1, eq55_e1702_d_n2, eq55_e1702_d_n3, eq55_e1702_d_n4, eq55_e1702_d_n5, eq55_e1702_d_n6, eq55_e1702_d_n7, eq55_e1702_d_n8, eq55_e1702_d_n9, eq55_e1702_d_n10, eq55_e1702_d_n11, eq55_e1702_d_n12, eq55_e1702_d_n13, eq55_e1702_d_n14, eq55_e1702_d_n15, eq55_e1702_d_n16, eq55_e1702_d_b0, eq55_e1702_d_b1, eq55_e1702_d_b2, eq55_e1702_d_b3, eq55_e1702_d_b4, eq55_e1702_d_b5, eq55_e1702_d_b6, eq55_e1702_d_b7, eq55_e1702_d_b8, eq55_e1702_d_b9, eq55_e1702_d_b10, eq55_e1702_d_b11, eq55_e1702_d_b12, eq55_e1702_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e1708_q_d_n0, eq55_e1708_q_d_n1, eq55_e1708_q_d_n2, eq55_e1708_q_d_n3, eq55_e1708_q_d_n4, eq55_e1708_q_d_n5, eq55_e1708_q_d_n6, eq55_e1708_q_d_n7, eq55_e1708_q_d_n8, eq55_e1708_q_d_n9, eq55_e1708_q_d_n10, eq55_e1708_q_d_n11, eq55_e1708_q_d_n12, eq55_e1708_q_d_n13, eq55_e1708_q_d_n14, eq55_e1708_q_d_n15, eq55_e1708_q_d_n16];
        let eq55_reactive_branch_derivatives: [f64; 14] = [eq55_e1708_q_d_b0, eq55_e1708_q_d_b1, eq55_e1708_q_d_b2, eq55_e1708_q_d_b3, eq55_e1708_q_d_b4, eq55_e1708_q_d_b5, eq55_e1708_q_d_b6, eq55_e1708_q_d_b7, eq55_e1708_q_d_b8, eq55_e1708_q_d_b9, eq55_e1708_q_d_b10, eq55_e1708_q_d_b11, eq55_e1708_q_d_b12, eq55_e1708_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n1, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_d_n15, eq71_e1841_d_n16, eq71_e1841_d_b0, eq71_e1841_d_b1, eq71_e1841_d_b2, eq71_e1841_d_b3, eq71_e1841_d_b4, eq71_e1841_d_b5, eq71_e1841_d_b6, eq71_e1841_d_b7, eq71_e1841_d_b8, eq71_e1841_d_b9, eq71_e1841_d_b10, eq71_e1841_d_b11, eq71_e1841_d_b12, eq71_e1841_d_b13, eq71_e1841_q, eq71_e1841_q_d_n0, eq71_e1841_q_d_n1, eq71_e1841_q_d_n2, eq71_e1841_q_d_n3, eq71_e1841_q_d_n4, eq71_e1841_q_d_n5, eq71_e1841_q_d_n6, eq71_e1841_q_d_n7, eq71_e1841_q_d_n8, eq71_e1841_q_d_n9, eq71_e1841_q_d_n10, eq71_e1841_q_d_n11, eq71_e1841_q_d_n12, eq71_e1841_q_d_n13, eq71_e1841_q_d_n14, eq71_e1841_q_d_n15, eq71_e1841_q_d_n16, eq71_e1841_q_d_b0, eq71_e1841_q_d_b1, eq71_e1841_q_d_b2, eq71_e1841_q_d_b3, eq71_e1841_q_d_b4, eq71_e1841_q_d_b5, eq71_e1841_q_d_b6, eq71_e1841_q_d_b7, eq71_e1841_q_d_b8, eq71_e1841_q_d_b9, eq71_e1841_q_d_b10, eq71_e1841_q_d_b11, eq71_e1841_q_d_b12, eq71_e1841_q_d_b13,) = {
    if s.b[1627] {
        let eq71_e1837: f64 = (p.p29 * s.v[330]);
        let eq71_e1838_q: f64 = eq71_e1837;
        let eq71_e1839: f64 = (s.v[187] * eq71_e1837);
        let eq71_e1839_d_n0: f64 = ((s.dn[187][0] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][0])));
        let eq71_e1839_d_n1: f64 = ((s.dn[187][1] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][1])));
        let eq71_e1839_d_n2: f64 = ((s.dn[187][2] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][2])));
        let eq71_e1839_d_n3: f64 = ((s.dn[187][3] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][3])));
        let eq71_e1839_d_n4: f64 = ((s.dn[187][4] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][4])));
        let eq71_e1839_d_n5: f64 = ((s.dn[187][5] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][5])));
        let eq71_e1839_d_n6: f64 = ((s.dn[187][6] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][6])));
        let eq71_e1839_d_n7: f64 = ((s.dn[187][7] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][7])));
        let eq71_e1839_d_n8: f64 = ((s.dn[187][8] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][8])));
        let eq71_e1839_d_n9: f64 = ((s.dn[187][9] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][9])));
        let eq71_e1839_d_n10: f64 = ((s.dn[187][10] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][10])));
        let eq71_e1839_d_n11: f64 = ((s.dn[187][11] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][11])));
        let eq71_e1839_d_n12: f64 = ((s.dn[187][12] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][12])));
        let eq71_e1839_d_n13: f64 = ((s.dn[187][13] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][13])));
        let eq71_e1839_d_n14: f64 = ((s.dn[187][14] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][14])));
        let eq71_e1839_d_n15: f64 = ((s.dn[187][15] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][15])));
        let eq71_e1839_d_n16: f64 = ((s.dn[187][16] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][16])));
        let eq71_e1839_d_b0: f64 = ((s.db[187][0] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][0])));
        let eq71_e1839_d_b1: f64 = ((s.db[187][1] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][1])));
        let eq71_e1839_d_b2: f64 = ((s.db[187][2] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][2])));
        let eq71_e1839_d_b3: f64 = ((s.db[187][3] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][3])));
        let eq71_e1839_d_b4: f64 = ((s.db[187][4] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][4])));
        let eq71_e1839_d_b5: f64 = ((s.db[187][5] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][5])));
        let eq71_e1839_d_b6: f64 = ((s.db[187][6] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][6])));
        let eq71_e1839_d_b7: f64 = ((s.db[187][7] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][7])));
        let eq71_e1839_d_b8: f64 = ((s.db[187][8] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][8])));
        let eq71_e1839_d_b9: f64 = ((s.db[187][9] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][9])));
        let eq71_e1839_d_b10: f64 = ((s.db[187][10] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][10])));
        let eq71_e1839_d_b11: f64 = ((s.db[187][11] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][11])));
        let eq71_e1839_d_b12: f64 = ((s.db[187][12] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][12])));
        let eq71_e1839_d_b13: f64 = ((s.db[187][13] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][13])));
        let eq71_e1839_q: f64 = (s.v[187] * eq71_e1838_q);
        let eq71_e1839_q_d_n0: f64 = ((s.dn[187][0] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][0])));
        let eq71_e1839_q_d_n1: f64 = ((s.dn[187][1] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][1])));
        let eq71_e1839_q_d_n2: f64 = ((s.dn[187][2] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][2])));
        let eq71_e1839_q_d_n3: f64 = ((s.dn[187][3] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][3])));
        let eq71_e1839_q_d_n4: f64 = ((s.dn[187][4] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][4])));
        let eq71_e1839_q_d_n5: f64 = ((s.dn[187][5] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][5])));
        let eq71_e1839_q_d_n6: f64 = ((s.dn[187][6] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][6])));
        let eq71_e1839_q_d_n7: f64 = ((s.dn[187][7] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][7])));
        let eq71_e1839_q_d_n8: f64 = ((s.dn[187][8] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][8])));
        let eq71_e1839_q_d_n9: f64 = ((s.dn[187][9] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][9])));
        let eq71_e1839_q_d_n10: f64 = ((s.dn[187][10] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][10])));
        let eq71_e1839_q_d_n11: f64 = ((s.dn[187][11] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][11])));
        let eq71_e1839_q_d_n12: f64 = ((s.dn[187][12] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][12])));
        let eq71_e1839_q_d_n13: f64 = ((s.dn[187][13] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][13])));
        let eq71_e1839_q_d_n14: f64 = ((s.dn[187][14] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][14])));
        let eq71_e1839_q_d_n15: f64 = ((s.dn[187][15] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][15])));
        let eq71_e1839_q_d_n16: f64 = ((s.dn[187][16] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][16])));
        let eq71_e1839_q_d_b0: f64 = ((s.db[187][0] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][0])));
        let eq71_e1839_q_d_b1: f64 = ((s.db[187][1] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][1])));
        let eq71_e1839_q_d_b2: f64 = ((s.db[187][2] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][2])));
        let eq71_e1839_q_d_b3: f64 = ((s.db[187][3] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][3])));
        let eq71_e1839_q_d_b4: f64 = ((s.db[187][4] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][4])));
        let eq71_e1839_q_d_b5: f64 = ((s.db[187][5] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][5])));
        let eq71_e1839_q_d_b6: f64 = ((s.db[187][6] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][6])));
        let eq71_e1839_q_d_b7: f64 = ((s.db[187][7] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][7])));
        let eq71_e1839_q_d_b8: f64 = ((s.db[187][8] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][8])));
        let eq71_e1839_q_d_b9: f64 = ((s.db[187][9] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][9])));
        let eq71_e1839_q_d_b10: f64 = ((s.db[187][10] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][10])));
        let eq71_e1839_q_d_b11: f64 = ((s.db[187][11] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][11])));
        let eq71_e1839_q_d_b12: f64 = ((s.db[187][12] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][12])));
        let eq71_e1839_q_d_b13: f64 = ((s.db[187][13] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][13])));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n1, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14, eq71_e1839_d_n15, eq71_e1839_d_n16, eq71_e1839_d_b0, eq71_e1839_d_b1, eq71_e1839_d_b2, eq71_e1839_d_b3, eq71_e1839_d_b4, eq71_e1839_d_b5, eq71_e1839_d_b6, eq71_e1839_d_b7, eq71_e1839_d_b8, eq71_e1839_d_b9, eq71_e1839_d_b10, eq71_e1839_d_b11, eq71_e1839_d_b12, eq71_e1839_d_b13, eq71_e1839_q, eq71_e1839_q_d_n0, eq71_e1839_q_d_n1, eq71_e1839_q_d_n2, eq71_e1839_q_d_n3, eq71_e1839_q_d_n4, eq71_e1839_q_d_n5, eq71_e1839_q_d_n6, eq71_e1839_q_d_n7, eq71_e1839_q_d_n8, eq71_e1839_q_d_n9, eq71_e1839_q_d_n10, eq71_e1839_q_d_n11, eq71_e1839_q_d_n12, eq71_e1839_q_d_n13, eq71_e1839_q_d_n14, eq71_e1839_q_d_n15, eq71_e1839_q_d_n16, eq71_e1839_q_d_b0, eq71_e1839_q_d_b1, eq71_e1839_q_d_b2, eq71_e1839_q_d_b3, eq71_e1839_q_d_b4, eq71_e1839_q_d_b5, eq71_e1839_q_d_b6, eq71_e1839_q_d_b7, eq71_e1839_q_d_b8, eq71_e1839_q_d_b9, eq71_e1839_q_d_b10, eq71_e1839_q_d_b11, eq71_e1839_q_d_b12, eq71_e1839_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 17] = [eq71_e1841_q_d_n0, eq71_e1841_q_d_n1, eq71_e1841_q_d_n2, eq71_e1841_q_d_n3, eq71_e1841_q_d_n4, eq71_e1841_q_d_n5, eq71_e1841_q_d_n6, eq71_e1841_q_d_n7, eq71_e1841_q_d_n8, eq71_e1841_q_d_n9, eq71_e1841_q_d_n10, eq71_e1841_q_d_n11, eq71_e1841_q_d_n12, eq71_e1841_q_d_n13, eq71_e1841_q_d_n14, eq71_e1841_q_d_n15, eq71_e1841_q_d_n16];
        let eq71_reactive_branch_derivatives: [f64; 14] = [eq71_e1841_q_d_b0, eq71_e1841_q_d_b1, eq71_e1841_q_d_b2, eq71_e1841_q_d_b3, eq71_e1841_q_d_b4, eq71_e1841_q_d_b5, eq71_e1841_q_d_b6, eq71_e1841_q_d_b7, eq71_e1841_q_d_b8, eq71_e1841_q_d_b9, eq71_e1841_q_d_b10, eq71_e1841_q_d_b11, eq71_e1841_q_d_b12, eq71_e1841_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n1, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_d_n15, eq73_e1868_d_n16, eq73_e1868_d_b0, eq73_e1868_d_b1, eq73_e1868_d_b2, eq73_e1868_d_b3, eq73_e1868_d_b4, eq73_e1868_d_b5, eq73_e1868_d_b6, eq73_e1868_d_b7, eq73_e1868_d_b8, eq73_e1868_d_b9, eq73_e1868_d_b10, eq73_e1868_d_b11, eq73_e1868_d_b12, eq73_e1868_d_b13, eq73_e1868_q, eq73_e1868_q_d_n0, eq73_e1868_q_d_n1, eq73_e1868_q_d_n2, eq73_e1868_q_d_n3, eq73_e1868_q_d_n4, eq73_e1868_q_d_n5, eq73_e1868_q_d_n6, eq73_e1868_q_d_n7, eq73_e1868_q_d_n8, eq73_e1868_q_d_n9, eq73_e1868_q_d_n10, eq73_e1868_q_d_n11, eq73_e1868_q_d_n12, eq73_e1868_q_d_n13, eq73_e1868_q_d_n14, eq73_e1868_q_d_n15, eq73_e1868_q_d_n16, eq73_e1868_q_d_b0, eq73_e1868_q_d_b1, eq73_e1868_q_d_b2, eq73_e1868_q_d_b3, eq73_e1868_q_d_b4, eq73_e1868_q_d_b5, eq73_e1868_q_d_b6, eq73_e1868_q_d_b7, eq73_e1868_q_d_b8, eq73_e1868_q_d_b9, eq73_e1868_q_d_b10, eq73_e1868_q_d_b11, eq73_e1868_q_d_b12, eq73_e1868_q_d_b13,) = {
    if (s.b[1627] && s.b[1628]) {
        let eq73_e1864: f64 = (p.p29 * s.v[334]);
        let eq73_e1865_q: f64 = eq73_e1864;
        let eq73_e1866: f64 = (s.v[187] * eq73_e1864);
        let eq73_e1866_d_n0: f64 = ((s.dn[187][0] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq73_e1866_d_n1: f64 = ((s.dn[187][1] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq73_e1866_d_n2: f64 = ((s.dn[187][2] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq73_e1866_d_n3: f64 = ((s.dn[187][3] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq73_e1866_d_n4: f64 = ((s.dn[187][4] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq73_e1866_d_n5: f64 = ((s.dn[187][5] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq73_e1866_d_n6: f64 = ((s.dn[187][6] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq73_e1866_d_n7: f64 = ((s.dn[187][7] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq73_e1866_d_n8: f64 = ((s.dn[187][8] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq73_e1866_d_n9: f64 = ((s.dn[187][9] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq73_e1866_d_n10: f64 = ((s.dn[187][10] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq73_e1866_d_n11: f64 = ((s.dn[187][11] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq73_e1866_d_n12: f64 = ((s.dn[187][12] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq73_e1866_d_n13: f64 = ((s.dn[187][13] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq73_e1866_d_n14: f64 = ((s.dn[187][14] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq73_e1866_d_n15: f64 = ((s.dn[187][15] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq73_e1866_d_n16: f64 = ((s.dn[187][16] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq73_e1866_d_b0: f64 = ((s.db[187][0] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq73_e1866_d_b1: f64 = ((s.db[187][1] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq73_e1866_d_b2: f64 = ((s.db[187][2] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq73_e1866_d_b3: f64 = ((s.db[187][3] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq73_e1866_d_b4: f64 = ((s.db[187][4] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq73_e1866_d_b5: f64 = ((s.db[187][5] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq73_e1866_d_b6: f64 = ((s.db[187][6] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq73_e1866_d_b7: f64 = ((s.db[187][7] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq73_e1866_d_b8: f64 = ((s.db[187][8] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq73_e1866_d_b9: f64 = ((s.db[187][9] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq73_e1866_d_b10: f64 = ((s.db[187][10] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq73_e1866_d_b11: f64 = ((s.db[187][11] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq73_e1866_d_b12: f64 = ((s.db[187][12] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq73_e1866_d_b13: f64 = ((s.db[187][13] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][13])));
        let eq73_e1866_q: f64 = (s.v[187] * eq73_e1865_q);
        let eq73_e1866_q_d_n0: f64 = ((s.dn[187][0] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq73_e1866_q_d_n1: f64 = ((s.dn[187][1] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq73_e1866_q_d_n2: f64 = ((s.dn[187][2] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq73_e1866_q_d_n3: f64 = ((s.dn[187][3] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq73_e1866_q_d_n4: f64 = ((s.dn[187][4] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq73_e1866_q_d_n5: f64 = ((s.dn[187][5] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq73_e1866_q_d_n6: f64 = ((s.dn[187][6] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq73_e1866_q_d_n7: f64 = ((s.dn[187][7] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq73_e1866_q_d_n8: f64 = ((s.dn[187][8] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq73_e1866_q_d_n9: f64 = ((s.dn[187][9] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq73_e1866_q_d_n10: f64 = ((s.dn[187][10] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq73_e1866_q_d_n11: f64 = ((s.dn[187][11] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq73_e1866_q_d_n12: f64 = ((s.dn[187][12] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq73_e1866_q_d_n13: f64 = ((s.dn[187][13] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq73_e1866_q_d_n14: f64 = ((s.dn[187][14] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq73_e1866_q_d_n15: f64 = ((s.dn[187][15] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq73_e1866_q_d_n16: f64 = ((s.dn[187][16] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq73_e1866_q_d_b0: f64 = ((s.db[187][0] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq73_e1866_q_d_b1: f64 = ((s.db[187][1] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq73_e1866_q_d_b2: f64 = ((s.db[187][2] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq73_e1866_q_d_b3: f64 = ((s.db[187][3] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq73_e1866_q_d_b4: f64 = ((s.db[187][4] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq73_e1866_q_d_b5: f64 = ((s.db[187][5] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq73_e1866_q_d_b6: f64 = ((s.db[187][6] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq73_e1866_q_d_b7: f64 = ((s.db[187][7] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq73_e1866_q_d_b8: f64 = ((s.db[187][8] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq73_e1866_q_d_b9: f64 = ((s.db[187][9] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq73_e1866_q_d_b10: f64 = ((s.db[187][10] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq73_e1866_q_d_b11: f64 = ((s.db[187][11] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq73_e1866_q_d_b12: f64 = ((s.db[187][12] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq73_e1866_q_d_b13: f64 = ((s.db[187][13] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][13])));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n1, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14, eq73_e1866_d_n15, eq73_e1866_d_n16, eq73_e1866_d_b0, eq73_e1866_d_b1, eq73_e1866_d_b2, eq73_e1866_d_b3, eq73_e1866_d_b4, eq73_e1866_d_b5, eq73_e1866_d_b6, eq73_e1866_d_b7, eq73_e1866_d_b8, eq73_e1866_d_b9, eq73_e1866_d_b10, eq73_e1866_d_b11, eq73_e1866_d_b12, eq73_e1866_d_b13, eq73_e1866_q, eq73_e1866_q_d_n0, eq73_e1866_q_d_n1, eq73_e1866_q_d_n2, eq73_e1866_q_d_n3, eq73_e1866_q_d_n4, eq73_e1866_q_d_n5, eq73_e1866_q_d_n6, eq73_e1866_q_d_n7, eq73_e1866_q_d_n8, eq73_e1866_q_d_n9, eq73_e1866_q_d_n10, eq73_e1866_q_d_n11, eq73_e1866_q_d_n12, eq73_e1866_q_d_n13, eq73_e1866_q_d_n14, eq73_e1866_q_d_n15, eq73_e1866_q_d_n16, eq73_e1866_q_d_b0, eq73_e1866_q_d_b1, eq73_e1866_q_d_b2, eq73_e1866_q_d_b3, eq73_e1866_q_d_b4, eq73_e1866_q_d_b5, eq73_e1866_q_d_b6, eq73_e1866_q_d_b7, eq73_e1866_q_d_b8, eq73_e1866_q_d_b9, eq73_e1866_q_d_b10, eq73_e1866_q_d_b11, eq73_e1866_q_d_b12, eq73_e1866_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 17] = [eq73_e1868_q_d_n0, eq73_e1868_q_d_n1, eq73_e1868_q_d_n2, eq73_e1868_q_d_n3, eq73_e1868_q_d_n4, eq73_e1868_q_d_n5, eq73_e1868_q_d_n6, eq73_e1868_q_d_n7, eq73_e1868_q_d_n8, eq73_e1868_q_d_n9, eq73_e1868_q_d_n10, eq73_e1868_q_d_n11, eq73_e1868_q_d_n12, eq73_e1868_q_d_n13, eq73_e1868_q_d_n14, eq73_e1868_q_d_n15, eq73_e1868_q_d_n16];
        let eq73_reactive_branch_derivatives: [f64; 14] = [eq73_e1868_q_d_b0, eq73_e1868_q_d_b1, eq73_e1868_q_d_b2, eq73_e1868_q_d_b3, eq73_e1868_q_d_b4, eq73_e1868_q_d_b5, eq73_e1868_q_d_b6, eq73_e1868_q_d_b7, eq73_e1868_q_d_b8, eq73_e1868_q_d_b9, eq73_e1868_q_d_b10, eq73_e1868_q_d_b11, eq73_e1868_q_d_b12, eq73_e1868_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16, eq76_e1908_d_b0, eq76_e1908_d_b1, eq76_e1908_d_b2, eq76_e1908_d_b3, eq76_e1908_d_b4, eq76_e1908_d_b5, eq76_e1908_d_b6, eq76_e1908_d_b7, eq76_e1908_d_b8, eq76_e1908_d_b9, eq76_e1908_d_b10, eq76_e1908_d_b11, eq76_e1908_d_b12, eq76_e1908_d_b13, eq76_e1908_q, eq76_e1908_q_d_n0, eq76_e1908_q_d_n1, eq76_e1908_q_d_n2, eq76_e1908_q_d_n3, eq76_e1908_q_d_n4, eq76_e1908_q_d_n5, eq76_e1908_q_d_n6, eq76_e1908_q_d_n7, eq76_e1908_q_d_n8, eq76_e1908_q_d_n9, eq76_e1908_q_d_n10, eq76_e1908_q_d_n11, eq76_e1908_q_d_n12, eq76_e1908_q_d_n13, eq76_e1908_q_d_n14, eq76_e1908_q_d_n15, eq76_e1908_q_d_n16, eq76_e1908_q_d_b0, eq76_e1908_q_d_b1, eq76_e1908_q_d_b2, eq76_e1908_q_d_b3, eq76_e1908_q_d_b4, eq76_e1908_q_d_b5, eq76_e1908_q_d_b6, eq76_e1908_q_d_b7, eq76_e1908_q_d_b8, eq76_e1908_q_d_b9, eq76_e1908_q_d_b10, eq76_e1908_q_d_b11, eq76_e1908_q_d_b12, eq76_e1908_q_d_b13,) = {
    if (!s.b[1627]) {
        let eq76_e1904: f64 = (p.p29 * s.v[330]);
        let eq76_e1905_q: f64 = eq76_e1904;
        let eq76_e1906: f64 = (s.v[187] * eq76_e1904);
        let eq76_e1906_d_n0: f64 = ((s.dn[187][0] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][0])));
        let eq76_e1906_d_n1: f64 = ((s.dn[187][1] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][1])));
        let eq76_e1906_d_n2: f64 = ((s.dn[187][2] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][2])));
        let eq76_e1906_d_n3: f64 = ((s.dn[187][3] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][3])));
        let eq76_e1906_d_n4: f64 = ((s.dn[187][4] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][4])));
        let eq76_e1906_d_n5: f64 = ((s.dn[187][5] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][5])));
        let eq76_e1906_d_n6: f64 = ((s.dn[187][6] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][6])));
        let eq76_e1906_d_n7: f64 = ((s.dn[187][7] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][7])));
        let eq76_e1906_d_n8: f64 = ((s.dn[187][8] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][8])));
        let eq76_e1906_d_n9: f64 = ((s.dn[187][9] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][9])));
        let eq76_e1906_d_n10: f64 = ((s.dn[187][10] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][10])));
        let eq76_e1906_d_n11: f64 = ((s.dn[187][11] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][11])));
        let eq76_e1906_d_n12: f64 = ((s.dn[187][12] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][12])));
        let eq76_e1906_d_n13: f64 = ((s.dn[187][13] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][13])));
        let eq76_e1906_d_n14: f64 = ((s.dn[187][14] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][14])));
        let eq76_e1906_d_n15: f64 = ((s.dn[187][15] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][15])));
        let eq76_e1906_d_n16: f64 = ((s.dn[187][16] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][16])));
        let eq76_e1906_d_b0: f64 = ((s.db[187][0] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][0])));
        let eq76_e1906_d_b1: f64 = ((s.db[187][1] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][1])));
        let eq76_e1906_d_b2: f64 = ((s.db[187][2] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][2])));
        let eq76_e1906_d_b3: f64 = ((s.db[187][3] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][3])));
        let eq76_e1906_d_b4: f64 = ((s.db[187][4] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][4])));
        let eq76_e1906_d_b5: f64 = ((s.db[187][5] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][5])));
        let eq76_e1906_d_b6: f64 = ((s.db[187][6] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][6])));
        let eq76_e1906_d_b7: f64 = ((s.db[187][7] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][7])));
        let eq76_e1906_d_b8: f64 = ((s.db[187][8] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][8])));
        let eq76_e1906_d_b9: f64 = ((s.db[187][9] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][9])));
        let eq76_e1906_d_b10: f64 = ((s.db[187][10] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][10])));
        let eq76_e1906_d_b11: f64 = ((s.db[187][11] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][11])));
        let eq76_e1906_d_b12: f64 = ((s.db[187][12] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][12])));
        let eq76_e1906_d_b13: f64 = ((s.db[187][13] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][13])));
        let eq76_e1906_q: f64 = (s.v[187] * eq76_e1905_q);
        let eq76_e1906_q_d_n0: f64 = ((s.dn[187][0] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][0])));
        let eq76_e1906_q_d_n1: f64 = ((s.dn[187][1] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][1])));
        let eq76_e1906_q_d_n2: f64 = ((s.dn[187][2] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][2])));
        let eq76_e1906_q_d_n3: f64 = ((s.dn[187][3] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][3])));
        let eq76_e1906_q_d_n4: f64 = ((s.dn[187][4] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][4])));
        let eq76_e1906_q_d_n5: f64 = ((s.dn[187][5] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][5])));
        let eq76_e1906_q_d_n6: f64 = ((s.dn[187][6] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][6])));
        let eq76_e1906_q_d_n7: f64 = ((s.dn[187][7] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][7])));
        let eq76_e1906_q_d_n8: f64 = ((s.dn[187][8] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][8])));
        let eq76_e1906_q_d_n9: f64 = ((s.dn[187][9] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][9])));
        let eq76_e1906_q_d_n10: f64 = ((s.dn[187][10] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][10])));
        let eq76_e1906_q_d_n11: f64 = ((s.dn[187][11] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][11])));
        let eq76_e1906_q_d_n12: f64 = ((s.dn[187][12] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][12])));
        let eq76_e1906_q_d_n13: f64 = ((s.dn[187][13] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][13])));
        let eq76_e1906_q_d_n14: f64 = ((s.dn[187][14] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][14])));
        let eq76_e1906_q_d_n15: f64 = ((s.dn[187][15] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][15])));
        let eq76_e1906_q_d_n16: f64 = ((s.dn[187][16] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][16])));
        let eq76_e1906_q_d_b0: f64 = ((s.db[187][0] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][0])));
        let eq76_e1906_q_d_b1: f64 = ((s.db[187][1] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][1])));
        let eq76_e1906_q_d_b2: f64 = ((s.db[187][2] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][2])));
        let eq76_e1906_q_d_b3: f64 = ((s.db[187][3] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][3])));
        let eq76_e1906_q_d_b4: f64 = ((s.db[187][4] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][4])));
        let eq76_e1906_q_d_b5: f64 = ((s.db[187][5] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][5])));
        let eq76_e1906_q_d_b6: f64 = ((s.db[187][6] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][6])));
        let eq76_e1906_q_d_b7: f64 = ((s.db[187][7] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][7])));
        let eq76_e1906_q_d_b8: f64 = ((s.db[187][8] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][8])));
        let eq76_e1906_q_d_b9: f64 = ((s.db[187][9] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][9])));
        let eq76_e1906_q_d_b10: f64 = ((s.db[187][10] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][10])));
        let eq76_e1906_q_d_b11: f64 = ((s.db[187][11] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][11])));
        let eq76_e1906_q_d_b12: f64 = ((s.db[187][12] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][12])));
        let eq76_e1906_q_d_b13: f64 = ((s.db[187][13] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][13])));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n1, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_d_n15, eq76_e1906_d_n16, eq76_e1906_d_b0, eq76_e1906_d_b1, eq76_e1906_d_b2, eq76_e1906_d_b3, eq76_e1906_d_b4, eq76_e1906_d_b5, eq76_e1906_d_b6, eq76_e1906_d_b7, eq76_e1906_d_b8, eq76_e1906_d_b9, eq76_e1906_d_b10, eq76_e1906_d_b11, eq76_e1906_d_b12, eq76_e1906_d_b13, eq76_e1906_q, eq76_e1906_q_d_n0, eq76_e1906_q_d_n1, eq76_e1906_q_d_n2, eq76_e1906_q_d_n3, eq76_e1906_q_d_n4, eq76_e1906_q_d_n5, eq76_e1906_q_d_n6, eq76_e1906_q_d_n7, eq76_e1906_q_d_n8, eq76_e1906_q_d_n9, eq76_e1906_q_d_n10, eq76_e1906_q_d_n11, eq76_e1906_q_d_n12, eq76_e1906_q_d_n13, eq76_e1906_q_d_n14, eq76_e1906_q_d_n15, eq76_e1906_q_d_n16, eq76_e1906_q_d_b0, eq76_e1906_q_d_b1, eq76_e1906_q_d_b2, eq76_e1906_q_d_b3, eq76_e1906_q_d_b4, eq76_e1906_q_d_b5, eq76_e1906_q_d_b6, eq76_e1906_q_d_b7, eq76_e1906_q_d_b8, eq76_e1906_q_d_b9, eq76_e1906_q_d_b10, eq76_e1906_q_d_b11, eq76_e1906_q_d_b12, eq76_e1906_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 17] = [eq76_e1908_q_d_n0, eq76_e1908_q_d_n1, eq76_e1908_q_d_n2, eq76_e1908_q_d_n3, eq76_e1908_q_d_n4, eq76_e1908_q_d_n5, eq76_e1908_q_d_n6, eq76_e1908_q_d_n7, eq76_e1908_q_d_n8, eq76_e1908_q_d_n9, eq76_e1908_q_d_n10, eq76_e1908_q_d_n11, eq76_e1908_q_d_n12, eq76_e1908_q_d_n13, eq76_e1908_q_d_n14, eq76_e1908_q_d_n15, eq76_e1908_q_d_n16];
        let eq76_reactive_branch_derivatives: [f64; 14] = [eq76_e1908_q_d_b0, eq76_e1908_q_d_b1, eq76_e1908_q_d_b2, eq76_e1908_q_d_b3, eq76_e1908_q_d_b4, eq76_e1908_q_d_b5, eq76_e1908_q_d_b6, eq76_e1908_q_d_b7, eq76_e1908_q_d_b8, eq76_e1908_q_d_b9, eq76_e1908_q_d_b10, eq76_e1908_q_d_b11, eq76_e1908_q_d_b12, eq76_e1908_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16, eq77_e1918_d_b0, eq77_e1918_d_b1, eq77_e1918_d_b2, eq77_e1918_d_b3, eq77_e1918_d_b4, eq77_e1918_d_b5, eq77_e1918_d_b6, eq77_e1918_d_b7, eq77_e1918_d_b8, eq77_e1918_d_b9, eq77_e1918_d_b10, eq77_e1918_d_b11, eq77_e1918_d_b12, eq77_e1918_d_b13, eq77_e1918_q, eq77_e1918_q_d_n0, eq77_e1918_q_d_n1, eq77_e1918_q_d_n2, eq77_e1918_q_d_n3, eq77_e1918_q_d_n4, eq77_e1918_q_d_n5, eq77_e1918_q_d_n6, eq77_e1918_q_d_n7, eq77_e1918_q_d_n8, eq77_e1918_q_d_n9, eq77_e1918_q_d_n10, eq77_e1918_q_d_n11, eq77_e1918_q_d_n12, eq77_e1918_q_d_n13, eq77_e1918_q_d_n14, eq77_e1918_q_d_n15, eq77_e1918_q_d_n16, eq77_e1918_q_d_b0, eq77_e1918_q_d_b1, eq77_e1918_q_d_b2, eq77_e1918_q_d_b3, eq77_e1918_q_d_b4, eq77_e1918_q_d_b5, eq77_e1918_q_d_b6, eq77_e1918_q_d_b7, eq77_e1918_q_d_b8, eq77_e1918_q_d_b9, eq77_e1918_q_d_b10, eq77_e1918_q_d_b11, eq77_e1918_q_d_b12, eq77_e1918_q_d_b13,) = {
    if (!s.b[1627]) {
        let eq77_e1914: f64 = (p.p29 * s.v[334]);
        let eq77_e1915_q: f64 = eq77_e1914;
        let eq77_e1916: f64 = (s.v[187] * eq77_e1914);
        let eq77_e1916_d_n0: f64 = ((s.dn[187][0] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq77_e1916_d_n1: f64 = ((s.dn[187][1] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq77_e1916_d_n2: f64 = ((s.dn[187][2] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq77_e1916_d_n3: f64 = ((s.dn[187][3] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq77_e1916_d_n4: f64 = ((s.dn[187][4] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq77_e1916_d_n5: f64 = ((s.dn[187][5] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq77_e1916_d_n6: f64 = ((s.dn[187][6] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq77_e1916_d_n7: f64 = ((s.dn[187][7] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq77_e1916_d_n8: f64 = ((s.dn[187][8] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq77_e1916_d_n9: f64 = ((s.dn[187][9] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq77_e1916_d_n10: f64 = ((s.dn[187][10] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq77_e1916_d_n11: f64 = ((s.dn[187][11] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq77_e1916_d_n12: f64 = ((s.dn[187][12] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq77_e1916_d_n13: f64 = ((s.dn[187][13] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq77_e1916_d_n14: f64 = ((s.dn[187][14] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq77_e1916_d_n15: f64 = ((s.dn[187][15] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq77_e1916_d_n16: f64 = ((s.dn[187][16] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq77_e1916_d_b0: f64 = ((s.db[187][0] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq77_e1916_d_b1: f64 = ((s.db[187][1] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq77_e1916_d_b2: f64 = ((s.db[187][2] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq77_e1916_d_b3: f64 = ((s.db[187][3] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq77_e1916_d_b4: f64 = ((s.db[187][4] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq77_e1916_d_b5: f64 = ((s.db[187][5] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq77_e1916_d_b6: f64 = ((s.db[187][6] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq77_e1916_d_b7: f64 = ((s.db[187][7] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq77_e1916_d_b8: f64 = ((s.db[187][8] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq77_e1916_d_b9: f64 = ((s.db[187][9] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq77_e1916_d_b10: f64 = ((s.db[187][10] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq77_e1916_d_b11: f64 = ((s.db[187][11] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq77_e1916_d_b12: f64 = ((s.db[187][12] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq77_e1916_d_b13: f64 = ((s.db[187][13] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][13])));
        let eq77_e1916_q: f64 = (s.v[187] * eq77_e1915_q);
        let eq77_e1916_q_d_n0: f64 = ((s.dn[187][0] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq77_e1916_q_d_n1: f64 = ((s.dn[187][1] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq77_e1916_q_d_n2: f64 = ((s.dn[187][2] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq77_e1916_q_d_n3: f64 = ((s.dn[187][3] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq77_e1916_q_d_n4: f64 = ((s.dn[187][4] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq77_e1916_q_d_n5: f64 = ((s.dn[187][5] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq77_e1916_q_d_n6: f64 = ((s.dn[187][6] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq77_e1916_q_d_n7: f64 = ((s.dn[187][7] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq77_e1916_q_d_n8: f64 = ((s.dn[187][8] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq77_e1916_q_d_n9: f64 = ((s.dn[187][9] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq77_e1916_q_d_n10: f64 = ((s.dn[187][10] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq77_e1916_q_d_n11: f64 = ((s.dn[187][11] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq77_e1916_q_d_n12: f64 = ((s.dn[187][12] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq77_e1916_q_d_n13: f64 = ((s.dn[187][13] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq77_e1916_q_d_n14: f64 = ((s.dn[187][14] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq77_e1916_q_d_n15: f64 = ((s.dn[187][15] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq77_e1916_q_d_n16: f64 = ((s.dn[187][16] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq77_e1916_q_d_b0: f64 = ((s.db[187][0] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq77_e1916_q_d_b1: f64 = ((s.db[187][1] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq77_e1916_q_d_b2: f64 = ((s.db[187][2] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq77_e1916_q_d_b3: f64 = ((s.db[187][3] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq77_e1916_q_d_b4: f64 = ((s.db[187][4] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq77_e1916_q_d_b5: f64 = ((s.db[187][5] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq77_e1916_q_d_b6: f64 = ((s.db[187][6] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq77_e1916_q_d_b7: f64 = ((s.db[187][7] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq77_e1916_q_d_b8: f64 = ((s.db[187][8] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq77_e1916_q_d_b9: f64 = ((s.db[187][9] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq77_e1916_q_d_b10: f64 = ((s.db[187][10] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq77_e1916_q_d_b11: f64 = ((s.db[187][11] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq77_e1916_q_d_b12: f64 = ((s.db[187][12] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq77_e1916_q_d_b13: f64 = ((s.db[187][13] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][13])));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n1, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_d_n15, eq77_e1916_d_n16, eq77_e1916_d_b0, eq77_e1916_d_b1, eq77_e1916_d_b2, eq77_e1916_d_b3, eq77_e1916_d_b4, eq77_e1916_d_b5, eq77_e1916_d_b6, eq77_e1916_d_b7, eq77_e1916_d_b8, eq77_e1916_d_b9, eq77_e1916_d_b10, eq77_e1916_d_b11, eq77_e1916_d_b12, eq77_e1916_d_b13, eq77_e1916_q, eq77_e1916_q_d_n0, eq77_e1916_q_d_n1, eq77_e1916_q_d_n2, eq77_e1916_q_d_n3, eq77_e1916_q_d_n4, eq77_e1916_q_d_n5, eq77_e1916_q_d_n6, eq77_e1916_q_d_n7, eq77_e1916_q_d_n8, eq77_e1916_q_d_n9, eq77_e1916_q_d_n10, eq77_e1916_q_d_n11, eq77_e1916_q_d_n12, eq77_e1916_q_d_n13, eq77_e1916_q_d_n14, eq77_e1916_q_d_n15, eq77_e1916_q_d_n16, eq77_e1916_q_d_b0, eq77_e1916_q_d_b1, eq77_e1916_q_d_b2, eq77_e1916_q_d_b3, eq77_e1916_q_d_b4, eq77_e1916_q_d_b5, eq77_e1916_q_d_b6, eq77_e1916_q_d_b7, eq77_e1916_q_d_b8, eq77_e1916_q_d_b9, eq77_e1916_q_d_b10, eq77_e1916_q_d_b11, eq77_e1916_q_d_b12, eq77_e1916_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 17] = [eq77_e1918_q_d_n0, eq77_e1918_q_d_n1, eq77_e1918_q_d_n2, eq77_e1918_q_d_n3, eq77_e1918_q_d_n4, eq77_e1918_q_d_n5, eq77_e1918_q_d_n6, eq77_e1918_q_d_n7, eq77_e1918_q_d_n8, eq77_e1918_q_d_n9, eq77_e1918_q_d_n10, eq77_e1918_q_d_n11, eq77_e1918_q_d_n12, eq77_e1918_q_d_n13, eq77_e1918_q_d_n14, eq77_e1918_q_d_n15, eq77_e1918_q_d_n16];
        let eq77_reactive_branch_derivatives: [f64; 14] = [eq77_e1918_q_d_b0, eq77_e1918_q_d_b1, eq77_e1918_q_d_b2, eq77_e1918_q_d_b3, eq77_e1918_q_d_b4, eq77_e1918_q_d_b5, eq77_e1918_q_d_b6, eq77_e1918_q_d_b7, eq77_e1918_q_d_b8, eq77_e1918_q_d_b9, eq77_e1918_q_d_b10, eq77_e1918_q_d_b11, eq77_e1918_q_d_b12, eq77_e1918_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq77_reactive_node_derivatives,
            branches,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16, eq83_e1984_d_b0, eq83_e1984_d_b1, eq83_e1984_d_b2, eq83_e1984_d_b3, eq83_e1984_d_b4, eq83_e1984_d_b5, eq83_e1984_d_b6, eq83_e1984_d_b7, eq83_e1984_d_b8, eq83_e1984_d_b9, eq83_e1984_d_b10, eq83_e1984_d_b11, eq83_e1984_d_b12, eq83_e1984_d_b13, eq83_e1984_q, eq83_e1984_q_d_n0, eq83_e1984_q_d_n1, eq83_e1984_q_d_n2, eq83_e1984_q_d_n3, eq83_e1984_q_d_n4, eq83_e1984_q_d_n5, eq83_e1984_q_d_n6, eq83_e1984_q_d_n7, eq83_e1984_q_d_n8, eq83_e1984_q_d_n9, eq83_e1984_q_d_n10, eq83_e1984_q_d_n11, eq83_e1984_q_d_n12, eq83_e1984_q_d_n13, eq83_e1984_q_d_n14, eq83_e1984_q_d_n15, eq83_e1984_q_d_n16, eq83_e1984_q_d_b0, eq83_e1984_q_d_b1, eq83_e1984_q_d_b2, eq83_e1984_q_d_b3, eq83_e1984_q_d_b4, eq83_e1984_q_d_b5, eq83_e1984_q_d_b6, eq83_e1984_q_d_b7, eq83_e1984_q_d_b8, eq83_e1984_q_d_b9, eq83_e1984_q_d_b10, eq83_e1984_q_d_b11, eq83_e1984_q_d_b12, eq83_e1984_q_d_b13,) = {
    if s.b[1630] {
        let eq83_e1980: f64 = (p.p29 * s.v[334]);
        let eq83_e1981_q: f64 = eq83_e1980;
        let eq83_e1982: f64 = (s.v[187] * eq83_e1980);
        let eq83_e1982_d_n0: f64 = ((s.dn[187][0] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq83_e1982_d_n1: f64 = ((s.dn[187][1] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq83_e1982_d_n2: f64 = ((s.dn[187][2] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq83_e1982_d_n3: f64 = ((s.dn[187][3] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq83_e1982_d_n4: f64 = ((s.dn[187][4] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq83_e1982_d_n5: f64 = ((s.dn[187][5] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq83_e1982_d_n6: f64 = ((s.dn[187][6] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq83_e1982_d_n7: f64 = ((s.dn[187][7] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq83_e1982_d_n8: f64 = ((s.dn[187][8] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq83_e1982_d_n9: f64 = ((s.dn[187][9] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq83_e1982_d_n10: f64 = ((s.dn[187][10] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq83_e1982_d_n11: f64 = ((s.dn[187][11] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq83_e1982_d_n12: f64 = ((s.dn[187][12] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq83_e1982_d_n13: f64 = ((s.dn[187][13] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq83_e1982_d_n14: f64 = ((s.dn[187][14] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq83_e1982_d_n15: f64 = ((s.dn[187][15] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq83_e1982_d_n16: f64 = ((s.dn[187][16] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq83_e1982_d_b0: f64 = ((s.db[187][0] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq83_e1982_d_b1: f64 = ((s.db[187][1] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq83_e1982_d_b2: f64 = ((s.db[187][2] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq83_e1982_d_b3: f64 = ((s.db[187][3] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq83_e1982_d_b4: f64 = ((s.db[187][4] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq83_e1982_d_b5: f64 = ((s.db[187][5] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq83_e1982_d_b6: f64 = ((s.db[187][6] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq83_e1982_d_b7: f64 = ((s.db[187][7] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq83_e1982_d_b8: f64 = ((s.db[187][8] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq83_e1982_d_b9: f64 = ((s.db[187][9] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq83_e1982_d_b10: f64 = ((s.db[187][10] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq83_e1982_d_b11: f64 = ((s.db[187][11] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq83_e1982_d_b12: f64 = ((s.db[187][12] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq83_e1982_d_b13: f64 = ((s.db[187][13] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][13])));
        let eq83_e1982_q: f64 = (s.v[187] * eq83_e1981_q);
        let eq83_e1982_q_d_n0: f64 = ((s.dn[187][0] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq83_e1982_q_d_n1: f64 = ((s.dn[187][1] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq83_e1982_q_d_n2: f64 = ((s.dn[187][2] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq83_e1982_q_d_n3: f64 = ((s.dn[187][3] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq83_e1982_q_d_n4: f64 = ((s.dn[187][4] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq83_e1982_q_d_n5: f64 = ((s.dn[187][5] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq83_e1982_q_d_n6: f64 = ((s.dn[187][6] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq83_e1982_q_d_n7: f64 = ((s.dn[187][7] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq83_e1982_q_d_n8: f64 = ((s.dn[187][8] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq83_e1982_q_d_n9: f64 = ((s.dn[187][9] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq83_e1982_q_d_n10: f64 = ((s.dn[187][10] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq83_e1982_q_d_n11: f64 = ((s.dn[187][11] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq83_e1982_q_d_n12: f64 = ((s.dn[187][12] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq83_e1982_q_d_n13: f64 = ((s.dn[187][13] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq83_e1982_q_d_n14: f64 = ((s.dn[187][14] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq83_e1982_q_d_n15: f64 = ((s.dn[187][15] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq83_e1982_q_d_n16: f64 = ((s.dn[187][16] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq83_e1982_q_d_b0: f64 = ((s.db[187][0] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq83_e1982_q_d_b1: f64 = ((s.db[187][1] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq83_e1982_q_d_b2: f64 = ((s.db[187][2] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq83_e1982_q_d_b3: f64 = ((s.db[187][3] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq83_e1982_q_d_b4: f64 = ((s.db[187][4] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq83_e1982_q_d_b5: f64 = ((s.db[187][5] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq83_e1982_q_d_b6: f64 = ((s.db[187][6] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq83_e1982_q_d_b7: f64 = ((s.db[187][7] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq83_e1982_q_d_b8: f64 = ((s.db[187][8] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq83_e1982_q_d_b9: f64 = ((s.db[187][9] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq83_e1982_q_d_b10: f64 = ((s.db[187][10] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq83_e1982_q_d_b11: f64 = ((s.db[187][11] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq83_e1982_q_d_b12: f64 = ((s.db[187][12] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq83_e1982_q_d_b13: f64 = ((s.db[187][13] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][13])));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n1, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_d_n15, eq83_e1982_d_n16, eq83_e1982_d_b0, eq83_e1982_d_b1, eq83_e1982_d_b2, eq83_e1982_d_b3, eq83_e1982_d_b4, eq83_e1982_d_b5, eq83_e1982_d_b6, eq83_e1982_d_b7, eq83_e1982_d_b8, eq83_e1982_d_b9, eq83_e1982_d_b10, eq83_e1982_d_b11, eq83_e1982_d_b12, eq83_e1982_d_b13, eq83_e1982_q, eq83_e1982_q_d_n0, eq83_e1982_q_d_n1, eq83_e1982_q_d_n2, eq83_e1982_q_d_n3, eq83_e1982_q_d_n4, eq83_e1982_q_d_n5, eq83_e1982_q_d_n6, eq83_e1982_q_d_n7, eq83_e1982_q_d_n8, eq83_e1982_q_d_n9, eq83_e1982_q_d_n10, eq83_e1982_q_d_n11, eq83_e1982_q_d_n12, eq83_e1982_q_d_n13, eq83_e1982_q_d_n14, eq83_e1982_q_d_n15, eq83_e1982_q_d_n16, eq83_e1982_q_d_b0, eq83_e1982_q_d_b1, eq83_e1982_q_d_b2, eq83_e1982_q_d_b3, eq83_e1982_q_d_b4, eq83_e1982_q_d_b5, eq83_e1982_q_d_b6, eq83_e1982_q_d_b7, eq83_e1982_q_d_b8, eq83_e1982_q_d_b9, eq83_e1982_q_d_b10, eq83_e1982_q_d_b11, eq83_e1982_q_d_b12, eq83_e1982_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_reactive_node_derivatives: [f64; 17] = [eq83_e1984_q_d_n0, eq83_e1984_q_d_n1, eq83_e1984_q_d_n2, eq83_e1984_q_d_n3, eq83_e1984_q_d_n4, eq83_e1984_q_d_n5, eq83_e1984_q_d_n6, eq83_e1984_q_d_n7, eq83_e1984_q_d_n8, eq83_e1984_q_d_n9, eq83_e1984_q_d_n10, eq83_e1984_q_d_n11, eq83_e1984_q_d_n12, eq83_e1984_q_d_n13, eq83_e1984_q_d_n14, eq83_e1984_q_d_n15, eq83_e1984_q_d_n16];
        let eq83_reactive_branch_derivatives: [f64; 14] = [eq83_e1984_q_d_b0, eq83_e1984_q_d_b1, eq83_e1984_q_d_b2, eq83_e1984_q_d_b3, eq83_e1984_q_d_b4, eq83_e1984_q_d_b5, eq83_e1984_q_d_b6, eq83_e1984_q_d_b7, eq83_e1984_q_d_b8, eq83_e1984_q_d_b9, eq83_e1984_q_d_b10, eq83_e1984_q_d_b11, eq83_e1984_q_d_b12, eq83_e1984_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq83_reactive_node_derivatives,
            branches,
            &eq83_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16, eq84_e1993_d_b0, eq84_e1993_d_b1, eq84_e1993_d_b2, eq84_e1993_d_b3, eq84_e1993_d_b4, eq84_e1993_d_b5, eq84_e1993_d_b6, eq84_e1993_d_b7, eq84_e1993_d_b8, eq84_e1993_d_b9, eq84_e1993_d_b10, eq84_e1993_d_b11, eq84_e1993_d_b12, eq84_e1993_d_b13, eq84_e1993_q, eq84_e1993_q_d_n0, eq84_e1993_q_d_n1, eq84_e1993_q_d_n2, eq84_e1993_q_d_n3, eq84_e1993_q_d_n4, eq84_e1993_q_d_n5, eq84_e1993_q_d_n6, eq84_e1993_q_d_n7, eq84_e1993_q_d_n8, eq84_e1993_q_d_n9, eq84_e1993_q_d_n10, eq84_e1993_q_d_n11, eq84_e1993_q_d_n12, eq84_e1993_q_d_n13, eq84_e1993_q_d_n14, eq84_e1993_q_d_n15, eq84_e1993_q_d_n16, eq84_e1993_q_d_b0, eq84_e1993_q_d_b1, eq84_e1993_q_d_b2, eq84_e1993_q_d_b3, eq84_e1993_q_d_b4, eq84_e1993_q_d_b5, eq84_e1993_q_d_b6, eq84_e1993_q_d_b7, eq84_e1993_q_d_b8, eq84_e1993_q_d_b9, eq84_e1993_q_d_b10, eq84_e1993_q_d_b11, eq84_e1993_q_d_b12, eq84_e1993_q_d_b13,) = {
    if s.b[1630] {
        let eq84_e1989: f64 = (p.p29 * s.v[338]);
        let eq84_e1990_q: f64 = eq84_e1989;
        let eq84_e1991: f64 = (s.v[187] * eq84_e1989);
        let eq84_e1991_d_n0: f64 = ((s.dn[187][0] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][0])));
        let eq84_e1991_d_n1: f64 = ((s.dn[187][1] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][1])));
        let eq84_e1991_d_n2: f64 = ((s.dn[187][2] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][2])));
        let eq84_e1991_d_n3: f64 = ((s.dn[187][3] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][3])));
        let eq84_e1991_d_n4: f64 = ((s.dn[187][4] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][4])));
        let eq84_e1991_d_n5: f64 = ((s.dn[187][5] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][5])));
        let eq84_e1991_d_n6: f64 = ((s.dn[187][6] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][6])));
        let eq84_e1991_d_n7: f64 = ((s.dn[187][7] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][7])));
        let eq84_e1991_d_n8: f64 = ((s.dn[187][8] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][8])));
        let eq84_e1991_d_n9: f64 = ((s.dn[187][9] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][9])));
        let eq84_e1991_d_n10: f64 = ((s.dn[187][10] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][10])));
        let eq84_e1991_d_n11: f64 = ((s.dn[187][11] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][11])));
        let eq84_e1991_d_n12: f64 = ((s.dn[187][12] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][12])));
        let eq84_e1991_d_n13: f64 = ((s.dn[187][13] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][13])));
        let eq84_e1991_d_n14: f64 = ((s.dn[187][14] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][14])));
        let eq84_e1991_d_n15: f64 = ((s.dn[187][15] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][15])));
        let eq84_e1991_d_n16: f64 = ((s.dn[187][16] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][16])));
        let eq84_e1991_d_b0: f64 = ((s.db[187][0] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][0])));
        let eq84_e1991_d_b1: f64 = ((s.db[187][1] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][1])));
        let eq84_e1991_d_b2: f64 = ((s.db[187][2] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][2])));
        let eq84_e1991_d_b3: f64 = ((s.db[187][3] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][3])));
        let eq84_e1991_d_b4: f64 = ((s.db[187][4] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][4])));
        let eq84_e1991_d_b5: f64 = ((s.db[187][5] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][5])));
        let eq84_e1991_d_b6: f64 = ((s.db[187][6] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][6])));
        let eq84_e1991_d_b7: f64 = ((s.db[187][7] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][7])));
        let eq84_e1991_d_b8: f64 = ((s.db[187][8] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][8])));
        let eq84_e1991_d_b9: f64 = ((s.db[187][9] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][9])));
        let eq84_e1991_d_b10: f64 = ((s.db[187][10] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][10])));
        let eq84_e1991_d_b11: f64 = ((s.db[187][11] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][11])));
        let eq84_e1991_d_b12: f64 = ((s.db[187][12] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][12])));
        let eq84_e1991_d_b13: f64 = ((s.db[187][13] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][13])));
        let eq84_e1991_q: f64 = (s.v[187] * eq84_e1990_q);
        let eq84_e1991_q_d_n0: f64 = ((s.dn[187][0] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][0])));
        let eq84_e1991_q_d_n1: f64 = ((s.dn[187][1] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][1])));
        let eq84_e1991_q_d_n2: f64 = ((s.dn[187][2] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][2])));
        let eq84_e1991_q_d_n3: f64 = ((s.dn[187][3] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][3])));
        let eq84_e1991_q_d_n4: f64 = ((s.dn[187][4] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][4])));
        let eq84_e1991_q_d_n5: f64 = ((s.dn[187][5] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][5])));
        let eq84_e1991_q_d_n6: f64 = ((s.dn[187][6] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][6])));
        let eq84_e1991_q_d_n7: f64 = ((s.dn[187][7] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][7])));
        let eq84_e1991_q_d_n8: f64 = ((s.dn[187][8] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][8])));
        let eq84_e1991_q_d_n9: f64 = ((s.dn[187][9] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][9])));
        let eq84_e1991_q_d_n10: f64 = ((s.dn[187][10] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][10])));
        let eq84_e1991_q_d_n11: f64 = ((s.dn[187][11] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][11])));
        let eq84_e1991_q_d_n12: f64 = ((s.dn[187][12] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][12])));
        let eq84_e1991_q_d_n13: f64 = ((s.dn[187][13] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][13])));
        let eq84_e1991_q_d_n14: f64 = ((s.dn[187][14] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][14])));
        let eq84_e1991_q_d_n15: f64 = ((s.dn[187][15] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][15])));
        let eq84_e1991_q_d_n16: f64 = ((s.dn[187][16] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][16])));
        let eq84_e1991_q_d_b0: f64 = ((s.db[187][0] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][0])));
        let eq84_e1991_q_d_b1: f64 = ((s.db[187][1] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][1])));
        let eq84_e1991_q_d_b2: f64 = ((s.db[187][2] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][2])));
        let eq84_e1991_q_d_b3: f64 = ((s.db[187][3] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][3])));
        let eq84_e1991_q_d_b4: f64 = ((s.db[187][4] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][4])));
        let eq84_e1991_q_d_b5: f64 = ((s.db[187][5] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][5])));
        let eq84_e1991_q_d_b6: f64 = ((s.db[187][6] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][6])));
        let eq84_e1991_q_d_b7: f64 = ((s.db[187][7] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][7])));
        let eq84_e1991_q_d_b8: f64 = ((s.db[187][8] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][8])));
        let eq84_e1991_q_d_b9: f64 = ((s.db[187][9] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][9])));
        let eq84_e1991_q_d_b10: f64 = ((s.db[187][10] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][10])));
        let eq84_e1991_q_d_b11: f64 = ((s.db[187][11] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][11])));
        let eq84_e1991_q_d_b12: f64 = ((s.db[187][12] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][12])));
        let eq84_e1991_q_d_b13: f64 = ((s.db[187][13] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][13])));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n1, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_d_n15, eq84_e1991_d_n16, eq84_e1991_d_b0, eq84_e1991_d_b1, eq84_e1991_d_b2, eq84_e1991_d_b3, eq84_e1991_d_b4, eq84_e1991_d_b5, eq84_e1991_d_b6, eq84_e1991_d_b7, eq84_e1991_d_b8, eq84_e1991_d_b9, eq84_e1991_d_b10, eq84_e1991_d_b11, eq84_e1991_d_b12, eq84_e1991_d_b13, eq84_e1991_q, eq84_e1991_q_d_n0, eq84_e1991_q_d_n1, eq84_e1991_q_d_n2, eq84_e1991_q_d_n3, eq84_e1991_q_d_n4, eq84_e1991_q_d_n5, eq84_e1991_q_d_n6, eq84_e1991_q_d_n7, eq84_e1991_q_d_n8, eq84_e1991_q_d_n9, eq84_e1991_q_d_n10, eq84_e1991_q_d_n11, eq84_e1991_q_d_n12, eq84_e1991_q_d_n13, eq84_e1991_q_d_n14, eq84_e1991_q_d_n15, eq84_e1991_q_d_n16, eq84_e1991_q_d_b0, eq84_e1991_q_d_b1, eq84_e1991_q_d_b2, eq84_e1991_q_d_b3, eq84_e1991_q_d_b4, eq84_e1991_q_d_b5, eq84_e1991_q_d_b6, eq84_e1991_q_d_b7, eq84_e1991_q_d_b8, eq84_e1991_q_d_b9, eq84_e1991_q_d_b10, eq84_e1991_q_d_b11, eq84_e1991_q_d_b12, eq84_e1991_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_reactive_node_derivatives: [f64; 17] = [eq84_e1993_q_d_n0, eq84_e1993_q_d_n1, eq84_e1993_q_d_n2, eq84_e1993_q_d_n3, eq84_e1993_q_d_n4, eq84_e1993_q_d_n5, eq84_e1993_q_d_n6, eq84_e1993_q_d_n7, eq84_e1993_q_d_n8, eq84_e1993_q_d_n9, eq84_e1993_q_d_n10, eq84_e1993_q_d_n11, eq84_e1993_q_d_n12, eq84_e1993_q_d_n13, eq84_e1993_q_d_n14, eq84_e1993_q_d_n15, eq84_e1993_q_d_n16];
        let eq84_reactive_branch_derivatives: [f64; 14] = [eq84_e1993_q_d_b0, eq84_e1993_q_d_b1, eq84_e1993_q_d_b2, eq84_e1993_q_d_b3, eq84_e1993_q_d_b4, eq84_e1993_q_d_b5, eq84_e1993_q_d_b6, eq84_e1993_q_d_b7, eq84_e1993_q_d_b8, eq84_e1993_q_d_b9, eq84_e1993_q_d_b10, eq84_e1993_q_d_b11, eq84_e1993_q_d_b12, eq84_e1993_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            nodes,
            &eq84_reactive_node_derivatives,
            branches,
            &eq84_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
