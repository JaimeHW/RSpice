#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_126(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && s.b[1799]) {s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(951)), 0.4), s.ad_value(729)), 0.4);s.store_sqrt(299, 298);s.store_sqrt_div_from_scalar_ad(277, (2.0 * s.v[180]), A::scale(s.ad_value(953), 1.602176462e-19));s.store_mul_add_scaled_inputs_rhs(941, 835, A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_scale_offset_mixed_ia(940, 841, A::mul_offset_rhs(s.ad_value(848), s.ad_value(639), (-1.0)), 1.0, 1.0);s.store_add_scaled_inputs3_offset_mixed_iia(273, 298, 0.5, 218, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05)), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));s.store_sqrt(274, 273);s.store_mul(275, 277, 274);s.store_div_from_scalar(260, s.v[180], 275);s.store_div_scaled_product_add_scaled_denominator_indices(169, 5, 7, 1.0, 5, 1.0, 7, 1.0, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(170, 227, 838, 1.0, 220, p.p1183, 218, (-p.p1195), 0.0);s.store_add_scaled_inputs_products_mixed_aiiiia(171, A::add_scaled_product(s.ad_value(220), p.p1181, s.ad_value(220), s.ad_value(220), p.p1182), 1.0, 218, (-p.p1184), 218, 218, (-p.p1185), 955, A::add(A::add_scaled_product(A::add_scaled_value_products3(s.ad_value(715), 1.0, s.ad_value(712), s.ad_value(220), 1.0, s.ad_value(220), s.ad_value(220), p.p1180, s.ad_value(716), s.ad_value(218), 1.0), 1.0, s.ad_value(218), s.ad_value(218), p.p1190), s.ad_value(170)), 1.0);s.store_div_ad(168, A::add_scaled_inputs4_offset(s.ad_value(169), 1.0, s.ad_value(836), 1.0, s.ad_value(941), 1.0, s.ad_value(171), 1.0, s.v[199]), A::offset(s.ad_value(169), s.v[199]));s.store_scaled_add_offset_sqrt_square_offset(267, 168, 1.0, (-1.0), ((0.25 * 0.05) * 0.05), 0.5);s.store_mul(269, 267, 271);s.store_div_from_scalar(270, 1.0, 269);s.store_mul(222, 221, 270);s.store_mul(225, 224, 270);s.store_mul(212, 707, 270);s.store_mul(215, 708, 270);s.store_mul(238, 234, 270);s.store_add_scaled_products_mixed_iaii(291, 736, A::sub(s.ad_value(274), s.ad_value(299)), 1.0, 849, 218, (-1.0));s.store_mul_scale_offset_mixed_ia(944, 227, A::add_scaled_product(s.ad_value(940), 1.0, s.ad_value(842), s.ad_value(218), 1.0), -1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_127(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && s.b[1799]) {s.store_mul_scale_offset(293, A::add_scaled_inputs_product(s.ad_value(843), 1.0, s.ad_value(844), 1.0 / (s.v[184]), s.ad_value(845), s.ad_value(218), 1.0), A::pow(s.ad_value(639), s.ad_value(846)), 1.0, (-1.0));s.store_mul_scale_offset_rhs(946, 300, 218, p.p1264, 1.0);}
        s.b[1800] = (s.v[946] > 0.0);s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1800]) {s.store_div_from_scalar(167, (p.p1263 * s.v[184]), 946);}
        s.b[1801] = (s.v[167] < 40.0);s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && s.b[1800]) && s.b[1801]) {s.store_div_from_scalar_offset_ad(943, (0.5 * p.p1262), A::cosh(s.ad_value(167)), (-1.0));}
        if (((s.b[1620] && s.b[1799]) && s.b[1800]) && (!s.b[1801])) {s.store_scaled_limited_exp_scaled_input(943, 167, -1.0, p.p1262);}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1800])) {s.store_scalar(943, 0.0);}
        if (s.b[1620] && s.b[1799]) {s.store_mul_sub_rhs(945, 943, 942, 298);}
        s.b[1802] = (s.v[280] > 0.0);s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1802]) {s.store_mul_scale_offset_indices(167, 227, 282, -1.0, 0.0);}
        s.b[1803] = (s.v[167] < (-80.0));s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && s.b[1802]) && s.b[1803]) {s.store_scalar(169, 1.804851387e-35);}
        if (((s.b[1620] && s.b[1799]) && s.b[1802]) && (!s.b[1803])) {s.store_limited_exp(169, 167);}
        if ((s.b[1620] && s.b[1799]) && s.b[1802]) {s.store_offset_mul_offset_rhs(170, 280, 169, 1.0, s.v[184]);s.store_mul_scaled_ln_ad_rhs(278, 269, -1.0, A::max_with_scalar(A::div_from_scalar(s.v[184], s.ad_value(170)), 1e-38));}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1802])) {s.store_scalar(278, 0.0);}
        if (s.b[1620] && s.b[1799]) {s.store_add_div_rhs_mixed_ia(171, 290, 284, A::pow_from_scalar(s.v[184], s.ad_value(286)));s.store_add_scaled_product_mixed_iia(278, 278, 1.0, 171, A::tanh(A::mul(s.ad_value(288), s.ad_value(227))), (-1.0));s.store_add_scaled_inputs3_mixed_aii(242, A::offset(A::add(A::add_scaled_inputs4(s.ad_value(291), 1.0, s.ad_value(278), 1.0, s.ad_value(944), 1.0, s.ad_value(293), -1.0), s.ad_value(945)), p.p1151), 1.0, 956, 1.0, 932, 1.0);s.store_add_scaled_inputs_product_indices(213, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));s.store_add_scaled_inputs_product_mixed_aiii(367, A::add_scaled_product(s.ad_value(222), 1.0, s.ad_value(218), s.ad_value(270), (-1.0)), 1.0, 212, (-1.0), 242, 270, (-1.0));s.store_add_scaled_inputs_product_indices(214, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));s.store_sub(216, 238, 215);s.store_scaled_sqrt_mul_scaled_lhs(294, 953, ((2.0 * 1.602176462e-19) * s.v[180]), 270, 1.0 / (s.v[199]));s.store_scalar(947, (p.p1148 * (1.0 + (p.p1149 * ((s.v[184]) as f64).powf((-p.p1150))))));s.store_mul_scale_offset_indices(294, 294, 947, 1.0, 1.0);s.store_div_from_scalar(295, 1.0, 294);s.store_square(296, 294);s.store_div_from_scalar(297, 1.0, 296);s.store_scalar(5, (s.v[180] / p.p74));s.store_scalar(7, (s.v[181] / p.p75));s.store_primal_div_scaled_inputs2_indices(3, 7, 1.0, 728, 1.0, 5, 1.0);s.store_scalar(2, (p.p76 / p.p75));s.store_div(124, 294, 2);s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);s.store_scale(126, 125, 1e-7);s.store_scalar(127, (5.0 / 4.0));s.store_div_from_scalar(128, 1.0, 124);s.store_square(129, 124);s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));}
        s.b[1804] = (((s.v[216]) as f64).abs() <= s.v[126]);s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1804]) {s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);}
        s.b[1805] = (s.v[216] < (-s.v[126]));s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1804])) && s.b[1805]) {s.store_neg(132, 216);s.store_mul3_lhs(133, 127, 132, 128);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_128(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1804])) && s.b[1805]) {s.store_scaled_sub_offset_sqrt_square_offset(134, 133, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(135, A::sub(s.ad_value(132), s.ad_value(134)), 1.0, 129, A::offset(s.ad_value(134), 1.0), 1.0);s.store_add_scaled_inputs3_indices(137, 132, 2.0, 134, (-2.0), 129, -1.0);s.store_sub_mixed_ai(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);s.store_add(0, 135, 137);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 138, A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.5, s.ad_value(135), 1.0), 1.0);s.store_add_mixed_ia(140, 134, A::div_scaled_product3(s.ad_value(135), s.ad_value(0), s.ad_value(138), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138), s.ad_value(138)), s.ad_value(137), A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.3333333333333333, s.ad_value(135), 1.0))), 1.0));s.store_limited_exp(141, 140);s.store_sub(142, 132, 140);s.store_add_scaled_offset_product_rhs(143, 142, 2.0, 129, 141, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(136, 142, 1.0, 129, A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141)), 1.0);s.store_sub_from_scalar_scaled_mul(144, 1.0, 129, 141, 0.5);s.store_add_scaled_square_product_indices(142, 143, 1.0, 144, 136, (-4.0));s.store_scaled_div_mixed_ia(145, 136, A::add(s.ad_value(143), A::sqrt(s.ad_value(142))), 2.0);s.store_neg_add(131, 140, 145);}
        if (((s.b[1620] && s.b[1799]) && (!s.b[1804])) && (!s.b[1805])) {s.store_mul_scale_offset_mixed_ia(146, 130, A::mul3(s.ad_value(125), s.ad_value(127), s.ad_value(130)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(147, 216, 128, A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));s.store_limited_exp_neg_input(150, 147);s.store_sub_from_scalar(149, 1.0, 150);s.store_add_scaled_inputs_product_mixed_iiia(148, 216, 1.0, 129, 0.5, 124, A::sqrt(A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(129), 0.25, s.ad_value(149), -1.0)), (-1.0));s.store_limited_exp_neg_input(151, 148);s.store_add_scaled_inputs3_mixed_iia(152, 216, 2.0, 148, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(129), 1.0, s.ad_value(151)), 1.0);s.store_add_scaled_square_product_mixed_aia(153, A::sub(s.ad_value(216), s.ad_value(148)), 1.0, 129, A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151)), (-1.0));s.store_sub_from_scalar_scaled_mul(154, 1.0, 129, 151, 0.5);s.store_add_scaled_square_product_indices(150, 152, 1.0, 154, 153, (-4.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_129(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1804])) && (!s.b[1805])) {s.store_scaled_div_mixed_ia(139, 153, A::add(s.ad_value(152), A::sqrt(s.ad_value(150))), 2.0);s.store_add(131, 148, 139);}
        s.b[1806] = (((s.v[216]) as f64).abs() < s.v[126]);s.store_scalar(1806, if s.b[1806] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1806]) {s.store_mul_ad_affine_product_rhs(46, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1806])) {s.store_add_scaled_inputs3_offset_mixed_aai(19, A::mul3(A::mul3(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131)), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294)), A::div_from_scalar(1.0, s.ad_value(294))), 1.0, A::limited_exp_scaled_input(s.ad_value(131), -1.0), -1.0, 131, -1.0, (-(-1.0)));s.store_offset_add_ad(20, A::limited_exp_scaled_input(s.ad_value(131), -1.0), A::div_scaled_product(A::square(s.ad_value(2)), A::sub_scaled_inputs(s.ad_value(131), 2.0, s.ad_value(216), 2.0), 1.0, A::square(s.ad_value(294)), 1.0), (-1.0));s.store_sub_div_rhs_indices(46, 131, 19, 20);}
        if (s.b[1620] && s.b[1799]) {s.store_mul(46, 46, 269);s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);s.store_div_from_scalar(96, 1.0, 95);s.store_add_mixed_ai(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 225);s.store_limited_exp_neg_input(99, 97);s.store_scale(101, 95, 0.001);s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_mul_ad_product_lhs_mixed_ia(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, 3, 216, (-1.0));s.store_add_scaled_product_mixed_iia(104, 4, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(4), -1.0), s.ad_value(4)), (-1.0))), 1.0);}
        s.b[1807] = (s.v[4] < s.v[97]);s.store_scalar(1807, if s.b[1807] { 1.0 } else { 0.0 });s.b[1808] = (s.v[214] < s.v[104]);s.store_scalar(1808, if s.b[1808] { 1.0 } else { 0.0 });s.b[1809] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1809, if s.b[1809] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_130(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1810] = (s.v[214] < (-s.v[101]));s.store_scalar(1810, if s.b[1810] { 1.0 } else { 0.0 });
        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && s.b[1810]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 99, 29);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_131(
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && s.b[1810]) {s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 97, 3.0);s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_132(
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[1620] && s.b[1799]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) {s.store_add_mixed_ia(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));s.store_limited_exp(28, 44);s.store_div_from_scalar(29, 1.0, 28);s.store_limited_exp_sub(28, 44, 97);s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);s.store_mul_square_lhs(30, 44, 13);s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 214, 44);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_add_scaled_inputs_mixed_ia(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) {s.copy_ad(47, 2);s.store_primal_square(48, 47);s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));s.store_add_scaled_product_mixed_iia(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));s.store_offset(43, 97, 3.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(106, 105, 0.5, 43, 0.5, 105, 43, 40.0, (-0.5));s.store_add_scaled_inputs_product_mixed_aaii(107, A::square(A::sub(s.ad_value(214), s.ad_value(106))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, 4, (-1.0));s.store_add_scaled_inputs_product_mixed_iiia(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));s.store_square(109, 108);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_133(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) {s.store_primal_sub_from_scalar(110, 1.0, 48);}
        s.b[1811] = (s.v[107] < 0.0);s.store_scalar(1811, if s.b[1811] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) && s.b[1811]) {s.store_scalar(107, 0.0);}
        if (((s.b[1620] && s.b[1799]) && s.b[1807]) && (!s.b[1808])) {s.store_add_scaled_inputs3_mixed_iia(49, 97, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);s.store_add(111, 107, 108);s.store_square(112, 111);s.store_add_scaled_inputs_product_mixed_aiii(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));s.store_div_scaled_product_mixed_iia(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);s.store_add(117, 106, 116);s.store_limited_exp_sub(118, 117, 97);s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);s.store_add_scaled_inputs_product_mixed_aaia(120, A::square(A::sub(s.ad_value(214), s.ad_value(117))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, A::add(s.ad_value(4), s.ad_value(118)), (-1.0));s.store_mul_add_scaled_sub_value_product_rhs_mixed_aii(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, 296, 118, (((-1.0)) * (2.0)));s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);s.store_add(9, 117, 122);}
        s.b[1812] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1812, if s.b[1812] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1807])) && s.b[1812]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1813] = (s.v[214] < (-s.v[101]));s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && s.b[1813]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_134(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && s.b[1813]) {s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 99, 29);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && (!s.b[1813])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 97, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_135(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && (!s.b[1813])) {s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);s.store_add_mixed_ia(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));s.store_limited_exp(28, 44);s.store_div_from_scalar(29, 1.0, 28);s.store_limited_exp_sub(28, 44, 97);s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);s.store_mul_square_lhs(30, 44, 13);s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 214, 44);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_136(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1807])) && (!s.b[1812])) && (!s.b[1813])) {s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));s.store_add_scaled_inputs_mixed_ia(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (s.b[1620] && s.b[1799]) {s.copy_ad(123, 9);s.store_scalar(102, 1e-7);s.store_scalar(103, 2.0);s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_mul_ad_product_lhs_mixed_ia(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);s.store_add_scaled_value_products_mixed_iaiai(6, 24, 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), 727, (-1.0), A::offset(s.ad_value(3), 1.0), 46, 1.0);}
        s.b[1814] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1814]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_137(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
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
        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 97);s.store_limited_exp_sub(177, 123, 97);s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_138(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_139(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {let t0: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t0, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if (s.b[1620] && s.b[1799]) {s.copy_ad(123, 22);}
        s.b[1815] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1815]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_140(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && s.b[1815]) {s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
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
        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 97);s.store_limited_exp_sub(177, 123, 97);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_141(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
}
