#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(272), 1.0, s.ad_value(724), s.ad_value(272), 1.0));s.store_add_scaled_inputs_product_mixed_aaai(6, A::add_scaled_product(s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(271)), s.ad_value(727), (-1.0)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), s.ad_value(168), s.ad_value(271)), 1.0, A::offset(s.ad_value(3), 1.0), 46, 1.0);}
        s.b[1767] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1767]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        s: &mut Scratch,
    ) {
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
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1767])) {let t0: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t0, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {s.copy_ad(123, 23);}
        s.b[1768] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1768]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1768]) {s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));}
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
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1768])) {let t1: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t1, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {s.copy_ad(123, 23);}
        s.b[1769] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1769]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1769]) {s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));}
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
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && (!s.b[1769])) {let t2: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t2, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {s.store_sub(62, 23, 22);s.store_mul(63, 226, 272);s.store_limited_exp_neg_input(64, 63);}
        s.b[1770] = (s.v[62] < 1e-10);s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_122(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0));}
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
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(271), 1.0, s.ad_value(271), 1.0)));s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_sub(177, 123, 98);s.store_limited_exp_ad(178, A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(271), 2.0, s.ad_value(271), 1.0));s.store_limited_exp_ad(179, A::add(A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(271), 2.0, s.ad_value(271), 1.0), s.ad_value(170)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_123(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {s.store_neg_ad(65, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(63), (-1.0), s.ad_value(98), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(63), -1.0, s.ad_value(98), 1.0)), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), 1.0, (-1.0)));s.store_mul_mixed_ai(66, A::mul_sub_from_scalar_rhs(s.ad_value(296), 1.0, s.ad_value(64)), 57);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_124(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {let t3: A = A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product3_by_product(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0, A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), (-1.0), s.ad_value(296), A::sub(A::sub(A::add_scaled_inputs4(A::add_scaled_inputs_product(s.ad_value(173), 1.0, A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(98), (-1.0), s.ad_value(63), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(63), 1.0)), A::sub(A::add_scaled_product(s.ad_value(175), (-2.0), A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(175), 10.0), s.ad_value(175), 1.0), A::mul3(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 8.0), s.ad_value(123), s.ad_value(175)), s.ad_value(175), s.ad_value(175))), 1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), (-1.0), A::div(s.ad_value(178), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), 1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), A::div(s.ad_value(179), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), A::div(s.ad_value(179), A::mul(A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), (-1.0));s.store_offset_sub_ad(54, t3, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_125(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {s.store_add_scaled_square_product_indices(54, 65, 1.0, 54, 66, (-2.0));}
        s.b[1771] = (s.v[54] >= 0.0);s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) && s.b[1771]) {s.store_scaled_div_mixed_ia(62, 66, A::add(s.ad_value(65), A::sqrt(s.ad_value(54))), 2.0);}
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1770]) {s.store_add(23, 22, 62);}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {s.store_mul(250, 62, 271);s.store_div_scaled_product_offset_denominator_mixed_iia(67, 23, 23, 1.0, A::square(s.ad_value(23)), 2.0, 1.0);s.store_limited_exp_neg_input(68, 23);s.store_add_scaled_product(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67)), (-1.0));s.store_add_scaled_product_mixed_iaa(70, 69, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(23))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_offset_add_scaled_inputs(70, A::offset(s.ad_value(70), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(70), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(60, 70);s.store_mul_sqrt_mixed_ia(72, 294, A::add(s.ad_value(70), s.ad_value(69)));s.store_div_scaled_product3_mixed_iiia(73, 296, 69, 271, 1.0, A::add_scaled_product(s.ad_value(72), 1.0, s.ad_value(294), s.ad_value(60), 1.0), 1.0);s.store_scaled_add(75, 22, 23, 0.5);s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));s.store_sqrt(76, 54);s.store_scaled_add(77, 57, 69, 0.5);s.store_add_scaled_product_mixed_iaa(78, 77, 1.0, A::square(s.ad_value(62)), A::sub_scaled_inputs(s.ad_value(76), 1.0, s.ad_value(297), 2.0), 0.125);s.store_add_scaled_product_mixed_iaa(79, 78, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(75))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_mul_sqrt_mixed_ia(51, 294, A::add(s.ad_value(78), s.ad_value(79)));s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(79), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(71, 79);}
        s.b[1772] = (((s.v[250]) as f64).abs() > 1e-35);s.store_scalar(1772, if s.b[1772] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1741]) && (!s.b[1755])) && s.b[1772]) {s.store_div_scaled_inputs2_indices(74, 306, 1.0, 73, (-1.0), 250, 1.0);}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {s.store_mul_div_scaled_product_mixed_iiia(80, 271, 296, 78, 1.0, A::add_scaled_product(s.ad_value(51), 1.0, s.ad_value(294), s.ad_value(71), 1.0), 1.0);s.store_mul(52, 51, 271);s.copy_ad(83, 74);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_126(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1741]) && (!s.b[1755])) {s.store_offset_add_scaled_inputs(83, A::offset(s.ad_value(83), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(83), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_add_scaled_product_indices(81, 80, 1.0, 271, 83, 1.0);s.store_div(84, 81, 83);}
        s.b[1773] = (s.v[22] <= 0.0);s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1741]) && s.b[1773]) {s.copy_ad(447, 52);s.store_scalar(444, 0.0);s.copy_ad(445, 447);s.store_scalar(446, 0.0);}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1773])) {s.store_scaled_div(26, 250, 84, 0.5);s.store_square(27, 26);s.store_add_scaled_product_indices(447, 52, 1.0, 250, 26, (0.3333333333333333 * 0.5));s.store_scaled_mul(54, 74, 250, 0.16666666666666666);s.store_add_scaled_product_indices(443, 80, 1.0, 54, 26, 1.0);s.store_add_scaled_product_mixed_iia(444, 80, 0.5, 54, A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(26)), 1.0, s.ad_value(27), 0.2), ((-1.0) * 0.5));s.store_sub(445, 447, 443);s.store_add_scaled_inputs3_indices(446, 447, 1.0, 445, (-1.0), 444, -1.0);}
        if (s.b[1620] && s.b[1741]) {s.store_scale_ad(437, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), p.p1380);s.copy_ad(391, 437);s.store_mul_scale_offset_indices(440, 445, 391, -1.0, 0.0);}
        s.b[1774] = (s.v[211] > 0.0);s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1741]) && s.b[1774]) {s.store_mul_scale_offset_indices(441, 446, 391, -1.0, 0.0);s.store_mul_scale_offset_indices(439, 444, 391, -1.0, 0.0);}
        if ((s.b[1620] && s.b[1741]) && (!s.b[1774])) {s.store_mul_scale_offset_indices(441, 444, 391, -1.0, 0.0);s.store_mul_scale_offset_indices(439, 446, 391, -1.0, 0.0);}
        if (s.b[1620] && s.b[1741]) {s.store_add_scaled_inputs3_indices(442, 440, (-1.0), 441, (-1.0), 439, (-1.0));}
        if (s.b[1620] && (!s.b[1741])) {s.store_scalar(440, 0.0);s.store_scalar(439, 0.0);s.store_scalar(438, 0.0);s.store_scalar(441, 0.0);s.store_scalar(442, 0.0);}
        if s.b[1620] {s.store_mul_add_mixed_iai(1075, 379, A::add_scaled_inputs4(s.ad_value(387), 1.0, s.ad_value(440), 1.0, s.ad_value(421), 1.0, s.ad_value(520), 1.0), 525);s.store_mul_add_rhs(1050, 379, 388, 441);s.store_mul_add_rhs(1053, 379, 389, 439);s.store_mul_add_scaled_inputs4_indices_rhs(1076, 379, 388, 1.0, 441, 1.0, 423, 1.0, 520, -1.0);s.store_mul_add_scaled_inputs4_indices_rhs(1077, 379, 389, 1.0, 439, 1.0, 424, 1.0, 525, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(1078, 379, 390, 1.0, 442, 1.0, 422, 1.0, 0.0);s.store_mul(1057, 379, 390);s.store_mul(1058, 379, 442);s.store_mul(1051, 379, 388);s.store_mul(1052, 379, 441);s.store_mul(1054, 379, 389);s.store_mul(1055, 379, 439);s.store_add_scaled_offset_product_rhs(810, 810, 1.0, 813, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(816, 816, 1.0, 814, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(819, 819, 1.0, 815, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(884, 884, 1.0, 886, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(882, 882, 1.0, 887, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(888, 888, 1.0, 891, 639, (-1.0), 1.0);s.store_scalar(477, 0.0);s.store_scalar(479, 0.0);s.store_scalar(480, 0.0);s.store_scalar(483, 0.0);s.store_scalar(484, 0.0);}
        s.b[1775] = ((p.p37 != 0.0) || (p.p38 != 0.0));s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1775]) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(469, 269, 213, 1.0, 22, (-0.5), 23, (-0.5), 0.0);s.store_sqrt_square_offset(168, 469, 0.0001);s.store_scaled_sub(471, 168, 469, 0.5);s.store_scaled_add(470, 469, 168, 0.5);}
        s.b[1776] = (p.p38 != 0.0);s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {s.store_scale(168, 469, 1.0 / (p.p671));}
        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            s.store_scale_ad(474, {
                if ((!((-s.v[168]) > 37.0)) && (!((-s.v[168]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::neg(s.ad_value(168)))
                } else {
                    {
                        if ((!((-s.v[168]) > 37.0)) && ((-s.v[168]) < (-37.0))) {
                            A::exp_scaled_input(s.ad_value(168), -1.0)
                        } else {
                            {
                                if ((-s.v[168]) > 37.0) {
                                    A::neg(s.ad_value(168))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, p.p671);
        }
        s.b[1777] = (p.p696 != 0.0);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1777]) {s.store_sub_from_scalar_scaled_input(167, 1.0, 471, 1.0 / (p.p696));}
        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && (!s.b[1777])) {s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_127(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1778] = (s.v[167] < 0.01);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1778]) {s.store_scalar(167, 0.01);}
        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));s.store_scalar(169, (p.p701 * p.p76));s.store_div_scaled_product_mixed_iai(170, 169, A::add_scaled_product(s.ad_value(882), 1.0, s.ad_value(883), s.ad_value(471), (-1.0)), 1.0, 167, 1.0);s.store_limited_exp(171, 170);s.store_mul_product3_indices(476, 171, 168, 221, 474, 1.0);s.store_mul(476, 476, 662);s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));}
        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {
            s.store_scale_ad(473, {
                if ((!(s.v[168] > 37.0)) && (!(s.v[168] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(168))
                } else {
                    {
                        if ((!(s.v[168] > 37.0)) && (s.v[168] < (-37.0))) {
                            A::exp(s.ad_value(168))
                        } else {
                            {
                                if (s.v[168] > 37.0) {
                                    s.ad_value(168)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, p.p671);
        }
        s.b[1779] = (p.p697 != 0.0);s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1779]) {s.store_sub_from_scalar_scaled_input(167, 1.0, 470, 1.0 / (p.p697));}
        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && (!s.b[1779])) {s.store_scalar(167, 1.0);}
        s.b[1780] = (s.v[167] < 0.01);s.store_scalar(1780, if s.b[1780] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1775]) && s.b[1776]) && s.b[1780]) {s.store_scalar(167, 0.01);}
        if ((s.b[1620] && s.b[1775]) && s.b[1776]) {s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));s.store_scalar(169, (p.p699 * p.p76));s.store_div_scaled_product_mixed_iai(170, 169, A::add_scaled_product(s.ad_value(884), 1.0, s.ad_value(885), s.ad_value(470), (-1.0)), 1.0, 167, 1.0);s.store_limited_exp(171, 170);s.store_mul_product3_indices(475, 171, 168, 221, 473, 1.0);s.store_mul(475, 475, 662);s.store_scaled_add(477, 476, 475, p.p2);s.store_offset_mul(478, 212, 269, p.p1383);}
        s.b[1781] = (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0));s.store_scalar(1781, if s.b[1781] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {s.store_mul_voltage_ad(208, s.ad_value(379), ctx, nodes, Some(8), Some(11));s.store_sub(167, 208, 478);s.store_sqrt_square_offset(168, 167, 0.0001);s.store_offset_scaled_sub(209, 168, 167, 0.5, (((-0.01)) * (0.5)));}
        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));}
        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));}
        if ((s.b[1620] && s.b[1775]) && s.b[1781]) {s.store_mul(169, 208, 209);s.store_add_scaled_product_indices(170, 889, (-1.0), 888, 890, 1.0);s.store_mul(171, 889, 890);s.store_mul_sub_scaled_inputs_rhs(172, 179, A::add_scaled_product(s.ad_value(888), 1.0, s.ad_value(170), s.ad_value(209), 1.0), (-p.p76), A::mul3(s.ad_value(171), s.ad_value(209), s.ad_value(209)), (-p.p76));s.store_limited_exp(173, 172);s.store_scaled_mul(178, 178, 492, p.p1380);s.store_mul_product3_indices(210, 662, 178, 169, 173, 1.0);}
        if ((s.b[1620] && s.b[1775]) && (!s.b[1781])) {s.store_scalar(210, 0.0);}
        s.b[1782] = (p.p37 != 0.0);s.store_scalar(1782, if s.b[1782] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1775]) && s.b[1782]) {s.store_add_scaled_product_indices(168, 810, 1.0, 811, 470, (-1.0));s.store_offset_mul(169, 812, 470, 1.0);s.store_scaled_mul(170, 168, 169, s.v[488]);s.store_mul_product3_mixed_aiii(171, A::limited_exp(s.ad_value(170)), 253, 269, 243, 1.0);s.store_mul_product3_mixed_iiia(481, 662, 487, 171, A::add_scaled_inputs4(s.ad_value(221), 1.0, s.ad_value(227), 0.5, s.ad_value(224), (-0.5), s.ad_value(223), (-0.5)), p.p2);s.store_offset_sqrt_ad(472, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));s.store_scale(168, 472, s.v[823]);s.store_limited_exp_neg_input(482, 168);s.store_offset_add(170, 168, 482, (((-1.0)) + (0.0001)));s.store_offset_sub_from_scalar_ad(171, 1.0, A::mul_offset_lhs(s.ad_value(168), 1.0, s.ad_value(482)), 0.0001);s.store_offset_square(172, 168, 0.0002);}
    }
}
