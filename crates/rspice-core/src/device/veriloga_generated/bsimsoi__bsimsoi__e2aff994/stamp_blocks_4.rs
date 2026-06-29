#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1737]) && (!s.b[1740])) {
            s.store_mul(1046, 1042, 177);
            s.store_mul(1047, 1044, 178);
        }

        if (s.b[1620] && s.b[1737]) {
            s.store_add_scaled_product_indices(1046, 1046, 1.0, 1040, 177, 1.0);
            s.store_add_scaled_product_indices(1047, 1047, 1.0, 1041, 178, 1.0);
        }

        if (s.b[1620] && (!s.b[1737])) {
            s.store_scalar(1046, 0.0);
            s.store_scalar(1047, 0.0);
        }

        s.b[1741] = (p.p45 == 1.0);
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1741]) {
            s.store_scalar(795, (p.p140 + p.p25));
            s.store_mul(231, 230, 272);
            s.store_mul(233, 232, 272);
            s.store_mul(212, 795, 272);
            s.store_mul(240, 239, 272);
            s.store_sub(434, 231, 212);
            s.store_ln_ad(435, A::max_with_scalar(A::div_from_scalar(p.p141, s.ad_value(182)), 1e-38));
            s.store_scaled_sqrt_scaled_input(436, 272, (((2.0 * 1.602176462e-19) * s.v[180]) * p.p141), 1.0 / (s.v[199]));
            s.copy_ad(294, 436);
            s.copy_ad(214, 434);
            s.store_mul(215, 708, 272);
            s.store_sub(216, 240, 215);
            s.store_div_from_scalar(295, 1.0, 294);
            s.store_square(296, 294);
            s.store_div_from_scalar(297, 1.0, 296);
            s.copy_ad(251, 435);
            s.store_scalar(706, p.p141);
            s.store_div(124, 294, 2);
            s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);
            s.store_scale(126, 125, 1e-7);
            s.store_scalar(127, (5.0 / 4.0));
            s.store_div_from_scalar(128, 1.0, 124);
            s.store_square(129, 124);
            s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));
        }

        s.b[1742] = (((s.v[216]) as f64).abs() <= s.v[126]);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1742]) {
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        s.b[1743] = (s.v[216] < (-s.v[126]));
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1742])) && s.b[1743]) {
            s.store_neg(132, 216);
            s.store_mul3_lhs(133, 127, 132, 128);
            s.store_scaled_sub_offset_sqrt_square_offset(134, 133, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(135, A::sub(s.ad_value(132), s.ad_value(134)), 1.0, 129, A::offset(s.ad_value(134), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(137, 132, 2.0, 134, (-2.0), 129, -1.0);
            s.store_sub_ad_lhs(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);
            s.store_add(0, 135, 137);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 138, A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.5, s.ad_value(135), 1.0), 1.0);
            s.store_add_ad_rhs(140, 134, A::div_scaled_product3(s.ad_value(135), s.ad_value(0), s.ad_value(138), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138), s.ad_value(138)), s.ad_value(137), A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.3333333333333333, s.ad_value(135), 1.0))), 1.0));
            s.store_limited_exp(141, 140);
            s.store_sub(142, 132, 140);
            s.store_add_scaled_offset_product_rhs(143, 142, 2.0, 129, 141, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(136, 142, 1.0, 129, A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141)), 1.0);
            s.store_sub_from_scalar_scaled_mul(144, 1.0, 129, 141, 0.5);
            s.store_add_scaled_square_product_indices(142, 143, 1.0, 144, 136, (-4.0));
            s.store_scaled_div_ad_rhs(145, 136, A::add(s.ad_value(143), A::sqrt(s.ad_value(142))), 2.0);
            s.store_neg_add(131, 140, 145);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1742])) && (!s.b[1743])) {
            s.store_mul_offset_ad_lhs(146, A::mul3(s.ad_value(125), s.ad_value(127), s.ad_value(130)), (-1.0), 130);
            s.store_mul_ad_product_rhs_mixed_ia(147, 216, 128, A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));
            s.store_limited_exp_neg_input(150, 147);
            s.store_sub_from_scalar(149, 1.0, 150);
            s.store_add_scaled_inputs_product_right_ad(148, 216, 1.0, 129, 0.5, 124, A::sqrt(A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(129), 0.25, s.ad_value(149), -1.0)), (-1.0));
            s.store_limited_exp_neg_input(151, 148);
            s.store_add_scaled_inputs3_mixed_iia(152, 216, 2.0, 148, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(129), 1.0, s.ad_value(151)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(153, A::sub(s.ad_value(216), s.ad_value(148)), 1.0, 129, A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151)), (-1.0));
            s.store_sub_from_scalar_scaled_mul(154, 1.0, 129, 151, 0.5);
            s.store_add_scaled_square_product_indices(150, 152, 1.0, 154, 153, (-4.0));
            s.store_scaled_div_ad_rhs(139, 153, A::add(s.ad_value(152), A::sqrt(s.ad_value(150))), 2.0);
            s.store_add(131, 148, 139);
        }

        s.b[1744] = (((s.v[216]) as f64).abs() < s.v[126]);
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1744]) {
            s.store_mul_ad_affine_product_rhs(46, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1744])) {
            s.store_add_scaled_inputs3_offset_mixed_aai(19, A::mul3(A::mul3(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131)), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294)), A::div_from_scalar(1.0, s.ad_value(294))), 1.0, A::limited_exp_scaled_input(s.ad_value(131), -1.0), -1.0, 131, -1.0, (-(-1.0)));
            s.store_offset_add_ad(20, A::limited_exp_scaled_input(s.ad_value(131), -1.0), A::div_scaled_product(A::square(s.ad_value(2)), A::sub_scaled_inputs(s.ad_value(131), 2.0, s.ad_value(216), 2.0), 1.0, A::square(s.ad_value(294)), 1.0), (-1.0));
            s.store_sub_div_rhs_indices(46, 131, 19, 20);
        }

        if (s.b[1620] && s.b[1741]) {
            s.store_mul(46, 46, 271);
            s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);
            s.store_div_from_scalar(96, 1.0, 95);
            s.store_add_ad_lhs(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 233);
            s.store_limited_exp_neg_input(99, 97);
            s.store_scale(101, 95, 0.001);
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(271), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(271)), 1.0, 3, 216, (-1.0));
            s.store_add_scaled_product_right_ad(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);
        }

        s.b[1745] = (s.v[4] < s.v[97]);
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        s.b[1746] = (s.v[214] < s.v[104]);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        s.b[1747] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && s.b[1747]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1748] = (s.v[214] < (-s.v[101]));
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && s.b[1748]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1620] && s.b[1741]) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) {
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 272, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(106, 105, 0.5, 43, 0.5, 105, 43, 40.0, (-0.5));
            s.store_add_scaled_inputs_product_mixed_aaii(107, A::square(A::sub(s.ad_value(214), s.ad_value(106))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, 4, (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1749] = (s.v[107] < 0.0);
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) && s.b[1749]) {
            s.store_scalar(107, 0.0);
        }

        if (((s.b[1620] && s.b[1741]) && s.b[1745]) && (!s.b[1746])) {
            s.store_add_scaled_inputs3_mixed_iia(49, 97, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 97);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_inputs_product_mixed_aaia(120, A::square(A::sub(s.ad_value(214), s.ad_value(117))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_add_scaled_sub_value_product_rhs(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, s.ad_value(296), s.ad_value(118), (((-1.0)) * (2.0)));
            s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1750] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1745])) && s.b[1750]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1751] = (s.v[214] < (-s.v[101]));
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && s.b[1751]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1745])) && (!s.b[1750])) && (!s.b[1751])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (s.b[1620] && s.b[1741]) {
            s.copy_ad(123, 9);
            s.store_scalar(102, 1e-7);
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_add_scaled_value_products(6, s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(271)), s.ad_value(727), (-1.0), A::offset(s.ad_value(3), 1.0), s.ad_value(46), 1.0);
        }

        s.b[1752] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1752]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            let assign32190_ad_e49076: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign32190_ad_e49076, 2.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1752])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1741]) {
            s.copy_ad(123, 22);
        }

        s.b[1753] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1753]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            let assign32380_ad_e49751: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign32380_ad_e49751, 2.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1753])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1741]) {
            s.copy_ad(123, 22);
        }

        s.b[1754] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1754]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            let assign32570_ad_e50426: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign32570_ad_e50426, 2.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1754])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1741]) {
            s.store_scale(50, 271, 3.912023005);
        }

        s.b[1755] = (s.v[22] <= 0.0);
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1755]) {
            s.store_scalar(306, 0.0);
            s.store_sub(51, 214, 22);
            s.store_mul(52, 51, 271);
            s.copy_ad(312, 50);
            s.store_scalar(458, 1.0);
            s.store_scalar(834, 1.0);
            s.store_scalar(853, 1.0);
            s.store_scalar(343, 1.0);
            s.store_scalar(339, 1.0);
            s.store_scalar(363, 1.0);
            s.store_scalar(365, 1.0);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_div_from_scalar_offset_square(54, 1.0, 22, 2.0);
            s.store_mul_square_lhs(55, 22, 54);
            s.store_limited_exp(53, 22);
            s.store_div_from_scalar(56, 1.0, 53);
            s.store_limited_exp_sub(53, 22, 97);
            s.store_add_scaled_product_mixed_iaa(57, 53, 1.0, A::limited_exp_scaled_input(s.ad_value(97), -1.0), A::add(A::offset(s.ad_value(22), 1.0), s.ad_value(55)), (-1.0));
            s.store_add_scaled_product_mixed_iaa(58, 57, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(22))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);
            s.store_offset_add_scaled_inputs(58, A::offset(s.ad_value(58), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(58), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(59, 58);
            s.store_mul_sqrt_ad_rhs(61, 294, A::add(s.ad_value(58), s.ad_value(57)));
            s.store_div_scaled_product3_mixed_iiia(306, 296, 57, 271, 1.0, A::add_scaled_product(s.ad_value(61), 1.0, s.ad_value(294), s.ad_value(59), 1.0), 1.0);
            s.store_mul3_lhs(247, 59, 294, 271);
            s.copy_ad(76, 56);
            s.copy_ad(78, 57);
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
            s.store_mul_add_scaled_inputs_rhs(308, 335, s.ad_value(247), 1.0, s.ad_value(306), s.v[338]);
            s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scaled_offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0, 0.5), 1e-38))));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(241), 1.0), A::pow(s.ad_value(308), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(309, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);
            s.store_div_from_scalar_scaled_ad(448, 1.0, A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2);
            s.store_add_scaled_inputs3_offset_mixed_iia(273, 298, 0.5, 241, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(298), s.ad_value(241)), (-0.05)), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));
            s.store_sqrt(274, 273);
        }

        s.b[1756] = (p.p33 == 1.0);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1756]) {
            s.store_scalar(456, 0.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1756])) {
            s.store_offset_mul(167, 770, 306, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_add_ad_rhs(170, 169, A::sqrt_square_offset(s.ad_value(169), 0.01));
            s.store_mul_ad_affine_product_lhs(456, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2, 0.0, 652);
        }

        s.b[1757] = (p.p33 == 2.0);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1756])) && s.b[1757]) {
            s.store_mul_add_ad_lhs(456, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453), 652);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_mul_div_scaled_inputs_indices(310, 309, 746, 2.0, 740, 1.0);
            s.store_scale(311, 310, s.v[184]);
            s.store_mul_add_scaled_inputs_rhs(173, 742, s.ad_value(306), 1.0, s.ad_value(271), 2.0);
        }

        s.b[1758] = (s.v[456] > 0.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1758]) {
            s.store_scale(324, 746, (s.v[183] * s.v[199]));
            s.store_mul(167, 324, 456);
            s.store_scale(325, 167, 2.0);
            s.store_add_scaled_inputs_product_indices(326, 173, 1.0, 311, 1.0, 173, 167, 3.0);
            s.store_mul_add_scaled_product_rhs(327, 173, s.ad_value(311), 1.0, s.ad_value(173), s.ad_value(167), 2.0);
            s.store_div_scaled_inputs2_mixed_iai(312, 326, 1.0, A::sqrt(A::add_scaled_square_product(s.ad_value(326), 1.0, s.ad_value(325), s.ad_value(327), (-2.0))), (-1.0), 325, 1.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1758])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(312, 311, 173, 1.0, 311, 1.0, 173, 1.0, 1.0);
        }

        s.b[1759] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1759]) {
            s.store_scalar(1019, 1.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1759])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_div_scaled_inputs2_mixed_iaa(1019, 168, p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(306), p.p1351), s.ad_value(271), p.p1350), (-1.0), A::scale_offset(s.ad_value(241), p.p1352, 1.0), 1.0, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(1019, 1019, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_offset_add_scaled_inputs(312, A::offset(s.ad_value(312), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(312), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_div(312, 312, 1019);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(312)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 232, 272);
            s.store_add_ad_lhs(98, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 318);
            s.store_limited_exp_neg_input(100, 98);
            s.store_scale(101, 95, 0.001);
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(272), 1.0, s.ad_value(724), s.ad_value(272), 1.0));
            s.store_add_scaled_offset_product_lhs_mixed_aii(4, A::add_scaled_inputs3(A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(271), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(271)), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(727)), -1.0), 1.0, 3, 1.0, 168, 1.0);
            s.store_add_scaled_product_right_ad(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);
        }

        s.b[1760] = (s.v[4] < s.v[98]);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        s.b[1761] = (s.v[214] < s.v[104]);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        s.b[1762] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && s.b[1761]) && s.b[1762]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1763] = (s.v[214] < (-s.v[101]));
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if ((((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && s.b[1761]) && (!s.b[1762])) && s.b[1763]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && s.b[1761]) && (!s.b[1762])) && (!s.b[1763])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && (!s.b[1761])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 272, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(106, 105, 0.5, 43, 0.5, 105, 43, 40.0, (-0.5));
            s.store_add_scaled_inputs_product_mixed_aaii(107, A::square(A::sub(s.ad_value(214), s.ad_value(106))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, 4, (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1764] = (s.v[107] < 0.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && (!s.b[1761])) && s.b[1764]) {
            s.store_scalar(107, 0.0);
        }

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1760]) && (!s.b[1761])) {
            s.store_add_scaled_inputs3_mixed_iia(49, 98, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 98);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_inputs_product_mixed_aaia(120, A::square(A::sub(s.ad_value(214), s.ad_value(117))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_add_scaled_sub_value_product_rhs(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, s.ad_value(296), s.ad_value(118), (((-1.0)) * (2.0)));
            s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1765] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1760])) && s.b[1765]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1766] = (s.v[214] < (-s.v[101]));
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1760])) && (!s.b[1765])) && s.b[1766]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1760])) && (!s.b[1765])) && (!s.b[1766])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1760])) && (!s.b[1765])) && (!s.b[1766])) {
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.copy_ad(123, 9);
            s.store_scalar(102, 1e-7);
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(272), 1.0, s.ad_value(724), s.ad_value(272), 1.0));
            s.store_add_scaled_inputs_product_mixed_aaai(6, A::add_scaled_product(s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(271)), s.ad_value(727), (-1.0)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), s.ad_value(168), s.ad_value(271)), 1.0, A::offset(s.ad_value(3), 1.0), 46, 1.0);
        }

        s.b[1767] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1767]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            let assign35090_ad_e56087: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign35090_ad_e56087, 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.copy_ad(123, 23);
        }

        s.b[1768] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1768]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            let assign35280_ad_e56816: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign35280_ad_e56816, 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.copy_ad(123, 23);
        }

        s.b[1769] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1769]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            let assign35470_ad_e57545: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign35470_ad_e57545, 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_sub(62, 23, 22);
            s.store_mul(63, 226, 272);
            s.store_limited_exp_neg_input(64, 63);
        }

        s.b[1770] = (s.v[62] < 1e-10);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(271), 1.0, s.ad_value(271), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)));
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_limited_exp_ad(178, A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(271), 2.0, s.ad_value(271), 1.0));
            s.store_limited_exp_ad(179, A::add(A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(271), 2.0, s.ad_value(271), 1.0), s.ad_value(170)));
            s.store_neg_ad(65, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(63), (-1.0), s.ad_value(98), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(63), -1.0, s.ad_value(98), 1.0)), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), 1.0, (-1.0)));
            s.store_mul_ad_lhs(66, A::mul_sub_from_scalar_rhs(s.ad_value(296), 1.0, s.ad_value(64)), 57);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            let assign35660_ad_e58235: A = A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product3_by_product(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0, A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), (-1.0), s.ad_value(296), A::sub(A::sub(A::add_scaled_inputs4(A::add_scaled_inputs_product(s.ad_value(173), 1.0, A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(98), (-1.0), s.ad_value(63), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(63), 1.0)), A::sub(A::add_scaled_product(s.ad_value(175), (-2.0), A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(175), 10.0), s.ad_value(175), 1.0), A::mul3(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 8.0), s.ad_value(123), s.ad_value(175)), s.ad_value(175), s.ad_value(175))), 1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), (-1.0), A::div(s.ad_value(178), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), 1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), A::div(s.ad_value(179), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), A::div(s.ad_value(179), A::mul(A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), (-1.0));
            s.store_offset_sub_ad(54, assign35660_ad_e58235, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_add_scaled_square_product_indices(54, 65, 1.0, 54, 66, (-2.0));
        }

        s.b[1771] = (s.v[54] >= 0.0);
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) && s.b[1771]) {
            s.store_scaled_div_ad_rhs(62, 66, A::add(s.ad_value(65), A::sqrt(s.ad_value(54))), 2.0);
        }

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {
            s.store_add(23, 22, 62);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_mul(250, 62, 271);
            s.store_div_scaled_product_offset_denominator(67, s.ad_value(23), s.ad_value(23), 1.0, A::square(s.ad_value(23)), 2.0, 1.0);
            s.store_limited_exp_neg_input(68, 23);
            s.store_add_scaled_product(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67)), (-1.0));
            s.store_add_scaled_product_mixed_iaa(70, 69, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(23))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);
            s.store_offset_add_scaled_inputs(70, A::offset(s.ad_value(70), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(70), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(60, 70);
            s.store_mul_sqrt_ad_rhs(72, 294, A::add(s.ad_value(70), s.ad_value(69)));
            s.store_div_scaled_product3_mixed_iiia(73, 296, 69, 271, 1.0, A::add_scaled_product(s.ad_value(72), 1.0, s.ad_value(294), s.ad_value(60), 1.0), 1.0);
            s.store_scaled_add(75, 22, 23, 0.5);
            s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));
            s.store_sqrt(76, 54);
            s.store_scaled_add(77, 57, 69, 0.5);
            s.store_add_scaled_product_mixed_iaa(78, 77, 1.0, A::square(s.ad_value(62)), A::sub_scaled_inputs(s.ad_value(76), 1.0, s.ad_value(297), 2.0), 0.125);
            s.store_add_scaled_product_mixed_iaa(79, 78, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(75))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);
            s.store_mul_sqrt_ad_rhs(51, 294, A::add(s.ad_value(78), s.ad_value(79)));
            s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(79), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(71, 79);
        }

        s.b[1772] = (((s.v[250]) as f64).abs() > 1e-35);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1772]) {
            s.store_div_scaled_inputs2_indices(74, 306, 1.0, 73, (-1.0), 250, 1.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {
            s.store_mul_div_scaled_product_mixed_iiia(80, 271, 296, 78, 1.0, A::add_scaled_product(s.ad_value(51), 1.0, s.ad_value(294), s.ad_value(71), 1.0), 1.0);
            s.store_mul(52, 51, 271);
            s.copy_ad(83, 74);
            s.store_offset_add_scaled_inputs(83, A::offset(s.ad_value(83), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(83), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_add_scaled_product_indices(81, 80, 1.0, 271, 83, 1.0);
            s.store_div(84, 81, 83);
        }

        s.b[1773] = (s.v[22] <= 0.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1773]) {
            s.copy_ad(447, 52);
            s.store_scalar(444, 0.0);
            s.copy_ad(445, 447);
            s.store_scalar(446, 0.0);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1773])) {
            s.store_scaled_div(26, 250, 84, 0.5);
            s.store_square(27, 26);
            s.store_add_scaled_product_indices(447, 52, 1.0, 250, 26, (0.3333333333333333 * 0.5));
            s.store_scaled_mul(54, 74, 250, 0.16666666666666666);
            s.store_add_scaled_product_indices(443, 80, 1.0, 54, 26, 1.0);
            s.store_add_scaled_product_right_ad(444, 80, 0.5, 54, A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(26)), 1.0, s.ad_value(27), 0.2), ((-1.0) * 0.5));
            s.store_sub(445, 447, 443);
            s.store_add_scaled_inputs3_indices(446, 447, 1.0, 445, (-1.0), 444, -1.0);
        }

        if (s.b[1620] && s.b[1741]) {
            s.store_scale_ad(437, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), p.p1380);
            s.copy_ad(391, 437);
            s.store_mul_neg_lhs(440, 391, 445);
        }

        s.b[1774] = (s.v[211] > 0.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1741]) && s.b[1774]) {
            s.store_mul_neg_lhs(441, 391, 446);
            s.store_mul_neg_lhs(439, 391, 444);
        }

        if ((s.b[1620] && s.b[1741]) && (!s.b[1774])) {
            s.store_mul_neg_lhs(441, 391, 444);
            s.store_mul_neg_lhs(439, 391, 446);
        }

        if (s.b[1620] && s.b[1741]) {
            s.store_add_scaled_inputs3_indices(442, 440, (-1.0), 441, (-1.0), 439, (-1.0));
        }

        if (s.b[1620] && (!s.b[1741])) {
            s.store_scalar(440, 0.0);
            s.store_scalar(439, 0.0);
            s.store_scalar(438, 0.0);
            s.store_scalar(441, 0.0);
            s.store_scalar(442, 0.0);
        }

        if s.b[1620] {
            s.store_mul_add_ad_rhs(1075, 379, A::add_scaled_inputs4(s.ad_value(387), 1.0, s.ad_value(440), 1.0, s.ad_value(421), 1.0, s.ad_value(520), 1.0), s.ad_value(525));
            s.store_mul_add_rhs(1050, 379, 388, 441);
            s.store_mul_add_rhs(1053, 379, 389, 439);
            s.store_mul_ad_rhs(1076, 379, A::add_scaled_inputs4(s.ad_value(388), 1.0, s.ad_value(441), 1.0, s.ad_value(423), 1.0, s.ad_value(520), -1.0));
            s.store_mul_ad_rhs(1077, 379, A::add_scaled_inputs4(s.ad_value(389), 1.0, s.ad_value(439), 1.0, s.ad_value(424), 1.0, s.ad_value(525), -1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(1078, 379, s.ad_value(390), 1.0, s.ad_value(442), 1.0, s.ad_value(422), 1.0, 0.0);
            s.store_mul(1057, 379, 390);
            s.store_mul(1058, 379, 442);
            s.store_mul(1051, 379, 388);
            s.store_mul(1052, 379, 441);
            s.store_mul(1054, 379, 389);
            s.store_mul(1055, 379, 439);
            s.store_add_scaled_offset_product_rhs(810, 810, 1.0, 813, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(816, 816, 1.0, 814, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(819, 819, 1.0, 815, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(884, 884, 1.0, 886, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(882, 882, 1.0, 887, 639, (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs(888, 888, 1.0, 891, 639, (-1.0), 1.0);
        }

        s.b[1775] = ((p.p37 != 0.0) || (p.p38 != 0.0));
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1775]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(469, 269, s.ad_value(213), 1.0, s.ad_value(22), (-0.5), s.ad_value(23), (-0.5), 0.0);
            s.store_sqrt_square_offset(168, 469, 0.0001);
            s.store_scaled_sub(471, 168, 469, 0.5);
            s.store_scaled_add(470, 469, 168, 0.5);
        }

        s.b[1776] = (p.p38 != 0.0);
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            s.store_scale(168, 469, 1.0 / (p.p671));
        }

        s.b[1777] = (p.p696 != 0.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1777]) {
            s.store_sub_from_scalar_scaled_input(167, 1.0, 471, 1.0 / (p.p696));
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && (!s.b[1777])) {
            s.store_scalar(167, 1.0);
        }

        s.b[1778] = (s.v[167] < 0.01);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1778]) {
            s.store_scalar(167, 0.01);
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));
            s.store_scalar(169, (p.p701 * p.p76));
            s.store_div_scaled_product_right_ad(170, 169, A::add_scaled_product(s.ad_value(882), 1.0, s.ad_value(883), s.ad_value(471), (-1.0)), 1.0, 167, 1.0);
            s.store_limited_exp(171, 170);
            s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));
        }

        s.b[1779] = (p.p697 != 0.0);
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1779]) {
            s.store_sub_from_scalar_scaled_input(167, 1.0, 470, 1.0 / (p.p697));
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && (!s.b[1779])) {
            s.store_scalar(167, 1.0);
        }

        s.b[1780] = (s.v[167] < 0.01);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1780]) {
            s.store_scalar(167, 0.01);
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));
            s.store_scalar(169, (p.p699 * p.p76));
            s.store_div_scaled_product_right_ad(170, 169, A::add_scaled_product(s.ad_value(884), 1.0, s.ad_value(885), s.ad_value(470), (-1.0)), 1.0, 167, 1.0);
            s.store_limited_exp(171, 170);
            s.store_offset_mul(478, 212, 269, p.p1383);
        }

        s.b[1781] = (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0));
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {
            s.store_mul_voltage_ad(208, s.ad_value(379), ctx, nodes, Some(8), Some(11));
            s.store_sub(167, 208, 478);
            s.store_sqrt_square_offset(168, 167, 0.0001);
            s.store_offset_scaled_sub(209, 168, 167, 0.5, (((-0.01)) * (0.5)));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {
            s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));
        }

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {
            s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {
            s.store_mul(169, 208, 209);
            s.store_add_scaled_product_indices(170, 889, (-1.0), 888, 890, 1.0);
            s.store_mul(171, 889, 890);
            s.store_mul_sub_scaled_inputs_rhs(172, 179, A::add_scaled_product(s.ad_value(888), 1.0, s.ad_value(170), s.ad_value(209), 1.0), (-p.p76), A::mul3(s.ad_value(171), s.ad_value(209), s.ad_value(209)), (-p.p76));
            s.store_limited_exp(173, 172);
            s.store_scaled_mul(178, 178, 492, p.p1380);
        }

        s.b[1782] = (p.p37 != 0.0);
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1775]) && s.b[1782]) {
            s.store_add_scaled_product_indices(168, 810, 1.0, 811, 470, (-1.0));
            s.store_offset_mul(169, 812, 470, 1.0);
            s.store_scaled_mul(170, 168, 169, s.v[488]);
            s.store_mul_product3_mixed_aiii(171, A::limited_exp(s.ad_value(170)), 253, 269, 243, 1.0);
            s.store_offset_sqrt_ad(472, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));
            s.store_scale(168, 472, s.v[823]);
            s.store_limited_exp_neg_input(482, 168);
            s.store_offset_add(170, 168, 482, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(171, 1.0, A::mul_offset_lhs(s.ad_value(168), 1.0, s.ad_value(482)), 0.0001);
            s.store_offset_square(172, 168, 0.0002);
            s.store_sub(169, 203, 219);
            s.store_sqrt_square_offset(228, 169, 0.0001);
        }

        s.b[1784] = (p.p1295 == 1.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1784]) {
            s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1785] = (s.v[818] < 0.01);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1784]) && s.b[1785]) {
            s.store_scalar(818, 0.01);
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && (!s.b[1784])) {
            s.store_add_scaled_product_indices(168, 816, 1.0, 817, 228, (-1.0));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1782]) {
            s.store_offset_mul(169, 818, 228, 1.0);
            s.store_mul3_lhs(170, 491, 168, 169);
            s.store_limited_exp(171, 170);
            s.store_sub(169, 204, 219);
            s.store_sqrt_square_offset(229, 169, 0.0001);
        }

        s.b[1786] = (p.p1295 == 1.0);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1786]) {
            s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1787] = (s.v[821] < 0.01);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1775]) && s.b[1782]) && s.b[1786]) && s.b[1787]) {
            s.store_scalar(821, 0.01);
        }

        if (((s.b[1620] && s.b[1775]) && s.b[1782]) && (!s.b[1786])) {
            s.store_add_scaled_product_indices(168, 819, 1.0, 820, 229, (-1.0));
        }

        if ((s.b[1620] && s.b[1775]) && s.b[1782]) {
            s.store_offset_mul(169, 821, 229, 1.0);
            s.store_mul3_lhs(170, 491, 168, 169);
            s.store_limited_exp(171, 170);
        }

        if s.b[1620] {
            s.store_div_scaled_inputs_indices(607, 746, 2.0, 337, 1.0);
        }

        s.b[1788] = (p.p1011 <= 0.0);
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1788]) {
            s.store_scalar(610, 0.0);
        }

        if (s.b[1620] && (!s.b[1788])) {
            s.store_div_scaled_offset_numerator(167, A::div(s.ad_value(355), s.ad_value(300)), 1.0, p.p1011, s.ad_value(607), 1.0);
            s.store_mul_ln_ad_rhs(610, 300, A::max_with_scalar(s.ad_value(167), 1e-38));
        }

        s.b[1789] = (s.v[610] < 0.0);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1788])) && s.b[1789]) {
            s.store_scalar(610, 0.0);
        }

        if s.b[1620] {
            s.store_mul_add_scaled_inputs_rhs(613, 271, A::offset(s.ad_value(260), s.v[199]), 1.0 / (1.602176462e-19), s.ad_value(709), 1.0 / (1.602176462e-19));
            s.store_mul_ad_affine_product_lhs(612, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(73), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);
            s.store_mul_ad_affine_product_lhs(1004, s.ad_value(271), A::abs(s.ad_value(380)), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19), 0.0, 337);
            s.store_mul3_affine_lhs(1005, 271, 380, 1.602176462e-19, 0.0, 380);
            s.store_add_scaled_product_value_ad(1006, A::scale_offset(s.ad_value(612), p.p1013, p.p1012), 1.0, 612, 612, p.p1014);
            s.store_square_ad(1007, A::add(s.ad_value(612), s.ad_value(613)));
            s.store_scale(1008, 271, (p.p1012 * 1.602176462e-19));
        }

        s.b[1790] = (p.p1015 >= (s.v[184] / 2.0));
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1790]) {
            s.store_scalar(606, 0.0);
        }

        if (s.b[1620] && (!s.b[1790])) {
            s.store_scalar(606, p.p1015);
        }

        s.b[1791] = (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0));
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1791]) {
            s.store_sub_from_scalar_scaled_input(608, s.v[184], 606, 2.0);
            s.store_square(609, 608);
            s.store_scale(167, 609, (10000000000.0 * s.v[199]));
            s.store_mul_ad_affine_product_lhs(611, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(306), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);
            s.store_scaled_ln_ad(168, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(611), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p.p1012);
            s.store_scaled_sub(169, 611, 612, p.p1013);
            s.store_scaled_sub_ad(170, A::square(s.ad_value(611)), A::square(s.ad_value(612)), (0.5 * p.p1014));
            s.store_scale(171, 609, (10000000000.0 * (s.v[183] * p.p2)));
            s.store_add_scaled_product(614, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(171), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(167)), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, s.ad_value(170), 1.0), 1.0);
            s.store_mul3_affine_lhs(172, 608, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs_mixed_ai(615, A::div(s.ad_value(1008), s.ad_value(172)), 380, 380);
            s.store_add(173, 615, 614);
        }

        if s.b[1620] {
            s.store_scaled_div(167, 243, 607, 1.0 / (s.v[184]));
            s.store_square(168, 167);
            s.store_offset_scaled(170, 168, (((p.p1022 * s.v[184])) * (p.p1019)), p.p1019);
            s.store_offset_scaled(171, 168, (((p.p1023 * s.v[184])) * (p.p1020)), p.p1020);
            s.store_offset_scaled(172, 168, (((p.p1298 * s.v[184])) * (p.p1297)), p.p1297);
            s.store_square(633, 172);
            s.store_square(632, 171);
            s.copy_ad(345, 343);
        }

        s.b[1793] = (p.p39 == 0.0);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        s.b[1794] = (p.p39 == 1.0);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1793]) {
            s.store_scaled_mul(388, 271, 1050, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));
            s.store_scaled_mul(389, 271, 1053, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));
            s.store_mul_abs_ad_rhs(167, 337, A::add(s.ad_value(388), s.ad_value(389)));
            s.store_offset_mul(168, 167, 457, (s.v[184] * s.v[184]));
        }

        if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
            s.store_scaled_mul(626, 253, 269, 2.0);
            s.store_mul_scale_ad_lhs(167, A::mul3(s.ad_value(337), s.ad_value(345), s.ad_value(363)), s.v[199], 626);
            s.store_scaled_add(168, 306, 73, 0.5);
            s.store_offset(170, 168, 0.5);
            s.store_square(171, 170);
            s.store_mul(172, 171, 170);
            s.store_sub(173, 306, 73);
            s.store_square(174, 173);
            s.store_mul(175, 174, 173);
            s.store_mul_scale_offset_rhs(176, 174, 168, 6.0, 0.5);
            s.store_scale(625, 345, s.v[184]);
            s.store_scale(177, 625, 1.0 / (s.v[184]));
            s.store_offset_ad(179, A::div_scaled_product_by_product(s.ad_value(633), s.ad_value(315), 1.0, s.ad_value(312), A::offset(s.ad_value(243), p.p1299), 1.0), 1.0);
        }

        if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
            s.store_offset_scaled(179, 179, { let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));
        }

        if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
            s.store_scaled_add_sqrt_square_offset_rhs(179, 179, 179, ((0.25 * 0.1) * 0.1), 0.5);
            s.store_div_scaled_product3_mixed_aaii(622, A::mul3(s.ad_value(625), s.ad_value(177), s.ad_value(177)), A::add_scaled_inputs3(A::div(s.ad_value(168), s.ad_value(171)), 1.0, A::div(s.ad_value(176), A::mul_scaled_lhs(s.ad_value(171), 60.0, s.ad_value(171))), (-1.0), A::div_scaled_product_by_product(s.ad_value(174), s.ad_value(174), 1.0, s.ad_value(171), s.ad_value(172), 144.0), 1.0), 632, (15.0 * 1.0 / (4.0)), 167, ((p.p2 * s.v[183]) * 12.0));
        }

        s.b[1799] = (p.p27 == 1.0);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1799]) {
            s.store_ln_ad(951, A::max_with_scalar(A::div(s.ad_value(953), s.ad_value(182)), 1e-38));
            s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(951)), 0.4), s.ad_value(729)), 0.4);
            s.store_sqrt(299, 298);
            s.store_sqrt_div_from_scalar_ad(277, (2.0 * s.v[180]), A::scale(s.ad_value(953), 1.602176462e-19));
            s.store_mul_add_scaled_inputs_rhs(941, 835, A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5);
            s.store_mul_offset_ad_rhs(940, 841, A::mul_offset_rhs(s.ad_value(848), s.ad_value(639), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_offset_mixed_iia(273, 298, 0.5, 218, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05)), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));
            s.store_sqrt(274, 273);
            s.store_mul(275, 277, 274);
            s.store_div_from_scalar(260, s.v[180], 275);
            s.store_div_scaled_product_add_scaled_denominator_indices(169, 5, 7, 1.0, 5, 1.0, 7, 1.0, 1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(170, 227, s.ad_value(838), 1.0, s.ad_value(220), p.p1183, s.ad_value(218), (-p.p1195), 0.0);
            s.store_add_scaled_inputs_products_mixed_aiiiia(171, A::add_scaled_product(s.ad_value(220), p.p1181, s.ad_value(220), s.ad_value(220), p.p1182), 1.0, 218, (-p.p1184), 218, 218, (-p.p1185), 955, A::add(A::add_scaled_product(A::add_scaled_value_products3(s.ad_value(715), 1.0, s.ad_value(712), s.ad_value(220), 1.0, s.ad_value(220), s.ad_value(220), p.p1180, s.ad_value(716), s.ad_value(218), 1.0), 1.0, s.ad_value(218), s.ad_value(218), p.p1190), s.ad_value(170)), 1.0);
            s.store_div_ad(168, A::add_scaled_inputs4_offset(s.ad_value(169), 1.0, s.ad_value(836), 1.0, s.ad_value(941), 1.0, s.ad_value(171), 1.0, s.v[199]), A::offset(s.ad_value(169), s.v[199]));
            s.store_scaled_add_offset_sqrt_square_offset(267, 168, 1.0, (-1.0), ((0.25 * 0.05) * 0.05), 0.5);
            s.store_mul(269, 267, 271);
            s.store_div_from_scalar(270, 1.0, 269);
            s.store_mul(222, 221, 270);
            s.store_mul(225, 224, 270);
            s.store_mul(212, 707, 270);
            s.store_mul(215, 708, 270);
            s.store_mul(238, 234, 270);
            s.store_add_scaled_products_left_right_ad(291, 736, A::sub(s.ad_value(274), s.ad_value(299)), 1.0, 849, 218, (-1.0));
            s.store_mul_neg_ad_lhs(944, A::add_scaled_product(s.ad_value(940), 1.0, s.ad_value(842), s.ad_value(218), 1.0), 227);
            s.store_mul_offset_rhs_ad(293, A::add_scaled_inputs_product(s.ad_value(843), 1.0, s.ad_value(844), 1.0 / (s.v[184]), s.ad_value(845), s.ad_value(218), 1.0), A::pow(s.ad_value(639), s.ad_value(846)), (-1.0));
            s.store_mul_scale_offset_rhs(946, 300, 218, p.p1264, 1.0);
        }

        s.b[1800] = (s.v[946] > 0.0);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1800]) {
            s.store_div_from_scalar(167, (p.p1263 * s.v[184]), 946);
        }

        s.b[1801] = (s.v[167] < 40.0);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && s.b[1800]) && s.b[1801]) {
            s.store_div_from_scalar_offset_ad(943, (0.5 * p.p1262), A::cosh(s.ad_value(167)), (-1.0));
        }

        if (((s.b[1620] && s.b[1799]) && s.b[1800]) && (!s.b[1801])) {
            s.store_scaled_limited_exp_scaled_input(943, 167, -1.0, p.p1262);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1800])) {
            s.store_scalar(943, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && s.b[1799]) {
            s.store_mul_sub_rhs(945, 943, 942, 298);
        }

        s.b[1802] = (s.v[280] > 0.0);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1802]) {
            s.store_mul_neg_lhs(167, 282, 227);
        }

        s.b[1803] = (s.v[167] < (-80.0));
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && s.b[1802]) && s.b[1803]) {
            s.store_scalar(169, 1.804851387e-35);
        }

        if (((s.b[1620] && s.b[1799]) && s.b[1802]) && (!s.b[1803])) {
            s.store_limited_exp(169, 167);
        }

        if ((s.b[1620] && s.b[1799]) && s.b[1802]) {
            s.store_offset_mul_offset_rhs(170, 280, 169, 1.0, s.v[184]);
            s.store_mul_scaled_ln_ad_rhs(278, 269, -1.0, A::max_with_scalar(A::div_from_scalar(s.v[184], s.ad_value(170)), 1e-38));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1802])) {
            s.store_scalar(278, 0.0);
        }

        if (s.b[1620] && s.b[1799]) {
            s.store_add_div_rhs_mixed_ia(171, 290, 284, A::pow_from_scalar(s.v[184], s.ad_value(286)));
            s.store_add_scaled_product_right_ad(278, 278, 1.0, 171, A::tanh(A::mul(s.ad_value(288), s.ad_value(227))), (-1.0));
            s.store_add_scaled_inputs3_mixed_aii(242, A::offset(A::add(A::add_scaled_inputs4(s.ad_value(291), 1.0, s.ad_value(278), 1.0, s.ad_value(944), 1.0, s.ad_value(293), -1.0), s.ad_value(945)), p.p1151), 1.0, 956, 1.0, 932, 1.0);
            s.store_add_scaled_inputs_product_indices(213, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));
            s.store_add_scaled_inputs_product_first_ad(367, A::add_scaled_product(s.ad_value(222), 1.0, s.ad_value(218), s.ad_value(270), (-1.0)), 1.0, 212, (-1.0), 242, 270, (-1.0));
            s.store_add_scaled_inputs_product_indices(214, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));
            s.store_sub(216, 238, 215);
            s.store_scaled_sqrt_mul_scaled_lhs(294, 953, ((2.0 * 1.602176462e-19) * s.v[180]), 270, 1.0 / (s.v[199]));
            s.store_scalar(947, (p.p1148 * (1.0 + (p.p1149 * ((s.v[184]) as f64).powf((-p.p1150))))));
            s.store_mul_offset_rhs(294, 294, 947, 1.0);
            s.store_div_from_scalar(295, 1.0, 294);
            s.store_square(296, 294);
            s.store_div_from_scalar(297, 1.0, 296);
            s.store_scalar(5, (s.v[180] / p.p74));
            s.store_scalar(7, (s.v[181] / p.p75));
            s.store_div_scaled_inputs2_indices(3, 7, 1.0, 728, 1.0, 5, 1.0);
            s.store_scalar(2, (p.p76 / p.p75));
            s.store_div(124, 294, 2);
            s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);
            s.store_scale(126, 125, 1e-7);
            s.store_scalar(127, (5.0 / 4.0));
            s.store_div_from_scalar(128, 1.0, 124);
            s.store_square(129, 124);
            s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));
        }

        s.b[1804] = (((s.v[216]) as f64).abs() <= s.v[126]);
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1804]) {
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        s.b[1805] = (s.v[216] < (-s.v[126]));
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1804])) && s.b[1805]) {
            s.store_neg(132, 216);
            s.store_mul3_lhs(133, 127, 132, 128);
            s.store_scaled_sub_offset_sqrt_square_offset(134, 133, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(135, A::sub(s.ad_value(132), s.ad_value(134)), 1.0, 129, A::offset(s.ad_value(134), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(137, 132, 2.0, 134, (-2.0), 129, -1.0);
            s.store_sub_ad_lhs(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);
            s.store_add(0, 135, 137);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 138, A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.5, s.ad_value(135), 1.0), 1.0);
            s.store_add_ad_rhs(140, 134, A::div_scaled_product3(s.ad_value(135), s.ad_value(0), s.ad_value(138), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138), s.ad_value(138)), s.ad_value(137), A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.3333333333333333, s.ad_value(135), 1.0))), 1.0));
            s.store_limited_exp(141, 140);
            s.store_sub(142, 132, 140);
            s.store_add_scaled_offset_product_rhs(143, 142, 2.0, 129, 141, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(136, 142, 1.0, 129, A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141)), 1.0);
            s.store_sub_from_scalar_scaled_mul(144, 1.0, 129, 141, 0.5);
            s.store_add_scaled_square_product_indices(142, 143, 1.0, 144, 136, (-4.0));
            s.store_scaled_div_ad_rhs(145, 136, A::add(s.ad_value(143), A::sqrt(s.ad_value(142))), 2.0);
            s.store_neg_add(131, 140, 145);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1804])) && (!s.b[1805])) {
            s.store_mul_offset_ad_lhs(146, A::mul3(s.ad_value(125), s.ad_value(127), s.ad_value(130)), (-1.0), 130);
            s.store_mul_ad_product_rhs_mixed_ia(147, 216, 128, A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));
            s.store_limited_exp_neg_input(150, 147);
            s.store_sub_from_scalar(149, 1.0, 150);
            s.store_add_scaled_inputs_product_right_ad(148, 216, 1.0, 129, 0.5, 124, A::sqrt(A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(129), 0.25, s.ad_value(149), -1.0)), (-1.0));
            s.store_limited_exp_neg_input(151, 148);
            s.store_add_scaled_inputs3_mixed_iia(152, 216, 2.0, 148, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(129), 1.0, s.ad_value(151)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(153, A::sub(s.ad_value(216), s.ad_value(148)), 1.0, 129, A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151)), (-1.0));
            s.store_sub_from_scalar_scaled_mul(154, 1.0, 129, 151, 0.5);
            s.store_add_scaled_square_product_indices(150, 152, 1.0, 154, 153, (-4.0));
            s.store_scaled_div_ad_rhs(139, 153, A::add(s.ad_value(152), A::sqrt(s.ad_value(150))), 2.0);
            s.store_add(131, 148, 139);
        }

        s.b[1806] = (((s.v[216]) as f64).abs() < s.v[126]);
        s.v[1806] = if s.b[1806] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1806]) {
            s.store_mul_ad_affine_product_rhs(46, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1806])) {
            s.store_add_scaled_inputs3_offset_mixed_aai(19, A::mul3(A::mul3(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131)), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294)), A::div_from_scalar(1.0, s.ad_value(294))), 1.0, A::limited_exp_scaled_input(s.ad_value(131), -1.0), -1.0, 131, -1.0, (-(-1.0)));
            s.store_offset_add_ad(20, A::limited_exp_scaled_input(s.ad_value(131), -1.0), A::div_scaled_product(A::square(s.ad_value(2)), A::sub_scaled_inputs(s.ad_value(131), 2.0, s.ad_value(216), 2.0), 1.0, A::square(s.ad_value(294)), 1.0), (-1.0));
            s.store_sub_div_rhs_indices(46, 131, 19, 20);
        }

        if (s.b[1620] && s.b[1799]) {
            s.store_mul(46, 46, 269);
            s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);
            s.store_div_from_scalar(96, 1.0, 95);
            s.store_add_ad_lhs(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 225);
            s.store_limited_exp_neg_input(99, 97);
            s.store_scale(101, 95, 0.001);
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_lhs_mixed_ia(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);
            s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, 3, 216, (-1.0));
            s.store_add_scaled_product_right_ad(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);
        }

        s.b[1807] = (s.v[4] < s.v[97]);
        s.v[1807] = if s.b[1807] { 1.0 } else { 0.0 };

        s.b[1808] = (s.v[214] < s.v[104]);
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        s.b[1809] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1810] = (s.v[214] < (-s.v[101]));
        s.v[1810] = if s.b[1810] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && s.b[1810]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) {
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(106, 105, 0.5, 43, 0.5, 105, 43, 40.0, (-0.5));
            s.store_add_scaled_inputs_product_mixed_aaii(107, A::square(A::sub(s.ad_value(214), s.ad_value(106))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, 4, (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1811] = (s.v[107] < 0.0);
        s.v[1811] = if s.b[1811] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) && s.b[1811]) {
            s.store_scalar(107, 0.0);
        }

        if (((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) {
            s.store_add_scaled_inputs3_mixed_iia(49, 97, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 97);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_inputs_product_mixed_aaia(120, A::square(A::sub(s.ad_value(214), s.ad_value(117))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_add_scaled_sub_value_product_rhs(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, s.ad_value(296), s.ad_value(118), (((-1.0)) * (2.0)));
            s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1812] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1812] = if s.b[1812] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1807])) && s.b[1812]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1813] = (s.v[214] < (-s.v[101]));
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && s.b[1813]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && (!s.b[1813])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (s.b[1620] && s.b[1799]) {
            s.copy_ad(123, 9);
            s.store_scalar(102, 1e-7);
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_lhs_mixed_ia(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_add_scaled_value_products(6, s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), s.ad_value(727), (-1.0), A::offset(s.ad_value(3), 1.0), s.ad_value(46), 1.0);
        }

        s.b[1814] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1814]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

    }

    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            let assign41180_ad_e67042: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign41180_ad_e67042, 2.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1799]) {
            s.copy_ad(123, 22);
        }

        s.b[1815] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1815]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            let assign41380_ad_e67733: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign41380_ad_e67733, 2.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1799]) {
            s.copy_ad(123, 22);
        }

        s.b[1816] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1816] = if s.b[1816] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1816]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            let assign41580_ad_e68424: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign41580_ad_e68424, 2.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && s.b[1799]) {
            s.store_scale(50, 269, 3.912023005);
        }

        s.b[1817] = (s.v[22] <= 0.0);
        s.v[1817] = if s.b[1817] { 1.0 } else { 0.0 };

        if ((s.b[1620] && s.b[1799]) && s.b[1817]) {
            s.store_scalar(306, 0.0);
            s.store_sub(51, 214, 22);
            s.copy_ad(312, 50);
            s.store_scalar(458, 1.0);
            s.store_scalar(334, 1.0);
            s.store_scalar(853, 1.0);
            s.store_scalar(343, 1.0);
            s.store_scalar(339, 1.0);
            s.store_scalar(363, 1.0);
            s.store_scalar(365, 1.0);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_div_from_scalar_offset_square(54, 1.0, 22, 2.0);
            s.store_mul_square_lhs(55, 22, 54);
            s.store_limited_exp(53, 22);
            s.store_div_from_scalar(56, 1.0, 53);
            s.store_limited_exp_sub(53, 22, 97);
            s.store_add_scaled_product_mixed_iaa(57, 53, 1.0, A::limited_exp_scaled_input(s.ad_value(97), -1.0), A::add(A::offset(s.ad_value(22), 1.0), s.ad_value(55)), (-1.0));
            s.store_add_scaled_product_mixed_iaa(58, 57, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(22))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);
            s.store_offset_add_scaled_inputs(58, A::offset(s.ad_value(58), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(58), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(59, 58);
            s.store_mul_sqrt_ad_rhs(61, 294, A::add(s.ad_value(58), s.ad_value(57)));
            s.store_div_scaled_product3_mixed_iiia(306, 296, 57, 269, 1.0, A::add_scaled_product(s.ad_value(61), 1.0, s.ad_value(294), s.ad_value(59), 1.0), 1.0);
            s.store_mul3_lhs(247, 59, 294, 269);
            s.copy_ad(76, 56);
            s.copy_ad(78, 57);
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
            s.store_mul_add_scaled_inputs_rhs(308, 335, s.ad_value(247), 1.0, s.ad_value(306), s.v[338]);
            s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scaled_offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0, 0.5), 1e-38))));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(308), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(309, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);
            s.store_div_from_scalar_scaled_ad(448, 1.0, A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2);
        }

        s.b[1818] = (p.p33 == 1.0);
        s.v[1818] = if s.b[1818] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1818]) {
            s.store_scalar(456, 0.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1818])) {
            s.store_offset_mul(167, 770, 306, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_add_ad_rhs(170, 169, A::sqrt_square_offset(s.ad_value(169), 0.01));
            s.store_mul_ad_affine_product_lhs(456, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2, 0.0, 652);
        }

        s.b[1819] = (p.p33 == 2.0);
        s.v[1819] = if s.b[1819] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1818])) && s.b[1819]) {
            s.store_mul_add_ad_lhs(456, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453), 652);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_mul_div_scaled_inputs_indices(310, 309, 746, 2.0, 740, 1.0);
            s.store_scale(311, 310, s.v[184]);
            s.store_mul_add_scaled_inputs_rhs(173, 742, s.ad_value(306), 1.0, s.ad_value(269), 2.0);
        }

        s.b[1820] = (s.v[456] > 0.0);
        s.v[1820] = if s.b[1820] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1820]) {
            s.store_scale(324, 746, (s.v[183] * s.v[199]));
            s.store_mul(167, 324, 456);
            s.store_scale(325, 167, 2.0);
            s.store_add_scaled_inputs_product_indices(326, 173, 1.0, 311, 1.0, 173, 167, 3.0);
            s.store_mul_add_scaled_product_rhs(327, 173, s.ad_value(311), 1.0, s.ad_value(173), s.ad_value(167), 2.0);
            s.store_div_scaled_inputs2_mixed_iai(312, 326, 1.0, A::sqrt(A::add_scaled_square_product(s.ad_value(326), 1.0, s.ad_value(325), s.ad_value(327), (-2.0))), (-1.0), 325, 1.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1820])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(312, 311, 173, 1.0, 311, 1.0, 173, 1.0, 1.0);
        }

        s.b[1821] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));
        s.v[1821] = if s.b[1821] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1821]) {
            s.store_scalar(1019, 1.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1821])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_div_scaled_inputs2_mixed_iaa(1019, 168, p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(306), p.p1351), s.ad_value(269), p.p1350), (-1.0), A::scale_offset(s.ad_value(218), p.p1352, 1.0), 1.0, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1821])) {
            s.store_scaled_add_offset_sqrt_square_offset(1019, 1019, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_offset_add_scaled_inputs(312, A::offset(s.ad_value(312), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(312), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_div(312, 312, 1019);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(312)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 270);
            s.store_add_ad_lhs(98, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 318);
            s.store_limited_exp_neg_input(100, 98);
            s.store_scale(101, 95, 0.001);
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));
            s.store_add_scaled_offset_product_lhs_mixed_aii(4, A::add_scaled_inputs3(A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(727)), -1.0), 1.0, 3, 1.0, 168, 1.0);
            s.store_add_scaled_product_right_ad(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);
        }

        s.b[1822] = (s.v[4] < s.v[98]);
        s.v[1822] = if s.b[1822] { 1.0 } else { 0.0 };

        s.b[1823] = (s.v[214] < s.v[104]);
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        s.b[1824] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && s.b[1824]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1825] = (s.v[214] < (-s.v[101]));
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && s.b[1825]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && (!s.b[1825])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(106, 105, 0.5, 43, 0.5, 105, 43, 40.0, (-0.5));
            s.store_add_scaled_inputs_product_mixed_aaii(107, A::square(A::sub(s.ad_value(214), s.ad_value(106))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, 4, (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1826] = (s.v[107] < 0.0);
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) && s.b[1826]) {
            s.store_scalar(107, 0.0);
        }

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) {
            s.store_add_scaled_inputs3_mixed_iia(49, 98, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 98);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_inputs_product_mixed_aaia(120, A::square(A::sub(s.ad_value(214), s.ad_value(117))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_add_scaled_sub_value_product_rhs(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, s.ad_value(296), s.ad_value(118), (((-1.0)) * (2.0)));
            s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1827] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && s.b[1827]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1828] = (s.v[214] < (-s.v[101]));
        s.v[1828] = if s.b[1828] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && s.b[1828]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
        }

    }

    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && s.b[1828]) {
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && (!s.b[1828])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.copy_ad(123, 9);
            s.store_scalar(102, 1e-7);
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));
            s.store_add_scaled_inputs_product_mixed_aaai(6, A::add_scaled_product(s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), s.ad_value(727), (-1.0)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), s.ad_value(168), s.ad_value(269)), 1.0, A::offset(s.ad_value(3), 1.0), 46, 1.0);
        }

        s.b[1829] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1829]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            let assign44080_ad_e74052: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign44080_ad_e74052, 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.copy_ad(123, 23);
        }

        s.b[1830] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1830] = if s.b[1830] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1830]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            let assign44280_ad_e74802: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign44280_ad_e74802, 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.copy_ad(123, 23);
        }

        s.b[1831] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1831] = if s.b[1831] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1831]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));
        }

    }

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            let assign44480_ad_e75552: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign44480_ad_e75552, 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_sub(62, 23, 22);
            s.store_mul(63, 226, 270);
            s.store_limited_exp_neg_input(64, 63);
        }

        s.b[1832] = (s.v[62] < 1e-10);
        s.v[1832] = if s.b[1832] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)));
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_limited_exp_ad(178, A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(269), 2.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(179, A::add(A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(269), 2.0, s.ad_value(269), 1.0), s.ad_value(170)));
            s.store_neg_ad(65, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(63), (-1.0), s.ad_value(98), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(63), -1.0, s.ad_value(98), 1.0)), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), 1.0, (-1.0)));
            s.store_mul_ad_lhs(66, A::mul_sub_from_scalar_rhs(s.ad_value(296), 1.0, s.ad_value(64)), 57);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            let assign44680_ad_e76260: A = A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product3_by_product(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0, A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), (-1.0), s.ad_value(296), A::sub(A::sub(A::add_scaled_inputs4(A::add_scaled_inputs_product(s.ad_value(173), 1.0, A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(98), (-1.0), s.ad_value(63), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(63), 1.0)), A::sub(A::add_scaled_product(s.ad_value(175), (-2.0), A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(175), 10.0), s.ad_value(175), 1.0), A::mul3(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 8.0), s.ad_value(123), s.ad_value(175)), s.ad_value(175), s.ad_value(175))), 1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), (-1.0), A::div(s.ad_value(178), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), 1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), A::div(s.ad_value(179), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), A::div(s.ad_value(179), A::mul(A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), (-1.0));
            s.store_offset_sub_ad(54, assign44680_ad_e76260, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            s.store_add_scaled_square_product_indices(54, 65, 1.0, 54, 66, (-2.0));
        }

        s.b[1833] = (s.v[54] >= 0.0);
        s.v[1833] = if s.b[1833] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) && s.b[1833]) {
            s.store_scaled_div_ad_rhs(62, 66, A::add(s.ad_value(65), A::sqrt(s.ad_value(54))), 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            s.store_add(23, 22, 62);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_mul(250, 62, 269);
            s.store_div_scaled_product_offset_denominator(67, s.ad_value(23), s.ad_value(23), 1.0, A::square(s.ad_value(23)), 2.0, 1.0);
            s.store_limited_exp_neg_input(68, 23);
            s.store_add_scaled_product(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67)), (-1.0));
            s.store_add_scaled_product_mixed_iaa(70, 69, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(23))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);
            s.store_offset_add_scaled_inputs(70, A::offset(s.ad_value(70), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(70), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(60, 70);
            s.store_mul_sqrt_ad_rhs(72, 294, A::add(s.ad_value(70), s.ad_value(69)));
            s.store_div_scaled_product3_mixed_iiia(73, 296, 69, 269, 1.0, A::add_scaled_product(s.ad_value(72), 1.0, s.ad_value(294), s.ad_value(60), 1.0), 1.0);
            s.store_scaled_add(75, 22, 23, 0.5);
            s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));
            s.store_sqrt(76, 54);
            s.store_scaled_add(77, 57, 69, 0.5);
            s.store_add_scaled_product_mixed_iaa(78, 77, 1.0, A::square(s.ad_value(62)), A::sub_scaled_inputs(s.ad_value(76), 1.0, s.ad_value(297), 2.0), 0.125);
            s.store_add_scaled_product_mixed_iaa(79, 78, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(75))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);
            s.store_mul_sqrt_ad_rhs(51, 294, A::add(s.ad_value(78), s.ad_value(79)));
            s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(79), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(71, 79);
        }

        s.b[1834] = (p.p46 == 1.0);
        s.v[1834] = if s.b[1834] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1834]) {
            s.store_div_scaled_inputs_indices(85, 269, ((2.0 * s.v[199]) * s.v[199]), 704, (1.602176462e-19 * s.v[180]));
            s.store_add_scaled_sub_value_product_mixed_iia(86, 1.0, 76, 1.0, 51, A::div_from_scalar(1.0, s.ad_value(296)), 2.0);
            s.store_div_from_scalar_sqrt_ad(87, 1.0, A::offset(A::mul(s.ad_value(85), s.ad_value(51)), 1.0));
            s.store_div_scaled_value_offset_denominator(54, s.ad_value(87), 1.0, s.ad_value(87), 1.0, 1.0);
            s.store_mul_ad_product_rhs(88, 85, A::mul3(A::square(s.ad_value(54)), s.ad_value(51), s.ad_value(51)), A::div(s.ad_value(78), A::add(s.ad_value(78), s.ad_value(79))));
            s.store_add_scaled_inputs_product_right_ad(89, 51, 2.0, 88, (-2.0), 296, A::add(A::sub_from_scalar(1.0, s.ad_value(76)), s.ad_value(78)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(90, 88, s.ad_value(88), 1.0, s.ad_value(51), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(91, 1.0, 296, A::add(s.ad_value(76), s.ad_value(78)), 0.5);
            s.store_div_scaled_product_denominator_ad(92, 90, 89, 1.0, A::add_scaled_square_product(s.ad_value(89), 1.0, s.ad_value(91), s.ad_value(90), (-1.0)), 1.0);
            s.store_add(75, 75, 92);
            s.store_limited_exp(93, 92);
            s.store_div(76, 76, 93);
            s.store_mul(78, 78, 93);
            s.store_add_scaled_product(79, A::div(s.ad_value(78), s.ad_value(93)), (-1.0), A::square(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(75), (-1.0), s.ad_value(92), 1.0)), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);
            s.store_mul_sqrt_ad_rhs(51, 294, A::add(s.ad_value(78), s.ad_value(79)));
            s.store_add_ad(94, A::sub_from_scalar(1.0, s.ad_value(76)), A::mul3_scaled_output(s.ad_value(51), s.ad_value(87), s.ad_value(297), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(62, 62, 93, A::add(s.ad_value(86), s.ad_value(77)), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(93), s.ad_value(77), 1.0), 1.0);
            s.store_mul(250, 62, 269);
            s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(79), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(71, 79);
        }

        s.b[1835] = (((s.v[250]) as f64).abs() > 1e-35);
        s.v[1835] = if s.b[1835] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1835]) {
            s.store_div_scaled_inputs2_indices(74, 306, 1.0, 73, (-1.0), 250, 1.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_mul_div_scaled_product_mixed_iiia(80, 269, 296, 78, 1.0, A::add_scaled_product(s.ad_value(51), 1.0, s.ad_value(294), s.ad_value(71), 1.0), 1.0);
            s.store_add_scaled_product_indices(81, 80, 1.0, 269, 74, 1.0);
        }

        if (s.b[1620] && s.b[1799]) {
            s.store_mul_ad_product_lhs(939, A::mul3_scaled_output(s.ad_value(740), s.ad_value(81), s.ad_value(250), ((p.p2 * (p.p1147 / s.v[184])) * s.v[199])), A::div_scaled_product(s.ad_value(354), s.ad_value(344), 1.0, s.ad_value(458), 1.0), 363);
            s.store_add(380, 380, 939);
        }

        if (!s.b[1620]) {
            s.store_div(252, 251, 267);
            s.store_scalar(168, 1.0);
            s.store_div(404, 213, 168);
            s.store_div(405, 294, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1836] = (s.v[404] < 0.0);
        s.v[1836] = if s.b[1836] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1836]) {
            s.store_div_scaled_inputs2_indices(170, 404, 1.0, 169, (-1.0), 405, 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if ((!s.b[1620]) && (!s.b[1836])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_offset_ad_lhs(254, A::square(s.ad_value(169)), 1.0, 170);
        }

        if (!s.b[1620]) {
            s.store_scaled_add_offset_sqrt_square_offset(175, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(294), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(294), 1.0);
            s.store_add_scaled_inputs3_indices(168, 254, 1.0, 252, (-2.0), 225, -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1837] = (s.v[175] <= (-68.0));
        s.v[1837] = if s.b[1837] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1837]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1838] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1838] = if s.b[1838] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1837]) && s.b[1838]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1839] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1839] = if s.b[1839] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1837]) && (!s.b[1838])) && s.b[1839]) {
            s.store_limited_exp(170, 175);
        }

        if ((((!s.b[1620]) && s.b[1837]) && (!s.b[1838])) && (!s.b[1839])) {
            s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if ((!s.b[1620]) && s.b[1837]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(400, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1837])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_square_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3_mixed_aai(174, A::square(A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), 173, -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(400, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if (!s.b[1620]) {
            s.store_scaled_add_offset_sqrt_square_offset(256, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_sqrt(259, 256);
            s.store_sub_scaled_inputs(255, 254, 1.0, 400, 2.0);
            s.store_scaled_add_offset_sqrt_square_offset(167, 255, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_offset_div_ad(253, s.ad_value(294), A::add(s.ad_value(259), A::sqrt(s.ad_value(167))), 1.0);
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
            s.store_mul_ad_rhs(167, 269, A::add_scaled_inputs_product(s.ad_value(213), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_sqrt_square_offset_rhs(247, 167, 167, ((0.25 * 0.1) * 0.1), 0.5);
            s.store_mul3_affine_lhs(306, 253, 269, 2.0, 0.0, 400);
        }

    }

    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {
            s.store_mul_add_scaled_inputs_rhs(308, 335, s.ad_value(247), 1.0, s.ad_value(306), s.v[338]);
            s.store_pow_ad(169, A::scaled_offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0, 0.5), s.ad_value(757));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(308), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(309, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);
            s.store_div_from_scalar_scaled_ad(448, 1.0, A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2);
        }

        s.b[1840] = (p.p33 == 1.0);
        s.v[1840] = if s.b[1840] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1840]) {
            s.store_scalar(456, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[1840])) {
            s.store_offset_mul(167, 770, 306, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_add_ad_rhs(170, 169, A::sqrt_square_offset(s.ad_value(169), 0.01));
        }

        s.b[1841] = (p.p33 == 0.0);
        s.v[1841] = if s.b[1841] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1840])) && s.b[1841]) {
            s.store_mul_ad_affine_product_lhs(456, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2, 0.0, 652);
        }

        if (((!s.b[1620]) && (!s.b[1840])) && (!s.b[1841])) {
            s.store_mul_add_ad_lhs(456, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453), 652);
        }

        if (!s.b[1620]) {
            s.store_pow_ad(167, s.ad_value(309), A::div_from_scalar(1.0, s.ad_value(348)));
            s.store_mul(178, 678, 218);
            s.store_sqrt_square_offset(179, 178, 0.1);
            s.store_scaled_add_ad(168, A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::square(A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179))), 0.5);
            s.store_div_scaled_product_offset_denominator(169, s.ad_value(400), s.ad_value(168), (10.0 * p.p497), A::mul(s.ad_value(400), s.ad_value(168)), (10.0 * p.p497), 1.0);
        }

        s.b[1842] = (s.v[780] < 0.0);
        s.v[1842] = if s.b[1842] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1842]) {
            s.store_scaled_mul_ad(314, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(269), 1.0, s.ad_value(167), s.ad_value(746), s.v[184]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(780), s.ad_value(169)))), 2.0);
        }

        if ((!s.b[1620]) && (!s.b[1842])) {
            s.store_scaled_mul_ad(314, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(269), 1.0, s.ad_value(167), s.ad_value(746), s.v[184]), A::offset(A::mul(s.ad_value(780), s.ad_value(169)), 1.0), 2.0);
        }

        s.b[1843] = (s.v[456] > 0.0);
        s.v[1843] = if s.b[1843] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_mul3_affine_lhs(178, 253, 269, ((s.v[183] * 2.0) * s.v[199]), 0.0, 746);
            s.store_div_scaled_product3_indices(179, 178, 314, 456, 1.0, 269, 2.0);
            s.store_div_scaled_product_offset_denominator(167, s.ad_value(314), A::add(A::square(s.ad_value(400)), s.ad_value(400)), 0.5, A::mul_scaled_lhs(s.ad_value(314), 0.5, A::offset(s.ad_value(400), 1.0)), 1.0, 1.0);
            s.store_mul_sub_scaled_inputs_rhs(168, 314, s.ad_value(400), 2.0, s.ad_value(167), 2.0);
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1844] = (s.v[168] != 0.0);
        s.v[1844] = if s.b[1844] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1843]) && s.b[1844]) {
            s.store_asinh(323, 168);
            s.store_add_scaled_product_left_ad(170, 169, 1.0, A::div_from_scalar(1.0, s.ad_value(168)), 323, 1.0);
        }

        if (((!s.b[1620]) && s.b[1843]) && (!s.b[1844])) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_add_scaled_value_products(171, A::mul3(s.ad_value(179), s.ad_value(167), A::offset(A::add(s.ad_value(400), s.ad_value(167)), 1.0)), 1.0, s.ad_value(167), s.ad_value(170), 1.0, s.ad_value(314), A::add_scaled_inputs4(A::square(s.ad_value(400)), 1.0, s.ad_value(400), 1.0, A::square(s.ad_value(167)), -1.0, s.ad_value(167), -1.0), (-1.0));
        }

        s.b[1845] = (s.v[168] != 0.0);
        s.v[1845] = if s.b[1845] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1843]) && s.b[1845]) {
            s.store_div_scaled_product_mixed_iaa(172, 314, A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);
        }

        if (((!s.b[1620]) && s.b[1843]) && (!s.b[1845])) {
            s.store_mul_div_scaled_inputs_indices(172, 314, 168, (-2.0), 169, 1.0);
        }

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_add_scaled_value_products3_mixed_iiiiaia(173, 170, 1.0, 167, 172, 1.0, 179, A::offset(A::add_scaled_inputs(s.ad_value(400), 1.0, s.ad_value(167), 2.0), 1.0), 1.0, 314, A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0);
            s.store_sub_div_rhs_indices(167, 167, 171, 173);
            s.store_mul_sub_scaled_inputs_rhs(168, 314, s.ad_value(400), 2.0, s.ad_value(167), 2.0);
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1846] = (s.v[168] != 0.0);
        s.v[1846] = if s.b[1846] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1843]) && s.b[1846]) {
            s.store_asinh(323, 168);
            s.store_add_scaled_product_left_ad(170, 169, 1.0, A::div_from_scalar(1.0, s.ad_value(168)), 323, 1.0);
        }

        if (((!s.b[1620]) && s.b[1843]) && (!s.b[1846])) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_add_scaled_value_products(171, A::mul3(s.ad_value(179), s.ad_value(167), A::offset(A::add(s.ad_value(400), s.ad_value(167)), 1.0)), 1.0, s.ad_value(167), s.ad_value(170), 1.0, s.ad_value(314), A::add_scaled_inputs4(A::square(s.ad_value(400)), 1.0, s.ad_value(400), 1.0, A::square(s.ad_value(167)), -1.0, s.ad_value(167), -1.0), (-1.0));
        }

        s.b[1847] = (s.v[168] != 0.0);
        s.v[1847] = if s.b[1847] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1843]) && s.b[1847]) {
            s.store_div_scaled_product_mixed_iaa(172, 314, A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);
        }

        if (((!s.b[1620]) && s.b[1843]) && (!s.b[1847])) {
            s.store_mul_div_scaled_inputs_indices(172, 314, 168, (-2.0), 169, 1.0);
        }

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_add_scaled_value_products3_mixed_iiiiaia(173, 170, 1.0, 167, 172, 1.0, 179, A::offset(A::add_scaled_inputs(s.ad_value(400), 1.0, s.ad_value(167), 2.0), 1.0), 1.0, 314, A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0);
            s.store_sub_div_rhs_indices(307, 167, 171, 173);
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_div_scaled_product_offset_denominator(167, s.ad_value(314), A::add(A::square(s.ad_value(400)), s.ad_value(400)), 0.5, A::mul_scaled_lhs(s.ad_value(314), 0.5, A::offset(s.ad_value(400), 1.0)), 1.0, 1.0);
            s.store_mul_sub_scaled_inputs_rhs(168, 314, s.ad_value(400), 2.0, s.ad_value(167), 2.0);
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1848] = (s.v[168] != 0.0);
        s.v[1848] = if s.b[1848] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1848]) {
            s.store_asinh(323, 168);
            s.store_add_scaled_product_left_ad(170, 169, 1.0, A::div_from_scalar(1.0, s.ad_value(168)), 323, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1848])) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_add_scaled_products_right_right_ad(171, 167, 170, 1.0, 314, A::add_scaled_inputs4(A::square(s.ad_value(400)), 1.0, s.ad_value(400), 1.0, A::square(s.ad_value(167)), -1.0, s.ad_value(167), -1.0), (-1.0));
        }

        s.b[1849] = (s.v[168] != 0.0);
        s.v[1849] = if s.b[1849] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1849]) {
            s.store_div_scaled_product_mixed_iaa(172, 314, A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1849])) {
            s.store_mul_div_scaled_inputs_indices(172, 314, 168, (-2.0), 169, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_add_scaled_value_products(173, s.ad_value(170), 1.0, s.ad_value(167), s.ad_value(172), 1.0, s.ad_value(314), A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0);
            s.store_sub_div_rhs_indices(167, 167, 171, 173);
            s.store_mul_sub_scaled_inputs_rhs(168, 314, s.ad_value(400), 2.0, s.ad_value(167), 2.0);
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1850] = (s.v[168] != 0.0);
        s.v[1850] = if s.b[1850] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1850]) {
            s.store_asinh(323, 168);
            s.store_add_scaled_product_left_ad(170, 169, 1.0, A::div_from_scalar(1.0, s.ad_value(168)), 323, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1850])) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_add_scaled_products_right_right_ad(171, 167, 170, 1.0, 314, A::add_scaled_inputs4(A::square(s.ad_value(400)), 1.0, s.ad_value(400), 1.0, A::square(s.ad_value(167)), -1.0, s.ad_value(167), -1.0), (-1.0));
        }

        s.b[1851] = (s.v[168] != 0.0);
        s.v[1851] = if s.b[1851] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1851]) {
            s.store_div_scaled_product_mixed_iaa(172, 314, A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1851])) {
            s.store_mul_div_scaled_inputs_indices(172, 314, 168, (-2.0), 169, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_add_scaled_value_products(173, s.ad_value(170), 1.0, s.ad_value(167), s.ad_value(172), 1.0, s.ad_value(314), A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0);
            s.store_sub_div_rhs_indices(307, 167, 171, 173);
        }

        if (!s.b[1620]) {
            s.store_add_scaled_inputs4_mixed_iiia(319, 254, 1.0, 252, (-2.0), 307, (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::add(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(294), 1.0, s.ad_value(253), (-1.0), 1.0))), 1e-38)), -1.0);
            s.store_mul(312, 319, 269);
        }

        s.b[1852] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));
        s.v[1852] = if s.b[1852] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1852]) {
            s.store_scalar(1019, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1852])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_div_scaled_inputs2_mixed_iaa(1019, 168, p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(400), p.p1351), s.ad_value(269), p.p1350), (-1.0), A::scale_offset(s.ad_value(218), p.p1352, 1.0), 1.0, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(1019, 1019, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);
        }

        if (!s.b[1620]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(316, 312, 0.5, 224, ((-1.0) * 0.5), 312, 224, ((0.25 * 0.001) * 0.001), 0.5);
            s.store_div(316, 316, 1019);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(316)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 270);
            s.store_scaled_add_offset_sqrt_square_offset(175, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_sqrt(259, 175);
            s.store_div_scaled_offset_numerator(167, A::div_scaled_inputs(s.ad_value(294), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(294), 1.0);
            s.store_add_scaled_inputs3_indices(168, 254, 1.0, 252, (-2.0), 318, -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1853] = (s.v[175] <= (-68.0));
        s.v[1853] = if s.b[1853] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1853]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1854] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1854] = if s.b[1854] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1853]) && s.b[1854]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1855] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1855] = if s.b[1855] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1853]) && (!s.b[1854])) && s.b[1855]) {
            s.store_limited_exp(170, 175);
        }

        if ((((!s.b[1620]) && s.b[1853]) && (!s.b[1854])) && (!s.b[1855])) {
            s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if ((!s.b[1620]) && s.b[1853]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(320, 170, s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1853])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_div_rhs_indices(170, 170, 171, 172);
            s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_square_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3_mixed_aai(174, A::square(A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), 173, -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iaa(320, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));
        }

        if (!s.b[1620]) {
            s.store_add_scaled_inputs3_offset_indices(255, 254, 1.0, 400, (-1.0), 320, -1.0, (-1.0));
            s.store_scaled_add_offset_sqrt_square_offset(167, 255, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_sqrt(169, 167);
            s.store_offset_div_ad(253, s.ad_value(294), A::add(s.ad_value(259), s.ad_value(169)), 1.0);
            s.store_square_ad(417, A::sub(s.ad_value(400), s.ad_value(320)));
            s.store_div_from_scalar_add_ad(167, 1.0, A::offset(s.ad_value(400), 1.0), s.ad_value(320));
            s.store_mul(168, 417, 167);
            s.store_add_scaled_inputs_product_mixed_iiaa(381, 213, 1.0, 254, (-1.0), A::offset(s.ad_value(253), (-1.0)), A::add_scaled_inputs3(s.ad_value(400), 1.0, s.ad_value(320), 1.0, s.ad_value(168), 0.3333333333333333), (-1.0));
            s.store_scale(169, 253, 0.3333333333333333);
            s.store_mul(170, 168, 167);
            s.store_mul_ad_rhs(382, 169, A::add_scaled_inputs_product(s.ad_value(400), 2.0, s.ad_value(320), 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(400), 0.8, 1.0), 1.0, s.ad_value(320), 1.2), s.ad_value(170), 0.5));
            s.store_mul_ad_rhs(385, 169, A::add_scaled_inputs_product(s.ad_value(400), 1.0, s.ad_value(320), 2.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(400), 1.2, 1.0), 1.0, s.ad_value(320), 0.8), s.ad_value(170), 0.5));
            s.store_add_scaled_product_value_ad(244, A::sqrt_square_offset(A::mul(s.ad_value(269), s.ad_value(381)), ((0.25 * 0.1) * 0.1)), 0.5, 269, 381, 0.5);
            s.store_mul_add_rhs(243, 269, 382, 385);
            s.store_mul_add_scaled_inputs_rhs(336, 335, s.ad_value(244), 1.0, s.ad_value(243), s.v[338]);
        }

    }

    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {
            s.store_pow_ad(169, A::scaled_offset(A::div(s.ad_value(243), s.ad_value(244)), 1.0, 0.5), s.ad_value(757));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(336), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(339, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);
            s.store_div_scaled_inputs_mixed_ia(310, 746, 2.0, A::div(s.ad_value(740), s.ad_value(339)), 1.0);
            s.store_scale(311, 310, s.v[184]);
        }

        s.b[1856] = (s.v[781] > 0.0);
        s.v[1856] = if s.b[1856] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1856]) {
            s.store_offset_div_scaled_product(360, s.ad_value(781), s.ad_value(243), 1.0, s.ad_value(311), 1.0, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1856])) {
            s.store_div_from_scalar_sub_from_scalar_ad(360, 1.0, 1.0, A::div_scaled_product(s.ad_value(781), s.ad_value(243), 1.0, s.ad_value(311), 1.0));
        }

        if (!s.b[1620]) {
            s.copy_ad(359, 763);
            s.store_sub(355, 226, 315);
            s.store_add_scaled_inputs(362, 243, 1.0, 269, 2.0);
        }

        s.b[1857] = (s.v[359] > 0.0);
        s.v[1857] = if s.b[1857] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1857]) {
            s.store_div_add_scaled_inputs_rhs_indices(170, 362, 316, 1.0, 362, 1.0);
            s.store_scaled_add_sqrt_square_offset_ad(171, A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), ((4.0 * 0.001) * 0.001), 0.5);
            s.store_div_from_scalar(172, 1.0, 171);
            s.store_mul_product3_mixed_iaii(361, 172, A::div(s.ad_value(362), s.ad_value(359)), 170, 360, 1.0);
            s.store_offset_div(363, 355, 361, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1857])) {
            s.store_scalar(363, 1.0);
        }

        s.b[1858] = (s.v[769] <= 0.0);
        s.v[1858] = if s.b[1858] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1858]) {
            s.store_scalar(268, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1858])) {
            s.store_div_scaled_inputs_indices(176, 769, ((s.v[184]) as f64).sqrt(), 362, 1.0);
            s.store_div_from_scalar_offset_input(268, 1.0, 176, 1.0);
        }

        if (!s.b[1620]) {
            s.store_add(358, 316, 311);
        }

        s.b[1859] = (s.v[785] > 0.0);
        s.v[1859] = if s.b[1859] { 1.0 } else { 0.0 };

        s.b[1860] = (p.p414 < 0.0);
        s.v[1860] = if s.b[1860] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1859]) && s.b[1860]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(785), 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0)), s.ad_value(268), 1.0);
        }

        if (((!s.b[1620]) && s.b[1859]) && (!s.b[1860])) {
            s.store_div_scaled_product_offset_rhs(168, s.ad_value(785), A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, s.ad_value(268), 1.0);
        }

        if ((!s.b[1620]) && s.b[1859]) {
            s.store_offset_mul_ad(364, s.ad_value(168), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(355), 1.0, s.ad_value(168), s.ad_value(358), 1.0), 1.0), 1e-38)), 1.0);
        }

        s.b[1861] = (p.p414 < 0.0);
        s.v[1861] = if s.b[1861] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1859])) && s.b[1861]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(785), 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0)), s.ad_value(268), 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1859])) && (!s.b[1861])) {
            s.store_div_scaled_product_offset_rhs(168, s.ad_value(785), A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, s.ad_value(268), 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1859])) {
            s.store_offset(364, 168, 1.0);
        }

        if (!s.b[1620]) {
            s.store_mul(363, 363, 364);
            s.store_limited_exp_mul(168, 768, 226);
        }

        s.b[1862] = (s.v[767] > 0.0);
        s.v[1862] = if s.b[1862] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1862]) {
            s.store_scalar(169, (1.0 + (p.p433 * s.v[184])));
            s.store_div_scaled_offset_numerator(356, A::mul(s.ad_value(169), s.ad_value(168)), 1.0, 1.0, s.ad_value(767), 1.0);
            s.store_mul(356, 356, 268);
        }

        if ((!s.b[1620]) && (!s.b[1862])) {
            s.store_scalar(356, 5.540622384e34);
        }

        if (!s.b[1620]) {
            s.store_div(171, 355, 356);
            s.store_offset(167, 171, 1.0);
            s.store_mul(363, 363, 167);
        }

        s.b[1863] = (s.v[766] > 0.0);
        s.v[1863] = if s.b[1863] { 1.0 } else { 0.0 };

        s.b[1864] = (s.v[355] > ((s.v[765] * s.v[300]) / 80.0));
        s.v[1864] = if s.b[1864] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1863]) && s.b[1864]) {
            s.store_div_scaled_product_indices(167, 765, 300, 1.0, 355, 1.0);
            s.store_div_scaled_inputs_mixed_ai(357, A::limited_exp(s.ad_value(167)), s.v[184], 766, 1.0);
        }

        if (((!s.b[1620]) && s.b[1863]) && (!s.b[1864])) {
            s.store_div_from_scalar(357, (5.540622384e34 * s.v[184]), 766);
        }

        if ((!s.b[1620]) && (!s.b[1863])) {
            s.store_scalar(357, 5.540622384e34);
        }

        if (!s.b[1620]) {
            s.store_offset_div(365, 355, 357, 1.0);
            s.store_mul(363, 363, 365);
            s.store_pow_ad(167, s.ad_value(339), A::div_from_scalar(1.0, s.ad_value(348)));
            s.store_mul(178, 678, 218);
            s.store_sqrt_square_offset(179, 178, 0.1);
            s.store_scaled_add_ad(168, A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::square(A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179))), 0.5);
            s.store_div_scaled_product_offset_denominator(169, s.ad_value(243), s.ad_value(168), (10.0 * p.p497), A::mul(s.ad_value(243), s.ad_value(168)), (10.0 * p.p497), 1.0);
        }

        s.b[1865] = (s.v[780] < 0.0);
        s.v[1865] = if s.b[1865] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1865]) {
            s.store_scaled_mul_ad(314, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(269), 1.0, s.ad_value(167), s.ad_value(746), s.v[184]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(780), s.ad_value(169)))), 2.0);
        }

        if ((!s.b[1620]) && (!s.b[1865])) {
            s.store_scaled_mul_ad(314, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(269), 1.0, s.ad_value(167), s.ad_value(746), s.v[184]), A::offset(A::mul(s.ad_value(780), s.ad_value(169)), 1.0), 2.0);
        }

        if (!s.b[1620]) {
            s.store_mul_sub_scaled_inputs_rhs(168, 314, s.ad_value(400), 2.0, s.ad_value(320), 2.0);
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1866] = (s.v[168] != 0.0);
        s.v[1866] = if s.b[1866] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1866]) {
            s.store_add_scaled_product_mixed_iaa(343, 169, 0.5, A::div_from_scalar(1.0, s.ad_value(168)), A::asinh(s.ad_value(168)), 0.5);
        }

        if ((!s.b[1620]) && (!s.b[1866])) {
            s.store_scaled_add_ad_rhs(343, 169, A::div_from_scalar(1.0, s.ad_value(169)), 0.5);
        }

        if (!s.b[1620]) {
            s.copy_ad(345, 343);
            s.store_scalar(454, 0.0);
            s.store_scalar(455, 0.0);
        }

        s.b[1867] = (p.p33 == 1.0);
        s.v[1867] = if s.b[1867] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1867]) {
            s.store_scalar(457, 0.0);
            s.store_scalar(458, 1.0);
            s.store_sub(169, 203, 219);
            s.store_sqrt_square_offset(170, 169, 0.01);
            s.store_scaled_add(228, 169, 170, 0.5);
            s.store_offset_mul(172, 770, 228, 1.0);
            s.store_add_scaled_product_value_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 202, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(171, 173, 173, 0.01, 0.5);
            s.store_mul_add_scaled_product_rhs(454, 652, s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(773), 1.0, s.ad_value(775), s.ad_value(171), 1.0), s.ad_value(448), 1.0);
            s.store_sub(169, 204, 219);
            s.store_sqrt_square_offset(170, 169, 0.01);
            s.store_scaled_add(229, 169, 170, 0.5);
            s.store_offset_mul(172, 770, 229, 1.0);
            s.store_add_scaled_product_value_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 201, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(171, 173, 173, 0.01, 0.5);
            s.store_mul_add_scaled_product_rhs(455, 652, s.ad_value(453), 1.0, A::add_scaled_product(s.ad_value(772), 1.0, s.ad_value(774), s.ad_value(171), 1.0), s.ad_value(448), 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1867])) {
            s.store_offset_mul(167, 770, 243, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_scaled_add_sqrt_square_offset_rhs(170, 169, 169, 0.01, 0.5);
            s.store_mul_ad_affine_product_lhs(457, s.ad_value(652), A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), p.p2, 0.0, 448);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
            s.store_offset_product3(458, A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);
        }

        s.b[1868] = (p.p33 == 2.0);
        s.v[1868] = if s.b[1868] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1867])) && s.b[1868]) {
            s.store_mul_add_ad_rhs(457, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453));
            s.store_scalar(455, 0.0);
            s.store_scalar(454, 0.0);
            s.store_offset_product3(458, A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);
        }

        if (!s.b[1620]) {
            s.store_add_div_rhs_mixed_ia(167, 330, 333, A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(267), s.ad_value(637), 2.0));
            s.store_sub(416, 400, 320);
            s.store_mul3_lhs(168, 167, 416, 416);
            s.store_offset(169, 168, ((1.0) + ((-0.001))));
            s.store_offset_add_scaled_inputs_mixed_ia(170, 169, 0.5, A::sqrt_square_offset(s.ad_value(169), 0.004), 0.5, (-1.0));
            s.store_scaled_offset_ad(334, A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0, 0.5);
            s.store_offset_sub_scaled_inputs(334, A::offset(s.ad_value(334), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(334), (-1.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));
            s.store_add(167, 400, 320);
            s.store_sub(168, 400, 320);
            s.store_div_add_scaled_inputs_rhs_indices(169, 168, 167, 1.0, 833, 1.0);
            s.store_mul3_lhs(170, 832, 169, 169);
            s.store_offset(834, 170, 1.0);
            s.store_div_ad_rhs(176, 858, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul3(s.ad_value(864), s.ad_value(168), s.ad_value(168)))), s.ad_value(167), 1.0, s.ad_value(267), s.ad_value(637), 2.0));
            s.store_limited_exp_neg_input(853, 176);
            s.store_mul3_lhs(340, 339, 343, 458);
            s.store_div(337, 740, 340);
            s.store_mul_ad_product_lhs_mixed_ai(380, A::div_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(253), s.ad_value(337), s.ad_value(269), ((2.0 * p.p2) * ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]))), s.ad_value(269), A::mul(A::sub(s.ad_value(400), s.ad_value(320)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)))), s.ad_value(363), 1.0, s.ad_value(334), 1.0), 834, 853);
            s.store_scale(380, 380, p.p26);
            s.store_scalar(467, 0.0);
        }

        s.b[1869] = (p.p7 > 1.0);
        s.v[1869] = if s.b[1869] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1869]) {
            s.store_scaled_mul(468, 337, 243, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));
            s.store_scale(176, 271, p.p1009);
            s.store_scaled_mul(167, 176, 337, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));
            s.store_scaled_add(467, 167, 468, (p.p1008 * p.p2));
        }

        s.b[1870] = (p.p7 == 2.0);
        s.v[1870] = if s.b[1870] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1869]) && s.b[1870]) {
            s.store_div_from_scalar(466, 1.0, 465);
        }

        s.b[1871] = (s.v[466] < p.p1347);
        s.v[1871] = if s.b[1871] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1869]) && s.b[1870]) && s.b[1871]) {
            s.store_scalar(466, p.p1347);
            s.store_div_from_scalar(465, 1.0, 466);
        }

        if (((!s.b[1620]) && s.b[1869]) && s.b[1870]) {
            s.store_add(178, 465, 467);
            s.store_div_scaled_product_indices(467, 465, 467, 1.0, 178, 1.0);
        }

        if (!s.b[1620]) {
            s.store_scalar(544, ((s.v[183] / p.p1373) + p.p1377));
            s.store_scalar(543, ((s.v[183] / p.p1373) + p.p1378));
            s.store_scale(545, 543, p.p74);
            s.store_scale(546, 544, p.p74);
        }

    }

    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {
            s.store_mul(593, 637, 590);
            s.store_div(167, 498, 593);
            s.store_limited_exp(595, 167);
            s.store_mul(594, 637, 590);
            s.store_div(167, 499, 594);
            s.store_limited_exp(596, 167);
            s.store_mul_offset_rhs_ad_lhs(171, A::div_from_scalar(1.115, s.ad_value(637)), 639, (-1.0));
        }

        s.b[1872] = (s.v[550] == 0.0);
        s.v[1872] = if s.b[1872] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1872])) {
            s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);
            s.store_limited_exp(168, 174);
            s.store_mul(548, 550, 168);
            s.store_mul(167, 545, 548);
        }

        s.b[1873] = (s.v[551] == 0.0);
        s.v[1873] = if s.b[1873] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1873])) {
            s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);
            s.store_limited_exp(168, 174);
            s.store_mul(549, 551, 168);
            s.store_mul(167, 546, 549);
        }

        s.b[1874] = (s.v[552] == 0.0);
        s.v[1874] = if s.b[1874] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1874])) {
            s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);
            s.store_limited_exp(169, 174);
            s.store_mul(554, 552, 169);
            s.store_mul_scaled_offset_ad_rhs(562, 557, p.p925, A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);
            s.store_mul_scaled_offset_ad_rhs(563, 564, p.p925, A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);
            s.store_div(167, 498, 562);
            s.store_limited_exp(177, 167);
        }

        s.b[1875] = ((s.v[558] - s.v[498]) < 0.001);
        s.v[1875] = if s.b[1875] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1874])) && s.b[1875]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if (((!s.b[1620]) && (!s.b[1874])) && (!s.b[1875])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(558), s.ad_value(498));
            s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if ((!s.b[1620]) && (!s.b[1874])) {
            s.store_mul(170, 545, 554);
        }

        s.b[1876] = (s.v[553] == 0.0);
        s.v[1876] = if s.b[1876] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1876])) {
            s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);
            s.store_limited_exp(169, 174);
            s.store_mul(555, 553, 169);
            s.store_mul_scaled_offset_ad_rhs(562, 557, p.p925, A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);
            s.store_mul_scaled_offset_ad_rhs(563, 564, p.p925, A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);
            s.store_div(167, 499, 562);
            s.store_limited_exp(177, 167);
        }

        s.b[1877] = ((s.v[559] - s.v[499]) < 0.001);
        s.v[1877] = if s.b[1877] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1876])) && s.b[1877]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if (((!s.b[1620]) && (!s.b[1876])) && (!s.b[1877])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(559), s.ad_value(499));
            s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if ((!s.b[1620]) && (!s.b[1876])) {
            s.store_mul(170, 546, 555);
        }

        if (!s.b[1620]) {
            s.store_scalar(602, ((s.v[183] / p.p1373) * p.p74));
        }

        s.b[1878] = ((s.v[598] == 0.0) && (s.v[597] == 0.0));
        s.v[1878] = if s.b[1878] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1878])) {
            s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);
            s.store_limited_exp(167, 174);
            s.store_mul(585, 587, 167);
            s.store_mul(578, 598, 167);
            s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);
            s.store_limited_exp(167, 174);
            s.store_mul(586, 588, 167);
            s.store_mul(577, 597, 167);
            s.store_mul_offset_rhs(583, 585, 595, (-1.0));
        }

        s.b[1879] = (s.v[583] < 1e-5);
        s.v[1879] = if s.b[1879] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1879]) {
            s.store_scalar(583, 0.0);
            s.store_scalar(591, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1879])) {
            s.store_div_from_scalar_sqrt_ad(591, 1.0, A::offset(s.ad_value(583), 1.0));
        }

        if ((!s.b[1620]) && (!s.b[1878])) {
            s.store_mul_offset_rhs(584, 586, 596, (-1.0));
        }

        s.b[1880] = (s.v[584] < 1e-5);
        s.v[1880] = if s.b[1880] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1880]) {
            s.store_scalar(584, 0.0);
            s.store_scalar(592, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1880])) {
            s.store_div_from_scalar_sqrt_ad(592, 1.0, A::offset(s.ad_value(584), 1.0));
        }

        if ((!s.b[1620]) && (!s.b[1878])) {
            s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p.p595) / p.p595));
            s.store_limited_exp(603, 167);
            s.store_sub_from_scalar(169, 1.0, 603);
            s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p.p595)));
            s.store_pow_ad(599, s.ad_value(167), s.ad_value(600));
            s.store_mul3_lhs(604, 602, 578, 599);
            s.store_mul(168, 167, 604);
            s.store_mul3_lhs(604, 602, 577, 599);
            s.store_mul(168, 167, 604);
            s.store_offset_scaled_ad(531, A::pow(s.ad_value(167), s.ad_value(530)), p.p920, 1.0);
            s.store_mul3_lhs(532, 602, 578, 531);
            s.store_mul_ad_product_lhs_mixed_ia(533, 532, A::offset(s.ad_value(595), (-1.0)), 591);
            s.store_mul3_lhs(532, 602, 577, 531);
            s.store_mul_ad_product_lhs_mixed_ia(534, 532, A::offset(s.ad_value(596), (-1.0)), 592);
            s.store_add_scaled_inputs(580, 581, 1.0, 582, s.v[184]);
        }

        s.b[1881] = (s.v[580] < 1.0);
        s.v[1881] = if s.b[1881] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1881]) {
            s.store_scalar(580, 1.0);
        }

        s.b[1882] = (p.p554 == 1.0);
        s.v[1882] = if s.b[1882] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1882])) {
            s.store_offset_div_scaled_inputs2_indices(167, 498, 1.0, 499, 1.0, 580, 1.0, 1.0);
            s.store_add(168, 583, 584);
            s.store_sqrt_add_scaled_square_input(170, 167, 1.0, 168, 4.0);
            s.store_scaled_add(169, 167, 170, 0.5);
            s.store_mul(167, 603, 604);
        }

        s.b[1884] = ((s.v[567] == 0.0) && (s.v[568] == 0.0));
        s.v[1884] = if s.b[1884] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && (!s.b[1884])) {
            s.store_mul_offset_rhs(174, 569, 639, (-1.0));
            s.store_limited_exp(167, 174);
            s.store_mul(571, 567, 167);
            s.store_mul_offset_rhs(174, 570, 639, (-1.0));
            s.store_limited_exp(167, 174);
            s.store_mul(572, 568, 167);
            s.store_scale(594, 573, p.p925);
        }

        s.b[1885] = ((s.v[575] - s.v[498]) < 0.001);
        s.v[1885] = if s.b[1885] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1885]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 571);
        }

        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1885])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(575), s.ad_value(498));
            s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 571);
        }

        if ((!s.b[1620]) && (!s.b[1884])) {
            s.store_scale(594, 574, p.p925);
        }

        s.b[1886] = ((s.v[576] - s.v[499]) < 0.001);
        s.v[1886] = if s.b[1886] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1886]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 572);
        }

        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1886])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(576), s.ad_value(499));
            s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 572);
        }

        s.b[1887] = (p.p36 == 0.0);
        s.v[1887] = if s.b[1887] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1887]) {
            s.store_scalar(167, (s.v[200] * p.p76));
        }

        s.b[1888] = (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0));
        s.v[1888] = if s.b[1888] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1887]) && s.b[1888]) {
            s.store_scalar(173, 0.0);
        }

        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) {
            s.store_div_scaled_inputs3_indices(168, 204, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 168, 168, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);
        }

        s.b[1889] = (s.v[894] != 0.0);
        s.v[1889] = if s.b[1889] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) && s.b[1889]) {
            s.store_mul_square_lhs(170, 201, 201);
        }

    }
}
