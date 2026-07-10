#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {let t0: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t0, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if (s.b[1620] && (!s.b[1634])) {s.store_sub(62, 23, 22);s.store_mul(63, 226, 270);s.store_limited_exp_neg_input(64, 63);}
        s.b[1649] = (s.v[62] < 1e-10);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
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
        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)));s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_sub(177, 123, 98);s.store_limited_exp_ad(178, A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(269), 2.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(179, A::add(A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(269), 2.0, s.ad_value(269), 1.0), s.ad_value(170)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {s.store_neg_ad(65, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(63), (-1.0), s.ad_value(98), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(63), -1.0, s.ad_value(98), 1.0)), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), 1.0, (-1.0)));s.store_mul_mixed_ai(66, A::mul_sub_from_scalar_rhs(s.ad_value(296), 1.0, s.ad_value(64)), 57);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {let t1: A = A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product3_by_product(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0, A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), (-1.0), s.ad_value(296), A::sub(A::sub(A::add_scaled_inputs4(A::add_scaled_inputs_product(s.ad_value(173), 1.0, A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(98), (-1.0), s.ad_value(63), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(63), 1.0)), A::sub(A::add_scaled_product(s.ad_value(175), (-2.0), A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(175), 10.0), s.ad_value(175), 1.0), A::mul3(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 8.0), s.ad_value(123), s.ad_value(175)), s.ad_value(175), s.ad_value(175))), 1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), (-1.0), A::div(s.ad_value(178), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), 1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), A::div(s.ad_value(179), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), A::div(s.ad_value(179), A::mul(A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), (-1.0));s.store_offset_sub_ad(54, t1, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {s.store_add_scaled_square_product_indices(54, 65, 1.0, 54, 66, (-2.0));}
        s.b[1650] = (s.v[54] >= 0.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && s.b[1649]) && s.b[1650]) {s.store_scaled_div_mixed_ia(62, 66, A::add(s.ad_value(65), A::sqrt(s.ad_value(54))), 2.0);}
        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {s.store_add(23, 22, 62);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul(250, 62, 269);s.store_div_scaled_product_offset_denominator_mixed_iia(67, 23, 23, 1.0, A::square(s.ad_value(23)), 2.0, 1.0);s.store_limited_exp_neg_input(68, 23);s.store_add_scaled_product(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67)), (-1.0));s.store_add_scaled_product_mixed_iaa(70, 69, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(23))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_offset_add_scaled_inputs(70, A::offset(s.ad_value(70), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(70), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(60, 70);s.store_mul_sqrt_mixed_ia(72, 294, A::add(s.ad_value(70), s.ad_value(69)));s.store_div_scaled_product3_mixed_iiia(73, 296, 69, 269, 1.0, A::add_scaled_product(s.ad_value(72), 1.0, s.ad_value(294), s.ad_value(60), 1.0), 1.0);s.store_scaled_add(75, 22, 23, 0.5);s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));s.store_sqrt(76, 54);s.store_scaled_add(77, 57, 69, 0.5);s.store_add_scaled_product_mixed_iaa(78, 77, 1.0, A::square(s.ad_value(62)), A::sub_scaled_inputs(s.ad_value(76), 1.0, s.ad_value(297), 2.0), 0.125);s.store_add_scaled_product_mixed_iaa(79, 78, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(75))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_mul_sqrt_mixed_ia(51, 294, A::add(s.ad_value(78), s.ad_value(79)));s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(79), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(71, 79);}
        s.b[1651] = (p.p46 == 1.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1651]) {s.store_div_scaled_inputs_indices(85, 269, ((2.0 * s.v[199]) * s.v[199]), 704, (1.602176462e-19 * s.v[180]));s.store_add_scaled_sub_value_product_mixed_iia(86, 1.0, 76, 1.0, 51, A::div_from_scalar(1.0, s.ad_value(296)), 2.0);s.store_div_from_scalar_sqrt_ad(87, 1.0, A::offset(A::mul(s.ad_value(85), s.ad_value(51)), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1651]) {s.store_div_scaled_value_offset_denominator(54, s.ad_value(87), 1.0, s.ad_value(87), 1.0, 1.0);s.store_mul_ad_product_rhs(88, 85, A::mul3(A::square(s.ad_value(54)), s.ad_value(51), s.ad_value(51)), A::div(s.ad_value(78), A::add(s.ad_value(78), s.ad_value(79))));s.store_add_scaled_inputs_product_mixed_iiia(89, 51, 2.0, 88, (-2.0), 296, A::add(A::sub_from_scalar(1.0, s.ad_value(76)), s.ad_value(78)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(90, 88, 88, 1.0, 51, 2.0);s.store_sub_from_scalar_scaled_mul_mixed_ia(91, 1.0, 296, A::add(s.ad_value(76), s.ad_value(78)), 0.5);s.store_div_scaled_product_mixed_iia(92, 90, 89, 1.0, A::add_scaled_square_product(s.ad_value(89), 1.0, s.ad_value(91), s.ad_value(90), (-1.0)), 1.0);s.store_add(75, 75, 92);s.store_limited_exp(93, 92);s.store_div(76, 76, 93);s.store_mul(78, 78, 93);s.store_add_scaled_product(79, A::div(s.ad_value(78), s.ad_value(93)), (-1.0), A::square(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(75), (-1.0), s.ad_value(92), 1.0)), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_mul_sqrt_mixed_ia(51, 294, A::add(s.ad_value(78), s.ad_value(79)));s.store_add_ad(94, A::sub_from_scalar(1.0, s.ad_value(76)), A::mul3_scaled_output(s.ad_value(51), s.ad_value(87), s.ad_value(297), 2.0));s.store_div_scaled_product3_mixed_iiaa(62, 62, 93, A::add(s.ad_value(86), s.ad_value(77)), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(93), s.ad_value(77), 1.0), 1.0);s.store_mul(250, 62, 269);s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(79), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(71, 79);}
        s.b[1652] = (((s.v[250]) as f64).abs() > 1e-35);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1652]) {s.store_div_scaled_inputs2_indices(74, 306, 1.0, 73, (-1.0), 250, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul_div_scaled_product_mixed_iiia(80, 269, 296, 78, 1.0, A::add_scaled_product(s.ad_value(51), 1.0, s.ad_value(294), s.ad_value(71), 1.0), 1.0);s.store_mul3_lhs(82, 71, 294, 269);s.store_mul(52, 51, 269);s.store_mul_add_scaled_inputs_rhs_indices(336, 335, 82, 1.0, 80, s.v[338]);s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scaled_offset(A::div(s.ad_value(80), s.ad_value(82)), 1.0, 0.5), 1e-38))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1634])) {s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(336), s.ad_value(651)), 1.0);s.store_offset(171, 170, 1.0);s.store_scaled_add_offset_sqrt_square_offset(339, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);s.store_div_scaled_inputs_mixed_ia(310, 746, 2.0, A::div(s.ad_value(740), s.ad_value(339)), 1.0);s.store_scale(311, 310, s.v[184]);}
        s.b[1653] = (s.v[781] > 0.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1653]) {s.store_offset_div_scaled_product_indices(360, 781, 80, 1.0, 311, 1.0, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1653])) {s.store_div_from_scalar_sub_from_scalar_ad(360, 1.0, 1.0, A::div_scaled_product(s.ad_value(781), s.ad_value(80), 1.0, s.ad_value(311), 1.0));}
        if (s.b[1620] && (!s.b[1634])) {s.copy_ad(359, 763);s.store_sub(355, 226, 315);s.store_add_scaled_inputs(362, 80, 1.0, 269, 2.0);}
        s.b[1654] = (s.v[359] > 0.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1654]) {s.store_div_add_scaled_inputs_rhs_indices(170, 362, 312, 1.0, 362, 1.0);s.store_scaled_add_sqrt_square_offset_ad(171, A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), ((4.0 * 0.001) * 0.001), 0.5);s.store_div_from_scalar(172, 1.0, 171);s.store_mul_product3_mixed_iaii(361, 172, A::div(s.ad_value(362), s.ad_value(359)), 170, 360, 1.0);s.store_offset_div(363, 355, 361, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1654])) {s.store_scalar(363, 1.0);}
        s.b[1655] = (s.v[769] <= 0.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1655]) {s.store_scalar(268, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1655])) {s.store_div_scaled_inputs_indices(176, 769, ((s.v[184]) as f64).sqrt(), 362, 1.0);s.store_div_from_scalar_offset_input(268, 1.0, 176, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_add(358, 312, 311);}
        s.b[1656] = (s.v[785] > 0.0);s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });s.b[1657] = (p.p414 < 0.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && s.b[1656]) && s.b[1657]) {s.store_div_scaled_value_by_product_mixed_iai(168, 785, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p414, s.ad_value(311), 1.0)), 268, 1.0);}
        if (((s.b[1620] && (!s.b[1634])) && s.b[1656]) && (!s.b[1657])) {s.store_div_scaled_product_offset_rhs_mixed_iai(168, 785, A::div_scaled_inputs(s.ad_value(80), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, 268, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && s.b[1656]) {s.store_offset_mul_ad(364, s.ad_value(168), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(355), 1.0, s.ad_value(168), s.ad_value(358), 1.0), 1.0), 1e-38)), 1.0);}
        s.b[1658] = (p.p414 < 0.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) && s.b[1658]) {s.store_div_scaled_value_by_product_mixed_iai(168, 785, 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p414, s.ad_value(311), 1.0)), 268, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) && (!s.b[1658])) {s.store_div_scaled_product_offset_rhs_mixed_iai(168, 785, A::div_scaled_inputs(s.ad_value(80), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, 268, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) {s.store_offset(364, 168, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul(363, 363, 364);s.store_limited_exp_mul(168, 768, 226);}
        s.b[1659] = (s.v[767] > 0.0);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1659]) {s.store_scalar(169, (1.0 + (p.p433 * s.v[184])));s.store_div_scaled_offset_numerator_mixed_ai(356, A::mul(s.ad_value(169), s.ad_value(168)), 1.0, 1.0, 767, 1.0);s.store_mul(356, 356, 268);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1659])) {s.store_scalar(356, 5.540622384e34);}
        if (s.b[1620] && (!s.b[1634])) {s.store_div(171, 355, 356);s.store_offset(167, 171, 1.0);s.store_mul(363, 363, 167);}
        s.b[1660] = (s.v[766] > 0.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });s.b[1661] = (s.v[355] > ((s.v[765] * s.v[300]) / 80.0));s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && s.b[1660]) && s.b[1661]) {s.store_div_scaled_product_indices(167, 765, 300, 1.0, 355, 1.0);s.store_div_scaled_inputs_limited_exp_lhs(357, 167, s.v[184], 766, 1.0);}
        if (((s.b[1620] && (!s.b[1634])) && s.b[1660]) && (!s.b[1661])) {s.store_div_from_scalar(357, (5.540622384e34 * s.v[184]), 766);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1660])) {s.store_scalar(357, 5.540622384e34);}
        if (s.b[1620] && (!s.b[1634])) {s.store_offset_div(365, 355, 357, 1.0);s.store_mul(363, 363, 365);}
        s.b[1662] = (s.v[678] < 0.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1662]) {s.store_div_from_scalar_sub_from_scalar_ad(349, 1.0, 1.0, A::mul(s.ad_value(678), s.ad_value(218)));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1662])) {s.store_offset_mul(349, 678, 218, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul(167, 80, 349);s.store_div_scaled_value_offset_denominator(350, s.ad_value(167), 100.0, s.ad_value(167), 100.0, 1.0);s.store_scalar(352, (1.0 / p.p503));s.store_ln_ad(167, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(226), s.ad_value(250)), s.ad_value(352)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(315), s.ad_value(250)), s.ad_value(352)), 1.0), 1.0));s.store_scale(353, 167, p.p504);s.store_div_from_scalar_add_ad(354, 1.0, A::offset(s.ad_value(353), 1.0), A::square(s.ad_value(353)));s.store_mul(341, 339, 354);}
        s.b[1663] = (s.v[346] < 0.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1663]) {s.store_div_from_scalar_sub_from_scalar_ad(168, 1.0, 1.0, A::mul(s.ad_value(346), s.ad_value(350)));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1663])) {s.store_offset_mul(168, 346, 350, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul_div_rhs(351, 744, 168, 341);s.store_mul_ad_product_lhs_mixed_ai(342, A::square(s.ad_value(351)), 250, 250);}
        s.b[1664] = (p.p30 == (-1.0));s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1664]) {s.store_div_scaled_value_offset_denominator(342, s.ad_value(342), 1.0, A::mul(s.ad_value(351), s.ad_value(250)), 1.0, 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_mul_scale_offset_mixed_ia(343, 341, A::sqrt(A::scale_offset(s.ad_value(342), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div_from_scalar(344, 1.0, 343);s.store_scalar(454, 0.0);s.store_scalar(455, 0.0);s.store_add(243, 306, 73);}
        s.b[1665] = (p.p33 == 1.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1665]) {s.store_scalar(457, 0.0);s.store_scalar(458, 1.0);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(170, 169, 0.01);s.store_scaled_add(228, 169, 170, 0.5);s.store_offset_mul(172, 770, 228, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1665]) {s.store_add_scaled_product_mixed_aii(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 202, 1.0);s.store_scaled_add_mixed_ia(171, 173, A::sqrt_square_offset(s.ad_value(173), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(454, 652, 452, 1.0, A::add_scaled_product(s.ad_value(773), 1.0, s.ad_value(775), s.ad_value(171), 1.0), 448, 1.0);s.store_sub(169, 204, 219);s.store_sqrt_square_offset(170, 169, 0.01);s.store_scaled_add(229, 169, 170, 0.5);s.store_offset_mul(172, 770, 229, 1.0);s.store_add_scaled_product_mixed_aii(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 201, 1.0);s.store_scaled_add_mixed_ia(171, 173, A::sqrt_square_offset(s.ad_value(173), 0.01), 0.5);s.store_mul_add_scaled_product_rhs_mixed_iai(455, 652, 453, 1.0, A::add_scaled_product(s.ad_value(772), 1.0, s.ad_value(774), s.ad_value(171), 1.0), 448, 1.0);}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1665])) {s.store_offset_mul(167, 770, 243, 1.0);s.store_mul_sub_rhs(168, 787, 274, 299);s.store_add_mixed_ai(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);s.store_scaled_add_mixed_ia(170, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(457, s.ad_value(652), A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), p.p2, 0.0, 448);s.copy_ad(455, 453);s.copy_ad(454, 452);s.store_offset_product3(458, A::div(s.ad_value(740), s.ad_value(343)), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);}
        s.b[1666] = (p.p33 == 2.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1665])) && s.b[1666]) {s.store_mul_add_mixed_iai(457, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), 453);s.store_scalar(455, 0.0);s.store_scalar(454, 0.0);s.store_offset_product3(458, A::div(s.ad_value(740), s.ad_value(343)), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);}
        if (s.b[1620] && (!s.b[1634])) {s.store_add_div_rhs_mixed_ia(167, 330, 333, A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(267), s.ad_value(637), 2.0));s.store_sub(416, 306, 73);s.store_mul3_lhs(168, 167, 416, 416);s.store_offset(169, 168, ((1.0) + ((-0.001))));s.store_offset_add_scaled_inputs_mixed_ia(170, 169, 0.5, A::sqrt_square_offset(s.ad_value(169), 0.004), 0.5, (-1.0));s.store_scaled_offset_ad(334, A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1634])) {s.store_offset_sub_scaled_inputs(334, A::offset(s.ad_value(334), 1.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(334), (-1.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));s.store_add(167, 306, 73);s.store_sub(168, 306, 73);s.store_div_add_scaled_inputs_rhs_indices(169, 168, 167, 1.0, 833, 1.0);s.store_mul3_lhs(170, 832, 169, 169);s.store_offset(834, 170, 1.0);s.store_div_mixed_ia(176, 858, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul3(s.ad_value(864), s.ad_value(168), s.ad_value(168)))), s.ad_value(167), 1.0, s.ad_value(267), s.ad_value(637), 2.0));s.store_limited_exp_neg_input(853, 176);s.store_mul(167, 341, 344);s.store_mul_scale_offset_mixed_ia(83, 74, A::mul3_scaled_output(s.ad_value(342), s.ad_value(167), s.ad_value(167), 0.5), 1.0, 1.0);s.store_offset_add_scaled_inputs(83, A::offset(s.ad_value(83), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(83), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_add_scaled_product_indices(81, 80, 1.0, 269, 74, 1.0);s.store_mul_div_scaled_inputs_mixed_aii(84, A::div(s.ad_value(341), s.ad_value(343)), 81, 1.0, 83, 1.0);}
        if s.b[1620] {s.store_mul_ad_product_lhs_mixed_ai(380, A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(740), s.ad_value(81), s.ad_value(250), ((p.p2 * (s.v[183] / s.v[184])) * s.v[199])), A::div_scaled_product(s.ad_value(354), s.ad_value(344), 1.0, s.ad_value(458), 1.0), s.ad_value(363), 1.0, s.ad_value(334), 1.0), 834, 853);s.store_mul3_lhs(340, 339, 343, 458);s.store_div(337, 740, 340);s.store_scalar(467, 0.0);}
        s.b[1667] = (p.p7 > 1.0);s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1667]) {s.store_scaled_mul(468, 337, 243, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));s.store_scale(176, 271, p.p1009);s.store_scaled_mul(167, 176, 337, ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]));s.store_scaled_add(467, 167, 468, (p.p1008 * p.p2));}
        s.b[1668] = (p.p7 == 2.0);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1667]) && s.b[1668]) {s.store_primal_div_from_scalar(466, 1.0, 465);}
        s.b[1669] = (s.v[466] < p.p1347);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1667]) && s.b[1668]) && s.b[1669]) {s.store_scalar(466, p.p1347);s.store_primal_div_from_scalar(465, 1.0, 466);}
        if ((s.b[1620] && s.b[1667]) && s.b[1668]) {s.store_add(178, 465, 467);s.store_div_scaled_product_indices(467, 465, 467, 1.0, 178, 1.0);}
        if s.b[1620] {s.store_scalar(544, ((s.v[183] / p.p1373) + p.p1377));s.store_scalar(543, ((s.v[183] / p.p1373) + p.p1378));s.store_primal_scale(545, 543, p.p74);s.store_primal_scale(546, 544, p.p74);s.store_mul(593, 637, 590);s.store_div(167, 498, 593);s.store_limited_exp(595, 167);s.store_mul(594, 637, 590);s.store_div(167, 499, 594);s.store_limited_exp(596, 167);s.store_mul_scale_offset_mixed_ai(171, A::div_from_scalar(1.115, s.ad_value(637)), 639, 1.0, (-1.0));}
        s.b[1670] = (s.v[550] == 0.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1670]) {s.store_scalar(535, 0.0);}
        if (s.b[1620] && (!s.b[1670])) {s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);s.store_limited_exp(168, 174);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1670])) {s.store_mul(548, 550, 168);s.store_mul(167, 545, 548);s.store_mul_scale_offset_indices(535, 167, 595, 1.0, (-1.0));}
        s.b[1671] = (s.v[551] == 0.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1671]) {s.store_scalar(536, 0.0);}
        if (s.b[1620] && (!s.b[1671])) {s.store_div_scaled_product_indices(174, 547, 171, 1.0, 590, 1.0);s.store_limited_exp(168, 174);s.store_mul(549, 551, 168);s.store_mul(167, 546, 549);s.store_mul_scale_offset_indices(536, 167, 596, 1.0, (-1.0));}
        s.b[1672] = (s.v[552] == 0.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1672]) {s.store_scalar(537, 0.0);}
        if (s.b[1620] && (!s.b[1672])) {s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);s.store_limited_exp(169, 174);s.store_mul(554, 552, 169);s.store_mul_scaled_offset_ad_rhs(562, 557, p.p925, A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(563, 564, p.p925, A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);s.store_div(167, 498, 562);s.store_limited_exp(177, 167);}
        s.b[1673] = ((s.v[558] - s.v[498]) < 0.001);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1672])) && s.b[1673]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if ((s.b[1620] && (!s.b[1672])) && (!s.b[1673])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(558), s.ad_value(498));s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 563, 1.0, 558, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if (s.b[1620] && (!s.b[1672])) {s.store_mul(170, 545, 554);s.store_mul_add_rhs(537, 170, 177, 178);}
        s.b[1674] = (s.v[553] == 0.0);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1674]) {s.store_scalar(538, 0.0);}
        if (s.b[1620] && (!s.b[1674])) {s.store_div_scaled_product_indices(174, 556, 171, 1.0, 557, 1.0);s.store_limited_exp(169, 174);s.store_mul(555, 553, 169);s.store_mul_scaled_offset_ad_rhs(562, 557, p.p925, A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(563, 564, p.p925, A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0);s.store_div(167, 499, 562);s.store_limited_exp(177, 167);}
        s.b[1675] = ((s.v[559] - s.v[499]) < 0.001);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1674])) && s.b[1675]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if ((s.b[1620] && (!s.b[1674])) && (!s.b[1675])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(559), s.ad_value(499));s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 563, 1.0, 559, 168);s.store_limited_exp(178, 167);s.store_neg(178, 178);}
        if (s.b[1620] && (!s.b[1674])) {s.store_mul(170, 546, 555);s.store_mul_add_rhs(538, 170, 177, 178);}
        if s.b[1620] {s.store_scalar(602, ((s.v[183] / p.p1373) * p.p74));}
        s.b[1676] = ((s.v[598] == 0.0) && (s.v[597] == 0.0));s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1676]) {s.store_scalar(539, 0.0);s.store_scalar(540, 0.0);s.store_scalar(579, 0.0);}
        if (s.b[1620] && (!s.b[1676])) {s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);s.store_limited_exp(167, 174);s.store_mul(585, 587, 167);s.store_mul(578, 598, 167);s.store_div_scaled_product_indices(174, 589, 171, 1.0, 590, 1.0);s.store_limited_exp(167, 174);s.store_mul(586, 588, 167);s.store_mul(577, 597, 167);s.store_mul_scale_offset_indices(583, 585, 595, 1.0, (-1.0));}
        s.b[1677] = (s.v[583] < 1e-5);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1676])) && s.b[1677]) {s.store_scalar(583, 0.0);s.store_scalar(591, 1.0);}
        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1677])) {s.store_div_from_scalar_sqrt_ad(591, 1.0, A::offset(s.ad_value(583), 1.0));}
        if (s.b[1620] && (!s.b[1676])) {s.store_mul_scale_offset_indices(584, 586, 596, 1.0, (-1.0));}
        s.b[1678] = (s.v[584] < 1e-5);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && (!s.b[1676])) && s.b[1678]) {s.store_scalar(584, 0.0);s.store_scalar(592, 1.0);}
        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1678])) {s.store_div_from_scalar_sqrt_ad(592, 1.0, A::offset(s.ad_value(584), 1.0));}
        if (s.b[1620] && (!s.b[1676])) {s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p.p595) / p.p595));s.store_limited_exp(603, 167);s.store_sub_from_scalar(169, 1.0, 603);s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p.p595)));s.store_pow_indices(599, 167, 600);s.store_mul3_lhs(604, 602, 578, 599);s.store_mul(168, 167, 604);s.store_mul_ad_product_lhs_mixed_ia(539, 168, A::offset(s.ad_value(595), (-1.0)), 591);s.store_mul3_lhs(604, 602, 577, 599);s.store_mul(168, 167, 604);s.store_mul_ad_product_lhs_mixed_ia(540, 168, A::offset(s.ad_value(596), (-1.0)), 592);s.store_offset_scaled_ad(531, A::pow(s.ad_value(167), s.ad_value(530)), p.p920, 1.0);s.store_mul3_lhs(532, 602, 578, 531);s.store_mul_ad_product_lhs_mixed_ia(533, 532, A::offset(s.ad_value(595), (-1.0)), 591);s.store_mul3_lhs(532, 602, 577, 531);s.store_mul_ad_product_lhs_mixed_ia(534, 532, A::offset(s.ad_value(596), (-1.0)), 592);s.store_primal_add_scaled_inputs(580, 581, 1.0, 582, s.v[184]);}
        s.b[1679] = (s.v[580] < 1.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1676])) && s.b[1679]) {s.store_scalar(580, 1.0);}
        s.b[1680] = (p.p554 == 1.0);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1676])) && s.b[1680]) {s.store_scalar(579, 0.0);}
        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) {s.store_offset_div_scaled_inputs2_indices(167, 498, 1.0, 499, 1.0, 580, 1.0, 1.0);s.store_add(168, 583, 584);s.store_sqrt_add_scaled_square_input(170, 167, 1.0, 168, 4.0);s.store_scaled_add(169, 167, 170, 0.5);}
        s.b[1681] = (s.v[169] < 0.1);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) && s.b[1681]) {s.store_scalar(605, 10.0);}
        if (((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) && (!s.b[1681])) {s.store_div_from_scalar(605, 1.0, 169);}
        if ((s.b[1620] && (!s.b[1676])) && (!s.b[1680])) {s.store_mul(167, 603, 604);s.store_mul_ad_affine_product_lhs(579, s.ad_value(167), A::sub(s.ad_value(595), s.ad_value(596)), p.p2, 0.0, 605);}
        s.b[1682] = ((s.v[567] == 0.0) && (s.v[568] == 0.0));s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1682]) {s.store_scalar(541, 0.0);s.store_scalar(542, 0.0);}
        if (s.b[1620] && (!s.b[1682])) {s.store_mul_scale_offset_indices(174, 569, 639, 1.0, (-1.0));s.store_limited_exp(167, 174);s.store_mul(571, 567, 167);s.store_mul_scale_offset_indices(174, 570, 639, 1.0, (-1.0));s.store_limited_exp(167, 174);s.store_mul(572, 568, 167);s.store_scale(594, 573, p.p925);}
        s.b[1683] = ((s.v[575] - s.v[498]) < 0.001);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1682])) && s.b[1683]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 571);s.store_mul_scale_offset_indices(541, 170, 168, -1.0, 1.0);}
        if ((s.b[1620] && (!s.b[1682])) && (!s.b[1683])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(575), s.ad_value(498));s.store_mul_div_scaled_inputs_product_lhs(167, 498, -1.0, 594, 1.0, 575, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 571);s.store_mul_scale_offset_indices(541, 170, 168, -1.0, 1.0);}
        if (s.b[1620] && (!s.b[1682])) {s.store_scale(594, 574, p.p925);}
        s.b[1684] = ((s.v[576] - s.v[499]) < 0.001);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1682])) && s.b[1684]) {s.store_scalar(168, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 572);s.store_mul_scale_offset_indices(542, 170, 168, -1.0, 1.0);}
        if ((s.b[1620] && (!s.b[1682])) && (!s.b[1684])) {s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(576), s.ad_value(499));s.store_mul_div_scaled_inputs_product_lhs(167, 499, -1.0, 594, 1.0, 576, 168);s.store_limited_exp(168, 167);s.store_mul(170, 545, 572);s.store_mul_scale_offset_indices(542, 170, 168, -1.0, 1.0);}
        if s.b[1620] {s.store_add_scaled_inputs4_indices(496, 535, p.p2, 537, p.p2, 539, p.p2, 541, p.p2);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1620] {s.store_add_scaled_inputs4_indices(497, 536, p.p2, 538, p.p2, 540, p.p2, 542, p.p2);s.store_scalar(375, 0.0);s.store_scalar(374, 0.0);}
        s.b[1685] = (p.p36 == 0.0);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1685]) {s.store_scalar(167, (s.v[200] * p.p76));}
        s.b[1686] = (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0));s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1685]) && s.b[1686]) {s.store_scalar(173, 0.0);}
        if ((s.b[1620] && s.b[1685]) && (!s.b[1686])) {s.store_div_scaled_inputs3_indices(168, 204, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1687] = (s.v[894] != 0.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1685]) && (!s.b[1686])) && s.b[1687]) {s.store_mul_square_lhs(170, 201, 201);s.store_offset_add_ad(171, s.ad_value(894), A::abs(s.ad_value(170)), 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(170), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if (((s.b[1620] && s.b[1685]) && (!s.b[1686])) && (!s.b[1687])) {s.store_scalar(172, 1.0);}
        if ((s.b[1620] && s.b[1685]) && (!s.b[1686])) {s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);}
        if (s.b[1620] && s.b[1685]) {s.copy_ad(374, 173);}
        s.b[1688] = (((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) || (s.v[898] < 0.0));s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1685]) && s.b[1688]) {s.store_scalar(173, 0.0);}
        if ((s.b[1620] && s.b[1685]) && (!s.b[1688])) {s.store_div_scaled_inputs3_indices(168, 203, -1.0, 899, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1689] = (s.v[898] != 0.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1685]) && (!s.b[1688])) && s.b[1689]) {s.store_mul_square_lhs(170, 202, 202);s.store_offset_add_ad(171, s.ad_value(898), A::abs(s.ad_value(170)), 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(170), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if (((s.b[1620] && s.b[1685]) && (!s.b[1688])) && (!s.b[1689])) {s.store_scalar(172, 1.0);}
        if ((s.b[1620] && s.b[1685]) && (!s.b[1688])) {s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);}
        if (s.b[1620] && s.b[1685]) {s.copy_ad(375, 173);}
        if (s.b[1620] && (!s.b[1685])) {s.store_scalar(167, (s.v[200] * p.p76));s.store_add_scaled_product_indices(207, 223, (-1.0), 905, 221, 1.0);s.store_add_scaled_product_indices(206, 224, (-1.0), 902, 221, 1.0);s.store_sub(169, 203, 219);s.store_sqrt_square_offset(228, 169, 0.0001);}
        s.b[1690] = ((s.v[892] <= 0.0) || (s.v[660] <= 0.0));s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1685])) && s.b[1690]) {s.store_scalar(173, 0.0);}
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) {s.store_div_scaled_inputs3_indices(168, 207, -1.0, 895, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) {s.store_div_scaled_value_offset_denominator(169, s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1691] = (s.v[903] != 0.0);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) && s.b[1691]) {s.store_sub_scaled_inputs(170, 201, -1.0, 904, 1.0);s.store_offset(171, 170, 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(903), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(903), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) && (!s.b[1691])) {s.store_scalar(172, 1.0);}
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1690])) {s.store_mul3_ad(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));}
        if (s.b[1620] && (!s.b[1685])) {s.copy_ad(374, 173);}
        s.b[1692] = ((s.v[896] <= 0.0) || (s.v[661] <= 0.0));s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1685])) && s.b[1692]) {s.store_scalar(173, 0.0);}
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) {s.store_div_scaled_inputs3_indices(168, 206, -1.0, 899, (-1.0), 219, 1.0, 167, 1.0);s.store_scaled_add_mixed_ia(168, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0);}
        s.b[1693] = (s.v[906] != 0.0);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) && s.b[1693]) {s.store_sub_scaled_inputs(170, 202, -1.0, 907, 1.0);s.store_offset(171, 170, 0.0001);s.store_offset_add_scaled_inputs(172, A::div(s.ad_value(906), s.ad_value(171)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(906), s.ad_value(171)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        if (((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) && (!s.b[1693])) {s.store_scalar(172, 1.0);}
        if ((s.b[1620] && (!s.b[1685])) && (!s.b[1692])) {s.store_mul3_ad(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172)));}
        if (s.b[1620] && (!s.b[1685])) {s.copy_ad(375, 173);}
        if s.b[1620] {s.store_scaled_mul(1096, 379, 374, p.p2);s.store_scaled_mul(1097, 379, 375, p.p2);}
        s.b[1694] = (p.p44 == 0.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });s.b[1695] = ((s.v[865] <= 0.0) || (s.v[659] <= 0.0));s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1694]) && s.b[1695]) {s.store_scalar(373, 0.0);}
        s.b[1696] = (s.v[355] > (s.v[659] / 80.0));s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1694]) && (!s.b[1695])) && s.b[1696]) {s.store_div_scaled_inputs_indices(168, 659, -1.0, 355, 1.0);s.store_div_scaled_product_mixed_aai(373, A::mul3(s.ad_value(865), s.ad_value(355), s.ad_value(380)), A::limited_exp(s.ad_value(168)), 1.0, 365, 1.0);}
        if (((s.b[1620] && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) {s.store_div_scaled_product3_indices(373, 865, 355, 380, 1.804851387e-35, 365, 1.0);}
        s.b[1697] = (p.p44 == 1.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });s.b[1698] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1694])) && s.b[1697]) && s.b[1698]) {s.store_scalar(373, 0.0);}
        if (((s.b[1620] && (!s.b[1694])) && s.b[1697]) && (!s.b[1698])) {s.store_add_scaled_product_mixed_iia(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p.p600, (((((-1.0)) * (p.p600))) + (1.0))), 1.0);s.store_scale(167, 875, s.v[184]);s.store_div_scaled_product_offset_denominator_indices(168, 870, 167, 1.0, 167, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && (!s.b[1694])) && s.b[1697]) && (!s.b[1698])) {s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt_square_offset(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), ((4.0 * p.p643) * p.p643)), 0.5), 1.0);s.store_add(170, 167, 872);s.store_scaled_add_sqrt_square_offset_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), ((4.0 * p.p644) * p.p644), 0.5);s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);let t2: A = A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)));s.store_neg_ad(372, A::offset(A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, t2), (((-(-10.0))) + ((-p.p645)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, t2), (((-(-10.0))) + ((-p.p645)))), (-((4.0 * (-10.0)) * p.p645))), 0.5), (-10.0)));s.store_mul_add_mixed_iia(373, 372, 380, A::mul3(s.ad_value(876), s.ad_value(211), s.ad_value(579)));}
        s.b[1699] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && s.b[1699]) {s.store_scalar(373, 0.0);}
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1699])) {s.store_add_scaled_product_mixed_iia(370, 869, (-1.0 / (s.v[184])), 874, A::scale_offset(s.ad_value(639), p.p600, (((((-1.0)) * (p.p600))) + (1.0))), 1.0);s.store_scale(167, 875, s.v[184]);s.store_div_scaled_product_offset_denominator_indices(168, 870, 167, 1.0, 167, 1.0, 1.0);s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt_square_offset(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), ((4.0 * p.p643) * p.p643)), 0.5), 1.0);s.store_add(170, 167, 872);s.store_scaled_add_sqrt_square_offset_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), ((4.0 * p.p644) * p.p644), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1699])) {s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);let t3: A = A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)));s.store_neg_ad(372, A::offset(A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, t3), (((-(-10.0))) + ((-p.p645)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, t3), (((-(-10.0))) + ((-p.p645)))), (-((4.0 * (-10.0)) * p.p645))), 0.5), (-10.0)));s.store_mul(376, 372, 380);}
        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {s.store_add_scaled_inputs(167, 878, 1.0 / (s.v[184]), 877, (s.v[184] * 1.0 / (s.v[184])));s.store_mul_scale_offset_rhs(378, 880, 639, p.p666, (((((-1.0)) * (p.p666))) + (1.0)));}
        s.b[1700] = (s.v[211] > 0.0);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && s.b[1700]) {s.store_sub(168, 378, 499);}
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1700])) {s.store_sub(168, 378, 498);}
        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {s.store_offset(169, 881, (-1.0));}
        s.b[1701] = (s.v[168] > 0.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && s.b[1701]) {s.store_mul_scaled_pow_ad_rhs(170, 879, -1.0, s.ad_value(168), s.ad_value(169));}
        if (((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) && (!s.b[1701])) {s.store_scalar(170, 0.0);}
        if ((s.b[1620] && (!s.b[1694])) && (!s.b[1697])) {s.store_limited_exp(171, 170);s.store_mul_ad_product_lhs_mixed_ai(377, A::mul3(s.ad_value(167), s.ad_value(211), s.ad_value(579)), 168, 171);s.store_add(373, 376, 377);}
        if s.b[1620] {s.store_mul(1095, 373, 379);s.store_mul(502, 666, 463);s.store_mul(505, 667, 494);s.store_scale(508, 671, (s.v[189] * p.p2));s.store_scalar(503, ((0.1) as f64).powf((-p.p913)));}
        s.b[1702] = (p.p913 == 1.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1702]) {s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1702])) {s.store_primal_offset_scaled_ad(504, A::scale(s.ad_value(503), ((0.05 * p.p913) * (1.0 + p.p913))), (-(1.0 / (1.0 - p.p913))), (1.0 / (1.0 - p.p913)));}
        if s.b[1620] {s.store_scalar(506, ((0.1) as f64).powf((-p.p915)));}
        s.b[1703] = (p.p915 == 1.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1703]) {s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1703])) {s.store_primal_offset_scaled_ad(507, A::scale(s.ad_value(506), ((0.05 * p.p915) * (1.0 + p.p915))), (-(1.0 / (1.0 - p.p915))), (1.0 / (1.0 - p.p915)));}
        if s.b[1620] {s.store_scalar(509, ((0.1) as f64).powf((-p.p917)));}
        s.b[1704] = (p.p917 == 1.0);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1704]) {s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));}
        if (s.b[1620] && (!s.b[1704])) {s.store_primal_offset_scaled_ad(510, A::scale(s.ad_value(509), ((0.05 * p.p917) * (1.0 + p.p917))), (-(1.0 / (1.0 - p.p917))), (1.0 / (1.0 - p.p917)));}
        s.b[1705] = (s.v[502] > 0.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1705]) {s.store_div(168, 498, 672);}
        s.b[1706] = (s.v[168] < 0.9);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1705]) && s.b[1706]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1707] = (p.p913 != 1.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });s.b[1708] = (p.p913 == 0.5);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1705]) && s.b[1706]) && s.b[1707]) && s.b[1708]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if ((((s.b[1620] && s.b[1705]) && s.b[1706]) && s.b[1707]) && (!s.b[1708])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p913));}
    }
}
