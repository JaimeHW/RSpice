#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            let assign44260_ad_e74567: A = A::add_scaled_value_products(A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_ad_value(19, assign44260_ad_e74567);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            let assign44270_ad_e74642: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            let assign44270_ad_e74658: A = A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add(assign44270_ad_e74642, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
            s.store_ad_value(20, assign44270_ad_e74658);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            let assign44280_ad_e74714: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign44280_ad_e74773: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign44280_ad_e74802: A = A::sub(A::add_scaled_product(assign44280_ad_e74714, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign44280_ad_e74773, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign44280_ad_e74802, 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1830])) {
            s.store_ad_value(23, A::add_scaled_offset_product_rhs(s.ad_value(123), 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0)));
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.copy_ad(123, 23);
        }

        s.b[1831] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1831] = if s.b[1831] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1831]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_ad_lhs(45, A::mul3(s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0)), 167);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            let assign44380_ad_e75075: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign44380_ad_e75074: A = {
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
                    };
                    assign44380_ad_e75074
                }
            };
            let assign44380_ad_e75157: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign44380_ad_e75156: A = {
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
                    };
                    assign44380_ad_e75156
                }
            };
            s.store_sub_ad(169, assign44380_ad_e75075, assign44380_ad_e75157);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            let assign44460_ad_e75317: A = A::add_scaled_value_products(A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
            s.store_ad_value(19, assign44460_ad_e75317);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            let assign44470_ad_e75392: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            let assign44470_ad_e75408: A = A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add(assign44470_ad_e75392, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
            s.store_ad_value(20, assign44470_ad_e75408);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            let assign44480_ad_e75464: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign44480_ad_e75523: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign44480_ad_e75552: A = A::sub(A::add_scaled_product(assign44480_ad_e75464, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign44480_ad_e75523, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign44480_ad_e75552, 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && (!s.b[1831])) {
            s.store_ad_value(23, A::add_scaled_offset_product_rhs(s.ad_value(123), 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0)));
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
            let assign44560_ad_e75746: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign44560_ad_e75745: A = {
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
                    };
                    assign44560_ad_e75745
                }
            };
            let assign44560_ad_e75828: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign44560_ad_e75827: A = {
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
                    };
                    assign44560_ad_e75827
                }
            };
            s.store_sub_ad(169, assign44560_ad_e75746, assign44560_ad_e75828);
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
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            let assign44660_ad_e76049: A = A::add_scaled_offset_product_rhs(A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(63), (-1.0), s.ad_value(98), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(63), -1.0, s.ad_value(98), 1.0)), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0);
            let assign44660_ad_e76075: A = A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add_scaled_inputs4(assign44660_ad_e76049, 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), 1.0, (-1.0));
            s.store_neg_ad(65, assign44660_ad_e76075);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            s.store_mul_ad_lhs(66, A::mul_sub_from_scalar_rhs(s.ad_value(296), 1.0, s.ad_value(64)), 57);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            let assign44680_ad_e76146: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product3(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))), 1.0));
            let assign44680_ad_e76191: A = A::mul(A::limited_exp(A::sub_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(63), 1.0)), A::sub(A::add_scaled_product(s.ad_value(175), (-2.0), A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(175), 10.0), s.ad_value(175), 1.0), A::mul3(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 8.0), s.ad_value(123), s.ad_value(175)), s.ad_value(175), s.ad_value(175))));
            let assign44680_ad_e76216: A = A::add(A::add_scaled_inputs4(s.ad_value(173), 1.0, A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(98), (-1.0), s.ad_value(63), -1.0)), 1.0, assign44680_ad_e76191, 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0), A::div(s.ad_value(178), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))));
            let assign44680_ad_e76258: A = A::add_scaled_inputs4(assign44680_ad_e76216, 1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0, A::div(s.ad_value(179), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), -1.0, A::div(s.ad_value(179), A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), -1.0);
            s.store_offset_sub_ad(54, A::add_scaled_product(assign44680_ad_e76146, 1.0, s.ad_value(296), assign44680_ad_e76258, (-1.0)), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), 2.0);
        }

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1832]) {
            s.store_ad_value(54, A::add_scaled_square_product(s.ad_value(65), 1.0, s.ad_value(54), s.ad_value(66), (-2.0)));
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
            s.store_ad_value(67, A::div_scaled_product_offset_denominator(s.ad_value(23), s.ad_value(23), 1.0, A::square(s.ad_value(23)), 2.0, 1.0));
            s.store_limited_exp_neg_input(68, 23);
            s.store_add_scaled_product(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67)), (-1.0));
            s.store_sub_ad_lhs(70, A::mul3(A::sub(s.ad_value(214), s.ad_value(23)), A::sub(s.ad_value(214), s.ad_value(23)), A::div_from_scalar(1.0, s.ad_value(296))), 69);
            s.store_offset_ad(70, A::add_scaled_inputs(A::offset(s.ad_value(70), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(70), (-0.001), A::offset(s.ad_value(70), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_sqrt(60, 70);
            s.store_mul_sqrt_ad_rhs(72, 294, A::add(s.ad_value(70), s.ad_value(69)));
            s.store_ad_value(73, A::div_scaled_product3(s.ad_value(296), s.ad_value(69), s.ad_value(269), 1.0, A::add_scaled_product(s.ad_value(72), 1.0, s.ad_value(294), s.ad_value(60), 1.0), 1.0));
            s.store_scaled_add(75, 22, 23, 0.5);
            s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));
            s.store_sqrt(76, 54);
            s.store_scaled_add(77, 57, 69, 0.5);
            s.store_add_scaled_product(78, s.ad_value(77), 1.0, A::square(s.ad_value(62)), A::sub_scaled_inputs(s.ad_value(76), 1.0, s.ad_value(297), 2.0), 0.125);
            s.store_sub_ad_lhs(79, A::mul3(A::sub(s.ad_value(214), s.ad_value(75)), A::sub(s.ad_value(214), s.ad_value(75)), A::div_from_scalar(1.0, s.ad_value(296))), 78);
            s.store_mul_sqrt_ad_rhs(51, 294, A::add(s.ad_value(78), s.ad_value(79)));
            s.store_offset_ad(79, A::add_scaled_inputs(A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(79), (-0.001), A::offset(s.ad_value(79), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_sqrt(71, 79);
        }

        s.b[1834] = (p.p46 == 1.0);
        s.v[1834] = if s.b[1834] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1834]) {
            s.store_div_scaled_inputs(85, s.ad_value(269), ((2.0 * s.v[199]) * s.v[199]), s.ad_value(704), (1.602176462e-19 * s.v[180]));
            s.store_ad_value(86, A::add_scaled_sub_value_product(1.0, s.ad_value(76), 1.0, s.ad_value(51), A::div_from_scalar(1.0, s.ad_value(296)), 2.0));
            s.store_div_from_scalar_sqrt_ad(87, 1.0, A::offset(A::mul(s.ad_value(85), s.ad_value(51)), 1.0));
            s.store_ad_value(54, A::div_scaled_value_offset_denominator(s.ad_value(87), 1.0, s.ad_value(87), 1.0, 1.0));
            s.store_mul_ad_product_rhs(88, 85, A::mul3(A::square(s.ad_value(54)), s.ad_value(51), s.ad_value(51)), A::div(s.ad_value(78), A::add(s.ad_value(78), s.ad_value(79))));
            s.store_ad_value(89, A::add_scaled_inputs_product(s.ad_value(51), 2.0, s.ad_value(88), (-2.0), s.ad_value(296), A::add(A::sub_from_scalar(1.0, s.ad_value(76)), s.ad_value(78)), 1.0));
            s.store_mul_ad_rhs(90, 88, A::sub_scaled_inputs(s.ad_value(88), 1.0, s.ad_value(51), 2.0));
            s.store_sub_from_scalar_ad(91, 1.0, A::mul_scaled_output(s.ad_value(296), A::add(s.ad_value(76), s.ad_value(78)), 0.5));
            s.store_div_scaled_product(92, s.ad_value(90), s.ad_value(89), 1.0, A::add_scaled_square_product(s.ad_value(89), 1.0, s.ad_value(91), s.ad_value(90), (-1.0)), 1.0);
            s.store_add(75, 75, 92);
            s.store_limited_exp(93, 92);
            s.store_div(76, 76, 93);
            s.store_mul(78, 78, 93);
            s.store_sub_ad(79, A::mul3(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(75), (-1.0), s.ad_value(92), 1.0), A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(75), (-1.0), s.ad_value(92), 1.0), A::div_from_scalar(1.0, s.ad_value(296))), A::div(s.ad_value(78), s.ad_value(93)));
            s.store_mul_sqrt_ad_rhs(51, 294, A::add(s.ad_value(78), s.ad_value(79)));
            s.store_add_ad(94, A::sub_from_scalar(1.0, s.ad_value(76)), A::mul3_scaled_output(s.ad_value(51), s.ad_value(87), s.ad_value(297), 2.0));
            s.store_ad_value(62, A::div_scaled_product3(s.ad_value(62), s.ad_value(93), A::add(s.ad_value(86), s.ad_value(77)), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(93), s.ad_value(77), 1.0), 1.0));
            s.store_mul(250, 62, 269);
            s.store_offset_ad(79, A::add_scaled_inputs(A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(79), (-0.001), A::offset(s.ad_value(79), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);
            s.store_sqrt(71, 79);
        }

        s.b[1835] = (((s.v[250]) as f64).abs() > 1e-35);
        s.v[1835] = if s.b[1835] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1799]) && (!s.b[1817])) && s.b[1835]) {
            s.store_div_scaled_inputs2(74, s.ad_value(306), 1.0, s.ad_value(73), (-1.0), s.ad_value(250), 1.0);
        }

        if ((s.b[1620] && s.b[1799]) && (!s.b[1817])) {
            s.store_mul_ad_rhs(80, 269, A::div_scaled_product(s.ad_value(296), s.ad_value(78), 1.0, A::add_scaled_product(s.ad_value(51), 1.0, s.ad_value(294), s.ad_value(71), 1.0), 1.0));
            s.store_add_scaled_product(81, s.ad_value(80), 1.0, s.ad_value(269), s.ad_value(74), 1.0);
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
        }

    }

    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1836] = (s.v[404] < 0.0);
        s.v[1836] = if s.b[1836] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1836]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if ((!s.b[1620]) && (!s.b[1836])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if (!s.b[1620]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(294), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(294), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(252), (-2.0), s.ad_value(225), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
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
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if ((!s.b[1620]) && s.b[1837]) {
            s.store_mul_ad_rhs(400, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if ((!s.b[1620]) && (!s.b[1837])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(400, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if (!s.b[1620]) {
            s.store_scaled_add_ad(256, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 256);
            s.store_sub_scaled_inputs(255, 254, 1.0, 400, 2.0);
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_offset_div_ad(253, s.ad_value(294), A::add(s.ad_value(259), A::sqrt(s.ad_value(167))), 1.0);
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
            s.store_mul_ad_rhs(167, 269, A::add_scaled_inputs_product(s.ad_value(213), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(247, 167, A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul3_affine_lhs(306, 253, 269, 2.0, 0.0, 400);
            s.store_mul_ad_rhs(308, 335, A::add_scaled_inputs(s.ad_value(247), 1.0, s.ad_value(306), s.v[338]));
            s.store_pow_ad(169, A::scaled_offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0, 0.5), s.ad_value(757));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(308), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(309, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
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
            s.store_add_ad_rhs(170, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)));
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
            s.store_scaled_add_ad(168, A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(178), A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179))), 0.5);
            s.store_ad_value(169, A::div_scaled_product_offset_denominator(s.ad_value(400), s.ad_value(168), (10.0 * p.p497), A::mul(s.ad_value(400), s.ad_value(168)), (10.0 * p.p497), 1.0));
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
            s.store_ad_value(179, A::div_scaled_product3(s.ad_value(178), s.ad_value(314), s.ad_value(456), 1.0, s.ad_value(269), 2.0));
            s.store_ad_value(167, A::div_scaled_product_offset_denominator(s.ad_value(314), A::add(A::square(s.ad_value(400)), s.ad_value(400)), 0.5, A::mul_scaled_lhs(s.ad_value(314), 0.5, A::offset(s.ad_value(400), 1.0)), 1.0, 1.0));
            s.store_mul_scaled_ad_rhs(168, 314, 2.0, A::sub(s.ad_value(400), s.ad_value(167)));
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1844] = (s.v[168] != 0.0);
        s.v[1844] = if s.b[1844] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1843]) && s.b[1844]) {
            s.store_asinh(323, 168);
            s.store_add_scaled_product(170, s.ad_value(169), 1.0, A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323), 1.0);
        }

        if (((!s.b[1620]) && s.b[1843]) && (!s.b[1844])) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_ad_value(171, A::add_scaled_value_products(A::mul3(s.ad_value(179), s.ad_value(167), A::offset(A::add(s.ad_value(400), s.ad_value(167)), 1.0)), 1.0, s.ad_value(167), s.ad_value(170), 1.0, s.ad_value(314), A::add_scaled_inputs4(A::square(s.ad_value(400)), 1.0, s.ad_value(400), 1.0, A::square(s.ad_value(167)), -1.0, s.ad_value(167), -1.0), (-1.0)));
        }

        s.b[1845] = (s.v[168] != 0.0);
        s.v[1845] = if s.b[1845] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1843]) && s.b[1845]) {
            s.store_div_scaled_product(172, s.ad_value(314), A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);
        }

        if (((!s.b[1620]) && s.b[1843]) && (!s.b[1845])) {
            s.store_mul_scaled_ad_rhs(172, 314, (-2.0), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_ad_value(173, A::add_scaled_value_products3(s.ad_value(170), 1.0, s.ad_value(167), s.ad_value(172), 1.0, s.ad_value(179), A::offset(A::add_scaled_inputs(s.ad_value(400), 1.0, s.ad_value(167), 2.0), 1.0), 1.0, s.ad_value(314), A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(167, 167, A::div(s.ad_value(171), s.ad_value(173)));
            s.store_mul_scaled_ad_rhs(168, 314, 2.0, A::sub(s.ad_value(400), s.ad_value(167)));
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1846] = (s.v[168] != 0.0);
        s.v[1846] = if s.b[1846] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1843]) && s.b[1846]) {
            s.store_asinh(323, 168);
            s.store_add_scaled_product(170, s.ad_value(169), 1.0, A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323), 1.0);
        }

        if (((!s.b[1620]) && s.b[1843]) && (!s.b[1846])) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_ad_value(171, A::add_scaled_value_products(A::mul3(s.ad_value(179), s.ad_value(167), A::offset(A::add(s.ad_value(400), s.ad_value(167)), 1.0)), 1.0, s.ad_value(167), s.ad_value(170), 1.0, s.ad_value(314), A::add_scaled_inputs4(A::square(s.ad_value(400)), 1.0, s.ad_value(400), 1.0, A::square(s.ad_value(167)), -1.0, s.ad_value(167), -1.0), (-1.0)));
        }

        s.b[1847] = (s.v[168] != 0.0);
        s.v[1847] = if s.b[1847] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1843]) && s.b[1847]) {
            s.store_div_scaled_product(172, s.ad_value(314), A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);
        }

        if (((!s.b[1620]) && s.b[1843]) && (!s.b[1847])) {
            s.store_mul_scaled_ad_rhs(172, 314, (-2.0), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!s.b[1620]) && s.b[1843]) {
            s.store_ad_value(173, A::add_scaled_value_products3(s.ad_value(170), 1.0, s.ad_value(167), s.ad_value(172), 1.0, s.ad_value(179), A::offset(A::add_scaled_inputs(s.ad_value(400), 1.0, s.ad_value(167), 2.0), 1.0), 1.0, s.ad_value(314), A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(307, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_ad_value(167, A::div_scaled_product_offset_denominator(s.ad_value(314), A::add(A::square(s.ad_value(400)), s.ad_value(400)), 0.5, A::mul_scaled_lhs(s.ad_value(314), 0.5, A::offset(s.ad_value(400), 1.0)), 1.0, 1.0));
            s.store_mul_scaled_ad_rhs(168, 314, 2.0, A::sub(s.ad_value(400), s.ad_value(167)));
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1848] = (s.v[168] != 0.0);
        s.v[1848] = if s.b[1848] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1848]) {
            s.store_asinh(323, 168);
            s.store_add_scaled_product(170, s.ad_value(169), 1.0, A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323), 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1848])) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_add_scaled_products(171, s.ad_value(167), s.ad_value(170), 1.0, s.ad_value(314), A::add_scaled_inputs4(A::square(s.ad_value(400)), 1.0, s.ad_value(400), 1.0, A::square(s.ad_value(167)), -1.0, s.ad_value(167), -1.0), (-1.0));
        }

        s.b[1849] = (s.v[168] != 0.0);
        s.v[1849] = if s.b[1849] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1849]) {
            s.store_div_scaled_product(172, s.ad_value(314), A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1849])) {
            s.store_mul_scaled_ad_rhs(172, 314, (-2.0), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_ad_value(173, A::add_scaled_value_products(s.ad_value(170), 1.0, s.ad_value(167), s.ad_value(172), 1.0, s.ad_value(314), A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(167, 167, A::div(s.ad_value(171), s.ad_value(173)));
            s.store_mul_scaled_ad_rhs(168, 314, 2.0, A::sub(s.ad_value(400), s.ad_value(167)));
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1850] = (s.v[168] != 0.0);
        s.v[1850] = if s.b[1850] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1850]) {
            s.store_asinh(323, 168);
            s.store_add_scaled_product(170, s.ad_value(169), 1.0, A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323), 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1850])) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_add_scaled_products(171, s.ad_value(167), s.ad_value(170), 1.0, s.ad_value(314), A::add_scaled_inputs4(A::square(s.ad_value(400)), 1.0, s.ad_value(400), 1.0, A::square(s.ad_value(167)), -1.0, s.ad_value(167), -1.0), (-1.0));
        }

        s.b[1851] = (s.v[168] != 0.0);
        s.v[1851] = if s.b[1851] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1843])) && s.b[1851]) {
            s.store_div_scaled_product(172, s.ad_value(314), A::add_scaled_product(s.ad_value(323), (-1.0), s.ad_value(168), s.ad_value(169), 1.0), (-2.0), A::square(s.ad_value(168)), 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1843])) && (!s.b[1851])) {
            s.store_mul_scaled_ad_rhs(172, 314, (-2.0), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!s.b[1620]) && (!s.b[1843])) {
            s.store_ad_value(173, A::add_scaled_value_products(s.ad_value(170), 1.0, s.ad_value(167), s.ad_value(172), 1.0, s.ad_value(314), A::scale_offset(s.ad_value(167), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(307, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if (!s.b[1620]) {
            let assign46440_ad_e78990: A = A::add_scaled_inputs4(s.ad_value(254), 1.0, s.ad_value(252), (-2.0), s.ad_value(307), (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::add(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(294), 1.0, s.ad_value(253), (-1.0), 1.0))), 1e-38)), -1.0);
            s.store_ad_value(319, assign46440_ad_e78990);
        }

        if (!s.b[1620]) {
            s.store_mul(312, 319, 269);
        }

        s.b[1852] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));
        s.v[1852] = if s.b[1852] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1852]) {
            s.store_scalar(1019, 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1852])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_ad(1019, A::div_scaled_inputs2(s.ad_value(168), p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(400), p.p1351), s.ad_value(269), p.p1350), (-1.0), A::scale_offset(s.ad_value(218), p.p1352, 1.0), 1.0), 1.0);
            s.store_scaled_add_ad(1019, A::offset(s.ad_value(1019), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(1019), (-0.1), A::offset(s.ad_value(1019), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        if (!s.b[1620]) {
            s.store_add_scaled_inputs3(316, s.ad_value(312), 0.5, s.ad_value(224), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(312), s.ad_value(224)), A::sub(s.ad_value(312), s.ad_value(224))), ((0.25 * 0.001) * 0.001))), 0.5);
            s.store_div(316, 316, 1019);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(316)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
        }

    }

    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 270);
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(294), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(294), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(252), (-2.0), s.ad_value(318), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
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
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if ((!s.b[1620]) && s.b[1853]) {
            s.store_mul_ad_rhs(320, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if ((!s.b[1620]) && (!s.b[1853])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(320, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if (!s.b[1620]) {
            s.store_add_scaled_inputs3_offset(255, s.ad_value(254), 1.0, s.ad_value(400), (-1.0), s.ad_value(320), -1.0, (-1.0));
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(169, 167);
            s.store_offset_div_ad(253, s.ad_value(294), A::add(s.ad_value(259), s.ad_value(169)), 1.0);
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
            s.store_div_from_scalar_add_ad(167, 1.0, A::offset(s.ad_value(400), 1.0), s.ad_value(320));
            s.store_mul(168, 417, 167);
            s.store_ad_value(381, A::add_scaled_inputs_product(s.ad_value(213), 1.0, s.ad_value(254), (-1.0), A::offset(s.ad_value(253), (-1.0)), A::add_scaled_inputs3(s.ad_value(400), 1.0, s.ad_value(320), 1.0, s.ad_value(168), 0.3333333333333333), (-1.0)));
            s.store_scale(169, 253, 0.3333333333333333);
            s.store_mul(170, 168, 167);
            s.store_mul_ad_rhs(382, 169, A::add_scaled_inputs_product(s.ad_value(400), 2.0, s.ad_value(320), 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(400), 0.8, 1.0), 1.0, s.ad_value(320), 1.2), s.ad_value(170), 0.5));
            s.store_mul_ad_rhs(385, 169, A::add_scaled_inputs_product(s.ad_value(400), 1.0, s.ad_value(320), 2.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(400), 1.2, 1.0), 1.0, s.ad_value(320), 0.8), s.ad_value(170), 0.5));
            s.store_add_scaled_product(244, A::sqrt(A::offset(A::mul3(s.ad_value(269), s.ad_value(381), A::mul(s.ad_value(269), s.ad_value(381))), ((0.25 * 0.1) * 0.1))), 0.5, s.ad_value(269), s.ad_value(381), 0.5);
            s.store_mul_add_rhs(243, 269, 382, 385);
            s.store_mul_ad_rhs(336, 335, A::add_scaled_inputs(s.ad_value(244), 1.0, s.ad_value(243), s.v[338]));
            s.store_pow_ad(169, A::scaled_offset(A::div(s.ad_value(243), s.ad_value(244)), 1.0, 0.5), s.ad_value(757));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(336), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(339, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_div_scaled_inputs(310, s.ad_value(746), 2.0, A::div(s.ad_value(740), s.ad_value(339)), 1.0);
            s.store_scale(311, 310, s.v[184]);
        }

        s.b[1856] = (s.v[781] > 0.0);
        s.v[1856] = if s.b[1856] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1856]) {
            s.store_offset_ad(360, A::div_scaled_product(s.ad_value(781), s.ad_value(243), 1.0, s.ad_value(311), 1.0), 1.0);
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
            s.store_div_ad_rhs(170, 362, A::add(s.ad_value(316), s.ad_value(362)));
            s.store_scaled_add_ad(171, A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(764), s.ad_value(218)), 1.0, A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5);
            s.store_div_from_scalar(172, 1.0, 171);
            s.store_mul_ad_lhs(361, A::mul3(A::div(s.ad_value(362), s.ad_value(359)), s.ad_value(170), s.ad_value(360)), 172);
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
            s.store_div_scaled_inputs(176, s.ad_value(769), ((s.v[184]) as f64).sqrt(), s.ad_value(362), 1.0);
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
            s.store_ad_value(168, A::div_scaled_value_by_product(s.ad_value(785), 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0)), s.ad_value(268), 1.0));
        }

        if (((!s.b[1620]) && s.b[1859]) && (!s.b[1860])) {
            s.store_ad_value(168, A::div_scaled_product_offset_rhs(s.ad_value(785), A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, s.ad_value(268), 1.0));
        }

        if ((!s.b[1620]) && s.b[1859]) {
            s.store_offset_mul_ad(364, s.ad_value(168), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(355), 1.0, s.ad_value(168), s.ad_value(358), 1.0), 1.0), 1e-38)), 1.0);
        }

        s.b[1861] = (p.p414 < 0.0);
        s.v[1861] = if s.b[1861] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1859])) && s.b[1861]) {
            s.store_ad_value(168, A::div_scaled_value_by_product(s.ad_value(785), 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0)), s.ad_value(268), 1.0));
        }

        if (((!s.b[1620]) && (!s.b[1859])) && (!s.b[1861])) {
            s.store_ad_value(168, A::div_scaled_product_offset_rhs(s.ad_value(785), A::div_scaled_inputs(s.ad_value(243), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, s.ad_value(268), 1.0));
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
            s.store_ad_value(356, A::div_scaled_offset_numerator(A::mul(s.ad_value(169), s.ad_value(168)), 1.0, 1.0, s.ad_value(767), 1.0));
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
            s.store_div_scaled_product(167, s.ad_value(765), s.ad_value(300), 1.0, s.ad_value(355), 1.0);
            s.store_div_scaled_inputs(357, A::limited_exp(s.ad_value(167)), s.v[184], s.ad_value(766), 1.0);
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
            s.store_scaled_add_ad(168, A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(178), A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179))), 0.5);
            s.store_ad_value(169, A::div_scaled_product_offset_denominator(s.ad_value(243), s.ad_value(168), (10.0 * p.p497), A::mul(s.ad_value(243), s.ad_value(168)), (10.0 * p.p497), 1.0));
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
            s.store_mul_scaled_ad_rhs(168, 314, 2.0, A::sub(s.ad_value(400), s.ad_value(320)));
            s.store_sqrt_square_offset(169, 168, 1.0);
        }

        s.b[1866] = (s.v[168] != 0.0);
        s.v[1866] = if s.b[1866] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1866]) {
            s.store_add_scaled_product(343, s.ad_value(169), 0.5, A::div_from_scalar(1.0, s.ad_value(168)), A::asinh(s.ad_value(168)), 0.5);
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
            s.store_add_scaled_product(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, s.ad_value(787), s.ad_value(202), 1.0);
            s.store_scaled_add_ad_rhs(171, 173, A::sqrt(A::offset(A::square(s.ad_value(173)), 0.01)), 0.5);
            s.store_mul_ad_rhs(454, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(773), 1.0, s.ad_value(775), s.ad_value(171), 1.0), s.ad_value(448), 1.0));
            s.store_sub(169, 204, 219);
            s.store_sqrt_square_offset(170, 169, 0.01);
            s.store_scaled_add(229, 169, 170, 0.5);
            s.store_offset_mul(172, 770, 229, 1.0);
            s.store_add_scaled_product(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, s.ad_value(787), s.ad_value(201), 1.0);
            s.store_scaled_add_ad_rhs(171, 173, A::sqrt(A::offset(A::square(s.ad_value(173)), 0.01)), 0.5);
            s.store_mul_ad_rhs(455, 652, A::add_scaled_product(s.ad_value(453), 1.0, A::add_scaled_product(s.ad_value(772), 1.0, s.ad_value(774), s.ad_value(171), 1.0), s.ad_value(448), 1.0));
        }

        if ((!s.b[1620]) && (!s.b[1867])) {
            s.store_offset_mul(167, 770, 243, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
        }

    }

    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (!s.b[1867])) {
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_scaled_add_ad_rhs(170, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(457, s.ad_value(652), A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), p.p2, 0.0, 448);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
            s.store_offset_ad(458, A::mul3_scaled_output(A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184]))), 1.0);
        }

        s.b[1868] = (p.p33 == 2.0);
        s.v[1868] = if s.b[1868] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1867])) && s.b[1868]) {
            s.store_mul_add_ad_rhs(457, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453));
            s.store_scalar(455, 0.0);
            s.store_scalar(454, 0.0);
            s.store_offset_ad(458, A::mul3_scaled_output(A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184]))), 1.0);
        }

        if (!s.b[1620]) {
            s.store_add_ad_rhs(167, 330, A::div(s.ad_value(333), A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(267), s.ad_value(637), 2.0)));
            s.store_sub(416, 400, 320);
            s.store_mul3_lhs(168, 167, 416, 416);
            s.store_offset(169, 168, ((1.0) + ((-0.001))));
            s.store_offset_ad(170, A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.004)), 0.5), (-1.0));
            s.store_scaled_offset_ad(334, A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0, 0.5);
            s.store_offset_ad(334, A::sub_scaled_inputs(A::offset(s.ad_value(334), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(334), (-1.0), A::offset(s.ad_value(334), (-1.0))), ((0.25 * 0.01) * 0.01))), 0.5), (0.25 * 0.01));
            s.store_add(167, 400, 320);
            s.store_sub(168, 400, 320);
            s.store_div_ad_rhs(169, 168, A::add(s.ad_value(167), s.ad_value(833)));
            s.store_mul3_lhs(170, 832, 169, 169);
            s.store_offset(834, 170, 1.0);
            s.store_div_ad_rhs(176, 858, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul3(s.ad_value(864), s.ad_value(168), s.ad_value(168)))), s.ad_value(167), 1.0, s.ad_value(267), s.ad_value(637), 2.0));
            s.store_limited_exp_neg_input(853, 176);
            s.store_mul3_lhs(340, 339, 343, 458);
            s.store_div(337, 740, 340);
        }

        if (!s.b[1620]) {
            let assign48130_ad_e81157: A = A::mul3(A::div_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(253), s.ad_value(337), s.ad_value(269), ((2.0 * p.p2) * ((s.v[183] * 1.0 / (s.v[184])) * s.v[199]))), s.ad_value(269), A::mul(A::sub(s.ad_value(400), s.ad_value(320)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)))), s.ad_value(363), 1.0, s.ad_value(334), 1.0), s.ad_value(834), s.ad_value(853));
            s.store_ad_value(380, assign48130_ad_e81157);
        }

        if (!s.b[1620]) {
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
            s.store_div_scaled_product(467, s.ad_value(465), s.ad_value(467), 1.0, s.ad_value(178), 1.0);
        }

        if (!s.b[1620]) {
            s.store_scalar(544, ((s.v[183] / p.p1373) + p.p1377));
            s.store_scalar(543, ((s.v[183] / p.p1373) + p.p1378));
            s.store_scale(545, 543, p.p74);
            s.store_scale(546, 544, p.p74);
            s.store_mul(593, 637, 590);
            s.store_div(167, 498, 593);
            s.store_limited_exp(595, 167);
            s.store_mul(594, 637, 590);
            s.store_div(167, 499, 594);
            s.store_limited_exp(596, 167);
            s.store_ad_value(171, A::mul_offset_rhs(A::div_from_scalar(1.115, s.ad_value(637)), s.ad_value(639), (-1.0)));
        }

        s.b[1872] = (s.v[550] == 0.0);
        s.v[1872] = if s.b[1872] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1872]) {
            s.store_scalar(535, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[1872])) {
            s.store_div_scaled_product(174, s.ad_value(547), s.ad_value(171), 1.0, s.ad_value(590), 1.0);
            s.store_limited_exp(168, 174);
            s.store_mul(548, 550, 168);
            s.store_mul(167, 545, 548);
            s.store_mul_offset_rhs(535, 167, 595, (-1.0));
        }

        s.b[1873] = (s.v[551] == 0.0);
        s.v[1873] = if s.b[1873] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1873]) {
            s.store_scalar(536, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[1873])) {
            s.store_div_scaled_product(174, s.ad_value(547), s.ad_value(171), 1.0, s.ad_value(590), 1.0);
            s.store_limited_exp(168, 174);
            s.store_mul(549, 551, 168);
            s.store_mul(167, 546, 549);
            s.store_mul_offset_rhs(536, 167, 596, (-1.0));
        }

        s.b[1874] = (s.v[552] == 0.0);
        s.v[1874] = if s.b[1874] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1874]) {
            s.store_scalar(537, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[1874])) {
            s.store_div_scaled_product(174, s.ad_value(556), s.ad_value(171), 1.0, s.ad_value(557), 1.0);
            s.store_limited_exp(169, 174);
            s.store_mul(554, 552, 169);
            s.store_mul_scaled_ad_rhs(562, 557, p.p925, A::offset(A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0));
            s.store_mul_scaled_ad_rhs(563, 564, p.p925, A::offset(A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0));
            s.store_div(167, 498, 562);
            s.store_limited_exp(177, 167);
        }

        s.b[1875] = ((s.v[558] - s.v[498]) < 0.001);
        s.v[1875] = if s.b[1875] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1874])) && s.b[1875]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(563), 1.0), s.ad_value(558), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if (((!s.b[1620]) && (!s.b[1874])) && (!s.b[1875])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(558), s.ad_value(498));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(563), 1.0), s.ad_value(558), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if ((!s.b[1620]) && (!s.b[1874])) {
            s.store_mul(170, 545, 554);
            s.store_mul_add_rhs(537, 170, 177, 178);
        }

        s.b[1876] = (s.v[553] == 0.0);
        s.v[1876] = if s.b[1876] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1876]) {
            s.store_scalar(538, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[1876])) {
            s.store_div_scaled_product(174, s.ad_value(556), s.ad_value(171), 1.0, s.ad_value(557), 1.0);
            s.store_limited_exp(169, 174);
            s.store_mul(555, 553, 169);
            s.store_mul_scaled_ad_rhs(562, 557, p.p925, A::offset(A::mul_offset_rhs(s.ad_value(565), s.ad_value(639), (-1.0)), 1.0));
            s.store_mul_scaled_ad_rhs(563, 564, p.p925, A::offset(A::mul_offset_rhs(s.ad_value(566), s.ad_value(639), (-1.0)), 1.0));
            s.store_div(167, 499, 562);
            s.store_limited_exp(177, 167);
        }

        s.b[1877] = ((s.v[559] - s.v[499]) < 0.001);
        s.v[1877] = if s.b[1877] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1876])) && s.b[1877]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(563), 1.0), s.ad_value(559), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if (((!s.b[1620]) && (!s.b[1876])) && (!s.b[1877])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(559), s.ad_value(499));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(563), 1.0), s.ad_value(559), 168);
            s.store_limited_exp(178, 167);
            s.store_neg(178, 178);
        }

        if ((!s.b[1620]) && (!s.b[1876])) {
            s.store_mul(170, 546, 555);
            s.store_mul_add_rhs(538, 170, 177, 178);
        }

        if (!s.b[1620]) {
            s.store_scalar(602, ((s.v[183] / p.p1373) * p.p74));
        }

        s.b[1878] = ((s.v[598] == 0.0) && (s.v[597] == 0.0));
        s.v[1878] = if s.b[1878] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1878]) {
            s.store_scalar(539, 0.0);
            s.store_scalar(540, 0.0);
            s.store_scalar(579, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[1878])) {
            s.store_div_scaled_product(174, s.ad_value(589), s.ad_value(171), 1.0, s.ad_value(590), 1.0);
            s.store_limited_exp(167, 174);
            s.store_mul(585, 587, 167);
            s.store_mul(578, 598, 167);
            s.store_div_scaled_product(174, s.ad_value(589), s.ad_value(171), 1.0, s.ad_value(590), 1.0);
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
        }

    }

    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (!s.b[1878])) {
            s.store_limited_exp(603, 167);
            s.store_sub_from_scalar(169, 1.0, 603);
            s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p.p595)));
            s.store_pow_ad(599, s.ad_value(167), s.ad_value(600));
            s.store_mul3_lhs(604, 602, 578, 599);
            s.store_mul(168, 167, 604);
            s.store_mul_ad_product_lhs(539, s.ad_value(168), A::offset(s.ad_value(595), (-1.0)), 591);
            s.store_mul3_lhs(604, 602, 577, 599);
            s.store_mul(168, 167, 604);
            s.store_mul_ad_product_lhs(540, s.ad_value(168), A::offset(s.ad_value(596), (-1.0)), 592);
            s.store_offset_scaled_ad(531, A::pow(s.ad_value(167), s.ad_value(530)), p.p920, 1.0);
            s.store_mul3_lhs(532, 602, 578, 531);
            s.store_mul_ad_product_lhs(533, s.ad_value(532), A::offset(s.ad_value(595), (-1.0)), 591);
            s.store_mul3_lhs(532, 602, 577, 531);
            s.store_mul_ad_product_lhs(534, s.ad_value(532), A::offset(s.ad_value(596), (-1.0)), 592);
            s.store_add_scaled_inputs(580, 581, 1.0, 582, s.v[184]);
        }

        s.b[1881] = (s.v[580] < 1.0);
        s.v[1881] = if s.b[1881] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1881]) {
            s.store_scalar(580, 1.0);
        }

        s.b[1882] = (p.p554 == 1.0);
        s.v[1882] = if s.b[1882] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1878])) && s.b[1882]) {
            s.store_scalar(579, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1882])) {
            s.store_offset_ad(167, A::div_scaled_inputs2(s.ad_value(498), 1.0, s.ad_value(499), 1.0, s.ad_value(580), 1.0), 1.0);
            s.store_add(168, 583, 584);
            s.store_sqrt_ad(170, A::add_scaled_inputs(A::square(s.ad_value(167)), 1.0, s.ad_value(168), 4.0));
            s.store_scaled_add(169, 167, 170, 0.5);
        }

        s.b[1883] = (s.v[169] < 0.1);
        s.v[1883] = if s.b[1883] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1878])) && (!s.b[1882])) && s.b[1883]) {
            s.store_scalar(605, 10.0);
        }

        if ((((!s.b[1620]) && (!s.b[1878])) && (!s.b[1882])) && (!s.b[1883])) {
            s.store_div_from_scalar(605, 1.0, 169);
        }

        if (((!s.b[1620]) && (!s.b[1878])) && (!s.b[1882])) {
            s.store_mul(167, 603, 604);
            s.store_mul_ad_affine_product_lhs(579, s.ad_value(167), A::sub(s.ad_value(595), s.ad_value(596)), p.p2, 0.0, 605);
        }

        s.b[1884] = ((s.v[567] == 0.0) && (s.v[568] == 0.0));
        s.v[1884] = if s.b[1884] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1884]) {
            s.store_scalar(541, 0.0);
            s.store_scalar(542, 0.0);
        }

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
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(594), 1.0), s.ad_value(575), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 571);
            s.store_mul_sub_from_scalar_rhs(541, 170, 1.0, 168);
        }

        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1885])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(575), s.ad_value(498));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(498), -1.0, s.ad_value(594), 1.0), s.ad_value(575), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 571);
            s.store_mul_sub_from_scalar_rhs(541, 170, 1.0, 168);
        }

        if ((!s.b[1620]) && (!s.b[1884])) {
            s.store_scale(594, 574, p.p925);
        }

        s.b[1886] = ((s.v[576] - s.v[499]) < 0.001);
        s.v[1886] = if s.b[1886] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1884])) && s.b[1886]) {
            s.store_scalar(168, 1000.0);
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(594), 1.0), s.ad_value(576), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 572);
            s.store_mul_sub_from_scalar_rhs(542, 170, 1.0, 168);
        }

        if (((!s.b[1620]) && (!s.b[1884])) && (!s.b[1886])) {
            s.store_div_from_scalar_sub_ad(168, 1.0, s.ad_value(576), s.ad_value(499));
            s.store_mul_ad_product_lhs(167, A::div_scaled_inputs(s.ad_value(499), -1.0, s.ad_value(594), 1.0), s.ad_value(576), 168);
            s.store_limited_exp(168, 167);
            s.store_mul(170, 545, 572);
            s.store_mul_sub_from_scalar_rhs(542, 170, 1.0, 168);
        }

        if (!s.b[1620]) {
            s.store_add_scaled_inputs4(496, s.ad_value(535), p.p2, s.ad_value(537), p.p2, s.ad_value(539), p.p2, s.ad_value(541), p.p2);
            s.store_add_scaled_inputs4(497, s.ad_value(536), p.p2, s.ad_value(538), p.p2, s.ad_value(540), p.p2, s.ad_value(542), p.p2);
            s.store_scalar(375, 0.0);
            s.store_scalar(374, 0.0);
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
            s.store_div_scaled_inputs3(168, s.ad_value(204), -1.0, s.ad_value(895), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_ad_rhs(168, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_ad_value(169, A::div_scaled_value_offset_denominator(s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0));
        }

        s.b[1889] = (s.v[894] != 0.0);
        s.v[1889] = if s.b[1889] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) && s.b[1889]) {
            s.store_mul_square_lhs(170, 201, 201);
            s.store_offset_add_ad(171, s.ad_value(894), A::abs(s.ad_value(170)), 0.0001);
            s.store_offset_ad(172, A::add_scaled_inputs(A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
        }

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) && (!s.b[1889])) {
            s.store_scalar(172, 1.0);
        }

        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1888])) {
            s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);
        }

        if ((!s.b[1620]) && s.b[1887]) {
            s.copy_ad(374, 173);
        }

        s.b[1890] = (((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) || (s.v[898] < 0.0));
        s.v[1890] = if s.b[1890] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1887]) && s.b[1890]) {
            s.store_scalar(173, 0.0);
        }

        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {
            s.store_div_scaled_inputs3(168, s.ad_value(203), -1.0, s.ad_value(899), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_ad_rhs(168, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_ad_value(169, A::div_scaled_value_offset_denominator(s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0));
        }

        s.b[1891] = (s.v[898] != 0.0);
        s.v[1891] = if s.b[1891] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) && s.b[1891]) {
            s.store_mul_square_lhs(170, 202, 202);
            s.store_offset_add_ad(171, s.ad_value(898), A::abs(s.ad_value(170)), 0.0001);
            s.store_offset_ad(172, A::add_scaled_inputs(A::div(s.ad_value(170), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
        }

        if ((((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) && (!s.b[1891])) {
            s.store_scalar(172, 1.0);
        }

        if (((!s.b[1620]) && s.b[1887]) && (!s.b[1890])) {
            s.store_mul_ad_product_lhs(173, A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), 172);
        }

        if ((!s.b[1620]) && s.b[1887]) {
            s.copy_ad(375, 173);
        }

        if ((!s.b[1620]) && (!s.b[1887])) {
            s.store_scalar(167, (s.v[200] * p.p76));
            s.store_add_scaled_product(207, s.ad_value(223), (-1.0), s.ad_value(905), s.ad_value(221), 1.0);
            s.store_add_scaled_product(206, s.ad_value(224), (-1.0), s.ad_value(902), s.ad_value(221), 1.0);
            s.store_sub(169, 203, 219);
            s.store_sqrt_square_offset(228, 169, 0.0001);
        }

        s.b[1892] = ((s.v[892] <= 0.0) || (s.v[660] <= 0.0));
        s.v[1892] = if s.b[1892] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1887])) && s.b[1892]) {
            s.store_scalar(173, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) {
            s.store_div_scaled_inputs3(168, s.ad_value(207), -1.0, s.ad_value(895), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_ad_rhs(168, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_ad_value(169, A::div_scaled_value_offset_denominator(s.ad_value(660), 1.0, s.ad_value(168), 0.001, 1.0));
        }

        s.b[1893] = (s.v[903] != 0.0);
        s.v[1893] = if s.b[1893] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) && s.b[1893]) {
            s.store_sub_scaled_inputs(170, 201, -1.0, 904, 1.0);
            s.store_offset(171, 170, 0.0001);
            s.store_offset_ad(172, A::add_scaled_inputs(A::div(s.ad_value(903), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(903), s.ad_value(171)), A::div(s.ad_value(903), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
        }

        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) && (!s.b[1893])) {
            s.store_scalar(172, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1892])) {
            s.store_ad_value(173, A::mul3(A::mul3(s.ad_value(892), s.ad_value(544), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172))));
        }

        if ((!s.b[1620]) && (!s.b[1887])) {
            s.copy_ad(374, 173);
        }

        s.b[1894] = ((s.v[896] <= 0.0) || (s.v[661] <= 0.0));
        s.v[1894] = if s.b[1894] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1887])) && s.b[1894]) {
            s.store_scalar(173, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) {
            s.store_div_scaled_inputs3(168, s.ad_value(206), -1.0, s.ad_value(899), (-1.0), s.ad_value(219), 1.0, s.ad_value(167), 1.0);
            s.store_scaled_add_ad_rhs(168, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_ad_value(169, A::div_scaled_value_offset_denominator(s.ad_value(661), 1.0, s.ad_value(168), 0.001, 1.0));
        }

        s.b[1895] = (s.v[906] != 0.0);
        s.v[1895] = if s.b[1895] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) && s.b[1895]) {
            s.store_sub_scaled_inputs(170, 202, -1.0, 907, 1.0);
            s.store_offset(171, 170, 0.0001);
            s.store_offset_ad(172, A::add_scaled_inputs(A::div(s.ad_value(906), s.ad_value(171)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(906), s.ad_value(171)), A::div(s.ad_value(906), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
        }

        if ((((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) && (!s.b[1895])) {
            s.store_scalar(172, 1.0);
        }

        if (((!s.b[1620]) && (!s.b[1887])) && (!s.b[1894])) {
            s.store_ad_value(173, A::mul3(A::mul3(s.ad_value(896), s.ad_value(543), s.ad_value(168)), A::limited_exp_scaled_input(s.ad_value(169), -1.0), A::limited_exp(s.ad_value(172))));
        }

        if ((!s.b[1620]) && (!s.b[1887])) {
            s.copy_ad(375, 173);
        }

        if (!s.b[1620]) {
            s.store_scaled_mul(1096, 379, 374, p.p2);
            s.store_scaled_mul(1097, 379, 375, p.p2);
        }

        s.b[1896] = (p.p44 == 0.0);
        s.v[1896] = if s.b[1896] { 1.0 } else { 0.0 };

        s.b[1897] = ((s.v[865] <= 0.0) || (s.v[659] <= 0.0));
        s.v[1897] = if s.b[1897] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1896]) && s.b[1897]) {
            s.store_scalar(373, 0.0);
        }

        s.b[1898] = (s.v[355] > (s.v[659] / 80.0));
        s.v[1898] = if s.b[1898] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1896]) && (!s.b[1897])) && s.b[1898]) {
            s.store_div_scaled_inputs(168, s.ad_value(659), -1.0, s.ad_value(355), 1.0);
            s.store_div_scaled_product(373, A::mul3(s.ad_value(865), s.ad_value(355), s.ad_value(380)), A::limited_exp(s.ad_value(168)), 1.0, s.ad_value(365), 1.0);
        }

        if ((((!s.b[1620]) && s.b[1896]) && (!s.b[1897])) && (!s.b[1898])) {
            s.store_ad_value(373, A::div_scaled_product3(s.ad_value(865), s.ad_value(355), s.ad_value(380), 1.804851387e-35, s.ad_value(365), 1.0));
        }

        s.b[1899] = (p.p44 == 1.0);
        s.v[1899] = if s.b[1899] { 1.0 } else { 0.0 };

        s.b[1900] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));
        s.v[1900] = if s.b[1900] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1896])) && s.b[1899]) && s.b[1900]) {
            s.store_scalar(373, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_37(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((!s.b[1620]) && (!s.b[1896])) && s.b[1899]) && (!s.b[1900])) {
            s.store_add_scaled_product(370, s.ad_value(869), (-1.0 / (s.v[184])), s.ad_value(874), A::scale_offset(s.ad_value(639), p.p600, (((((-1.0)) * (p.p600))) + (1.0))), 1.0);
            s.store_scale(167, 875, s.v[184]);
            s.store_ad_value(168, A::div_scaled_product_offset_denominator(s.ad_value(870), s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0));
            s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt(A::offset(A::mul(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269))), ((4.0 * p.p643) * p.p643))), 0.5), 1.0);
            s.store_add(170, 167, 872);
            s.store_scaled_add_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), A::sqrt(A::offset(A::mul(A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170))), ((4.0 * p.p644) * p.p644))), 0.5);
            s.store_div_from_scalar_offset_ad(170, 1.0, A::mul(s.ad_value(873), s.ad_value(227)), 1.0);
            s.store_mul3_lhs(368, 168, 169, 170);
            s.store_add(369, 370, 368);
            s.store_sub(371, 227, 369);
            s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));
            s.store_sqrt_square_offset(168, 167, 1e-10);
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && s.b[1899]) && (!s.b[1900])) {
            let assign50620_ad_e84223: A = A::add(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645))), A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645))))), (-((4.0 * (-10.0)) * p.p645)))));
            s.store_neg_ad(372, A::scale_offset(assign50620_ad_e84223, 0.5, (-10.0)));
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && s.b[1899]) && (!s.b[1900])) {
            s.store_mul_add_ad_rhs(373, 372, s.ad_value(380), A::mul3(s.ad_value(876), s.ad_value(211), s.ad_value(579)));
        }

        s.b[1901] = ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0)));
        s.v[1901] = if s.b[1901] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1901]) {
            s.store_scalar(373, 0.0);
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {
            s.store_add_scaled_product(370, s.ad_value(869), (-1.0 / (s.v[184])), s.ad_value(874), A::scale_offset(s.ad_value(639), p.p600, (((((-1.0)) * (p.p600))) + (1.0))), 1.0);
            s.store_scale(167, 875, s.v[184]);
            s.store_ad_value(168, A::div_scaled_product_offset_denominator(s.ad_value(870), s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0));
            s.store_div_from_scalar_offset_ad(167, 1.0, A::add_scaled_inputs(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), 0.5, A::sqrt(A::offset(A::mul(A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269)), A::mul3(s.ad_value(871), s.ad_value(367), s.ad_value(269))), ((4.0 * p.p643) * p.p643))), 0.5), 1.0);
            s.store_add(170, 167, 872);
            s.store_scaled_add_ad(169, A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), A::sqrt(A::offset(A::mul(A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170)), A::mul3(s.ad_value(367), s.ad_value(269), s.ad_value(170))), ((4.0 * p.p644) * p.p644))), 0.5);
            s.store_div_from_scalar_offset_ad(170, 1.0, A::mul(s.ad_value(873), s.ad_value(227)), 1.0);
            s.store_mul3_lhs(368, 168, 169, 170);
            s.store_add(369, 370, 368);
            s.store_sub(371, 227, 369);
            s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));
            s.store_sqrt_square_offset(168, 167, 1e-10);
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {
            let assign50780_ad_e84610: A = A::add(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645))), A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (((-(-10.0))) + ((-p.p645))))), (-((4.0 * (-10.0)) * p.p645)))));
            s.store_neg_ad(372, A::scale_offset(assign50780_ad_e84610, 0.5, (-10.0)));
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {
            s.store_mul(376, 372, 380);
        }

        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {
            s.store_add_scaled_inputs(167, 878, 1.0 / (s.v[184]), 877, (s.v[184] * 1.0 / (s.v[184])));
            s.store_mul_ad_rhs(378, 880, A::scale_offset(s.ad_value(639), p.p666, (((((-1.0)) * (p.p666))) + (1.0))));
        }

        s.b[1902] = (s.v[211] > 0.0);
        s.v[1902] = if s.b[1902] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1902]) {
            s.store_sub(168, 378, 499);
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1902])) {
            s.store_sub(168, 378, 498);
        }

        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {
            s.store_offset(169, 881, (-1.0));
        }

        s.b[1903] = (s.v[168] > 0.0);
        s.v[1903] = if s.b[1903] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1903]) {
            s.store_mul_scaled_ad_rhs(170, 879, -1.0, A::pow(s.ad_value(168), s.ad_value(169)));
        }

        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1903])) {
            s.store_scalar(170, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {
            s.store_limited_exp(171, 170);
            s.store_mul_ad_product_lhs(377, A::mul3(s.ad_value(167), s.ad_value(211), s.ad_value(579)), s.ad_value(168), 171);
            s.store_add(373, 376, 377);
        }

        if (!s.b[1620]) {
            s.store_mul(1095, 373, 379);
            s.store_ad_value(810, A::add_scaled_offset_product_rhs(s.ad_value(810), 1.0, s.ad_value(813), s.ad_value(639), (-1.0), 1.0));
            s.store_ad_value(816, A::add_scaled_offset_product_rhs(s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(639), (-1.0), 1.0));
            s.store_ad_value(819, A::add_scaled_offset_product_rhs(s.ad_value(819), 1.0, s.ad_value(815), s.ad_value(639), (-1.0), 1.0));
            s.store_ad_value(884, A::add_scaled_offset_product_rhs(s.ad_value(884), 1.0, s.ad_value(886), s.ad_value(639), (-1.0), 1.0));
            s.store_ad_value(882, A::add_scaled_offset_product_rhs(s.ad_value(882), 1.0, s.ad_value(887), s.ad_value(639), (-1.0), 1.0));
            s.store_ad_value(888, A::add_scaled_offset_product_rhs(s.ad_value(888), 1.0, s.ad_value(891), s.ad_value(639), (-1.0), 1.0));
            s.store_scalar(477, 0.0);
            s.store_scalar(479, 0.0);
            s.store_scalar(480, 0.0);
            s.store_scalar(483, 0.0);
            s.store_scalar(484, 0.0);
        }

        s.b[1904] = ((p.p37 != 0.0) || (p.p38 != 0.0));
        s.v[1904] = if s.b[1904] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1904]) {
            s.store_mul_ad_rhs(469, 269, A::add_scaled_inputs4(s.ad_value(213), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), 1.0, s.ad_value(320), 1.0));
            s.store_sqrt_square_offset(168, 469, 0.0001);
            s.store_scaled_sub(471, 168, 469, 0.5);
            s.store_scaled_add(470, 469, 168, 0.5);
        }

        s.b[1905] = (p.p38 != 0.0);
        s.v[1905] = if s.b[1905] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            s.store_scale(168, 469, 1.0 / (p.p671));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            let assign51110_ad_e85011: A = {
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
            };
            s.store_scale_ad(474, assign51110_ad_e85011, p.p671);
        }

        s.b[1906] = (p.p696 != 0.0);
        s.v[1906] = if s.b[1906] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1906]) {
            s.store_sub_from_scalar_ad(167, 1.0, A::scale(s.ad_value(471), 1.0 / (p.p696)));
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1906])) {
            s.store_scalar(167, 1.0);
        }

        s.b[1907] = (s.v[167] < 0.01);
        s.v[1907] = if s.b[1907] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1907]) {
            s.store_scalar(167, 0.01);
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));
            s.store_scalar(169, (p.p701 * p.p76));
            s.store_div_scaled_product(170, s.ad_value(169), A::add_scaled_product(s.ad_value(882), 1.0, s.ad_value(883), s.ad_value(471), (-1.0)), 1.0, s.ad_value(167), 1.0);
            s.store_limited_exp(171, 170);
            s.store_mul_ad_lhs(476, A::mul3(s.ad_value(168), s.ad_value(221), s.ad_value(474)), 171);
            s.store_mul(476, 476, 662);
            s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            let assign51240_ad_e85197: A = {
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
            };
            s.store_scale_ad(473, assign51240_ad_e85197, p.p671);
        }

        s.b[1908] = (p.p697 != 0.0);
        s.v[1908] = if s.b[1908] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1908]) {
            s.store_sub_from_scalar_ad(167, 1.0, A::scale(s.ad_value(470), 1.0 / (p.p697)));
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1908])) {
            s.store_scalar(167, 1.0);
        }

        s.b[1909] = (s.v[167] < 0.01);
        s.v[1909] = if s.b[1909] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1909]) {
            s.store_scalar(167, 0.01);
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));
            s.store_scalar(169, (p.p699 * p.p76));
            s.store_div_scaled_product(170, s.ad_value(169), A::add_scaled_product(s.ad_value(884), 1.0, s.ad_value(885), s.ad_value(470), (-1.0)), 1.0, s.ad_value(167), 1.0);
            s.store_limited_exp(171, 170);
            s.store_mul_ad_lhs(475, A::mul3(s.ad_value(168), s.ad_value(221), s.ad_value(473)), 171);
            s.store_mul(475, 475, 662);
            s.store_scaled_add(477, 476, 475, p.p2);
            s.store_offset_mul(478, 212, 269, p.p1383);
        }

        s.b[1910] = (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0));
        s.v[1910] = if s.b[1910] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {
            s.store_mul_voltage_ad(208, s.ad_value(379), ctx, nodes, Some(8), Some(11));
            s.store_sub(167, 208, 478);
            s.store_sqrt_square_offset(168, 167, 0.0001);
            s.store_offset_scaled_sub(209, 168, 167, 0.5, (((-0.01)) * (0.5)));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {
            s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {
            s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {
            s.store_mul(169, 208, 209);
            s.store_add_scaled_product(170, s.ad_value(889), (-1.0), s.ad_value(888), s.ad_value(890), 1.0);
            s.store_mul(171, 889, 890);
            s.store_mul_scaled_ad_rhs(172, 179, (-p.p76), A::sub(A::add_scaled_product(s.ad_value(888), 1.0, s.ad_value(170), s.ad_value(209), 1.0), A::mul3(s.ad_value(171), s.ad_value(209), s.ad_value(209))));
            s.store_limited_exp(173, 172);
            s.store_scaled_mul(178, 178, 492, p.p1380);
            s.store_mul_ad_lhs(210, A::mul3(s.ad_value(178), s.ad_value(169), s.ad_value(173)), 662);
        }

        if (((!s.b[1620]) && s.b[1904]) && (!s.b[1910])) {
            s.store_scalar(210, 0.0);
        }

        s.b[1911] = (p.p37 != 0.0);
        s.v[1911] = if s.b[1911] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {
            s.store_add_scaled_product(168, s.ad_value(810), 1.0, s.ad_value(811), s.ad_value(470), (-1.0));
            s.store_offset_mul(169, 812, 470, 1.0);
            s.store_scaled_mul(170, 168, 169, s.v[488]);
            s.store_mul_ad(171, A::mul3(s.ad_value(253), s.ad_value(269), A::add(s.ad_value(400), s.ad_value(320))), A::limited_exp(s.ad_value(170)));
            s.store_mul_ad_lhs(481, A::mul3_scaled_output(s.ad_value(487), s.ad_value(171), A::add_scaled_inputs4(s.ad_value(221), 1.0, s.ad_value(227), 0.5, s.ad_value(224), (-0.5), s.ad_value(223), (-0.5)), p.p2), 662);
            s.store_offset_sqrt_ad(472, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));
            s.store_scale(168, 472, s.v[823]);
            s.store_limited_exp_neg_input(482, 168);
            s.store_offset_add(170, 168, 482, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(171, 1.0, A::mul_offset_lhs(s.ad_value(168), 1.0, s.ad_value(482)), 0.0001);
            s.store_offset_square(172, 168, 0.0002);
        }

        s.b[1912] = (s.v[211] > 0.0);
        s.v[1912] = if s.b[1912] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1912]) {
            s.store_div_scaled_product(480, s.ad_value(481), s.ad_value(171), 1.0, s.ad_value(172), 1.0);
            s.store_div_scaled_product(479, s.ad_value(481), s.ad_value(170), 1.0, s.ad_value(172), 1.0);
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1912])) {
            s.store_div_scaled_product(479, s.ad_value(481), s.ad_value(171), 1.0, s.ad_value(172), 1.0);
            s.store_div_scaled_product(480, s.ad_value(481), s.ad_value(170), 1.0, s.ad_value(172), 1.0);
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {
            s.store_sub(169, 203, 219);
        }

    }

    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {
            s.store_sqrt_square_offset(228, 169, 0.0001);
        }

        s.b[1913] = (p.p1295 == 1.0);
        s.v[1913] = if s.b[1913] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) {
            s.store_scaled_add_ad(168, A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1914] = (s.v[818] < 0.01);
        s.v[1914] = if s.b[1914] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) && s.b[1914]) {
            s.store_scalar(818, 0.01);
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1913])) {
            s.store_add_scaled_product(168, s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {
            s.store_offset_mul(169, 818, 228, 1.0);
            s.store_mul3_lhs(170, 491, 168, 169);
            s.store_limited_exp(171, 170);
            s.store_mul3_affine_lhs(485, 662, 489, p.p2, 0.0, 824);
            s.store_mul_ad_lhs(483, A::mul3(s.ad_value(485), s.ad_value(203), s.ad_value(228)), 171);
            s.store_sub(169, 204, 219);
            s.store_sqrt_square_offset(229, 169, 0.0001);
        }

        s.b[1915] = (p.p1295 == 1.0);
        s.v[1915] = if s.b[1915] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) {
            s.store_scaled_add_ad(168, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1916] = (s.v[821] < 0.01);
        s.v[1916] = if s.b[1916] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) && s.b[1916]) {
            s.store_scalar(821, 0.01);
        }

        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1915])) {
            s.store_add_scaled_product(168, s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0));
        }

        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {
            s.store_offset_mul(169, 821, 229, 1.0);
            s.store_mul3_lhs(170, 491, 168, 169);
            s.store_limited_exp(171, 170);
            s.store_mul3_affine_lhs(486, 662, 490, p.p2, 0.0, 825);
            s.store_mul_ad_lhs(484, A::mul3(s.ad_value(486), s.ad_value(204), s.ad_value(229)), 171);
        }

        if (!s.b[1620]) {
            s.store_mul(1098, 379, 483);
            s.store_mul(1099, 379, 484);
            s.store_mul(1102, 379, 477);
            s.store_mul(1100, 379, 479);
            s.store_mul(1101, 379, 480);
            s.store_mul(502, 666, 463);
            s.store_mul(505, 667, 494);
            s.store_scale(508, 671, (s.v[189] * p.p2));
            s.store_scalar(503, ((0.1) as f64).powf((-p.p913)));
        }

        s.b[1917] = (p.p913 == 1.0);
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1917]) {
            s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1917])) {
            s.store_offset_scaled_ad(504, A::scale(s.ad_value(503), ((0.05 * p.p913) * (1.0 + p.p913))), (-(1.0 / (1.0 - p.p913))), (1.0 / (1.0 - p.p913)));
        }

        if (!s.b[1620]) {
            s.store_scalar(506, ((0.1) as f64).powf((-p.p915)));
        }

        s.b[1918] = (p.p915 == 1.0);
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1918]) {
            s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1918])) {
            s.store_offset_scaled_ad(507, A::scale(s.ad_value(506), ((0.05 * p.p915) * (1.0 + p.p915))), (-(1.0 / (1.0 - p.p915))), (1.0 / (1.0 - p.p915)));
        }

        if (!s.b[1620]) {
            s.store_scalar(509, ((0.1) as f64).powf((-p.p917)));
        }

        s.b[1919] = (p.p917 == 1.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1919]) {
            s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1919])) {
            s.store_offset_scaled_ad(510, A::scale(s.ad_value(509), ((0.05 * p.p917) * (1.0 + p.p917))), (-(1.0 / (1.0 - p.p917))), (1.0 / (1.0 - p.p917)));
        }

        s.b[1920] = (s.v[502] > 0.0);
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1920]) {
            s.store_div(168, 498, 672);
        }

        s.b[1921] = (s.v[168] < 0.9);
        s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1920]) && s.b[1921]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1922] = (p.p913 != 1.0);
        s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };

        s.b[1923] = (p.p913 == 0.5);
        s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && s.b[1923]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) {
            s.store_ad_value(501, A::limited_exp_scaled_input(A::ln(s.ad_value(500)), (-p.p913)));
        }

        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) {
            s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p913)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && (!s.b[1922])) {
            s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1920]) && (!s.b[1921])) {
            s.store_mul_ad_product_rhs(169, 503, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p913), (((((-1.0)) * ((5.0 * p.p913)))) + ((1.0 + p.p913)))));
            s.store_mul_ad_product_rhs(521, 672, s.ad_value(502), A::add(s.ad_value(169), s.ad_value(504)));
        }

        if ((!s.b[1620]) && (!s.b[1920])) {
            s.store_scalar(521, 0.0);
        }

        s.b[1924] = (s.v[505] > 0.0);
        s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1924]) {
            s.store_div(168, 498, 673);
        }

        s.b[1925] = (s.v[168] < 0.9);
        s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1924]) && s.b[1925]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1926] = (p.p915 != 1.0);
        s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };

        s.b[1927] = (p.p915 == 0.5);
        s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && s.b[1927]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) {
            s.store_ad_value(501, A::limited_exp_scaled_input(A::ln(s.ad_value(500)), (-p.p915)));
        }

        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) {
            s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p915)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && (!s.b[1926])) {
            s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1924]) && (!s.b[1925])) {
            s.store_mul_ad_product_rhs(169, 506, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p915), (((((-1.0)) * ((5.0 * p.p915)))) + ((1.0 + p.p915)))));
            s.store_mul_ad_product_rhs(522, 673, s.ad_value(505), A::add(s.ad_value(169), s.ad_value(507)));
        }

        if ((!s.b[1620]) && (!s.b[1924])) {
            s.store_scalar(522, 0.0);
        }

        s.b[1928] = (s.v[508] > 0.0);
        s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1928]) {
            s.store_div(168, 498, 674);
        }

        s.b[1929] = (s.v[168] < 0.9);
        s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1928]) && s.b[1929]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1930] = (p.p917 != 1.0);
        s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };

        s.b[1931] = (p.p917 == 0.5);
        s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && s.b[1931]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) {
            s.store_ad_value(501, A::limited_exp_scaled_input(A::ln(s.ad_value(500)), (-p.p917)));
        }

        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) {
            s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p917)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && (!s.b[1930])) {
            s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1928]) && (!s.b[1929])) {
            s.store_mul_ad_product_rhs(169, 509, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p917), (((((-1.0)) * ((5.0 * p.p917)))) + ((1.0 + p.p917)))));
            s.store_mul_ad_product_rhs(523, 674, s.ad_value(508), A::add(s.ad_value(169), s.ad_value(510)));
        }

        if ((!s.b[1620]) && (!s.b[1928])) {
            s.store_scalar(523, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scale(524, 533, (p.p919 * p.p2));
            s.store_add_scaled_inputs4(520, s.ad_value(521), 1.0, s.ad_value(522), 1.0, s.ad_value(523), 1.0, s.ad_value(524), 1.0);
            s.store_mul(511, 669, 464);
            s.store_mul(514, 670, 495);
            s.store_scale(517, 668, (s.v[189] * p.p2));
            s.store_scalar(512, ((0.1) as f64).powf((-p.p914)));
        }

        s.b[1932] = (p.p914 == 1.0);
        s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1932]) {
            s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1932])) {
            s.store_offset_scaled_ad(513, A::scale(s.ad_value(512), ((0.05 * p.p914) * (1.0 + p.p914))), (-(1.0 / (1.0 - p.p914))), (1.0 / (1.0 - p.p914)));
        }

        if (!s.b[1620]) {
            s.store_scalar(515, ((0.1) as f64).powf((-p.p916)));
        }

        s.b[1933] = (p.p916 == 1.0);
        s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1933]) {
            s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1933])) {
            s.store_offset_scaled_ad(516, A::scale(s.ad_value(515), ((0.05 * p.p916) * (1.0 + p.p916))), (-(1.0 / (1.0 - p.p916))), (1.0 / (1.0 - p.p916)));
        }

        if (!s.b[1620]) {
            s.store_scalar(518, ((0.1) as f64).powf((-p.p918)));
        }

        s.b[1934] = (p.p918 == 1.0);
        s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1934]) {
            s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!s.b[1620]) && (!s.b[1934])) {
            s.store_offset_scaled_ad(519, A::scale(s.ad_value(518), ((0.05 * p.p918) * (1.0 + p.p918))), (-(1.0 / (1.0 - p.p918))), (1.0 / (1.0 - p.p918)));
        }

        s.b[1935] = (s.v[511] > 0.0);
        s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1935]) {
            s.store_div(168, 499, 675);
        }

        s.b[1936] = (s.v[168] < 0.9);
        s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1935]) && s.b[1936]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1937] = (p.p914 != 1.0);
        s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };

        s.b[1938] = (p.p914 == 0.5);
        s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && s.b[1938]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) {
            s.store_ad_value(501, A::limited_exp_scaled_input(A::ln(s.ad_value(500)), (-p.p914)));
        }

        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) {
            s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p914)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && (!s.b[1937])) {
            s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1935]) && (!s.b[1936])) {
            s.store_mul_ad_product_rhs(169, 512, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p914), (((((-1.0)) * ((5.0 * p.p914)))) + ((1.0 + p.p914)))));
            s.store_mul_ad_product_rhs(526, 675, s.ad_value(511), A::add(s.ad_value(169), s.ad_value(513)));
        }

        if ((!s.b[1620]) && (!s.b[1935])) {
            s.store_scalar(526, 0.0);
        }

        s.b[1939] = (s.v[514] > 0.0);
        s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1939]) {
            s.store_div(168, 499, 676);
        }

        s.b[1940] = (s.v[168] < 0.9);
        s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1939]) && s.b[1940]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1941] = (p.p916 != 1.0);
        s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };

        s.b[1942] = (p.p916 == 0.5);
        s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && s.b[1942]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) {
            s.store_ad_value(501, A::limited_exp_scaled_input(A::ln(s.ad_value(500)), (-p.p916)));
        }

        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) {
            s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p916)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && (!s.b[1941])) {
            s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1939]) && (!s.b[1940])) {
            s.store_mul_ad_product_rhs(169, 515, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p916), (((((-1.0)) * ((5.0 * p.p916)))) + ((1.0 + p.p916)))));
            s.store_mul_ad_product_rhs(527, 676, s.ad_value(514), A::add(s.ad_value(169), s.ad_value(516)));
        }

        if ((!s.b[1620]) && (!s.b[1939])) {
            s.store_scalar(527, 0.0);
        }

        s.b[1943] = (s.v[517] > 0.0);
        s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1943]) {
            s.store_div(168, 499, 677);
        }

        s.b[1944] = (s.v[168] < 0.9);
        s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1943]) && s.b[1944]) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.b[1945] = (p.p918 != 1.0);
        s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };

        s.b[1946] = (p.p918 == 0.5);
        s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && s.b[1946]) {
            s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));
        }

        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) {
            s.store_ad_value(501, A::limited_exp_scaled_input(A::ln(s.ad_value(500)), (-p.p918)));
        }

        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) {
            s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p918)), 0.0);
        }

        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && (!s.b[1945])) {
            s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::ln(s.ad_value(500)), -1.0, 0.0);
        }

        if (((!s.b[1620]) && s.b[1943]) && (!s.b[1944])) {
            s.store_mul_ad_product_rhs(169, 518, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p918), (((((-1.0)) * ((5.0 * p.p918)))) + ((1.0 + p.p918)))));
        }

    }

    pub(super) fn stamp_transient_block_39(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[1620]) && s.b[1943]) && (!s.b[1944])) {
            s.store_mul_ad_product_rhs(528, 677, s.ad_value(517), A::add(s.ad_value(169), s.ad_value(519)));
        }

        if ((!s.b[1620]) && (!s.b[1943])) {
            s.store_scalar(528, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scale(529, 534, (p.p919 * p.p2));
            s.store_add_scaled_inputs4(525, s.ad_value(526), 1.0, s.ad_value(527), 1.0, s.ad_value(528), 1.0, s.ad_value(529), 1.0);
        }

        s.b[1947] = (p.p28 != 0.0);
        s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1947]) {
            s.store_powf_ad(168, A::scale(s.ad_value(706), 1.0000000000000001e-23), p.p1144);
            s.store_powf_ad(169, A::div_from_scalar(300.0, s.ad_value(635)), p.p1145);
            s.store_div_scaled_product(170, s.ad_value(379), A::voltage(ctx, nodes, Some(10), Some(7)), p.p1143, s.ad_value(271), 1.0);
            s.store_scaled_limited_exp_ad(975, A::mul_scaled_lhs(s.ad_value(168), -1.0, s.ad_value(169)), p.p1138);
            s.store_scaled_mul(976, 169, 168, p.p1139);
            s.store_scale_ad(977, A::tanh(A::limited_exp(A::mul_scaled_lhs(s.ad_value(379), p.p1142, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(8), Some(10)), 1.0, s.ad_value(1128), (-1.0), A::voltage(ctx, nodes, Some(7), Some(10)), -1.0)))), p.p1141);
            s.store_ad_value(974, A::mul_offset_rhs(A::mul3(A::mul3_scaled_output(s.ad_value(211), s.ad_value(975), A::limited_exp(s.ad_value(170)), (p.p2 * s.v[183])), A::limited_exp_scaled_input(s.ad_value(976), (-s.v[184])), A::limited_exp(A::div(s.ad_value(977), s.ad_value(271)))), A::limited_exp(A::div_scaled_inputs(s.ad_value(227), p.p1140, s.ad_value(271), 1.0)), (-1.0)));
        }

        if (!s.b[1620]) {
            s.store_scale(621, 271, (4.0 * 1.602176462e-19));
            s.store_div_scaled_inputs(607, s.ad_value(746), 2.0, s.ad_value(337), 1.0);
        }

        s.b[1948] = (p.p1011 <= 0.0);
        s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1948]) {
            s.store_scalar(610, 0.0);
        }

        if ((!s.b[1620]) && (!s.b[1948])) {
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div(s.ad_value(355), s.ad_value(300)), 1.0, p.p1011, s.ad_value(607), 1.0));
            s.store_mul_ln_ad_rhs(610, 300, A::max_with_scalar(s.ad_value(167), 1e-38));
        }

        s.b[1949] = (s.v[610] < 0.0);
        s.v[1949] = if s.b[1949] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1948])) && s.b[1949]) {
            s.store_scalar(610, 0.0);
        }

        if (!s.b[1620]) {
            s.store_mul_scaled_ad_rhs(613, 271, 1.0 / (1.602176462e-19), A::add(A::offset(s.ad_value(260), s.v[199]), s.ad_value(709)));
            s.store_mul_ad_affine_product_lhs(612, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(320), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);
            s.store_mul_ad_affine_product_lhs(1004, s.ad_value(271), A::abs(s.ad_value(380)), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19), 0.0, 337);
            s.store_mul3_affine_lhs(1005, 271, 380, 1.602176462e-19, 0.0, 380);
            s.store_add_scaled_product(1006, A::scale_offset(s.ad_value(612), p.p1013, p.p1012), 1.0, s.ad_value(612), s.ad_value(612), p.p1014);
            s.store_mul_ad(1007, A::add(s.ad_value(612), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613)));
            s.store_scale(1008, 271, (p.p1012 * 1.602176462e-19));
        }

        s.b[1950] = (p.p1319 == 1.0);
        s.v[1950] = if s.b[1950] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scalar(1014, p.p1320);
        }

        s.b[1951] = (s.v[184] > s.v[1014]);
        s.v[1951] = if s.b[1951] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1951]) {
            s.store_sub_from_scalar(167, s.v[184], 1014);
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1951])) {
            s.store_scalar(1014, s.v[184]);
            s.copy_ad(167, 1014);
        }

        s.b[1952] = (p.p1015 >= (s.v[167] / 2.0));
        s.v[1952] = if s.b[1952] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1952]) {
            s.store_scalar(606, 0.0);
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1952])) {
            s.store_scalar(606, p.p1015);
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scalar(1013, s.v[184]);
            s.store_div_scaled_inputs2(980, s.ad_value(221), 1.0, s.ad_value(707), (-1.0), s.ad_value(271), 1.0);
            s.store_scaled_sqrt_ad(981, A::div_from_scalar((((2.0 * 1.602176462e-19) * s.v[180]) * p.p1322), s.ad_value(271)), 1.0 / (s.v[199]));
            s.store_ln_ad(982, A::div_from_scalar(p.p1322, s.ad_value(182)));
            s.store_scalar(168, 1.0);
            s.store_div(404, 980, 168);
            s.store_div(405, 981, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1953] = (s.v[404] < 0.0);
        s.v[1953] = if s.b[1953] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1953]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(983, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1953])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_ad_lhs(983, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(983), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(983), (-1.0), A::offset(s.ad_value(983), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(981), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(981), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(983), 1.0, s.ad_value(982), (-2.0), s.ad_value(225), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1954] = (s.v[175] <= (-68.0));
        s.v[1954] = if s.b[1954] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1954]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1955] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1955] = if s.b[1955] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && s.b[1955]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1956] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1956] = if s.b[1956] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && (!s.b[1955])) && s.b[1956]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && (!s.b[1955])) && (!s.b[1956])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1950]) && s.b[1954]) {
            s.store_mul_ad_rhs(985, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1954])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(985, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scaled_add_ad(984, A::offset(s.ad_value(983), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(983), (-1.0), A::offset(s.ad_value(983), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_offset_ad(986, A::div_scaled_inputs(s.ad_value(981), 1.0, A::sqrt(s.ad_value(984)), 2.0), 1.0);
            s.copy_ad(987, 337);
            s.store_scale(994, 987, (s.v[199] * s.v[183]));
            s.store_scale(993, 337, (s.v[199] * s.v[183]));
            s.store_ad_value(988, A::div_scaled_product_by_product(s.ad_value(380), s.ad_value(1014), 1.0, A::mul3_scaled_output(s.ad_value(986), s.ad_value(994), s.ad_value(271), 2.0), s.ad_value(271), 1.0));
            s.store_ad_value(990, A::div_scaled_product_by_product(s.ad_value(380), A::sub(s.ad_value(1013), s.ad_value(1014)), 1.0, A::mul3_scaled_output(s.ad_value(253), s.ad_value(993), s.ad_value(269), 2.0), s.ad_value(269), 1.0));
            s.store_add_scaled_inputs3_offset(167, A::square(s.ad_value(985)), 4.0, s.ad_value(985), 4.0, s.ad_value(988), (-4.0), 1.0);
        }

        s.b[1957] = (s.v[167] < 1.0);
        s.v[1957] = if s.b[1957] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1957]) {
            s.store_scalar(989, 0.0);
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1957])) {
            s.store_offset_scaled_ad(989, A::sqrt(s.ad_value(167)), 0.5, (-0.5));
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_offset_scaled_ad(991, A::sqrt(A::offset(A::add_scaled_inputs3(A::square(s.ad_value(320)), 4.0, s.ad_value(320), 4.0, s.ad_value(990), 4.0), 1.0)), 0.5, (-0.5));
            s.store_mul_ad_lhs(995, A::mul3_scaled_output(s.ad_value(986), s.ad_value(994), s.ad_value(271), 2.0), 989);
            s.store_mul_ad_lhs(996, A::mul3_scaled_output(s.ad_value(253), s.ad_value(993), s.ad_value(271), 2.0), 320);
            s.store_mul_ad_affine_product_rhs(997, 993, s.ad_value(271), A::sub(s.ad_value(991), s.ad_value(320)), 2.0, 0.0);
            s.store_mul_sub_rhs(998, 995, 1013, 1014);
            s.store_add_scaled_products(999, s.ad_value(997), s.ad_value(1014), 1.0, s.ad_value(996), s.ad_value(1014), 1.0);
            s.store_ad_value(1010, A::div_scalar_by_product(1.0, A::add(s.ad_value(998), s.ad_value(999)), A::add(s.ad_value(998), s.ad_value(999)), 1.0));
            s.store_mul_square_lhs(1011, 998, 1010);
            s.store_mul_square_lhs(1012, 999, 1010);
        }

        s.b[1958] = (s.v[184] != s.v[1014]);
        s.v[1958] = if s.b[1958] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1958]) {
            s.store_mul3_affine_lhs(992, 253, 271, ((2.0 * s.v[199]) * 6.241509744511525e18), 0.0, 991);
            s.store_add_scaled_inputs3(608, s.ad_value(1013), 1.0, s.ad_value(606), (-2.0), s.ad_value(1014), -1.0);
            s.store_square(609, 608);
            s.store_scale(168, 609, (10000000000.0 * s.v[199]));
            s.store_scaled_ln_ad(169, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(992), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p.p1012);
            s.store_scaled_sub(170, 992, 612, p.p1013);
            s.store_scaled_sub_ad(171, A::square(s.ad_value(992)), A::square(s.ad_value(612)), (0.5 * p.p1014));
            s.store_scale(172, 609, (10000000000.0 * (s.v[183] * p.p2)));
            s.store_add_scaled_product(1000, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(172), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(168)), A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(170), 1.0, s.ad_value(171), 1.0), 1.0);
            s.store_mul3_affine_lhs(173, 608, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(1001, A::div(s.ad_value(1008), s.ad_value(173)), s.ad_value(380), 380);
            s.store_add(174, 1001, 1000);
        }

        s.b[1959] = (s.v[174] > 0.0);
        s.v[1959] = if s.b[1959] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1950]) && s.b[1958]) && s.b[1959]) {
            s.store_div_scaled_product(1002, s.ad_value(1000), s.ad_value(1001), 1.0, s.ad_value(174), 1.0);
        }

        if ((((!s.b[1620]) && s.b[1950]) && s.b[1958]) && (!s.b[1959])) {
            s.store_scalar(1002, 0.0);
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1958])) {
            s.store_scalar(1002, 0.0);
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_scale(175, 271, (p.p1321 * 1.602176462e-19));
            s.store_mul3_affine_lhs(176, 1014, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(1009, A::div(s.ad_value(175), s.ad_value(176)), s.ad_value(380), 380);
            s.copy_ad(177, 1009);
        }

        s.b[1960] = (s.v[177] > 0.0);
        s.v[1960] = if s.b[1960] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1950]) && s.b[1960]) {
            s.copy_ad(1003, 1009);
        }

        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1960])) {
            s.store_scalar(1003, 0.0);
        }

        if ((!s.b[1620]) && s.b[1950]) {
            s.store_add_scaled_products(616, s.ad_value(1002), s.ad_value(1011), 1.0, s.ad_value(1003), s.ad_value(1012), 1.0);
        }

        s.b[1961] = (p.p1015 >= (s.v[184] / 2.0));
        s.v[1961] = if s.b[1961] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1961]) {
            s.store_scalar(606, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1950])) && (!s.b[1961])) {
            s.store_scalar(606, p.p1015);
        }

        s.b[1962] = (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0));
        s.v[1962] = if s.b[1962] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1962]) {
            s.store_sub_from_scalar_ad(608, s.v[184], A::scale(s.ad_value(606), 2.0));
            s.store_square(609, 608);
            s.store_scale(167, 609, (10000000000.0 * s.v[199]));
        }

    }

    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1962]) {
            s.store_mul_ad_affine_product_lhs(611, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(400), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);
            s.store_scaled_ln_ad(168, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(611), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p.p1012);
            s.store_scaled_sub(169, 611, 612, p.p1013);
            s.store_scaled_sub_ad(170, A::square(s.ad_value(611)), A::square(s.ad_value(612)), (0.5 * p.p1014));
            s.store_scale(171, 609, (10000000000.0 * (s.v[183] * p.p2)));
            s.store_add_scaled_product(614, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(171), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(167)), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, s.ad_value(170), 1.0), 1.0);
            s.store_mul3_affine_lhs(172, 608, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(615, A::div(s.ad_value(1008), s.ad_value(172)), s.ad_value(380), 380);
            s.store_add(173, 615, 614);
        }

        s.b[1963] = (s.v[173] > 0.0);
        s.v[1963] = if s.b[1963] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (!s.b[1950])) && s.b[1962]) && s.b[1963]) {
            s.store_ad_value(616, A::div_scaled_product_by_product(s.ad_value(614), s.ad_value(615), 1.0, s.ad_value(173), A::scale_offset(A::powf(A::sub(s.ad_value(400), s.ad_value(320)), p.p1017), p.p1016, 1.0), 1.0));
        }

        if ((((!s.b[1620]) && (!s.b[1950])) && s.b[1962]) && (!s.b[1963])) {
            s.store_scalar(616, 0.0);
        }

        if (((!s.b[1620]) && (!s.b[1950])) && (!s.b[1962])) {
            s.store_scalar(616, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scaled_div(167, 243, 607, 1.0 / (s.v[184]));
            s.store_square(168, 167);
            s.store_offset_scaled(170, 168, (((p.p1022 * s.v[184])) * (p.p1019)), p.p1019);
            s.store_offset_scaled(171, 168, (((p.p1023 * s.v[184])) * (p.p1020)), p.p1020);
            s.store_offset_scaled(172, 168, (((p.p1298 * s.v[184])) * (p.p1297)), p.p1297);
            s.store_offset_scaled(630, 168, (((p.p1024 * s.v[184])) * (p.p1021)), p.p1021);
            s.store_scaled_mul(631, 170, 170, 3.0);
        }

        if (!s.b[1620]) {
            let assign54590_ad_e89522: A = A::scale_offset(s.ad_value(631), { let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));
            s.store_ad_value(631, assign54590_ad_e89522);
        }

        if (!s.b[1620]) {
            s.store_square(633, 172);
            s.store_square(632, 171);
            s.store_scalar(627, 0.0);
        }

        s.b[1964] = (p.p39 == 0.0);
        s.v[1964] = if s.b[1964] { 1.0 } else { 0.0 };

        s.b[1965] = (p.p39 == 1.0);
        s.v[1965] = if s.b[1965] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1964]) {
            s.store_scaled_mul(388, 271, 382, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));
            s.store_scaled_mul(389, 271, 385, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));
            s.store_mul_abs_ad_rhs(167, 337, A::add(s.ad_value(388), s.ad_value(389)));
            s.store_offset_mul(168, 167, 457, (s.v[184] * s.v[184]));
            s.store_scaled_div(619, 167, 168, p.p1018);
            s.store_mul(620, 621, 619);
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            s.store_scaled_mul(626, 253, 269, 2.0);
            s.store_mul_scale_ad_lhs(167, A::mul3(s.ad_value(337), s.ad_value(345), s.ad_value(363)), s.v[199], 626);
            s.store_scaled_add(168, 400, 320, 0.5);
            s.store_offset(170, 168, 0.5);
            s.store_square(171, 170);
            s.store_mul(172, 171, 170);
            s.store_sub(173, 400, 320);
            s.store_square(174, 173);
            s.store_mul(175, 174, 173);
            s.store_mul_ad_lhs(176, A::scale_offset(s.ad_value(168), 6.0, 0.5), 174);
            s.store_scale(625, 345, s.v[184]);
            s.store_scale(177, 625, 1.0 / (s.v[184]));
            s.store_offset_ad(179, A::div_scaled_product_by_product(s.ad_value(633), s.ad_value(315), 1.0, s.ad_value(316), A::offset(s.ad_value(243), p.p1299), 1.0), 1.0);
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            let assign54840_ad_e89826: A = A::scale_offset(s.ad_value(179), { let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));
            s.store_ad_value(179, assign54840_ad_e89826);
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            s.store_scaled_add_ad_rhs(179, 179, A::sqrt(A::offset(A::mul(s.ad_value(179), s.ad_value(179)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul_ad(624, A::div_scaled_inputs(s.ad_value(167), (p.p2 * s.v[183]), s.ad_value(625), 1.0), A::add_scaled_product(A::div_scaled_product(s.ad_value(174), s.ad_value(631), 1.0, s.ad_value(170), 12.0), 1.0, s.ad_value(168), s.ad_value(179), 1.0));
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            let assign54870_ad_e89925: A = A::mul3_scaled_output(A::mul3(s.ad_value(625), s.ad_value(177), s.ad_value(177)), A::add_scaled_inputs3(A::div(s.ad_value(168), s.ad_value(171)), 1.0, A::div(s.ad_value(176), A::mul_scaled_lhs(s.ad_value(171), 60.0, s.ad_value(171))), (-1.0), A::div_scaled_product_by_product(s.ad_value(174), s.ad_value(174), 1.0, s.ad_value(171), s.ad_value(172), 144.0), 1.0), s.ad_value(632), (15.0 * 1.0 / (4.0)));
            s.store_div_scaled_inputs(622, assign54870_ad_e89925, 1.0, s.ad_value(167), ((p.p2 * s.v[183]) * 12.0));
        }

        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
            s.store_mul_ad_affine_product_lhs(623, s.ad_value(177), A::sub(A::div_scaled_inputs(s.ad_value(173), 1.0, s.ad_value(170), 12.0), A::div_scaled_inputs(s.ad_value(175), 1.0, s.ad_value(172), 144.0)), 2.531645569620253, 0.0, 630);
            s.store_sqrt_mul(628, 621, 624);
        }

        s.b[1966] = (s.v[622] > 0.0);
        s.v[1966] = if s.b[1966] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) && s.b[1966]) {
            s.store_sqrt_div(629, 621, 622);
        }

        s.b[1967] = (s.v[628] > 0.0);
        s.v[1967] = if s.b[1967] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) && s.b[1966]) && s.b[1967]) {
            s.store_div_scaled_product(627, s.ad_value(623), s.ad_value(629), 1.0, s.ad_value(628), 1.0);
        }

        if ((((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) && s.b[1966]) && (!s.b[1967])) {
            s.store_scalar(627, 0.0);
        }

        if (((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) && (!s.b[1966])) {
            s.store_scalar(629, 0.0);
            s.store_scalar(627, 0.0);
        }

        s.b[1968] = (p.p37 != 0.0);
        s.v[1968] = if s.b[1968] { 1.0 } else { 0.0 };

        s.b[1969] = (p.p38 != 0.0);
        s.v[1969] = if s.b[1969] { 1.0 } else { 0.0 };

        if (!s.b[1620]) {
            s.copy_ad(217, 213);
            s.store_scalar(418, 0.0);
        }

        s.b[1970] = (p.p31 == 1.0);
        s.v[1970] = if s.b[1970] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_offset(793, 793, p.p25);
            s.store_mul(222, 221, 272);
            s.store_mul(225, 224, 272);
            s.store_mul(212, 793, 272);
            s.store_sub(217, 222, 212);
            s.store_ln_ad(432, A::max_with_scalar(A::div(s.ad_value(794), s.ad_value(182)), 1e-38));
            s.store_scaled_sqrt_ad(433, A::mul_scaled_lhs(s.ad_value(794), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(272)), 1.0 / (s.v[199]));
            s.store_div_from_scalar(295, 1.0, 433);
            s.store_div_scaled_inputs(406, s.ad_value(704), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(271), (s.v[199] * s.v[199]));
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_ad_value(418, {
                if (s.v[704] > 0.0) {
                    A::div_from_scalar(1.0, s.ad_value(406))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_ad_value(403, {
                if (s.v[704] > 0.0) {
                    A::div(s.ad_value(794), s.ad_value(704))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_offset(168, 403, 1.0);
            s.store_div(404, 217, 168);
            s.store_div(405, 433, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1971] = (s.v[404] < 0.0);
        s.v[1971] = if s.b[1971] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1971]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1971])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(433), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(432), (-2.0), s.ad_value(225), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1972] = (s.v[175] <= (-68.0));
        s.v[1972] = if s.b[1972] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1972]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1973] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1973] = if s.b[1973] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && s.b[1973]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1974] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1974] = if s.b[1974] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && (!s.b[1973])) && s.b[1974]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && (!s.b[1973])) && (!s.b[1974])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1970]) && s.b[1972]) {
            s.store_mul_ad_rhs(400, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1972])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(400, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_scaled_add_ad(256, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 256);
            s.store_sub_scaled_inputs(255, 254, 1.0, 400, 2.0);
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_offset_div_ad(253, s.ad_value(433), A::add(s.ad_value(259), A::sqrt(s.ad_value(167))), 1.0);
            s.store_mul_ad_rhs(167, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(247, 167, A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul3_affine_lhs(306, 253, 271, 2.0, 0.0, 400);
            s.store_mul_ad_rhs(308, 335, A::add_scaled_inputs(s.ad_value(247), 1.0, s.ad_value(306), s.v[338]));
            s.store_mul_ad(170, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(308), s.ad_value(651)));
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(309, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_ad_value(313, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(271), 1.0, s.ad_value(309), s.ad_value(655), s.v[188]));
        }

    }

    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && s.b[1970]) {
            s.store_ad_value(307, A::div_scaled_product_offset_denominator(s.ad_value(313), A::add(A::square(s.ad_value(400)), s.ad_value(400)), 1.0, A::mul_offset_rhs(s.ad_value(313), s.ad_value(400), 1.0), 1.0, 1.0));
        }

        if ((!s.b[1620]) && s.b[1970]) {
            let assign55670_ad_e91153: A = A::add_scaled_inputs4(s.ad_value(254), 1.0, s.ad_value(432), (-2.0), s.ad_value(307), (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::add(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(433), 1.0, s.ad_value(253), (-1.0), 1.0))), 1e-38)), -1.0);
            s.store_ad_value(321, assign55670_ad_e91153);
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_mul(322, 321, 271);
            s.store_add_scaled_inputs3(317, s.ad_value(322), 0.5, s.ad_value(224), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(322), s.ad_value(224)), A::sub(s.ad_value(322), s.ad_value(224))), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        s.b[1975] = ((p.p1353 == 0.0) && (p.p1354 == 0.0));
        s.v[1975] = if s.b[1975] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1975]) {
            s.store_scalar(1020, p.p1348);
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1975])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_ad(1020, A::div_scaled_inputs2(s.ad_value(168), p.p1353, A::mul3_scaled_output(s.ad_value(168), s.ad_value(400), s.ad_value(269), p.p1354), (-1.0), A::scale_offset(s.ad_value(218), p.p1355, 1.0), 1.0), 1.0);
            s.store_scaled_add_ad(1020, A::offset(s.ad_value(1020), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(1020), (-0.1), A::offset(s.ad_value(1020), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_div(317, 317, 1020);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(317)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 272);
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(433), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(432), (-2.0), s.ad_value(318), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1976] = (s.v[175] <= (-68.0));
        s.v[1976] = if s.b[1976] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1976]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1977] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1977] = if s.b[1977] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && s.b[1977]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1978] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1978] = if s.b[1978] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && (!s.b[1977])) && s.b[1978]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && (!s.b[1977])) && (!s.b[1978])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1970]) && s.b[1976]) {
            s.store_mul_ad_rhs(320, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1976])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(320, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_add_scaled_inputs3_offset(255, s.ad_value(254), 1.0, s.ad_value(400), (-1.0), s.ad_value(320), -1.0, (-1.0));
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(169, 167);
            s.store_add_ad(170, A::offset(s.ad_value(403), 1.0), A::div(s.ad_value(433), A::add(s.ad_value(259), s.ad_value(169))));
            s.store_offset_ad(171, A::mul3(s.ad_value(403), s.ad_value(169), s.ad_value(295)), 0.5);
            s.store_sqrt_add_ad(172, A::square(s.ad_value(171)), A::mul3(s.ad_value(170), A::add(s.ad_value(400), s.ad_value(320)), s.ad_value(418)));
            s.store_div_ad_rhs(253, 170, A::add(s.ad_value(171), s.ad_value(172)));
            s.store_mul_ad_rhs(167, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(247, 167, A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul_ad_rhs(168, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(320), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(248, 168, A::sqrt(A::offset(A::mul(s.ad_value(168), s.ad_value(168)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_scaled_add(249, 247, 248, 0.5);
            s.store_mul_ad_product_rhs(243, 253, s.ad_value(271), A::add(s.ad_value(400), s.ad_value(320)));
            s.store_mul_ad_rhs(336, 335, A::add_scaled_inputs(s.ad_value(249), 1.0, s.ad_value(243), s.v[338]));
            s.store_offset(168, 403, 1.0);
            s.store_div_scaled_inputs2(404, s.ad_value(217), 1.0, s.ad_value(272), p.p139, s.ad_value(168), 1.0);
            s.store_div(405, 433, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1979] = (s.v[404] < 0.0);
        s.v[1979] = if s.b[1979] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1970]) && s.b[1979]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1979])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!s.b[1620]) && s.b[1970]) {
            s.store_mul_ad(170, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(336), s.ad_value(651)));
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(339, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_ad_value(314, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(271), 2.0, s.ad_value(339), s.ad_value(655), s.v[188]));
            s.store_sub(250, 400, 320);
            s.store_mul_ad_affine_product_rhs(168, 314, s.ad_value(250), A::mul(s.ad_value(314), s.ad_value(250)), 2.0, 0.0);
            s.store_sqrt_offset_input(342, 168, 1.0);
            s.store_scaled_offset(343, 342, 1.0, 0.5);
            s.store_div_scaled_inputs(310, s.ad_value(655), 2.0, A::div(s.ad_value(740), s.ad_value(339)), 1.0);
            s.store_scale(311, 310, s.v[188]);
            s.store_add(358, 317, 311);
            s.store_sub(355, 226, 315);
        }

        s.b[1980] = (s.v[786] != 0.0);
        s.v[1980] = if s.b[1980] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1980]) {
            s.store_offset_mul_ad(364, s.ad_value(786), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(355), 1.0, s.ad_value(786), s.ad_value(358), 1.0), 1.0), 1e-38)), 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1980])) {
            s.store_scalar(364, 1.0);
        }

        if (!s.b[1620]) {
            s.store_square(407, 364);
            s.store_div_from_scalar(408, 1.0, 364);
            s.store_div_from_scalar(409, 1.0, 407);
            s.store_offset(410, 364, (-1.0));
            s.store_sub(413, 217, 254);
            s.store_sub(416, 400, 320);
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
            s.store_add_scaled_inputs(411, 413, 1.0, 400, 2.0);
            s.store_add_scaled_inputs(412, 413, 1.0, 320, 2.0);
            s.store_scaled_add_ad_rhs(168, 411, A::sqrt(A::offset(A::mul(s.ad_value(411), s.ad_value(411)), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_scaled_add_ad_rhs(169, 412, A::sqrt(A::offset(A::mul(s.ad_value(412), s.ad_value(412)), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_sqrt_offset_ad(414, A::mul(s.ad_value(168), s.ad_value(418)), 0.25);
            s.store_sqrt_offset_ad(415, A::mul(s.ad_value(169), s.ad_value(418)), 0.25);
            s.store_div_ad_rhs(168, 411, A::scale_offset(s.ad_value(414), 2.0, 1.0));
            s.store_div_ad_rhs(169, 412, A::scale_offset(s.ad_value(415), 2.0, 1.0));
            s.store_add(170, 414, 415);
            s.store_scaled_div_ad_rhs(171, 417, A::mul(A::square(s.ad_value(170)), s.ad_value(170)), 0.3333333333333333);
            s.store_ad_value(172, A::div_scaled_product3(s.ad_value(1020), s.ad_value(343), s.ad_value(408), 1.0, A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)), 1.0));
            s.store_mul_scale_ad_lhs(173, A::add_scaled_square_product(s.ad_value(170), 1.0, s.ad_value(414), s.ad_value(415), 1.0), 0.8, 172);
            s.store_add_scaled_inputs(174, 173, 1.0, 418, 2.0);
            s.store_scaled_mul(175, 417, 172, 0.3333333333333333);
            s.store_div_scaled_product(402, s.ad_value(412), A::scale_offset(s.ad_value(415), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(415), 2.0, 1.0), 1.0);
            s.store_add_ad_lhs(401, A::add_scaled_offset_product_lhs(s.ad_value(413), 1.0, s.ad_value(253), (-1.0), s.ad_value(320), (-2.0)), 402);
            s.store_add_scaled_products(381, s.ad_value(408), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, A::add_scaled_products(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(253), A::add_scaled_inputs3(s.ad_value(400), 1.0, s.ad_value(320), 1.0, s.ad_value(175), 1.0), (-1.0)), 1.0), 1.0, s.ad_value(410), s.ad_value(401), 1.0);
            s.store_add(176, 400, 320);
            s.store_mul3_lhs(177, 417, 172, 172);
            s.store_add_ad(386, A::mul3(s.ad_value(253), s.ad_value(408), A::add_scaled_product(s.ad_value(176), 1.0, s.ad_value(417), s.ad_value(172), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(253), s.ad_value(410), s.ad_value(320), 2.0));
            s.store_mul_ad_product_rhs(383, 253, s.ad_value(409), A::add_scaled_product(s.ad_value(176), 0.5, s.ad_value(416), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(416), s.ad_value(172))), 1.0, s.ad_value(177), 0.2), (-1.0 / (6.0))));
            s.store_mul_ad_product_lhs(384, s.ad_value(253), A::sub(s.ad_value(364), s.ad_value(408)), 320);
            s.store_add(385, 383, 384);
            s.store_sub(382, 386, 385);
            s.store_add_scaled_product(246, A::sqrt(A::offset(A::mul3(s.ad_value(271), s.ad_value(381), A::mul(s.ad_value(271), s.ad_value(381))), ((0.25 * 0.1) * 0.1))), 0.5, s.ad_value(271), s.ad_value(381), 0.5);
            s.store_mul_add_rhs(245, 271, 382, 385);
            s.store_add_scaled_inputs(167, 245, 1.0 / (p.p230), 246, (p.p231 * 1.0 / (p.p230)));
            s.store_offset_powf_ad(168, s.ad_value(167), (0.7 * p.p229), 1.0);
            s.store_div_from_scalar(427, (p.p228 * 1.9e-9), 168);
            s.store_div_from_scalar_ad(428, (3.9 * 8.8541878128e-12), A::add_scaled_inputs(s.ad_value(429), (3.9 * 1.0 / (p.p110)), s.ad_value(427), 1.0 / (s.v[200])));
            s.store_mul_ad_affine_product_lhs(387, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), s.ad_value(271), (-(((p.p2 * s.v[187]) * s.v[188]) + p.p1379)), 0.0, 381);
            s.store_scaled_mul(391, 428, 271, (((p.p2 * s.v[187]) * s.v[188]) + p.p1379));
            s.store_mul_neg_lhs(389, 391, 385);
            s.store_mul_neg_lhs(388, 391, 382);
        }

        s.b[1981] = (p.p45 == 1.0);
        s.v[1981] = if s.b[1981] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_scalar(795, (p.p140 + p.p25));
            s.store_mul(231, 230, 272);
            s.store_mul(233, 232, 272);
            s.store_mul(212, 795, 272);
        }

    }

    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && s.b[1981]) {
            s.store_sub(434, 231, 212);
            s.store_ln_ad(435, A::max_with_scalar(A::div_from_scalar(p.p141, s.ad_value(182)), 1e-38));
            s.store_scaled_sqrt_scaled_input(436, 272, (((2.0 * 1.602176462e-19) * s.v[180]) * p.p141), 1.0 / (s.v[199]));
            s.store_div_from_scalar(295, 1.0, 436);
            s.store_scalar(418, 0.0);
            s.store_scalar(403, 0.0);
            s.store_offset(168, 403, 1.0);
            s.store_div(404, 434, 168);
            s.store_div(405, 436, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1982] = (s.v[404] < 0.0);
        s.v[1982] = if s.b[1982] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1981]) && s.b[1982]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1981]) && (!s.b[1982])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(436), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(436), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(435), (-2.0), s.ad_value(233), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1983] = (s.v[175] <= (-68.0));
        s.v[1983] = if s.b[1983] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1981]) && s.b[1983]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1984] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1984] = if s.b[1984] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1981]) && s.b[1983]) && s.b[1984]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1985] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1985] = if s.b[1985] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1981]) && s.b[1983]) && (!s.b[1984])) && s.b[1985]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1981]) && s.b[1983]) && (!s.b[1984])) && (!s.b[1985])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1981]) && s.b[1983]) {
            s.store_mul_ad_rhs(400, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (((!s.b[1620]) && s.b[1981]) && (!s.b[1983])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(400, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_scaled_add_ad(256, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 256);
            s.store_sub_scaled_inputs(255, 254, 1.0, 400, 2.0);
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_offset_div_ad(253, s.ad_value(436), A::add(s.ad_value(259), A::sqrt(s.ad_value(167))), 1.0);
            s.store_mul_ad_rhs(167, 271, A::add_scaled_inputs_product(s.ad_value(434), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));
            s.store_scaled_add_ad_rhs(247, 167, A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1))), 0.5);
            s.store_mul3_affine_lhs(306, 253, 271, 2.0, 0.0, 400);
            s.store_mul_ad_rhs(308, 335, A::add_scaled_inputs(s.ad_value(247), 1.0, s.ad_value(306), s.v[338]));
            s.store_mul_ad(170, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(241), 1.0), A::pow(s.ad_value(308), s.ad_value(651)));
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_ad(309, A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(171), (-1.0), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
            s.store_ad_value(313, A::div_scaled_product_by_product(s.ad_value(740), s.ad_value(271), 1.0, s.ad_value(309), s.ad_value(655), s.v[188]));
            s.store_ad_value(307, A::div_scaled_product_offset_denominator(s.ad_value(313), A::add(A::square(s.ad_value(400)), s.ad_value(400)), 1.0, A::mul_offset_rhs(s.ad_value(313), s.ad_value(400), 1.0), 1.0, 1.0));
        }

        if ((!s.b[1620]) && s.b[1981]) {
            let assign57550_ad_e93994: A = A::add_scaled_inputs4(s.ad_value(254), 1.0, s.ad_value(435), (-2.0), s.ad_value(307), (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::add(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(436), 1.0, s.ad_value(253), (-1.0), 1.0))), 1e-38)), -1.0);
            s.store_ad_value(321, assign57550_ad_e93994);
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_mul(322, 321, 271);
            s.store_add_scaled_inputs3(317, s.ad_value(322), 0.5, s.ad_value(232), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(322), s.ad_value(232)), A::sub(s.ad_value(322), s.ad_value(232))), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if ((!s.b[1620]) && s.b[1981]) {
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(317)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 232, 272);
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(436), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(436), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(435), (-2.0), s.ad_value(318), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1986] = (s.v[175] <= (-68.0));
        s.v[1986] = if s.b[1986] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1981]) && s.b[1986]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[1987] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[1987] = if s.b[1987] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1981]) && s.b[1986]) && s.b[1987]) {
            s.store_limited_exp(170, 171);
        }

        s.b[1988] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[1988] = if s.b[1988] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1981]) && s.b[1986]) && (!s.b[1987])) && s.b[1988]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1981]) && s.b[1986]) && (!s.b[1987])) && (!s.b[1988])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1981]) && s.b[1986]) {
            s.store_mul_ad_rhs(320, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (((!s.b[1620]) && s.b[1981]) && (!s.b[1986])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(320, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if ((!s.b[1620]) && s.b[1981]) {
            s.store_add_scaled_inputs3_offset(255, s.ad_value(254), 1.0, s.ad_value(400), (-1.0), s.ad_value(320), -1.0, (-1.0));
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(169, 167);
            s.store_add_ad(170, A::offset(s.ad_value(403), 1.0), A::div(s.ad_value(436), A::add(s.ad_value(259), s.ad_value(169))));
            s.store_offset_ad(171, A::mul3(s.ad_value(403), s.ad_value(169), s.ad_value(295)), 0.5);
            s.store_sqrt_add_ad(172, A::square(s.ad_value(171)), A::mul3(s.ad_value(170), A::add(s.ad_value(400), s.ad_value(320)), s.ad_value(418)));
            s.store_div_ad_rhs(253, 170, A::add(s.ad_value(171), s.ad_value(172)));
            s.store_scalar(364, 1.0);
            s.store_square(407, 364);
            s.store_div_from_scalar(408, 1.0, 364);
            s.store_div_from_scalar(409, 1.0, 407);
            s.store_offset(410, 364, (-1.0));
            s.store_sub(413, 434, 254);
            s.store_sub(416, 400, 320);
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
            s.store_add_scaled_inputs(411, 413, 1.0, 400, 2.0);
            s.store_add_scaled_inputs(412, 413, 1.0, 320, 2.0);
            s.store_scaled_add_ad_rhs(168, 411, A::sqrt(A::offset(A::mul(s.ad_value(411), s.ad_value(411)), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_scaled_add_ad_rhs(169, 412, A::sqrt(A::offset(A::mul(s.ad_value(412), s.ad_value(412)), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_sqrt_offset_ad(414, A::mul(s.ad_value(168), s.ad_value(418)), 0.25);
            s.store_sqrt_offset_ad(415, A::mul(s.ad_value(169), s.ad_value(418)), 0.25);
            s.store_div_ad_rhs(168, 411, A::scale_offset(s.ad_value(414), 2.0, 1.0));
            s.store_div_ad_rhs(169, 412, A::scale_offset(s.ad_value(415), 2.0, 1.0));
            s.store_add(170, 414, 415);
            s.store_scaled_div_ad_rhs(171, 417, A::mul(A::square(s.ad_value(170)), s.ad_value(170)), 0.3333333333333333);
            s.store_scalar(343, 0.0);
            s.store_div_scaled_product(172, s.ad_value(343), s.ad_value(408), 1.0, A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)), 1.0);
            s.store_mul_scale_ad_lhs(173, A::add_scaled_square_product(s.ad_value(170), 1.0, s.ad_value(414), s.ad_value(415), 1.0), 0.8, 172);
            s.store_add_scaled_inputs(174, 173, 1.0, 418, 2.0);
            s.store_scaled_mul(175, 417, 172, 0.3333333333333333);
            s.store_div_scaled_product(402, s.ad_value(412), A::scale_offset(s.ad_value(415), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(415), 2.0, 1.0), 1.0);
            s.store_add_ad_lhs(401, A::add_scaled_offset_product_lhs(s.ad_value(413), 1.0, s.ad_value(253), (-1.0), s.ad_value(320), (-2.0)), 402);
            s.store_add_scaled_products(381, s.ad_value(408), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, A::add_scaled_products(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(253), A::add_scaled_inputs3(s.ad_value(400), 1.0, s.ad_value(320), 1.0, s.ad_value(175), 1.0), (-1.0)), 1.0), 1.0, s.ad_value(410), s.ad_value(401), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_43(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[1620]) && s.b[1981]) {
            s.store_add(176, 400, 320);
            s.store_mul3_lhs(177, 417, 172, 172);
            s.store_add_ad(386, A::mul3(s.ad_value(253), s.ad_value(408), A::add_scaled_product(s.ad_value(176), 1.0, s.ad_value(417), s.ad_value(172), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(253), s.ad_value(410), s.ad_value(320), 2.0));
            s.store_mul_ad_product_rhs(383, 253, s.ad_value(409), A::add_scaled_product(s.ad_value(176), 0.5, s.ad_value(416), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(416), s.ad_value(172))), 1.0, s.ad_value(177), 0.2), (-1.0 / (6.0))));
            s.store_mul_ad_product_lhs(384, s.ad_value(253), A::sub(s.ad_value(364), s.ad_value(408)), 320);
            s.store_add(385, 383, 384);
            s.store_mul_scale_ad_lhs(437, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), p.p1380, 271);
            s.store_mul(440, 437, 381);
            s.store_mul(439, 437, 385);
            s.store_mul(438, 437, 386);
        }

        if ((!s.b[1620]) && (!s.b[1981])) {
            s.store_scalar(440, 0.0);
            s.store_scalar(439, 0.0);
            s.store_scalar(438, 0.0);
        }

        if (!s.b[1620]) {
            s.copy_ad(394, 389);
            s.copy_ad(395, 388);
            s.copy_ad(393, 387);
            s.store_neg_ad(392, A::add_scaled_inputs3(s.ad_value(393), 1.0, s.ad_value(395), 1.0, s.ad_value(394), 1.0));
            s.store_neg_ad(398, A::scale(s.ad_value(439), p.p45));
            s.store_neg_ad(399, A::sub_scaled_inputs(s.ad_value(438), p.p45, s.ad_value(439), p.p45));
            s.store_neg_ad(397, A::scale(s.ad_value(440), p.p45));
            s.store_neg_ad(396, A::add_scaled_inputs3(s.ad_value(397), 1.0, s.ad_value(399), 1.0, s.ad_value(398), 1.0));
            s.store_neg_ad(389, A::sub(A::scale(s.ad_value(439), p.p45), s.ad_value(389)));
            s.store_neg_ad(388, A::add_scaled_inputs3(s.ad_value(438), p.p45, s.ad_value(388), (-1.0), s.ad_value(439), (-p.p45)));
            s.store_neg_ad(387, A::sub(A::scale(s.ad_value(440), p.p45), s.ad_value(387)));
            s.store_neg_ad(390, A::add_scaled_inputs3(s.ad_value(387), 1.0, s.ad_value(388), 1.0, s.ad_value(389), 1.0));
        }

        s.b[1989] = (!param_given[867]);
        s.v[1989] = if s.b[1989] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1989]) {
            s.store_scalar(788, ((((2.0 * p.p110) * 8.8541878128e-12) / 3.141592653589793) * ((((p.p871 * (1.0 + (4e-7 / p.p76)))).max(1e-38)) as f64).ln()));
        }

        if (!s.b[1620]) {
            s.store_offset(425, 788, p.p872);
            s.store_offset(426, 788, p.p873);
        }

        s.b[1990] = (p.p32 == 0.0);
        s.v[1990] = if s.b[1990] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1990]) {
            s.store_scaled_mul(423, 425, 431, ((-s.v[187]) * p.p2));
            s.store_scaled_mul(424, 426, 430, ((-s.v[187]) * p.p2));
        }

        if ((!s.b[1620]) && (!s.b[1990])) {
            s.store_sqrt_offset_ad(167, A::mul_offset_lhs(A::sub(s.ad_value(431), s.ad_value(219)), 0.02, A::offset(A::sub(s.ad_value(431), s.ad_value(219)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset(419, s.ad_value(431), 0.5, s.ad_value(219), ((-1.0) * 0.5), s.ad_value(167), (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(173, 419, A::powf(A::offset(A::powf(A::scale(s.ad_value(419), (-1.0 / (p.p893))), p.p894), 1.0), (1.0 / p.p894)));
            s.store_sqrt_sub_from_scalar_ad(168, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(791), 1.0));
            s.store_add_scaled_products(423, s.ad_value(425), s.ad_value(431), ((-s.v[187]) * p.p2), s.ad_value(789), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(431), 1.0, s.ad_value(219), (-1.0), s.ad_value(419), -1.0), 1.0, s.ad_value(791), s.ad_value(168), (-1.0), (-0.5)), ((-s.v[187]) * p.p2));
            s.store_sqrt_offset_ad(167, A::mul_offset_lhs(A::sub(s.ad_value(430), s.ad_value(219)), 0.02, A::offset(A::sub(s.ad_value(430), s.ad_value(219)), 0.02)), (4.0 * 0.02));
            s.store_add_scaled_inputs3_offset(420, s.ad_value(430), 0.5, s.ad_value(219), ((-1.0) * 0.5), s.ad_value(167), (-0.5), (0.02 * 0.5));
            s.store_div_ad_rhs(173, 420, A::powf(A::offset(A::powf(A::scale(s.ad_value(420), (-1.0 / (p.p891))), p.p892), 1.0), (1.0 / p.p892)));
            s.store_sqrt_sub_from_scalar_ad(169, 1.0, A::div_scaled_inputs(s.ad_value(173), 4.0, s.ad_value(792), 1.0));
            s.store_add_scaled_products(424, s.ad_value(426), s.ad_value(430), ((-s.v[187]) * p.p2), s.ad_value(790), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(430), 1.0, s.ad_value(219), (-1.0), s.ad_value(420), -1.0), 1.0, s.ad_value(792), s.ad_value(169), (-1.0), (-0.5)), ((-s.v[187]) * p.p2));
        }

        if (!s.b[1620]) {
            s.store_ad_value(421, A::mul_scaled_lhs(s.ad_value(379), (((-p.p2) * s.v[188]) * p.p874), A::voltage(ctx, nodes, Some(9), Some(10))));
            s.store_neg_ad(422, A::add_scaled_inputs3(s.ad_value(423), 1.0, s.ad_value(424), 1.0, s.ad_value(421), 1.0));
            s.store_scalar(1035, ((s.v[261] - (2.0 * s.v[196])) - p.p1394));
            s.store_offset(1036, 1035, (2.0 * p.p1393));
        }

        s.b[1991] = (s.v[908] > 0.0);
        s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1991]) {
            s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(908)), 1e-38));
            s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);
        }

        if ((!s.b[1620]) && (!s.b[1991])) {
            s.store_ln_ad(167, A::max_with_scalar(A::div_scaled_product_by_product(s.ad_value(706), s.ad_value(908), -1.0, s.ad_value(182), s.ad_value(182), 1.0), 1e-38));
            s.store_mul3_affine_lhs(215, 379, 637, -1.0, 0.0, 167);
        }

        if (!s.b[1620]) {
            s.store_sub(1032, 235, 215);
            s.store_scalar(1034, (3.453133e-11 / p.p75));
            s.store_mul_ad_affine_product_rhs(1037, 909, s.ad_value(1034), A::scale_offset(s.ad_value(1036), ((s.v[187] / p.p1373) * p.p2), p.p1382), p.p1388, 0.0);
            s.store_mul_sub_rhs(1038, 1037, 1032, 1033);
            s.copy_ad(1039, 1038);
            s.store_scalar(167, (p.p1395 * ((((p.p871 * (1.0 + (p.p74 / p.p75)))).max(1e-38)) as f64).ln()));
            s.store_scalar(168, (p.p19 - p.p1));
        }

        s.b[1992] = (s.v[168] > 0.0);
        s.v[1992] = if s.b[1992] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1992]) {
            s.store_mul(1040, 167, 168);
        }

        if ((!s.b[1620]) && (!s.b[1992])) {
            s.store_scalar(1040, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scalar(168, (p.p20 - p.p1));
        }

        s.b[1993] = (s.v[168] > 0.0);
        s.v[1993] = if s.b[1993] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1993]) {
            s.store_mul(1041, 167, 168);
        }

        if ((!s.b[1620]) && (!s.b[1993])) {
            s.store_scalar(1041, 0.0);
        }

        if (!s.b[1620]) {
            s.store_scale(1042, 1034, p.p17);
            s.store_scalar(1043, (p.p1396 * p.p17));
            s.store_scale(1044, 1034, p.p18);
            s.store_scalar(1045, (p.p1396 * p.p18));
            s.store_mul_neg_lhs(177, 379, 236);
            s.store_mul_neg_lhs(178, 379, 237);
        }

        s.b[1994] = (p.p1396 != 0.0);
        s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1994]) {
            s.store_scaled_sub(168, 1044, 1045, ((-0.5) * 1.0 / (p.p1399)));
            s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(178), (-p.p1399), p.p1400)), 1e-38));
            s.store_mul_scale_ad_lhs(170, A::add(s.ad_value(1044), s.ad_value(1045)), 0.5, 178);
            s.store_add_scaled_product(1047, s.ad_value(170), 1.0, s.ad_value(168), s.ad_value(169), 1.0);
            s.store_scaled_sub(168, 1042, 1043, ((-0.5) * 1.0 / (p.p1397)));
            s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::scale_offset(s.ad_value(177), (-p.p1397), p.p1398)), 1e-38));
            s.store_mul_scale_ad_lhs(170, A::add(s.ad_value(1042), s.ad_value(1043)), 0.5, 177);
            s.store_add_scaled_product(1046, s.ad_value(170), 1.0, s.ad_value(168), s.ad_value(169), 1.0);
        }

        if ((!s.b[1620]) && (!s.b[1994])) {
            s.store_mul(1046, 1042, 177);
            s.store_mul(1047, 1044, 178);
        }

        if (!s.b[1620]) {
            s.store_add_scaled_product(1046, s.ad_value(1046), 1.0, s.ad_value(1040), s.ad_value(177), 1.0);
            s.store_add_scaled_product(1047, s.ad_value(1047), 1.0, s.ad_value(1041), s.ad_value(178), 1.0);
        }

        s.b[1995] = (p.p27 == 1.0);
        s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_ln_ad(951, A::max_with_scalar(A::div(s.ad_value(953), s.ad_value(182)), 1e-38));
            s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(951)), 0.4), s.ad_value(729)), 0.4);
            s.store_sqrt_div_from_scalar_ad(277, (2.0 * s.v[180]), A::scale(s.ad_value(953), 1.602176462e-19));
            s.store_mul_scale_ad_rhs(941, 835, A::add(A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0, A::offset(A::mul_offset_rhs(s.ad_value(847), s.ad_value(639), (-1.0)), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5);
            s.store_mul_offset_ad_rhs(940, 841, A::mul_offset_rhs(s.ad_value(848), s.ad_value(639), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_offset(273, s.ad_value(298), 0.5, s.ad_value(218), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05), A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05))), ((0.25 * 0.1) * 0.1))), 0.5, (0.05 * 0.5));
            s.store_sqrt(274, 273);
            s.store_mul(275, 277, 274);
            s.store_div_from_scalar(260, s.v[180], 275);
            s.store_ad_value(276, A::add_scaled_inputs_products(s.ad_value(836), 1.0, s.ad_value(941), 1.0, s.ad_value(838), s.ad_value(227), 1.0, s.ad_value(840), s.ad_value(218), (-1.0)));
            s.store_offset_scaled(168, 276, 1.0 / (s.v[199]), 1.0);
            s.store_scaled_add_ad(267, A::offset(s.ad_value(168), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(168), (-1.0), A::offset(s.ad_value(168), (-1.0))), ((0.25 * 0.05) * 0.05))), 0.5);
            s.store_mul(269, 267, 271);
            s.store_div_from_scalar(270, 1.0, 269);
            s.store_mul(222, 221, 270);
            s.store_mul(225, 224, 270);
            s.store_mul(212, 707, 270);
            s.store_mul_neg_ad_lhs(944, A::add_scaled_product(s.ad_value(940), 1.0, s.ad_value(842), s.ad_value(218), 1.0), 227);
            s.store_ad_value(293, A::mul_offset_rhs(A::add_scaled_inputs_product(s.ad_value(843), 1.0, s.ad_value(844), 1.0 / (s.v[184]), s.ad_value(845), s.ad_value(218), 1.0), A::pow(s.ad_value(639), s.ad_value(846)), (-1.0)));
            s.store_mul_ad_rhs(946, 300, A::scale_offset(s.ad_value(218), p.p1264, 1.0));
        }

        s.b[1996] = (s.v[946] > 0.0);
        s.v[1996] = if s.b[1996] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[1996]) {
            s.store_div_from_scalar(167, (p.p1263 * s.v[184]), 946);
        }

        s.b[1997] = (s.v[167] < 40.0);
        s.v[1997] = if s.b[1997] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1995]) && s.b[1996]) && s.b[1997]) {
            s.store_div_from_scalar_offset_ad(943, (0.5 * p.p1262), A::cosh(s.ad_value(167)), (-1.0));
        }

        if ((((!s.b[1620]) && s.b[1995]) && s.b[1996]) && (!s.b[1997])) {
            s.store_scaled_limited_exp_scaled_input(943, 167, -1.0, p.p1262);
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[1996])) {
            s.store_scalar(943, 0.0);
        }

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_mul_sub_rhs(945, 943, 942, 298);
            s.store_add_ad_lhs(242, A::add_scaled_product(A::add_scaled_inputs4_offset(s.ad_value(944), 1.0, s.ad_value(293), (-1.0), s.ad_value(945), 1.0, s.ad_value(956), 1.0, p.p1151), 1.0, s.ad_value(849), s.ad_value(218), (-1.0)), 932);
            s.store_ad_value(213, A::add_scaled_inputs_product(s.ad_value(222), 1.0, s.ad_value(212), (-1.0), s.ad_value(242), s.ad_value(270), (-1.0)));
            s.store_scalar(947, (p.p1148 * (1.0 + (p.p1149 * ((s.v[184]) as f64).powf((-p.p1150))))));
            s.store_scaled_sqrt_ad(954, A::mul_scaled_lhs(s.ad_value(953), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(270)), 1.0 / (s.v[199]));
            s.store_mul_offset_rhs(954, 954, 947, 1.0);
            s.store_div(952, 951, 267);
            s.store_scalar(168, 1.0);
            s.store_div(404, 213, 168);
            s.store_div(405, 954, 168);
            s.store_sub_scaled_ad_rhs(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));
        }

        s.b[1998] = (s.v[404] < 0.0);
        s.v[1998] = if s.b[1998] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[1998]) {
            s.store_div_scaled_inputs2(170, s.ad_value(404), 1.0, s.ad_value(169), (-1.0), s.ad_value(405), 1.0);
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[1998])) {
            s.store_limited_exp_neg_input(170, 169);
            s.store_scale(168, 405, 0.5);
            s.store_sub_ad_lhs(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

    }

    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && s.b[1995]) {
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(954), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(954), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(952), (-2.0), s.ad_value(225), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[1999] = (s.v[175] <= (-68.0));
        s.v[1999] = if s.b[1999] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[1999]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[2000] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[2000] = if s.b[2000] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1995]) && s.b[1999]) && s.b[2000]) {
            s.store_limited_exp(170, 171);
        }

        s.b[2001] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[2001] = if s.b[2001] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1995]) && s.b[1999]) && (!s.b[2000])) && s.b[2001]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1995]) && s.b[1999]) && (!s.b[2000])) && (!s.b[2001])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1995]) && s.b[1999]) {
            s.store_mul_ad_rhs(961, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[1999])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(961, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_add_scaled_product(948, s.ad_value(269), 2.0, s.ad_value(269), s.ad_value(961), 2.0);
            s.copy_ad(949, 948);
            s.store_add(949, 949, 224);
            s.store_add_scaled_inputs3(950, s.ad_value(949), 0.5, s.ad_value(224), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(949), s.ad_value(224)), A::sub(s.ad_value(949), s.ad_value(224))), ((0.25 * 0.001) * 0.001))), 0.5);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(950)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 270);
            s.store_scaled_add_ad(175, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 175);
            s.store_ad_value(167, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(954), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, s.ad_value(954), 1.0));
            s.store_add_scaled_inputs3(168, s.ad_value(254), 1.0, s.ad_value(952), (-2.0), s.ad_value(318), -1.0);
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));
            s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(169), s.ad_value(169), 0.402982), 2.446562)), 0.5);
            s.copy_ad(257, 259);
        }

        s.b[2002] = (s.v[175] <= (-68.0));
        s.v[2002] = if s.b[2002] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[2002]) {
            s.store_scalar(171, (-100.0));
            s.store_scalar(172, 20.0);
        }

        s.b[2003] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));
        s.v[2003] = if s.b[2003] { 1.0 } else { 0.0 };

        if ((((!s.b[1620]) && s.b[1995]) && s.b[2002]) && s.b[2003]) {
            s.store_limited_exp(170, 171);
        }

        s.b[2004] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));
        s.v[2004] = if s.b[2004] { 1.0 } else { 0.0 };

        if (((((!s.b[1620]) && s.b[1995]) && s.b[2002]) && (!s.b[2003])) && s.b[2004]) {
            s.store_limited_exp(170, 175);
        }

        if (((((!s.b[1620]) && s.b[1995]) && s.b[2002]) && (!s.b[2003])) && (!s.b[2004])) {
            s.store_div_scaled_inputs2(169, s.ad_value(175), 1.0, s.ad_value(171), (-1.0), s.ad_value(172), 1.0);
            s.store_square(173, 169);
            s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));
        }

        if (((!s.b[1620]) && s.b[1995]) && s.b[2002]) {
            s.store_mul_ad_rhs(960, 170, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(175), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[2002])) {
            s.store_limited_exp(170, 175);
            s.store_div_from_scalar(258, 1.0, 257);
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
            s.store_add_scaled_inputs3(171, s.ad_value(170), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(168), -1.0);
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_mul_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));
            s.store_add_scaled_inputs3(174, A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), s.ad_value(173), -1.0);
            s.store_ad_value(960, A::add_scaled_offset_product_rhs(s.ad_value(170), 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0)));
        }

        if ((!s.b[1620]) && s.b[1995]) {
            s.store_scaled_add_ad(256, A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(254), (-1.0), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(259, 256);
            s.store_add_scaled_inputs3_offset(255, s.ad_value(254), 1.0, s.ad_value(961), (-1.0), s.ad_value(960), -1.0, (-1.0));
            s.store_scaled_add_ad(167, A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(255), (-1.0), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(169, 167);
            s.store_offset_div_ad(959, s.ad_value(954), A::add(s.ad_value(259), s.ad_value(169)), 1.0);
            s.store_mul_ad_lhs(939, A::mul3(A::mul3_scaled_output(s.ad_value(959), s.ad_value(337), s.ad_value(269), ((2.0 * p.p2) * ((p.p1147 * 1.0 / (s.v[184])) * s.v[199]))), s.ad_value(269), A::mul(A::sub(s.ad_value(961), s.ad_value(960)), A::add(A::offset(s.ad_value(961), 1.0), s.ad_value(960)))), 363);
            s.store_add(380, 939, 380);
            s.store_scalar(964, (p.p1012 * p.p1316));
            s.store_scalar(965, (p.p1013 * p.p1316));
            s.store_scalar(966, (p.p1014 * p.p1316));
            s.store_sub_from_scalar_ad(962, s.v[184], A::scale(s.ad_value(606), 2.0));
            s.store_square(963, 962);
            s.store_mul_scaled_ad_rhs(613, 271, 1.0 / (1.602176462e-19), A::add(A::offset(s.ad_value(260), s.v[199]), s.ad_value(836)));
            s.store_mul3_affine_lhs(612, 959, 271, ((2.0 * s.v[199]) * 6.241509744511525e18), 0.0, 960);
            s.store_mul_ad_affine_product_lhs(1004, s.ad_value(271), A::abs(s.ad_value(939)), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19), 0.0, 337);
            s.store_mul3_affine_lhs(1005, 271, 939, 1.602176462e-19, 0.0, 939);
            s.store_add_ad(1006, A::add_scaled_product(s.ad_value(964), 1.0, s.ad_value(965), s.ad_value(612), 1.0), A::mul3(s.ad_value(966), s.ad_value(612), s.ad_value(612)));
            s.store_mul_ad(1007, A::add(s.ad_value(612), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613)));
            s.store_scaled_mul(1008, 964, 271, 1.602176462e-19);
            s.store_mul3_affine_lhs(611, 959, 271, ((2.0 * s.v[199]) * 6.241509744511525e18), 0.0, 961);
            s.store_mul_ln_ad_rhs(168, 964, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(611), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38));
            s.store_mul_sub_rhs(169, 965, 611, 612);
            s.store_mul_scaled_ad_rhs(170, 966, 0.5, A::sub(A::square(s.ad_value(611)), A::square(s.ad_value(612))));
            s.store_scale(171, 963, (10000000000.0 * (p.p1147 * p.p2)));
            s.store_add_scaled_product(614, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(171), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(167)), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, s.ad_value(170), 1.0), 1.0);
            s.store_mul3_affine_lhs(172, 962, 613, ((p.p1147 * p.p2) * 10000000000.0), 0.0, 613);
            s.store_mul_ad_product_lhs(615, A::div(s.ad_value(1008), s.ad_value(172)), s.ad_value(939), 939);
            s.store_add(173, 615, 614);
        }

        s.b[2005] = (s.v[173] > 0.0);
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if (((!s.b[1620]) && s.b[1995]) && s.b[2005]) {
            s.store_div_scaled_product(174, s.ad_value(614), s.ad_value(615), 1.0, s.ad_value(173), 1.0);
            s.store_offset_scaled_ad(175, A::powf(A::sub(s.ad_value(961), s.ad_value(960)), p.p1318), p.p1317, 1.0);
            s.store_div(967, 174, 175);
        }

        if (((!s.b[1620]) && s.b[1995]) && (!s.b[2005])) {
            s.store_scalar(967, 0.0);
        }

        if (!s.b[1620]) {
            s.store_mul_ad_rhs(1075, 379, A::add_scaled_inputs4(s.ad_value(387), 1.0, s.ad_value(421), 1.0, s.ad_value(520), 1.0, s.ad_value(525), 1.0));
        }

        s.b[2006] = (s.v[211] > 0.0);
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        if ((!s.b[1620]) && s.b[2006]) {
            s.store_mul(1050, 379, 388);
            s.store_mul(1051, 379, 395);
            s.store_mul(1052, 379, 399);
            s.store_mul(1053, 379, 389);
            s.store_mul(1054, 379, 394);
            s.store_mul(1055, 379, 398);
            s.store_mul_ad_rhs(1076, 379, A::add_scaled_inputs3(s.ad_value(388), 1.0, s.ad_value(423), 1.0, s.ad_value(520), -1.0));
            s.store_mul_ad_rhs(1077, 379, A::add_scaled_inputs3(s.ad_value(389), 1.0, s.ad_value(424), 1.0, s.ad_value(525), -1.0));
        }

        if ((!s.b[1620]) && (!s.b[2006])) {
            s.store_mul(1050, 379, 389);
            s.store_mul(1051, 379, 394);
            s.store_mul(1052, 379, 398);
            s.store_mul(1053, 379, 388);
            s.store_mul(1054, 379, 395);
            s.store_mul(1055, 379, 399);
            s.store_mul_ad_rhs(1076, 379, A::add_scaled_inputs3(s.ad_value(389), 1.0, s.ad_value(423), 1.0, s.ad_value(520), -1.0));
            s.store_mul_ad_rhs(1077, 379, A::add_scaled_inputs3(s.ad_value(388), 1.0, s.ad_value(424), 1.0, s.ad_value(525), -1.0));
        }

        if (!s.b[1620]) {
            s.store_mul_add_rhs(1078, 379, 390, 422);
            s.store_mul(1057, 379, 392);
            s.store_mul(1058, 379, 396);
        }

        s.v[1108] = s.v[183];

        s.v[1109] = s.v[184];

        s.b[2009] = (p.p38 != 0.0);
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        s.b[2010] = (p.p37 != 0.0);
        s.v[2010] = if s.b[2010] { 1.0 } else { 0.0 };

        s.b[2011] = (s.v[211] > 0.0);
        s.v[2011] = if s.b[2011] { 1.0 } else { 0.0 };

        s.b[2012] = (p.p7 == 0.0);
        s.v[2012] = if s.b[2012] { 1.0 } else { 0.0 };

        s.b[2015] = (p.p7 == 2.0);
        s.v[2015] = if s.b[2015] { 1.0 } else { 0.0 };

        if ((!s.b[2012]) && s.b[2015]) {
            s.copy_ad(2013, 467);
            s.store_div_scaled_product(2014, s.ad_value(467), s.ad_value(467), 1.0, s.ad_value(465), 1.0);
        }

        if ((!s.b[2012]) && (!s.b[2015])) {
            s.copy_ad(2013, 465);
            s.copy_ad(2014, 465);
        }

        s.b[2016] = ((p.p33 != 2.0) && (s.v[453] > 0.0));
        s.v[2016] = if s.b[2016] { 1.0 } else { 0.0 };

        if s.b[2016] {
            s.store_div_from_scalar(618, 1.0, 455);
        }

    }

    pub(super) fn stamp_transient_block_45(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[2017] = ((p.p33 != 2.0) && (s.v[453] > 0.0));
        s.v[2017] = if s.b[2017] { 1.0 } else { 0.0 };

        s.b[2018] = ((p.p33 != 2.0) && (s.v[452] > 0.0));
        s.v[2018] = if s.b[2018] { 1.0 } else { 0.0 };

        if s.b[2018] {
            s.store_div_from_scalar(617, 1.0, 454);
        }

        s.b[2019] = ((p.p33 != 2.0) && (s.v[452] > 0.0));
        s.v[2019] = if s.b[2019] { 1.0 } else { 0.0 };

        s.b[2020] = (p.p7 == 3.0);
        s.v[2020] = if s.b[2020] { 1.0 } else { 0.0 };

        s.b[2021] = ((p.p41 != 0.0) && (p.p1099 > 0.0));
        s.v[2021] = if s.b[2021] { 1.0 } else { 0.0 };

        if s.b[2021] {
            s.store_mul_voltage_ad(1017, A::mul3(s.ad_value(379), s.ad_value(211), s.ad_value(380)), ctx, nodes, Some(6), Some(7));
        }

        s.b[2022] = ((p.p33 != 2.0) && (s.v[453] > 0.0));
        s.v[2022] = if s.b[2022] { 1.0 } else { 0.0 };

        if (s.b[2021] && s.b[2022]) {
            s.store_add_ad_rhs(1017, 1017, A::div_scaled_product(A::voltage(ctx, nodes, Some(0), Some(6)), A::voltage(ctx, nodes, Some(0), Some(6)), 1.0, s.ad_value(455), 1.0));
        }

        s.b[2023] = ((p.p33 != 2.0) && (s.v[452] > 0.0));
        s.v[2023] = if s.b[2023] { 1.0 } else { 0.0 };

        if (s.b[2021] && s.b[2023]) {
            s.store_add_ad_rhs(1017, 1017, A::div_scaled_product(A::voltage(ctx, nodes, Some(2), Some(7)), A::voltage(ctx, nodes, Some(2), Some(7)), 1.0, s.ad_value(454), 1.0));
        }

        s.b[2024] = ((p.p40 != 0.0) && (!true));
        s.v[2024] = if s.b[2024] { 1.0 } else { 0.0 };

        s.b[2025] = true;
        s.v[2025] = if s.b[2025] { 1.0 } else { 0.0 };

        s.b[2026] = ((p.p40 != 0.0) && (!true));
        s.v[2026] = if s.b[2026] { 1.0 } else { 0.0 };

        s.b[2027] = true;
        s.v[2027] = if s.b[2027] { 1.0 } else { 0.0 };

        s.v[1024] = (p.p1359 * p.p1358);

        s.b[2028] = ((p.p43 == 0.0) || (!true));
        s.v[2028] = if s.b[2028] { 1.0 } else { 0.0 };

        s.b[2029] = ((p.p40 != 0.0) && (!true));
        s.v[2029] = if s.b[2029] { 1.0 } else { 0.0 };

        s.b[2030] = (p.p43 == 1.0);
        s.v[2030] = if s.b[2030] { 1.0 } else { 0.0 };

        if (((!s.b[2028]) && (!s.b[2029])) && s.b[2030]) {
            s.store_scalar(1025, ((((((p.p1357 * p.p1356) * p.p1360) / ((2.0 * p.p1356) + (p.p1360 * s.v[1109]))) * s.v[1108]) / p.p1373) / p.p2));
        }

        s.b[2031] = (s.v[1025] < 0.001);
        s.v[2031] = if s.b[2031] { 1.0 } else { 0.0 };

        s.b[2032] = (s.v[1024] <= 0.001);
        s.v[2032] = if s.b[2032] { 1.0 } else { 0.0 };

        if (((((!s.b[2028]) && (!s.b[2029])) && s.b[2030]) && s.b[2031]) && s.b[2032]) {
            s.store_scalar(167, (1.0 / 0.001));
        }

        if (((((!s.b[2028]) && (!s.b[2029])) && s.b[2030]) && s.b[2031]) && (!s.b[2032])) {
            s.store_scalar(167, (1.0 / s.v[1024]));
        }

        if ((((!s.b[2028]) && (!s.b[2029])) && s.b[2030]) && s.b[2031]) {
            s.copy_ad(1021, 167);
        }

        if ((((!s.b[2028]) && (!s.b[2029])) && s.b[2030]) && (!s.b[2031])) {
            s.store_div_from_scalar_offset_input(1021, 1.0, 1025, s.v[1024]);
        }

        if (((!s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) {
            s.store_mul_pow_ad_rhs(1027, 1028, s.ad_value(639), s.ad_value(1029));
            s.store_add_scaled_inputs4(1026, s.ad_value(387), -1.0, s.ad_value(520), -1.0, s.ad_value(525), -1.0, s.ad_value(1039), 1.0);
            s.store_sub_scaled_inputs(1031, 1030, (1.602176462e-19 * (p.p74 * (s.v[1108] * s.v[1109]))), 1026, 1.0);
            s.store_mul(167, 1027, 1031);
            s.store_scalar(168, (s.v[1108] * s.v[1108]));
            s.store_div_scaled_inputs(1023, s.ad_value(167), p.p2, s.ad_value(168), 1.0);
            s.store_div_from_scalar(1025, 1.0, 1023);
        }

        s.b[2033] = (s.v[1025] < 0.001);
        s.v[2033] = if s.b[2033] { 1.0 } else { 0.0 };

        s.b[2034] = (s.v[1024] <= 0.001);
        s.v[2034] = if s.b[2034] { 1.0 } else { 0.0 };

        if (((((!s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && s.b[2033]) && s.b[2034]) {
            s.store_scalar(167, (1.0 / 0.001));
        }

        if (((((!s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && s.b[2033]) && (!s.b[2034])) {
            s.store_scalar(167, (1.0 / s.v[1024]));
        }

        if ((((!s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && s.b[2033]) {
            s.copy_ad(1021, 167);
        }

        if ((((!s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && (!s.b[2033])) {
            s.store_div_from_scalar_offset_input(1021, 1.0, 1025, s.v[1024]);
        }

        s.b[2035] = (p.p1375 == 2.0);
        s.v[2035] = if s.b[2035] { 1.0 } else { 0.0 };

        if s.b[2035] {
            s.store_scale(1021, 1021, 2.0);
        }

        s.b[2036] = (p.p1374 < 0.001);
        s.v[2036] = if s.b[2036] { 1.0 } else { 0.0 };

        if s.b[2036] {
            s.store_scalar(167, (1.0 / 0.001));
            s.copy_ad(1022, 167);
        }

        if (!s.b[2036]) {
            s.store_scalar(1022, (1.0 / p.p1374));
        }

        s.b[2037] = true;
        s.v[2037] = if s.b[2037] { 1.0 } else { 0.0 };

        s.b[2038] = ((p.p40 == 0.0) || true);
        s.v[2038] = if s.b[2038] { 1.0 } else { 0.0 };

        s.b[2039] = ((((p.p43 != 0.0) && true) && (!((p.p40 == 1.0) && (!true)))) && (p.p45 == 1.0));
        s.v[2039] = if s.b[2039] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[45] = 0.0;

        s.v[40] = 0.0;

        s.v[254] = 0.0;

        s.v[295] = 0.0;

        s.v[316] = 0.0;

        s.v[478] = 0.0;

        s.v[839] = 0.0;

        s.v[717] = 0.0;

        s.v[691] = 0.0;

        s.v[779] = 0.0;

        s.v[749] = 0.0;

        s.v[756] = 0.0;

        s.v[754] = 0.0;

        s.v[692] = 0.0;

        s.v[916] = 0.0;

        s.v[928] = 0.0;

        s.v[829] = 0.0;

        s.v[833] = 0.0;

        s.v[841] = 0.0;

        s.v[845] = 0.0;

        s.v[849] = 0.0;

        s.v[853] = 0.0;

        s.v[859] = 0.0;

        s.v[863] = 0.0;

        s.v[731] = 0.0;

        s.v[784] = 0.0;

        s.v[658] = 0.0;

        s.v[644] = 0.0;

        s.v[650] = 0.0;

        s.v[745] = 0.0;

        s.v[936] = 0.0;

        s.v[917] = 0.0;

        s.v[830] = 0.0;

        s.v[836] = 0.0;

        s.v[842] = 0.0;

        s.v[846] = 0.0;

        s.v[850] = 0.0;

        s.v[856] = 0.0;

        s.v[860] = 0.0;

        s.v[864] = 0.0;

        s.v[664] = 0.0;

        s.v[762] = 0.0;

        s.v[739] = 0.0;

        s.v[759] = 0.0;

        s.v[753] = 0.0;

        s.v[654] = 0.0;

        s.v[937] = 0.0;

        s.v[956] = 0.0;

        s.v[958] = 0.0;

        s.v[831] = 0.0;

        s.v[837] = 0.0;

        s.v[843] = 0.0;

        s.v[847] = 0.0;

        s.v[851] = 0.0;

        s.v[857] = 0.0;

        s.v[861] = 0.0;

        s.v[685] = 0.0;

        s.v[347] = 0.0;

        s.v[642] = 0.0;

        s.v[646] = 0.0;

        s.v[648] = 0.0;

        s.v[686] = 0.0;

        s.v[938] = 0.0;

        s.v[957] = 0.0;

        s.v[828] = 0.0;

        s.v[832] = 0.0;

        s.v[840] = 0.0;

        s.v[844] = 0.0;

        s.v[848] = 0.0;

        s.v[852] = 0.0;

        s.v[858] = 0.0;

        s.v[862] = 0.0;

        s.v[854] = 0.0;

        s.v[855] = 0.0;

        s.v[460] = 0.0;

        s.v[459] = 0.0;

        s.v[462] = 0.0;

        s.v[461] = 0.0;

        s.v[1019] = 1.0;

        s.v[1020] = 1.0;

        s.v[87] = 1.0;

        s.v[354] = 0.0;

        s.v[339] = 0.0;

        s.v[458] = 0.0;

        s.v[343] = 0.0;

        s.v[344] = 0.0;

        s.v[534] = 0.0;

        s.v[533] = 0.0;

        s.v[834] = 0.0;

        s.v[363] = 0.0;

        s.v[365] = 0.0;

        s.v[334] = 0.0;

        s.v[455] = 0.0;

        s.v[454] = 0.0;

        s.v[315] = 0.0;

        s.v[355] = 0.0;

        s.v[250] = 0.0;

        s.v[243] = 0.0;

        s.v[73] = 0.0;

        s.v[81] = 0.0;

        s.v[457] = 0.0;

        s.v[1048] = (1.3806503e-23 / 1.602176462e-19);

        s.v[320] = 0.0;

        s.v[400] = 0.0;

        s.v[23] = 0.0;

        s.v[22] = 0.0;

        s.v[323] = 0.0;

        s.v[74] = 0.0;

        s.v[80] = 0.0;

        s.v[84] = 0.0;

        s.v[959] = 0.0;

        s.v[960] = 0.0;

        s.v[961] = 0.0;

        s.b[1129] = (p.p30 == 1.0);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if s.b[1129] {
            s.store_scalar(379, 1.0);
        }

        if (!s.b[1129]) {
            s.store_scalar(379, (-1.0));
        }

        s.v[180] = (p.p109 * 8.8541878128e-12);

        s.v[181] = (p.p110 * 8.8541878128e-12);

        s.v[199] = ((p.p110 * 8.8541878128e-12) / p.p76);

        s.v[200] = (p.p109 / p.p110);

        s.b[1130] = (!param_given[77]);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if s.b[1130] {
            s.store_scalar(429, (((p.p76 * p.p110) / 3.9) - p.p78));
        }

        if (!s.b[1130]) {
            s.store_scalar(429, p.p77);
        }

        s.v[262] = (p.p0 * p.p49);

        s.v[264] = (p.p1 * p.p50);

        s.v[261] = (s.v[262] + p.p51);

        s.v[681] = (s.v[264] / p.p2);

        s.v[263] = (s.v[681] + p.p53);

        s.v[682] = ((s.v[261]) as f64).powf((-p.p58));

        s.v[683] = ((s.v[263]) as f64).powf((-p.p59));

        s.v[684] = (s.v[682] * s.v[683]);

        s.v[192] = (((p.p54 + (p.p55 * s.v[682])) + (p.p56 * s.v[683])) + (p.p57 * s.v[684]));

        s.v[688] = ((s.v[261]) as f64).powf((-p.p64));

        s.v[689] = ((s.v[263]) as f64).powf((-p.p65));

        s.v[690] = (s.v[688] * s.v[689]);

        s.v[193] = (((p.p60 + (p.p61 * s.v[688])) + (p.p62 * s.v[689])) + (p.p63 * s.v[690]));

        s.v[184] = (s.v[261] - (2.0 * s.v[192]));

        s.v[183] = ((s.v[263] - (p.p1375 * p.p1376)) - ((2.0 - p.p1375) * s.v[193]));

        s.v[196] = (((p.p66 + (p.p67 * s.v[682])) + (p.p68 * s.v[683])) + (p.p69 * s.v[684]));

        s.v[197] = (((p.p70 + (p.p71 * s.v[688])) + (p.p72 * s.v[689])) + (p.p73 * s.v[690]));

        s.v[188] = (s.v[261] - (2.0 * s.v[196]));

        s.v[187] = ((s.v[263] - (p.p1375 * p.p1376)) - ((2.0 - p.p1375) * s.v[197]));

        s.v[198] = (((p.p927 + (p.p71 / ((s.v[261]) as f64).powf(p.p64))) + (p.p72 / ((s.v[263]) as f64).powf(p.p65))) + ((p.p73 / ((s.v[261]) as f64).powf(p.p64)) / ((s.v[263]) as f64).powf(p.p65)));

        s.v[189] = (s.v[263] - (2.0 * s.v[198]));

        s.v[694] = (1e-6 / s.v[184]);

        s.v[695] = (1e-6 / s.v[183]);

        s.v[697] = (1e-6 / s.v[188]);

        s.v[698] = (1e-6 / s.v[187]);

        s.v[699] = (1e-6 / p.p48);

        s.v[700] = (1e-6 / p.p52);

        s.v[696] = (s.v[694] * s.v[695]);

        s.v[685] = s.v[682];

        s.v[691] = s.v[688];

        s.b[1142] = (p.p1026 != 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        s.b[1143] = (p.p1026 <= (-s.v[261]));
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if (s.b[1142] && (!s.b[1143])) {
            s.store_scalar(685, (((s.v[261] + p.p1026)) as f64).powf((-p.p58)));
            s.store_scalar(691, (((s.v[261] + p.p1026)) as f64).powf((-p.p64)));
        }

        s.v[686] = s.v[683];

        s.v[692] = s.v[689];

        s.b[1144] = (p.p1027 != 0.0);
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        s.b[1145] = (p.p1027 <= (-s.v[263]));
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if (s.b[1144] && (!s.b[1145])) {
            s.store_scalar(686, (((s.v[263] + p.p1027)) as f64).powf((-p.p59)));
            s.store_scalar(692, (((s.v[263] + p.p1027)) as f64).powf((-p.p65)));
        }

        s.store_mul(687, 685, 686);

        s.store_add_scaled_inputs3_offset(194, s.ad_value(685), p.p55, s.ad_value(686), p.p56, s.ad_value(687), p.p57, p.p54);

        s.store_mul(693, 691, 692);

        s.store_add_scaled_inputs3_offset(195, s.ad_value(691), p.p61, s.ad_value(692), p.p62, s.ad_value(693), p.p63, p.p60);

        s.store_offset_sub_from_scalar_ad(186, s.v[261], A::scale(s.ad_value(194), 2.0), p.p1026);

        s.store_offset_sub_from_scalar_ad(185, s.v[263], A::scale(s.ad_value(195), 2.0), p.p1027);

        s.b[1148] = (p.p1025 == 1.0);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if s.b[1148] {
            s.store_div_from_scalar(701, 1e-6, 186);
            s.store_div_from_scalar(702, 1e-6, 185);
        }

        if (!s.b[1148]) {
            s.store_div_from_scalar(701, 1.0, 186);
            s.store_div_from_scalar(702, 1.0, 185);
        }

        s.store_mul(703, 701, 702);

        s.store_add_scaled_inputs3_offset(707, s.ad_value(701), p.p116, s.ad_value(702), p.p117, s.ad_value(703), p.p118, p.p115);

        s.store_add_scaled_inputs3_offset(708, s.ad_value(701), p.p120, s.ad_value(702), p.p121, s.ad_value(703), p.p122, p.p119);

        s.store_add_scaled_inputs3_offset(793, s.ad_value(701), p.p130, s.ad_value(702), p.p131, s.ad_value(703), p.p132, p.p129);

        s.store_add_scaled_inputs3_offset(705, s.ad_value(701), p.p143, s.ad_value(702), p.p144, s.ad_value(703), p.p145, p.p142);

        s.store_add_scaled_inputs3_offset(706, s.ad_value(701), p.p88, s.ad_value(702), p.p89, s.ad_value(703), p.p90, p.p79);

        s.store_add_scaled_inputs3_offset(794, s.ad_value(701), p.p100, s.ad_value(702), p.p101, s.ad_value(703), p.p102, p.p91);

        s.store_add_scaled_inputs3_offset(704, s.ad_value(701), p.p104, s.ad_value(702), p.p105, s.ad_value(703), p.p106, p.p103);

        s.store_add_scaled_inputs3_offset(709, s.ad_value(701), p.p233, s.ad_value(702), p.p234, s.ad_value(703), p.p235, p.p232);

        s.store_add_scaled_inputs3_offset(720, s.ad_value(701), p.p243, s.ad_value(702), p.p244, s.ad_value(703), p.p245, p.p236);

        s.store_add_scaled_inputs3_offset(721, s.ad_value(701), p.p247, s.ad_value(702), p.p248, s.ad_value(703), p.p249, p.p246);

        s.store_add_scaled_inputs3_offset(722, s.ad_value(701), p.p251, s.ad_value(702), p.p252, s.ad_value(703), p.p253, p.p250);

        s.store_add_scaled_inputs3_offset(725, s.ad_value(701), p.p171, s.ad_value(702), p.p172, s.ad_value(703), p.p173, p.p170);

        s.store_add_scaled_inputs3_offset(726, s.ad_value(701), p.p175, s.ad_value(702), p.p176, s.ad_value(703), p.p177, p.p174);

        s.store_add_scaled_inputs3_offset(724, s.ad_value(701), p.p179, s.ad_value(702), p.p180, s.ad_value(703), p.p181, p.p178);

        s.store_add_scaled_inputs3_offset(728, s.ad_value(701), p.p187, s.ad_value(702), p.p188, s.ad_value(703), p.p189, p.p186);

        s.store_add_scaled_inputs3_offset(727, s.ad_value(701), p.p183, s.ad_value(702), p.p184, s.ad_value(703), p.p185, p.p182);

        s.store_add_scaled_inputs3_offset(723, s.ad_value(701), p.p255, s.ad_value(702), p.p256, s.ad_value(703), p.p257, p.p254);

        s.store_add_scaled_inputs3_offset(710, s.ad_value(701), p.p259, s.ad_value(702), p.p260, s.ad_value(703), p.p261, p.p258);

        s.store_add_scaled_inputs3_offset(714, s.ad_value(701), p.p263, s.ad_value(702), p.p264, s.ad_value(703), p.p265, p.p262);

        s.store_add_scaled_inputs3_offset(715, s.ad_value(701), p.p1165, s.ad_value(702), p.p1166, s.ad_value(703), p.p1167, p.p1164);

        s.store_add_scaled_inputs3_offset(716, s.ad_value(701), p.p1192, s.ad_value(702), p.p1193, s.ad_value(703), p.p1194, p.p1191);

        s.store_add_scaled_inputs3_offset(719, s.ad_value(701), p.p291, s.ad_value(702), p.p292, s.ad_value(703), p.p293, p.p288);

        s.store_add_scaled_inputs3_offset(711, s.ad_value(701), p.p271, s.ad_value(702), p.p272, s.ad_value(703), p.p273, p.p270);

        s.store_add_scaled_inputs3_offset(712, s.ad_value(701), p.p1177, s.ad_value(702), p.p1178, s.ad_value(703), p.p1179, p.p1176);

        s.store_add_scaled_inputs3_offset(713, s.ad_value(701), p.p276, s.ad_value(702), p.p277, s.ad_value(703), p.p278, p.p275);

        s.store_add_scaled_inputs3_offset(279, s.ad_value(701), p.p147, s.ad_value(702), p.p148, s.ad_value(703), p.p149, p.p146);

        s.store_add_scaled_inputs3_offset(280, s.ad_value(701), p.p1239, s.ad_value(702), p.p1240, s.ad_value(703), p.p1241, p.p1238);

        s.store_add_scaled_inputs3_offset(281, s.ad_value(701), p.p151, s.ad_value(702), p.p152, s.ad_value(703), p.p153, p.p150);

        s.store_add_scaled_inputs3_offset(282, s.ad_value(701), p.p1243, s.ad_value(702), p.p1244, s.ad_value(703), p.p1245, p.p1242);

        s.store_add_scaled_inputs3_offset(283, s.ad_value(701), p.p155, s.ad_value(702), p.p156, s.ad_value(703), p.p157, p.p154);

        s.store_add_scaled_inputs3_offset(285, s.ad_value(701), p.p159, s.ad_value(702), p.p160, s.ad_value(703), p.p161, p.p158);

        s.store_add_scaled_inputs3_offset(287, s.ad_value(701), p.p163, s.ad_value(702), p.p164, s.ad_value(703), p.p165, p.p162);

        s.store_add_scaled_inputs3_offset(289, s.ad_value(701), p.p167, s.ad_value(702), p.p168, s.ad_value(703), p.p169, p.p166);

        s.store_add_scaled_inputs3_offset(284, s.ad_value(701), p.p1247, s.ad_value(702), p.p1248, s.ad_value(703), p.p1249, p.p1246);

        s.store_add_scaled_inputs3_offset(286, s.ad_value(701), p.p1251, s.ad_value(702), p.p1252, s.ad_value(703), p.p1253, p.p1250);

        s.store_add_scaled_inputs3_offset(288, s.ad_value(701), p.p1255, s.ad_value(702), p.p1256, s.ad_value(703), p.p1257, p.p1254);

        s.store_add_scaled_inputs3_offset(290, s.ad_value(701), p.p1259, s.ad_value(702), p.p1260, s.ad_value(703), p.p1261, p.p1258);

        s.store_add_scaled_inputs3_offset(734, s.ad_value(701), p.p225, s.ad_value(702), p.p226, s.ad_value(703), p.p227, p.p218);

        s.store_add_scaled_inputs3_offset(735, s.ad_value(701), p.p215, s.ad_value(702), p.p216, s.ad_value(703), p.p217, p.p208);

        s.store_add_scaled_inputs3_offset(736, s.ad_value(701), p.p1203, s.ad_value(702), p.p1204, s.ad_value(703), p.p1205, p.p1196);

        s.store_add_scaled_inputs3_offset(782, s.ad_value(701), p.p112, s.ad_value(702), p.p113, s.ad_value(703), p.p114, p.p111);

        s.store_add_scaled_inputs3_offset(729, s.ad_value(701), p.p191, s.ad_value(702), p.p192, s.ad_value(703), p.p193, p.p190);

        s.store_add_scaled_inputs3_offset(730, s.ad_value(701), p.p195, s.ad_value(702), p.p196, s.ad_value(703), p.p197, p.p194);

        s.store_add_scaled_inputs3_offset(733, s.ad_value(701), p.p205, s.ad_value(702), p.p206, s.ad_value(703), p.p207, p.p203);

        s.store_add_scaled_inputs3_offset(737, s.ad_value(701), p.p310, s.ad_value(702), p.p311, s.ad_value(703), p.p312, p.p309);

        s.store_add_scaled_inputs3_offset(738, s.ad_value(701), p.p340, s.ad_value(702), p.p341, s.ad_value(703), p.p342, p.p337);

        s.store_add_scaled_inputs3_offset(748, s.ad_value(701), p.p355, s.ad_value(702), p.p356, s.ad_value(703), p.p357, p.p348);

        s.store_add_scaled_inputs3_offset(752, s.ad_value(701), p.p375, s.ad_value(702), p.p376, s.ad_value(703), p.p377, p.p372);

        s.store_add_scaled_inputs3_offset(751, s.ad_value(701), p.p363, s.ad_value(702), p.p364, s.ad_value(703), p.p365, p.p362);

        s.store_add_scaled_inputs3_offset(755, s.ad_value(701), p.p383, s.ad_value(702), p.p384, s.ad_value(703), p.p385, p.p382);

        s.store_add_scaled_inputs3_offset(758, s.ad_value(701), p.p397, s.ad_value(702), p.p398, s.ad_value(703), p.p399, p.p390);

        s.store_add_scaled_inputs3_offset(783, s.ad_value(701), p.p407, s.ad_value(702), p.p408, s.ad_value(703), p.p409, p.p404);

        s.store_add_scaled_inputs3_offset(786, s.ad_value(701), p.p418, s.ad_value(702), p.p419, s.ad_value(703), p.p420, p.p415);

        s.store_add_scaled_inputs3_offset(775, s.ad_value(701), p.p458, s.ad_value(702), p.p459, s.ad_value(703), p.p460, p.p457);

        s.store_add_scaled_inputs3_offset(774, s.ad_value(701), p.p468, s.ad_value(702), p.p469, s.ad_value(703), p.p470, p.p467);

        s.store_add_scaled_inputs3_offset(770, s.ad_value(701), p.p440, s.ad_value(702), p.p441, s.ad_value(703), p.p442, p.p439);

        s.store_add_scaled_inputs3_offset(787, s.ad_value(701), p.p444, s.ad_value(702), p.p445, s.ad_value(703), p.p446, p.p443);

        s.store_add_scaled_inputs3_offset(771, s.ad_value(701), p.p450, s.ad_value(702), p.p451, s.ad_value(703), p.p452, p.p449);

        s.store_add_scaled_inputs3_offset(773, s.ad_value(701), p.p454, s.ad_value(702), p.p455, s.ad_value(703), p.p456, p.p453);

        s.store_add_scaled_inputs3_offset(772, s.ad_value(701), p.p464, s.ad_value(702), p.p465, s.ad_value(703), p.p466, p.p463);

        s.store_add_scaled_inputs3_offset(776, s.ad_value(701), p.p480, s.ad_value(702), p.p481, s.ad_value(703), p.p482, p.p477);

        s.store_add_scaled_inputs3_offset(777, s.ad_value(701), p.p474, s.ad_value(702), p.p475, s.ad_value(703), p.p476, p.p473);

        s.store_add_scaled_inputs3_offset(778, s.ad_value(701), p.p499, s.ad_value(702), p.p500, s.ad_value(703), p.p501, p.p498);

        s.store_add_scaled_inputs3_offset(761, s.ad_value(701), p.p533, s.ad_value(702), p.p534, s.ad_value(703), p.p535, p.p530);

        s.store_add_scaled_inputs3_offset(764, s.ad_value(701), p.p541, s.ad_value(702), p.p542, s.ad_value(703), p.p543, p.p540);

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset(765, s.ad_value(701), p.p422, s.ad_value(702), p.p423, s.ad_value(703), p.p424, p.p421);

        s.store_add_scaled_inputs3_offset(766, s.ad_value(701), p.p426, s.ad_value(702), p.p427, s.ad_value(703), p.p428, p.p425);

        s.store_add_scaled_inputs3_offset(767, s.ad_value(701), p.p430, s.ad_value(702), p.p431, s.ad_value(703), p.p432, p.p429);

        s.store_add_scaled_inputs3_offset(768, s.ad_value(701), p.p435, s.ad_value(702), p.p436, s.ad_value(703), p.p437, p.p434);

        s.store_add_scaled_inputs3_offset(769, s.ad_value(701), p.p551, s.ad_value(702), p.p552, s.ad_value(703), p.p553, p.p548);

        s.store_add_scaled_inputs3_offset(781, s.ad_value(701), p.p545, s.ad_value(702), p.p546, s.ad_value(703), p.p547, p.p544);

        s.store_add_scaled_inputs3_offset(741, s.ad_value(701), p.p296, s.ad_value(702), p.p297, s.ad_value(703), p.p298, p.p295);

        s.store_add_scaled_inputs3_offset(742, s.ad_value(701), p.p511, s.ad_value(702), p.p512, s.ad_value(703), p.p513, p.p510);

        s.store_add_scaled_inputs3_offset(744, s.ad_value(701), p.p326, s.ad_value(702), p.p327, s.ad_value(703), p.p328, p.p325);

        s.store_add_scaled_inputs3_offset(743, s.ad_value(701), p.p330, s.ad_value(702), p.p331, s.ad_value(703), p.p332, p.p329);

        s.store_add_scaled_inputs3_offset(346, s.ad_value(701), p.p484, s.ad_value(702), p.p485, s.ad_value(703), p.p486, p.p483);

        s.store_add_scaled_inputs3_offset(747, s.ad_value(701), p.p316, s.ad_value(702), p.p317, s.ad_value(703), p.p318, p.p315);

        s.store_add_scaled_inputs3_offset(788, s.ad_value(701), p.p868, s.ad_value(702), p.p869, s.ad_value(703), p.p870, p.p867);

        s.store_add_scaled_inputs3_offset(789, s.ad_value(701), p.p876, s.ad_value(702), p.p877, s.ad_value(703), p.p878, p.p875);

        s.store_add_scaled_inputs3_offset(790, s.ad_value(701), p.p880, s.ad_value(702), p.p881, s.ad_value(703), p.p882, p.p879);

        s.store_add_scaled_inputs3_offset(791, s.ad_value(701), p.p884, s.ad_value(702), p.p885, s.ad_value(703), p.p886, p.p883);

        s.store_add_scaled_inputs3_offset(792, s.ad_value(701), p.p888, s.ad_value(702), p.p889, s.ad_value(703), p.p890, p.p887);

        s.store_add_scaled_inputs3_offset(865, s.ad_value(701), p.p604, s.ad_value(702), p.p605, s.ad_value(703), p.p606, p.p601);

        s.store_add_scaled_inputs3_offset(866, s.ad_value(701), p.p608, s.ad_value(702), p.p609, s.ad_value(703), p.p610, p.p607);

        s.store_add_scaled_inputs3_offset(867, s.ad_value(701), p.p612, s.ad_value(702), p.p613, s.ad_value(703), p.p614, p.p611);

        s.store_add_scaled_inputs3_offset(868, s.ad_value(701), p.p616, s.ad_value(702), p.p617, s.ad_value(703), p.p618, p.p615);

        s.store_add_scaled_inputs3_offset(869, s.ad_value(701), p.p620, s.ad_value(702), p.p621, s.ad_value(703), p.p622, p.p619);

        s.store_add_scaled_inputs3_offset(870, s.ad_value(701), p.p624, s.ad_value(702), p.p625, s.ad_value(703), p.p626, p.p623);

        s.store_add_scaled_inputs3_offset(871, s.ad_value(701), p.p628, s.ad_value(702), p.p629, s.ad_value(703), p.p630, p.p627);

        s.store_add_scaled_inputs3_offset(872, s.ad_value(701), p.p632, s.ad_value(702), p.p633, s.ad_value(703), p.p634, p.p631);

        s.store_add_scaled_inputs3_offset(873, s.ad_value(701), p.p636, s.ad_value(702), p.p637, s.ad_value(703), p.p638, p.p635);

        s.store_add_scaled_inputs3_offset(874, s.ad_value(701), p.p597, s.ad_value(702), p.p598, s.ad_value(703), p.p599, p.p596);

        s.store_add_scaled_inputs3_offset(875, s.ad_value(701), p.p640, s.ad_value(702), p.p641, s.ad_value(703), p.p642, p.p639);

        s.store_add_scaled_inputs3_offset(877, s.ad_value(701), p.p655, s.ad_value(702), p.p658, s.ad_value(703), p.p661, p.p650);

        s.store_add_scaled_inputs3_offset(878, s.ad_value(701), p.p654, s.ad_value(702), p.p657, s.ad_value(703), p.p660, p.p651);

        s.store_add_scaled_inputs3_offset(879, s.ad_value(701), p.p653, s.ad_value(702), p.p656, s.ad_value(703), p.p659, p.p652);

        s.store_add_scaled_inputs3_offset(880, s.ad_value(701), p.p663, s.ad_value(702), p.p664, s.ad_value(703), p.p665, p.p662);

        s.store_add_scaled_inputs3_offset(881, s.ad_value(701), p.p668, s.ad_value(702), p.p669, s.ad_value(703), p.p670, p.p667);

        s.store_add_scaled_inputs3_offset(1028, s.ad_value(701), p.p1362, s.ad_value(702), p.p1363, s.ad_value(703), p.p1364, p.p1361);

        s.store_add_scaled_inputs3_offset(1029, s.ad_value(701), p.p1366, s.ad_value(702), p.p1367, s.ad_value(703), p.p1368, p.p1365);

        s.store_add_scaled_inputs3_offset(1030, s.ad_value(701), p.p1370, s.ad_value(702), p.p1371, s.ad_value(703), p.p1372, p.p1369);

        s.store_add_scaled_inputs3_offset(547, s.ad_value(701), p.p929, s.ad_value(702), p.p930, s.ad_value(703), p.p931, p.p928);

        s.store_add_scaled_inputs3_offset(550, s.ad_value(701), p.p934, s.ad_value(702), p.p936, s.ad_value(703), p.p938, p.p932);

        s.store_add_scaled_inputs3_offset(551, s.ad_value(701), p.p935, s.ad_value(702), p.p937, s.ad_value(703), p.p939, p.p933);

        s.store_add_scaled_inputs3_offset(557, s.ad_value(701), p.p941, s.ad_value(702), p.p942, s.ad_value(703), p.p943, p.p940);

        s.store_add_scaled_inputs3_offset(564, s.ad_value(701), p.p945, s.ad_value(702), p.p946, s.ad_value(703), p.p947, p.p944);

        s.store_add_scaled_inputs3_offset(556, s.ad_value(701), p.p949, s.ad_value(702), p.p950, s.ad_value(703), p.p951, p.p948);

        s.store_add_scaled_inputs3_offset(552, s.ad_value(701), p.p954, s.ad_value(702), p.p956, s.ad_value(703), p.p958, p.p952);

        s.store_add_scaled_inputs3_offset(553, s.ad_value(701), p.p955, s.ad_value(702), p.p957, s.ad_value(703), p.p959, p.p953);

        s.store_add_scaled_inputs3_offset(565, s.ad_value(701), p.p962, s.ad_value(702), p.p964, s.ad_value(703), p.p966, p.p960);

        s.store_add_scaled_inputs3_offset(566, s.ad_value(701), p.p963, s.ad_value(702), p.p965, s.ad_value(703), p.p967, p.p961);

        s.store_add_scaled_inputs3_offset(567, s.ad_value(701), p.p970, s.ad_value(702), p.p972, s.ad_value(703), p.p974, p.p968);

        s.store_add_scaled_inputs3_offset(568, s.ad_value(701), p.p971, s.ad_value(702), p.p973, s.ad_value(703), p.p975, p.p969);

        s.store_add_scaled_inputs3_offset(569, s.ad_value(701), p.p978, s.ad_value(702), p.p980, s.ad_value(703), p.p982, p.p976);

        s.store_add_scaled_inputs3_offset(570, s.ad_value(701), p.p979, s.ad_value(702), p.p981, s.ad_value(703), p.p983, p.p977);

        s.store_add_scaled_inputs3_offset(573, s.ad_value(701), p.p986, s.ad_value(702), p.p988, s.ad_value(703), p.p990, p.p984);

        s.store_add_scaled_inputs3_offset(574, s.ad_value(701), p.p987, s.ad_value(702), p.p989, s.ad_value(703), p.p991, p.p985);

        s.store_add_scaled_inputs3_offset(575, s.ad_value(701), p.p994, s.ad_value(702), p.p996, s.ad_value(703), p.p998, p.p992);

        s.store_add_scaled_inputs3_offset(576, s.ad_value(701), p.p995, s.ad_value(702), p.p997, s.ad_value(703), p.p999, p.p993);

        s.store_add_scaled_inputs3_offset(558, s.ad_value(701), p.p1002, s.ad_value(702), p.p1004, s.ad_value(703), p.p1006, p.p1000);

        s.store_add_scaled_inputs3_offset(559, s.ad_value(701), p.p1003, s.ad_value(702), p.p1005, s.ad_value(703), p.p1007, p.p1001);

        s.store_add_scaled_inputs3_offset(581, s.ad_value(701), p.p556, s.ad_value(702), p.p557, s.ad_value(703), p.p558, p.p555);

        s.store_add_scaled_inputs3_offset(582, s.ad_value(701), p.p560, s.ad_value(702), p.p561, s.ad_value(703), p.p562, p.p559);

        s.store_add_scaled_inputs3_offset(587, s.ad_value(701), p.p565, s.ad_value(702), p.p567, s.ad_value(703), p.p569, p.p563);

        s.store_add_scaled_inputs3_offset(588, s.ad_value(701), p.p566, s.ad_value(702), p.p568, s.ad_value(703), p.p570, p.p564);

        s.store_add_scaled_inputs3_offset(589, s.ad_value(701), p.p572, s.ad_value(702), p.p573, s.ad_value(703), p.p574, p.p571);

        s.store_add_scaled_inputs3_offset(590, s.ad_value(701), p.p576, s.ad_value(702), p.p577, s.ad_value(703), p.p578, p.p575);

        s.store_add_scaled_inputs3_offset(598, s.ad_value(701), p.p582, s.ad_value(702), p.p581, s.ad_value(703), p.p580, p.p579);

        s.store_add_scaled_inputs3_offset(597, s.ad_value(701), p.p584, s.ad_value(702), p.p585, s.ad_value(703), p.p586, p.p583);

        s.store_add_scaled_inputs3_offset(600, s.ad_value(701), p.p588, s.ad_value(702), p.p590, s.ad_value(703), p.p592, p.p587);

        s.store_add_scaled_inputs3_offset(601, s.ad_value(701), p.p589, s.ad_value(702), p.p591, s.ad_value(703), p.p593, p.p594);

        s.store_add_scaled_inputs3_offset(530, s.ad_value(701), p.p922, s.ad_value(702), p.p923, s.ad_value(703), p.p924, p.p921);

        s.store_add_scaled_inputs3_offset(806, s.ad_value(701), p.p1126, s.ad_value(702), p.p1127, s.ad_value(703), p.p1128, p.p1125);

        s.store_add_scaled_inputs3_offset(807, s.ad_value(701), p.p1130, s.ad_value(702), p.p1131, s.ad_value(703), p.p1132, p.p1129);

        s.store_add_scaled_inputs3_offset(808, s.ad_value(701), p.p1134, s.ad_value(702), p.p1135, s.ad_value(703), p.p1136, p.p1133);

        s.store_add_scaled_inputs3_offset(892, s.ad_value(701), p.p802, s.ad_value(702), p.p803, s.ad_value(703), p.p804, p.p799);

        s.store_add_scaled_inputs3_offset(893, s.ad_value(701), p.p807, s.ad_value(702), p.p808, s.ad_value(703), p.p809, p.p805);

        s.store_add_scaled_inputs3_offset(900, s.ad_value(701), p.p810, s.ad_value(702), p.p811, s.ad_value(703), p.p812, p.p806);

        s.store_add_scaled_inputs3_offset(894, s.ad_value(701), p.p814, s.ad_value(702), p.p815, s.ad_value(703), p.p816, p.p813);

        s.store_add_scaled_inputs3_offset(895, s.ad_value(701), p.p818, s.ad_value(702), p.p819, s.ad_value(703), p.p820, p.p817);

        s.store_add_scaled_inputs3_offset(896, s.ad_value(701), p.p824, s.ad_value(702), p.p825, s.ad_value(703), p.p826, p.p821);

        s.store_add_scaled_inputs3_offset(897, s.ad_value(701), p.p829, s.ad_value(702), p.p830, s.ad_value(703), p.p831, p.p827);

        s.store_add_scaled_inputs3_offset(901, s.ad_value(701), p.p832, s.ad_value(702), p.p833, s.ad_value(703), p.p834, p.p828);

        s.store_add_scaled_inputs3_offset(898, s.ad_value(701), p.p836, s.ad_value(702), p.p837, s.ad_value(703), p.p838, p.p835);

        s.store_add_scaled_inputs3_offset(899, s.ad_value(701), p.p840, s.ad_value(702), p.p841, s.ad_value(703), p.p842, p.p839);

        s.store_add_scaled_inputs3_offset(905, s.ad_value(701), p.p856, s.ad_value(702), p.p857, s.ad_value(703), p.p858, p.p855);

        s.store_add_scaled_inputs3_offset(902, s.ad_value(701), p.p844, s.ad_value(702), p.p845, s.ad_value(703), p.p846, p.p843);

        s.store_add_scaled_inputs3_offset(906, s.ad_value(701), p.p860, s.ad_value(702), p.p861, s.ad_value(703), p.p862, p.p859);

        s.store_add_scaled_inputs3_offset(903, s.ad_value(701), p.p848, s.ad_value(702), p.p849, s.ad_value(703), p.p850, p.p847);

        s.store_add_scaled_inputs3_offset(907, s.ad_value(701), p.p864, s.ad_value(702), p.p865, s.ad_value(703), p.p866, p.p863);

        s.store_add_scaled_inputs3_offset(904, s.ad_value(701), p.p852, s.ad_value(702), p.p853, s.ad_value(703), p.p854, p.p851);

        s.store_add_scaled_inputs3_offset(796, s.ad_value(701), p.p1033, s.ad_value(702), p.p1034, s.ad_value(703), p.p1035, p.p1032);

        s.store_add_scaled_inputs3_offset(797, s.ad_value(701), p.p1038, s.ad_value(702), p.p1039, s.ad_value(703), p.p1040, p.p1037);

        s.store_add_scaled_inputs3_offset(798, s.ad_value(701), p.p1043, s.ad_value(702), p.p1044, s.ad_value(703), p.p1045, p.p1042);

        s.store_add_scaled_inputs3_offset(799, s.ad_value(701), p.p1047, s.ad_value(702), p.p1048, s.ad_value(703), p.p1049, p.p1046);

        s.store_add_scaled_inputs3_offset(805, s.ad_value(701), p.p1052, s.ad_value(702), p.p1053, s.ad_value(703), p.p1054, p.p1051);

        s.store_add_scaled_inputs3_offset(800, s.ad_value(701), p.p1056, s.ad_value(702), p.p1057, s.ad_value(703), p.p1058, p.p1055);

        s.store_add_scaled_inputs3_offset(801, s.ad_value(701), p.p1061, s.ad_value(702), p.p1062, s.ad_value(703), p.p1063, p.p1060);

        s.store_add_scaled_inputs3_offset(802, s.ad_value(701), p.p1065, s.ad_value(702), p.p1066, s.ad_value(703), p.p1067, p.p1064);

        s.store_add_scaled_inputs3_offset(803, s.ad_value(701), p.p1071, s.ad_value(702), p.p1072, s.ad_value(703), p.p1073, p.p1070);

        s.store_add_scaled_inputs3_offset(804, s.ad_value(701), p.p1086, s.ad_value(702), p.p1087, s.ad_value(703), p.p1088, p.p1085);

        s.store_add_scaled_inputs3_offset(809, s.ad_value(701), p.p732, s.ad_value(702), p.p733, s.ad_value(703), p.p734, p.p706);

        s.store_add_scaled_inputs3_offset(882, s.ad_value(701), p.p685, s.ad_value(702), p.p686, s.ad_value(703), p.p687, p.p684);

        s.store_add_scaled_inputs3_offset(887, s.ad_value(701), p.p689, s.ad_value(702), p.p690, s.ad_value(703), p.p691, p.p688);

        s.store_add_scaled_inputs3_offset(883, s.ad_value(701), p.p693, s.ad_value(702), p.p694, s.ad_value(703), p.p695, p.p692);

        s.store_add_scaled_inputs3_offset(884, s.ad_value(701), p.p673, s.ad_value(702), p.p674, s.ad_value(703), p.p675, p.p672);

        s.store_add_scaled_inputs3_offset(886, s.ad_value(701), p.p677, s.ad_value(702), p.p678, s.ad_value(703), p.p679, p.p676);

        s.store_add_scaled_inputs3_offset(885, s.ad_value(701), p.p681, s.ad_value(702), p.p682, s.ad_value(703), p.p683, p.p680);

        s.store_add_scaled_inputs3_offset(810, s.ad_value(701), p.p735, s.ad_value(702), p.p737, s.ad_value(703), p.p739, p.p707);

        s.store_add_scaled_inputs3_offset(813, s.ad_value(701), p.p736, s.ad_value(702), p.p738, s.ad_value(703), p.p740, p.p726);

        s.store_add_scaled_inputs3_offset(811, s.ad_value(701), p.p741, s.ad_value(702), p.p742, s.ad_value(703), p.p743, p.p708);

        s.store_add_scaled_inputs3_offset(812, s.ad_value(701), p.p744, s.ad_value(702), p.p745, s.ad_value(703), p.p746, p.p709);

        s.store_add_scaled_inputs3_offset(816, s.ad_value(701), p.p747, s.ad_value(702), p.p749, s.ad_value(703), p.p751, p.p710);

        s.store_add_scaled_inputs3_offset(814, s.ad_value(701), p.p748, s.ad_value(702), p.p750, s.ad_value(703), p.p752, p.p711);

        s.store_add_scaled_inputs3_offset(817, s.ad_value(701), p.p753, s.ad_value(702), p.p754, s.ad_value(703), p.p755, p.p712);

        s.store_add_scaled_inputs3_offset(818, s.ad_value(701), p.p756, s.ad_value(702), p.p757, s.ad_value(703), p.p758, p.p713);

        s.store_add_scaled_inputs3_offset(819, s.ad_value(701), p.p759, s.ad_value(702), p.p761, s.ad_value(703), p.p763, p.p714);

        s.store_add_scaled_inputs3_offset(815, s.ad_value(701), p.p760, s.ad_value(702), p.p762, s.ad_value(703), p.p764, p.p715);

        s.store_add_scaled_inputs3_offset(820, s.ad_value(701), p.p765, s.ad_value(702), p.p766, s.ad_value(703), p.p767, p.p716);

        s.store_add_scaled_inputs3_offset(821, s.ad_value(701), p.p768, s.ad_value(702), p.p769, s.ad_value(703), p.p770, p.p717);

        s.store_add_scaled_inputs3_offset(822, s.ad_value(701), p.p771, s.ad_value(702), p.p772, s.ad_value(703), p.p773, p.p720);

        s.store_add_scaled_inputs3_offset(826, s.ad_value(701), p.p780, s.ad_value(702), p.p781, s.ad_value(703), p.p782, p.p721);

        s.store_add_scaled_inputs3_offset(679, s.ad_value(701), p.p1078, s.ad_value(702), p.p1079, s.ad_value(703), p.p1080, p.p1075);

        s.store_add_scaled_inputs3_offset(680, s.ad_value(701), p.p1082, s.ad_value(702), p.p1083, s.ad_value(703), p.p1084, p.p1081);

        s.store_add_scaled_inputs3_offset(678, s.ad_value(701), p.p494, s.ad_value(702), p.p495, s.ad_value(703), p.p496, p.p489);

        s.store_add_scaled_inputs3_offset(328, s.ad_value(701), p.p515, s.ad_value(702), p.p516, s.ad_value(703), p.p517, p.p514);

        s.store_add_scaled_inputs3_offset(329, s.ad_value(701), p.p519, s.ad_value(702), p.p520, s.ad_value(703), p.p521, p.p518);

        s.store_add_scaled_inputs3_offset(331, s.ad_value(701), p.p523, s.ad_value(702), p.p524, s.ad_value(703), p.p525, p.p522);

        s.store_add_scaled_inputs3_offset(332, s.ad_value(701), p.p527, s.ad_value(702), p.p528, s.ad_value(703), p.p529, p.p526);

        s.store_add_scaled_inputs3_offset(828, s.ad_value(701), p.p1301, s.ad_value(702), p.p1302, s.ad_value(703), p.p1303, p.p1300);

        s.store_add_scaled_inputs3_offset(829, s.ad_value(701), p.p1309, s.ad_value(702), p.p1310, s.ad_value(703), p.p1311, p.p1308);

        s.store_add_scaled_inputs3_offset(830, s.ad_value(701), p.p1305, s.ad_value(702), p.p1306, s.ad_value(703), p.p1307, p.p1304);

        s.store_add_scaled_inputs3_offset(831, s.ad_value(701), p.p1313, s.ad_value(702), p.p1314, s.ad_value(703), p.p1315, p.p1312);

        s.store_add_scaled_inputs3_offset(835, s.ad_value(701), p.p1157, s.ad_value(702), p.p1158, s.ad_value(703), p.p1159, p.p1156);

        s.store_add_scaled_inputs3_offset(953, s.ad_value(701), p.p1153, s.ad_value(702), p.p1154, s.ad_value(703), p.p1155, p.p1152);

        s.store_add_scaled_inputs3_offset(836, s.ad_value(701), p.p1161, s.ad_value(702), p.p1162, s.ad_value(703), p.p1163, p.p1160);

        s.store_add_scaled_inputs3_offset(837, s.ad_value(701), p.p1169, s.ad_value(702), p.p1170, s.ad_value(703), p.p1171, p.p1168);

        s.store_add_scaled_inputs3_offset(840, s.ad_value(701), p.p1187, s.ad_value(702), p.p1188, s.ad_value(703), p.p1189, p.p1186);

        s.store_add_scaled_inputs3_offset(841, s.ad_value(701), p.p1207, s.ad_value(702), p.p1208, s.ad_value(703), p.p1209, p.p1206);

        s.store_add_scaled_inputs3_offset(842, s.ad_value(701), p.p1211, s.ad_value(702), p.p1212, s.ad_value(703), p.p1213, p.p1210);

        s.store_add_scaled_inputs3_offset(843, s.ad_value(701), p.p1215, s.ad_value(702), p.p1216, s.ad_value(703), p.p1217, p.p1214);

        s.store_add_scaled_inputs3_offset(844, s.ad_value(701), p.p1219, s.ad_value(702), p.p1220, s.ad_value(703), p.p1221, p.p1218);

        s.store_add_scaled_inputs3_offset(845, s.ad_value(701), p.p1223, s.ad_value(702), p.p1224, s.ad_value(703), p.p1225, p.p1222);

        s.store_add_scaled_inputs3_offset(846, s.ad_value(701), p.p1227, s.ad_value(702), p.p1228, s.ad_value(703), p.p1229, p.p1226);

        s.store_add_scaled_inputs3_offset(847, s.ad_value(701), p.p1231, s.ad_value(702), p.p1232, s.ad_value(703), p.p1233, p.p1230);

        s.store_add_scaled_inputs3_offset(848, s.ad_value(701), p.p1235, s.ad_value(702), p.p1236, s.ad_value(703), p.p1237, p.p1234);

        s.store_add_scaled_inputs3_offset(849, s.ad_value(701), p.p1272, s.ad_value(702), p.p1273, s.ad_value(703), p.p1274, p.p1265);

        s.store_add_scaled_inputs3_offset(850, s.ad_value(701), p.p1276, s.ad_value(702), p.p1277, s.ad_value(703), p.p1278, p.p1275);

        s.store_add_scaled_inputs3_offset(854, s.ad_value(701), p.p1284, s.ad_value(702), p.p1285, s.ad_value(703), p.p1286, p.p1283);

        s.store_add_scaled_inputs3_offset(855, s.ad_value(701), p.p1280, s.ad_value(702), p.p1281, s.ad_value(703), p.p1282, p.p1279);

        s.store_add_scaled_inputs3_offset(851, s.ad_value(701), p.p1288, s.ad_value(702), p.p1289, s.ad_value(703), p.p1290, p.p1287);

        s.store_add_scaled_inputs3_offset(852, s.ad_value(701), p.p1292, s.ad_value(702), p.p1293, s.ad_value(703), p.p1294, p.p1291);

        s.store_add_scaled_inputs3_offset(856, s.ad_value(701), p.p1324, s.ad_value(702), p.p1325, s.ad_value(703), p.p1326, p.p1323);

        s.store_add_scaled_inputs3_offset(857, s.ad_value(701), p.p1328, s.ad_value(702), p.p1329, s.ad_value(703), p.p1330, p.p1327);

        s.store_add_scaled_inputs3_offset(859, s.ad_value(701), p.p1332, s.ad_value(702), p.p1333, s.ad_value(703), p.p1334, p.p1331);

        s.store_add_scaled_inputs3_offset(860, s.ad_value(701), p.p1336, s.ad_value(702), p.p1337, s.ad_value(703), p.p1338, p.p1335);

        s.store_add_scaled_inputs3_offset(862, s.ad_value(701), p.p1340, s.ad_value(702), p.p1341, s.ad_value(703), p.p1342, p.p1339);

        s.store_add_scaled_inputs3_offset(863, s.ad_value(701), p.p1344, s.ad_value(702), p.p1345, s.ad_value(703), p.p1346, p.p1343);

        s.store_add_scaled_inputs3_offset(888, s.ad_value(701), p.p787, s.ad_value(702), p.p791, s.ad_value(703), p.p795, p.p783);

        s.store_add_scaled_inputs3_offset(891, s.ad_value(701), p.p788, s.ad_value(702), p.p792, s.ad_value(703), p.p796, p.p784);

        s.store_add_scaled_inputs3_offset(889, s.ad_value(701), p.p789, s.ad_value(702), p.p793, s.ad_value(703), p.p797, p.p785);

        s.store_add_scaled_inputs3_offset(890, s.ad_value(701), p.p790, s.ad_value(702), p.p794, s.ad_value(703), p.p798, p.p786);

        s.store_add_scaled_inputs3_offset(908, s.ad_value(701), p.p1385, s.ad_value(702), p.p1386, s.ad_value(703), p.p1387, p.p1384);

        s.store_add_scaled_inputs3_offset(909, s.ad_value(701), p.p1390, s.ad_value(702), p.p1391, s.ad_value(703), p.p1392, p.p1389);

        s.b[1149] = (p.p35 != 0.0);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if s.b[1149] {
            s.store_add_scaled_inputs3_offset(839, s.ad_value(701), p.p1173, s.ad_value(702), p.p1174, s.ad_value(703), p.p1175, p.p1172);
            s.store_add_scaled_inputs3_offset(717, s.ad_value(701), p.p285, s.ad_value(702), p.p286, s.ad_value(703), p.p287, p.p284);
            s.store_add_scaled_inputs3_offset(731, s.ad_value(701), p.p199, s.ad_value(702), p.p200, s.ad_value(703), p.p201, p.p198);
            s.store_add_scaled_inputs3_offset(739, s.ad_value(701), p.p344, s.ad_value(702), p.p345, s.ad_value(703), p.p346, p.p343);
            s.store_add_scaled_inputs3_offset(749, s.ad_value(701), p.p359, s.ad_value(702), p.p360, s.ad_value(703), p.p361, p.p358);
            s.store_add_scaled_inputs3_offset(753, s.ad_value(701), p.p379, s.ad_value(702), p.p380, s.ad_value(703), p.p381, p.p378);
            s.store_add_scaled_inputs3_offset(756, s.ad_value(701), p.p387, s.ad_value(702), p.p388, s.ad_value(703), p.p389, p.p386);
            s.store_add_scaled_inputs3_offset(759, s.ad_value(701), p.p401, s.ad_value(702), p.p402, s.ad_value(703), p.p403, p.p400);
            s.store_add_scaled_inputs3_offset(784, s.ad_value(701), p.p411, s.ad_value(702), p.p412, s.ad_value(703), p.p413, p.p410);
            s.store_add_scaled_inputs3_offset(762, s.ad_value(701), p.p537, s.ad_value(702), p.p538, s.ad_value(703), p.p539, p.p536);
            s.store_add_scaled_inputs3_offset(745, s.ad_value(701), p.p306, s.ad_value(702), p.p307, s.ad_value(703), p.p308, p.p305);
            s.store_add_scaled_inputs3_offset(347, s.ad_value(701), p.p491, s.ad_value(702), p.p492, s.ad_value(703), p.p493, p.p490);
            s.store_add_scaled_inputs3_offset(779, s.ad_value(701), p.p507, s.ad_value(702), p.p508, s.ad_value(703), p.p509, p.p506);
        }

        s.v[167] = ((p.p80 * ((((s.v[694]) as f64).powf(p.p81) - ((s.v[699]) as f64).powf(p.p81))).max(0.0)) + (p.p82 * ((((s.v[694]) as f64).powf(p.p83) - ((s.v[699]) as f64).powf(p.p83))).max(0.0)));

        s.v[168] = ((p.p84 * ((((s.v[695]) as f64).powf(p.p85) - ((s.v[700]) as f64).powf(p.p85))).max(0.0)) + (p.p86 * (((s.v[695] * s.v[694])) as f64).powf(p.p87)));

        s.store_scale(706, 706, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p237 * ((((s.v[694]) as f64).powf(p.p238) - ((s.v[699]) as f64).powf(p.p238))).max(0.0));

        s.v[168] = ((p.p239 * ((((s.v[695]) as f64).powf(p.p240) - ((s.v[700]) as f64).powf(p.p240))).max(0.0)) + (p.p241 * ((s.v[696]) as f64).powf(p.p242)));

        s.store_scale(720, 720, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (1.0 + (p.p282 * ((((s.v[694]) as f64).powf(p.p283) - ((s.v[699]) as f64).powf(p.p283))).max(0.0)));

        s.store_scale(710, 710, s.v[167]);

        s.b[1150] = (p.p35 != 0.0);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if s.b[1150] {
            s.store_scale(839, 839, s.v[167]);
            s.store_scale(717, 717, s.v[167]);
        }

        s.store_scale(719, 719, (1.0 + (p.p289 * ((((s.v[694]) as f64).powf(p.p290) - ((s.v[699]) as f64).powf(p.p290))).max(0.0))));

        s.store_scale(738, 738, p.p24);

        s.b[1151] = (p.p42 != 1.0);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        s.b[1152] = (p.p339 > 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if (s.b[1151] && s.b[1152]) {
            s.store_scale(738, 738, (1.0 - (p.p338 * ((((s.v[694]) as f64).powf(p.p339) - ((s.v[699]) as f64).powf(p.p339))).max(0.0))));
        }

        s.b[1153] = (p.p35 != 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if ((s.b[1151] && s.b[1152]) && s.b[1153]) {
            s.store_scale(739, 739, (1.0 - (p.p338 * ((((s.v[694]) as f64).powf(p.p339) - ((s.v[699]) as f64).powf(p.p339))).max(0.0))));
        }

        if (s.b[1151] && (!s.b[1152])) {
            s.store_scale(738, 738, (1.0 - p.p338));
        }

        s.b[1154] = (p.p35 != 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if ((s.b[1151] && (!s.b[1152])) && s.b[1154]) {
            s.store_scale(739, 739, (1.0 - p.p338));
        }

        if (!s.b[1151]) {
            let assign4590_ad_e6159: A = A::scale(s.ad_value(738), ((1.0 - (p.p333 * { let limited_exp_arg = ((-s.v[184]) / p.p334); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p.p335 * { let limited_exp_arg = ((-s.v[184]) / p.p336); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
            s.store_ad_value(738, assign4590_ad_e6159);
        }

        s.b[1155] = (p.p35 != 0.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if ((!s.b[1151]) && s.b[1155]) {
            let assign4610_ad_e6187: A = A::scale(s.ad_value(739), ((1.0 - (p.p333 * { let limited_exp_arg = ((-s.v[184]) / p.p334); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p.p335 * { let limited_exp_arg = ((-s.v[184]) / p.p336); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
            s.store_ad_value(739, assign4610_ad_e6187);
        }

        s.v[167] = (p.p349 * ((((s.v[694]) as f64).powf(p.p350) - ((s.v[699]) as f64).powf(p.p350))).max(0.0));

        s.v[168] = ((p.p351 * ((((s.v[695]) as f64).powf(p.p352) - ((s.v[700]) as f64).powf(p.p352))).max(0.0)) + (p.p353 * ((s.v[696]) as f64).powf(p.p354)));

        s.store_scale(748, 748, ((1.0 + s.v[167]) + s.v[168]));

        s.b[1156] = (p.p35 != 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_scale(749, 749, ((1.0 + s.v[167]) + s.v[168]));
        }

        s.v[167] = (p.p366 * ((((s.v[694]) as f64).powf(p.p367) - ((s.v[699]) as f64).powf(p.p367))).max(0.0));

        s.v[168] = ((p.p368 * ((((s.v[695]) as f64).powf(p.p369) - ((s.v[700]) as f64).powf(p.p369))).max(0.0)) + (p.p370 * ((s.v[696]) as f64).powf(p.p371)));

        s.store_scale(751, 751, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (1.0 + (p.p373 * ((((s.v[694]) as f64).powf(p.p374) - ((s.v[699]) as f64).powf(p.p374))).max(0.0)));

        s.store_scale(752, 752, s.v[167]);

        s.b[1157] = (p.p35 != 0.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_scale(753, 753, s.v[167]);
        }

        s.v[167] = (p.p391 * ((((s.v[694]) as f64).powf(p.p392) - ((s.v[699]) as f64).powf(p.p392))).max(0.0));

        s.v[168] = ((p.p393 * ((((s.v[695]) as f64).powf(p.p394) - ((s.v[700]) as f64).powf(p.p394))).max(0.0)) + (p.p395 * ((s.v[696]) as f64).powf(p.p396)));

        s.store_scale(758, 758, ((1.0 + s.v[167]) + s.v[168]));

        s.b[1158] = (p.p35 != 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if s.b[1158] {
            s.store_scale(759, 759, ((1.0 + s.v[167]) + s.v[168]));
        }

        s.v[167] = ((((s.v[694]) as f64).powf(p.p202) - ((s.v[699]) as f64).powf(p.p202))).max(0.0);

        s.store_scale(730, 730, s.v[167]);

        s.b[1159] = (p.p35 != 0.0);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if s.b[1159] {
            s.store_scale(731, 731, s.v[167]);
        }

        s.store_scale(733, 733, ((((s.v[694]) as f64).powf(p.p204) - ((s.v[699]) as f64).powf(p.p204))).max(0.0));

        s.v[167] = (1.0 + (p.p531 * ((((s.v[694]) as f64).powf(p.p532) - ((s.v[699]) as f64).powf(p.p532))).max(0.0)));

        s.store_scale(761, 761, s.v[167]);

        s.b[1160] = (p.p35 != 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if s.b[1160] {
            s.store_scale(762, 762, s.v[167]);
        }

    }
}
