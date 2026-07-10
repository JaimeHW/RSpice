#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_160(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));s.store_add_scaled_inputs_product_mixed_aaai(6, A::add_scaled_product(s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), s.ad_value(727), (-1.0)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), s.ad_value(168), s.ad_value(269)), 1.0, A::offset(s.ad_value(3), 1.0), 46, 1.0);}
        s.b[1829] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1829, if s.b[1829] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1829]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_161(
        s: &mut Scratch,
    ) {
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
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_162(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_163(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1829])) {let t0: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t0, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.copy_ad(123, 23);}
        s.b[1830] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1830, if s.b[1830] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1830]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_164(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1830]) {s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
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
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_165(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_166(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {let t1: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t1, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.copy_ad(123, 23);}
        s.b[1831] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1831, if s.b[1831] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1831]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_167(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1831]) {s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
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
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_168(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_169(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {let t2: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t2, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_sub(62, 23, 22);s.store_mul(63, 226, 270);s.store_limited_exp_neg_input(64, 63);}
        s.b[1832] = (s.v[62] < 1e-10);s.store_scalar(1832, if s.b[1832] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_170(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
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
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)));s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_sub(177, 123, 98);s.store_limited_exp_ad(178, A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(269), 2.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(179, A::add(A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(269), 2.0, s.ad_value(269), 1.0), s.ad_value(170)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_171(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {s.store_neg_ad(65, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(63), (-1.0), s.ad_value(98), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(63), -1.0, s.ad_value(98), 1.0)), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), 1.0, (-1.0)));s.store_mul_mixed_ai(66, A::mul_sub_from_scalar_rhs(s.ad_value(296), 1.0, s.ad_value(64)), 57);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_172(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {let t3: A = A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product3_by_product(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0, A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), (-1.0), s.ad_value(296), A::sub(A::sub(A::add_scaled_inputs4(A::add_scaled_inputs_product(s.ad_value(173), 1.0, A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(98), (-1.0), s.ad_value(63), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(63), 1.0)), A::sub(A::add_scaled_product(s.ad_value(175), (-2.0), A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(175), 10.0), s.ad_value(175), 1.0), A::mul3(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 8.0), s.ad_value(123), s.ad_value(175)), s.ad_value(175), s.ad_value(175))), 1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), (-1.0), A::div(s.ad_value(178), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), 1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), A::div(s.ad_value(179), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), A::div(s.ad_value(179), A::mul(A::square(A::offset(s.ad_value(3), 1.0)), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))))), (-1.0));s.store_offset_sub_ad(54, t3, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_173(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {s.store_add_scaled_square_product_indices(54, 65, 1.0, 54, 66, (-2.0));}
        s.b[1833] = (s.v[54] >= 0.0);s.store_scalar(1833, if s.b[1833] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) && s.b[1833]) {s.store_scaled_div_mixed_ia(62, 66, A::add(s.ad_value(65), A::sqrt(s.ad_value(54))), 2.0);}
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {s.store_add(23, 22, 62);}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_mul(250, 62, 269);s.store_div_scaled_product_offset_denominator_mixed_iia(67, 23, 23, 1.0, A::square(s.ad_value(23)), 2.0, 1.0);s.store_limited_exp_neg_input(68, 23);s.store_add_scaled_product(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67)), (-1.0));s.store_add_scaled_product_mixed_iaa(70, 69, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(23))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_offset_add_scaled_inputs(70, A::offset(s.ad_value(70), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(70), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(60, 70);s.store_mul_sqrt_mixed_ia(72, 294, A::add(s.ad_value(70), s.ad_value(69)));s.store_div_scaled_product3_mixed_iiia(73, 296, 69, 269, 1.0, A::add_scaled_product(s.ad_value(72), 1.0, s.ad_value(294), s.ad_value(60), 1.0), 1.0);s.store_scaled_add(75, 22, 23, 0.5);s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));s.store_sqrt(76, 54);s.store_scaled_add(77, 57, 69, 0.5);s.store_add_scaled_product_mixed_iaa(78, 77, 1.0, A::square(s.ad_value(62)), A::sub_scaled_inputs(s.ad_value(76), 1.0, s.ad_value(297), 2.0), 0.125);s.store_add_scaled_product_mixed_iaa(79, 78, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(75))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_mul_sqrt_mixed_ia(51, 294, A::add(s.ad_value(78), s.ad_value(79)));s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(79), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(71, 79);}
        s.b[1834] = (p.p46 == 1.0);s.store_scalar(1834, if s.b[1834] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1834]) {s.store_div_scaled_inputs_indices(85, 269, ((2.0 * s.v[199]) * s.v[199]), 704, (1.602176462e-19 * s.v[180]));s.store_add_scaled_sub_value_product_mixed_iia(86, 1.0, 76, 1.0, 51, A::div_from_scalar(1.0, s.ad_value(296)), 2.0);s.store_div_from_scalar_sqrt_ad(87, 1.0, A::offset(A::mul(s.ad_value(85), s.ad_value(51)), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_174(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1834]) {s.store_div_scaled_value_offset_denominator(54, s.ad_value(87), 1.0, s.ad_value(87), 1.0, 1.0);s.store_mul_ad_product_rhs(88, 85, A::mul3(A::square(s.ad_value(54)), s.ad_value(51), s.ad_value(51)), A::div(s.ad_value(78), A::add(s.ad_value(78), s.ad_value(79))));s.store_add_scaled_inputs_product_mixed_iiia(89, 51, 2.0, 88, (-2.0), 296, A::add(A::sub_from_scalar(1.0, s.ad_value(76)), s.ad_value(78)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(90, 88, 88, 1.0, 51, 2.0);s.store_sub_from_scalar_scaled_mul_mixed_ia(91, 1.0, 296, A::add(s.ad_value(76), s.ad_value(78)), 0.5);s.store_div_scaled_product_mixed_iia(92, 90, 89, 1.0, A::add_scaled_square_product(s.ad_value(89), 1.0, s.ad_value(91), s.ad_value(90), (-1.0)), 1.0);s.store_add(75, 75, 92);s.store_limited_exp(93, 92);s.store_div(76, 76, 93);s.store_mul(78, 78, 93);s.store_add_scaled_product(79, A::div(s.ad_value(78), s.ad_value(93)), (-1.0), A::square(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(75), (-1.0), s.ad_value(92), 1.0)), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_mul_sqrt_mixed_ia(51, 294, A::add(s.ad_value(78), s.ad_value(79)));s.store_add_ad(94, A::sub_from_scalar(1.0, s.ad_value(76)), A::mul3_scaled_output(s.ad_value(51), s.ad_value(87), s.ad_value(297), 2.0));s.store_div_scaled_product3_mixed_iiaa(62, 62, 93, A::add(s.ad_value(86), s.ad_value(77)), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(93), s.ad_value(77), 1.0), 1.0);s.store_mul(250, 62, 269);s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(79), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(71, 79);}
        s.b[1835] = (((s.v[250]) as f64).abs() > 1e-35);s.store_scalar(1835, if s.b[1835] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1835]) {s.store_div_scaled_inputs2_indices(74, 306, 1.0, 73, (-1.0), 250, 1.0);}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_mul_div_scaled_product_mixed_iiia(80, 269, 296, 78, 1.0, A::add_scaled_product(s.ad_value(51), 1.0, s.ad_value(294), s.ad_value(71), 1.0), 1.0);s.store_add_scaled_product_indices(81, 80, 1.0, 269, 74, 1.0);}
        if (s.b[1620] && s.b[1799]) {s.store_mul_ad_product_lhs(939, A::mul3_scaled_output(s.ad_value(740), s.ad_value(81), s.ad_value(250), ((p.p2 * (p.p1147 / s.v[184])) * s.v[199])), A::div_scaled_product(s.ad_value(354), s.ad_value(344), 1.0, s.ad_value(458), 1.0), 363);s.store_add(380, 380, 939);}
        if (!s.b[1620]) {s.store_div(252, 251, 267);s.store_scalar(168, 1.0);s.store_div(404, 213, 168);s.store_div(405, 294, 168);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_175(
        s: &mut Scratch,
    ) {
        if (!s.b[1620]) {s.store_sub_scaled_inputs_mixed_ia(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));}
        s.b[1836] = (s.v[404] < 0.0);s.store_scalar(1836, if s.b[1836] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1836]) {s.store_div_scaled_inputs2_indices(170, 404, 1.0, 169, (-1.0), 405, 1.0);s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));}
        if ((!s.b[1620]) && (!s.b[1836])) {s.store_limited_exp_neg_input(170, 169);s.store_scale(168, 405, 0.5);s.store_sub_mixed_ai(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);s.store_sub_offset_lhs_mixed_ai(254, A::square(s.ad_value(169)), 1.0, 170);}
        if (!s.b[1620]) {s.store_scaled_add_offset_sqrt_square_offset(175, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(259, 175);s.store_div_scaled_offset_numerator_mixed_ai(167, A::div_scaled_inputs(s.ad_value(294), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, 294, 1.0);s.store_add_scaled_inputs3_indices(168, 254, 1.0, 252, (-2.0), 225, -1.0);s.store_sub_mixed_ia(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);s.copy_ad(257, 259);}
        s.b[1837] = (s.v[175] <= (-68.0));s.store_scalar(1837, if s.b[1837] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1837]) {s.store_scalar(171, (-100.0));s.store_scalar(172, 20.0);}
        s.b[1838] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));s.store_scalar(1838, if s.b[1838] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1837]) && s.b[1838]) {s.store_limited_exp(170, 171);}
        s.b[1839] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));s.store_scalar(1839, if s.b[1839] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1837]) && (!s.b[1838])) && s.b[1839]) {s.store_limited_exp(170, 175);}
        if ((((!s.b[1620]) && s.b[1837]) && (!s.b[1838])) && (!s.b[1839])) {s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);s.store_square(173, 169);s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));}
        if ((!s.b[1620]) && s.b[1837]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(400, 170, 168, 1.0, 175, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);}
        if ((!s.b[1620]) && (!s.b[1837])) {s.store_limited_exp(170, 175);s.store_div_from_scalar(258, 1.0, 257);}
    }
}
