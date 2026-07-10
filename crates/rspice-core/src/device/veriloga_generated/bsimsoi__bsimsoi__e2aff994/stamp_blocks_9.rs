#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_144(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1814])) {let t0: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t0, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if (s.b[1620] && s.b[1799]) {s.copy_ad(123, 22);}
        s.b[1815] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1815]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_145(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_146(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_147(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1815])) {let t1: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t1, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if (s.b[1620] && s.b[1799]) {s.copy_ad(123, 22);}
        s.b[1816] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1816, if s.b[1816] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1816]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_148(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && s.b[1816]) {s.store_mul_ad_product_rhs_mixed_ia(22, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
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
        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 97);s.store_limited_exp_sub(177, 123, 97);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_149(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_150(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1816])) {let t2: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t2, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if (s.b[1620] && s.b[1799]) {s.store_scale(50, 269, 3.912023005);}
        s.b[1817] = (s.v[22] <= 0.0);s.store_scalar(1817, if s.b[1817] { 1.0 } else { 0.0 });
        if ((s.b[1620] && s.b[1799]) && s.b[1817]) {s.store_scalar(306, 0.0);s.store_sub(51, 214, 22);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_151(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1799]) && s.b[1817]) {s.copy_ad(312, 50);s.store_scalar(458, 1.0);s.store_scalar(334, 1.0);s.store_scalar(853, 1.0);s.store_scalar(343, 1.0);s.store_scalar(339, 1.0);s.store_scalar(363, 1.0);s.store_scalar(365, 1.0);s.copy_ad(455, 453);s.copy_ad(454, 452);}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_div_from_scalar_offset_square(54, 1.0, 22, 2.0);s.store_mul_square_lhs(55, 22, 54);s.store_limited_exp(53, 22);s.store_div_from_scalar(56, 1.0, 53);s.store_limited_exp_sub(53, 22, 97);s.store_add_scaled_product_mixed_iaa(57, 53, 1.0, A::limited_exp_scaled_input(s.ad_value(97), -1.0), A::add(A::offset(s.ad_value(22), 1.0), s.ad_value(55)), (-1.0));s.store_add_scaled_product_mixed_iaa(58, 57, (-1.0), A::square(A::sub(s.ad_value(214), s.ad_value(22))), A::div_from_scalar(1.0, s.ad_value(296)), 1.0);s.store_offset_add_scaled_inputs(58, A::offset(s.ad_value(58), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(58), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_sqrt(59, 58);s.store_mul_sqrt_mixed_ia(61, 294, A::add(s.ad_value(58), s.ad_value(57)));s.store_div_scaled_product3_mixed_iiia(306, 296, 57, 269, 1.0, A::add_scaled_product(s.ad_value(61), 1.0, s.ad_value(294), s.ad_value(59), 1.0), 1.0);s.store_mul3_lhs(247, 59, 294, 269);s.copy_ad(76, 56);s.copy_ad(78, 57);s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));s.store_mul_add_scaled_inputs_rhs_indices(308, 335, 247, 1.0, 306, s.v[338]);s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scaled_offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0, 0.5), 1e-38))));s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(308), s.ad_value(651)), 1.0);s.store_offset(171, 170, 1.0);s.store_scaled_add_offset_sqrt_square_offset(309, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);s.store_primal_div_from_scalar_scaled_ad(448, 1.0, A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2);}
        s.b[1818] = (p.p33 == 1.0);s.store_scalar(1818, if s.b[1818] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1818]) {s.store_scalar(456, 0.0);}
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1818])) {s.store_offset_mul(167, 770, 306, 1.0);s.store_mul_sub_rhs(168, 787, 274, 299);s.store_add_mixed_ai(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_152(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1818])) {s.store_add_mixed_ia(170, 169, A::sqrt_square_offset(s.ad_value(169), 0.01));s.store_mul_ad_affine_product_lhs(456, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2, 0.0, 652);}
        s.b[1819] = (p.p33 == 2.0);s.store_scalar(1819, if s.b[1819] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1818])) && s.b[1819]) {s.store_mul_add_mixed_iai(456, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), 453);}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_mul_div_scaled_inputs_indices(310, 309, 746, 2.0, 740, 1.0);s.store_scale(311, 310, s.v[184]);s.store_mul_add_scaled_inputs_rhs_indices(173, 742, 306, 1.0, 269, 2.0);}
        s.b[1820] = (s.v[456] > 0.0);s.store_scalar(1820, if s.b[1820] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1820]) {s.store_scale(324, 746, (s.v[183] * s.v[199]));s.store_mul(167, 324, 456);s.store_scale(325, 167, 2.0);s.store_add_scaled_inputs_product_indices(326, 173, 1.0, 311, 1.0, 173, 167, 3.0);s.store_mul_add_scaled_product_rhs_indices(327, 173, 311, 1.0, 173, 167, 2.0);s.store_div_scaled_inputs2_mixed_iai(312, 326, 1.0, A::sqrt(A::add_scaled_square_product(s.ad_value(326), 1.0, s.ad_value(325), s.ad_value(327), (-2.0))), (-1.0), 325, 1.0);}
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1820])) {s.store_div_scaled_product_add_scaled_denominator_indices(312, 311, 173, 1.0, 311, 1.0, 173, 1.0, 1.0);}
        s.b[1821] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));s.store_scalar(1821, if s.b[1821] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1821]) {s.store_scalar(1019, 1.0);}
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1821])) {s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);s.store_offset_div_scaled_inputs2_mixed_iaa(1019, 168, p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(306), p.p1351), s.ad_value(269), p.p1350), (-1.0), A::scale_offset(s.ad_value(218), p.p1352, 1.0), 1.0, 1.0);s.store_scaled_add_offset_sqrt_square_offset(1019, 1019, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_offset_add_scaled_inputs(312, A::offset(s.ad_value(312), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(312), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);s.store_div(312, 312, 1019);s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(312)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));s.store_mul(315, 226, 175);s.store_mul_add_lhs(318, 315, 224, 270);s.store_add_mixed_ai(98, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 318);s.store_limited_exp_neg_input(100, 98);}
        let (t4,) = {
    if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
        let t3: f64 = (0.001 * s.v[95]);
        (t3,)
    } else {
        (s.v[101],)
    }
};
        s.store_scalar(101, t4);
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_153(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));s.store_add_scaled_offset_product_lhs_mixed_aii(4, A::add_scaled_inputs3(A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(727)), -1.0), 1.0, 3, 1.0, 168, 1.0);}
        let (tc,) = {
    if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
        let t5: f64 = (-s.v[4]);let t6: f64 = { let limited_exp_arg = t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };let t7: f64 = (t6 + s.v[4]);let t8: f64 = (t7 - 1.0);let t9: f64 = (t8).sqrt();let ta: f64 = (s.v[294] * t9);let tb: f64 = (s.v[4] + ta);
        (tb,)
    } else {
        (s.v[104],)
    }
};
        s.store_scalar(104, tc);s.b[1822] = (s.v[4] < s.v[98]);s.store_scalar(1822, if s.b[1822] { 1.0 } else { 0.0 });s.b[1823] = (s.v[214] < s.v[104]);s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });s.b[1824] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });
        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && s.b[1824]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1825] = (s.v[214] < (-s.v[101]));s.store_scalar(1825, if s.b[1825] { 1.0 } else { 0.0 });
        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && s.b[1825]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_154(
        s: &mut Scratch,
    ) {
        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && s.b[1825]) {s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 100, 29);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && (!s.b[1825])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 98, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_155(
        s: &mut Scratch,
    ) {
        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && (!s.b[1825])) {s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);s.store_add_mixed_ia(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));s.store_limited_exp(28, 44);s.store_div_from_scalar(29, 1.0, 28);s.store_limited_exp_sub(28, 44, 98);s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);s.store_mul_square_lhs(30, 44, 13);s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 214, 44);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_156(
        s: &mut Scratch,
    ) {
        if ((((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && s.b[1823]) && (!s.b[1824])) && (!s.b[1825])) {s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_add_scaled_inputs_mixed_ia(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) {s.copy_ad(47, 2);s.store_primal_square(48, 47);s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));s.store_add_scaled_product_mixed_iia(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));s.store_offset(43, 98, 3.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(106, 105, 0.5, 43, 0.5, 105, 43, 40.0, (-0.5));s.store_add_scaled_inputs_product_mixed_aaii(107, A::square(A::sub(s.ad_value(214), s.ad_value(106))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, 4, (-1.0));s.store_add_scaled_inputs_product_mixed_iiia(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));s.store_square(109, 108);s.store_primal_sub_from_scalar(110, 1.0, 48);}
        s.b[1826] = (s.v[107] < 0.0);s.store_scalar(1826, if s.b[1826] { 1.0 } else { 0.0 });
        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) && s.b[1826]) {s.store_scalar(107, 0.0);}
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) {s.store_add_scaled_inputs3_mixed_iia(49, 98, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);s.store_add(111, 107, 108);s.store_square(112, 111);s.store_add_scaled_inputs_product_mixed_aiii(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));s.store_div_scaled_product_mixed_iia(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);s.store_add(117, 106, 116);s.store_limited_exp_sub(118, 117, 98);s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_157(
        s: &mut Scratch,
    ) {
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1822]) && (!s.b[1823])) {s.store_add_scaled_inputs_product_mixed_aaia(120, A::square(A::sub(s.ad_value(214), s.ad_value(117))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, A::add(s.ad_value(4), s.ad_value(118)), (-1.0));s.store_mul_add_scaled_sub_value_product_rhs_mixed_aii(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, 296, 118, (((-1.0)) * (2.0)));s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);s.store_add(9, 117, 122);}
        s.b[1827] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1827, if s.b[1827] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && s.b[1827]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1828] = (s.v[214] < (-s.v[101]));s.store_scalar(1828, if s.b[1828] { 1.0 } else { 0.0 });
        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && s.b[1828]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_158(
        s: &mut Scratch,
    ) {
        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && s.b[1828]) {s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 100, 29);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && (!s.b[1828])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 98, 3.0);s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_159(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1822])) && (!s.b[1827])) && (!s.b[1828])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);s.store_add_mixed_ia(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));s.store_limited_exp(28, 44);s.store_div_from_scalar(29, 1.0, 28);s.store_limited_exp_sub(28, 44, 98);s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);s.store_mul_square_lhs(30, 44, 13);s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 214, 44);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_add_scaled_inputs_mixed_ia(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.copy_ad(123, 9);}
        let (td,) = {
    if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
        (1e-7,)
    } else {
        (s.v[102],)
    }
};
        s.store_scalar(102, td);
        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {s.store_scalar(103, 2.0);s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);}
    }
}
